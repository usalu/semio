//! 📍️ Triangulation and perspective-n-point of the multi-view geometry: the linear and refined triangulation, the P3P, EPnP and RANSAC solvers and the pose refinement. See [`super`].

use super::*;

// #region 🔖️Triangulate
/// 📐️ `n`-view (`n >= 2`) linear DLT triangulation: unprojects each pixel observation to a normalized
/// ray via [`Intrinsics::unproject_ray`], stacks the `2n x 4` homogeneous system from each view's
/// `[R|t]` projection matrix, and solves for the nullspace via [`svd_nullvector`].
pub fn triangulate_dlt(poses: &[(CameraPose, Intrinsics)], obs_px: &[[f64; 2]]) -> Option<[f64; 3]> {
    if poses.len() < 2 || poses.len() != obs_px.len() {
        return None;
    }
    let mut design = MatD::zeros(2 * poses.len(), 4);
    for (i, (item, &px)) in poses.iter().zip(obs_px.iter()).enumerate() {
        let (pose, intr) = item;
        let ray = intr.unproject_ray(px);
        let r = mat3d_to_array(&pose.0.r.0);
        let t = pose.0.t;
        let row0 = [r[0][0], r[0][1], r[0][2], t[0]];
        let row1 = [r[1][0], r[1][1], r[1][2], t[1]];
        let row2 = [r[2][0], r[2][1], r[2][2], t[2]];
        for c in 0..4 {
            design.set(2 * i, c, ray[0] * row2[c] - row0[c]);
            design.set(2 * i + 1, c, ray[1] * row2[c] - row1[c]);
        }
    }
    let sol = svd_nullvector(&design).ok()?;
    let w = sol.get(3);
    if w.abs() < 1e-12 {
        return None;
    }
    Some([sol.get(0) / w, sol.get(1) / w, sol.get(2) / w])
}

/// 🧩️ Single-point bundle-refinement problem: the 3 unknowns are a point's world XYZ, poses/intrinsics are fixed.
struct PointRefineProblem<'a> {
    pub(super) poses: &'a [(CameraPose, Intrinsics)],
    obs_px: &'a [[f64; 2]],
}

impl LeastSquaresProblem for PointRefineProblem<'_> {
    fn residual_count(&self) -> usize {
        self.obs_px.len() * 2
    }

    fn parameter_count(&self) -> usize {
        3
    }

    fn residuals(&self, x: &VecD, out: &mut VecD) {
        let point = [x.get(0), x.get(1), x.get(2)];
        for (row, (item, &obs)) in self.poses.iter().zip(self.obs_px.iter()).enumerate() {
            let (pose, intr) = item;
            let pred = reproject(intr, pose, point).unwrap_or([obs[0] + 1.0e3, obs[1] + 1.0e3]);
            out.set(2 * row, pred[0] - obs[0]);
            out.set(2 * row + 1, pred[1] - obs[1]);
        }
    }

    fn jacobian(&self, x: &VecD, out: &mut MatD) {
        numeric_jacobian(self, x, 1e-6, out);
    }
}

/// 📐️ Refines a triangulated point's XYZ via Levenberg-Marquardt on its reprojection error, poses and intrinsics held fixed.
pub fn refine_point_lm(poses: &[(CameraPose, Intrinsics)], obs_px: &[[f64; 2]], initial: [f64; 3]) -> [f64; 3] {
    let problem = PointRefineProblem { poses, obs_px };
    let mut x0 = VecD::zeros(3);
    x0.set(0, initial[0]);
    x0.set(1, initial[1]);
    x0.set(2, initial[2]);
    let cfg = LmConfig { max_iters: 8, ..LmConfig::default() };
    let result = semio_s_artifact_remodel_remodeling_optimize::standards::v1::subsets::any::schema::optimize_internals::levenberg_marquardt(&problem, x0, &cfg);
    [result.x.get(0), result.x.get(1), result.x.get(2)]
}

/// 📐️ Angle in radians between the two viewing rays from `pose_a` and `pose_b`'s optical centers to `point`.
pub fn triangulation_angle(pose_a: &CameraPose, pose_b: &CameraPose, point: [f64; 3]) -> f64 {
    let ca = camera_center(pose_a);
    let cb = camera_center(pose_b);
    let da = vec3d_normalize(vec3d_sub(point, ca));
    let db = vec3d_normalize(vec3d_sub(point, cb));
    dot3(da, db).clamp(-1.0, 1.0).acos()
}

/// 📐️ Triangulates via [`triangulate_dlt`] + [`refine_point_lm`], then rejects the result unless every
/// pairwise viewing-ray angle is at least `min_angle_rad`, every view's reprojection error is at most
/// `max_reproj_err_px`, and every view sees the point in front of its camera (folded into the
/// reprojection check via [`reproject`]'s own cheirality test).
///
/// 🩹️ One wrong observation in a track of three or more does not sink the track: when the
/// full-track estimate is rejected by some views but at least two views agree with it, the point is
/// re-triangulated from the agreeing views alone and validated strictly on them (one round).
pub fn triangulate_and_validate(poses: &[(CameraPose, Intrinsics)], obs_px: &[[f64; 2]], min_angle_rad: f64, max_reproj_err_px: f64) -> Option<[f64; 3]> {
    fn validate(poses: &[(CameraPose, Intrinsics)], obs_px: &[[f64; 2]], min_angle_rad: f64, max_reproj_err_px: f64) -> Result<[f64; 3], Vec<usize>> {
        let initial = triangulate_dlt(poses, obs_px).ok_or_else(Vec::new)?;
        let refined = refine_point_lm(poses, obs_px, initial);
        let mut max_angle = 0.0_f64;
        for i in 0..poses.len() {
            for j in (i + 1)..poses.len() {
                max_angle = max_angle.max(triangulation_angle(&poses[i].0, &poses[j].0, refined));
            }
        }
        if max_angle < min_angle_rad {
            return Err(Vec::new());
        }
        let agreeing: Vec<usize> = poses
            .iter()
            .zip(obs_px.iter())
            .enumerate()
            .filter(|(_, ((pose, intr), &obs))| reproject(intr, pose, refined).is_some_and(|pred| ((pred[0] - obs[0]).powi(2) + (pred[1] - obs[1]).powi(2)).sqrt() <= max_reproj_err_px))
            .map(|(index, _)| index)
            .collect();
        if agreeing.len() == poses.len() { Ok(refined) } else { Err(agreeing) }
    }
    match validate(poses, obs_px, min_angle_rad, max_reproj_err_px) {
        Ok(point) => Some(point),
        Err(agreeing) if agreeing.len() >= 2 && agreeing.len() < poses.len() => {
            let subset_poses: Vec<(CameraPose, Intrinsics)> = agreeing.iter().map(|&index| poses[index]).collect();
            let subset_obs: Vec<[f64; 2]> = agreeing.iter().map(|&index| obs_px[index]).collect();
            validate(&subset_poses, &subset_obs, min_angle_rad, max_reproj_err_px).ok()
        }
        Err(_) => None,
    }
}
// #endregion 🔖️Triangulate

// #region 🔖️Pnp
/// 🧮️ Real roots (imaginary part below `1e-6`) of the polynomial with low-to-high coefficients `coeffs`,
/// via [`poly_roots_companion`]'s companion-matrix eigenvalue solver — reused here instead of a
/// hand-derived closed-form (e.g. Ferrari's method for the quartic below) since a general, already-tested
/// polynomial solver is far less likely to hide a sign/derivation bug than a bespoke one.
fn real_roots_of(coeffs: &[f64]) -> Vec<f64> {
    poly_roots_companion(coeffs).map(|roots| roots.into_iter().filter(|&(_, im)| im.abs() < 1e-6).map(|(re, _)| re).collect()).unwrap_or_default()
}

/// 🎯️ Classic Grunert P3P: given 3 unit camera rays and their corresponding world points, the law of
/// cosines between the three unknown camera-to-point distances `s1, s2, s3` reduces (eliminating `s3`
/// via a quadratic, then substituting into the remaining constraint) to a quartic in `x = s2/s1`, solved
/// via [`real_roots_of`]. Because the elimination squares an intermediate equation (introducing a sign
/// ambiguity in `s3`'s quadratic root and potential extraneous roots), every candidate `(s1, s2, s3)` is
/// re-validated against all three *original* (unsquared) law-of-cosines equations before being accepted —
/// a defensive check that also naturally discards numerically spurious roots. Each accepted candidate's
/// absolute pose is recovered via [`crate::standards::v1::subsets::any::schema::lie_internals::umeyama`] (no scale) between the world points and
/// their now-known camera-frame positions `s_i * f_i`. Returns up to 4 poses (after deduplicating
/// near-identical candidates), or none for degenerate (collinear/coincident) input points.
/// <https://en.wikipedia.org/wiki/Perspective-n-Point#P3P>
pub fn p3p_grunert(cam_rays: &[[f64; 3]; 3], world_pts: &[[f64; 3]; 3]) -> Vec<Se3> {
    let f: [[f64; 3]; 3] = std::array::from_fn(|i| vec3d_normalize(cam_rays[i]));
    let cos_alpha = dot3(f[1], f[2]);
    let cos_beta = dot3(f[0], f[2]);
    let cos_gamma = dot3(f[0], f[1]);
    let a = vec3d_length(vec3d_sub(world_pts[1], world_pts[2]));
    let b = vec3d_length(vec3d_sub(world_pts[0], world_pts[2]));
    let c = vec3d_length(vec3d_sub(world_pts[0], world_pts[1]));
    if a < 1e-9 || b < 1e-9 || c < 1e-9 {
        return Vec::new();
    }
    let p = (b * b) / (c * c);
    let q = (a * a) / (c * c);

    let m2 = 1.0 - q + p;
    let m1 = 2.0 * (cos_gamma * (q - p) - cos_alpha * cos_beta);
    let m0 = 2.0 * cos_beta * cos_beta - q + p - 1.0;
    let w2 = p;
    let w1 = -2.0 * p * cos_gamma;
    let w0 = p - 1.0 + cos_beta * cos_beta;
    let n2 = cos_alpha * cos_alpha;
    let n1 = -2.0 * cos_alpha * cos_beta;
    let n0 = cos_beta * cos_beta;

    let a4 = m2 * m2 - 4.0 * n2 * w2;
    let a3 = 2.0 * m1 * m2 - 4.0 * (n2 * w1 + n1 * w2);
    let a2 = m1 * m1 + 2.0 * m0 * m2 - 4.0 * (n2 * w0 + n1 * w1 + n0 * w2);
    let a1 = 2.0 * m0 * m1 - 4.0 * (n1 * w0 + n0 * w1);
    let a0 = m0 * m0 - 4.0 * n0 * w0;

    let scale2 = a * a + b * b + c * c + 1.0;
    let mut poses: Vec<Se3> = Vec::new();
    for x in real_roots_of(&[a0, a1, a2, a3, a4]) {
        if x <= 0.0 {
            continue;
        }
        let denom_c = 1.0 + x * x - 2.0 * x * cos_gamma;
        if denom_c <= 1e-12 {
            continue;
        }
        let s1_sq = c * c / denom_c;
        if s1_sq <= 0.0 {
            continue;
        }
        let s1 = s1_sq.sqrt();
        let w_sq = p * x * x - 2.0 * p * cos_gamma * x + (p - 1.0 + cos_beta * cos_beta);
        if w_sq < -1e-9 {
            continue;
        }
        let w_val = w_sq.max(0.0).sqrt();
        for y in [cos_beta + w_val, cos_beta - w_val] {
            if y <= 0.0 {
                continue;
            }
            let s2 = s1 * x;
            let s3 = s1 * y;
            let residual = (s2 * s2 + s3 * s3 - 2.0 * s2 * s3 * cos_alpha - a * a).abs() + (s1 * s1 + s3 * s3 - 2.0 * s1 * s3 * cos_beta - b * b).abs() + (s1 * s1 + s2 * s2 - 2.0 * s1 * s2 * cos_gamma - c * c).abs();
            if residual > 1e-6 * scale2 {
                continue;
            }
            let camera_pts = [scale3(f[0], s1), scale3(f[1], s2), scale3(f[2], s3)];
            if let Some(sim) = umeyama(world_pts, &camera_pts, false) {
                let is_dup = poses.iter().any(|p: &Se3| vec3d_length(vec3d_sub(p.t, sim.t)) < 1e-6 && dot3([p.r.0.cols[0][0], p.r.0.cols[1][1], p.r.0.cols[2][2]], [sim.r.0.cols[0][0], sim.r.0.cols[1][1], sim.r.0.cols[2][2]]) > 3.0 - 1e-6);
                if !is_dup {
                    poses.push(Se3 { r: sim.r, t: sim.t });
                }
            }
        }
    }
    poses
}

/// 🎲️ [`MinimalSolver`] wrapping [`p3p_grunert`] (`SAMPLE_SIZE = 3`) so it plugs into
/// [`crate::standards::v1::subsets::any::schema::optimize_internals::ransac`]/[`lo_ransac`] for outlier-robust PnP.
pub struct P3pSolver {
    intr: Intrinsics,
}

impl MinimalSolver for P3pSolver {
    type Datum = ([f64; 3], [f64; 2]);
    type Model = Se3;
    const SAMPLE_SIZE: usize = 3;

    fn solve(&self, sample: &[Self::Datum]) -> Vec<Self::Model> {
        let world: [[f64; 3]; 3] = std::array::from_fn(|i| sample[i].0);
        let rays: [[f64; 3]; 3] = std::array::from_fn(|i| self.intr.unproject_ray(sample[i].1));
        p3p_grunert(&rays, &world)
    }

    fn residual(&self, model: &Self::Model, datum: &Self::Datum) -> f64 {
        let pose = CameraPose(*model);
        match reproject(&self.intr, &pose, datum.0) {
            Some(pred) => ((pred[0] - datum.1[0]).powi(2) + (pred[1] - datum.1[1]).powi(2)).sqrt(),
            None => f64::MAX,
        }
    }
}

/// 🎯️ Outlier-robust perspective-n-point: RANSAC over [`P3pSolver`]'s minimal 3-point samples, with
/// [`refine_pose_lm`] as the locally-optimized-RANSAC polish on each new best model's inlier set.
pub fn pnp_ransac(intr: &Intrinsics, world_pts: &[[f64; 3]], obs_px: &[[f64; 2]], cfg: &RansacConfig) -> Option<(CameraPose, Vec<usize>)> {
    pnp_ransac_bounded(intr, world_pts, obs_px, cfg, POSE_REFINE_ITERATIONS)
}

/// 🎯️ Levenberg–Marquardt iterations [`refine_pose_lm`] spends, and the smaller budget a bounded
/// registration step affords its local optimisation (the global bundle adjustment refines again).
const POSE_REFINE_ITERATIONS: usize = 50;

pub const INTERACTIVE_POSE_REFINE_ITERATIONS: usize = 8;

/// 🎯️ [`pnp_ransac`] with an explicit iteration budget for the local-optimisation polish of each
/// improved model, so one hypothesis draw stays inside a worker step.
pub fn pnp_ransac_bounded(intr: &Intrinsics, world_pts: &[[f64; 3]], obs_px: &[[f64; 2]], cfg: &RansacConfig, refine_iterations: usize) -> Option<(CameraPose, Vec<usize>)> {
    if world_pts.len() != obs_px.len() || world_pts.len() < 3 {
        return None;
    }
    let data: Vec<([f64; 3], [f64; 2])> = world_pts.iter().copied().zip(obs_px.iter().copied()).collect();
    let solver = P3pSolver { intr: *intr };
    let local_opt = |subset: &[([f64; 3], [f64; 2])], model: &Se3| -> Option<Se3> {
        if subset.len() < 3 {
            return None;
        }
        let w: Vec<[f64; 3]> = subset.iter().map(|d| d.0).collect();
        let o: Vec<[f64; 2]> = subset.iter().map(|d| d.1).collect();
        Some(refine_pose_lm_bounded(&solver.intr, &w, &o, *model, refine_iterations))
    };
    let result = lo_ransac(&solver, &data, cfg, local_opt)?;
    Some((CameraPose(result.model), result.inliers))
}

/// 📐️ EPnP control points: the centroid plus three points offset along the PCA axes of `world_pts`,
/// scaled by `sqrt(eigenvalue)` so the resulting tetrahedron spans the point cloud's spread.
fn epnp_control_points(world_pts: &[[f64; 3]]) -> Option<[[f64; 3]; 4]> {
    let n = world_pts.len() as f64;
    let c0 = scale3(world_pts.iter().fold([0.0; 3], |acc, p| add3(acc, *p)), 1.0 / n);
    let mut cov = MatD::zeros(3, 3);
    for p in world_pts {
        let d = vec3d_sub(*p, c0);
        for r in 0..3 {
            for c in 0..3 {
                cov.add_at(r, c, d[r] * d[c]);
            }
        }
    }
    for v in cov.data.iter_mut() {
        *v /= n;
    }
    let (eigvals, eigvecs) = jacobi_eigen_symmetric(&cov, 100).ok()?;
    let mut ctrl = [c0; 4];
    for k in 0..3 {
        let idx = 2 - k;
        let eigval = eigvals[idx].max(0.0);
        let axis = [eigvecs.get(0, idx), eigvecs.get(1, idx), eigvecs.get(2, idx)];
        ctrl[k + 1] = add3(c0, scale3(axis, eigval.sqrt()));
    }
    Some(ctrl)
}

/// 📐️ Barycentric weights `[a0..a3]` (summing to 1) expressing `p` as a combination of the 4 control points.
fn epnp_barycentric(ctrl: &[[f64; 3]; 4], p: [f64; 3]) -> Option<[f64; 4]> {
    let mut basis = MatD::zeros(3, 3);
    for c in 0..3 {
        let col = vec3d_sub(ctrl[c + 1], ctrl[0]);
        for (r, &v) in col.iter().enumerate() {
            basis.set(r, c, v);
        }
    }
    let rhs = vec3d_sub(p, ctrl[0]);
    let sol = basis.lu_solve(&VecD::from_vec(rhs.to_vec()))?;
    let (a1, a2, a3) = (sol.get(0), sol.get(1), sol.get(2));
    Some([1.0 - a1 - a2 - a3, a1, a2, a3])
}

/// 🎯️ EPnP (Lepetit, Moreno-Noguer & Fua 2009), the `N = 1` null-space-vector variant: expresses every
/// world point as a barycentric combination of 4 control points ([`epnp_control_points`]), builds the
/// `2n x 12` linear system relating each normalized-ray observation to the (unknown) camera-frame control
/// points, and takes the eigenvector of `MᵀM`'s smallest eigenvalue (via [`jacobi_eigen_symmetric`]) as
/// the camera-frame control points up to an unknown scale and sign — recovered from the known
/// control-point mutual distances and a positive-mean-depth check. Full EPnP additionally searches linear
/// combinations of the smallest `N = 1..4` eigenvectors for better noise robustness at the cost of a
/// polynomial solve per `N`; this crate's `N = 1` variant is the closed-form baseline, meant to be
/// followed by [`refine_pose_lm`] for a final nonlinear polish.
pub fn epnp(intr: &Intrinsics, world_pts: &[[f64; 3]], obs_px: &[[f64; 2]]) -> Option<Se3> {
    if world_pts.len() < 6 || world_pts.len() != obs_px.len() {
        return None;
    }
    let ctrl_world = epnp_control_points(world_pts)?;
    let alphas: Vec<[f64; 4]> = world_pts.iter().map(|&p| epnp_barycentric(&ctrl_world, p)).collect::<Option<Vec<_>>>()?;
    let rays: Vec<[f64; 2]> = obs_px
        .iter()
        .map(|&px| {
            let r = intr.unproject_ray(px);
            [r[0], r[1]]
        })
        .collect();

    let n = world_pts.len();
    let mut m = MatD::zeros(2 * n, 12);
    for (i, (&a, &[xn, yn])) in alphas.iter().zip(rays.iter()).enumerate() {
        for (j, &aj) in a.iter().enumerate() {
            m.set(2 * i, 3 * j, aj);
            m.set(2 * i, 3 * j + 2, -xn * aj);
            m.set(2 * i + 1, 3 * j + 1, aj);
            m.set(2 * i + 1, 3 * j + 2, -yn * aj);
        }
    }
    let mtm = m.transpose().matmul(&m);
    let (_eigvals, eigvecs) = jacobi_eigen_symmetric(&mtm, 100).ok()?;
    let v: [f64; 12] = std::array::from_fn(|k| eigvecs.get(k, 0));
    let null_ctrl: [[f64; 3]; 4] = std::array::from_fn(|j| [v[3 * j], v[3 * j + 1], v[3 * j + 2]]);

    let mut ratio_sum = 0.0;
    let mut ratio_count = 0.0;
    for i in 0..4 {
        for j in (i + 1)..4 {
            let world_d = vec3d_length(vec3d_sub(ctrl_world[i], ctrl_world[j]));
            let null_d = vec3d_length(vec3d_sub(null_ctrl[i], null_ctrl[j]));
            if null_d > 1e-12 {
                ratio_sum += world_d / null_d;
                ratio_count += 1.0;
            }
        }
    }
    if ratio_count < 1.0 {
        return None;
    }
    let mut scale = ratio_sum / ratio_count;
    let ctrl_cam_of = |scale: f64| -> [[f64; 3]; 4] { std::array::from_fn(|j| scale3(null_ctrl[j], scale)) };
    let mut ctrl_cam = ctrl_cam_of(scale);
    if ctrl_cam.iter().map(|c| c[2]).sum::<f64>() < 0.0 {
        scale = -scale;
        ctrl_cam = ctrl_cam_of(scale);
    }

    let camera_pts: Vec<[f64; 3]> = alphas
        .iter()
        .map(|a| {
            let mut p = [0.0; 3];
            for j in 0..4 {
                p = add3(p, scale3(ctrl_cam[j], a[j]));
            }
            p
        })
        .collect();

    let sim = umeyama(world_pts, &camera_pts, false)?;
    Some(Se3 { r: sim.r, t: sim.t })
}

/// 🧩️ Single-pose bundle-refinement problem: the 6 unknowns are the camera pose's `se(3)` log-tangent, world points are fixed.
struct PoseRefineProblem<'a> {
    intr: &'a Intrinsics,
    world_pts: &'a [[f64; 3]],
    obs_px: &'a [[f64; 2]],
}

impl LeastSquaresProblem for PoseRefineProblem<'_> {
    fn residual_count(&self) -> usize {
        self.obs_px.len() * 2
    }

    fn parameter_count(&self) -> usize {
        6
    }

    fn residuals(&self, x: &VecD, out: &mut VecD) {
        let xi: [f64; 6] = std::array::from_fn(|k| x.get(k));
        let pose = CameraPose(Se3::exp(xi));
        for (row, (&p, &obs)) in self.world_pts.iter().zip(self.obs_px.iter()).enumerate() {
            let pred = reproject(self.intr, &pose, p).unwrap_or([obs[0] + 1.0e3, obs[1] + 1.0e3]);
            out.set(2 * row, pred[0] - obs[0]);
            out.set(2 * row + 1, pred[1] - obs[1]);
        }
    }

    fn jacobian(&self, x: &VecD, out: &mut MatD) {
        numeric_jacobian(self, x, 1e-6, out);
    }

    fn plus(&self, x: &VecD, dx: &VecD) -> VecD {
        let cur: [f64; 6] = std::array::from_fn(|k| x.get(k));
        let ddelta: [f64; 6] = std::array::from_fn(|k| dx.get(k));
        VecD::from_vec(Se3::exp(ddelta).semio_compose_rs(&Se3::exp(cur)).log().to_vec())
    }
}

/// 📐️ Refines a camera pose via Levenberg-Marquardt on its reprojection error over fixed world points.
pub fn refine_pose_lm(intr: &Intrinsics, world_pts: &[[f64; 3]], obs_px: &[[f64; 2]], initial: Se3) -> Se3 {
    refine_pose_lm_bounded(intr, world_pts, obs_px, initial, POSE_REFINE_ITERATIONS)
}

/// 🎯️ [`refine_pose_lm`] with an explicit iteration budget.
pub fn refine_pose_lm_bounded(intr: &Intrinsics, world_pts: &[[f64; 3]], obs_px: &[[f64; 2]], initial: Se3, max_iterations: usize) -> Se3 {
    let problem = PoseRefineProblem { intr, world_pts, obs_px };
    let x0 = VecD::from_vec(initial.log().to_vec());
    let cfg = LmConfig { max_iters: max_iterations.max(1), ..LmConfig::default() };
    let result = semio_s_artifact_remodel_remodeling_optimize::standards::v1::subsets::any::schema::optimize_internals::levenberg_marquardt(&problem, x0, &cfg);
    let xi: [f64; 6] = std::array::from_fn(|k| result.x.get(k));
    Se3::exp(xi)
}
// #endregion 🔖️Pnp
