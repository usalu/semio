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
