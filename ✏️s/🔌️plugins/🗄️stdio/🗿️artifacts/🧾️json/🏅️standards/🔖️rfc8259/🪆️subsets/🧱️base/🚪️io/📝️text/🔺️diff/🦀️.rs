//! 📝️ Text representation codec surface for `stdio.json` (diff).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type JsonDiffText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v_rfc8259::subsets::base::schema::diff::*;
use crate::schema::snapshot::JsonMember;
use crate::JsonSnapshot;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use framework_schema::ArtifactSchema;
use protocol::os_spr::command::DiffAlgebra;
use std::collections::{HashMap, HashSet};
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use crate::schema::snapshot::JsonValue;


































}
pub use diff_codec::*;

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v_rfc8259::subsets::base::schema::diff::*;
use protocol::{DiffText,DiffBinary};
use crate::schema::snapshot::JsonMember;
use crate::JsonSnapshot;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use framework_schema::ArtifactSchema;
use protocol::os_spr::command::DiffAlgebra;
use std::collections::{HashMap, HashSet};
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use crate::schema::snapshot::JsonValue;

/// 🧭️ Single-field top level (`value=<enc>`, absent = unchanged) — `JsonDiff` has exactly one
/// diffable field (`schema` is identity-only, never diffed), so there is only ever zero or one
/// space-separated token, unlike `SvgDiff`'s multi-field line.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_json_diff(d: &JsonDiff) -> String {
    match &d.value {
        Some(v) => format!("value={}", enc_value_diff(v)),
        None => String::new(),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_json_diff(line: &str) -> Result<JsonDiff, String> {
    let mut d = JsonDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("value=") {
            d.value = Some(dec_value_diff(rest)?);
        } else {
            return Err(format!("json diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

/// 🌳 `JsonValueDiff` itself needs a tag (`R`=Replace, `B`=Bool, `N`=Number, `S`=String, `A`=Array,
/// `O`=Object) since, unlike a plain [`JsonValue`], it appears standalone (not always inside a
/// bracketed container) at the top-level `value=` token position.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_value_diff(d: &JsonValueDiff) -> String {
    match d {
        JsonValueDiff::Replace { value } => format!("R[{}]", enc_json_value(value)),
        JsonValueDiff::Bool { value } => format!("B[{}]", if *value { "1" } else { "0" }),
        JsonValueDiff::Number { lexeme } => format!("N[{}]", enc_str(lexeme)),
        JsonValueDiff::String { value } => format!("S[{}]", enc_str(value)),
        JsonValueDiff::Array { diff } => format!("A[{}]", enc_array_diff(diff)),
        JsonValueDiff::Object { diff } => format!("O[{}]", enc_object_diff(diff)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_value_diff(s: &str) -> Result<JsonValueDiff, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "R" => Ok(JsonValueDiff::Replace { value: dec_json_value(inner)? }),
        "B" => Ok(JsonValueDiff::Bool { value: inner == "1" }),
        "N" => Ok(JsonValueDiff::Number { lexeme: dec_str(inner)? }),
        "S" => Ok(JsonValueDiff::String { value: dec_str(inner)? }),
        "A" => Ok(JsonValueDiff::Array { diff: dec_array_diff(inner)? }),
        "O" => Ok(JsonValueDiff::Object { diff: dec_object_diff(inner)? }),
        other => Err(format!("json value diff: unknown tag {other:?}")),
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
pub(crate) fn strip_brackets(s: &str) -> Result<&str, String> {
    s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))
}

/// 🌳 Tag-prefixed like `SvgDiff`'s `enc_xml_node`: `Z` (null, no payload, no brackets) / `B[0|1]`
/// / `N[hex(lexeme)]` / `S[hex(value)]` / `A[v1,v2,...]` / `O[hexkey1:v1,hexkey2:v2,...]` — member
/// insertion order preserved by construction (a list, never re-sorted).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_json_value(v: &JsonValue) -> String {
    match v {
        JsonValue::Null => "Z".to_string(),
        JsonValue::Bool { value } => format!("B[{}]", if *value { "1" } else { "0" }),
        JsonValue::Number { lexeme } => format!("N[{}]", enc_str(lexeme)),
        JsonValue::String { value } => format!("S[{}]", enc_str(value)),
        JsonValue::Array { items } => format!("A[{}]", items.iter().map(enc_json_value).collect::<Vec<_>>().join(",")),
        JsonValue::Object { members } => format!("O[{}]", members.iter().map(|m| format!("{}:{}", enc_str(&m.key), enc_json_value(&m.value))).collect::<Vec<_>>().join(",")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_json_value(s: &str) -> Result<JsonValue, String> {
    if s == "Z" {
        return Ok(JsonValue::Null);
    }
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "B" => Ok(JsonValue::Bool { value: inner == "1" }),
        "N" => Ok(JsonValue::Number { lexeme: dec_str(inner)? }),
        "S" => Ok(JsonValue::String { value: dec_str(inner)? }),
        "A" => Ok(JsonValue::Array { items: split_top_level(inner, ',').into_iter().filter(|s| !s.is_empty()).map(dec_json_value).collect::<Result<Vec<_>, String>>()? }),
        "O" => {
            let members = split_top_level(inner, ',')
                .into_iter()
                .filter(|s| !s.is_empty())
                .map(|entry| {
                    let (key, value) = entry.split_once(':').ok_or_else(|| format!("object member: bad entry {entry:?}"))?;
                    Ok(JsonMember { key: dec_str(key)?, value: dec_json_value(value)? })
                })
                .collect::<Result<Vec<_>, String>>()?;
            Ok(JsonValue::Object { members })
        }
        other => Err(format!("json value: unknown tag {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_array_diff(d: &JsonArrayDiff) -> String {
    let removed = d.removed.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:{}", m.index, enc_value_diff(&m.diff))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}", a.index, enc_json_value(&a.item))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_array_diff(body: &str) -> Result<JsonArrayDiff, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("array diff: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_usize).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("array modified: bad entry {entry:?}"))?;
            Ok(JsonArrayModified { index: parse_usize(idx)?, diff: dec_value_diff(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("array added: bad entry {entry:?}"))?;
            Ok(JsonArrayAdded { index: parse_usize(idx)?, item: dec_json_value(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(JsonArrayDiff { removed, modified, added })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_object_diff(d: &JsonObjectDiff) -> String {
    let removed = d.removed.iter().map(|k| enc_str(k)).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:{}", enc_str(&m.key), enc_value_diff(&m.diff))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}:{}", a.index, enc_str(&a.key), enc_json_value(&a.item))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_object_diff(body: &str) -> Result<JsonObjectDiff, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("object diff: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_str).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (key, rest) = entry.split_once(':').ok_or_else(|| format!("object modified: bad entry {entry:?}"))?;
            Ok(JsonObjectModified { key: dec_str(key)?, diff: dec_value_diff(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("object added: bad entry {entry:?}"))?;
            let (key, item) = rest.split_once(':').ok_or_else(|| format!("object added: bad entry {entry:?}"))?;
            Ok(JsonObjectAdded { index: parse_usize(idx)?, key: dec_str(key)?, item: dec_json_value(item)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(JsonObjectDiff { removed, modified, added })
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
pub(crate) fn parse_usize(s: &str) -> Result<usize, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}

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

impl protocol::DiffText for JsonDiff {
fn print_diff(&self) -> String {
    print_json_diff(self)
}
fn parse_diff(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
    parse_json_diff(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
}
}
}
pub use diff_codec::*;
