//! 💾️ Binary representation codec surface for `s.stdio.semio.document` (snapshot) — protocol include.
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::document::schema::snapshot::*;
use crate::standards::v1::subsets::base::schema::triples::{split_top_level, strip_brackets};
use crate::document::io::text::diff::{dec_image};
use crate::document::io::text::diff::{enc_image};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{dec_style};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{enc_style};
use crate::standards::v1::subsets::cad::io::text::snapshot::{dec_block};
use crate::standards::v1::subsets::cad::io::text::snapshot::{enc_block};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{dec_str};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{enc_str};
use framework_schema::ArtifactSchema;

/// 🧪️ Real LEB128-varint-length-prefixed binary primitives (`store::pack_rt::write_varint_u64` /
/// `store::ByteReader`, same helpers flow/model/brep's own upgraded `ArtifactPack`s use)
/// backing `encode_document_snapshot_binary`/`decode_document_snapshot_binary` below.
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
pub(crate) fn write_bool(out: &mut Vec<u8>, b: bool) {
    out.push(if b { 1 } else { 0 });
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bool(reader: &mut store::ByteReader<'_>) -> Result<bool, String> {
    Ok(reader.read_u8().map_err(|e| e.to_string())? != 0)
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
pub(crate) fn write_opt_f64(out: &mut Vec<u8>, v: Option<f64>) {
    match v {
        None => out.push(0),
        Some(f) => {
            out.push(1);
            out.extend_from_slice(&f.to_le_bytes());
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_opt_f64(reader: &mut store::ByteReader<'_>) -> Result<Option<f64>, String> {
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => Ok(None),
        1 => Ok(Some(reader.read_f64_le().map_err(|e| e.to_string())?)),
        other => Err(format!("opt f64: bad tag {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_run_style(out: &mut Vec<u8>, s: &RunStyle) {
    write_bool(out, s.bold);
    write_bool(out, s.italic);
    write_bool(out, s.underline);
    write_opt_f64(out, s.size);
    write_opt_str(out, &s.font);
    write_opt_str(out, &s.color);
    write_opt_str(out, &s.link);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_run_style(reader: &mut store::ByteReader<'_>) -> Result<RunStyle, String> {
    Ok(RunStyle { bold: read_bool(reader)?, italic: read_bool(reader)?, underline: read_bool(reader)?, size: read_opt_f64(reader)?, font: read_opt_str(reader)?, color: read_opt_str(reader)?, link: read_opt_str(reader)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_run(out: &mut Vec<u8>, r: &DocRun) {
    write_str_lp(out, &r.text);
    write_run_style(out, &r.style);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_run(reader: &mut store::ByteReader<'_>) -> Result<DocRun, String> {
    Ok(DocRun { text: read_str_lp(reader)?, style: read_run_style(reader)? })
}

/// 🌳️ Real per-variant tag byte (0=Paragraph 1=Heading 2=List 3=Table 4=Code 5=Quote 6=Image
/// 7=PageBreak) + fields, genuinely recursive for `List`/`Table`/`Quote`'s nested `Vec<DocBlock>`
/// (`protocol-prim-ref-recursion`/`protocol-array-of-records` — the Rust side stays fully
/// structured; only the `.protocol.semio` DESCRIPTION stops at an opaque tail, per the recipe).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_block(out: &mut Vec<u8>, b: &DocBlock) {
    match b {
        DocBlock::Paragraph { style_id, runs } => {
            out.push(0);
            write_opt_str(out, style_id);
            store::pack_rt::write_varint_u64(out, runs.len() as u64);
            for r in runs {
                write_run(out, r);
            }
        }
        DocBlock::Heading { level, style_id, runs } => {
            out.push(1);
            out.push(*level);
            write_opt_str(out, style_id);
            store::pack_rt::write_varint_u64(out, runs.len() as u64);
            for r in runs {
                write_run(out, r);
            }
        }
        DocBlock::List { ordered, items } => {
            out.push(2);
            write_bool(out, *ordered);
            store::pack_rt::write_varint_u64(out, items.len() as u64);
            for item in items {
                store::pack_rt::write_varint_u64(out, item.blocks.len() as u64);
                for b in &item.blocks {
                    write_block(out, b);
                }
            }
        }
        DocBlock::Table { rows } => {
            out.push(3);
            store::pack_rt::write_varint_u64(out, rows.len() as u64);
            for row in rows {
                store::pack_rt::write_varint_u64(out, row.cells.len() as u64);
                for cell in &row.cells {
                    store::pack_rt::write_varint_u64(out, cell.blocks.len() as u64);
                    for b in &cell.blocks {
                        write_block(out, b);
                    }
                }
            }
        }
        DocBlock::Code { language, text } => {
            out.push(4);
            write_opt_str(out, language);
            write_str_lp(out, text);
        }
        DocBlock::Quote { blocks } => {
            out.push(5);
            store::pack_rt::write_varint_u64(out, blocks.len() as u64);
            for b in blocks {
                write_block(out, b);
            }
        }
        DocBlock::Image { image_id, alt, width, height } => {
            out.push(6);
            write_str_lp(out, image_id);
            write_str_lp(out, alt);
            write_opt_f64(out, *width);
            write_opt_f64(out, *height);
        }
        DocBlock::PageBreak => out.push(7),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_block(reader: &mut store::ByteReader<'_>) -> Result<DocBlock, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => {
            let style_id = read_opt_str(reader)?;
            let n = reader.read_varint_u64().map_err(|e| e.to_string())?;
            let mut runs = Vec::with_capacity(n as usize);
            for _ in 0..n {
                runs.push(read_run(reader)?);
            }
            Ok(DocBlock::Paragraph { style_id, runs })
        }
        1 => {
            let level = reader.read_u8().map_err(|e| e.to_string())?;
            let style_id = read_opt_str(reader)?;
            let n = reader.read_varint_u64().map_err(|e| e.to_string())?;
            let mut runs = Vec::with_capacity(n as usize);
            for _ in 0..n {
                runs.push(read_run(reader)?);
            }
            Ok(DocBlock::Heading { level, style_id, runs })
        }
        2 => {
            let ordered = read_bool(reader)?;
            let n = reader.read_varint_u64().map_err(|e| e.to_string())?;
            let mut items = Vec::with_capacity(n as usize);
            for _ in 0..n {
                let bn = reader.read_varint_u64().map_err(|e| e.to_string())?;
                let mut blocks = Vec::with_capacity(bn as usize);
                for _ in 0..bn {
                    blocks.push(read_block(reader)?);
                }
                items.push(DocListItem { blocks });
            }
            Ok(DocBlock::List { ordered, items })
        }
        3 => {
            let n = reader.read_varint_u64().map_err(|e| e.to_string())?;
            let mut rows = Vec::with_capacity(n as usize);
            for _ in 0..n {
                let cn = reader.read_varint_u64().map_err(|e| e.to_string())?;
                let mut cells = Vec::with_capacity(cn as usize);
                for _ in 0..cn {
                    let bn = reader.read_varint_u64().map_err(|e| e.to_string())?;
                    let mut blocks = Vec::with_capacity(bn as usize);
                    for _ in 0..bn {
                        blocks.push(read_block(reader)?);
                    }
                    cells.push(DocTableCell { blocks });
                }
                rows.push(DocTableRow { cells });
            }
            Ok(DocBlock::Table { rows })
        }
        4 => {
            let language = read_opt_str(reader)?;
            let text = read_str_lp(reader)?;
            Ok(DocBlock::Code { language, text })
        }
        5 => {
            let n = reader.read_varint_u64().map_err(|e| e.to_string())?;
            let mut blocks = Vec::with_capacity(n as usize);
            for _ in 0..n {
                blocks.push(read_block(reader)?);
            }
            Ok(DocBlock::Quote { blocks })
        }
        6 => {
            let image_id = read_str_lp(reader)?;
            let alt = read_str_lp(reader)?;
            let width = read_opt_f64(reader)?;
            let height = read_opt_f64(reader)?;
            Ok(DocBlock::Image { image_id, alt, width, height })
        }
        7 => Ok(DocBlock::PageBreak),
        other => Err(format!("block: unknown tag {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_style(out: &mut Vec<u8>, s: &DocStyle) {
    write_str_lp(out, &s.id);
    write_str_lp(out, &s.name);
    write_opt_str(out, &s.based_on);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_style(reader: &mut store::ByteReader<'_>) -> Result<DocStyle, String> {
    Ok(DocStyle { id: read_str_lp(reader)?, name: read_str_lp(reader)?, based_on: read_opt_str(reader)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_image(out: &mut Vec<u8>, i: &DocImage) {
    write_str_lp(out, &i.id);
    write_str_lp(out, &i.mime);
    write_bytes_lp(out, &i.bytes);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_image(reader: &mut store::ByteReader<'_>) -> Result<DocImage, String> {
    Ok(DocImage { id: read_str_lp(reader)?, mime: read_str_lp(reader)?, bytes: read_bytes_lp(reader)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn encode_document_snapshot_binary(s: &SemioDocumentSnapshot) -> Vec<u8> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut out = Vec::new();
    out.push(PACK_BINARY_FORMAT);
    write_str_lp(&mut out, &s.schema);
    store::pack_rt::write_varint_u64(&mut out, s.styles.len() as u64);
    for st in &s.styles {
        write_style(&mut out, st);
    }
    store::pack_rt::write_varint_u64(&mut out, s.images.len() as u64);
    for im in &s.images {
        write_image(&mut out, im);
    }
    store::pack_rt::write_varint_u64(&mut out, s.blocks.len() as u64);
    for b in &s.blocks {
        write_block(&mut out, b);
    }
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn decode_document_snapshot_binary(bytes: &[u8]) -> Result<SemioDocumentSnapshot, String> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut reader = store::ByteReader::new(bytes);
    let format = reader.read_u8().map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT {
        return Err(format!("unsupported pack format {format}"));
    }
    let schema = read_str_lp(&mut reader)?;
    let style_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut styles = Vec::with_capacity(style_count as usize);
    for _ in 0..style_count {
        styles.push(read_style(&mut reader)?);
    }
    let image_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut images = Vec::with_capacity(image_count as usize);
    for _ in 0..image_count {
        images.push(read_image(&mut reader)?);
    }
    let block_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut blocks = Vec::with_capacity(block_count as usize);
    for _ in 0..block_count {
        blocks.push(read_block(&mut reader)?);
    }
    Ok(SemioDocumentSnapshot { schema, styles, images, blocks })
}

impl store::ArtifactPack for SemioDocumentSnapshot {
    /// 🪶️ Publishes the owned typed relational snapshot capability.
    fn sqlite_snapshot_codec() -> Option<store::ArtifactSqliteSnapshotCodec> { Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec()) }

    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_document_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let _ = options;
        decode_document_snapshot_binary(&inner).map_err(|detail| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, detail)))
    }
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod native_snapshot_codec {
use super::*;
use super::*;
use crate::standards::v1::subsets::document::schema::snapshot::*;
use crate::audio::io::text::diff::{strip_brackets};
use crate::audio::io::text::diff::{split_top_level};
use crate::document::io::text::diff::{dec_image};
use crate::document::io::text::diff::{enc_image};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{dec_style};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{enc_style};
use crate::standards::v1::subsets::cad::io::text::snapshot::{dec_block};
use crate::standards::v1::subsets::cad::io::text::snapshot::{enc_block};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{dec_str};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{enc_str};
use framework_schema::ArtifactSchema;
use crate::standards::v1::subsets::document::io::text::snapshot::*;
/// 📥️ Decodes this subset's own committed `.pack.semio` bytes into a real [`SemioDocumentSnapshot`] — the
/// binary half of the same bridge, so a caller outside this crate can check the two codecs against
/// each other on the two real committed artifacts instead of against itself.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_document_pack(bytes: &[u8]) -> Result<SemioDocumentSnapshot, String> {
    <SemioDocumentSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| error.to_string())
}
/// 📤️ The `store::ArtifactPack::encode_pack` inverse of [`decode_semio_document_pack`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_semio_document_pack(snapshot: &SemioDocumentSnapshot) -> Vec<u8> {
    <SemioDocumentSnapshot as store::ArtifactPack>::encode_pack(snapshot)
}
}
pub use native_snapshot_codec::*;
