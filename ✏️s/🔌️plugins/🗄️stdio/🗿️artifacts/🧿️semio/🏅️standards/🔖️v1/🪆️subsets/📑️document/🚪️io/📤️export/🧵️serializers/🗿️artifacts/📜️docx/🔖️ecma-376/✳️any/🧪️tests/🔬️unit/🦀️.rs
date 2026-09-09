use super::*;
use crate::standards::v1::subsets::document::schema::snapshot::{DocImage, DocStyle, DocTableCell, DocTableRow, RunStyle, STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_semio() -> SemioDocumentSnapshot {
    SemioDocumentSnapshot {
        schema: STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA.into(),
        styles: vec![DocStyle { id: "Heading1".into(), name: "Heading 1".into(), based_on: None }],
        images: Vec::new(),
        blocks: vec![
            DocBlock::Heading { level: 1, style_id: None, runs: vec![DocRun { text: "Title".into(), style: RunStyle { bold: true, ..Default::default() } }] },
            DocBlock::Paragraph { style_id: None, runs: vec![DocRun::plain("Body")] },
            DocBlock::Table { rows: vec![DocTableRow { cells: vec![DocTableCell { blocks: vec![DocBlock::paragraph("cell")] }] }] },
        ],
    }
}

#[semio_framework_async_macros::async_test]
async fn maps_heading_paragraph_and_table() {
    let docx = semio_framework_plugin::resolve_ready(SemioDocumentToDocx::serialize(&sample_semio())).expect("serialize");
    assert_eq!(docx.document.styles.len(), 1);
    assert_eq!(docx.document.body.len(), 3);
    assert!(matches!(&docx.document.body[0], DocxBlock::Paragraph(p) if p.style.as_deref() == Some("Heading1") && p.runs[0].bold));
    assert!(matches!(&docx.document.body[1], DocxBlock::Paragraph(p) if p.style.is_none()));
    assert!(matches!(&docx.document.body[2], DocxBlock::Table(t) if t.rows.len() == 1));
}

#[semio_framework_async_macros::async_test]
async fn list_and_quote_flatten_image_and_pagebreak_drop() {
    let snap = SemioDocumentSnapshot {
        schema: STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA.into(),
        styles: Vec::new(),
        images: vec![DocImage { id: "img1".into(), mime: "image/png".into(), bytes: vec![1, 2, 3] }],
        blocks: vec![
            DocBlock::List { ordered: true, items: vec![crate::standards::v1::subsets::document::schema::snapshot::DocListItem { blocks: vec![DocBlock::paragraph("item one")] }] },
            DocBlock::Quote { blocks: vec![DocBlock::paragraph("quoted")] },
            DocBlock::Image { image_id: "img1".into(), alt: "alt text".into(), width: None, height: None },
            DocBlock::PageBreak,
        ],
    };
    let docx = semio_framework_plugin::resolve_ready(SemioDocumentToDocx::serialize(&snap)).expect("serialize");
    // list item + quote paragraph + image-alt paragraph = 3 blocks; PageBreak drops entirely.
    assert_eq!(docx.document.body.len(), 3);
    assert!(matches!(&docx.document.body[2], DocxBlock::Paragraph(p) if p.runs[0].text == "alt text"));
}
