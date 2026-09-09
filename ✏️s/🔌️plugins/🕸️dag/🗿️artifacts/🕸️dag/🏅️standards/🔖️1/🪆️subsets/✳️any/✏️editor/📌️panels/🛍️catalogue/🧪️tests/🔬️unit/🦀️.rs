use super::*;
use crate::editor::dag::testkit::{new_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn renders_every_node_kind() {
    let mut app = new_app().await;
    let json = render_body(&mut app, DAG_PLAY_BODY_CATALOGUE).await;
    for kind in ["computation", "slider", "select", "screen", "note", "preview"] {
        assert!(json.contains(kind), "catalogue must list the {kind} kind: {json}");
    }
}

#[semio_framework_async_macros::async_test]
async fn definition_binds_the_framework_catalogue_tab_to_this_body_key() {
    let definition = definition();
    assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_CATALOGUE_ID);
    assert_eq!(definition.body_key.as_deref(), Some(DAG_PLAY_BODY_CATALOGUE));
}
