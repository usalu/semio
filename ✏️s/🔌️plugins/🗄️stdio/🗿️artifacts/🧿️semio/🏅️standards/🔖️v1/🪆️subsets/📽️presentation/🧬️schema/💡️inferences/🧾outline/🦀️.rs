//! 🧾 `outline` — one named inference: this semio presentation's own heading/word structure, the
//! same shape `document`'s own outline facet establishes (`SlideShape::TextBox`/`Table` cell
//! content reuse `document::DocBlock` verbatim — this subset's own module doc comment) — walked
//! across every scope in document order (`masters`, then `layouts`, then `slides` incl. each
//! slide's own `notes`). `sectionOutline` is every `DocBlock::Heading` found anywhere in that walk;
//! `shapeCount` is every `SlideShape` visited; `blockCount` is a real recursive walk counting every
//! `DocBlock` node (table cells included); `wordCount` is a whitespace-split word count over every
//! Paragraph/Heading run's text plus every Code block's literal text plus every slide's notes.

use crate::standards::v1::subsets::document::schema::snapshot::{DocBlock, DocRun};
use crate::standards::v1::subsets::presentation::schema::snapshot::{SemioPresentationSnapshot, SlideShape};

//#region 🔖️Outline
/// 🧾️ One `sectionOutline` entry — a heading's level + flattened run text.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SemioPresentationHeadingEntry {
    pub level: u8,
    pub text: String,
}

/// 🧾️ Semio presentation outline.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SemioPresentationOutline {
    pub section_outline: Vec<SemioPresentationHeadingEntry>,
    pub slide_count: u32,
    pub shape_count: u32,
    pub block_count: u32,
    pub word_count: u32,
}

/// 🔤️ Concatenates a run of `DocRun`s' literal text (formatting is ignored — a plain-text
/// flattening, not a re-render).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn run_text(runs: &[DocRun]) -> String {
    runs.iter().map(|r| r.text.as_str()).collect::<Vec<_>>().join(" ")
}

/// 🌳️ Recursively walks `block`, appending every `Heading` encountered to `headings`, adding to
/// `block_count`, and appending flattened text to `word_source` — same shape `document`'s own
/// `walk_block` establishes.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn walk_block(block: &DocBlock, headings: &mut Vec<SemioPresentationHeadingEntry>, block_count: &mut u32, word_source: &mut String) {
    *block_count += 1;
    match block {
        DocBlock::Heading { level, runs, .. } => {
            let text = run_text(runs);
            word_source.push(' ');
            word_source.push_str(&text);
            headings.push(SemioPresentationHeadingEntry { level: *level, text });
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

/// 🧩️ Walks every shape's own text-bearing content (`TextBox.blocks`, `Table` cell blocks) —
/// `Picture`/`Placeholder` carry no block content, only a `frame` + non-textual payload.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn walk_shape(shape: &SlideShape, headings: &mut Vec<SemioPresentationHeadingEntry>, block_count: &mut u32, word_source: &mut String) {
    match shape {
        SlideShape::TextBox { blocks, .. } => {
            for block in blocks {
                walk_block(block, headings, block_count, word_source);
            }
        }
        SlideShape::Table { rows, .. } => {
            for row in rows {
                for cell in &row.cells {
                    for block in &cell.blocks {
                        walk_block(block, headings, block_count, word_source);
                    }
                }
            }
        }
        SlideShape::Picture { .. } | SlideShape::Placeholder { .. } => {}
    }
}

/// 🧾️ Computes [`SemioPresentationOutline`] via a recursive walk across `masters`, `layouts`,
/// `slides` (incl. each slide's own `notes`) — see module doc comment.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn compute_semio_presentation_outline(snapshot: &SemioPresentationSnapshot) -> SemioPresentationOutline {
    let mut section_outline = Vec::new();
    let mut block_count = 0u32;
    let mut shape_count = 0u32;
    let mut word_source = String::new();

    for master in &snapshot.masters {
        for shape in &master.shapes {
            shape_count += 1;
            walk_shape(shape, &mut section_outline, &mut block_count, &mut word_source);
        }
    }
    for layout in &snapshot.layouts {
        for shape in &layout.shapes {
            shape_count += 1;
            walk_shape(shape, &mut section_outline, &mut block_count, &mut word_source);
        }
    }
    for slide in &snapshot.slides {
        for shape in &slide.shapes {
            shape_count += 1;
            walk_shape(shape, &mut section_outline, &mut block_count, &mut word_source);
        }
        for block in &slide.notes {
            walk_block(block, &mut section_outline, &mut block_count, &mut word_source);
        }
    }

    let word_count = word_source.split_whitespace().count() as u32;
    SemioPresentationOutline { section_outline, slide_count: snapshot.slides.len() as u32, shape_count, block_count, word_count }
}
//#endregion 🔖️Outline

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
