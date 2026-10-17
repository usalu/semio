//! 📐️ Calibration of the camera model: the matrix helpers, the planar calibration and the self calibration. See [`super`].

use super::*;

// #region 🔖️Mat3Helpers
pub(super) fn scale3(v: [f64; 3], s: f64) -> [f64; 3] {
    [v[0] * s, v[1] * s, v[2] * s]
}

pub(super) fn skew3(v: [f64; 3]) -> [[f64; 3]; 3] {
    [[0.0, -v[2], v[1]], [v[2], 0.0, -v[0]], [-v[1], v[0], 0.0]]
}

fn mat3_vec(m: &[[f64; 3]; 3], v: [f64; 3]) -> [f64; 3] {
    std::array::from_fn(|r| m[r][0] * v[0] + m[r][1] * v[1] + m[r][2] * v[2])
}

pub(super) fn mat3_mul(a: &[[f64; 3]; 3], b: &[[f64; 3]; 3]) -> [[f64; 3]; 3] {
    std::array::from_fn(|r| std::array::from_fn(|c| (0..3).map(|k| a[r][k] * b[k][c]).sum()))
}

pub(super) fn transpose3(m: &[[f64; 3]; 3]) -> [[f64; 3]; 3] {
    std::array::from_fn(|r| std::array::from_fn(|c| m[c][r]))
}

pub(super) fn mat3d_rowmajor(m: &Mat3d) -> [[f64; 3]; 3] {
    std::array::from_fn(|r| std::array::from_fn(|c| m.cols[c][r]))
}

pub(super) fn mat2_mul(a: &[[f64; 2]; 2], b: &[[f64; 2]; 2]) -> [[f64; 2]; 2] {
    std::array::from_fn(|r| std::array::from_fn(|c| a[r][0] * b[0][c] + a[r][1] * b[1][c]))
}

pub(super) fn mat2x2_mul_2x3(a: &[[f64; 2]; 2], b: &[[f64; 3]; 2]) -> [[f64; 3]; 2] {
    std::array::from_fn(|r| std::array::from_fn(|c| a[r][0] * b[0][c] + a[r][1] * b[1][c]))
}

pub(super) fn mat2x3_mul_3x3(a: &[[f64; 3]; 2], b: &[[f64; 3]; 3]) -> [[f64; 3]; 2] {
    std::array::from_fn(|r| std::array::from_fn(|c| (0..3).map(|k| a[r][k] * b[k][c]).sum()))
}
// #endregion 🔖️Mat3Helpers

// #region 🔖️PlanarCalibration
/// 📐️ Result of [`calibrate_planar`]: recovered intrinsics, one pose per input view, the final RMS
/// reprojection error in pixels, and the marginal covariance of the full parameter vector (intrinsics
/// followed by per-view poses) from the last accepted Levenberg-Marquardt iteration.
#[derive(Clone, Debug)]
pub struct CalibrationResult {
    pub intrinsics: Intrinsics,
    pub poses: Vec<CameraPose>,
    pub rms_px: f64,
    pub covariance: MatD,
}

fn normalize_2d(pts: &[[f64; 2]]) -> (Vec<[f64; 2]>, [[f64; 3]; 3]) {
    let n = pts.len() as f64;
    let mean = pts.iter().fold([0.0, 0.0], |acc, p| [acc[0] + p[0], acc[1] + p[1]]);
    let mean = [mean[0] / n, mean[1] / n];
    let avg_dist = pts.iter().map(|p| ((p[0] - mean[0]).powi(2) + (p[1] - mean[1]).powi(2)).sqrt()).sum::<f64>() / n;
    let scale = if avg_dist > 1e-12 { 2f64.sqrt() / avg_dist } else { 1.0 };
    let normalized = pts.iter().map(|p| [(p[0] - mean[0]) * scale, (p[1] - mean[1]) * scale]).collect();
    let t = [[scale, 0.0, -scale * mean[0]], [0.0, scale, -scale * mean[1]], [0.0, 0.0, 1.0]];
    (normalized, t)
}

fn invert_similarity(t: &[[f64; 3]; 3]) -> [[f64; 3]; 3] {
    let s = t[0][0];
    let mx = -t[0][2] / s;
    let my = -t[1][2] / s;
    [[1.0 / s, 0.0, mx], [0.0, 1.0 / s, my], [0.0, 0.0, 1.0]]
}

/// 📐️ Estimates a planar homography `image_px ~ H * [board_xy, 1]` via normalized DLT (Hartley
/// normalization on both point sets, nullspace of the 2n x 9 design matrix via [`svd_nullvector`]).
fn estimate_homography(board_xy: &[[f64; 2]], image_px: &[[f64; 2]]) -> Result<[[f64; 3]; 3], CameraError> {
    let n = board_xy.len();
    if n < 4 {
        return Err(CameraError::DegenerateHomography);
    }
    let (nb, tb) = normalize_2d(board_xy);
    let (ni, ti) = normalize_2d(image_px);
    let mut a = MatD::zeros(2 * n, 9);
    for (row, (b, im)) in nb.iter().zip(ni.iter()).enumerate() {
        let (x, y) = (b[0], b[1]);
        let (u, v) = (im[0], im[1]);
        a.set(2 * row, 0, -x);
        a.set(2 * row, 1, -y);
        a.set(2 * row, 2, -1.0);
        a.set(2 * row, 6, u * x);
        a.set(2 * row, 7, u * y);
        a.set(2 * row, 8, u);
        a.set(2 * row + 1, 3, -x);
        a.set(2 * row + 1, 4, -y);
        a.set(2 * row + 1, 5, -1.0);
        a.set(2 * row + 1, 6, v * x);
        a.set(2 * row + 1, 7, v * y);
        a.set(2 * row + 1, 8, v);
    }
    let h = svd_nullvector(&a).map_err(|_| CameraError::DegenerateHomography)?;
    let h_tilde = [[h.get(0), h.get(1), h.get(2)], [h.get(3), h.get(4), h.get(5)], [h.get(6), h.get(7), h.get(8)]];
    let ti_inv = invert_similarity(&ti);
    Ok(mat3_mul(&mat3_mul(&ti_inv, &h_tilde), &tb))
}

fn v_pq(h: &[[f64; 3]; 3], p: usize, q: usize) -> [f64; 6] {
    let hp = |r: usize| h[r][p];
    let hq = |r: usize| h[r][q];
    [hp(0) * hq(0), hp(0) * hq(1) + hp(1) * hq(0), hp(1) * hq(1), hp(2) * hq(0) + hp(0) * hq(2), hp(2) * hq(1) + hp(1) * hq(2), hp(2) * hq(2)]
}

/// 📐️ Recovers `(fx, fy, cx, cy, skew)` from the symmetric image-of-the-absolute-conic vector
/// `b = [B11, B12, B22, B13, B23, B33]` via Zhang's closed-form.
fn recover_intrinsics(b: &VecD) -> Result<(f64, f64, f64, f64, f64), CameraError> {
    let mut vals: [f64; 6] = std::array::from_fn(|i| b.get(i));
    if vals[0] < 0.0 {
        for v in vals.iter_mut() {
            *v = -*v;
        }
    }
    let (b11, b12, b22, b13, b23, b33) = (vals[0], vals[1], vals[2], vals[3], vals[4], vals[5]);
    let denom = b11 * b22 - b12 * b12;
    if denom.abs() < 1e-300 || b11.abs() < 1e-300 {
        return Err(CameraError::SingularSystem);
    }
    let cy = (b12 * b13 - b11 * b23) / denom;
    let lambda = b33 - (b13 * b13 + cy * (b12 * b13 - b11 * b23)) / b11;
    let fx_sq = lambda / b11;
    let fy_sq = lambda * b11 / denom;
    if fx_sq <= 0.0 || fy_sq <= 0.0 || !fx_sq.is_finite() || !fy_sq.is_finite() {
        return Err(CameraError::SingularSystem);
    }
    let fx = fx_sq.sqrt();
    let fy = fy_sq.sqrt();
    let skew = -b12 * fx * fx * fy / lambda;
    let cx = skew * cy / fy - b13 * fx * fx / lambda;
    if !(fx.is_finite() && fy.is_finite() && cx.is_finite() && cy.is_finite() && skew.is_finite()) {
        return Err(CameraError::SingularSystem);
    }
    Ok((fx, fy, cx, cy, skew))
}

/// 📐️ Recovers a view's extrinsic pose from its homography and the shared intrinsics: `r1 = K⁻¹h1/‖·‖`,
/// `r2 = K⁻¹h2/‖·‖`, `r3 = r1 × r2`, `t = K⁻¹h3/‖·‖`, orthonormalized via [`So3::project_to_so3`].
fn recover_pose(h: &[[f64; 3]; 3], fx: f64, fy: f64, cx: f64, cy: f64, skew: f64) -> Result<CameraPose, CameraError> {
    let k_inv = [[1.0 / fx, -skew / (fx * fy), (skew * cy - cx * fy) / (fx * fy)], [0.0, 1.0 / fy, -cy / fy], [0.0, 0.0, 1.0]];
    let h1 = [h[0][0], h[1][0], h[2][0]];
    let h2 = [h[0][1], h[1][1], h[2][1]];
    let h3 = [h[0][2], h[1][2], h[2][2]];
    let r1_raw = mat3_vec(&k_inv, h1);
    let r2_raw = mat3_vec(&k_inv, h2);
    let t_raw = mat3_vec(&k_inv, h3);
    let n1 = vec3d_length(r1_raw);
    let n2 = vec3d_length(r2_raw);
    if n1 < 1e-12 || n2 < 1e-12 {
        return Err(CameraError::DegenerateHomography);
    }
    let mut lambda_scale = 2.0 / (n1 + n2);
    if lambda_scale * t_raw[2] < 0.0 {
        lambda_scale = -lambda_scale;
    }
    let r1 = scale3(r1_raw, lambda_scale);
    let r2 = scale3(r2_raw, lambda_scale);
    let r3 = vec3d_cross(r1, r2);
    let t = scale3(t_raw, lambda_scale);
    let rot = Mat3d::from_axes(r1, r2, r3);
    Ok(CameraPose(Se3 { r: So3::project_to_so3(&rot), t }))
}

fn intrinsics_from_params(x: &VecD) -> Intrinsics {
    Intrinsics { fx: x.get(0), fy: x.get(1), cx: x.get(2), cy: x.get(3), skew: x.get(4), distortion: Distortion::BrownConrady { k1: x.get(5), k2: x.get(6), k3: 0.0, p1: x.get(7), p2: x.get(8) } }
}

/// 🧩️ Joint planar-calibration least-squares problem: one shared `Intrinsics` (9 params: fx, fy, cx, cy,
/// skew, k1, k2, p1, p2) plus one SE(3) log-tangent pose per view. A local wrapper distinct from
/// [`ReprojectionProblem`] because planar calibration has a single shared intrinsics block and known
/// (not optimized) board points, rather than per-observation shared points across many cameras.
struct PlanarCalibrationProblem<'a> {
    pub(super) views: &'a [Vec<([f64; 2], [f64; 2])>],
}

impl LeastSquaresProblem for PlanarCalibrationProblem<'_> {
    fn residual_count(&self) -> usize {
        self.views.iter().map(|v| v.len() * 2).sum()
    }

    fn parameter_count(&self) -> usize {
        9 + 6 * self.views.len()
    }

    fn residuals(&self, x: &VecD, out: &mut VecD) {
        let intr = intrinsics_from_params(x);
        let mut row = 0usize;
        for (i, view) in self.views.iter().enumerate() {
            let pose = CameraPose(se3_at(x, 9 + i * 6));
            for &(board_xy, obs) in view {
                let point_world = [board_xy[0], board_xy[1], 0.0];
                let pred = reproject(&intr, &pose, point_world).unwrap_or([obs[0] + 1.0e3, obs[1] + 1.0e3]);
                out.set(row, pred[0] - obs[0]);
                out.set(row + 1, pred[1] - obs[1]);
                row += 2;
            }
        }
    }

    /// 🔬️ Analytic Jacobians via [`reprojection_jacobians`]: the pose block scatters into this view's 6
    /// pose columns, and the intrinsics block (ordered `[fx,fy,cx,cy,skew,k1,k2,k3,p1,p2]`) drops its `k3`
    /// column (index 7, fixed at zero and not part of `x`) when scattering into this problem's 9 shared
    /// intrinsics columns `[fx,fy,cx,cy,skew,k1,k2,p1,p2]`.
    fn jacobian(&self, x: &VecD, out: &mut MatD) {
        const INTRINSICS_SRC_COLS: [usize; 9] = [0, 1, 2, 3, 4, 5, 6, 8, 9];
        let intr = intrinsics_from_params(x);
        let mut row = 0usize;
        for (i, view) in self.views.iter().enumerate() {
            let pose = CameraPose(se3_at(x, 9 + i * 6));
            for &(board_xy, obs) in view {
                let point_world = [board_xy[0], board_xy[1], 0.0];
                let (j_pose, _j_point, j_intr) = reprojection_jacobians(&intr, &pose, point_world, obs);
                for r in 0..2 {
                    for (dest, &src) in INTRINSICS_SRC_COLS.iter().enumerate() {
                        out.set(row + r, dest, j_intr.get(r, src));
                    }
                    for c in 0..6 {
                        out.set(row + r, 9 + i * 6 + c, j_pose.get(r, c));
                    }
                }
                row += 2;
            }
        }
    }

    fn plus(&self, x: &VecD, dx: &VecD) -> VecD {
        let mut out = VecD::zeros(x.len());
        for k in 0..9 {
            out.set(k, x.get(k) + dx.get(k));
        }
        for i in 0..self.views.len() {
            let updated = retract_se3_block(x, dx, 9 + i * 6);
            for (k, &v) in updated.iter().enumerate() {
                out.set(9 + i * 6 + k, v);
            }
        }
        out
    }
}

/// 📐️ Zhang's method planar calibration: per-view homographies via normalized DLT, closed-form intrinsics
/// from the image-of-the-absolute-conic linear system, per-view extrinsics from each homography, then a
/// full nonlinear joint refinement (shared intrinsics + BrownConrady distortion + all view poses) via
/// [`levenberg_marquardt`]. Requires at least 3 views; returns [`CameraError`] for degenerate inputs.
pub fn calibrate_planar(views: &[Vec<([f64; 2], [f64; 2])>], image_width: u32, image_height: u32) -> Result<CalibrationResult, CameraError> {
    if views.len() < 3 {
        return Err(CameraError::TooFewViews);
    }
    let homographies: Vec<[[f64; 3]; 3]> = views
        .iter()
        .map(|v| {
            let board: Vec<[f64; 2]> = v.iter().map(|&(b, _)| b).collect();
            let img: Vec<[f64; 2]> = v.iter().map(|&(_, p)| p).collect();
            estimate_homography(&board, &img)
        })
        .collect::<Result<_, _>>()?;

    let mut v_mat = MatD::zeros(2 * homographies.len(), 6);
    for (i, h) in homographies.iter().enumerate() {
        let v12 = v_pq(h, 0, 1);
        let v11 = v_pq(h, 0, 0);
        let v22 = v_pq(h, 1, 1);
        for (k, ((&v12k, &v11k), &v22k)) in v12.iter().zip(v11.iter()).zip(v22.iter()).enumerate() {
            v_mat.set(2 * i, k, v12k);
            v_mat.set(2 * i + 1, k, v11k - v22k);
        }
    }
    let b = svd_nullvector(&v_mat).map_err(|_| CameraError::SingularSystem)?;
    let (fx, fy, cx, cy, skew) = recover_intrinsics(&b)?;
    let (w, h) = (image_width as f64, image_height as f64);
    if cx < -w || cx > 2.0 * w || cy < -h || cy > 2.0 * h {
        return Err(CameraError::SingularSystem);
    }

    let poses: Vec<CameraPose> = homographies.iter().map(|hg| recover_pose(hg, fx, fy, cx, cy, skew)).collect::<Result<_, _>>()?;

    let mut x0 = VecD::zeros(9 + 6 * views.len());
    x0.set(0, fx);
    x0.set(1, fy);
    x0.set(2, cx);
    x0.set(3, cy);
    x0.set(4, skew);
    for (i, pose) in poses.iter().enumerate() {
        let log = pose.0.log();
        for (k, v) in log.into_iter().enumerate() {
            x0.set(9 + i * 6 + k, v);
        }
    }

    let problem = PlanarCalibrationProblem { views };
    let cfg = LmConfig { max_iters: 200, ..LmConfig::default() };
    let result = levenberg_marquardt(&problem, x0, &cfg);
    if !result.converged {
        return Err(CameraError::Convergence);
    }

    let final_intr = intrinsics_from_params(&result.x);
    let final_poses: Vec<CameraPose> = (0..views.len()).map(|i| CameraPose(se3_at(&result.x, 9 + i * 6))).collect();
    let residual_count = problem.residual_count();
    let rms_px = (4.0 * result.cost / residual_count as f64).sqrt();
    let covariance = pseudo_inverse(&result.jtj, 1e-9).map_err(|_| CameraError::SingularSystem)?;
    Ok(CalibrationResult { intrinsics: final_intr, poses: final_poses, rms_px, covariance })
}
// #endregion 🔖️PlanarCalibration

// #region 🔖️SelfCalibration
fn nullvector_3x3(m: &[[f64; 3]; 3]) -> Option<[f64; 3]> {
    let mut a = MatD::zeros(3, 3);
    for (r, row) in m.iter().enumerate() {
        for (c, &v) in row.iter().enumerate() {
            a.set(r, c, v);
        }
    }
    let n = svd_nullvector(&a).ok()?;
    Some([n.get(0), n.get(1), n.get(2)])
}

/// 📍️ `(e1, e2)` epipoles of a fundamental matrix: `e1` satisfies `F·e1 = 0` (epipole in image 1), `e2`
/// satisfies `Fᵀ·e2 = 0` (epipole in image 2), both as raw (not `z`-normalized) SVD nullvectors.
fn epipoles_from_fundamental_matrix(f: &[[f64; 3]; 3]) -> Option<([f64; 3], [f64; 3])> {
    let e1 = nullvector_3x3(f)?;
    let e2 = nullvector_3x3(&transpose3(f))?;
    Some((e1, e2))
}

/// 🔄️ 2D rotation (embedded in the top-left 3x3 block) that eliminates the `y`-component of `v`'s
/// direction: the [`bougnoux_focal_from_fundamental`] epipole-alignment step.
fn rotation_eliminate_y(v: [f64; 3]) -> [[f64; 3]; 3] {
    let r = (v[0] * v[0] + v[1] * v[1]).sqrt();
    if r < 1e-300 {
        return [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
    }
    let (c, s) = (v[0] / r, -v[1] / r);
    [[c, -s, 0.0], [s, c, 0.0], [0.0, 0.0, 1.0]]
}

/// 📐️ Fundamental matrix corresponding to both principal points shifted to the coordinate origin:
/// `F_shifted = T2⁻ᵀ · F · T1⁻¹` for the pure-translation `T1, T2` moving `pp1, pp2` to `(0,0)`.
fn shift_principal_points_to_origin(f: &[[f64; 3]; 3], pp1: [f64; 2], pp2: [f64; 2]) -> [[f64; 3]; 3] {
    let t1_inv = [[1.0, 0.0, pp1[0]], [0.0, 1.0, pp1[1]], [0.0, 0.0, 1.0]];
    let t2_inv_transpose = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [pp2[0], pp2[1], 1.0]];
    mat3_mul(&mat3_mul(&t2_inv_transpose, f), &t1_inv)
}

/// 🔭️ Bougnoux/Hartley closed-form recovery of two cameras' distinct focal lengths `(f1, f2)` from their
/// shared fundamental matrix and known principal points: shift both principal points to the origin, rotate
/// each image so its epipole's `y`-component vanishes, and read the closed-form focal pair off the
/// resulting 2x2 block (Hartley, "Extraction of focal lengths from the fundamental matrix",
/// axiom.anu.edu.au/~hartley/Papers/focal-lengths/focal.pdf — the same reduction implemented in libmv's
/// `FocalFromFundamental`). Scale-invariant in `f_matrix` (numerator and denominator are both linear in
/// it, so any nonzero multiple gives the same answer). `None` for degenerate epipolar geometry (near-zero
/// epipole components, singular denominators) or a non-positive focal-length estimate.
pub fn bougnoux_focal_from_fundamental(f_matrix: &MatD, pp1: [f64; 2], pp2: [f64; 2]) -> Option<(f64, f64)> {
    if f_matrix.rows != 3 || f_matrix.cols != 3 {
        return None;
    }
    let f: [[f64; 3]; 3] = std::array::from_fn(|r| std::array::from_fn(|c| f_matrix.get(r, c)));
    let f_shifted = shift_principal_points_to_origin(&f, pp1, pp2);
    let (e1, e2) = epipoles_from_fundamental_matrix(&f_shifted)?;
    let t1 = rotation_eliminate_y(e1);
    let t2 = rotation_eliminate_y(e2);
    let f_rotated = mat3_mul(&mat3_mul(&t2, &f_shifted), &transpose3(&t1));
    let (e1r, e2r) = epipoles_from_fundamental_matrix(&f_rotated)?;
    if e1r[0].abs() < 1e-9 || e1r[2].abs() < 1e-9 || e2r[0].abs() < 1e-9 || e2r[2].abs() < 1e-9 {
        return None;
    }
    let t1b0 = 1.0 / e1r[2];
    let t2b0 = 1.0 / e2r[2];
    let a = t2b0 * f_rotated[0][0] * t1b0;
    let b = t2b0 * f_rotated[0][1];
    let c = f_rotated[1][0] * t1b0;
    let d = f_rotated[1][1];
    let f1_denom = a * c * e1r[2] * e1r[2] + b * d;
    let f2_denom = a * b * e2r[2] * e2r[2] + c * d;
    if f1_denom.abs() < 1e-300 || f2_denom.abs() < 1e-300 {
        return None;
    }
    let f1_sq = -(a * c * e1r[0] * e1r[0]) / f1_denom;
    let f2_sq = -(a * b * e2r[0] * e2r[0]) / f2_denom;
    if !(f1_sq.is_finite() && f2_sq.is_finite()) || f1_sq <= 0.0 || f2_sq <= 0.0 {
        return None;
    }
    Some((f1_sq.sqrt(), f2_sq.sqrt()))
}

/// 📊️ Componentwise median aggregation of noisy per-pair `(f1, f2)` candidates (e.g. many
/// [`bougnoux_focal_from_fundamental`] estimates across image pairs of the same two cameras): each
/// closed-form estimate is sensitive to noise in `F` and to near-degenerate epipolar geometry, so the
/// median across many pairs is far more stable than trusting any single one. `(0.0, 0.0)` when empty.
pub fn aggregate_self_calibration(candidates: &[(f64, f64)]) -> (f64, f64) {
    fn median_of(values: &mut [f64]) -> f64 {
        values.sort_by(f64::total_cmp);
        let n = values.len();
        if n == 0 {
            return 0.0;
        }
        if n % 2 == 1 {
            values[n / 2]
        } else {
            0.5 * (values[n / 2 - 1] + values[n / 2])
        }
    }
    let mut f1s: Vec<f64> = candidates.iter().map(|&(f1, _)| f1).collect();
    let mut f2s: Vec<f64> = candidates.iter().map(|&(_, f2)| f2).collect();
    (median_of(&mut f1s), median_of(&mut f2s))
}
// #endregion 🔖️SelfCalibration
