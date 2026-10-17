//! 🔁️ Iterative solvers and the SVD of the linear algebra: the power iteration, the conjugate gradient, the Pade matrix exponential, the singular value decomposition and the least-squares solvers built on it. See [`super`].

use super::*;

// #region 🔖️PowerIteration
/// 🔁️ Dominant eigenpair of a large sparse symmetric matrix via power iteration; convergence is measured via the residual `‖A x - λ x‖`. To recover further eigenpairs, deflate by rebuilding `a`'s triplets with `λ v vᵀ` subtracted and re-call — this returns a single eigenpair by design so callers control the deflation loop.
pub fn power_iteration(a: &CsrMatrix, max_iter: usize, tol: f64, seed: u64) -> Result<(f64, VecD), AlgebraError> {
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
pub fn conjugate_gradient(a: &CsrMatrix, b: &VecD, tol: f64, max_iter: usize) -> Result<VecD, AlgebraError> {
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
pub fn expm_pade(a: &MatD) -> MatD {
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
pub fn svd(a: &MatD) -> Result<(MatD, Vec<f64>, MatD), AlgebraError> {
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
pub fn svd_nullvector(a: &MatD) -> Result<VecD, AlgebraError> {
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
pub fn solve_llsq(a: &MatD, b: &VecD) -> Result<VecD, AlgebraError> {
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
pub fn pseudo_inverse(a: &MatD, tol: f64) -> Result<MatD, AlgebraError> {
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
pub fn weighted_normal_equations(a: &MatD, b: &VecD, w: &[f64]) -> (MatD, VecD) {
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
