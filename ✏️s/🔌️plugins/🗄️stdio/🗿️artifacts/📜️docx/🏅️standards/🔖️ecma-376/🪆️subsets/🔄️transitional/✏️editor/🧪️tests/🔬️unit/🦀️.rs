use super::*;
use semio_framework_plugin::app::DocumentWindowKit;

#[semio_framework_async_macros::async_test]
async fn create_docx_transitional_editor_builds_a_definition_for_the_editor_role() {
    let def = create_docx_transitional_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, DOCX_TRANSITIONAL_EDITOR_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<DocxTransitionalEditor as ArtifactEditor>::DIALECT, DOCX_TRANSITIONAL_EDITOR_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_declares_the_document_window() {
    let def = create_docx_transitional_editor();
    assert!(def.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
}

#[semio_framework_async_macros::async_test]
async fn set_page_replaces_paragraph_text_through_an_addressed_revision() {
    let mut snapshot = DocxSnapshot::default();
    snapshot.document.body.push(DocxBlock::paragraph("hello"));
    let mutation = build_set_page_mutation(&snapshot, 0, 0, &DocumentWindowKit::text_revision("hello"), "goodbye").expect("valid edit").expect("changed edit");
    let DocxMutation::SetRunText(set_run_text::SetRunText { path, run_index, text }) = &mutation else { panic!("expected SetRunText") };
    assert_eq!(path.index, 0);
    assert_eq!(*run_index, 0);
    assert_eq!(text, "goodbye");
}

#[semio_framework_async_macros::async_test]
async fn set_page_rejects_a_non_text_target() {
    let mut snapshot = DocxSnapshot::default();
    snapshot.document.body.push(DocxBlock::Table(crate::schema::snapshot::DocxTable::default()));
    assert!(build_set_page_mutation(&snapshot, 0, 0, "", "text").is_err());
}

#[semio_framework_async_macros::async_test]
async fn op_text_roundtrip() {
    let command = DocxTransitionalEditorCommand::SetPage { page: 2, item: 0, revision: "0123456789abcdef".into(), text: "a\nmulti line value".into() };
    let printed = <DocxTransitionalEditorCommand as protocol::OpText>::print_op(&command);
    let parsed = <DocxTransitionalEditorCommand as protocol::OpText>::parse_op(&printed).expect("parse ok");
    assert_eq!(parsed, command);
}

#[semio_framework_async_macros::async_test]
async fn missing_set_page_payload_is_rejected() {
    assert!(<DocxTransitionalEditor as ArtifactEditor>::command_from_action("set-page", None).is_err());
}

#[semio_framework_async_macros::async_test]
async fn stale_set_page_revision_is_rejected() {
    let mut snapshot = DocxSnapshot::default();
    snapshot.document.body.push(DocxBlock::paragraph("current"));
    assert!(build_set_page_mutation(&snapshot, 0, 0, "0000000000000000", "draft").is_err());
}
