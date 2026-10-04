//! 🎞️ Pptx strict editor — `main` window: a real, directly editable page view of the slide list,
//! built from the framework `DocumentWindowKit` (contract §2.6). One page per slide — its text is
//! the CONCATENATION of every text-bearing shape (`TextBox`/`Placeholder`) on that slide, joined by
//! newlines; `Picture`/`Other` shapes contribute nothing. Editing writes back to shape 0 only (see
//! the surface root's `PptxStrictEditorCommand::SetPage` for the honest multi-shape scope note).

use crate::PptxSnapshot;
use semio_framework_plugin::app::{DocumentWindowKit, EditableDocumentPage, EditableDocumentView, WindowKit};
use semio_framework_plugin::BuiltNode;
use semio_framework_plugin::TreeWindows;
use semio_framework_plugin::WindowKindDefinition;
use semio_framework_ui_locale::Locale;
use semio_framework_ui_locale::LocalizedLabel;

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = DocumentWindowKit::KIND_ID;
pub const BODY_KEY: &str = DocumentWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by `create_pptx_strict_editor` (this subset's surface
/// root).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Slides", "Folien"), icon_id: "presentation".into(), ..DocumentWindowKit::editable_window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
/// ✏️ Builds one faithfully addressed draft per text-bearing slide shape.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn editable_pages(document: &PptxSnapshot) -> Result<Vec<EditableDocumentPage>, String> {
    Ok(crate::schema::mutations::xml_address::pptx_slides(document)?
        .iter()
        .enumerate()
        .flat_map(|(page_index, slide)| slide.shapes.iter().enumerate().filter_map(move |(item_index, shape)| shape.text.clone().map(|text| EditableDocumentPage::new(page_index as u32, item_index as u32, text))))
        .collect())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(document: &PptxSnapshot, publication_revision: semio_framework_plugin::UiPublicationRevision) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    DocumentWindowKit::render_editable_windowed(
        &EditableDocumentView { pages: editable_pages(document).map_err(|error| semio_framework_plugin::PluginAssemblyError::new("pptx.projection", error))?, publication_revision },
        &TreeWindows::unhosted(),
        Locale::En,
    )
}

pub fn render_windowed(document: &PptxSnapshot, windows: &TreeWindows<'_>, locale: Locale, publication_revision: semio_framework_plugin::UiPublicationRevision) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    DocumentWindowKit::render_editable_windowed(&EditableDocumentView { pages: editable_pages(document).map_err(|error| semio_framework_plugin::PluginAssemblyError::new("pptx.projection", error))?, publication_revision }, windows, locale)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
