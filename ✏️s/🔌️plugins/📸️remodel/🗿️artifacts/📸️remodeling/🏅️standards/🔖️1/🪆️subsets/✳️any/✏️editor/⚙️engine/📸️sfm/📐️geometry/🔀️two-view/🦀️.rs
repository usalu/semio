//! 🔀️ Two-view models of the multi-view geometry: the fundamental, homography and essential estimation (including the five-point solver), the essential decomposition and refinement and the model selection. See [`super`].

use super::*;

// #region 🔖️TwoView
/// 📐️ One of the two planar/epipolar two-view models recovered by [`estimate_fundamental`] / [`estimate_homography`] / [`select_two_view_model`].
#[derive(Clone, Copy, Debug, PartialEq, semio_framework_value::RetireOwned)]
pub enum TwoViewModel {
    Fundamental([[f64; 3]; 3]),
    Homography([[f64; 3]; 3]),
}

/// 📦️ A two-view geometry fit: the recovered model, the inlier indices into the input correspondence slice, and its RANSAC score.
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
pub struct TwoViewResult {
    pub model: TwoViewModel,
    pub inliers: Vec<usize>,
    pub score: f64,
}

/// 🧮️ Hartley normalization: translates `pts` to a zero centroid and scales so the mean distance from
/// the origin is `sqrt(2)`, returning the normalized points and the `3x3` similarity transform.
/// <https://en.wikipedia.org/wiki/Eight-point_algorithm#Normalized_eight-point_algorithm>
fn normalize_pts(pts: &[[f64; 2]]) -> (Vec<[f64; 2]>, [[f64; 3]; 3]) {
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

/// 🌀️ Projects a `3x3` matrix to the nearest (Frobenius) rank-2 matrix by zeroing its smallest singular value.
fn enforce_rank2(m: &[[f64; 3]; 3]) -> Option<[[f64; 3]; 3]> {
    let (u, mut sigma, v) = svd(&matd_from_mat3(m)).ok()?;
    if sigma.len() < 3 {
        return None;
    }
    sigma[2] = 0.0;
    let mut sigma_mat = MatD::zeros(3, 3);
    for (i, &s) in sigma.iter().enumerate() {
        sigma_mat.set(i, i, s);
    }
    Some(mat3_from_matd(&u.matmul(&sigma_mat).matmul(&v.transpose())))
}

/// 📐️ Normalized `n`-point (`n >= 8`) linear fundamental-matrix DLT: Hartley-normalizes both point sets,
/// solves the homogeneous `Af = 0` system via [`svd_nullvector`], projects to rank 2, then denormalizes
/// `F = Tbᵀ F̃ Ta`.
/// <https://en.wikipedia.org/wiki/Eight-point_algorithm>
fn fit_fundamental_dlt(corr: &[([f64; 2], [f64; 2])]) -> Option<[[f64; 3]; 3]> {
    if corr.len() < 8 {
        return None;
    }
    let a_pts: Vec<[f64; 2]> = corr.iter().map(|&(a, _)| a).collect();
    let b_pts: Vec<[f64; 2]> = corr.iter().map(|&(_, b)| b).collect();
    let (na, ta) = normalize_pts(&a_pts);
    let (nb, tb) = normalize_pts(&b_pts);
    let mut design = MatD::zeros(corr.len(), 9);
    for (row, (&[x, y], &[xp, yp])) in na.iter().zip(nb.iter()).enumerate() {
        design.set(row, 0, xp * x);
        design.set(row, 1, xp * y);
        design.set(row, 2, xp);
        design.set(row, 3, yp * x);
        design.set(row, 4, yp * y);
        design.set(row, 5, yp);
        design.set(row, 6, x);
        design.set(row, 7, y);
        design.set(row, 8, 1.0);
    }
    let f_vec = nullvector_via_gram(&design)?;
    let f_tilde = [[f_vec.get(0), f_vec.get(1), f_vec.get(2)], [f_vec.get(3), f_vec.get(4), f_vec.get(5)], [f_vec.get(6), f_vec.get(7), f_vec.get(8)]];
    let f_rank2 = enforce_rank2(&f_tilde)?;
    Some(mat3_mul(&mat3_mul(&mat3_transpose(&tb), &f_rank2), &ta))
}

/// 📏️ Signed Sampson first-order approximation to the geometric epipolar reprojection error, in the same
/// units (pixels, or normalized-ray units for an essential matrix) as the input correspondences — the
/// square root of [`sampson_distance`]'s squared error, sign-preserved so it doubles as a proper
/// nonlinear-least-squares residual (e.g. [`EssentialRefineProblem`]).
/// <https://en.wikipedia.org/wiki/Epipolar_geometry>
fn signed_sampson_residual(f: &[[f64; 3]; 3], a: [f64; 2], b: [f64; 2]) -> f64 {
    let x = [a[0], a[1], 1.0];
    let xp = [b[0], b[1], 1.0];
    let fx = mat3_vec(f, x);
    let ftxp = mat3_vec(&mat3_transpose(f), xp);
    let numer = dot3(xp, fx);
    let denom = fx[0] * fx[0] + fx[1] * fx[1] + ftxp[0] * ftxp[0] + ftxp[1] * ftxp[1];
    if denom < 1e-300 {
        return f64::MAX;
    }
    numer / denom.sqrt()
}

/// 📏️ Sampson first-order approximation to the geometric epipolar reprojection error (see
/// [`signed_sampson_residual`]), as a nonnegative distance for RANSAC scoring/thresholding.
fn sampson_distance(f: &[[f64; 3]; 3], a: [f64; 2], b: [f64; 2]) -> f64 {
    signed_sampson_residual(f, a, b).abs()
}

/// 🎲️ [`MinimalSolver`] fitting a fundamental matrix from exactly 8 correspondences via [`fit_fundamental_dlt`], scored by [`sampson_distance`].
struct FundamentalSolver;

impl MinimalSolver for FundamentalSolver {
    type Datum = ([f64; 2], [f64; 2]);
    type Model = [[f64; 3]; 3];
    const SAMPLE_SIZE: usize = 8;

    fn solve(&self, sample: &[Self::Datum]) -> Vec<Self::Model> {
        fit_fundamental_dlt(sample).into_iter().collect()
    }

    fn residual(&self, model: &Self::Model, datum: &Self::Datum) -> f64 {
        sampson_distance(model, datum.0, datum.1)
    }
}

/// 📐️ Normalized 8-point fundamental-matrix estimation robustified by locally-optimized RANSAC
/// ([`crate::standards::v1::subsets::any::schema::optimize_internals::lo_ransac`]): the local optimization step refits [`fit_fundamental_dlt`]
/// over the current inlier set (which accepts any `n >= 8`, not just the minimal 8), which is a fast,
/// closed-form stand-in for a full Sampson-error LM polish.
pub fn estimate_fundamental(matches: &[([f64; 2], [f64; 2])]) -> Option<TwoViewResult> {
    let cfg = RansacConfig { threshold: 1.5, confidence: 0.999, max_iters: 2000, seed: 0, scoring: RansacScoring::Msac };
    let local_opt = |subset: &[([f64; 2], [f64; 2])], _model: &[[f64; 3]; 3]| fit_fundamental_dlt(subset);
    let result = lo_ransac(&FundamentalSolver, matches, &cfg, local_opt)?;
    Some(TwoViewResult { model: TwoViewModel::Fundamental(result.model), inliers: result.inliers, score: result.score })
}

/// 📐️ Normalized 4-point DLT homography (Hartley-normalizes both point sets, nullspace of the `2n x 9`
/// design matrix via [`svd_nullvector`], denormalized `H = Tb⁻¹ H̃ Ta`).
fn fit_homography_dlt(corr: &[([f64; 2], [f64; 2])]) -> Option<[[f64; 3]; 3]> {
    if corr.len() < 4 {
        return None;
    }
    let a_pts: Vec<[f64; 2]> = corr.iter().map(|&(a, _)| a).collect();
    let b_pts: Vec<[f64; 2]> = corr.iter().map(|&(_, b)| b).collect();
    let (na, ta) = normalize_pts(&a_pts);
    let (nb, tb) = normalize_pts(&b_pts);
    let mut design = MatD::zeros(2 * corr.len(), 9);
    for (row, (&[x, y], &[u, v])) in na.iter().zip(nb.iter()).enumerate() {
        design.set(2 * row, 0, -x);
        design.set(2 * row, 1, -y);
        design.set(2 * row, 2, -1.0);
        design.set(2 * row, 6, u * x);
        design.set(2 * row, 7, u * y);
        design.set(2 * row, 8, u);
        design.set(2 * row + 1, 3, -x);
        design.set(2 * row + 1, 4, -y);
        design.set(2 * row + 1, 5, -1.0);
        design.set(2 * row + 1, 6, v * x);
        design.set(2 * row + 1, 7, v * y);
        design.set(2 * row + 1, 8, v);
    }
    let h_vec = nullvector_via_gram(&design)?;
    let h_tilde = [[h_vec.get(0), h_vec.get(1), h_vec.get(2)], [h_vec.get(3), h_vec.get(4), h_vec.get(5)], [h_vec.get(6), h_vec.get(7), h_vec.get(8)]];
    let tb_inv = invert_similarity(&tb);
    Some(mat3_mul(&mat3_mul(&tb_inv, &h_tilde), &ta))
}

fn homography_residual(h: &[[f64; 3]; 3], a: [f64; 2], b: [f64; 2]) -> f64 {
    let p = mat3_vec(h, [a[0], a[1], 1.0]);
    if p[2].abs() < 1e-12 {
        return f64::MAX;
    }
    ((p[0] / p[2] - b[0]).powi(2) + (p[1] / p[2] - b[1]).powi(2)).sqrt()
}

/// 🎲️ [`MinimalSolver`] fitting a homography from exactly 4 correspondences via [`fit_homography_dlt`],
/// scored by one-directional (`a -> b`) point-transfer error.
struct HomographySolver;

impl MinimalSolver for HomographySolver {
    type Datum = ([f64; 2], [f64; 2]);
    type Model = [[f64; 3]; 3];
    const SAMPLE_SIZE: usize = 4;

    fn solve(&self, sample: &[Self::Datum]) -> Vec<Self::Model> {
        fit_homography_dlt(sample).into_iter().collect()
    }

    fn residual(&self, model: &Self::Model, datum: &Self::Datum) -> f64 {
        homography_residual(model, datum.0, datum.1)
    }
}

/// 📐️ Normalized 4-point DLT homography estimation robustified by locally-optimized RANSAC, refitting
/// [`fit_homography_dlt`] over the current inlier set on every improvement.
pub fn estimate_homography(matches: &[([f64; 2], [f64; 2])]) -> Option<TwoViewResult> {
    let cfg = RansacConfig { threshold: 1.5, confidence: 0.999, max_iters: 2000, seed: 0, scoring: RansacScoring::Msac };
    let local_opt = |subset: &[([f64; 2], [f64; 2])], _model: &[[f64; 3]; 3]| fit_homography_dlt(subset);
    let result = lo_ransac(&HomographySolver, matches, &cfg, local_opt)?;
    Some(TwoViewResult { model: TwoViewModel::Homography(result.model), inliers: result.inliers, score: result.score })
}

/// 🎥️ Essential-matrix estimation: unprojects pixel matches through each camera's
/// [`Intrinsics::unproject_ray`] (undoing both the linear intrinsic map and lens distortion) into
/// normalized camera rays, then runs the same normalized 8-point + RANSAC pipeline as
/// [`estimate_fundamental`] directly in normalized-ray space, where the "fundamental matrix" of the
/// normalized correspondences *is* the essential matrix. Callers whose points are already undistorted
/// may equivalently pass `Distortion::None` intrinsics with the identity linear map.
pub fn estimate_essential(matches: &[([f64; 2], [f64; 2])], k_a: &Intrinsics, k_b: &Intrinsics) -> Option<TwoViewResult> {
    estimate_essential_with_limit(matches, k_a, k_b, 2_000)
}

pub fn estimate_essential_with_limit(matches: &[([f64; 2], [f64; 2])], k_a: &Intrinsics, k_b: &Intrinsics, maximum_hypotheses: usize) -> Option<TwoViewResult> {
    estimate_essential_with_threshold(matches, k_a, k_b, 0.005, maximum_hypotheses)
}

/// 🎥️ [`estimate_essential_with_limit`] at an explicit Sampson tolerance in normalized coordinates
/// (the caller derives it from a pixel tolerance and the focal length).
pub fn estimate_essential_with_threshold(matches: &[([f64; 2], [f64; 2])], k_a: &Intrinsics, k_b: &Intrinsics, threshold: f64, maximum_hypotheses: usize) -> Option<TwoViewResult> {
    let normalized: Vec<([f64; 2], [f64; 2])> = matches
        .iter()
        .map(|&(pa, pb)| {
            let ra = k_a.unproject_ray(pa);
            let rb = k_b.unproject_ray(pb);
            ([ra[0], ra[1]], [rb[0], rb[1]])
        })
        .collect();
    let cfg = RansacConfig { threshold, confidence: 0.999, max_iters: maximum_hypotheses, seed: 1, scoring: RansacScoring::Msac };
    let local_opt = |subset: &[([f64; 2], [f64; 2])], _model: &[[f64; 3]; 3]| fit_fundamental_dlt(subset);
    let result = lo_ransac(&FundamentalSolver, &normalized, &cfg, local_opt)?;
    Some(TwoViewResult { model: TwoViewModel::Fundamental(result.model), inliers: result.inliers, score: result.score })
}

/// 📐️ Two-view linear triangulation (see [`triangulate_dlt`]) specialized to normalized-ray coordinates
/// and a relative pose `p_b = R p_a + t`, used only for [`decompose_essential`]'s cheirality vote.
pub fn triangulate_normalized_pair(r: &[[f64; 3]; 3], t: [f64; 3], xa: [f64; 2], xb: [f64; 2]) -> Option<[f64; 3]> {
    let p_a = [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0]];
    let p_b = [[r[0][0], r[0][1], r[0][2], t[0]], [r[1][0], r[1][1], r[1][2], t[1]], [r[2][0], r[2][1], r[2][2], t[2]]];
    let mut design = MatD::zeros(4, 4);
    for c in 0..4 {
        design.set(0, c, xa[0] * p_a[2][c] - p_a[0][c]);
        design.set(1, c, xa[1] * p_a[2][c] - p_a[1][c]);
        design.set(2, c, xb[0] * p_b[2][c] - p_b[0][c]);
        design.set(3, c, xb[1] * p_b[2][c] - p_b[1][c]);
    }
    let sol = svd_nullvector(&design).ok()?;
    let w = sol.get(3);
    if w.abs() < 1e-12 {
        return None;
    }
    Some([sol.get(0) / w, sol.get(1) / w, sol.get(2) / w])
}

/// 🔀️ Recovers the unique physically-valid relative pose from an essential matrix: SVD gives
/// `E = U diag(1,1,0) Vᵀ`, and the four `(R, t)` candidates are `U W±¹ Vᵀ` (with the classic `W` matrix
/// trick, sign-corrected to `det(R) = +1`) paired with `±U`'s third column; the candidate whose
/// triangulated points are in front of both cameras most often wins.
/// <https://en.wikipedia.org/wiki/Essential_matrix#Determining_R_and_t_from_E>
pub fn decompose_essential(e: &[[f64; 3]; 3], inlier_matches: &[([f64; 2], [f64; 2])]) -> Option<Se3> {
    let (u, _sigma, v) = svd(&matd_from_mat3(e)).ok()?;
    let u3 = mat3_from_matd(&u);
    let v3 = mat3_from_matd(&v);
    let w = [[0.0, -1.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]];
    let wt = mat3_transpose(&w);
    let fix_det = |m: [[f64; 3]; 3]| if mat3_det(&m) < 0.0 { std::array::from_fn(|r: usize| std::array::from_fn(|c: usize| -m[r][c])) } else { m };
    let r1 = fix_det(mat3_mul(&mat3_mul(&u3, &w), &mat3_transpose(&v3)));
    let r2 = fix_det(mat3_mul(&mat3_mul(&u3, &wt), &mat3_transpose(&v3)));
    let t_col = [u3[0][2], u3[1][2], u3[2][2]];
    let t_pos = vec3d_normalize(t_col);
    let t_neg = scale3(t_pos, -1.0);
    let candidates = [(r1, t_pos), (r1, t_neg), (r2, t_pos), (r2, t_neg)];
    let mut best_idx = 0usize;
    let mut best_count = -1i64;
    for (idx, &(r, t)) in candidates.iter().enumerate() {
        let mut count = 0i64;
        for &(xa, xb) in inlier_matches {
            if let Some(p) = triangulate_normalized_pair(&r, t, xa, xb) {
                let depth_b = add3(mat3_vec(&r, p), t)[2];
                if p[2] > 0.0 && depth_b > 0.0 {
                    count += 1;
                }
            }
        }
        if count > best_count {
            best_count = count;
            best_idx = idx;
        }
    }
    if best_count <= 0 {
        return None;
    }
    let (r_best, t_best) = candidates[best_idx];
    Some(Se3 { r: So3::project_to_so3(&array_to_mat3d(&r_best)), t: t_best })
}

/// 🧭️ Orthonormal basis `(u, v)` spanning the tangent plane of the unit sphere at `t`, used to give the
/// scale-free translation *direction* of a two-view relative pose a 2-DOF local chart for
/// [`refine_essential_lm`] (mirrors how [`PoseRefineProblem`] gives `Se3` a 6-DOF chart via `se3` exp/log).
fn sphere_tangent_basis(t: [f64; 3]) -> ([f64; 3], [f64; 3]) {
    let seed = if t[0].abs() < 0.9 { [1.0, 0.0, 0.0] } else { [0.0, 1.0, 0.0] };
    let u = vec3d_normalize(cross3(t, seed));
    let v = cross3(t, u);
    (u, v)
}

/// 🧭️ Exponential-map retraction of a 2-vector `phi` from the tangent plane at `t0` (spanned by `u, v`,
/// see [`sphere_tangent_basis`]) back onto the unit sphere.
fn sphere_retract(t0: [f64; 3], u: [f64; 3], v: [f64; 3], phi: [f64; 2]) -> [f64; 3] {
    let theta = (phi[0] * phi[0] + phi[1] * phi[1]).sqrt();
    if theta < 1e-12 {
        return t0;
    }
    let (s, c) = theta.sin_cos();
    add3(scale3(t0, c), scale3(add3(scale3(u, phi[0]), scale3(v, phi[1])), s / theta))
}

/// 🧩️ Two-view relative-pose refinement problem: 3 DOF for rotation (`so3` log-tangent, same convention
/// as [`PoseRefineProblem`]) plus 2 DOF for the scale-free translation *direction* (tangent-plane
/// coordinates around the initial guess, see [`sphere_tangent_basis`]/[`sphere_retract`]) — the essential
/// matrix's true 5-DOF manifold, unlike the unconstrained 8-DOF linear fit [`fit_fundamental_dlt`] uses.
struct EssentialRefineProblem<'a> {
    pub(super) corr: &'a [([f64; 2], [f64; 2])],
    t0: [f64; 3],
    tangent_u: [f64; 3],
    tangent_v: [f64; 3],
}

impl EssentialRefineProblem<'_> {
    fn essential_at(&self, x: &VecD) -> [[f64; 3]; 3] {
        let omega: [f64; 3] = std::array::from_fn(|k| x.get(k));
        let r = mat3d_to_array(&So3::exp(omega).0);
        let phi = [x.get(3), x.get(4)];
        let t = sphere_retract(self.t0, self.tangent_u, self.tangent_v, phi);
        mat3_mul(&skew3(t), &r)
    }
}

impl LeastSquaresProblem for EssentialRefineProblem<'_> {
    fn residual_count(&self) -> usize {
        self.corr.len()
    }

    fn parameter_count(&self) -> usize {
        5
    }

    fn residuals(&self, x: &VecD, out: &mut VecD) {
        let e = self.essential_at(x);
        for (row, &(a, b)) in self.corr.iter().enumerate() {
            out.set(row, signed_sampson_residual(&e, a, b));
        }
    }

    fn jacobian(&self, x: &VecD, out: &mut MatD) {
        numeric_jacobian(self, x, 1e-6, out);
    }

    fn plus(&self, x: &VecD, dx: &VecD) -> VecD {
        let cur_r: [f64; 3] = std::array::from_fn(|k| x.get(k));
        let d_r: [f64; 3] = std::array::from_fn(|k| dx.get(k));
        let new_r = So3::exp(d_r).semio_compose_rs(&So3::exp(cur_r)).log();
        VecD::from_vec(vec![new_r[0], new_r[1], new_r[2], x.get(3) + dx.get(3), x.get(4) + dx.get(4)])
    }
}

/// 📐️ Polishes a [`decompose_essential`]-recovered relative pose by Levenberg-Marquardt over its true
/// 5-DOF manifold ([`EssentialRefineProblem`]), minimizing signed Sampson epipolar residuals across
/// `corr` (normalized-ray correspondences). Unlike refitting through [`fit_fundamental_dlt`] (an
/// unconstrained 8-DOF linear fit, degenerate on planar/low-parallax scenes — see
/// [`estimate_essential_five_point`]), this never leaves the essential matrix's constrained manifold, so
/// it remains well-behaved exactly where the minimal 5-point solve most needs a noise-robust polish.
fn refine_essential_lm(initial: &Se3, corr: &[([f64; 2], [f64; 2])]) -> Se3 {
    refine_essential_lm_bounded(initial, corr, 50)
}

/// 🎯️ [`refine_essential_lm`] with the Levenberg–Marquardt iteration count as a parameter, for the
/// interactive polish whose unit must stay inside the worker law.
fn refine_essential_lm_bounded(initial: &Se3, corr: &[([f64; 2], [f64; 2])], max_iters: usize) -> Se3 {
    let t0 = vec3d_normalize(initial.t);
    let (tangent_u, tangent_v) = sphere_tangent_basis(t0);
    let problem = EssentialRefineProblem { corr, t0, tangent_u, tangent_v };
    let x0 = VecD::from_vec(vec![initial.r.log()[0], initial.r.log()[1], initial.r.log()[2], 0.0, 0.0]);
    let cfg = LmConfig { max_iters, ..LmConfig::default() };
    let result = semio_s_artifact_remodel_remodeling_optimize::standards::v1::subsets::any::schema::optimize_internals::levenberg_marquardt(&problem, x0, &cfg);
    let omega: [f64; 3] = std::array::from_fn(|k| result.x.get(k));
    let phi = [result.x.get(3), result.x.get(4)];
    Se3 { r: So3::exp(omega), t: sphere_retract(t0, tangent_u, tangent_v, phi) }
}

/// 🧮️ Simplified GRIC (geometric robust information criterion, Torr 1997) score comparing a
/// fundamental-matrix vs. homography fit: `GRIC = truncated_residual_sum + lambda1 * d * n_inliers +
/// lambda2 * k`, where `d` is the model's per-correspondence manifold dimension (3 for F's single
/// epipolar constraint per point, 2 for H's fully-determined point map) and `k` is the model's parameter
/// count (7 for F, 8 for H). `truncated_residual_sum` reuses the MSAC-truncated cost already returned as
/// [`TwoViewResult::score`]. `lambda1 = 1.0` and `lambda2 = 2.0` are fixed constants rather than the full
/// noise-variance-derived weights from Torr's original formulation — a documented simplification.
fn simplified_gric(score: f64, n_inliers: usize, d: usize, k: usize) -> f64 {
    score + (d * n_inliers) as f64 + 2.0 * k as f64
}

/// 📐️ Picks between [`estimate_fundamental`] and [`estimate_homography`] via [`simplified_gric`], so
/// planar or low-parallax scenes (where a homography fully explains the data) don't get forced through a
/// degenerate epipolar fit.
pub fn select_two_view_model(matches: &[([f64; 2], [f64; 2])]) -> Option<TwoViewResult> {
    match (estimate_fundamental(matches), estimate_homography(matches)) {
        (Some(f), Some(h)) => {
            let gric_f = simplified_gric(f.score, f.inliers.len(), 3, 7);
            let gric_h = simplified_gric(h.score, h.inliers.len(), 2, 8);
            Some(if gric_f <= gric_h { f } else { h })
        }
        (Some(f), None) => Some(f),
        (None, Some(h)) => Some(h),
        (None, None) => None,
    }
}

/// 🌱️ The 20 monomials of total degree `<= 3` in three variables `x, y, z`, ordered so index `0..10` are
/// exactly the ten *degree-3* monomials and index `10..20` are exactly the ten degree-`<= 2` monomials
/// (the [`EssentialFivePoint`] quotient-ring basis). This split is not an arbitrary convention borrowed
/// from a reference implementation — it falls out of Bezout/dimension counting (`C(3+3,3) = 20` monomials
/// total, and the ten cubic constraint polynomials generically cut the degree-`<=3` space down to a
/// 10-dimensional quotient with the degree-`<=2` monomials as a natural basis, since `C(2+3,3) = 10`
/// exactly matches), so it is re-derivable from first principles rather than memorized.
const MONO3: [(u8, u8, u8); 20] =
    [(3, 0, 0), (0, 3, 0), (0, 0, 3), (2, 1, 0), (2, 0, 1), (1, 2, 0), (0, 2, 1), (1, 0, 2), (0, 1, 2), (1, 1, 1), (0, 0, 0), (1, 0, 0), (0, 1, 0), (0, 0, 1), (2, 0, 0), (0, 2, 0), (0, 0, 2), (1, 1, 0), (1, 0, 1), (0, 1, 1)];

fn mono3_index(i: u8, j: u8, k: u8) -> Option<usize> {
    MONO3.iter().position(|&m| m == (i, j, k))
}

/// 🌱️ A trivariate polynomial in `x, y, z`, truncated to total degree `<= 3` (20 coefficients, ordered by
/// [`MONO3`]) — the symbolic scratch type [`essential_five_point_candidates`] uses to expand the Nistér
/// constraint equations *mechanically* (generic polynomial add/sub/mul) rather than via a hand-derived,
/// hard-to-verify closed-form coefficient table. [`Poly3::mul`] is only ever called on operands whose
/// combined degree is provably `<= 3` by the call graph below (degree-1 `E` times degree-1 `E`, then the
/// degree-2 result times degree-1 `E` again), so truncation never silently drops a real term.
#[derive(Clone, Copy)]
struct Poly3 {
    pub(super) c: [f64; 20],
}

impl Poly3 {
    fn zero() -> Self {
        Self { c: [0.0; 20] }
    }

    fn linear(x_coef: f64, y_coef: f64, z_coef: f64, const_coef: f64) -> Self {
        let mut p = Self::zero();
        p.c[10] = const_coef;
        p.c[11] = x_coef;
        p.c[12] = y_coef;
        p.c[13] = z_coef;
        p
    }

    fn add(&self, other: &Self) -> Self {
        let mut out = *self;
        for i in 0..20 {
            out.c[i] += other.c[i];
        }
        out
    }

    fn sub(&self, other: &Self) -> Self {
        let mut out = *self;
        for i in 0..20 {
            out.c[i] -= other.c[i];
        }
        out
    }

    pub(super) fn scale(&self, s: f64) -> Self {
        let mut out = *self;
        for v in &mut out.c {
            *v *= s;
        }
        out
    }

    fn mul(&self, other: &Self) -> Self {
        let mut out = Self::zero();
        for (ia, &ca) in self.c.iter().enumerate() {
            if ca == 0.0 {
                continue;
            }
            let (i1, j1, k1) = MONO3[ia];
            for (ib, &cb) in other.c.iter().enumerate() {
                if cb == 0.0 {
                    continue;
                }
                let (i2, j2, k2) = MONO3[ib];
                let (i, j, k) = (i1 + i2, j1 + j2, k1 + k2);
                if let Some(idx) = mono3_index(i, j, k) {
                    out.c[idx] += ca * cb;
                }
            }
        }
        out
    }
}

/// 🎥️ The five-correspondence linear system `[xb*xa, xb*ya, xb, yb*xa, yb*ya, yb, xa, ya, 1] . e = 0`
/// (same row layout as [`fit_fundamental_dlt`]'s 8-point design matrix, just five rows instead of eight)
/// whose 4-dimensional null space (via [`nullspace_via_gram`], since 5 rows/9 cols is exactly the wide,
/// full-row-rank shape [`svd_nullvector`] cannot reach directly) parameterizes every essential matrix
/// consistent with the 5 correspondences as `E(x, y, z) = x X + y Y + z Z + W`.
fn essential_five_point_null_basis(corr: &[([f64; 2], [f64; 2]); 5]) -> Option<[[[f64; 3]; 3]; 4]> {
    let mut design = MatD::zeros(5, 9);
    for (row, ((xa, ya), (xb, yb))) in corr.iter().map(|&(a, b)| ((a[0], a[1]), (b[0], b[1]))).enumerate() {
        design.set(row, 0, xb * xa);
        design.set(row, 1, xb * ya);
        design.set(row, 2, xb);
        design.set(row, 3, yb * xa);
        design.set(row, 4, yb * ya);
        design.set(row, 5, yb);
        design.set(row, 6, xa);
        design.set(row, 7, ya);
        design.set(row, 8, 1.0);
    }
    let null_vecs = nullspace_via_gram(&design, 4)?;
    let reshape = |v: &VecD| -> [[f64; 3]; 3] { [[v.get(0), v.get(1), v.get(2)], [v.get(3), v.get(4), v.get(5)], [v.get(6), v.get(7), v.get(8)]] };
    Some(std::array::from_fn(|i| reshape(&null_vecs[i])))
}

/// 🌱️ Reduces `m` (`rows >= k`) to reduced row-echelon form with partial pivoting, restricted to pivoting
/// on the first `k` columns; returns `None` if that leading `k x k` block is singular (a degenerate 5-point
/// sample). On success the first `k` columns are the `k x k` identity and the rest hold the eliminated
/// system, i.e. `[I | B]`.
fn gauss_jordan_leading(mut m: MatD, k: usize) -> Option<MatD> {
    for pivot in 0..k {
        let mut best_row = pivot;
        let mut best_val = m.get(pivot, pivot).abs();
        for r in (pivot + 1)..m.rows {
            let v = m.get(r, pivot).abs();
            if v > best_val {
                best_val = v;
                best_row = r;
            }
        }
        if best_val < 1e-9 {
            return None;
        }
        if best_row != pivot {
            for c in 0..m.cols {
                let tmp = m.get(pivot, c);
                m.set(pivot, c, m.get(best_row, c));
                m.set(best_row, c, tmp);
            }
        }
        let pv = m.get(pivot, pivot);
        for c in 0..m.cols {
            m.set(pivot, c, m.get(pivot, c) / pv);
        }
        for r in 0..m.rows {
            if r == pivot {
                continue;
            }
            let factor = m.get(r, pivot);
            if factor == 0.0 {
                continue;
            }
            for c in 0..m.cols {
                let v = m.get(r, c) - factor * m.get(pivot, c);
                m.set(r, c, v);
            }
        }
    }
    Some(m)
}

/// 🎥️ Nistér's five-point essential-matrix solver: recovers up to 10 candidate essential matrices from
/// exactly 5 calibrated (normalized-ray) correspondences — the minimal case, and (unlike the 8-point
/// fundamental solver) numerically well-behaved on planar/low-parallax scenes.
///
/// Derivation (symbolic, not a memorized coefficient table — see [`Poly3`]): `E = x X + y Y + z Z + W`
/// spans the 5-correspondence null space ([`essential_five_point_null_basis`]); every valid essential
/// matrix additionally satisfies the trace constraint `2 E Eᵀ E - trace(E Eᵀ) E = 0` (9 scalar cubics)
/// and `det(E) = 0` (1 more), all expanded exactly via [`Poly3`] arithmetic into a `10x20` matrix over the
/// monomials of [`MONO3`]. [`gauss_jordan_leading`] eliminates the ten degree-3 monomial columns, leaving
/// `[I | B]`; multiplication-by-`z` on the resulting 10-dimensional quotient ring (basis: the ten
/// degree-`<=2` monomials) is then a `10x10` "action matrix" built directly from `B` (see the inline
/// column-by-column derivation below), whose eigenvalues are exactly the solutions' `z` values
/// (Stickelberger's theorem) and whose eigenvectors (recovered as nullvectors of `action_matrix - z*I`,
/// since the matrix is small and square) recover `x, y` from the eigenvector's monomial ratios.
/// <https://www.cs.unc.edu/~marc/tutorial/node51.html>
pub fn essential_five_point_candidates(corr: &[([f64; 2], [f64; 2]); 5]) -> Vec<[[f64; 3]; 3]> {
    let Some([x_mat, y_mat, z_mat, w_mat]) = essential_five_point_null_basis(corr) else {
        return Vec::new();
    };
    let mut e_poly: [[Poly3; 3]; 3] = [[Poly3::zero(); 3]; 3];
    for r in 0..3 {
        for c in 0..3 {
            e_poly[r][c] = Poly3::linear(x_mat[r][c], y_mat[r][c], z_mat[r][c], w_mat[r][c]);
        }
    }
    let mut eet_poly: [[Poly3; 3]; 3] = [[Poly3::zero(); 3]; 3];
    for r in 0..3 {
        for c in 0..3 {
            let mut acc = Poly3::zero();
            for (er, ec) in e_poly[r].iter().zip(e_poly[c].iter()) {
                acc = acc.add(&er.mul(ec));
            }
            eet_poly[r][c] = acc;
        }
    }
    let trace_poly = eet_poly[0][0].add(&eet_poly[1][1]).add(&eet_poly[2][2]);
    let mut rows: Vec<Poly3> = Vec::with_capacity(10);
    #[allow(clippy::needless_range_loop, reason = "r/c/k jointly index a 3x3x3 tensor contraction; iterator rewrites would need per-iteration column copies for no clarity gain")]
    for r in 0..3 {
        for c in 0..3 {
            let mut eete = Poly3::zero();
            for k in 0..3 {
                eete = eete.add(&eet_poly[r][k].mul(&e_poly[k][c]));
            }
            rows.push(eete.scale(2.0).sub(&trace_poly.mul(&e_poly[r][c])));
        }
    }
    let det_poly = e_poly[0][0]
        .mul(&e_poly[1][1].mul(&e_poly[2][2]).sub(&e_poly[1][2].mul(&e_poly[2][1])))
        .sub(&e_poly[0][1].mul(&e_poly[1][0].mul(&e_poly[2][2]).sub(&e_poly[1][2].mul(&e_poly[2][0]))))
        .add(&e_poly[0][2].mul(&e_poly[1][0].mul(&e_poly[2][1]).sub(&e_poly[1][1].mul(&e_poly[2][0]))));
    rows.push(det_poly);

    let mut a20 = MatD::zeros(10, 20);
    for (row_idx, poly) in rows.iter().enumerate() {
        for col in 0..20 {
            a20.set(row_idx, col, poly.c[col]);
        }
    }
    let Some(reduced) = gauss_jordan_leading(a20, 10) else {
        return Vec::new();
    };

    let mut t_z = MatD::zeros(10, 10);
    t_z.set(3, 0, 1.0);
    t_z.set(8, 1, 1.0);
    t_z.set(9, 2, 1.0);
    t_z.set(6, 3, 1.0);
    for (q_col, d3_row) in [(4usize, 4usize), (5, 6), (6, 2), (7, 9), (8, 7), (9, 8)] {
        for k in 0..10 {
            t_z.set(k, q_col, -reduced.get(d3_row, 10 + k));
        }
    }

    let Ok(eigs) = real_eigenvalues(&t_z) else {
        return Vec::new();
    };
    let t_z_transpose = t_z.transpose();
    let mut candidates = Vec::new();
    for (re, im) in eigs {
        if im.abs() > 1e-5 {
            continue;
        }
        let mut shifted = t_z_transpose.clone();
        for i in 0..10 {
            shifted.add_at(i, i, -re);
        }
        let Ok(v) = svd_nullvector(&shifted) else {
            continue;
        };
        let w = v.get(0);
        if w.abs() < 1e-9 {
            continue;
        }
        let (x, y, z) = (v.get(1) / w, v.get(2) / w, v.get(3) / w);
        let e_candidate: [[f64; 3]; 3] = std::array::from_fn(|r| std::array::from_fn(|c| x_mat[r][c] * x + y_mat[r][c] * y + z_mat[r][c] * z + w_mat[r][c]));
        candidates.push(e_candidate);
    }
    candidates
}

/// 🎲️ [`MinimalSolver`] wrapping [`essential_five_point_candidates`] (`SAMPLE_SIZE = 5`), operating on
/// normalized-ray correspondences and scored by the same [`sampson_distance`] as [`FundamentalSolver`].
struct EssentialFivePointSolver;

impl MinimalSolver for EssentialFivePointSolver {
    type Datum = ([f64; 2], [f64; 2]);
    type Model = [[f64; 3]; 3];
    const SAMPLE_SIZE: usize = 5;

    fn solve(&self, sample: &[Self::Datum]) -> Vec<Self::Model> {
        let corr: [([f64; 2], [f64; 2]); 5] = std::array::from_fn(|i| sample[i]);
        essential_five_point_candidates(&corr)
    }

    fn residual(&self, model: &Self::Model, datum: &Self::Datum) -> f64 {
        sampson_distance(model, datum.0, datum.1)
    }
}

/// 🎥️ Nistér five-point essential-matrix estimation: unprojects pixel matches into normalized camera
/// rays (as [`estimate_essential`]) and runs [`EssentialFivePointSolver`] under plain (non-local-optimized)
/// RANSAC, then iteratively polishes the winning minimal-sample model with [`refine_essential_lm`] over
/// its inlier set, re-classifying inliers against each polished model (a handful of rounds is enough to
/// converge, since each round only has to correct the previous round's residual bias). Unlike
/// [`estimate_essential`]'s 8-point-based fit, this remains well-conditioned on planar or low-parallax
/// scenes, since neither the minimal solve nor the polish ever linearizes away the `det(E) = 0` / trace
/// constraints that make the essential matrix's 5-DOF manifold degenerate for coplanar points under an
/// unconstrained 8-point fit.
pub fn estimate_essential_five_point(matches: &[([f64; 2], [f64; 2])], k_a: &Intrinsics, k_b: &Intrinsics, threshold: f64, seed: u64) -> Option<TwoViewResult> {
    estimate_essential_five_point_with_limit(matches, k_a, k_b, threshold, seed, 2_000)
}

/// 🎯️ [`estimate_essential_five_point`] with an explicit RANSAC hypothesis cap, so an interactive
/// worker step can spend a fixed number of minimal samples per call and keep the best model it has
/// seen across calls instead of running an unbounded search in one step.
pub fn estimate_essential_five_point_with_limit(matches: &[([f64; 2], [f64; 2])], k_a: &Intrinsics, k_b: &Intrinsics, threshold: f64, seed: u64, maximum_hypotheses: usize) -> Option<TwoViewResult> {
    estimate_essential_five_point_bounded(matches, k_a, k_b, threshold, seed, maximum_hypotheses, 5)
}

/// 🎯️ [`estimate_essential_five_point_with_limit`] with an explicit polish budget: `polish_rounds`
/// rounds of the nonlinear refinement (each an LM solve over the inlier set), so a worker step can
/// spend one round per call and polish the best model it keeps across calls.
pub fn estimate_essential_five_point_bounded(matches: &[([f64; 2], [f64; 2])], k_a: &Intrinsics, k_b: &Intrinsics, threshold: f64, seed: u64, maximum_hypotheses: usize, polish_rounds: usize) -> Option<TwoViewResult> {
    let normalized: Vec<([f64; 2], [f64; 2])> = matches
        .iter()
        .map(|&(pa, pb)| {
            let ra = k_a.unproject_ray(pa);
            let rb = k_b.unproject_ray(pb);
            ([ra[0], ra[1]], [rb[0], rb[1]])
        })
        .collect();
    let cfg = RansacConfig { threshold, confidence: 0.999, max_iters: maximum_hypotheses, seed, scoring: RansacScoring::Msac };
    let result = ransac(&EssentialFivePointSolver, &normalized, &cfg)?;
    Some(polish_essential_normalized(TwoViewResult { model: TwoViewModel::Fundamental(result.model), inliers: result.inliers, score: result.score }, &normalized, threshold, polish_rounds))
}

/// 🎯️ The nonlinear polish of [`estimate_essential_five_point_bounded`] on its own, bounded for
/// one interactive unit: one round of decompose → LM refine → re-classify, over pixel matches
/// unprojected through the intrinsics, for a caller that keeps the RANSAC search and the polish in
/// separate bounded units. The refine runs [`INTERACTIVE_ESSENTIAL_REFINE_ITERATIONS`] iterations
/// over at most [`INTERACTIVE_ESSENTIAL_REFINE_CORRESPONDENCES`] of the estimate's inliers (an
/// evenly strided subset): the unbounded polish of a true model with 60 raw inliers measured 4–12 ms
/// in a debug build, the whole worker law, and its cost grows with the inliers it explains.
pub fn polish_essential(estimate: TwoViewResult, matches: &[([f64; 2], [f64; 2])], intrinsics: &Intrinsics, threshold: f64) -> TwoViewResult {
    let normalized: Vec<([f64; 2], [f64; 2])> = matches
        .iter()
        .map(|&(pa, pb)| {
            let ra = intrinsics.unproject_ray(pa);
            let rb = intrinsics.unproject_ray(pb);
            ([ra[0], ra[1]], [rb[0], rb[1]])
        })
        .collect();
    polish_essential_normalized_with(estimate, &normalized, threshold, 1, INTERACTIVE_ESSENTIAL_REFINE_ITERATIONS, INTERACTIVE_ESSENTIAL_REFINE_CORRESPONDENCES)
}

/// ⏱️ Levenberg–Marquardt iterations of one interactive essential-matrix polish.
pub const INTERACTIVE_ESSENTIAL_REFINE_ITERATIONS: usize = 8;

/// ⏱️ Correspondences one interactive essential-matrix polish refines over at most.
pub const INTERACTIVE_ESSENTIAL_REFINE_CORRESPONDENCES: usize = 48;

fn polish_essential_normalized(estimate: TwoViewResult, normalized: &[([f64; 2], [f64; 2])], threshold: f64, polish_rounds: usize) -> TwoViewResult {
    polish_essential_normalized_with(estimate, normalized, threshold, polish_rounds, 50, usize::MAX)
}

fn polish_essential_normalized_with(estimate: TwoViewResult, normalized: &[([f64; 2], [f64; 2])], threshold: f64, polish_rounds: usize, max_iters: usize, max_correspondences: usize) -> TwoViewResult {
    let TwoViewModel::Fundamental(mut model) = estimate.model else { return estimate };
    let mut inliers = estimate.inliers.clone();
    for _ in 0..polish_rounds {
        let stride = inliers.len().div_ceil(max_correspondences.max(1)).max(1);
        let inlier_corr: Vec<([f64; 2], [f64; 2])> = inliers.iter().step_by(stride).map(|&i| normalized[i]).collect();
        let Some(pose) = decompose_essential(&model, &inlier_corr) else { break };
        let refined_pose = refine_essential_lm_bounded(&pose, &inlier_corr, max_iters);
        let r = mat3d_to_array(&refined_pose.r.0);
        model = mat3_mul(&skew3(refined_pose.t), &r);
        inliers = (0..normalized.len()).filter(|&i| sampson_distance(&model, normalized[i].0, normalized[i].1) < threshold).collect();
    }
    TwoViewResult { model: TwoViewModel::Fundamental(model), inliers, score: estimate.score }
}
// #endregion 🔖️TwoView
