use super::*;

#[test]
fn linear_curve_midpoint() {
    let c = PerformanceCurve::Linear { x1: 0.0, y1: 0.0, x2: 1.0, y2: 10.0 };
    assert!((c.evaluate(0.5) - 5.0).abs() < 1e-9);
}

#[test]
fn quadratic_curve_evaluates() {
    let c = PerformanceCurve::Quadratic { coeffs: [1.0, 2.0, 3.0] };
    assert!((c.evaluate(2.0) - 17.0).abs() < 1e-9);
}

#[test]
fn biquadratic_at_origin() {
    let c = PerformanceCurve::Biquadratic { coeffs: [1.0, 0.0, 0.0, 0.0, 0.0, 0.0] };
    assert!((c.evaluate_2d(2.0, 3.0) - 1.0).abs() < 1e-9);
}

#[test]
fn triquadratic_includes_cross_terms() {
    let c = [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0];
    assert!((triquadratic(c, 2.0, 3.0) - 6.0).abs() < 1e-9);
}

#[test]
fn lookup_wrapper_evaluates() {
    let table = CurveLookupTable2D::new("test", LookupTable2D { x: vec![0.0, 1.0], y: vec![0.0, 1.0], values: vec![vec![0.0, 10.0], vec![0.0, 20.0]] });
    let curve = PerformanceCurve::Table(table);
    assert!((curve.evaluate_2d(1.0, 1.0) - 20.0).abs() < 1e-9);
}

#[test]
fn validate_rejects_degenerate_linear() {
    let c = PerformanceCurve::Linear { x1: 1.0, y1: 0.0, x2: 1.0, y2: 5.0 };
    assert!(validate_curve(&c).is_err());
}

#[test]
fn validate_accepts_valid_quadratic() {
    let c = PerformanceCurve::Quadratic { coeffs: [0.5, 0.1, 0.0] };
    assert!(validate_curve(&c).is_ok());
}

#[test]
fn part_load_clamps() {
    let c = PerformanceCurve::Constant(1.0);
    assert!((c.part_load(150.0, 100.0) - 1.0).abs() < 1e-9);
}
