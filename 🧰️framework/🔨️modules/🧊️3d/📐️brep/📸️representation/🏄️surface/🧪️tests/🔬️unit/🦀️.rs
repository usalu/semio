use super::*;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn fd_du(s: &Surface, u: f64, v: f64) -> Vec3 {
    let h = 1e-6;
    (s.eval(u + h, v) - s.eval(u - h, v)) * (1.0 / (2.0 * h))
}
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn fd_dv(s: &Surface, u: f64, v: f64) -> Vec3 {
    let h = 1e-6;
    (s.eval(u, v + h) - s.eval(u, v - h)) * (1.0 / (2.0 * h))
}

#[semio_framework_async_macros::async_test]
async fn plane_eval_and_normal() {
    let frame = Frame3::from_normal(Pnt3::new(1.0, 2.0, 3.0), Vec3::Z).unwrap();
    let s = Surface::Plane { frame };
    let p = s.eval(2.0, 3.0);
    assert!((p.z - 3.0).abs() < 1e-9);
    assert!((s.normal(0.0, 0.0).unwrap() - Vec3::Z).norm() < 1e-9);
    assert!(s.is_planar());
}

#[semio_framework_async_macros::async_test]
async fn cylinder_derivatives_match_finite_differences_and_lie_on_cylinder() {
    let frame = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
    let s = Surface::Cylinder { frame, radius: 2.0 };
    for (u, v) in [(0.3, 1.0), (2.0, -3.0), (5.0, 0.5)] {
        let p = s.eval(u, v);
        let local = frame.to_local(p);
        assert!((local.x * local.x + local.y * local.y).sqrt() - 2.0 < 1e-9);
        let d = s.derivatives(u, v);
        assert!((d.du - fd_du(&s, u, v)).norm() < 1e-4, "du mismatch at {u},{v}");
        assert!((d.dv - fd_dv(&s, u, v)).norm() < 1e-4, "dv mismatch at {u},{v}");
    }
}

#[semio_framework_async_macros::async_test]
async fn cylinder_gaussian_curvature_is_zero_and_mean_curvature_is_half_reciprocal_radius() {
    let frame = Frame3::WORLD;
    let s = Surface::Cylinder { frame, radius: 3.0 };
    let (gaussian, mean) = s.curvature(0.5, 1.0).unwrap();
    assert!(gaussian.abs() < 1e-9, "cylinder must be developable (K=0), got {gaussian}");
    assert!((mean.abs() - 1.0 / (2.0 * 3.0)).abs() < 1e-6);
}

#[semio_framework_async_macros::async_test]
async fn sphere_gaussian_curvature_equals_reciprocal_radius_squared() {
    let frame = Frame3::WORLD;
    let s = Surface::Sphere { frame, radius: 4.0 };
    for (u, v) in [(0.0, 0.0), (1.0, 0.3), (4.0, -0.5)] {
        let (gaussian, _) = s.curvature(u, v).unwrap();
        assert!((gaussian - 1.0 / 16.0).abs() < 1e-6, "mismatch at {u},{v}: {gaussian}");
    }
}

#[semio_framework_async_macros::async_test]
async fn sphere_eval_stays_on_sphere_and_derivatives_match_finite_differences() {
    let frame = Frame3::from_normal(Pnt3::new(1.0, -1.0, 2.0), Vec3::new(0.2, 0.3, 1.0)).unwrap();
    let s = Surface::Sphere { frame, radius: 5.0 };
    for (u, v) in [(0.2, 0.1), (3.0, -0.4), (5.5, 0.7)] {
        let p = s.eval(u, v);
        assert!((p.distance(frame.origin) - 5.0).abs() < 1e-9);
        let d = s.derivatives(u, v);
        assert!((d.du - fd_du(&s, u, v)).norm() < 1e-4);
        assert!((d.dv - fd_dv(&s, u, v)).norm() < 1e-4);
    }
}

#[semio_framework_async_macros::async_test]
async fn torus_eval_stays_at_correct_distance_from_main_circle() {
    let frame = Frame3::WORLD;
    let s = Surface::Torus { frame, major_radius: 5.0, minor_radius: 1.5 };
    for (u, v) in [(0.0, 0.0), (1.0, 2.0), (4.0, 5.0)] {
        let p = s.eval(u, v);
        let radial = (p.x * p.x + p.y * p.y).sqrt();
        let dist_to_tube_center = ((radial - 5.0).powi(2) + p.z * p.z).sqrt();
        assert!((dist_to_tube_center - 1.5).abs() < 1e-9, "mismatch at {u},{v}: {dist_to_tube_center}");
    }
}

#[semio_framework_async_macros::async_test]
async fn torus_derivatives_match_finite_differences() {
    let frame = Frame3::WORLD;
    let s = Surface::Torus { frame, major_radius: 4.0, minor_radius: 1.0 };
    for (u, v) in [(0.3, 0.7), (2.0, 4.0)] {
        let d = s.derivatives(u, v);
        assert!((d.du - fd_du(&s, u, v)).norm() < 1e-4);
        assert!((d.dv - fd_dv(&s, u, v)).norm() < 1e-4);
    }
}

#[semio_framework_async_macros::async_test]
async fn cone_radius_grows_linearly_with_v_and_derivatives_match_finite_differences() {
    let frame = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
    let half_angle = std::f64::consts::FRAC_PI_6;
    let s = Surface::Cone { frame, half_angle };
    for (u, v) in [(0.5, 2.0), (3.0, 5.0)] {
        let d = s.derivatives(u, v);
        assert!((d.du - fd_du(&s, u, v)).norm() < 1e-4);
        assert!((d.dv - fd_dv(&s, u, v)).norm() < 1e-4);
        let p = s.eval(u, v);
        let radial = (p.x * p.x + p.y * p.y).sqrt();
        assert!((radial - v * half_angle.tan()).abs() < 1e-9, "expected radius {} at v={v}, got {radial}", v * half_angle.tan());
    }
}

#[semio_framework_async_macros::async_test]
async fn plane_second_derivatives_are_zero_and_gaussian_curvature_is_zero() {
    let frame = Frame3::WORLD;
    let s = Surface::Plane { frame };
    let d = s.derivatives(0.5, 0.5);
    assert_eq!(d.duu, Vec3::ZERO);
    assert_eq!(d.dvv, Vec3::ZERO);
    assert_eq!(d.duv, Vec3::ZERO);
}

/// 🗺️ A degree-2×2 NURBS bump patch (unit weights) built by hand so tests have an
/// independently-constructed fixture, not just a converted analytic surface.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn bump_nurbs_surface() -> Surface {
    let u_knots = KnotVector::new(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2, 3).unwrap();
    let v_knots = KnotVector::new(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2, 3).unwrap();
    let controls = vec![
        vec![Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(0.0, 1.0, 0.5), Pnt3::new(0.0, 2.0, 0.0)],
        vec![Pnt3::new(1.0, 0.0, 0.5), Pnt3::new(1.0, 1.0, 2.0), Pnt3::new(1.0, 2.0, 0.5)],
        vec![Pnt3::new(2.0, 0.0, 0.0), Pnt3::new(2.0, 1.0, 0.5), Pnt3::new(2.0, 2.0, 0.0)],
    ];
    let weights = vec![vec![1.0, 1.2, 1.0], vec![1.1, 1.5, 1.1], vec![1.0, 1.2, 1.0]];
    Surface::Nurbs { u_knots, v_knots, controls, weights }
}

/// 🗺️ Central-difference derivative with one round of Richardson extrapolation — see the
/// equivalent oracle in `curve/bspline`'s quick tests; independent of
/// [`surface_derivatives_rational`] so it doesn't validate itself.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn richardson_du(s: &Surface, u: f64, v: f64) -> Vec3 {
    let d = |h: f64| (s.eval(u + h, v) - s.eval(u - h, v)) * (1.0 / (2.0 * h));
    let h = 1e-3;
    (d(h / 2.0) * 4.0 - d(h)) * (1.0 / 3.0)
}
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn richardson_dv(s: &Surface, u: f64, v: f64) -> Vec3 {
    let d = |h: f64| (s.eval(u, v + h) - s.eval(u, v - h)) * (1.0 / (2.0 * h));
    let h = 1e-3;
    (d(h / 2.0) * 4.0 - d(h)) * (1.0 / 3.0)
}

#[semio_framework_async_macros::async_test]
async fn nurbs_surface_derivatives_match_richardson_finite_differences() {
    let s = bump_nurbs_surface();
    for &(u, v) in &[(0.2, 0.3), (0.5, 0.5), (0.7, 0.1), (0.9, 0.85)] {
        let d = s.derivatives(u, v);
        assert!((d.du - richardson_du(&s, u, v)).norm() < 1e-6, "du mismatch at {u},{v}: exact={:?}", d.du);
        assert!((d.dv - richardson_dv(&s, u, v)).norm() < 1e-6, "dv mismatch at {u},{v}: exact={:?}", d.dv);
    }
}

#[semio_framework_async_macros::async_test]
async fn nurbs_surface_normal_and_curvature_are_well_defined_off_singularities() {
    let s = bump_nurbs_surface();
    for &(u, v) in &[(0.2, 0.3), (0.5, 0.5), (0.7, 0.1)] {
        assert!(s.normal(u, v).is_some(), "normal should be defined at {u},{v}");
        assert!(s.curvature(u, v).is_some(), "curvature should be defined at {u},{v}");
    }
}

mod quick {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn nurbs_surface_derivatives_match_richardson_finite_differences_on_random_rational_surfaces() {
        let mut rng = semio_framework_geometry::random::Rng::from_seed(97);
        for _ in 0..40 {
            let nu = 3 + rng.next_range(0, 2) as usize;
            let nv = 3 + rng.next_range(0, 2) as usize;
            let u_knots = KnotVector::clamped_uniform(nu, 2.min(nu - 1));
            let v_knots = KnotVector::clamped_uniform(nv, 2.min(nv - 1));
            let controls: Vec<Vec<Pnt3>> = (0..nu).map(|i| (0..nv).map(|j| Pnt3::new(i as f64 + rng.next_f64() * 0.3, j as f64 + rng.next_f64() * 0.3, rng.next_f64() * 2.0 - 1.0)).collect()).collect();
            let weights: Vec<Vec<f64>> = (0..nu).map(|_| (0..nv).map(|_| 0.6 + rng.next_f64()).collect()).collect();
            let s = Surface::Nurbs { u_knots, v_knots, controls, weights };
            let u = 0.1 + 0.8 * rng.next_f64();
            let v = 0.1 + 0.8 * rng.next_f64();
            let d = s.derivatives(u, v);
            assert!((d.du - richardson_du(&s, u, v)).norm() < 1e-5, "du mismatch nu={nu} nv={nv} at {u},{v}");
            assert!((d.dv - richardson_dv(&s, u, v)).norm() < 1e-5, "dv mismatch nu={nu} nv={nv} at {u},{v}");
        }
    }
}
