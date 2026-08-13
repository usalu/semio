//! 📥️ Deserialize `s.stdio.semio/v1/presentation` from a real `s.stdio.pptx` (ecma-376) snapshot
//! — pptx's own slide/shape tree maps closely, presentation's W2 design having been informed
//! directly by pptx per the master plan.
//!
//! Honest, documented losses (never fabricated):
//! - `masters`/`layouts` are always empty on import: `PptxSnapshot`'s typed model has no
//!   `p:sldMaster`/`p:sldLayout` view (only the resolved slide list) — that structure lives, if at
//!   all, inside unmodeled `opc` parts, out of reach without re-parsing bytes.
//! - `Slide::id` is synthesized as a positional index (`"slide{i}"`) — pptx's typed `PptxSlide` has
//!   no id field; `layout_id`/`notes` are always `None`/empty for the same reason
//!   (`p:notesSlide` parts are not modeled at this level).
//! - `PptxShape::Other` (raw retention for `p:graphicFrame` charts/tables/SmartArt, `p:grpSp`
//!   groups, `p:cxnSp` connectors, anything unrecognized) has no `SlideShape` counterpart and is
//!   DROPPED on import — recovering a typed shape from its raw XML would be codec
//!   reimplementation, which this leaf must not do.
//! - `Picture::image.bytes`/`mime` are always empty: `PptxShape::Picture::blip_rel_id` is only a
//!   relationship id (resolves through `.rels` to the real media part, which lives in unmodeled
//!   `opc` parts) — carried over as `asset_id` so the identity survives, but real bytes are never
//!   fabricated.
//! - `Placeholder`'s own text content (`text_frame`) is dropped: `SlideShape::Placeholder` has no
//!   block-content field to receive it (a real, spec-mandated shape limitation of this subset's
//!   own type, not invented here).

use semio_framework_plugin::{ArtifactDeserializer, Dialect, StandardId, SubsetId};
use crate::artifacts::pptx::PptxSnapshot;
use crate::artifacts::pptx::schema::snapshot::{PptxParagraph, PptxRun, PptxShape, PptxTransform};
use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint2;
use crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::{DocBlock, DocRun, RunStyle};
use crate::artifacts::semio::standards::v1::subsets::presentation::schema::snapshot::{PlaceholderKind, SemioPresentationSnapshot, Slide, SlideFrame, SlidePictureImage, SlideShape, STDIO_SEMIOPRESENTATION_DOCUMENT_SCHEMA};

//#region 🔖️FieldMapping
fn frame_from_transform(t: &PptxTransform) -> SlideFrame {
    SlideFrame { origin: SemioPoint2 { x: t.x as f64, y: t.y as f64 }, width: t.cx as f64, height: t.cy as f64 }
}

fn map_run(run: &PptxRun) -> DocRun {
    DocRun { text: run.text.clone(), style: RunStyle { bold: run.bold, italic: run.italic, underline: false, size: run.font_size.map(|v| v as f64), font: None, color: None, link: None } }
}

fn map_text_frame(paragraphs: &[PptxParagraph]) -> Vec<DocBlock> {
    paragraphs.iter().map(|p| DocBlock::Paragraph { style_id: None, runs: p.runs.iter().map(map_run).collect() }).collect()
}

/// 🏷️ pptx placeholder type strings (ECMA-376 `ST_PlaceholderType`) -> `PlaceholderKind`.
pub(crate) fn placeholder_kind_from_str(kind: &str) -> PlaceholderKind {
    match kind {
        "title" | "ctrTitle" => PlaceholderKind::Title,
        "subTitle" => PlaceholderKind::Subtitle,
        "body" => PlaceholderKind::Body,
        "ftr" => PlaceholderKind::Footer,
        "sldNum" => PlaceholderKind::SlideNumber,
        "dt" => PlaceholderKind::DateTime,
        other => PlaceholderKind::Other { value: other.to_string() },
    }
}

fn map_shape(shape: &PptxShape) -> Option<SlideShape> {
    match shape {
        PptxShape::TextBox { text_frame, position } => Some(SlideShape::TextBox { frame: frame_from_transform(position), blocks: map_text_frame(text_frame) }),
        PptxShape::Picture { blip_rel_id, position } => Some(SlideShape::Picture { frame: frame_from_transform(position), image: SlidePictureImage { asset_id: blip_rel_id.clone(), mime: String::new(), bytes: Vec::new() } }),
        PptxShape::Placeholder { kind, position, .. } => Some(SlideShape::Placeholder { frame: frame_from_transform(position), kind: placeholder_kind_from_str(kind) }),
        PptxShape::Other { .. } => None,
    }
}
//#endregion 🔖️FieldMapping

//#region 🔖️Deserializer
pub struct SemioPresentationFromPptx;

impl ArtifactDeserializer for SemioPresentationFromPptx {
    type From = PptxSnapshot;
    type Into = SemioPresentationSnapshot;
    const FROM: Dialect = Dialect { artifact_kind: "s.stdio.pptx", standard: StandardId("ecma-376"), subset: SubsetId::ANY };
    const INTO: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("presentation") };

    fn deserialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let slides = from
            .presentation
            .slides
            .iter()
            .enumerate()
            .map(|(i, slide)| Slide { id: format!("slide{i}"), layout_id: None, shapes: slide.shapes.iter().filter_map(map_shape).collect(), notes: Vec::new() })
            .collect();
        Ok(SemioPresentationSnapshot { schema: STDIO_SEMIOPRESENTATION_DOCUMENT_SCHEMA.into(), masters: Vec::new(), layouts: Vec::new(), slides })
    }
}
//#endregion 🔖️Deserializer

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::pptx::schema::snapshot::{PptxPresentation, PptxSlide};
    use crate::artifacts::zip::opc::OpcPackage;

    pub(crate) fn sample_pptx() -> PptxSnapshot {
        PptxSnapshot::from_parts(
            OpcPackage::default(),
            PptxPresentation {
                slides: vec![PptxSlide {
                    shapes: vec![
                        PptxShape::TextBox { text_frame: vec![PptxParagraph { runs: vec![PptxRun { text: "Hello".into(), bold: true, italic: false, font_size: Some(24) }] }], position: PptxTransform { x: 0, y: 0, cx: 100, cy: 20 } },
                        PptxShape::Picture { blip_rel_id: "rId2".into(), position: PptxTransform { x: 0, y: 30, cx: 50, cy: 50 } },
                        PptxShape::Placeholder { kind: "title".into(), text_frame: vec![PptxParagraph::text("Title text")], position: PptxTransform { x: 0, y: 0, cx: 200, cy: 40 } },
                        PptxShape::Other { xml: "<p:graphicFrame/>".into() },
                    ],
                }],
            },
        )
    }

    #[test]
    fn maps_shapes_and_drops_other() {
        let semio = SemioPresentationFromPptx::deserialize(&sample_pptx()).expect("deserialize");
        assert!(semio.masters.is_empty() && semio.layouts.is_empty());
        assert_eq!(semio.slides.len(), 1);
        let slide = &semio.slides[0];
        assert_eq!(slide.id, "slide0");
        assert_eq!(slide.shapes.len(), 3, "Other{{xml}} shape must be dropped");
        assert!(matches!(&slide.shapes[0], SlideShape::TextBox { blocks, .. } if matches!(&blocks[0], DocBlock::Paragraph { runs, .. } if runs[0].text == "Hello" && runs[0].style.bold)));
        assert!(matches!(&slide.shapes[1], SlideShape::Picture { image, .. } if image.asset_id == "rId2" && image.bytes.is_empty()));
        assert!(matches!(&slide.shapes[2], SlideShape::Placeholder { kind: PlaceholderKind::Title, .. }));
    }
}
//#endregion 🔖️Tests
