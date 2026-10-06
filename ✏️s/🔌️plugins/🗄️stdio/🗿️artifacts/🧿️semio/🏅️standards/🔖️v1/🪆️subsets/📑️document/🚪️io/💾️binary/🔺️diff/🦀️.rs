//! 💾️ Binary representation codec surface for `s.stdio.semio.document.diff` — protocol include.
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::document::schema::diff::*;
use crate::standards::v1::subsets::base::schema::triples::{dec_indexed_triple, dec_named_triple, enc_indexed_triple, enc_named_triple, IndexAdded, IndexModified, IndexedTripleDiff, NamedModified, NamedTripleDiff};
use crate::audio::io::text::diff::{strip_brackets};
use crate::audio::io::text::diff::{split_top_level};
use crate::standards::v1::subsets::document::schema::snapshot::{DocBlock, DocImage, DocListItem, DocRun, DocStyle, DocTableCell, DocTableRow, RunStyle, SemioDocumentSnapshot};
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





/// 🧪️ ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION document wave: real LEB128-
/// varint-length-prefixed binary primitives (`store::pack_rt::write_varint_u64` /
/// `store::ByteReader`, same helpers flow/model/brep's own upgraded `DiffCodec`s reuse) backing
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

























































































































impl protocol::DiffBinary for SemioDocumentDiff {
/// ⚡️ ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION document wave: real binary
/// diff frame, replacing the old `print_diff().into_bytes()` text-as-binary shortcut. `format
/// u8` + `presence u8` (bit0=`styles`, bit1=`images`, bit2=`blocks`) are two REAL fixed fields;
/// each present collection then follows as its own varint-length-prefixed opaque blob (the same
/// `enc_styles_diff`/`enc_images_diff`/`enc_blocks_diff` bracket/hex text `print_diff` already
/// produces) — one opaque blob per present collection rather than a per-segment `Cond` because a
/// SECOND `if`-guard on a field that's itself only conditionally decoded hard-errors `eval_cond`
/// (`protocol-cond-cannot-chain`, per the grammar recipe's own gap table; flow's/model's own
/// diff binary upgrade hit the identical shape).
fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
    const DIFF_BINARY_FORMAT: u8 = 1;
    let mut presence = 0u8;
    if self.styles.is_some() {
        presence |= 0b001;
    }
    if self.images.is_some() {
        presence |= 0b010;
    }
    if self.blocks.is_some() {
        presence |= 0b100;
    }
    let mut out = vec![DIFF_BINARY_FORMAT, presence];
    if let Some(v) = &self.styles {
        write_str_lp(&mut out, &enc_styles_diff(v));
    }
    if let Some(v) = &self.images {
        write_str_lp(&mut out, &enc_images_diff(v));
    }
    if let Some(v) = &self.blocks {
        write_str_lp(&mut out, &enc_blocks_diff(v));
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
    let styles = if presence & 0b001 != 0 {
        let text = read_str_lp(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff styles blob", offset: 2, detail: e })?;
        Some(dec_styles_diff(&text).map_err(|e| protocol::ProtocolError::Malformed { what: "diff styles text", offset: 2, detail: e })?)
    } else {
        None
    };
    let images = if presence & 0b010 != 0 {
        let text = read_str_lp(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff images blob", offset: 2, detail: e })?;
        Some(dec_images_diff(&text).map_err(|e| protocol::ProtocolError::Malformed { what: "diff images text", offset: 2, detail: e })?)
    } else {
        None
    };
    let blocks = if presence & 0b100 != 0 {
        let text = read_str_lp(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff blocks blob", offset: 2, detail: e })?;
        Some(dec_blocks_diff(&text).map_err(|e| protocol::ProtocolError::Malformed { what: "diff blocks text", offset: 2, detail: e })?)
    } else {
        None
    };
    Ok(SemioDocumentDiff { styles, images, blocks })
}
}

use crate::document::io::text::diff::{hex_encode, hex_decode, enc_str, dec_str, enc_bool, dec_bool, enc_u8, dec_u8, enc_f64, dec_f64, enc_list, dec_list, enc_run_style, dec_run_style, enc_run, dec_run, enc_block, dec_block, enc_list_item, dec_list_item, enc_cell, dec_cell, enc_row, dec_row, enc_style, dec_style, enc_image, dec_image, enc_runs_diff, dec_runs_diff, enc_blocks_diff, dec_blocks_diff, enc_list_items_diff, dec_list_items_diff, enc_table_rows_diff, dec_table_rows_diff, enc_table_cells_diff, dec_table_cells_diff, enc_styles_diff, dec_styles_diff, enc_images_diff, dec_images_diff, enc_run_style_diff, dec_run_style_diff, enc_run_diff, dec_run_diff, enc_list_item_diff, dec_list_item_diff, enc_cell_diff, dec_cell_diff, enc_row_diff, dec_row_diff, enc_style_diff, dec_style_diff, enc_image_diff, dec_image_diff, enc_paragraph_diff, dec_paragraph_diff, enc_heading_diff, dec_heading_diff, enc_list_diff, dec_list_diff, enc_table_diff, dec_table_diff, enc_code_diff, dec_code_diff, enc_quote_diff, dec_quote_diff, enc_image_block_diff, dec_image_block_diff, enc_block_diff, dec_block_diff};
}
pub use diff_codec::*;
