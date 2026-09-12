use super::*;
use crate::editor::dag::unit_tests::context::{new_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn definition_binds_the_framework_inspection_tab_to_this_body_key() {
    let definition = definition();
    assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_INSPECTION_ID);
    assert_eq!(definition.body_key.as_deref(), Some(DAG_PLAY_BODY_INSPECTOR));
}

#[semio_framework_async_macros::async_test]
async fn renders_the_select_a_node_placeholder_when_nothing_is_selected() {
    let mut app = new_app().await;
    assert!(render_body(&mut app, DAG_PLAY_BODY_INSPECTOR).await.contains("Select a node"));
}

/// 🕹️ `render` carries no `InteractionView` (`DagPlayApp::render` always calls this panel's own
/// `render` with an empty selection now — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM,
/// matching `space`'s identical gap), so this exercises the panel's OWN rendering logic directly
/// with an explicit selection, the way `space`'s inspector test does, rather than driving it
/// end-to-end through a (now selection-blind) app dispatch.
#[semio_framework_async_macros::async_test]
async fn renders_id_name_and_kind_fields_for_a_single_selected_node() {
    let document = crate::default_snapshot();
    let node_id = document.nodes().first().map(|node| node.id.clone()).expect("node");
    let labels = crate::editor::dag::terminology::dag_play_labels(&semio_framework_plugin::ViewModel::default());
    let node = render(&document, &[node_id.clone()], labels).expect("inspector component tree");
    let json = serde_json::to_string(&node).unwrap();
    assert!(json.contains(&node_id));
    assert!(json.contains("Name") || json.contains("Kind"));
}
