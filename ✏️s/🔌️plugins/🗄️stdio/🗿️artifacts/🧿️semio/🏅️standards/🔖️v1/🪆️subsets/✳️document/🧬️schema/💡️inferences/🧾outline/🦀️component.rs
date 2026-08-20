//! 🧾 `outline` — one named inference: this semio document's own section/word structure, the
//! same shape stdio's own `md` inference facet establishes (`section_outline` is every `Heading`
//! block found anywhere in the recursive tree, in document order, as `(level, text)`; `text` is
//! the heading's flattened run text; `block_count` is a real recursive walk counting every
//! `DocBlock` node — list items, table cells, and block-quote contents all included; `word_count`
//! is a whitespace-split word count over every Paragraph/Heading run's text plus every Code
//! block's literal text).

use crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::{DocBlock, DocRun, SemioDocumentSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Outline
/// 🧾️ One `sectionOutline` entry — a heading's level + flattened run text.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemioDocumentHeadingEntry {
    pub level: u8,
    pub text: String,
}

/// 🧾️ Semio document outline.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemioDocumentOutline {
    pub section_outline: Vec<SemioDocumentHeadingEntry>,
    pub block_count: u32,
    pub word_count: u32,
}

/// 🔤️ Concatenates a run of `DocRun`s' literal text (formatting is ignored — this is a plain-text
/// flattening, not a re-render).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn run_text(runs: &[DocRun]) -> String {
    runs.iter().map(|r| r.text.as_str()).collect::<Vec<_>>().join(" ")
}

/// 🌳️ Recursively walks `block`, appending every `Heading` encountered to `headings`, adding to
/// `block_count`, and appending flattened text to `word_source`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn walk_block(block: &DocBlock, headings: &mut Vec<SemioDocumentHeadingEntry>, block_count: &mut u32, word_source: &mut String) {
    *block_count += 1;
    match block {
        DocBlock::Heading { level, runs, .. } => {
            let text = run_text(runs);
            word_source.push(' ');
            word_source.push_str(&text);
            headings.push(SemioDocumentHeadingEntry { level: *level, text });
        }
        DocBlock::Paragraph { runs, .. } => {
            word_source.push(' ');
            word_source.push_str(&run_text(runs));
        }
        DocBlock::List { items, .. } => {
            for item in items {
                for child in &item.blocks {
                    walk_block(child, headings, block_count, word_source);
                }
            }
        }
        DocBlock::Table { rows } => {
            for row in rows {
                for cell in &row.cells {
                    for child in &cell.blocks {
                        walk_block(child, headings, block_count, word_source);
                    }
                }
            }
        }
        DocBlock::Code { text, .. } => {
            word_source.push(' ');
            word_source.push_str(text);
        }
        DocBlock::Quote { blocks } => {
            for child in blocks {
                walk_block(child, headings, block_count, word_source);
            }
        }
        DocBlock::Image { .. } | DocBlock::PageBreak => {}
    }
}

/// 🧾️ Computes [`SemioDocumentOutline`] via a recursive walk of `blocks` — see module doc comment.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn compute_semio_document_outline(snapshot: &SemioDocumentSnapshot) -> SemioDocumentOutline {
    let mut section_outline = Vec::new();
    let mut block_count = 0u32;
    let mut word_source = String::new();
    for block in &snapshot.blocks {
        walk_block(block, &mut section_outline, &mut block_count, &mut word_source);
    }
    let word_count = word_source.split_whitespace().count() as u32;
    SemioDocumentOutline { section_outline, block_count, word_count }
}
//#endregion 🔖️Outline

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::{DocListItem, DocTableCell, DocTableRow, STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA};

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
}
//#endregion 🧪️Tests
