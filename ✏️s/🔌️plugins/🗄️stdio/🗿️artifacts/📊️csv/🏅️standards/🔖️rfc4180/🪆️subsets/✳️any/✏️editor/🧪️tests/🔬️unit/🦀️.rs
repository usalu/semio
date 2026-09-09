use super::*;

#[semio_framework_async_macros::async_test]
async fn create_csv_editor_builds_a_definition_for_the_editor_role() {
    let def = create_csv_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, CSV_EDITOR_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<CsvEditor as ArtifactEditor>::DIALECT, CSV_EDITOR_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_declares_the_table_window() {
    let def = create_csv_editor();
    assert!(def.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
}

#[semio_framework_async_macros::async_test]
async fn grid_row_offsets_by_one_when_has_header() {
    assert_eq!(grid_row_to_record_index(true, 0), 1);
    assert_eq!(grid_row_to_record_index(false, 0), 0);
    assert_eq!(grid_row_to_record_index(true, 3), 4);
}

#[semio_framework_async_macros::async_test]
async fn op_text_roundtrip() {
    let command = CsvEditorCommand::SetCell { row: 2, column: 5, value: "a value".into() };
    let printed = <CsvEditorCommand as protocol::OpText>::print_op(&command);
    let parsed = <CsvEditorCommand as protocol::OpText>::parse_op(&printed).expect("parse ok");
    assert_eq!(parsed, command);
}
