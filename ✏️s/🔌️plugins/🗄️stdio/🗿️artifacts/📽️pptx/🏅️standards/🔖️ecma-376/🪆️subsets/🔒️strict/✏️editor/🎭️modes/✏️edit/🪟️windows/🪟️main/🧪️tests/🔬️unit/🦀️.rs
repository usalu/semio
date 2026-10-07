use super::*;
use crate::schema::snapshot::{PptxParagraph, PptxPresentation, PptxShape, PptxSlide};

#[semio_framework_async_macros::async_test]
async fn definition_declares_a_document_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.body_key, BODY_KEY);
}

#[semio_framework_async_macros::async_test]
async fn render_emits_one_draft_per_text_bearing_shape() {
    let document = crate::standards::v_ecma_376::subsets::base::schema::construction::minimal::build_minimal_pptx(PptxPresentation {
        slides: vec![PptxSlide {
            shapes: vec![PptxShape::TextBox { text_frame: vec![PptxParagraph::text("a")], position: Default::default() }, PptxShape::Placeholder { kind: "body".into(), text_frame: vec![PptxParagraph::text("b")], position: Default::default() }],
        }],
    });
    assert_eq!(render(&document, semio_framework_plugin::UiPublicationRevision(23)).expect("render").children.len(), 2);
}
