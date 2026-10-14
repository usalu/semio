use super::*;
use semio_framework_2d::PathSegment;

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).expect("path segment JSON fixture")
}

#[test]
fn path_segment_json_round_trips_every_variant_with_the_camel_case_wire_shape() {
    let fixture = fixture();
    let expected = vec![
        PathSegment::Move { to: [0.0, 1.5] },
        PathSegment::Line { to: [10.0, 1.5] },
        PathSegment::Quad { ctrl: [12.0, 4.0], to: [10.0, 8.0] },
        PathSegment::Cubic { ctrl1: [8.0, 9.0], ctrl2: [2.0, 9.0], to: [0.0, 8.0] },
        PathSegment::Arc { rx: 3.0, ry: 2.0, rotation: 0.25, large_arc: true, sweep: false, to: [0.0, 1.5] },
        PathSegment::Close,
    ];
    let decoded: Vec<PathSegment> = serde_json::from_value::<Vec<PathSegmentJson>>(fixture["segments"].clone()).expect("decode").into_iter().map(Into::into).collect();
    assert_eq!(decoded, expected);
    assert_eq!(serde_json::to_value(expected.iter().map(PathSegmentJson::from).collect::<Vec<_>>()).expect("encode"), fixture["segments"]);
}

#[test]
fn scene_node_path_and_clip_round_trip_through_the_conversion_type() {
    let fixture = fixture();
    for key in ["sceneNode", "sceneNodeWithoutClip"] {
        let node: SceneNode = serde_json::from_value(fixture[key].clone()).expect("decode scene node");
        assert_eq!(serde_json::to_value(&node).expect("encode scene node"), fixture[key], "{key}");
    }
    let clipped: SceneNode = serde_json::from_value(fixture["sceneNode"].clone()).unwrap();
    assert_eq!(clipped.clip.as_deref(), Some([PathSegment::Move { to: [0.0, 0.0] }, PathSegment::Line { to: [2.0, 2.0] }, PathSegment::Close].as_slice()));
}

#[test]
fn boolean_segments_json_accepts_and_answers_the_fixture_wire_shape() {
    let square = |x: f64| serde_json::json!({ "segments": [{ "kind": "move", "to": [x, 0.0] }, { "kind": "line", "to": [x + 4.0, 0.0] }, { "kind": "line", "to": [x + 4.0, 4.0] }, { "kind": "line", "to": [x, 4.0] }, { "kind": "close" }] }).to_string();
    let answer: serde_json::Value = serde_json::from_str(&boolean_segments_json(&square(0.0), &square(2.0), "union")).expect("boolean answer JSON");
    let segments = answer["segments"].as_array().expect("segments answer");
    assert!(!segments.is_empty());
    assert!(segments.iter().all(|segment| segment["kind"].is_string()), "{answer}");
}
