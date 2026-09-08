
use super::*;
use serde_json::Value;

#[test]
fn world3d_scene_domain_id_round_trips_as_camel_case_and_omits_when_none() {
    let mut scene = World3dScene::base("{}".into(), "[]".into(), "[]".into(), "{}".into());
    scene.domain_id = Some("cad".into());
    scene.domain_granularity_id = Some("handle".into());
    let value = serde_json::to_value(&scene).expect("serialize");
    assert_eq!(value.get("domainId").and_then(Value::as_str), Some("cad"));
    assert_eq!(value.get("domainGranularityId").and_then(Value::as_str), Some("handle"));
    let back: World3dScene = serde_json::from_value(value).expect("deserialize");
    assert_eq!(back, scene);

    let bare = World3dScene::base("{}".into(), "[]".into(), "[]".into(), "{}".into());
    let bare_value = serde_json::to_value(&bare).expect("serialize");
    assert!(bare_value.get("domainId").is_none());
    assert!(bare_value.get("domainGranularityId").is_none());
}

#[test]
fn node_graph_hover_port_id_round_trips_as_camel_case_and_omits_when_none() {
    let hover = NodeGraphHover { node_id: Some("combine".into()), port_id: Some("b".into()) };
    let value = serde_json::to_value(&hover).expect("serialize");
    assert_eq!(value.get("nodeId").and_then(Value::as_str), Some("combine"));
    assert_eq!(value.get("portId").and_then(Value::as_str), Some("b"));
    let back: NodeGraphHover = serde_json::from_value(value).expect("deserialize");
    assert_eq!(back, hover);

    let bare = NodeGraphHover { node_id: Some("combine".into()), port_id: None };
    let bare_value = serde_json::to_value(&bare).expect("serialize");
    assert!(bare_value.get("portId").is_none());
}

#[test]
fn node_graph_scene_highlighted_round_trips_and_omits_when_empty() {
    let viewport = NodeGraphViewport { x: 0.0, y: 0.0, zoom: 1.0 };
    let mut scene = NodeGraphScene { highlighted: vec!["a".into(), "b@out".into()], ..NodeGraphScene::base(Vec::new(), Vec::new(), viewport.clone()) };
    let value = serde_json::to_value(&scene).expect("serialize");
    assert_eq!(value.get("highlighted").and_then(Value::as_array).map(Vec::len), Some(2));
    let back: NodeGraphScene = serde_json::from_value(value).expect("deserialize");
    assert_eq!(back, scene);

    scene.highlighted = Vec::new();
    let bare_value = serde_json::to_value(&scene).expect("serialize");
    assert!(bare_value.get("highlighted").is_none());
}
