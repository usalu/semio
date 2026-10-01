use super::*;

#[semio_framework_async_macros::async_test]
async fn orient2d_detects_counterclockwise_and_clockwise() {
    let a = Pnt2::new(0.0, 0.0);
    let b = Pnt2::new(1.0, 0.0);
    let c = Pnt2::new(0.0, 1.0);
    assert_eq!(orient2d(a, b, c), Orient::Positive);
    assert_eq!(orient2d(a, c, b), Orient::Negative);
}

#[semio_framework_async_macros::async_test]
async fn orient2d_detects_exact_collinearity() {
    let a = Pnt2::new(0.0, 0.0);
    let b = Pnt2::new(1.0, 1.0);
    let c = Pnt2::new(2.0, 2.0);
    assert_eq!(orient2d(a, b, c), Orient::Zero);
    assert!(collinear2d(a, b, c));
}

/// 🎯️ The true next representable `f64` above/below `x` — unlike adding `f64::EPSILON`, this
/// is a real one-bit perturbation regardless of `x`'s magnitude (ULP scales with exponent).
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn next_up(x: f64) -> f64 {
    f64::from_bits(x.to_bits() + 1)
}
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn next_down(x: f64) -> f64 {
    f64::from_bits(x.to_bits() - 1)
}

#[semio_framework_async_macros::async_test]
async fn orient2d_resolves_near_degenerate_case_correctly() {
    // c sits exactly on line a->b->(2,2); perturbing it by a single ULP must still resolve
    // to the geometrically correct sign, which the interval filter alone cannot certify.
    let a = Pnt2::new(0.0, 0.0);
    let b = Pnt2::new(1.0, 1.0);
    let c_on = Pnt2::new(2.0, 2.0);
    let c_left = Pnt2::new(2.0, next_up(2.0));
    let c_right = Pnt2::new(2.0, next_down(2.0));
    assert_eq!(orient2d(a, b, c_on), Orient::Zero);
    assert_eq!(orient2d(a, b, c_left), Orient::Positive);
    assert_eq!(orient2d(a, b, c_right), Orient::Negative);
}

#[semio_framework_async_macros::async_test]
async fn orient3d_detects_right_handed_and_left_handed_tetrahedra() {
    let a = Pnt3::new(0.0, 0.0, 0.0);
    let b = Pnt3::new(1.0, 0.0, 0.0);
    let c = Pnt3::new(0.0, 1.0, 0.0);
    let d = Pnt3::new(0.0, 0.0, 1.0);
    assert_eq!(orient3d(a, b, c, d), Orient::Positive);
    assert_eq!(orient3d(a, c, b, d), Orient::Negative);
}

#[semio_framework_async_macros::async_test]
async fn orient3d_detects_exact_coplanarity() {
    let a = Pnt3::new(0.0, 0.0, 0.0);
    let b = Pnt3::new(1.0, 0.0, 0.0);
    let c = Pnt3::new(0.0, 1.0, 0.0);
    let d = Pnt3::new(1.0, 1.0, 0.0);
    assert_eq!(orient3d(a, b, c, d), Orient::Zero);
    assert!(coplanar3d(a, b, c, d));
}

#[semio_framework_async_macros::async_test]
async fn orient3d_resolves_near_degenerate_case_correctly() {
    let a = Pnt3::new(0.0, 0.0, 0.0);
    let b = Pnt3::new(1.0, 0.0, 0.0);
    let c = Pnt3::new(0.0, 1.0, 0.0);
    let tiny = f64::EPSILON;
    let d_above = Pnt3::new(0.3, 0.3, tiny);
    let d_below = Pnt3::new(0.3, 0.3, -tiny);
    assert_eq!(orient3d(a, b, c, d_above), Orient::Positive);
    assert_eq!(orient3d(a, b, c, d_below), Orient::Negative);
}

#[semio_framework_async_macros::async_test]
async fn in_circle2d_detects_inside_and_outside_unit_circle() {
    let a = Pnt2::new(1.0, 0.0);
    let b = Pnt2::new(0.0, 1.0);
    let c = Pnt2::new(-1.0, 0.0);
    let inside = Pnt2::new(0.0, 0.0);
    let outside = Pnt2::new(0.0, 5.0);
    let on = Pnt2::new(0.0, -1.0);
    assert_eq!(in_circle2d(a, b, c, inside), Orient::Positive);
    assert_eq!(in_circle2d(a, b, c, outside), Orient::Negative);
    assert_eq!(in_circle2d(a, b, c, on), Orient::Zero);
}

#[semio_framework_async_macros::async_test]
async fn sign_of_dot_classifies_acute_right_obtuse() {
    assert_eq!(sign_of_dot(Vec3::X, Vec3::X), Orient::Positive);
    assert_eq!(sign_of_dot(Vec3::X, Vec3::Y), Orient::Zero);
    assert_eq!(sign_of_dot(Vec3::X, -Vec3::X), Orient::Negative);
}

mod quick {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn orient2d_filtered_agrees_with_exact_on_random_and_near_degenerate_triples() {
        let mut rng = semio_framework_geometry::random::Rng::from_seed(11);
        for _ in 0..5000 {
            let a = Pnt2::new(rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0);
            let b = Pnt2::new(rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0);
            // Bias half the sample toward near-collinear configurations, where the filter
            // is most likely to need the exact escalation path.
            let c = if rng.next_bool(0.5) {
                let t = rng.next_f64() * 2.0 - 0.5;
                let perturb = (rng.next_f64() - 0.5) * 1e-12;
                Pnt2::new(a.x + (b.x - a.x) * t + perturb, a.y + (b.y - a.y) * t)
            } else {
                Pnt2::new(rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0)
            };
            assert_eq!(orient2d(a, b, c), orient2d_exact(a, b, c), "mismatch for {a:?} {b:?} {c:?}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn orient3d_filtered_agrees_with_exact_on_random_and_near_degenerate_quadruples() {
        let mut rng = semio_framework_geometry::random::Rng::from_seed(13);
        for _ in 0..3000 {
            let a = Pnt3::new(rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0);
            let b = Pnt3::new(rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0);
            let c = Pnt3::new(rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0);
            let d = if rng.next_bool(0.5) {
                let u = rng.next_f64() * 2.0 - 0.5;
                let v = rng.next_f64() * 2.0 - 0.5;
                let perturb = (rng.next_f64() - 0.5) * 1e-12;
                Pnt3::new(a.x + (b.x - a.x) * u + (c.x - a.x) * v + perturb, a.y + (b.y - a.y) * u + (c.y - a.y) * v, a.z + (b.z - a.z) * u + (c.z - a.z) * v)
            } else {
                Pnt3::new(rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0)
            };
            assert_eq!(orient3d(a, b, c, d), orient3d_exact(a, b, c, d), "mismatch for {a:?} {b:?} {c:?} {d:?}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn in_circle2d_filtered_agrees_with_exact_on_random_configurations() {
        let mut rng = semio_framework_geometry::random::Rng::from_seed(17);
        for _ in 0..3000 {
            let pts: Vec<Pnt2> = (0..4).map(|_| Pnt2::new(rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0)).collect();
            assert_eq!(in_circle2d(pts[0], pts[1], pts[2], pts[3]), in_circle2d_exact(pts[0], pts[1], pts[2], pts[3]));
        }
    }
}

/// ⚖️ The third-party half of the exact path's case. Every predicate here decides a sign by
/// nonoverlapping floating-point EXPANSION arithmetic; `num-rational`'s arbitrary-precision
/// `BigRational` recomputes the same determinant in a completely different representation, by a
/// completely different algorithm, written outside this repository. A disagreement convicts the
/// expansion code — the rational side cannot round, and both sides read the same `f64` inputs
/// losslessly (`BigRational::from_float` is exact for every finite `f64`).
///
/// ⚠️ WHAT THIS CANNOT ESTABLISH, stated so the evidence is not overstated: it says nothing about
/// the FILTER — whether the cheap `f64` path escalates when it must is the `quick` module's
/// question, and the two are kept separate on purpose. Together they close the case: `quick` says
/// the filter never answers where it should escalate, and this says the escalation is exact.
///
/// @see ../../🦀️.rs — the expansion arithmetic under test.
/// @see https://docs.rs/num-rational — the oracle.
mod oracle {
    use super::*;
    use num_rational::BigRational;

    /// 🔢 One finite `f64` as an exact rational.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn exact(value: f64) -> BigRational {
        BigRational::from_float(value).expect("a finite f64 is exactly representable as a rational")
    }

    /// 🎯️ The oracle's own `orient2d` determinant sign.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn orient2d_oracle(a: Pnt2, b: Pnt2, c: Pnt2) -> Orient {
        let det = (exact(b.x) - exact(a.x)) * (exact(c.y) - exact(a.y)) - (exact(b.y) - exact(a.y)) * (exact(c.x) - exact(a.x));
        Orient::from(det.cmp(&BigRational::from_float(0.0).expect("zero is rational")))
    }

    /// 🎯️ The oracle's own `orient3d` determinant sign.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn orient3d_oracle(a: Pnt3, b: Pnt3, c: Pnt3, d: Pnt3) -> Orient {
        let component = |value: f64, origin: f64| exact(value) - exact(origin);
        let u = [component(b.x, a.x), component(b.y, a.y), component(b.z, a.z)];
        let v = [component(c.x, a.x), component(c.y, a.y), component(c.z, a.z)];
        let w = [component(d.x, a.x), component(d.y, a.y), component(d.z, a.z)];
        let det = u[0].clone() * v[1].clone() * w[2].clone() - u[0].clone() * v[2].clone() * w[1].clone() + u[1].clone() * v[2].clone() * w[0].clone() - u[1].clone() * v[0].clone() * w[2].clone()
            + u[2].clone() * v[0].clone() * w[1].clone()
            - u[2].clone() * v[1].clone() * w[0].clone();
        Orient::from(det.cmp(&BigRational::from_float(0.0).expect("zero is rational")))
    }

    /// 🎯️ The oracle's own `in_circle2d` determinant sign.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn in_circle2d_oracle(a: Pnt2, b: Pnt2, c: Pnt2, d: Pnt2) -> Orient {
        let delta = |p: Pnt2| (exact(p.x) - exact(d.x), exact(p.y) - exact(d.y));
        let (adx, ady) = delta(a);
        let (bdx, bdy) = delta(b);
        let (cdx, cdy) = delta(c);
        let square = |x: &BigRational, y: &BigRational| x.clone() * x.clone() + y.clone() * y.clone();
        let ad2 = square(&adx, &ady);
        let bd2 = square(&bdx, &bdy);
        let cd2 = square(&cdx, &cdy);
        let t1 = adx.clone() * (bdy.clone() * cd2.clone() - cdy.clone() * bd2.clone());
        let t2 = ady * (bdx.clone() * cd2 - cdx.clone() * bd2);
        let t3 = ad2 * (bdx * cdy - cdx * bdy);
        Orient::from((t1 - t2 + t3).cmp(&BigRational::from_float(0.0).expect("zero is rational")))
    }

    /// 🎯️ The oracle's own `sign_of_dot`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sign_of_dot_oracle(u: Vec3, v: Vec3) -> Orient {
        let dot = exact(u.x) * exact(v.x) + exact(u.y) * exact(v.y) + exact(u.z) * exact(v.z);
        Orient::from(dot.cmp(&BigRational::from_float(0.0).expect("zero is rational")))
    }

    #[semio_framework_async_macros::async_test]
    async fn orient2d_expansion_matches_arbitrary_precision_rationals() {
        let mut rng = semio_framework_geometry::random::Rng::from_seed(101);
        for _ in 0..2000 {
            let a = Pnt2::new(rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0);
            let b = Pnt2::new(rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0);
            let c = if rng.next_bool(0.5) {
                let t = rng.next_f64() * 2.0 - 0.5;
                Pnt2::new(a.x + (b.x - a.x) * t, a.y + (b.y - a.y) * t)
            } else {
                Pnt2::new(rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0)
            };
            assert_eq!(orient2d_exact(a, b, c), orient2d_oracle(a, b, c), "expansion disagreed with the rational oracle for {a:?} {b:?} {c:?}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn orient2d_expansion_matches_the_oracle_one_ulp_either_side_of_collinear() {
        let a = Pnt2::new(0.0, 0.0);
        let b = Pnt2::new(1.0, 1.0);
        for x in [0.5, 2.0, 7.0, 1e-7, 1e7] {
            for y in [x, next_up(x), next_down(x)] {
                let c = Pnt2::new(x, y);
                assert_eq!(orient2d_exact(a, b, c), orient2d_oracle(a, b, c), "expansion disagreed with the rational oracle for {c:?}");
            }
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn orient3d_expansion_matches_arbitrary_precision_rationals() {
        let mut rng = semio_framework_geometry::random::Rng::from_seed(103);
        for _ in 0..600 {
            let point = |rng: &mut semio_framework_geometry::random::Rng| Pnt3::new(rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0);
            let a = point(&mut rng);
            let b = point(&mut rng);
            let c = point(&mut rng);
            let d = if rng.next_bool(0.5) {
                let (u, v) = (rng.next_f64() * 2.0 - 0.5, rng.next_f64() * 2.0 - 0.5);
                Pnt3::new(a.x + (b.x - a.x) * u + (c.x - a.x) * v, a.y + (b.y - a.y) * u + (c.y - a.y) * v, a.z + (b.z - a.z) * u + (c.z - a.z) * v)
            } else {
                point(&mut rng)
            };
            assert_eq!(orient3d_exact(a, b, c, d), orient3d_oracle(a, b, c, d), "expansion disagreed with the rational oracle for {a:?} {b:?} {c:?} {d:?}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn in_circle2d_expansion_matches_arbitrary_precision_rationals() {
        let mut rng = semio_framework_geometry::random::Rng::from_seed(107);
        for _ in 0..300 {
            let pts: Vec<Pnt2> = (0..4).map(|_| Pnt2::new(rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0)).collect();
            assert_eq!(in_circle2d_exact(pts[0], pts[1], pts[2], pts[3]), in_circle2d_oracle(pts[0], pts[1], pts[2], pts[3]));
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn sign_of_dot_expansion_matches_arbitrary_precision_rationals() {
        let mut rng = semio_framework_geometry::random::Rng::from_seed(109);
        for _ in 0..2000 {
            let u = Vec3::new(rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0);
            let v = if rng.next_bool(0.5) { Vec3::new(-u.y, u.x, 0.0) } else { Vec3::new(rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0) };
            assert_eq!(sign_of_dot_exact(u, v), sign_of_dot_oracle(u, v), "expansion disagreed with the rational oracle for {u:?} {v:?}");
        }
    }
}
