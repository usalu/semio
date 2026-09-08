
use super::*;
use crate::standards::v1::subsets::document::schema::snapshot::{DocListItem, DocTableCell, DocTableRow, STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA};

#[semio_framework_async_macros::async_test]
async fn collects_headings_and_counts_words_and_blocks() {
    let snapshot = SemioDocumentSnapshot {
        schema: STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA.into(),
        styles: Vec::new(),
        images: Vec::new(),
        blocks: vec![
            DocBlock::Heading { level: 1, style_id: None, runs: vec![DocRun::plain("Hello World")] },
            DocBlock::Paragraph { style_id: None, runs: vec![DocRun::plain("one two three")] },
            DocBlock::Quote { blocks: vec![DocBlock::Heading { level: 2, style_id: None, runs: vec![DocRun::plain("Nested")] }] },
            DocBlock::Table { rows: vec![DocTableRow { cells: vec![DocTableCell { blocks: vec![DocBlock::paragraph("cell text")] }] }] },
            DocBlock::List { ordered: false, items: vec![DocListItem { blocks: vec![DocBlock::paragraph("item text")] }] },
        ],
    };
    let outline = compute_semio_document_outline(&snapshot);
    assert_eq!(outline.section_outline, vec![SemioDocumentHeadingEntry { level: 1, text: "Hello World".into() }, SemioDocumentHeadingEntry { level: 2, text: "Nested".into() }]);
    // 5 top-level blocks + 1 nested heading (quote) + 1 nested paragraph (table cell) + 1 nested paragraph (list item) = 8
    assert_eq!(outline.block_count, 8);
    assert_eq!(outline.word_count, 10); // Hello World + one two three + Nested + cell text + item text
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = SemioDocumentSnapshot::default();
    assert_eq!(compute_semio_document_outline(&snapshot), compute_semio_document_outline(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(compute_semio_document_outline(&SemioDocumentSnapshot::default()), SemioDocumentOutline::default());
}
