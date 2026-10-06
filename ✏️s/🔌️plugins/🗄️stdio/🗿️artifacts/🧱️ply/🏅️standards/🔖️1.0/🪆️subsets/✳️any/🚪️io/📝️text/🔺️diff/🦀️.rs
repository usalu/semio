//! 📝️ Text representation codec surface for `stdio.ply` (diff).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type PlyDiffText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v1_0::subsets::any::schema::diff::*;
use crate::schema::snapshot::{PlyElement, PlyFormat, PlyProperty, PlyRow, PlyScalarType};
use crate::PlySnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{DiffBinary,DiffCodec,DiffText};
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use std::collections::{BTreeMap, BTreeSet, HashSet};
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use crate::schema::snapshot::PlyValue;

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
pub(crate) fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_format(f: PlyFormat) -> char {
    match f {
        PlyFormat::Ascii => 'a',
        PlyFormat::BinaryLittleEndian => 'l',
        PlyFormat::BinaryBigEndian => 'b',
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_format(s: &str) -> Result<PlyFormat, String> {
    match s {
        "a" => Ok(PlyFormat::Ascii),
        "l" => Ok(PlyFormat::BinaryLittleEndian),
        "b" => Ok(PlyFormat::BinaryBigEndian),
        other => Err(format!("bad ply format {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_scalar_type(k: PlyScalarType) -> char {
    match k {
        PlyScalarType::Char => 'c',
        PlyScalarType::UChar => 'C',
        PlyScalarType::Short => 's',
        PlyScalarType::UShort => 'w',
        PlyScalarType::Int => 'i',
        PlyScalarType::UInt => 'u',
        PlyScalarType::Float => 'f',
        PlyScalarType::Double => 'd',
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_scalar_type(s: &str) -> Result<PlyScalarType, String> {
    match s {
        "c" => Ok(PlyScalarType::Char),
        "C" => Ok(PlyScalarType::UChar),
        "s" => Ok(PlyScalarType::Short),
        "w" => Ok(PlyScalarType::UShort),
        "i" => Ok(PlyScalarType::Int),
        "u" => Ok(PlyScalarType::UInt),
        "f" => Ok(PlyScalarType::Float),
        "d" => Ok(PlyScalarType::Double),
        other => Err(format!("bad ply scalar type {other:?}")),
    }
}

/// 🔣️ `PlyProperty` is a data-carrying enum (the module doc comment's cited 3a blocker) —
/// tag-prefixed like svg's `enc_xml_node`: `S[name,kind]` (Scalar) / `L[name,count_kind,value_kind]`
/// (List).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_property(p: &PlyProperty) -> String {
    match p {
        PlyProperty::Scalar { name, kind } => format!("S[{},{}]", enc_str(name), enc_scalar_type(*kind)),
        PlyProperty::List { name, count_kind, value_kind } => {
            format!("L[{},{},{}]", enc_str(name), enc_scalar_type(*count_kind), enc_scalar_type(*value_kind))
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_property(s: &str) -> Result<PlyProperty, String> {
    if s.len() < 2 {
        return Err(format!("property: too short {s:?}"));
    }
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    let parts = split_top_level(inner, ',');
    match tag {
        "S" => {
            let [name, kind] = parts.as_slice() else { return Err(format!("scalar property: expected 2 fields, got {}", parts.len())) };
            Ok(PlyProperty::Scalar { name: dec_str(name)?, kind: dec_scalar_type(kind)? })
        }
        "L" => {
            let [name, count_kind, value_kind] = parts.as_slice() else { return Err(format!("list property: expected 3 fields, got {}", parts.len())) };
            Ok(PlyProperty::List { name: dec_str(name)?, count_kind: dec_scalar_type(count_kind)?, value_kind: dec_scalar_type(value_kind)? })
        }
        other => Err(format!("property: unknown tag {other:?}")),
    }
}

/// 🔣️ `PlyValue` is the OTHER data-carrying enum reachable from the diff (`PlyRowFieldChange::value`)
/// — same tag-prefix convention, one lowercase letter per scalar kind (matching `enc_scalar_type`'s
/// own letters) plus `L[...]` for the recursive `List(Vec<PlyValue>)` variant.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_value(v: &PlyValue) -> String {
    match v {
        PlyValue::Char(x) => format!("c[{x}]"),
        PlyValue::UChar(x) => format!("C[{x}]"),
        PlyValue::Short(x) => format!("s[{x}]"),
        PlyValue::UShort(x) => format!("w[{x}]"),
        PlyValue::Int(x) => format!("i[{x}]"),
        PlyValue::UInt(x) => format!("u[{x}]"),
        PlyValue::Float(x) => format!("f[{x}]"),
        PlyValue::Double(x) => format!("d[{x}]"),
        PlyValue::List(items) => format!("L[{}]", items.iter().map(enc_value).collect::<Vec<_>>().join(",")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_value(s: &str) -> Result<PlyValue, String> {
    if s.len() < 2 {
        return Err(format!("value: too short {s:?}"));
    }
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn parse_i<T: std::str::FromStr<Err = std::num::ParseIntError>>(s: &str) -> Result<T, String> {
        s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
    }
    let parse_f32 = |s: &str| s.parse::<f32>().map_err(|e: std::num::ParseFloatError| e.to_string());
    let parse_f64 = |s: &str| s.parse::<f64>().map_err(|e: std::num::ParseFloatError| e.to_string());
    match tag {
        "c" => Ok(PlyValue::Char(parse_i(inner)?)),
        "C" => Ok(PlyValue::UChar(parse_i(inner)?)),
        "s" => Ok(PlyValue::Short(parse_i(inner)?)),
        "w" => Ok(PlyValue::UShort(parse_i(inner)?)),
        "i" => Ok(PlyValue::Int(parse_i(inner)?)),
        "u" => Ok(PlyValue::UInt(parse_i(inner)?)),
        "f" => Ok(PlyValue::Float(parse_f32(inner)?)),
        "d" => Ok(PlyValue::Double(parse_f64(inner)?)),
        "L" => Ok(PlyValue::List(split_top_level(inner, ',').into_iter().filter(|s| !s.is_empty()).map(dec_value).collect::<Result<Vec<_>, String>>()?)),
        other => Err(format!("value: unknown tag {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_row(r: &PlyRow) -> String {
    format!("[{}]", r.values.iter().map(enc_value).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_row(s: &str) -> Result<PlyRow, String> {
    let inner = strip_brackets(s)?;
    let values = split_top_level(inner, ',').into_iter().filter(|s| !s.is_empty()).map(dec_value).collect::<Result<Vec<_>, String>>()?;
    Ok(PlyRow { values })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_element(e: &PlyElement) -> String {
    format!("[{},{},[{}],[{}]]", enc_str(&e.name), e.count, e.properties.iter().map(enc_property).collect::<Vec<_>>().join(","), e.rows.iter().map(enc_row).collect::<Vec<_>>().join(","),)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_element(s: &str) -> Result<PlyElement, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [name, count, properties, rows] = parts.as_slice() else { return Err(format!("element: expected 4 fields, got {}", parts.len())) };
    Ok(PlyElement {
        name: dec_str(name)?,
        count: count.trim().parse::<u64>().map_err(|error|error.to_string())?,
        properties: split_top_level(strip_brackets(properties)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_property).collect::<Result<Vec<_>, String>>()?,
        rows: split_top_level(strip_brackets(rows)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_row).collect::<Result<Vec<_>, String>>()?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_row_field_change(c: &PlyRowFieldChange) -> String {
    format!("[{},{}]", enc_str(&c.name), enc_value(&c.value))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_row_field_change(s: &str) -> Result<PlyRowFieldChange, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, value] = parts.as_slice() else { return Err(format!("row field change: expected 2 fields, got {}", parts.len())) };
    Ok(PlyRowFieldChange { name: dec_str(name)?, value: dec_value(value)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_row_diff(d: &PlyRowDiff) -> String {
    format!("[{}]", d.fields.iter().map(enc_row_field_change).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_row_diff(s: &str) -> Result<PlyRowDiff, String> {
    let inner = strip_brackets(s)?;
    let fields = split_top_level(inner, ',').into_iter().filter(|s| !s.is_empty()).map(dec_row_field_change).collect::<Result<Vec<_>, String>>()?;
    Ok(PlyRowDiff { fields })
}

/// 🧭️ Generic `{[removed];[modified];[added]}` INDEX-keyed collection-triple parser (mirrors
/// gif89a's `dec_collection_triple`, without the `name{` prefix — ply's tokens are all uniform
/// `key=value`, so the key already carries the name). Used for `rows` (index-keyed on both
/// `removed` and `modified`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_index_triple_body(body: &str) -> Result<IndexedDiffParts<String, String>, String> {
    let inner = body.strip_prefix('{').and_then(|s| s.strip_suffix('}')).ok_or_else(|| format!("triple: expected {{...}}, got {body:?}"))?;
    let three = split_top_level(inner, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("triple: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_usize).collect::<Result<Vec<_>, String>>()?;
    let parse_entries = |s: &str| -> Result<Vec<(usize, String)>, String> {
        split_top_level(strip_brackets(s)?, ',')
            .into_iter()
            .filter(|s| !s.is_empty())
            .map(|entry| {
                let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("triple entry: bad entry {entry:?}"))?;
                Ok((parse_usize(idx)?, rest.to_string()))
            })
            .collect()
    };
    Ok((removed, parse_entries(modified_s)?, parse_entries(added_s)?))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_rows_diff(d: &PlyRowsDiff) -> String {
    let removed = d.removed.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:{}", m.index, enc_row_diff(&m.diff))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}", a.index, enc_row(&a.row))).collect::<Vec<_>>().join(",");
    format!("{{[{removed}];[{modified}];[{added}]}}")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_rows_diff(body: &str) -> Result<PlyRowsDiff, String> {
    let (removed, modified, added) = dec_index_triple_body(body)?;
    Ok(PlyRowsDiff {
        removed,
        modified: modified.into_iter().map(|(index, enc)| Ok(PlyRowModified { index, diff: dec_row_diff(&enc)? })).collect::<Result<Vec<_>, String>>()?,
        added: added.into_iter().map(|(index, enc)| Ok(PlyRowAdded { index, row: dec_row(&enc)? })).collect::<Result<Vec<_>, String>>()?,
    })
}

/// 🔺️ `PlyElementDiff`'s own sparse fields print as single-letter `tag:value` pairs (`P`/`R`)
/// inside its own `[...]` — same shape as gif89a's `enc_frame_diff`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_element_diff(d: &PlyElementDiff) -> String {
    let mut parts = Vec::new();
    if let Some(count)=d.count { parts.push(format!("C:{count}")); }
    if let Some(props) = &d.properties {
        parts.push(format!("P:[{}]", props.iter().map(enc_property).collect::<Vec<_>>().join(",")));
    }
    if let Some(rows) = &d.rows {
        parts.push(format!("R:{}", enc_rows_diff(rows)));
    }
    format!("[{}]", parts.join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_element_diff(s: &str) -> Result<PlyElementDiff, String> {
    let inner = strip_brackets(s)?;
    let mut d = PlyElementDiff::default();
    for entry in split_top_level(inner, ',') {
        if entry.is_empty() {
            continue;
        }
        let (tag, val) = entry.split_once(':').ok_or_else(|| format!("element diff: bad entry {entry:?}"))?;
        match tag {
            "C" => {d.count=Some(val.parse::<u64>().map_err(|error|error.to_string())?);}
            "P" => {
                let props_inner = strip_brackets(val)?;
                d.properties = Some(split_top_level(props_inner, ',').into_iter().filter(|s| !s.is_empty()).map(dec_property).collect::<Result<Vec<_>, String>>()?);
            }
            "R" => {
                d.rows = Some(dec_rows_diff(val)?);
            }
            other => return Err(format!("element diff: unknown tag {other:?}")),
        }
    }
    Ok(d)
}

/// 🔺️ `PlyElementsDiff` — NAME-keyed `removed`/`modified` (identity is `PlyElement::name`, no
/// `RenameElement` mutation) but INDEX-keyed `added` (matches `PlyElementAdded::index`'s own real
/// shape) — deliberately NOT the same uniform-index-keyed triple gif89a's frames use, see the
/// region doc comment.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_elements_diff(d: &PlyElementsDiff) -> String {
    let removed = d.removed.iter().map(|n| enc_str(n)).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:{}", enc_str(&m.name), enc_element_diff(&m.diff))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}", a.index, enc_element(&a.element))).collect::<Vec<_>>().join(",");
    format!("{{[{removed}];[{modified}];[{added}]}}")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_elements_diff(body: &str) -> Result<PlyElementsDiff, String> {
    let inner = body.strip_prefix('{').and_then(|s| s.strip_suffix('}')).ok_or_else(|| format!("elements triple: expected {{...}}, got {body:?}"))?;
    let three = split_top_level(inner, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("elements triple: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_str).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (name_hex, rest) = entry.split_once(':').ok_or_else(|| format!("elements modified: bad entry {entry:?}"))?;
            Ok(PlyElementModified { name: dec_str(name_hex)?, diff: dec_element_diff(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("elements added: bad entry {entry:?}"))?;
            Ok(PlyElementAdded { index: parse_usize(idx)?, element: dec_element(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(PlyElementsDiff { removed, modified, added })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_ply_diff(d: &PlyDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(f) = d.format {
        tokens.push(format!("format={}", enc_format(f)));
    }
    if let Some(c) = &d.comments {
        tokens.push(format!("comments=[{}]", c.iter().map(|s| enc_str(s)).collect::<Vec<_>>().join(",")));
    }
    if let Some(e) = &d.elements {
        tokens.push(format!("elements={}", enc_elements_diff(e)));
    }
    tokens.join(" ")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_ply_diff(line: &str) -> Result<PlyDiff, String> {
    let mut d = PlyDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("format=") {
            d.format = Some(dec_format(rest)?);
        } else if let Some(rest) = token.strip_prefix("comments=") {
            d.comments = Some(split_top_level(strip_brackets(rest)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_str).collect::<Result<Vec<_>, String>>()?);
        } else if let Some(rest) = token.strip_prefix("elements=") {
            d.elements = Some(dec_elements_diff(rest)?);
        } else {
            return Err(format!("ply diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl protocol::DiffText for PlyDiff {
fn print_diff(&self) -> String {
    print_ply_diff(self)
}
fn parse_diff(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
    parse_ply_diff(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
}
}
}
pub use diff_codec::*;
