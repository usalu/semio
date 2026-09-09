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
    let Component::Surface(props) = node.component else { panic!("expected a retained text surface") };
    let scene: semio_framework_ui_scene::TextEditorScene = semio_framework_ui_scene::decode(&props).expect("decode text scene");
    let parsed = parse_hex_dump(&scene.buffer).expect("well-formed hex dump must parse");
    assert_eq!(parsed, vec![0xde, 0xad, 0xbe, 0xef]);
}

#[semio_framework_async_macros::async_test]
async fn parse_hex_dump_rejects_odd_length_hex() {
    assert!(parse_hex_dump("abc").is_none());
}
