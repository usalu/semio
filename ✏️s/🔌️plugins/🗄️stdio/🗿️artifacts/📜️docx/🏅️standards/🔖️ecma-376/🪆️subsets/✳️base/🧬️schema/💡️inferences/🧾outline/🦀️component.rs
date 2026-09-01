//! 🧾 `outline` — one named inference: this WordprocessingML document's own paragraph/table/word
//! structure. `paragraphCount`/`tableCount` are a real recursive walk over `document.body`
//! (table cells may themselves hold nested paragraphs/tables — both count is recursive);
//! `wordCount` is a whitespace-split word count over every run's `text`, anywhere in the tree.

use crate::artifacts::docx::schema::snapshot::DocxBlock;
use crate::artifacts::docx::DocxSnapshot;

//#region 🔖️Outline
/// 🧾️ `Docx` document outline.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocxOutline {
    pub paragraph_count: u32,
    pub table_count: u32,
    pub word_count: u32,
}

/// 🌳️ Recursively walks `blocks`, accumulating `(paragraph_count, table_count, word_count)` —
/// table cells recurse into their own `blocks`, so a paragraph nested inside a table cell is
/// counted exactly like a top-level one.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn walk_blocks(blocks: &[DocxBlock], paragraph_count: &mut u32, table_count: &mut u32, word_count: &mut u32) {
    for block in blocks {
        match block {
            DocxBlock::Paragraph(paragraph) => {
                *paragraph_count += 1;
                for run in &paragraph.runs {
                    *word_count += run.text.split_whitespace().count() as u32;
                }
            }
            DocxBlock::Table(table) => {
                *table_count += 1;
                for row in &table.rows {
                    for cell in &row.cells {
                        walk_blocks(&cell.blocks, paragraph_count, table_count, word_count);
                    }
                }
            }
        }
    }
}

impl DocxOutline {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compute(snapshot: &DocxSnapshot) -> Self {
        let mut paragraph_count = 0u32;
        let mut table_count = 0u32;
        let mut word_count = 0u32;
        walk_blocks(&snapshot.document.body, &mut paragraph_count, &mut table_count, &mut word_count);
        Self { paragraph_count, table_count, word_count }
    }
}
//#endregion 🔖️Outline

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::docx::schema::snapshot::{DocxDocument, DocxTable, DocxTableCell, DocxTableRow};

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
}
//#endregion 🧪️Tests
