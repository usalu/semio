use super::*;
use crate::standards::v1::subsets::document::schema::snapshot::{DocRun, STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_semio() -> SemioDocumentSnapshot {
    SemioDocumentSnapshot {
        schema: STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA.into(),
        styles: Vec::new(),
        images: Vec::new(),
        blocks: vec![DocBlock::Paragraph { style_id: None, runs: vec![DocRun::plain("Page one text.")] }, DocBlock::PageBreak, DocBlock::Paragraph { style_id: None, runs: vec![DocRun::plain("Page two text.")] }],
    }
}

#[semio_framework_async_macros::async_test]
async fn splits_pages_on_pagebreak() {
    let pdf = semio_framework_plugin::resolve_ready(SemioDocumentToPdf::serialize(&sample_semio())).expect("serialize");
    assert_eq!(pdf.pages.len(), 2);
    assert_eq!(pdf.pages[0].text, "Page one text.");
    assert_eq!(pdf.pages[1].text, "Page two text.");
    assert_eq!(pdf.declared_version, "1.7");
}

#[semio_framework_async_macros::async_test]
async fn empty_document_yields_zero_pages() {
    let snap = SemioDocumentSnapshot { schema: STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA.into(), styles: Vec::new(), images: Vec::new(), blocks: Vec::new() };
    let pdf = semio_framework_plugin::resolve_ready(SemioDocumentToPdf::serialize(&snap)).expect("serialize");
    assert!(pdf.pages.is_empty());
}
