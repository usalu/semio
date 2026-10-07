//! 🧬️ SemioPresentationSnapshot — masters/layouts/slides -> shapes (TextBox/Picture/Table/
//! Placeholder) + per-slide notes — from pptx. `SlideShape::TextBox`/`Table` cell content
//! deliberately REUSE `document`'s `DocBlock` per the master plan's spec-mandated cross-reuse note
//! ("presentation mirrors document's block shape with own types" — the shape types themselves
//! (`SlideMaster`/`SlideLayout`/`Slide`/`SlideShape`) are owned here; only the block-tree LEAF is
//! shared, per `w1b-type-ownership.md`).

use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;


use crate::standards::v1::subsets::document::schema::snapshot::DocBlock;










use framework_schema::ArtifactSchema;

//#region 🔖️Geometry
/// 📐️ A shape's on-slide placement: top-left `origin` (EMU-agnostic plane coordinates, matching
/// pptx's `a:off`/`a:ext`) + `width`/`height` (matching `a:ext`). Reuses the shared engine's
/// `SemioPoint2` for the position field per the type-ownership doc's geometry rule; `width`/
/// `height` stay plain `f64` (a size is not itself a position, and the shared engine has no `Size`
/// type — inventing a two-field wrapper here would just be a bare-tuple-in-disguise).
#[derive(Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SlideFrame {
    pub origin: SemioPoint2,
    pub width: f64,
    pub height: f64,
}
//#endregion 🔖️Geometry

//#region 🔖️Shapes
/// 🖼️ An embedded raster image (pptx `p:pic` -> `a:blip` target part), self-contained (no
/// cross-reference to the `image` subset — presentation embeds its own media parts, same as pptx
/// itself does not share media storage with other OOXML packages).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SlidePictureImage {
    pub asset_id: String,
    pub mime: String,
    #[value(default)]
    pub bytes: Vec<u8>,
}

/// 🏷️ pptx placeholder type (`p:ph/@type`), the subset every named placeholder in a layout/slide
/// declares itself as.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum PlaceholderKind {
    Title,
    Subtitle,
    Body,
    Footer,
    SlideNumber,
    DateTime,
    Other { value: String },
}

/// 🔲️ One `a:tc` table cell — holds its own block content, reusing `document`'s `DocBlock` (same
/// cross-reuse the master plan calls out for `TextBox`; a table cell's text content is shaped
/// identically to a text box's).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SlideTableCell {
    #[value(default)]
    pub blocks: Vec<DocBlock>,
}

/// ➖️ One `a:tr` table row.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SlideTableRow {
    #[value(default)]
    pub cells: Vec<SlideTableCell>,
}

/// 🧩️ One shape on a master/layout/🎞️slide's shape tree (pptx `p:spTree` children) — the master
/// plan's four kinds: `TextBox`, `Picture`, `Table`, `Placeholder`. Tag is `shapeKind` (not
/// `kind`) because the `Placeholder` variant's own field is itself named `kind` (its pptx
/// placeholder type) — an internally-tagged enum's tag name must not collide with any variant's
/// own field name, so this avoids the collision rather than renaming the more-natural field.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "shapeKind", rename_all = "camelCase")]
pub enum SlideShape {
    /// ✍️ `p:sp` with a text body — `blocks` reuses `document::DocBlock` verbatim (spec-mandated
    /// cross-reuse, see module doc comment).
    TextBox {
        frame: SlideFrame,
        #[value(default)]
        blocks: Vec<DocBlock>,
    },
    /// 🖼️ `p:pic`.
    Picture { frame: SlideFrame, image: SlidePictureImage },
    /// 🏛️ `p:graphicFrame` holding `a:tbl`.
    Table {
        frame: SlideFrame,
        #[value(default)]
        rows: Vec<SlideTableRow>,
    },
    /// 🏷️ `p:sp` with a `p:ph` placeholder reference.
    Placeholder { frame: SlideFrame, kind: PlaceholderKind },
}
//#endregion 🔖️Shapes

//#region 🔖️Structure
/// 🗂️ One `p:sldMaster` — id-keyed (matches pptx's own part-relationship identity), a shape tree.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SlideMaster {
    pub id: String,
    #[value(default)]
    pub shapes: Vec<SlideShape>,
}

/// 📐️ One `p:sldLayout` — references its owning master by id (`master_id`), like pptx's
/// layout-to-master relationship part.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SlideLayout {
    pub id: String,
    pub master_id: String,
    #[value(default)]
    pub shapes: Vec<SlideShape>,
}

/// 🎞️ One `p:sld` — ordered (presentation order is significant, like pdf page order), so `id` is
/// carried as the slide's own persistent identity while the COLLECTION itself is index-addressed
/// (see the diff facet's `SlidesDiff` for why: an index-keyed collection, not name-keyed).
/// `notes` is the slide's own `p:notesSlide` content (one notes page per slide in pptx, so it is
/// modeled per-slide rather than as a top-level sibling collection).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Slide {
    pub id: String,
    #[value(default)]
    pub layout_id: Option<String>,
    #[value(default)]
    pub shapes: Vec<SlideShape>,
    #[value(default)]
    pub notes: Vec<DocBlock>,
}
//#endregion 🔖️Structure

//#region 🔖️Ids
pub const STDIO_SEMIOPRESENTATION_DOCUMENT_SCHEMA: &str = "s.stdio.semio.presentation";
//#endregion 🔖️Ids

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.presentation")]
pub struct SemioPresentationSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub masters: Vec<SlideMaster>,
    #[state(artifact)]
    #[value(default)]
    pub layouts: Vec<SlideLayout>,
    #[state(artifact)]
    #[value(default)]
    pub slides: Vec<Slide>,
}

impl Default for SemioPresentationSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_SEMIOPRESENTATION_DOCUMENT_SCHEMA.into(), masters: Vec::new(), layouts: Vec::new(), slides: Vec::new() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️TextCodec


//#endregion 🔖️TextCodec

//#region 🔖️BinaryCodec




































//#endregion 🔖️BinaryCodec

//#region 🔖️HandcraftedArtifactCodecs



//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️ReachableCodecs




//#endregion 🔖️ReachableCodecs

//#region 🔖️Demo
/// 🌱 The demo `s.stdio.semio.presentation` snapshot — masters/layouts/slides all populated,
/// exercising every `SlideShape` variant (incl. `Table`) and every `PlaceholderKind` variant (incl.
/// `Other`), plus the `document::DocBlock` reuse in `TextBox.blocks`/table cell `blocks`/
/// `Slide.notes`. Single source of truth for `📚️examples/…/🖼️assets/🗣️.dsl.semio`/
/// `🎒️.pack.semio` and for the conformance-law tests in `🎹️composer/🦀️.rs`.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_semio_presentation_snapshot() -> SemioPresentationSnapshot {
    let frame = SlideFrame { origin: SemioPoint2 { x: 1.0, y: 2.0 }, width: 50.0, height: 10.0 };
    SemioPresentationSnapshot {
        schema: STDIO_SEMIOPRESENTATION_DOCUMENT_SCHEMA.into(),
        masters: vec![SlideMaster { id: "master1".into(), shapes: vec![SlideShape::Placeholder { frame: SlideFrame { origin: SemioPoint2 { x: 0.0, y: 0.0 }, width: 100.0, height: 20.0 }, kind: PlaceholderKind::Title }] }],
        layouts: vec![SlideLayout {
            id: "layout1".into(),
            master_id: "master1".into(),
            shapes: vec![SlideShape::Placeholder { frame: SlideFrame { origin: SemioPoint2 { x: 0.0, y: 30.0 }, width: 100.0, height: 15.0 }, kind: PlaceholderKind::Subtitle }],
        }],
        slides: vec![Slide {
            id: "slide1".into(),
            layout_id: Some("layout1".into()),
            shapes: vec![
                SlideShape::TextBox { frame, blocks: vec![DocBlock::paragraph("Hello Slide")] },
                SlideShape::Picture { frame: SlideFrame { origin: SemioPoint2 { x: 0.0, y: 0.0 }, width: 10.0, height: 10.0 }, image: SlidePictureImage { asset_id: "img1".into(), mime: "image/png".into(), bytes: vec![1, 2, 3] } },
                SlideShape::Table { frame: SlideFrame { origin: SemioPoint2 { x: 0.0, y: 0.0 }, width: 30.0, height: 30.0 }, rows: vec![SlideTableRow { cells: vec![SlideTableCell { blocks: vec![DocBlock::paragraph("cell")] }] }] },
                SlideShape::Placeholder { frame: SlideFrame { origin: SemioPoint2 { x: 0.0, y: 40.0 }, width: 100.0, height: 10.0 }, kind: PlaceholderKind::Other { value: "custom".into() } },
            ],
            notes: vec![DocBlock::paragraph("Speaker notes")],
        }],
    }
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests





