use super::*;
use crate::editor::remodeling::unit_tests::context::{app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn a_document_without_a_report_renders_the_empty_state() {
    let mut app = app().await;
    assert!(render_body(&mut app, REMODELING_PLAY_BODY_QC).await.contains("No quality report yet"));
}
