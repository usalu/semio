
use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_a_node_graph_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.surface_kind, semio_framework_plugin::SurfaceKind::NodeGraph);
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_scene_node_for_the_default_document() {
    let document = crate::empty_trinity_graph_fixture();
    let node = render(&document).expect("node graph surface");
    assert!(serde_json::to_string(&node).expect("serialize semantic UI test tree").contains("node-graph"));
}
