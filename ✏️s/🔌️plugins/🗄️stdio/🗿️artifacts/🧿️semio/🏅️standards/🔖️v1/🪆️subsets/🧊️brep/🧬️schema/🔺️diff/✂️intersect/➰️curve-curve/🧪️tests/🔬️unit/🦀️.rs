
use super::*;

#[semio_framework_async_macros::async_test]
async fn perpendicular_lines_meet_at_origin() {
    let a = Curve3::Line { origin: Pnt3::new(-1.0, 0.0, 0.0), dir: Vec3::new(1.0, 0.0, 0.0) };
    let b = Curve3::Line { origin: Pnt3::new(0.0, -1.0, 0.0), dir: Vec3::new(0.0, 1.0, 0.0) };
    let hits = intersect_curve_curve(&a, &b, 1e-9).unwrap();
    assert_eq!(hits.len(), 1);
    assert!(hits[0].point.distance(Pnt3::new(0.0, 0.0, 0.0)) < 1e-9);
    assert!((hits[0].t_a - 1.0).abs() < 1e-9);
    assert!((hits[0].t_b - 1.0).abs() < 1e-9);
    assert!(!hits[0].tangent);
}

#[semio_framework_async_macros::async_test]
async fn unit_circle_with_diameter_line() {
    let circle = Curve3::Circle { frame: Frame3::WORLD, radius: 1.0 };
    let line = Curve3::Line { origin: Pnt3::new(0.0, 0.0, 0.0), dir: Vec3::new(1.0, 0.0, 0.0) };
    let hits = intersect_curve_curve(&circle, &line, 1e-9).unwrap();
    assert_eq!(hits.len(), 2);
    let mut xs: Vec<f64> = hits.iter().map(|h| h.point.x).collect();
    xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert!((xs[0] + 1.0).abs() < 1e-9);
    assert!((xs[1] - 1.0).abs() < 1e-9);
    for h in &hits {
        assert!(h.point.y.abs() < 1e-9);
        assert!(h.point.z.abs() < 1e-9);
        assert!((h.point.x * h.point.x + h.point.y * h.point.y - 1.0).abs() < 1e-9);
        assert!(!h.tangent);
    }
}

#[semio_framework_async_macros::async_test]
async fn line_circle_order_preserves_parameters() {
    let circle = Curve3::Circle { frame: Frame3::WORLD, radius: 1.0 };
    let line = Curve3::Line { origin: Pnt3::new(-2.0, 0.0, 0.0), dir: Vec3::new(1.0, 0.0, 0.0) };
    let hits = intersect_curve_curve(&line, &circle, 1e-9).unwrap();
    assert_eq!(hits.len(), 2);
    for h in &hits {
        let on_line = line.eval(h.t_a);
        let on_circle = circle.eval(h.t_b);
        assert!(on_line.distance(h.point) < 1e-8);
        assert!(on_circle.distance(h.point) < 1e-8);
    }
}

#[semio_framework_async_macros::async_test]
async fn line_tangent_to_circle_reports_single_tangent_hit() {
    let circle = Curve3::Circle { frame: Frame3::WORLD, radius: 1.0 };
    let line = Curve3::Line { origin: Pnt3::new(-2.0, 1.0, 0.0), dir: Vec3::X };
    let hits = intersect_curve_curve(&line, &circle, 1e-7).unwrap();
    assert_eq!(hits.len(), 1);
    assert!(hits[0].tangent);
    assert!(hits[0].point.distance(Pnt3::new(0.0, 1.0, 0.0)) < 1e-6);
}

#[semio_framework_async_macros::async_test]
async fn externally_tangent_circles_report_single_tangent_hit() {
    let a = Curve3::Circle { frame: Frame3::WORLD, radius: 1.0 };
    let b = Curve3::Circle { frame: Frame3 { origin: Pnt3::new(3.0, 0.0, 0.0), ..Frame3::WORLD }, radius: 2.0 };
    let hits = intersect_curve_curve(&a, &b, 1e-7).unwrap();
    assert_eq!(hits.len(), 1);
    assert!(hits[0].tangent);
    assert!(hits[0].point.distance(Pnt3::new(1.0, 0.0, 0.0)) < 1e-6);
}

mod quick {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn skew_lines_do_not_intersect() {
        let a = Curve3::Line { origin: Pnt3::new(0.0, 0.0, 0.0), dir: Vec3::X };
        let b = Curve3::Line { origin: Pnt3::new(0.0, 1.0, 1.0), dir: Vec3::Y };
        let hits = intersect_curve_curve(&a, &b, 1e-9).unwrap();
        assert!(hits.is_empty());
    }
}
