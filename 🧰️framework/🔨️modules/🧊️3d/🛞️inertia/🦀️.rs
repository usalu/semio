//! 🛞️ Rigid-body inertia algebra shared by the B-Rep and mesh analysis queries: a symmetric 3x3
//! Jacobi eigen solver, principal moments and axes, and the parallel-axis theorem. First-party and
//! dependency-free; `nalgebra`'s `SymmetricEigen` is the differential oracle in the unit tests only.
//!
//! 🔗️ [Jacobi eigenvalue algorithm](https://en.wikipedia.org/wiki/Jacobi_eigenvalue_algorithm)

/// 🎡 A symmetric 3x3 inertia tensor in row-major order.
pub type InertiaTensor = [[f64; 3]; 3];

/// 🧭 Eigenvalues (ascending) and the unit eigenvector of each, right-handed (`axes[2] = axes[0] x axes[1]`).
#[derive(Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::protocol::value")]
pub struct SymmetricEigen3 {
    pub values: [f64; 3],
    pub axes: [[f64; 3]; 3],
}

/// 🧲 Principal moments (ascending) and the principal axes they belong to.
pub type PrincipalInertia = SymmetricEigen3;

const MAX_SWEEPS: usize = 64;

/// 🧮 Diagonalises a symmetric 3x3 matrix with cyclic Jacobi rotations.
///
/// Repeated eigenvalues leave their eigenspace axes arbitrary but orthonormal; each of the first
/// two axes is signed so its largest component is positive, which makes the result deterministic.
pub fn symmetric_eigen3(matrix: [[f64; 3]; 3]) -> SymmetricEigen3 {
    let mut a = matrix;
    let mut v = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
    let scale = a.iter().flatten().fold(0.0_f64, |m, x| m.max(x.abs())).max(f64::MIN_POSITIVE);
    for _ in 0..MAX_SWEEPS {
        let off = a[0][1] * a[0][1] + a[0][2] * a[0][2] + a[1][2] * a[1][2];
        if off <= 1e-32 * scale * scale {
            break;
        }
        for (p, q) in [(0, 1), (0, 2), (1, 2)] {
            if a[p][q] == 0.0 {
                continue;
            }
            let theta = (a[q][q] - a[p][p]) / (2.0 * a[p][q]);
            let t = theta.signum() / (theta.abs() + (theta * theta + 1.0).sqrt());
            let t = if theta == 0.0 { 1.0 } else { t };
            let c = 1.0 / (t * t + 1.0).sqrt();
            let s = t * c;
            for k in 0..3 {
                let (akp, akq) = (a[k][p], a[k][q]);
                a[k][p] = c * akp - s * akq;
                a[k][q] = s * akp + c * akq;
            }
            for k in 0..3 {
                let (apk, aqk) = (a[p][k], a[q][k]);
                a[p][k] = c * apk - s * aqk;
                a[q][k] = s * apk + c * aqk;
            }
            for row in v.iter_mut() {
                let (vp, vq) = (row[p], row[q]);
                row[p] = c * vp - s * vq;
                row[q] = s * vp + c * vq;
            }
        }
    }
    let mut order = [0usize, 1, 2];
    order.sort_by(|&i, &j| a[i][i].total_cmp(&a[j][j]));
    let values = [a[order[0]][order[0]], a[order[1]][order[1]], a[order[2]][order[2]]];
    let mut axes = [[0.0; 3]; 3];
    for (slot, &column) in order.iter().enumerate() {
        axes[slot] = [v[0][column], v[1][column], v[2][column]];
    }
    for axis in axes.iter_mut().take(2) {
        let dominant = (0..3).max_by(|&i, &j| axis[i].abs().total_cmp(&axis[j].abs())).unwrap_or(0);
        if axis[dominant] < 0.0 {
            axis.iter_mut().for_each(|x| *x = -*x);
        }
    }
    axes[2] = cross(axes[0], axes[1]);
    SymmetricEigen3 { values, axes }
}

/// 🔁 Principal moments and axes of an inertia tensor about its own reference point.
pub fn principal_inertia(tensor: InertiaTensor) -> PrincipalInertia {
    symmetric_eigen3(tensor)
}

/// 🪩 Parallel-axis theorem: the tensor about a point displaced by `offset` from the centroid,
/// `I' = I + m (|d|^2 E - d d^T)`.
pub fn parallel_axis(about_centroid: InertiaTensor, mass: f64, offset: [f64; 3]) -> InertiaTensor {
    let d2 = offset[0] * offset[0] + offset[1] * offset[1] + offset[2] * offset[2];
    let mut out = about_centroid;
    for (i, row) in out.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            let delta = if i == j { d2 } else { 0.0 };
            *cell += mass * (delta - offset[i] * offset[j]);
        }
    }
    out
}

/// ⚙️ Inverse of [`parallel_axis`]: the centroid tensor from one taken about a displaced point.
pub fn to_centroid(about_point: InertiaTensor, mass: f64, offset_of_centroid: [f64; 3]) -> InertiaTensor {
    let moved = parallel_axis([[0.0; 3]; 3], mass, offset_of_centroid);
    let mut out = about_point;
    for (i, row) in out.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            *cell -= moved[i][j];
        }
    }
    out
}

/// 🧩 One rigid piece of a composite: its mass, centroid and tensor about that centroid.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InertiaPiece {
    pub mass: f64,
    pub centroid: [f64; 3],
    pub about_centroid: InertiaTensor,
}

/// 🛞️ Total mass, centroid and centroid tensor of disjoint pieces (unit density, so mass = volume).
pub fn combine(pieces: &[InertiaPiece]) -> Option<InertiaPiece> {
    let mass: f64 = pieces.iter().map(|piece| piece.mass).sum();
    if mass == 0.0 || !mass.is_finite() {
        return None;
    }
    let mut centroid = [0.0; 3];
    for piece in pieces {
        for axis in 0..3 {
            centroid[axis] += piece.mass * piece.centroid[axis];
        }
    }
    centroid.iter_mut().for_each(|x| *x /= mass);
    let mut tensor = [[0.0; 3]; 3];
    for piece in pieces {
        let offset = [piece.centroid[0] - centroid[0], piece.centroid[1] - centroid[1], piece.centroid[2] - centroid[2]];
        let shifted = parallel_axis(piece.about_centroid, piece.mass, offset);
        for i in 0..3 {
            for j in 0..3 {
                tensor[i][j] += shifted[i][j];
            }
        }
    }
    Some(InertiaPiece { mass, centroid, about_centroid: tensor })
}

fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
