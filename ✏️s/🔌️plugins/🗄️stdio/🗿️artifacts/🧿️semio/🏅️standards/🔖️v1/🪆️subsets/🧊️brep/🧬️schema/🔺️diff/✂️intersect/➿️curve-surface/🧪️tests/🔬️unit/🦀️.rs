use super::*;

#[semio_framework_async_macros::async_test]
async fn line_pierces_plane_z0() {
    let curve = Curve3::Line { origin: Pnt3::new(0.0, 0.0, -1.0), dir: Vec3::new(0.0, 0.0, 1.0) };
    let surface = Surface::Plane { frame: Frame3::WORLD };
    let hits = intersect_curve_surface(&curve, &surface, 1e-9).unwrap();
    assert_eq!(hits.len(), 1);
    assert!(hits[0].point.distance(Pnt3::new(0.0, 0.0, 0.0)) < 1e-9);
    assert!((hits[0].t - 1.0).abs() < 1e-9);
    assert!(hits[0].u.abs() < 1e-9);
    assert!(hits[0].v.abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn line_through_sphere() {
    let curve = Curve3::Line { origin: Pnt3::new(-2.0, 0.0, 0.0), dir: Vec3::new(1.0, 0.0, 0.0) };
    let surface = Surface::Sphere { frame: Frame3::WORLD, radius: 1.0 };
    let hits = intersect_curve_surface(&curve, &surface, 1e-9).unwrap();
    assert_eq!(hits.len(), 2);
    for h in &hits {
        assert!((h.point.to_vec().norm() - 1.0).abs() < 1e-9);
        let on_curve = curve.eval(h.t);
        let on_surf = surface.eval(h.u, h.v);
        assert!(on_curve.distance(h.point) < 1e-8);
        assert!(on_surf.distance(h.point) < 1e-8);
    }
}

#[semio_framework_async_macros::async_test]
async fn line_through_cylinder() {
    let curve = Curve3::Line { origin: Pnt3::new(-2.0, 0.0, 1.0), dir: Vec3::new(1.0, 0.0, 0.0) };
    let surface = Surface::Cylinder { frame: Frame3::WORLD, radius: 1.0 };
    let hits = intersect_curve_surface(&curve, &surface, 1e-9).unwrap();
    assert_eq!(hits.len(), 2);
    for h in &hits {
        assert!((h.point.x * h.point.x + h.point.y * h.point.y - 1.0).abs() < 1e-9);
        assert!((h.point.z - 1.0).abs() < 1e-9);
        assert!((h.v - 1.0).abs() < 1e-9);
    }
}

#[semio_framework_async_macros::async_test]
async fn line_through_cone() {
    let curve = Curve3::Line { origin: Pnt3::new(-5.0, 0.0, 2.0), dir: Vec3::X };
    let surface = Surface::Cone { frame: Frame3::WORLD, half_angle: std::f64::consts::FRAC_PI_4 };
    let hits = intersect_curve_surface(&curve, &surface, 1e-9).unwrap();
    assert_eq!(hits.len(), 2);
    for h in &hits {
        let on_surf = surface.eval(h.u, h.v);
        assert!(on_surf.distance(h.point) < 1e-8);
        assert!((h.point.z - 2.0).abs() < 1e-9);
    }
}

#[semio_framework_async_macros::async_test]
async fn circle_plane_off_axis_is_exact() {
    let circle = Curve3::Circle { frame: Frame3::WORLD, radius: 2.0 };
    let plane = Surface::Plane { frame: Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 1.0)).unwrap() };
    let hits = intersect_curve_surface(&circle, &plane, 1e-9).unwrap();
    assert_eq!(hits.len(), 2);
    for h in &hits {
        let on_curve = circle.eval(h.t);
        assert!(on_curve.distance(h.point) < 1e-8);
    }
}

#[semio_framework_async_macros::async_test]
async fn circle_sphere_exact() {
    let circle = Curve3::Circle { frame: Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap(), radius: 1.0 };
    let sphere = Surface::Sphere { frame: Frame3 { origin: Pnt3::new(0.5, 0.0, 0.0), ..Frame3::WORLD }, radius: 1.0 };
    let hits = intersect_curve_surface(&circle, &sphere, 1e-9).unwrap();
    assert!(!hits.is_empty());
    for h in &hits {
        let on_curve = circle.eval(h.t);
        let on_surf = sphere.eval(h.u, h.v);
        assert!(on_curve.distance(h.point) < 1e-8);
        assert!(on_surf.distance(h.point) < 1e-7);
    }
}

mod quick {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn parallel_line_misses_plane() {
        let curve = Curve3::Line { origin: Pnt3::new(0.0, 0.0, 1.0), dir: Vec3::X };
        let surface = Surface::Plane { frame: Frame3::WORLD };
        let hits = intersect_curve_surface(&curve, &surface, 1e-9).unwrap();
        assert!(hits.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn circle_coincident_with_plane_is_tangent_degenerate() {
        // The circle lies entirely *in* the plane (every point is an "intersection") — the
        // same degenerate convention `intersect_line_plane` already uses for an in-plane line.
        let curve = Curve3::Circle { frame: Frame3::WORLD, radius: 2.0 };
        let surface = Surface::Plane { frame: Frame3::WORLD };
        assert!(matches!(intersect_curve_surface(&curve, &surface, 1e-6), Err(IntersectError::Tangent)));
    }

    #[semio_framework_async_macros::async_test]
    async fn circle_plane_transversal_two_hits() {
        let curve = Curve3::Circle { frame: Frame3::WORLD, radius: 2.0 };
        let surface = Surface::Plane { frame: Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.3, 1.0)).unwrap() };
        let hits = intersect_curve_surface(&curve, &surface, 1e-6).unwrap();
        assert_eq!(hits.len(), 2);
        for h in &hits {
            let on_curve = curve.eval(h.t);
            assert!(on_curve.distance(h.point) < 1e-8);
            assert!(surface.eval(h.u, h.v).distance(h.point) < 1e-6);
        }
    }
}
