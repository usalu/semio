use super::*;

#[semio_framework_async_macros::async_test]
async fn unweighted_bezier_eval_matches_de_casteljau_by_hand() {
    // Quadratic bezier: (0,0),(1,2),(2,0) at t=0.5 -> (1, 1)
    let b = RationalBezier2::unweighted(vec![Pnt2::new(0.0, 0.0), Pnt2::new(1.0, 2.0), Pnt2::new(2.0, 0.0)]);
    let p = b.eval(0.5);
    assert!((p.x - 1.0).abs() < 1e-12);
    assert!((p.y - 1.0).abs() < 1e-12);
}

#[semio_framework_async_macros::async_test]
async fn eval_at_endpoints_matches_first_and_last_control_point() {
    let b = RationalBezier3::unweighted(vec![Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 5.0, -2.0), Pnt3::new(3.0, 1.0, 4.0)]);
    assert_eq!(b.eval(0.0), b.controls[0]);
    assert_eq!(b.eval(1.0), *b.controls.last().unwrap());
}

#[semio_framework_async_macros::async_test]
async fn subdivide_matches_original_at_endpoints_and_split_point() {
    let b = RationalBezier3::unweighted(vec![Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 2.0, 0.0), Pnt3::new(2.0, -1.0, 1.0), Pnt3::new(3.0, 0.0, 2.0)]);
    let t = 0.35;
    let (left, right) = b.subdivide(t);
    assert!(left.eval(0.0).distance(b.eval(0.0)) < 1e-9);
    assert!(left.eval(1.0).distance(b.eval(t)) < 1e-9);
    assert!(right.eval(0.0).distance(b.eval(t)) < 1e-9);
    assert!(right.eval(1.0).distance(b.eval(1.0)) < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn subdivide_of_rational_bezier_preserves_curve_points() {
    // Quarter circle as a rational quadratic Bezier.
    let s = std::f64::consts::FRAC_1_SQRT_2;
    let b = RationalBezier2::new(vec![Pnt2::new(1.0, 0.0), Pnt2::new(1.0, 1.0), Pnt2::new(0.0, 1.0)], vec![1.0, s, 1.0]);
    let t = 0.3;
    let expected = b.eval(t);
    let (left, right) = b.subdivide(t);
    assert!((left.eval(1.0).x - expected.x).abs() < 1e-9);
    assert!((left.eval(1.0).y - expected.y).abs() < 1e-9);
    assert!((right.eval(0.0).x - expected.x).abs() < 1e-9);
    // All points on a quarter unit circle satisfy x^2+y^2=1.
    let sample = b.eval(0.6);
    assert!((sample.x * sample.x + sample.y * sample.y - 1.0).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn control_hull_box_contains_all_sampled_curve_points() {
    let b = RationalBezier3::unweighted(vec![Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(5.0, 5.0, 5.0), Pnt3::new(-3.0, 2.0, -1.0), Pnt3::new(1.0, -2.0, 3.0)]);
    let (lo, hi) = b.control_hull_box();
    for i in 0..=20 {
        let p = b.eval(i as f64 / 20.0);
        assert!(p.x >= lo.x - 1e-9 && p.x <= hi.x + 1e-9);
        assert!(p.y >= lo.y - 1e-9 && p.y <= hi.y + 1e-9);
        assert!(p.z >= lo.z - 1e-9 && p.z <= hi.z + 1e-9);
    }
}

#[semio_framework_async_macros::async_test]
async fn elevate_preserves_the_curve_exactly() {
    let b = RationalBezier3::unweighted(vec![Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 3.0, -1.0), Pnt3::new(2.0, -1.0, 2.0)]);
    let elevated = b.elevate();
    assert_eq!(elevated.degree(), b.degree() + 1);
    for i in 0..=10 {
        let t = i as f64 / 10.0;
        assert!(b.eval(t).distance(elevated.eval(t)) < 1e-9, "mismatch at t={t}");
    }
}

#[semio_framework_async_macros::async_test]
async fn subdivide_until_flat_leaves_cover_the_full_parameter_range() {
    let b = RationalBezier2::unweighted(vec![Pnt2::new(0.0, 0.0), Pnt2::new(1.0, 5.0), Pnt2::new(2.0, -3.0), Pnt2::new(3.0, 1.0)]);
    let leaves = subdivide_until_flat(&b, 0.1, 12);
    assert!(!leaves.is_empty());
    // Endpoints of the whole curve must be reproduced by the first/last leaf.
    assert!(leaves.first().unwrap().eval(0.0).distance(b.eval(0.0)) < 1e-9);
    assert!(leaves.last().unwrap().eval(1.0).distance(b.eval(1.0)) < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn boxes_overlap_detects_disjoint_and_touching_boxes() {
    let a = (Pnt2::new(0.0, 0.0), Pnt2::new(1.0, 1.0));
    let b = (Pnt2::new(0.5, 0.5), Pnt2::new(2.0, 2.0));
    let c = (Pnt2::new(5.0, 5.0), Pnt2::new(6.0, 6.0));
    assert!(boxes_overlap2(a, b, 1e-9));
    assert!(!boxes_overlap2(a, c, 1e-9));
}
