use super::*;
use crate::editor::procedure::unit_tests::context::{imperative_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn document_lists_steps() {
    let mut app = imperative_app().await;
    assert!(render_body(&mut app, IMPERATIVE_PLAY_BODY_DOCUMENT).await.contains("imperative-play-document.steps"));
}

#[semio_framework_async_macros::async_test]
async fn definition_binds_the_framework_document_tab_to_this_body_key() {
    let definition = definition();
    assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_ARTIFACT_ID);
    assert_eq!(definition.body_key.as_deref(), Some(IMPERATIVE_PLAY_BODY_DOCUMENT));
}
