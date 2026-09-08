//! 🎞️ Pptx transitional editor — `main` window: a real, directly editable page view of the slide
//! list, built from the framework `DocumentWindowKit` (contract §2.6). One page per slide — its
//! text is the CONCATENATION of every text-bearing shape (`TextBox`/`Placeholder`) on that slide,
//! joined by newlines; `Picture`/`Other` shapes contribute nothing. Editing writes back to shape 0
//! only (see the surface root's `PptxTransitionalEditorCommand::SetPage` for the honest
//! multi-shape scope note).

use crate::schema::snapshot::{PptxParagraph, PptxShape};
use crate::PptxSnapshot;
use semio_framework_plugin::app::{DocumentPage, DocumentView, DocumentWindowKit, WindowKit};
use semio_framework_plugin::{BuiltNode, LocalizedLabel, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = DocumentWindowKit::KIND_ID;
pub const BODY_KEY: &str = DocumentWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by `create_pptx_transitional_editor` (this subset's
/// surface root).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Slides", "Folien"), icon_id: "presentation".into(), ..DocumentWindowKit::editable_window_kind() }
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

/// ✏️ Real `PptxSnapshot -> BuiltNode`: one `DocumentPage` per slide.
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
