use super::*;
use semio_framework_plugin::Component;

#[semio_framework_async_macros::async_test]
async fn definition_declares_a_read_only_text_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.body_key, BODY_KEY);
    assert!(def.actions.is_empty(), "a viewer window kind declares no mutation-shaped actions");
}

#[semio_framework_async_macros::async_test]
async fn render_carries_the_header_fields_as_read_only_text() {
    let document = DeflateSnapshot { compression_method: 8, window_bits: 7, compression_level_hint: DeflateLevelHint::Default, dict_id: None, payload: vec![1, 2, 3, 4], ..DeflateSnapshot::default() };
    let node = render(&document).expect("render");
    let Component::Surface(props) = node.component else { panic!("expected a retained text surface") };
    let scene: semio_framework_ui_scene::TextEditorScene = semio_framework_ui_scene::decode(&props).expect("decode text scene");
    assert!(scene.buffer.contains("method=8"));
    assert!(scene.buffer.contains("presetDictionary=none"));
    assert!(scene.settings_json.unwrap_or_default().contains("readOnly"));
}
