//! 🧾 `outline` — one named inference: this PresentationML deck's own slide/shape/word
//! structure. `slideCount` is `presentation.slides.len()` verbatim; `shapeCount` is the total
//! shape count across every slide's `p:spTree` (including logical `Other` XML nodes);
//! `wordCount` is a whitespace-split word count over every `TextBox`/`Placeholder` run's `text`
//! (`Picture`/`Other` shapes carry no modeled text).

use crate::artifacts::pptx::schema::snapshot::PptxShape;
use crate::artifacts::pptx::PptxSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Outline
/// 🧾️ `Pptx` document outline.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PptxOutline {
    pub slide_count: u32,
    pub shape_count: u32,
    pub word_count: u32,
}

fn shape_word_count(shape: &PptxShape) -> u32 {
    let text_frame = match shape {
        PptxShape::TextBox { text_frame, .. } | PptxShape::Placeholder { text_frame, .. } => text_frame,
        PptxShape::Picture { .. } | PptxShape::Other { .. } => return 0,
    };
    text_frame.iter().flat_map(|paragraph| &paragraph.runs).map(|run| run.text.split_whitespace().count() as u32).sum()
}

impl PptxOutline {
    pub fn compute(snapshot: &PptxSnapshot) -> Self {
        let slide_count = snapshot.presentation.slides.len() as u32;
        let mut shape_count = 0u32;
        let mut word_count = 0u32;
        for slide in &snapshot.presentation.slides {
            shape_count += slide.shapes.len() as u32;
            word_count += slide.shapes.iter().map(shape_word_count).sum::<u32>();
        }
        Self { slide_count, shape_count, word_count }
    }
}
//#endregion 🔖️Outline

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::pptx::schema::snapshot::{PptxParagraph, PptxSlide};

    #[test]
    fn counts_slides_shapes_and_words() {
        let snapshot = PptxSnapshot {
            schema: "stdio.pptx".into(),
            opc: Default::default(),
            presentation: crate::artifacts::pptx::schema::snapshot::PptxPresentation {
                slides: vec![PptxSlide { shapes: vec![PptxShape::TextBox { text_frame: vec![PptxParagraph::text("hello world")], position: Default::default() }, PptxShape::Picture { blip_rel_id: "rId1".into(), position: Default::default() }] }],
            },
            xml_parts: Vec::new(),
        };
        let outline = PptxOutline::compute(&snapshot);
        assert_eq!(outline.slide_count, 1);
        assert_eq!(outline.shape_count, 2);
        assert_eq!(outline.word_count, 2);
    }

    #[test]
    fn outline_is_deterministic() {
        let snapshot = PptxSnapshot::default();
        assert_eq!(PptxOutline::compute(&snapshot), PptxOutline::compute(&snapshot));
    }
}
//#endregion 🧪️Tests
