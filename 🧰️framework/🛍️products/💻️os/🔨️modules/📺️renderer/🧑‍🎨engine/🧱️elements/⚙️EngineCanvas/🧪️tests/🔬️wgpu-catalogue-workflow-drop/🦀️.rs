
use super::*;

#[test]
fn puzzle3d_catalogue_drag_payload_parses_object_kind_and_mesh_url() {
    let (kind, mesh) = puzzle3d_catalogue_drag_payload_json(r#"{"objectKind":"Capsule","meshUrl":"puzzle3d://capsule"}"#).unwrap();
    assert_eq!(kind, "Capsule");
    assert_eq!(mesh.as_deref(), Some("puzzle3d://capsule"));
    assert!(puzzle3d_catalogue_drag_payload_json(r#"{"meshUrl":"puzzle3d://capsule"}"#).is_none());
}

#[test]
fn catalogue_ghost_prefers_label_then_app_id() {
    let with_label = catalogue_ghost_descriptor_json(r#"{"pluginId":"draw","appId":"draw","label":"Draw"}"#).unwrap();
    assert_eq!(serde_json::from_str::<Value>(&with_label).unwrap(), json!({ "kind": "neuron", "neuronKind": "Draw" }));
    let without_label = catalogue_ghost_descriptor_json(r#"{"pluginId":"draw","appId":"draw"}"#).unwrap();
    assert_eq!(serde_json::from_str::<Value>(&without_label).unwrap(), json!({ "kind": "neuron", "neuronKind": "draw" }));
}

#[test]
fn catalogue_ghost_rejects_incomplete_payloads() {
    assert!(catalogue_ghost_descriptor_json(r#"{"appId":"draw"}"#).is_none());
    assert!(catalogue_ghost_descriptor_json(r#"{"kind":"neuron"}"#).is_none());
    assert!(catalogue_ghost_descriptor_json("not-json").is_none());
}

#[test]
fn drag_ghost_descriptor_accepts_flow_widget_and_catalogue_mimes() {
    let mut flow = HashMap::new();
    flow.insert(FLOW_WIDGET_DRAG_MIME.into(), r#"{"kind":"inputSlider"}"#.into());
    assert_eq!(node_graph_drag_ghost_descriptor(&flow).as_deref(), Some(r#"{"kind":"inputSlider"}"#));
    let mut catalogue = HashMap::new();
    catalogue.insert(CATALOGUE_DRAG_MIME.into(), r#"{"pluginId":"draw","appId":"draw","label":"Draw"}"#.into());
    let ghost = node_graph_drag_ghost_descriptor(&catalogue).unwrap();
    assert_eq!(serde_json::from_str::<Value>(&ghost).unwrap(), json!({ "kind": "neuron", "neuronKind": "Draw" }));
    assert!(node_graph_drag_ghost_descriptor(&HashMap::new()).is_none());
}

#[test]
fn catalogue_drop_spawns_app_over_node_graph_bounds_with_surface_local_position() {
    let mut drag_data = HashMap::new();
    drag_data.insert(CATALOGUE_DRAG_MIME.into(), r#"{"pluginId":"draw","appId":"draw","label":"Draw"}"#.into());
    let bounds = Rect { x: 100.0, y: 50.0, w: 400.0, h: 300.0 };
    let action = node_graph_catalogue_drop_action(140.0, 90.0, &drag_data, &[("s.play.workflow", bounds, "s-play")]).expect("drop over workflow");
    assert_eq!(action.controller_id, "s-play");
    assert_eq!(action.action, "spawnApp");
    let args = action.args.unwrap();
    assert_eq!(args.get("pluginId").and_then(semio_framework::DslValue::as_str), Some("draw"));
    assert_eq!(args.get("appId").and_then(semio_framework::DslValue::as_str), Some("draw"));
    assert_eq!(args.get("position").and_then(|value| value.get("x")).and_then(semio_framework::DslValue::as_f64), Some(40.0));
    assert_eq!(args.get("position").and_then(|value| value.get("y")).and_then(semio_framework::DslValue::as_f64), Some(40.0));
}

#[test]
fn catalogue_drop_ignores_pointer_outside_node_graph_and_wrong_mime() {
    let bounds = Rect { x: 100.0, y: 50.0, w: 400.0, h: 300.0 };
    let mut catalogue = HashMap::new();
    catalogue.insert(CATALOGUE_DRAG_MIME.into(), r#"{"pluginId":"draw","appId":"draw"}"#.into());
    assert!(node_graph_catalogue_drop_action(10.0, 10.0, &catalogue, &[("s.play.workflow", bounds, "s-play")],).is_none());
    let mut flow = HashMap::new();
    flow.insert(FLOW_WIDGET_DRAG_MIME.into(), r#"{"kind":"inputSlider"}"#.into());
    assert!(node_graph_catalogue_drop_action(140.0, 90.0, &flow, &[("s.play.workflow", bounds, "s-play")],).is_none());
}
