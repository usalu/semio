
use super::*;
use crate::standards::v1::subsets::brep::schema::snapshot::curve::Curve2;
use crate::standards::v1::subsets::brep::schema::snapshot::vector::{Pnt2, Vec2};

#[semio_framework_async_macros::async_test]
async fn plane_transformed_matches_mapped_eval_under_non_similarity() {
    let frame = Frame3::from_normal(Pnt3::new(1.0, -1.0, 2.0), Vec3::Z).unwrap();
    let s = Surface::Plane { frame };
    let map = Affine3::scaling(Pnt3::new(0.0, 0.0, 0.0), Vec3::new(2.0, 3.0, 5.0)).compose(&Affine3::translation(Vec3::new(1.0, 0.0, 0.0)));
    assert!(map.is_similarity().is_none());
    let transformed = s.transformed(&map);
    for (u, v) in [(0.0, 0.0), (1.0, 2.0), (-3.0, 4.0)] {
        assert!(transformed.eval(u, v).distance(map.apply_point(s.eval(u, v))) < 1e-9, "mismatch at {u},{v}");
    }
}

#[semio_framework_async_macros::async_test]
async fn cylinder_transformed_stays_cylinder_under_similarity() {
    let frame = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
    let s = Surface::Cylinder { frame, radius: 2.0 };
    let map = Affine3::rotation_about(Pnt3::new(1.0, 0.0, 0.0), Vec3::new(0.2, 1.0, 0.1), 0.6).compose(&Affine3::scaling(Pnt3::new(0.0, 0.0, 0.0), Vec3::new(2.0, 2.0, 2.0)));
    let (_, scale, _) = map.is_similarity().expect("must be a similarity");
    let transformed = s.transformed(&map);
    assert!(matches!(transformed, Surface::Cylinder { .. }));
    if let Surface::Cylinder { radius, .. } = transformed {
        assert!((radius - 2.0 * scale).abs() < 1e-9);
    }
    for (u, v) in [(0.3, 1.0), (2.0, -3.0), (5.0, 0.5)] {
        assert!(transformed.eval(u, v).distance(map.apply_point(s.eval(u, v))) < 1e-7, "mismatch at {u},{v}");
    }
}

#[semio_framework_async_macros::async_test]
async fn sphere_transformed_via_nurbs_under_non_similarity_matches_mapped_eval() {
    let frame = Frame3::WORLD;
    let s = Surface::Sphere { frame, radius: 3.0 };
    let map = Affine3::scaling(Pnt3::new(0.0, 0.0, 0.0), Vec3::new(2.0, 1.0, 1.0));
    assert!(map.is_similarity().is_none());
    let transformed = s.transformed(&map);
    assert!(matches!(transformed, Surface::Nurbs { .. }), "non-similarity must force NURBS");
    for (u, v) in [(0.0, 0.0), (1.0, 0.3), (4.0, -0.5), (0.5, 1.5)] {
        assert!(transformed.eval(u, v).distance(map.apply_point(s.eval(u, v))) < 1e-6, "mismatch at {u},{v}");
    }
}

#[semio_framework_async_macros::async_test]
async fn torus_transformed_via_nurbs_under_non_similarity_matches_mapped_eval() {
    let frame = Frame3::WORLD;
    let s = Surface::Torus { frame, major_radius: 4.0, minor_radius: 1.0 };
    let map = Affine3::mirror(Pnt3::new(0.0, 0.0, 0.0), Vec3::X).compose(&Affine3::scaling(Pnt3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 2.0, 1.0)));
    assert!(map.is_similarity().is_none());
    let transformed = s.transformed(&map);
    for (u, v) in [(0.3, 0.7), (2.0, 4.0), (5.5, 1.2)] {
        assert!(transformed.eval(u, v).distance(map.apply_point(s.eval(u, v))) < 1e-6, "mismatch at {u},{v}");
    }
}

#[semio_framework_async_macros::async_test]
async fn cone_transformed_via_nurbs_under_non_similarity_matches_mapped_eval_near_apex() {
    let frame = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
    let s = Surface::Cone { frame, half_angle: std::f64::consts::FRAC_PI_6 };
    let map = Affine3::scaling(Pnt3::new(0.0, 0.0, 0.0), Vec3::new(3.0, 1.0, 1.0));
    assert!(map.is_similarity().is_none());
    let transformed = s.transformed(&map);
    for (u, v) in [(0.5, 2.0), (3.0, 5.0)] {
        assert!(transformed.eval(u, v).distance(map.apply_point(s.eval(u, v))) < 1e-6, "mismatch at {u},{v}");
    }
}

/// 🗺️ The invariant this ticket's transform work is built around: a p-curve is a curve in the
/// FACE's own `(u, v)` parameter space, so it must not need any change when the surface it
/// trims is transformed by the same map — evaluating `surface(pcurve(t))` before and after
/// [`Surface::transformed`] and mapping the "before" result must agree with the "after" result,
/// for the SAME unmodified `pcurve`.
#[semio_framework_async_macros::async_test]
async fn pcurve_stays_unchanged_when_surface_is_transformed_by_the_same_map() {
    let frame = Frame3::from_normal(Pnt3::new(1.0, 2.0, -1.0), Vec3::new(0.1, 0.2, 1.0)).unwrap();
    let s = Surface::Cylinder { frame, radius: 2.0 };
    let pcurve = Curve2::Line { origin: Pnt2::new(0.2, -1.0), dir: Vec2::new(0.6, 2.0) };
    let map = Affine3::rotation_about(Pnt3::new(0.0, 1.0, 0.0), Vec3::new(0.3, 1.0, -0.2), 1.0).compose(&Affine3::scaling(Pnt3::new(0.0, 0.0, 0.0), Vec3::new(1.5, 1.5, 1.5)));
    let transformed_surface = s.transformed(&map);
    for t in [0.0, 0.3, 0.7, 1.2] {
        let uv = pcurve.eval(t);
        let before = map.apply_point(s.eval(uv.x, uv.y));
        let after = transformed_surface.eval(uv.x, uv.y);
        assert!(before.distance(after) < 1e-7, "p-curve point at t={t} diverged after transform");
    }
}
