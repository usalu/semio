
use super::*;

#[semio_framework_async_macros::async_test]
async fn line_transformed_matches_mapped_eval() {
    let l = Curve3::Line { origin: Pnt3::new(1.0, 2.0, 3.0), dir: Vec3::new(2.0, -1.0, 0.5) };
    let map = Affine3::scaling(Pnt3::new(0.0, 0.0, 0.0), Vec3::new(2.0, 3.0, 5.0)).compose(&Affine3::translation(Vec3::new(1.0, 0.0, 0.0)));
    assert!(map.is_similarity().is_none(), "test fixture must actually be non-similarity");
    let transformed = l.transformed(&map);
    for i in 0..=5 {
        let t = i as f64 - 2.0;
        assert!(transformed.eval(t).distance(map.apply_point(l.eval(t))) < 1e-9, "line must stay exact under a non-similarity map at t={t}");
    }
}

#[semio_framework_async_macros::async_test]
async fn circle_transformed_stays_circle_under_similarity_and_matches_mapped_eval() {
    let frame = Frame3::from_normal(Pnt3::new(1.0, -1.0, 2.0), Vec3::new(0.3, 0.2, 1.0)).unwrap();
    let c = Curve3::Circle { frame, radius: 2.5 };
    let map = Affine3::rotation_about(Pnt3::new(0.5, 0.0, 0.0), Vec3::new(0.1, 1.0, 0.2), 0.7).compose(&Affine3::scaling(Pnt3::new(0.0, 0.0, 0.0), Vec3::new(3.0, 3.0, 3.0)));
    let (_, scale, _) = map.is_similarity().expect("rotation + uniform scale must be a similarity");
    let transformed = c.transformed(&map);
    assert!(matches!(transformed, Curve3::Circle { .. }), "a similarity must keep a circle a circle");
    if let Curve3::Circle { radius, .. } = transformed {
        assert!((radius - 2.5 * scale).abs() < 1e-9);
    }
    for i in 0..8 {
        let t = i as f64 * 0.8;
        assert!(transformed.eval(t).distance(map.apply_point(c.eval(t))) < 1e-7, "mismatch at t={t}");
    }
}

#[semio_framework_async_macros::async_test]
async fn circle_transformed_under_reflection_still_matches_mapped_eval() {
    let frame = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
    let c = Curve3::Circle { frame, radius: 4.0 };
    let map = Affine3::mirror(Pnt3::new(0.0, 0.0, 0.0), Vec3::X);
    let (_, _, is_reflection) = map.is_similarity().expect("a mirror must be a similarity");
    assert!(is_reflection);
    let transformed = c.transformed(&map);
    for i in 0..8 {
        let t = i as f64 * 0.8;
        assert!(transformed.eval(t).distance(map.apply_point(c.eval(t))) < 1e-9, "mismatch at t={t}");
    }
}

#[semio_framework_async_macros::async_test]
async fn circle_transformed_via_nurbs_under_non_similarity_matches_mapped_eval() {
    let frame = Frame3::WORLD;
    let c = Curve3::Circle { frame, radius: 1.0 };
    let map = Affine3::scaling(Pnt3::new(0.0, 0.0, 0.0), Vec3::new(2.0, 1.0, 1.0));
    assert!(map.is_similarity().is_none());
    let transformed = c.transformed(&map);
    assert!(matches!(transformed, Curve3::Nurbs { .. }), "non-similarity must force NURBS");
    for i in 0..=20 {
        let t = i as f64 / 20.0 * std::f64::consts::TAU;
        assert!(transformed.eval(t).distance(map.apply_point(c.eval(t))) < 1e-7, "mismatch at t={t}");
    }
}

#[semio_framework_async_macros::async_test]
async fn nurbs_transformed_matches_mapped_eval_and_keeps_weights() {
    let l = Curve3::Line { origin: Pnt3::new(0.0, 0.0, 0.0), dir: Vec3::new(1.0, 2.0, 3.0) };
    let base = l.to_nurbs((0.0, 1.0));
    let nurbs = Curve3::Nurbs { knots: base.knots, controls: base.controls, weights: vec![1.0, 2.0] };
    let map = Affine3::translation(Vec3::new(5.0, -1.0, 2.0));
    let transformed = nurbs.transformed(&map);
    if let (Curve3::Nurbs { weights: original_weights, .. }, Curve3::Nurbs { weights: transformed_weights, .. }) = (&nurbs, &transformed) {
        assert_eq!(original_weights, transformed_weights);
    } else {
        panic!("expected Nurbs variants");
    }
    for i in 0..=10 {
        let t = i as f64 / 10.0;
        assert!(transformed.eval(t).distance(map.apply_point(nurbs.eval(t))) < 1e-9, "mismatch at t={t}");
    }
}

#[semio_framework_async_macros::async_test]
async fn transformed_inverse_round_trips_a_circle() {
    let frame = Frame3::from_normal(Pnt3::new(2.0, 1.0, -1.0), Vec3::new(0.2, -0.4, 1.0)).unwrap();
    let c = Curve3::Circle { frame, radius: 3.3 };
    let map = Affine3::rotation_about(Pnt3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), 1.1).compose(&Affine3::translation(Vec3::new(2.0, -3.0, 1.0)));
    let inverse = map.inverse().unwrap();
    let round_trip = c.transformed(&map).transformed(&inverse);
    for i in 0..6 {
        let t = i as f64 * 1.0;
        assert!(round_trip.eval(t).distance(c.eval(t)) < 1e-7, "round trip drifted at t={t}");
    }
}
