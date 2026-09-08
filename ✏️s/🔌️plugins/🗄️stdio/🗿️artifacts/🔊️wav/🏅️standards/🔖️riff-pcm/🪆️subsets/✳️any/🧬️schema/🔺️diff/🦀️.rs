//! 🔺️ WavDiff — sparse per-field RIFF/WAVE diff. `WavSnapshot` has exactly three top-level
//! fields (`fmt`, `data`, `other_chunks`), none independently nullable (unlike deflate's
//! `dict_id`), so every field here is a plain `Option<T>` "changed or not" slot — the same
//! "Scalars: LWW" shape `DeflateDiff` uses, adapted to wav's own value types.

use crate::standards::riff_pcm::subsets::any::schema::snapshot::{RiffChunk, WavData, WavFmt, WavSnapshot};
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct WavDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub fmt: Option<WavFmt>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<WavData>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub other_chunks: Option<Vec<RiffChunk>>,
}

impl MutationDiff<WavSnapshot> for WavDiff {
    fn apply(&self, base: &WavSnapshot) -> protocol::MutationApplyResult<WavSnapshot> {
        let mut next = base.clone();
        if let Some(v) = &self.fmt {
            next.fmt = v.clone();
        }
        if let Some(v) = &self.data {
            next.data = v.clone();
        }
        if let Some(v) = &self.other_chunks {
            next.other_chunks = v.clone();
        }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        if other.fmt.is_some() {
            self.fmt = other.fmt;
        }
        if other.data.is_some() {
            self.data = other.data;
        }
        if other.other_chunks.is_some() {
            self.other_chunks = other.other_chunks;
        }
    }
}

impl DiffAlgebra<WavSnapshot> for WavDiff {
    fn between(base: &WavSnapshot, other: &WavSnapshot) -> Self {
        WavDiff { fmt: (base.fmt != other.fmt).then(|| other.fmt.clone()), data: (base.data != other.data).then(|| other.data.clone()), other_chunks: (base.other_chunks != other.other_chunks).then(|| other.other_chunks.clone()) }
    }
    fn inverse(&self, base: &WavSnapshot) -> Self {
        WavDiff { fmt: self.fmt.as_ref().map(|_| base.fmt.clone()), data: self.data.as_ref().map(|_| base.data.clone()), other_chunks: self.other_chunks.as_ref().map(|_| base.other_chunks.clone()) }
    }
    fn is_empty(&self) -> bool {
        self.fmt.is_none() && self.data.is_none() && self.other_chunks.is_none()
    }
}

/// 🧩 Builds a set-snapshot diff: the sparse field-by-field delta, never a full-replace slot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_snapshot(base: &WavSnapshot, snapshot: &WavSnapshot) -> WavDiff {
    WavDiff::between(base, snapshot)
}
/// 🧩 Builds a set-fmt diff.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_fmt(fmt: WavFmt) -> WavDiff {
    WavDiff { fmt: Some(fmt), ..Default::default() }
}
/// 🧩 Builds a set-data diff.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_data(data: WavData) -> WavDiff {
    WavDiff { data: Some(data), ..Default::default() }
}
/// 🧩 Builds a set-other-chunks diff.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_other_chunks(chunks: Vec<RiffChunk>) -> WavDiff {
    WavDiff { other_chunks: Some(chunks), ..Default::default() }
}
//#endregion 🔖️Diff

//#region 🔖️HandcraftedDiffCodec
/// 🧪️ Hand-rolled `protocol::DiffCodec` (per ticket `26/08/11/…-RETIREMENT`'s mandate: no
/// `#[derive(dsl::DslDiff)]` here — `WavData` is a data-carrying enum, the same shape `f6-final-
/// summary.md` §4.4 documents as structurally unbindable by the derive machinery today; hand-
/// rolled following `DeflateDiff`'s own grammar template, `f6-recon-report.md` §5's primitive
/// set copied verbatim). Grammar: one space-separated `name=value` token per changed top-level
/// field (a field absent from the line = unchanged); `WavFmt`/`RiffChunk` values are their own
/// bracketed `[a,b,c,…]` tuple; `WavData` values are `tag:hex` (`p16`/`p8`/`f32`/`raw`); a chunk
/// list is `[chunk1;chunk2;…]`. Worked example:
/// `fmt=[1,1,8000,16000,2,16,[0]] data=p16:0100feff`.
//#region 🔖️Primitives
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if !s.len().is_multiple_of(2) {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
/// 🧭️ Bracket-depth-aware split (tracks `[`/`]` only) — the shared grammar contract every
/// hand-rolled codec in this repo uses (`f6-recon-report.md` §5), kept verbatim.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn split_top_level(s: &str, sep: char) -> Vec<&str> {
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
fn strip_brackets(s: &str) -> Result<&str, String> {
    s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_option<T>(opt: &Option<T>, enc: impl Fn(&T) -> String) -> String {
    match opt {
        None => "[0]".to_string(),
        Some(v) => format!("[1,{}]", enc(v)),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_option<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Option<T>, String> {
    let inner = strip_brackets(s)?;
    match split_top_level(inner, ',').as_slice() {
        ["0"] => Ok(None),
        [tag, value] if *tag == "1" => Ok(Some(dec(value)?)),
        other => Err(format!("option decode: bad shape {other:?}")),
    }
}
//#endregion 🔖️Primitives

//#region 🔖️ValueCodecs
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_wav_fmt(f: &WavFmt) -> String {
    format!("[{},{},{},{},{},{},{}]", f.audio_format, f.channels, f.sample_rate, f.byte_rate, f.block_align, f.bits_per_sample, encode_option(&f.ext, |v| hex_encode(v)))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_wav_fmt(s: &str) -> Result<WavFmt, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    if parts.len() != 7 {
        return Err(format!("wav fmt: expected 7 fields, got {}", parts.len()));
    }
    Ok(WavFmt {
        audio_format: parts[0].parse().map_err(|e: std::num::ParseIntError| e.to_string())?,
        channels: parts[1].parse().map_err(|e: std::num::ParseIntError| e.to_string())?,
        sample_rate: parts[2].parse().map_err(|e: std::num::ParseIntError| e.to_string())?,
        byte_rate: parts[3].parse().map_err(|e: std::num::ParseIntError| e.to_string())?,
        block_align: parts[4].parse().map_err(|e: std::num::ParseIntError| e.to_string())?,
        bits_per_sample: parts[5].parse().map_err(|e: std::num::ParseIntError| e.to_string())?,
        ext: decode_option(parts[6], hex_decode)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_wav_data(d: &WavData) -> String {
    match d {
        WavData::Pcm16(v) => format!("p16:{}", hex_encode(&v.iter().flat_map(|s| s.to_le_bytes()).collect::<Vec<u8>>())),
        WavData::Pcm8(v) => format!("p8:{}", hex_encode(v)),
        WavData::Float32(v) => format!("f32:{}", hex_encode(&v.iter().flat_map(|s| s.to_le_bytes()).collect::<Vec<u8>>())),
        WavData::Raw(v) => format!("raw:{}", hex_encode(v)),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_wav_data(s: &str) -> Result<WavData, String> {
    let (tag, rest) = s.split_once(':').ok_or_else(|| format!("wav data: missing tag in {s:?}"))?;
    let bytes = hex_decode(rest)?;
    match tag {
        "p16" => {
            if bytes.len() % 2 != 0 {
                return Err("wav data p16: odd byte length".into());
            }
            Ok(WavData::Pcm16(bytes.as_chunks::<2>().0.iter().map(|c| i16::from_le_bytes([c[0], c[1]])).collect()))
        }
        "p8" => Ok(WavData::Pcm8(bytes)),
        "f32" => {
            if bytes.len() % 4 != 0 {
                return Err("wav data f32: bad byte length".into());
            }
            Ok(WavData::Float32(bytes.as_chunks::<4>().0.iter().map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect()))
        }
        "raw" => Ok(WavData::Raw(bytes)),
        other => Err(format!("wav data: unknown tag {other:?}")),
    }
}

/// 🧭️ `RiffChunk.fourcc` is a 4-char printable RIFF tag (`fmt `/`data`/`LIST`/`INFO`/…) — never
/// contains `,`/`[`/`]`/`;` in practice, so it's safe as a bare top-level token.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_riff_chunk(c: &RiffChunk) -> String {
    format!("[{},{}]", c.fourcc, hex_encode(&c.data))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_riff_chunk(s: &str) -> Result<RiffChunk, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    if parts.len() != 2 {
        return Err(format!("riff chunk: expected 2 fields, got {}", parts.len()));
    }
    Ok(RiffChunk { fourcc: parts[0].to_string(), data: hex_decode(parts[1])? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_riff_chunks(chunks: &[RiffChunk]) -> String {
    format!("[{}]", chunks.iter().map(enc_riff_chunk).collect::<Vec<_>>().join(";"))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_riff_chunks(s: &str) -> Result<Vec<RiffChunk>, String> {
    let inner = strip_brackets(s)?;
    split_top_level(inner, ';').into_iter().filter(|p| !p.is_empty()).map(dec_riff_chunk).collect()
}
//#endregion 🔖️ValueCodecs

//#region 🔖️TopLevel
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_wav_diff(d: &WavDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = &d.fmt {
        tokens.push(format!("fmt={}", enc_wav_fmt(v)));
    }
    if let Some(v) = &d.data {
        tokens.push(format!("data={}", enc_wav_data(v)));
    }
    if let Some(v) = &d.other_chunks {
        tokens.push(format!("other-chunks={}", enc_riff_chunks(v)));
    }
    tokens.join(" ")
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_wav_diff(line: &str) -> Result<WavDiff, String> {
    let mut d = WavDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("fmt=") {
            d.fmt = Some(dec_wav_fmt(rest)?);
        } else if let Some(rest) = token.strip_prefix("data=") {
            d.data = Some(dec_wav_data(rest)?);
        } else if let Some(rest) = token.strip_prefix("other-chunks=") {
            d.other_chunks = Some(dec_riff_chunks(rest)?);
        } else {
            return Err(format!("wav diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl protocol::DiffCodec for WavDiff {
    fn print_diff(&self) -> String {
        print_wav_diff(self)
    }
    fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        parse_wav_diff(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    /// ⚡️ Binary = the text bytes verbatim (same simplification `DeflateDiff`/`GifDiff`'s
    /// hand-rolled `DiffCodec` impls use).
    fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(self.print_diff().into_bytes())
    }
    fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let line = std::str::from_utf8(bytes).map_err(|e| protocol::ProtocolError::Malformed { what: "diff utf8", offset: 0, detail: e.to_string() })?;
        Self::parse_diff(line).map_err(|e| protocol::ProtocolError::Malformed { what: "diff text", offset: 0, detail: e.to_string() })
    }
}
//#endregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
