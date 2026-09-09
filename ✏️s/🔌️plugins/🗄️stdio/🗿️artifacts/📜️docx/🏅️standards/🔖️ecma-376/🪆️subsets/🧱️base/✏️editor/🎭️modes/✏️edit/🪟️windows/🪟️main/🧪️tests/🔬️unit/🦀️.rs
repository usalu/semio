use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_a_document_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.body_key, BODY_KEY);
}

#[semio_framework_async_macros::async_test]
async fn render_emits_one_page_per_top_level_block() {
    let mut document = DocxSnapshot::default();
    document.document.body.push(DocxBlock::paragraph("first"));
    document.document.body.push(DocxBlock::paragraph("second"));
    let stack = render(&document).expect("render");
    assert_eq!(stack.children.len(), 2);
}
