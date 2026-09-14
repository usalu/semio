use super::*;
use crate::editor::remodeling::unit_tests::context::{app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn definition_nests_the_reconstruction_readout_and_the_framework_run_panel_under_the_document_tab() {
    let definition = definition();
    assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_ARTIFACT_ID);
    let bodies: Vec<Option<&str>> = definition.children.iter().map(|child| child.body_key.as_deref()).collect();
    assert_eq!(bodies, vec![Some(REMODELING_PLAY_BODY_PIPELINE), Some(FRAMEWORK_TOOL_RUN_BODY_KEY)]);
}

#[semio_framework_async_macros::async_test]
async fn a_fresh_document_reports_no_reconstruction_run_and_the_start_chord() {
    let mut app = app().await;
    let body = render_body(&mut app, REMODELING_PLAY_BODY_PIPELINE).await;
    assert!(body.contains("No reconstruction run"), "a fresh document has no run: {body}");
    assert!(body.contains(ToolRunAction::Start.chord()), "the start chord is rendered: {body}");
}
