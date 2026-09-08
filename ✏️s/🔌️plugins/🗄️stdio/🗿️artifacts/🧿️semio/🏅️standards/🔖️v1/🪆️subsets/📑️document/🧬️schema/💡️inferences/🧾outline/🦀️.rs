//! 🧾 `outline` — one named inference: this semio document's own section/word structure, the
//! same shape stdio's own `md` inference facet establishes (`section_outline` is every `Heading`
//! block found anywhere in the recursive tree, in document order, as `(level, text)`; `text` is
//! the heading's flattened run text; `block_count` is a real recursive walk counting every
//! `DocBlock` node — list items, table cells, and block-quote contents all included; `word_count`
//! is a whitespace-split word count over every Paragraph/Heading run's text plus every Code
//! block's literal text).

use crate::standards::v1::subsets::document::schema::snapshot::{DocBlock, DocRun, SemioDocumentSnapshot};

//#region 🔖️Outline
/// 🧾️ One `sectionOutline` entry — a heading's level + flattened run text.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SemioDocumentHeadingEntry {
    pub level: u8,
    pub text: String,
}

/// 🧾️ Semio document outline.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
