//! ➕️ Minimal dense f64 linear algebra: `VecD`/`MatD` with Gaussian-elimination `lu_solve` — the
//! exact subset `🌫️fuzzy-internals` needs (least-squares fuzzy-rule learning, ANFIS parameter fits).
//!
//! Extracted from `🧰️framework/🔨️modules/🧮️math/➕️algebra` (ticket 26/08/12/
//! DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS, wave M3e) after a CONCURRENT wave
//! (M3d) moved that crate's full algebra module into `📸️remodel`'s own artifact schema, verified at
//! the time as `📸️remodel`'s sole consumer — a verification `🌫️fuzzy`'s later, still-in-flight move
//! into stdio invalidated (a second, independent consumer M3d's grep couldn't have seen). Rather than
//! stdio (domain-neutral) depending on `📸️remodel` (a domain-specific plugin) — the wrong layering
//! direction per this repo's own multi-app rule — this duplicates just `VecD`/`MatD`, mirroring the
//! SAME precedent `🏗️fem` already set for its own `➕️algebra` needs (see that crate's own module).
//! Zero external deps (no `crate::`, no other math sibling) — verbatim body, byte-identical to the
//! deleted original's `VecD`/`MatD` region.

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
