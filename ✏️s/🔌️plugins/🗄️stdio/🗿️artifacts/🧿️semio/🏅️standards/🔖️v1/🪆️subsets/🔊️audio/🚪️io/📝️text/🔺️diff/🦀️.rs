//! 📝️ Text representation codec surface for `s.stdio.semio.audio` (diff).

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type SemioAudioDiffText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v1::subsets::audio::io::binary::diff::{encode_option, decode_option};
use crate::standards::v1::subsets::base::io::text::snapshot::{enc_indexed_triple, dec_indexed_triple};
use crate::standards::v1::subsets::audio::schema::diff::*;
use crate::standards::v1::subsets::audio::schema::snapshot::{SemioAudioChannel, SemioAudioFormat, SemioAudioSnapshot, SemioAudioTag};
use crate::standards::v1::subsets::base::schema::triples::{IndexAdded, IndexModified, IndexedTripleDiff};
use crate::standards::v1::subsets::base::io::text::snapshot as triples;
use protocol::command::DiffAlgebra;
/// 🔧️ Unconditional — `impl protocol::DiffCodec for SemioAudioDiff` below's `encode_diff`/
/// `decode_diff` are now real production code (binary upgrade, this wave), not test-only.
use protocol::{DiffBinary,DiffCodec,DiffText};
use protocol::MutationDiff;
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_audio_diff(d: &SemioAudioDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = d.sample_rate {
        tokens.push(format!("rate={v}"));
    }
    if let Some(v) = d.format {
        tokens.push(format!("format={}", enc_format(v)));
    }
    if let Some(v) = &d.channels {
        tokens.push(format!("channels{{{}}}", triples::enc_indexed_triple(v, enc_channel_diff, enc_channel)));
    }
    if let Some(v) = &d.tags {
        tokens.push(format!("tags{{{}}}", triples::enc_indexed_triple(v, enc_tag, enc_tag)));
    }
    tokens.join(" ")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_audio_diff(line: &str) -> Result<SemioAudioDiff, String> {
    let mut d = SemioAudioDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("rate=") {
            d.sample_rate = Some(parse_u32(rest)?);
        } else if let Some(rest) = token.strip_prefix("format=") {
            d.format = Some(dec_format(rest)?);
        } else if let Some(rest) = token.strip_prefix("channels{") {
            let body = rest.strip_suffix('}').ok_or_else(|| "channels: missing closing brace".to_string())?;
            d.channels = Some(triples::dec_indexed_triple(body, dec_channel_diff, dec_channel)?);
        } else if let Some(rest) = token.strip_prefix("tags{") {
            let body = rest.strip_suffix('}').ok_or_else(|| "tags: missing closing brace".to_string())?;
            d.tags = Some(triples::dec_indexed_triple(body, dec_tag, dec_tag)?);
        } else {
            return Err(format!("audio diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl protocol::DiffText for SemioAudioDiff {
fn print_diff(&self) -> String {
    print_audio_diff(self)
}
fn parse_diff(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
    parse_audio_diff(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
}
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if !s.len().is_multiple_of(2) {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_decode_string(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_u32(s: &str) -> Result<u32, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn split_top_level(s: &str, sep: char) -> Vec<&str> {
    crate::standards::v1::subsets::base::io::text::snapshot::split_top_level(s, sep)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn strip_brackets(s: &str) -> Result<&str, String> {
    crate::standards::v1::subsets::base::io::text::snapshot::strip_brackets(s)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_format(f: SemioAudioFormat) -> &'static str {
    match f {
        SemioAudioFormat::Pcm8 => "pcm8",
        SemioAudioFormat::Pcm16 => "pcm16",
        SemioAudioFormat::Pcm24 => "pcm24",
        SemioAudioFormat::Pcm32 => "pcm32",
        SemioAudioFormat::Float32 => "f32",
        SemioAudioFormat::Float64 => "f64",
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_format(s: &str) -> Result<SemioAudioFormat, String> {
    match s {
        "pcm8" => Ok(SemioAudioFormat::Pcm8),
        "pcm16" => Ok(SemioAudioFormat::Pcm16),
        "pcm24" => Ok(SemioAudioFormat::Pcm24),
        "pcm32" => Ok(SemioAudioFormat::Pcm32),
        "f32" => Ok(SemioAudioFormat::Float32),
        "f64" => Ok(SemioAudioFormat::Float64),
        other => Err(format!("bad audio format {other:?}")),
    }
}

/// 🔢️ Exact-round-trip `f32` list — `to_bits()` hex tokens inside a bracket, never decimal
/// text (sidesteps float-formatting precision loss and NaN/-0.0 print-ambiguity entirely).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_f32_list(v: &[f32]) -> String {
    format!("[{}]", v.iter().map(|f| format!("{:08x}", f.to_bits())).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_f32_list(s: &str) -> Result<Vec<f32>, String> {
    let inner = strip_brackets(s)?;
    if inner.is_empty() {
        return Ok(Vec::new());
    }
    split_top_level(inner, ',').into_iter().map(|tok| u32::from_str_radix(tok, 16).map(f32::from_bits).map_err(|e| e.to_string())).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_channel(c: &SemioAudioChannel) -> String {
    enc_f32_list(&c.samples)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_channel(s: &str) -> Result<SemioAudioChannel, String> {
    Ok(SemioAudioChannel { samples: dec_f32_list(s)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_channel_diff(d: &SemioAudioChannelDiff) -> String {
    encode_option(&d.samples, |v| enc_f32_list(v))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_channel_diff(s: &str) -> Result<SemioAudioChannelDiff, String> {
    Ok(SemioAudioChannelDiff { samples: decode_option(s, dec_f32_list)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_tag(t: &SemioAudioTag) -> String {
    format!("[{},{}]", hex_encode(t.key.as_bytes()), hex_encode(t.value.as_bytes()))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_tag(s: &str) -> Result<SemioAudioTag, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [key, value] = parts.as_slice() else { return Err(format!("tag: expected 2 fields, got {}", parts.len())) };
    Ok(SemioAudioTag { key: hex_decode_string(key)?, value: hex_decode_string(value)? })
}
}
pub use diff_codec::*;

#[allow(unused_imports)]
mod residual_diff_helper {
use crate::standards::v1::subsets::audio::schema::diff::*;
use crate::standards::v1::subsets::audio::schema::snapshot::{SemioAudioChannel, SemioAudioFormat, SemioAudioSnapshot, SemioAudioTag};
use crate::standards::v1::subsets::base::schema::triples::{IndexAdded, IndexModified, IndexedTripleDiff};
use crate::standards::v1::subsets::base::io::text::snapshot as triples;
use protocol::command::DiffAlgebra;
/// 🔧️ Unconditional — `impl protocol::DiffCodec for SemioAudioDiff` below's `encode_diff`/
/// `decode_diff` are now real production code (binary upgrade, this wave), not test-only.
use protocol::{DiffBinary,DiffCodec,DiffText};
use protocol::MutationDiff;
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_usize(s: &str) -> Result<usize, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}
}
pub(crate) use residual_diff_helper::*;
