//! 🛰️ Synthetic fixtures and pose priors of the incremental structure from motion: the seeded synthetic scene with its observations and textured renderer, and the similarity alignment and ground-control residual of the pose priors. See [`super`].

use super::*;

// #region 🔖️Fixtures
pub(super) fn norm3(a: [f64; 3]) -> f64 {
    dot3(a, a).sqrt()
}

pub(super) fn normalize3(a: [f64; 3]) -> [f64; 3] {
    let n = norm3(a);
    if n < 1e-300 {
        a
    } else {
        scale3(a, 1.0 / n)
    }
}

pub(super) fn sub3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

/// 🎥️ Builds a world-to-camera [`Se3`] whose optical axis looks from `eye` toward `target`, with `up`
/// as the approximate world-up direction (Gram-Schmidt'd against the look direction).
fn look_at_pose(eye: [f64; 3], target: [f64; 3], up: [f64; 3]) -> Se3 {
    let forward = normalize3(sub3(target, eye));
    let right = normalize3(cross3(up, forward));
    let true_up = cross3(forward, right);
    let r_cw = Mat3d::from_axes(right, true_up, forward);
    let r_wc = So3(r_cw).inverse();
    let t = scale3(r_wc.act(eye), -1.0);
    Se3 { r: r_wc, t }
}

/// 📸️ A deterministic synthetic multi-camera, multi-point scene: cameras on an orbit around a point
/// cloud (or a planar patch, for exercising the two-view solvers' degeneracy behavior), everything
/// generated from a single seed so downstream tests are fully reproducible without file fixtures.
#[derive(Clone, Debug)]
pub struct SyntheticScene {
    pub cameras: Vec<(Intrinsics, CameraPose)>,
    pub points_world: Vec<[f64; 3]>,
    pub image_width: u32,
    pub image_height: u32,
}

/// 📷️ One projected (and possibly noised/outlier-corrupted) observation of a scene point by a camera.
#[derive(Clone, Copy, Debug)]
pub struct Observation {
    pub camera_index: usize,
    pub point_index: usize,
    pub pixel: [f64; 2],
}

/// 📸️ Builds a deterministic synthetic scene: `camera_count` cameras orbiting the origin at a fixed
/// radius (small seeded jitter in elevation/look-at target), and `point_count` scene points either
/// scattered through a cube (`planar = false`) or confined to the `y = 0` ground plane (`planar =
/// true` — the exact degeneracy [`EssentialFivePoint`] is meant to survive and the 8-point/fundamental
/// solver struggles with).
pub fn synthetic_scene(seed: u64, camera_count: usize, point_count: usize, planar: bool) -> SyntheticScene {
    let mut rng = Rng::from_seed(seed);
    let (width, height) = (640u32, 480u32);
    let intrinsics = Intrinsics { fx: 700.0, fy: 700.0, cx: f64::from(width) / 2.0, cy: f64::from(height) / 2.0, skew: 0.0, distortion: Distortion::None };
    let orbit_radius = 6.0;
    let mut cameras = Vec::with_capacity(camera_count.max(1));
    for i in 0..camera_count {
        let base_angle = if camera_count > 0 { std::f64::consts::TAU * (i as f64) / (camera_count as f64) } else { 0.0 };
        let jitter_angle = (rng.next_f64() - 0.5) * 0.15;
        let elevation = (rng.next_f64() - 0.5) * 0.6;
        let angle = base_angle + jitter_angle;
        let eye = [orbit_radius * angle.cos() * elevation.cos(), orbit_radius * elevation.sin(), orbit_radius * angle.sin() * elevation.cos()];
        let target = [(rng.next_f64() - 0.5) * 0.2, (rng.next_f64() - 0.5) * 0.2, (rng.next_f64() - 0.5) * 0.2];
        let pose = CameraPose(look_at_pose(eye, target, [0.0, 1.0, 0.0]));
        cameras.push((intrinsics, pose));
    }
    let mut points_world = Vec::with_capacity(point_count);
    for _ in 0..point_count {
        if planar {
            points_world.push([(rng.next_f64() - 0.5) * 3.0, 0.0, (rng.next_f64() - 0.5) * 3.0]);
        } else {
            points_world.push([(rng.next_f64() - 0.5) * 3.0, (rng.next_f64() - 0.5) * 3.0, (rng.next_f64() - 0.5) * 3.0]);
        }
    }
    SyntheticScene { cameras, points_world, image_width: width, image_height: height }
}

/// 🎯️ Projects every scene point through every camera, keeping only observations that land in front of
/// the camera and inside the image bounds; adds i.i.d. Gaussian pixel noise (`pixel_noise_std`) and
/// replaces `outlier_fraction` of the surviving observations with a uniform-random pixel (for exercising
/// RANSAC/bundle-adjustment robustness).
pub fn project_observations(scene: &SyntheticScene, pixel_noise_std: f64, outlier_fraction: f64, seed: u64) -> Vec<Observation> {
    let mut rng = Rng::from_seed(seed);
    let mut out = Vec::new();
    for (camera_index, (intr, pose)) in scene.cameras.iter().enumerate() {
        for (point_index, &point) in scene.points_world.iter().enumerate() {
            let Some(mut pixel) = reproject(intr, pose, point) else { continue };
            if pixel[0] < 0.0 || pixel[1] < 0.0 || pixel[0] >= f64::from(scene.image_width) || pixel[1] >= f64::from(scene.image_height) {
                continue;
            }
            if rng.next_bool(outlier_fraction) {
                pixel = [rng.next_f64() * f64::from(scene.image_width), rng.next_f64() * f64::from(scene.image_height)];
            } else if pixel_noise_std > 0.0 {
                pixel = [pixel[0] + normal(&mut rng, 0.0, pixel_noise_std), pixel[1] + normal(&mut rng, 0.0, pixel_noise_std)];
            }
            out.push(Observation { camera_index, point_index, pixel });
        }
    }
    out
}

/// 🖌️ Flat-shaded rasterizer: for every camera, splats a small fixed pseudo-random patch (keyed by point
/// index, so the same point looks visually consistent across views) at each in-bounds projected point
/// location — not a physically-based renderer, just enough real pixel content for downstream
/// feature-detection tests to have something to detect and match. Required explicitly by name in this
/// crate's task brief (base-plan `remodeling/sfm` 🔖️Fixtures bullet: "`render_textured_scene(..) ->
/// Vec<remodeling_image::ImageGray>`") — please keep this function if you're reconciling concurrent edits
/// to this region rather than re-removing it as out of scope; it has been dropped and re-added twice
/// already during development.
pub fn render_textured_scene(scene: &SyntheticScene) -> Vec<ImageGray> {
    let patch_radius: i32 = 3;
    let mut images = Vec::with_capacity(scene.cameras.len());
    for (intr, pose) in &scene.cameras {
        let mut img = ImageGray::new(scene.image_width, scene.image_height);
        for y in 0..scene.image_height {
            for x in 0..scene.image_width {
                img.set(x, y, 0.5);
            }
        }
        for (point_index, &point) in scene.points_world.iter().enumerate() {
            let Some(pixel) = reproject(intr, pose, point) else { continue };
            if pixel[0] < 0.0 || pixel[1] < 0.0 || pixel[0] >= f64::from(scene.image_width) || pixel[1] >= f64::from(scene.image_height) {
                continue;
            }
            let mut patch_rng = Rng::from_seed(point_index as u64 ^ 0x9E37_79B9_7F4A_7C15);
            let (cx, cy) = (pixel[0].round() as i32, pixel[1].round() as i32);
            for dy in -patch_radius..=patch_radius {
                for dx in -patch_radius..=patch_radius {
                    let (px, py) = (cx + dx, cy + dy);
                    if px < 0 || py < 0 || px >= scene.image_width as i32 || py >= scene.image_height as i32 {
                        continue;
                    }
                    let value = (0.2 + 0.6 * patch_rng.next_f64()).clamp(0.0, 1.0) as f32;
                    img.set(px as u32, py as u32, value);
                }
            }
        }
        images.push(img);
    }
    images
}
// #endregion 🔖️Fixtures

// #region 🔖️Priors
/// 📌️ An absolute-reference constraint pinning the otherwise gauge-free (and, for monocular pipelines,
/// scale-free) reconstruction to real-world measurements. `GpsPosition`/`GravityUp` pin a camera frame's
/// pose; `Gcp` (ground control point) pins a triangulated point's world position. The natural extension
/// point for injecting these into bundle adjustment is as extra `a_index: None`/`b_index: None`-paired
/// [`crate::standards::v1::subsets::any::schema::optimize_internals::ResidualTerm`]s inside a [`crate::standards::v1::subsets::any::schema::optimize_internals::BipartiteResiduals`]
/// problem — a `GpsPosition`/`GravityUp` prior becomes an A-only term (touching just that camera's
/// 6-dof block), a `Gcp` prior a B-only term (touching just that point's 3-dof block, see
/// [`apply_gcp_prior_residual`]) — added to [`SfmBundleProblem`]'s `residual_terms`/`evaluate` alongside
/// the ordinary reprojection terms. Wiring that fully into [`IncrementalSfm`]'s bundle-adjustment calls is
/// a follow-up; this crate concretely implements the camera-center alignment case ([`align_to_priors`])
/// and the GCP residual primitive ([`apply_gcp_prior_residual`]) now.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PosePrior {
    GpsPosition { frame: usize, enu: [f64; 3], sigma: f64 },
    GravityUp { frame: usize, up_enu: [f64; 3], sigma: f64 },
    Gcp { point_track_id: usize, world: [f64; 3], sigma: f64 },
}

/// 📌️ Umeyama alignment (with scale) of a reconstruction's camera centers to their GPS positions:
/// `(frame, enu)` pairs are matched against `recon.cameras` by frame index, and the resulting similarity
/// maps the reconstruction's (arbitrary monocular-SfM gauge/scale) frame into the GPS/ENU frame. `None`
/// if fewer than 3 frames match or the matched centers are degenerate (collinear/coincident).
pub fn align_to_priors(recon: &Reconstruction, gps_priors: &[(usize, [f64; 3])]) -> Option<Sim3> {
    let mut src = Vec::new();
    let mut dst = Vec::new();
    for &(frame_idx, gps) in gps_priors {
        if let Some(&(_, pose)) = recon.cameras.iter().find(|&&(f, _)| f == frame_idx) {
            src.push(camera_center(&pose));
            dst.push(gps);
        }
    }
    umeyama(&src, &dst, true)
}

/// 📌️ GCP (ground control point) prior residual: `(point - known_world) / sigma`, the extension-point
/// primitive described on [`PosePrior`] — usable as the payload of a B-only
/// ([`crate::standards::v1::subsets::any::schema::optimize_internals::ResidualTerm`] with `a_index: None`) term inside a
/// [`crate::standards::v1::subsets::any::schema::optimize_internals::BipartiteResiduals`] problem. Returns `(residual, jacobian_wrt_point)`;
/// `sigma <= 0` is treated as `1.0` (an un-weighted prior) rather than dividing by zero.
pub fn apply_gcp_prior_residual(point: [f64; 3], known_world: [f64; 3], sigma: f64) -> (VecD, MatD) {
    let inv_sigma = if sigma > 1e-12 { 1.0 / sigma } else { 1.0 };
    let r = VecD::from_vec((0..3).map(|k| (point[k] - known_world[k]) * inv_sigma).collect());
    let mut jb = MatD::zeros(3, 3);
    for k in 0..3 {
        jb.set(k, k, inv_sigma);
    }
    (r, jb)
}
// #endregion 🔖️Priors
