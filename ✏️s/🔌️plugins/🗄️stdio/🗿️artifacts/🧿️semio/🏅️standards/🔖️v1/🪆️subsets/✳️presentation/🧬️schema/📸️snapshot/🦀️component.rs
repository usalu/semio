//! 🧬️ SemioPresentationSnapshot — masters/layouts/slides -> shapes (TextBox/Picture/Table/
//! Placeholder) + per-slide notes — from pptx. `SlideShape::TextBox`/`Table` cell content
//! deliberately REUSE `document`'s `DocBlock` per the master plan's spec-mandated cross-reuse note
//! ("presentation mirrors document's block shape with own types" — the shape types themselves
//! (`SlideMaster`/`SlideLayout`/`Slide`/`SlideShape`) are owned here; only the block-tree LEAF is
//! shared, per `w1b-type-ownership.md`).

use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint2;
use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::{split_top_level, strip_brackets};
use crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::DocBlock;
/// 🧱️ REUSE, don't reinvent — the sibling `🔺️diff` facet re-exports document's own real, already-
/// tested `DocBlock` codec (`enc_block`/`dec_block`) plus the entity value-codecs it owns
/// (`enc_master`/`enc_layout`/`enc_slide`, `enc_str`, `enc_list`) — this facet imports them rather
/// than duplicating a third independent copy (ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-
/// EVOLUTION presentation wave, following `document`'s own snapshot-imports-from-diff convention).
use crate::artifacts::semio::standards::v1::subsets::presentation::schema::diff::{dec_block, dec_layout, dec_master, dec_slide, dec_str, enc_block, enc_layout, enc_master, enc_slide, enc_str};
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
    TextBox {
        frame: SlideFrame,
        #[serde(default)]
        blocks: Vec<DocBlock>,
    },
    /// 🖼️ `p:pic`.
    Picture { frame: SlideFrame, image: SlidePictureImage },
    /// 🏛️ `p:graphicFrame` holding `a:tbl`.
    Table {
        frame: SlideFrame,
        #[serde(default)]
        rows: Vec<SlideTableRow>,
    },
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
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[serde(default)]
    pub masters: Vec<SlideMaster>,
    #[state(artifact)]
    #[serde(default)]
    pub layouts: Vec<SlideLayout>,
    #[state(artifact)]
    #[serde(default)]
    pub slides: Vec<Slide>,
}

impl Default for SemioPresentationSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_SEMIOPRESENTATION_DOCUMENT_SCHEMA.into(), masters: Vec::new(), layouts: Vec::new(), slides: Vec::new() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️TextCodec
/// 🧪️ ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION presentation wave: real
/// structured text body — `schema=<hex>` / `masters=[<master>,...]` / `layouts=[<layout>,...]` /
/// `slides=[<slide>,...]`, one line per top-level field, matching the flow/model/brep/document
/// pilots' own `print_*_snapshot_body` shape. Reuses `🔺️diff`'s ALREADY-real, already-tested
/// `enc_str`/`enc_master`/`enc_layout`/`enc_slide` value codecs (which themselves reuse document's
/// real `enc_block` for every `blocks`/`notes` leaf) rather than duplicating a third independent
/// copy. Replaces the old hex-of-`serde_json` passthrough.
fn print_presentation_snapshot_body(s: &SemioPresentationSnapshot) -> String {
    format!(
        "schema={}\nmasters=[{}]\nlayouts=[{}]\nslides=[{}]",
        enc_str(&s.schema),
        s.masters.iter().map(enc_master).collect::<Vec<_>>().join(","),
        s.layouts.iter().map(enc_layout).collect::<Vec<_>>().join(","),
        s.slides.iter().map(enc_slide).collect::<Vec<_>>().join(",")
    )
}
fn parse_presentation_snapshot_body(body: &str) -> Result<SemioPresentationSnapshot, String> {
    let mut schema = None;
    let mut masters = Vec::new();
    let mut layouts = Vec::new();
    let mut slides = Vec::new();
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("schema=") {
            schema = Some(dec_str(rest)?);
        } else if let Some(rest) = line.strip_prefix("masters=") {
            let inner = strip_brackets(rest)?;
            masters = split_top_level(inner, ',').into_iter().filter(|s| !s.is_empty()).map(dec_master).collect::<Result<Vec<_>, String>>()?;
        } else if let Some(rest) = line.strip_prefix("layouts=") {
            let inner = strip_brackets(rest)?;
            layouts = split_top_level(inner, ',').into_iter().filter(|s| !s.is_empty()).map(dec_layout).collect::<Result<Vec<_>, String>>()?;
        } else if let Some(rest) = line.strip_prefix("slides=") {
            let inner = strip_brackets(rest)?;
            slides = split_top_level(inner, ',').into_iter().filter(|s| !s.is_empty()).map(dec_slide).collect::<Result<Vec<_>, String>>()?;
        } else {
            return Err(format!("presentation snapshot: unknown line {line:?}"));
        }
    }
    let schema = schema.ok_or_else(|| "presentation snapshot: missing schema line".to_string())?;
    Ok(SemioPresentationSnapshot { schema, masters, layouts, slides })
}
//#endregion 🔖️TextCodec

//#region 🔖️BinaryCodec
/// 🧪️ Real LEB128-varint-length-prefixed binary primitives (`store::pack_rt::write_varint_u64` /
/// `store::ByteReader`, same helpers every other semio wave's `ArtifactPack` reuses) backing
/// `encode_presentation_snapshot_binary`/`decode_presentation_snapshot_binary` below.
fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}
fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}
fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}
fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}
fn write_f64(out: &mut Vec<u8>, v: f64) {
    out.extend_from_slice(&v.to_le_bytes());
}
fn read_f64(reader: &mut store::ByteReader<'_>) -> Result<f64, String> {
    reader.read_f64_le().map_err(|e| e.to_string())
}
fn write_opt_str(out: &mut Vec<u8>, v: &Option<String>) {
    match v {
        None => out.push(0),
        Some(s) => {
            out.push(1);
            write_str_lp(out, s);
        }
    }
}
fn read_opt_str(reader: &mut store::ByteReader<'_>) -> Result<Option<String>, String> {
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => Ok(None),
        1 => Ok(Some(read_str_lp(reader)?)),
        other => Err(format!("opt str: bad tag {other}")),
    }
}

fn write_point2(out: &mut Vec<u8>, p: &SemioPoint2) {
    write_f64(out, p.x);
    write_f64(out, p.y);
}
fn read_point2(reader: &mut store::ByteReader<'_>) -> Result<SemioPoint2, String> {
    Ok(SemioPoint2 { x: read_f64(reader)?, y: read_f64(reader)? })
}
fn write_frame(out: &mut Vec<u8>, f: &SlideFrame) {
    write_point2(out, &f.origin);
    write_f64(out, f.width);
    write_f64(out, f.height);
}
fn read_frame(reader: &mut store::ByteReader<'_>) -> Result<SlideFrame, String> {
    Ok(SlideFrame { origin: read_point2(reader)?, width: read_f64(reader)?, height: read_f64(reader)? })
}
fn write_image(out: &mut Vec<u8>, img: &SlidePictureImage) {
    write_str_lp(out, &img.asset_id);
    write_str_lp(out, &img.mime);
    write_bytes_lp(out, &img.bytes);
}
fn read_image(reader: &mut store::ByteReader<'_>) -> Result<SlidePictureImage, String> {
    Ok(SlidePictureImage { asset_id: read_str_lp(reader)?, mime: read_str_lp(reader)?, bytes: read_bytes_lp(reader)? })
}
/// 🌳️ Real per-variant tag byte (0=Title 1=Subtitle 2=Body 3=Footer 4=SlideNumber 5=DateTime
/// 6=Other) + fields.
fn write_placeholder_kind(out: &mut Vec<u8>, k: &PlaceholderKind) {
    match k {
        PlaceholderKind::Title => out.push(0),
        PlaceholderKind::Subtitle => out.push(1),
        PlaceholderKind::Body => out.push(2),
        PlaceholderKind::Footer => out.push(3),
        PlaceholderKind::SlideNumber => out.push(4),
        PlaceholderKind::DateTime => out.push(5),
        PlaceholderKind::Other { value } => {
            out.push(6);
            write_str_lp(out, value);
        }
    }
}
fn read_placeholder_kind(reader: &mut store::ByteReader<'_>) -> Result<PlaceholderKind, String> {
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => Ok(PlaceholderKind::Title),
        1 => Ok(PlaceholderKind::Subtitle),
        2 => Ok(PlaceholderKind::Body),
        3 => Ok(PlaceholderKind::Footer),
        4 => Ok(PlaceholderKind::SlideNumber),
        5 => Ok(PlaceholderKind::DateTime),
        6 => Ok(PlaceholderKind::Other { value: read_str_lp(reader)? }),
        other => Err(format!("placeholder kind: bad tag {other}")),
    }
}
/// 🧱️ REUSE, don't reinvent — each `DocBlock` element is encoded via document's real, already-
/// tested `enc_block`/`dec_block` TEXT codec (imported above), embedded as one length-prefixed
/// UTF-8 blob per element. Never re-derives `DocBlock`'s own binary shape (which is private to
/// `document`'s snapshot facet) — this is the honest boundary the grammar recipe's own
/// `protocol-array-of-records`/`protocol-prim-ref-recursion` gaps describe, applied at the Rust
/// level too: reuse the real encoder, don't duplicate it.
fn write_blocks(out: &mut Vec<u8>, blocks: &[DocBlock]) {
    store::pack_rt::write_varint_u64(out, blocks.len() as u64);
    for b in blocks {
        write_str_lp(out, &enc_block(b));
    }
}
fn read_blocks(reader: &mut store::ByteReader<'_>) -> Result<Vec<DocBlock>, String> {
    let n = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut out = Vec::with_capacity(n as usize);
    for _ in 0..n {
        out.push(dec_block(&read_str_lp(reader)?)?);
    }
    Ok(out)
}
fn write_table_cell(out: &mut Vec<u8>, c: &SlideTableCell) {
    write_blocks(out, &c.blocks);
}
fn read_table_cell(reader: &mut store::ByteReader<'_>) -> Result<SlideTableCell, String> {
    Ok(SlideTableCell { blocks: read_blocks(reader)? })
}
fn write_table_row(out: &mut Vec<u8>, r: &SlideTableRow) {
    store::pack_rt::write_varint_u64(out, r.cells.len() as u64);
    for c in &r.cells {
        write_table_cell(out, c);
    }
}
fn read_table_row(reader: &mut store::ByteReader<'_>) -> Result<SlideTableRow, String> {
    let n = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut cells = Vec::with_capacity(n as usize);
    for _ in 0..n {
        cells.push(read_table_cell(reader)?);
    }
    Ok(SlideTableRow { cells })
}
/// 🌳️ Real per-variant tag byte (0=TextBox 1=Picture 2=Table 3=Placeholder) + `frame` + fields.
fn write_shape(out: &mut Vec<u8>, shape: &SlideShape) {
    match shape {
        SlideShape::TextBox { frame, blocks } => {
            out.push(0);
            write_frame(out, frame);
            write_blocks(out, blocks);
        }
        SlideShape::Picture { frame, image } => {
            out.push(1);
            write_frame(out, frame);
            write_image(out, image);
        }
        SlideShape::Table { frame, rows } => {
            out.push(2);
            write_frame(out, frame);
            store::pack_rt::write_varint_u64(out, rows.len() as u64);
            for r in rows {
                write_table_row(out, r);
            }
        }
        SlideShape::Placeholder { frame, kind } => {
            out.push(3);
            write_frame(out, frame);
            write_placeholder_kind(out, kind);
        }
    }
}
fn read_shape(reader: &mut store::ByteReader<'_>) -> Result<SlideShape, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => Ok(SlideShape::TextBox { frame: read_frame(reader)?, blocks: read_blocks(reader)? }),
        1 => Ok(SlideShape::Picture { frame: read_frame(reader)?, image: read_image(reader)? }),
        2 => {
            let frame = read_frame(reader)?;
            let n = reader.read_varint_u64().map_err(|e| e.to_string())?;
            let mut rows = Vec::with_capacity(n as usize);
            for _ in 0..n {
                rows.push(read_table_row(reader)?);
            }
            Ok(SlideShape::Table { frame, rows })
        }
        3 => Ok(SlideShape::Placeholder { frame: read_frame(reader)?, kind: read_placeholder_kind(reader)? }),
        other => Err(format!("shape: bad tag {other}")),
    }
}
fn write_shapes(out: &mut Vec<u8>, shapes: &[SlideShape]) {
    store::pack_rt::write_varint_u64(out, shapes.len() as u64);
    for s in shapes {
        write_shape(out, s);
    }
}
fn read_shapes(reader: &mut store::ByteReader<'_>) -> Result<Vec<SlideShape>, String> {
    let n = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut out = Vec::with_capacity(n as usize);
    for _ in 0..n {
        out.push(read_shape(reader)?);
    }
    Ok(out)
}
fn write_master(out: &mut Vec<u8>, m: &SlideMaster) {
    write_str_lp(out, &m.id);
    write_shapes(out, &m.shapes);
}
fn read_master(reader: &mut store::ByteReader<'_>) -> Result<SlideMaster, String> {
    Ok(SlideMaster { id: read_str_lp(reader)?, shapes: read_shapes(reader)? })
}
fn write_layout(out: &mut Vec<u8>, l: &SlideLayout) {
    write_str_lp(out, &l.id);
    write_str_lp(out, &l.master_id);
    write_shapes(out, &l.shapes);
}
fn read_layout(reader: &mut store::ByteReader<'_>) -> Result<SlideLayout, String> {
    Ok(SlideLayout { id: read_str_lp(reader)?, master_id: read_str_lp(reader)?, shapes: read_shapes(reader)? })
}
fn write_slide(out: &mut Vec<u8>, s: &Slide) {
    write_str_lp(out, &s.id);
    write_opt_str(out, &s.layout_id);
    write_shapes(out, &s.shapes);
    write_blocks(out, &s.notes);
}
fn read_slide(reader: &mut store::ByteReader<'_>) -> Result<Slide, String> {
    Ok(Slide { id: read_str_lp(reader)?, layout_id: read_opt_str(reader)?, shapes: read_shapes(reader)?, notes: read_blocks(reader)? })
}

fn encode_presentation_snapshot_binary(s: &SemioPresentationSnapshot) -> Vec<u8> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut out = Vec::new();
    out.push(PACK_BINARY_FORMAT);
    write_str_lp(&mut out, &s.schema);
    store::pack_rt::write_varint_u64(&mut out, s.masters.len() as u64);
    for m in &s.masters {
        write_master(&mut out, m);
    }
    store::pack_rt::write_varint_u64(&mut out, s.layouts.len() as u64);
    for l in &s.layouts {
        write_layout(&mut out, l);
    }
    store::pack_rt::write_varint_u64(&mut out, s.slides.len() as u64);
    for sl in &s.slides {
        write_slide(&mut out, sl);
    }
    out
}
fn decode_presentation_snapshot_binary(bytes: &[u8]) -> Result<SemioPresentationSnapshot, String> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut reader = store::ByteReader::new(bytes);
    let format = reader.read_u8().map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT {
        return Err(format!("unsupported pack format {format}"));
    }
    let schema = read_str_lp(&mut reader)?;
    let master_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut masters = Vec::with_capacity(master_count as usize);
    for _ in 0..master_count {
        masters.push(read_master(&mut reader)?);
    }
    let layout_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut layouts = Vec::with_capacity(layout_count as usize);
    for _ in 0..layout_count {
        layouts.push(read_layout(&mut reader)?);
    }
    let slide_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut slides = Vec::with_capacity(slide_count as usize);
    for _ in 0..slide_count {
        slides.push(read_slide(&mut reader)?);
    }
    Ok(SemioPresentationSnapshot { schema, masters, layouts, slides })
}
//#endregion 🔖️BinaryCodec

//#region 🔖️HandcraftedArtifactCodecs
/// 🧩️ ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION presentation wave: real
/// structured text/binary codecs, replacing the old hex-of-`serde_json` passthrough (both
/// `parse_dsl`'s and `encode_pack_with`'s old bodies called straight into
/// `serde_json::{to_vec,from_slice}`). The derive path (`#[derive(dsl::DslArtifact)]`) hits the
/// same wall every hand-rolled-tagged-enum semio subset (model/brep/drawing/document) already hit:
/// `SlideShape`/`PlaceholderKind` are `#[serde(tag = ...)]` data-carrying enums with heterogeneous
/// per-variant field sets (and `SlideShape::TextBox`/`Table` transitively embed `DocBlock`, itself
/// the same shape) — no `dsl::DslField`/`DslEnum` impl exists for either. Hand-rolled instead,
/// matching `🔺️diff`'s already-hand-rolled convention.
impl store::ArtifactDsl for SemioPresentationSnapshot {
    const EXTENSION: &'static str = "semio";
    fn envelope_id() -> &'static str {
        STDIO_SEMIOPRESENTATION_DOCUMENT_SCHEMA
    }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_presentation_snapshot_body(body).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        let body = print_presentation_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for SemioPresentationSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_presentation_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let _ = options;
        decode_presentation_snapshot_binary(&inner).map_err(store::PackError::Schema)
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️Demo
/// 🌱 The demo `s.stdio.semio.presentation` snapshot — masters/layouts/slides all populated,
/// exercising every `SlideShape` variant (incl. `Table`) and every `PlaceholderKind` variant (incl.
/// `Other`), plus the `document::DocBlock` reuse in `TextBox.blocks`/table cell `blocks`/
/// `Slide.notes`. Single source of truth for `📚️examples/…/🖼️assets/🗣️example.dsl.semio`/
/// `🎒️example.pack.semio` and for the conformance-law tests in `🎹️composer/🦀️component.rs`.
#[cfg(test)]
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
mod tests {
    use super::*;

    #[test]
    fn pack_round_trips() {
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

    #[test]
    fn demo_snapshot_pack_and_dsl_round_trip() {
        let snap = demo_semio_presentation_snapshot();
        let bytes = <SemioPresentationSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <SemioPresentationSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(snap, back);

        let text = <SemioPresentationSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        let back_text = <SemioPresentationSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(snap, back_text);
    }

    /// 🧪️ Non-empty structural round trip: masters/layouts/slides all populated, exercising every
    /// shape kind + the document-block reuse.
    #[test]
    fn pack_round_trips_populated_structure() {
        let snap = SemioPresentationSnapshot {
            schema: STDIO_SEMIOPRESENTATION_DOCUMENT_SCHEMA.into(),
            masters: vec![SlideMaster { id: "master1".into(), shapes: vec![SlideShape::Placeholder { frame: SlideFrame { origin: SemioPoint2 { x: 0.0, y: 0.0 }, width: 100.0, height: 20.0 }, kind: PlaceholderKind::Title }] }],
            layouts: vec![SlideLayout { id: "layout1".into(), master_id: "master1".into(), shapes: Vec::new() }],
            slides: vec![Slide {
                id: "slide1".into(),
                layout_id: Some("layout1".into()),
                shapes: vec![
                    SlideShape::TextBox { frame: SlideFrame { origin: SemioPoint2 { x: 1.0, y: 2.0 }, width: 50.0, height: 10.0 }, blocks: vec![DocBlock::paragraph("x")] },
                    SlideShape::Picture { frame: SlideFrame { origin: SemioPoint2 { x: 0.0, y: 0.0 }, width: 10.0, height: 10.0 }, image: SlidePictureImage { asset_id: "img1".into(), mime: "image/png".into(), bytes: vec![1, 2, 3] } },
                    SlideShape::Table { frame: SlideFrame { origin: SemioPoint2 { x: 0.0, y: 0.0 }, width: 30.0, height: 30.0 }, rows: vec![SlideTableRow { cells: vec![SlideTableCell { blocks: vec![DocBlock::paragraph("x")] }] }] },
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
