//! 📤️ Serialize `s.stdio.semio/v1/presentation` into a real `s.stdio.pptx` (ecma-376) snapshot —
//! the mirror of this pair's deserializer.
//!
//! Honest, documented losses (never fabricated):
//! - `masters`/`layouts` are dropped entirely — `PptxSnapshot`'s typed model has no
//!   `p:sldMaster`/`p:sldLayout` slot to receive them.
//! - `Slide::{id,layout_id,notes}` are dropped — `PptxSlide` has no field for any of the three.
//! - `SlideShape::Table` has NO pptx shape counterpart at this typed level (only `PptxShape::
//!   Other{xml}` could hold a real `p:graphicFrame` table, and fabricating well-formed OOXML table
//!   markup by hand would be codec reimplementation) — Table shapes are DROPPED on export, not
//!   silently coerced into something else.
//! - `Picture::image.{mime,bytes}` are dropped — `PptxShape::Picture` only carries a relationship
//!   id (`asset_id` reused verbatim as `blip_rel_id`), never raw bytes.
//! - `Placeholder`'s `frame`/`kind` map over; there is no block content to give its `text_frame`
//!   (`SlideShape::Placeholder` has none), so it is emitted empty.
//! - Non-`Paragraph` `DocBlock`s nested inside a `TextBox` (`Heading`/`List`/`Table`/`Code`/
//!   `Quote`/`Image`/`PageBreak`) flatten to plain paragraphs of their extracted text (pptx text
//!   frames only support flat paragraphs of runs, never nested block structure — an honest
//!   limitation of pptx's own shape, not this mapping's).

use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};
use crate::artifacts::pptx::PptxSnapshot;
use crate::artifacts::pptx::schema::snapshot::{PptxParagraph, PptxPresentation, PptxRun, PptxShape, PptxSlide, PptxTransform};
use crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::{DocBlock, DocRun};
use crate::artifacts::semio::standards::v1::subsets::presentation::schema::snapshot::{PlaceholderKind, SemioPresentationSnapshot, SlideFrame, SlideShape};
use crate::artifacts::zip::opc::OpcPackage;

//#region 🔖️FieldMapping
fn transform_from_frame(f: &SlideFrame) -> PptxTransform {
    PptxTransform { x: f.origin.x.round() as i64, y: f.origin.y.round() as i64, cx: f.width.round() as i64, cy: f.height.round() as i64 }
}

fn map_semio_run(run: &DocRun) -> PptxRun {
    PptxRun { text: run.text.clone(), bold: run.style.bold, italic: run.style.italic, font_size: run.style.size.map(|s| s.round() as u32) }
}

/// 🧱 One `DocBlock` -> zero or more `PptxParagraph`s — flattening non-`Paragraph` kinds since a
/// pptx text frame only supports flat paragraphs of runs (see module doc comment).
pub(crate) fn block_to_pptx_paragraphs(block: &DocBlock) -> Vec<PptxParagraph> {
    match block {
        DocBlock::Paragraph { runs, .. } => vec![PptxParagraph { runs: runs.iter().map(map_semio_run).collect() }],
        DocBlock::Heading { runs, .. } => vec![PptxParagraph { runs: runs.iter().map(map_semio_run).collect() }],
        DocBlock::List { items, .. } => items.iter().flat_map(|item| item.blocks.iter().flat_map(block_to_pptx_paragraphs)).collect(),
        DocBlock::Table { rows } => rows.iter().flat_map(|row| row.cells.iter().flat_map(|cell| cell.blocks.iter().flat_map(block_to_pptx_paragraphs))).collect(),
        DocBlock::Code { text, .. } => vec![PptxParagraph::text(text.clone())],
        DocBlock::Quote { blocks } => blocks.iter().flat_map(block_to_pptx_paragraphs).collect(),
        DocBlock::Image { alt, .. } => vec![PptxParagraph::text(alt.clone())],
        DocBlock::PageBreak => Vec::new(),
    }
}

/// 🏷️ `PlaceholderKind` -> pptx `ST_PlaceholderType` string (canonical form — `Title` always
/// emits `"title"`, never `"ctrTitle"`; a real, documented normalization, not data loss of
/// meaning).
pub(crate) fn placeholder_kind_to_str(kind: &PlaceholderKind) -> String {
    match kind {
        PlaceholderKind::Title => "title".into(),
        PlaceholderKind::Subtitle => "subTitle".into(),
        PlaceholderKind::Body => "body".into(),
        PlaceholderKind::Footer => "ftr".into(),
        PlaceholderKind::SlideNumber => "sldNum".into(),
        PlaceholderKind::DateTime => "dt".into(),
        PlaceholderKind::Other { value } => value.clone(),
    }
}

fn map_shape(shape: &SlideShape) -> Option<PptxShape> {
    match shape {
        SlideShape::TextBox { frame, blocks } => Some(PptxShape::TextBox { text_frame: blocks.iter().flat_map(block_to_pptx_paragraphs).collect(), position: transform_from_frame(frame) }),
        SlideShape::Picture { frame, image } => Some(PptxShape::Picture { blip_rel_id: image.asset_id.clone(), position: transform_from_frame(frame) }),
        SlideShape::Table { .. } => None,
        SlideShape::Placeholder { frame, kind } => Some(PptxShape::Placeholder { kind: placeholder_kind_to_str(kind), text_frame: Vec::new(), position: transform_from_frame(frame) }),
    }
}
//#endregion 🔖️FieldMapping

//#region 🔖️Serializer
pub struct SemioPresentationToPptx;

impl ArtifactSerializer for SemioPresentationToPptx {
    type From = SemioPresentationSnapshot;
    type Into = PptxSnapshot;
    const FROM: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("presentation") };
    const INTO: Dialect = Dialect { artifact_kind: "s.stdio.pptx", standard: StandardId("ecma-376"), subset: SubsetId::ANY };

    fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let slides = from.slides.iter().map(|slide| PptxSlide { shapes: slide.shapes.iter().filter_map(map_shape).collect() }).collect();
        Ok(PptxSnapshot::from_parts(OpcPackage::default(), PptxPresentation { slides }))
    }
}
//#endregion 🔖️Serializer

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::engine::geometry::SemioPoint2;
    use crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::RunStyle;
    use crate::artifacts::semio::standards::v1::subsets::presentation::schema::snapshot::{STDIO_SEMIOPRESENTATION_DOCUMENT_SCHEMA, Slide, SlidePictureImage};

    fn sample_semio() -> SemioPresentationSnapshot {
        SemioPresentationSnapshot {
            schema: STDIO_SEMIOPRESENTATION_DOCUMENT_SCHEMA.into(),
            masters: Vec::new(),
            layouts: Vec::new(),
            slides: vec![Slide {
                id: "slide0".into(),
                layout_id: None,
                shapes: vec![
                    SlideShape::TextBox { frame: SlideFrame { origin: SemioPoint2 { x: 0.0, y: 0.0 }, width: 100.0, height: 20.0 }, blocks: vec![DocBlock::Paragraph { style_id: None, runs: vec![DocRun { text: "Hi".into(), style: RunStyle { bold: true, ..Default::default() } }] }] },
                    SlideShape::Picture { frame: SlideFrame { origin: SemioPoint2 { x: 0.0, y: 30.0 }, width: 50.0, height: 50.0 }, image: SlidePictureImage { asset_id: "rId2".into(), mime: "image/png".into(), bytes: vec![1, 2, 3] } },
                    SlideShape::Placeholder { frame: SlideFrame { origin: SemioPoint2 { x: 0.0, y: 0.0 }, width: 200.0, height: 40.0 }, kind: PlaceholderKind::Title },
                ],
                notes: Vec::new(),
            }],
        }
    }

    #[test]
    fn maps_shapes_positions_and_placeholder_kind() {
        let pptx = SemioPresentationToPptx::serialize(&sample_semio()).expect("serialize");
        assert_eq!(pptx.presentation.slides.len(), 1);
        let shapes = &pptx.presentation.slides[0].shapes;
        assert_eq!(shapes.len(), 3);
        assert!(matches!(&shapes[0], PptxShape::TextBox { text_frame, position } if text_frame[0].runs[0].text == "Hi" && text_frame[0].runs[0].bold && position.cx == 100));
        assert!(matches!(&shapes[1], PptxShape::Picture { blip_rel_id, .. } if blip_rel_id == "rId2"));
        assert!(matches!(&shapes[2], PptxShape::Placeholder { kind, .. } if kind == "title"));
    }
}
//#endregion 🔖️Tests
