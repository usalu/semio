//! 🗜️ Deflate editor — the `main` window: the RFC1950 zlib container's typed header metadata as an
//! editable `key=value` text summary, built from the framework `TextWindowKit` (contract §2.6).
//! Scope note: only the header fields (`method`, `windowBits`, `levelHint`, `presetDictionary`) are
//! editable through `replace-text`; the decompressed `payload` itself is shown ONLY as a trailing
//! `#`-prefixed byte-count comment, never as editable text — a compressed byte stream has no honest
//! text representation, so real payload editing is out of this first pass's scope (documented,
//! matching the ticket brief's own container/opaque framing for this kind).

use crate::artifacts::deflate::schema::snapshot::DeflateLevelHint;
use crate::artifacts::deflate::DeflateSnapshot;
use semio_framework_plugin::app::{TextView, TextWindowKit, WindowKit};
use semio_framework_plugin::{LocalizedLabel, UiNode, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TextWindowKit::KIND_ID;
pub const BODY_KEY: &str = TextWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by `crate::editor::deflate::create_deflate_editor`.
pub async fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Compression Header", "Komprimierungs-Header"), ..TextWindowKit::editable_window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️Codec
/// 📐️ `DeflateLevelHint <-> lowercase keyword` — shared by `render`'s summary line and the surface
/// root's `parse_level_hint` (`replace-text` reverse direction).
pub async fn level_hint_keyword(hint: DeflateLevelHint) -> &'static str {
    match hint {
        DeflateLevelHint::Fastest => "fastest",
        DeflateLevelHint::Fast => "fast",
        DeflateLevelHint::Default => "default",
        DeflateLevelHint::Maximum => "maximum",
    }
}

/// 📐️ Inverse of [`level_hint_keyword`]. `None` on an unrecognized keyword — the surface root
/// treats that as a malformed `replace-text` (documented no-op), never a panic.
pub async fn parse_level_hint(keyword: &str) -> Option<DeflateLevelHint> {
    match keyword {
        "fastest" => Some(DeflateLevelHint::Fastest),
        "fast" => Some(DeflateLevelHint::Fast),
        "default" => Some(DeflateLevelHint::Default),
        "maximum" => Some(DeflateLevelHint::Maximum),
        _ => None,
    }
}

/// 📐️ `dict_id -> "none" | "<id>"` — shared by `render` and the surface root's reverse parse.
pub async fn preset_dictionary_text(dict_id: Option<u32>) -> String {
    match dict_id {
        Some(id) => id.to_string(),
        None => "none".into(),
    }
}
//#endregion 🔖️Codec

//#region 🔖️Render
/// ✏️ Real `DeflateSnapshot -> UiNode`: a `key=value` line per header field, editable
/// (`read_only: false`), plus a trailing `#`-prefixed comment line stating the payload byte count
/// (informational only — `#`-prefixed lines are never parsed back on `replace-text`).
pub async fn render(document: &DeflateSnapshot) -> UiNode {
    let text = format!(
        "method={}\nwindowBits={}\nlevelHint={}\npresetDictionary={}\n# payloadBytes: {} (payload content is not shown or editable here)",
        document.compression_method,
        document.window_bits,
        level_hint_keyword(document.compression_level_hint),
        preset_dictionary_text(document.dict_id),
        document.payload.len(),
    );
    TextWindowKit::render(&TextView { text, language: Some("deflate-summary".into()), read_only: false })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn definition_declares_an_editable_text_window() {
        let def = definition();
        assert_eq!(def.id, WINDOW_KIND_ID);
        assert_eq!(def.body_key, BODY_KEY);
        assert!(def.actions.iter().any(|action| action.id == "replace-text"), "editable text window must carry the replace-text catalog action");
    }

    #[test]
    async fn render_carries_the_header_fields_as_editable_text() {
        let document = DeflateSnapshot { compression_method: 8, window_bits: 7, compression_level_hint: DeflateLevelHint::Fast, dict_id: Some(42), payload: vec![1, 2, 3], ..DeflateSnapshot::default() };
        let UiNode::ComponentScene(node) = render(&document) else { panic!("expected ComponentScene") };
        let scene = node.text_editor.expect("text_editor scene");
        assert!(scene.buffer.contains("method=8"));
        assert!(scene.buffer.contains("windowBits=7"));
        assert!(scene.buffer.contains("levelHint=fast"));
        assert!(scene.buffer.contains("presetDictionary=42"));
        assert!(scene.buffer.contains("payloadBytes: 3"));
        assert!(scene.settings_json.is_none(), "editable window must not stamp readOnly");
    }
}
//#endregion 🧪️Tests
