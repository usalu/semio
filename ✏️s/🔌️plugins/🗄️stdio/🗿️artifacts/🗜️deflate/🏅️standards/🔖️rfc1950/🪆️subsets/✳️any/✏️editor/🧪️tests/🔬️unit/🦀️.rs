
use super::*;

#[semio_framework_async_macros::async_test]
async fn create_deflate_editor_builds_a_definition_for_the_editor_role() {
    let def = create_deflate_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, DEFLATE_EDITOR_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<DeflateEditor as ArtifactEditor>::DIALECT, DEFLATE_EDITOR_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_declares_the_main_window() {
    let def = create_deflate_editor();
    assert!(def.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
}

#[semio_framework_async_macros::async_test]
async fn parse_header_summary_round_trips_a_rendered_snapshot() {
    let document = DeflateSnapshot { compression_method: 8, window_bits: 9, compression_level_hint: crate::schema::snapshot::DeflateLevelHint::Maximum, dict_id: Some(7), payload: vec![9, 9], ..DeflateSnapshot::default() };
    let node = main::render(&document).expect("render");
    let Component::Surface(props) = node.component else { panic!("expected a retained text surface") };
    let scene: semio_framework_ui_scene::TextEditorScene = semio_framework_ui_scene::decode(&props).expect("decode text scene");
    let (method, window_bits, level_hint, dict_id) = parse_header_summary(&scene.buffer).expect("well-formed summary must parse");
    assert_eq!(method, 8);
    assert_eq!(window_bits, 9);
    assert_eq!(level_hint, crate::schema::snapshot::DeflateLevelHint::Maximum);
    assert_eq!(dict_id, Some(7));
}

#[semio_framework_async_macros::async_test]
async fn parse_header_summary_rejects_a_missing_required_field() {
    assert!(parse_header_summary("method=8\nwindowBits=7").is_none());
}
