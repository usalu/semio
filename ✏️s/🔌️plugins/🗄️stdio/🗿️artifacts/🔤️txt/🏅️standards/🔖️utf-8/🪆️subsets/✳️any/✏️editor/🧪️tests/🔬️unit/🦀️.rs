
use super::*;

#[semio_framework_async_macros::async_test]
async fn create_txt_editor_builds_a_definition_for_the_editor_role() {
    let def = create_txt_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, TXT_EDITOR_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<TxtEditor as ArtifactEditor>::DIALECT, TXT_EDITOR_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_declares_the_text_window() {
    let def = create_txt_editor();
    assert!(def.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
}

#[semio_framework_async_macros::async_test]
async fn split_text_detects_trailing_newline() {
    assert_eq!(split_text("a\nb\n"), (vec!["a".to_string(), "b".to_string()], true));
    assert_eq!(split_text("a\nb"), (vec!["a".to_string(), "b".to_string()], false));
    assert_eq!(split_text(""), (Vec::new(), false));
}

#[semio_framework_async_macros::async_test]
async fn op_text_roundtrip() {
    let command = TxtEditorCommand::ReplaceText { text: "hello\nworld".into() };
    let printed = <TxtEditorCommand as protocol::OpText>::print_op(&command);
    let parsed = <TxtEditorCommand as protocol::OpText>::parse_op(&printed).expect("parse ok");
    assert_eq!(parsed, command);
}
