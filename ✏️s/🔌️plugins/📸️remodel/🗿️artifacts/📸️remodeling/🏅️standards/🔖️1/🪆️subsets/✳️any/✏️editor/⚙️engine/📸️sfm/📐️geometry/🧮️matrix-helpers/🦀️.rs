//! 🧮️ Matrix helpers of the multi-view geometry: the small vector and matrix functions every model shares. See [`super`].

use super::*;

// #region 🔖️Mat3Helpers
pub fn dot3(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

pub fn scale3(v: [f64; 3], s: f64) -> [f64; 3] {
    [v[0] * s, v[1] * s, v[2] * s]
}

pub(super) fn add3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
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
pub(super) fn nullspace_via_gram(a: &MatD, k: usize) -> Option<Vec<VecD>> {
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

pub(super) fn nullvector_via_gram(a: &MatD) -> Option<VecD> {
    nullspace_via_gram(a, 1)?.into_iter().next()
}

pub(super) fn mat3_vec(m: &[[f64; 3]; 3], v: [f64; 3]) -> [f64; 3] {
    std::array::from_fn(|r| m[r][0] * v[0] + m[r][1] * v[1] + m[r][2] * v[2])
}

/// ⚔️ Skew-symmetric cross-product matrix `[v]_x`, so `[v]_x . w == v x w`.
pub(super) fn skew3(v: [f64; 3]) -> [[f64; 3]; 3] {
    [[0.0, -v[2], v[1]], [v[2], 0.0, -v[0]], [-v[1], v[0], 0.0]]
}

pub(super) fn mat3_mul(a: &[[f64; 3]; 3], b: &[[f64; 3]; 3]) -> [[f64; 3]; 3] {
    std::array::from_fn(|r| std::array::from_fn(|c| (0..3).map(|k| a[r][k] * b[k][c]).sum()))
}

pub(super) fn mat3_transpose(m: &[[f64; 3]; 3]) -> [[f64; 3]; 3] {
    std::array::from_fn(|r| std::array::from_fn(|c| m[c][r]))
}

pub(super) fn mat3_det(m: &[[f64; 3]; 3]) -> f64 {
    m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1]) - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0]) + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
}

pub(super) fn mat3_from_matd(m: &MatD) -> [[f64; 3]; 3] {
    std::array::from_fn(|r| std::array::from_fn(|c| m.get(r, c)))
}

pub(super) fn matd_from_mat3(m: &[[f64; 3]; 3]) -> MatD {
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
pub(super) fn array_to_mat3d(m: &[[f64; 3]; 3]) -> Mat3d {
    Mat3d::from_axes([m[0][0], m[1][0], m[2][0]], [m[0][1], m[1][1], m[2][1]], [m[0][2], m[1][2], m[2][2]])
}

/// 🎯️ World-space optical center of a camera pose: `-Rᵀt`, the point mapping to the camera-space origin.
pub fn camera_center(pose: &CameraPose) -> [f64; 3] {
    let r_inv = pose.0.r.inverse();
    scale3(r_inv.act(pose.0.t), -1.0)
}
// #endregion 🔖️Mat3Helpers
