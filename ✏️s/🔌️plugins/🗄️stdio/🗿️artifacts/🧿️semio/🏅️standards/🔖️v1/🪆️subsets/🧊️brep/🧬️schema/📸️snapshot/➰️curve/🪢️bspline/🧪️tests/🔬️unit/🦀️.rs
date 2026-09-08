
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
    let b = super::super::bezier::RationalBezier2::unweighted(control_values.iter().map(|&v| crate::standards::v1::subsets::brep::schema::snapshot::vector::Pnt2::new(v, 0.0)).collect());
    let be = super::super::bezier::RationalBezier2::unweighted(elevated.iter().map(|&v| crate::standards::v1::subsets::brep::schema::snapshot::vector::Pnt2::new(v, 0.0)).collect());
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
        let expected: Vec<f64> = (0..=2).map(|k| (0..=kv.degree).map(|j| plain[k][j] * values[span - kv.degree + j]).sum()).collect();
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
    let controls_h: Vec<Vec<Vec<f64>>> = controls_h
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|mut p| {
                    p.push(1.0);
                    p
                })
                .collect()
        })
        .collect();
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
            let bernstein = crate::standards::v1::subsets::brep::schema::snapshot::polynomial::Bernstein::new(values.clone());
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
