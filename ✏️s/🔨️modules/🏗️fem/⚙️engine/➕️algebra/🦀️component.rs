//! 🧮️ Dense linear-algebra basics (`Mat2`/`VecD`/`MatD`/`Mat3d`/`vec3d_*`) — FEM's own copy of the
//! ~300-LOC subset it actually calls out of `🧮️math/➕️algebra`'s 2,875-LOC surface, duplicated rather
//! than shared: FEM never touches the sparse/eigen/SVD/CG machinery `📸️remodel` needs (that bulk
//! moved wholesale into `📸️remodel`'s own `➕️algebra-internals` instead), measured via
//! `grep -rohE "math::algebra::[A-Za-z_]+" ✏️s/🔌️plugins/📸️remodel ✏️s/🔨️modules/🏗️fem` before this
//! split — see `26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS` wave M3d's report.

// #region 🔖️Mat2
/// 🧮️ 2x2 matrix, column-major storage; `new(a, b, c, d)` takes row-major entries of `[[a, b], [c, d]]`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Mat2 {
    pub cols: [[f64; 2]; 2],
}

impl Mat2 {
    pub const IDENTITY: Self = Self { cols: [[1.0, 0.0], [0.0, 1.0]] };

    pub fn new(a: f64, b: f64, c: f64, d: f64) -> Self {
        Self { cols: [[a, c], [b, d]] }
    }

    #[allow(clippy::should_implement_trait, reason = "value-semantics mul used pervasively as a plain method (not operator overload) by dependent crates outside this campaign wave's scope; renaming is a breaking API change")]
    pub fn mul(self, other: Self) -> Self {
        let entry = |row: usize, col: usize| self.cols[0][row] * other.cols[col][0] + self.cols[1][row] * other.cols[col][1];
        Self { cols: [[entry(0, 0), entry(1, 0)], [entry(0, 1), entry(1, 1)]] }
    }

    pub fn apply(self, v: (f64, f64)) -> (f64, f64) {
        (self.cols[0][0] * v.0 + self.cols[1][0] * v.1, self.cols[0][1] * v.0 + self.cols[1][1] * v.1)
    }

    pub fn det(self) -> f64 {
        self.cols[0][0] * self.cols[1][1] - self.cols[1][0] * self.cols[0][1]
    }

    pub fn trace(self) -> f64 {
        self.cols[0][0] + self.cols[1][1]
    }

    pub fn transpose(self) -> Self {
        Self::new(self.cols[0][0], self.cols[0][1], self.cols[1][0], self.cols[1][1])
    }

    pub fn inverse(self) -> Option<Self> {
        let d = self.det();
        if d.abs() < 1e-12 {
            return None;
        }
        let inv_d = 1.0 / d;
        Some(Self::new(self.cols[1][1] * inv_d, -self.cols[1][0] * inv_d, -self.cols[0][1] * inv_d, self.cols[0][0] * inv_d))
    }

    /// 🧮️ Real eigenvalues (if any) of a 2x2 matrix via the characteristic polynomial `λ² - tr·λ + det = 0`.
    pub fn eigenvalues(self) -> Option<(f64, f64)> {
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
    pub fn zeros(n: usize) -> Self {
        Self(vec![0.0; n])
    }

    pub fn from_vec(data: Vec<f64>) -> Self {
        Self(data)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get(&self, i: usize) -> f64 {
        self.0[i]
    }

    pub fn set(&mut self, i: usize, value: f64) {
        self.0[i] = value;
    }

    pub fn add_at(&mut self, i: usize, value: f64) {
        self.0[i] += value;
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.0.iter().zip(other.0.iter()).map(|(a, b)| a * b).sum()
    }

    pub fn scale(&self, s: f64) -> Self {
        Self(self.0.iter().map(|v| v * s).collect())
    }

    pub fn add(&self, other: &Self) -> Self {
        Self(self.0.iter().zip(other.0.iter()).map(|(a, b)| a + b).collect())
    }

    pub fn sub(&self, other: &Self) -> Self {
        Self(self.0.iter().zip(other.0.iter()).map(|(a, b)| a - b).collect())
    }

    pub fn norm2(&self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn norm_inf(&self) -> f64 {
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
    pub fn zeros(rows: usize, cols: usize) -> Self {
        Self { rows, cols, data: vec![0.0; rows * cols] }
    }

    pub fn identity(n: usize) -> Self {
        let mut m = Self::zeros(n, n);
        for i in 0..n {
            m.set(i, i, 1.0);
        }
        m
    }

    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.data[row * self.cols + col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: f64) {
        self.data[row * self.cols + col] = value;
    }

    pub fn add_at(&mut self, row: usize, col: usize, value: f64) {
        self.data[row * self.cols + col] += value;
    }

    pub fn transpose(&self) -> Self {
        let mut out = Self::zeros(self.cols, self.rows);
        for row in 0..self.rows {
            for col in 0..self.cols {
                out.set(col, row, self.get(row, col));
            }
        }
        out
    }

    pub fn matmul(&self, other: &Self) -> Self {
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

    pub fn mul_vec(&self, x: &VecD) -> VecD {
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
    pub fn add_triple_product(&mut self, b: &MatD, d: &MatD, weight: f64) {
        let btdb = b.transpose().matmul(d).matmul(b);
        for i in 0..self.data.len() {
            self.data[i] += weight * btdb.data[i];
        }
    }

    /// 🧮️ Solves `Ax = b` via Gaussian elimination with partial pivoting; `None` if `A` is singular.
    pub fn lu_solve(&self, b: &VecD) -> Option<VecD> {
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
    pub fn from_axes(x: [f64; 3], y: [f64; 3], z: [f64; 3]) -> Self {
        Self { cols: [x, y, z] }
    }

    pub fn transpose(self) -> Self {
        Self { cols: [[self.cols[0][0], self.cols[1][0], self.cols[2][0]], [self.cols[0][1], self.cols[1][1], self.cols[2][1]], [self.cols[0][2], self.cols[1][2], self.cols[2][2]]] }
    }

    #[allow(clippy::should_implement_trait, reason = "value-semantics mul used pervasively as a plain method (not operator overload) by dependent crates outside this campaign wave's scope; renaming is a breaking API change")]
    pub fn mul(self, other: Self) -> Self {
        let entry = |row: usize, col: usize| self.cols[0][row] * other.cols[col][0] + self.cols[1][row] * other.cols[col][1] + self.cols[2][row] * other.cols[col][2];
        Self { cols: [[entry(0, 0), entry(1, 0), entry(2, 0)], [entry(0, 1), entry(1, 1), entry(2, 1)], [entry(0, 2), entry(1, 2), entry(2, 2)]] }
    }

    pub fn mul_vec3(self, v: [f64; 3]) -> [f64; 3] {
        [self.cols[0][0] * v[0] + self.cols[1][0] * v[1] + self.cols[2][0] * v[2], self.cols[0][1] * v[0] + self.cols[1][1] * v[1] + self.cols[2][1] * v[2], self.cols[0][2] * v[0] + self.cols[1][2] * v[1] + self.cols[2][2] * v[2]]
    }
}

pub fn vec3d_sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

pub fn vec3d_length(v: [f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

pub fn vec3d_normalize(v: [f64; 3]) -> [f64; 3] {
    let len = vec3d_length(v);
    if len < 1e-12 {
        return [0.0, 0.0, 0.0];
    }
    [v[0] / len, v[1] / len, v[2] / len]
}

pub fn vec3d_cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}
// #endregion 🔖️Mat3d
