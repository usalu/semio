//! 📝️ Imperative viewer — the script window: a read-only compiled textual form of the document, built
//! from the framework's `TextWindowKit` (contract §2.6). Compiles straight off the pure shared-kernel
//! `imperative_engine::compile_to_text` free function — the sibling editor window's `ImperativeHost`
//! wrapper adds no logic this call needs, it just owns `&mut self` execution state a read-only render
//! never touches, so this file never reaches into the editor module for it.

use crate::artifacts::imperative::ImperativeSnapshot;
use semio_framework_plugin::app::{TextView, TextWindowKit, WindowKit};
use semio_framework_plugin::{LocalizedLabel, UiNode, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TextWindowKit::KIND_ID;
pub const BODY_KEY: &str = TextWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::imperative::create_imperative_viewer`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Script", "Skript"), icon_id: "file-code".into(), ..TextWindowKit::window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 👁️ Pure `ImperativeSnapshot -> UiNode` read: the compiled text of the document's own working
/// `Path`, always `read_only: true` (a viewer never emits a `replace-text` command).
pub fn render(document: &ImperativeSnapshot) -> UiNode {
    let path = crate::artifacts::imperative::imperative_working_scene(document).path;
    TextWindowKit::render(&TextView { text: imperative_engine::compile_to_text(&path), language: Some("imperative".into()), read_only: true })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn definition_declares_a_text_window() {
        let def = definition();
        assert_eq!(def.id, WINDOW_KIND_ID);
        assert_eq!(def.body_key, BODY_KEY);
    }

    #[test]
    fn render_compiles_the_default_document_into_read_only_text() {
        let document = crate::artifacts::imperative::schema::default_snapshot();
        let UiNode::ComponentScene(node) = render(&document) else { panic!("expected ComponentScene") };
        let scene = node.text_editor.expect("text_editor scene");
        assert!(scene.buffer.contains("log.print") || scene.buffer.contains("state.set"), "compiled text should mention a default step kind: {}", scene.buffer);
        assert!(scene.settings_json.unwrap_or_default().contains("readOnly"));
    }
}
//#endregion 🧪️Tests
