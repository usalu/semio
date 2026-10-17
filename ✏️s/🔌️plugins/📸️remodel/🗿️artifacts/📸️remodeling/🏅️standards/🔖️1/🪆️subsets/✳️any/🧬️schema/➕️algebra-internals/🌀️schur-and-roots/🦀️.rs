//! 🌀️ Hessenberg, real Schur and polynomial roots of the linear algebra: the Hessenberg reduction, the Francis double-shift Schur form, the real eigenvalues and the companion-matrix roots. See [`super`].

use super::*;

// #region 🔖️Hessenberg
/// 🪜️ Reduces a general real square matrix to upper Hessenberg form via Householder reflectors applied column-by-column below the first subdiagonal (Golub & Van Loan, Algorithm 7.4.2). Returns `(H, Q)` with `Q` orthogonal and `A = Q H Qᵀ`; `H` is zero below its first subdiagonal. This is the stable first stage of [`real_schur`]'s Francis double-shift QR iteration.
pub fn hessenberg(a: &MatD) -> (MatD, MatD) {
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
fn francis_double_shift_step(h: &mut MatD, q: &mut MatD, l: usize, m: usize) {
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
pub fn real_schur(a: &MatD) -> Result<(MatD, MatD), AlgebraError> {
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
pub fn real_eigenvalues(a: &MatD) -> Result<Vec<(f64, f64)>, AlgebraError> {
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
pub fn poly_roots_companion(coeffs: &[f64]) -> Result<Vec<(f64, f64)>, AlgebraError> {
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
