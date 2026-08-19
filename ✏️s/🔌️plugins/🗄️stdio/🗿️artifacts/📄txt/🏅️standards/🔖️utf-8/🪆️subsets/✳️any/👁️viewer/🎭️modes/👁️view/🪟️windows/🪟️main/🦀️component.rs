//! 📄️ Txt viewer — `main` window: a real, READ-ONLY whole-document text buffer, built from the
//! framework `TextWindowKit` (contract §2.6). Independent render from the sibling mutation-capable
//! surface — same `lines`/`line_ending` join, `read_only: true` stamps the host renderer.

use crate::artifacts::txt::TxtSnapshot;
use semio_framework_plugin::app::{TextView, TextWindowKit, WindowKit};
use semio_framework_plugin::{LocalizedLabel, UiNode, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TextWindowKit::KIND_ID;
pub const BODY_KEY: &str = TextWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::txt::create_txt_viewer`.
pub async fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Text", "Text"), icon_id: "type".into(), ..TextWindowKit::window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 👁️ Pure `TxtSnapshot -> UiNode` read: same join as the editor's own render, `read_only: true`.
pub async fn render(document: &TxtSnapshot) -> UiNode {
    let mut text = document.lines.join(document.line_ending.as_str());
    if document.trailing_newline && !document.lines.is_empty() {
        text.push_str(document.line_ending.as_str());
    }
    TextWindowKit::render(&TextView { text, language: Some("text".into()), read_only: true })
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
    async fn render_joins_lines_with_the_line_ending() {
        let document = TxtSnapshot { schema: "stdio.txt".into(), lines: vec!["a".into(), "b".into()], trailing_newline: false, line_ending: Default::default() };
        let UiNode::ComponentScene(node) = render(&document) else { panic!("expected ComponentScene") };
        let scene = node.text_editor.expect("text editor scene");
        assert_eq!(scene.buffer, "a\nb");
    }
}
//#endregion 🧪️Tests
