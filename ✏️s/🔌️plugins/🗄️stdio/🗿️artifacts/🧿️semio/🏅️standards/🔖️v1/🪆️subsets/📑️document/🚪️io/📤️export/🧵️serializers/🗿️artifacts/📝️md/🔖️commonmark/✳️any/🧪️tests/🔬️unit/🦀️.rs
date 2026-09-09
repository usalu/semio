use super::*;
use crate::standards::v1::subsets::document::schema::snapshot::{DocListItem, DocTableCell, DocTableRow, RunStyle, STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_semio() -> SemioDocumentSnapshot {
    SemioDocumentSnapshot {
        schema: STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA.into(),
        styles: Vec::new(),
        images: Vec::new(),
        blocks: vec![
            DocBlock::Heading { level: 2, style_id: None, runs: vec![DocRun { text: "Section".into(), style: RunStyle { bold: true, ..Default::default() } }] },
            DocBlock::Paragraph { style_id: None, runs: vec![DocRun::plain("plain text")] },
            DocBlock::List { ordered: false, items: vec![DocListItem { blocks: vec![DocBlock::paragraph("item")] }] },
            DocBlock::Code { language: Some("rust".into()), text: "fn main() {}".into() },
            DocBlock::Quote { blocks: vec![DocBlock::paragraph("quoted")] },
            DocBlock::Table { rows: vec![DocTableRow { cells: vec![DocTableCell { blocks: vec![DocBlock::paragraph("cell")] }] }] },
        ],
    }
}

#[semio_framework_async_macros::async_test]
async fn maps_headings_lists_code_quotes_and_flattens_tables() {
    let md = semio_framework_plugin::resolve_ready(SemioDocumentToMd::serialize(&sample_semio())).expect("serialize");
    assert!(matches!(&md.blocks[0], MdBlock::Heading { level: 2, inlines } if matches!(&inlines[0], MdInline::Strong { .. })));
    assert!(matches!(&md.blocks[1], MdBlock::Paragraph { .. }));
    assert!(matches!(&md.blocks[2], MdBlock::List { ordered: false, items, .. } if items.len() == 1));
    assert!(matches!(&md.blocks[3], MdBlock::CodeBlock { info: Some(l), .. } if l == "rust"));
    assert!(matches!(&md.blocks[4], MdBlock::BlockQuote { blocks } if blocks.len() == 1));
    // table flattens to a single paragraph (its one cell's one block), not a table construct.
    assert!(matches!(&md.blocks[5], MdBlock::Paragraph { .. }));
}
