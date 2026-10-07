//! 💾️ Binary representation codec surface for `stdio.semio.image` (diff) — protocol include.
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v1::subsets::image::schema::diff::*;
use crate::standards::v1::subsets::base::schema::triples::{IndexAdded, IndexModified, IndexedTripleDiff, NamedModified, NamedTripleDiff};
use crate::standards::v1::subsets::base::io::text::snapshot::{dec_indexed_triple, dec_named_triple, enc_indexed_triple, enc_named_triple};
use crate::standards::v1::subsets::audio::io::text::diff::{strip_brackets};
use crate::standards::v1::subsets::audio::io::text::diff::{split_top_level};
use crate::standards::v1::subsets::image::schema::snapshot::{SemioColorspace, SemioImageFrame, SemioImageMetadataEntry, SemioImageSnapshot};
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
/// 🔧️ Unconditional — the `#[cfg(test)] mod tests` block below calls `print_diff`/`parse_diff`/
/// `encode_diff`/`decode_diff` via method syntax on `SemioImageDiff`, which needs `DiffCodec` in
/// scope (the `impl protocol::DiffCodec for SemioImageDiff` block itself compiles fine unqualified,
/// but callers using method syntax do not get the trait for free) (W2b closer fix).
use protocol::{DiffBinary,DiffCodec,DiffText};
use protocol::MutationDiff;













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

























impl protocol::DiffBinary for SemioImageDiff {
/// ⚡️ Real binary diff frame, replacing the old `print_diff().into_bytes()` text-as-binary
/// shortcut. `format u8` + `presence u8` (bit0=`width` bit1=`height` bit2=`colorspace`
/// bit3=`bitDepth` bit4=`icc` bit5=`frames` bit6=`metadata`) are two REAL fixed fields; each
/// present field then follows as its own varint-length-prefixed opaque text blob (the same
/// per-field `enc_*`/`enc_frames_diff`/`enc_metadata_diff` text `print_diff` already produces)
/// — independently-delimited segments rather than one bare trailing `bytes` because there can
/// be 0-7 of them (chaining a `Cond` per-segment hits the `protocol-cond-cannot-chain` gap: a
/// second `if`-guard on a field that was itself only conditionally decoded hard-errors
/// `eval_cond` — see `🌊️flow`'s/`🔺️mesh`'s pilot reports).
fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
    const DIFF_BINARY_FORMAT: u8 = 1;
    let mut presence = 0u8;
    if self.width.is_some() {
        presence |= 0b0000_0001;
    }
    if self.height.is_some() {
        presence |= 0b0000_0010;
    }
    if self.colorspace.is_some() {
        presence |= 0b0000_0100;
    }
    if self.bit_depth.is_some() {
        presence |= 0b0000_1000;
    }
    if self.icc.is_some() {
        presence |= 0b0001_0000;
    }
    if self.frames.is_some() {
        presence |= 0b0010_0000;
    }
    if self.metadata.is_some() {
        presence |= 0b0100_0000;
    }
    let mut out = vec![DIFF_BINARY_FORMAT, presence];
    if let Some(v) = self.width {
        write_str_lp(&mut out, &v.to_string());
    }
    if let Some(v) = self.height {
        write_str_lp(&mut out, &v.to_string());
    }
    if let Some(v) = self.colorspace {
        write_str_lp(&mut out, &enc_colorspace(v).to_string());
    }
    if let Some(v) = self.bit_depth {
        write_str_lp(&mut out, &v.to_string());
    }
    if let Some(v) = &self.icc {
        write_str_lp(&mut out, &encode_option(v, |b| hex_encode(b)));
    }
    if let Some(v) = &self.frames {
        write_str_lp(&mut out, &enc_frames_diff(v));
    }
    if let Some(v) = &self.metadata {
        write_str_lp(&mut out, &enc_metadata_diff(v));
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
    let width = if presence & 0b0000_0001 != 0 {
        let text = read_str_lp(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff width blob", offset: 2, detail: e })?;
        Some(parse_u32(&text).map_err(|e| protocol::ProtocolError::Malformed { what: "diff width text", offset: 2, detail: e })?)
    } else {
        None
    };
    let height = if presence & 0b0000_0010 != 0 {
        let text = read_str_lp(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff height blob", offset: 2, detail: e })?;
        Some(parse_u32(&text).map_err(|e| protocol::ProtocolError::Malformed { what: "diff height text", offset: 2, detail: e })?)
    } else {
        None
    };
    let colorspace = if presence & 0b0000_0100 != 0 {
        let text = read_str_lp(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff colorspace blob", offset: 2, detail: e })?;
        Some(dec_colorspace(&text).map_err(|e| protocol::ProtocolError::Malformed { what: "diff colorspace text", offset: 2, detail: e })?)
    } else {
        None
    };
    let bit_depth = if presence & 0b0000_1000 != 0 {
        let text = read_str_lp(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff bit_depth blob", offset: 2, detail: e })?;
        Some(parse_u8(&text).map_err(|e| protocol::ProtocolError::Malformed { what: "diff bit_depth text", offset: 2, detail: e })?)
    } else {
        None
    };
    let icc = if presence & 0b0001_0000 != 0 {
        let text = read_str_lp(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff icc blob", offset: 2, detail: e })?;
        Some(decode_option(&text, hex_decode).map_err(|e| protocol::ProtocolError::Malformed { what: "diff icc text", offset: 2, detail: e })?)
    } else {
        None
    };
    let frames = if presence & 0b0010_0000 != 0 {
        let text = read_str_lp(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff frames blob", offset: 2, detail: e })?;
        Some(dec_frames_diff(&text).map_err(|e| protocol::ProtocolError::Malformed { what: "diff frames text", offset: 2, detail: e })?)
    } else {
        None
    };
    let metadata = if presence & 0b0100_0000 != 0 {
        let text = read_str_lp(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff metadata blob", offset: 2, detail: e })?;
        Some(dec_metadata_diff(&text).map_err(|e| protocol::ProtocolError::Malformed { what: "diff metadata text", offset: 2, detail: e })?)
    } else {
        None
    };
    Ok(SemioImageDiff { width, height, colorspace, bit_depth, icc, frames, metadata })
}
}

use crate::standards::v1::subsets::image::io::text::diff::{hex_encode, hex_decode, hex_encode_str, hex_decode_str, parse_u8, parse_u32, enc_colorspace, dec_colorspace, enc_frame, dec_frame, enc_frame_diff, dec_frame_diff, enc_metadata_entry, dec_metadata_entry, enc_frames_diff, dec_frames_diff, enc_metadata_diff, dec_metadata_diff};
}
pub use diff_codec::*;
