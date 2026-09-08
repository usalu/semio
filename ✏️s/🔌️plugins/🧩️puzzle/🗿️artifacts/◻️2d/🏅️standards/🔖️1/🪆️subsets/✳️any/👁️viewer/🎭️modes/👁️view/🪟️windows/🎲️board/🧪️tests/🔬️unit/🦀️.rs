
use super::*;

#[test]
fn definition_declares_the_shared_mesh_window_kit() {
    let def = definition();
    assert_eq!(def.id, MeshWindowKit::KIND_ID);
}

#[test]
fn render_produces_a_scene_node_for_the_default_document() {
    let document = Puzzle2dSnapshot::default();
    let _node = render(&document);
}

#[test]
fn render_places_rectangle_and_circle_nodes_at_their_real_positions() {
    let mut document = Puzzle2dSnapshot::default();
    document.nodes.push(Puzzle2dNode { id: "n1".into(), shape: Some("circle".into()), x: 10.0, y: 20.0, radius: Some(5.0), ..Default::default() });
    document.nodes.push(Puzzle2dNode { id: "n2".into(), shape: Some("rectangle".into()), x: 30.0, y: 40.0, width: Some(12.0), height: Some(8.0), ..Default::default() });
    let json = world_instances_json(&document);
    assert!(json.contains("\"id\":\"n1\""));
    assert!(json.contains("\"meshId\":\"sphere\""));
    assert!(json.contains("\"id\":\"n2\""));
    assert!(json.contains("\"meshId\":\"box\""));
}
