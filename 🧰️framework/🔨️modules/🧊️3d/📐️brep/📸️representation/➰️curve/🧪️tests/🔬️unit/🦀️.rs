use super::*;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn fd_d1(curve: &Curve3, t: f64) -> Vec3 {
    let h = 1e-6;
    (curve.eval(t + h) - curve.eval(t - h)) * (1.0 / (2.0 * h))
}
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn fd_d2(curve: &Curve3, t: f64) -> Vec3 {
    let h = 1e-4;
    let a = curve.eval(t + h).to_vec();
    let b = curve.eval(t).to_vec();
    let c = curve.eval(t - h).to_vec();
    (a - b * 2.0 + c) * (1.0 / (h * h))
}

#[semio_framework_async_macros::async_test]
async fn line_eval_and_derivatives() {
    let l = Curve3::Line { origin: Pnt3::new(1.0, 2.0, 3.0), dir: Vec3::new(2.0, 0.0, 0.0) };
    assert_eq!(l.eval(0.5), Pnt3::new(2.0, 2.0, 3.0));
    assert_eq!(l.d1(0.5), Vec3::new(2.0, 0.0, 0.0));
    assert_eq!(l.d2(0.5), Vec3::ZERO);
    assert_eq!(l.curvature(0.5), 0.0);
}

#[semio_framework_async_macros::async_test]
async fn circle_eval_stays_on_circle_and_derivatives_match_finite_differences() {
    let frame = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
    let c = Curve3::Circle { frame, radius: 3.0 };
    for i in 0..10 {
        let t = i as f64 * 0.5;
        let p = c.eval(t);
        assert!((p.to_vec().norm() - 3.0).abs() < 1e-9);
        assert!((c.d1(t) - fd_d1(&c, t)).norm() < 1e-5);
        assert!((c.d2(t) - fd_d2(&c, t)).norm() < 1e-2);
    }
}

#[semio_framework_async_macros::async_test]
async fn circle_curvature_equals_reciprocal_radius() {
    let frame = Frame3::from_normal(Pnt3::new(1.0, 1.0, 1.0), Vec3::X).unwrap();
    let c = Curve3::Circle { frame, radius: 2.5 };
    for t in [0.0, 1.0, 3.0, 5.5] {
        assert!((c.curvature(t) - 1.0 / 2.5).abs() < 1e-6, "curvature mismatch at t={t}");
    }
}

#[semio_framework_async_macros::async_test]
async fn ellipse_derivatives_match_finite_differences() {
    let frame = Frame3::WORLD;
    let e = Curve3::Ellipse { frame, major_radius: 4.0, minor_radius: 2.0 };
    for i in 0..8 {
        let t = i as f64 * 0.7;
        assert!((e.d1(t) - fd_d1(&e, t)).norm() < 1e-5, "d1 mismatch at t={t}");
    }
}

#[semio_framework_async_macros::async_test]
async fn line_to_nurbs_matches_line_eval() {
    let l = Curve3::Line { origin: Pnt3::new(0.0, 0.0, 0.0), dir: Vec3::new(1.0, 2.0, 3.0) };
    let nurbs = l.to_nurbs((0.0, 2.0));
    for i in 0..=10 {
        let t = i as f64 / 10.0 * 2.0;
        let via_nurbs = eval_nurbs_curve(&nurbs.knots, &nurbs.controls, &nurbs.weights, t);
        assert!(via_nurbs.distance(l.eval(t)) < 1e-9, "mismatch at t={t}");
    }
}

/// 🌀️ The invariant a rational arc-to-NURBS conversion actually guarantees: every produced
/// point lies exactly on the circle (radius match at the frame's own scale), and the curve
/// agrees with the original at `domain.0`/`domain.1` — NOT pointwise parameter equality
/// in between, since the standard construction is not angle-linear except at breakpoints
/// (confirmed by hand + a standalone check: see phase-2 scope note).
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn assert_nurbs_traces_circle(nurbs: &NurbsCurve3, frame: &Frame3, radius: f64, domain: (f64, f64), samples: usize) {
    for i in 0..=samples {
        let t = domain.0 + (domain.1 - domain.0) * (i as f64 / samples as f64);
        let p = eval_nurbs_curve(&nurbs.knots, &nurbs.controls, &nurbs.weights, t);
        let local = frame.to_local(p);
        assert!((local.to_vec().norm() - radius).abs() < 1e-8, "point at t={t} is not on the circle: radius {}", local.to_vec().norm());
    }
}

#[semio_framework_async_macros::async_test]
async fn circle_to_nurbs_traces_the_circle_exactly_for_small_arc() {
    let frame = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
    let c = Curve3::Circle { frame, radius: 5.0 };
    let domain = (0.2, 0.2 + std::f64::consts::FRAC_PI_3); // 60 degrees, single span
    let nurbs = c.to_nurbs(domain);
    assert_nurbs_traces_circle(&nurbs, &frame, 5.0, domain, 20);
    assert!(nurbs.controls[0].distance(c.eval(domain.0)) < 1e-9);
    assert!(nurbs.controls.last().unwrap().distance(c.eval(domain.1)) < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn circle_to_nurbs_traces_the_circle_exactly_for_full_circle_multi_span() {
    let frame = Frame3::from_normal(Pnt3::new(2.0, -1.0, 0.5), Vec3::new(0.3, 0.2, 1.0)).unwrap();
    let c = Curve3::Circle { frame, radius: 1.7 };
    let domain = c.domain();
    let nurbs = c.to_nurbs(domain);
    assert!(nurbs.controls.len() > 3, "a full circle must be split into more than one span");
    assert_nurbs_traces_circle(&nurbs, &frame, 1.7, domain, 60);
    assert!(nurbs.controls[0].distance(c.eval(domain.0)) < 1e-8);
    assert!(nurbs.controls.last().unwrap().distance(c.eval(domain.1)) < 1e-8);
}

#[semio_framework_async_macros::async_test]
async fn ellipse_to_nurbs_traces_the_ellipse_exactly() {
    let frame = Frame3::WORLD;
    let major = 3.0;
    let minor = 1.0;
    let e = Curve3::Ellipse { frame, major_radius: major, minor_radius: minor };
    let domain = (0.0, std::f64::consts::PI * 1.5);
    let nurbs = e.to_nurbs(domain);
    for i in 0..=30 {
        let t = domain.0 + (domain.1 - domain.0) * (i as f64 / 30.0);
        let p = eval_nurbs_curve(&nurbs.knots, &nurbs.controls, &nurbs.weights, t);
        let local = frame.to_local(p);
        let residual = (local.x / major).powi(2) + (local.y / minor).powi(2) - 1.0;
        assert!(residual.abs() < 1e-8, "point at t={t} is not on the ellipse: residual={residual}");
    }
    assert!(nurbs.controls[0].distance(e.eval(domain.0)) < 1e-9);
    assert!(nurbs.controls.last().unwrap().distance(e.eval(domain.1)) < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn curve2_line_and_circle_eval() {
    let l = Curve2::Line { origin: Pnt2::new(0.0, 0.0), dir: Vec2::new(1.0, 1.0) };
    assert_eq!(l.eval(2.0), Pnt2::new(2.0, 2.0));
    let c = Curve2::Circle { center: Pnt2::new(1.0, 1.0), radius: 2.0 };
    let p = c.eval(0.0);
    assert!(((p - Pnt2::new(1.0, 1.0)).norm() - 2.0).abs() < 1e-9);
}

/// 🌀️ A quarter-circle as an exact rational-quadratic NURBS (radius 1, centered at origin,
/// `t=0` at `(1,0)`, `t=1` at `(0,1)`) — the same construction as [`Curve3::to_nurbs`] would
/// produce for a 90° arc, built by hand here so the test has an independent oracle.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn quarter_circle_nurbs() -> Curve3 {
    let w = std::f64::consts::FRAC_PI_4.cos();
    Curve3::Nurbs { knots: KnotVector::new(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2, 3).unwrap(), controls: vec![Pnt3::new(1.0, 0.0, 0.0), Pnt3::new(1.0, 1.0, 0.0), Pnt3::new(0.0, 1.0, 0.0)], weights: vec![1.0, w, 1.0] }
}

#[semio_framework_async_macros::async_test]
async fn nurbs_circle_d1_d2_are_exact_not_finite_difference() {
    let c = quarter_circle_nurbs();
    for i in 1..20 {
        let t = i as f64 / 20.0;
        let p = c.eval(t).to_vec();
        let d1 = c.d1(t);
        let d2 = c.d2(t);
        // On a unit circle: |C|=1, C·C'=0 (tangent ⟂ radius), and the exact curvature formula
        // |C'×C''|/|C'|³ must equal 1 (reciprocal of unit radius) to within 1e-9 — a much
        // tighter bound than the old finite-difference implementation could ever satisfy.
        assert!((p.norm() - 1.0).abs() < 1e-9, "off unit circle at t={t}");
        assert!(p.dot(d1).abs() < 1e-9, "tangent not perpendicular to radius at t={t}");
        let speed = d1.norm();
        let curvature = d1.cross(d2).norm() / speed.powi(3);
        assert!((curvature - 1.0).abs() < 1e-9, "curvature mismatch at t={t}: {curvature}");
    }
}

#[semio_framework_async_macros::async_test]
async fn nurbs_d1_matches_analytic_circle_d1_within_tight_tolerance() {
    let analytic = Curve3::Circle { frame: Frame3::WORLD, radius: 1.0 };
    let nurbs = quarter_circle_nurbs();
    for i in 1..20 {
        let nurbs_t = i as f64 / 20.0;
        // The rational-quadratic parametrization is not angle-linear (see `to_nurbs`'s own
        // doc), so derive the true angle from the NURBS point itself rather than assuming a
        // linear map, then compare unit tangent *directions* (which depend only on position on
        // the circle, not on parametrization speed).
        let p = nurbs.eval(nurbs_t);
        let angle = p.y.atan2(p.x);
        let a_dir = analytic.d1(angle).normalized().unwrap();
        let n_dir = nurbs.d1(nurbs_t).normalized().unwrap();
        assert!((a_dir - n_dir).norm() < 1e-6, "tangent direction mismatch at nurbs_t={nurbs_t} (angle={angle})");
    }
}

mod quick {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn circle_to_nurbs_traces_the_circle_exactly_for_random_arcs() {
        let mut rng = semio_framework_geometry::random::Rng::from_seed(53);
        for _ in 0..100 {
            let frame =
                Frame3::from_normal(Pnt3::new(rng.next_f64() * 4.0 - 2.0, rng.next_f64() * 4.0 - 2.0, rng.next_f64() * 4.0 - 2.0), Vec3::new(rng.next_f64() - 0.5, rng.next_f64() - 0.5, rng.next_f64() - 0.5).normalized().unwrap_or(Vec3::Z)).unwrap();
            let radius = 0.1 + rng.next_f64() * 10.0;
            let c = Curve3::Circle { frame, radius };
            let a0 = rng.next_f64() * std::f64::consts::TAU;
            let span = rng.next_f64() * std::f64::consts::TAU * 1.5;
            let domain = (a0, a0 + span);
            let nurbs = c.to_nurbs(domain);
            assert_nurbs_traces_circle(&nurbs, &frame, radius, domain, 25);
            assert!(nurbs.controls[0].distance(c.eval(domain.0)) < 1e-7, "start point mismatch radius={radius} domain={domain:?}");
            assert!(nurbs.controls.last().unwrap().distance(c.eval(domain.1)) < 1e-7, "end point mismatch radius={radius} domain={domain:?}");
        }
    }
}
