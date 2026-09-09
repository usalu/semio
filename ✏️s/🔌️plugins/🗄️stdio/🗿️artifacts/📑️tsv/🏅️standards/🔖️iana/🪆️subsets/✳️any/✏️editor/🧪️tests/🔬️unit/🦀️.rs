use super::*;

#[semio_framework_async_macros::async_test]
async fn create_tsv_editor_builds_a_definition_for_the_editor_role() {
    let def = create_tsv_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, TSV_EDITOR_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<TsvEditor as ArtifactEditor>::DIALECT, TSV_EDITOR_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_declares_the_table_window() {
    let def = create_tsv_editor();
    assert!(def.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
}

#[semio_framework_async_macros::async_test]
async fn op_text_roundtrip() {
    let command = TsvEditorCommand::SetCell { row: 2, column: 5, value: "a value".into() };
    let printed = <TsvEditorCommand as protocol::OpText>::print_op(&command);
    let parsed = <TsvEditorCommand as protocol::OpText>::parse_op(&printed).expect("parse ok");
    assert_eq!(parsed, command);
}
