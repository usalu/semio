//! ∿ Univariate polynomials in monomial and Bernstein form, closed-form low-degree solvers, and a
//! certified general root isolator (Bernstein sign-variation subdivision + safeguarded Newton).
//! The Bernstein form is the workhorse for [`crate::brep::bezier`] and [`crate::brep::bspline`]: its control
//! polygon convex-hulls the curve, so a control-polygon sign change is a *necessary* condition for
//! a root, which is exactly what Descartes' rule of signs turns into a certified root count.

// #region 🔖️Poly

/// ∿ A polynomial in monomial basis: `coeffs[i]` is the coefficient of `x^i`.
#[derive(Clone, Debug, PartialEq)]
pub struct Poly {
    pub coeffs: Vec<f64>,
}

impl Poly {
    pub fn new(coeffs: Vec<f64>) -> Self {
        Poly { coeffs }
    }
    /// ∿ Degree of the polynomial after trimming trailing (highest-order) exact zeros; a
    /// constant zero polynomial has degree `0`.
    pub fn degree(&self) -> usize {
        let mut d = self.coeffs.len().saturating_sub(1);
        while d > 0 && self.coeffs[d] == 0.0 {
            d -= 1;
        }
        d
    }
    /// ∿ Horner evaluation.
    pub fn eval(&self, x: f64) -> f64 {
        self.coeffs.iter().rev().fold(0.0, |acc, &c| acc * x + c)
    }
    /// ∿ Simultaneous Horner evaluation of the polynomial and its derivative (one pass, no
    /// separate `derivative()` allocation on the hot Newton-iteration path).
    pub fn eval_with_derivative(&self, x: f64) -> (f64, f64) {
        let mut value = 0.0;
        let mut deriv = 0.0;
        for &c in self.coeffs.iter().rev() {
            deriv = deriv * x + value;
            value = value * x + c;
        }
        (value, deriv)
    }
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
    pub fn new(coeffs: Vec<f64>) -> Self {
        Bernstein { coeffs }
    }
    pub fn degree(&self) -> usize {
        self.coeffs.len().saturating_sub(1)
    }
    /// ∿ De Casteljau evaluation at `t` (need not lie in `[0, 1]`; the polynomial extends).
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
    pub fn from_monomial(p: &Poly) -> Bernstein {
        let n = p.degree();
        let coeffs = (0..=n).map(|i| (0..=i).map(|j| p.coeffs.get(j).copied().unwrap_or(0.0) * binomial(i, j) / binomial(n, j)).sum::<f64>()).collect();
        Bernstein::new(coeffs)
    }
    /// ∿ Descartes' rule of signs applied to the control polygon: the number of sign changes in
    /// `coeffs` (ignoring exact zeros) is an upper bound on, and has the same parity as, the
    /// number of real roots in `(0, 1)`. `0` sign changes certifies *no* root; `1` certifies
    /// *exactly one*.
    pub fn sign_variations(&self) -> usize {
        let nonzero: Vec<f64> = self.coeffs.iter().copied().filter(|c| *c != 0.0).collect();
        nonzero.windows(2).filter(|w| w[0].signum() != w[1].signum()).count()
    }
}

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
/// inputs — see [`crate::brep::error`] for how callers should react if isolation is incomplete
/// (the kernel's "never wrong, fail loud" invariant: a caller hitting `max_depth` should treat
/// the sub-interval as unresolved rather than guess).
pub fn isolate_roots(b: &Bernstein, max_depth: u32) -> Vec<(f64, f64)> {
    let mut intervals = Vec::new();
    isolate_recursive(b, 0.0, 1.0, max_depth, &mut intervals);
    intervals
}

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
mod tests {
    use super::*;

    #[test]
    fn poly_eval_matches_direct_computation() {
        let p = Poly::new(vec![1.0, -2.0, 3.0]); // 1 - 2x + 3x^2
        assert!((p.eval(2.0) - (1.0 - 4.0 + 12.0)).abs() < 1e-12);
    }

    #[test]
    fn poly_derivative_matches_power_rule() {
        let p = Poly::new(vec![1.0, -2.0, 3.0, 4.0]); // 1 - 2x + 3x^2 + 4x^3
        let d = p.derivative();
        assert_eq!(d.coeffs, vec![-2.0, 6.0, 12.0]);
    }

    #[test]
    fn eval_with_derivative_matches_separate_calls() {
        let p = Poly::new(vec![2.0, -1.0, 0.5, 3.0]);
        let (v, dv) = p.eval_with_derivative(1.5);
        assert!((v - p.eval(1.5)).abs() < 1e-12);
        assert!((dv - p.derivative().eval(1.5)).abs() < 1e-12);
    }

    #[test]
    fn solve_quadratic_finds_known_roots() {
        // (x-2)(x-3) = x^2 -5x+6
        let roots = solve_quadratic(1.0, -5.0, 6.0);
        assert_eq!(roots.len(), 2);
        assert!((roots[0] - 2.0).abs() < 1e-9);
        assert!((roots[1] - 3.0).abs() < 1e-9);
    }

    #[test]
    fn solve_quadratic_handles_no_real_roots() {
        assert!(solve_quadratic(1.0, 0.0, 1.0).is_empty());
    }

    #[test]
    fn solve_quadratic_avoids_cancellation_for_large_b() {
        // Classic near-cancellation case: a=1, b=1e8, c=1. Naive formula loses precision.
        let roots = solve_quadratic(1.0, 1e8, 1.0);
        assert_eq!(roots.len(), 2);
        for r in &roots {
            let p = Poly::new(vec![1.0, 1e8, 1.0]);
            assert!(p.eval(*r).abs() / (1e8 * r.abs() + 1.0) < 1e-6, "root {r} not accurate enough");
        }
    }

    #[test]
    fn solve_cubic_finds_three_known_real_roots() {
        // (x-1)(x-2)(x-3) = x^3 -6x^2+11x-6
        let mut roots = solve_cubic(1.0, -6.0, 11.0, -6.0);
        roots.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(roots.len(), 3);
        assert!((roots[0] - 1.0).abs() < 1e-9);
        assert!((roots[1] - 2.0).abs() < 1e-9);
        assert!((roots[2] - 3.0).abs() < 1e-9);
    }

    #[test]
    fn solve_cubic_finds_single_real_root() {
        // x^3 + x + 1 has exactly one real root (~-0.6823)
        let roots = solve_cubic(1.0, 0.0, 1.0, 1.0);
        assert_eq!(roots.len(), 1);
        let p = Poly::new(vec![1.0, 1.0, 0.0, 1.0]);
        assert!(p.eval(roots[0]).abs() < 1e-9);
    }

    #[test]
    fn bernstein_eval_matches_monomial_conversion() {
        let p = Poly::new(vec![1.0, 2.0, -3.0, 0.5]);
        let b = Bernstein::from_monomial(&p);
        for i in 0..=10 {
            let t = i as f64 / 10.0;
            assert!((b.eval(t) - p.eval(t)).abs() < 1e-9, "mismatch at t={t}");
        }
    }

    #[test]
    fn bernstein_to_monomial_round_trips_from_monomial() {
        let p = Poly::new(vec![3.0, -1.5, 2.0, 4.0, -0.25]);
        let b = Bernstein::from_monomial(&p);
        let back = b.to_monomial();
        assert_eq!(back.coeffs.len(), p.coeffs.len());
        for (a, c) in back.coeffs.iter().zip(p.coeffs.iter()) {
            assert!((a - c).abs() < 1e-8, "coefficient mismatch: {a} vs {c}");
        }
    }

    #[test]
    fn bernstein_subdivide_matches_original_at_shared_endpoints_and_split_point() {
        let b = Bernstein::new(vec![0.0, 3.0, -1.0, 2.0]);
        let t = 0.4;
        let (left, right) = b.subdivide(t);
        assert!((left.eval(0.0) - b.eval(0.0)).abs() < 1e-9);
        assert!((left.eval(1.0) - b.eval(t)).abs() < 1e-9);
        assert!((right.eval(0.0) - b.eval(t)).abs() < 1e-9);
        assert!((right.eval(1.0) - b.eval(1.0)).abs() < 1e-9);
        // Sample a mid-point of the left piece and confirm it agrees with the original curve.
        assert!((left.eval(0.5) - b.eval(t * 0.5)).abs() < 1e-9);
    }

    #[test]
    fn sign_variations_certifies_no_root_for_monotone_positive_control_polygon() {
        let b = Bernstein::new(vec![1.0, 2.0, 3.0, 4.0]);
        assert_eq!(b.sign_variations(), 0);
    }

    #[test]
    fn sign_variations_detects_single_sign_change() {
        let b = Bernstein::new(vec![-1.0, -0.5, 1.0, 2.0]);
        assert_eq!(b.sign_variations(), 1);
    }

    #[test]
    fn isolate_roots_finds_single_root_of_linear_bernstein() {
        // Line from -1 at t=0 to 1 at t=1: root at t=0.5.
        let b = Bernstein::new(vec![-1.0, 1.0]);
        let intervals = isolate_roots(&b, 20);
        assert_eq!(intervals.len(), 1);
        assert!(intervals[0].0 <= 0.5 && intervals[0].1 >= 0.5);
    }

    #[test]
    fn isolate_roots_finds_no_intervals_for_root_free_polynomial() {
        let b = Bernstein::new(vec![1.0, 2.0, 3.0]);
        assert!(isolate_roots(&b, 20).is_empty());
    }

    #[test]
    fn refine_root_converges_to_known_root() {
        let p = Poly::new(vec![-6.0, 11.0, -6.0, 1.0]); // (x-1)(x-2)(x-3)
        let root = refine_root(&p, 2.5, 3.5, 1e-12, 100);
        assert!((root - 3.0).abs() < 1e-9);
    }

    mod quick {
        use super::*;

        /// 🔮️ Brute-force oracle: dense sampling + bisection finds every sign-change interval,
        /// independent of the Bernstein/Descartes machinery under test.
        fn bisection_oracle(p: &Poly, samples: usize) -> Vec<f64> {
            let mut roots = Vec::new();
            let xs: Vec<f64> = (0..=samples).map(|i| i as f64 / samples as f64).collect();
            for w in xs.windows(2) {
                let (a, b) = (w[0], w[1]);
                let (fa, fb) = (p.eval(a), p.eval(b));
                if fa == 0.0 {
                    roots.push(a);
                } else if fa.signum() != fb.signum() {
                    roots.push(refine_root(p, a, b, 1e-12, 100));
                }
            }
            roots
        }

        #[test]
        fn isolate_roots_plus_refine_matches_bisection_oracle_on_random_polynomials() {
            let mut rng = mathematical_random::Rng::from_seed(23);
            for _ in 0..200 {
                let degree = 1 + (rng.next_range(0, 4) as usize);
                let coeffs: Vec<f64> = (0..=degree).map(|_| rng.next_f64() * 10.0 - 5.0).collect();
                let p = Poly::new(coeffs);
                if p.coeffs[p.degree()] == 0.0 {
                    continue;
                }
                let b = Bernstein::from_monomial(&p);
                let intervals = isolate_roots(&b, 30);
                let mut found: Vec<f64> = intervals.iter().map(|(lo, hi)| refine_root(&p, *lo, *hi, 1e-11, 100)).collect();
                found.sort_by(|a, c| a.partial_cmp(c).unwrap());
                let expected = bisection_oracle(&p, 4000);
                assert_eq!(found.len(), expected.len(), "root count mismatch for {:?}: found {found:?} expected {expected:?}", p.coeffs);
                for (f, e) in found.iter().zip(expected.iter()) {
                    assert!((f - e).abs() < 1e-6, "root mismatch: found {f} expected {e} for {:?}", p.coeffs);
                }
            }
        }

        #[test]
        fn bernstein_monomial_round_trip_holds_on_random_polynomials() {
            let mut rng = mathematical_random::Rng::from_seed(29);
            for _ in 0..200 {
                let degree = rng.next_range(0, 6) as usize;
                let coeffs: Vec<f64> = (0..=degree).map(|_| rng.next_f64() * 10.0 - 5.0).collect();
                let p = Poly::new(coeffs);
                let b = Bernstein::from_monomial(&p);
                let back = b.to_monomial();
                for (a, c) in back.coeffs.iter().zip(p.coeffs.iter()) {
                    assert!((a - c).abs() < 1e-6, "round trip mismatch: {a} vs {c}");
                }
            }
        }
    }
}
// #endregion 🔖️Tests
