//! 📄️ Txt editor — `main` window: a real, directly editable whole-document text buffer, built
//! from the framework `TextWindowKit` (contract §2.6). `TxtSnapshot.lines` is joined with the
//! document's own `line_ending` on render, and re-split the same way on `replace-text`.

use crate::TxtSnapshot;
use semio_framework_plugin::app::{TextView, TextWindowKit, WindowKit};
use semio_framework_plugin::{BuiltNode, LocalizedLabel, WindowKindDefinition};

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
/// ✏️ Real `TxtSnapshot -> BuiltNode`: `lines` joined by the document's own `line_ending`, with a
/// trailing terminator when `trailing_newline` is set — the exact same join the artifact's own
/// codec uses to re-serialize, so what's shown here IS what re-encoding would emit.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(document: &TxtSnapshot) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let mut text = document.lines.join(document.line_ending.as_str());
    if document.trailing_newline && !document.lines.is_empty() {
        text.push_str(document.line_ending.as_str());
    }
    TextWindowKit::render(&TextView { text, language: Some("text".into()), read_only: false })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
