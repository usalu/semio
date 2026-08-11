//! 🧬️ SemioPresentationSnapshot — masters/layouts/slides -> shapes (TextBox/Picture/Table/
//! Placeholder) + per-slide notes — from pptx. `SlideShape::TextBox`/`Table` cell content
//! deliberately REUSE `document`'s `DocBlock` per the master plan's spec-mandated cross-reuse note
//! ("presentation mirrors document's block shape with own types" — the shape types themselves
//! (`SlideMaster`/`SlideLayout`/`Slide`/`SlideShape`) are owned here; only the block-tree LEAF is
//! shared, per `w1b-type-ownership.md`).

use crate::artifacts::semio::standards::v1::engine::geometry::SemioPoint2;
use crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::DocBlock;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Geometry
/// 📐️ A shape's on-slide placement: top-left `origin` (EMU-agnostic plane coordinates, matching
/// pptx's `a:off`/`a:ext`) + `width`/`height` (matching `a:ext`). Reuses the shared engine's
/// `SemioPoint2` for the position field per the type-ownership doc's geometry rule; `width`/
/// `height` stay plain `f64` (a size is not itself a position, and the shared engine has no `Size`
/// type — inventing a two-field wrapper here would just be a bare-tuple-in-disguise).
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SlidePictureImage {
    pub asset_id: String,
    pub mime: String,
    #[serde(default)]
    pub bytes: Vec<u8>,
}

/// 🏷️ pptx placeholder type (`p:ph/@type`), the subset every named placeholder in a layout/slide
/// declares itself as.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SlideTableCell {
    #[serde(default)]
    pub blocks: Vec<DocBlock>,
}

/// ➖️ One `a:tr` table row.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SlideTableRow {
    #[serde(default)]
    pub cells: Vec<SlideTableCell>,
}

/// 🧩️ One shape on a master/layout/slide's shape tree (pptx `p:spTree` children) — the master
/// plan's four kinds: `TextBox`, `Picture`, `Table`, `Placeholder`. Tag is `shapeKind` (not
/// `kind`) because the `Placeholder` variant's own field is itself named `kind` (its pptx
/// placeholder type) — an internally-tagged enum's tag name must not collide with any variant's
/// own field name, so this avoids the collision rather than renaming the more-natural field.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "shapeKind", rename_all = "camelCase")]
pub enum SlideShape {
    /// ✍️ `p:sp` with a text body — `blocks` reuses `document::DocBlock` verbatim (spec-mandated
    /// cross-reuse, see module doc comment).
    TextBox { frame: SlideFrame, #[serde(default)] blocks: Vec<DocBlock> },
    /// 🖼️ `p:pic`.
    Picture { frame: SlideFrame, image: SlidePictureImage },
    /// 🏛️ `p:graphicFrame` holding `a:tbl`.
    Table { frame: SlideFrame, #[serde(default)] rows: Vec<SlideTableRow> },
    /// 🏷️ `p:sp` with a `p:ph` placeholder reference.
    Placeholder { frame: SlideFrame, kind: PlaceholderKind },
}
//#endregion 🔖️Shapes

//#region 🔖️Structure
/// 🗂️ One `p:sldMaster` — id-keyed (matches pptx's own part-relationship identity), a shape tree.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SlideMaster {
    pub id: String,
    #[serde(default)]
    pub shapes: Vec<SlideShape>,
}

/// 📐️ One `p:sldLayout` — references its owning master by id (`master_id`), like pptx's
/// layout-to-master relationship part.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SlideLayout {
    pub id: String,
    pub master_id: String,
    #[serde(default)]
    pub shapes: Vec<SlideShape>,
}

/// 🎞️ One `p:sld` — ordered (presentation order is significant, like pdf page order), so `id` is
/// carried as the slide's own persistent identity while the COLLECTION itself is index-addressed
/// (see the diff facet's `SlidesDiff` for why: an index-keyed collection, not name-keyed).
/// `notes` is the slide's own `p:notesSlide` content (one notes page per slide in pptx, so it is
/// modeled per-slide rather than as a top-level sibling collection).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Slide {
    pub id: String,
    #[serde(default)]
    pub layout_id: Option<String>,
    #[serde(default)]
    pub shapes: Vec<SlideShape>,
    #[serde(default)]
    pub notes: Vec<DocBlock>,
}
//#endregion 🔖️Structure

//#region 🔖️Ids
pub const STDIO_SEMIOPRESENTATION_DOCUMENT_SCHEMA: &str = "s.stdio.semio.presentation";
//#endregion 🔖️Ids

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.presentation")]
pub struct SemioPresentationSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub masters: Vec<SlideMaster>,
    #[state(persistent)]
    #[serde(default)]
    pub layouts: Vec<SlideLayout>,
    #[state(persistent)]
    #[serde(default)]
    pub slides: Vec<Slide>,
}

impl Default for SemioPresentationSnapshot {
    fn default() -> Self {
        Self {
            schema: STDIO_SEMIOPRESENTATION_DOCUMENT_SCHEMA.into(),
            masters: Vec::new(),
            layouts: Vec::new(),
            slides: Vec::new(),
        }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
/// 📦️ JSON-pack round trip wrapped in the shared `store::semio_format` envelope — honest for a
/// NEUTRAL semio type (not an on-disk file format with its own byte grammar), the same convention
/// `document`'s own snapshot uses.
impl store::ArtifactDsl for SemioPresentationSnapshot {
    const EXTENSION: &'static str = "semio";
    fn envelope_id() -> &'static str { STDIO_SEMIOPRESENTATION_DOCUMENT_SCHEMA }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let hex: String = body.chars().filter(|c| !c.is_whitespace()).collect();
        if hex.len() % 2 != 0 {
            return Err(store::TextError::new("odd hex length", dsl::TextSpan::at(1, 1)));
        }
        let mut bytes = Vec::with_capacity(hex.len() / 2);
        let mut i = 0usize;
        while i < hex.len() {
            let byte = u8::from_str_radix(&hex[i..i + 2], 16)
                .map_err(|e| store::TextError::new(format!("invalid hex: {e}"), dsl::TextSpan::at(1, 1)))?;
            bytes.push(byte);
            i += 2;
        }
        serde_json::from_slice(&bytes).map_err(|e| store::TextError::new(format!("json decode: {e}"), dsl::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        let bytes = serde_json::to_vec(self).unwrap_or_default();
        let body: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for SemioPresentationSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = serde_json::to_vec(self).map_err(|e| store::PackError::Schema(e.to_string()))?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::ArtifactDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let _ = options;
        serde_json::from_slice(&inner).map_err(|e| store::PackError::Schema(e.to_string()))
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_pack_round_trips() {
        let snap = SemioPresentationSnapshot::default();
        let bytes = <SemioPresentationSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <SemioPresentationSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(snap, back);
    }

    #[test]
    fn dsl_text_round_trips() {
        let snap = SemioPresentationSnapshot::default();
        let text = <SemioPresentationSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        let back = <SemioPresentationSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(snap, back);
    }

    /// 🧪️ Non-empty structural round trip: masters/layouts/slides all populated, exercising every
    /// shape kind + the document-block reuse.
    #[test]
    fn json_pack_round_trips_populated_structure() {
        let snap = SemioPresentationSnapshot {
            schema: STDIO_SEMIOPRESENTATION_DOCUMENT_SCHEMA.into(),
            masters: vec![SlideMaster {
                id: "master1".into(),
                shapes: vec![SlideShape::Placeholder {
                    frame: SlideFrame { origin: SemioPoint2 { x: 0.0, y: 0.0 }, width: 100.0, height: 20.0 },
                    kind: PlaceholderKind::Title,
                }],
            }],
            layouts: vec![SlideLayout {
                id: "layout1".into(),
                master_id: "master1".into(),
                shapes: Vec::new(),
            }],
            slides: vec![Slide {
                id: "slide1".into(),
                layout_id: Some("layout1".into()),
                shapes: vec![
                    SlideShape::TextBox {
                        frame: SlideFrame { origin: SemioPoint2 { x: 1.0, y: 2.0 }, width: 50.0, height: 10.0 },
                        blocks: vec![DocBlock::paragraph("x")],
                    },
                    SlideShape::Picture {
                        frame: SlideFrame { origin: SemioPoint2 { x: 0.0, y: 0.0 }, width: 10.0, height: 10.0 },
                        image: SlidePictureImage { asset_id: "img1".into(), mime: "image/png".into(), bytes: vec![1, 2, 3] },
                    },
                    SlideShape::Table {
                        frame: SlideFrame { origin: SemioPoint2 { x: 0.0, y: 0.0 }, width: 30.0, height: 30.0 },
                        rows: vec![SlideTableRow { cells: vec![SlideTableCell { blocks: vec![DocBlock::paragraph("x")] }] }],
                    },
                ],
                notes: vec![DocBlock::paragraph("x")],
            }],
        };
        let bytes = <SemioPresentationSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <SemioPresentationSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(snap, back);
    }
}
//#endregion 🔖️Tests
