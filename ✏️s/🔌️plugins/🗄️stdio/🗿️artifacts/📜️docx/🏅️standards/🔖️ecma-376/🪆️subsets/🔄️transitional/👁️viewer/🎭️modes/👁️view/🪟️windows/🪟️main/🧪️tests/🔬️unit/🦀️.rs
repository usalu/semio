use super::*;
use crate::schema::snapshot::DocxDocument;
use crate::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_docx;

#[semio_framework_async_macros::async_test]
async fn definition_declares_a_document_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.body_key, BODY_KEY);
}

#[semio_framework_async_macros::async_test]
async fn render_emits_one_page_per_top_level_block() {
    let document = build_minimal_docx(DocxDocument { body: vec![DocxBlock::paragraph("only")], styles: Vec::new() });
    let stack = render(&document).expect("render");
    assert_eq!(stack.children.len(), 1);
}
