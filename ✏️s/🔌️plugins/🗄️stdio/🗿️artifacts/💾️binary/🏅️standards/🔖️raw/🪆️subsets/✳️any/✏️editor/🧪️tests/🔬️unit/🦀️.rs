use super::*;

#[semio_framework_async_macros::async_test]
async fn create_binary_editor_builds_a_definition_for_the_editor_role() {
    let def = create_binary_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, BINARY_EDITOR_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<BinaryEditor as ArtifactEditor>::DIALECT, BINARY_EDITOR_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_declares_the_main_window() {
    let def = create_binary_editor();
    assert!(def.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
}

#[semio_framework_async_macros::async_test]
async fn parse_hex_dump_round_trips_a_rendered_snapshot() {
    let document = BinarySnapshot { bytes: vec![0xde, 0xad, 0xbe, 0xef], ..BinarySnapshot::default() };
    let node = main::render(&document).expect("render");
    let scene: semio_framework_ui_scene::TextEditorScene = semio_framework_plugin::artifact_app_laws::built_surface_scene(&node).expect("decode the text scene with its lanes");
    let parsed = parse_hex_dump(&scene.buffer).expect("well-formed hex dump must parse");
    assert_eq!(parsed, vec![0xde, 0xad, 0xbe, 0xef]);
}

#[semio_framework_async_macros::async_test]
async fn parse_hex_dump_rejects_odd_length_hex() {
    assert!(parse_hex_dump("abc").is_none());
    assert!(parse_hex_dump("€0").is_none());
}

#[test]
fn text_edit_requires_an_explicit_text_argument_and_allows_intentional_empty_bytes() {
    assert!(<BinaryEditor as ArtifactEditor>::command_from_action("textEdit", None).is_err());
    let args = dsl::DslValue::object([("text".into(), dsl::DslValue::String(String::new()))]);
    let command = <BinaryEditor as ArtifactEditor>::command_from_action("textEdit", Some(&args)).expect("explicit empty text");
    let source = BinarySnapshot { bytes: vec![1, 2, 3], ..BinarySnapshot::default() };
    let emitted = binary_text_emit(&command, &source).expect("empty hex intentionally clears the byte buffer");
    assert!(matches!(
        emitted.artifact_mutations.as_slice(),
        [BinaryMutation::ReplaceByteRange(replace_byte_range::ReplaceByteRange { offset: 0, remove_len: 3, insert })] if insert.is_empty()
    ));
}
