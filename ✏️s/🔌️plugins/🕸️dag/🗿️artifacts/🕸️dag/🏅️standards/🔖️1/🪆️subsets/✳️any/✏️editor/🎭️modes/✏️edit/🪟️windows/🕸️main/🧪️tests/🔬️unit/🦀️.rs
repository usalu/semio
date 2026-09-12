use super::*;
use crate::editor::dag::unit_tests::context::{new_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn renders_node_graph_scene() {
    let mut app = new_app().await;
    let json = render_body(&mut app, DAG_PLAY_BODY_MAIN).await;
    assert!(json.contains("node-graph"));
}

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_node_graph_surface_and_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key, DAG_PLAY_BODY_MAIN);
    assert!(matches!(definition.surface_kind, SurfaceKind::NodeGraph));
    assert!(definition.options.measures.is_empty(), "dag's main window has no chrome measures");
}
