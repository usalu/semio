//! 🧵️ Knot vectors, B-spline basis functions and de Boor evaluation for rational curves and
//! tensor-product surfaces — the machinery [`crate::brep::curve::Curve3::Nurbs`] and
//! [`crate::brep::surface::Surface::Nurbs`] are built on. Curves and surfaces themselves stay in their
//! own modules; this file is purely the numerical core, independent of any particular dimension.

// #region 🔖️Knots

/// 🧵️ A non-decreasing knot vector for a degree-`p` B-spline with `n` control points, satisfying
/// `len == n + p + 1`.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct KnotVector {
    pub knots: Vec<f64>,
    pub degree: usize,
}

impl KnotVector {
    /// 🧵️ Builds and validates a knot vector: non-decreasing, correct length for `(n, degree)`.
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
    pub fn domain(&self) -> (f64, f64) {
        (self.knots[self.degree], self.knots[self.knots.len() - self.degree - 1])
    }
    pub fn control_point_count(&self) -> usize {
        self.knots.len() - self.degree - 1
    }
    pub fn is_periodic_compatible(&self) -> bool {
        // A periodic (non-clamped) knot vector has no repeated end knots beyond multiplicity 1.
        self.multiplicity_at_index(0) == 1
    }
    fn multiplicity_at_index(&self, i: usize) -> usize {
        self.knots.iter().filter(|&&k| k == self.knots[i]).count()
    }
    /// 🧵️ Finds the knot span index `i` such that `knots[i] <= u < knots[i+1]` (or the last valid
    /// span if `u` equals the domain's upper bound), via binary search — O(log n) per evaluation.
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
    pub fn multiplicity(&self, u: f64) -> usize {
        self.knots.iter().filter(|&&k| k == u).count()
    }
}

// #endregion 🔖️Knots

// #region 🔖️Basis

/// 🧵️ Evaluates all `degree+1` nonzero basis functions at `u` in the knot span `span` (the
/// Cox-de Boor triangular recurrence, computed bottom-up per the standard NURBS-book algorithm —
/// `O(p²)` and numerically stable, unlike the naive top-down recursive definition).
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

/// 🧵️ Elevates a Bézier segment's degree by one via the shared [`crate::brep::bezier`] elevation
/// formula, exposed here so B-spline code can raise a single-span curve's degree without
/// round-tripping through the `Bernstein`/`Poly` types.
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

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn cubic_clamped_5cp() -> KnotVector {
        // degree 3, 5 control points -> knot vector length 9: [0,0,0,0, 0.5, 1,1,1,1]
        KnotVector::new(vec![0.0, 0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0, 1.0], 3, 5).unwrap()
    }

    #[test]
    fn knot_vector_rejects_wrong_length() {
        assert!(KnotVector::new(vec![0.0, 0.0, 1.0, 1.0], 3, 5).is_none());
    }

    #[test]
    fn knot_vector_rejects_decreasing_sequence() {
        assert!(KnotVector::new(vec![0.0, 0.5, 0.2, 1.0, 1.0], 1, 3).is_none());
    }

    #[test]
    fn clamped_uniform_has_correct_domain_and_multiplicity() {
        let kv = KnotVector::clamped_uniform(5, 3);
        assert_eq!(kv.domain(), (0.0, 1.0));
        assert_eq!(kv.multiplicity(0.0), 4);
        assert_eq!(kv.multiplicity(1.0), 4);
        assert_eq!(kv.control_point_count(), 5);
    }

    #[test]
    fn find_span_matches_brute_force_scan() {
        let kv = cubic_clamped_5cp();
        for i in 0..=100 {
            let u = i as f64 / 100.0;
            let expected = brute_force_span(&kv, u);
            assert_eq!(kv.find_span(u), expected, "mismatch at u={u}");
        }
    }

    fn brute_force_span(kv: &KnotVector, u: f64) -> usize {
        let n = kv.control_point_count() - 1;
        for i in kv.degree..=n {
            if u >= kv.knots[i] && u < kv.knots[i + 1] {
                return i;
            }
        }
        n
    }

    #[test]
    fn basis_functions_sum_to_one_everywhere_in_domain() {
        let kv = cubic_clamped_5cp();
        for i in 0..=50 {
            let u = i as f64 / 50.0;
            let span = kv.find_span(u);
            let n = basis_functions(&kv, span, u);
            let sum: f64 = n.iter().sum();
            assert!((sum - 1.0).abs() < 1e-10, "partition of unity violated at u={u}: sum={sum}");
        }
    }

    #[test]
    fn basis_functions_are_nonnegative() {
        let kv = cubic_clamped_5cp();
        for i in 0..=50 {
            let u = i as f64 / 50.0;
            let span = kv.find_span(u);
            let n = basis_functions(&kv, span, u);
            assert!(n.iter().all(|&v| v >= -1e-12), "negative basis value at u={u}: {n:?}");
        }
    }

    #[test]
    fn de_boor_interpolates_endpoints_of_clamped_curve() {
        let kv = cubic_clamped_5cp();
        let values = vec![0.0, 1.0, -2.0, 3.0, 5.0];
        let (lo, hi) = kv.domain();
        assert!((de_boor(&kv, &values, lo) - values[0]).abs() < 1e-9);
        assert!((de_boor(&kv, &values, hi) - *values.last().unwrap()).abs() < 1e-9);
    }

    #[test]
    fn basis_function_derivatives_match_finite_differences() {
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

    #[test]
    fn basis_function_derivatives_order_zero_matches_basis_functions() {
        let kv = cubic_clamped_5cp();
        let u = 0.63;
        let span = kv.find_span(u);
        let plain = basis_functions(&kv, span, u);
        let derivs = basis_function_derivatives(&kv, span, u, 2);
        for j in 0..=kv.degree {
            assert!((plain[j] - derivs[0][j]).abs() < 1e-12);
        }
    }

    #[test]
    fn insert_knot_does_not_change_the_curve() {
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

    #[test]
    fn elevate_bezier_span_preserves_curve_value() {
        // Single bezier span is a B-spline with degree = n and a clamped, no-interior-knot vector.
        let control_values = vec![0.0, 3.0, -2.0, 5.0];
        let elevated = elevate_bezier_span(&control_values);
        assert_eq!(elevated.len(), control_values.len() + 1);
        let b = crate::brep::bezier::RationalBezier2::unweighted(control_values.iter().map(|&v| crate::brep::vec::Pnt2::new(v, 0.0)).collect());
        let be = crate::brep::bezier::RationalBezier2::unweighted(elevated.iter().map(|&v| crate::brep::vec::Pnt2::new(v, 0.0)).collect());
        for i in 0..=10 {
            let t = i as f64 / 10.0;
            assert!((b.eval(t).x - be.eval(t).x).abs() < 1e-9);
        }
    }

    mod quick {
        use super::*;

        #[test]
        fn de_boor_matches_bernstein_sum_oracle_on_random_bezier_span_curves() {
            // A single-span (no interior knots) clamped B-spline of degree p is exactly the
            // Bernstein-basis polynomial with the same control values — an independent oracle.
            let mut rng = semio_framework_geometry::random::Rng::from_seed(41);
            for _ in 0..200 {
                let degree = 1 + rng.next_range(0, 5) as usize;
                let n_cp = degree + 1;
                let kv = KnotVector::clamped_uniform(n_cp, degree);
                let values: Vec<f64> = (0..n_cp).map(|_| rng.next_f64() * 10.0 - 5.0).collect();
                let bernstein = crate::brep::poly::Bernstein::new(values.clone());
                for i in 0..=20 {
                    let u = i as f64 / 20.0;
                    let via_de_boor = de_boor(&kv, &values, u);
                    let via_bernstein = bernstein.eval(u);
                    assert!((via_de_boor - via_bernstein).abs() < 1e-9, "mismatch at u={u} degree={degree}: de_boor={via_de_boor} bernstein={via_bernstein}");
                }
            }
        }

        #[test]
        fn knot_insertion_is_geometrically_a_no_op_on_random_curves() {
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
    }
}
// #endregion 🔖️Tests
