//! 💾️ Binary representation codec surface for `s.stdio.semio.audio` (diff).

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::audio::schema::diff::*;
use crate::standards::v1::subsets::audio::schema::snapshot::{SemioAudioChannel, SemioAudioFormat, SemioAudioSnapshot, SemioAudioTag};
use crate::standards::v1::subsets::base::schema::triples::{self, IndexAdded, IndexModified, IndexedTripleDiff};
use protocol::command::DiffAlgebra;
/// 🔧️ Unconditional — `impl protocol::DiffCodec for SemioAudioDiff` below's `encode_diff`/
/// `decode_diff` are now real production code (binary upgrade, this wave), not test-only.
use protocol::{DiffBinary,DiffCodec,DiffText};
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
/// `store::ByteReader`, same helpers this subset's own `📸️snapshot` facet's `ArtifactPack` uses)
/// backing the real `DiffBinary::encode_diff`/`decode_diff` below.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    store::pack_rt::write_varint_u64(out, s.len() as u64);
    out.extend_from_slice(s.as_bytes());
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    let bytes = reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec();
    String::from_utf8(bytes).map_err(|e| e.to_string())
}

impl protocol::DiffBinary for SemioAudioDiff {
/// ⚡️ Real binary diff frame, replacing the old `print_diff().into_bytes()` text-as-binary
/// shortcut. `format u8` + `presence u8` (bit0=`sample_rate` bit1=`format` bit2=`channels`
/// bit3=`tags`) are two REAL fixed fields; each present field then follows as its own
/// varint-length-prefixed opaque text blob (the same per-field `rate=`/`format=`/
/// `enc_indexed_triple`-based text `print_diff` already produces) — independently-delimited
/// segments rather than one bare trailing `bytes` because there can be 0-4 of them (chaining a
/// `Cond` per-segment hits the `protocol-cond-cannot-chain` gap: a second `if`-guard on a field
/// that was itself only conditionally decoded hard-errors `eval_cond` — see `🌊️flow`'s/
/// `🖼️image`'s pilot reports).
fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
    const DIFF_BINARY_FORMAT: u8 = 1;
    let mut presence = 0u8;
    if self.sample_rate.is_some() {
        presence |= 0b0001;
    }
    if self.format.is_some() {
        presence |= 0b0010;
    }
    if self.channels.is_some() {
        presence |= 0b0100;
    }
    if self.tags.is_some() {
        presence |= 0b1000;
    }
    let mut out = vec![DIFF_BINARY_FORMAT, presence];
    if let Some(v) = self.sample_rate {
        write_str_lp(&mut out, &v.to_string());
    }
    if let Some(v) = self.format {
        write_str_lp(&mut out, enc_format(v));
    }
    if let Some(v) = &self.channels {
        write_str_lp(&mut out, &triples::enc_indexed_triple(v, enc_channel_diff, enc_channel));
    }
    if let Some(v) = &self.tags {
        write_str_lp(&mut out, &triples::enc_indexed_triple(v, enc_tag, enc_tag));
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
    let sample_rate = if presence & 0b0001 != 0 {
        let text = read_str_lp(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff sample_rate blob", offset: 2, detail: e })?;
        Some(parse_u32(&text).map_err(|e| protocol::ProtocolError::Malformed { what: "diff sample_rate text", offset: 2, detail: e })?)
    } else {
        None
    };
    let format = if presence & 0b0010 != 0 {
        let text = read_str_lp(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff format blob", offset: 2, detail: e })?;
        Some(dec_format(&text).map_err(|e| protocol::ProtocolError::Malformed { what: "diff format text", offset: 2, detail: e })?)
    } else {
        None
    };
    let channels = if presence & 0b0100 != 0 {
        let text = read_str_lp(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff channels blob", offset: 2, detail: e })?;
        Some(triples::dec_indexed_triple(&text, dec_channel_diff, dec_channel).map_err(|e| protocol::ProtocolError::Malformed { what: "diff channels text", offset: 2, detail: e })?)
    } else {
        None
    };
    let tags = if presence & 0b1000 != 0 {
        let text = read_str_lp(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff tags blob", offset: 2, detail: e })?;
        Some(triples::dec_indexed_triple(&text, dec_tag, dec_tag).map_err(|e| protocol::ProtocolError::Malformed { what: "diff tags text", offset: 2, detail: e })?)
    } else {
        None
    };
    Ok(SemioAudioDiff { sample_rate, format, channels, tags })
}
}

use crate::audio::io::text::diff::{hex_encode, hex_decode, hex_decode_string, parse_u32, split_top_level, strip_brackets, enc_format, dec_format, enc_f32_list, dec_f32_list, enc_channel, dec_channel, enc_channel_diff, dec_channel_diff, enc_tag, dec_tag};
}
pub use diff_codec::*;
