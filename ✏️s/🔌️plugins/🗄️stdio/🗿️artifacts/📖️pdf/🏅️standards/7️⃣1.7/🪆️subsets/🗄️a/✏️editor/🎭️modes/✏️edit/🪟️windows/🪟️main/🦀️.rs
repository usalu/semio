//! 📄 PDF page window — the visual document surface shared by every subset editor.

use crate::editor::page;
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use semio_plugin::WindowKindDefinition;
use semio_ui_contract::BuiltNode;

pub const WINDOW_KIND_ID: &str = page::WINDOW_KIND_ID;
pub const BODY_KEY: &str = page::BODY_KEY;

/// 🪟 Canvas of every page, with the page-edit actions.
pub fn definition() -> WindowKindDefinition {
    page::window_definition()
}

/// 🖼️ Paints the document for an unhosted window.
pub fn render(document: &PdfSnapshot, locale: semio_ui_locale::Locale) -> semio_plugin::UiAssemblyResult<BuiltNode> {
    page::render_window(document, locale)
}

/// 🖼️ Paints the document through the ephemeral local view (zoom and anchored page) with search highlights on top.
pub fn render_windowed(document: &PdfSnapshot, selected: Option<&str>, view: &page::ViewState, viewport: (f64, f64), highlights: Vec<page::OverlayLayer>, locale: semio_ui_locale::Locale) -> semio_plugin::UiAssemblyResult<BuiltNode> {
    page::render_view(document, selected, view, viewport, highlights, locale)
}


