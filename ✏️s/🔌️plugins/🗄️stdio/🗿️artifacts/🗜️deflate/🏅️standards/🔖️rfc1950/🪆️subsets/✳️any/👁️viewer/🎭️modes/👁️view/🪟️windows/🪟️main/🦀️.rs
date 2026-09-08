//! 🗜️ Deflate viewer — the `main` window: the RFC1950 zlib container's typed header metadata as a
//! real, READ-ONLY `key=value` text summary, built from the framework `TextWindowKit` (contract
//! §2.6). Independent render from the sibling mutation-capable surface — the same
//! `DeflateSnapshot` header fields read, no edit affordances (`window_kind()`, the read-only
//! variant, not the editable one). The decompressed `payload` itself is never shown as text (only a
//! byte-count comment), same honest scope as the sibling authoring surface.

use crate::schema::snapshot::DeflateLevelHint;
use crate::DeflateSnapshot;
use semio_framework_plugin::app::{TextView, TextWindowKit, WindowKit};
use semio_framework_plugin::{BuiltNode, LocalizedLabel, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TextWindowKit::KIND_ID;
pub const BODY_KEY: &str = TextWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::deflate::create_deflate_viewer`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Compression Header", "Komprimierungs-Header"), ..TextWindowKit::window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️Codec
/// 📐️ `DeflateLevelHint -> lowercase keyword` — mirrors the sibling authoring surface's own
/// constant, kept as an independent copy per this ticket's viewer-purity rule.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn level_hint_keyword(hint: DeflateLevelHint) -> &'static str {
    match hint {
        DeflateLevelHint::Fastest => "fastest",
        DeflateLevelHint::Fast => "fast",
        DeflateLevelHint::Default => "default",
        DeflateLevelHint::Maximum => "maximum",
    }
}

/// 📐️ `dict_id -> "none" | "<id>"`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn preset_dictionary_text(dict_id: Option<u32>) -> String {
    match dict_id {
        Some(id) => id.to_string(),
        None => "none".into(),
    }
}
//#endregion 🔖️Codec

//#region 🔖️Render
/// 👁️ Pure `DeflateSnapshot -> BuiltNode` read: the same `key=value` header summary as the sibling
/// authoring surface, always `read_only: true`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(document: &DeflateSnapshot) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let text = format!(
        "method={}\nwindowBits={}\nlevelHint={}\npresetDictionary={}\n# payloadBytes: {} (payload content is not shown here)",
        document.compression_method,
        document.window_bits,
        level_hint_keyword(document.compression_level_hint),
        preset_dictionary_text(document.dict_id),
        document.payload.len(),
    );
    TextWindowKit::render(&TextView { text, language: Some("deflate-summary".into()), read_only: true })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
