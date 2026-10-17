//! 🧊️ Synthetic scene of the staged reconstruction engine tests: the vector helpers, the textured cube renderer, the orbiting frames and the tiny engine parameters. See [`super`].

use super::*;

// #region 🔖️SyntheticScene
pub(super) fn add3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

pub(super) fn sub3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

pub(super) fn scale3(a: [f64; 3], s: f64) -> [f64; 3] {
    [a[0] * s, a[1] * s, a[2] * s]
}

fn cross3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

pub(super) fn norm3(a: [f64; 3]) -> f64 {
    (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).sqrt()
}

pub(super) fn normalize3(a: [f64; 3]) -> [f64; 3] {
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
    let rotation = semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::Mat3d::from_axes(right, up, forward).transpose();
    let translation = scale3(rotation.mul_vec3(eye), -1.0);
    remodeling_camera::CameraPose(semio_s_artifact_remodel_remodeling_lie::standards::v1::subsets::any::schema::lie_internals::Se3 { r: semio_s_artifact_remodel_remodeling_lie::standards::v1::subsets::any::schema::lie_internals::So3(rotation), t: translation })
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
    pub(super) u: f64,
    pub(super) v: f64,
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

pub(super) fn orbiting_cube_frames(n: usize, size: u32, half: f64, radius: f64) -> (Vec<remodeling_image::ImageRgba8>, [f64; 3], [f64; 3], Vec<[f64; 3]>) {
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
pub(super) fn tiny_engine_params(half: f64, radius: f64) -> EngineParams {
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

pub(super) fn run_to_done(engine: &mut ReconstructionEngine, budget: usize) -> semio_framework::MeshData {
    loop {
        match engine.advance(budget) {
            EngineStatus::Working { .. } => {}
            EngineStatus::Done => break,
            EngineStatus::Failed(msg) => panic!("engine unexpectedly failed: {msg}"),
        }
    }
    engine.take_mesh().expect("Done status must yield a mesh")
}
// #endregion 🔖️ChunkingInvariance
