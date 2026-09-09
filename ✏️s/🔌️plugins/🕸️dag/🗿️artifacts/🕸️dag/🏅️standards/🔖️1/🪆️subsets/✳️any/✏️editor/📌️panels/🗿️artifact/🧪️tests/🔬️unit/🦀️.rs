use super::*;
use crate::editor::dag::testkit::{new_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn dag_play_labels_resolve_native_by_default() {
    let mut app = new_app().await;
    let json = render_body(&mut app, DAG_PLAY_BODY_DOCUMENT).await;
    assert!(json.contains("Nodes"));
    assert!(json.contains("Edges"));
}

#[semio_framework_async_macros::async_test]
async fn definition_binds_the_framework_document_tab_to_this_body_key() {
    let definition = definition();
    assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_ARTIFACT_ID);
    assert_eq!(definition.body_key.as_deref(), Some(DAG_PLAY_BODY_DOCUMENT));
}
