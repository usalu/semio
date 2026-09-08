
use super::*;

#[semio_framework_async_macros::async_test]
async fn create_json_i_json_editor_builds_a_definition_for_the_editor_role() {
    let def = create_json_i_json_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, JSON_I_JSON_EDITOR_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<JsonIJsonEditor as ArtifactEditor>::DIALECT, JSON_I_JSON_EDITOR_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_declares_the_tree_window() {
    let def = create_json_i_json_editor();
    assert!(def.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
}

#[semio_framework_async_macros::async_test]
async fn decode_path_id_roundtrips_root_and_nested() {
    assert_eq!(decode_path_id("").unwrap(), Vec::<JsonPathSegment>::new());
    assert_eq!(decode_path_id("k=a/i=0").unwrap(), vec![JsonPathSegment::Key("a".into()), JsonPathSegment::Index(0)]);
    assert!(decode_path_id("bad").is_err());
}

#[semio_framework_async_macros::async_test]
async fn op_text_roundtrip() {
    let command = JsonIJsonIJsonEditorCommand::SetNode { node_id: "k=a/i=0".into(), value: "hello world".into() };
    let printed = <JsonIJsonIJsonEditorCommand as protocol::OpText>::print_op(&command);
    let parsed = <JsonIJsonIJsonEditorCommand as protocol::OpText>::parse_op(&printed).expect("parse ok");
    assert_eq!(parsed, command);
}
