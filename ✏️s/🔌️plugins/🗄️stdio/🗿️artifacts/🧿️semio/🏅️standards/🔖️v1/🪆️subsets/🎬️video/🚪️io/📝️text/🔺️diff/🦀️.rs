//! 📝️ Text representation grammar surface for `stdio.semio.video` (diff): the `streams=` triple
//! grammar — actual print/parse lives on `SemioVideoDiff`'s `protocol::DiffCodec` impl in the
//! facet root `🦀️.rs`; this leaf carries the normative grammar description.

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v1::subsets::video::io::binary::diff::{encode_option, decode_option};
use crate::standards::v1::subsets::video::schema::diff::*;
use crate::standards::v1::subsets::base::schema::triples::{IndexAdded, IndexModified, IndexedTripleDiff};
use crate::standards::v1::subsets::base::io::text::snapshot::{dec_indexed_triple, enc_indexed_triple};
use crate::standards::v1::subsets::audio::io::text::diff::{strip_brackets};
use crate::standards::v1::subsets::audio::io::text::diff::{split_top_level};
use crate::standards::v1::subsets::video::schema::snapshot::{SemioRational, SemioVideoSample, SemioVideoSnapshot, SemioVideoStream, SemioVideoStreamKind};
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_semio_video_diff(d: &SemioVideoDiff) -> String {
    match &d.streams {
        Some(v) => format!("streams={}", enc_streams_diff(v)),
        None => String::new(),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_semio_video_diff(line: &str) -> Result<SemioVideoDiff, String> {
    if line.is_empty() {
        return Ok(SemioVideoDiff::default());
    }
    let rest = line.strip_prefix("streams=").ok_or_else(|| format!("semio video diff: unknown token {line:?}"))?;
    Ok(SemioVideoDiff { streams: Some(dec_streams_diff(rest)?) })
}

impl protocol::DiffText for SemioVideoDiff {
fn print_diff(&self) -> String {
    print_semio_video_diff(self)
}
fn parse_diff(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
    parse_semio_video_diff(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
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
pub(crate) fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_bool(b: &bool) -> String {
    if *b {
        "1".to_string()
    } else {
        "0".to_string()
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_bool(s: &str) -> Result<bool, String> {
    match s {
        "1" => Ok(true),
        "0" => Ok(false),
        other => Err(format!("bool: bad value {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_list<T>(items: &[T], enc: impl Fn(&T) -> String) -> String {
    format!("[{}]", items.iter().map(enc).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_list<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Vec<T>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_kind(k: &SemioVideoStreamKind) -> String {
    match k {
        SemioVideoStreamKind::Video => "V",
        SemioVideoStreamKind::Audio => "A",
        SemioVideoStreamKind::Subtitle => "S",
    }
    .to_string()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_kind(s: &str) -> Result<SemioVideoStreamKind, String> {
    match s {
        "V" => Ok(SemioVideoStreamKind::Video),
        "A" => Ok(SemioVideoStreamKind::Audio),
        "S" => Ok(SemioVideoStreamKind::Subtitle),
        other => Err(format!("stream kind: bad value {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_rational(r: &SemioRational) -> String {
    format!("[{},{}]", r.num, r.den)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_rational(s: &str) -> Result<SemioRational, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [num, den] = parts.as_slice() else { return Err(format!("rational: expected 2 fields, got {}", parts.len())) };
    Ok(SemioRational { num: num.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, den: den.parse().map_err(|e: std::num::ParseIntError| e.to_string())? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_sample(s: &SemioVideoSample) -> String {
    format!("[{},{},{}]", s.pts, enc_bool(&s.key), hex_encode(&s.data))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_sample(s: &str) -> Result<SemioVideoSample, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [pts, key, data] = parts.as_slice() else { return Err(format!("sample: expected 3 fields, got {}", parts.len())) };
    Ok(SemioVideoSample { pts: pts.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, key: dec_bool(key)?, data: hex_decode(data)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_stream(s: &SemioVideoStream) -> String {
    format!("[{},{},{},{},{},{}]", enc_kind(&s.kind), enc_str(&s.codec), s.width, s.height, enc_rational(&s.rate), enc_list(&s.samples, enc_sample))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_stream(s: &str) -> Result<SemioVideoStream, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [kind, codec, width, height, rate, samples] = parts.as_slice() else { return Err(format!("stream: expected 6 fields, got {}", parts.len())) };
    Ok(SemioVideoStream {
        kind: dec_kind(kind)?,
        codec: dec_str(codec)?,
        width: width.parse().map_err(|e: std::num::ParseIntError| e.to_string())?,
        height: height.parse().map_err(|e: std::num::ParseIntError| e.to_string())?,
        rate: dec_rational(rate)?,
        samples: dec_list(samples, dec_sample)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_sample_diff(d: &SemioVideoSampleDiff) -> String {
    format!("[{},{},{}]", encode_option(&d.pts, |v| v.to_string()), encode_option(&d.key, enc_bool), encode_option(&d.data, |v| hex_encode(v)))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_sample_diff(s: &str) -> Result<SemioVideoSampleDiff, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [pts, key, data] = parts.as_slice() else { return Err(format!("sample diff: expected 3 fields, got {}", parts.len())) };
    Ok(SemioVideoSampleDiff { pts: decode_option(pts, |v| v.parse().map_err(|e: std::num::ParseIntError| e.to_string()))?, key: decode_option(key, dec_bool)?, data: decode_option(data, hex_decode)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_samples_diff(d: &SemioVideoSamplesDiff) -> String {
    enc_indexed_triple(d, enc_sample_diff, enc_sample)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_samples_diff(s: &str) -> Result<SemioVideoSamplesDiff, String> {
    dec_indexed_triple(s, dec_sample_diff, dec_sample)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_stream_diff(d: &SemioVideoStreamDiff) -> String {
    format!(
        "[{},{},{},{},{},{}]",
        encode_option(&d.kind, enc_kind),
        encode_option(&d.codec, |v| enc_str(v)),
        encode_option(&d.width, |v| v.to_string()),
        encode_option(&d.height, |v| v.to_string()),
        encode_option(&d.rate, enc_rational),
        encode_option(&d.samples, enc_samples_diff),
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_stream_diff(s: &str) -> Result<SemioVideoStreamDiff, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [kind, codec, width, height, rate, samples] = parts.as_slice() else { return Err(format!("stream diff: expected 6 fields, got {}", parts.len())) };
    Ok(SemioVideoStreamDiff {
        kind: decode_option(kind, dec_kind)?,
        codec: decode_option(codec, dec_str)?,
        width: decode_option(width, |v| v.parse().map_err(|e: std::num::ParseIntError| e.to_string()))?,
        height: decode_option(height, |v| v.parse().map_err(|e: std::num::ParseIntError| e.to_string()))?,
        rate: decode_option(rate, dec_rational)?,
        samples: decode_option(samples, dec_samples_diff)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_streams_diff(d: &SemioVideoStreamsDiff) -> String {
    enc_indexed_triple(d, enc_stream_diff, enc_stream)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_streams_diff(s: &str) -> Result<SemioVideoStreamsDiff, String> {
    dec_indexed_triple(s, dec_stream_diff, dec_stream)
}
}
pub use diff_codec::*;

#[allow(unused_imports)]
mod residual_diff_helper {
use crate::standards::v1::subsets::video::schema::diff::*;
use crate::standards::v1::subsets::base::schema::triples::{IndexAdded, IndexModified, IndexedTripleDiff};
use crate::standards::v1::subsets::base::io::text::snapshot::{dec_indexed_triple, enc_indexed_triple};
use crate::standards::v1::subsets::audio::io::text::diff::{strip_brackets};
use crate::standards::v1::subsets::audio::io::text::diff::{split_top_level};
use crate::standards::v1::subsets::video::schema::snapshot::{SemioRational, SemioVideoSample, SemioVideoSnapshot, SemioVideoStream, SemioVideoStreamKind};
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_usize(s: &str) -> Result<usize, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}
}
pub(crate) use residual_diff_helper::*;
