//! 📝️ Text representation codec surface for `stdio.deflate` (diff).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type DeflateDiffText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v_rfc1950::subsets::any::schema::diff::*;
use protocol::{DiffText,DiffBinary};
use crate::schema::snapshot::DeflateLevelHint;
use crate::DeflateSnapshot;
use protocol::MutationDiff;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;

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
pub(crate) fn parse_u8(s: &str) -> Result<u8, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_u32(s: &str) -> Result<u32, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}

/// 🧭️ Bracket-depth-aware split (tracks `[`/`]` only) — needed even for this small a grammar
/// because `decode_option`'s own `[0]`/`[1,<v>]` payload can itself contain a `,` (none here
/// today, but the primitive is the shared grammar contract every hand-rolled codec in this repo
/// uses, per `f6-recon-report.md` §5 -- kept verbatim rather than hand-simplified).
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
pub(crate) fn enc_level_hint(h: DeflateLevelHint) -> char {
    match h {
        DeflateLevelHint::Fastest => 'f',
        DeflateLevelHint::Fast => 'a',
        DeflateLevelHint::Default => 'd',
        DeflateLevelHint::Maximum => 'm',
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_level_hint(s: &str) -> Result<DeflateLevelHint, String> {
    match s {
        "f" => Ok(DeflateLevelHint::Fastest),
        "a" => Ok(DeflateLevelHint::Fast),
        "d" => Ok(DeflateLevelHint::Default),
        "m" => Ok(DeflateLevelHint::Maximum),
        other => Err(format!("bad level hint {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_deflate_diff(d: &DeflateDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = d.compression_method {
        tokens.push(format!("compression-method={v}"));
    }
    if let Some(v) = d.window_bits {
        tokens.push(format!("window-bits={v}"));
    }
    if let Some(v) = d.compression_level_hint {
        tokens.push(format!("level={}", enc_level_hint(v)));
    }
    if let Some(v) = &d.dict_id {
        tokens.push(format!("dict-id={}", encode_option(v, |x| x.to_string())));
    }
    if let Some(v) = &d.payload {
        tokens.push(format!("payload={}", hex_encode(v)));
    }
    tokens.join(" ")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_deflate_diff(line: &str) -> Result<DeflateDiff, String> {
    let mut d = DeflateDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("compression-method=") {
            d.compression_method = Some(parse_u8(rest)?);
        } else if let Some(rest) = token.strip_prefix("window-bits=") {
            d.window_bits = Some(parse_u8(rest)?);
        } else if let Some(rest) = token.strip_prefix("level=") {
            d.compression_level_hint = Some(dec_level_hint(rest)?);
        } else if let Some(rest) = token.strip_prefix("dict-id=") {
            d.dict_id = Some(decode_option(rest, parse_u32)?);
        } else if let Some(rest) = token.strip_prefix("payload=") {
            d.payload = Some(hex_decode(rest)?);
        } else {
            return Err(format!("deflate diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl protocol::DiffText for DeflateDiff {
fn print_diff(&self) -> String {
    print_deflate_diff(self)
}
fn parse_diff(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
    parse_deflate_diff(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
}
}
}
pub use diff_codec::*;
