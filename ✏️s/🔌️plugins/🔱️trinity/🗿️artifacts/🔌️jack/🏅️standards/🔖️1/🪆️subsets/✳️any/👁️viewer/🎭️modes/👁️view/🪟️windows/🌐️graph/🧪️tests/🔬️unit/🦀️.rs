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
    let node = render(&document, crate::jack_content_for_handle(&document.content).expect("retained content child").snapshot()).expect("node graph surface");
    assert!(semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("project semantic UI test tree").contains("node-graph"));
}
