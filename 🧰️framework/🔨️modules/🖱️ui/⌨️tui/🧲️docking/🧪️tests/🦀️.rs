use super::{DockPlacement, WindowStackCorner};
use crate::{geometry::{Pos, Rect}, signal::WidgetSignal};
use semio_framework_pack_json::{parse, JsonMemberPolicy, Value};

fn placement(value: &str) -> DockPlacement {
    match value {
        "tabs" => DockPlacement::Tabs, "left" => DockPlacement::Left, "right" => DockPlacement::Right,
        "top" => DockPlacement::Top, "bottom" => DockPlacement::Bottom,
        "top-left" => DockPlacement::Corner(WindowStackCorner::TopLeft), "top-right" => DockPlacement::Corner(WindowStackCorner::TopRight),
        "bottom-left" => DockPlacement::Corner(WindowStackCorner::BottomLeft), "bottom-right" => DockPlacement::Corner(WindowStackCorner::BottomRight),
        other => panic!("unknown docking placement {other}"),
    }
}
fn numbers(value: &Value) -> Vec<u16> { value.as_array().unwrap().iter().map(|v| v.as_u64().unwrap() as u16).collect() }
fn area(value: &Value) -> Rect { let n = numbers(value); Rect::new(n[0], n[1], n[2], n[3]) }

#[test]
fn shared_docking_contract_matches_neutral_vectors_without_frame_dependency() {
    assert!(include_str!("🥒️.feature").starts_with("@capability-tui-docking-contract"));
    let corpus = parse(include_str!("🔣️.json"), JsonMemberPolicy::Reject).unwrap();
    for row in corpus.get("previews").unwrap().as_array().unwrap() {
        let choice = placement(row.get("placement").unwrap().as_str().unwrap());
        assert_eq!(choice.preview(area(row.get("area").unwrap())), area(row.get("expected").unwrap()));
        let committed = WidgetSignal::WindowDocked { source: "source".into(), target: "target".into(), placement: choice, tab_index: Some(1) };
        let proposed = WidgetSignal::WindowDockPreview { source: "source".into(), target: "target".into(), placement: choice, tab_index: Some(1) };
        assert!(matches!(committed, WidgetSignal::WindowDocked { placement, .. } if placement == choice));
        assert!(matches!(proposed, WidgetSignal::WindowDockPreview { placement, .. } if placement == choice));
    }
    for row in corpus.get("hits").unwrap().as_array().unwrap() {
        let point = numbers(row.get("point").unwrap());
        let expected = row.get("expected").unwrap().as_str().map(placement);
        assert_eq!(DockPlacement::at(area(row.get("area").unwrap()), Pos { x: point[0], y: point[1] }), expected);
    }
}
