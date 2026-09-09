use super::*;

#[semio_framework_async_macros::async_test]
async fn poly_eval_matches_direct_computation() {
    let p = Poly::new(vec![1.0, -2.0, 3.0]); // 1 - 2x + 3x^2
    assert!((p.eval(2.0) - (1.0 - 4.0 + 12.0)).abs() < 1e-12);
}

#[semio_framework_async_macros::async_test]
async fn poly_derivative_matches_power_rule() {
    let p = Poly::new(vec![1.0, -2.0, 3.0, 4.0]); // 1 - 2x + 3x^2 + 4x^3
    let d = p.derivative();
    assert_eq!(d.coeffs, vec![-2.0, 6.0, 12.0]);
}

#[semio_framework_async_macros::async_test]
async fn eval_with_derivative_matches_separate_calls() {
    let p = Poly::new(vec![2.0, -1.0, 0.5, 3.0]);
    let (v, dv) = p.eval_with_derivative(1.5);
    assert!((v - p.eval(1.5)).abs() < 1e-12);
    assert!((dv - p.derivative().eval(1.5)).abs() < 1e-12);
}

#[semio_framework_async_macros::async_test]
async fn solve_quadratic_finds_known_roots() {
    // (x-2)(x-3) = x^2 -5x+6
    let roots = solve_quadratic(1.0, -5.0, 6.0);
    assert_eq!(roots.len(), 2);
    assert!((roots[0] - 2.0).abs() < 1e-9);
    assert!((roots[1] - 3.0).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn solve_quadratic_handles_no_real_roots() {
    assert!(solve_quadratic(1.0, 0.0, 1.0).is_empty());
}

#[semio_framework_async_macros::async_test]
async fn solve_quadratic_avoids_cancellation_for_large_b() {
    // Classic near-cancellation case: a=1, b=1e8, c=1. Naive formula loses precision.
    let roots = solve_quadratic(1.0, 1e8, 1.0);
    assert_eq!(roots.len(), 2);
    for r in &roots {
        let p = Poly::new(vec![1.0, 1e8, 1.0]);
        assert!(p.eval(*r).abs() / (1e8 * r.abs() + 1.0) < 1e-6, "root {r} not accurate enough");
    }
}

#[semio_framework_async_macros::async_test]
async fn solve_cubic_finds_three_known_real_roots() {
    // (x-1)(x-2)(x-3) = x^3 -6x^2+11x-6
    let mut roots = solve_cubic(1.0, -6.0, 11.0, -6.0);
    roots.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(roots.len(), 3);
    assert!((roots[0] - 1.0).abs() < 1e-9);
    assert!((roots[1] - 2.0).abs() < 1e-9);
    assert!((roots[2] - 3.0).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn solve_cubic_finds_single_real_root() {
    // x^3 + x + 1 has exactly one real root (~-0.6823)
    let roots = solve_cubic(1.0, 0.0, 1.0, 1.0);
    assert_eq!(roots.len(), 1);
    let p = Poly::new(vec![1.0, 1.0, 0.0, 1.0]);
    assert!(p.eval(roots[0]).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn bernstein_eval_matches_monomial_conversion() {
    let p = Poly::new(vec![1.0, 2.0, -3.0, 0.5]);
    let b = Bernstein::from_monomial(&p);
    for i in 0..=10 {
        let t = i as f64 / 10.0;
        assert!((b.eval(t) - p.eval(t)).abs() < 1e-9, "mismatch at t={t}");
    }
}

#[semio_framework_async_macros::async_test]
async fn bernstein_to_monomial_round_trips_from_monomial() {
    let p = Poly::new(vec![3.0, -1.5, 2.0, 4.0, -0.25]);
    let b = Bernstein::from_monomial(&p);
    let back = b.to_monomial();
    assert_eq!(back.coeffs.len(), p.coeffs.len());
    for (a, c) in back.coeffs.iter().zip(p.coeffs.iter()) {
        assert!((a - c).abs() < 1e-8, "coefficient mismatch: {a} vs {c}");
    }
}

#[semio_framework_async_macros::async_test]
async fn bernstein_subdivide_matches_original_at_shared_endpoints_and_split_point() {
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

#[semio_framework_async_macros::async_test]
async fn sign_variations_certifies_no_root_for_monotone_positive_control_polygon() {
    let b = Bernstein::new(vec![1.0, 2.0, 3.0, 4.0]);
    assert_eq!(b.sign_variations(), 0);
}

#[semio_framework_async_macros::async_test]
async fn sign_variations_detects_single_sign_change() {
    let b = Bernstein::new(vec![-1.0, -0.5, 1.0, 2.0]);
    assert_eq!(b.sign_variations(), 1);
}

#[semio_framework_async_macros::async_test]
async fn isolate_roots_finds_single_root_of_linear_bernstein() {
    // Line from -1 at t=0 to 1 at t=1: root at t=0.5.
    let b = Bernstein::new(vec![-1.0, 1.0]);
    let intervals = isolate_roots(&b, 20);
    assert_eq!(intervals.len(), 1);
    assert!(intervals[0].0 <= 0.5 && intervals[0].1 >= 0.5);
}

#[semio_framework_async_macros::async_test]
async fn isolate_roots_finds_no_intervals_for_root_free_polynomial() {
    let b = Bernstein::new(vec![1.0, 2.0, 3.0]);
    assert!(isolate_roots(&b, 20).is_empty());
}

#[semio_framework_async_macros::async_test]
async fn refine_root_converges_to_known_root() {
    let p = Poly::new(vec![-6.0, 11.0, -6.0, 1.0]); // (x-1)(x-2)(x-3)
    let root = refine_root(&p, 2.5, 3.5, 1e-12, 100);
    assert!((root - 3.0).abs() < 1e-9);
}

mod quick {
    use super::*;

    /// 🔮️ Brute-force oracle: dense sampling + bisection finds every sign-change interval,
    /// independent of the Bernstein/Descartes machinery under test.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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

    #[semio_framework_async_macros::async_test]
    async fn isolate_roots_plus_refine_matches_bisection_oracle_on_random_polynomials() {
        let mut rng = semio_framework_geometry::random::Rng::from_seed(23);
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

    #[semio_framework_async_macros::async_test]
    async fn bernstein_monomial_round_trip_holds_on_random_polynomials() {
        let mut rng = semio_framework_geometry::random::Rng::from_seed(29);
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
