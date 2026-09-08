
use super::*;
use crate::schema::snapshot::PptxSlide;

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
    let stack = render(&document).expect("render");
    assert_eq!(stack.children.len(), 1);
}
