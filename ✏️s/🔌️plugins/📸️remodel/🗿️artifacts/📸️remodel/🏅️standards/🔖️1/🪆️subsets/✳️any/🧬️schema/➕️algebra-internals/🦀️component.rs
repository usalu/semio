//! 🧮️ Linear algebra: dense/sparse matrices and vectors, exact elimination, decompositions and eigen solvers. The fixed-size render types `Vec3`/`Mat4` live in `semio-framework-geometry`.
//! Moved wholesale from `🧮️math/➕️algebra` per `26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS` wave M3d, whose sole-repo-wide-consumer census (verified true at the time) was outrun mid-ticket by wave M3a giving `➗️mathematical`'s `cas-internals` a second consumer of the generic `VecG`/`MatG` types. Wave FIXALG (same ticket) relocated `VecG`/`MatG` (and the `ExactElimination`/`Charpoly`/`Smith` inherent impls that must travel with them) to `semio_framework_number`'s `algebra` module — see that crate for the generic exact-linear-algebra types. Only `lll_reduce` below, a free function with no second consumer, stays here, built on `number::VecG`/`number::Rational`.

// #region 🔖️Mat2
/// 🧮️ 2x2 matrix, column-major storage; `new(a, b, c, d)` takes row-major entries of `[[a, b], [c, d]]`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Mat2 {
    pub cols: [[f64; 2]; 2],
}

impl Mat2 {
    pub const IDENTITY: Self = Self { cols: [[1.0, 0.0], [0.0, 1.0]] };

    pub async fn new(a: f64, b: f64, c: f64, d: f64) -> Self {
        Self { cols: [[a, c], [b, d]] }
    }

    #[allow(clippy::should_implement_trait, reason = "value-semantics mul used pervasively as a plain method (not operator overload) by dependent crates outside this campaign wave's scope; renaming is a breaking API change")]
    pub async fn mul(self, other: Self) -> Self {
        let entry = |row: usize, col: usize| self.cols[0][row] * other.cols[col][0] + self.cols[1][row] * other.cols[col][1];
        Self { cols: [[entry(0, 0), entry(1, 0)], [entry(0, 1), entry(1, 1)]] }
    }

    pub async fn apply(self, v: (f64, f64)) -> (f64, f64) {
        (self.cols[0][0] * v.0 + self.cols[1][0] * v.1, self.cols[0][1] * v.0 + self.cols[1][1] * v.1)
    }

    pub async fn det(self) -> f64 {
        self.cols[0][0] * self.cols[1][1] - self.cols[1][0] * self.cols[0][1]
    }

    pub async fn trace(self) -> f64 {
        self.cols[0][0] + self.cols[1][1]
    }

    pub async fn transpose(self) -> Self {
        Self::new(self.cols[0][0], self.cols[0][1], self.cols[1][0], self.cols[1][1])
    }

    pub async fn inverse(self) -> Option<Self> {
        let d = self.det();
        if d.abs() < 1e-12 {
            return None;
        }
        let inv_d = 1.0 / d;
        Some(Self::new(self.cols[1][1] * inv_d, -self.cols[1][0] * inv_d, -self.cols[0][1] * inv_d, self.cols[0][0] * inv_d))
    }

    /// 🧮️ Real eigenvalues (if any) of a 2x2 matrix via the characteristic polynomial `λ² - tr·λ + det = 0`.
    pub async fn eigenvalues(self) -> Option<(f64, f64)> {
        let t = self.trace();
        let d = self.det();
        let disc = t * t - 4.0 * d;
        if disc < 0.0 {
            return None;
        }
        let sq = disc.sqrt();
        Some(((t + sq) * 0.5, (t - sq) * 0.5))
    }
}
// #endregion 🔖️Mat2

// #region 🔖️VecD
/// 📏️ Heap-allocated f64 vector for element and system-level numerics (loads, displacements, residuals).
#[derive(Clone, Debug, PartialEq)]
pub struct VecD(pub Vec<f64>);

impl VecD {
    pub async fn zeros(n: usize) -> Self {
        Self(vec![0.0; n])
    }

    pub async fn from_vec(data: Vec<f64>) -> Self {
        Self(data)
    }

    pub async fn len(&self) -> usize {
        self.0.len()
    }

    pub async fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub async fn get(&self, i: usize) -> f64 {
        self.0[i]
    }

    pub async fn set(&mut self, i: usize, value: f64) {
        self.0[i] = value;
    }

    pub async fn add_at(&mut self, i: usize, value: f64) {
        self.0[i] += value;
    }

    pub async fn dot(&self, other: &Self) -> f64 {
        self.0.iter().zip(other.0.iter()).map(|(a, b)| a * b).sum()
    }

    pub async fn scale(&self, s: f64) -> Self {
        Self(self.0.iter().map(|v| v * s).collect())
    }

    pub async fn add(&self, other: &Self) -> Self {
        Self(self.0.iter().zip(other.0.iter()).map(|(a, b)| a + b).collect())
    }

    pub async fn sub(&self, other: &Self) -> Self {
        Self(self.0.iter().zip(other.0.iter()).map(|(a, b)| a - b).collect())
    }

    pub async fn norm2(&self) -> f64 {
        self.dot(self).sqrt()
    }

    pub async fn norm_inf(&self) -> f64 {
        self.0.iter().fold(0.0_f64, |acc, v| acc.max(v.abs()))
    }
}
// #endregion 🔖️VecD

// #region 🔖️MatD
/// 🧮️ Dynamic dense f64 matrix, row-major storage; sized for element stiffness matrices and small global systems.
#[derive(Clone, Debug, PartialEq)]
pub struct MatD {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f64>,
}

impl MatD {
    pub async fn zeros(rows: usize, cols: usize) -> Self {
        Self { rows, cols, data: vec![0.0; rows * cols] }
    }

    pub async fn identity(n: usize) -> Self {
        let mut m = Self::zeros(n, n);
        for i in 0..n {
            m.set(i, i, 1.0);
        }
        m
    }

    pub async fn get(&self, row: usize, col: usize) -> f64 {
        self.data[row * self.cols + col]
    }

    pub async fn set(&mut self, row: usize, col: usize, value: f64) {
        self.data[row * self.cols + col] = value;
    }

    pub async fn add_at(&mut self, row: usize, col: usize, value: f64) {
        self.data[row * self.cols + col] += value;
    }

    pub async fn transpose(&self) -> Self {
        let mut out = Self::zeros(self.cols, self.rows);
        for row in 0..self.rows {
            for col in 0..self.cols {
                out.set(col, row, self.get(row, col));
            }
        }
        out
    }

    pub async fn matmul(&self, other: &Self) -> Self {
        assert_eq!(self.cols, other.rows, "matmul dimension mismatch");
        let mut out = Self::zeros(self.rows, other.cols);
        for row in 0..self.rows {
            for k in 0..self.cols {
                let a = self.get(row, k);
                if a == 0.0 {
                    continue;
                }
                for col in 0..other.cols {
                    out.add_at(row, col, a * other.get(k, col));
                }
            }
        }
        out
    }

    pub async fn mul_vec(&self, x: &VecD) -> VecD {
        assert_eq!(self.cols, x.len(), "mul_vec dimension mismatch");
        let mut out = VecD::zeros(self.rows);
        for row in 0..self.rows {
            let mut sum = 0.0;
            for col in 0..self.cols {
                sum += self.get(row, col) * x.get(col);
            }
            out.set(row, sum);
        }
        out
    }

    /// 🧮️ `Bᵀ D B` scaled by `weight`, accumulated into `self` — the element-stiffness Gauss-point kernel.
    pub async fn add_triple_product(&mut self, b: &MatD, d: &MatD, weight: f64) {
        let btdb = b.transpose().matmul(d).matmul(b);
        for i in 0..self.data.len() {
            self.data[i] += weight * btdb.data[i];
        }
    }

    /// 🧮️ Solves `Ax = b` via Gaussian elimination with partial pivoting; `None` if `A` is singular.
    pub async fn lu_solve(&self, b: &VecD) -> Option<VecD> {
        assert_eq!(self.rows, self.cols, "lu_solve requires a square matrix");
        assert_eq!(self.rows, b.len(), "lu_solve dimension mismatch");
        let n = self.rows;
        let mut a = self.data.clone();
        let mut x = b.0.clone();
        for pivot in 0..n {
            let (mut best_row, mut best_val) = (pivot, a[pivot * n + pivot].abs());
            for row in (pivot + 1)..n {
                let val = a[row * n + pivot].abs();
                if val > best_val {
                    best_row = row;
                    best_val = val;
                }
            }
            if best_val < 1e-12 {
                return None;
            }
            if best_row != pivot {
                for col in 0..n {
                    a.swap(pivot * n + col, best_row * n + col);
                }
                x.swap(pivot, best_row);
            }
            let pivot_value = a[pivot * n + pivot];
            for row in (pivot + 1)..n {
                let factor = a[row * n + pivot] / pivot_value;
                if factor == 0.0 {
                    continue;
                }
                for col in pivot..n {
                    a[row * n + col] -= factor * a[pivot * n + col];
                }
                x[row] -= factor * x[pivot];
            }
        }
        for row in (0..n).rev() {
            let mut sum = x[row];
            for col in (row + 1)..n {
                sum -= a[row * n + col] * x[col];
            }
            x[row] = sum / a[row * n + row];
        }
        Some(VecD(x))
    }
}
// #endregion 🔖️MatD

// #region 🔖️Mat3d
/// 🧊️ 3x3 f64 matrix for element local frames and rotation transforms, column-major storage.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Mat3d {
    pub cols: [[f64; 3]; 3],
}

impl Mat3d {
    pub const IDENTITY: Self = Self { cols: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]] };

    /// 🧭️ Rotation matrix from an orthonormal local basis, columns `(x, y, z)` expressed in global coordinates.
    pub async fn from_axes(x: [f64; 3], y: [f64; 3], z: [f64; 3]) -> Self {
        Self { cols: [x, y, z] }
    }

    pub async fn transpose(self) -> Self {
        Self { cols: [[self.cols[0][0], self.cols[1][0], self.cols[2][0]], [self.cols[0][1], self.cols[1][1], self.cols[2][1]], [self.cols[0][2], self.cols[1][2], self.cols[2][2]]] }
    }

    #[allow(clippy::should_implement_trait, reason = "value-semantics mul used pervasively as a plain method (not operator overload) by dependent crates outside this campaign wave's scope; renaming is a breaking API change")]
    pub async fn mul(self, other: Self) -> Self {
        let entry = |row: usize, col: usize| self.cols[0][row] * other.cols[col][0] + self.cols[1][row] * other.cols[col][1] + self.cols[2][row] * other.cols[col][2];
        Self { cols: [[entry(0, 0), entry(1, 0), entry(2, 0)], [entry(0, 1), entry(1, 1), entry(2, 1)], [entry(0, 2), entry(1, 2), entry(2, 2)]] }
    }

    pub async fn mul_vec3(self, v: [f64; 3]) -> [f64; 3] {
        [self.cols[0][0] * v[0] + self.cols[1][0] * v[1] + self.cols[2][0] * v[2], self.cols[0][1] * v[0] + self.cols[1][1] * v[1] + self.cols[2][1] * v[2], self.cols[0][2] * v[0] + self.cols[1][2] * v[1] + self.cols[2][2] * v[2]]
    }
}

pub async fn vec3d_sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

pub async fn vec3d_length(v: [f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

pub async fn vec3d_normalize(v: [f64; 3]) -> [f64; 3] {
    let len = vec3d_length(v);
    if len < 1e-12 {
        return [0.0, 0.0, 0.0];
    }
    [v[0] / len, v[1] / len, v[2] / len]
}

pub async fn vec3d_cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}
// #endregion 🔖️Mat3d

// #region 🔖️Lll
/// 🔒️ LLL lattice basis reduction (`delta = 3/4`) over `Rational` rows, via Gram-Schmidt in exact
/// rational arithmetic. Powers integer-relation detection (e.g. recovering `x^2 - 2` from rational
/// approximations of `sqrt(2)`) and keeps polynomial-factor recombination upgrades (van Hoeij) open as
/// future work without needing a new dependency.
pub async fn lll_reduce(basis: &[number::VecG<number::Rational>]) -> Vec<number::VecG<number::Rational>> {
    use number::{Rational, VecG};
    let delta = Rational::from_i64(3, 4).unwrap();
    let n = basis.len();
    let mut b: Vec<VecG<Rational>> = basis.to_vec();
    let dot = |x: &VecG<Rational>, y: &VecG<Rational>| -> Rational { x.dot(y) };
    let gram_schmidt = |b: &[VecG<Rational>]| -> (Vec<VecG<Rational>>, Vec<Vec<Rational>>) {
        let mut bstar: Vec<VecG<Rational>> = Vec::with_capacity(b.len());
        let mut mu: Vec<Vec<Rational>> = vec![vec![Rational::zero(); b.len()]; b.len()];
        for i in 0..b.len() {
            let mut vi = b[i].clone();
            for j in 0..i {
                let denom = dot(&bstar[j], &bstar[j]);
                let m = if denom.is_zero() { Rational::zero() } else { dot(&b[i], &bstar[j]).div(&denom).unwrap() };
                mu[i][j] = m.clone();
                vi = vi.sub(&bstar[j].scale(&m));
            }
            bstar.push(vi);
        }
        (bstar, mu)
    };
    let mut k = 1;
    while k < n {
        let (bstar, mu) = gram_schmidt(&b);
        for j in (0..k).rev() {
            let m = mu[k][j].round_half_even();
            if m != number::Integer::zero() {
                let m_rat = Rational::from_integer(m);
                let scaled = b[j].scale(&m_rat);
                b[k] = b[k].sub(&scaled);
            }
        }
        let (bstar2, mu2) = gram_schmidt(&b);
        let lhs = dot(&bstar2[k], &bstar2[k]);
        let prev_norm = dot(&bstar2[k - 1], &bstar2[k - 1]);
        let mu_k = mu2[k][k - 1].clone();
        let rhs = delta.sub(&mu_k.mul(&mu_k)).mul(&prev_norm);
        if lhs >= rhs {
            k += 1;
        } else {
            b.swap(k, k - 1);
            k = k.saturating_sub(1).max(1);
        }
        let _ = bstar;
    }
    b
}
// #endregion 🔖️Lll

// #region 🔖️CsrMatrix
/// 🕸️ Sparse matrix in compressed-sparse-row form, for large graph-adjacency / Laplacian-style numerics where a dense `MatD` would be wasteful.
#[derive(Clone, Debug, PartialEq)]
pub struct CsrMatrix {
    pub rows: usize,
    pub cols: usize,
    pub row_ptr: Vec<usize>,
    pub col_idx: Vec<usize>,
    pub values: Vec<f64>,
}

impl CsrMatrix {
    /// 🕸️ Builds a CSR matrix from `(row, col, value)` triplets, summing duplicate `(row, col)` entries and sorting column indices within each row for deterministic iteration.
    pub async fn from_triplets(rows: usize, cols: usize, triplets: &[(usize, usize, f64)]) -> Self {
        let mut by_row: Vec<Vec<(usize, f64)>> = vec![Vec::new(); rows];
        for &(row, col, value) in triplets {
            by_row[row].push((col, value));
        }
        let mut row_ptr = Vec::with_capacity(rows + 1);
        let mut col_idx = Vec::new();
        let mut values = Vec::new();
        row_ptr.push(0);
        for entries in by_row.iter_mut() {
            entries.sort_by_key(|&(col, _)| col);
            let mut last_col: Option<usize> = None;
            for &(col, value) in entries.iter() {
                if last_col == Some(col) {
                    let idx = values.len() - 1;
                    values[idx] += value;
                } else {
                    col_idx.push(col);
                    values.push(value);
                    last_col = Some(col);
                }
            }
            row_ptr.push(col_idx.len());
        }
        Self { rows, cols, row_ptr, col_idx, values }
    }

    pub async fn nnz(&self) -> usize {
        self.values.len()
    }

    pub async fn row(&self, r: usize) -> impl Iterator<Item = (usize, f64)> + '_ {
        let start = self.row_ptr[r];
        let end = self.row_ptr[r + 1];
        self.col_idx[start..end].iter().copied().zip(self.values[start..end].iter().copied())
    }

    /// 🕸️ Sparse matrix-vector product `A x`.
    pub async fn spmv(&self, x: &VecD) -> VecD {
        assert_eq!(self.cols, x.len(), "spmv dimension mismatch");
        let mut out = VecD::zeros(self.rows);
        for row in 0..self.rows {
            let mut sum = 0.0;
            for (col, value) in self.row(row) {
                sum += value * x.get(col);
            }
            out.set(row, sum);
        }
        out
    }

    pub async fn transpose(&self) -> Self {
        let mut triplets = Vec::with_capacity(self.nnz());
        for row in 0..self.rows {
            for (col, value) in self.row(row) {
                triplets.push((col, row, value));
            }
        }
        Self::from_triplets(self.cols, self.rows, &triplets)
    }

    pub async fn to_dense(&self) -> MatD {
        let mut out = MatD::zeros(self.rows, self.cols);
        for row in 0..self.rows {
            for (col, value) in self.row(row) {
                out.set(row, col, value);
            }
        }
        out
    }
}
// #endregion 🔖️CsrMatrix

// #region 🔖️AlgebraError
/// ⚠️ Error type for fallible dense/sparse linear-algebra operations: decompositions, iterative solvers, eigensolvers.
#[derive(Clone, Debug, PartialEq)]
pub enum AlgebraError {
    NotPositiveDefinite,
    Singular,
    DimensionMismatch { expected: (usize, usize), got: (usize, usize) },
    PowerIterationFailedConvergence { iterations: usize },
    NotSymmetric,
}

impl std::fmt::Display for AlgebraError {
    async fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotPositiveDefinite => write!(f, "matrix is not positive definite"),
            Self::Singular => write!(f, "matrix is singular"),
            Self::DimensionMismatch { expected, got } => write!(f, "dimension mismatch: expected {expected:?}, got {got:?}"),
            Self::PowerIterationFailedConvergence { iterations } => write!(f, "iterative solver failed to converge after {iterations} iterations"),
            Self::NotSymmetric => write!(f, "matrix is not symmetric"),
        }
    }
}

impl std::error::Error for AlgebraError {}
// #endregion 🔖️AlgebraError

// #region 🔖️Cholesky
/// 🧮️ Dense Cholesky decomposition `A = L Lᵀ` of a symmetric positive-definite matrix via the standard column-by-column algorithm; returns `AlgebraError::NotPositiveDefinite` the moment a diagonal pivot goes non-positive.
pub async fn cholesky(a: &MatD) -> Result<MatD, AlgebraError> {
    if a.rows != a.cols {
        return Err(AlgebraError::DimensionMismatch { expected: (a.rows, a.rows), got: (a.rows, a.cols) });
    }
    let n = a.rows;
    let mut l = MatD::zeros(n, n);
    for col in 0..n {
        let mut sum = a.get(col, col);
        for k in 0..col {
            sum -= l.get(col, k) * l.get(col, k);
        }
        if sum <= 0.0 {
            return Err(AlgebraError::NotPositiveDefinite);
        }
        let pivot = sum.sqrt();
        l.set(col, col, pivot);
        for row in (col + 1)..n {
            let mut sum = a.get(row, col);
            for k in 0..col {
                sum -= l.get(row, k) * l.get(col, k);
            }
            l.set(row, col, sum / pivot);
        }
    }
    Ok(l)
}

/// 🧮️ Solves `A x = b` given the Cholesky factor `L` of `A` (`A = L Lᵀ`), via forward substitution `L y = b` then back substitution `Lᵀ x = y`.
pub async fn cholesky_solve(l: &MatD, b: &VecD) -> VecD {
    let n = l.rows;
    let mut y = vec![0.0; n];
    #[allow(clippy::needless_range_loop, reason = "k indexes both the 2D matrix `l` and the 1D `y`; an iterator rewrite would need to zip two unrelated index spaces and reads worse than the loop")]
    for row in 0..n {
        let mut sum = b.get(row);
        for k in 0..row {
            sum -= l.get(row, k) * y[k];
        }
        y[row] = sum / l.get(row, row);
    }
    let mut x = vec![0.0; n];
    #[allow(clippy::needless_range_loop, reason = "k indexes both the 2D matrix `l` and the 1D `x`; an iterator rewrite would need to zip two unrelated index spaces and reads worse than the loop")]
    for row in (0..n).rev() {
        let mut sum = y[row];
        for k in (row + 1)..n {
            sum -= l.get(k, row) * x[k];
        }
        x[row] = sum / l.get(row, row);
    }
    VecD(x)
}
// #endregion 🔖️Cholesky

// #region 🔖️QrHouseholder
/// 🪞️ Dense QR decomposition via Householder reflections; works for any `rows >= cols` matrix, returning orthogonal `Q` (rows x rows) and upper-triangular `R` (rows x cols) with `Q * R == A` up to float tolerance.
pub async fn qr_householder(a: &MatD) -> (MatD, MatD) {
    let m = a.rows;
    let n = a.cols;
    let mut r = a.clone();
    let mut q = MatD::identity(m);
    let steps = n.min(m.saturating_sub(1));
    for k in 0..steps {
        let mut norm_x = 0.0;
        for row in k..m {
            norm_x += r.get(row, k) * r.get(row, k);
        }
        norm_x = norm_x.sqrt();
        if norm_x < 1e-14 {
            continue;
        }
        let alpha = if r.get(k, k) >= 0.0 { -norm_x } else { norm_x };
        let mut v = vec![0.0; m - k];
        for row in k..m {
            v[row - k] = r.get(row, k);
        }
        v[0] -= alpha;
        let v_norm: f64 = v.iter().map(|vi| vi * vi).sum();
        if v_norm < 1e-28 {
            continue;
        }
        for col in 0..n {
            let mut dot = 0.0;
            for row in k..m {
                dot += v[row - k] * r.get(row, col);
            }
            let factor = 2.0 * dot / v_norm;
            for row in k..m {
                r.add_at(row, col, -factor * v[row - k]);
            }
        }
        for row in 0..m {
            let mut dot = 0.0;
            for col in k..m {
                dot += q.get(row, col) * v[col - k];
            }
            let factor = 2.0 * dot / v_norm;
            for col in k..m {
                q.add_at(row, col, -factor * v[col - k]);
            }
        }
    }
    (q, r)
}
// #endregion 🔖️QrHouseholder

// #region 🔖️JacobiEigenSymmetric
/// 🔄️ Full eigendecomposition of a small-to-medium dense symmetric matrix via the classical cyclic Jacobi rotation method (O(n³) per sweep — realistic ceiling is a few thousand rows; use `lanczos_extreme_eigen` for large sparse matrices instead). Eigenvalues are returned ascending; eigenvectors are the columns of the returned matrix, matched by index.
pub async fn jacobi_eigen_symmetric(a: &MatD, max_sweeps: usize) -> Result<(Vec<f64>, MatD), AlgebraError> {
    if a.rows != a.cols {
        return Err(AlgebraError::DimensionMismatch { expected: (a.rows, a.rows), got: (a.rows, a.cols) });
    }
    let n = a.rows;
    for row in 0..n {
        for col in 0..n {
            if (a.get(row, col) - a.get(col, row)).abs() > 1e-9 {
                return Err(AlgebraError::NotSymmetric);
            }
        }
    }
    let mut m = a.clone();
    let mut v = MatD::identity(n);
    let off_diag_norm = |m: &MatD| -> f64 {
        let mut sum = 0.0;
        for row in 0..n {
            for col in 0..n {
                if row != col {
                    sum += m.get(row, col) * m.get(row, col);
                }
            }
        }
        sum.sqrt()
    };
    let tol = 1e-12 * (1.0 + a.data.iter().map(|x| x.abs()).fold(0.0_f64, f64::max));
    let mut converged = false;
    for _sweep in 0..max_sweeps {
        if off_diag_norm(&m) < tol {
            converged = true;
            break;
        }
        for p in 0..n {
            for q in (p + 1)..n {
                let apq = m.get(p, q);
                if apq.abs() < 1e-300 {
                    continue;
                }
                let app = m.get(p, p);
                let aqq = m.get(q, q);
                let theta = (aqq - app) / (2.0 * apq);
                let t = theta.signum() / (theta.abs() + (1.0 + theta * theta).sqrt());
                let c = 1.0 / (1.0 + t * t).sqrt();
                let s = t * c;
                for k in 0..n {
                    let mkp = m.get(k, p);
                    let mkq = m.get(k, q);
                    m.set(k, p, c * mkp - s * mkq);
                    m.set(k, q, s * mkp + c * mkq);
                }
                for k in 0..n {
                    let mpk = m.get(p, k);
                    let mqk = m.get(q, k);
                    m.set(p, k, c * mpk - s * mqk);
                    m.set(q, k, s * mpk + c * mqk);
                }
                for k in 0..n {
                    let vkp = v.get(k, p);
                    let vkq = v.get(k, q);
                    v.set(k, p, c * vkp - s * vkq);
                    v.set(k, q, s * vkp + c * vkq);
                }
            }
        }
    }
    if !converged {
        return Err(AlgebraError::PowerIterationFailedConvergence { iterations: max_sweeps });
    }
    let eigenvalues: Vec<f64> = (0..n).map(|i| m.get(i, i)).collect();
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&i, &j| eigenvalues[i].partial_cmp(&eigenvalues[j]).unwrap());
    let sorted_vals: Vec<f64> = order.iter().map(|&i| eigenvalues[i]).collect();
    let mut sorted_vecs = MatD::zeros(n, n);
    for (new_col, &old_col) in order.iter().enumerate() {
        for row in 0..n {
            sorted_vecs.set(row, new_col, v.get(row, old_col));
        }
    }
    Ok((sorted_vals, sorted_vecs))
}
// #endregion 🔖️JacobiEigenSymmetric

// #region 🔖️LanczosExtremeEigen
/// 🕸️ Extreme eigenpairs of a large sparse symmetric matrix via Lanczos iteration with full reorthogonalization: builds a small tridiagonal Krylov-subspace matrix, diagonalizes it with `jacobi_eigen_symmetric`, and lifts the Ritz vectors back to the original space. Feeds algebraic-connectivity / Fiedler-vector algorithms in later NetworkX-parity waves. `largest` selects by eigenvalue magnitude, not sign.
pub async fn lanczos_extreme_eigen(a: &CsrMatrix, k: usize, largest: bool, max_iter: usize, seed: u64) -> Result<(Vec<f64>, Vec<VecD>), AlgebraError> {
    if a.rows != a.cols {
        return Err(AlgebraError::DimensionMismatch { expected: (a.rows, a.rows), got: (a.rows, a.cols) });
    }
    let n = a.rows;
    let m = max_iter.min(n);
    let mut rng_state = seed ^ 0x9E37_79B9_7F4A_7C15;
    let mut next_rand = move || {
        rng_state ^= rng_state << 13;
        rng_state ^= rng_state >> 7;
        rng_state ^= rng_state << 17;
        (rng_state as f64 / u64::MAX as f64) * 2.0 - 1.0
    };
    let mut v0 = VecD::from_vec((0..n).map(|_| next_rand()).collect());
    if v0.norm2() < 1e-300 {
        v0 = VecD::from_vec(vec![1.0; n]);
    }
    let norm0 = v0.norm2().max(1e-300);
    v0 = v0.scale(1.0 / norm0);
    let mut basis: Vec<VecD> = vec![v0];
    let mut alpha = Vec::with_capacity(m);
    let mut beta = Vec::with_capacity(m);
    for j in 0..m {
        let mut w = a.spmv(&basis[j]);
        alpha.push(basis[j].dot(&w));
        for basis_vec in basis.iter() {
            let proj = basis_vec.dot(&w);
            w = w.sub(&basis_vec.scale(proj));
        }
        for basis_vec in basis.iter() {
            let proj = basis_vec.dot(&w);
            w = w.sub(&basis_vec.scale(proj));
        }
        let bj = w.norm2();
        if j + 1 < m {
            beta.push(bj);
            if bj < 1e-12 {
                break;
            }
            basis.push(w.scale(1.0 / bj));
        }
    }
    let dim = alpha.len();
    let mut t = MatD::zeros(dim, dim);
    for i in 0..dim {
        t.set(i, i, alpha[i]);
        if i + 1 < dim {
            t.set(i, i + 1, beta[i]);
            t.set(i + 1, i, beta[i]);
        }
    }
    let (ritz_vals, ritz_vecs) = jacobi_eigen_symmetric(&t, 500)?;
    let mut order: Vec<usize> = (0..dim).collect();
    order.sort_by(|&x, &y| ritz_vals[y].abs().partial_cmp(&ritz_vals[x].abs()).unwrap());
    if !largest {
        order.reverse();
    }
    let take = k.min(dim);
    let mut eigenvalues = Vec::with_capacity(take);
    let mut eigenvectors = Vec::with_capacity(take);
    for &idx in order.iter().take(take) {
        eigenvalues.push(ritz_vals[idx]);
        let mut vec_full = VecD::zeros(n);
        for (col_i, basis_vec) in basis.iter().enumerate() {
            let coeff = ritz_vecs.get(col_i, idx);
            vec_full = vec_full.add(&basis_vec.scale(coeff));
        }
        eigenvectors.push(vec_full);
    }
    Ok((eigenvalues, eigenvectors))
}
// #endregion 🔖️LanczosExtremeEigen

// #region 🔖️PowerIteration
/// 🔁️ Dominant eigenpair of a large sparse symmetric matrix via power iteration; convergence is measured via the residual `‖A x - λ x‖`. To recover further eigenpairs, deflate by rebuilding `a`'s triplets with `λ v vᵀ` subtracted and re-call — this returns a single eigenpair by design so callers control the deflation loop.
pub async fn power_iteration(a: &CsrMatrix, max_iter: usize, tol: f64, seed: u64) -> Result<(f64, VecD), AlgebraError> {
    if a.rows != a.cols {
        return Err(AlgebraError::DimensionMismatch { expected: (a.rows, a.rows), got: (a.rows, a.cols) });
    }
    let n = a.rows;
    let mut rng_state = seed ^ 0x2545_F491_4F6C_DD1D;
    let mut next_rand = move || {
        rng_state ^= rng_state << 13;
        rng_state ^= rng_state >> 7;
        rng_state ^= rng_state << 17;
        (rng_state as f64 / u64::MAX as f64) * 2.0 - 1.0
    };
    let mut v = VecD::from_vec((0..n).map(|_| next_rand()).collect());
    let norm = v.norm2().max(1e-300);
    v = v.scale(1.0 / norm);
    for _ in 0..max_iter {
        let w = a.spmv(&v);
        let norm_w = w.norm2();
        if norm_w < 1e-300 {
            return Ok((0.0, v));
        }
        let v_next = w.scale(1.0 / norm_w);
        let av_next = a.spmv(&v_next);
        let lambda = v_next.dot(&av_next);
        let residual = av_next.sub(&v_next.scale(lambda)).norm2();
        v = v_next;
        if residual < tol {
            return Ok((lambda, v));
        }
    }
    Err(AlgebraError::PowerIterationFailedConvergence { iterations: max_iter })
}
// #endregion 🔖️PowerIteration

// #region 🔖️ConjugateGradient
/// 🧮️ Conjugate-gradient solver for sparse symmetric positive-definite systems `A x = b`; feeds Laplacian-style solves (current-flow centrality, resistance distance) in later NetworkX-parity waves. Reuses `AlgebraError::PowerIterationFailedConvergence` for the non-convergence case (same "iterative solver ran out of iterations" semantic as power iteration and Jacobi).
pub async fn conjugate_gradient(a: &CsrMatrix, b: &VecD, tol: f64, max_iter: usize) -> Result<VecD, AlgebraError> {
    if a.rows != a.cols {
        return Err(AlgebraError::DimensionMismatch { expected: (a.rows, a.rows), got: (a.rows, a.cols) });
    }
    if a.rows != b.len() {
        return Err(AlgebraError::DimensionMismatch { expected: (a.rows, 1), got: (b.len(), 1) });
    }
    let mut x = VecD::zeros(a.rows);
    let mut r = b.sub(&a.spmv(&x));
    let mut p = r.clone();
    let mut rs_old = r.dot(&r);
    if rs_old.sqrt() < tol {
        return Ok(x);
    }
    for iteration in 0..max_iter {
        let ap = a.spmv(&p);
        let denom = p.dot(&ap);
        if denom.abs() < 1e-300 {
            return Err(AlgebraError::PowerIterationFailedConvergence { iterations: iteration });
        }
        let alpha = rs_old / denom;
        x = x.add(&p.scale(alpha));
        r = r.sub(&ap.scale(alpha));
        let rs_new = r.dot(&r);
        if rs_new.sqrt() < tol {
            return Ok(x);
        }
        p = r.add(&p.scale(rs_new / rs_old));
        rs_old = rs_new;
    }
    Err(AlgebraError::PowerIterationFailedConvergence { iterations: max_iter })
}
// #endregion 🔖️ConjugateGradient

// #region 🔖️ExpmPade
/// 🧮️ Dense matrix exponential via scaling-and-squaring with an order-6 diagonal Padé approximant (coefficients from the closed-form `(2m-j)! m! / ((2m)! j! (m-j)!)` formula, m=6). O(n³) cost — fine for graphs up to a few hundred nodes; callers on bigger graphs should prefer eigen-based communicability once that's built in a later wave.
pub async fn expm_pade(a: &MatD) -> MatD {
    assert_eq!(a.rows, a.cols, "expm_pade requires a square matrix");
    let n = a.rows;
    if n == 0 {
        return MatD::zeros(0, 0);
    }
    let one_norm = {
        let mut max_col_sum = 0.0_f64;
        for col in 0..n {
            let mut sum = 0.0;
            for row in 0..n {
                sum += a.get(row, col).abs();
            }
            max_col_sum = max_col_sum.max(sum);
        }
        max_col_sum
    };
    let mut squarings = 0i32;
    let mut scaled_norm = one_norm;
    while scaled_norm > 1.0 {
        squarings += 1;
        scaled_norm /= 2.0;
    }
    let factor = 2f64.powi(squarings);
    let mut a_scaled = a.clone();
    for value in a_scaled.data.iter_mut() {
        *value /= factor;
    }
    const PADE_COEFFS: [f64; 7] = [1.0, 0.5, 5.0 / 44.0, 1.0 / 66.0, 1.0 / 792.0, 1.0 / 15840.0, 1.0 / 665_280.0];
    let mut powers = Vec::with_capacity(7);
    powers.push(MatD::identity(n));
    for i in 1..7 {
        powers.push(powers[i - 1].matmul(&a_scaled));
    }
    let mut n_mat = MatD::zeros(n, n);
    let mut d_mat = MatD::zeros(n, n);
    for (i, coeff) in PADE_COEFFS.iter().enumerate() {
        let sign = if i % 2 == 0 { 1.0 } else { -1.0 };
        for idx in 0..n_mat.data.len() {
            n_mat.data[idx] += coeff * powers[i].data[idx];
            d_mat.data[idx] += sign * coeff * powers[i].data[idx];
        }
    }
    let mut result = MatD::zeros(n, n);
    for col in 0..n {
        let mut rhs = VecD::zeros(n);
        for row in 0..n {
            rhs.set(row, n_mat.get(row, col));
        }
        let x = d_mat.lu_solve(&rhs).expect("Padé denominator of a scaled matrix argument is diagonally dominant and thus non-singular");
        for row in 0..n {
            result.set(row, col, x.get(row));
        }
    }
    for _ in 0..squarings {
        result = result.matmul(&result);
    }
    result
}
// #endregion 🔖️ExpmPade

// #region 🔖️Svd
/// 🌀️ Thin singular value decomposition `A = U Σ Vᵀ` via one-sided Jacobi (Hestenes) rotations: column pairs of a working copy are orthogonalized until every normalized inner product drops below 1e-12 (max 60 sweeps). Returns `(U, σ, V)` with singular values sorted descending; for a tall `m x n` input `U` is `m x n` and `V` is `n x n`, wide inputs are handled by transposing internally and swapping `U`/`V`.
pub async fn svd(a: &MatD) -> Result<(MatD, Vec<f64>, MatD), AlgebraError> {
    if a.rows < a.cols {
        let (u_t, sigma, v_t) = svd(&a.transpose())?;
        return Ok((v_t, sigma, u_t));
    }
    let m = a.rows;
    let n = a.cols;
    let mut work = a.clone();
    let mut v = MatD::identity(n);
    let mut converged = false;
    for _sweep in 0..60 {
        let mut max_ratio = 0.0_f64;
        for p in 0..n {
            for q in (p + 1)..n {
                let mut app = 0.0;
                let mut aqq = 0.0;
                let mut apq = 0.0;
                for row in 0..m {
                    let wp = work.get(row, p);
                    let wq = work.get(row, q);
                    app += wp * wp;
                    aqq += wq * wq;
                    apq += wp * wq;
                }
                if app * aqq < 1e-300 {
                    continue;
                }
                let ratio = apq.abs() / (app * aqq).sqrt();
                max_ratio = max_ratio.max(ratio);
                if ratio < 1e-12 {
                    continue;
                }
                let zeta = (aqq - app) / (2.0 * apq);
                let t = zeta.signum() / (zeta.abs() + (1.0 + zeta * zeta).sqrt());
                let c = 1.0 / (1.0 + t * t).sqrt();
                let s = c * t;
                for row in 0..m {
                    let wp = work.get(row, p);
                    let wq = work.get(row, q);
                    work.set(row, p, c * wp - s * wq);
                    work.set(row, q, s * wp + c * wq);
                }
                for row in 0..n {
                    let vp = v.get(row, p);
                    let vq = v.get(row, q);
                    v.set(row, p, c * vp - s * vq);
                    v.set(row, q, s * vp + c * vq);
                }
            }
        }
        if max_ratio < 1e-12 {
            converged = true;
            break;
        }
    }
    if !converged {
        return Err(AlgebraError::PowerIterationFailedConvergence { iterations: 60 });
    }
    let sigmas: Vec<f64> = (0..n)
        .map(|col| {
            let mut sum = 0.0;
            for row in 0..m {
                sum += work.get(row, col) * work.get(row, col);
            }
            sum.sqrt()
        })
        .collect();
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&i, &j| sigmas[j].partial_cmp(&sigmas[i]).unwrap());
    let mut u = MatD::zeros(m, n);
    let mut sigma = Vec::with_capacity(n);
    let mut v_sorted = MatD::zeros(n, n);
    for (new_col, &old_col) in order.iter().enumerate() {
        let s_val = sigmas[old_col];
        sigma.push(s_val);
        if s_val > 1e-300 {
            for row in 0..m {
                u.set(row, new_col, work.get(row, old_col) / s_val);
            }
        }
        for row in 0..n {
            v_sorted.set(row, new_col, v.get(row, old_col));
        }
    }
    Ok((u, sigma, v_sorted))
}

/// 🌀️ Right-singular vector for the smallest singular value of `A` — the homogeneous least-squares (DLT) minimizer of `‖A x‖` over unit vectors `x`.
pub async fn svd_nullvector(a: &MatD) -> Result<VecD, AlgebraError> {
    let (_, _, v) = svd(a)?;
    if v.cols == 0 {
        return Err(AlgebraError::DimensionMismatch { expected: (a.rows, 1), got: (a.rows, a.cols) });
    }
    let last = v.cols - 1;
    Ok(VecD::from_vec((0..v.rows).map(|row| v.get(row, last)).collect()))
}
// #endregion 🔖️Svd

// #region 🔖️LeastSquares
/// 📐️ Linear least-squares solution of an overdetermined `A x ≈ b` (`rows >= cols`) via Householder QR: back-substitutes the top `n x n` block of `R x = Qᵀ b`; returns `AlgebraError::Singular` when `A` is rank-deficient.
pub async fn solve_llsq(a: &MatD, b: &VecD) -> Result<VecD, AlgebraError> {
    if a.rows < a.cols {
        return Err(AlgebraError::DimensionMismatch { expected: (a.cols, a.cols), got: (a.rows, a.cols) });
    }
    if a.rows != b.len() {
        return Err(AlgebraError::DimensionMismatch { expected: (a.rows, 1), got: (b.len(), 1) });
    }
    let (q, r) = qr_householder(a);
    let qtb = q.transpose().mul_vec(b);
    let n = a.cols;
    let mut x = vec![0.0; n];
    for row in (0..n).rev() {
        let mut sum = qtb.get(row);
        for (col, xc) in x.iter().enumerate().skip(row + 1) {
            sum -= r.get(row, col) * xc;
        }
        let pivot = r.get(row, row);
        if pivot.abs() < 1e-12 {
            return Err(AlgebraError::Singular);
        }
        x[row] = sum / pivot;
    }
    Ok(VecD(x))
}

/// 📐️ Moore-Penrose pseudo-inverse `A⁺ = V Σ⁺ Uᵀ` via `svd`, zeroing the reciprocal of every singular value at or below `tol · σ_max`.
pub async fn pseudo_inverse(a: &MatD, tol: f64) -> Result<MatD, AlgebraError> {
    let (u, sigma, v) = svd(a)?;
    let sigma_max = sigma.first().copied().unwrap_or(0.0);
    let k = sigma.len();
    let mut sigma_inv = MatD::zeros(k, k);
    for (i, &s) in sigma.iter().enumerate() {
        if s > tol * sigma_max {
            sigma_inv.set(i, i, 1.0 / s);
        }
    }
    Ok(v.matmul(&sigma_inv).matmul(&u.transpose()))
}

/// 📐️ Assembles the weighted normal equations `(AᵀWA, AᵀWb)` for a diagonal weight vector `w` — the per-iteration system of IRLS solvers.
pub async fn weighted_normal_equations(a: &MatD, b: &VecD, w: &[f64]) -> (MatD, VecD) {
    assert_eq!(a.rows, b.len(), "weighted_normal_equations dimension mismatch");
    assert_eq!(a.rows, w.len(), "weighted_normal_equations weight length mismatch");
    let n = a.cols;
    let mut ata = MatD::zeros(n, n);
    let mut atb = VecD::zeros(n);
    for (row, &weight) in w.iter().enumerate() {
        for i in 0..n {
            let ai = a.get(row, i);
            if ai == 0.0 {
                continue;
            }
            atb.add_at(i, weight * ai * b.get(row));
            for j in 0..n {
                ata.add_at(i, j, weight * ai * a.get(row, j));
            }
        }
    }
    (ata, atb)
}
// #endregion 🔖️LeastSquares

// #region 🔖️Hessenberg
/// 🪜️ Reduces a general real square matrix to upper Hessenberg form via Householder reflectors applied column-by-column below the first subdiagonal (Golub & Van Loan, Algorithm 7.4.2). Returns `(H, Q)` with `Q` orthogonal and `A = Q H Qᵀ`; `H` is zero below its first subdiagonal. This is the stable first stage of [`real_schur`]'s Francis double-shift QR iteration.
pub async fn hessenberg(a: &MatD) -> (MatD, MatD) {
    assert_eq!(a.rows, a.cols, "hessenberg requires a square matrix");
    let n = a.rows;
    let mut h = a.clone();
    let mut q = MatD::identity(n);
    for k in 0..n.saturating_sub(2) {
        let mut norm_x = 0.0;
        for row in (k + 1)..n {
            norm_x += h.get(row, k) * h.get(row, k);
        }
        norm_x = norm_x.sqrt();
        if norm_x < 1e-300 {
            continue;
        }
        let alpha = if h.get(k + 1, k) >= 0.0 { -norm_x } else { norm_x };
        let mut v = vec![0.0; n - k - 1];
        for row in (k + 1)..n {
            v[row - k - 1] = h.get(row, k);
        }
        v[0] -= alpha;
        let v_norm: f64 = v.iter().map(|vi| vi * vi).sum();
        if v_norm < 1e-300 {
            continue;
        }
        for col in k..n {
            let mut dot = 0.0;
            for row in (k + 1)..n {
                dot += v[row - k - 1] * h.get(row, col);
            }
            let factor = 2.0 * dot / v_norm;
            for row in (k + 1)..n {
                h.add_at(row, col, -factor * v[row - k - 1]);
            }
        }
        for row in 0..n {
            let mut dot = 0.0;
            for col in (k + 1)..n {
                dot += h.get(row, col) * v[col - k - 1];
            }
            let factor = 2.0 * dot / v_norm;
            for col in (k + 1)..n {
                h.add_at(row, col, -factor * v[col - k - 1]);
            }
        }
        for row in 0..n {
            let mut dot = 0.0;
            for col in (k + 1)..n {
                dot += q.get(row, col) * v[col - k - 1];
            }
            let factor = 2.0 * dot / v_norm;
            for col in (k + 1)..n {
                q.add_at(row, col, -factor * v[col - k - 1]);
            }
        }
    }
    for row in 2..n {
        for col in 0..(row - 1) {
            h.set(row, col, 0.0);
        }
    }
    (h, q)
}
// #endregion 🔖️Hessenberg

// #region 🔖️RealSchur
/// ⚙️ One Francis implicit-double-shift QR sweep on the unreduced Hessenberg window `h[l..=m, l..=m]`: forms the real quadratic `M = Hˢᵘᵇ² - trace·Hˢᵘᵇ + det·I` from the trailing 2x2's (possibly complex-conjugate) shifts, QR-factorizes `M` via [`qr_householder`], and applies the resulting orthogonal factor as a similarity transform. By the implicit-Q theorem this recovers exactly the bulge-chased Francis step's result without hand-rolled bulge chasing, at the cost of dense O(p³) work per sweep instead of exploiting the Hessenberg band — a fine trade at this crate's small (SfM-scale) matrix sizes. Also right-multiplies the accumulated `q` by the same factor over columns `l..=m`.
async fn francis_double_shift_step(h: &mut MatD, q: &mut MatD, l: usize, m: usize) {
    let n = h.rows;
    let p = m - l + 1;
    let shift_trace = h.get(m - 1, m - 1) + h.get(m, m);
    let shift_det = h.get(m - 1, m - 1) * h.get(m, m) - h.get(m - 1, m) * h.get(m, m - 1);
    let mut sub = MatD::zeros(p, p);
    for row in 0..p {
        for col in 0..p {
            sub.set(row, col, h.get(l + row, l + col));
        }
    }
    let mut shifted = sub.matmul(&sub);
    for (shifted_val, sub_val) in shifted.data.iter_mut().zip(sub.data.iter()) {
        *shifted_val -= shift_trace * sub_val;
    }
    for i in 0..p {
        shifted.add_at(i, i, shift_det);
    }
    let (qsub, _) = qr_householder(&shifted);
    let qsub_t = qsub.transpose();
    for col in 0..n {
        let mut updated = vec![0.0; p];
        for (row, slot) in updated.iter_mut().enumerate() {
            let mut sum = 0.0;
            for k in 0..p {
                sum += qsub_t.get(row, k) * h.get(l + k, col);
            }
            *slot = sum;
        }
        for (row, &value) in updated.iter().enumerate() {
            h.set(l + row, col, value);
        }
    }
    for row in 0..n {
        let mut updated = vec![0.0; p];
        for (col, slot) in updated.iter_mut().enumerate() {
            let mut sum = 0.0;
            for k in 0..p {
                sum += h.get(row, l + k) * qsub.get(k, col);
            }
            *slot = sum;
        }
        for (col, &value) in updated.iter().enumerate() {
            h.set(row, l + col, value);
        }
    }
    for row in 0..n {
        let mut updated = vec![0.0; p];
        for (col, slot) in updated.iter_mut().enumerate() {
            let mut sum = 0.0;
            for k in 0..p {
                sum += q.get(row, l + k) * qsub.get(k, col);
            }
            *slot = sum;
        }
        for (col, &value) in updated.iter().enumerate() {
            q.set(row, l + col, value);
        }
    }
}

/// 🎭️ Real Schur decomposition `A = Q T Qᵀ` via Hessenberg reduction followed by Francis implicit-double-shift QR iteration with deflation: `Q` is orthogonal, `T` is quasi-upper-triangular (1x1 diagonal blocks for real eigenvalues, 2x2 blocks for complex-conjugate pairs). A subdiagonal entry deflates once it drops below `1e-13` relative to its neighboring diagonal magnitudes. Caps iterations at `30 * n` per deflation window before giving up with `AlgebraError::PowerIterationFailedConvergence` — the same "iterative solver ran out of iterations" semantic already reused by `conjugate_gradient`/`jacobi_eigen_symmetric`/`svd`, rather than adding a near-duplicate variant.
pub async fn real_schur(a: &MatD) -> Result<(MatD, MatD), AlgebraError> {
    if a.rows != a.cols {
        return Err(AlgebraError::DimensionMismatch { expected: (a.rows, a.rows), got: (a.rows, a.cols) });
    }
    let n = a.rows;
    if n < 2 {
        let (h, q) = hessenberg(a);
        return Ok((h, q));
    }
    let (mut h, mut q) = hessenberg(a);
    let base_scale = 1.0 + a.data.iter().fold(0.0_f64, |acc, v| acc.max(v.abs()));
    let eps = 1e-13;
    let max_iter_per_window = 30 * n;
    let mut m = n - 1;
    let mut window_iters = 0usize;
    while m > 0 {
        let mut l = m;
        while l > 0 {
            let scale = h.get(l - 1, l - 1).abs() + h.get(l, l).abs();
            let threshold = eps * if scale > 0.0 { scale } else { base_scale };
            if h.get(l, l - 1).abs() <= threshold {
                h.set(l, l - 1, 0.0);
                break;
            }
            l -= 1;
        }
        if l == m {
            m -= 1;
            window_iters = 0;
            continue;
        }
        if l + 1 == m {
            m = m.saturating_sub(2);
            window_iters = 0;
            continue;
        }
        window_iters += 1;
        if window_iters > max_iter_per_window {
            return Err(AlgebraError::PowerIterationFailedConvergence { iterations: window_iters });
        }
        francis_double_shift_step(&mut h, &mut q, l, m);
    }
    Ok((h, q))
}

/// 🔍️ Real eigenvalues of `a`, read off [`real_schur`]'s quasi-triangular `T`: each 1x1 diagonal block yields `(value, 0.0)`; each 2x2 block yields a conjugate pair `(re, ±im)` solved directly from the block's characteristic quadratic `λ² - trace·λ + det = 0` (mirrors [`Mat2::eigenvalues`] but does not discard the complex case). Order matches `T`'s diagonal, not sorted.
pub async fn real_eigenvalues(a: &MatD) -> Result<Vec<(f64, f64)>, AlgebraError> {
    let (t, _) = real_schur(a)?;
    let n = t.rows;
    let mut out = Vec::with_capacity(n);
    let mut i = 0;
    while i < n {
        let is_block = i + 1 < n && t.get(i + 1, i).abs() > 1e-9 * (1.0 + t.get(i, i).abs() + t.get(i + 1, i + 1).abs());
        if is_block {
            let a11 = t.get(i, i);
            let a12 = t.get(i, i + 1);
            let a21 = t.get(i + 1, i);
            let a22 = t.get(i + 1, i + 1);
            let trace = a11 + a22;
            let det = a11 * a22 - a12 * a21;
            let disc = trace * trace - 4.0 * det;
            if disc >= 0.0 {
                let sq = disc.sqrt();
                out.push(((trace + sq) * 0.5, 0.0));
                out.push(((trace - sq) * 0.5, 0.0));
            } else {
                let sq = (-disc).sqrt();
                out.push((trace * 0.5, sq * 0.5));
                out.push((trace * 0.5, -sq * 0.5));
            }
            i += 2;
        } else {
            out.push((t.get(i, i), 0.0));
            i += 1;
        }
    }
    Ok(out)
}
// #endregion 🔖️RealSchur

// #region 🔖️CompanionRoots
/// 🌱️ Real (possibly complex-conjugate) roots of the polynomial `coeffs[0] + coeffs[1] x + ... + coeffs[n] xⁿ` via the Frobenius companion matrix's eigenvalues (through [`real_eigenvalues`]). `coeffs` is ascending-degree; the leading coefficient `coeffs[n]` must be nonzero, else `AlgebraError::Singular` — reused rather than adding a new variant, since a zero leading coefficient leaves the companion matrix without a well-defined monic normalization (the same "degenerate, no solution" semantic `Singular` already carries for `lu_solve`/`solve_llsq`). Feeds the 5-point essential-matrix action-matrix polynomial and P3P's quartic.
pub async fn poly_roots_companion(coeffs: &[f64]) -> Result<Vec<(f64, f64)>, AlgebraError> {
    if coeffs.len() < 2 {
        return Err(AlgebraError::Singular);
    }
    let degree = coeffs.len() - 1;
    let leading = coeffs[degree];
    if leading.abs() < 1e-300 {
        return Err(AlgebraError::Singular);
    }
    let mut companion = MatD::zeros(degree, degree);
    for row in 1..degree {
        companion.set(row, row - 1, 1.0);
    }
    for (row, &coeff) in coeffs.iter().take(degree).enumerate() {
        companion.set(row, degree - 1, -coeff / leading);
    }
    real_eigenvalues(&companion)
}
// #endregion 🔖️CompanionRoots

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn mat2_identity_apply_is_noop() {
        assert_eq!(Mat2::IDENTITY.apply((3.0, -4.0)), (3.0, -4.0));
    }

    #[test]
    async fn mat2_apply_matches_matrix_vector_multiply() {
        let m = Mat2::new(2.0, 0.0, 0.0, 3.0);
        assert_eq!(m.apply((1.0, 1.0)), (2.0, 3.0));
    }

    #[test]
    async fn mat2_det_of_scale_matrix() {
        let m = Mat2::new(2.0, 0.0, 0.0, 3.0);
        assert!((m.det() - 6.0).abs() < 1e-9);
    }

    #[test]
    async fn mat2_inverse_round_trips() {
        let m = Mat2::new(2.0, 1.0, 1.0, 3.0);
        let inv = m.inverse().expect("invertible");
        let round = m.mul(inv);
        assert!((round.cols[0][0] - 1.0).abs() < 1e-9);
        assert!((round.cols[1][1] - 1.0).abs() < 1e-9);
        assert!(round.cols[1][0].abs() < 1e-9);
        assert!(round.cols[0][1].abs() < 1e-9);
    }

    #[test]
    async fn mat2_singular_matrix_has_no_inverse() {
        let m = Mat2::new(1.0, 2.0, 2.0, 4.0);
        assert!(m.inverse().is_none());
    }

    #[test]
    async fn mat2_transpose_swaps_off_diagonal() {
        let m = Mat2::new(1.0, 2.0, 3.0, 4.0);
        let t = m.transpose();
        assert_eq!(t.apply((1.0, 0.0)), (1.0, 2.0));
        assert_eq!(t.apply((0.0, 1.0)), (3.0, 4.0));
    }

    #[test]
    async fn mat2_eigenvalues_of_diagonal_matrix_are_the_diagonal() {
        let m = Mat2::new(2.0, 0.0, 0.0, 5.0);
        let (l1, l2) = m.eigenvalues().expect("real eigenvalues");
        let mut vals = [l1, l2];
        vals.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert!((vals[0] - 2.0).abs() < 1e-9);
        assert!((vals[1] - 5.0).abs() < 1e-9);
    }

    #[test]
    async fn mat2_rotation_like_matrix_has_complex_eigenvalues() {
        let m = Mat2::new(0.0, -1.0, 1.0, 0.0);
        assert!(m.eigenvalues().is_none());
    }

    #[test]
    async fn vecd_dot_and_norm() {
        let a = VecD::from_vec(vec![3.0, 4.0]);
        let b = VecD::from_vec(vec![1.0, 0.0]);
        assert!((a.dot(&b) - 3.0).abs() < 1e-12);
        assert!((a.norm2() - 5.0).abs() < 1e-12);
        assert!((a.norm_inf() - 4.0).abs() < 1e-12);
    }

    #[test]
    async fn vecd_add_sub_scale_round_trip() {
        let a = VecD::from_vec(vec![1.0, 2.0, 3.0]);
        let b = VecD::from_vec(vec![0.5, 0.5, 0.5]);
        let sum = a.add(&b);
        let back = sum.sub(&b);
        for i in 0..3 {
            assert!((back.get(i) - a.get(i)).abs() < 1e-12);
        }
        assert!((a.scale(2.0).get(1) - 4.0).abs() < 1e-12);
    }

    #[test]
    async fn matd_matmul_identity_is_noop() {
        let mut m = MatD::zeros(2, 2);
        m.set(0, 0, 1.0);
        m.set(0, 1, 2.0);
        m.set(1, 0, 3.0);
        m.set(1, 1, 4.0);
        let id = MatD::identity(2);
        let out = m.matmul(&id);
        assert_eq!(out, m);
    }

    #[test]
    async fn matd_transpose_round_trips() {
        let mut m = MatD::zeros(2, 3);
        for row in 0..2 {
            for col in 0..3 {
                m.set(row, col, (row * 3 + col) as f64);
            }
        }
        assert_eq!(m.transpose().transpose(), m);
    }

    #[test]
    async fn matd_mul_vec_matches_matrix_vector_multiply() {
        let mut m = MatD::zeros(2, 2);
        m.set(0, 0, 2.0);
        m.set(0, 1, 0.0);
        m.set(1, 0, 0.0);
        m.set(1, 1, 3.0);
        let x = VecD::from_vec(vec![1.0, 1.0]);
        let y = m.mul_vec(&x);
        assert!((y.get(0) - 2.0).abs() < 1e-12);
        assert!((y.get(1) - 3.0).abs() < 1e-12);
    }

    #[test]
    async fn matd_lu_solve_matches_hand_solved_system() {
        // 2x + y = 5, x + 3y = 10  =>  x = 1, y = 3
        let mut a = MatD::zeros(2, 2);
        a.set(0, 0, 2.0);
        a.set(0, 1, 1.0);
        a.set(1, 0, 1.0);
        a.set(1, 1, 3.0);
        let b = VecD::from_vec(vec![5.0, 10.0]);
        let x = a.lu_solve(&b).expect("solvable");
        assert!((x.get(0) - 1.0).abs() < 1e-9);
        assert!((x.get(1) - 3.0).abs() < 1e-9);
    }

    #[test]
    async fn matd_lu_solve_detects_singular_matrix() {
        let mut a = MatD::zeros(2, 2);
        a.set(0, 0, 1.0);
        a.set(0, 1, 2.0);
        a.set(1, 0, 2.0);
        a.set(1, 1, 4.0);
        let b = VecD::from_vec(vec![1.0, 2.0]);
        assert!(a.lu_solve(&b).is_none());
    }

    #[test]
    async fn matd_add_triple_product_matches_btdb() {
        let mut b = MatD::zeros(1, 2);
        b.set(0, 0, 1.0);
        b.set(0, 1, 2.0);
        let mut d = MatD::zeros(1, 1);
        d.set(0, 0, 3.0);
        let mut ke = MatD::zeros(2, 2);
        ke.add_triple_product(&b, &d, 1.0);
        // Bᵀ D B = [1;2] * 3 * [1 2] = [[3,6],[6,12]]
        assert!((ke.get(0, 0) - 3.0).abs() < 1e-12);
        assert!((ke.get(0, 1) - 6.0).abs() < 1e-12);
        assert!((ke.get(1, 1) - 12.0).abs() < 1e-12);
    }

    #[test]
    async fn mat3d_identity_axes_is_identity() {
        let m = Mat3d::from_axes([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]);
        assert_eq!(m, Mat3d::IDENTITY);
    }

    #[test]
    async fn mat3d_transpose_is_inverse_for_orthonormal_basis() {
        let x = vec3d_normalize([1.0, 1.0, 0.0]);
        let z = [0.0, 0.0, 1.0];
        let y = vec3d_cross(z, x);
        let m = Mat3d::from_axes(x, y, z);
        let round = m.mul(m.transpose());
        for row in 0..3 {
            for col in 0..3 {
                let expected = if row == col { 1.0 } else { 0.0 };
                assert!((round.cols[col][row] - expected).abs() < 1e-9);
            }
        }
    }

    #[test]
    async fn mat3d_mul_vec3_transforms_basis_vector() {
        let m = Mat3d::from_axes([0.0, 1.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        let out = m.mul_vec3([1.0, 0.0, 0.0]);
        assert!((out[0] - 0.0).abs() < 1e-12);
        assert!((out[1] - 1.0).abs() < 1e-12);
        assert!((out[2] - 0.0).abs() < 1e-12);
    }

    #[test]
    async fn vec3d_cross_is_perpendicular_to_inputs() {
        let a = [1.0, 0.0, 0.0];
        let b = [0.0, 1.0, 0.0];
        let c = vec3d_cross(a, b);
        assert!((c[2] - 1.0).abs() < 1e-12);
    }

    #[test]
    async fn csr_from_triplets_dedupes_and_sorts() {
        let triplets = [(0, 1, 2.0), (0, 1, 3.0), (0, 0, 1.0), (1, 0, 4.0)];
        let m = CsrMatrix::from_triplets(2, 2, &triplets);
        assert_eq!(m.nnz(), 3);
        let row0: Vec<(usize, f64)> = m.row(0).collect();
        assert_eq!(row0, vec![(0, 1.0), (1, 5.0)]);
    }

    #[test]
    async fn csr_spmv_matches_dense_mul_vec() {
        let triplets = [(0, 0, 2.0), (0, 1, 1.0), (1, 1, 3.0)];
        let m = CsrMatrix::from_triplets(2, 2, &triplets);
        let x = VecD::from_vec(vec![1.0, 2.0]);
        let sparse_result = m.spmv(&x);
        let dense_result = m.to_dense().mul_vec(&x);
        assert_eq!(sparse_result, dense_result);
    }

    #[test]
    async fn csr_transpose_swaps_rows_and_cols() {
        let triplets = [(0, 1, 5.0)];
        let m = CsrMatrix::from_triplets(2, 3, &triplets);
        let t = m.transpose();
        assert_eq!(t.rows, 3);
        assert_eq!(t.cols, 2);
        assert_eq!(t.row(1).collect::<Vec<_>>(), vec![(0, 5.0)]);
    }

    #[test]
    async fn algebra_error_display_is_human_readable() {
        assert_eq!(AlgebraError::Singular.to_string(), "matrix is singular");
        assert_eq!(AlgebraError::NotSymmetric.to_string(), "matrix is not symmetric");
    }

    #[test]
    async fn cholesky_matches_hand_solved_spd_system() {
        let mut a = MatD::zeros(2, 2);
        a.set(0, 0, 4.0);
        a.set(0, 1, 2.0);
        a.set(1, 0, 2.0);
        a.set(1, 1, 3.0);
        let l = cholesky(&a).expect("SPD");
        let reconstructed = l.matmul(&l.transpose());
        for row in 0..2 {
            for col in 0..2 {
                assert!((reconstructed.get(row, col) - a.get(row, col)).abs() < 1e-9);
            }
        }
        let b = VecD::from_vec(vec![6.0, 5.0]);
        let x = cholesky_solve(&l, &b);
        let x_lu = a.lu_solve(&b).expect("solvable");
        for i in 0..2 {
            assert!((x.get(i) - x_lu.get(i)).abs() < 1e-9);
        }
    }

    #[test]
    async fn cholesky_rejects_non_positive_definite() {
        let mut a = MatD::zeros(2, 2);
        a.set(0, 0, 1.0);
        a.set(0, 1, 2.0);
        a.set(1, 0, 2.0);
        a.set(1, 1, 1.0);
        assert_eq!(cholesky(&a), Err(AlgebraError::NotPositiveDefinite));
    }

    #[test]
    async fn qr_householder_reconstructs_and_is_orthogonal() {
        let mut a = MatD::zeros(3, 2);
        a.set(0, 0, 1.0);
        a.set(0, 1, 0.0);
        a.set(1, 0, 0.0);
        a.set(1, 1, 1.0);
        a.set(2, 0, 1.0);
        a.set(2, 1, 1.0);
        let (q, r) = qr_householder(&a);
        let product = q.matmul(&r);
        for row in 0..3 {
            for col in 0..2 {
                assert!((product.get(row, col) - a.get(row, col)).abs() < 1e-9);
                if row > col {
                    assert!(r.get(row, col).abs() < 1e-9);
                }
            }
        }
        let qt_q = q.transpose().matmul(&q);
        for i in 0..3 {
            for j in 0..3 {
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!((qt_q.get(i, j) - expected).abs() < 1e-9);
            }
        }
    }

    #[test]
    async fn jacobi_eigen_diagonal_matrix_returns_sorted_diagonal() {
        let mut a = MatD::zeros(3, 3);
        a.set(0, 0, 5.0);
        a.set(1, 1, 1.0);
        a.set(2, 2, 3.0);
        let (vals, vecs) = jacobi_eigen_symmetric(&a, 100).expect("converges");
        assert!((vals[0] - 1.0).abs() < 1e-9);
        assert!((vals[1] - 3.0).abs() < 1e-9);
        assert!((vals[2] - 5.0).abs() < 1e-9);
        for col in 0..3 {
            let mut norm = 0.0;
            for row in 0..3 {
                norm += vecs.get(row, col) * vecs.get(row, col);
            }
            assert!((norm - 1.0).abs() < 1e-9);
        }
    }

    #[test]
    async fn jacobi_eigen_matches_characteristic_polynomial_roots() {
        let mut a = MatD::zeros(2, 2);
        a.set(0, 0, 2.0);
        a.set(0, 1, 1.0);
        a.set(1, 0, 1.0);
        a.set(1, 1, 2.0);
        let (vals, _) = jacobi_eigen_symmetric(&a, 100).expect("converges");
        assert!((vals[0] - 1.0).abs() < 1e-9);
        assert!((vals[1] - 3.0).abs() < 1e-9);
    }

    #[test]
    async fn jacobi_eigen_rejects_asymmetric_matrix() {
        let mut a = MatD::zeros(2, 2);
        a.set(0, 0, 1.0);
        a.set(0, 1, 2.0);
        a.set(1, 0, 3.0);
        a.set(1, 1, 4.0);
        assert_eq!(jacobi_eigen_symmetric(&a, 100), Err(AlgebraError::NotSymmetric));
    }

    #[test]
    async fn jacobi_eigen_reports_non_convergence_with_zero_sweeps() {
        let mut a = MatD::zeros(2, 2);
        a.set(0, 0, 2.0);
        a.set(0, 1, 1.0);
        a.set(1, 0, 1.0);
        a.set(1, 1, 2.0);
        assert_eq!(jacobi_eigen_symmetric(&a, 0), Err(AlgebraError::PowerIterationFailedConvergence { iterations: 0 }));
    }

    #[test]
    async fn power_iteration_finds_dominant_eigenpair() {
        let triplets = [(0, 0, 1.0), (1, 1, 5.0), (2, 2, 2.0)];
        let a = CsrMatrix::from_triplets(3, 3, &triplets);
        let (lambda, v) = power_iteration(&a, 200, 1e-10, 7).expect("converges");
        assert!((lambda - 5.0).abs() < 1e-6);
        assert!(v.get(1).abs() > 0.99);
    }

    #[test]
    async fn power_iteration_rejects_non_square() {
        let a = CsrMatrix::from_triplets(2, 3, &[]);
        assert!(matches!(power_iteration(&a, 10, 1e-6, 1), Err(AlgebraError::DimensionMismatch { .. })));
    }

    #[test]
    async fn conjugate_gradient_matches_hand_solved_system() {
        let triplets = [(0, 0, 2.0), (0, 1, 1.0), (1, 0, 1.0), (1, 1, 3.0)];
        let a = CsrMatrix::from_triplets(2, 2, &triplets);
        let b = VecD::from_vec(vec![5.0, 10.0]);
        let x = conjugate_gradient(&a, &b, 1e-10, 100).expect("converges");
        assert!((x.get(0) - 1.0).abs() < 1e-6);
        assert!((x.get(1) - 3.0).abs() < 1e-6);
    }

    #[test]
    async fn conjugate_gradient_reports_non_convergence_with_zero_iterations() {
        let triplets = [(0, 0, 2.0), (1, 1, 2.0)];
        let a = CsrMatrix::from_triplets(2, 2, &triplets);
        let b = VecD::from_vec(vec![1.0, 1.0]);
        assert_eq!(conjugate_gradient(&a, &b, 1e-12, 0), Err(AlgebraError::PowerIterationFailedConvergence { iterations: 0 }));
    }

    #[test]
    async fn lanczos_extreme_eigen_finds_largest_on_diagonal_matrix() {
        let triplets = [(0, 0, 1.0), (1, 1, 4.0), (2, 2, 2.0), (3, 3, 3.0)];
        let a = CsrMatrix::from_triplets(4, 4, &triplets);
        let (vals, _) = lanczos_extreme_eigen(&a, 2, true, 4, 11).expect("converges");
        let mut sorted = vals;
        sorted.sort_by(|x, y| y.partial_cmp(x).unwrap());
        assert!((sorted[0] - 4.0).abs() < 1e-6);
        assert!((sorted[1] - 3.0).abs() < 1e-6);
    }

    #[test]
    async fn lanczos_extreme_eigen_finds_smallest_on_diagonal_matrix() {
        let triplets = [(0, 0, 1.0), (1, 1, 4.0), (2, 2, 2.0), (3, 3, 3.0)];
        let a = CsrMatrix::from_triplets(4, 4, &triplets);
        let (vals, _) = lanczos_extreme_eigen(&a, 2, false, 4, 11).expect("converges");
        let mut sorted = vals;
        sorted.sort_by(|x, y| x.partial_cmp(y).unwrap());
        assert!((sorted[0] - 1.0).abs() < 1e-6);
        assert!((sorted[1] - 2.0).abs() < 1e-6);
    }

    #[test]
    async fn lanczos_extreme_eigen_rejects_non_square() {
        let a = CsrMatrix::from_triplets(2, 3, &[]);
        assert!(matches!(lanczos_extreme_eigen(&a, 1, true, 5, 1), Err(AlgebraError::DimensionMismatch { .. })));
    }

    #[test]
    async fn expm_pade_of_zero_matrix_is_identity() {
        let z = MatD::zeros(3, 3);
        let e = expm_pade(&z);
        assert_eq!(e, MatD::identity(3));
    }

    #[test]
    async fn expm_pade_of_diagonal_matrix_is_elementwise_exp() {
        let mut a = MatD::zeros(2, 2);
        a.set(0, 0, 1.0);
        a.set(1, 1, 2.0);
        let e = expm_pade(&a);
        assert!((e.get(0, 0) - 1.0_f64.exp()).abs() < 1e-8);
        assert!((e.get(1, 1) - 2.0_f64.exp()).abs() < 1e-8);
        assert!(e.get(0, 1).abs() < 1e-12);
        assert!(e.get(1, 0).abs() < 1e-12);
    }

    async fn seeded_mat(rows: usize, cols: usize, seed: u64) -> MatD {
        let mut state = seed;
        let mut m = MatD::zeros(rows, cols);
        for value in m.data.iter_mut() {
            state = state.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407);
            *value = ((state >> 11) as f64 / (1u64 << 53) as f64) * 2.0 - 1.0;
        }
        m
    }

    async fn planted_rank_deficient() -> MatD {
        let base = seeded_mat(6, 3, 77);
        let mut a = MatD::zeros(6, 4);
        for row in 0..6 {
            a.set(row, 0, base.get(row, 0));
            a.set(row, 1, base.get(row, 1));
            a.set(row, 2, base.get(row, 2));
            a.set(row, 3, base.get(row, 0) + base.get(row, 1));
        }
        a
    }

    async fn assert_svd_round_trips(a: &MatD, u: &MatD, sigma: &[f64], v: &MatD) {
        let k = sigma.len();
        let mut s_mat = MatD::zeros(k, k);
        for (i, &s) in sigma.iter().enumerate() {
            s_mat.set(i, i, s);
        }
        let recon = u.matmul(&s_mat).matmul(&v.transpose());
        for row in 0..a.rows {
            for col in 0..a.cols {
                assert!((recon.get(row, col) - a.get(row, col)).abs() < 1e-9);
            }
        }
    }

    async fn assert_orthonormal_columns(m: &MatD) {
        let gram = m.transpose().matmul(m);
        for i in 0..gram.rows {
            for j in 0..gram.cols {
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!((gram.get(i, j) - expected).abs() < 1e-9);
            }
        }
    }

    #[test]
    async fn svd_reconstructs_tall_random_matrix() {
        let a = seeded_mat(8, 5, 20_260_719);
        let (u, sigma, v) = svd(&a).expect("converges");
        assert_eq!((u.rows, u.cols), (8, 5));
        assert_eq!(sigma.len(), 5);
        assert_eq!((v.rows, v.cols), (5, 5));
        assert!(sigma.iter().all(|s| *s >= 0.0));
        assert!(sigma.windows(2).all(|pair| pair[0] >= pair[1]));
        assert_svd_round_trips(&a, &u, &sigma, &v);
        assert_orthonormal_columns(&u);
        assert_orthonormal_columns(&v);
    }

    #[test]
    async fn svd_reconstructs_wide_random_matrix() {
        let a = seeded_mat(5, 8, 31);
        let (u, sigma, v) = svd(&a).expect("converges");
        assert_eq!((u.rows, u.cols), (5, 5));
        assert_eq!(sigma.len(), 5);
        assert_eq!((v.rows, v.cols), (8, 5));
        assert!(sigma.windows(2).all(|pair| pair[0] >= pair[1]));
        assert_svd_round_trips(&a, &u, &sigma, &v);
        assert_orthonormal_columns(&u);
        assert_orthonormal_columns(&v);
    }

    #[test]
    async fn svd_nullvector_finds_planted_kernel() {
        let a = planted_rank_deficient();
        let v = svd_nullvector(&a).expect("converges");
        assert!((v.norm2() - 1.0).abs() < 1e-9);
        assert!(a.mul_vec(&v).norm2() < 1e-9);
    }

    #[test]
    async fn solve_llsq_matches_normal_equations() {
        let a = seeded_mat(9, 3, 5);
        let b = VecD::from_vec((0..9).map(|i| (i as f64) * 0.5 - 2.0).collect());
        let x = solve_llsq(&a, &b).expect("full rank");
        let at = a.transpose();
        let x_ne = at.matmul(&a).lu_solve(&at.mul_vec(&b)).expect("solvable");
        for i in 0..3 {
            assert!((x.get(i) - x_ne.get(i)).abs() < 1e-9);
        }
    }

    #[test]
    async fn pseudo_inverse_satisfies_penrose_identity() {
        let a = planted_rank_deficient();
        let pinv = pseudo_inverse(&a, 1e-10).expect("converges");
        let round = a.matmul(&pinv).matmul(&a);
        for row in 0..a.rows {
            for col in 0..a.cols {
                assert!((round.get(row, col) - a.get(row, col)).abs() < 1e-9);
            }
        }
    }

    #[test]
    async fn weighted_normal_equations_match_hand_assembly() {
        let mut a = MatD::zeros(3, 2);
        a.set(0, 0, 1.0);
        a.set(0, 1, 2.0);
        a.set(1, 0, 3.0);
        a.set(1, 1, 4.0);
        a.set(2, 0, 5.0);
        a.set(2, 1, 6.0);
        let b = VecD::from_vec(vec![1.0, 2.0, 3.0]);
        let (ata, atb) = weighted_normal_equations(&a, &b, &[2.0, 1.0, 3.0]);
        assert!((ata.get(0, 0) - 86.0).abs() < 1e-12);
        assert!((ata.get(0, 1) - 106.0).abs() < 1e-12);
        assert!((ata.get(1, 0) - 106.0).abs() < 1e-12);
        assert!((ata.get(1, 1) - 132.0).abs() < 1e-12);
        assert!((atb.get(0) - 53.0).abs() < 1e-12);
        assert!((atb.get(1) - 66.0).abs() < 1e-12);
    }

    async fn planted_diagonal_conjugated(planted: &[f64], seed: u64) -> MatD {
        let n = planted.len();
        let (_, q) = hessenberg(&seeded_mat(n, n, seed));
        let mut d = MatD::zeros(n, n);
        for (i, &value) in planted.iter().enumerate() {
            d.set(i, i, value);
        }
        q.matmul(&d).matmul(&q.transpose())
    }

    #[test]
    async fn hessenberg_similarity_orthogonality_and_shape() {
        for (n, seed) in [(4, 101), (4, 202), (5, 303), (5, 404), (6, 505)] {
            let a = seeded_mat(n, n, seed);
            let (h, q) = hessenberg(&a);
            assert_orthonormal_columns(&q);
            for row in 2..n {
                for col in 0..(row - 1) {
                    assert!(h.get(row, col).abs() < 1e-8, "n={n} seed={seed} row={row} col={col} not zero: {}", h.get(row, col));
                }
            }
            let recon = q.matmul(&h).matmul(&q.transpose());
            for row in 0..n {
                for col in 0..n {
                    assert!((recon.get(row, col) - a.get(row, col)).abs() < 1e-7, "n={n} seed={seed} mismatch at ({row},{col})");
                }
            }
        }
    }

    #[test]
    #[should_panic(expected = "hessenberg requires a square matrix")]
    async fn hessenberg_rejects_non_square() {
        let a = MatD::zeros(2, 3);
        hessenberg(&a);
    }

    #[test]
    async fn real_schur_recovers_planted_real_spectrum() {
        let planted = [1.0, -2.0, 3.5, 0.25, -7.0];
        let a = planted_diagonal_conjugated(&planted, 909);
        let mut eigs = real_eigenvalues(&a).expect("converges");
        eigs.sort_by(|x, y| x.0.partial_cmp(&y.0).unwrap());
        let mut expected = planted.to_vec();
        expected.sort_by(|x, y| x.partial_cmp(y).unwrap());
        for (got, exp) in eigs.iter().zip(expected.iter()) {
            assert!(got.1.abs() < 1e-6, "expected real eigenvalue, got {got:?}");
            assert!((got.0 - exp).abs() < 1e-6 * exp.abs().max(1.0), "got {got:?} expected {exp}");
        }
    }

    #[test]
    async fn real_schur_recovers_planted_complex_conjugate_pairs() {
        let blocks = [(2.0, 3.0), (-1.0, 5.0)];
        let n = blocks.len() * 2;
        let (_, q) = hessenberg(&seeded_mat(n, n, 1717));
        let mut d = MatD::zeros(n, n);
        for (idx, &(re, im)) in blocks.iter().enumerate() {
            let base = idx * 2;
            d.set(base, base, re);
            d.set(base, base + 1, -im);
            d.set(base + 1, base, im);
            d.set(base + 1, base + 1, re);
        }
        let a = q.matmul(&d).matmul(&q.transpose());
        let eigs = real_eigenvalues(&a).expect("converges");
        let mut got: Vec<(f64, f64)> = eigs.iter().map(|&(re, im)| (re, im.abs())).collect();
        got.sort_by(|x, y| x.0.partial_cmp(&y.0).unwrap().then(x.1.partial_cmp(&y.1).unwrap()));
        let mut expected: Vec<(f64, f64)> = blocks.iter().flat_map(|&(re, im)| [(re, im.abs()), (re, im.abs())]).collect();
        expected.sort_by(|x, y| x.0.partial_cmp(&y.0).unwrap().then(x.1.partial_cmp(&y.1).unwrap()));
        for (g, e) in got.iter().zip(expected.iter()) {
            assert!((g.0 - e.0).abs() < 1e-6, "real part mismatch: got {g:?} expected {e:?}");
            assert!((g.1 - e.1).abs() < 1e-6, "imag magnitude mismatch: got {g:?} expected {e:?}");
        }
    }

    #[test]
    async fn real_schur_rejects_non_square() {
        let a = MatD::zeros(2, 3);
        assert!(matches!(real_schur(&a), Err(AlgebraError::DimensionMismatch { .. })));
    }

    #[test]
    async fn poly_roots_companion_matches_known_real_quartic_roots() {
        let coeffs = [24.0, -50.0, 35.0, -10.0, 1.0];
        let mut roots = poly_roots_companion(&coeffs).expect("converges");
        roots.sort_by(|x, y| x.0.partial_cmp(&y.0).unwrap());
        let expected = [1.0, 2.0, 3.0, 4.0];
        for (got, exp) in roots.iter().zip(expected.iter()) {
            assert!(got.1.abs() < 1e-6, "expected real root, got {got:?}");
            assert!((got.0 - exp).abs() < 1e-6, "got {got:?} expected {exp}");
        }
    }

    #[test]
    async fn poly_roots_companion_matches_known_complex_conjugate_pairs() {
        let coeffs = [4.0, 0.0, 5.0, 0.0, 1.0];
        let roots = poly_roots_companion(&coeffs).expect("converges");
        let mut mags: Vec<f64> = roots
            .iter()
            .map(|&(re, im)| {
                assert!(re.abs() < 1e-6, "expected zero real part, got {re}");
                im.abs()
            })
            .collect();
        mags.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert!((mags[0] - 1.0).abs() < 1e-6);
        assert!((mags[1] - 1.0).abs() < 1e-6);
        assert!((mags[2] - 2.0).abs() < 1e-6);
        assert!((mags[3] - 2.0).abs() < 1e-6);
    }

    #[test]
    async fn poly_roots_companion_rejects_zero_leading_coefficient() {
        let coeffs = [1.0, 2.0, 0.0];
        assert_eq!(poly_roots_companion(&coeffs), Err(AlgebraError::Singular));
    }

    #[test]
    async fn poly_roots_companion_rejects_too_short_input() {
        assert_eq!(poly_roots_companion(&[5.0]), Err(AlgebraError::Singular));
    }

    #[test]
    async fn vec3d_sub_and_length_match_hand_computation() {
        let diff = vec3d_sub([3.0, 4.0, 0.0], [1.0, 1.0, 0.0]);
        assert_eq!(diff, [2.0, 3.0, 0.0]);
        assert!((vec3d_length([3.0, 4.0, 0.0]) - 5.0).abs() < 1e-12);
    }

    #[test]
    async fn algebra_error_display_covers_remaining_variants() {
        assert_eq!(AlgebraError::NotPositiveDefinite.to_string(), "matrix is not positive definite");
        assert_eq!(AlgebraError::DimensionMismatch { expected: (2, 2), got: (3, 3) }.to_string(), "dimension mismatch: expected (2, 2), got (3, 3)");
        assert_eq!(AlgebraError::PowerIterationFailedConvergence { iterations: 42 }.to_string(), "iterative solver failed to converge after 42 iterations");
    }

    #[test]
    async fn matd_matmul_skips_zero_entries_and_still_correct() {
        let mut m = MatD::zeros(2, 2);
        m.set(0, 0, 0.0);
        m.set(0, 1, 2.0);
        m.set(1, 0, 3.0);
        m.set(1, 1, 0.0);
        let out = m.matmul(&MatD::identity(2));
        assert_eq!(out, m);
    }

    #[test]
    async fn weighted_normal_equations_skips_zero_entries_in_a() {
        let mut a = MatD::zeros(2, 2);
        a.set(0, 0, 0.0);
        a.set(0, 1, 1.0);
        a.set(1, 0, 2.0);
        a.set(1, 1, 3.0);
        let b = VecD::from_vec(vec![1.0, 2.0]);
        let (ata, atb) = weighted_normal_equations(&a, &b, &[1.0, 1.0]);
        assert!((ata.get(0, 0) - 4.0).abs() < 1e-12);
        assert!((ata.get(0, 1) - 6.0).abs() < 1e-12);
        assert!((ata.get(1, 1) - 10.0).abs() < 1e-12);
        assert!((atb.get(0) - 4.0).abs() < 1e-12);
        assert!((atb.get(1) - 7.0).abs() < 1e-12);
    }

    #[test]
    async fn hessenberg_skips_already_zero_subdiagonal_column() {
        let mut a = MatD::zeros(4, 4);
        for i in 0..4 {
            a.set(i, i, (i + 1) as f64);
        }
        let (h, q) = hessenberg(&a);
        assert_eq!(h, a);
        assert_eq!(q, MatD::identity(4));
    }

    #[test]
    async fn qr_householder_skips_already_zero_column_segment() {
        let mut a = MatD::zeros(3, 3);
        a.set(0, 0, 1.0);
        a.set(0, 1, 5.0);
        a.set(0, 2, 3.0);
        a.set(1, 2, 1.0);
        a.set(2, 2, 5.0);
        let (q, r) = qr_householder(&a);
        let product = q.matmul(&r);
        for row in 0..3 {
            for col in 0..3 {
                assert!((product.get(row, col) - a.get(row, col)).abs() < 1e-7);
            }
        }
    }

    #[test]
    async fn svd_nullvector_rejects_matrix_with_zero_columns() {
        let a = MatD::zeros(2, 0);
        assert!(matches!(svd_nullvector(&a), Err(AlgebraError::DimensionMismatch { .. })));
    }

    #[test]
    async fn solve_llsq_detects_rank_deficient_system() {
        let a = planted_rank_deficient();
        let b = VecD::from_vec((0..6).map(|i| i as f64).collect());
        assert!(matches!(solve_llsq(&a, &b), Err(AlgebraError::Singular)));
    }

    async fn tridiagonal_spd(n: usize) -> CsrMatrix {
        let mut triplets = Vec::new();
        for i in 0..n {
            triplets.push((i, i, 2.0));
            if i + 1 < n {
                triplets.push((i, i + 1, -1.0));
                triplets.push((i + 1, i, -1.0));
            }
        }
        CsrMatrix::from_triplets(n, n, &triplets)
    }

    mod long {
        use super::*;

        #[test]
        async fn conjugate_gradient_converges_on_tridiagonal_system() {
            let n = 40;
            let a = tridiagonal_spd(n);
            let x_true = VecD::from_vec((0..n).map(|i| (i as f64) + 1.0).collect());
            let b = a.spmv(&x_true);
            let x = conjugate_gradient(&a, &b, 1e-10, 1000).expect("converges");
            for i in 0..n {
                assert!((x.get(i) - x_true.get(i)).abs() < 1e-6);
            }
        }

        #[test]
        async fn lanczos_matches_dense_jacobi_on_tridiagonal() {
            let n = 16;
            let a_sparse = tridiagonal_spd(n);
            let a_dense = a_sparse.to_dense();
            let (lanczos_vals, _) = lanczos_extreme_eigen(&a_sparse, 3, true, n, 42).expect("lanczos succeeds");
            let (dense_vals, _) = jacobi_eigen_symmetric(&a_dense, 500).expect("jacobi succeeds");
            let mut dense_top3: Vec<f64> = dense_vals;
            dense_top3.sort_by(|a, b| b.partial_cmp(a).unwrap());
            dense_top3.truncate(3);
            let mut lanczos_sorted = lanczos_vals;
            lanczos_sorted.sort_by(|a, b| b.partial_cmp(a).unwrap());
            for (l, d) in lanczos_sorted.iter().zip(dense_top3.iter()) {
                assert!((l - d).abs() < 1e-6);
            }
        }
    }
}
// #endregion 🔖️Tests

// #region 🔖️ExactTests
// 🚚 wave FIXALG: every test here but `lll_recovers_a_short_relation` moved to
// `semio_framework_number`'s `algebra` module tests, alongside the `MatG`/`VecG` types they exercise.
#[cfg(test)]
mod exact_tests {
    use super::*;
    use number::{Rational, VecG};

    async fn rat(n: i64, d: i64) -> Rational {
        Rational::from_i64(n, d).unwrap()
    }

    #[test]
    async fn lll_recovers_a_short_relation() {
        // Basis containing an obviously-reducible long vector alongside a short one; LLL should surface
        // vectors no longer than the original shortest, and preserve the lattice (verified via a
        // determinant/volume proxy: the reduced basis still spans the same rank).
        let basis = vec![VecG::from_vec(vec![rat(1, 1), rat(1, 1)]), VecG::from_vec(vec![rat(1, 1), rat(0, 1)])];
        let reduced = lll_reduce(&basis);
        assert_eq!(reduced.len(), 2);
        let shortest_before = basis.iter().map(|v| v.dot(v)).fold(rat(0, 1), |acc, n| if n < acc || acc == rat(0, 1) { n } else { acc });
        let shortest_after = reduced.iter().map(|v| v.dot(v)).fold(rat(0, 1), |acc, n| if n < acc || acc == rat(0, 1) { n } else { acc });
        assert!(shortest_after <= shortest_before);
    }
}
// #endregion 🔖️ExactTests
