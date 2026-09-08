//! 💾️ Binary viewer — the `main` window: the raw byte buffer as a real, READ-ONLY lowercase-hex
//! text dump, built from the framework `TextWindowKit` (contract §2.6). Independent render from the
//! sibling mutation-capable surface — the same `BinarySnapshot.bytes` read, no edit affordances
//! (`window_kind()`, the read-only variant, not the editable one). Same
//! `HEX_PREVIEW_CAP_BYTES`-capped display as the sibling authoring surface.

use crate::BinarySnapshot;
use semio_framework_plugin::app::{TextView, TextWindowKit, WindowKit};
use semio_framework_plugin::{BuiltNode, LocalizedLabel, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TextWindowKit::KIND_ID;
pub const BODY_KEY: &str = TextWindowKit::KIND_ID;
pub const HEX_PREVIEW_CAP_BYTES: usize = 4096;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::binary::create_binary_viewer`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Bytes", "Bytes"), ..TextWindowKit::window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 👁️ Pure `BinarySnapshot -> BuiltNode` read: the first `HEX_PREVIEW_CAP_BYTES` bytes as contiguous
/// lowercase hex, always `read_only: true`, plus a trailing `#`-prefixed byte-count comment.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(document: &BinarySnapshot) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let total = document.bytes.len();
    let shown = total.min(HEX_PREVIEW_CAP_BYTES);
    let hex: String = document.bytes[..shown].iter().map(|byte| format!("{byte:02x}")).collect();
    let text = if total > shown { format!("{hex}\n# total bytes: {total} (showing first {shown})") } else { format!("{hex}\n# total bytes: {total}") };
    TextWindowKit::render(&TextView { text, language: Some("hex".into()), read_only: true })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
