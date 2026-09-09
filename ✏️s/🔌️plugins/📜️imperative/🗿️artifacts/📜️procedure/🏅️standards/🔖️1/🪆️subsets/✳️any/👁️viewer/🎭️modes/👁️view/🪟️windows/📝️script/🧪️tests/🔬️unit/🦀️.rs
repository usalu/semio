use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_a_text_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.body_key, BODY_KEY);
}

#[semio_framework_async_macros::async_test]
async fn render_compiles_the_default_document_into_read_only_text() {
    let document = crate::schema::default_snapshot();
    let node = render(&document).expect("viewer script");
    let semio_framework_plugin::Component::Surface(props) = &node.component else { panic!("text surface") };
    let scene: semio_framework_plugin::TextEditorScene = semio_framework_ui_scene::decode(props).expect("packed text");
    assert!(scene.buffer.contains("log.print") || scene.buffer.contains("state.set"), "compiled text should mention a default step kind: {}", scene.buffer);
    let settings: serde_json::Value = serde_json::from_str(scene.settings_json.as_deref().expect("text settings")).expect("independent settings oracle");
    assert_eq!(settings["readOnly"], true);
    semio_framework_plugin::testkit::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("retire text");
    crate::retire_procedure_fixture(document);
}
