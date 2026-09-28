//! 📄 PDF page window — the visual document surface shared by every subset editor.

use crate::editor::page;
use crate::PdfSnapshot;
use semio_framework_plugin::{Locale, TreeWindows, WindowKindDefinition};
use semio_framework_ui_contract::BuiltNode;

pub const WINDOW_KIND_ID: &str = page::WINDOW_KIND_ID;
pub const BODY_KEY: &str = page::BODY_KEY;

/// 🪟 Canvas of every page, with the page-edit actions.
pub fn definition() -> WindowKindDefinition {
    page::window_definition()
}

/// 🖼️ Paints the document for an unhosted window.
pub fn render(document: &PdfSnapshot) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    page::render_window(document)
}

/// 🖼️ Paints the document. Locale and tree chrome do not change the page geometry.
pub fn render_windowed(document: &PdfSnapshot, windows: &TreeWindows<'_>, locale: Locale) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let _ = (windows, locale);
    page::render_window(document)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
