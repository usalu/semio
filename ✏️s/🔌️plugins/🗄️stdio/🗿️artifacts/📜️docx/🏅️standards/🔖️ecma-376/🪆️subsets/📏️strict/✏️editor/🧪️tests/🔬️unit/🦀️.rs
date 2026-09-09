use super::*;

#[semio_framework_async_macros::async_test]
async fn create_docx_strict_editor_builds_a_definition_for_the_editor_role() {
    let def = create_docx_strict_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, DOCX_STRICT_EDITOR_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<DocxStrictEditor as ArtifactEditor>::DIALECT, DOCX_STRICT_EDITOR_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_declares_the_document_window() {
    let def = create_docx_strict_editor();
    assert!(def.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
}

#[semio_framework_async_macros::async_test]
async fn set_page_replaces_a_paragraph_blocks_runs_with_a_single_plain_run() {
    let mut snapshot = DocxSnapshot::default();
    snapshot.document.body.push(DocxBlock::paragraph("hello"));
    let mutation = build_set_page_mutation(&snapshot, 0, "goodbye").expect("mutation");
    let DocxMutation::SetBlockContent(set_block_content::SetBlockContent { path, block }) = &mutation else { panic!("expected SetBlockContent") };
    assert_eq!(path.index, 0);
    let DocxBlock::Paragraph(paragraph) = block else { panic!("expected Paragraph") };
    assert_eq!(paragraph.runs.len(), 1);
    assert_eq!(paragraph.runs[0].text, "goodbye");
}

#[semio_framework_async_macros::async_test]
async fn set_page_on_a_table_block_is_a_documented_no_op() {
    let mut snapshot = DocxSnapshot::default();
    snapshot.document.body.push(DocxBlock::Table(crate::schema::snapshot::DocxTable::default()));
    assert!(build_set_page_mutation(&snapshot, 0, "text").is_none());
}

#[semio_framework_async_macros::async_test]
async fn op_text_roundtrip() {
    let command = DocxStrictEditorCommand::SetPage { index: 2, text: "a\nmulti line value".into() };
    let printed = <DocxStrictEditorCommand as protocol::OpText>::print_op(&command);
    let parsed = <DocxStrictEditorCommand as protocol::OpText>::parse_op(&printed).expect("parse ok");
    assert_eq!(parsed, command);
}
