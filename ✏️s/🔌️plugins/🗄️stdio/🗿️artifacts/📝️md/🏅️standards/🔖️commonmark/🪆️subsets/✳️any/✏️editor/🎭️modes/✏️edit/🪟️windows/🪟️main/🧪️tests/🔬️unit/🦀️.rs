use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_uses_the_frozen_window_kit_kind_id() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
}

/// ⚖️ LAW (explicit-draft policy, audit F-5): the markup source is an explicit draft of the kit's `textEdit` verb in every
/// locale — edited locally, ONE edit on Apply — never live whole-text typing.
#[semio_framework_async_macros::async_test]
async fn render_produces_an_explicit_draft_for_the_default_document() {
    for (locale, apply) in [(semio_framework_ui_locale::Locale::En, "Apply"), (semio_framework_ui_locale::Locale::De, "Anwenden")] {
        let node = render(&MdSnapshot::default(), locale).expect("render");
        let scene: semio_framework_ui_scene::TextEditorScene = semio_framework_plugin::artifact_app_laws::built_surface_scene(&node).expect("decode the text scene with its lanes");
        let settings: serde_json::Value = serde_json::from_str(scene.settings_json.as_deref().expect("explicit draft settings")).expect("settings json");
        assert_eq!((settings["commit"].as_str(), settings["editAction"].as_str(), settings["applyLabel"].as_str()), (Some("explicit"), Some("textEdit"), Some(apply)));
    }
}
