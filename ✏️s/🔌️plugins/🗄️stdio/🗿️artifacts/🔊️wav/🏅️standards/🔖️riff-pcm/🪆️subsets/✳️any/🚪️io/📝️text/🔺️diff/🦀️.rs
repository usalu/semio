//! 🚧 scaffolded by W1b — text representation marker for `stdio.wav.diff`. Full grammar-backed
//! parse/print lands in W2/W3.
pub const TEXT_MARKER: &str = "stdio.wav.diff";

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::riff_pcm::subsets::any::schema::diff::*;
use protocol::{DiffText,DiffBinary};
use crate::standards::riff_pcm::subsets::any::schema::snapshot::{RiffChunk, WavChunkRef, WavData, WavFmt, WavSnapshot};
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
pub(crate) fn enc_wav_fmt(f: &WavFmt) -> String {
    format!("[{},{},{},{},{},{},{}]", f.audio_format, f.channels, f.sample_rate, f.byte_rate, f.block_align, f.bits_per_sample, encode_option(&f.ext, |v| hex_encode(v)))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_wav_fmt(s: &str) -> Result<WavFmt, String> {
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
pub(crate) fn enc_wav_data(d: &WavData) -> String {
    match d {
        WavData::Pcm16(v) => format!("p16:{}", hex_encode(&v.iter().flat_map(|s| s.to_le_bytes()).collect::<Vec<u8>>())),
        WavData::Pcm8(v) => format!("p8:{}", hex_encode(v)),
        WavData::Float32(v) => format!("f32:{}", hex_encode(&v.iter().flat_map(|s| s.to_le_bytes()).collect::<Vec<u8>>())),
        WavData::Raw(v) => format!("raw:{}", hex_encode(v)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_wav_data(s: &str) -> Result<WavData, String> {
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
pub(crate) fn enc_riff_chunk(c: &RiffChunk) -> String {
    format!("[{},{},{}]", c.fourcc, hex_encode(&c.data), c.pad_byte)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_riff_chunk(s: &str) -> Result<RiffChunk, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    if parts.len() != 3 {
        return Err(format!("riff chunk: expected 3 fields, got {}", parts.len()));
    }
    Ok(RiffChunk { fourcc: parts[0].to_string(), data: hex_decode(parts[1])?, pad_byte: parts[2].parse::<u8>().map_err(|error| error.to_string())? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_riff_chunks(chunks: &[RiffChunk]) -> String {
    format!("[{}]", chunks.iter().map(enc_riff_chunk).collect::<Vec<_>>().join(";"))
}

pub(crate) fn enc_chunk_order(order: &[WavChunkRef]) -> String {
    format!("[{}]", order.iter().map(|reference| match reference { WavChunkRef::Format => "f".to_string(), WavChunkRef::Samples => "d".to_string(), WavChunkRef::Other(index) => format!("o{index}") }).collect::<Vec<_>>().join(";"))
}

pub(crate) fn dec_chunk_order(s: &str) -> Result<Vec<WavChunkRef>, String> {
    split_top_level(strip_brackets(s)?, ';').into_iter().filter(|part| !part.is_empty()).map(|part| match part {
        "f" => Ok(WavChunkRef::Format),
        "d" => Ok(WavChunkRef::Samples),
        other => other.strip_prefix('o').ok_or_else(|| format!("chunk order: unknown reference {other:?}")).and_then(|index| index.parse::<u64>().map(WavChunkRef::Other).map_err(|error| error.to_string())),
    }).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_riff_chunks(s: &str) -> Result<Vec<RiffChunk>, String> {
    let inner = strip_brackets(s)?;
    split_top_level(inner, ';').into_iter().filter(|p| !p.is_empty()).map(dec_riff_chunk).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_wav_splice(s: &str) -> Result<WavSplice, String> {
    let mut parts = s.splitn(3, ',');
    let (Some(index), Some(remove), Some(insert)) = (parts.next(), parts.next(), parts.next()) else { return Err(format!("wav diff: malformed splice {s:?}")) };
    Ok(WavSplice { index: index.parse::<u64>().map_err(|error| error.to_string())?, remove: remove.parse::<u64>().map_err(|error| error.to_string())?, insert: dec_wav_data(insert)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_wav_diff(d: &WavDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = &d.fmt {
        tokens.push(format!("fmt={}", enc_wav_fmt(v)));
    }
    if let Some(v) = &d.data {
        tokens.push(format!("data={}", enc_wav_data(v)));
    }
    if !d.data_splices.is_empty() {
        tokens.push(format!("data-splices={}", d.data_splices.iter().map(|splice| format!("{},{},{}", splice.index, splice.remove, enc_wav_data(&splice.insert))).collect::<Vec<_>>().join(";")));
    }
    if let Some(v) = d.fmt_pad_byte {
        tokens.push(format!("fmt-pad-byte={v}"));
    }
    if let Some(v) = d.data_pad_byte {
        tokens.push(format!("data-pad-byte={v}"));
    }
    if let Some(v) = &d.other_chunks {
        tokens.push(format!("other-chunks={}", enc_riff_chunks(v)));
    }
    if let Some(v) = &d.chunk_order {
        tokens.push(format!("chunk-order={}", enc_chunk_order(v)));
    }
    tokens.join(" ")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_wav_diff(line: &str) -> Result<WavDiff, String> {
    let mut d = WavDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("fmt=") {
            d.fmt = Some(dec_wav_fmt(rest)?);
        } else if let Some(rest) = token.strip_prefix("data=") {
            d.data = Some(dec_wav_data(rest)?);
        } else if let Some(rest) = token.strip_prefix("data-splices=") {
            d.data_splices = rest.split(';').map(dec_wav_splice).collect::<Result<_, _>>()?;
        } else if let Some(rest) = token.strip_prefix("fmt-pad-byte=") {
            d.fmt_pad_byte = Some(rest.parse::<u8>().map_err(|error| error.to_string())?);
        } else if let Some(rest) = token.strip_prefix("data-pad-byte=") {
            d.data_pad_byte = Some(rest.parse::<u8>().map_err(|error| error.to_string())?);
        } else if let Some(rest) = token.strip_prefix("other-chunks=") {
            d.other_chunks = Some(dec_riff_chunks(rest)?);
        } else if let Some(rest) = token.strip_prefix("chunk-order=") {
            d.chunk_order = Some(dec_chunk_order(rest)?);
        } else {
            return Err(format!("wav diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl protocol::DiffText for WavDiff {
fn print_diff(&self) -> String {
    print_wav_diff(self)
}
fn parse_diff(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
    parse_wav_diff(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
}
}
}
pub use diff_codec::*;
