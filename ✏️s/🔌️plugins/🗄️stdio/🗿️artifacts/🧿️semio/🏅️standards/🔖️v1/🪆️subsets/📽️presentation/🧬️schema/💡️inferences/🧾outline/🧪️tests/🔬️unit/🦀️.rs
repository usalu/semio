
use super::*;
use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use crate::standards::v1::subsets::presentation::schema::snapshot::{PlaceholderKind, STDIO_SEMIOPRESENTATION_DOCUMENT_SCHEMA, Slide, SlideFrame, SlideLayout, SlideMaster, SlideTableCell, SlideTableRow};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn frame() -> SlideFrame {
    SlideFrame { origin: SemioPoint2 { x: 0.0, y: 0.0 }, width: 10.0, height: 10.0 }
}

#[semio_framework_async_macros::async_test]
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

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = SemioPresentationSnapshot::default();
    assert_eq!(compute_semio_presentation_outline(&snapshot), compute_semio_presentation_outline(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(compute_semio_presentation_outline(&SemioPresentationSnapshot::default()), SemioPresentationOutline::default());
}
