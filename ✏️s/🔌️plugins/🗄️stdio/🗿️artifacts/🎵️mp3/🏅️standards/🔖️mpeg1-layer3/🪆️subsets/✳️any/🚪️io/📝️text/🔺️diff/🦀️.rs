//! 🚧 scaffolded by W1b — text representation marker for `stdio.mp3.diff`. Full grammar-backed
//! parse/print lands in W2/W3.
pub const TEXT_MARKER: &str = "stdio.mp3.diff";

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::mpeg1_layer3::subsets::any::schema::diff::*;
use protocol::{DiffText,DiffBinary};
use crate::standards::mpeg1_layer3::subsets::any::schema::snapshot::{Id3Frame, Id3v1Tag, Id3v2Tag, Mp3Frame, Mp3FrameHeader, Mp3Snapshot};
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;

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

/// 🧭️ Bracket-depth-aware split (tracks `[`/`]` only) — the shared grammar contract every
/// hand-rolled codec in this repo uses (`f6-recon-report.md` §5), kept verbatim.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn split_top_level(s: &str, sep: char) -> Vec<&str> {
    if s.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut depth = 0i32;
    let mut start = 0usize;
    for (i, c) in s.char_indices() {
        match c {
            '[' => depth += 1,
            ']' => depth -= 1,
            c if c == sep && depth == 0 => {
                out.push(&s[start..i]);
                start = i + c.len_utf8();
            }
            _ => {}
        }
    }
    out.push(&s[start..]);
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn strip_brackets(s: &str) -> Result<&str, String> {
    s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_u8(s: &str) -> Result<u8, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_u16(s: &str) -> Result<u16, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_bool(s: &str) -> Result<bool, String> {
    match s {
        "1" => Ok(true),
        "0" => Ok(false),
        other => Err(format!("bad bool {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_bool(b: bool) -> &'static str {
    if b {
        "1"
    } else {
        "0"
    }
}

/// 🧭️ `Id3Frame.id` is a 4-char printable ID3 frame id (`TIT2`/`TPE1`/…) — never contains
/// `,`/`[`/`]`/`;` in practice, so it's safe as a bare top-level token.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_id3_frame(f: &Id3Frame) -> String {
    format!("[{},{},{}]", f.id, f.flags, hex_encode(&f.data))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_id3_frame(s: &str) -> Result<Id3Frame, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    if parts.len() != 3 {
        return Err(format!("id3 frame: expected 3 fields, got {}", parts.len()));
    }
    Ok(Id3Frame { id: parts[0].to_string(), flags: parse_u16(parts[1])?, data: hex_decode(parts[2])? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_id3_frames(frames: &[Id3Frame]) -> String {
    format!("[{}]", frames.iter().map(enc_id3_frame).collect::<Vec<_>>().join(";"))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_id3_frames(s: &str) -> Result<Vec<Id3Frame>, String> {
    let inner = strip_brackets(s)?;
    split_top_level(inner, ';').into_iter().filter(|p| !p.is_empty()).map(dec_id3_frame).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_id3v2(tag: &Id3v2Tag) -> String {
    format!("[{},{},{},{}]", tag.major_version, tag.minor_version, tag.flags, enc_id3_frames(&tag.frames))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_id3v2(s: &str) -> Result<Id3v2Tag, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    if parts.len() != 4 {
        return Err(format!("id3v2: expected 4 fields, got {}", parts.len()));
    }
    Ok(Id3v2Tag { major_version: parse_u8(parts[0])?, minor_version: parse_u8(parts[1])?, flags: parse_u8(parts[2])?, frames: dec_id3_frames(parts[3])? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_id3v1(tag: &Id3v1Tag) -> String {
    format!("[{}]", hex_encode(&tag.raw))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_id3v1(s: &str) -> Result<Id3v1Tag, String> {
    Ok(Id3v1Tag { raw: hex_decode(strip_brackets(s)?)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_mp3_header(h: &Mp3FrameHeader) -> String {
    format!(
        "[{},{},{},{},{},{},{},{},{},{},{},{}]",
        h.mpeg_version_id,
        h.layer,
        enc_bool(h.protection_bit),
        h.bitrate_index,
        h.sample_rate_index,
        enc_bool(h.padding),
        enc_bool(h.private_bit),
        h.channel_mode,
        h.mode_extension,
        enc_bool(h.copyright),
        enc_bool(h.original),
        h.emphasis
    )
}

/// 🧭️ `s` is the header's OWN bracketed group (e.g. `[3,1,1,9,0,0,0,3,0,0,1,0]`) — callers strip
/// it from its enclosing token first (see `dec_mp3_frame`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_mp3_header(s: &str) -> Result<Mp3FrameHeader, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    if parts.len() != 12 {
        return Err(format!("mp3 frame header: expected 12 fields, got {}", parts.len()));
    }
    Ok(Mp3FrameHeader {
        mpeg_version_id: parse_u8(parts[0])?,
        layer: parse_u8(parts[1])?,
        protection_bit: parse_bool(parts[2])?,
        bitrate_index: parse_u8(parts[3])?,
        sample_rate_index: parse_u8(parts[4])?,
        padding: parse_bool(parts[5])?,
        private_bit: parse_bool(parts[6])?,
        channel_mode: parse_u8(parts[7])?,
        mode_extension: parse_u8(parts[8])?,
        copyright: parse_bool(parts[9])?,
        original: parse_bool(parts[10])?,
        emphasis: parse_u8(parts[11])?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_mp3_frame(f: &Mp3Frame) -> String {
    format!("[{},{}]", enc_mp3_header(&f.header), hex_encode(&f.payload))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_mp3_frame(s: &str) -> Result<Mp3Frame, String> {
    let inner = strip_brackets(s)?;
    // 🧭️ The header is itself a bracketed 12-field group, so at depth-0 it is ONE token (same
    // nesting trick `decode_option` relies on) — split at depth 0 gives exactly
    // `["[12 header fields]", "<payload-hex>"]`, 2 top-level tokens, never 13.
    let parts = split_top_level(inner, ',');
    if parts.len() != 2 {
        return Err(format!("mp3 frame: expected header+payload=2 top-level fields, got {}", parts.len()));
    }
    let header = dec_mp3_header(parts[0])?;
    Ok(Mp3Frame { header, payload: hex_decode(parts[1])? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_mp3_frames(frames: &[Mp3Frame]) -> String {
    format!("[{}]", frames.iter().map(enc_mp3_frame).collect::<Vec<_>>().join(";"))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_mp3_frames(s: &str) -> Result<Vec<Mp3Frame>, String> {
    let inner = strip_brackets(s)?;
    split_top_level(inner, ';').into_iter().filter(|p| !p.is_empty()).map(dec_mp3_frame).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_mp3_diff(d: &Mp3Diff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = &d.id3v2 {
        tokens.push(format!("id3v2={}", encode_option(v, enc_id3v2)));
    }
    if let Some(v) = &d.frames {
        tokens.push(format!("frames={}", enc_mp3_frames(v)));
    }
    if let Some(v) = &d.id3v1 {
        tokens.push(format!("id3v1={}", encode_option(v, enc_id3v1)));
    }
    tokens.join(" ")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_mp3_diff(line: &str) -> Result<Mp3Diff, String> {
    let mut d = Mp3Diff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("id3v2=") {
            d.id3v2 = Some(decode_option(rest, dec_id3v2)?);
        } else if let Some(rest) = token.strip_prefix("frames=") {
            d.frames = Some(dec_mp3_frames(rest)?);
        } else if let Some(rest) = token.strip_prefix("id3v1=") {
            d.id3v1 = Some(decode_option(rest, dec_id3v1)?);
        } else {
            return Err(format!("mp3 diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl protocol::DiffText for Mp3Diff {
fn print_diff(&self) -> String {
    print_mp3_diff(self)
}
fn parse_diff(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
    parse_mp3_diff(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
}
}
}
pub use diff_codec::*;
