use super::*;

#[semio_framework_async_macros::async_test]
async fn create_xml_editor_builds_a_definition_for_the_editor_role() {
    let def = create_xml_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, XML_EDITOR_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<XmlAnyEditor as ArtifactEditor>::DIALECT, XML_EDITOR_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_declares_the_tree_window() {
    let def = create_xml_editor();
    assert!(def.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
}

#[semio_framework_async_macros::async_test]
async fn decode_node_id_roundtrips_root_and_nested() {
    assert_eq!(decode_node_id("").unwrap(), Vec::<usize>::new());
    assert_eq!(decode_node_id("0/2").unwrap(), vec![0, 2]);
    assert!(decode_node_id("bad").is_err());
}

#[semio_framework_async_macros::async_test]
async fn op_text_roundtrip() {
    let command = XmlAnyEditorCommand::SetNode { node_id: "0/2".into(), value: "hello world".into() };
    let printed = <XmlAnyEditorCommand as protocol::OpText>::print_op(&command);
    let parsed = <XmlAnyEditorCommand as protocol::OpText>::parse_op(&printed).expect("parse ok");
    assert_eq!(parsed, command);
}
