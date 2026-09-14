use super::*;

/// 🌳️ Every node of a built tree as one debug text, since `BuiltChildren` debug prints only its length.
fn described(node: &BuiltNode) -> String {
    node.children.iter().fold(format!("{node:?}"), |text, child| text + &described(child))
}

#[semio_framework_async_macros::async_test]
async fn definition_nests_the_reconstruction_readout_and_the_framework_run_panel_under_the_document_tab() {
    let definition = definition();
    assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_ARTIFACT_ID);
    let bodies: Vec<Option<&str>> = definition.children.iter().map(|child| child.body_key.as_deref()).collect();
    assert_eq!(bodies, vec![Some(REMODELING_PLAY_BODY_PIPELINE), Some(FRAMEWORK_TOOL_RUN_BODY_KEY)]);
}

#[semio_framework_async_macros::async_test]
async fn a_fresh_document_reports_no_reconstruction_run_and_the_start_chord() {
    let body = described(&render(&crate::default_remodeling_scene(), None, Locale::En).expect("the readout builds"));
    assert!(body.contains("No reconstruction run"), "a fresh document has no run: {body}");
    assert!(body.contains(ToolRunAction::Start.chord()), "the start chord is rendered: {body}");
    let german = described(&render(&crate::default_remodeling_scene(), None, Locale::De).expect("the readout builds"));
    assert!(german.contains("Kein Rekonstruktionslauf"), "the readout is localized: {german}");
}
