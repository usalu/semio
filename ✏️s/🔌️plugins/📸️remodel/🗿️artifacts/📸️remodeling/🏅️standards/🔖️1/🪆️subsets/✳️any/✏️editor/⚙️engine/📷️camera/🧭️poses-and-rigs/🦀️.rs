//! 🧭️ Poses and rigs of the camera model: the extrinsics, the rolling-shutter model and the multi-camera rig with its refinement. See [`super`].

use super::*;

// #region 🔖️Extrinsics
/// 🎥️ World-to-camera rigid transform: `p_cam = pose.0.act(p_world)`.
#[derive(Clone, Copy, Debug, PartialEq, semio_framework_value::RetireOwned)]
pub struct CameraPose(pub Se3);

pub(super) fn se3_at(x: &VecD, offset: usize) -> Se3 {
    Se3::exp(std::array::from_fn(|k| x.get(offset + k)))
}

pub(super) fn retract_se3_block(x: &VecD, dx: &VecD, offset: usize) -> [f64; 6] {
    let pose = se3_at(x, offset);
    let dxi: [f64; 6] = std::array::from_fn(|k| dx.get(offset + k));
    Se3::exp(dxi).semio_compose_rs(&pose).log()
}

/// 🎯️ Transforms a world point by a camera pose and projects it through the camera's intrinsics.
pub fn reproject(intr: &Intrinsics, pose: &CameraPose, point_world: [f64; 3]) -> Option<[f64; 2]> {
    intr.project(pose.0.act(point_world))
}

/// 🎯️ Reprojection residual `predicted_pixel - observed_pixel` — the shared kernel behind bundle
/// adjustment, PnP and calibration in this crate and downstream (`remodeling_sfm`). Points behind the camera
/// (`z <= 0`) fall back to a large constant residual so callers never see `NaN` or a discontinuous jump.
pub fn reprojection_residual(intrinsics: &Intrinsics, pose: &CameraPose, point_world: [f64; 3], observed_pixel: [f64; 2]) -> [f64; 2] {
    match reproject(intrinsics, pose, point_world) {
        Some(pred) => [pred[0] - observed_pixel[0], pred[1] - observed_pixel[1]],
        None => [1.0e3, 1.0e3],
    }
}

/// 🔬️ Analytic Jacobians of [`reprojection_residual`], returned as `(d/d(pose tangent) 2x6, d/d(point) 2x3,
/// d/d(intrinsics) 2xK)`. The pose block is w.r.t. the camera's SE(3) *left*-perturbation tangent
/// `(rho, phi)` (matching how [`CameraPose`]-carrying least-squares problems in this crate retract via
/// `Se3::exp(dxi).semio_compose_rs(pose)`): since `p_cam ↦ p_cam + rho + phi × p_cam` to first order, `d(p_cam)/d(xi)
/// = [I₃ | -hat(p_cam)]`. The intrinsics block is ordered `[fx, fy, cx, cy, skew, <distortion params>]`
/// (`K = 5 + distortion.param_count()`), chaining the linear intrinsic map through
/// [`Distortion::distort_with_jacobian`]. Points behind the camera return all-zero Jacobians, matching
/// [`reprojection_residual`]'s constant-residual fallback so a damped LM step simply ignores the observation.
pub fn reprojection_jacobians(intrinsics: &Intrinsics, pose: &CameraPose, point_world: [f64; 3], _observed_pixel: [f64; 2]) -> (MatD, MatD, MatD) {
    let k = 5 + intrinsics.distortion.param_count();
    let mut j_pose = MatD::zeros(2, 6);
    let mut j_point = MatD::zeros(2, 3);
    let mut j_intr = MatD::zeros(2, k);

    let p_cam = pose.0.act(point_world);
    let pz = p_cam[2];
    if pz <= 1e-9 {
        return (j_pose, j_point, j_intr);
    }
    let (px, py) = (p_cam[0], p_cam[1]);
    let (x, y) = (px / pz, py / pz);

    let j_proj: [[f64; 3]; 2] = [[1.0 / pz, 0.0, -px / (pz * pz)], [0.0, 1.0 / pz, -py / (pz * pz)]];
    let (distorted, j_distort, param_derivs) = intrinsics.distortion.distort_with_jacobian([x, y]);
    let (xd, yd) = (distorted[0], distorted[1]);
    let j_lin: [[f64; 2]; 2] = [[intrinsics.fx, intrinsics.skew], [0.0, intrinsics.fy]];

    let j_uv_xy = mat2_mul(&j_lin, &j_distort);
    let j_uv_pcam = mat2x2_mul_2x3(&j_uv_xy, &j_proj);

    let neg_hat_pcam = skew3(scale3(p_cam, -1.0));
    let j_uv_phi = mat2x3_mul_3x3(&j_uv_pcam, &neg_hat_pcam);
    for row in 0..2 {
        for col in 0..3 {
            j_pose.set(row, col, j_uv_pcam[row][col]);
            j_pose.set(row, col + 3, j_uv_phi[row][col]);
        }
    }

    let j_uv_point = mat2x3_mul_3x3(&j_uv_pcam, &mat3d_rowmajor(&pose.0.r.0));
    for (row, cols) in j_uv_point.iter().enumerate() {
        for (col, &value) in cols.iter().enumerate() {
            j_point.set(row, col, value);
        }
    }

    j_intr.set(0, 0, xd);
    j_intr.set(0, 2, 1.0);
    j_intr.set(0, 4, yd);
    j_intr.set(1, 1, yd);
    j_intr.set(1, 3, 1.0);
    for (i, dparam) in param_derivs.iter().enumerate() {
        let (dxd, dyd) = (dparam[0], dparam[1]);
        j_intr.set(0, 5 + i, intrinsics.fx * dxd + intrinsics.skew * dyd);
        j_intr.set(1, 5 + i, intrinsics.fy * dyd);
    }

    (j_pose, j_point, j_intr)
}

/// 🧩️ Multi-camera multi-point reprojection least-squares problem: the shared residual kernel reused by
/// bundle adjustment, PnP and calibration solvers built on `remodeling_camera`. The parameter vector packs an
/// SE(3) log-tangent per camera (6 values, interpreted as `Se3::exp(tangent)`) followed by an XYZ position
/// per point (3 values); `intrinsics` are fixed and shared across every camera and observation.
#[derive(Clone, Debug)]
pub struct ReprojectionProblem {
    pub observations: Vec<(usize, usize, [f64; 2])>,
    pub num_cameras: usize,
    pub num_points: usize,
    pub intrinsics: Intrinsics,
}

impl ReprojectionProblem {
    fn point_offset(&self, point_idx: usize) -> usize {
        self.num_cameras * 6 + point_idx * 3
    }

    fn point_at(&self, x: &VecD, point_idx: usize) -> [f64; 3] {
        let base = self.point_offset(point_idx);
        [x.get(base), x.get(base + 1), x.get(base + 2)]
    }
}

impl LeastSquaresProblem for ReprojectionProblem {
    fn residual_count(&self) -> usize {
        self.observations.len() * 2
    }

    fn parameter_count(&self) -> usize {
        self.num_cameras * 6 + self.num_points * 3
    }

    fn residuals(&self, x: &VecD, out: &mut VecD) {
        for (row, &(cam_idx, point_idx, obs)) in self.observations.iter().enumerate() {
            let pose = CameraPose(se3_at(x, cam_idx * 6));
            let point = self.point_at(x, point_idx);
            let pred = reproject(&self.intrinsics, &pose, point).unwrap_or([obs[0] + 1.0e3, obs[1] + 1.0e3]);
            out.set(2 * row, pred[0] - obs[0]);
            out.set(2 * row + 1, pred[1] - obs[1]);
        }
    }

    /// 🔬️ Analytic Jacobians assembled per-observation from [`reprojection_jacobians`]'s pose/point blocks
    /// (intrinsics are fixed here, so its intrinsics block is discarded), scattered into this problem's
    /// per-camera/per-point column ranges.
    fn jacobian(&self, x: &VecD, out: &mut MatD) {
        for (row, &(cam_idx, point_idx, obs)) in self.observations.iter().enumerate() {
            let pose = CameraPose(se3_at(x, cam_idx * 6));
            let point = self.point_at(x, point_idx);
            let (j_pose, j_point, _j_intr) = reprojection_jacobians(&self.intrinsics, &pose, point, obs);
            let point_base = self.point_offset(point_idx);
            for r in 0..2 {
                for c in 0..6 {
                    out.set(2 * row + r, cam_idx * 6 + c, j_pose.get(r, c));
                }
                for c in 0..3 {
                    out.set(2 * row + r, point_base + c, j_point.get(r, c));
                }
            }
        }
    }

    fn plus(&self, x: &VecD, dx: &VecD) -> VecD {
        let mut out = VecD::zeros(x.len());
        for cam_idx in 0..self.num_cameras {
            let updated = retract_se3_block(x, dx, cam_idx * 6);
            for (k, &v) in updated.iter().enumerate() {
                out.set(cam_idx * 6 + k, v);
            }
        }
        let base = self.num_cameras * 6;
        for i in base..x.len() {
            out.set(i, x.get(i) + dx.get(i));
        }
        out
    }
}
// #endregion 🔖️Extrinsics

// #region 🔖️RollingShutter
/// 📐️↕️ Rolling-shutter readout model: per-row time delay and scan direction.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RollingShutterModel {
    pub line_delay_s: f64,
    pub readout: ReadoutDirection,
}

/// ↕️ Which way the sensor is scanned during rolling-shutter readout.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReadoutDirection {
    TopToBottom,
    BottomToTop,
}

fn scale6(xi: [f64; 6], s: f64) -> [f64; 6] {
    std::array::from_fn(|k| xi[k] * s)
}

/// 🎞️ Camera pose at a given sensor row under a linearized constant-velocity rolling-shutter model: the
/// row's readout time `t = effective_row * line_delay_s` (top-to-bottom counts rows directly, bottom-to-top
/// counts from the last row) integrates `velocity_se3` on top of `pose0` via the SE(3) exponential.
pub fn pose_at_row(model: &RollingShutterModel, row: u32, image_height: u32, pose0: &CameraPose, velocity_se3: [f64; 6]) -> CameraPose {
    let last_row = image_height.saturating_sub(1);
    let clamped_row = row.min(last_row);
    let effective_row = match model.readout {
        ReadoutDirection::TopToBottom => clamped_row,
        ReadoutDirection::BottomToTop => last_row - clamped_row,
    };
    let t = effective_row as f64 * model.line_delay_s;
    CameraPose(Se3::exp(scale6(velocity_se3, t)).semio_compose_rs(&pose0.0))
}

/// 🗺️ Per-pixel rolling-shutter rectification field: for every output pixel `(col, row)`, interpreted
/// under the reference pose `pose0`'s geometry, finds the source pixel in the *actual* rolling-shutter
/// image a consumer (e.g. `remodeling_image::remap`) should sample to undo the skew. Direction-only: the
/// world-frame bearing of the output pixel is held fixed and re-projected through each candidate sensor
/// row's own pose (bounded 3-step fixed-point search, since "which row" and "what pose that row had" are
/// mutually dependent), ignoring translational parallax — the standard rolling-shutter-without-depth
/// approximation, exact for rotation-dominated motion. Pixels that never reproject in front of the camera
/// fall back to their own (unchanged) coordinate.
pub fn rectify_remap_field(intrinsics: &Intrinsics, model: &RollingShutterModel, pose0: &CameraPose, velocity_se3: [f64; 6], width: u32, height: u32) -> Vec<[f32; 2]> {
    let mut field = vec![[0.0f32; 2]; width as usize * height as usize];
    for row in 0..height {
        for col in 0..width {
            let idx = row as usize * width as usize + col as usize;
            let px = [col as f64 + 0.5, row as f64 + 0.5];
            let ray_cam0 = intrinsics.unproject_ray(px);
            let ray_world = pose0.0.r.inverse().act(ray_cam0);
            let mut src_row = row;
            let mut source_px = px;
            for _ in 0..3 {
                let pose_row = pose_at_row(model, src_row, height, pose0, velocity_se3);
                let ray_local = pose_row.0.r.act(ray_world);
                if ray_local[2] <= 1e-9 {
                    source_px = px;
                    break;
                }
                source_px = intrinsics.project(ray_local).unwrap_or(px);
                let next_row = (source_px[1].round().clamp(0.0, height.saturating_sub(1) as f64)) as u32;
                if next_row == src_row {
                    break;
                }
                src_row = next_row;
            }
            field[idx] = [source_px[0] as f32, source_px[1] as f32];
        }
    }
    field
}
// #endregion 🔖️RollingShutter

// #region 🔖️Rig
/// 🧷️ One camera's fixed extrinsic pose within a multi-camera rig, relative to the rig's own reference
/// frame: `p_camera = pose_in_rig.act(p_rig)`.
#[derive(Clone, Debug, PartialEq, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct RigExtrinsic {
    pub camera_id: String,
    pub pose_in_rig: Se3,
}

/// 🧷️ Multi-camera rig: one fixed extrinsic per camera, relative to the rig's own reference frame.
#[derive(Clone, Debug, PartialEq)]
pub struct CameraRig {
    pub cameras: Vec<RigExtrinsic>,
}

/// 🔗️ Composes a camera's fixed rig-relative pose with the rig's world pose (`rig_pose`: world-to-rig)
/// into that camera's world-to-camera transform.
pub fn rig_pose_of_camera(rig_pose: &Se3, camera_in_rig: &Se3) -> Se3 {
    camera_in_rig.semio_compose_rs(rig_pose)
}

/// 📡️ Projects a world point through one rig camera via [`rig_pose_of_camera`], then reprojects. `None`
/// if `camera_id` isn't in the rig or the point is behind the camera.
pub fn rig_project(rig: &CameraRig, intrinsics: &[Intrinsics], camera_id: &str, rig_pose: &CameraPose, point_world: [f64; 3]) -> Option<[f64; 2]> {
    let idx = rig.cameras.iter().position(|c| c.camera_id == camera_id)?;
    let world_to_camera = rig_pose_of_camera(&rig_pose.0, &rig.cameras[idx].pose_in_rig);
    reproject(intrinsics.get(idx)?, &CameraPose(world_to_camera), point_world)
}

/// 📡️ One 2D observation feeding [`refine_rig`]: a known world point seen by one rig camera at one
/// shared-timestamp rig instance.
#[derive(Clone, Copy, Debug)]
pub struct RigObservation {
    pub instance_idx: usize,
    pub camera_idx: usize,
    pub point_world: [f64; 3],
    pub pixel: [f64; 2],
}

/// 📦️ Outcome of [`refine_rig`]: refined per-camera rig extrinsics (camera 0 fixed at its initial
/// `pose_in_rig`, the rig's frame anchor) and refined per-instance world-to-rig poses.
#[derive(Clone, Debug)]
pub struct RigRefinementResult {
    pub cameras: Vec<RigExtrinsic>,
    pub rig_poses: Vec<Se3>,
    pub rms_px: f64,
    pub converged: bool,
}

/// 🧩️ Multi-camera rig bundle-style refinement problem: `(num_cameras - 1)` free camera-in-rig SE(3)
/// tangents (camera 0 is the fixed rig-frame anchor) followed by `num_instances` free world-to-rig SE(3)
/// tangents. Since `world_to_camera = camera_in_rig ∘ rig_pose`, a left-perturbation `da` of `camera_in_rig`
/// acts directly on `world_to_camera`'s tangent, while a left-perturbation `db` of `rig_pose` acts through
/// `camera_in_rig`'s adjoint (`exp(da) ∘ A ∘ exp(db) ∘ B = exp(da + Adj_A(db)) ∘ A ∘ B` to first order) — so
/// [`Se3::adjoint`] turns [`reprojection_jacobians`]'s single pose block into both halves without needing a
/// second hand-derived chain rule.
struct RigRefinementProblem<'a> {
    pub(super) intrinsics: &'a [Intrinsics],
    pub(super) observations: &'a [RigObservation],
    pub(super) num_cameras: usize,
    pub(super) num_instances: usize,
    camera0_pose_in_rig: Se3,
}

impl RigRefinementProblem<'_> {
    fn camera_in_rig(&self, x: &VecD, camera_idx: usize) -> Se3 {
        if camera_idx == 0 {
            self.camera0_pose_in_rig
        } else {
            se3_at(x, (camera_idx - 1) * 6)
        }
    }

    fn rig_instance_base(&self) -> usize {
        (self.num_cameras - 1) * 6
    }
}

impl LeastSquaresProblem for RigRefinementProblem<'_> {
    fn residual_count(&self) -> usize {
        self.observations.len() * 2
    }

    fn parameter_count(&self) -> usize {
        (self.num_cameras - 1) * 6 + self.num_instances * 6
    }

    fn residuals(&self, x: &VecD, out: &mut VecD) {
        let base = self.rig_instance_base();
        for (row, obs) in self.observations.iter().enumerate() {
            let camera_in_rig = self.camera_in_rig(x, obs.camera_idx);
            let rig_pose = se3_at(x, base + obs.instance_idx * 6);
            let world_to_camera = rig_pose_of_camera(&rig_pose, &camera_in_rig);
            let r = reprojection_residual(&self.intrinsics[obs.camera_idx], &CameraPose(world_to_camera), obs.point_world, obs.pixel);
            out.set(2 * row, r[0]);
            out.set(2 * row + 1, r[1]);
        }
    }

    fn jacobian(&self, x: &VecD, out: &mut MatD) {
        let base = self.rig_instance_base();
        for (row, obs) in self.observations.iter().enumerate() {
            let camera_in_rig = self.camera_in_rig(x, obs.camera_idx);
            let rig_pose = se3_at(x, base + obs.instance_idx * 6);
            let world_to_camera = rig_pose_of_camera(&rig_pose, &camera_in_rig);
            let (j_pose, _j_point, _j_intr) = reprojection_jacobians(&self.intrinsics[obs.camera_idx], &CameraPose(world_to_camera), obs.point_world, obs.pixel);
            if obs.camera_idx != 0 {
                let camera_base = (obs.camera_idx - 1) * 6;
                for r in 0..2 {
                    for c in 0..6 {
                        out.set(2 * row + r, camera_base + c, j_pose.get(r, c));
                    }
                }
            }
            let adjoint = camera_in_rig.adjoint();
            let j_db = j_pose.matmul(&adjoint);
            let instance_base = base + obs.instance_idx * 6;
            for r in 0..2 {
                for c in 0..6 {
                    out.set(2 * row + r, instance_base + c, j_db.get(r, c));
                }
            }
        }
    }

    fn plus(&self, x: &VecD, dx: &VecD) -> VecD {
        let mut out = VecD::zeros(x.len());
        for i in 0..(self.num_cameras - 1) {
            let updated = retract_se3_block(x, dx, i * 6);
            for (k, &v) in updated.iter().enumerate() {
                out.set(i * 6 + k, v);
            }
        }
        let base = self.rig_instance_base();
        for i in 0..self.num_instances {
            let updated = retract_se3_block(x, dx, base + i * 6);
            for (k, &v) in updated.iter().enumerate() {
                out.set(base + i * 6 + k, v);
            }
        }
        out
    }
}

/// 🧷️ Jointly refines a multi-camera rig's per-camera extrinsics and per-instance world-to-rig poses from
/// shared-timestamp observations of known world points, via Levenberg-Marquardt (Huber-robustified) over
/// the shared [`reprojection_residual`]. Camera 0's `pose_in_rig` is held fixed at its initial value — it
/// defines the rig's own reference frame, so refining it too would leave the whole rig gauge-free and the
/// normal equations singular. Requires at least one camera, one rig instance and one observation.
pub fn refine_rig(intrinsics: &[Intrinsics], initial: &CameraRig, initial_rig_poses: &[Se3], observations: &[RigObservation]) -> Result<RigRefinementResult, CameraError> {
    let num_cameras = initial.cameras.len();
    let num_instances = initial_rig_poses.len();
    if num_cameras == 0 || num_instances == 0 || observations.is_empty() {
        return Err(CameraError::TooFewViews);
    }
    let problem = RigRefinementProblem { intrinsics, observations, num_cameras, num_instances, camera0_pose_in_rig: initial.cameras[0].pose_in_rig };
    let base = problem.rig_instance_base();
    let mut x0 = VecD::zeros(base + num_instances * 6);
    for i in 1..num_cameras {
        for (k, v) in initial.cameras[i].pose_in_rig.log().into_iter().enumerate() {
            x0.set((i - 1) * 6 + k, v);
        }
    }
    for (i, pose) in initial_rig_poses.iter().enumerate() {
        for (k, v) in pose.log().into_iter().enumerate() {
            x0.set(base + i * 6 + k, v);
        }
    }
    let cfg = LmConfig { max_iters: 100, loss: RobustLoss::Huber(2.0), ..LmConfig::default() };
    let residual_count = problem.residual_count();
    let result = levenberg_marquardt(&problem, x0, &cfg);
    if !result.converged {
        return Err(CameraError::Convergence);
    }
    let mut cameras = vec![initial.cameras[0].clone()];
    for i in 1..num_cameras {
        cameras.push(RigExtrinsic { camera_id: initial.cameras[i].camera_id.clone(), pose_in_rig: se3_at(&result.x, (i - 1) * 6) });
    }
    let rig_poses: Vec<Se3> = (0..num_instances).map(|i| se3_at(&result.x, base + i * 6)).collect();
    let rms_px = (4.0 * result.cost / residual_count as f64).sqrt();
    Ok(RigRefinementResult { cameras, rig_poses, rms_px, converged: result.converged })
}
// #endregion 🔖️Rig
