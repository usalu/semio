use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_a_read_only_text_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.body_key, BODY_KEY);
    assert!(def.actions.is_empty(), "a viewer window kind declares no mutation-shaped actions");
}

#[semio_framework_async_macros::async_test]
async fn render_carries_the_bytes_as_read_only_hex() {
    let document = BinarySnapshot { bytes: vec![0xde, 0xad, 0xbe, 0xef], ..BinarySnapshot::default() };
    let node = render(&document).expect("render");
    let scene: semio_framework_ui_scene::TextEditorScene = semio_framework_plugin::artifact_app_laws::built_surface_scene(&node).expect("decode the text scene with its lanes");
    assert!(scene.buffer.starts_with("deadbeef"));
    assert!(scene.settings_json.unwrap_or_default().contains("readOnly"));
}
