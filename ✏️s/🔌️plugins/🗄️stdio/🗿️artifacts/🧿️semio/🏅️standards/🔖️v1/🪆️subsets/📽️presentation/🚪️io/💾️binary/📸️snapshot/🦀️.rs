//! 💾️ Binary representation codec surface for `stdio.semio.presentation` (snapshot).

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::presentation::schema::snapshot::*;
use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use crate::standards::v1::subsets::base::schema::triples::{split_top_level, strip_brackets};
use crate::standards::v1::subsets::document::schema::snapshot::DocBlock;
/// 🧱️ REUSE, don't reinvent — the sibling `🔺️diff` facet re-exports document's own real, already-
/// tested `DocBlock` codec (`enc_block`/`dec_block`) plus the entity value-codecs it owns
/// (`enc_master`/`enc_layout`/`enc_slide`, `enc_str`, `enc_list`) — this facet imports them rather
/// than duplicating a third independent copy (ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-
/// EVOLUTION presentation wave, following `document`'s own snapshot-imports-from-diff convention).
use crate::presentation::io::text::diff::{dec_slide};
use crate::presentation::io::text::diff::{enc_slide};
use crate::presentation::io::text::diff::{dec_layout};
use crate::presentation::io::text::diff::{enc_layout};
use crate::presentation::io::text::diff::{dec_master};
use crate::presentation::io::text::diff::{enc_master};
use crate::standards::v1::subsets::cad::io::text::snapshot::{dec_block};
use crate::standards::v1::subsets::cad::io::text::snapshot::{enc_block};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{dec_str};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{enc_str};
use framework_schema::ArtifactSchema;

/// 🧪️ Real LEB128-varint-length-prefixed binary primitives (`store::pack_rt::write_varint_u64` /
/// `store::ByteReader`, same helpers every other semio wave's `ArtifactPack` reuses) backing
/// `encode_presentation_snapshot_binary`/`decode_presentation_snapshot_binary` below.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_f64(out: &mut Vec<u8>, v: f64) {
    out.extend_from_slice(&v.to_le_bytes());
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_f64(reader: &mut store::ByteReader<'_>) -> Result<f64, String> {
    reader.read_f64_le().map_err(|e| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_opt_str(out: &mut Vec<u8>, v: &Option<String>) {
    match v {
        None => out.push(0),
        Some(s) => {
            out.push(1);
            write_str_lp(out, s);
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_opt_str(reader: &mut store::ByteReader<'_>) -> Result<Option<String>, String> {
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => Ok(None),
        1 => Ok(Some(read_str_lp(reader)?)),
        other => Err(format!("opt str: bad tag {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_point2(out: &mut Vec<u8>, p: &SemioPoint2) {
    write_f64(out, p.x);
    write_f64(out, p.y);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_point2(reader: &mut store::ByteReader<'_>) -> Result<SemioPoint2, String> {
    Ok(SemioPoint2 { x: read_f64(reader)?, y: read_f64(reader)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_frame(out: &mut Vec<u8>, f: &SlideFrame) {
    write_point2(out, &f.origin);
    write_f64(out, f.width);
    write_f64(out, f.height);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_frame(reader: &mut store::ByteReader<'_>) -> Result<SlideFrame, String> {
    Ok(SlideFrame { origin: read_point2(reader)?, width: read_f64(reader)?, height: read_f64(reader)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_image(out: &mut Vec<u8>, img: &SlidePictureImage) {
    write_str_lp(out, &img.asset_id);
    write_str_lp(out, &img.mime);
    write_bytes_lp(out, &img.bytes);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_image(reader: &mut store::ByteReader<'_>) -> Result<SlidePictureImage, String> {
    Ok(SlidePictureImage { asset_id: read_str_lp(reader)?, mime: read_str_lp(reader)?, bytes: read_bytes_lp(reader)? })
}

/// 🌳️ Real per-variant tag byte (0=Title 1=Subtitle 2=Body 3=Footer 4=SlideNumber 5=DateTime
/// 6=Other) + fields.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_placeholder_kind(out: &mut Vec<u8>, k: &PlaceholderKind) {
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_placeholder_kind(reader: &mut store::ByteReader<'_>) -> Result<PlaceholderKind, String> {
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_blocks(out: &mut Vec<u8>, blocks: &[DocBlock]) {
    store::pack_rt::write_varint_u64(out, blocks.len() as u64);
    for b in blocks {
        write_str_lp(out, &enc_block(b));
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_blocks(reader: &mut store::ByteReader<'_>) -> Result<Vec<DocBlock>, String> {
    let n = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut out = Vec::with_capacity(n as usize);
    for _ in 0..n {
        out.push(dec_block(&read_str_lp(reader)?)?);
    }
    Ok(out)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_table_cell(out: &mut Vec<u8>, c: &SlideTableCell) {
    write_blocks(out, &c.blocks);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_table_cell(reader: &mut store::ByteReader<'_>) -> Result<SlideTableCell, String> {
    Ok(SlideTableCell { blocks: read_blocks(reader)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_table_row(out: &mut Vec<u8>, r: &SlideTableRow) {
    store::pack_rt::write_varint_u64(out, r.cells.len() as u64);
    for c in &r.cells {
        write_table_cell(out, c);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_table_row(reader: &mut store::ByteReader<'_>) -> Result<SlideTableRow, String> {
    let n = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut cells = Vec::with_capacity(n as usize);
    for _ in 0..n {
        cells.push(read_table_cell(reader)?);
    }
    Ok(SlideTableRow { cells })
}

/// 🌳️ Real per-variant tag byte (0=TextBox 1=Picture 2=Table 3=Placeholder) + `frame` + fields.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_shape(out: &mut Vec<u8>, shape: &SlideShape) {
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_shape(reader: &mut store::ByteReader<'_>) -> Result<SlideShape, String> {
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_shapes(out: &mut Vec<u8>, shapes: &[SlideShape]) {
    store::pack_rt::write_varint_u64(out, shapes.len() as u64);
    for s in shapes {
        write_shape(out, s);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_shapes(reader: &mut store::ByteReader<'_>) -> Result<Vec<SlideShape>, String> {
    let n = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut out = Vec::with_capacity(n as usize);
    for _ in 0..n {
        out.push(read_shape(reader)?);
    }
    Ok(out)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_master(out: &mut Vec<u8>, m: &SlideMaster) {
    write_str_lp(out, &m.id);
    write_shapes(out, &m.shapes);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_master(reader: &mut store::ByteReader<'_>) -> Result<SlideMaster, String> {
    Ok(SlideMaster { id: read_str_lp(reader)?, shapes: read_shapes(reader)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_layout(out: &mut Vec<u8>, l: &SlideLayout) {
    write_str_lp(out, &l.id);
    write_str_lp(out, &l.master_id);
    write_shapes(out, &l.shapes);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_layout(reader: &mut store::ByteReader<'_>) -> Result<SlideLayout, String> {
    Ok(SlideLayout { id: read_str_lp(reader)?, master_id: read_str_lp(reader)?, shapes: read_shapes(reader)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_slide(out: &mut Vec<u8>, s: &Slide) {
    write_str_lp(out, &s.id);
    write_opt_str(out, &s.layout_id);
    write_shapes(out, &s.shapes);
    write_blocks(out, &s.notes);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_slide(reader: &mut store::ByteReader<'_>) -> Result<Slide, String> {
    Ok(Slide { id: read_str_lp(reader)?, layout_id: read_opt_str(reader)?, shapes: read_shapes(reader)?, notes: read_blocks(reader)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn encode_presentation_snapshot_binary(s: &SemioPresentationSnapshot) -> Vec<u8> {
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn decode_presentation_snapshot_binary(bytes: &[u8]) -> Result<SemioPresentationSnapshot, String> {
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

impl store::ArtifactPack for SemioPresentationSnapshot {
    /// 🪶️ Publishes the owned typed relational snapshot capability.
    fn sqlite_snapshot_codec() -> Option<store::ArtifactSqliteSnapshotCodec> { Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec()) }

    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_presentation_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let _ = options;
        decode_presentation_snapshot_binary(&inner).map_err(|detail| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, detail)))
    }
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod native_snapshot_codec {
use super::*;
use super::*;
use crate::standards::v1::subsets::presentation::schema::snapshot::*;
use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use crate::audio::io::text::diff::{strip_brackets};
use crate::audio::io::text::diff::{split_top_level};
use crate::standards::v1::subsets::document::schema::snapshot::DocBlock;
/// 🧱️ REUSE, don't reinvent — the sibling `🔺️diff` facet re-exports document's own real, already-
/// tested `DocBlock` codec (`enc_block`/`dec_block`) plus the entity value-codecs it owns
/// (`enc_master`/`enc_layout`/`enc_slide`, `enc_str`, `enc_list`) — this facet imports them rather
/// than duplicating a third independent copy (ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-
/// EVOLUTION presentation wave, following `document`'s own snapshot-imports-from-diff convention).
use crate::presentation::io::text::diff::{dec_slide};
use crate::presentation::io::text::diff::{enc_slide};
use crate::presentation::io::text::diff::{dec_layout};
use crate::presentation::io::text::diff::{enc_layout};
use crate::presentation::io::text::diff::{dec_master};
use crate::presentation::io::text::diff::{enc_master};
use crate::standards::v1::subsets::cad::io::text::snapshot::{dec_block};
use crate::standards::v1::subsets::cad::io::text::snapshot::{enc_block};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{dec_str};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{enc_str};
use framework_schema::ArtifactSchema;
use crate::standards::v1::subsets::presentation::io::text::snapshot::*;
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_semio_presentation_pack(snapshot: &SemioPresentationSnapshot) -> Vec<u8> {
    <SemioPresentationSnapshot as store::ArtifactPack>::encode_pack(snapshot)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_presentation_pack(bytes: &[u8]) -> Result<SemioPresentationSnapshot, String> {
    <SemioPresentationSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|e| e.to_string())
}
}
pub use native_snapshot_codec::*;
