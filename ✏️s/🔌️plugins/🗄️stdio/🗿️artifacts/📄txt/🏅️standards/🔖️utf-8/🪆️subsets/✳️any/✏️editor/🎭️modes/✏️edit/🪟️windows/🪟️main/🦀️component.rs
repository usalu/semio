//! 📄️ Txt editor — `main` window: a real, directly editable whole-document text buffer, built
//! from the framework `TextWindowKit` (contract §2.6). `TxtSnapshot.lines` is joined with the
//! document's own `line_ending` on render, and re-split the same way on `replace-text`.

use crate::artifacts::txt::TxtSnapshot;
use semio_framework_plugin::app::{TextView, TextWindowKit, WindowKit};
use semio_framework_plugin::{LocalizedLabel, UiNode, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TextWindowKit::KIND_ID;
pub const BODY_KEY: &str = TextWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by `crate::editor::txt::create_txt_editor`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Text", "Text"), icon_id: "type".into(), ..TextWindowKit::editable_window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// ✏️ Real `TxtSnapshot -> UiNode`: `lines` joined by the document's own `line_ending`, with a
/// trailing terminator when `trailing_newline` is set — the exact same join the artifact's own
/// codec uses to re-serialize, so what's shown here IS what re-encoding would emit.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(document: &TxtSnapshot) -> UiNode {
    let mut text = document.lines.join(document.line_ending.as_str());
    if document.trailing_newline && !document.lines.is_empty() {
        text.push_str(document.line_ending.as_str());
    }
    TextWindowKit::render(&TextView { text, language: Some("text".into()), read_only: false })
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
