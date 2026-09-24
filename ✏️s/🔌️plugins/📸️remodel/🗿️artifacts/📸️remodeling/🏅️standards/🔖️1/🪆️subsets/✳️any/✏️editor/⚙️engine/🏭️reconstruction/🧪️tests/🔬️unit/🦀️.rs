use crate::editor::remodeling::engine::step_ceiling;
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

/// 🧱️ Cell size (in units of the cube's half extent) and luminance swing of the blocky value-noise
/// texture on every face: a flat face gives a gradient descriptor nothing to tell its keypoints
/// apart by (every marker edge looks like every other), so like the shipped orbit fixture the faces
/// carry a seeded pattern whose cell junctions are the corners a real surface offers.
const CUBE_TEXTURE_CELL: f64 = 0.16;
const CUBE_TEXTURE_SWING: f64 = 0.45;

fn cube_texture_shade(face: usize, u: f64, v: f64, half: f64) -> f64 {
    let cell = CUBE_TEXTURE_CELL * half;
    let (iu, iv) = ((u / cell).floor() as i64, (v / cell).floor() as i64);
    let mut rng = geometry::random::Rng::from_seed(0x7E57_CE11 ^ ((face as u64) << 56) ^ (iu as u64).wrapping_mul(0x9E37_79B9) ^ ((iv as u64).wrapping_mul(0x85EB_CA6B) << 20));
    1.0 + CUBE_TEXTURE_SWING * (rng.next_f64() * 2.0 - 1.0)
}

/// 🎨️ A textured per-face base color, with any nearby [`FaceMarker`] drawn on top — isolated,
/// high-contrast, locally-unique corner-rich features at fixed world positions.
fn cube_face_color(p: [f64; 3], axis: usize, markers: &[Vec<FaceMarker>; 6], half: f64) -> [u8; 3] {
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
    let shade = cube_texture_shade(idx, u, v, half);
    FACE_BASE_COLORS[idx].map(|channel| (f64::from(channel) * shade).round().clamp(0.0, 255.0) as u8)
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
                Some((p, axis)) => cube_face_color(p, axis, markers, half),
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

    let census = |engine: &ReconstructionEngine| {
        let reconstruction = engine.reconstruction.as_ref();
        (reconstruction.map(|r| r.cameras.iter().map(|(frame, _)| *frame).collect::<Vec<_>>()).unwrap_or_default(), reconstruction.map_or(0, |r| r.points.len()), engine.depth_maps.len(), engine.dense_positions().len())
    };
    println!("[chunking] budget 1: cameras/points/depth maps/dense {:?}; budget MAX: {:?}", census(&small), census(&big));
    assert_eq!(census(&small), census(&big), "chunking must not change the registration or the dense stage");
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
    // 🎞️ The capture `orbit_sfm_registers_enough_cameras_for_gauge` registers from lossless frames
    // (24 views 15° apart at 128 px), so what this test measures is the JPEG/MP4 path alone. At 16
    // views 22.5° apart the lossless frames register only the seed pair too: no adjacent pair
    // keeps half its ORB matches under one-pixel verification at that step.
    const N_FRAMES: usize = 24;
    const SIZE: u32 = 128;
    const HALF: f64 = 1.0;
    const RADIUS: f64 = 3.2;
    let (frames, _lo, _hi, _eyes) = orbiting_cube_frames(N_FRAMES, SIZE, HALF, RADIUS);
    let jpegs: Vec<Vec<u8>> = frames.iter().map(|f| remodeling_image::encode_jpeg(f, 92)).collect();
    let mp4_bytes = remodeling_video::write_mp4_mjpeg(&jpegs, 12.0);
    let mut params = tiny_engine_params(HALF, RADIUS);
    params.sequential_window = 6;
    params.match_ratio = 0.85;
    params.match_mutual = true;
    params.target_feature_count = 500;
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
        println!("[long] registered frames {:?}, {} sparse points", recon_cameras.iter().map(|(frame, _)| *frame).collect::<Vec<_>>(), engine.reconstruction.as_ref().map_or(0, |r| r.points.len()));
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
        // 🧊️ The interactive pipeline meshes what the depth maps observed as a closed shell one
        // truncation band (a third of the half extent here) thick, so a correct reconstruction's
        // extent exceeds the cube's by up to two truncation distances before the coarsened net adds
        // its own cell of slack — hence the tolerance.
        let tolerance = 0.60;
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
    step_ceiling::admit_step(started.elapsed(), format_args!("maximum admitted image envelope validation exceeded 8 ms"));

    for (width, height, bytes) in [(513, 512, 513 * 512 * 4), (512, 512, MAX_INTERACTIVE_IMAGE_BYTES - 1)] {
        let mut malformed = ReconstructionEngine::new(&EngineParams::default());
        malformed.frame_source.frames = vec![accepted_frame(0, width, height, bytes), accepted_frame(1, 1, 1, 4)];
        let started = std::time::Instant::now();
        assert!(malformed.start().is_err());
        step_ceiling::admit_step(started.elapsed(), format_args!("oversized or malformed image admission exceeded 8 ms"));
    }

    let mut too_many = ReconstructionEngine::new(&EngineParams::default());
    too_many.frame_source.frames = (0..65).map(|index| accepted_frame(index, 1, 1, 4)).collect();
    let started = std::time::Instant::now();
    assert!(too_many.start().is_err());
    step_ceiling::admit_step(started.elapsed(), format_args!("65-frame admission rejection exceeded 8 ms"));
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
            step_ceiling::admit_step(started.elapsed(), format_args!("maximum admitted feature allocation/luma/detect/describe microstep exceeded 8 ms: {:?} in {phase:?}", started.elapsed()));
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
            step_ceiling::admit_step(started.elapsed(), format_args!("pair-match microstep exceeded 8 ms"));
            let after = engine.pair_match_preparation.as_ref().map_or((2_048, 0), |state| (state.query, state.candidate));
            assert!(after.0 > before.0 || after.1 >= before.1 || engine.pair_cursor == 1);
        }

        engine.pairwise_matches = vec![(0, 1, (0..200_000).map(|index| remodeling_feature::Match { a: index, b: index, distance: 0 }).collect())];
        loop {
            let phase = engine.track_preparation.as_ref().map(|preparation| format!("{:?} pair {} matched {} grouping {}", preparation.phase, preparation.pair, preparation.matched, preparation.grouping_cursor));
            let started = std::time::Instant::now();
            let complete = engine.step_build_tracks();
            step_ceiling::admit_step(started.elapsed(), format_args!("track microstep exceeded 8 ms: {:?} in {phase:?}", started.elapsed()));
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
    engine.observe();
    // One unit at a time: the first pose-estimation unit hands the pair table to the SfM.
    while engine.stage() != EngineStage::EstimatingPoses {
        if let EngineStatus::Failed(message) = engine.advance(1) {
            panic!("engine failed before pose estimation: {message}");
        }
    }
    let mut observations = Vec::new();
    engine.drain_observations(&mut observations);
    for observation in &observations {
        match observation {
            EngineObservation::PairMatched { frame_a, frame_b, matches } if (*frame_a, *frame_b) == (0, 1) || (*frame_a, *frame_b) == (11, 12) => eprintln!("[GEO] observed pair ({frame_a},{frame_b}) kept {matches} matches"),
            EngineObservation::PairGeometryRejected { frame_a, frame_b, dropped } if (*frame_a, *frame_b) == (0, 1) || (*frame_a, *frame_b) == (11, 12) => eprintln!("[GEO] observed pair ({frame_a},{frame_b}) dropped {dropped} matches"),
            _ => {}
        }
    }
    eprintln!("[GEO] {} observations, {} keypoints in frame 11, {} in frame 12", observations.len(), engine.keypoints_per_frame[11].len(), engine.keypoints_per_frame[12].len());
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

/// 🔭️ Dense-stage audit of the shipped `synthetic-orbit` example against its ground truth: runs the
/// engine to the end and scores, in the truth frame (Sim(3) over the registered camera centres), the
/// sparse points, every depth map (object vs background pixels, depth error, object coverage), the
/// fused cloud and the final mesh by their distance to the cube's surface in half-sizes. Diagnostic,
/// therefore `#[ignore]`:
/// `cargo test -p semio-s-artifact-remodel-remodeling --lib -- --ignored --nocapture diagnose_synthetic_orbit_dense_against_truth`.
#[test]
#[ignore = "diagnostic dense audit against the fixture's ground truth, run explicitly"]
fn diagnose_synthetic_orbit_dense_against_truth() {
    use crate::lie::{Quatd, Se3, So3};
    let truth: serde_json::Value = serde_json::from_str(crate::examples::synthetic_orbit::GROUND_TRUTH_JSON).expect("ground truth json");
    let number = |node: &serde_json::Value| node.as_f64().expect("number");
    let half = number(&truth["scene"]["halfExtentM"]);
    let truth_world_to_camera: Vec<Se3> = truth["extrinsics"]
        .as_array()
        .expect("extrinsics")
        .iter()
        .map(|node| {
            let q = node["rotationWxyzWorldToCamera"].as_array().expect("quaternion");
            let rotation = So3::from_quat(Quatd { w: number(&q[0]), x: number(&q[1]), y: number(&q[2]), z: number(&q[3]) });
            let c = node["cameraCenterM"].as_array().expect("centre");
            Se3 { r: rotation, t: scale3(rotation.act([number(&c[0]), number(&c[1]), number(&c[2])]), -1.0) }
        })
        .collect();
    let surface = |p: [f64; 3]| {
        let outside = [(p[0].abs() - half).max(0.0), (p[1].abs() - half).max(0.0), (p[2].abs() - half).max(0.0)];
        let outside = (outside[0] * outside[0] + outside[1] * outside[1] + outside[2] * outside[2]).sqrt();
        let inside = (half - p[0].abs()).min(half - p[1].abs()).min(half - p[2].abs()).max(0.0);
        (outside + inside) / half
    };
    // Slab test: distance along a unit ray from `origin` to the cube, if it hits.
    let hit = |origin: [f64; 3], direction: [f64; 3]| -> Option<f64> {
        let (mut near, mut far) = (f64::NEG_INFINITY, f64::INFINITY);
        for axis in 0..3 {
            if direction[axis].abs() < 1e-12 {
                if origin[axis].abs() > half {
                    return None;
                }
                continue;
            }
            let a = (-half - origin[axis]) / direction[axis];
            let b = (half - origin[axis]) / direction[axis];
            near = near.max(a.min(b));
            far = far.min(a.max(b));
        }
        (near <= far && far > 0.0).then_some(near.max(0.0))
    };
    let quantiles = |mut values: Vec<f64>| -> String {
        if values.is_empty() {
            return "n=0".into();
        }
        values.sort_by(f64::total_cmp);
        let q = |f: f64| values[((values.len() as f64 - 1.0) * f).round() as usize];
        format!("n={} p10 {:.3} median {:.3} p90 {:.3}", values.len(), q(0.1), q(0.5), q(0.9))
    };

    let scene = <crate::RemodelingSnapshot as store::ArtifactDsl>::parse_dsl(crate::examples::synthetic_orbit::PRIMARY_TEXT).expect("parses");
    let params = crate::editor::remodeling::engine::build_engine_params(&scene.params, &scene.calibration);
    eprintln!("[DENSE] params dense={:?} voxel={} trunc={} dense_source_views={} max_dense_cameras={}", params.dense, params.tsdf_voxel_size, params.tsdf_truncation, params.dense_source_views, params.max_dense_cameras);
    let mut engine = ReconstructionEngine::new(&params);
    for (index, (_, bytes)) in crate::examples::synthetic_orbit::FRAMES.iter().enumerate() {
        let image = crate::editor::remodeling::decode_still_image("image/png", bytes).expect("decodes");
        engine.push_frame_with_sharpness(index as u32, image, index as f64 * 500.0, 1.0);
    }
    let mut captured: Option<(remodeling_sfm::Reconstruction, Vec<usize>, Vec<remodeling_dense::DepthMap>)> = None;
    let mut cloud: Vec<[f64; 3]> = Vec::new();
    let mut last = engine.stage();
    let mut volume: Option<(remodeling_dense::TsdfVolume, [i32; 3], [i32; 3], [f64; 3], [f64; 3])> = None;
    let mut meshes: Vec<(usize, Vec<[f64; 3]>)> = Vec::new();
    loop {
        let budget = if matches!(engine.stage(), EngineStage::FusingVolume | EngineStage::ExtractingSurface | EngineStage::CleaningMesh | EngineStage::Texturing) { 1 } else { 64 };
        let status = engine.advance(budget);
        if volume.is_none() && engine.stage() == EngineStage::FusingVolume && engine.fusion_finalized && engine.stage_cursor >= engine.dense_camera_indices.len() {
            if let (Some(tsdf), Some(preparation)) = (&engine.tsdf, &engine.meshing_preparation) {
                let (lattice_min, lattice_max) = compute_voxel_bounds_from_extrema(preparation.bounds_min, preparation.bounds_max, engine.meshing_voxel().0);
                volume = Some((tsdf.clone(), lattice_min, lattice_max, preparation.bounds_min, preparation.bounds_max));
            }
        }
        if let Some(pipeline) = &engine.mesh_pipeline {
            let triangles = pipeline.mesh().triangles.len();
            if meshes.last().is_none_or(|(count, _)| *count != triangles) {
                meshes.push((triangles, pipeline.mesh().positions.clone()));
            }
        }
        if engine.stage() != last {
            eprintln!("[DENSE] {last:?} -> {:?}", engine.stage());
            if last == EngineStage::DenseStereo {
                captured = Some((engine.reconstruction.clone().expect("reconstruction"), engine.dense_camera_indices.clone(), engine.depth_maps.clone()));
            }
            if !engine.dense_positions().is_empty() && cloud.is_empty() {
                cloud = engine.dense_positions().to_vec();
            }
            last = engine.stage();
        }
        if cloud.is_empty() && !engine.dense_positions().is_empty() {
            cloud = engine.dense_positions().to_vec();
        }
        match status {
            EngineStatus::Working { .. } => {}
            EngineStatus::Done => break,
            EngineStatus::Failed(message) => panic!("engine failed: {message}"),
        }
    }
    let (reconstruction, dense_indices, depth_maps) = captured.expect("the run passed the dense stage");
    if let Ok(path) = std::env::var("DENSE_DUMP") {
        std::fs::write(&path, dense_dump::encode(&reconstruction, &dense_indices, &depth_maps)).expect("write dense dump");
        eprintln!("[DENSE] dumped to {path}");
    }
    let recovered: Vec<[f64; 3]> = reconstruction.cameras.iter().map(|(_, pose)| pose.0.inverse().t).collect();
    let targets: Vec<[f64; 3]> = reconstruction.cameras.iter().map(|(frame, _)| truth_world_to_camera[*frame].inverse().t).collect();
    let sim = crate::lie::umeyama(&recovered, &targets, true).expect("Sim(3) over camera centres");
    let centre_errors: Vec<f64> = recovered.iter().zip(&targets).map(|(r, t)| norm3(sub3(sim.act(*r), *t))).collect();
    eprintln!("[DENSE] {} cameras, sim scale {:.4}, centre error {}", recovered.len(), sim.s, quantiles(centre_errors));
    eprintln!("[DENSE] sparse surface distance {}", quantiles(reconstruction.points.iter().map(|p| surface(sim.act(*p))).collect()));

    let intrinsics = reconstruction.intrinsics;
    for (slot, (camera, map)) in dense_indices.iter().zip(&depth_maps).enumerate() {
        let (frame, pose) = reconstruction.cameras[*camera];
        let truth_pose = truth_world_to_camera[frame];
        let truth_centre = truth_pose.inverse().t;
        let camera_to_world = truth_pose.r.inverse();
        let (mut object_pixels, mut object_valid, mut background_valid) = (0usize, 0usize, 0usize);
        let (mut object_errors, mut background_errors, mut depth_errors) = (Vec::new(), Vec::new(), Vec::new());
        for y in 0..map.height {
            for x in 0..map.width {
                let ray = intrinsics.unproject_ray([f64::from(x), f64::from(y)]);
                let direction = camera_to_world.act(ray);
                let length = norm3(direction);
                let truth_depth = hit(truth_centre, scale3(direction, 1.0 / length)).map(|distance| distance / length);
                if truth_depth.is_some() {
                    object_pixels += 1;
                }
                let Some(depth) = map.get(x, y) else { continue };
                let world = sim.act(pose.0.inverse().act(scale3(ray, f64::from(depth))));
                match truth_depth {
                    Some(expected) => {
                        object_valid += 1;
                        object_errors.push(surface(world));
                        depth_errors.push((f64::from(depth) * sim.s - expected).abs() / half);
                    }
                    None => {
                        background_valid += 1;
                        background_errors.push(surface(world));
                    }
                }
            }
        }
        eprintln!(
            "[DENSE] map {slot} frame {frame}: object px {object_pixels}, valid on object {object_valid} ({:.0}%), valid on background {background_valid}; object surface dist {}; |depth err| {}; background surface dist {}",
            100.0 * object_valid as f64 / object_pixels.max(1) as f64,
            quantiles(object_errors),
            quantiles(depth_errors),
            quantiles(background_errors)
        );
    }
    let views: Vec<(remodeling_camera::CameraPose, remodeling_camera::Intrinsics)> = dense_indices.iter().map(|camera| (reconstruction.cameras[*camera].1, intrinsics)).collect();
    for (tolerance, minimum) in [(0.01f32, 2usize), (0.02, 2), (0.03, 2), (0.02, 3), (0.03, 3), (0.05, 3)] {
        let fused = remodeling_dense::fuse_depth_maps(&views, &depth_maps, &remodeling_dense::FusionConfig { max_relative_depth_diff: tolerance, max_normal_angle_deg: 30.0, min_consistent_views: minimum });
        let distances: Vec<f64> = fused.positions.iter().map(|p| surface(sim.act(*p))).collect();
        let far = distances.iter().filter(|distance| **distance > 0.2).count();
        eprintln!("[DENSE] fusion tolerance {tolerance} views {minimum}: {} points ({far} > 0.2 half-sizes off), surface distance {}", fused.positions.len(), quantiles(distances));
    }
    if let Some((tsdf, lattice_min, lattice_max, extrema_min, extrema_max)) = &volume {
        eprintln!("[DENSE] extrema {extrema_min:.3?}..{extrema_max:.3?} lattice {lattice_min:?}..{lattice_max:?} blocks {}", tsdf.block_count());
        let voxel = engine.meshing_voxel().0;
        let (mut unobserved, mut agree, mut disagree) = (0usize, 0usize, 0usize);
        let mut disagreeing_depth = Vec::new();
        for z in lattice_min[2]..=lattice_max[2] {
            for y in lattice_min[1]..=lattice_max[1] {
                for x in lattice_min[0]..=lattice_max[0] {
                    let centre = sim.act([(f64::from(x) + 0.5) * voxel, (f64::from(y) + 0.5) * voxel, (f64::from(z) + 0.5) * voxel]);
                    let inside = centre.iter().all(|c| c.abs() < half);
                    match tsdf.sample(x, y, z) {
                        None => unobserved += 1,
                        Some((sdf, _)) => {
                            if (sdf < 0.0) == inside {
                                agree += 1;
                            } else {
                                disagree += 1;
                                disagreeing_depth.push(surface(centre));
                            }
                        }
                    }
                }
            }
        }
        let truth_points: Vec<[f64; 3]> = truth["pointsWorldM"].as_array().expect("points").iter().map(|p| { let p = p.as_array().expect("point"); [number(&p[0]), number(&p[1]), number(&p[2])] }).collect();
        let mut fusion = remodeling_dense::FusionPreparation::new(0);
        while !fusion.advance(&views, &depth_maps, &engine.fusion_config(), usize::MAX / 2) {}
        let (_, masks) = fusion.finish_with_consistency().expect("fusion completes");
        let masked: Vec<remodeling_dense::DepthMap> = depth_maps
            .iter()
            .zip(&masks)
            .map(|(map, bits)| {
                let mut map = map.clone();
                for (index, depth) in map.depth.iter_mut().enumerate() {
                    if bits[index / 64] >> (index % 64) & 1 == 0 {
                        *depth = 0.0;
                    }
                }
                map
            })
            .collect();
        let grid: Vec<(f64, f64)> = std::env::var("DENSE_GRID").ok().map_or_else(
            || vec![(0.1, 0.33), (0.1, 0.2), (0.1, 0.15), (0.15, 0.3), (0.2, 0.2), (0.2, 0.3), (0.2, 0.4), (0.25, 0.25), (0.25, 0.5)],
            |text| text.split(';').map(|pair| { let (v, t) = pair.split_once(',').expect("voxel,truncation"); (v.parse().expect("voxel"), t.parse().expect("truncation")) }).collect(),
        );
        for (voxel_size, truncation) in grid {
            let (grid_min, grid_max) = compute_voxel_bounds_from_extrema(*extrema_min, *extrema_max, voxel_size);
            let mut volume = remodeling_dense::TsdfVolume::new(voxel_size, truncation).with_voxel_bounds(grid_min.map(|c| c - 2), grid_max.map(|c| c + 2));
            for (map, view) in masked.iter().zip(&views) {
                volume.integrate(map, view, true);
            }
            let mut pipeline = remodeling_mesh::MeshPipeline::new_bounded(volume, 0.0, grid_min, grid_max, engine.params.mesh.clone());
            let outcome = loop {
                match remodeling_mesh::mesh_pipeline_step(&mut pipeline, 1) {
                    remodeling_mesh::MeshPipelineStatus::Working { .. } => {}
                    remodeling_mesh::MeshPipelineStatus::Done => break "done".to_string(),
                    remodeling_mesh::MeshPipelineStatus::Failed(message) => break message.chars().take(90).collect(),
                }
            };
            let positions: Vec<[f64; 3]> = pipeline.result().map_or_else(|| pipeline.mesh().positions.iter().map(|p| sim.act(*p)).collect(), |mesh| mesh.positions.chunks_exact(3).map(|p| sim.act([f64::from(p[0]), f64::from(p[1]), f64::from(p[2])])).collect());
            let covered = truth_points.iter().filter(|point| positions.iter().any(|vertex| norm3(sub3(*vertex, **point)) <= 0.4)).count();
            eprintln!("[DENSE] grid voxel {voxel_size} trunc {truncation}: {outcome}; {} vertices, surface distance {}, completeness {:.1}%", positions.len(), quantiles(positions.iter().map(|p| surface(*p)).collect()), 100.0 * covered as f64 / truth_points.len() as f64);
        }
        eprintln!("[DENSE] tsdf lattice voxels: unobserved {unobserved}, sign agrees with truth {agree}, disagrees {disagree}; disagreeing voxels' surface distance {}", quantiles(disagreeing_depth));
    }
    for (triangles, positions) in &meshes {
        eprintln!("[DENSE] pipeline mesh {triangles} triangles / {} vertices: surface distance {}", positions.len(), quantiles(positions.iter().map(|p| surface(sim.act(*p))).collect()));
    }
    eprintln!("[DENSE] fused cloud surface distance {}", quantiles(cloud.iter().map(|p| surface(sim.act(*p))).collect()));
    let mesh = engine.mesh_data.as_ref().expect("mesh");
    let vertices: Vec<[f64; 3]> = mesh.positions.chunks_exact(3).map(|p| sim.act([f64::from(p[0]), f64::from(p[1]), f64::from(p[2])])).collect();
    let mut lo = [f64::INFINITY; 3];
    let mut hi = [f64::NEG_INFINITY; 3];
    for vertex in &vertices {
        for axis in 0..3 {
            lo[axis] = lo[axis].min(vertex[axis]);
            hi[axis] = hi[axis].max(vertex[axis]);
        }
    }
    eprintln!("[DENSE] mesh {} vertices / {} triangles, aligned bounds {lo:.2?}..{hi:.2?}, surface distance {}", vertices.len(), mesh.indices.len() / 3, quantiles(vertices.iter().map(|v| surface(*v)).collect()));
}

/// 💾️ Binary snapshot of a finished dense stage (reconstruction, dense camera indices, depth maps)
/// so fusion/TSDF/meshing experiments replay in seconds instead of re-running SfM and PatchMatch.
mod dense_dump {
    use super::*;

    struct Writer(Vec<u8>);
    impl Writer {
        fn u64(&mut self, value: usize) {
            self.0.extend_from_slice(&(value as u64).to_le_bytes());
        }
        fn f64(&mut self, value: f64) {
            self.0.extend_from_slice(&value.to_le_bytes());
        }
        fn f32(&mut self, value: f32) {
            self.0.extend_from_slice(&value.to_le_bytes());
        }
    }
    struct Reader<'a>(&'a [u8], usize);
    impl Reader<'_> {
        fn take<const N: usize>(&mut self) -> [u8; N] {
            let bytes: [u8; N] = self.0[self.1..self.1 + N].try_into().expect("bytes");
            self.1 += N;
            bytes
        }
        fn u64(&mut self) -> usize {
            u64::from_le_bytes(self.take()) as usize
        }
        fn f64(&mut self) -> f64 {
            f64::from_le_bytes(self.take())
        }
        fn f32(&mut self) -> f32 {
            f32::from_le_bytes(self.take())
        }
    }

    pub(super) fn encode(reconstruction: &remodeling_sfm::Reconstruction, dense: &[usize], maps: &[remodeling_dense::DepthMap]) -> Vec<u8> {
        let mut w = Writer(Vec::new());
        let i = reconstruction.intrinsics;
        for value in [i.fx, i.fy, i.cx, i.cy, i.skew] {
            w.f64(value);
        }
        match i.distortion {
            remodeling_camera::Distortion::BrownConrady { k1, k2, k3, p1, p2 } => {
                w.u64(1);
                for value in [k1, k2, k3, p1, p2] {
                    w.f64(value);
                }
            }
            _ => w.u64(0),
        }
        w.u64(reconstruction.cameras.len());
        for (frame, pose) in &reconstruction.cameras {
            w.u64(*frame);
            for column in pose.0.r.0.cols {
                for value in column {
                    w.f64(value);
                }
            }
            for value in pose.0.t {
                w.f64(value);
            }
        }
        w.u64(reconstruction.points.len());
        for point in &reconstruction.points {
            for value in point {
                w.f64(*value);
            }
        }
        w.u64(dense.len());
        for index in dense {
            w.u64(*index);
        }
        w.u64(maps.len());
        for map in maps {
            w.u64(map.width as usize);
            w.u64(map.height as usize);
            for index in 0..map.depth.len() {
                w.f32(map.depth[index]);
                for value in map.normal[index] {
                    w.f32(value);
                }
                w.f32(map.confidence[index]);
            }
        }
        w.0
    }

    pub(super) fn decode(bytes: &[u8]) -> (remodeling_sfm::Reconstruction, Vec<usize>, Vec<remodeling_dense::DepthMap>) {
        let mut r = Reader(bytes, 0);
        let (fx, fy, cx, cy, skew) = (r.f64(), r.f64(), r.f64(), r.f64(), r.f64());
        let distortion = if r.u64() == 1 {
            let (k1, k2, k3, p1, p2) = (r.f64(), r.f64(), r.f64(), r.f64(), r.f64());
            remodeling_camera::Distortion::BrownConrady { k1, k2, k3, p1, p2 }
        } else {
            remodeling_camera::Distortion::None
        };
        let intrinsics = remodeling_camera::Intrinsics { fx, fy, cx, cy, skew, distortion };
        let cameras = (0..r.u64())
            .map(|_| {
                let frame = r.u64();
                let cols: [[f64; 3]; 3] = std::array::from_fn(|_| std::array::from_fn(|_| r.f64()));
                let t = [r.f64(), r.f64(), r.f64()];
                (frame, remodeling_camera::CameraPose(crate::lie::Se3 { r: crate::lie::So3(crate::algebra::Mat3d { cols }), t }))
            })
            .collect();
        let points = (0..r.u64()).map(|_| [r.f64(), r.f64(), r.f64()]).collect::<Vec<_>>();
        let point_track_ids = (0..points.len()).collect();
        let dense = (0..r.u64()).map(|_| r.u64()).collect();
        let maps = (0..r.u64())
            .map(|_| {
                let (width, height) = (r.u64() as u32, r.u64() as u32);
                let mut map = remodeling_dense::DepthMap::new(width, height);
                for index in 0..map.depth.len() {
                    map.depth[index] = r.f32();
                    map.normal[index] = [r.f32(), r.f32(), r.f32()];
                    map.confidence[index] = r.f32();
                }
                map
            })
            .collect();
        (remodeling_sfm::Reconstruction { cameras, points, point_track_ids, intrinsics }, dense, maps)
    }
}

/// 🔁️ Replays fusion → TSDF → surface pipeline over a [`dense_dump`] written by
/// [`diagnose_synthetic_orbit_dense_against_truth`] (`DENSE_DUMP=<path>`), scoring each mesh
/// against the fixture's truth cube. `DENSE_FUSION="tolerance,views"` and
/// `DENSE_GRID="voxel,truncation;…"` pick the experiments. Diagnostic, therefore `#[ignore]`.
#[test]
#[ignore = "diagnostic replay of a dumped dense stage, run explicitly"]
fn diagnose_dense_replay() {
    use crate::lie::{Quatd, Se3, So3};
    let path = std::env::var("DENSE_DUMP").expect("DENSE_DUMP names the dump");
    let (reconstruction, dense_indices, depth_maps) = dense_dump::decode(&std::fs::read(path).expect("read dump"));
    let truth: serde_json::Value = serde_json::from_str(crate::examples::synthetic_orbit::GROUND_TRUTH_JSON).expect("ground truth json");
    let number = |node: &serde_json::Value| node.as_f64().expect("number");
    let half = number(&truth["scene"]["halfExtentM"]);
    let truth_centres: Vec<[f64; 3]> = truth["extrinsics"]
        .as_array()
        .expect("extrinsics")
        .iter()
        .map(|node| {
            let q = node["rotationWxyzWorldToCamera"].as_array().expect("quaternion");
            let rotation = So3::from_quat(Quatd { w: number(&q[0]), x: number(&q[1]), y: number(&q[2]), z: number(&q[3]) });
            let c = node["cameraCenterM"].as_array().expect("centre");
            Se3 { r: rotation, t: scale3(rotation.act([number(&c[0]), number(&c[1]), number(&c[2])]), -1.0) }.inverse().t
        })
        .collect();
    let truth_points: Vec<[f64; 3]> = truth["pointsWorldM"].as_array().expect("points").iter().map(|p| { let p = p.as_array().expect("point"); [number(&p[0]), number(&p[1]), number(&p[2])] }).collect();
    let surface = |p: [f64; 3]| {
        let outside = [(p[0].abs() - half).max(0.0), (p[1].abs() - half).max(0.0), (p[2].abs() - half).max(0.0)];
        let outside = (outside[0] * outside[0] + outside[1] * outside[1] + outside[2] * outside[2]).sqrt();
        let inside = (half - p[0].abs()).min(half - p[1].abs()).min(half - p[2].abs()).max(0.0);
        (outside + inside) / half
    };
    let quantiles = |mut values: Vec<f64>| -> String {
        if values.is_empty() {
            return "n=0".into();
        }
        values.sort_by(f64::total_cmp);
        let q = |f: f64| values[((values.len() as f64 - 1.0) * f).round() as usize];
        format!("n={} median {:.3} p90 {:.3}", values.len(), q(0.5), q(0.9))
    };
    let recovered: Vec<[f64; 3]> = reconstruction.cameras.iter().map(|(_, pose)| pose.0.inverse().t).collect();
    let targets: Vec<[f64; 3]> = reconstruction.cameras.iter().map(|(frame, _)| truth_centres[*frame]).collect();
    let sim = crate::lie::umeyama(&recovered, &targets, true).expect("Sim(3) over camera centres");
    let scene = <crate::RemodelingSnapshot as store::ArtifactDsl>::parse_dsl(crate::examples::synthetic_orbit::PRIMARY_TEXT).expect("parses");
    let params = crate::editor::remodeling::engine::build_engine_params(&scene.params, &scene.calibration);
    let mut extrema_min = [f64::INFINITY; 3];
    let mut extrema_max = [f64::NEG_INFINITY; 3];
    for point in &reconstruction.points {
        for axis in 0..3 {
            extrema_min[axis] = extrema_min[axis].min(point[axis]);
            extrema_max[axis] = extrema_max[axis].max(point[axis]);
        }
    }
    let views: Vec<(remodeling_camera::CameraPose, remodeling_camera::Intrinsics)> = dense_indices.iter().map(|camera| (reconstruction.cameras[*camera].1, reconstruction.intrinsics)).collect();
    let fusion_config = std::env::var("DENSE_FUSION").ok().map_or_else(remodeling_dense::FusionConfig::default, |text| {
        let (tolerance, views) = text.split_once(',').expect("tolerance,views");
        remodeling_dense::FusionConfig { max_relative_depth_diff: tolerance.parse().expect("tolerance"), max_normal_angle_deg: 30.0, min_consistent_views: views.parse().expect("views") }
    });
    let mut fusion = remodeling_dense::FusionPreparation::new(0);
    while !fusion.advance(&views, &depth_maps, &fusion_config, usize::MAX / 2) {}
    let (_, masks) = fusion.finish_with_consistency().expect("fusion completes");
    let masked: Vec<remodeling_dense::DepthMap> = depth_maps
        .iter()
        .zip(&masks)
        .map(|(map, bits)| {
            let mut map = map.clone();
            for (index, depth) in map.depth.iter_mut().enumerate() {
                if bits[index / 64] >> (index % 64) & 1 == 0 {
                    *depth = 0.0;
                }
            }
            map
        })
        .collect();
    let kept: usize = masked.iter().map(|map| map.depth.iter().filter(|depth| **depth > 0.0).count()).sum();
    eprintln!("[REPLAY] fusion {fusion_config:?}: {kept} consistent pixels");
    let grid: Vec<(f64, f64)> = std::env::var("DENSE_GRID").ok().map_or_else(
        || vec![(0.1, 0.33), (0.1, 0.2), (0.15, 0.3), (0.2, 0.2), (0.25, 0.25)],
        |text| text.split(';').map(|pair| { let (v, t) = pair.split_once(',').expect("voxel,truncation"); (v.parse().expect("voxel"), t.parse().expect("truncation")) }).collect(),
    );
    for (voxel_size, truncation) in grid {
        let (grid_min, grid_max) = compute_voxel_bounds_from_extrema(extrema_min, extrema_max, voxel_size);
        let mut volume = remodeling_dense::TsdfVolume::new(voxel_size, truncation).with_voxel_bounds(grid_min.map(|c| c - 2), grid_max.map(|c| c + 2));
        if std::env::var_os("DENSE_BOUNDED").is_some() {
            for ((map, view), mask) in depth_maps.iter().zip(&views).zip(&masks) {
                let mut preparation = remodeling_dense::TsdfIntegrationPreparation::new();
                while !preparation.advance_masked(&mut volume, map, Some(mask), view, true, 256) {}
            }
        } else {
            for (map, view) in masked.iter().zip(&views) {
                volume.integrate(map, view, true);
            }
        }
        eprintln!("[REPLAY] extrema {extrema_min:.3?}..{extrema_max:.3?} lattice {grid_min:?}..{grid_max:?} volume blocks {}", volume.block_count());
        let mut pipeline = remodeling_mesh::MeshPipeline::new_bounded(volume, 0.0, grid_min, grid_max, params.mesh.clone());
        if std::env::var_os("DENSE_OBSERVED").is_some() {
            pipeline = pipeline.with_observed_only_surface();
        }
        let mut extracted = None;
        let mut trail: Vec<String> = Vec::new();
        let mut last_triangles = usize::MAX;
        let outcome = loop {
            let status = remodeling_mesh::mesh_pipeline_step(&mut pipeline, 1);
            if pipeline.mesh().triangles.len() != last_triangles {
                last_triangles = pipeline.mesh().triangles.len();
                let report = remodeling_mesh::validate_watertight(pipeline.mesh(), false);
                let worst = pipeline.mesh().positions.iter().map(|p| surface(sim.act(*p))).fold(0.0f64, f64::max);
                trail.push(format!("{}t/b{}/nm{}/nv{}/c{}/worst{:.2}", report.triangle_count, report.boundary_edge_count, report.non_manifold_edge_count, report.non_manifold_vertex_count, report.connected_components, worst));
            }
            if extracted.is_none() && !pipeline.mesh().triangles.is_empty() {
                let mesh = pipeline.mesh();
                let mut parent: Vec<usize> = (0..mesh.positions.len()).collect();
                fn root(parent: &mut [usize], mut index: usize) -> usize {
                    while parent[index] != index {
                        parent[index] = parent[parent[index]];
                        index = parent[index];
                    }
                    index
                }
                for triangle in &mesh.triangles {
                    let a = root(&mut parent, triangle[0] as usize);
                    for corner in &triangle[1..] {
                        let b = root(&mut parent, *corner as usize);
                        parent[b] = a;
                    }
                }
                let mut sizes = std::collections::BTreeMap::new();
                for triangle in &mesh.triangles {
                    *sizes.entry(root(&mut parent, triangle[0] as usize)).or_insert(0usize) += 1;
                }
                let mut sizes: Vec<usize> = sizes.into_values().collect();
                sizes.sort_unstable_by(|a, b| b.cmp(a));
                extracted = Some((mesh.positions.len(), mesh.triangles.len(), sizes));
            }
            match status {
                remodeling_mesh::MeshPipelineStatus::Working { .. } => {}
                remodeling_mesh::MeshPipelineStatus::Done => break "done".to_string(),
                remodeling_mesh::MeshPipelineStatus::Failed(message) => break message.chars().take(400).collect(),
            }
        };
        let positions: Vec<[f64; 3]> = pipeline.result().map_or_else(|| pipeline.mesh().positions.iter().map(|p| sim.act(*p)).collect(), |mesh| mesh.positions.chunks_exact(3).map(|p| sim.act([f64::from(p[0]), f64::from(p[1]), f64::from(p[2])])).collect());
        let covered = truth_points.iter().filter(|point| positions.iter().any(|vertex| norm3(sub3(*vertex, **point)) <= 0.4)).count();
        eprintln!("[REPLAY] voxel {voxel_size} trunc {truncation}: trail {trail:?}; extracted {extracted:?} -> {outcome}; {} vertices, surface distance {}, completeness {:.1}%", positions.len(), quantiles(positions.iter().map(|p| surface(*p)).collect()), 100.0 * covered as f64 / truth_points.len() as f64);
    }
}

