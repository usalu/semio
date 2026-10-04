//! 🎞️ Pptx viewer — `main` window: a real, READ-ONLY page view of the slide list, built from the
//! framework `DocumentWindowKit` (contract §2.6). Independent render from the sibling
//! mutation-capable surface — the same slide-to-page mapping, no edit affordances
//! (`window_kind()`, the read-only variant, not the editable one).

use crate::PptxSnapshot;
use semio_framework_plugin::app::{DocumentPage, DocumentView, DocumentWindowKit, WindowKit};
use semio_framework_plugin::BuiltNode;
use semio_framework_plugin::WindowKindDefinition;
use semio_framework_ui_locale::LocalizedLabel;

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = DocumentWindowKit::KIND_ID;
pub const BODY_KEY: &str = DocumentWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `create_pptx_viewer` (this subset's surface root).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Slides", "Folien"), icon_id: "presentation".into(), ..DocumentWindowKit::window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
/// 👁️ Pure `PptxSnapshot -> BuiltNode` read: one `DocumentPage` per slide.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(document: &PptxSnapshot) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let pages = crate::schema::mutations::xml_address::pptx_slides(document)
        .map_err(|error| semio_framework_plugin::PluginAssemblyError::new("pptx.projection", error))?
        .into_iter()
        .map(|slide| DocumentPage { text: slide.shapes.into_iter().filter_map(|shape| shape.text).collect::<Vec<_>>().join("\n") })
        .collect();
    DocumentWindowKit::render(&DocumentView { pages })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
