//! 💾️ Binary representation grammar surface for `stdio.semio.video` (diff): real binary diff
//! frame — `format u8` + `presence u8` (bit0 = `streams`) real and fully described, the optional
//! `streams` blob an opaque trailing payload (video wave, replacing the old
//! `print_diff().into_bytes()` text-as-binary shortcut) — actual encode/decode lives on
//! `SemioVideoDiff`'s `protocol::DiffCodec` impl in the facet root `🦀️.rs`; this leaf
//! carries the normative protocol description.

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v1::subsets::video::schema::diff::*;
use crate::standards::v1::subsets::base::schema::triples::{IndexAdded, IndexModified, IndexedTripleDiff};
use crate::standards::v1::subsets::base::io::text::snapshot::{dec_indexed_triple, enc_indexed_triple};
use crate::standards::v1::subsets::audio::io::text::diff::{strip_brackets};
use crate::standards::v1::subsets::audio::io::text::diff::{split_top_level};
use crate::standards::v1::subsets::video::schema::snapshot::{SemioRational, SemioVideoSample, SemioVideoSnapshot, SemioVideoStream, SemioVideoStreamKind};
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
/// `store::ByteReader`, same helpers flow's/mesh's upgraded diff facets reuse) backing the
/// real `DiffBinary::encode_diff`/`decode_diff` below.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    store::pack_rt::write_varint_u64(out, s.len() as u64);
    out.extend_from_slice(s.as_bytes());
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    String::from_utf8(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec()).map_err(|e| e.to_string())
}

impl protocol::DiffBinary for SemioVideoDiff {
/// ⚡️ Real binary diff frame, replacing the old `print_diff().into_bytes()` text-as-binary
/// shortcut (same treatment flow's/mesh's own upgraded diff facets use). `format u8` +
/// `presence u8` (bit0 = `streams` present) are two REAL fixed fields; when present, `streams`
/// follows as one varint-length-prefixed opaque blob (the same `enc_streams_diff` bracket/hex
/// text `print_diff` already emits) — a length-prefixed segment rather than a bare trailing
/// `bytes` chain so the shape stays uniform with flow's/mesh's multi-field diff frames
/// (`protocol-cond-cannot-chain`: a second `if`-guard on a field that's itself only
/// conditionally decoded hard-errors `eval_cond` — see this wave's report).
fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
    const DIFF_BINARY_FORMAT: u8 = 1;
    let presence: u8 = if self.streams.is_some() { 0b01 } else { 0b00 };
    let mut out = vec![DIFF_BINARY_FORMAT, presence];
    if let Some(v) = &self.streams {
        write_str_lp(&mut out, &enc_streams_diff(v));
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
    let streams = if presence & 0b01 != 0 {
        let text = read_str_lp(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff streams blob", offset: 2, detail: e })?;
        Some(dec_streams_diff(&text).map_err(|e| protocol::ProtocolError::Malformed { what: "diff streams text", offset: 2, detail: e })?)
    } else {
        None
    };
    Ok(SemioVideoDiff { streams })
}
}

use crate::standards::v1::subsets::video::io::text::diff::{hex_encode, hex_decode, enc_str, dec_str, enc_bool, dec_bool, enc_list, dec_list, enc_kind, dec_kind, enc_rational, dec_rational, enc_sample, dec_sample, enc_stream, dec_stream, enc_sample_diff, dec_sample_diff, enc_samples_diff, dec_samples_diff, enc_stream_diff, dec_stream_diff, enc_streams_diff, dec_streams_diff};
}
pub use diff_codec::*;
