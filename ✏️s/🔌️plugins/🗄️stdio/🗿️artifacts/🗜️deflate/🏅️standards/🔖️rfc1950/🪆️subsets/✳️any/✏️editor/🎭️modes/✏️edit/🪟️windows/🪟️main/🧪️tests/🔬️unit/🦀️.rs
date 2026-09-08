
use super::*;
use semio_framework_plugin::Component;

#[semio_framework_async_macros::async_test]
async fn definition_declares_an_editable_text_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.body_key, BODY_KEY);
    assert!(def.actions.iter().any(|action| action.id == "replace-text"), "editable text window must carry the replace-text catalog action");
}

#[semio_framework_async_macros::async_test]
async fn render_carries_the_header_fields_as_editable_text() {
    let document = DeflateSnapshot { compression_method: 8, window_bits: 7, compression_level_hint: DeflateLevelHint::Fast, dict_id: Some(42), payload: vec![1, 2, 3], ..DeflateSnapshot::default() };
    let node = render(&document).expect("render");
    let Component::Surface(props) = node.component else { panic!("expected a retained text surface") };
    let scene: semio_framework_ui_scene::TextEditorScene = semio_framework_ui_scene::decode(&props).expect("decode text scene");
    assert!(scene.buffer.contains("method=8"));
    assert!(scene.buffer.contains("windowBits=7"));
    assert!(scene.buffer.contains("levelHint=fast"));
    assert!(scene.buffer.contains("presetDictionary=42"));
    assert!(scene.buffer.contains("payloadBytes: 3"));
    assert!(scene.settings_json.is_none(), "editable window must not stamp readOnly");
}
