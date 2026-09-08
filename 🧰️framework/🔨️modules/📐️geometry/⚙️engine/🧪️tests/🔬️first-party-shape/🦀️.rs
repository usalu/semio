
use super::*;

/// 🎲️ Constant-seeded LCG (never the `rand` crate) — same idiom `path_seg_tests` uses.
struct Lcg(u64);

impl Lcg {
    fn next_f64(&mut self, lo: f64, hi: f64) -> f64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let bits = (self.0 >> 11) as f64 / (1u64 << 53) as f64;
        lo + bits * (hi - lo)
    }
}

fn oracle_point(point: Point) -> kurbo::Point {
    kurbo::Point::new(point.x, point.y)
}

fn assert_points_close(ours: Point, oracle: kurbo::Point, epsilon: f64, context: &str) {
    let d = ((ours.x - oracle.x).powi(2) + (ours.y - oracle.y).powi(2)).sqrt();
    assert!(d < epsilon, "{context}: ours={ours:?} oracle={oracle:?} dist={d}");
}

fn assert_path_elements_close(ours: &[PathEl], oracle: &[kurbo::PathEl], epsilon: f64, context: &str) {
    assert_eq!(ours.len(), oracle.len(), "{context}: ours={ours:?} oracle={oracle:?}");
    for (index, (ours, oracle)) in ours.iter().zip(oracle).enumerate() {
        let element_context = format!("{context} element {index}");
        match (ours, oracle) {
            (PathEl::MoveTo(ours), kurbo::PathEl::MoveTo(oracle)) | (PathEl::LineTo(ours), kurbo::PathEl::LineTo(oracle)) => {
                assert_points_close(*ours, *oracle, epsilon, &element_context);
            }
            (PathEl::QuadTo(ours_control, ours_point), kurbo::PathEl::QuadTo(oracle_control, oracle_point)) => {
                assert_points_close(*ours_control, *oracle_control, epsilon, &element_context);
                assert_points_close(*ours_point, *oracle_point, epsilon, &element_context);
            }
            (PathEl::CurveTo(ours_control1, ours_control2, ours_point), kurbo::PathEl::CurveTo(oracle_control1, oracle_control2, oracle_point)) => {
                assert_points_close(*ours_control1, *oracle_control1, epsilon, &element_context);
                assert_points_close(*ours_control2, *oracle_control2, epsilon, &element_context);
                assert_points_close(*ours_point, *oracle_point, epsilon, &element_context);
            }
            (PathEl::ClosePath, kurbo::PathEl::ClosePath) => {}
            _ => panic!("{element_context}: ours={ours:?} oracle={oracle:?}"),
        }
    }
}

#[test]
fn point_vec2_arithmetic_matches_hand_computation() {
    let p = Point::new(1.0, 2.0) + Vec2::new(3.0, 4.0);
    assert_eq!(p, Point::new(4.0, 6.0));
    let d = Point::new(4.0, 6.0) - Point::new(1.0, 2.0);
    assert_eq!(d, Vec2::new(3.0, 4.0));
    assert!((Vec2::new(3.0, 4.0).hypot() - 5.0).abs() < 1e-12);
}

#[test]
fn point_vec2_and_rect_arithmetic_agree_with_kurbo() {
    let mut rng = Lcg(0xA0761D6478BD642F);
    for case in 0..64 {
        let p0 = Point::new(rng.next_f64(-100.0, 100.0), rng.next_f64(-100.0, 100.0));
        let p1 = Point::new(rng.next_f64(-100.0, 100.0), rng.next_f64(-100.0, 100.0));
        let vector = Vec2::new(rng.next_f64(-20.0, 20.0), rng.next_f64(-20.0, 20.0));
        let oracle_p0 = oracle_point(p0);
        let oracle_p1 = oracle_point(p1);
        let oracle_vector = kurbo::Vec2::new(vector.x, vector.y);
        assert!((p0.distance(p1) - oracle_p0.distance(oracle_p1)).abs() < 1e-12, "case {case} point distance");
        assert!((vector.hypot() - oracle_vector.hypot()).abs() < 1e-12, "case {case} vector hypot");
        assert!((vector.dot(p1 - p0) - oracle_vector.dot(oracle_p1 - oracle_p0)).abs() < 1e-9, "case {case} vector dot");
        assert_points_close(p0 + vector, oracle_p0 + oracle_vector, 1e-12, "point plus vector");
        let ours = Rect::from_points(p0, p1).inflate(2.5, 1.25);
        let oracle = kurbo::Rect::from_points(oracle_p0, oracle_p1).inflate(2.5, 1.25);
        assert!((ours.x0() - oracle.x0).abs() < 1e-12 && (ours.y0() - oracle.y0).abs() < 1e-12 && (ours.x1() - oracle.x1).abs() < 1e-12 && (ours.y1() - oracle.y1).abs() < 1e-12, "case {case} rect: ours={ours:?} oracle={oracle:?}");
    }
}

/// 🔬️ DIFFERENTIAL ORACLE: `Affine::translate`/`rotate`/`scale`/composition/apply-to-point vs
/// the real `kurbo::Affine`, built independently from the same deterministic pseudo-random
/// parameters (not by converting one representation into the other), across 64 cases.
#[test]
fn affine_translate_rotate_scale_composition_and_apply_agree_with_kurbo() {
    let mut rng = Lcg(0xD1B54A32D192ED03);
    for i in 0..64 {
        let tx = rng.next_f64(-50.0, 50.0);
        let ty = rng.next_f64(-50.0, 50.0);
        let angle = rng.next_f64(-std::f64::consts::PI, std::f64::consts::PI);
        let scale = rng.next_f64(0.1, 5.0);
        let ours = Affine::IDENTITY.translate(Vec2::new(tx, ty)).rotate(angle).scale(scale);
        let oracle = kurbo::Affine::IDENTITY * kurbo::Affine::translate(kurbo::Vec2::new(tx, ty)) * kurbo::Affine::rotate(angle) * kurbo::Affine::scale(scale);
        let our_coeffs = ours.as_coeffs();
        let oracle_coeffs = oracle.as_coeffs();
        for k in 0..6 {
            assert!((our_coeffs[k] - oracle_coeffs[k]).abs() < 1e-9, "case {i} coeff {k}: ours={our_coeffs:?} oracle={oracle_coeffs:?}");
        }
        let p = Point::new(rng.next_f64(-100.0, 100.0), rng.next_f64(-100.0, 100.0));
        assert_points_close(ours * p, oracle * oracle_point(p), 1e-6, "affine apply-to-point");
    }
}

/// 🔬️ DIFFERENTIAL ORACLE: `BezPath::bounding_box`'s tight per-segment extrema box vs
/// `kurbo::Shape::bounding_box` on the same mixed line/quad/cubic path, across 32 randomly
/// generated multi-segment paths.
#[test]
fn bezpath_bounding_box_agrees_with_kurbo_shape_bounding_box_on_curved_paths() {
    let mut rng = Lcg(0x2545F4914F6CDD1D);
    for case in 0..32 {
        let mut ours = BezPath::new();
        let mut oracle = kurbo::BezPath::new();
        let start = (rng.next_f64(-50.0, 50.0), rng.next_f64(-50.0, 50.0));
        ours.move_to(start);
        oracle.move_to(start);
        let steps = 3 + (case % 4);
        for _ in 0..steps {
            match (rng.next_f64(0.0, 3.0) as u32).min(2) {
                0 => {
                    let p = (rng.next_f64(-50.0, 50.0), rng.next_f64(-50.0, 50.0));
                    ours.line_to(p);
                    oracle.line_to(p);
                }
                1 => {
                    let c = Point::new(rng.next_f64(-80.0, 80.0), rng.next_f64(-80.0, 80.0));
                    let p = Point::new(rng.next_f64(-80.0, 80.0), rng.next_f64(-80.0, 80.0));
                    ours.quad_to(c, p);
                    oracle.quad_to(oracle_point(c), oracle_point(p));
                }
                _ => {
                    let c1 = Point::new(rng.next_f64(-80.0, 80.0), rng.next_f64(-80.0, 80.0));
                    let c2 = Point::new(rng.next_f64(-80.0, 80.0), rng.next_f64(-80.0, 80.0));
                    let p = Point::new(rng.next_f64(-80.0, 80.0), rng.next_f64(-80.0, 80.0));
                    ours.curve_to(c1, c2, p);
                    oracle.curve_to(oracle_point(c1), oracle_point(c2), oracle_point(p));
                }
            }
        }
        let ours_bb = ours.bounding_box();
        let oracle_bb = kurbo::Shape::bounding_box(&oracle);
        assert!((ours_bb.x0() - oracle_bb.x0).abs() < 1e-6, "case {case} x0: ours={ours_bb:?} oracle={oracle_bb:?}");
        assert!((ours_bb.y0() - oracle_bb.y0).abs() < 1e-6, "case {case} y0: ours={ours_bb:?} oracle={oracle_bb:?}");
        assert!((ours_bb.x1() - oracle_bb.x1).abs() < 1e-6, "case {case} x1: ours={ours_bb:?} oracle={oracle_bb:?}");
        assert!((ours_bb.y1() - oracle_bb.y1).abs() < 1e-6, "case {case} y1: ours={ours_bb:?} oracle={oracle_bb:?}");
    }
}

#[test]
fn rect_line_cubic_path_elements_are_exact() {
    let rect = Rect::new(-5.0, -5.0, 15.0, 25.0);
    let els = rect.path_elements(0.1);
    assert_eq!(els.len(), 5);
    assert!(matches!(els[0], PathEl::MoveTo(_)));
    assert!(matches!(els[4], PathEl::ClosePath));

    let line = Line::new(Point::new(1.0, 2.0), Point::new(3.0, 4.0));
    assert_eq!(line.path_elements(0.1), vec![PathEl::MoveTo(Point::new(1.0, 2.0)), PathEl::LineTo(Point::new(3.0, 4.0))]);

    let cubic = CubicBez::new(Point::new(0.0, 0.0), Point::new(0.0, 10.0), Point::new(10.0, 10.0), Point::new(10.0, 0.0));
    assert_eq!(cubic.path_elements(0.1), vec![PathEl::MoveTo(Point::new(0.0, 0.0)), PathEl::CurveTo(Point::new(0.0, 10.0), Point::new(10.0, 10.0), Point::new(10.0, 0.0))]);
}

#[test]
fn cubic_eval_agrees_with_kurbo_inside_and_outside_unit_interval() {
    let ours = CubicBez::new(Point::new(-3.0, 2.0), Point::new(4.0, 11.0), Point::new(9.0, -7.0), Point::new(15.0, 5.0));
    let oracle = kurbo::CubicBez::new(oracle_point(ours.p0), oracle_point(ours.p1), oracle_point(ours.p2), oracle_point(ours.p3));
    for t in [-1.0, -0.25, 0.0, 0.2, 0.5, 1.0, 1.5, 3.0] {
        assert_points_close(ours.eval(t), kurbo::ParamCurve::eval(&oracle, t), 1e-10, "cubic eval");
    }
}

#[test]
fn circle_arc_and_rounded_rect_path_elements_agree_with_kurbo() {
    for (center, radius, tolerance) in [(Point::new(0.0, 0.0), 1.0, 0.1), (Point::new(12.5, -9.0), 75.0, 0.01), (Point::new(-4.0, 3.0), -12.0, 0.0001)] {
        let ours = Circle::new(center, radius).path_elements(tolerance);
        let oracle = kurbo::Shape::path_elements(&kurbo::Circle::new(oracle_point(center), radius), tolerance).collect::<Vec<_>>();
        assert_path_elements_close(&ours, &oracle, 1e-10, "circle");
    }

    for (center, radii, start, sweep, rotation, tolerance) in [
        (Point::new(0.0, 0.0), (20.0, 10.0), 0.0, std::f64::consts::FRAC_PI_2, 0.0, 0.1),
        (Point::new(3.0, -7.0), (40.0, 5.0), -1.25, 5.5, 0.4, 0.01),
        (Point::new(-8.0, 2.0), (7.0, 19.0), 2.0, -4.75, -0.7, 0.0005),
        (Point::new(5.0, 6.0), (8.0, 3.0), 1.0, 0.0, 0.2, 0.1),
    ] {
        let ours = Arc::new(center, radii, start, sweep, rotation).path_elements(tolerance);
        let oracle = kurbo::Shape::path_elements(&kurbo::Arc::new(oracle_point(center), radii, start, sweep, rotation), tolerance).collect::<Vec<_>>();
        assert_path_elements_close(&ours, &oracle, 1e-10, "arc");
    }

    for (rect, radii, tolerance) in [(Rect::new(0.0, 0.0, 200.0, 100.0), RoundedRectRadii::new(10.0, 20.0, 30.0, 40.0), 0.1), (Rect::new(30.0, 20.0, -10.0, -40.0), RoundedRectRadii::new(-5.0, 100.0, 0.0, 11.0), 0.01)] {
        let ours = RoundedRect::new(rect, radii).path_elements(tolerance);
        let oracle_radii = kurbo::RoundedRectRadii::new(radii.top_left, radii.top_right, radii.bottom_right, radii.bottom_left);
        let oracle = kurbo::Shape::path_elements(&kurbo::RoundedRect::new(rect.x0, rect.y0, rect.x1, rect.y1, oracle_radii), tolerance).collect::<Vec<_>>();
        assert_path_elements_close(&ours, &oracle, 1e-10, "rounded rect");
    }
}

#[test]
fn empty_or_move_only_bezpath_bounding_box_agrees_with_kurbo() {
    let empty = BezPath::new();
    assert_eq!(empty.bounding_box(), Rect::new(0.0, 0.0, 0.0, 0.0));
    let mut move_only = BezPath::new();
    move_only.move_to((25.0, -15.0));
    let ours = move_only.bounding_box();
    let mut oracle = kurbo::BezPath::new();
    oracle.move_to((25.0, -15.0));
    let oracle = kurbo::Shape::bounding_box(&oracle);
    assert!((ours.x0() - oracle.x0).abs() < 1e-12 && (ours.y0() - oracle.y0).abs() < 1e-12 && (ours.x1() - oracle.x1).abs() < 1e-12 && (ours.y1() - oracle.y1).abs() < 1e-12);
}

/// 🔬️ DIFFERENTIAL ORACLE: `Arc::eval` vs `kurbo::ParamCurve::eval(&kurbo::Arc, t)`, built
/// independently from the same deterministic pseudo-random center/radii/angles/rotation.
#[test]
fn arc_eval_agrees_with_kurbo_arc_eval() {
    let mut rng = Lcg(0x853C49E6748FEA9B);
    for _ in 0..32 {
        let center = Point::new(rng.next_f64(-50.0, 50.0), rng.next_f64(-50.0, 50.0));
        let radii = (rng.next_f64(1.0, 80.0), rng.next_f64(1.0, 80.0));
        let start_angle = rng.next_f64(-6.0, 6.0);
        let sweep = rng.next_f64(-6.0, 6.0);
        let x_rotation = rng.next_f64(-3.2, 3.2);
        let arc = Arc::new(center, radii, start_angle, sweep, x_rotation);
        let oracle_arc = kurbo::Arc::new(oracle_point(center), radii, start_angle, sweep, x_rotation);
        for k in 0..=8 {
            let t = k as f64 / 8.0;
            assert_points_close(arc.eval(t), kurbo::ParamCurve::eval(&oracle_arc, t), 1e-6, "arc eval");
        }
    }
}

/// 🔬️ SELF-CONSISTENCY: every point sampled along a flattened circle's cubic segments stays
/// within a generous multiple of `tolerance` of the analytic circle (`|distance_to_center -
/// radius|`) — proves [`elliptical_arc_segments`]' tolerance-driven segment count actually
/// honors the requested accuracy, independent of `kurbo`.
#[test]
fn circle_flattening_stays_within_tolerance_of_the_analytic_circle() {
    let mut rng = Lcg(0x9E3779B97F4A7C15);
    for _ in 0..16 {
        let center = Point::new(rng.next_f64(-50.0, 50.0), rng.next_f64(-50.0, 50.0));
        let radius = rng.next_f64(1.0, 100.0);
        let tolerance = rng.next_f64(0.001, 1.0);
        let circle = Circle::new(center, radius);
        let mut path = BezPath::new();
        append_shape_to_path(&mut path, &circle, tolerance);
        for seg in path.path_segments() {
            for k in 0..=8 {
                let t = k as f64 / 8.0;
                let p = seg.eval(t);
                let deviation = (distance_between(p, center) - radius).abs();
                assert!(deviation <= tolerance * 3.0 + radius * 1e-6, "deviation {deviation} exceeds slack for tolerance {tolerance} radius {radius}");
            }
        }
    }
}

/// 🔬️ DIFFERENTIAL ORACLE: a flattened circle's `BezPath::bounding_box` vs the analytic
/// `kurbo::Circle`'s own exact `bounding_box` (`center ± radius`), within `2 * tolerance`.
#[test]
fn circle_flattening_bounding_box_matches_kurbo_circle_bounding_box() {
    let center = Point::new(10.0, -5.0);
    let radius = 25.0;
    let tolerance = 0.01;
    let mut path = BezPath::new();
    append_shape_to_path(&mut path, &Circle::new(center, radius), tolerance);
    let bb = path.bounding_box();
    let oracle_bb = kurbo::Shape::bounding_box(&kurbo::Circle::new(oracle_point(center), radius));
    assert!((bb.x0() - oracle_bb.x0).abs() < tolerance * 2.0, "x0: ours={bb:?} oracle={oracle_bb:?}");
    assert!((bb.y0() - oracle_bb.y0).abs() < tolerance * 2.0, "y0: ours={bb:?} oracle={oracle_bb:?}");
    assert!((bb.x1() - oracle_bb.x1).abs() < tolerance * 2.0, "x1: ours={bb:?} oracle={oracle_bb:?}");
    assert!((bb.y1() - oracle_bb.y1).abs() < tolerance * 2.0, "y1: ours={bb:?} oracle={oracle_bb:?}");
}

/// 🔬️ SELF-CONSISTENCY + shape smoke test: a rounded rect's flattened outline is closed and
/// its bounding box matches the outer rect within a small slack.
#[test]
fn rounded_rect_flattening_is_closed_and_bounded_by_the_outer_rect() {
    let rect = Rect::new(0.0, 0.0, 200.0, 100.0);
    let radii = RoundedRectRadii::new(10.0, 20.0, 30.0, 0.0);
    let tolerance = 0.05;
    let mut path = BezPath::new();
    append_shape_to_path(&mut path, &RoundedRect::new(rect, radii), tolerance);
    assert!(matches!(path.elements().last(), Some(PathEl::ClosePath)));
    let bb = path.bounding_box();
    assert!(bb.x0() >= -1.0 && bb.x0() <= 1.0, "x0={}", bb.x0());
    assert!(bb.y0() >= -1.0 && bb.y0() <= 1.0, "y0={}", bb.y0());
    assert!(bb.x1() >= 199.0 && bb.x1() <= 201.0, "x1={}", bb.x1());
    assert!(bb.y1() >= 99.0 && bb.y1() <= 101.0, "y1={}", bb.y1());
}
