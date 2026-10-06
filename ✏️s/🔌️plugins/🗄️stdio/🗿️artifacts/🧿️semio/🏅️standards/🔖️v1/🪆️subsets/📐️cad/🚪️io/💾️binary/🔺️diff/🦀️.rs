//! 💾️ Binary representation grammar surface for `s.stdio.semio.cad.diff`.

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::cad::schema::diff::*;
use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use crate::standards::v1::subsets::base::schema::triples::{dec_named_triple, enc_named_triple, NamedModified, NamedTripleDiff};
use crate::audio::io::text::diff::{strip_brackets};
use crate::audio::io::text::diff::{split_top_level};
use crate::standards::v1::subsets::cad::schema::snapshot::{CadBlock, CadEntity, CadEntityRecord, CadLayer, SemioCadSnapshot};
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
/// `store::ByteReader`) backing the real `DiffBinary::encode_diff`/`decode_diff` below — replaces
/// the old `print_diff().into_bytes()` text-as-binary shortcut.
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

impl protocol::DiffBinary for SemioCadDiff {
/// ⚡️ Real binary diff frame, replacing the old `print_diff().into_bytes()` text-as-binary
/// shortcut. `format u8` + `presence u8` (bit0=`layers`, bit1=`blocks`, bit2=`entities`) are
/// two REAL fixed fields; each present collection then follows as its own varint-length-
/// prefixed opaque blob (the same `enc_named_triple` bracket/hex text this type's `print_diff`
/// already produces) — independently-delimited segments rather than one bare trailing `bytes`
/// because there can be 0-3 of them (chaining a `Cond` per-segment hits the
/// `protocol-cond-cannot-chain` gap: a second `if`-guard on a field that was itself only
/// conditionally decoded hard-errors `eval_cond`).
fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
    const DIFF_BINARY_FORMAT: u8 = 1;
    let mut presence = 0u8;
    if self.layers.is_some() {
        presence |= 0b0000_0001;
    }
    if self.blocks.is_some() {
        presence |= 0b0000_0010;
    }
    if self.entities.is_some() {
        presence |= 0b0000_0100;
    }
    let mut out = vec![DIFF_BINARY_FORMAT, presence];
    if let Some(v) = &self.layers {
        write_str_lp(&mut out, &enc_named_triple(v, |k: &String| enc_str(k), enc_layer_diff, enc_layer));
    }
    if let Some(v) = &self.blocks {
        write_str_lp(&mut out, &enc_named_triple(v, |k: &String| enc_str(k), enc_block_diff, enc_block));
    }
    if let Some(v) = &self.entities {
        write_str_lp(&mut out, &enc_named_triple(v, |k: &String| enc_str(k), enc_entity_record_diff, enc_entity_record));
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
    let mut next_blob = |what: &'static str| -> Result<String, protocol::ProtocolError> { read_str_lp(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what, offset: 2, detail: e }) };
    let layers = if presence & 0b0000_0001 != 0 {
        Some(dec_named_triple(&next_blob("diff layers blob")?, dec_str, dec_layer_diff, dec_layer).map_err(|e| protocol::ProtocolError::Malformed { what: "diff layers text", offset: 2, detail: e })?)
    } else {
        None
    };
    let blocks = if presence & 0b0000_0010 != 0 {
        Some(dec_named_triple(&next_blob("diff blocks blob")?, dec_str, dec_block_diff, dec_block).map_err(|e| protocol::ProtocolError::Malformed { what: "diff blocks text", offset: 2, detail: e })?)
    } else {
        None
    };
    let entities = if presence & 0b0000_0100 != 0 {
        Some(dec_named_triple(&next_blob("diff entities blob")?, dec_str, dec_entity_record_diff, dec_entity_record).map_err(|e| protocol::ProtocolError::Malformed { what: "diff entities text", offset: 2, detail: e })?)
    } else {
        None
    };
    Ok(SemioCadDiff { layers, blocks, entities })
}
}

use crate::cad::io::text::diff::{hex_encode, hex_decode, enc_str, dec_str, parse_f64, parse_i32, enc_list, dec_list, enc_point2, dec_point2, enc_entity, dec_entity, enc_layer, dec_layer, enc_entity_record, dec_entity_record, enc_block, dec_block, enc_layer_diff, dec_layer_diff, enc_entity_record_diff, dec_entity_record_diff, enc_block_diff, dec_block_diff};
}
pub use diff_codec::*;
