//! 💾️ Binary representation codec surface for `s.stdio.semio.flow` (diff) — protocol include.
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::flow::schema::diff::*;
use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use crate::standards::v1::subsets::base::schema::triples::{dec_named_triple, enc_named_triple, NamedModified, NamedTripleDiff};
use crate::audio::io::text::diff::{strip_brackets};
use crate::audio::io::text::diff::{split_top_level};
use crate::standards::v1::subsets::flow::schema::snapshot::{FlowEdge, FlowNode, FlowParam, PortRef, SemioFlowSnapshot};
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

/// 🧪️ P2 pilot: real LEB128-varint-length-prefixed binary primitives (`store::pack_rt::
/// write_varint_u64` / `store::ByteReader` — same helpers `stdio.json`'s upgraded `DiffCodec`
/// reuses) backing the real `DiffBinary::encode_diff`/`decode_diff` below.
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













































impl protocol::DiffBinary for SemioFlowDiff {
/// ⚡️ P2 pilot: real binary diff frame, replacing the old `print_diff().into_bytes()`
/// text-as-binary shortcut. `format u8` + `presence u8` (bit0 = `nodes` present, bit1 =
/// `edges` present) are two REAL fixed fields; each present collection then follows as its own
/// varint-length-prefixed opaque blob (the same `enc_nodes_diff`/`enc_edges_diff` bracket/hex
/// text this type's `print_diff` already produces) — two independently-delimited segments
/// rather than one bare trailing `bytes` because there can be 0, 1, or 2 of them (chaining a
/// `Cond` per-segment hits the `protocol-cond-cannot-chain` gap: a second `if`-guard on a field
/// that was itself only conditionally decoded hard-errors `eval_cond` — see this wave's report).
fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
    const DIFF_BINARY_FORMAT: u8 = 1;
    let mut presence = 0u8;
    if self.nodes.is_some() {
        presence |= 0b01;
    }
    if self.edges.is_some() {
        presence |= 0b10;
    }
    let mut out = vec![DIFF_BINARY_FORMAT, presence];
    if let Some(v) = &self.nodes {
        write_str_lp(&mut out, &enc_nodes_diff(v));
    }
    if let Some(v) = &self.edges {
        write_str_lp(&mut out, &enc_edges_diff(v));
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
    let nodes = if presence & 0b01 != 0 {
        let text = read_str_lp(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff nodes blob", offset: 2, detail: e })?;
        Some(dec_nodes_diff(&text).map_err(|e| protocol::ProtocolError::Malformed { what: "diff nodes text", offset: 2, detail: e })?)
    } else {
        None
    };
    let edges = if presence & 0b10 != 0 {
        let text = read_str_lp(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff edges blob", offset: 2, detail: e })?;
        Some(dec_edges_diff(&text).map_err(|e| protocol::ProtocolError::Malformed { what: "diff edges text", offset: 2, detail: e })?)
    } else {
        None
    };
    Ok(SemioFlowDiff { nodes, edges })
}
}

use crate::flow::io::text::diff::{hex_encode, hex_decode, enc_str, dec_str, enc_f64, dec_f64, enc_point2, dec_point2, enc_port_ref, dec_port_ref, enc_param, dec_param, enc_node, dec_node, enc_edge, dec_edge, enc_param_diff, dec_param_diff, enc_params_diff, dec_params_diff, enc_node_diff, dec_node_diff, enc_nodes_diff, dec_nodes_diff, enc_edge_diff, dec_edge_diff, enc_edges_diff, dec_edges_diff};
}
pub use diff_codec::*;
