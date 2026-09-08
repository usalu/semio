
use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_node_graph_surface_and_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key, BODY_KEY);
    assert!(matches!(definition.surface_kind, SurfaceKind::NodeGraph));
    assert!(definition.options.measures.is_empty(), "dag's viewer main window has no chrome measures");
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_read_only_node_graph_scene() {
    let document = crate::default_snapshot();
    let node = render(&document).expect("viewer surface");
    let semio_framework_plugin::plugin_app_close_prelude::Component::Surface(props) = node.component else { panic!("viewer must produce a surface") };
    let scene: NodeGraphScene = semio_framework_ui_scene::decode(&props).expect("node graph scene");
    assert_eq!(scene.editable, Some(false));
    let (nodes, edges) = document_to_workflow(&document);
    assert_eq!(scene.nodes, nodes);
    assert_eq!(scene.edges, edges);
}
