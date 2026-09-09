use super::*;
use crate::editor::remodeling::testkit::{app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn definition_binds_the_framework_document_tab_to_this_body_key() {
    let definition = definition();
    assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_ARTIFACT_ID);
    assert_eq!(definition.body_key.as_deref(), Some(REMODELING_PLAY_BODY_PIPELINE));
}

#[semio_framework_async_macros::async_test]
async fn a_fresh_document_reports_an_idle_job() {
    let mut app = app().await;
    let body = render_body(&mut app, REMODELING_PLAY_BODY_PIPELINE).await;
    assert!(body.contains("Idle"), "a fresh document's job is idle: {body}");
}
