use semio_framework_geometry::{Affine, Arc, BezPath, Circle, CubicBez, Line, PathEl, Point, Rect, RoundedRect, RoundedRectRadii, Vec2};

struct Lcg(u64);

impl Lcg {
    fn next_f64(&mut self, low: f64, high: f64) -> f64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let unit = (self.0 >> 11) as f64 / (1_u64 << 53) as f64;
        low + unit * (high - low)
    }
}

fn oracle_point(point: Point) -> kurbo::Point {
    kurbo::Point::new(point.x, point.y)
}

fn assert_point(ours: Point, oracle: kurbo::Point, epsilon: f64, context: &str) {
    let distance = ((ours.x - oracle.x).powi(2) + (ours.y - oracle.y).powi(2)).sqrt();
    assert!(distance <= epsilon, "{context}: ours={ours:?} oracle={oracle:?} distance={distance}");
}

fn assert_elements(ours: &[PathEl], oracle: &[kurbo::PathEl], context: &str) {
    assert_eq!(ours.len(), oracle.len(), "{context}: ours={ours:?} oracle={oracle:?}");
    for (index, (ours, oracle)) in ours.iter().zip(oracle).enumerate() {
        let context = format!("{context} element {index}");
        match (ours, oracle) {
            (PathEl::MoveTo(ours), kurbo::PathEl::MoveTo(oracle)) | (PathEl::LineTo(ours), kurbo::PathEl::LineTo(oracle)) => assert_point(*ours, *oracle, 1e-10, &context),
            (PathEl::QuadTo(ours_control, ours_point), kurbo::PathEl::QuadTo(oracle_control, oracle_point)) => {
                assert_point(*ours_control, *oracle_control, 1e-10, &context);
                assert_point(*ours_point, *oracle_point, 1e-10, &context);
            }
            (PathEl::CurveTo(ours_control1, ours_control2, ours_point), kurbo::PathEl::CurveTo(oracle_control1, oracle_control2, oracle_point)) => {
                assert_point(*ours_control1, *oracle_control1, 1e-10, &context);
                assert_point(*ours_control2, *oracle_control2, 1e-10, &context);
                assert_point(*ours_point, *oracle_point, 1e-10, &context);
            }
            (PathEl::ClosePath, kurbo::PathEl::ClosePath) => {}
            _ => panic!("{context}: ours={ours:?} oracle={oracle:?}"),
        }
    }
}

#[test]
fn value_arithmetic_matches_language_agnostic_fixtures_and_kurbo() {
    let fixtures = [(Point::new(1.0, 2.0), Vec2::new(3.0, 4.0), Point::new(4.0, 6.0), 5.0), (Point::new(-5.0, 8.0), Vec2::new(2.5, -6.0), Point::new(-2.5, 2.0), 6.5)];
    for (point, vector, expected_point, expected_length) in fixtures {
        assert_eq!(point + vector, expected_point);
        assert!((vector.hypot() - expected_length).abs() < 1e-12);
    }

    let mut rng = Lcg(0xA0761D6478BD642F);
    for case in 0..64 {
        let p0 = Point::new(rng.next_f64(-100.0, 100.0), rng.next_f64(-100.0, 100.0));
        let p1 = Point::new(rng.next_f64(-100.0, 100.0), rng.next_f64(-100.0, 100.0));
        let vector = Vec2::new(rng.next_f64(-20.0, 20.0), rng.next_f64(-20.0, 20.0));
        let oracle_p0 = oracle_point(p0);
        let oracle_p1 = oracle_point(p1);
        let oracle_vector = kurbo::Vec2::new(vector.x, vector.y);
        assert!((p0.distance(p1) - oracle_p0.distance(oracle_p1)).abs() < 1e-12, "case {case} distance");
        assert!((vector.dot(p1 - p0) - oracle_vector.dot(oracle_p1 - oracle_p0)).abs() < 1e-9, "case {case} dot");
        assert_point(p0 + vector, oracle_p0 + oracle_vector, 1e-12, "point plus vector");
        let ours = Rect::from_points(p0, p1).inflate(2.5, 1.25);
        let oracle = kurbo::Rect::from_points(oracle_p0, oracle_p1).inflate(2.5, 1.25);
        assert!((ours.x0() - oracle.x0).abs() < 1e-12 && (ours.y0() - oracle.y0).abs() < 1e-12 && (ours.x1() - oracle.x1).abs() < 1e-12 && (ours.y1() - oracle.y1).abs() < 1e-12, "case {case} rect");
    }
}

#[test]
fn affine_composition_and_application_match_kurbo() {
    let mut rng = Lcg(0xD1B54A32D192ED03);
    for case in 0..64 {
        let offset = Vec2::new(rng.next_f64(-50.0, 50.0), rng.next_f64(-50.0, 50.0));
        let angle = rng.next_f64(-std::f64::consts::PI, std::f64::consts::PI);
        let scale = rng.next_f64(0.1, 5.0);
        let ours = Affine::IDENTITY.translate(offset).rotate(angle).scale(scale);
        let oracle = kurbo::Affine::IDENTITY * kurbo::Affine::translate(kurbo::Vec2::new(offset.x, offset.y)) * kurbo::Affine::rotate(angle) * kurbo::Affine::scale(scale);
        for (index, (ours, oracle)) in ours.as_coeffs().into_iter().zip(oracle.as_coeffs()).enumerate() {
            assert!((ours - oracle).abs() < 1e-9, "case {case} coefficient {index}");
        }
        let point = Point::new(rng.next_f64(-100.0, 100.0), rng.next_f64(-100.0, 100.0));
        assert_point(ours * point, oracle * oracle_point(point), 1e-6, "affine point");
    }
}

#[test]
fn primitive_path_elements_match_kurbo() {
    let rect = Rect::new(-5.0, -3.0, 11.0, 17.0);
    let oracle_rect = kurbo::Rect::new(-5.0, -3.0, 11.0, 17.0);
    assert_elements(&rect.path_elements(0.1), &kurbo::Shape::path_elements(&oracle_rect, 0.1).collect::<Vec<_>>(), "rect");

    let line = Line::new(Point::new(1.0, 2.0), Point::new(3.0, 4.0));
    let oracle_line = kurbo::Line::new((1.0, 2.0), (3.0, 4.0));
    assert_elements(&line.path_elements(0.1), &kurbo::Shape::path_elements(&oracle_line, 0.1).collect::<Vec<_>>(), "line");

    let cubic = CubicBez::new(Point::new(-3.0, 2.0), Point::new(4.0, 11.0), Point::new(9.0, -7.0), Point::new(15.0, 5.0));
    let oracle_cubic = kurbo::CubicBez::new(oracle_point(cubic.p0()), oracle_point(cubic.p1()), oracle_point(cubic.p2()), oracle_point(cubic.p3()));
    assert_elements(&cubic.path_elements(0.1), &kurbo::Shape::path_elements(&oracle_cubic, 0.1).collect::<Vec<_>>(), "cubic");
    for t in [-1.0, -0.25, 0.0, 0.2, 0.5, 1.0, 1.5, 3.0] {
        assert_point(cubic.eval(t), kurbo::ParamCurve::eval(&oracle_cubic, t), 1e-10, "cubic eval");
    }
}

#[test]
fn adaptive_circle_arc_and_rounded_rect_paths_match_kurbo() {
    for (center, radius, tolerance) in [(Point::new(0.0, 0.0), 1.0, 0.1), (Point::new(12.5, -9.0), 75.0, 0.01), (Point::new(-4.0, 3.0), -12.0, 0.0001)] {
        let ours = Circle::new(center, radius).path_elements(tolerance);
        let oracle = kurbo::Shape::path_elements(&kurbo::Circle::new(oracle_point(center), radius), tolerance).collect::<Vec<_>>();
        assert_elements(&ours, &oracle, "circle");
    }

    for (center, radii, start, sweep, rotation, tolerance) in [
        (Point::new(0.0, 0.0), (20.0, 10.0), 0.0, std::f64::consts::FRAC_PI_2, 0.0, 0.1),
        (Point::new(3.0, -7.0), (40.0, 5.0), -1.25, 5.5, 0.4, 0.01),
        (Point::new(-8.0, 2.0), (7.0, 19.0), 2.0, -4.75, -0.7, 0.0005),
        (Point::new(5.0, 6.0), (8.0, 3.0), 1.0, 0.0, 0.2, 0.1),
    ] {
        let ours = Arc::new(center, radii, start, sweep, rotation);
        let oracle = kurbo::Arc::new(oracle_point(center), radii, start, sweep, rotation);
        assert_elements(&ours.path_elements(tolerance), &kurbo::Shape::path_elements(&oracle, tolerance).collect::<Vec<_>>(), "arc");
        for t in [0.0, 0.25, 0.5, 0.75, 1.0] {
            assert_point(ours.eval(t), kurbo::ParamCurve::eval(&oracle, t), 1e-10, "arc eval");
        }
    }

    for (rect, radii, oracle_radii, tolerance) in [
        (Rect::new(0.0, 0.0, 200.0, 100.0), RoundedRectRadii::new(10.0, 20.0, 30.0, 40.0), kurbo::RoundedRectRadii::new(10.0, 20.0, 30.0, 40.0), 0.1),
        (Rect::new(30.0, 20.0, -10.0, -40.0), RoundedRectRadii::new(-5.0, 100.0, 0.0, 11.0), kurbo::RoundedRectRadii::new(-5.0, 100.0, 0.0, 11.0), 0.01),
    ] {
        let ours = RoundedRect::new(rect, radii).path_elements(tolerance);
        let oracle = kurbo::RoundedRect::new(rect.x0(), rect.y0(), rect.x1(), rect.y1(), oracle_radii);
        assert_elements(&ours, &kurbo::Shape::path_elements(&oracle, tolerance).collect::<Vec<_>>(), "rounded rect");
    }
}

#[test]
fn bezpath_segments_transform_and_tight_bounds_match_kurbo() {
    let mut rng = Lcg(0x2545F4914F6CDD1D);
    for case in 0..32 {
        let mut ours = BezPath::new();
        let mut oracle = kurbo::BezPath::new();
        let start = (rng.next_f64(-50.0, 50.0), rng.next_f64(-50.0, 50.0));
        ours.move_to(start);
        oracle.move_to(start);
        for _ in 0..3 + case % 4 {
            let control1 = Point::new(rng.next_f64(-80.0, 80.0), rng.next_f64(-80.0, 80.0));
            let control2 = Point::new(rng.next_f64(-80.0, 80.0), rng.next_f64(-80.0, 80.0));
            let end = Point::new(rng.next_f64(-80.0, 80.0), rng.next_f64(-80.0, 80.0));
            ours.curve_to(control1, control2, end);
            oracle.curve_to(oracle_point(control1), oracle_point(control2), oracle_point(end));
        }
        let ours_bounds = ours.bounding_box();
        let oracle_bounds = kurbo::Shape::bounding_box(&oracle);
        assert!(
            (ours_bounds.x0() - oracle_bounds.x0).abs() < 1e-6 && (ours_bounds.y0() - oracle_bounds.y0).abs() < 1e-6 && (ours_bounds.x1() - oracle_bounds.x1).abs() < 1e-6 && (ours_bounds.y1() - oracle_bounds.y1).abs() < 1e-6,
            "case {case}: ours={ours_bounds:?} oracle={oracle_bounds:?}"
        );

        let transform = Affine::new([0.8, 0.2, -0.3, 1.1, 5.0, -7.0]);
        let transformed = ours.apply_affine(transform);
        assert_eq!(transformed.path_segments().len(), ours.path_segments().len());
        for (original, transformed) in ours.path_segments().iter().zip(transformed.path_segments()) {
            assert_eq!(transformed.start(), transform * original.start());
            assert_eq!(transformed.end(), transform * original.end());
        }
    }

    let empty = BezPath::new();
    assert_eq!(empty.bounding_box(), Rect::new(0.0, 0.0, 0.0, 0.0));
    let mut move_only = BezPath::new();
    move_only.move_to((25.0, -15.0));
    assert_eq!(move_only.bounding_box(), Rect::new(0.0, 0.0, 0.0, 0.0));
}
