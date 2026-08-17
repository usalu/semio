//! 💾️ Binary viewer — the `main` window: the raw byte buffer as a real, READ-ONLY lowercase-hex
//! text dump, built from the framework `TextWindowKit` (contract §2.6). Independent render from the
//! sibling mutation-capable surface — the same `BinarySnapshot.bytes` read, no edit affordances
//! (`window_kind()`, the read-only variant, not the editable one). Same
//! `HEX_PREVIEW_CAP_BYTES`-capped display as the sibling authoring surface.

use crate::artifacts::binary::BinarySnapshot;
use semio_framework_plugin::app::{TextView, TextWindowKit, WindowKit};
use semio_framework_plugin::{LocalizedLabel, UiNode, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TextWindowKit::KIND_ID;
pub const BODY_KEY: &str = TextWindowKit::KIND_ID;
pub const HEX_PREVIEW_CAP_BYTES: usize = 4096;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::binary::create_binary_viewer`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Bytes", "Bytes"), ..TextWindowKit::window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 👁️ Pure `BinarySnapshot -> UiNode` read: the first `HEX_PREVIEW_CAP_BYTES` bytes as contiguous
/// lowercase hex, always `read_only: true`, plus a trailing `#`-prefixed byte-count comment.
pub fn render(document: &BinarySnapshot) -> UiNode {
    let total = document.bytes.len();
    let shown = total.min(HEX_PREVIEW_CAP_BYTES);
    let hex: String = document.bytes[..shown].iter().map(|byte| format!("{byte:02x}")).collect();
    let text = if total > shown { format!("{hex}\n# total bytes: {total} (showing first {shown})") } else { format!("{hex}\n# total bytes: {total}") };
    TextWindowKit::render(&TextView { text, language: Some("hex".into()), read_only: true })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn definition_declares_a_read_only_text_window() {
        let def = definition();
        assert_eq!(def.id, WINDOW_KIND_ID);
        assert_eq!(def.body_key, BODY_KEY);
        assert!(def.actions.is_empty(), "a viewer window kind declares no mutation-shaped actions");
    }

    #[test]
    fn render_carries_the_bytes_as_read_only_hex() {
        let document = BinarySnapshot { bytes: vec![0xde, 0xad, 0xbe, 0xef], ..BinarySnapshot::default() };
        let UiNode::ComponentScene(node) = render(&document) else { panic!("expected ComponentScene") };
        let scene = node.text_editor.expect("text_editor scene");
        assert!(scene.buffer.starts_with("deadbeef"));
        assert!(scene.settings_json.unwrap_or_default().contains("readOnly"));
    }
}
//#endregion 🧪️Tests
