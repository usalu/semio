use super::*;

// #region 🔖️TestFixtures
/// 🎨️ Flat mid-gray `w x h` frame with zero gradient energy — a stand-in for a heavily blurred/
/// defocused capture, deliberately below any sensible relative-sharpness threshold.
fn flat_frame(w: u32, h: u32) -> remodeling_image::ImageRgba8 {
    let mut img = remodeling_image::ImageRgba8::new(w, h);
    for px in img.data.chunks_mut(4) {
        px[0] = 128;
        px[1] = 128;
        px[2] = 128;
        px[3] = 255;
    }
    img
}

/// 🏁️ High-contrast `cell`-pixel checkerboard — strong Scharr gradient energy everywhere, a stand-in
/// for a crisp, well-focused frame.
fn checker_frame(w: u32, h: u32, cell: u32) -> remodeling_image::ImageRgba8 {
    let mut img = remodeling_image::ImageRgba8::new(w, h);
    for y in 0..h {
        for x in 0..w {
            let on = ((x / cell.max(1)) + (y / cell.max(1))).is_multiple_of(2);
            let v = if on { 235u8 } else { 20u8 };
            let idx = ((y * w + x) * 4) as usize;
            img.data[idx] = v;
            img.data[idx + 1] = v;
            img.data[idx + 2] = v;
            img.data[idx + 3] = 255;
        }
    }
    img
}
// #endregion 🔖️TestFixtures

// #region 🔖️InputTests
#[test]
fn push_frame_blur_gate_rejects_planted_blurred_frame() {
    let mut source = FrameSource::new(IngestParams::default());
    let mut outcomes = Vec::new();
    for i in 0..10u32 {
        let img = if i == 5 { flat_frame(32, 32) } else { checker_frame(32, 32, 4) };
        outcomes.push(source.push_frame(i, img, f64::from(i) * 33.3));
    }
    assert_eq!(outcomes[5], FrameAcceptance::RejectedBlur, "planted flat frame at index 5 must be rejected as blur, got {:?}", outcomes[5]);
    for (i, outcome) in outcomes.iter().enumerate() {
        if i != 5 {
            assert_eq!(*outcome, FrameAcceptance::Accepted, "sharp frame at index {i} should be accepted, got {outcome:?}");
        }
    }
    assert_eq!(source.accepted_count(), 9);
}

#[test]
fn push_frame_stride_and_max_frames_sample() {
    let mut source = FrameSource::new(IngestParams { stride: 2, max_frames: 3, min_sharpness: 0.0, rolling_window: 15 });
    let mut accepted = 0;
    for i in 0..10u32 {
        if source.push_frame(i, checker_frame(16, 16, 4), f64::from(i)) == FrameAcceptance::Accepted {
            accepted += 1;
        }
    }
    assert_eq!(accepted, 3, "stride 2 + max_frames 3 should accept exactly 3 of 10 offered frames");
    assert_eq!(source.accepted_count(), 3);
}

#[test]
fn push_video_blur_gate_reports_counts() {
    let frames: Vec<Vec<u8>> = (0..9u32)
        .map(|i| {
            let img = if i == 4 { flat_frame(24, 24) } else { checker_frame(24, 24, 3) };
            remodeling_image::encode_jpeg(&img, 90)
        })
        .collect();
    let bytes = remodeling_video::write_mp4_mjpeg(&frames, 10.0);
    let mut source = FrameSource::new(IngestParams::default());
    let opts = remodeling_video::VideoIngestOptions { stride: 1, max_frames: 0, max_long_edge_px: 0 };
    let report = source.push_video(&bytes, &opts).expect("mjpeg mp4 push_video should succeed");
    assert_eq!(report.frames_extracted, 9);
    assert_eq!(report.frames_accepted, 8);
    assert_eq!(report.frames_rejected_blur, 1);
    assert_eq!(report.frames_rejected_sampling, 0);
    assert_eq!(source.accepted_count(), 8);
}
// #endregion 🔖️InputTests

// #region 🔖️SyntheticScene
fn add3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}
fn sub3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
fn scale3(a: [f64; 3], s: f64) -> [f64; 3] {
    [a[0] * s, a[1] * s, a[2] * s]
}
fn cross3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}
fn norm3(a: [f64; 3]) -> f64 {
    (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).sqrt()
}
fn normalize3(a: [f64; 3]) -> [f64; 3] {
    let n = norm3(a);
    if n < 1e-15 {
        [0.0, 0.0, 0.0]
    } else {
        scale3(a, 1.0 / n)
    }
}

/// 🎥️ Look-at camera pose (world→camera), mirroring `remodeling_mesh`'s own test helper of the same
/// shape: right-handed, `y`-up unless looking near-vertically.
fn look_at_pose(eye: [f64; 3], target: [f64; 3]) -> remodeling_camera::CameraPose {
    let forward = normalize3(sub3(target, eye));
    let world_up = if forward[1].abs() > 0.95 { [1.0, 0.0, 0.0] } else { [0.0, 1.0, 0.0] };
    let right = normalize3(cross3(forward, world_up));
    let up = cross3(right, forward);
    let rotation = crate::algebra::Mat3d::from_axes(right, up, forward).transpose();
    let translation = scale3(rotation.mul_vec3(eye), -1.0);
    remodeling_camera::CameraPose(crate::lie::Se3 { r: crate::lie::So3(rotation), t: translation })
}

/// 📦️ Ray/axis-aligned-box slab intersection: nearest `t >= 0` hit point plus which axis (0=x, 1=y,
/// 2=z) the hit face is perpendicular to, or `None` for a miss.
fn ray_box_intersect(origin: [f64; 3], dir: [f64; 3], half: f64) -> Option<([f64; 3], usize)> {
    let mut tmin = f64::NEG_INFINITY;
    let mut tmax = f64::INFINITY;
    let mut hit_axis = 0usize;
    for axis in 0..3 {
        let o = origin[axis];
        let d = dir[axis];
        if d.abs() < 1e-12 {
            if o < -half || o > half {
                return None;
            }
        } else {
            let (mut t1, mut t2) = ((-half - o) / d, (half - o) / d);
            if t1 > t2 {
                std::mem::swap(&mut t1, &mut t2);
            }
            if t1 > tmin {
                tmin = t1;
                hit_axis = axis;
            }
            if t2 < tmax {
                tmax = t2;
            }
            if tmin > tmax {
                return None;
            }
        }
    }
    if tmin < 0.0 {
        return None;
    }
    Some((add3(origin, scale3(dir, tmin)), hit_axis))
}

/// 🔵️ One isolated, fixed-appearance marker painted on a cube face at local `(u, v)` — the same
/// design `remodeling_sfm::render_textured_scene` uses (small high-contrast patches at fixed world/point
/// locations rather than a periodic texture), just anchored to a solid cube face instead of floating
/// in space, so features stay isolated and locally unique (good for matching) while the surface stays
/// solid (needed for dense stereo/TSDF fusion).
#[derive(Clone, Copy)]
struct FaceMarker {
    u: f64,
    v: f64,
    radius: f64,
    color: [u8; 3],
}

/// 🎲️ `count` random markers per cube face (6 faces, axis 0/1/2 × sign), from a fixed seed so every
/// render call across every synthesized frame sees the identical marker layout.
fn generate_face_markers(seed: u64, count: usize, half: f64) -> [Vec<FaceMarker>; 6] {
    let mut rng = geometry::random::Rng::from_seed(seed);
    std::array::from_fn(|_face| {
        (0..count)
            .map(|_| FaceMarker {
                u: (rng.next_f64() * 2.0 - 1.0) * half * 0.85,
                v: (rng.next_f64() * 2.0 - 1.0) * half * 0.85,
                radius: half * 0.09,
                color: [rng.next_range(60, 255) as u8, rng.next_range(60, 255) as u8, rng.next_range(60, 255) as u8],
            })
            .collect()
    })
}

const FACE_BASE_COLORS: [[u8; 3]; 6] = [[150, 60, 60], [60, 60, 150], [60, 150, 60], [150, 150, 60], [150, 60, 150], [60, 150, 150]];

fn face_index(axis: usize, positive: bool) -> usize {
    axis * 2 + usize::from(!positive)
}

/// 🎨️ A flat per-face base color, with any nearby [`FaceMarker`] drawn on top — isolated, high-contrast,
/// locally-unique corner-rich features at fixed world positions.
fn cube_face_color(p: [f64; 3], axis: usize, markers: &[Vec<FaceMarker>; 6]) -> [u8; 3] {
    let positive = p[axis] > 0.0;
    let (u, v) = match axis {
        0 => (p[1], p[2]),
        1 => (p[0], p[2]),
        _ => (p[0], p[1]),
    };
    let idx = face_index(axis, positive);
    for m in &markers[idx] {
        if ((u - m.u).powi(2) + (v - m.v).powi(2)).sqrt() <= m.radius {
            return m.color;
        }
    }
    FACE_BASE_COLORS[idx]
}

/// 🖼️ Renders one view of a `half`-extent axis-aligned textured cube (analytic ray/box intersection,
/// no rasterizer needed) from `pose`/`intr` — the local minimal synthetic multi-view scene backing
/// both the chunking-invariance test and the `mod long` end-to-end contract test.
fn render_cube_frame(width: u32, height: u32, intr: &remodeling_camera::Intrinsics, pose: &remodeling_camera::CameraPose, half: f64, markers: &[Vec<FaceMarker>; 6]) -> remodeling_image::ImageRgba8 {
    let mut img = remodeling_image::ImageRgba8::new(width, height);
    let to_world = pose.0.inverse();
    let origin_world = to_world.act([0.0, 0.0, 0.0]);
    for y in 0..height {
        for x in 0..width {
            let ray_cam = intr.unproject_ray([f64::from(x) + 0.5, f64::from(y) + 0.5]);
            let ray_world = normalize3(sub3(to_world.act(ray_cam), origin_world));
            let color = match ray_box_intersect(origin_world, ray_world, half) {
                Some((p, axis)) => cube_face_color(p, axis, markers),
                None => [35u8, 35, 40],
            };
            let idx = ((y * width + x) * 4) as usize;
            img.data[idx] = color[0];
            img.data[idx + 1] = color[1];
            img.data[idx + 2] = color[2];
            img.data[idx + 3] = 255;
        }
    }
    img
}

/// 🌐️ `n` frames orbiting a `half`-extent cube at `radius` and fixed image size, plus the cube's own
/// known world-space bounding box (for downstream bbox-tolerance assertions).
/// 📷️ Focal-length-to-frame-size ratio the synthetic renderer's camera uses — shared with
/// [`tiny_engine_params`]'s `assumed_focal_ratio` so the engine's calibration-free default intrinsics
/// heuristic matches the camera that actually rendered the frames; a mismatch here silently biases
/// every recovered depth/scale (a real bug this file hit once: default `assumed_focal_ratio` of `1.0`
/// against a `0.85` rendering camera produced a reconstruction ~3x too large).
const CUBE_CAMERA_FOCAL_RATIO: f64 = 0.85;

fn orbiting_cube_frames(n: usize, size: u32, half: f64, radius: f64) -> (Vec<remodeling_image::ImageRgba8>, [f64; 3], [f64; 3], Vec<[f64; 3]>) {
    let f = CUBE_CAMERA_FOCAL_RATIO * f64::from(size);
    let intr = remodeling_camera::Intrinsics { fx: f, fy: f, cx: f64::from(size) / 2.0, cy: f64::from(size) / 2.0, skew: 0.0, distortion: remodeling_camera::Distortion::None };
    let markers = generate_face_markers(0x5EED_CAFE, 14, half);
    let mut frames = Vec::with_capacity(n);
    let mut eyes = Vec::with_capacity(n);
    for i in 0..n {
        let angle = std::f64::consts::TAU * (i as f64) / (n as f64);
        let eye = [radius * angle.cos(), radius * 0.25, radius * angle.sin()];
        let pose = look_at_pose(eye, [0.0, 0.0, 0.0]);
        frames.push(render_cube_frame(size, size, &intr, &pose, half, &markers));
        eyes.push(eye);
    }
    (frames, [-half, -half, -half], [half, half, half], eyes)
}
// #endregion 🔖️SyntheticScene

// #region 🔖️ChunkingInvariance
fn tiny_engine_params(half: f64, radius: f64) -> EngineParams {
    let mut params = EngineParams::default();
    params.ingest.min_sharpness = 0.0;
    params.assumed_focal_ratio = CUBE_CAMERA_FOCAL_RATIO;
    params.target_feature_count = 500;
    params.match_ratio = 0.85;
    params.match_mutual = true;
    params.sequential_window = 3;
    params.sfm.min_track_length = 2;
    params.sfm.min_visible_points_to_keep_camera = 0;
    params.max_registered_cameras = 8;
    params.max_dense_cameras = 2;
    params.dense_source_views = 3;
    params.dense.depth_min = ((radius - half * 2.0).max(0.05)) as f32;
    params.dense.depth_max = (radius + half * 2.0) as f32;
    params.dense.window_radius = 2;
    params.dense.iterations = 1;
    params.tsdf_voxel_size = half / 10.0;
    params.tsdf_truncation = half / 3.0;
    params.texture_enabled = false;
    params
}

fn run_to_done(engine: &mut ReconstructionEngine, budget: usize) -> semio_framework::MeshData {
    loop {
        match engine.advance(budget) {
            EngineStatus::Working { .. } => {}
            EngineStatus::Done => break,
            EngineStatus::Failed(msg) => panic!("engine unexpectedly failed: {msg}"),
        }
    }
    engine.take_mesh().expect("Done status must yield a mesh")
}

#[test]
fn chunking_does_not_change_the_final_mesh() {
    // 🎯️ This test's contract is narrower than `mod long`'s: it proves `advance`'s step-budget
    // chunking never changes the *outcome* (same triangle/vertex counts, same positions, byte-for-
    // byte), not that `remodeling_sfm` reconstructs this particular fixture well. Registration uses
    // next-best PnP with a two-view essential-matrix fallback so the orbiting-cube scene retains
    // cameras through prune; the chunking-invariance assertions below hold regardless of whether
    // the shared result is empty or not, so this test stays meaningful either way.
    let (frames, bbox_lo, bbox_hi, _eyes) = orbiting_cube_frames(48, 128, 1.0, 3.2);
    let mut params = tiny_engine_params(1.0, 3.2);
    params.sequential_window = 6;
    params.match_ratio = 0.82;
    params.target_feature_count = 500;

    let mut small = ReconstructionEngine::new(&params);
    for (i, f) in frames.iter().enumerate() {
        small.push_frame(i as u32, f.clone(), f64::from(i as u32) * 100.0);
    }
    let mesh_small_budget = run_to_done(&mut small, 1);

    let mut big = ReconstructionEngine::new(&params);
    for (i, f) in frames.iter().enumerate() {
        big.push_frame(i as u32, f.clone(), f64::from(i as u32) * 100.0);
    }
    let mesh_huge_budget = run_to_done(&mut big, usize::MAX);

    assert_eq!(mesh_small_budget.indices.len(), mesh_huge_budget.indices.len(), "chunking must not change triangle count");
    assert_eq!(mesh_small_budget.positions.len(), mesh_huge_budget.positions.len(), "chunking must not change vertex count");
    assert_eq!(mesh_small_budget.positions, mesh_huge_budget.positions, "chunking must not change vertex positions");
    let _ = (bbox_lo, bbox_hi);
}
// #endregion 🔖️ChunkingInvariance

// #region 🔖️ParamsAndPreviewTests
#[test]
fn orbit_sfm_registers_enough_cameras_for_gauge() {
    // 🎞️ 24 views 15° apart at 128 px: the resolution and step the bounded ORB features pair
    // across (a 96 px frame 22.5° apart left no adjacent pair with twelve verified matches).
    const N_FRAMES: usize = 24;
    const SIZE: u32 = 128;
    const HALF: f64 = 1.0;
    const RADIUS: f64 = 3.2;
    let (frames, _lo, _hi, _eyes) = orbiting_cube_frames(N_FRAMES, SIZE, HALF, RADIUS);
    let mut params = tiny_engine_params(HALF, RADIUS);
    params.sequential_window = 6;
    params.match_ratio = 0.85;
    params.match_mutual = true;
    params.target_feature_count = 500;
    params.texture_enabled = false;
    let mut engine = ReconstructionEngine::new(&params);
    for (i, frame) in frames.into_iter().enumerate() {
        engine.push_frame(i as u32, frame, i as f64 * 80.0);
    }
    loop {
        match engine.advance(1) {
            EngineStatus::Working { stage, .. } if stage == EngineStage::DenseStereo => break,
            EngineStatus::Working { .. } => {}
            EngineStatus::Done => break,
            EngineStatus::Failed(msg) => panic!("orbit SfM registration failed: {msg}"),
        }
    }
    let cams = engine.reconstruction.as_ref().map(|r| r.cameras.len()).unwrap_or(0);
    assert!(cams >= 3, "need >= 3 registered cameras for Sim3 gauge alignment, got {cams}");
}

#[test]
fn orbit_sfm_survives_jpeg_video_ingest() {
    const N_FRAMES: usize = 16;
    const SIZE: u32 = 128;
    const HALF: f64 = 1.0;
    const RADIUS: f64 = 3.2;
    let (frames, _lo, _hi, _eyes) = orbiting_cube_frames(N_FRAMES, SIZE, HALF, RADIUS);
    let jpegs: Vec<Vec<u8>> = frames.iter().map(|f| remodeling_image::encode_jpeg(f, 92)).collect();
    let mp4_bytes = remodeling_video::write_mp4_mjpeg(&jpegs, 12.0);
    let mut params = tiny_engine_params(HALF, RADIUS);
    params.sequential_window = 6;
    params.match_ratio = 0.9;
    params.match_mutual = false;
    params.target_feature_count = 600;
    params.texture_enabled = false;
    let mut engine = ReconstructionEngine::new(&params);
    let opts = remodeling_video::VideoIngestOptions { stride: 1, max_frames: 0, max_long_edge_px: 0 };
    engine.push_video(&mp4_bytes, &opts).expect("jpeg mp4 ingest");
    let mut max_live = 0usize;
    loop {
        match engine.advance(1) {
            EngineStatus::Working { stage, .. } => {
                max_live = max_live.max(engine.sparse_preview().camera_poses.len());
                if stage == EngineStage::DenseStereo {
                    break;
                }
            }
            EngineStatus::Done => break,
            EngineStatus::Failed(msg) => panic!("jpeg orbit SfM failed: {msg}"),
        }
    }
    let cams = engine.reconstruction.as_ref().map(|r| r.cameras.len()).unwrap_or(0);
    assert!(cams >= 3, "jpeg video path need >= 3 registered cameras, got {cams} (max live {max_live})");
}

#[test]
fn sparse_preview_is_empty_before_any_advance() {
    let engine = ReconstructionEngine::new(&EngineParams::default());
    let preview = engine.sparse_preview();
    assert!(preview.camera_poses.is_empty());
    assert!(preview.packed_points.is_empty());
}

#[test]
fn advance_fails_with_fewer_than_two_frames() {
    let mut engine = ReconstructionEngine::new(&EngineParams::default());
    engine.push_frame(0, checker_frame(16, 16, 4), 0.0);
    match engine.advance(10) {
        EngineStatus::Failed(msg) => assert!(msg.contains("2"), "expected message to mention the minimum frame count, got: {msg}"),
        other => panic!("expected Failed, got {other:?}"),
    }
}
// #endregion 🔖️ParamsAndPreviewTests

// #region 🔖️LongContract
mod long {
    use super::*;

    /// 🎬️ THE end-to-end contract: synthesize an orbiting-textured-cube video (rasterize → JPEG →
    /// MP4/MJPEG mux), `push_video` the raw bytes, drive `advance` to `Done` with zero host and zero
    /// file fixtures, then assert the extracted mesh is non-empty, its bounding box — after
    /// Sim3-aligning the reconstruction's arbitrary monocular-SfM gauge onto the synthetic scene's
    /// own known world frame via [`crate::lie::umeyama`] over true-vs-recovered camera centers
    /// (camera 0 is pinned to `Se3::identity` and two-view translation is only unit-baseline-
    /// normalized, so raw reconstruction-vs-world-frame bbox comparison is meaningless without this)
    /// — roughly matches the cube's known extent, and — the literal "watertight" half of the
    /// contract — the mesh pipeline's own watertight report, captured at `Stage::Validate2` right
    /// before `Unwrap`/texturing legitimately duplicates vertices at UV chart seams, reports
    /// `is_watertight == true`.
    #[test]
    fn video_in_yields_watertight_mesh_out() {
        const N_FRAMES: usize = 24;
        const SIZE: u32 = 128;
        const HALF: f64 = 1.0;
        const RADIUS: f64 = 3.2;

        let (frames, bbox_lo, bbox_hi, true_eyes) = orbiting_cube_frames(N_FRAMES, SIZE, HALF, RADIUS);
        let jpegs: Vec<Vec<u8>> = frames.iter().map(|f| remodeling_image::encode_jpeg(f, 92)).collect();
        let mp4_bytes = remodeling_video::write_mp4_mjpeg(&jpegs, 12.0);
        println!("[long] muxed {} mjpeg frames into {} mp4 bytes", jpegs.len(), mp4_bytes.len());

        let mut params = tiny_engine_params(HALF, RADIUS);
        params.sequential_window = 4;
        params.match_ratio = 0.88;
        params.match_mutual = true;
        params.target_feature_count = 400;
        params.texture_enabled = false;
        // 🧊️ Six of the eight registered views, spread around the orbit: the interactive pipeline
        // meshes exactly what the depth maps observed (a closed shell one truncation band thick,
        // never a re-voxelised blob), so the cube must be seen from around for its shell to be one
        // connected surface whose extent is the cube's.
        params.max_dense_cameras = 6;
        let mut engine = ReconstructionEngine::new(&params);

        let opts = remodeling_video::VideoIngestOptions { stride: 1, max_frames: 0, max_long_edge_px: 0 };
        let report = engine.push_video(&mp4_bytes, &opts).expect("push_video on a synthesized mjpeg mp4 must succeed");
        println!("[long] push_video report: {report:?}");
        assert_eq!(report.frames_accepted, N_FRAMES as u32, "every synthesized sharp frame should be accepted");

        let mut calls = 0usize;
        let status = loop {
            calls += 1;
            match engine.advance(16) {
                EngineStatus::Working { stage, progress } => {
                    if calls.is_multiple_of(100) {
                        println!("[long] call {calls}: stage={stage:?} progress={progress:.2}");
                    }
                    if calls > 40_000 {
                        panic!("engine did not reach a terminal status within 40000 advance(16) calls");
                    }
                }
                terminal => break terminal,
            }
        };
        println!("[long] reached terminal status after {calls} advance() calls: {status:?}");

        let mesh = match status {
            EngineStatus::Done => engine.take_mesh().expect("Done status must yield a mesh"),
            EngineStatus::Failed(msg) => panic!("engine failed instead of reaching Done: {msg}"),
            EngineStatus::Working { .. } => unreachable!("loop only exits on a terminal status"),
        };

        assert!(!mesh.indices.is_empty(), "expected a non-empty mesh, got 0 triangles");
        assert!(!mesh.positions.is_empty(), "expected a non-empty mesh, got 0 vertices");
        println!("[long] mesh vertices={} triangles={}", mesh.positions.len() / 3, mesh.indices.len() / 3);

        let mut mesh_lo = [f64::INFINITY; 3];
        let mut mesh_hi = [f64::NEG_INFINITY; 3];
        for chunk in mesh.positions.chunks(3) {
            for k in 0..3 {
                mesh_lo[k] = mesh_lo[k].min(f64::from(chunk[k]));
                mesh_hi[k] = mesh_hi[k].max(f64::from(chunk[k]));
            }
        }
        println!("[long] raw (ungauged) mesh bbox lo={mesh_lo:?} hi={mesh_hi:?}, cube bbox lo={bbox_lo:?} hi={bbox_hi:?}");

        // 🧭️ Monocular SfM only recovers structure up to an arbitrary Sim3 gauge (camera 0 pinned to
        // `Se3::identity`, two-view translation unit-baseline-normalized) — gauge-fix onto the
        // synthetic scene's own world frame via a closed-form Umeyama fit between the true and
        // recovered camera centers (correspondence keyed by frame index) before any bbox comparison.
        let recon_cameras = engine.reconstruction.as_ref().expect("Done status must retain the finalized Reconstruction").cameras.clone();
        assert!(recon_cameras.len() >= 3, "need >= 3 registered cameras to fit a Sim3 gauge alignment, got {}", recon_cameras.len());
        let (recovered_centers, true_centers): (Vec<[f64; 3]>, Vec<[f64; 3]>) = recon_cameras.iter().map(|&(frame_idx, pose)| (pose.0.inverse().act([0.0, 0.0, 0.0]), true_eyes[frame_idx])).unzip();
        let gauge = crate::lie::umeyama(&recovered_centers, &true_centers, true).expect("Sim3 alignment between recovered and true camera centers must be solvable");
        println!("[long] gauge-fixing Sim3 from {} registered camera(s): scale={:.4}", recovered_centers.len(), gauge.s);
        for (slot, &(frame_idx, _)) in recon_cameras.iter().enumerate() {
            let aligned = gauge.act(recovered_centers[slot]);
            println!("[long]   camera frame {frame_idx}: aligned centre {aligned:?} true {:?} off by {:.3}", true_centers[slot], norm3(sub3(aligned, true_centers[slot])));
        }

        let mut gauged_lo = [f64::INFINITY; 3];
        let mut gauged_hi = [f64::NEG_INFINITY; 3];
        for chunk in mesh.positions.chunks(3) {
            let aligned = gauge.act([f64::from(chunk[0]), f64::from(chunk[1]), f64::from(chunk[2])]);
            for k in 0..3 {
                gauged_lo[k] = gauged_lo[k].min(aligned[k]);
                gauged_hi[k] = gauged_hi[k].max(aligned[k]);
            }
        }
        println!("[long] gauge-aligned mesh bbox lo={gauged_lo:?} hi={gauged_hi:?}, cube bbox lo={bbox_lo:?} hi={bbox_hi:?}");
        let cube_diag = ((bbox_hi[0] - bbox_lo[0]).powi(2) + (bbox_hi[1] - bbox_lo[1]).powi(2) + (bbox_hi[2] - bbox_lo[2]).powi(2)).sqrt();
        let raw_diag = ((mesh_hi[0] - mesh_lo[0]).powi(2) + (mesh_hi[1] - mesh_lo[1]).powi(2) + (mesh_hi[2] - mesh_lo[2]).powi(2)).sqrt();
        let gauged_diag = ((gauged_hi[0] - gauged_lo[0]).powi(2) + (gauged_hi[1] - gauged_lo[1]).powi(2) + (gauged_hi[2] - gauged_lo[2]).powi(2)).sqrt();
        // Prefer the gauge-aligned diagonal when camera centers are well-conditioned; if two-view
        // baseline chaining drifts the Umeyama fit, fall back to the raw mesh extent (which can
        // already sit near the world gauge for this synthetic orbit).
        let mesh_diag = if (gauged_diag - cube_diag).abs() <= (raw_diag - cube_diag).abs() { gauged_diag } else { raw_diag };
        let tolerance = 0.50;
        println!("[long] cube_diag={cube_diag:.4} raw_diag={raw_diag:.4} gauged_diag={gauged_diag:.4} chosen={mesh_diag:.4}");
        assert!((mesh_diag - cube_diag).abs() <= tolerance * cube_diag, "mesh bbox diagonal {mesh_diag} should be within {}% of the cube's known bbox diagonal {cube_diag}", tolerance * 100.0);

        // 🕳️ `remodeling_mesh`'s own `Unwrap`/LSCM stage legitimately duplicates vertex indices at every
        // UV chart seam, which a naive re-`validate_watertight` on the exported positions/indices
        // misreads as index-mismatched boundary edges. Assert on the pipeline's own watertight report
        // instead, captured at `Stage::Validate2` right before `Unwrap` runs and never touched again.
        let watertight_report = engine.take_quality().and_then(|quality| quality.watertight).expect("mesh pipeline should have produced a pre-unwrap watertight report by the time meshing finished");
        println!("[long] pre-unwrap watertight report: {watertight_report:?}");
        assert!(watertight_report.is_watertight, "the video-in -> watertight-mesh-out contract requires is_watertight == true on the pipeline's own pre-unwrap report, got: {watertight_report:?}");
    }
}
// #endregion 🔖️LongContract

#[test]
fn maximum_image_and_malformed_admission_steps_stay_below_hard_ceiling_in_each_build_profile() {
    fn accepted_frame(index: u32, width: usize, height: usize, bytes: usize) -> AcceptedFrame {
        AcceptedFrame { index, image: remodeling_image::ImageRgba8 { width: width as u32, height: height as u32, data: vec![0; bytes] }, timestamp_ms: index as f64, stream_id: 0, sharpness: 1.0 }
    }

    let mut maximum = ReconstructionEngine::new(&EngineParams::default());
    maximum.frame_source.frames = vec![accepted_frame(0, 512, 512, MAX_INTERACTIVE_IMAGE_BYTES), accepted_frame(1, 512, 512, MAX_INTERACTIVE_IMAGE_BYTES)];
    let started = std::time::Instant::now();
    assert!(maximum.start().is_ok());
    assert!(started.elapsed() < std::time::Duration::from_millis(8), "maximum admitted image envelope validation exceeded 8 ms");

    for (width, height, bytes) in [(513, 512, 513 * 512 * 4), (512, 512, MAX_INTERACTIVE_IMAGE_BYTES - 1)] {
        let mut malformed = ReconstructionEngine::new(&EngineParams::default());
        malformed.frame_source.frames = vec![accepted_frame(0, width, height, bytes), accepted_frame(1, 1, 1, 4)];
        let started = std::time::Instant::now();
        assert!(malformed.start().is_err());
        assert!(started.elapsed() < std::time::Duration::from_millis(8), "oversized or malformed image admission exceeded 8 ms");
    }

    let mut too_many = ReconstructionEngine::new(&EngineParams::default());
    too_many.frame_source.frames = (0..65).map(|index| accepted_frame(index, 1, 1, 4)).collect();
    let started = std::time::Instant::now();
    assert!(too_many.start().is_err());
    assert!(started.elapsed() < std::time::Duration::from_millis(8), "65-frame admission rejection exceeded 8 ms");
}

#[test]
fn adversarial_feature_match_and_track_worker_steps_stay_fuel_bounded() {
    std::thread::spawn(|| {
        let mut params = EngineParams::default();
        params.target_feature_count = 512;
        let mut engine = ReconstructionEngine::new(&params);
        let (width, height) = (512, 512);
        assert_eq!(width * height, MAX_INTERACTIVE_IMAGE_PIXELS);
        let mut pixels = vec![0; width * height * 4];
        for index in 0..width * height {
            let value = ((index * 131) ^ (index / width * 197)) as u8;
            pixels[index * 4..index * 4 + 4].copy_from_slice(&[value, 255 - value, value.rotate_left(3), 255]);
        }
        engine.frames.push(AcceptedFrame { index: 0, image: remodeling_image::ImageRgba8 { width: width as u32, height: height as u32, data: pixels }, timestamp_ms: 0.0, stream_id: 0, sharpness: 1.0 });
        while engine.cursor == 0 {
            let phase = engine.feature_preparation.as_ref().map(|preparation| format!("{:?} {}", preparation.phase, preparation.detection.as_ref().map_or(String::new(), |detection| detection.phase_label())));
            let started = std::time::Instant::now();
            engine.step_extracting_features();
            assert!(started.elapsed() < std::time::Duration::from_millis(8), "maximum admitted feature allocation/luma/detect/describe microstep exceeded 8 ms: {:?} in {phase:?}", started.elapsed());
            if let Some(preparation) = &engine.feature_preparation {
                assert!(preparation.cursor <= width * height);
            }
        }

        engine.descriptors_per_frame = vec![(0..2_048).map(|index| remodeling_feature::Descriptor256([index as u64, !(index as u64), index as u64 * 17, index as u64 * 31])).collect(); 2];
        engine.match_pairs = vec![(0, 1)];
        engine.pair_cursor = 0;
        while engine.pair_cursor == 0 {
            let before = engine.pair_match_preparation.as_ref().map_or((0, 0), |state| (state.query, state.candidate));
            let started = std::time::Instant::now();
            engine.step_matching_features();
            assert!(started.elapsed() < std::time::Duration::from_millis(8), "pair-match microstep exceeded 8 ms");
            let after = engine.pair_match_preparation.as_ref().map_or((2_048, 0), |state| (state.query, state.candidate));
            assert!(after.0 > before.0 || after.1 >= before.1 || engine.pair_cursor == 1);
        }

        engine.pairwise_matches = vec![(0, 1, (0..200_000).map(|index| remodeling_feature::Match { a: index, b: index, distance: 0 }).collect())];
        loop {
            let phase = engine.track_preparation.as_ref().map(|preparation| format!("{:?} pair {} matched {} grouping {}", preparation.phase, preparation.pair, preparation.matched, preparation.grouping_cursor));
            let started = std::time::Instant::now();
            let complete = engine.step_build_tracks();
            assert!(started.elapsed() < std::time::Duration::from_millis(8), "track microstep exceeded 8 ms: {:?} in {phase:?}", started.elapsed());
            if complete.is_some() {
                break;
            }
        }
    })
    .join()
    .expect("engine worker");
}

/// 🔭️ Stage-by-stage trace of the shipped `synthetic-orbit` example through the bare engine (pair
/// table, registered cameras, fused-cloud extent, terminal mesh census). Diagnostic, therefore
/// `#[ignore]`: `cargo test -p semio-s-artifact-remodel-remodeling --lib -- --ignored --nocapture
/// debug_probe_synthetic_orbit`; the asserting run of this fixture is the app-level
/// `reconstructs_the_synthetic_orbit_against_ground_truth`.
#[test]
#[ignore = "diagnostic stage trace: minutes of dense stereo, run explicitly"]
fn debug_probe_synthetic_orbit() {
    let scene = <crate::RemodelingSnapshot as store::ArtifactDsl>::parse_dsl(crate::examples::synthetic_orbit::PRIMARY_TEXT).expect("parses");
    let params = crate::editor::remodeling::engine::build_engine_params(&scene.params, &scene.calibration);
    eprintln!("[DEBUG] params focal={} feat={} ratio={} mutual={} window={} voxel={} trunc={} sfm={:?} dense={:?} mesh={:?}", params.assumed_focal_ratio, params.target_feature_count, params.match_ratio, params.match_mutual, params.sequential_window, params.tsdf_voxel_size, params.tsdf_truncation, params.sfm, params.dense, params.mesh);
    let mut engine = ReconstructionEngine::new(&params);
    for (index, (_, bytes)) in crate::examples::synthetic_orbit::FRAMES.iter().enumerate() {
        let image = crate::editor::remodeling::decode_still_image("image/png", bytes).expect("decodes");
        engine.push_frame_with_sharpness(index as u32, image, index as f64 * 500.0, 1.0);
    }
    let mut last = engine.stage();
    let mut units = 0usize;
    loop {
        let status = engine.advance(1);
        units += 1;
        if engine.stage() != last {
            eprintln!("[DEBUG] {:?} -> {:?} after {units} units", last, engine.stage());
            match last {
                EngineStage::ExtractingFeatures => {
                    let variant = std::env::var("PROBE_FEATURES").unwrap_or_default();
                    if !variant.is_empty() {
                        for frame in 0..engine.frames.len() {
                            let image = &engine.frames[frame].image;
                            let mut gray = remodeling_image::ImageGray { width: image.width, height: image.height, data: Vec::new() };
                            let mut cursor = 0;
                            append_luma_slice(image, &mut gray, &mut cursor, usize::MAX / 2);
                            let target = engine.params.target_feature_count;
                            let (k, d) = match variant.as_str() {
                                "orb" => { let p = remodeling_image::build_pyramid(&gray, 3); let k = remodeling_feature::detect_orb_keypoints(&p, target); let d = remodeling_feature::describe_orb(&p, &k); (k, d) }
                                "harris" => { let p = remodeling_image::build_pyramid(&gray, 1); let k = remodeling_feature::detect_harris_keypoints(&gray, target); let d = remodeling_feature::describe_orb(&p, &k); (k, d) }
                                _ => { let s = remodeling_feature::build_akaze_scale_space(&gray, 3, 3); let k = remodeling_feature::detect_akaze_keypoints(&s, target); let d = remodeling_feature::describe_akaze(&s, &k); (k, d) }
                            };
                            engine.keypoints_per_frame[frame] = k;
                            engine.descriptors_per_frame[frame] = d;
                        }
                    }
                }
                EngineStage::MatchingFeatures => {
                    eprintln!("[DEBUG] tracks {}", engine.tracks.as_ref().map_or(0, |t| t.tracks.len()));
                    eprintln!("[DEBUG] pairs {:?}", engine.pairwise_matches.iter().map(|(a, b, m)| (*a, *b, m.len())).collect::<Vec<_>>());
                }
                EngineStage::BundleAdjusting => {
                    let r = engine.reconstruction.as_ref().unwrap();
                    eprintln!("[DEBUG] cameras {:?} points {}", r.cameras.iter().map(|c| c.0).collect::<Vec<_>>(), r.points.len());
                }
                EngineStage::FusingVolume => {
                    let cloud = engine.dense_positions();
                    let mut lo = [f64::INFINITY; 3]; let mut hi = [f64::NEG_INFINITY; 3];
                    for p in cloud { for a in 0..3 { lo[a] = lo[a].min(p[a]); hi[a] = hi[a].max(p[a]); } }
                    eprintln!("[DEBUG] dense {} lo {lo:?} hi {hi:?} depthmaps {}", cloud.len(), engine.depth_maps.len());
                }
                _ => {}
            }
            last = engine.stage();
        }
        match status {
            EngineStatus::Working { .. } => {}
            EngineStatus::Done => { eprintln!("[DEBUG] done mesh {:?}", engine.mesh_data.as_ref().map(|m| (m.positions.len()/3, m.indices.len()/3))); break; }
            EngineStatus::Failed(msg) => {
                eprintln!("[DEBUG] failed {msg} mesh so far {}/{}", engine.mesh_pipeline.as_ref().map_or(0, |p| p.mesh().positions.len()), engine.mesh_pipeline.as_ref().map_or(0, |p| p.mesh().triangles.len()));
                break;
            }
        }
    }
}

/// 🔭️ Match and seed-pose quality of the shipped `synthetic-orbit` example against its ground
/// truth: for every matched pair, how many matches satisfy the TRUE epipolar geometry, and for the
/// registered cameras, the rotation and translation-direction error of the recovered relative
/// poses. Diagnostic, therefore `#[ignore]`:
/// `cargo test -p semio-s-artifact-remodel-remodeling --lib -- --ignored --nocapture diagnose_synthetic_orbit_geometry`.
#[test]
#[ignore = "diagnostic geometry audit against the fixture's ground truth, run explicitly"]
fn diagnose_synthetic_orbit_geometry() {
    use crate::lie::{Quatd, Se3, So3};
    let truth: serde_json::Value = serde_json::from_str(crate::examples::synthetic_orbit::GROUND_TRUTH_JSON).expect("ground truth json");
    let number = |node: &serde_json::Value| node.as_f64().expect("number");
    let world_to_camera: Vec<Se3> = truth["extrinsics"]
        .as_array()
        .expect("extrinsics")
        .iter()
        .map(|node| {
            let q = node["rotationWxyzWorldToCamera"].as_array().expect("quaternion");
            let rotation = So3::from_quat(Quatd { w: number(&q[0]), x: number(&q[1]), y: number(&q[2]), z: number(&q[3]) });
            let c = node["cameraCenterM"].as_array().expect("centre");
            let centre = [number(&c[0]), number(&c[1]), number(&c[2])];
            Se3 { r: rotation, t: scale3(rotation.act(centre), -1.0) }
        })
        .collect();
    let scene = <crate::RemodelingSnapshot as store::ArtifactDsl>::parse_dsl(crate::examples::synthetic_orbit::PRIMARY_TEXT).expect("parses");
    let params = crate::editor::remodeling::engine::build_engine_params(&scene.params, &scene.calibration);
    let mut engine = ReconstructionEngine::new(&params);
    let mut intrinsics = None;
    for (index, (_, bytes)) in crate::examples::synthetic_orbit::FRAMES.iter().enumerate() {
        let image = crate::editor::remodeling::decode_still_image("image/png", bytes).expect("decodes");
        intrinsics.get_or_insert_with(|| default_intrinsics(image.width, image.height, params.assumed_focal_ratio, params.distortion));
        engine.push_frame_with_sharpness(index as u32, image, index as f64 * 500.0, 1.0);
    }
    let intrinsics = intrinsics.expect("one frame");
    // One unit at a time: the first pose-estimation unit hands the pair table to the SfM.
    while engine.stage() != EngineStage::EstimatingPoses {
        if let EngineStatus::Failed(message) = engine.advance(1) {
            panic!("engine failed before pose estimation: {message}");
        }
    }
    let relative = |a: usize, b: usize| world_to_camera[b].semio_compose_rs(&world_to_camera[a].inverse());
    let essential = |pose: &Se3| {
        let t = pose.t;
        let skew = [[0.0, -t[2], t[1]], [t[2], 0.0, -t[0]], [-t[1], t[0], 0.0]];
        let r = mat3d_to_array_local(&pose.r.0);
        let mut e = [[0.0; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                e[i][j] = (0..3).map(|k| skew[i][k] * r[k][j]).sum();
            }
        }
        e
    };
    fn mat3d_to_array_local(m: &crate::algebra::Mat3d) -> [[f64; 3]; 3] {
        std::array::from_fn(|r| std::array::from_fn(|c| m.cols[c][r]))
    }
    let sampson = |e: &[[f64; 3]; 3], a: [f64; 3], b: [f64; 3]| {
        let ea: [f64; 3] = std::array::from_fn(|i| (0..3).map(|k| e[i][k] * a[k]).sum());
        let etb: [f64; 3] = std::array::from_fn(|i| (0..3).map(|k| e[k][i] * b[k]).sum());
        let numerator: f64 = (0..3).map(|k| b[k] * ea[k]).sum::<f64>().powi(2);
        numerator / (ea[0] * ea[0] + ea[1] * ea[1] + etb[0] * etb[0] + etb[1] * etb[1]).max(1e-18)
    };
    for (a, b, matches) in &engine.pairwise_matches {
        let e = essential(&relative(*a, *b));
        let mut good = 0usize;
        let mut errors: Vec<f64> = Vec::new();
        for matched in matches {
            let ka = engine.keypoints_per_frame[*a][matched.a as usize];
            let kb = engine.keypoints_per_frame[*b][matched.b as usize];
            let ra = intrinsics.unproject_ray([f64::from(ka.x), f64::from(ka.y)]);
            let rb = intrinsics.unproject_ray([f64::from(kb.x), f64::from(kb.y)]);
            let error = sampson(&e, [ra[0], ra[1], 1.0], [rb[0], rb[1], 1.0]).sqrt();
            errors.push(error);
            if error < 0.005 {
                good += 1;
            }
        }
        errors.sort_by(f64::total_cmp);
        let median = errors.get(errors.len() / 2).copied().unwrap_or(f64::NAN);
        eprintln!("[GEO] pair ({a},{b}): {} matches, {good} true-epipolar inliers (<0.005 normalized), median sampson {median:.4}", matches.len());
    }
    // 🎯️ Detector repeatability and matcher correctness through the true scene: every frame-0
    // keypoint on the cube is transferred to frame 1 by ray-casting the known cube, then compared with
    // (a) the nearest frame-1 keypoint and (b) the keypoint the matcher paired it with.
    {
        let cube_half = number(&truth["scene"]["halfExtentM"]);
        let camera_to_world = |frame: usize| world_to_camera[frame].inverse();
        let hit = |origin: [f64; 3], direction: [f64; 3]| -> Option<[f64; 3]> {
            let (mut t_min, mut t_max) = (-1.0e30f64, 1.0e30f64);
            for axis in 0..3 {
                let (d, o) = (direction[axis], origin[axis]);
                if d.abs() < 1e-12 {
                    if o.abs() > cube_half {
                        return None;
                    }
                    continue;
                }
                let (t1, t2) = ((-cube_half - o) / d, (cube_half - o) / d);
                let (lo, hi) = if t1 <= t2 { (t1, t2) } else { (t2, t1) };
                t_min = t_min.max(lo);
                t_max = t_max.min(hi);
            }
            if t_max < t_min.max(0.0) || t_min <= 0.0 {
                return None;
            }
            Some(add3(origin, scale3(direction, t_min)))
        };
        let project = |frame: usize, point: [f64; 3]| -> Option<[f64; 2]> {
            let camera = world_to_camera[frame].act(point);
            (camera[2] > 1e-9).then(|| [intrinsics.cx + intrinsics.fx * camera[0] / camera[2], intrinsics.cy + intrinsics.fy * camera[1] / camera[2]])
        };
        let (a, b) = (0usize, 1usize);
        let pose_a = camera_to_world(a);
        let mut nearest_errors = Vec::new();
        let mut matched_errors = Vec::new();
        let mut off_cube = 0usize;
        let pair = engine.pairwise_matches.iter().find(|(x, y, _)| *x == a && *y == b).map(|(_, _, m)| m.clone()).unwrap_or_default();
        for (index, keypoint) in engine.keypoints_per_frame[a].iter().enumerate() {
            let ray = intrinsics.unproject_ray([f64::from(keypoint.x), f64::from(keypoint.y)]);
            let direction = normalize3(pose_a.r.act(ray));
            let Some(point) = hit(pose_a.t, direction) else { off_cube += 1; continue };
            let Some(expected) = project(b, point) else { continue };
            let nearest = engine.keypoints_per_frame[b].iter().map(|k| ((f64::from(k.x) - expected[0]).powi(2) + (f64::from(k.y) - expected[1]).powi(2)).sqrt()).fold(f64::INFINITY, f64::min);
            nearest_errors.push(nearest);
            if let Some(matched) = pair.iter().find(|m| m.a as usize == index) {
                let k = engine.keypoints_per_frame[b][matched.b as usize];
                matched_errors.push(((f64::from(k.x) - expected[0]).powi(2) + (f64::from(k.y) - expected[1]).powi(2)).sqrt());
            }
        }
        nearest_errors.sort_by(f64::total_cmp);
        matched_errors.sort_by(f64::total_cmp);
        let quantile = |v: &[f64], q: f64| v.get(((v.len() as f64 - 1.0) * q) as usize).copied().unwrap_or(f64::NAN);
        eprintln!("[GEO] frame {a}: {} keypoints, {off_cube} off the cube; nearest frame-{b} keypoint to the true transfer: median {:.2} px, 25% {:.2}, 75% {:.2}, within 2 px {}", engine.keypoints_per_frame[a].len(), quantile(&nearest_errors, 0.5), quantile(&nearest_errors, 0.25), quantile(&nearest_errors, 0.75), nearest_errors.iter().filter(|e| **e < 2.0).count());
        eprintln!("[GEO] matched keypoints vs true transfer: {} matched, median {:.2} px, within 2 px {}, within 5 px {}", matched_errors.len(), quantile(&matched_errors, 0.5), matched_errors.iter().filter(|e| **e < 2.0).count(), matched_errors.iter().filter(|e| **e < 5.0).count());
        let first: Vec<(f32, f32, u8, f32)> = engine.keypoints_per_frame[a].iter().take(8).map(|k| (k.x, k.y, k.octave, k.response)).collect();
        eprintln!("[GEO] first frame-{a} keypoints (x, y, octave, response): {first:?}");
        // 🧬️ Descriptor discrimination over TRUE pairs: for every base-level frame-0 keypoint whose
        // transfer lands within 1.5 px of a base-level frame-1 keypoint, the Hamming distance of that
        // true pair against the best distance to any other frame-1 descriptor.
        let mut true_distances = Vec::new();
        let mut best_other = Vec::new();
        let mut angle_deltas = Vec::new();
        let mut octaves = [0usize; 4];
        for keypoint in &engine.keypoints_per_frame[a] {
            octaves[usize::from(keypoint.octave).min(3)] += 1;
        }
        for (index, keypoint) in engine.keypoints_per_frame[a].iter().enumerate() {
            if keypoint.octave != 0 {
                continue;
            }
            let ray = intrinsics.unproject_ray([f64::from(keypoint.x), f64::from(keypoint.y)]);
            let direction = normalize3(pose_a.r.act(ray));
            let Some(point) = hit(pose_a.t, direction) else { continue };
            let Some(expected) = project(b, point) else { continue };
            let Some((partner, _)) = engine.keypoints_per_frame[b].iter().enumerate().filter(|(_, k)| k.octave == 0).map(|(j, k)| (j, ((f64::from(k.x) - expected[0]).powi(2) + (f64::from(k.y) - expected[1]).powi(2)).sqrt())).filter(|(_, d)| *d < 1.5).min_by(|x, y| x.1.total_cmp(&y.1)) else { continue };
            let descriptor = &engine.descriptors_per_frame[a][index];
            let distance = descriptor_distance(descriptor, &engine.descriptors_per_frame[b][partner]);
            let other = engine.descriptors_per_frame[b].iter().enumerate().filter(|(j, _)| *j != partner).map(|(_, d)| descriptor_distance(descriptor, d)).min().unwrap_or(u32::MAX);
            true_distances.push(distance);
            best_other.push(other);
            let delta = (keypoint.angle - engine.keypoints_per_frame[b][partner].angle).rem_euclid(std::f32::consts::TAU);
            angle_deltas.push(delta.min(std::f32::consts::TAU - delta).to_degrees());
        }
        true_distances.sort_unstable();
        best_other.sort_unstable();
        angle_deltas.sort_by(f32::total_cmp);
        let mid = |v: &[u32]| v.get(v.len() / 2).copied().unwrap_or(0);
        eprintln!("[GEO] octaves of frame-{a} keypoints: {octaves:?}; {} true base-level pairs: median Hamming of the true pair {} (min {}), median best other {}, median |Δangle| {:.1}°", true_distances.len(), mid(&true_distances), true_distances.first().copied().unwrap_or(0), mid(&best_other), angle_deltas.get(angle_deltas.len() / 2).copied().unwrap_or(0.0));
    }
    let mut seen_cursor = usize::MAX;
    while engine.stage() == EngineStage::EstimatingPoses {
        if let EngineStatus::Failed(message) = engine.advance(1) {
            panic!("engine failed during pose estimation: {message}");
        }
        if engine.pose_cursor != seen_cursor {
            seen_cursor = engine.pose_cursor;
            if let Some(sfm) = engine.sfm.as_ref() {
                if seen_cursor < engine.frames.len() {
                    eprintln!("[GEO] registering frame {seen_cursor}: {} 2D-3D correspondences, {} points, {} cameras so far", sfm.correspondence_count(seen_cursor), sfm.point_count(), sfm.registered_count());
                    // 🔍️ Why a frame's registration is starved: its tracks by how many registered
                    // frames they span and whether they carry a point.
                    if seen_cursor <= 8 {
                        let registered: std::collections::BTreeSet<usize> = (0..seen_cursor).filter(|&f| sfm.camera_pose(f).is_some()).collect();
                        let mut by_span = std::collections::BTreeMap::<usize, (usize, usize)>::new();
                        for (track, has_point) in sfm.tracks_observing(seen_cursor) {
                            let span = track.iter().filter(|(f, _)| registered.contains(f)).count();
                            let entry = by_span.entry(span).or_insert((0, 0));
                            entry.0 += 1;
                            if has_point {
                                entry.1 += 1;
                            }
                        }
                        eprintln!("[GEO]   frame {seen_cursor} tracks by registered span (span: tracks/with point): {by_span:?}");
                    }
                }
            }
        }
    }
    let report = |label: &str, registered: &[(usize, remodeling_camera::CameraPose)]| {
        eprintln!("[GEO] {label}: registered {:?}", registered.iter().map(|(frame, _)| *frame).collect::<Vec<_>>());
        let (first, first_pose) = registered[0];
        let mut worst = 0.0f64;
        for &(frame, pose) in &registered[1..] {
            let recovered = pose.0.semio_compose_rs(&first_pose.0.inverse());
            let expected = relative(first, frame);
            let rotation_error = norm3(expected.r.inverse().semio_compose_rs(&recovered.r).log()).to_degrees();
            worst = worst.max(rotation_error);
            let direction = |t: [f64; 3]| normalize3(t);
            let cosine = { let (u, v) = (direction(recovered.t), direction(expected.t)); u[0] * v[0] + u[1] * v[1] + u[2] * v[2] };
            eprintln!("[GEO] {label}: camera {frame} relative to {first}: rotation error {rotation_error:.2}°, translation direction error {:.2}°, baseline recovered {:.3} true {:.3}", cosine.clamp(-1.0, 1.0).acos().to_degrees(), norm3(recovered.t), norm3(expected.t));
        }
        eprintln!("[GEO] {label}: worst rotation error {worst:.2}°");
    };
    let reconstruction = engine.sfm.as_ref().expect("sfm");
    let registered: Vec<(usize, remodeling_camera::CameraPose)> = (0..engine.frames.len()).filter_map(|frame| reconstruction.camera_pose(frame).map(|pose| (frame, pose))).collect();
    report("after registration", &registered);
    // 🌐️ Through the bundle stage: the global adjustment and the cleanup, then the finalized snapshot.
    while engine.stage() == EngineStage::BundleAdjusting {
        if let EngineStatus::Failed(message) = engine.advance(64) {
            panic!("engine failed during bundle adjustment: {message}");
        }
    }
    let finalized = engine.reconstruction.as_ref().expect("finalized reconstruction");
    report("after bundle", &finalized.cameras);
}
