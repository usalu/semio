
use super::*;

#[test]
fn definition_declares_the_framework_mesh_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.id, "framework.window.mesh");
}

#[test]
fn render_produces_a_scene_node_for_the_default_document() {
    let document = Puzzle3dSnapshot::default();
    let _node = render(&document);
}

#[test]
fn instances_json_carries_one_entry_per_object() {
    let mut document = Puzzle3dSnapshot::default();
    document.objects.push(Puzzle3dObject {
        id: "o1".into(),
        label: None,
        object_kind: None,
        anchor: Default::default(),
        origin: [1.0, 2.0, 3.0],
        orientation: None,
        scale: Some(Puzzle3dScale::Uniform(2.0)),
        mesh_url: None,
        vortices: Vec::new(),
        hidden: false,
        locked: false,
    });
    let instances: serde_json::Value = serde_json::from_str(&puzzle3d_view_instances_json(&document)).expect("instances json");
    assert_eq!(instances.as_array().map(Vec::len), Some(1));
    assert_eq!(instances[0]["scale"], serde_json::json!([2.0, 2.0, 2.0]));
}
