//! 🧪️ Shared projection and hidden-line fixtures for the native geometry owner.

use super::{frustum::clip_line, linework::LineGraph, visibility::{Occluders, LineStep}};

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧪️tests/🧫️fixtures/🕶️line-visibility/🔣️.json")).expect("projection and line visibility fixture")
}

fn point(value: &serde_json::Value) -> [f32; 3] {
    std::array::from_fn(|axis| value[axis].as_f64().expect("fixture coordinate") as f32)
}

#[test]
fn clip_edge_to_frustum_before_divide() {
    for case in fixture()["frustum"].as_array().unwrap() {
        let parse = |value: &serde_json::Value| {
            std::array::from_fn(|endpoint| std::array::from_fn(|axis| value[endpoint][axis].as_f64().unwrap() as f32))
        };
        let expected = (!case["clipped"].is_null()).then(|| parse(&case["clipped"]));
        assert_eq!(clip_line(parse(&case["input"])), expected, "{}", case["id"]);
    }
}

#[test]
fn classify_and_coalesce_visible_and_hidden_spans() {
    let fixture = fixture();
    for case in fixture["cases"].as_array().unwrap() {
        let mut occluders = Occluders::new((fixture["width"].as_u64().unwrap() as u32, fixture["height"].as_u64().unwrap() as u32));
        for triangle in case["triangles"].as_array().unwrap() {
            occluders.insert(std::array::from_fn(|index| point(&triangle[index])));
        }
        let points = std::array::from_fn(|index| point(&case["line"][index]));
        let mut line = occluders.line(points);
        let mut spans = [Vec::new(), Vec::new()];
        for _ in 0..100 {
            match line.next(&occluders) {
                LineStep::Segment { points: segment, hidden } => {
                    spans[usize::from(hidden)].push(segment.map(|point| (point[0] - points[0][0]) / (points[1][0] - points[0][0])));
                }
                LineStep::Pending => {}
                LineStep::Complete => break,
            }
        }
        for (actual, kind) in spans.iter().zip(["visible", "hidden"]) {
            let expected = case[kind].as_array().unwrap();
            assert_eq!(actual.len(), expected.len(), "{} {kind}", case["id"]);
            for (actual, expected) in actual.iter().zip(expected) {
                for endpoint in 0..2 {
                    assert!((f64::from(actual[endpoint]) - expected[endpoint].as_f64().unwrap()).abs() < 1e-6, "{} {kind}", case["id"]);
                }
            }
        }
        while !occluders.close_step() {}
    }
}

#[test]
fn assemble_projected_linework_through_corners_cycles_and_junctions() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).expect("projected linework fixture");
    for case in fixture["linework"].as_array().unwrap() {
        let mut graph = LineGraph::default();
        for segment in case["segments"].as_array().unwrap() {
            let points = std::array::from_fn(|end| [segment[end][0].as_f64().unwrap() as f32, segment[end][1].as_f64().unwrap() as f32, 0.0]);
            graph.insert(points);
        }
        let merge = case["merge"].as_bool().unwrap();
        let radius = case["easing"].as_f64().unwrap() as f32;
        let mut path = String::new();
        while let Some(part) = graph.next(merge, radius) {
            path.push_str(&part);
        }
        assert!(graph.is_empty());
        assert!(!graph.remove_one());
        if let Some(expected) = case["path"].as_str() {
            assert_eq!(path, expected, "{}", case["name"]);
        } else {
            assert_eq!(path.matches('M').count() as u64, case["moves"].as_u64().unwrap(), "{}", case["name"]);
            assert_eq!(path.matches('Q').count() as u64, case["curves"].as_u64().unwrap(), "{}", case["name"]);
        }
    }
}
