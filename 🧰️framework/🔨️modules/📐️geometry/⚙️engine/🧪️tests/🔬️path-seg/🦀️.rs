
use super::*;

/// 🎲️ Constant-seeded LCG (never the `rand` crate) — deterministic pseudo-random control
/// points for the differential oracle test below, same idiom the `ticket`'s dev-dependency
/// rule asks for.
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

/// 🧪️ Language-agnostic fixture table: (segment, t, expected point) for [`PathSeg::eval`],
/// hand-computed from the Bernstein-basis formulas so any language's own De Casteljau
/// implementation can be checked against the same table.
#[test]
fn eval_matches_hand_computed_fixtures() {
    let fixtures: &[(PathSeg, f64, Point)] = &[
        (PathSeg::Line(Point::new(0.0, 0.0), Point::new(10.0, 0.0)), 0.5, Point::new(5.0, 0.0)),
        (PathSeg::Line(Point::new(0.0, 0.0), Point::new(10.0, 20.0)), 0.25, Point::new(2.5, 5.0)),
        (PathSeg::Quad(Point::new(0.0, 0.0), Point::new(5.0, 10.0), Point::new(10.0, 0.0)), 0.5, Point::new(5.0, 5.0)),
        (PathSeg::Cubic(Point::new(0.0, 0.0), Point::new(0.0, 10.0), Point::new(10.0, 10.0), Point::new(10.0, 0.0)), 0.5, Point::new(5.0, 7.5)),
    ];
    for (seg, t, expected) in fixtures {
        let actual = seg.eval(*t);
        assert!(distance_between(actual, *expected) < 1e-9, "eval({t}) on {seg:?} expected {expected:?}, got {actual:?}");
    }
}

#[test]
fn start_and_end_match_endpoints_for_every_variant() {
    let line = PathSeg::Line(Point::new(0.0, 0.0), Point::new(1.0, 1.0));
    let quad = PathSeg::Quad(Point::new(0.0, 0.0), Point::new(1.0, 2.0), Point::new(2.0, 0.0));
    let cubic = PathSeg::Cubic(Point::new(0.0, 0.0), Point::new(1.0, 1.0), Point::new(2.0, 1.0), Point::new(3.0, 0.0));
    for seg in [line, quad, cubic] {
        assert_eq!(seg.eval(0.0), seg.start());
        assert!(distance_between(seg.eval(1.0), seg.end()) < 1e-9);
    }
}

#[test]
fn subdivide_at_endpoints_join_at_the_split_point_and_reproduce_original_endpoints() {
    let cubic = PathSeg::Cubic(Point::new(0.0, 0.0), Point::new(0.0, 10.0), Point::new(10.0, 10.0), Point::new(10.0, 0.0));
    for t in [0.1, 0.3, 0.5, 0.7, 0.9] {
        let (left, right) = cubic.subdivide_at(t);
        assert!(distance_between(left.start(), cubic.start()) < 1e-9);
        assert!(distance_between(right.end(), cubic.end()) < 1e-9);
        assert!(distance_between(left.end(), right.start()) < 1e-9, "split halves must join exactly at t={t}");
        assert!(distance_between(left.end(), cubic.eval(t)) < 1e-9, "split point must equal eval(t) at t={t}");
    }
}

#[test]
fn subsegment_full_range_is_identity_and_half_ranges_sum_to_whole_arclen() {
    let cubic = PathSeg::Cubic(Point::new(0.0, 0.0), Point::new(0.0, 10.0), Point::new(10.0, 10.0), Point::new(10.0, 0.0));
    let whole = cubic.subsegment(0.0, 1.0);
    assert!(distance_between(whole.start(), cubic.start()) < 1e-9);
    assert!(distance_between(whole.end(), cubic.end()) < 1e-9);
    let first_half = cubic.subsegment(0.0, 0.5);
    let second_half = cubic.subsegment(0.5, 1.0);
    let whole_len = cubic.arclen(1e-6);
    let split_len = first_half.arclen(1e-6) + second_half.arclen(1e-6);
    assert!((whole_len - split_len).abs() < 1e-4, "arclen of the two halves ({split_len}) must sum to the whole ({whole_len})");
}

#[test]
fn line_arclen_is_exact() {
    let line = PathSeg::Line(Point::new(0.0, 0.0), Point::new(3.0, 4.0));
    assert!((line.arclen(1e-9) - 5.0).abs() < 1e-12);
}

/// 🧪️ A cubic Bezier built from the standard `kappa ≈ 0.5522847498` construction approximates
/// a quarter circle of radius `r` to within a few parts in 10,000 — a well-known analytic
/// cross-check independent of `kurbo`.
#[test]
fn cubic_quarter_circle_approximation_matches_analytic_arc_length() {
    let r = 10.0;
    let k = 0.5522847498307936 * r;
    let seg = PathSeg::Cubic(Point::new(r, 0.0), Point::new(r, k), Point::new(k, r), Point::new(0.0, r));
    let analytic = std::f64::consts::FRAC_PI_2 * r;
    let estimated = seg.arclen(1e-6);
    assert!((estimated - analytic).abs() < analytic * 0.001, "quarter-circle cubic approximation arclen {estimated} should be within 0.1% of the analytic {analytic}");
}

/// 🔬️ DIFFERENTIAL ORACLE: our from-scratch [`PathSeg::arclen`] (recursive control-polygon
/// subdivision) vs `kurbo::ParamCurveArclen::arclen` (the crate's own adaptive Gauss-Legendre
/// quadrature) on 32 deterministic pseudo-random cubic/quad curves. Both estimators converge
/// to the true arc length as `accuracy` shrinks but via unrelated numerical methods, so
/// agreement is real evidence of correctness, not shared-bug coincidence. Tolerance is
/// relative (`0.5%` of the oracle's own length) rather than absolute, since curve sizes here
/// range from ~1 to ~200 units — an absolute epsilon would be either too loose on small
/// curves or too tight on large ones.
#[test]
fn arclen_agrees_with_kurbo_param_curve_arclen_across_random_curves() {
    let mut rng = Lcg(0x9E3779B97F4A7C15);
    let accuracy = 1e-4;
    for i in 0..32 {
        let p0 = Point::new(rng.next_f64(-50.0, 50.0), rng.next_f64(-50.0, 50.0));
        let p1 = Point::new(rng.next_f64(-50.0, 50.0), rng.next_f64(-50.0, 50.0));
        let p2 = Point::new(rng.next_f64(-50.0, 50.0), rng.next_f64(-50.0, 50.0));
        if i % 2 == 0 {
            let seg = PathSeg::Quad(p0, p1, p2);
            let oracle = kurbo::ParamCurveArclen::arclen(&kurbo::QuadBez::new(oracle_point(p0), oracle_point(p1), oracle_point(p2)), accuracy);
            let ours = seg.arclen(accuracy);
            assert!((ours - oracle).abs() <= oracle.max(1.0) * 0.005, "quad #{i}: ours={ours} oracle={oracle}");
        } else {
            let p3 = Point::new(rng.next_f64(-50.0, 50.0), rng.next_f64(-50.0, 50.0));
            let seg = PathSeg::Cubic(p0, p1, p2, p3);
            let oracle = kurbo::ParamCurveArclen::arclen(&kurbo::CubicBez::new(oracle_point(p0), oracle_point(p1), oracle_point(p2), oracle_point(p3)), accuracy);
            let ours = seg.arclen(accuracy);
            assert!((ours - oracle).abs() <= oracle.max(1.0) * 0.005, "cubic #{i}: ours={ours} oracle={oracle}");
        }
    }
}

#[test]
fn path_segments_walks_a_multi_subpath_document_and_closes_each_subpath() {
    let mut path = BezPath::new();
    path.move_to((0.0, 0.0));
    path.line_to((10.0, 0.0));
    path.line_to((10.0, 10.0));
    path.close_path();
    path.move_to((20.0, 20.0));
    path.curve_to(Point::new(20.0, 30.0), Point::new(30.0, 30.0), Point::new(30.0, 20.0));
    let segments = path.path_segments();
    assert_eq!(segments.len(), 4, "2 lines + 1 implicit close-line + 1 cubic");
    assert!(matches!(segments[0], PathSeg::Line(..)));
    assert!(matches!(segments[1], PathSeg::Line(..)));
    assert!(matches!(segments[2], PathSeg::Line(..)), "ClosePath must become an implicit closing Line");
    assert!(matches!(segments[3], PathSeg::Cubic(..)));
    assert!(distance_between(segments[2].end(), Point::new(0.0, 0.0)) < 1e-9, "the implicit close must land back on the subpath's MoveTo");
}

#[test]
fn path_segments_skips_a_redundant_close_when_already_at_the_start_point() {
    let mut path = BezPath::new();
    path.move_to((0.0, 0.0));
    path.line_to((10.0, 0.0));
    path.line_to((0.0, 0.0));
    path.close_path();
    let segments = path.path_segments();
    assert_eq!(segments.len(), 2, "a ClosePath that is already back at the start must not add a zero-length segment");
}

#[test]
fn apply_affine_translates_every_point_including_control_points() {
    let mut path = BezPath::new();
    path.move_to((0.0, 0.0));
    path.curve_to(Point::new(0.0, 10.0), Point::new(10.0, 10.0), Point::new(10.0, 0.0));
    let moved = path.apply_affine(Affine::IDENTITY.translate(Vec2::new(5.0, 5.0)));
    let original_segments = path.path_segments();
    let moved_segments = moved.path_segments();
    let PathSeg::Cubic(_, oc1, oc2, oend) = original_segments[0] else { panic!("expected a cubic segment") };
    let PathSeg::Cubic(_, mc1, mc2, mend) = moved_segments[0] else { panic!("expected a cubic segment") };
    assert!(distance_between(mc1, oc1 + Vec2::new(5.0, 5.0)) < 1e-9, "control point 1 must be translated");
    assert!(distance_between(mc2, oc2 + Vec2::new(5.0, 5.0)) < 1e-9, "control point 2 must be translated");
    assert!(distance_between(mend, oend + Vec2::new(5.0, 5.0)) < 1e-9, "endpoint must be translated");
}

#[test]
fn as_path_el_round_trips_through_a_fresh_bezpath() {
    let mut path = BezPath::new();
    path.move_to((0.0, 0.0));
    path.line_to((10.0, 0.0));
    path.curve_to(Point::new(10.0, 10.0), Point::new(20.0, 10.0), Point::new(20.0, 0.0));
    let segments = path.path_segments();
    let mut rebuilt = BezPath::new();
    rebuilt.move_to(segments[0].start());
    for seg in &segments {
        rebuilt.push(seg.as_path_el());
    }
    assert_eq!(rebuilt.elements(), path.elements());
}
