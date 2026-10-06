//! 📝️ Text representation codec surface for `stdio.csv` (diff).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type CsvDiffText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v_rfc4180::subsets::any::schema::diff::*;
use crate::schema::snapshot::{CsvField, CsvRecord, CsvSnapshot};
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{DiffBinary,DiffCodec,DiffText};
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use std::collections::{BTreeMap, HashMap};

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    fn nibble(value: u8) -> Option<u8> {
        match value {
            b'0'..=b'9' => Some(value - b'0'),
            b'a'..=b'f' => Some(value - b'a' + 10),
            b'A'..=b'F' => Some(value - b'A' + 10),
            _ => None,
        }
    }
    let mut pairs = s.as_bytes().chunks_exact(2);
    let decoded = pairs
        .by_ref()
        .map(|pair| Ok((nibble(pair[0]).ok_or_else(|| "invalid hexadecimal".to_string())? << 4) | nibble(pair[1]).ok_or_else(|| "invalid hexadecimal".to_string())?))
        .collect::<Result<Vec<_>, String>>()?;
    if pairs.remainder().is_empty() { Ok(decoded) } else { Err(format!("odd hex length: {s:?}")) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_usize(s: &str) -> Result<usize, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}

/// 🧭️ Bracket-depth-aware split (tracks `[`/`]` only): a top-level `sep` inside nested brackets is
/// never mistaken for a field separator — the whole hand-rolled grammar's parsing primitive.
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
pub(crate) fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_field(f: &CsvField) -> String {
    format!("[{},{}]", enc_str(&f.value), if f.quoted { 1 } else { 0 })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_field(s: &str) -> Result<CsvField, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [value, quoted] = parts.as_slice() else { return Err(format!("field: expected 2 fields, got {}", parts.len())) };
    Ok(CsvField { value: dec_str(value)?, quoted: *quoted == "1" })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_record(r: &CsvRecord) -> String {
    format!("[{}]", r.fields.iter().map(enc_field).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_record(s: &str) -> Result<CsvRecord, String> {
    let fields = split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_field).collect::<Result<Vec<_>, String>>()?;
    Ok(CsvRecord { fields })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_field_diff(d: &CsvFieldDiff) -> String {
    let mut parts = Vec::new();
    if let Some(v) = &d.value {
        parts.push(format!("V:{}", enc_str(v)));
    }
    if let Some(v) = d.quoted {
        parts.push(format!("Q:{}", if v { 1 } else { 0 }));
    }
    format!("[{}]", parts.join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_field_diff(s: &str) -> Result<CsvFieldDiff, String> {
    let inner = strip_brackets(s)?;
    let mut d = CsvFieldDiff::default();
    for entry in split_top_level(inner, ',') {
        if entry.is_empty() {
            continue;
        }
        let (tag, val) = entry.split_once(':').ok_or_else(|| format!("field diff: bad entry {entry:?}"))?;
        match tag {
            "V" => d.value = Some(dec_str(val)?),
            "Q" => d.quoted = Some(val == "1"),
            other => return Err(format!("field diff: unknown tag {other:?}")),
        }
    }
    Ok(d)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_record_diff(d: &CsvRecordDiff) -> String {
    encode_option(&d.fields, |fields| format!("[{}]", fields.iter().map(|f| encode_option(f, enc_field_diff)).collect::<Vec<_>>().join(",")))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_record_diff(s: &str) -> Result<CsvRecordDiff, String> {
    let fields = decode_option(s, |inner| split_top_level(strip_brackets(inner)?, ',').into_iter().filter(|s| !s.is_empty()).map(|p| decode_option(p, dec_field_diff)).collect::<Result<Vec<_>, String>>())?;
    Ok(CsvRecordDiff { fields })
}

/// 🧭️ Generic-shaped 3-section `[removed];[modified];[added]` collection-triple printer/parser
/// (mirrors gif89a's `enc_collection_triple`/`dec_collection_triple`, hand-instantiated here for
/// `records` since only one collection needs it in this artifact).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_records_diff(d: &CsvRecordsDiff) -> String {
    let removed = d.removed.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:{}", m.index, enc_record_diff(&m.diff))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}", a.index, enc_record(&a.record))).collect::<Vec<_>>().join(",");
    format!("records{{[{removed}];[{modified}];[{added}]}}")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_records_diff(body: &str) -> Result<CsvRecordsDiff, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("records: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_usize).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("records modified: bad entry {entry:?}"))?;
            Ok(CsvRecordModified { index: parse_usize(idx)?, diff: dec_record_diff(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("records added: bad entry {entry:?}"))?;
            Ok(CsvRecordAdded { index: parse_usize(idx)?, record: dec_record(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(CsvRecordsDiff { removed, modified, added })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_csv_diff(d: &CsvDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = d.has_header {
        tokens.push(format!("has-header={}", if v { 1 } else { 0 }));
    }
    if let Some(v) = &d.records {
        tokens.push(enc_records_diff(v));
    }
    tokens.join(" ")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_csv_diff(line: &str) -> Result<CsvDiff, String> {
    let mut d = CsvDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("has-header=") {
            d.has_header = Some(rest == "1");
        } else if let Some(rest) = token.strip_prefix("records{") {
            d.records = Some(dec_records_diff(rest.strip_suffix('}').ok_or_else(|| "records: missing closing brace".to_string())?)?);
        } else {
            return Err(format!("csv diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl protocol::DiffText for CsvDiff {
fn print_diff(&self) -> String {
    print_csv_diff(self)
}
fn parse_diff(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
    parse_csv_diff(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
}
}

}
pub use diff_codec::*;
