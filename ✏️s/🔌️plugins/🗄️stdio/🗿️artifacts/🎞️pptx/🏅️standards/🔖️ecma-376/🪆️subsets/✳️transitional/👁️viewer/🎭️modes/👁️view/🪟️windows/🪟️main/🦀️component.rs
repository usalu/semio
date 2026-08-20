//! 🎞️ Pptx transitional viewer — `main` window: a real, READ-ONLY page view of the slide list,
//! built from the framework `DocumentWindowKit` (contract §2.6). Independent render from the
//! sibling mutation-capable surface — the same slide-to-page mapping, no edit affordances
//! (`window_kind()`, the read-only variant, not the editable one).

use crate::artifacts::pptx::schema::snapshot::{PptxParagraph, PptxShape};
use crate::artifacts::pptx::PptxSnapshot;
use semio_framework_plugin::app::{DocumentPage, DocumentView, DocumentWindowKit, WindowKit};
use semio_framework_plugin::{LocalizedLabel, UiNode, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = DocumentWindowKit::KIND_ID;
pub const BODY_KEY: &str = DocumentWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `create_pptx_transitional_viewer` (this subset's
/// surface root).
pub async fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Slides", "Folien"), icon_id: "presentation".into(), ..DocumentWindowKit::window_kind().await }
}
//#endregion 🔖️Definition

//#region 🔖️Render
async fn paragraph_text(paragraph: &PptxParagraph) -> String {
    paragraph.runs.iter().map(|run| run.text.as_str()).collect::<Vec<_>>().join("")
}

async fn shape_text(shape: &PptxShape) -> Option<String> {
    match shape {
        PptxShape::TextBox { text_frame, .. } | PptxShape::Placeholder { text_frame, .. } => Some(text_frame.iter().map(paragraph_text).collect::<Vec<_>>().join("\n")),
        PptxShape::Picture { .. } | PptxShape::Other { .. } => None,
    }
}

/// 👁️ Pure `PptxSnapshot -> UiNode` read: one `DocumentPage` per slide.
pub async fn render(document: &PptxSnapshot) -> UiNode {
    let pages = document.presentation.slides.iter().map(|slide| DocumentPage { text: slide.shapes.iter().filter_map(shape_text).collect::<Vec<_>>().join("\n") }).collect();
    DocumentWindowKit::render(&DocumentView { pages }).await
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::pptx::schema::snapshot::PptxSlide;

    #[semio_framework_async_macros::async_test]
    async fn definition_declares_a_document_window() {
        let def = definition();
        assert_eq!(def.id, WINDOW_KIND_ID);
        assert_eq!(def.body_key, BODY_KEY);
    }

    #[semio_framework_async_macros::async_test]
    async fn render_emits_one_page_per_slide() {
        let mut document = PptxSnapshot::default();
        document.presentation.slides.push(PptxSlide { shapes: vec![PptxShape::TextBox { text_frame: vec![PptxParagraph::text("only")], position: Default::default() }] });
        let UiNode::Stack(stack) = render(&document) else { panic!("expected Stack") };
        assert_eq!(stack.children.len(), 1);
    }
}
//#endregion 🧪️Tests
