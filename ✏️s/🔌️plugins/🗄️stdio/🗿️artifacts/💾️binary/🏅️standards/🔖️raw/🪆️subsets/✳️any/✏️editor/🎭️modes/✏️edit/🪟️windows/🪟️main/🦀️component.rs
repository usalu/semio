//! 💾️ Binary editor — the `main` window: the raw byte buffer as an editable lowercase-hex text
//! dump, built from the framework `TextWindowKit` (contract §2.6). Capped at
//! `HEX_PREVIEW_CAP_BYTES` for display (a real buffer can be arbitrarily large; the same
//! contiguous-hex convention `BinarySnapshot::print_dsl`/`parse_dsl` already use, so
//! `replace-text`'s reverse parse is a drop-in reuse). Scope note: when the buffer exceeds the cap,
//! the dump shows only the first `HEX_PREVIEW_CAP_BYTES` bytes plus a trailing `#`-prefixed
//! byte-count comment — submitting an edit while truncated replaces the WHOLE persisted buffer with
//! exactly what is shown (a documented first-pass limitation, not silent data loss: the comment
//! line states it explicitly).

use crate::artifacts::binary::BinarySnapshot;
use semio_framework_plugin::app::{TextView, TextWindowKit, WindowKit};
use semio_framework_plugin::{LocalizedLabel, UiNode, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TextWindowKit::KIND_ID;
pub const BODY_KEY: &str = TextWindowKit::KIND_ID;

/// 🔢️ The first N bytes shown/round-tripped by this window — shared by `render` and the surface
/// root's `replace-text` reverse parse (which always splices the WHOLE original buffer, so editing
/// beyond this cap truncates, per this file's own module doc comment).
pub const HEX_PREVIEW_CAP_BYTES: usize = 4096;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by `crate::editor::binary::create_binary_editor`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Bytes", "Bytes"), ..TextWindowKit::editable_window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// ✏️ Real `BinarySnapshot -> UiNode`: the first `HEX_PREVIEW_CAP_BYTES` bytes as contiguous
/// lowercase hex, editable (`read_only: false`), plus a trailing `#`-prefixed byte-count comment
/// (never parsed back — informational only).
pub fn render(document: &BinarySnapshot) -> UiNode {
    let total = document.bytes.len();
    let shown = total.min(HEX_PREVIEW_CAP_BYTES);
    let hex: String = document.bytes[..shown].iter().map(|byte| format!("{byte:02x}")).collect();
    let text = if total > shown {
        format!("{hex}\n# total bytes: {total} (showing first {shown}; editing while truncated replaces the buffer with exactly what is shown)")
    } else {
        format!("{hex}\n# total bytes: {total}")
    };
    TextWindowKit::render(&TextView { text, language: Some("hex".into()), read_only: false })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn definition_declares_an_editable_text_window() {
        let def = definition();
        assert_eq!(def.id, WINDOW_KIND_ID);
        assert_eq!(def.body_key, BODY_KEY);
        assert!(def.actions.iter().any(|action| action.id == "replace-text"), "editable text window must carry the replace-text catalog action");
    }

    #[test]
    fn render_carries_the_bytes_as_editable_hex() {
        let document = BinarySnapshot { bytes: vec![0xde, 0xad, 0xbe, 0xef], ..BinarySnapshot::default() };
        let UiNode::ComponentScene(node) = render(&document) else { panic!("expected ComponentScene") };
        let scene = node.text_editor.expect("text_editor scene");
        assert!(scene.buffer.starts_with("deadbeef"));
        assert!(scene.buffer.contains("total bytes: 4"));
        assert!(scene.settings_json.is_none(), "editable window must not stamp readOnly");
    }
}
//#endregion 🧪️Tests
