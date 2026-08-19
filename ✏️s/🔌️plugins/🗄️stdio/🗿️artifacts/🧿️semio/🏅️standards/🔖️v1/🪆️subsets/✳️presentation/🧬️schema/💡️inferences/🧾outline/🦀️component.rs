//! 🧾 `outline` — one named inference: this semio presentation's own heading/word structure, the
//! same shape `document`'s own outline facet establishes (`SlideShape::TextBox`/`Table` cell
//! content reuse `document::DocBlock` verbatim — this subset's own module doc comment) — walked
//! across every scope in document order (`masters`, then `layouts`, then `slides` incl. each
//! slide's own `notes`). `sectionOutline` is every `DocBlock::Heading` found anywhere in that walk;
//! `shapeCount` is every `SlideShape` visited; `blockCount` is a real recursive walk counting every
//! `DocBlock` node (table cells included); `wordCount` is a whitespace-split word count over every
//! Paragraph/Heading run's text plus every Code block's literal text plus every slide's notes.

use crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::{DocBlock, DocRun};
use crate::artifacts::semio::standards::v1::subsets::presentation::schema::snapshot::{SemioPresentationSnapshot, SlideShape};
use serde::{Deserialize, Serialize};

//#region 🔖️Outline
/// 🧾️ One `sectionOutline` entry — a heading's level + flattened run text.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemioPresentationHeadingEntry {
    pub level: u8,
    pub text: String,
}

/// 🧾️ Semio presentation outline.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemioPresentationOutline {
    pub section_outline: Vec<SemioPresentationHeadingEntry>,
    pub slide_count: u32,
    pub shape_count: u32,
    pub block_count: u32,
    pub word_count: u32,
}

/// 🔤️ Concatenates a run of `DocRun`s' literal text (formatting is ignored — a plain-text
/// flattening, not a re-render).
async fn run_text(runs: &[DocRun]) -> String {
    runs.iter().map(|r| r.text.as_str()).collect::<Vec<_>>().join(" ")
}

/// 🌳️ Recursively walks `block`, appending every `Heading` encountered to `headings`, adding to
/// `block_count`, and appending flattened text to `word_source` — same shape `document`'s own
/// `walk_block` establishes.
async fn walk_block(block: &DocBlock, headings: &mut Vec<SemioPresentationHeadingEntry>, block_count: &mut u32, word_source: &mut String) {
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
async fn walk_shape(shape: &SlideShape, headings: &mut Vec<SemioPresentationHeadingEntry>, block_count: &mut u32, word_source: &mut String) {
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
pub async fn compute_semio_presentation_outline(snapshot: &SemioPresentationSnapshot) -> SemioPresentationOutline {
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
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint2;
    use crate::artifacts::semio::standards::v1::subsets::presentation::schema::snapshot::{PlaceholderKind, Slide, SlideFrame, SlideLayout, SlideMaster, SlideTableCell, SlideTableRow, STDIO_SEMIOPRESENTATION_DOCUMENT_SCHEMA};

    async fn frame() -> SlideFrame {
        SlideFrame { origin: SemioPoint2 { x: 0.0, y: 0.0 }, width: 10.0, height: 10.0 }
    }

    #[test]
    async fn collects_headings_and_counts_across_masters_layouts_slides_and_notes() {
        let snapshot = SemioPresentationSnapshot {
            schema: STDIO_SEMIOPRESENTATION_DOCUMENT_SCHEMA.into(),
            masters: vec![SlideMaster { id: "m1".into(), shapes: vec![SlideShape::TextBox { frame: frame(), blocks: vec![DocBlock::Heading { level: 1, style_id: None, runs: vec![DocRun::plain("Title Master")] }] }] }],
            layouts: vec![SlideLayout { id: "l1".into(), master_id: "m1".into(), shapes: vec![SlideShape::Placeholder { frame: frame(), kind: PlaceholderKind::Title }] }],
            slides: vec![Slide {
                id: "s1".into(),
                layout_id: Some("l1".into()),
                shapes: vec![
                    SlideShape::TextBox { frame: frame(), blocks: vec![DocBlock::Heading { level: 2, style_id: None, runs: vec![DocRun::plain("Slide Heading")] }, DocBlock::paragraph("one two three")] },
                    SlideShape::Table { frame: frame(), rows: vec![SlideTableRow { cells: vec![SlideTableCell { blocks: vec![DocBlock::paragraph("cell text")] }] }] },
                ],
                notes: vec![DocBlock::paragraph("speaker notes")],
            }],
        };
        let outline = compute_semio_presentation_outline(&snapshot);
        assert_eq!(outline.section_outline, vec![SemioPresentationHeadingEntry { level: 1, text: "Title Master".into() }, SemioPresentationHeadingEntry { level: 2, text: "Slide Heading".into() },]);
        assert_eq!(outline.slide_count, 1);
        assert_eq!(outline.shape_count, 4); // 1 master TextBox + 1 layout Placeholder + 1 slide TextBox + 1 slide Table
        assert_eq!(outline.block_count, 5); // Title Master heading + Slide Heading + paragraph + table-cell paragraph + notes paragraph
        assert_eq!(outline.word_count, 11); // "Title Master"(2) + "Slide Heading"(2) + "one two three"(3) + "cell text"(2) + "speaker notes"(2)
    }

    #[test]
    async fn inference_determinism_law() {
        let snapshot = SemioPresentationSnapshot::default();
        assert_eq!(compute_semio_presentation_outline(&snapshot), compute_semio_presentation_outline(&snapshot));
    }

    #[test]
    async fn inference_default_law() {
        assert_eq!(compute_semio_presentation_outline(&SemioPresentationSnapshot::default()), SemioPresentationOutline::default());
    }
}
//#endregion 🧪️Tests
