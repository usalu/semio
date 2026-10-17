//! 📉️ Factorizations and symmetric eigenproblems of the linear algebra: the Cholesky and Householder QR factorizations and the Jacobi and Lanczos eigen solvers. See [`super`].

use super::*;

// #region 🔖️Cholesky
/// 🧮️ Dense Cholesky decomposition `A = L Lᵀ` of a symmetric positive-definite matrix via the standard column-by-column algorithm; returns `AlgebraError::NotPositiveDefinite` the moment a diagonal pivot goes non-positive.
pub fn cholesky(a: &MatD) -> Result<MatD, AlgebraError> {
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
pub fn cholesky_solve(l: &MatD, b: &VecD) -> VecD {
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
pub fn qr_householder(a: &MatD) -> (MatD, MatD) {
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
pub fn jacobi_eigen_symmetric(a: &MatD, max_sweeps: usize) -> Result<(Vec<f64>, MatD), AlgebraError> {
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
pub fn lanczos_extreme_eigen(a: &CsrMatrix, k: usize, largest: bool, max_iter: usize, seed: u64) -> Result<(Vec<f64>, Vec<VecD>), AlgebraError> {
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
