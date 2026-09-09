use super::*;
use crate::schema::snapshot::{DocxDocument, DocxTable, DocxTableCell, DocxTableRow};

#[semio_framework_async_macros::async_test]
async fn counts_paragraphs_tables_and_words_including_nested_cells() {
    let snapshot = DocxSnapshot {
        schema: "stdio.docx".into(),
        opc: Default::default(),
        document: DocxDocument {
            body: vec![
                DocxBlock::paragraph("hello world"),
                DocxBlock::Table(DocxTable {
                    rows: vec![DocxTableRow { cells: vec![DocxTableCell { blocks: vec![DocxBlock::paragraph("nested cell text")], extra_cell_properties: vec![] }], extra_row_properties: vec![] }],
                    extra_table_properties: vec![],
                }),
            ],
            styles: vec![],
        },
    };
    let outline = DocxOutline::compute(&snapshot);
    assert_eq!(outline.paragraph_count, 2);
    assert_eq!(outline.table_count, 1);
    assert_eq!(outline.word_count, 5); // "hello world" (2) + "nested cell text" (3)
}

#[semio_framework_async_macros::async_test]
async fn outline_is_deterministic() {
    let snapshot = DocxSnapshot::default();
    assert_eq!(DocxOutline::compute(&snapshot), DocxOutline::compute(&snapshot));
}
