//! 🪟️ PDF/A Document (1.4) editor -- `main` window: a real per-page overview + authoring surface over the shared `PdfSnapshot`
//! (canonically 1.7-shaped -- see the surface root's own module doc comment), built from the framework
//! `DocumentWindowKit` (contract §2.6). One `DocumentPage` per `PdfPage`: the page's real `MediaBox`/
//! `CropBox` geometry (never fabricated) followed by its own `text` field -- a genuine field of
//! `PdfPage` itself (populated by ToUnicode-aware content-stream extraction on decode, or authored
//! directly on a fresh page), never a placeholder invented by this window.
//!
//! Text drafts replace the page content through the reversible `SetPageContent` mutation while
//! preserving page geometry, resources, annotations, and every non-text operation.
use crate::PdfSnapshot;
use semio_framework_plugin::app::{DocumentWindowKit, EditableDocumentPage, EditableDocumentView, WindowKit};
use semio_framework_plugin::{Locale, LocalizedLabel, TreeWindows, WindowKindDefinition};
use semio_framework_ui_contract::BuiltNode;

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = DocumentWindowKit::KIND_ID;
pub const BODY_KEY: &str = DocumentWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by `crate::editor::pdf14a::create_pdf14_a_editor`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Pages", "Seiten"), icon_id: "file-text".into(), ..DocumentWindowKit::editable_window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 📄️ Builds one prefilled text-content draft per page; geometry and every non-text operation remain in Details.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn editable_pages(document: &PdfSnapshot) -> Vec<EditableDocumentPage> {
    document.pages.iter().enumerate().map(|(page_index, page)| EditableDocumentPage::new(page_index as u32, 0, page.text())).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(document: &PdfSnapshot) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    DocumentWindowKit::render_editable_windowed(&EditableDocumentView { pages: editable_pages(document) }, &TreeWindows::unhosted(), Locale::En)
}

pub fn render_windowed(document: &PdfSnapshot, windows: &TreeWindows<'_>, locale: Locale) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    DocumentWindowKit::render_editable_windowed(&EditableDocumentView { pages: editable_pages(document) }, windows, locale)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
