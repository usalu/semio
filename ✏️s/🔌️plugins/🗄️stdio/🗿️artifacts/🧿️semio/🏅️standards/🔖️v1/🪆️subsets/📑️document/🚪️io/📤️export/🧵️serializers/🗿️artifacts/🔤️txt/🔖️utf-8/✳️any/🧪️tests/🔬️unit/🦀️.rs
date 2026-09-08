
use super::*;
use crate::standards::v1::subsets::document::schema::snapshot::{DocListItem, RunStyle, STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_semio() -> SemioDocumentSnapshot {
    SemioDocumentSnapshot {
        schema: STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA.into(),
        styles: Vec::new(),
        images: Vec::new(),
        blocks: vec![
            DocBlock::Heading { level: 1, style_id: None, runs: vec![DocRun { text: "Title".into(), style: RunStyle { bold: true, ..Default::default() } }] },
            DocBlock::Paragraph { style_id: None, runs: vec![DocRun::plain("Body text.")] },
            DocBlock::List { ordered: true, items: vec![DocListItem { blocks: vec![DocBlock::paragraph("item one")] }] },
            DocBlock::PageBreak,
        ],
    }
}

#[semio_framework_async_macros::async_test]
async fn extracts_plain_text_lines_and_drops_pagebreak() {
    let txt = semio_framework_plugin::resolve_ready(SemioDocumentToTxt::serialize(&sample_semio())).expect("serialize");
    assert_eq!(txt.lines, vec!["Title".to_string(), "Body text.".to_string(), "item one".to_string()]);
    assert!(txt.trailing_newline);
}
