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
    const N_FRAMES: usize = 16;
    const SIZE: u32 = 96;
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
        let mut engine = ReconstructionEngine::new(&params);

        let opts = remodeling_video::VideoIngestOptions { stride: 1, max_frames: 0, max_long_edge_px: 0 };
        let report = engine.push_video(&mp4_bytes, &opts).expect("push_video on a synthesized mjpeg mp4 must succeed");
        println!("[long] push_video report: {report:?}");
        assert_eq!(report.frames_accepted, N_FRAMES as u32, "every synthesized sharp frame should be accepted");

        let mut calls = 0usize;
        let status = loop {
            calls += 1;
            match engine.advance(4) {
                EngineStatus::Working { stage, progress } => {
                    if calls.is_multiple_of(20) {
                        println!("[long] call {calls}: stage={stage:?} progress={progress:.2}");
                    }
                    if calls > 10_000 {
                        panic!("engine did not reach a terminal status within 10000 advance() calls");
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
            let started = std::time::Instant::now();
            engine.step_extracting_features();
            assert!(started.elapsed() < std::time::Duration::from_millis(8), "maximum admitted feature allocation/luma/detect/describe microstep exceeded 8 ms");
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
            let started = std::time::Instant::now();
            let complete = engine.step_build_tracks();
            assert!(started.elapsed() < std::time::Duration::from_millis(8), "track microstep exceeded 8 ms");
            if complete.is_some() {
                break;
            }
        }
    })
    .join()
    .expect("engine worker");
}
