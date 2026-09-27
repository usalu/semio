//! 📄️ Docx editor — `main` window: a real, directly editable page view of `DocxDocument.body`,
//! built from the framework `DocumentWindowKit` (contract §2.6). Every paragraph run is an
//! independently revision-guarded text target, preserving formatting and sibling run text.

use crate::schema::snapshot::DocxBlock;
use crate::DocxSnapshot;
use semio_framework_plugin::app::{DocumentWindowKit, EditableDocumentPage, EditableDocumentView, WindowKit};
use semio_framework_plugin::{BuiltNode, Locale, LocalizedLabel, TreeWindows, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = DocumentWindowKit::KIND_ID;
pub const BODY_KEY: &str = DocumentWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by `create_docx_editor` (this subset's surface root).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Document", "Dokument"), icon_id: "file-text".into(), ..DocumentWindowKit::editable_window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// ✏️ Builds one faithfully editable draft for every existing paragraph run.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn editable_pages(document: &DocxSnapshot) -> Vec<EditableDocumentPage> {
    document
        .document
        .body
        .iter()
        .enumerate()
        .flat_map(|(page_index, block)| match block {
            DocxBlock::Paragraph(paragraph) => paragraph.runs.iter().enumerate().map(|(item_index, run)| EditableDocumentPage { page_index: page_index as u32, item_index: item_index as u32, text: run.text.clone() }).collect::<Vec<_>>(),
            DocxBlock::Table(_) => Vec::new(),
        })
        .collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(document: &DocxSnapshot) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    DocumentWindowKit::render_editable_windowed(&EditableDocumentView { pages: editable_pages(document) }, &TreeWindows::unhosted(), Locale::En)
}

pub fn render_windowed(document: &DocxSnapshot, windows: &TreeWindows<'_>, locale: Locale) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    DocumentWindowKit::render_editable_windowed(&EditableDocumentView { pages: editable_pages(document) }, windows, locale)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
