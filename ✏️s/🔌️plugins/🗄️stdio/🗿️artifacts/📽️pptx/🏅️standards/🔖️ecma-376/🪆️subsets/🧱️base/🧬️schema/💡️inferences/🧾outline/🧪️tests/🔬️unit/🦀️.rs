
use super::*;
use crate::schema::snapshot::{PptxParagraph, PptxSlide};

#[semio_framework_async_macros::async_test]
async fn counts_slides_shapes_and_words() {
    let snapshot = PptxSnapshot {
        schema: "stdio.pptx".into(),
        opc: Default::default(),
        presentation: crate::schema::snapshot::PptxPresentation {
            slides: vec![PptxSlide { shapes: vec![PptxShape::TextBox { text_frame: vec![PptxParagraph::text("hello world")], position: Default::default() }, PptxShape::Picture { blip_rel_id: "rId1".into(), position: Default::default() }] }],
        },
        xml_parts: Vec::new(),
    };
    let outline = PptxOutline::compute(&snapshot);
    assert_eq!(outline.slide_count, 1);
    assert_eq!(outline.shape_count, 2);
    assert_eq!(outline.word_count, 2);
}

#[semio_framework_async_macros::async_test]
async fn outline_is_deterministic() {
    let snapshot = PptxSnapshot::default();
    assert_eq!(PptxOutline::compute(&snapshot), PptxOutline::compute(&snapshot));
}
