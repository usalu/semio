//! 🔗️ Lattice reduction, sparse matrices and errors of the linear algebra: the LLL basis reduction, the compressed sparse row matrix and the algebra error. See [`super`].

use super::*;

// #region 🔖️Lll
/// 🔒️ LLL lattice basis reduction (`delta = 3/4`) over `Rational` rows, via Gram-Schmidt in exact
/// rational arithmetic. Powers integer-relation detection (e.g. recovering `x^2 - 2` from rational
/// approximations of `sqrt(2)`) and keeps polynomial-factor recombination upgrades (van Hoeij) open as
/// future work without needing a new dependency.
pub fn lll_reduce(basis: &[number::VecG<number::Rational>]) -> Vec<number::VecG<number::Rational>> {
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
    pub fn from_triplets(rows: usize, cols: usize, triplets: &[(usize, usize, f64)]) -> Self {
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

    pub fn nnz(&self) -> usize {
        self.values.len()
    }

    pub fn row(&self, r: usize) -> impl Iterator<Item = (usize, f64)> + '_ {
        let start = self.row_ptr[r];
        let end = self.row_ptr[r + 1];
        self.col_idx[start..end].iter().copied().zip(self.values[start..end].iter().copied())
    }

    /// 🕸️ Sparse matrix-vector product `A x`.
    pub fn spmv(&self, x: &VecD) -> VecD {
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

    pub fn transpose(&self) -> Self {
        let mut triplets = Vec::with_capacity(self.nnz());
        for row in 0..self.rows {
            for (col, value) in self.row(row) {
                triplets.push((col, row, value));
            }
        }
        Self::from_triplets(self.cols, self.rows, &triplets)
    }

    pub fn to_dense(&self) -> MatD {
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
