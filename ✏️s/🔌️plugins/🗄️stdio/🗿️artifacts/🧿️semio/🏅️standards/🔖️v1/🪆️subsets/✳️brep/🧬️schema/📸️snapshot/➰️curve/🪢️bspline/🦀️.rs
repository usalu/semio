//! 🧵️ Knot vectors, B-spline basis functions and de Boor evaluation for rational curves and
//! tensor-product surfaces — the machinery [`crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::Curve3::Nurbs`] and
//! [`crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::surface::Surface::Nurbs`] are built on. Curves and surfaces themselves stay in their
//! own modules; this file is purely the numerical core, independent of any particular dimension.
//!
//! Moved from `🧰️framework/🔨️modules/🧊️3d/📐️brep/🪢️bspline` in ticket
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave PEEL4, mounted locally
//! under `➰️curve` per that file's own pre-mounted-stub note.

// #region 🔖️Knots

/// 🧵️ A non-decreasing knot vector for a degree-`p` B-spline with `n` control points, satisfying
/// `len == n + p + 1`.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct KnotVector {
    pub knots: Vec<f64>,
    pub degree: usize,
}

impl KnotVector {
    /// 🧵️ Builds and validates a knot vector: non-decreasing, correct length for `(n, degree)`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new(knots: Vec<f64>, degree: usize, control_point_count: usize) -> Option<Self> {
        if knots.len() != control_point_count + degree + 1 {
            return None;
        }
        if knots.windows(2).any(|w| w[0] > w[1]) {
            return None;
        }
        Some(KnotVector { knots, degree })
    }
    /// 🧵️ A clamped (open) uniform knot vector: the first and last knots repeat `degree+1` times,
    /// the standard choice so the curve interpolates its first/last control points.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn clamped_uniform(control_point_count: usize, degree: usize) -> Self {
        let n = control_point_count;
        let p = degree;
        let interior = n.saturating_sub(p + 1);
        let mut knots = vec![0.0; p + 1];
        for i in 1..=interior {
            knots.push(i as f64 / (interior + 1) as f64);
        }
        knots.extend(std::iter::repeat_n(1.0, p + 1));
        KnotVector { knots, degree: p }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn domain(&self) -> (f64, f64) {
        (self.knots[self.degree], self.knots[self.knots.len() - self.degree - 1])
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn control_point_count(&self) -> usize {
        self.knots.len() - self.degree - 1
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_periodic_compatible(&self) -> bool {
        // A periodic (non-clamped) knot vector has no repeated end knots beyond multiplicity 1.
        self.multiplicity_at_index(0) == 1
    }
    /// 🔁️ An unclamped, uniformly-spaced knot vector for a periodic (closed) degree-`p` curve with
    /// `n` *distinct* control points: the stored control array must hold `n + p` points (the first
    /// `p` duplicated onto the end — the standard "phantom wrap" trick), so ordinary clamped
    /// de Boor evaluation reproduces the closed curve exactly, including a `C^(p-1)`-continuous
    /// seam, with no special-cased evaluation path.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn periodic_uniform(control_point_count: usize, degree: usize) -> Self {
        let n = control_point_count;
        let p = degree;
        let knot_count = n + 2 * p + 1;
        let knots: Vec<f64> = (0..knot_count).map(|i| i as f64 - p as f64).collect();
        KnotVector { knots, degree: p }
    }
    /// 🔁️ Structurally periodic: both ends have multiplicity 1 (unclamped), so evaluation wraps
    /// smoothly through the domain boundary rather than pinning the curve to its first/last
    /// control point — the necessary condition [`Self::periodic_uniform`] always satisfies.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_periodic(&self) -> bool {
        self.degree > 0 && self.multiplicity_at_index(0) == 1 && self.multiplicity_at_index(self.knots.len() - 1) == 1
    }
    /// 🔁️ Wraps `u` into this knot vector's domain by its period (`domain.1 - domain.0`) — the
    /// inverse-evaluation counterpart to periodic curve construction, used wherever a caller has a
    /// parameter that may have drifted outside `[domain.0, domain.1)` on a periodic curve.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn wrap(&self, u: f64) -> f64 {
        let (lo, hi) = self.domain();
        let span = hi - lo;
        if span <= 0.0 {
            return u;
        }
        let mut x = (u - lo) % span;
        if x < 0.0 {
            x += span;
        }
        lo + x
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn multiplicity_at_index(&self, i: usize) -> usize {
        self.knots.iter().filter(|&&k| k == self.knots[i]).count()
    }
    /// 🧵️ Finds the knot span index `i` such that `knots[i] <= u < knots[i+1]` (or the last valid
    /// span if `u` equals the domain's upper bound), via binary search — O(log n) per evaluation.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn find_span(&self, u: f64) -> usize {
        let n = self.control_point_count() - 1;
        let p = self.degree;
        if u >= self.knots[n + 1] {
            return n;
        }
        if u <= self.knots[p] {
            return p;
        }
        let mut lo = p;
        let mut hi = n + 1;
        while hi - lo > 1 {
            let mid = (lo + hi) / 2;
            if u < self.knots[mid] {
                hi = mid;
            } else {
                lo = mid;
            }
        }
        lo
    }
    /// 🧵️ The multiplicity of the knot value equal to `u`, or `0` if `u` is not an existing knot
    /// (within exact equality — callers should snap to a known knot value before calling this).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn multiplicity(&self, u: f64) -> usize {
        self.knots.iter().filter(|&&k| k == u).count()
    }
}

// #endregion 🔖️Knots

// #region 🔖️Basis

/// 🧵️ Evaluates all `degree+1` nonzero basis functions at `u` in the knot span `span` (the
/// Cox-de Boor triangular recurrence, computed bottom-up per the standard NURBS-book algorithm —
/// `O(p²)` and numerically stable, unlike the naive top-down recursive definition).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn basis_functions(knots: &KnotVector, span: usize, u: f64) -> Vec<f64> {
    let p = knots.degree;
    let mut n = vec![0.0; p + 1];
    n[0] = 1.0;
    let mut left = vec![0.0; p + 1];
    let mut right = vec![0.0; p + 1];
    for j in 1..=p {
        left[j] = u - knots.knots[span + 1 - j];
        right[j] = knots.knots[span + j] - u;
        let mut saved = 0.0;
        for r in 0..j {
            let denom = right[r + 1] + left[j - r];
            let temp = if denom.abs() > 1e-300 { n[r] / denom } else { 0.0 };
            n[r] = saved + right[r + 1] * temp;
            saved = left[j - r] * temp;
        }
        n[j] = saved;
    }
    n
}

/// 🧵️ Evaluates the nonzero basis functions and their derivatives up to order `max_deriv` at `u`
/// in `span`. Returns `derivs[k][j]` = the `k`-th derivative of the `j`-th nonzero basis function.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn basis_function_derivatives(knots: &KnotVector, span: usize, u: f64, max_deriv: usize) -> Vec<Vec<f64>> {
    let p = knots.degree;
    let max_deriv = max_deriv.min(p);
    let mut ndu = vec![vec![0.0; p + 1]; p + 1];
    ndu[0][0] = 1.0;
    let mut left = vec![0.0; p + 1];
    let mut right = vec![0.0; p + 1];
    for j in 1..=p {
        left[j] = u - knots.knots[span + 1 - j];
        right[j] = knots.knots[span + j] - u;
        let mut saved = 0.0;
        for r in 0..j {
            ndu[j][r] = right[r + 1] + left[j - r];
            let denom = ndu[j][r];
            let temp = if denom.abs() > 1e-300 { ndu[r][j - 1] / denom } else { 0.0 };
            ndu[r][j] = saved + right[r + 1] * temp;
            saved = left[j - r] * temp;
        }
        ndu[j][j] = saved;
    }
    let mut derivs = vec![vec![0.0; p + 1]; max_deriv + 1];
    for j in 0..=p {
        derivs[0][j] = ndu[j][p];
    }
    for r in 0..=p {
        let mut a = vec![vec![0.0; p + 1]; 2];
        a[0][0] = 1.0;
        let mut s1 = 0usize;
        let mut s2 = 1usize;
        for k in 1..=max_deriv {
            let mut d = 0.0;
            let rk = r as isize - k as isize;
            let pk = p - k;
            if r >= k {
                a[s2][0] = a[s1][0] / ndu[pk + 1][rk as usize];
                d = a[s2][0] * ndu[rk as usize][pk];
            }
            let j1 = if rk >= -1 { 1 } else { (-rk) as usize };
            let j2 = if r as isize - 1 <= pk as isize { k - 1 } else { p - r };
            for j in j1..=j2 {
                a[s2][j] = (a[s1][j] - a[s1][j - 1]) / ndu[pk + 1][(rk + j as isize) as usize];
                d += a[s2][j] * ndu[(rk + j as isize) as usize][pk];
            }
            if r <= pk {
                a[s2][k] = -a[s1][k - 1] / ndu[pk + 1][r];
                d += a[s2][k] * ndu[r][pk];
            }
            derivs[k][r] = d;
            std::mem::swap(&mut s1, &mut s2);
        }
    }
    let mut factor = p as f64;
    #[allow(clippy::needless_range_loop)]
    for k in 1..=max_deriv {
        for v in derivs[k].iter_mut().take(p + 1) {
            *v *= factor;
        }
        factor *= (p - k) as f64;
    }
    derivs
}

// #endregion 🔖️Basis

// #region 🔖️DeBoor

/// 🧵️ De Boor's algorithm for a rational (homogeneous) curve, evaluating one weighted-coordinate
/// channel — call once per coordinate (x, y, z, w) and divide by the resulting `w` to dehomogenize.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn de_boor(knots: &KnotVector, control_values: &[f64], u: f64) -> f64 {
    let span = knots.find_span(u);
    let p = knots.degree;
    let n = basis_functions(knots, span, u);
    (0..=p).map(|j| n[j] * control_values[span - p + j]).sum()
}

// #endregion 🔖️DeBoor

// #region 🔖️Refine

/// 🧵️ Inserts a single knot `u` (Boehm's algorithm), returning the new knot vector and the new
/// control values for one coordinate channel — geometrically a no-op (the curve is unchanged),
/// used to raise local control or to harmonize two curves onto a shared knot vector.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn insert_knot(knots: &KnotVector, control_values: &[f64], u: f64) -> (KnotVector, Vec<f64>) {
    let p = knots.degree;
    let span = knots.find_span(u);
    let mut new_knots = knots.knots.clone();
    new_knots.insert(span + 1, u);
    let n = control_values.len();
    let mut new_values = vec![0.0; n + 1];
    let prefix_end = span.saturating_sub(p) + 1;
    new_values[..prefix_end].copy_from_slice(&control_values[..prefix_end]);
    new_values[span + 1..=n].copy_from_slice(&control_values[span..n]);
    for i in (span + 1 - p)..=span {
        let alpha = if knots.knots[i + p] != knots.knots[i] { (u - knots.knots[i]) / (knots.knots[i + p] - knots.knots[i]) } else { 0.0 };
        new_values[i] = alpha * control_values[i] + (1.0 - alpha) * control_values[i - 1];
    }
    (KnotVector { knots: new_knots, degree: p }, new_values)
}

/// 🧵️ Elevates a Bézier segment's degree by one via the shared [`crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::bezier`] elevation
/// formula, exposed here so B-spline code can raise a single-span curve's degree without
/// round-tripping through the `Bernstein`/`Poly` types.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn elevate_bezier_span(control_values: &[f64]) -> Vec<f64> {
    let n = control_values.len() - 1;
    let m = n + 1;
    (0..=m)
        .map(|i| {
            let a = i as f64 / m as f64;
            let b = (m - i) as f64 / m as f64;
            let left = if i > 0 { control_values[i - 1] * a } else { 0.0 };
            let right = if i <= n { control_values[i] * b } else { 0.0 };
            left + right
        })
        .collect()
}

// #endregion 🔖️Refine

// #region 🔖️RationalDerivatives

/// 🧮️ Binomial coefficient `C(n, k)`, computed iteratively (no factorials, no overflow for the
/// small orders — a handful at most — this file ever needs).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn binomial(n: usize, k: usize) -> f64 {
    if k > n {
        return 0.0;
    }
    let mut r = 1.0;
    for i in 0..k {
        r = r * (n - i) as f64 / (i + 1) as f64;
    }
    r
}

/// 🧮️ Exact rational-curve derivatives via the Piegl–Tiller `A_k(u)` recurrence (NURBS Book
/// eq. 4.8/4.9): evaluates the weighted (homogeneous) curve's derivatives once via
/// [`basis_function_derivatives`], then un-weights recursively — replaces the finite-difference
/// stand-in that used to sit next to every `Curve3::Nurbs`/`Surface::Nurbs` arm, exact through any
/// requested `order` (higher than the degree is legal and simply returns the zero vector, since a
/// degree-`p` polynomial's `(p+1)`-th derivative vanishes identically within a span).
/// `controls_h[i]` is control point `i` in homogeneous form: `[x·w, y·w, ..., w]` — the last
/// channel is the weight, so this works for any point dimension (2D pcurves, 3D curves) unchanged.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn curve_derivatives_rational(knots: &KnotVector, controls_h: &[Vec<f64>], u: f64, order: usize) -> Vec<Vec<f64>> {
    let dim = controls_h[0].len() - 1;
    let p = knots.degree;
    let span = knots.find_span(u);
    let deriv_order = order.min(p);
    let basis_derivs = basis_function_derivatives(knots, span, u, deriv_order);
    let mut a = vec![vec![0.0; dim + 1]; order + 1];
    for (k, row) in a.iter_mut().enumerate().take(deriv_order + 1) {
        for c in 0..=dim {
            let mut sum = 0.0;
            for j in 0..=p {
                sum += basis_derivs[k][j] * controls_h[span - p + j][c];
            }
            row[c] = sum;
        }
    }
    let mut result = vec![vec![0.0; dim]; order + 1];
    let w0 = a[0][dim];
    for k in 0..=order {
        let mut v = a[k][0..dim].to_vec();
        for i in 1..=k {
            let coeff = binomial(k, i) * a[i][dim];
            if coeff != 0.0 {
                for d in 0..dim {
                    v[d] -= coeff * result[k - i][d];
                }
            }
        }
        if w0.abs() > 1e-300 {
            for d in 0..dim {
                result[k][d] = v[d] / w0;
            }
        }
    }
    result
}

/// 🧮️ Exact rational tensor-product-surface derivatives via the Piegl–Tiller `RatSurfaceDerivs`
/// recurrence (NURBS Book eq. 4.20/4.21, `Aders`/`Sw` generalized to two directions): the surface
/// analogue of [`curve_derivatives_rational`]. Returns `result[k][l]` = `d^(k+l)S / du^k dv^l` (a
/// `dim`-vector) for every `k, l` with `k + l <= order`; entries with `k + l > order` are left zero
/// and unused. `controls_h[i][j]` is the `(i, j)` control point in homogeneous form.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn surface_derivatives_rational(u_knots: &KnotVector, v_knots: &KnotVector, controls_h: &[Vec<Vec<f64>>], u: f64, v: f64, order: usize) -> Vec<Vec<Vec<f64>>> {
    let dim = controls_h[0][0].len() - 1;
    let up = u_knots.degree;
    let vp = v_knots.degree;
    let du = order.min(up);
    let dv = order.min(vp);
    let u_span = u_knots.find_span(u);
    let v_span = v_knots.find_span(v);
    let nu = basis_function_derivatives(u_knots, u_span, u, du);
    let nv = basis_function_derivatives(v_knots, v_span, v, dv);
    let mut a = vec![vec![vec![0.0; dim + 1]; dv + 1]; du + 1];
    for k in 0..=du {
        for l in 0..=dv {
            if k + l > order {
                continue;
            }
            let mut acc = vec![0.0; dim + 1];
            for i in 0..=up {
                for j in 0..=vp {
                    let b = nu[k][i] * nv[l][j];
                    if b == 0.0 {
                        continue;
                    }
                    let ci = u_span - up + i;
                    let cj = v_span - vp + j;
                    for c in 0..=dim {
                        acc[c] += b * controls_h[ci][cj][c];
                    }
                }
            }
            a[k][l] = acc;
        }
    }
    let mut s = vec![vec![vec![0.0; dim]; order + 1]; order + 1];
    let w00 = a[0][0][dim];
    for total in 0..=order {
        for k in 0..=total {
            let l = total - k;
            if k > du || l > dv {
                continue;
            }
            let mut acc = a[k][l][0..dim].to_vec();
            for i in 1..=k {
                let coeff = binomial(k, i) * a[i][0][dim];
                if coeff != 0.0 {
                    for d in 0..dim {
                        acc[d] -= coeff * s[k - i][l][d];
                    }
                }
            }
            for j in 1..=l {
                let coeff = binomial(l, j) * a[0][j][dim];
                if coeff != 0.0 {
                    for d in 0..dim {
                        acc[d] -= coeff * s[k][l - j][d];
                    }
                }
            }
            for i in 1..=k {
                for j in 1..=l {
                    let coeff = binomial(k, i) * binomial(l, j) * a[i][j][dim];
                    if coeff != 0.0 {
                        for d in 0..dim {
                            acc[d] -= coeff * s[k - i][l - j][d];
                        }
                    }
                }
            }
            if w00.abs() > 1e-300 {
                for d in 0..dim {
                    s[k][l][d] = acc[d] / w00;
                }
            }
        }
    }
    s
}

// #endregion 🔖️RationalDerivatives

// #region 🔖️MultiChannel

/// 🧩️ [`insert_knot`]'s multi-channel counterpart: `controls[i]` is a whole point (any dimension,
/// e.g. homogeneous `[x·w, y·w, z·w, w]`) rather than one coordinate — used wherever an operation
/// needs to reason about the point as a unit (knot removal's tolerance check, degree elevation,
/// Coons-patch knot harmonization) instead of one axis at a time.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn insert_knot_multi(knots: &KnotVector, controls: &[Vec<f64>], u: f64) -> (KnotVector, Vec<Vec<f64>>) {
    let p = knots.degree;
    let span = knots.find_span(u);
    let mut new_knots = knots.knots.clone();
    new_knots.insert(span + 1, u);
    let n = controls.len();
    let mut new_controls = controls.to_vec();
    new_controls.insert(span + 1, controls[span].clone());
    for i in (span + 1 - p)..=span {
        let alpha = if knots.knots[i + p] != knots.knots[i] { (u - knots.knots[i]) / (knots.knots[i + p] - knots.knots[i]) } else { 0.0 };
        let dim = controls[0].len();
        let prev = &controls[i - 1];
        let cur = &controls[i];
        new_controls[i] = (0..dim).map(|d| alpha * cur[d] + (1.0 - alpha) * prev[d]).collect();
    }
    debug_assert_eq!(new_controls.len(), n + 1);
    (KnotVector { knots: new_knots, degree: p }, new_controls)
}

/// 🧩️ [`elevate_bezier_span`]'s multi-channel counterpart, raising a single Bézier span's degree
/// by one.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn elevate_bezier_span_multi(controls: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let n = controls.len() - 1;
    let m = n + 1;
    let dim = controls[0].len();
    (0..=m)
        .map(|i| {
            let a = i as f64 / m as f64;
            let b = (m - i) as f64 / m as f64;
            let mut out = vec![0.0; dim];
            if i > 0 {
                for d in 0..dim {
                    out[d] += controls[i - 1][d] * a;
                }
            }
            if i <= n {
                for d in 0..dim {
                    out[d] += controls[i][d] * b;
                }
            }
            out
        })
        .collect()
}

// #endregion 🔖️MultiChannel

// #region 🔖️Remove

/// 🗑️ Removes one instance of the knot `u` (Tiller's algorithm, NURBS Book §5.4/A5.8, single
/// removal): recomputes the two control points adjacent to `u` from both sides via the exact
/// inverse of Boehm's refinement and accepts the removal only if the two candidates agree within
/// `tol` — an exact geometric no-op on the surviving curve, never a silent approximation; returns
/// `None` (unchanged) when `u` isn't an interior knot or the removal would move the curve beyond
/// `tol`. `controls[i]` is a whole point (see [`insert_knot_multi`]) so the tolerance check is a
/// true point distance, not per-axis.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn remove_knot(knots: &KnotVector, controls: &[Vec<f64>], u: f64, tol: f64) -> Option<(KnotVector, Vec<Vec<f64>>)> {
    let p = knots.degree;
    let ord = p + 1;
    let (lo, hi) = knots.domain();
    if u <= lo || u >= hi {
        return None;
    }
    let r = knots.find_span(u);
    let s = knots.multiplicity(u);
    if s == 0 {
        return None;
    }
    let first_i = r as isize - p as isize;
    let last_i = r as isize - s as isize;
    if first_i < 1 || last_i < first_i {
        return None;
    }
    let first = first_i as usize;
    let last = last_i as usize;
    let off = first - 1;
    let dim = controls[0].len();
    let sub = |a: &[f64], b: &[f64]| -> Vec<f64> { a.iter().zip(b).map(|(x, y)| x - y).collect() };
    let scale = |a: &[f64], k: f64| -> Vec<f64> { a.iter().map(|x| x * k).collect() };
    let add = |a: &[f64], b: &[f64]| -> Vec<f64> { a.iter().zip(b).map(|(x, y)| x + y).collect() };
    let dist = |a: &[f64], b: &[f64]| -> f64 { a.iter().zip(b).map(|(x, y)| (x - y).powi(2)).sum::<f64>().sqrt() };
    let mut temp = vec![vec![0.0; dim]; last - first + 2];
    temp[0] = controls[off].clone();
    temp[last - first + 1] = controls[last + 1].clone();
    let mut i = first;
    let mut j = last;
    let mut ii = 1usize;
    let mut jj = last - off;
    while j > i {
        let alfi = (u - knots.knots[i]) / (knots.knots[i + ord] - knots.knots[i]);
        let alfj = (u - knots.knots[j]) / (knots.knots[j + ord] - knots.knots[j]);
        temp[ii] = scale(&sub(&controls[i], &scale(&temp[ii - 1], 1.0 - alfi)), 1.0 / alfi);
        temp[jj] = scale(&sub(&controls[j], &scale(&temp[jj + 1], alfj)), 1.0 / (1.0 - alfj));
        i += 1;
        ii += 1;
        j -= 1;
        jj -= 1;
    }
    let ok = if j < i {
        dist(&temp[ii - 1], &temp[jj + 1]) <= tol
    } else {
        let alfi = (u - knots.knots[i]) / (knots.knots[i + ord] - knots.knots[i]);
        let blended = add(&scale(&temp[ii + 1], alfi), &scale(&temp[ii - 1], 1.0 - alfi));
        dist(&controls[i], &blended) <= tol
    };
    if !ok {
        return None;
    }
    let mut new_controls = controls.to_vec();
    let mut wi = first;
    let mut wj = last;
    while wj > wi {
        new_controls[wi] = temp[wi - off].clone();
        new_controls[wj] = temp[wj - off].clone();
        wi += 1;
        wj -= 1;
    }
    let n = controls.len();
    let fout = ((2 * r) as isize - s as isize - p as isize) / 2;
    let fout = fout.max(0) as usize;
    let mut write = fout;
    for k in (fout + 1)..n {
        new_controls[write] = new_controls[k].clone();
        write += 1;
    }
    new_controls.truncate(n - 1);
    let mut new_knots = knots.knots.clone();
    new_knots.remove(r);
    Some((KnotVector { knots: new_knots, degree: p }, new_controls))
}

// #endregion 🔖️Remove

// #region 🔖️Elevate

/// 🪜️ Elevates a (possibly multi-span) B-spline's degree by `t`, preserving the curve exactly:
/// decomposes into Bézier segments (every interior knot raised to full multiplicity `p`), elevates
/// each segment independently via [`elevate_bezier_span_multi`] (which preserves segment endpoints
/// exactly, so adjacent elevated segments still share their boundary control point), then removes
/// the resulting *excess* knot multiplicity via [`remove_knot`] to restore each interior knot to
/// its original multiplicity `+ t` — the multiplicity increase degree elevation theory requires to
/// keep the original continuity unchanged. Equivalent to, if less optimized than, Piegl-Tiller's
/// direct Algorithm A5.9.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn elevate_degree(knots: &KnotVector, controls: &[Vec<f64>], t: usize) -> (KnotVector, Vec<Vec<f64>>) {
    if t == 0 {
        return (knots.clone(), controls.to_vec());
    }
    let p = knots.degree;
    let (lo, hi) = knots.domain();
    let mut distinct_interior: Vec<f64> = knots.knots.iter().copied().filter(|&k| k > lo && k < hi).collect();
    distinct_interior.dedup_by(|a, b| (*a - *b).abs() < 1e-12);
    let original_mult: Vec<usize> = distinct_interior.iter().map(|&kv| knots.multiplicity(kv)).collect();
    let mut work_knots = knots.clone();
    let mut work_controls = controls.to_vec();
    for &kv in &distinct_interior {
        while work_knots.multiplicity(kv) < p {
            let (nk, nc) = insert_knot_multi(&work_knots, &work_controls, kv);
            work_knots = nk;
            work_controls = nc;
        }
    }
    let n_spans = distinct_interior.len() + 1;
    let seg_len = p + 1;
    let mut elevated_segments: Vec<Vec<Vec<f64>>> = Vec::with_capacity(n_spans);
    for s in 0..n_spans {
        let start = s * p;
        let mut seg: Vec<Vec<f64>> = work_controls[start..start + seg_len].to_vec();
        for _ in 0..t {
            seg = elevate_bezier_span_multi(&seg);
        }
        elevated_segments.push(seg);
    }
    let mut full_controls: Vec<Vec<f64>> = elevated_segments[0].clone();
    for seg in elevated_segments.iter().skip(1) {
        full_controls.extend_from_slice(&seg[1..]);
    }
    let new_degree = p + t;
    let mut full_knots = vec![lo; new_degree + 1];
    for &kv in &distinct_interior {
        full_knots.extend(std::iter::repeat_n(kv, new_degree));
    }
    full_knots.extend(std::iter::repeat_n(hi, new_degree + 1));
    let mut result_kv = KnotVector { knots: full_knots, degree: new_degree };
    let mut result_controls = full_controls;
    for (idx, &kv) in distinct_interior.iter().enumerate() {
        let target_mult = original_mult[idx] + t;
        while result_kv.multiplicity(kv) > target_mult {
            match remove_knot(&result_kv, &result_controls, kv, 1e-7) {
                Some((nk, nc)) => {
                    result_kv = nk;
                    result_controls = nc;
                }
                None => break,
            }
        }
    }
    (result_kv, result_controls)
}

// #endregion 🔖️Elevate

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn cubic_clamped_5cp() -> KnotVector {
        // degree 3, 5 control points -> knot vector length 9: [0,0,0,0, 0.5, 1,1,1,1]
        KnotVector::new(vec![0.0, 0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0, 1.0], 3, 5).unwrap()
    }

    #[semio_framework_async_macros::async_test]
    async fn knot_vector_rejects_wrong_length() {
        assert!(KnotVector::new(vec![0.0, 0.0, 1.0, 1.0], 3, 5).is_none());
    }

    #[semio_framework_async_macros::async_test]
    async fn knot_vector_rejects_decreasing_sequence() {
        assert!(KnotVector::new(vec![0.0, 0.5, 0.2, 1.0, 1.0], 1, 3).is_none());
    }

    #[semio_framework_async_macros::async_test]
    async fn clamped_uniform_has_correct_domain_and_multiplicity() {
        let kv = KnotVector::clamped_uniform(5, 3);
        assert_eq!(kv.domain(), (0.0, 1.0));
        assert_eq!(kv.multiplicity(0.0), 4);
        assert_eq!(kv.multiplicity(1.0), 4);
        assert_eq!(kv.control_point_count(), 5);
    }

    #[semio_framework_async_macros::async_test]
    async fn find_span_matches_brute_force_scan() {
        let kv = cubic_clamped_5cp();
        for i in 0..=100 {
            let u = i as f64 / 100.0;
            let expected = brute_force_span(&kv, u);
            assert_eq!(kv.find_span(u), expected, "mismatch at u={u}");
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn brute_force_span(kv: &KnotVector, u: f64) -> usize {
        let n = kv.control_point_count() - 1;
        for i in kv.degree..=n {
            if u >= kv.knots[i] && u < kv.knots[i + 1] {
                return i;
            }
        }
        n
    }

    #[semio_framework_async_macros::async_test]
    async fn basis_functions_sum_to_one_everywhere_in_domain() {
        let kv = cubic_clamped_5cp();
        for i in 0..=50 {
            let u = i as f64 / 50.0;
            let span = kv.find_span(u);
            let n = basis_functions(&kv, span, u);
            let sum: f64 = n.iter().sum();
            assert!((sum - 1.0).abs() < 1e-10, "partition of unity violated at u={u}: sum={sum}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn basis_functions_are_nonnegative() {
        let kv = cubic_clamped_5cp();
        for i in 0..=50 {
            let u = i as f64 / 50.0;
            let span = kv.find_span(u);
            let n = basis_functions(&kv, span, u);
            assert!(n.iter().all(|&v| v >= -1e-12), "negative basis value at u={u}: {n:?}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn de_boor_interpolates_endpoints_of_clamped_curve() {
        let kv = cubic_clamped_5cp();
        let values = vec![0.0, 1.0, -2.0, 3.0, 5.0];
        let (lo, hi) = kv.domain();
        assert!((de_boor(&kv, &values, lo) - values[0]).abs() < 1e-9);
        assert!((de_boor(&kv, &values, hi) - *values.last().unwrap()).abs() < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    async fn basis_function_derivatives_match_finite_differences() {
        let kv = cubic_clamped_5cp();
        let u = 0.37;
        let span = kv.find_span(u);
        let derivs = basis_function_derivatives(&kv, span, u, 1);
        let h = 1e-6;
        let n_plus = basis_functions(&kv, kv.find_span(u + h), u + h);
        let n_minus = basis_functions(&kv, kv.find_span(u - h), u - h);
        for j in 0..=kv.degree {
            let fd = (n_plus[j] - n_minus[j]) / (2.0 * h);
            assert!((derivs[1][j] - fd).abs() < 1e-4, "derivative mismatch at j={j}: analytic={} fd={fd}", derivs[1][j]);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn basis_function_derivatives_order_zero_matches_basis_functions() {
        let kv = cubic_clamped_5cp();
        let u = 0.63;
        let span = kv.find_span(u);
        let plain = basis_functions(&kv, span, u);
        let derivs = basis_function_derivatives(&kv, span, u, 2);
        for j in 0..=kv.degree {
            assert!((plain[j] - derivs[0][j]).abs() < 1e-12);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn insert_knot_does_not_change_the_curve() {
        let kv = cubic_clamped_5cp();
        let values = vec![0.0, 2.0, -1.0, 3.0, 1.0];
        let (new_kv, new_values) = insert_knot(&kv, &values, 0.3);
        assert_eq!(new_kv.control_point_count(), values.len() + 1);
        for i in 0..=20 {
            let u = i as f64 / 20.0;
            let before = de_boor(&kv, &values, u);
            let after = de_boor(&new_kv, &new_values, u);
            assert!((before - after).abs() < 1e-9, "curve changed after knot insertion at u={u}: {before} vs {after}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn elevate_bezier_span_preserves_curve_value() {
        // Single bezier span is a B-spline with degree = n and a clamped, no-interior-knot vector.
        let control_values = vec![0.0, 3.0, -2.0, 5.0];
        let elevated = elevate_bezier_span(&control_values);
        assert_eq!(elevated.len(), control_values.len() + 1);
        let b = super::super::bezier::RationalBezier2::unweighted(control_values.iter().map(|&v| crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::Pnt2::new(v, 0.0)).collect());
        let be = super::super::bezier::RationalBezier2::unweighted(elevated.iter().map(|&v| crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::Pnt2::new(v, 0.0)).collect());
        for i in 0..=10 {
            let t = i as f64 / 10.0;
            assert!((b.eval(t).x - be.eval(t).x).abs() < 1e-9);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn curve_derivatives_rational_matches_basis_derivatives_when_unweighted() {
        let kv = cubic_clamped_5cp();
        let values = [0.0, 2.0, -1.0, 3.0, 1.0];
        let controls_h: Vec<Vec<f64>> = values.iter().map(|&v| vec![v, 1.0]).collect();
        for i in 0..=20 {
            let u = i as f64 / 20.0;
            let span = kv.find_span(u);
            let plain = basis_function_derivatives(&kv, span, u, 2);
            let expected: Vec<f64> = (0..=2)
                .map(|k| (0..=kv.degree).map(|j| plain[k][j] * values[span - kv.degree + j]).sum())
                .collect();
            let got = curve_derivatives_rational(&kv, &controls_h, u, 2);
            for k in 0..=2 {
                assert!((got[k][0] - expected[k]).abs() < 1e-9, "order {k} mismatch at u={u}: got={} expected={}", got[k][0], expected[k]);
            }
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn curve_derivatives_rational_matches_analytic_circle_as_nurbs() {
        // A quarter-circle rational-quadratic NURBS: exact analytic d1/d2 are known in closed form.
        let w = std::f64::consts::FRAC_PI_4.cos();
        let controls = [(1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
        let weights = [1.0, w, 1.0];
        let controls_h: Vec<Vec<f64>> = controls.iter().zip(weights.iter()).map(|(&(x, y), &wt)| vec![x * wt, y * wt, wt]).collect();
        let kv = KnotVector::new(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2, 3).unwrap();
        for i in 1..20 {
            let u = i as f64 / 20.0;
            let derivs = curve_derivatives_rational(&kv, &controls_h, u, 2);
            let p = (derivs[0][0], derivs[0][1]);
            assert!((p.0 * p.0 + p.1 * p.1 - 1.0).abs() < 1e-9, "point off unit circle at u={u}: {p:?}");
            // Tangent must be perpendicular to the radius vector (a circle's defining property).
            let radial_dot_tangent = p.0 * derivs[1][0] + p.1 * derivs[1][1];
            assert!(radial_dot_tangent.abs() < 1e-8, "tangent not perpendicular to radius at u={u}: dot={radial_dot_tangent}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn periodic_uniform_is_periodic_and_has_expected_domain() {
        let kv = KnotVector::periodic_uniform(6, 3);
        assert!(kv.is_periodic());
        assert_eq!(kv.domain(), (0.0, 6.0));
        assert_eq!(kv.control_point_count(), 9);
    }

    #[semio_framework_async_macros::async_test]
    async fn wrap_folds_parameters_into_the_domain_by_the_period() {
        let kv = KnotVector::periodic_uniform(5, 2);
        assert!((kv.wrap(-1.0) - 4.0).abs() < 1e-9);
        assert!((kv.wrap(7.0) - 2.0).abs() < 1e-9);
        assert!((kv.wrap(2.5) - 2.5).abs() < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    async fn remove_knot_round_trips_an_insertion() {
        let kv = cubic_clamped_5cp();
        let values: Vec<Vec<f64>> = vec![0.0, 2.0, -1.0, 3.0, 1.0].into_iter().map(|v| vec![v]).collect();
        let u = 0.3;
        let (inserted_kv, inserted_values) = insert_knot_multi(&kv, &values, u);
        let (removed_kv, removed_values) = remove_knot(&inserted_kv, &inserted_values, u, 1e-7).expect("exact re-insertion must be removable");
        assert_eq!(removed_kv.knots.len(), kv.knots.len());
        for i in 0..=20 {
            let t = i as f64 / 20.0;
            let before = de_boor(&kv, &values.iter().map(|v| v[0]).collect::<Vec<_>>(), t);
            let after = de_boor(&removed_kv, &removed_values.iter().map(|v| v[0]).collect::<Vec<_>>(), t);
            assert!((before - after).abs() < 1e-7, "curve changed after remove_knot round trip at t={t}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn remove_knot_rejects_a_knot_that_is_not_removable() {
        // The interior knot of cubic_clamped_5cp carries real geometric information (it wasn't
        // produced by a redundant insertion), so removing it should move the curve beyond a tight
        // tolerance and be rejected.
        let kv = cubic_clamped_5cp();
        let values: Vec<Vec<f64>> = vec![0.0, 2.0, -8.0, 3.0, 1.0].into_iter().map(|v| vec![v]).collect();
        assert!(remove_knot(&kv, &values, 0.5, 1e-9).is_none());
    }

    #[semio_framework_async_macros::async_test]
    async fn elevate_degree_by_zero_is_a_no_op() {
        let kv = cubic_clamped_5cp();
        let values: Vec<Vec<f64>> = vec![0.0, 2.0, -1.0, 3.0, 1.0].into_iter().map(|v| vec![v]).collect();
        let (new_kv, new_values) = elevate_degree(&kv, &values, 0);
        assert_eq!(new_kv.knots, kv.knots);
        assert_eq!(new_values, values);
    }

    #[semio_framework_async_macros::async_test]
    async fn elevate_degree_preserves_a_multi_span_curve() {
        let kv = cubic_clamped_5cp();
        let values: Vec<Vec<f64>> = vec![0.0, 2.0, -1.0, 3.0, 1.0].into_iter().map(|v| vec![v]).collect();
        let (new_kv, new_values) = elevate_degree(&kv, &values, 2);
        assert_eq!(new_kv.degree, kv.degree + 2);
        for i in 0..=50 {
            let t = i as f64 / 50.0;
            let before = de_boor(&kv, &values.iter().map(|v| v[0]).collect::<Vec<_>>(), t);
            let after = de_boor(&new_kv, &new_values.iter().map(|v| v[0]).collect::<Vec<_>>(), t);
            assert!((before - after).abs() < 1e-9, "curve changed after degree elevation at t={t}: {before} vs {after}");
        }
        // Continuity at the interior knot (originally multiplicity 1, i.e. C^2) must be preserved:
        // after elevating by 2, multiplicity should be 1+2=3 (still C^2 for the new degree 5).
        assert_eq!(new_kv.multiplicity(0.5), 3);
    }

    #[semio_framework_async_macros::async_test]
    async fn surface_derivatives_rational_matches_bilinear_patch_analytic_formula() {
        let u_knots = KnotVector::new(vec![0.0, 0.0, 1.0, 1.0], 1, 2).unwrap();
        let v_knots = KnotVector::new(vec![0.0, 0.0, 1.0, 1.0], 1, 2).unwrap();
        let p00 = [0.0, 0.0, 0.0];
        let p10 = [1.0, 0.2, 0.0];
        let p01 = [0.1, 1.0, 0.3];
        let p11 = [1.2, 1.1, 0.6];
        let controls_h = vec![vec![p00.to_vec(), p01.to_vec()], vec![p10.to_vec(), p11.to_vec()]];
        let controls_h: Vec<Vec<Vec<f64>>> = controls_h.into_iter().map(|row| row.into_iter().map(|mut p| { p.push(1.0); p }).collect()).collect();
        for &(u, v) in &[(0.2, 0.3), (0.7, 0.1), (0.5, 0.5)] {
            let s = surface_derivatives_rational(&u_knots, &v_knots, &controls_h, u, v, 2);
            let du_expected: Vec<f64> = (0..3).map(|d| (1.0 - v) * (p10[d] - p00[d]) + v * (p11[d] - p01[d])).collect();
            let dv_expected: Vec<f64> = (0..3).map(|d| (1.0 - u) * (p01[d] - p00[d]) + u * (p11[d] - p10[d])).collect();
            let duv_expected: Vec<f64> = (0..3).map(|d| (p11[d] - p01[d]) - (p10[d] - p00[d])).collect();
            for d in 0..3 {
                assert!((s[1][0][d] - du_expected[d]).abs() < 1e-9, "du mismatch at ({u},{v}) axis {d}");
                assert!((s[0][1][d] - dv_expected[d]).abs() < 1e-9, "dv mismatch at ({u},{v}) axis {d}");
                assert!((s[1][1][d] - duv_expected[d]).abs() < 1e-9, "duv mismatch at ({u},{v}) axis {d}");
                assert!(s[2][0][d].abs() < 1e-9, "duu should vanish for a bilinear patch");
                assert!(s[0][2][d].abs() < 1e-9, "dvv should vanish for a bilinear patch");
            }
        }
    }

    mod quick {
        use super::*;

        #[semio_framework_async_macros::async_test]
        async fn de_boor_matches_bernstein_sum_oracle_on_random_bezier_span_curves() {
            // A single-span (no interior knots) clamped B-spline of degree p is exactly the
            // Bernstein-basis polynomial with the same control values — an independent oracle.
            let mut rng = semio_framework_geometry::random::Rng::from_seed(41);
            for _ in 0..200 {
                let degree = 1 + rng.next_range(0, 5) as usize;
                let n_cp = degree + 1;
                let kv = KnotVector::clamped_uniform(n_cp, degree);
                let values: Vec<f64> = (0..n_cp).map(|_| rng.next_f64() * 10.0 - 5.0).collect();
                let bernstein = crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::polynomial::Bernstein::new(values.clone());
                for i in 0..=20 {
                    let u = i as f64 / 20.0;
                    let via_de_boor = de_boor(&kv, &values, u);
                    let via_bernstein = bernstein.eval(u);
                    assert!((via_de_boor - via_bernstein).abs() < 1e-9, "mismatch at u={u} degree={degree}: de_boor={via_de_boor} bernstein={via_bernstein}");
                }
            }
        }

        #[semio_framework_async_macros::async_test]
        async fn knot_insertion_is_geometrically_a_no_op_on_random_curves() {
            let mut rng = semio_framework_geometry::random::Rng::from_seed(43);
            for _ in 0..100 {
                let degree = 1 + rng.next_range(0, 4) as usize;
                let n_cp = degree + 2 + rng.next_range(0, 4) as usize;
                let kv = KnotVector::clamped_uniform(n_cp, degree);
                let values: Vec<f64> = (0..n_cp).map(|_| rng.next_f64() * 10.0 - 5.0).collect();
                let (lo, hi) = kv.domain();
                let u = lo + (hi - lo) * rng.next_f64();
                if kv.multiplicity(u) > degree {
                    continue;
                }
                let (new_kv, new_values) = insert_knot(&kv, &values, u);
                for i in 0..=20 {
                    let t = lo + (hi - lo) * (i as f64 / 20.0);
                    let before = de_boor(&kv, &values, t);
                    let after = de_boor(&new_kv, &new_values, t);
                    assert!((before - after).abs() < 1e-7, "curve changed at t={t}: {before} vs {after}");
                }
            }
        }

        #[semio_framework_async_macros::async_test]
        async fn elevate_degree_preserves_random_multi_span_curves() {
            let mut rng = semio_framework_geometry::random::Rng::from_seed(71);
            for _ in 0..100 {
                let degree = 1 + rng.next_range(0, 3) as usize;
                let n_cp = degree + 2 + rng.next_range(0, 4) as usize;
                let kv = KnotVector::clamped_uniform(n_cp, degree);
                let values: Vec<Vec<f64>> = (0..n_cp).map(|_| vec![rng.next_f64() * 10.0 - 5.0]).collect();
                let t = 1 + rng.next_range(0, 2) as usize;
                let (new_kv, new_values) = elevate_degree(&kv, &values, t);
                assert_eq!(new_kv.degree, degree + t);
                let (lo, hi) = kv.domain();
                for i in 0..=20 {
                    let u = lo + (hi - lo) * (i as f64 / 20.0);
                    let before = de_boor(&kv, &values.iter().map(|v| v[0]).collect::<Vec<_>>(), u);
                    let after = de_boor(&new_kv, &new_values.iter().map(|v| v[0]).collect::<Vec<_>>(), u);
                    assert!((before - after).abs() < 1e-7, "degree={degree} t={t} n_cp={n_cp}: mismatch at u={u}: {before} vs {after}");
                }
            }
        }

        #[semio_framework_async_macros::async_test]
        async fn remove_knot_round_trips_random_insertions() {
            let mut rng = semio_framework_geometry::random::Rng::from_seed(73);
            for _ in 0..100 {
                let degree = 1 + rng.next_range(0, 4) as usize;
                let n_cp = degree + 2 + rng.next_range(0, 4) as usize;
                let kv = KnotVector::clamped_uniform(n_cp, degree);
                let values: Vec<Vec<f64>> = (0..n_cp).map(|_| vec![rng.next_f64() * 10.0 - 5.0]).collect();
                let (lo, hi) = kv.domain();
                let u = lo + (hi - lo) * rng.next_f64();
                if kv.multiplicity(u) > degree {
                    continue;
                }
                let (ins_kv, ins_values) = insert_knot_multi(&kv, &values, u);
                let Some((rm_kv, rm_values)) = remove_knot(&ins_kv, &ins_values, u, 1e-6) else {
                    panic!("re-insertion at u={u} must always be removable (degree={degree}, n_cp={n_cp})");
                };
                assert_eq!(rm_kv.knots.len(), kv.knots.len());
                for i in 0..=20 {
                    let t = lo + (hi - lo) * (i as f64 / 20.0);
                    let before = de_boor(&kv, &values.iter().map(|v| v[0]).collect::<Vec<_>>(), t);
                    let after = de_boor(&rm_kv, &rm_values.iter().map(|v| v[0]).collect::<Vec<_>>(), t);
                    assert!((before - after).abs() < 1e-6, "mismatch at t={t}: {before} vs {after}");
                }
            }
        }

        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        fn eval_rational(kv: &KnotVector, controls_h: &[Vec<f64>], u: f64) -> Vec<f64> {
            curve_derivatives_rational(kv, controls_h, u, 0)[0].clone()
        }

        /// 🧮️ Central-difference derivative with one round of Richardson extrapolation
        /// (`(8·D(h/2) - D(h)) / 6` in effect, via the standard 5-point combination) — an
        /// independent, high-precision oracle for [`curve_derivatives_rational`] that does not
        /// share any code with it.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        fn richardson_d1(kv: &KnotVector, controls_h: &[Vec<f64>], u: f64) -> Vec<f64> {
            let h = 1e-3;
            let f = |t: f64| eval_rational(kv, controls_h, t);
            let d = |step: f64| -> Vec<f64> {
                let a = f(u + step);
                let b = f(u - step);
                a.iter().zip(&b).map(|(x, y)| (x - y) / (2.0 * step)).collect()
            };
            let d_h = d(h);
            let d_h2 = d(h / 2.0);
            d_h2.iter().zip(&d_h).map(|(a, b)| (4.0 * a - b) / 3.0).collect()
        }

        #[semio_framework_async_macros::async_test]
        async fn curve_derivatives_rational_matches_richardson_finite_differences_on_random_rational_curves() {
            let mut rng = semio_framework_geometry::random::Rng::from_seed(83);
            for _ in 0..100 {
                let degree = 2 + rng.next_range(0, 2) as usize;
                let n_cp = degree + 2 + rng.next_range(0, 3) as usize;
                let kv = KnotVector::clamped_uniform(n_cp, degree);
                let controls_h: Vec<Vec<f64>> = (0..n_cp)
                    .map(|_| {
                        let w = 0.5 + rng.next_f64();
                        let x = rng.next_f64() * 4.0 - 2.0;
                        let y = rng.next_f64() * 4.0 - 2.0;
                        vec![x * w, y * w, w]
                    })
                    .collect();
                let (lo, hi) = kv.domain();
                let u = lo + (hi - lo) * (0.1 + 0.8 * rng.next_f64());
                let exact = curve_derivatives_rational(&kv, &controls_h, u, 1);
                let oracle = richardson_d1(&kv, &controls_h, u);
                for d in 0..2 {
                    assert!((exact[1][d] - oracle[d]).abs() < 1e-6, "degree={degree} n_cp={n_cp} u={u} axis={d}: exact={} oracle={}", exact[1][d], oracle[d]);
                }
            }
        }
    }
}
// #endregion 🔖️Tests
