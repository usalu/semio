//! 📐️ Multi-view geometry: two-view models and essential-matrix estimation, triangulation, PnP, feature tracks, global rotation/translation averaging, loop closure and pose priors.

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
use semio_s_artifact_remodel_remodeling_optimize::standards::v1::subsets::any::schema::optimize_internals::SchurLmPreparation;
#[cfg(test)] use semio_s_artifact_remodel_remodeling_optimize::standards::v1::subsets::any::schema::optimize_internals::{schur_lm, SchurResult};

// #region 🔖️Mat3Helpers

pub fn dot3(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

pub fn scale3(v: [f64; 3], s: f64) -> [f64; 3] {
    [v[0] * s, v[1] * s, v[2] * s]
}

fn add3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

pub fn cross3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

/// 🌱️ Homogeneous least-squares nullvector of `A`, computed via the symmetric Gram matrix `AᵀA` rather
/// than [`svd_nullvector`] directly. `svd`'s economy (thin) decomposition only ever returns
/// `min(rows, cols)` right-singular vectors — for a *wide* (`rows < cols`), full-row-rank design matrix
/// (exactly the shape every minimal-sample DLT solver below builds: 8 rows/9 cols for both the 8-point
/// and 4-point-homography minimal draws), that thin `V` spans the row space, not the null space, so
/// `svd_nullvector` silently returns a direction unrelated to the true kernel (verified empirically: it
/// fails to recover a planted null vector on an 8x9 full-row-rank matrix). Squaring into `AᵀA` (same
/// null space, always square) and taking [`jacobi_eigen_symmetric`]'s smallest-eigenvalue eigenvector
/// sidesteps the gap and works uniformly for both minimal (`rows < cols`) and over-determined
/// (`rows >= cols`) inputs.
fn nullspace_via_gram(a: &MatD, k: usize) -> Option<Vec<VecD>> {
    let n = a.cols;
    let mut g = MatD::zeros(n, n);
    for i in 0..n {
        for j in i..n {
            let mut s = 0.0;
            for row in 0..a.rows {
                s += a.get(row, i) * a.get(row, j);
            }
            g.set(i, j, s);
            g.set(j, i, s);
        }
    }
    let (_, vecs) = jacobi_eigen_symmetric(&g, 200).ok()?;
    Some((0..k).map(|col| VecD::from_vec((0..vecs.rows).map(|r| vecs.get(r, col)).collect())).collect())
}

fn nullvector_via_gram(a: &MatD) -> Option<VecD> {
    nullspace_via_gram(a, 1)?.into_iter().next()
}

fn mat3_vec(m: &[[f64; 3]; 3], v: [f64; 3]) -> [f64; 3] {
    std::array::from_fn(|r| m[r][0] * v[0] + m[r][1] * v[1] + m[r][2] * v[2])
}

/// ⚔️ Skew-symmetric cross-product matrix `[v]_x`, so `[v]_x . w == v x w`.
fn skew3(v: [f64; 3]) -> [[f64; 3]; 3] {
    [[0.0, -v[2], v[1]], [v[2], 0.0, -v[0]], [-v[1], v[0], 0.0]]
}

fn mat3_mul(a: &[[f64; 3]; 3], b: &[[f64; 3]; 3]) -> [[f64; 3]; 3] {
    std::array::from_fn(|r| std::array::from_fn(|c| (0..3).map(|k| a[r][k] * b[k][c]).sum()))
}

fn mat3_transpose(m: &[[f64; 3]; 3]) -> [[f64; 3]; 3] {
    std::array::from_fn(|r| std::array::from_fn(|c| m[c][r]))
}

fn mat3_det(m: &[[f64; 3]; 3]) -> f64 {
    m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1]) - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0]) + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
}

fn mat3_from_matd(m: &MatD) -> [[f64; 3]; 3] {
    std::array::from_fn(|r| std::array::from_fn(|c| m.get(r, c)))
}

fn matd_from_mat3(m: &[[f64; 3]; 3]) -> MatD {
    let mut out = MatD::zeros(3, 3);
    for (r, row) in m.iter().enumerate() {
        for (c, &v) in row.iter().enumerate() {
            out.set(r, c, v);
        }
    }
    out
}

/// 🧭️ Row-major `[[f64;3];3]` rotation array from a column-major [`Mat3d`] (as stored inside [`So3`]).
pub fn mat3d_to_array(m: &Mat3d) -> [[f64; 3]; 3] {
    std::array::from_fn(|r| std::array::from_fn(|c| m.cols[c][r]))
}

/// 🧭️ Column-major [`Mat3d`] from a row-major `[[f64;3];3]` rotation array, the inverse of [`mat3d_to_array`].
fn array_to_mat3d(m: &[[f64; 3]; 3]) -> Mat3d {
    Mat3d::from_axes([m[0][0], m[1][0], m[2][0]], [m[0][1], m[1][1], m[2][1]], [m[0][2], m[1][2], m[2][2]])
}

/// 🎯️ World-space optical center of a camera pose: `-Rᵀt`, the point mapping to the camera-space origin.
pub fn camera_center(pose: &CameraPose) -> [f64; 3] {
    let r_inv = pose.0.r.inverse();
    scale3(r_inv.act(pose.0.t), -1.0)
}

// #endregion 🔖️Mat3Helpers

// #region 🔖️Error

/// ⚠️ Error type for fallible structure-from-motion operations: two-view geometry, triangulation, PnP and incremental reconstruction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SfmError {
    TooFewFrames,
    DegenerateGeometry,
    InsufficientMatches,
    TriangulationFailed,
    PnpFailed,
    Convergence,
}

impl std::fmt::Display for SfmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooFewFrames => write!(f, "fewer than the minimum required number of frames"),
            Self::DegenerateGeometry => write!(f, "two-view geometry is degenerate"),
            Self::InsufficientMatches => write!(f, "not enough correspondences to fit a model"),
            Self::TriangulationFailed => write!(f, "triangulation failed to produce a valid point"),
            Self::PnpFailed => write!(f, "perspective-n-point pose estimation failed"),
            Self::Convergence => write!(f, "nonlinear refinement failed to converge"),
        }
    }
}

impl std::error::Error for SfmError {}

// #endregion 🔖️Error

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
    corr: &'a [([f64; 2], [f64; 2])],
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
    c: [f64; 20],
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

    fn scale(&self, s: f64) -> Self {
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
    // `c` genuinely indexes both the row-wise `eet_poly`/`e_poly` accumulation *and* is threaded through
    // to `e_poly[r][c]` below — a real 3-index tensor contraction over fixed 3x3 arrays, not a case
    // `.iter()`/`.enumerate()` simplifies without materializing an awkward column copy every iteration.
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

    // Action matrix for multiplication by `z` on the quotient basis `[1,x,y,z,x2,y2,z2,xy,xz,yz]`
    // (columns 10..20 of `reduced`, i.e. `B`). Columns 0-3 (`z*1=z`, `z*x=xz`, `z*y=yz`, `z*z=z2`) land
    // directly on another quotient-basis monomial; columns 4-9 overflow into a degree-3 monomial, which
    // `B`'s row for that monomial (from `MONO3`'s degree-3 ordering) re-expresses back in the quotient
    // basis: `x2*z=x2z` (D3 row 4), `y2*z=y2z` (row 6), `z2*z=z3` (row 2), `xy*z=xyz` (row 9),
    // `xz*z=xz2` (row 7), `yz*z=yz2` (row 8).
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
    // Stickelberger's theorem gives the evaluation-at-root vector `(1,x,y,z,x2,y2,z2,xy,xz,yz)` as a
    // *left* eigenvector of the action matrix (`q . T_z = z0 . q`), not a right eigenvector of `T_z`
    // itself — equivalently, a right eigenvector of `T_z`'s transpose (`T_z^T . q = z0 . q`). Verified
    // empirically against a known ground-truth root during development (a right-eigenvector-of-`T_z`
    // attempt was off by O(1), while the transpose recovers it to ~1e-13).
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
    // Deliberately plain `ransac`, not `lo_ransac` with a [`fit_fundamental_dlt`] local-optimization
    // refit: that linear 8-point-style refit is exactly the estimator this solver exists to outperform
    // on planar/low-parallax data, so refitting through it on every improved inlier set would silently
    // reintroduce the same degeneracy the five-point solver is meant to survive (caught by this crate's
    // own planar-scene test: the DLT refit was winning over the correct minimal-sample essential matrix).
    // The nonlinear polish below is safe where the DLT refit isn't: it stays on the essential matrix's
    // true 5-DOF manifold instead of an unconstrained 8-DOF linear one.
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
    poses: &'a [(CameraPose, Intrinsics)],
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
    // ⏱️ A three-parameter point from a DLT start converges in a handful of damped steps; fifty
    // bought nothing and made one bounded retriangulation unit (which may refine twice, on the full
    // track and on its agreeing subset) overrun the worker ceiling.
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

// #region 🔖️Tracks

/// 🧵️ Iterative (no recursion, wasm32-safe) union-find over observation node indices, with path-halving.
struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self { parent: (0..n).collect() }
    }

    fn find(&mut self, mut x: usize) -> usize {
        while self.parent[x] != x {
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }
        x
    }

    fn union(&mut self, a: usize, b: usize) {
        let (ra, rb) = (self.find(a), self.find(b));
        if ra != rb {
            self.parent[ra] = rb;
        }
    }
}

/// 🧵️ A feature track: one connected component of `(frame index, keypoint index in that frame)`
/// observations, linked transitively across pairwise matches.
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
pub struct FeatureTracks {
    pub tracks: Vec<Vec<(usize, u32)>>,
}

/// 🧵️ Builds tracks via union-find over every `(frame, keypoint)` observation touched by
/// `pairwise_matches`: each `Match { a, b, .. }` in a `(frame_a, frame_b, matches)` triple unions
/// `(frame_a, a)` with `(frame_b, b)`. A track that would end up with two observations from the *same*
/// frame (an inconsistent match chain — the union merged two genuinely different points from that frame
/// via a bad transitive match) is dropped entirely rather than repaired, since picking a "correct"
/// sub-chain from a corrupted component isn't well-defined without additional geometric evidence; a
/// clean re-match/retriangulation later naturally recovers the good sub-tracks.
pub fn build_tracks(num_frames: usize, pairwise_matches: &[(usize, usize, Vec<Match>)]) -> FeatureTracks {
    let _ = num_frames;
    let mut node_of: std::collections::HashMap<(usize, u32), usize> = std::collections::HashMap::new();
    let mut obs_of: Vec<(usize, u32)> = Vec::new();
    for (frame_a, frame_b, matches) in pairwise_matches {
        for m in matches {
            node_of.entry((*frame_a, m.a)).or_insert_with(|| {
                obs_of.push((*frame_a, m.a));
                obs_of.len() - 1
            });
            node_of.entry((*frame_b, m.b)).or_insert_with(|| {
                obs_of.push((*frame_b, m.b));
                obs_of.len() - 1
            });
        }
    }
    let mut uf = UnionFind::new(obs_of.len());
    for (frame_a, frame_b, matches) in pairwise_matches {
        for m in matches {
            let na = node_of[&(*frame_a, m.a)];
            let nb = node_of[&(*frame_b, m.b)];
            uf.union(na, nb);
        }
    }
    let mut groups: std::collections::HashMap<usize, Vec<(usize, u32)>> = std::collections::HashMap::new();
    for (idx, &obs) in obs_of.iter().enumerate() {
        let root = uf.find(idx);
        groups.entry(root).or_default().push(obs);
    }
    let mut tracks: Vec<Vec<(usize, u32)>> = Vec::new();
    for (_, mut obs) in groups {
        obs.sort_unstable();
        let has_conflict = obs.windows(2).any(|w| w[0].0 == w[1].0);
        if !has_conflict {
            tracks.push(obs);
        }
    }
    tracks.sort();
    FeatureTracks { tracks }
}

// #endregion 🔖️Tracks

// #region 🔖️Global

/// 🕸️ Adjacency list from relative-rotation edges: `(i,j,Rij)` with `Rj = Rij ∘ Ri` contributes `i -> (j,
/// Rij)` and the inverse edge `j -> (i, Rij⁻¹)`.
fn rotation_adjacency(n: usize, edges: &[(usize, usize, So3)]) -> Vec<Vec<(usize, So3)>> {
    let mut adj = vec![Vec::new(); n];
    for &(i, j, rij) in edges {
        if i < n && j < n {
            adj[i].push((j, rij));
            adj[j].push((i, rij.inverse()));
        }
    }
    adj
}

/// 🕸️ Simplified global rotation averaging: a BFS spanning-tree composition from anchor node 0 gives an
/// initial guess (exact whenever the relative rotations are noiseless and the graph is connected), then a
/// fixed number of IRLS sweeps refine every non-anchor node by averaging, in its own `so(3)` tangent
/// space, the rotation implied by each neighboring edge (`Ri ≈ Rij⁻¹ Rj`). This is a documented
/// simplification of full chordal-L2 rotation averaging (which solves a single `3n x 3n` linear
/// eigenproblem): per-node tangent averaging around the current estimate converges well for the
/// small-to-moderate noise and graph sizes this crate targets, but is not the eigendecomposition-exact
/// global optimum. Node 0 is always returned as [`So3::identity`] (the gauge anchor); nodes unreachable
/// from it also default to identity.
pub fn rotation_averaging(relative_rotations: &[(usize, usize, So3)]) -> Vec<So3> {
    let n = relative_rotations.iter().map(|&(i, j, _)| i.max(j)).max().map_or(0, |m| m + 1);
    if n == 0 {
        return Vec::new();
    }
    let adj = rotation_adjacency(n, relative_rotations);
    let mut rotations = vec![So3::identity(); n];
    let mut visited = vec![false; n];
    visited[0] = true;
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(0usize);
    while let Some(i) = queue.pop_front() {
        for &(j, rij) in &adj[i] {
            if !visited[j] {
                visited[j] = true;
                rotations[j] = rij.semio_compose_rs(&rotations[i]);
                queue.push_back(j);
            }
        }
    }
    for _ in 0..10 {
        for i in 1..n {
            if adj[i].is_empty() {
                continue;
            }
            let mut sum_tangent = [0.0; 3];
            for &(j, rij) in &adj[i] {
                let implied = rij.inverse().semio_compose_rs(&rotations[j]);
                let delta = implied.semio_compose_rs(&rotations[i].inverse()).log();
                sum_tangent = add3(sum_tangent, delta);
            }
            let avg = scale3(sum_tangent, 1.0 / adj[i].len() as f64);
            rotations[i] = So3::exp(avg).semio_compose_rs(&rotations[i]);
        }
    }
    rotations
}

/// 📐️ Simplified global translation averaging from baseline-direction constraints: for edge `(i, j,
/// local_dir)`, `local_dir` is the unit direction from camera `i`'s center to camera `j`'s center
/// *expressed in camera `i`'s local frame* (the same convention as the `t` component of the relative pose
/// [`decompose_essential`] returns). Each edge contributes the linear cross-product constraint `(Cj - Ci)
/// x world_dir_ij = 0` (rotated into the world frame via `rotations[i]`), stacked into one large
/// homogeneous system solved via [`svd_nullvector`]/gram-nullspace for the camera centers `C1..C(n-1)`
/// (`C0` fixed at the origin). Direction-only constraints carry no absolute distance information, so the
/// returned centers are correct only up to a single unknown global scale — the scale of the unit-norm
/// nullspace solution itself, not any physical unit. Callers needing metric scale should rescale via
/// [`align_to_priors`] or another absolute reference.
pub fn translation_averaging(relative_directions: &[(usize, usize, [f64; 3])], rotations: &[So3]) -> Vec<[f64; 3]> {
    let n = rotations.len();
    if n == 0 {
        return Vec::new();
    }
    if n == 1 || relative_directions.is_empty() {
        return vec![[0.0; 3]; n];
    }
    let col_of = |node: usize| -> Option<usize> {
        if node == 0 {
            None
        } else {
            Some(3 * (node - 1))
        }
    };
    let mut design = MatD::zeros(3 * relative_directions.len(), 3 * (n - 1));
    for (row, &(i, j, local_dir)) in relative_directions.iter().enumerate() {
        if i >= n || j >= n {
            continue;
        }
        let world_dir = vec3d_normalize(rotations[i].inverse().act(local_dir));
        let (dx, dy, dz) = (world_dir[0], world_dir[1], world_dir[2]);
        let skew_rows = [[0.0, -dz, dy], [dz, 0.0, -dx], [-dy, dx, 0.0]];
        for (r, skew_row) in skew_rows.iter().enumerate() {
            if let Some(cj) = col_of(j) {
                for (c, &value) in skew_row.iter().enumerate() {
                    design.add_at(3 * row + r, cj + c, value);
                }
            }
            if let Some(ci) = col_of(i) {
                for (c, &value) in skew_row.iter().enumerate() {
                    design.add_at(3 * row + r, ci + c, -value);
                }
            }
        }
    }
    let mut centers = vec![[0.0; 3]; n];
    if let Some(sol) = nullvector_via_gram(&design) {
        for (node, center) in centers.iter_mut().enumerate().skip(1) {
            let base = 3 * (node - 1);
            *center = [sol.get(base), sol.get(base + 1), sol.get(base + 2)];
        }
    }
    centers
}

// #endregion 🔖️Global

// #region 🔖️LoopClosure

const LSH_TABLES: usize = 4;

/// 🔎️ LSH keyframe index over [`Descriptor256`]s: 4 hash tables, each keyed by one 256-bit descriptor's
/// low 16 bits of a distinct `u64` word (words 0..3, i.e. bits `[0,16)`, `[64,80)`, `[128,144)`,
/// `[192,208)` — one substring per word for spread across the descriptor), with no learned vocabulary.
#[derive(Clone, Debug, Default)]
pub struct KeyframeIndex {
    tables: [std::collections::HashMap<u16, Vec<usize>>; LSH_TABLES],
}

impl KeyframeIndex {
    /// 🆕️ Empty index.
    pub fn new() -> Self {
        Self::default()
    }

    /// ➕️ Buckets `frame`'s descriptors into all 4 LSH tables (duplicate insertions of the same frame into
    /// the same bucket are harmless — `candidates` just counts them as extra votes).
    pub fn insert(&mut self, frame: usize, descriptors: &[Descriptor256]) {
        for desc in descriptors {
            for (t, table) in self.tables.iter_mut().enumerate() {
                table.entry((desc.0[t] & 0xFFFF) as u16).or_default().push(frame);
            }
        }
    }

    /// 🔍️ Frames voted for by at least `min_shared_buckets` total `(query descriptor, table)` bucket hits
    /// — i.e. the sum, over every query descriptor and every one of the 4 tables, of how many times that
    /// frame appears in the matching bucket. Ascending by frame index.
    pub fn candidates(&self, descriptors: &[Descriptor256], min_shared_buckets: usize) -> Vec<usize> {
        let mut votes: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
        for desc in descriptors {
            for (t, table) in self.tables.iter().enumerate() {
                if let Some(frames) = table.get(&((desc.0[t] & 0xFFFF) as u16)) {
                    for &frame in frames {
                        *votes.entry(frame).or_insert(0) += 1;
                    }
                }
            }
        }
        let mut out: Vec<usize> = votes.into_iter().filter(|&(_, count)| count >= min_shared_buckets).map(|(frame, _)| frame).collect();
        out.sort_unstable();
        out
    }
}

/// 📦️ A geometrically-verified loop-closure candidate: the revisited frame, its inlier count, and the recovered two-view model.
#[derive(Clone, Debug, PartialEq)]
pub struct LoopCandidate {
    pub frame: usize,
    pub inlier_count: usize,
    pub model: TwoViewModel,
}

/// 🔁️ Loop-closure detection: gathers LSH candidate frames from `index`, matches `current_descriptors`
/// against each candidate's descriptors ([`remodeling_feature::match_brute`], ratio `0.8`, mutual
/// cross-check), geometrically verifies via [`select_two_view_model`], and keeps candidates clearing a
/// minimum inlier count — descending by inlier count.
pub fn detect_loops(index: &KeyframeIndex, current_frame: usize, current_descriptors: &[Descriptor256], current_keypoints: &[Keypoint], all_keypoints: &[Vec<Keypoint>], all_descriptors: &[Vec<Descriptor256>]) -> Vec<LoopCandidate> {
    const MIN_SHARED_BUCKETS: usize = 3;
    const MIN_LOOP_INLIERS: usize = 12;
    let mut out = Vec::new();
    for frame in index.candidates(current_descriptors, MIN_SHARED_BUCKETS) {
        if frame == current_frame || frame >= all_keypoints.len() || frame >= all_descriptors.len() {
            continue;
        }
        let matches = match_brute(current_descriptors, &all_descriptors[frame], 0.8, true);
        if matches.len() < 8 {
            continue;
        }
        let corr: Vec<([f64; 2], [f64; 2])> = matches
            .iter()
            .filter(|m| (m.a as usize) < current_keypoints.len() && (m.b as usize) < all_keypoints[frame].len())
            .map(|m| {
                let a = current_keypoints[m.a as usize];
                let b = all_keypoints[frame][m.b as usize];
                ([f64::from(a.x), f64::from(a.y)], [f64::from(b.x), f64::from(b.y)])
            })
            .collect();
        if let Some(result) = select_two_view_model(&corr) {
            if result.inliers.len() >= MIN_LOOP_INLIERS {
                out.push(LoopCandidate { frame, inlier_count: result.inliers.len(), model: result.model });
            }
        }
    }
    out.sort_by_key(|c| std::cmp::Reverse(c.inlier_count));
    out
}

/// 🧩️ Sim3 pose-graph consistency problem, simplified to `se(3)` (rotation + translation only, dropping
/// each edge's scale dimension): parameters are one 6-dof `se(3)` log-tangent per non-anchor node (node 0
/// is fixed to `poses[0]`), and each edge residual is `log(Zij⁻¹ (Pj Pi⁻¹))` where `Zij` is the edge's
/// `Se3` part (`Sim3`'s `r`/`t`, ignoring `s`). A full Sim3-space optimization would carry the extra scale
/// tangent per node/edge; dropping it is the documented simplification here, appropriate when the input
/// poses/edges are already metrically consistent (e.g. from a calibrated monocular pipeline sharing one
/// global scale) and only rotation/translation drift needs correcting.
struct PoseGraphProblem<'a> {
    fixed_pose0: Se3,
    edges: &'a [(usize, usize, Sim3)],
    num_nodes: usize,
}

impl PoseGraphProblem<'_> {
    fn pose_at(&self, x: &VecD, node: usize) -> Se3 {
        if node == 0 {
            self.fixed_pose0
        } else {
            let base = 6 * (node - 1);
            Se3::exp(std::array::from_fn(|k| x.get(base + k)))
        }
    }
}

impl LeastSquaresProblem for PoseGraphProblem<'_> {
    fn residual_count(&self) -> usize {
        self.edges.len() * 6
    }

    fn parameter_count(&self) -> usize {
        6 * self.num_nodes.saturating_sub(1)
    }

    fn residuals(&self, x: &VecD, out: &mut VecD) {
        for (row, &(i, j, zij)) in self.edges.iter().enumerate() {
            let pi = self.pose_at(x, i);
            let pj = self.pose_at(x, j);
            let predicted = pj.semio_compose_rs(&pi.inverse());
            let z_se3 = Se3 { r: zij.r, t: zij.t };
            let err = z_se3.inverse().semio_compose_rs(&predicted).log();
            for (k, value) in err.into_iter().enumerate() {
                out.set(row * 6 + k, value);
            }
        }
    }

    fn jacobian(&self, x: &VecD, out: &mut MatD) {
        numeric_jacobian(self, x, 1e-6, out);
    }

    fn plus(&self, x: &VecD, dx: &VecD) -> VecD {
        let mut out = VecD::zeros(x.len());
        for node in 1..self.num_nodes {
            let base = 6 * (node - 1);
            let cur: [f64; 6] = std::array::from_fn(|k| x.get(base + k));
            let ddelta: [f64; 6] = std::array::from_fn(|k| dx.get(base + k));
            let updated = Se3::exp(ddelta).semio_compose_rs(&Se3::exp(cur)).log();
            for (k, value) in updated.into_iter().enumerate() {
                out.set(base + k, value);
            }
        }
        out
    }
}

/// 🔁️ Sim3-edge pose-graph optimization (see [`PoseGraphProblem`] for the documented `se(3)`
/// simplification): refines `poses` via Levenberg-Marquardt to best satisfy `edges`, anchoring `poses[0]`.
pub fn pose_graph_optimize(poses: &[Se3], edges: &[(usize, usize, Sim3)]) -> Vec<Se3> {
    let n = poses.len();
    if n == 0 {
        return Vec::new();
    }
    let problem = PoseGraphProblem { fixed_pose0: poses[0], edges, num_nodes: n };
    let mut x0 = VecD::zeros(problem.parameter_count());
    for (node, pose) in poses.iter().enumerate().skip(1) {
        let base = 6 * (node - 1);
        for (k, v) in pose.log().into_iter().enumerate() {
            x0.set(base + k, v);
        }
    }
    let cfg = LmConfig { max_iters: 100, ..LmConfig::default() };
    let result = semio_s_artifact_remodel_remodeling_optimize::standards::v1::subsets::any::schema::optimize_internals::levenberg_marquardt(&problem, x0, &cfg);
    (0..n).map(|node| problem.pose_at(&result.x, node)).collect()
}

// #endregion 🔖️LoopClosure
