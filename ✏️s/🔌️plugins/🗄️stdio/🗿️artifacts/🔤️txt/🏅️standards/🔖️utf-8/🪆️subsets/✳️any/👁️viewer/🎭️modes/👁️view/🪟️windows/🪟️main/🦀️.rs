//! 📄️ Txt viewer — `main` window: a real, READ-ONLY whole-document text buffer, built from the
//! framework `TextWindowKit` (contract §2.6). Independent render from the sibling mutation-capable
//! surface — same `lines`/`line_ending` join, `read_only: true` stamps the host renderer.

use crate::TxtSnapshot;
use semio_framework_plugin::app::{TextView, TextWindowKit, WindowKit};
use semio_framework_plugin::{BuiltNode, LocalizedLabel, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TextWindowKit::KIND_ID;
pub const BODY_KEY: &str = TextWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::txt::create_txt_viewer`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Text", "Text"), icon_id: "type".into(), ..TextWindowKit::window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 👁️ Pure `TxtSnapshot -> BuiltNode` read: same join as the editor's own render, `read_only: true`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(document: &TxtSnapshot) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let mut text = document.lines.join(document.line_ending.as_str());
    if document.trailing_newline && !document.lines.is_empty() {
        text.push_str(document.line_ending.as_str());
    }
    TextWindowKit::render(&TextView { text, language: Some("text".into()), read_only: true })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
