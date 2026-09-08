//! 🎞️ Pptx strict viewer — `main` window: a real, READ-ONLY page view of the slide list, built
//! from the framework `DocumentWindowKit` (contract §2.6). Independent render from the sibling
//! mutation-capable surface — the same slide-to-page mapping, no edit affordances
//! (`window_kind()`, the read-only variant, not the editable one).

use crate::schema::snapshot::{PptxParagraph, PptxShape};
use crate::PptxSnapshot;
use semio_framework_plugin::app::{DocumentPage, DocumentView, DocumentWindowKit, WindowKit};
use semio_framework_plugin::{BuiltNode, LocalizedLabel, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = DocumentWindowKit::KIND_ID;
pub const BODY_KEY: &str = DocumentWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `create_pptx_strict_viewer` (this subset's surface
/// root).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Slides", "Folien"), icon_id: "presentation".into(), ..DocumentWindowKit::window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn paragraph_text(paragraph: &PptxParagraph) -> String {
    paragraph.runs.iter().map(|run| run.text.as_str()).collect::<Vec<_>>().join("")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn shape_text(shape: &PptxShape) -> Option<String> {
    match shape {
        PptxShape::TextBox { text_frame, .. } | PptxShape::Placeholder { text_frame, .. } => Some(text_frame.iter().map(paragraph_text).collect::<Vec<_>>().join("\n")),
        PptxShape::Picture { .. } | PptxShape::Other { .. } => None,
    }
}

/// 👁️ Pure `PptxSnapshot -> BuiltNode` read: one `DocumentPage` per slide.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(document: &PptxSnapshot) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let pages = document.presentation.slides.iter().map(|slide| DocumentPage { text: slide.shapes.iter().filter_map(shape_text).collect::<Vec<_>>().join("\n") }).collect();
    DocumentWindowKit::render(&DocumentView { pages })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
