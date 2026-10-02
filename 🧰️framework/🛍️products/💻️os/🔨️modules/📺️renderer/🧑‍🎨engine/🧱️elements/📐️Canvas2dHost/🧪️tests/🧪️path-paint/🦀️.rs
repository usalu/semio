//! 🧫️ The shared Canvas2d path paint corpus (`🧫️fixtures/🧫️path-paint`) replayed through the Rust paint twin — the very
//! cases React's `🧪️tests/🧪️path-paint` replays through `drawSceneNode`: every probe shows the paint the corpus records
//! (stroke over fill over nothing). Plus the SVG path and canvas stroke rules the corpus cannot reach by probes.

use super::*;
use serde_json::Value;

const CORPUS: &str = include_str!("../../🧫️fixtures/🧫️path-paint/🔣️.json");

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CorpusLayer {
    segments: Vec<ScenePathSegment>,
    #[serde(default)]
    transform: Option<Vec<f64>>,
    #[serde(default)]
    fill: Option<Value>,
    #[serde(default)]
    stroke: Option<CorpusStroke>,
    #[serde(default)]
    fill_rule: Option<String>,
}

#[derive(Deserialize)]
struct CorpusStroke {
    width: f64,
    #[serde(default)]
    dash: Vec<f64>,
    #[serde(default)]
    cap: Option<String>,
    #[serde(default)]
    join: Option<String>,
}

fn number(value: &Value, key: &str) -> f64 {
    value[key].as_f64().unwrap_or_else(|| panic!("{key} is a number: {value}"))
}

/// 🔷️ Whether the convex `polygon` covers `point` (its boundary included); a polygon without area covers nothing.
fn covers(polygon: &[[f64; 2]], point: [f64; 2]) -> bool {
    let mut area = 0.0;
    let (mut positive, mut negative) = (false, false);
    for index in 0..polygon.len() {
        let (a, b) = (polygon[index], polygon[(index + 1) % polygon.len()]);
        area += a[0] * b[1] - b[0] * a[1];
        let cross = (b[0] - a[0]) * (point[1] - a[1]) - (b[1] - a[1]) * (point[0] - a[0]);
        positive |= cross > 1e-9;
        negative |= cross < -1e-9;
    }
    area.abs() > 1e-12 && !(positive && negative)
}

/// 🎨️ The paint `layer` shows at the screen `point` of a canvas `width × height` under `camera`.
fn paint_at(layer: &CorpusLayer, camera: &Value, width: f64, height: f64, point: [f64; 2]) -> &'static str {
    let map = Affine::node(layer.transform.as_deref()).then(Affine::camera(number(camera, "x"), number(camera, "y"), number(camera, "zoom"), 0.0, 0.0, width, height));
    let contours = flatten(&layer.segments, &map);
    if let Some(stroke) = &layer.stroke {
        let scale = map.line_scale();
        let geometry = StrokeGeometry { width: stroke.width * scale, cap: LineCap::of(stroke.cap.as_deref()), join: LineJoin::of(stroke.join.as_deref()), dash: stroke.dash.iter().map(|value| value * scale).collect() };
        if stroke_polygons(&contours, &geometry).iter().any(|polygon| covers(polygon, point)) {
            return "stroke";
        }
    }
    if layer.fill.is_some() && fill_trapezoids(&contours, FillRule::of(layer.fill_rule.as_deref()), (0.0, height)).iter().any(|trapezoid| covers(&trapezoid.corners(), point)) {
        return "fill";
    }
    "none"
}

fn segments(json: Value) -> Vec<ScenePathSegment> {
    serde_json::from_value(json).expect("segments decode")
}

fn identity_contours(json: Value) -> Vec<Contour> {
    flatten(&segments(json), &Affine::IDENTITY)
}

/// 📏️ The largest distance from a dense sampling of `exact` to the polyline `points`.
fn deviation(points: &[[f64; 2]], exact: impl Fn(f64) -> [f64; 2]) -> f64 {
    (0..=4096)
        .map(|index| exact(index as f64 / 4096.0))
        .map(|sample| {
            points
                .windows(2)
                .map(|edge| {
                    let span = sub(edge[1], edge[0]);
                    let length = span[0] * span[0] + span[1] * span[1];
                    let t = if length > 0.0 { (((sample[0] - edge[0][0]) * span[0] + (sample[1] - edge[0][1]) * span[1]) / length).clamp(0.0, 1.0) } else { 0.0 };
                    let nearest = add(edge[0], scale(span, t));
                    (sample[0] - nearest[0]).hypot(sample[1] - nearest[1])
                })
                .fold(f64::INFINITY, f64::min)
        })
        .fold(0.0, f64::max)
}

#[test]
fn every_corpus_probe_shows_the_recorded_paint() {
    let corpus: Value = serde_json::from_str(CORPUS).expect("corpus parses");
    let mut mismatches = Vec::new();
    let mut probes = 0;
    for case in corpus["cases"].as_array().expect("cases") {
        let layer: CorpusLayer = serde_json::from_value(case["layer"].clone()).expect("layer decodes");
        let (width, height) = (number(&case["viewport"], "width"), number(&case["viewport"], "height"));
        for probe in case["probes"].as_array().expect("probes") {
            let point = [probe["at"][0].as_f64().expect("x"), probe["at"][1].as_f64().expect("y")];
            let painted = paint_at(&layer, &case["camera"], width, height, point);
            probes += 1;
            if painted != probe["paint"].as_str().expect("paint") {
                mismatches.push(format!("{} {:?}: painted {painted}, recorded {} ({})", case["name"], point, probe["paint"], probe["reason"]));
            }
        }
    }
    assert!(mismatches.is_empty(), "{} of {probes} probes disagree:\n{}", mismatches.len(), mismatches.join("\n"));
    assert_eq!(probes, 68, "the corpus probe count moved; re-run the corpus author");
}

#[test]
fn a_path_that_does_not_start_with_a_move_paints_nothing() {
    let contours = identity_contours(serde_json::json!([{ "kind": "line", "to": [10, 0] }, { "kind": "line", "to": [10, 10] }, { "kind": "close" }]));
    assert!(contours.is_empty());
    let truncated = identity_contours(serde_json::json!([{ "kind": "move", "to": [0, 0] }, { "kind": "line", "to": [10, 0] }, { "kind": "quad", "to": [10, 10] }, { "kind": "line", "to": [0, 10] }]));
    assert_eq!(truncated, vec![Contour { points: vec![[0.0, 0.0], [10.0, 0.0]], closed: false }], "a segment missing its control point ends the path");
}

#[test]
fn unknown_kinds_are_skipped_and_a_segment_after_close_restarts_at_the_subpath_start() {
    let contours = identity_contours(serde_json::json!([{ "kind": "move", "to": [0, 0] }, { "kind": "line", "to": [4, 0] }, { "kind": "bogus", "to": [99, 99] }, { "kind": "line", "to": [4, 4] }, { "kind": "close" }, { "kind": "line", "to": [-4, 0] }]));
    assert_eq!(contours, vec![Contour { points: vec![[0.0, 0.0], [4.0, 0.0], [4.0, 4.0]], closed: true }, Contour { points: vec![[0.0, 0.0], [-4.0, 0.0]], closed: false }]);
}

#[test]
fn curves_and_arcs_stay_within_the_flatness_on_screen() {
    let map = Affine([1.5, 0.75, -0.5, 2.0, 7.0, -3.0]).then(Affine::camera(0.0, 0.0, 3.0, 0.0, 0.0, 100.0, 100.0));
    let quad_points = flatten(&segments(serde_json::json!([{ "kind": "move", "to": [0, 0] }, { "kind": "quad", "ctrl": [20, 30], "to": [40, 0] }])), &map);
    let quad_exact = |t: f64| map.apply([2.0 * (1.0 - t) * t * 20.0 + t * t * 40.0, 2.0 * (1.0 - t) * t * 30.0]);
    assert!(deviation(&quad_points[0].points, quad_exact) <= FLATNESS * 1.01);
    let cubic_points = flatten(&segments(serde_json::json!([{ "kind": "move", "to": [0, 0] }, { "kind": "cubic", "ctrl1": [10, 40], "ctrl2": [30, -40], "to": [40, 0] }])), &map);
    let cubic_exact = |t: f64| {
        let u = 1.0 - t;
        map.apply([3.0 * u * u * t * 10.0 + 3.0 * u * t * t * 30.0 + t * t * t * 40.0, 3.0 * u * u * t * 40.0 - 3.0 * u * t * t * 40.0])
    };
    assert!(deviation(&cubic_points[0].points, cubic_exact) <= FLATNESS * 1.01);
    let arc_points = flatten(&segments(serde_json::json!([{ "kind": "move", "to": [10, 0] }, { "kind": "arc", "rx": 10, "ry": 10, "rotation": 0, "largeArc": false, "sweep": true, "to": [0, 10] }])), &map);
    let arc_exact = |t: f64| map.apply([10.0 * (t * FRAC_PI_2).cos(), 10.0 * (t * FRAC_PI_2).sin()]);
    assert!(deviation(&arc_points[0].points, arc_exact) <= FLATNESS * 1.01);
    let midpoint = map.apply([50f64.sqrt(), 50f64.sqrt()]);
    assert!(arc_points[0].points.iter().any(|point| (point[0] - midpoint[0]).abs() < 1.0 && (point[1] - midpoint[1]).abs() < 1.0), "the arc bends through its circular midpoint");
}

#[test]
fn canvas_dash_and_miter_rules_hold() {
    assert_eq!(dash_pattern(&[]), None);
    assert_eq!(dash_pattern(&[4.0, -1.0]), None);
    assert_eq!(dash_pattern(&[0.0, 0.0]), None);
    assert_eq!(dash_pattern(&[3.0]), Some(vec![3.0, 3.0]));
    let style = |join| StrokeGeometry { width: 2.0, cap: LineCap::Butt, join, dash: Vec::new() };
    let corner = |end: [f64; 2], join| stroke_polygons(&[Contour { points: vec![[0.0, 0.0], [10.0, 0.0], end], closed: false }], &style(join));
    let square_corner = corner([10.0, 10.0], LineJoin::Miter);
    assert_eq!(square_corner.len(), 3, "two edge quads and one join");
    assert_eq!(square_corner[2].len(), 4, "a right angle keeps its miter");
    let hairpin = corner([0.0, 0.5], LineJoin::Miter);
    assert_eq!(hairpin[2].len(), 3, "a turn sharper than the miter limit bevels");
    assert_eq!(corner([10.0, 10.0], LineJoin::Bevel)[2].len(), 3);
    let round = &stroke_polygons(&[Contour { points: vec![[0.0, 0.0], [100.0, 0.0], [100.0, 100.0]], closed: false }], &StrokeGeometry { width: 20.0, cap: LineCap::Butt, join: LineJoin::Round, dash: Vec::new() })[2];
    assert!(round.len() > 4 && round[1..].iter().all(|point| ((point[0] - 100.0).hypot(point[1]) - 10.0).abs() < 1e-9), "a round join fans the outer corner on the half-width circle: {round:?}");
    let capped = stroke_polygons(&[Contour { points: vec![[0.0, 0.0], [10.0, 0.0]], closed: false }], &StrokeGeometry { width: 2.0, cap: LineCap::Square, join: LineJoin::Miter, dash: Vec::new() });
    assert_eq!(capped.len(), 3, "one edge and two square caps");
    assert!(capped.iter().any(|polygon| covers(polygon, [-0.5, 0.0])) && !capped.iter().any(|polygon| covers(polygon, [-1.5, 0.0])));
}

#[test]
fn gradient_pieces_tile_their_trapezoid() {
    let trapezoid = Trapezoid { top: 0.0, bottom: 10.0, top_left: 0.0, top_right: 20.0, bottom_left: -5.0, bottom_right: 30.0 };
    let pieces = trapezoid.pieces(4.0);
    assert!(pieces.iter().all(|piece| piece.bottom - piece.top <= 4.0 + 1e-9 && (piece.top_right - piece.top_left).max(piece.bottom_right - piece.bottom_left) <= 4.0 + 1e-9));
    let area = |t: &Trapezoid| ((t.top_right - t.top_left) + (t.bottom_right - t.bottom_left)) * 0.5 * (t.bottom - t.top);
    assert!((pieces.iter().map(area).sum::<f64>() - area(&trapezoid)).abs() < 1e-9);
}
