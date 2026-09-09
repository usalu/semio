use super::*;

#[semio_framework_async_macros::async_test]
async fn plane_isocurve_matches_surface_eval() {
    let frame = Frame3::from_normal(Pnt3::new(1.0, 2.0, 3.0), Vec3::Z).unwrap();
    let s = Surface::Plane { frame };
    let iso = s.isocurve(IsoDirection::U, 2.0);
    for v in [-3.0, 0.0, 4.5] {
        assert!(iso.eval(v).distance(s.eval(2.0, v)) < 1e-9);
    }
}

#[semio_framework_async_macros::async_test]
async fn cylinder_isocurves_match_surface_eval() {
    let frame = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
    let s = Surface::Cylinder { frame, radius: 2.0 };
    let u_iso = s.isocurve(IsoDirection::U, 0.7);
    for v in [-3.0, 0.0, 5.0] {
        assert!(u_iso.eval(v).distance(s.eval(0.7, v)) < 1e-9, "u-isocurve mismatch at v={v}");
    }
    let v_iso = s.isocurve(IsoDirection::V, 3.0);
    for u in [0.0, 1.5, 4.0] {
        assert!(v_iso.eval(u).distance(s.eval(u, 3.0)) < 1e-9, "v-isocurve mismatch at u={u}");
    }
}

#[semio_framework_async_macros::async_test]
async fn sphere_isocurves_match_surface_eval() {
    let frame = Frame3::from_normal(Pnt3::new(1.0, -1.0, 2.0), Vec3::new(0.2, 0.3, 1.0)).unwrap();
    let s = Surface::Sphere { frame, radius: 4.0 };
    let u_iso = s.isocurve(IsoDirection::U, 1.1);
    for v in [-1.2, 0.0, 1.2] {
        assert!(u_iso.eval(v).distance(s.eval(1.1, v)) < 1e-8, "meridian mismatch at v={v}");
    }
    let v_iso = s.isocurve(IsoDirection::V, 0.4);
    for u in [0.0, 2.0, 5.0] {
        assert!(v_iso.eval(u).distance(s.eval(u, 0.4)) < 1e-8, "parallel mismatch at u={u}");
    }
}

#[semio_framework_async_macros::async_test]
async fn torus_isocurves_match_surface_eval() {
    let frame = Frame3::WORLD;
    let s = Surface::Torus { frame, major_radius: 5.0, minor_radius: 1.5 };
    let u_iso = s.isocurve(IsoDirection::U, 0.9);
    for v in [0.0, 2.0, 5.0] {
        assert!(u_iso.eval(v).distance(s.eval(0.9, v)) < 1e-8, "tube-circle mismatch at v={v}");
    }
    let v_iso = s.isocurve(IsoDirection::V, 1.3);
    for u in [0.0, 2.0, 5.5] {
        assert!(v_iso.eval(u).distance(s.eval(u, 1.3)) < 1e-8, "parallel-circle mismatch at u={u}");
    }
}

#[semio_framework_async_macros::async_test]
async fn cone_isocurves_match_surface_eval() {
    let frame = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
    let s = Surface::Cone { frame, half_angle: std::f64::consts::FRAC_PI_6 };
    let u_iso = s.isocurve(IsoDirection::U, 0.5);
    for v in [0.0, 2.0, 6.0] {
        assert!(u_iso.eval(v).distance(s.eval(0.5, v)) < 1e-8, "generator mismatch at v={v}");
    }
    let v_iso = s.isocurve(IsoDirection::V, 3.0);
    for u in [0.0, 1.0, 4.0] {
        assert!(v_iso.eval(u).distance(s.eval(u, 3.0)) < 1e-8, "cone parallel mismatch at u={u}");
    }
}

#[semio_framework_async_macros::async_test]
async fn nurbs_isocurve_matches_surface_eval_and_lies_on_surface() {
    let u_knots = KnotVector::clamped_uniform(4, 3);
    let v_knots = KnotVector::clamped_uniform(4, 3);
    let controls: Vec<Vec<Pnt3>> = (0..4).map(|i| (0..4).map(|j| Pnt3::new(i as f64, j as f64, (i as f64 - j as f64).sin())).collect()).collect();
    let weights = vec![vec![1.0; 4]; 4];
    let s = Surface::Nurbs { u_knots, v_knots, controls, weights };
    let u_iso = s.isocurve(IsoDirection::U, 0.42);
    for v in [0.0, 0.3, 0.7, 1.0] {
        assert!(u_iso.eval(v).distance(s.eval(0.42, v)) < 1e-9, "u-isocurve mismatch at v={v}");
    }
    let v_iso = s.isocurve(IsoDirection::V, 0.6);
    for u in [0.0, 0.2, 0.9, 1.0] {
        assert!(v_iso.eval(u).distance(s.eval(u, 0.6)) < 1e-9, "v-isocurve mismatch at u={u}");
    }
}

mod quick {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn analytic_isocurves_lie_exactly_on_their_surfaces_for_random_kinds() {
        let mut rng = semio_framework_geometry::random::Rng::from_seed(97);
        for _ in 0..100 {
            let frame =
                Frame3::from_normal(Pnt3::new(rng.next_f64() * 4.0 - 2.0, rng.next_f64() * 4.0 - 2.0, rng.next_f64() * 4.0 - 2.0), Vec3::new(rng.next_f64() - 0.5, rng.next_f64() - 0.5, rng.next_f64() - 0.5).normalized().unwrap_or(Vec3::Z)).unwrap();
            let s = Surface::Torus { frame, major_radius: 2.0 + rng.next_f64() * 3.0, minor_radius: 0.2 + rng.next_f64() * 0.5 };
            let u_at = rng.next_f64() * std::f64::consts::TAU;
            let v_at = rng.next_f64() * std::f64::consts::TAU;
            let u_iso = s.isocurve(IsoDirection::U, u_at);
            let v_iso = s.isocurve(IsoDirection::V, v_at);
            for t in [0.1, 1.0, 3.0, 5.0] {
                assert!(u_iso.eval(t).distance(s.eval(u_at, t)) < 1e-8, "u-iso mismatch");
                assert!(v_iso.eval(t).distance(s.eval(t, v_at)) < 1e-8, "v-iso mismatch");
            }
        }
    }
}
