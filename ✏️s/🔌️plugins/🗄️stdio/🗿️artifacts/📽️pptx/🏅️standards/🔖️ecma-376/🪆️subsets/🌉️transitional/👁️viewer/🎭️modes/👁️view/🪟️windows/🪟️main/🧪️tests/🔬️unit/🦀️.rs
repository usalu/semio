use super::*;
use crate::schema::snapshot::{PptxParagraph, PptxPresentation, PptxShape, PptxSlide};

#[semio_framework_async_macros::async_test]
async fn definition_declares_a_document_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.body_key, BODY_KEY);
}

#[semio_framework_async_macros::async_test]
async fn render_emits_one_page_per_slide() {
    let document = crate::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_pptx(PptxPresentation {
        slides: vec![PptxSlide { shapes: vec![PptxShape::TextBox { text_frame: vec![PptxParagraph::text("only")], position: Default::default() }] }],
    });
    assert_eq!(render(&document).expect("render").children.len(), 1);
}
