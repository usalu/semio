//! 🏗️ Structure from motion: two-view geometry, triangulation, PnP, incremental and global reconstruction, bundle adjustment, loop closure and pose priors.

// 🔗️ Sibling engine topic files, aliased to their pre-merge crate names so every path in
// this file is byte-identical to the crate it was moved from (see 🦀️.rs for the wiring).

use semio_s_artifact_remodel_remodeling_camera::editor::remodeling::engine::camera as remodeling_camera;
use semio_s_artifact_remodel_remodeling_feature::editor::remodeling::engine::feature as remodeling_feature;
use semio_s_artifact_remodel_remodeling_images::editor::remodeling::engine::images as remodeling_image;
use semio_s_artifact_remodel_remodeling_lie::standards::v1::subsets::any::schema::lie_internals::Se3;
use semio_s_artifact_remodel_remodeling_lie::standards::v1::subsets::any::schema::lie_internals::Sim3;
use semio_s_artifact_remodel_remodeling_lie::standards::v1::subsets::any::schema::lie_internals::So3;
use semio_s_artifact_remodel_remodeling_optimize::standards::v1::subsets::any::schema::optimize_internals::RobustLoss;
use remodeling_camera::{CameraPose, Distortion, Intrinsics};
use semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::jacobi_eigen_symmetric;
use semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::poly_roots_companion;
use semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::real_eigenvalues;
use semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::svd;
use semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::svd_nullvector;
use semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::vec3d_length;
use semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::vec3d_normalize;
use semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::vec3d_sub;
use semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::Mat3d;
use semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::MatD;
use semio_s_artifact_remodel_remodeling_algebra::standards::v1::subsets::any::schema::algebra_internals::VecD;
use semio_s_artifact_remodel_remodeling_lie::standards::v1::subsets::any::schema::lie_internals::umeyama;
use semio_s_artifact_remodel_remodeling_optimize::standards::v1::subsets::any::schema::optimize_internals::lo_ransac;
use semio_s_artifact_remodel_remodeling_optimize::standards::v1::subsets::any::schema::optimize_internals::numeric_jacobian;
use semio_s_artifact_remodel_remodeling_optimize::standards::v1::subsets::any::schema::optimize_internals::ransac;
use semio_s_artifact_remodel_remodeling_optimize::standards::v1::subsets::any::schema::optimize_internals::BipartiteResiduals;
use semio_s_artifact_remodel_remodeling_optimize::standards::v1::subsets::any::schema::optimize_internals::LeastSquaresProblem;
use semio_s_artifact_remodel_remodeling_optimize::standards::v1::subsets::any::schema::optimize_internals::LmConfig;
use semio_s_artifact_remodel_remodeling_optimize::standards::v1::subsets::any::schema::optimize_internals::MinimalSolver;
use semio_s_artifact_remodel_remodeling_optimize::standards::v1::subsets::any::schema::optimize_internals::RansacConfig;
use semio_s_artifact_remodel_remodeling_optimize::standards::v1::subsets::any::schema::optimize_internals::RansacScoring;
use semio_s_artifact_remodel_remodeling_optimize::standards::v1::subsets::any::schema::optimize_internals::ResidualTerm;
use geometry::random::{normal, Rng};
use remodeling_camera::reproject;
use remodeling_feature::{match_brute, Descriptor256, Keypoint, Match};
use remodeling_image::ImageGray;
use semio_s_artifact_remodel_remodeling_optimize::standards::v1::subsets::any::schema::optimize_internals::SchurLmPreparation;
#[cfg(test)] use semio_s_artifact_remodel_remodeling_optimize::standards::v1::subsets::any::schema::optimize_internals::{schur_lm, SchurResult};
use semio_s_artifact_remodel_remodeling_sfm_geometry::editor::remodeling::engine::sfm::geometry::FeatureTracks;
use semio_s_artifact_remodel_remodeling_sfm_geometry::editor::remodeling::engine::sfm::geometry::INTERACTIVE_POSE_REFINE_ITERATIONS;
use semio_s_artifact_remodel_remodeling_sfm_geometry::editor::remodeling::engine::sfm::geometry::P3pSolver;
use semio_s_artifact_remodel_remodeling_sfm_geometry::editor::remodeling::engine::sfm::geometry::SfmError;
use semio_s_artifact_remodel_remodeling_sfm_geometry::editor::remodeling::engine::sfm::geometry::TwoViewModel;
use semio_s_artifact_remodel_remodeling_sfm_geometry::editor::remodeling::engine::sfm::geometry::TwoViewResult;
use semio_s_artifact_remodel_remodeling_sfm_geometry::editor::remodeling::engine::sfm::geometry::camera_center;
use semio_s_artifact_remodel_remodeling_sfm_geometry::editor::remodeling::engine::sfm::geometry::cross3;
use semio_s_artifact_remodel_remodeling_sfm_geometry::editor::remodeling::engine::sfm::geometry::decompose_essential;
use semio_s_artifact_remodel_remodeling_sfm_geometry::editor::remodeling::engine::sfm::geometry::dot3;
use semio_s_artifact_remodel_remodeling_sfm_geometry::editor::remodeling::engine::sfm::geometry::estimate_essential;
use semio_s_artifact_remodel_remodeling_sfm_geometry::editor::remodeling::engine::sfm::geometry::estimate_essential_five_point;
use semio_s_artifact_remodel_remodeling_sfm_geometry::editor::remodeling::engine::sfm::geometry::estimate_essential_five_point_bounded;
use semio_s_artifact_remodel_remodeling_sfm_geometry::editor::remodeling::engine::sfm::geometry::estimate_essential_with_limit;
use semio_s_artifact_remodel_remodeling_sfm_geometry::editor::remodeling::engine::sfm::geometry::estimate_essential_with_threshold;
use semio_s_artifact_remodel_remodeling_sfm_geometry::editor::remodeling::engine::sfm::geometry::mat3d_to_array;
use semio_s_artifact_remodel_remodeling_sfm_geometry::editor::remodeling::engine::sfm::geometry::pnp_ransac;
use semio_s_artifact_remodel_remodeling_sfm_geometry::editor::remodeling::engine::sfm::geometry::pnp_ransac_bounded;
use semio_s_artifact_remodel_remodeling_sfm_geometry::editor::remodeling::engine::sfm::geometry::polish_essential;
use semio_s_artifact_remodel_remodeling_sfm_geometry::editor::remodeling::engine::sfm::geometry::scale3;
use semio_s_artifact_remodel_remodeling_sfm_geometry::editor::remodeling::engine::sfm::geometry::triangulate_and_validate;
use semio_s_artifact_remodel_remodeling_sfm_geometry::editor::remodeling::engine::sfm::geometry::triangulate_normalized_pair;
use semio_s_artifact_remodel_remodeling_sfm_geometry::editor::remodeling::engine::sfm::geometry::triangulation_angle;


const MAX_INTERACTIVE_SEED_CORRESPONDENCES: usize = 64;

const MAX_INTERACTIVE_SEED_HYPOTHESES: usize = 32;

const MAX_INTERACTIVE_REGISTRATION_CORRESPONDENCES: usize = 64;

const MAX_INTERACTIVE_REGISTRATION_ATTEMPTS: u64 = 64;

/// 🎯️ Hypotheses the bounded PnP registration draws before it commits when the running best already
/// explains most correspondences; a poorer best keeps drawing up to
/// [`MAX_INTERACTIVE_REGISTRATION_ATTEMPTS`].
const MIN_INTERACTIVE_REGISTRATION_ATTEMPTS: u64 = 8;

/// 🎯️ Inlier share of the 2D–3D correspondences a bounded PnP pose must explain to be committed.
const MIN_INTERACTIVE_REGISTRATION_INLIER_SHARE: f64 = 0.25;

/// 🎯️ Reprojection tolerance of the bounded PnP relative to the configured RANSAC threshold: the
/// points it registers against were triangulated from neighbouring views a few degrees apart, from
/// integer-pixel keypoints, so they carry a few pixels of error the later bundle cleanup removes;
/// held to the seed threshold, a correct pose on the orbit fixture explained 29 % of its
/// correspondences and was refused.
const INTERACTIVE_REGISTRATION_THRESHOLD_FACTOR: f64 = 2.0;

/// 🎯️ Inlier share past which the bounded PnP stops drawing early.
const EARLY_INTERACTIVE_REGISTRATION_INLIER_SHARE: f64 = 0.6;

/// 🎯️ Bounded hypothesis batches the two-view registration fallback spends before it commits to the
/// best essential matrix it found (each batch is one [`MAX_INTERACTIVE_SEED_HYPOTHESES`] draw from
/// each of the five-point and eight-point estimators).
const MAX_INTERACTIVE_TWO_VIEW_ATTEMPTS: usize = 32;

/// 🎯️ Hypotheses each estimator draws per two-view fallback call: four five-point and four
/// eight-point solves, unpolished, keep one call inside a worker step; the best model is polished
/// once when the fallback commits.
const INTERACTIVE_TWO_VIEW_HYPOTHESES_PER_CALL: usize = 4;

const MAX_INTERACTIVE_TRACK_OBSERVATIONS: usize = 8;

/// 🧵️ Tracks the bounded registration, triangulation and snapshot walks consider. Every walk is a
/// cursor advanced by the call's work budget, so this caps the reconstruction's point count, not a
/// call's cost. It must exceed the tracks a registrable capture produces before its later frames
/// start: at 512 a 36-view orbit had every track past the first ~4 views ignored, so PnP starved
/// from the fifth camera on.
const MAX_INTERACTIVE_TRACKS: usize = 4_096;

// #region 🔖️Fixtures

fn norm3(a: [f64; 3]) -> f64 {
    dot3(a, a).sqrt()
}

fn normalize3(a: [f64; 3]) -> [f64; 3] {
    let n = norm3(a);
    if n < 1e-300 {
        a
    } else {
        scale3(a, 1.0 / n)
    }
}

fn sub3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

/// 🎥️ Builds a world-to-camera [`Se3`] whose optical axis looks from `eye` toward `target`, with `up`
/// as the approximate world-up direction (Gram-Schmidt'd against the look direction).
fn look_at_pose(eye: [f64; 3], target: [f64; 3], up: [f64; 3]) -> Se3 {
    let forward = normalize3(sub3(target, eye));
    let right = normalize3(cross3(up, forward));
    let true_up = cross3(forward, right);
    // Camera-to-world rotation has *columns* (right, true_up, forward) — applying it to a camera-frame
    // basis vector (e.g. camera +z) yields that axis expressed in world coordinates. `Mat3d::from_axes`
    // sets columns directly; using `array_to_mat3d` here (which treats its argument as *rows*) would
    // silently build the transpose (world-to-camera) and then invert it into camera-to-world again,
    // leaving every camera pointed backwards — verified as a real, caught bug during test development.
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

// #region 🔖️Incremental

/// 🎛️ Tuning for [`IncrementalSfm`]: RANSAC inlier threshold (pixels), minimum track length to
/// triangulate, bundle-adjustment iteration cap, IRLS robust loss, minimum triangulation angle, and
/// the per-camera visible-point floor used by [`IncrementalSfm::prune_outliers`].
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
pub struct SfmConfig {
    pub ransac_threshold_px: f64,
    pub min_track_length: usize,
    pub ba_max_iterations: usize,
    pub robust_loss: RobustLoss,
    pub min_triangulation_angle_rad: f64,
    pub min_visible_points_to_keep_camera: usize,
}

impl Default for SfmConfig {
    fn default() -> Self {
        Self { ransac_threshold_px: 2.0, min_track_length: 3, ba_max_iterations: 50, robust_loss: RobustLoss::Huber(2.0), min_triangulation_angle_rad: 2.0_f64.to_radians(), min_visible_points_to_keep_camera: 6 }
    }
}

/// 📦️ A finished (or in-progress) reconstruction snapshot: registered cameras (by frame index), their
/// triangulated points, each point's originating track id, and the shared calibration.
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
pub struct Reconstruction {
    pub cameras: Vec<(usize, CameraPose)>,
    pub points: Vec<[f64; 3]>,
    pub point_track_ids: Vec<usize>,
    pub intrinsics: Intrinsics,
}

/// 🧩️ Bundle-adjustment [`BipartiteResiduals`] problem for [`IncrementalSfm`]'s `local_ba`/`global_ba`:
/// A-blocks are cameras (6-dof `se(3)` log-tangent), B-blocks are points (3-dof XYZ), both updated via
/// `schur_lm`'s plain elementwise vector addition — since `BipartiteResiduals` has no manifold-retraction
/// hook (unlike [`LeastSquaresProblem::plus`]), a camera's raw tangent parameter is *not* re-retracted
/// through `Se3::exp`/`semio_compose_rs` between iterations, only added to directly. This is a first-order
/// approximation, exact only for small per-iteration steps; it's adequate here because `local_ba`/
/// `global_ba` are always seeded from an already-reasonable pose (PnP/triangulation output), keeping
/// steps small. `observations` maps `(a_index, b_index)` to that camera-point pair's pixel observation,
/// since [`BipartiteResiduals::evaluate`] receives a term but not its index into `residual_terms()`.
#[derive(semio_framework_value::RetireOwned)]
pub struct SfmBundleProblem {
    pub intrinsics: Intrinsics,
    pub num_cameras: usize,
    pub num_points: usize,
    pub terms: Vec<ResidualTerm>,
    pub observations: std::collections::HashMap<(usize, usize), [f64; 2]>,
    /// 📌️ Cameras (by A-index) the adjustment holds still: their pose Jacobian is zero, so with
    /// damping on the diagonal the solver's step for them is zero. Holding the two oldest cameras
    /// of a window pins the reconstruction's gauge — rotation, translation AND scale — which
    /// otherwise floats freely through every local adjustment and shows up as the seed pair's
    /// pose wandering off by degrees while nothing in the images moved.
    pub fixed_cameras: Vec<bool>,
}

impl BipartiteResiduals for SfmBundleProblem {
    fn num_a_blocks(&self) -> usize {
        self.num_cameras
    }

    fn num_b_blocks(&self) -> usize {
        self.num_points
    }

    fn a_block_dim(&self) -> usize {
        6
    }

    fn b_block_dim(&self) -> usize {
        3
    }

    fn residual_terms(&self) -> &[ResidualTerm] {
        &self.terms
    }

    fn evaluate(&self, a_params: &[VecD], b_params: &[VecD], term: &ResidualTerm) -> (VecD, MatD, MatD) {
        let ai = term.a_index.expect("sfm bundle terms always touch a camera");
        let bi = term.b_index.expect("sfm bundle terms always touch a point");
        let obs = self.observations[&(ai, bi)];
        let eps = 1e-6;
        let residual_at = |xi: [f64; 6], point: [f64; 3]| -> [f64; 2] {
            let pose = CameraPose(Se3::exp(xi));
            reproject(&self.intrinsics, &pose, point).map_or([obs[0] + 1.0e3, obs[1] + 1.0e3], |p| [p[0] - obs[0], p[1] - obs[1]])
        };
        let xi: [f64; 6] = std::array::from_fn(|k| a_params[ai].get(k));
        let point: [f64; 3] = std::array::from_fn(|k| b_params[bi].get(k));
        let r = residual_at(xi, point);
        let mut ja = MatD::zeros(2, 6);
        let camera_free = !self.fixed_cameras.get(ai).copied().unwrap_or(false);
        for k in (0..6).filter(|_| camera_free) {
            let mut xp = xi;
            xp[k] += eps;
            let mut xm = xi;
            xm[k] -= eps;
            let rp = residual_at(xp, point);
            let rm = residual_at(xm, point);
            ja.set(0, k, (rp[0] - rm[0]) / (2.0 * eps));
            ja.set(1, k, (rp[1] - rm[1]) / (2.0 * eps));
        }
        let mut jb = MatD::zeros(2, 3);
        for k in 0..3 {
            let mut pp = point;
            pp[k] += eps;
            let mut pm = point;
            pm[k] -= eps;
            let rp = residual_at(xi, pp);
            let rm = residual_at(xi, pm);
            jb.set(0, k, (rp[0] - rm[0]) / (2.0 * eps));
            jb.set(1, k, (rp[1] - rm[1]) / (2.0 * eps));
        }
        (VecD::from_vec(vec![r[0], r[1]]), ja, jb)
    }
}

/// 🎥️ Two-view essential-matrix estimation for [`IncrementalSfm::init_pair`]'s seed pair: runs both the
/// Nistér five-point solver ([`estimate_essential_five_point`], primary) and the normalized 8-point
/// [`estimate_essential`] (kept as a working fallback, not deleted), keeping whichever achieves the lower
/// (better) MSAC cost. Consecutive video frames — this crate's primary target, unlike a curated
/// wide-baseline photoset — have far smaller parallax than a typical photogrammetry pair, exactly the
/// regime where the unconstrained 8-DOF linear 8-point fit degenerates (see
/// [`estimate_essential_five_point`]'s docs and the `TwoViewFivePointTests` planar-scene contract test)
/// while the constrained 5-DOF five-point manifold stays well-conditioned. Both solvers run over the same
/// correspondences with the same normalized-ray MSAC threshold, so their [`TwoViewResult::score`]s are
/// directly comparable; five-point wins ties.
#[cfg(test)]
fn estimate_init_pair_essential(matches: &[([f64; 2], [f64; 2])], k: &Intrinsics, seed: u64) -> Option<TwoViewResult> {
    const FIVE_POINT_THRESHOLD: f64 = 0.005;
    let five_point = estimate_essential_five_point(matches, k, k, FIVE_POINT_THRESHOLD, seed);
    let eight_point = estimate_essential(matches, k, k);
    match (five_point, eight_point) {
        (Some(five), Some(eight)) => Some(if five.score <= eight.score { five } else { eight }),
        (Some(five), None) => Some(five),
        (None, Some(eight)) => Some(eight),
        (None, None) => None,
    }
}

/// 🏗️ Incremental structure-from-motion pipeline: register cameras one at a time via PnP against an
/// already-triangulated point cloud, growing the reconstruction frame by frame.
#[derive(semio_framework_value::RetireOwned)]
pub struct IncrementalSfm {
    intrinsics: Intrinsics,
    tracks: FeatureTracks,
    keypoints_per_frame: Vec<Vec<Keypoint>>,
    pairwise_matches: Vec<(usize, usize, Vec<Match>)>,
    cfg: SfmConfig,
    cameras: Vec<(usize, CameraPose)>,
    points: std::collections::BTreeMap<usize, [f64; 3]>,
    point_events: Option<Vec<(usize, [f64; 3], bool)>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
pub enum SeedPairPhase {
    Collect,
    Solve,
    Triangulate,
    Done,
}

#[derive(semio_framework_value::RetireOwned)]
pub struct SeedPairPreparation {
    frame_a: usize,
    frame_b: usize,
    matches: Vec<Match>,
    cursor: usize,
    correspondences: Vec<([f64; 2], [f64; 2])>,
    phase: SeedPairPhase,
    /// 🎯️ Hypothesis batches spent and the best (polished) essential-matrix estimate they
    /// produced, with the raw inlier count of the candidate it was polished from: one bounded
    /// five-point batch per call, so no call runs the whole search.
    attempts: usize,
    best: Option<TwoViewResult>,
    best_raw_score: f64,
}

/// 🎯️ Five-point hypotheses one seed-solve call draws, and the calls a seed spends before it
/// commits to the best model found.
const SEED_HYPOTHESES_PER_CALL: usize = 2;

const SEED_SOLVE_CALLS: usize = 32;

impl SeedPairPreparation {
    pub fn new(frame_a: usize, frame_b: usize, matches: &[Match]) -> Self {
        Self {
            frame_a,
            frame_b,
            matches: matches.iter().take(MAX_INTERACTIVE_SEED_CORRESPONDENCES).copied().collect(),
            cursor: 0,
            correspondences: Vec::with_capacity(matches.len().min(MAX_INTERACTIVE_SEED_CORRESPONDENCES)),
            phase: SeedPairPhase::Collect,
            attempts: 0,
            best: None,
            best_raw_score: f64::INFINITY,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
pub enum RegistrationPhase {
    Collect,
    Solve,
    /// 📐️ PnP found no pose: pick the registered reference sharing the most direct matches.
    TwoViewSelect,
    /// 📐️ Collect that reference's correspondences (stride-subsampled to the bounded maximum).
    TwoViewCollect,
    /// 📐️ One bounded essential-matrix hypothesis batch per call, keeping the best model so far.
    TwoViewSolve,
    /// 📐️ Walk the triangulated tracks shared with the reference to recover the metric scale.
    TwoViewScale,
    /// 📐️ Commit the scaled pose once it reprojects the shared points.
    TwoViewCommit,
    Triangulate,
    Done,
    Failed,
}

/// 📐️ Bounded two-view fallback state for one frame PnP could not register: essential matrix
/// against the best-connected registered camera, scaled by the shared triangulated points (or, when
/// matching left none, by the seed pair's baseline). Every field is a cursor or a fixed-size buffer,
/// so no call materializes more than [`MAX_INTERACTIVE_REGISTRATION_CORRESPONDENCES`] items.
#[derive(semio_framework_value::RetireOwned)]
struct TwoViewFallback {
    reference: usize,
    correspondences: Vec<([f64; 2], [f64; 2])>,
    attempts: usize,
    best: Option<TwoViewResult>,
    relative: Option<Se3>,
    cursor: usize,
    scales: Vec<f64>,
    shared: Vec<([f64; 3], [f64; 2])>,
}

#[derive(semio_framework_value::RetireOwned)]
pub struct RegistrationPreparation {
    frame: usize,
    cursor: usize,
    attempts: u64,
    world_points: Vec<[f64; 3]>,
    observations: Vec<[f64; 2]>,
    phase: RegistrationPhase,
    two_view: Option<Box<TwoViewFallback>>,
    /// 🎯️ Best PnP hypothesis so far across the one-draw-per-call attempts: pose and inlier count.
    /// One draw is one random 3-point sample, so committing the FIRST pose a call answers registers
    /// the camera on whatever three correspondences came up — and every later triangulation and
    /// registration inherits that pose's error (the orbit fixture's third camera came out 16° off,
    /// the fourth 108°). The draws are kept and the best-supported pose is committed instead.
    best: Option<(CameraPose, usize)>,
}

impl RegistrationPreparation {
    pub fn new(frame: usize) -> Self {
        Self { frame, cursor: 0, attempts: 0, world_points: Vec::with_capacity(MAX_INTERACTIVE_REGISTRATION_CORRESPONDENCES), observations: Vec::with_capacity(MAX_INTERACTIVE_REGISTRATION_CORRESPONDENCES), phase: RegistrationPhase::Collect, two_view: None, best: None }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
pub enum BundlePhase {
    Prune,
    Retriangulate,
    Done,
}

/// 🎯️ A bundle adjustment in flight (see [`IncrementalSfm::begin_bundle_adjustment`]).
#[derive(semio_framework_value::RetireOwned)]
pub struct BundleAdjustment {
    frames: Vec<usize>,
    point_track_ids: Vec<usize>,
    problem: SfmBundleProblem,
    cfg: LmConfig,
    preparation: SchurLmPreparation,
}

#[derive(semio_framework_value::RetireOwned)]
pub struct BundlePreparation {
    point_track_ids: Vec<usize>,
    cursor: usize,
    phase: BundlePhase,
    /// 🧹️ Multiple of the RANSAC threshold a point's worst reprojection may reach before pruning.
    tolerance: f64,
}

#[derive(semio_framework_value::RetireOwned)]
pub struct ReconstructionSnapshotPreparation {
    camera_cursor: usize,
    point_cursor: usize,
    point_track_ids: Vec<usize>,
    reconstruction: Reconstruction,
}

impl IncrementalSfm {
    /// 🆕️ Starts an empty incremental reconstruction over a shared calibration, precomputed feature tracks and per-frame keypoints.
    pub fn new(intrinsics: Intrinsics, tracks: FeatureTracks, keypoints_per_frame: Vec<Vec<Keypoint>>, cfg: SfmConfig) -> Self {
        Self { intrinsics, tracks, keypoints_per_frame, pairwise_matches: Vec::new(), cfg, cameras: Vec::new(), points: std::collections::BTreeMap::new(), point_events: None }
    }

    /// 🧮️ Retained heap bytes of everything this reconstruction holds on to: the tracks and keypoints it
    /// was handed, the pairwise match table, the registered cameras, the triangulated point map and any
    /// recorded point events. A memory probe needs this without seeing the private fields.
    pub fn retained_bytes(&self) -> usize {
        let nested = |outer: usize, inner: usize| outer + inner;
        nested(
            self.tracks.tracks.len() * size_of::<Vec<(usize, u32)>>(),
            self.tracks.tracks.iter().map(|track| track.capacity() * size_of::<(usize, u32)>()).sum::<usize>(),
        ) + nested(
            self.keypoints_per_frame.len() * size_of::<Vec<Keypoint>>(),
            self.keypoints_per_frame.iter().map(|frame| frame.capacity() * size_of::<Keypoint>()).sum::<usize>(),
        ) + nested(
            self.pairwise_matches.len() * size_of::<(usize, usize, Vec<Match>)>(),
            self.pairwise_matches.iter().map(|(_, _, matches)| matches.capacity() * size_of::<Match>()).sum::<usize>(),
        ) + self.cameras.capacity() * size_of::<(usize, CameraPose)>()
            + self.points.len() * (size_of::<usize>() + size_of::<[f64; 3]>() + 16)
            + self.point_events.as_ref().map_or(0, |events| events.capacity() * size_of::<(usize, [f64; 3], bool)>())
    }

    /// 🔭️ Starts recording every triangulated (`true`) and pruned (`false`) point by track id.
    pub fn observe_points(&mut self) {
        self.point_events.get_or_insert_with(Vec::new);
    }

    /// 📤️ Hands every recorded point event to `sink`, oldest first.
    pub fn drain_point_events(&mut self, mut sink: impl FnMut(usize, [f64; 3], bool)) {
        for (track, point, kept) in self.point_events.as_mut().map(std::mem::take).unwrap_or_default() {
            sink(track, point, kept);
        }
    }

    /// 📷️ The registered pose of `frame`, if any.
    pub fn camera_pose(&self, frame: usize) -> Option<CameraPose> {
        self.pose_of(frame)
    }

    /// 🕸️ Attaches the pairwise match table used for two-view registration fallbacks (track union-find alone
    /// drops conflicted chains that JPEG matching often creates, starving essential-matrix correspondence).
    pub fn set_pairwise_matches(&mut self, pairwise_matches: Vec<(usize, usize, Vec<Match>)>) {
        self.pairwise_matches = pairwise_matches;
    }

    #[cfg(test)]
    fn is_registered(&self, frame: usize) -> bool {
        self.cameras.iter().any(|&(f, _)| f == frame)
    }

    fn pose_of(&self, frame: usize) -> Option<CameraPose> {
        self.cameras.iter().find(|&&(f, _)| f == frame).map(|&(_, p)| p)
    }

    fn obs_px(&self, frame: usize, kp: u32) -> [f64; 2] {
        let k = self.keypoints_per_frame[frame][kp as usize];
        [f64::from(k.x), f64::from(k.y)]
    }

    fn track_obs_in(&self, track: &[(usize, u32)], frame: usize) -> Option<[f64; 2]> {
        track.iter().find(|&&(f, _)| f == frame).map(|&(f, kp)| self.obs_px(f, kp))
    }

    /// 🎯️ Number of already-triangulated tracks also observed in `frame` — the 2D-3D
    /// correspondence count [`register_next`](Self::register_next) would feed to PnP.
    /// 🕸️ Direct pairwise match count between `frame` and any currently registered camera.
    #[cfg(test)]
    pub fn pairwise_match_count(&self, frame: usize) -> usize {
        let registered: std::collections::HashSet<usize> = self.cameras.iter().map(|&(f, _)| f).collect();
        let mut best = 0usize;
        for &(a, b, ref matches) in &self.pairwise_matches {
            if matches.len() < 8 {
                continue;
            }
            if (a == frame && registered.contains(&b)) || (b == frame && registered.contains(&a)) {
                best = best.max(matches.len());
            }
        }
        best
    }

    #[cfg(test)]
    pub fn pnp_correspondence_count(&self, frame: usize) -> usize {
        self.tracks.tracks.iter().enumerate().filter(|(track_id, track)| self.points.contains_key(track_id) && self.track_obs_in(track, frame).is_some()).count()
    }

    /// 🌱️ Whether `frame` already has a registered camera pose in this reconstruction.
    #[cfg(test)]
    pub fn has_camera(&self, frame: usize) -> bool {
        self.is_registered(frame)
    }

    /// 🔢️ How many triangulated points `frame` observes — the 2D–3D correspondences a PnP
    /// registration of that frame can draw from.
    pub fn correspondence_count(&self, frame: usize) -> usize {
        self.points.keys().filter(|&&track_id| self.tracks.tracks.get(track_id).is_some_and(|track| track.iter().any(|&(observed, _)| observed == frame))).count()
    }

    /// 🧵️ The observations `(frame, keypoint)` of every triangulated track `frame` observes, with
    /// the track's point — what a registration of `frame` sees, laid out for an audit.
    pub fn correspondences_of(&self, frame: usize) -> Vec<(usize, [f64; 3], Vec<(usize, u32)>)> {
        self.points.iter().filter_map(|(&track_id, &point)| self.tracks.tracks.get(track_id).filter(|track| track.iter().any(|&(observed, _)| observed == frame)).map(|track| (track_id, point, track.clone()))).collect()
    }

    /// 🧵️ Every track observing `frame`, with whether it already carries a point.
    pub fn tracks_observing(&self, frame: usize) -> Vec<(Vec<(usize, u32)>, bool)> {
        self.tracks.tracks.iter().enumerate().filter(|(_, track)| track.iter().any(|&(observed, _)| observed == frame)).map(|(track_id, track)| (track.clone(), self.points.contains_key(&track_id))).collect()
    }

    /// 🔢️ Triangulated points.
    pub fn point_count(&self) -> usize {
        self.points.len()
    }

    pub fn registered_count(&self) -> usize {
        self.cameras.len()
    }

    pub fn camera_pose_prefix(&self, maximum: usize) -> Vec<CameraPose> {
        self.cameras.iter().take(maximum).map(|&(_, pose)| pose).collect()
    }

    pub fn point_prefix(&self, maximum: usize) -> Vec<[f64; 3]> {
        self.points.values().take(maximum).copied().collect()
    }

    /// 🌱️ Seeds the reconstruction from an initial pair: estimates the essential matrix + relative pose via
    /// [`estimate_init_pair_essential`] (frame_a at identity, frame_b at the recovered relative pose), then
    /// triangulates every track shared between the two frames.
    #[cfg(test)]
    pub fn init_pair(&mut self, frame_a: usize, frame_b: usize, matches: &[Match]) -> Result<(), SfmError> {
        if matches.len() < 8 {
            return Err(SfmError::InsufficientMatches);
        }
        let corr: Vec<([f64; 2], [f64; 2])> = matches.iter().map(|m| (self.obs_px(frame_a, m.a), self.obs_px(frame_b, m.b))).collect();
        let two_view = estimate_init_pair_essential(&corr, &self.intrinsics, frame_b as u64).ok_or(SfmError::DegenerateGeometry)?;
        let TwoViewModel::Fundamental(e) = two_view.model else {
            return Err(SfmError::DegenerateGeometry);
        };
        let inlier_rays: Vec<([f64; 2], [f64; 2])> = two_view
            .inliers
            .iter()
            .map(|&idx| {
                let ra = self.intrinsics.unproject_ray(corr[idx].0);
                let rb = self.intrinsics.unproject_ray(corr[idx].1);
                ([ra[0], ra[1]], [rb[0], rb[1]])
            })
            .collect();
        let relative_pose = decompose_essential(&e, &inlier_rays).ok_or(SfmError::DegenerateGeometry)?;

        self.cameras.clear();
        self.points.clear();
        self.cameras.push((frame_a, CameraPose(Se3::identity())));
        self.cameras.push((frame_b, CameraPose(relative_pose)));

        self.triangulate_new(frame_a);
        self.triangulate_new(frame_b);
        Ok(())
    }

    /// 🎯️ Registers `frame` via PnP against every already-triangulated track observed there. Requires at
    /// least [`P3pSolver::SAMPLE_SIZE`] 2D-3D correspondences. When PnP cannot run yet, falls back to a
    /// two-view essential-matrix pose against the best-connected registered reference only when enough
    /// shared triangulated points exist to recover a consistent metric scale and the refined pose
    /// reprojects those points within `3 * ransac_threshold_px`.
    #[cfg(test)]
    pub fn register_next(&mut self, frame: usize) -> Result<(), SfmError> {
        if self.is_registered(frame) {
            return Ok(());
        }
        let mut world_pts = Vec::new();
        let mut obs = Vec::new();
        for (track_id, track) in self.tracks.tracks.iter().enumerate() {
            let Some(&point) = self.points.get(&track_id) else { continue };
            let Some(px) = self.track_obs_in(track, frame) else { continue };
            world_pts.push(point);
            obs.push(px);
        }
        if world_pts.len() >= P3pSolver::SAMPLE_SIZE {
            let cfg = RansacConfig { threshold: self.cfg.ransac_threshold_px, confidence: 0.999, max_iters: 2000, seed: frame as u64, scoring: RansacScoring::Msac };
            if let Some((pose, _inliers)) = pnp_ransac(&self.intrinsics, &world_pts, &obs, &cfg) {
                self.cameras.push((frame, pose));
                return Ok(());
            }
        }
        self.register_next_two_view(frame)
    }

    /// 📐️ Two-view fallback for [`register_next`](Self::register_next): essential-matrix relative pose vs the
    /// registered frame sharing the most track observations. Metric scale prefers the median ratio of known
    /// triangulated-point distances to unit-baseline triangulations; when JPEG/matching leaves no shared
    /// 3D point, falls back to the most recent registered inter-camera baseline length.
    #[cfg(test)]
    fn register_next_two_view(&mut self, frame: usize) -> Result<(), SfmError> {
        let registered: Vec<usize> = self.cameras.iter().map(|&(f, _)| f).collect();
        let mut best: Option<(usize, Vec<([f64; 2], [f64; 2])>)> = None;
        for &ref_f in &registered {
            let mut corr = Vec::new();
            // Prefer direct pairwise matches (unordered) so JPEG conflicted track chains do not starve PnP fallback.
            for &(a, b, ref matches) in &self.pairwise_matches {
                let (src, dst, flip) = if a == ref_f && b == frame {
                    (ref_f, frame, false)
                } else if a == frame && b == ref_f {
                    (frame, ref_f, true)
                } else {
                    continue;
                };
                let _ = (src, dst);
                for m in matches {
                    let (px_r, px_f) = if !flip { (self.obs_px(ref_f, m.a), self.obs_px(frame, m.b)) } else { (self.obs_px(ref_f, m.b), self.obs_px(frame, m.a)) };
                    corr.push((px_r, px_f));
                }
            }
            if corr.is_empty() {
                for track in &self.tracks.tracks {
                    let Some(px_r) = self.track_obs_in(track, ref_f) else { continue };
                    let Some(px_f) = self.track_obs_in(track, frame) else { continue };
                    corr.push((px_r, px_f));
                }
            }
            if corr.len() >= 8 && best.as_ref().map_or(true, |(_, c)| corr.len() > c.len()) {
                best = Some((ref_f, corr));
            }
        }
        let (ref_f, mut corr) = best.ok_or(SfmError::PnpFailed)?;
        const MAX_TWO_VIEW_CORRS: usize = 64;
        if corr.len() > MAX_TWO_VIEW_CORRS {
            // Keep a deterministic stride subsample so LO-RANSAC stays bounded on dense JPEG match tables.
            let step = corr.len().div_ceil(MAX_TWO_VIEW_CORRS);
            corr = corr.into_iter().enumerate().filter_map(|(i, c)| (i % step == 0).then_some(c)).collect();
        }
        let two_view = estimate_init_pair_essential(&corr, &self.intrinsics, frame as u64).ok_or(SfmError::DegenerateGeometry)?;
        let TwoViewModel::Fundamental(e) = two_view.model else {
            return Err(SfmError::DegenerateGeometry);
        };
        let inlier_rays: Vec<([f64; 2], [f64; 2])> = two_view
            .inliers
            .iter()
            .map(|&idx| {
                let ra = self.intrinsics.unproject_ray(corr[idx].0);
                let rb = self.intrinsics.unproject_ray(corr[idx].1);
                ([ra[0], ra[1]], [rb[0], rb[1]])
            })
            .collect();
        let relative = decompose_essential(&e, &inlier_rays).ok_or(SfmError::DegenerateGeometry)?;
        let ref_pose = self.pose_of(ref_f).ok_or(SfmError::PnpFailed)?;
        let c_ref = camera_center(&ref_pose);
        let r_arr = mat3d_to_array(&relative.r.0);

        let mut scales = Vec::new();
        let mut shared: Vec<([f64; 3], [f64; 2])> = Vec::new();
        for (track_id, track) in self.tracks.tracks.iter().enumerate() {
            let Some(&point) = self.points.get(&track_id) else { continue };
            let Some(px_r) = self.track_obs_in(track, ref_f) else { continue };
            let Some(px_f) = self.track_obs_in(track, frame) else { continue };
            let ray_r = self.intrinsics.unproject_ray(px_r);
            let ray_f = self.intrinsics.unproject_ray(px_f);
            let Some(p_unit) = triangulate_normalized_pair(&r_arr, relative.t, [ray_r[0], ray_r[1]], [ray_f[0], ray_f[1]]) else {
                continue;
            };
            let dist_metric = norm3(sub3(point, c_ref));
            let dist_unit = norm3(p_unit);
            if dist_unit < 1e-9 || dist_metric < 1e-9 {
                continue;
            }
            scales.push(dist_metric / dist_unit);
            shared.push((point, px_f));
        }
        let s = if scales.is_empty() {
            // 🏃️ JPEG / sparse-track path: no shared triangulated point survived matching, so borrow the
            // metric from the most recent registered baseline (sequential orbit / video assumes similar
            // inter-frame translation magnitude) instead of abandoning the frame.
            if self.cameras.len() < 2 {
                return Err(SfmError::PnpFailed);
            }
            // Anchor metric to the seed pair's baseline so chained two-view poses share one scale.
            let baseline = norm3(sub3(camera_center(&self.cameras[0].1), camera_center(&self.cameras[1].1)));
            if baseline < 1e-9 {
                return Err(SfmError::PnpFailed);
            }
            baseline
        } else {
            scales.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            let median = scales[scales.len() / 2];
            if !median.is_finite() || median <= 1e-9 {
                return Err(SfmError::PnpFailed);
            }
            if scales.len() >= 2 {
                let spread = scales[scales.len() - 1] / scales[0];
                if !spread.is_finite() || spread > 1.5 {
                    return Err(SfmError::PnpFailed);
                }
            }
            median
        };
        let scaled = Se3 { r: relative.r, t: scale3(relative.t, s) };
        let pose = CameraPose(scaled.semio_compose_rs(&ref_pose.0));
        if !shared.is_empty() {
            let max_reproj = self.cfg.ransac_threshold_px * 3.0;
            let mut ok = 0usize;
            for &(point, px) in &shared {
                let Some(pred) = reproject(&self.intrinsics, &pose, point) else { continue };
                let err = ((pred[0] - px[0]).powi(2) + (pred[1] - px[1]).powi(2)).sqrt();
                if err <= max_reproj {
                    ok += 1;
                }
            }
            if ok == 0 {
                return Err(SfmError::PnpFailed);
            }
        }
        self.cameras.push((frame, pose));
        Ok(())
    }

    /// 🧵️ Triangulates every not-yet-triangulated track that (a) has an observation in `frame` and (b) is
    /// observed by at least 2 registered cameras overall and meets [`SfmConfig::min_track_length`], via
    /// [`triangulate_and_validate`].
    fn triangulate_track(&mut self, track_id: usize, required_frame: Option<usize>) {
        if self.points.contains_key(&track_id) {
            return;
        }
        let Some(track) = self.tracks.tracks.get(track_id) else { return };
        if track.len() < self.cfg.min_track_length || required_frame.is_some_and(|frame| !track.iter().any(|&(observed, _)| observed == frame)) {
            return;
        }
        let mut poses = Vec::with_capacity(track.len().min(MAX_INTERACTIVE_TRACK_OBSERVATIONS));
        let mut observations = Vec::with_capacity(track.len().min(MAX_INTERACTIVE_TRACK_OBSERVATIONS));
        for &(frame, keypoint) in track.iter().take(MAX_INTERACTIVE_TRACK_OBSERVATIONS) {
            let Some(pose) = self.pose_of(frame) else { continue };
            poses.push((pose, self.intrinsics));
            observations.push(self.obs_px(frame, keypoint));
        }
        if poses.len() < 2 {
            return;
        }
        if let Some(point) = triangulate_and_validate(&poses, &observations, self.cfg.min_triangulation_angle_rad, self.cfg.ransac_threshold_px * 3.0) {
            self.points.insert(track_id, point);
            if let Some(events) = self.point_events.as_mut() {
                events.push((track_id, point, true));
            }
        }
    }

    /// 🌱️ Advances seed correspondence collection, fixed-size essential solve, or one triangulation.
    pub fn advance_seed_pair(&mut self, preparation: &mut SeedPairPreparation, work_budget: usize) -> Result<bool, SfmError> {
        match preparation.phase {
            SeedPairPhase::Collect => {
                let end = preparation.cursor.saturating_add(work_budget.max(1)).min(preparation.matches.len());
                for matched in &preparation.matches[preparation.cursor..end] {
                    preparation.correspondences.push((self.obs_px(preparation.frame_a, matched.a), self.obs_px(preparation.frame_b, matched.b)));
                }
                preparation.cursor = end;
                if end == preparation.matches.len() {
                    preparation.phase = SeedPairPhase::Solve;
                }
            }
            SeedPairPhase::Solve => {
                if preparation.correspondences.len() < 8 {
                    return Err(SfmError::InsufficientMatches);
                }
                // 🎯️ Five-point minimal solves, one bounded batch per call, the best-supported model
                // kept across calls and polished once at the end: the seed pose anchors every
                // triangulation, PnP registration and bundle adjustment downstream, so it is worth
                // the manifold-true estimate; the unconstrained 8-point fit is only the fallback for
                // a pair the five-point search cannot solve at all.
                // The tolerance is the configured pixel threshold in normalized coordinates, so a
                // 128-pixel frame is judged as leniently as a 320-pixel one.
                let threshold = self.cfg.ransac_threshold_px / self.intrinsics.fx.max(self.intrinsics.fy).max(1.0);
                let seed = (preparation.frame_a as u64) << 32 ^ preparation.frame_b as u64 ^ (preparation.attempts as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15);
                let candidate = estimate_essential_five_point_bounded(&preparation.correspondences, &self.intrinsics, &self.intrinsics, threshold, seed, SEED_HYPOTHESES_PER_CALL, 0);
                let linear = || {
                    let linear = estimate_essential_with_threshold(&preparation.correspondences, &self.intrinsics, &self.intrinsics, threshold, MAX_INTERACTIVE_SEED_HYPOTHESES).ok_or(SfmError::DegenerateGeometry)?;
                    let TwoViewModel::Fundamental(essential) = linear.model else { return Err(SfmError::DegenerateGeometry) };
                    let rays: Vec<([f64; 2], [f64; 2])> = linear
                        .inliers
                        .iter()
                        .take(MAX_INTERACTIVE_SEED_CORRESPONDENCES)
                        .map(|&index| {
                            let left = self.intrinsics.unproject_ray(preparation.correspondences[index].0);
                            let right = self.intrinsics.unproject_ray(preparation.correspondences[index].1);
                            ([left[0], left[1]], [right[0], right[1]])
                        })
                        .collect();
                    decompose_essential(&essential, &rays).ok_or(SfmError::DegenerateGeometry)?;
                    Ok::<TwoViewResult, SfmError>(linear)
                };
                if preparation.attempts == 0 {
                    // 🛑️ Coincident correspondences (every point the same pixel) have no geometry
                    // to solve; answer on the first call rather than after every batch failed.
                    let spread = |side: usize| {
                        let coordinate = |pair: &([f64; 2], [f64; 2])| if side == 0 { pair.0 } else { pair.1 };
                        let first = coordinate(&preparation.correspondences[0]);
                        preparation.correspondences.iter().map(|pair| { let p = coordinate(pair); (p[0] - first[0]).abs().max((p[1] - first[1]).abs()) }).fold(0.0f64, f64::max)
                    };
                    if spread(0) < 1e-9 || spread(1) < 1e-9 {
                        return Err(SfmError::DegenerateGeometry);
                    }
                }
                if let Some(candidate) = candidate {
                    // A candidate that beats the running best is polished right away, so the
                    // comparison is between refined models (a raw five-point fit through five
                    // integer-pixel keypoints explains only a handful of correspondences).
                    if candidate.score < preparation.best_raw_score {
                        preparation.best_raw_score = candidate.score;
                        let polished = polish_essential(candidate, &preparation.correspondences, &self.intrinsics, threshold);
                        if preparation.best.as_ref().is_none_or(|best| polished.inliers.len() > best.inliers.len()) {
                            preparation.best = Some(polished);
                        }
                    }
                }
                preparation.attempts += 1;
                if preparation.attempts < SEED_SOLVE_CALLS {
                    return Ok(false);
                }
                // 🎯️ The five-point search's best model, or the linear fit as the fallback for a
                // pair the minimal solver could not support with eight inliers.
                let estimate = match preparation.best.take().filter(|estimate| estimate.inliers.len() >= 8) {
                    Some(estimate) => estimate,
                    None => linear()?,
                };
                let TwoViewModel::Fundamental(essential) = estimate.model else { return Err(SfmError::DegenerateGeometry) };
                let rays: Vec<([f64; 2], [f64; 2])> = estimate
                    .inliers
                    .iter()
                    .take(MAX_INTERACTIVE_SEED_CORRESPONDENCES)
                    .map(|&index| {
                        let left = self.intrinsics.unproject_ray(preparation.correspondences[index].0);
                        let right = self.intrinsics.unproject_ray(preparation.correspondences[index].1);
                        ([left[0], left[1]], [right[0], right[1]])
                    })
                    .collect();
                let relative = decompose_essential(&essential, &rays).ok_or(SfmError::DegenerateGeometry)?;
                self.cameras.clear();
                self.points.clear();
                self.cameras.push((preparation.frame_a, CameraPose(Se3::identity())));
                self.cameras.push((preparation.frame_b, CameraPose(relative)));
                preparation.cursor = 0;
                preparation.phase = SeedPairPhase::Triangulate;
            }
            SeedPairPhase::Triangulate => {
                for _ in 0..work_budget.max(1) {
                    if preparation.cursor >= self.tracks.tracks.len().min(MAX_INTERACTIVE_TRACKS) {
                        preparation.phase = SeedPairPhase::Done;
                        break;
                    }
                    self.triangulate_track(preparation.cursor, None);
                    preparation.cursor += 1;
                }
            }
            SeedPairPhase::Done => {}
        }
        Ok(preparation.phase == SeedPairPhase::Done)
    }

    /// 🎯️ Advances one frame through correspondence collection, one-iteration PnP attempts, and
    /// per-track triangulation. No solve call receives more than 64 correspondences or one RANSAC draw.
    pub fn advance_registration(&mut self, preparation: &mut RegistrationPreparation, work_budget: usize) -> Result<bool, SfmError> {
        match preparation.phase {
            RegistrationPhase::Collect => {
                for _ in 0..work_budget.max(1) {
                    if preparation.cursor >= self.tracks.tracks.len().min(MAX_INTERACTIVE_TRACKS) {
                        preparation.phase = RegistrationPhase::Solve;
                        break;
                    }
                    let track_id = preparation.cursor;
                    preparation.cursor += 1;
                    let Some(&point) = self.points.get(&track_id) else { continue };
                    let Some(pixel) = self.track_obs_in(&self.tracks.tracks[track_id], preparation.frame) else { continue };
                    if preparation.world_points.len() < MAX_INTERACTIVE_REGISTRATION_CORRESPONDENCES {
                        preparation.world_points.push(point);
                        preparation.observations.push(pixel);
                    }
                }
            }
            RegistrationPhase::Solve => {
                if preparation.world_points.len() < P3pSolver::SAMPLE_SIZE {
                    preparation.phase = RegistrationPhase::TwoViewSelect;
                } else {
                    let config = RansacConfig { threshold: self.cfg.ransac_threshold_px * INTERACTIVE_REGISTRATION_THRESHOLD_FACTOR, confidence: 0.5, max_iters: 1, seed: preparation.frame as u64 ^ preparation.attempts.wrapping_mul(0x9E37_79B9), scoring: RansacScoring::Msac };
                    preparation.attempts += 1;
                    if let Some((pose, inliers)) = pnp_ransac_bounded(&self.intrinsics, &preparation.world_points, &preparation.observations, &config, INTERACTIVE_POSE_REFINE_ITERATIONS) {
                        if preparation.best.as_ref().is_none_or(|(_, best)| inliers.len() > *best) {
                            preparation.best = Some((pose, inliers.len()));
                        }
                    }
                    let correspondences = preparation.world_points.len() as f64;
                    let share = preparation.best.as_ref().map_or(0.0, |(_, inliers)| *inliers as f64 / correspondences);
                    let settled = preparation.attempts >= MAX_INTERACTIVE_REGISTRATION_ATTEMPTS || (preparation.attempts >= MIN_INTERACTIVE_REGISTRATION_ATTEMPTS && share >= EARLY_INTERACTIVE_REGISTRATION_INLIER_SHARE);
                    if settled {
                        match preparation.best.take() {
                            Some((pose, inliers)) if inliers >= 2 * P3pSolver::SAMPLE_SIZE && share >= MIN_INTERACTIVE_REGISTRATION_INLIER_SHARE => {
                                self.cameras.push((preparation.frame, pose));
                                preparation.cursor = 0;
                                preparation.phase = RegistrationPhase::Triangulate;
                            }
                            _ => preparation.phase = RegistrationPhase::TwoViewSelect,
                        }
                    }
                }
            }
            RegistrationPhase::TwoViewSelect => {
                let registered: std::collections::BTreeSet<usize> = self.cameras.iter().map(|&(frame, _)| frame).collect();
                let mut best: Option<(usize, usize)> = None;
                for &(a, b, ref matches) in &self.pairwise_matches {
                    if matches.len() < 8 {
                        continue;
                    }
                    let reference = if a == preparation.frame && registered.contains(&b) {
                        b
                    } else if b == preparation.frame && registered.contains(&a) {
                        a
                    } else {
                        continue;
                    };
                    if best.is_none_or(|(_, count)| matches.len() > count) {
                        best = Some((reference, matches.len()));
                    }
                }
                match best {
                    Some((reference, _)) => {
                        preparation.two_view = Some(Box::new(TwoViewFallback {
                            reference,
                            correspondences: Vec::with_capacity(MAX_INTERACTIVE_REGISTRATION_CORRESPONDENCES),
                            attempts: 0,
                            best: None,
                            relative: None,
                            cursor: 0,
                            scales: Vec::new(),
                            shared: Vec::new(),
                        }));
                        preparation.phase = RegistrationPhase::TwoViewCollect;
                    }
                    None => preparation.phase = RegistrationPhase::Failed,
                }
            }
            RegistrationPhase::TwoViewCollect => {
                let fallback = preparation.two_view.as_mut().expect("two-view fallback");
                let mut collected: Vec<([f64; 2], [f64; 2])> = Vec::new();
                for &(a, b, ref matches) in &self.pairwise_matches {
                    let flip = if a == fallback.reference && b == preparation.frame {
                        false
                    } else if a == preparation.frame && b == fallback.reference {
                        true
                    } else {
                        continue;
                    };
                    // 📏️ Deterministic stride subsample so one solve never sees more than the bounded
                    // maximum, however dense the pair's match table is.
                    let stride = matches.len().div_ceil(MAX_INTERACTIVE_REGISTRATION_CORRESPONDENCES).max(1);
                    for matched in matches.iter().step_by(stride) {
                        let pair = if flip { (self.obs_px(fallback.reference, matched.b), self.obs_px(preparation.frame, matched.a)) } else { (self.obs_px(fallback.reference, matched.a), self.obs_px(preparation.frame, matched.b)) };
                        if collected.len() < MAX_INTERACTIVE_REGISTRATION_CORRESPONDENCES {
                            collected.push(pair);
                        }
                    }
                }
                fallback.correspondences = collected;
                preparation.phase = if fallback.correspondences.len() >= 8 { RegistrationPhase::TwoViewSolve } else { RegistrationPhase::Failed };
            }
            RegistrationPhase::TwoViewSolve => {
                const FIVE_POINT_THRESHOLD: f64 = 0.005;
                let intrinsics = self.intrinsics;
                let frame = preparation.frame;
                let fallback = preparation.two_view.as_mut().expect("two-view fallback");
                let seed = (frame as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ fallback.attempts as u64;
                // 🎯️ One bounded hypothesis batch per call from each estimator; the five-point
                // manifold survives the low-parallax video geometry the linear eight-point fit
                // degenerates on, and MSAC scores over the same correspondences are comparable, so
                // the better model simply wins across calls.
                let candidates = [
                    estimate_essential_five_point_bounded(&fallback.correspondences, &intrinsics, &intrinsics, FIVE_POINT_THRESHOLD, seed, INTERACTIVE_TWO_VIEW_HYPOTHESES_PER_CALL, 0),
                    estimate_essential_with_limit(&fallback.correspondences, &intrinsics, &intrinsics, INTERACTIVE_TWO_VIEW_HYPOTHESES_PER_CALL),
                ];
                for candidate in candidates.into_iter().flatten() {
                    if fallback.best.as_ref().is_none_or(|best| candidate.score < best.score) {
                        fallback.best = Some(candidate);
                    }
                }
                fallback.attempts += 1;
                if fallback.attempts < MAX_INTERACTIVE_TWO_VIEW_ATTEMPTS {
                    return Ok(false);
                }
                if let Some(best) = fallback.best.take() {
                    fallback.best = Some(polish_essential(best, &fallback.correspondences, &intrinsics, FIVE_POINT_THRESHOLD));
                }
                let relative = fallback.best.as_ref().and_then(|estimate| {
                    let TwoViewModel::Fundamental(essential) = estimate.model else { return None };
                    let rays: Vec<([f64; 2], [f64; 2])> = estimate
                        .inliers
                        .iter()
                        .take(MAX_INTERACTIVE_REGISTRATION_CORRESPONDENCES)
                        .map(|&index| {
                            let left = intrinsics.unproject_ray(fallback.correspondences[index].0);
                            let right = intrinsics.unproject_ray(fallback.correspondences[index].1);
                            ([left[0], left[1]], [right[0], right[1]])
                        })
                        .collect();
                    decompose_essential(&essential, &rays)
                });
                match relative {
                    Some(relative) => {
                        fallback.relative = Some(relative);
                        fallback.cursor = 0;
                        preparation.phase = RegistrationPhase::TwoViewScale;
                    }
                    None => preparation.phase = RegistrationPhase::Failed,
                }
            }
            RegistrationPhase::TwoViewScale => {
                let frame = preparation.frame;
                let limit = self.tracks.tracks.len().min(MAX_INTERACTIVE_TRACKS);
                for _ in 0..work_budget.max(1) {
                    let fallback = preparation.two_view.as_mut().expect("two-view fallback");
                    if fallback.cursor >= limit {
                        preparation.phase = RegistrationPhase::TwoViewCommit;
                        break;
                    }
                    let track_id = fallback.cursor;
                    fallback.cursor += 1;
                    let reference = fallback.reference;
                    let relative = fallback.relative.expect("solved relative pose");
                    let Some(&point) = self.points.get(&track_id) else { continue };
                    let track = &self.tracks.tracks[track_id];
                    let (Some(pixel_reference), Some(pixel_frame)) = (self.track_obs_in(track, reference), self.track_obs_in(track, frame)) else { continue };
                    let ray_reference = self.intrinsics.unproject_ray(pixel_reference);
                    let ray_frame = self.intrinsics.unproject_ray(pixel_frame);
                    let rotation = mat3d_to_array(&relative.r.0);
                    let Some(unit_point) = triangulate_normalized_pair(&rotation, relative.t, [ray_reference[0], ray_reference[1]], [ray_frame[0], ray_frame[1]]) else { continue };
                    let Some(reference_pose) = self.pose_of(reference) else { continue };
                    let metric_distance = norm3(sub3(point, camera_center(&reference_pose)));
                    let unit_distance = norm3(unit_point);
                    if unit_distance < 1e-9 || metric_distance < 1e-9 {
                        continue;
                    }
                    let fallback = preparation.two_view.as_mut().expect("two-view fallback");
                    if fallback.scales.len() < MAX_INTERACTIVE_REGISTRATION_CORRESPONDENCES {
                        fallback.scales.push(metric_distance / unit_distance);
                        fallback.shared.push((point, pixel_frame));
                    }
                }
            }
            RegistrationPhase::TwoViewCommit => {
                let frame = preparation.frame;
                let fallback = preparation.two_view.as_mut().expect("two-view fallback");
                let relative = fallback.relative.expect("solved relative pose");
                let Some(reference_pose) = self.pose_of(fallback.reference) else {
                    preparation.phase = RegistrationPhase::Failed;
                    return Ok(false);
                };
                let scale = if fallback.scales.is_empty() {
                    // 🏃️ Sparse-track path (JPEG/video matching left no shared triangulated point):
                    // borrow the seed pair's baseline so every chained two-view pose shares one
                    // metric, instead of abandoning the frame.
                    if self.cameras.len() < 2 {
                        preparation.phase = RegistrationPhase::Failed;
                        return Ok(false);
                    }
                    let baseline = norm3(sub3(camera_center(&self.cameras[0].1), camera_center(&self.cameras[1].1)));
                    if baseline < 1e-9 {
                        preparation.phase = RegistrationPhase::Failed;
                        return Ok(false);
                    }
                    baseline
                } else {
                    fallback.scales.sort_by(f64::total_cmp);
                    let median = fallback.scales[fallback.scales.len() / 2];
                    let spread = fallback.scales[fallback.scales.len() - 1] / fallback.scales[0];
                    if !median.is_finite() || median <= 1e-9 || !spread.is_finite() || (fallback.scales.len() >= 2 && spread > 1.5) {
                        preparation.phase = RegistrationPhase::Failed;
                        return Ok(false);
                    }
                    median
                };
                let pose = CameraPose(Se3 { r: relative.r, t: scale3(relative.t, scale) }.semio_compose_rs(&reference_pose.0));
                let maximum_reprojection = self.cfg.ransac_threshold_px * 3.0;
                let consistent = fallback.shared.iter().any(|&(point, pixel)| reproject(&self.intrinsics, &pose, point).is_some_and(|predicted| ((predicted[0] - pixel[0]).powi(2) + (predicted[1] - pixel[1]).powi(2)).sqrt() <= maximum_reprojection));
                if !fallback.shared.is_empty() && !consistent {
                    preparation.phase = RegistrationPhase::Failed;
                    return Ok(false);
                }
                self.cameras.push((frame, pose));
                preparation.two_view = None;
                preparation.cursor = 0;
                preparation.phase = RegistrationPhase::Triangulate;
            }
            RegistrationPhase::Triangulate => {
                for _ in 0..work_budget.max(1) {
                    if preparation.cursor >= self.tracks.tracks.len().min(MAX_INTERACTIVE_TRACKS) {
                        preparation.phase = RegistrationPhase::Done;
                        break;
                    }
                    self.triangulate_track(preparation.cursor, Some(preparation.frame));
                    preparation.cursor += 1;
                }
            }
            RegistrationPhase::Done => return Ok(true),
            RegistrationPhase::Failed => return Err(SfmError::PnpFailed),
        }
        Ok(preparation.phase == RegistrationPhase::Done)
    }

    /// 🎯️ Re-estimates every registered camera via PnP against the current triangulated cloud (frames with
    /// fewer than [`P3pSolver::SAMPLE_SIZE`] correspondences are left unchanged). Cleans up poses that were
    /// seeded by the two-view baseline-prior fallback before bundle adjustment / dense stereo.
    #[cfg(test)]
    pub fn refine_registered_poses_pnp(&mut self) {
        let frames: Vec<usize> = self.cameras.iter().map(|&(f, _)| f).collect();
        for frame in frames {
            let mut world_pts = Vec::new();
            let mut obs = Vec::new();
            for (track_id, track) in self.tracks.tracks.iter().enumerate() {
                let Some(&point) = self.points.get(&track_id) else { continue };
                let Some(px) = self.track_obs_in(track, frame) else { continue };
                world_pts.push(point);
                obs.push(px);
            }
            if world_pts.len() < P3pSolver::SAMPLE_SIZE {
                continue;
            }
            let cfg = RansacConfig { threshold: self.cfg.ransac_threshold_px, confidence: 0.999, max_iters: 2000, seed: frame as u64 ^ 0x9E37_79B9, scoring: RansacScoring::Msac };
            if let Some((pose, _)) = pnp_ransac(&self.intrinsics, &world_pts, &obs, &cfg) {
                if let Some(slot) = self.cameras.iter_mut().find(|(f, _)| *f == frame) {
                    slot.1 = pose;
                }
            }
        }
    }

    #[cfg(test)]
    pub fn triangulate_new(&mut self, frame: usize) {
        for track_id in 0..self.tracks.tracks.len() {
            self.triangulate_track(track_id, Some(frame));
        }
    }

    fn build_bundle_problem(&self, camera_frames: &[usize], fixed_frames: &[usize]) -> (SfmBundleProblem, Vec<VecD>, Vec<VecD>, Vec<usize>) {
        let a_index_of: std::collections::HashMap<usize, usize> = camera_frames.iter().enumerate().map(|(i, &f)| (f, i)).collect();
        let mut point_track_ids: Vec<usize> = Vec::new();
        let mut b_index_of: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
        let mut terms = Vec::new();
        let mut observations = std::collections::HashMap::new();
        for &track_id in self.points.keys() {
            for &(f, kp) in &self.tracks.tracks[track_id] {
                let Some(&ai) = a_index_of.get(&f) else { continue };
                let bi = *b_index_of.entry(track_id).or_insert_with(|| {
                    point_track_ids.push(track_id);
                    point_track_ids.len() - 1
                });
                terms.push(ResidualTerm { a_index: Some(ai), b_index: Some(bi), dim: 2 });
                observations.insert((ai, bi), self.obs_px(f, kp));
            }
        }
        let a0: Vec<VecD> = camera_frames
            .iter()
            .map(|&f| {
                let pose = self.pose_of(f).unwrap_or(CameraPose(Se3::identity()));
                VecD::from_vec(pose.0.log().to_vec())
            })
            .collect();
        let b0: Vec<VecD> = point_track_ids.iter().map(|&tid| VecD::from_vec(self.points[&tid].to_vec())).collect();
        let fixed_cameras = camera_frames.iter().map(|frame| fixed_frames.contains(frame)).collect();
        let problem = SfmBundleProblem { intrinsics: self.intrinsics, num_cameras: camera_frames.len(), num_points: point_track_ids.len(), terms, observations, fixed_cameras };
        (problem, a0, b0, point_track_ids)
    }

    /// 🎯️ Opens a bounded bundle adjustment over `camera_frames` (the `fixed_frames` among them held
    /// still) and the points they see: the problem is built once, the Levenberg–Marquardt iterations
    /// then run `term_budget` residual terms per [`Self::advance_bundle_adjustment`] call up to
    /// `max_iterations`, and the refined poses and points are written back when it completes.
    /// `None` when there is nothing to adjust.
    pub fn begin_bundle_adjustment(&self, camera_frames: &[usize], fixed_frames: &[usize], loss: RobustLoss, max_iterations: usize) -> Option<BundleAdjustment> {
        if camera_frames.is_empty() {
            return None;
        }
        let (problem, a0, b0, point_track_ids) = self.build_bundle_problem(camera_frames, fixed_frames);
        if problem.terms.is_empty() {
            return None;
        }
        let cfg = LmConfig { max_iters: max_iterations.max(1), loss, ..LmConfig::default() };
        let preparation = SchurLmPreparation::new(&problem, a0, b0, &cfg);
        Some(BundleAdjustment { frames: camera_frames.to_vec(), point_track_ids, problem, cfg, preparation })
    }

    /// 🎯️ One bounded unit of an open adjustment; `true` once it is done and written back.
    pub fn advance_bundle_adjustment(&mut self, adjustment: &mut BundleAdjustment, term_budget: usize) -> bool {
        if !adjustment.preparation.advance(&adjustment.problem, &adjustment.cfg, term_budget) {
            return false;
        }
        for (frame, a) in adjustment.frames.iter().zip(adjustment.preparation.a_params.iter()) {
            let xi: [f64; 6] = std::array::from_fn(|k| a.get(k));
            if let Some(entry) = self.cameras.iter_mut().find(|entry| entry.0 == *frame) {
                entry.1 = CameraPose(Se3::exp(xi));
            }
        }
        for (tid, b) in adjustment.point_track_ids.iter().zip(adjustment.preparation.b_params.iter()) {
            self.points.insert(*tid, [b.get(0), b.get(1), b.get(2)]);
        }
        true
    }

    /// 🎯️ The configured robust loss, the one a local adjustment uses.
    pub fn robust_loss(&self) -> RobustLoss {
        self.cfg.robust_loss
    }

    /// 🎯️ The frames a local bundle adjustment after the latest registration refines: the last
    /// `window` registered cameras in registration order.
    pub fn local_bundle_frames(&self, window: usize) -> Vec<usize> {
        let frames: Vec<usize> = self.cameras.iter().map(|&(f, _)| f).collect();
        let start = frames.len().saturating_sub(window);
        frames[start..].to_vec()
    }

    /// 🌐️ Every registered frame in registration order, for a global bundle adjustment.
    pub fn registered_frames(&self) -> Vec<usize> {
        self.cameras.iter().map(|&(f, _)| f).collect()
    }

    #[cfg(test)]
    fn run_bundle_adjustment(&mut self, camera_frames: &[usize]) {
        if camera_frames.is_empty() {
            return;
        }
        let (problem, a0, b0, point_track_ids) = self.build_bundle_problem(camera_frames, &[]);
        if problem.terms.is_empty() {
            return;
        }
        let cfg = LmConfig { max_iters: self.cfg.ba_max_iterations, loss: self.cfg.robust_loss, ..LmConfig::default() };
        let result: SchurResult = schur_lm(&problem, a0, b0, &cfg);
        for (frame, a) in camera_frames.iter().zip(result.a_params.iter()) {
            let xi: [f64; 6] = std::array::from_fn(|k| a.get(k));
            if let Some(entry) = self.cameras.iter_mut().find(|entry| entry.0 == *frame) {
                entry.1 = CameraPose(Se3::exp(xi));
            }
        }
        for (tid, b) in point_track_ids.iter().zip(result.b_params.iter()) {
            self.points.insert(*tid, [b.get(0), b.get(1), b.get(2)]);
        }
    }

    /// 🎯️ Bundle-adjusts the most-recently-registered `window` cameras (by frame insertion order) and every point they see.
    #[cfg(test)]
    pub fn local_ba(&mut self, window: usize) {
        let frames: Vec<usize> = self.cameras.iter().map(|&(f, _)| f).collect();
        let start = frames.len().saturating_sub(window);
        let window_frames = frames[start..].to_vec();
        self.run_bundle_adjustment(&window_frames);
    }

    /// 🌐️ Bundle-adjusts every registered camera and every triangulated point.
    #[cfg(test)]
    pub fn global_ba(&mut self) {
        let frames: Vec<usize> = self.cameras.iter().map(|&(f, _)| f).collect();
        self.run_bundle_adjustment(&frames);
    }

    /// 🧹️ Drops points whose worst-view reprojection error exceeds `3 * ransac_threshold_px` or whose
    /// best pairwise triangulation angle is below [`SfmConfig::min_triangulation_angle_rad`], then drops
    /// any camera left seeing fewer than [`SfmConfig::min_visible_points_to_keep_camera`] surviving points.
    #[cfg(test)]
    pub fn prune_outliers(&mut self) {
        let reproj_threshold = self.cfg.ransac_threshold_px * 3.0;
        let mut to_remove = Vec::new();
        for (&tid, &point) in &self.points {
            let track = &self.tracks.tracks[tid];
            let views: Vec<(CameraPose, [f64; 2])> = track.iter().filter_map(|&(f, kp)| self.pose_of(f).map(|pose| (pose, self.obs_px(f, kp)))).collect();
            if views.len() < 2 {
                to_remove.push(tid);
                continue;
            }
            let mut max_err = 0.0_f64;
            for (pose, px) in &views {
                match reproject(&self.intrinsics, pose, point) {
                    Some(pred) => max_err = max_err.max(((pred[0] - px[0]).powi(2) + (pred[1] - px[1]).powi(2)).sqrt()),
                    None => max_err = f64::MAX,
                }
            }
            let mut max_angle = 0.0_f64;
            for i in 0..views.len() {
                for j in (i + 1)..views.len() {
                    max_angle = max_angle.max(triangulation_angle(&views[i].0, &views[j].0, point));
                }
            }
            if max_err > reproj_threshold || max_angle < self.cfg.min_triangulation_angle_rad {
                to_remove.push(tid);
            }
        }
        for tid in to_remove {
            self.points.remove(&tid);
        }

        let min_visible = self.cfg.min_visible_points_to_keep_camera;
        let mut visible_count: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
        for &tid in self.points.keys() {
            for &(f, _) in &self.tracks.tracks[tid] {
                if self.is_registered(f) {
                    *visible_count.entry(f).or_insert(0) += 1;
                }
            }
        }
        self.cameras.retain(|&(f, _)| visible_count.get(&f).copied().unwrap_or(0) >= min_visible);
    }

    /// 🔁️ Re-runs [`triangulate_new`](Self::triangulate_new) over every registered frame, picking up
    /// tracks that a bundle-adjustment pass's refined poses may now connect.
    #[cfg(test)]
    pub fn retriangulate(&mut self) {
        let frames: Vec<usize> = self.cameras.iter().map(|&(f, _)| f).collect();
        for frame in frames {
            self.triangulate_new(frame);
        }
    }

    pub fn begin_bundle(&self) -> BundlePreparation {
        self.begin_bundle_with_tolerance(3.0)
    }

    /// 🧹️ [`Self::begin_bundle`] pruning at `tolerance` times the RANSAC threshold instead of three:
    /// the cleanup that precedes a global adjustment keeps the loop-closure observations — the ones
    /// a drifted chain reprojects tens of pixels off, and the only ones that can pull the loop shut —
    /// by pruning only gross outliers; the cleanup after it prunes at the strict tolerance.
    pub fn begin_bundle_with_tolerance(&self, tolerance: f64) -> BundlePreparation {
        let mut point_track_ids: Vec<usize> = self.points.keys().copied().take(MAX_INTERACTIVE_TRACKS).collect();
        point_track_ids.sort_unstable();
        BundlePreparation { point_track_ids, cursor: 0, phase: BundlePhase::Prune, tolerance }
    }

    /// 🎯️ Incremental robust bundle cleanup: one triangulated point's complete bounded observation
    /// residual set or one track retriangulation per fuel unit. The admitted engine envelope caps both
    /// tracks and cameras, so each inner residual set is finite and independently timed by tests.
    pub fn advance_bundle(&mut self, preparation: &mut BundlePreparation, work_budget: usize) -> bool {
        match preparation.phase {
            BundlePhase::Prune => {
                for _ in 0..work_budget.max(1) {
                    if preparation.cursor >= preparation.point_track_ids.len() {
                        preparation.cursor = 0;
                        preparation.phase = BundlePhase::Retriangulate;
                        break;
                    }
                    let track_id = preparation.point_track_ids[preparation.cursor];
                    preparation.cursor += 1;
                    let Some(&point) = self.points.get(&track_id) else { continue };
                    let Some(track) = self.tracks.tracks.get(track_id) else { continue };
                    let mut maximum = 0.0f64;
                    let mut visible = 0usize;
                    for &(frame, keypoint) in track.iter().take(MAX_INTERACTIVE_TRACK_OBSERVATIONS) {
                        let Some(pose) = self.pose_of(frame) else { continue };
                        visible += 1;
                        let observed = self.obs_px(frame, keypoint);
                        let Some(predicted) = reproject(&self.intrinsics, &pose, point) else {
                            maximum = f64::INFINITY;
                            break;
                        };
                        maximum = maximum.max(((predicted[0] - observed[0]).powi(2) + (predicted[1] - observed[1]).powi(2)).sqrt());
                    }
                    if visible < 2 || maximum > self.cfg.ransac_threshold_px * preparation.tolerance {
                        self.points.remove(&track_id);
                        if let Some(events) = self.point_events.as_mut() {
                            events.push((track_id, point, false));
                        }
                    }
                }
            }
            BundlePhase::Retriangulate => {
                for _ in 0..work_budget.max(1) {
                    if preparation.cursor >= self.tracks.tracks.len().min(MAX_INTERACTIVE_TRACKS) {
                        preparation.phase = BundlePhase::Done;
                        break;
                    }
                    self.triangulate_track(preparation.cursor, None);
                    preparation.cursor += 1;
                }
            }
            BundlePhase::Done => {}
        }
        preparation.phase == BundlePhase::Done
    }

    pub fn begin_reconstruction_snapshot(&self) -> ReconstructionSnapshotPreparation {
        let mut point_track_ids: Vec<usize> = self.points.keys().copied().take(MAX_INTERACTIVE_TRACKS).collect();
        point_track_ids.sort_unstable();
        ReconstructionSnapshotPreparation {
            camera_cursor: 0,
            point_cursor: 0,
            reconstruction: Reconstruction { cameras: Vec::with_capacity(self.cameras.len()), points: Vec::with_capacity(point_track_ids.len()), point_track_ids: Vec::with_capacity(point_track_ids.len()), intrinsics: self.intrinsics },
            point_track_ids,
        }
    }

    pub fn advance_reconstruction_snapshot(&self, preparation: &mut ReconstructionSnapshotPreparation, item_budget: usize) -> bool {
        let mut remaining = item_budget.max(1);
        while preparation.camera_cursor < self.cameras.len() && remaining > 0 {
            preparation.reconstruction.cameras.push(self.cameras[preparation.camera_cursor]);
            preparation.camera_cursor += 1;
            remaining -= 1;
        }
        while preparation.point_cursor < preparation.point_track_ids.len() && remaining > 0 {
            let track_id = preparation.point_track_ids[preparation.point_cursor];
            if let Some(&point) = self.points.get(&track_id) {
                preparation.reconstruction.points.push(point);
                preparation.reconstruction.point_track_ids.push(track_id);
            }
            preparation.point_cursor += 1;
            remaining -= 1;
        }
        preparation.camera_cursor == self.cameras.len() && preparation.point_cursor == preparation.point_track_ids.len()
    }

    pub fn finish_reconstruction_snapshot(mut preparation: ReconstructionSnapshotPreparation) -> Reconstruction {
        preparation.reconstruction.cameras.sort_by_key(|&(frame, _)| frame);
        preparation.reconstruction
    }

    /// 📦️ A snapshot of the current registered cameras and triangulated points.
    #[cfg(test)]
    pub fn reconstruction(&self) -> Reconstruction {
        let mut cameras = self.cameras.clone();
        cameras.sort_by_key(|&(f, _)| f);
        let mut point_track_ids: Vec<usize> = self.points.keys().copied().collect();
        point_track_ids.sort_unstable();
        let points = point_track_ids.iter().map(|tid| self.points[tid]).collect();
        Reconstruction { cameras, points, point_track_ids, intrinsics: self.intrinsics }
    }

    /// 🚀️ Convenience end-to-end driver: [`init_pair`](Self::init_pair) on the first two frames of
    /// `frame_order`, then for each subsequent frame `register_next` -> `triangulate_new` -> `local_ba`
    /// (window 5) -> `prune_outliers` -> `retriangulate`, finishing with one [`global_ba`](Self::global_ba).
    /// A frame that fails to register (e.g. too few 2D-3D correspondences yet) is skipped rather than
    /// aborting the whole run — a documented best-effort policy, since one poorly-connected frame
    /// shouldn't sink an otherwise-good reconstruction.
    #[cfg(test)]
    pub fn run_all(&mut self, frame_order: &[usize], pairwise_matches: &[(usize, usize, Vec<Match>)]) -> Result<Reconstruction, SfmError> {
        if frame_order.len() < 2 {
            return Err(SfmError::TooFewFrames);
        }
        let (f0, f1) = (frame_order[0], frame_order[1]);
        let matches01 = pairwise_matches.iter().find(|&&(a, b, _)| a == f0 && b == f1).map(|(_, _, m)| m).ok_or(SfmError::InsufficientMatches)?;
        self.set_pairwise_matches(pairwise_matches.to_vec());
        self.init_pair(f0, f1, matches01)?;

        for _ in 0..frame_order.len().saturating_sub(2) {
            let mut candidates: Vec<(usize, usize)> = frame_order.iter().copied().filter(|&frame| !self.is_registered(frame)).map(|frame| (self.pnp_correspondence_count(frame), frame)).collect();
            candidates.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
            let mut registered_any = false;
            for (_corrs, frame) in candidates {
                if self.register_next(frame).is_ok() {
                    self.triangulate_new(frame);
                    self.local_ba(5);
                    self.prune_outliers();
                    self.retriangulate();
                    registered_any = true;
                    break;
                }
            }
            if !registered_any {
                break;
            }
        }
        self.global_ba();
        Ok(self.reconstruction())
    }
}

// #endregion 🔖️Incremental

// #region 🔖️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

// #endregion 🔖️Tests
