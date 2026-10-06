//! 💾️ Binary representation codec surface for `stdio.semio.presentation` (diff).

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::presentation::schema::diff::*;
use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use crate::standards::v1::subsets::base::schema::triples::{IndexAdded, IndexModified, IndexedTripleDiff, NamedModified, NamedTripleDiff};
use crate::audio::io::text::diff::{strip_brackets};
use crate::audio::io::text::diff::{split_top_level};
/// 🧱️ REUSE, don't reinvent — `document::DocBlock`'s own real, already-tested text codec
/// (`ws-codec-document-report.md`), re-exported here so both this file's own leaf encoders AND
/// the sibling `🧬️mutations`/`📸️snapshot` facets can import `{enc_block, dec_block}` from THIS
/// module (matching the pre-existing convention where this file is the one place that owns every
/// value codec presentation's other facets import from).
use crate::document::io::text::diff::{dec_block};
use crate::document::io::text::diff::{enc_block};
use crate::standards::v1::subsets::document::schema::snapshot::DocBlock;
use crate::standards::v1::subsets::presentation::schema::snapshot::SemioPresentationSnapshot;
use crate::standards::v1::subsets::presentation::schema::snapshot::{PlaceholderKind, Slide, SlideFrame, SlideLayout, SlideMaster, SlidePictureImage, SlideShape, SlideTableCell, SlideTableRow};
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;















// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn encode_option<T>(opt: &Option<T>, enc: impl Fn(&T) -> String) -> String {
    match opt {
        None => "[0]".to_string(),
        Some(v) => format!("[1,{}]", enc(v)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn decode_option<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Option<T>, String> {
    let inner = strip_brackets(s)?;
    match split_top_level(inner, ',').as_slice() {
        ["0"] => Ok(None),
        [tag, value] if *tag == "1" => Ok(Some(dec(value)?)),
        other => Err(format!("option decode: bad shape {other:?}")),
    }
}

















































































































/// 🧪️ Real LEB128-varint-length-prefixed binary primitives (`store::pack_rt::write_varint_u64` /
/// `store::ByteReader`, same helpers every other semio wave's `DiffCodec` upgrade reuses) backing
/// the real `DiffBinary::encode_diff`/`decode_diff` below.
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

impl protocol::DiffBinary for SemioPresentationDiff {
/// ⚡️ ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION presentation wave: real
/// binary diff frame, replacing the old `print_diff().into_bytes()` text-as-binary shortcut.
/// `format u8` + `presence u8` (bit0=`masters`, bit1=`layouts`, bit2=`slides`) are two REAL
/// fixed fields; each present collection then follows as its own varint-length-prefixed opaque
/// blob (the same `enc_masters_diff`/`enc_layouts_diff`/`enc_slides_diff` bracket/hex text
/// `print_diff` already produces) — one opaque blob per present collection rather than a
/// per-segment `Cond` because a SECOND `if`-guard on a field that's itself only conditionally
/// decoded hard-errors `eval_cond` (`protocol-cond-cannot-chain`, per the grammar recipe's own
/// gap table; every prior semio wave's own diff binary upgrade hit the identical shape).
fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
    const DIFF_BINARY_FORMAT: u8 = 1;
    let mut presence = 0u8;
    if self.masters.is_some() {
        presence |= 0b001;
    }
    if self.layouts.is_some() {
        presence |= 0b010;
    }
    if self.slides.is_some() {
        presence |= 0b100;
    }
    let mut out = vec![DIFF_BINARY_FORMAT, presence];
    if let Some(v) = &self.masters {
        write_str_lp(&mut out, &enc_masters_diff(v));
    }
    if let Some(v) = &self.layouts {
        write_str_lp(&mut out, &enc_layouts_diff(v));
    }
    if let Some(v) = &self.slides {
        write_str_lp(&mut out, &enc_slides_diff(v));
    }
    Ok(out)
}
fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
    const DIFF_BINARY_FORMAT: u8 = 1;
    if bytes.len() < 2 {
        return Err(protocol::ProtocolError::Malformed { what: "diff header", offset: 0, detail: "truncated (need format+presence)".to_string() });
    }
    if bytes[0] != DIFF_BINARY_FORMAT {
        return Err(protocol::ProtocolError::Malformed { what: "diff format", offset: 0, detail: format!("unsupported diff format {}", bytes[0]) });
    }
    let presence = bytes[1];
    let mut reader = store::ByteReader::new(&bytes[2..]);
    let masters = if presence & 0b001 != 0 {
        let text = read_str_lp(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff masters blob", offset: 2, detail: e })?;
        Some(dec_masters_diff(&text).map_err(|e| protocol::ProtocolError::Malformed { what: "diff masters text", offset: 2, detail: e })?)
    } else {
        None
    };
    let layouts = if presence & 0b010 != 0 {
        let text = read_str_lp(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff layouts blob", offset: 2, detail: e })?;
        Some(dec_layouts_diff(&text).map_err(|e| protocol::ProtocolError::Malformed { what: "diff layouts text", offset: 2, detail: e })?)
    } else {
        None
    };
    let slides = if presence & 0b100 != 0 {
        let text = read_str_lp(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff slides blob", offset: 2, detail: e })?;
        Some(dec_slides_diff(&text).map_err(|e| protocol::ProtocolError::Malformed { what: "diff slides text", offset: 2, detail: e })?)
    } else {
        None
    };
    Ok(SemioPresentationDiff { masters, layouts, slides })
}
}

use crate::presentation::io::text::diff::{hex_encode, hex_decode, enc_str, dec_str, enc_f64, dec_f64, parse_usize, enc_list, dec_list, enc_semio_point2, dec_semio_point2, enc_frame, dec_frame, enc_image, dec_image, enc_placeholder_kind, dec_placeholder_kind, enc_table_cell, dec_table_cell, enc_table_row, dec_table_row, enc_shape, dec_shape, enc_master, dec_master, enc_layout, dec_layout, enc_slide, dec_slide, enc_indexed_triple, dec_indexed_triple, enc_named_triple, dec_named_triple, enc_frame_diff, dec_frame_diff, enc_image_diff, dec_image_diff, enc_doc_blocks_diff, dec_doc_blocks_diff, enc_table_cell_diff, dec_table_cell_diff, enc_table_cells_diff, dec_table_cells_diff, enc_table_row_diff, dec_table_row_diff, enc_table_rows_diff, dec_table_rows_diff, enc_shapes_diff, dec_shapes_diff, enc_shape_diff, dec_shape_diff, enc_master_diff, dec_master_diff, enc_layout_diff, dec_layout_diff, enc_slide_diff, dec_slide_diff, enc_masters_diff, dec_masters_diff, enc_layouts_diff, dec_layouts_diff, enc_slides_diff, dec_slides_diff};
}
pub use diff_codec::*;
