
use super::*;
use semio_s_artifact_stdio_pdf::standards::v1_7::subsets::base::schema::snapshot::PdfPage;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub(crate) fn sample_pdf() -> PdfSnapshot {
    let mut p1 = PdfPage::new(612.0, 792.0);
    p1.text = "Page one text.".into();
    let mut p2 = PdfPage::new(612.0, 792.0);
    p2.text = "Page two text.".into();
    PdfSnapshot { pages: vec![p1, p2], ..Default::default() }
}

#[semio_framework_async_macros::async_test]
async fn each_page_becomes_a_paragraph_separated_by_pagebreak() {
    let semio = semio_framework_plugin::resolve_ready(SemioDocumentFromPdf::deserialize(&sample_pdf())).expect("deserialize");
    assert_eq!(semio.blocks.len(), 3);
    assert!(matches!(&semio.blocks[0], DocBlock::Paragraph { runs, .. } if runs[0].text == "Page one text."));
    assert!(matches!(&semio.blocks[1], DocBlock::PageBreak));
    assert!(matches!(&semio.blocks[2], DocBlock::Paragraph { runs, .. } if runs[0].text == "Page two text."));
}

#[semio_framework_async_macros::async_test]
async fn zero_pages_yields_zero_blocks() {
    let semio = semio_framework_plugin::resolve_ready(SemioDocumentFromPdf::deserialize(&PdfSnapshot::default())).expect("deserialize");
    assert!(semio.blocks.is_empty());
}
