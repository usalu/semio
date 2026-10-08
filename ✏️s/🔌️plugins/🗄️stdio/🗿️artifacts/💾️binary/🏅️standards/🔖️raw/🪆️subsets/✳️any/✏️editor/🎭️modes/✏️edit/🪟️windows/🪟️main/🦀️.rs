//! 💾️ Binary editor — the `main` window: the complete raw byte buffer as editable lowercase hex,
//! carried by the framework's paged `TextWindowKit` scene.

use crate::BinarySnapshot;
use semio_framework_plugin::app::{TextEditView, TextWindowKit, WindowKit};
use semio_framework_plugin::BuiltNode;
use semio_framework_plugin::WindowKindDefinition;
use semio_framework_ui_locale::Locale;
use semio_framework_ui_locale::LocalizedLabel;

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TextWindowKit::KIND_ID;
pub const BODY_KEY: &str = TextWindowKit::KIND_ID;

/// 🔢️ The persisted store's one-item ceiling is the honest maximum a whole-buffer edit can publish.
pub const HEX_EDITOR_MAX_BYTES: usize = semio_framework_plugin::plugin_app_close_prelude::store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by `crate::editor::binary::create_binary_editor`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Bytes", "Bytes"), ..TextWindowKit::editable_window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// ✏️ Real `BinarySnapshot -> BuiltNode`: every byte as contiguous lowercase hex, plus a trailing informational byte-count
/// comment, as the kit's explicit draft (structured text: edited locally, ONE `textEdit` on Apply carrying the change set the editor made).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(document: &BinarySnapshot, locale: Locale, publication_revision: semio_framework_plugin::UiPublicationRevision) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let total = document.bytes.len();
    let hex: String = document.bytes.iter().map(|byte| format!("{byte:02x}")).collect();
    let text = format!("{hex}\n# total bytes: {total}");
    TextWindowKit::render_editable_by_splices(&TextEditView { surface_id: WINDOW_KIND_ID.into(), text, language: Some("hex".into()), revision: None, publication_revision }, locale)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
