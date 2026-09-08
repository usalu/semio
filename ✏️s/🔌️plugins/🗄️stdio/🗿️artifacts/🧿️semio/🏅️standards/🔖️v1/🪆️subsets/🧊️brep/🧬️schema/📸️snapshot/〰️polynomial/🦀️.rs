//! ∿ Univariate polynomials in monomial and Bernstein form, closed-form low-degree solvers, and a
//! certified general root isolator (Bernstein sign-variation subdivision + safeguarded Newton).
//! The Bernstein form is the workhorse for [`crate::standards::v1::subsets::brep::schema::snapshot::curve::bezier`] and [`crate::standards::v1::subsets::brep::schema::snapshot::curve::bspline`]: its control
//! polygon convex-hulls the curve, so a control-polygon sign change is a *necessary* condition for
//! a root, which is exactly what Descartes' rule of signs turns into a certified root count.
//!
//! Moved from `🧰️framework/🔨️modules/🧊️3d/📐️brep/〰️polynomial` in ticket
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave PEEL4.

// #region 🔖️Poly

/// ∿ A polynomial in monomial basis: `coeffs[i]` is the coefficient of `x^i`.
#[derive(Clone, Debug, PartialEq)]
pub struct Poly {
    pub coeffs: Vec<f64>,
}

impl Poly {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new(coeffs: Vec<f64>) -> Self {
        Poly { coeffs }
    }
    /// ∿ Degree of the polynomial after trimming trailing (highest-order) exact zeros; a
    /// constant zero polynomial has degree `0`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn degree(&self) -> usize {
        let mut d = self.coeffs.len().saturating_sub(1);
        while d > 0 && self.coeffs[d] == 0.0 {
            d -= 1;
        }
        d
    }
    /// ∿ Horner evaluation.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn eval(&self, x: f64) -> f64 {
        self.coeffs.iter().rev().fold(0.0, |acc, &c| acc * x + c)
    }
    /// ∿ Simultaneous Horner evaluation of the polynomial and its derivative (one pass, no
    /// separate `derivative()` allocation on the hot Newton-iteration path).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn eval_with_derivative(&self, x: f64) -> (f64, f64) {
        let mut value = 0.0;
        let mut deriv = 0.0;
        for &c in self.coeffs.iter().rev() {
            deriv = deriv * x + value;
            value = value * x + c;
        }
        (value, deriv)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn derivative(&self) -> Poly {
        if self.coeffs.len() <= 1 {
            return Poly::new(vec![0.0]);
        }
        Poly::new(self.coeffs.iter().enumerate().skip(1).map(|(i, &c)| c * i as f64).collect())
    }
}

// #endregion 🔖️Poly

// #region 🔖️ClosedForm

/// ∿ Real roots of `a·x² + b·x + c`, using the cancellation-safe form (`q = -½(b + sign(b)·√Δ)`,
/// roots `q/a` and `c/q`) rather than the naive quadratic formula.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn solve_quadratic(a: f64, b: f64, c: f64) -> Vec<f64> {
    if a == 0.0 {
        return if b == 0.0 { vec![] } else { vec![-c / b] };
    }
    let disc = b * b - 4.0 * a * c;
    if disc < 0.0 {
        return vec![];
    }
    if disc == 0.0 {
        return vec![-b / (2.0 * a)];
    }
    let sqrt_disc = disc.sqrt();
    let sign = if b >= 0.0 { 1.0 } else { -1.0 };
    let q = -0.5 * (b + sign * sqrt_disc);
    let mut roots = vec![q / a, c / q];
    roots.sort_by(|x, y| x.partial_cmp(y).unwrap());
    roots
}

/// ∿ Real roots of `a·x³ + b·x² + c·x + d` (`a ≠ 0`) via the depressed-cubic trigonometric method
/// for three real roots and Cardano's formula otherwise.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn solve_cubic(a: f64, b: f64, c: f64, d: f64) -> Vec<f64> {
    if a == 0.0 {
        return solve_quadratic(b, c, d);
    }
    let (b, c, d) = (b / a, c / a, d / a);
    let shift = b / 3.0;
    let p = c - b * b / 3.0;
    let q = 2.0 * b * b * b / 27.0 - b * c / 3.0 + d;
    let mut roots = if p.abs() < 1e-14 && q.abs() < 1e-14 {
        vec![0.0]
    } else {
        let discriminant = (q / 2.0).powi(2) + (p / 3.0).powi(3);
        if discriminant > 0.0 {
            let sqrt_disc = discriminant.sqrt();
            let u = cbrt(-q / 2.0 + sqrt_disc);
            let v = cbrt(-q / 2.0 - sqrt_disc);
            vec![u + v]
        } else if p.abs() < 1e-300 {
            vec![cbrt(-q)]
        } else {
            let r = (-p / 3.0).sqrt();
            let cos_arg = (3.0 * q / (2.0 * p * r)).clamp(-1.0, 1.0);
            let theta = cos_arg.acos();
            (0..3).map(|k| 2.0 * r * ((theta - std::f64::consts::TAU * k as f64) / 3.0).cos()).collect()
        }
    };
    for r in roots.iter_mut() {
        *r -= shift;
    }
    roots.sort_by(|x, y| x.partial_cmp(y).unwrap());
    roots
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn cbrt(x: f64) -> f64 {
    x.signum() * x.abs().powf(1.0 / 3.0)
}

// #endregion 🔖️ClosedForm

// #region 🔖️Bernstein

/// ∿ A polynomial in Bernstein basis on `[0, 1]`: `coeffs[i]` is the `i`-th control ordinate
/// `b_i` in `Σ b_i · C(n,i) · t^i · (1-t)^(n-i)`. The control polygon (the piecewise-linear
/// interpolant of `coeffs` at parameters `i/n`) convex-hulls the curve — the geometric fact
/// [`sign_variations`] exploits.
#[derive(Clone, Debug, PartialEq)]
pub struct Bernstein {
    pub coeffs: Vec<f64>,
}

impl Bernstein {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new(coeffs: Vec<f64>) -> Self {
        Bernstein { coeffs }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn degree(&self) -> usize {
        self.coeffs.len().saturating_sub(1)
    }
    /// ∿ De Casteljau evaluation at `t` (need not lie in `[0, 1]`; the polynomial extends).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn eval(&self, t: f64) -> f64 {
        let mut work = self.coeffs.clone();
        let n = work.len();
        for level in 1..n {
            for i in 0..n - level {
                work[i] = work[i] * (1.0 - t) + work[i + 1] * t;
            }
        }
        work.first().copied().unwrap_or(0.0)
    }
    /// ∿ De Casteljau subdivision at `t`: returns the control points of the restriction to
    /// `[0, t]` and to `[t, 1]`, each reparameterized back onto `[0, 1]`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn subdivide(&self, t: f64) -> (Bernstein, Bernstein) {
        let n = self.coeffs.len();
        let mut table = vec![self.coeffs.clone()];
        for level in 1..n {
            let prev = &table[level - 1];
            let next: Vec<f64> = (0..n - level).map(|i| prev[i] * (1.0 - t) + prev[i + 1] * t).collect();
            table.push(next);
        }
        let left: Vec<f64> = (0..n).map(|i| table[i][0]).collect();
        let right: Vec<f64> = (0..n).map(|i| table[n - 1 - i][i]).collect();
        (Bernstein::new(left), Bernstein::new(right))
    }
    /// ∿ Converts to monomial (power) basis via repeated finite differences of the control net:
    /// `coeff[k] = C(n,k) · Δ^k b_0`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_monomial(&self) -> Poly {
        let n = self.degree();
        let mut diffs = self.coeffs.clone();
        let mut monomial = vec![0.0; n + 1];
        monomial[0] = diffs[0];
        #[allow(clippy::needless_range_loop)]
        for k in 1..=n {
            for i in 0..diffs.len() - 1 {
                diffs[i] = diffs[i + 1] - diffs[i];
            }
            diffs.truncate(diffs.len() - 1);
            monomial[k] = binomial(n, k) * diffs[0];
        }
        Poly::new(monomial)
    }
    /// ∿ Converts a monomial polynomial to Bernstein form on `[0, 1]` (inverse of [`Self::to_monomial`]).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_monomial(p: &Poly) -> Bernstein {
        let n = p.degree();
        let coeffs = (0..=n).map(|i| (0..=i).map(|j| p.coeffs.get(j).copied().unwrap_or(0.0) * binomial(i, j) / binomial(n, j)).sum::<f64>()).collect();
        Bernstein::new(coeffs)
    }
    /// ∿ Descartes' rule of signs applied to the control polygon: the number of sign changes in
    /// `coeffs` (ignoring exact zeros) is an upper bound on, and has the same parity as, the
    /// number of real roots in `(0, 1)`. `0` sign changes certifies *no* root; `1` certifies
    /// *exactly one*.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn sign_variations(&self) -> usize {
        let nonzero: Vec<f64> = self.coeffs.iter().copied().filter(|c| *c != 0.0).collect();
        nonzero.windows(2).filter(|w| w[0].signum() != w[1].signum()).count()
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn binomial(n: usize, k: usize) -> f64 {
    if k > n {
        return 0.0;
    }
    let k = k.min(n - k);
    (0..k).fold(1.0, |acc, i| acc * (n - i) as f64 / (i + 1) as f64)
}

// #endregion 🔖️Bernstein

// #region 🔖️Isolation

/// ∿ Recursively subdivides `b` over `[0, 1]` until every sub-interval has `0` or `1` sign
/// variation (certified root-free or root-isolating), returning the isolating `(lo, hi)`
/// intervals in increasing order. `max_depth` bounds recursion for pathological clustered-root
/// inputs — see [`crate::standards::v1::subsets::brep::schema::snapshot::error`] for how callers should react if isolation is incomplete
/// (the kernel's "never wrong, fail loud" invariant: a caller hitting `max_depth` should treat
/// the sub-interval as unresolved rather than guess).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn isolate_roots(b: &Bernstein, max_depth: u32) -> Vec<(f64, f64)> {
    let mut intervals = Vec::new();
    isolate_recursive(b, 0.0, 1.0, max_depth, &mut intervals);
    intervals
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn isolate_recursive(b: &Bernstein, lo: f64, hi: f64, depth: u32, out: &mut Vec<(f64, f64)>) {
    let variations = b.sign_variations();
    if variations == 0 {
        return;
    }
    if variations == 1 || depth == 0 {
        out.push((lo, hi));
        return;
    }
    let mid = 0.5;
    let (left, right) = b.subdivide(mid);
    let mid_param = lo + (hi - lo) * mid;
    isolate_recursive(&left, lo, mid_param, depth - 1, out);
    isolate_recursive(&right, mid_param, hi, depth - 1, out);
}

// #endregion 🔖️Isolation

// #region 🔖️Refine

/// ∿ Safeguarded Newton (bisection fallback whenever a Newton step would leave the bracket or
/// fails to shrink it) — guaranteed to converge given a valid sign-changing bracket `[lo, hi]`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn refine_root(p: &Poly, mut lo: f64, mut hi: f64, tol: f64, max_iters: u32) -> f64 {
    let mut f_lo = p.eval(lo);
    let f_hi = p.eval(hi);
    if f_lo == 0.0 {
        return lo;
    }
    if f_hi == 0.0 {
        return hi;
    }
    debug_assert!(f_lo.signum() != f_hi.signum(), "refine_root requires a sign-changing bracket");
    let mut x = 0.5 * (lo + hi);
    for _ in 0..max_iters {
        let (fx, dfx) = p.eval_with_derivative(x);
        if fx.abs() <= tol {
            return x;
        }
        if fx.signum() == f_lo.signum() {
            lo = x;
            f_lo = fx;
        } else {
            hi = x;
        }
        let newton_step = if dfx.abs() > 1e-300 { x - fx / dfx } else { f64::NAN };
        x = if newton_step.is_finite() && newton_step > lo && newton_step < hi { newton_step } else { 0.5 * (lo + hi) };
        if (hi - lo).abs() < tol {
            return x;
        }
    }
    x
}

// #endregion 🔖️Refine

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
