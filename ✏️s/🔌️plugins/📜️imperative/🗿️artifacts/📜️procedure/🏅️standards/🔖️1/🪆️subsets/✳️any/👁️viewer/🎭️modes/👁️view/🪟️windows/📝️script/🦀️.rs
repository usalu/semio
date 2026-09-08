//! 📝️ Imperative viewer — the script window: a read-only compiled textual form of the document, built
//! from the framework's `TextWindowKit` (contract §2.6). Compiles straight off the pure shared-kernel
//! `imperative_engine::compile_to_text` free function — the sibling editor window's `ImperativeHost`
//! wrapper adds no logic this call needs, it just owns `&mut self` execution state a read-only render
//! never touches, so this file never reaches into the editor module for it.

use crate::ProcedureSnapshot;
use semio_framework_plugin::app::{TextView, TextWindowKit, WindowKit};
use semio_framework_plugin::{BuiltNode, LocalizedLabel, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TextWindowKit::KIND_ID;
pub const BODY_KEY: &str = TextWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::procedure::create_imperative_viewer`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Script", "Skript"), icon_id: "file-code".into(), ..TextWindowKit::window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 👁️ Pure `ProcedureSnapshot -> UiNode` read: the compiled text of the document's own working
/// `Path`, always `read_only: true` (a viewer never emits a `replace-text` command).
pub fn render(document: &ProcedureSnapshot) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let path = crate::procedure_working_scene(document).path;
    TextWindowKit::render(&TextView { text: imperative_engine::compile_to_text(&path), language: Some("imperative".into()), read_only: true })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
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
}
//#endregion 🧪️Tests
