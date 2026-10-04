use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_an_editable_text_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.body_key, BODY_KEY);
    assert!(def.actions.iter().any(|action| action.id == "textEdit"), "editable text window must carry the replace-text catalog action");
}

#[semio_framework_async_macros::async_test]
async fn render_carries_the_header_fields_as_editable_text() {
    let document = DeflateSnapshot { compression_method: 8, window_bits: 7, compression_level_hint: DeflateLevelHint::Fast, dict_id: Some(42), payload: vec![1, 2, 3], ..DeflateSnapshot::default() };
    let node = render(&document, semio_framework_ui_locale::Locale::En, semio_framework_plugin::UiPublicationRevision(23)).expect("render");
    let scene: semio_framework_ui_scene::TextEditorScene = semio_framework_plugin::artifact_app_laws::built_surface_scene(&node).expect("decode the text scene with its lanes");
    assert!(scene.buffer.contains("method=8"));
    assert!(scene.buffer.contains("windowBits=7"));
    assert!(scene.buffer.contains("levelHint=fast"));
    assert!(scene.buffer.contains("presetDictionary=42"));
    assert!(scene.buffer.contains("payloadBytes: 3"));
    let settings: serde_json::Value = serde_json::from_str(scene.settings_json.as_deref().expect("explicit draft settings")).expect("settings json");
    assert_eq!((settings["readOnly"].as_bool(), settings["commit"].as_str(), settings["editAction"].as_str()), (Some(false), Some("explicit"), Some("textEdit")), "an editable text window is the kit's explicit draft");
}
