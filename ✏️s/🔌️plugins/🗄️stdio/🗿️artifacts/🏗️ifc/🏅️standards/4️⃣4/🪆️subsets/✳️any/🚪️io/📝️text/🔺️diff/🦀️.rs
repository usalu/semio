//! 📝️ Text representation codec surface for `stdio.ifc` (diff).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type IfcDiffText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v4::subsets::any::schema::diff::*;
use protocol::{DiffText,DiffBinary};
use std::collections::{BTreeMap, BTreeSet, HashSet};
use crate::schema::snapshot::{IfcComplexType, IfcEntity, IfcTypedValue, IfcValue};
use crate::IfcSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};

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
pub(crate) fn parse_usize(s: &str) -> Result<usize, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_u64(s: &str) -> Result<u64, String> {
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

/// 🔤️ `IfcValue` tag scheme, single uppercase letter + bracketed positional payload (payload-free
/// variants `Unset`/`Derived` are the bare letter, no brackets — never ambiguous with a bracketed
/// payload since every token boundary is either whitespace, `,`, or `;`, never a bare letter
/// followed directly by more letters): `U`=Unset, `D`=Derived, `I[n]`=Integer, `R[n]`=Real (Rust's
/// `Display`/`FromStr` for `f64` round-trip exactly, the shortest decimal that parses back), `S[hex]`
/// =String, `E[hex]`=Enum, `F[n]`=Reference, `A[v,v,...]`=Aggregate, `T[hex,[v,v,...]]`=TypedValue.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_ifc_value(v: &IfcValue) -> String {
    match v {
        IfcValue::Unset => "U".to_string(),
        IfcValue::Derived => "D".to_string(),
        IfcValue::Integer(i) => format!("I[{i}]"),
        IfcValue::Real(r) => format!("R[{r}]"),
        IfcValue::String(s) => format!("S[{}]", enc_str(s)),
        IfcValue::Enum(s) => format!("E[{}]", enc_str(s)),
        IfcValue::Reference(id) => format!("F[{id}]"),
        IfcValue::Aggregate(items) => format!("A[{}]", items.iter().map(enc_ifc_value).collect::<Vec<_>>().join(",")),
        IfcValue::TypedValue(IfcTypedValue { name, items }) => {
            format!("T[{},[{}]]", enc_str(name), items.iter().map(enc_ifc_value).collect::<Vec<_>>().join(","))
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_ifc_value(s: &str) -> Result<IfcValue, String> {
    if s == "U" {
        return Ok(IfcValue::Unset);
    }
    if s == "D" {
        return Ok(IfcValue::Derived);
    }
    if s.is_empty() {
        return Err("ifc value: empty token".to_string());
    }
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "I" => Ok(IfcValue::Integer(inner.parse().map_err(|e: std::num::ParseIntError| e.to_string())?)),
        "R" => Ok(IfcValue::Real(inner.parse().map_err(|e: std::num::ParseFloatError| e.to_string())?)),
        "S" => Ok(IfcValue::String(dec_str(inner)?)),
        "E" => Ok(IfcValue::Enum(dec_str(inner)?)),
        "F" => Ok(IfcValue::Reference(inner.parse().map_err(|e: std::num::ParseIntError| e.to_string())?)),
        "A" => {
            let items = split_top_level(inner, ',').into_iter().filter(|s| !s.is_empty()).map(dec_ifc_value).collect::<Result<Vec<_>, String>>()?;
            Ok(IfcValue::Aggregate(items))
        }
        "T" => {
            let parts = split_top_level(inner, ',');
            let [name, items_s] = parts.as_slice() else { return Err(format!("typed value: expected 2 fields, got {}", parts.len())) };
            let items = split_top_level(strip_brackets(items_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_ifc_value).collect::<Result<Vec<_>, String>>()?;
            Ok(IfcValue::TypedValue(IfcTypedValue { name: dec_str(name)?, items }))
        }
        other => Err(format!("ifc value: unknown tag {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_ifc_value_list(vs: &[IfcValue]) -> String {
    format!("[{}]", vs.iter().map(enc_ifc_value).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_ifc_value_list(s: &str) -> Result<Vec<IfcValue>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_ifc_value).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_complex_type(c: &IfcComplexType) -> String {
    format!("[{},{}]", enc_str(&c.name), enc_ifc_value_list(&c.args))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_complex_type(s: &str) -> Result<IfcComplexType, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, args] = parts.as_slice() else { return Err(format!("complex type: expected 2 fields, got {}", parts.len())) };
    Ok(IfcComplexType { name: dec_str(name)?, args: dec_ifc_value_list(args)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_complex_list(list: &[IfcComplexType]) -> String {
    format!("[{}]", list.iter().map(enc_complex_type).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_complex_list(s: &str) -> Result<Vec<IfcComplexType>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_complex_type).collect()
}

/// 📦️ `[id,hexname,[args],[complex]]` — positional, mirrors [`IfcEntity`]'s own field order.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_entity(e: &IfcEntity) -> String {
    format!("[{},{},{},{}]", e.id, enc_str(&e.name), enc_ifc_value_list(&e.args), enc_complex_list(&e.complex))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_entity(s: &str) -> Result<IfcEntity, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, name, args, complex] = parts.as_slice() else { return Err(format!("entity: expected 4 fields, got {}", parts.len())) };
    Ok(IfcEntity { id: parse_u64(id)?, name: dec_str(name)?, args: dec_ifc_value_list(args)?, complex: dec_complex_list(complex)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_args_diff(d: &IfcArgsDiff) -> String {
    let removed = d.removed.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:{}", m.index, enc_ifc_value(&m.value))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}", a.index, enc_ifc_value(&a.value))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_args_diff(body: &str) -> Result<IfcArgsDiff, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("args diff: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_usize).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("arg modified: bad entry {entry:?}"))?;
            Ok(IfcArgModified { index: parse_usize(idx)?, value: dec_ifc_value(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("arg added: bad entry {entry:?}"))?;
            Ok(IfcArgAdded { index: parse_usize(idx)?, value: dec_ifc_value(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(IfcArgsDiff { removed, modified, added })
}

/// 🔖️ `[nameOpt,argsOpt,complexOpt]` — positional triple, each field individually `Option`-tagged.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_entity_diff(d: &IfcEntityDiff) -> String {
    format!(
        "[{},{},{}]",
        encode_option(&d.name, |v| enc_str(v)),
        match &d.args {
            Some(a) => format!("[1,{}]", enc_args_diff(a)),
            None => "[0]".to_string(),
        },
        match &d.complex {
            Some(c) => format!("[1,{}]", enc_complex_list(c)),
            None => "[0]".to_string(),
        },
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_entity_diff(s: &str) -> Result<IfcEntityDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, args, complex] = parts.as_slice() else { return Err(format!("entity diff: expected 3 fields, got {}", parts.len())) };
    let args = match split_top_level(strip_brackets(args)?, ',').as_slice() {
        ["0"] => None,
        [tag, rest @ ..] if *tag == "1" => Some(dec_args_diff(&rest.join(","))?),
        other => return Err(format!("entity diff args: bad shape {other:?}")),
    };
    let complex = match split_top_level(strip_brackets(complex)?, ',').as_slice() {
        ["0"] => None,
        [tag, rest @ ..] if *tag == "1" => Some(dec_complex_list(&rest.join(","))?),
        other => return Err(format!("entity diff complex: bad shape {other:?}")),
    };
    Ok(IfcEntityDiff { name: decode_option(name, dec_str)?, args, complex })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_entities_diff(d: &IfcEntitiesDiff) -> String {
    let removed = d.removed.iter().map(|id| id.to_string()).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:{}", m.id, enc_entity_diff(&m.diff))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}", a.index, enc_entity(&a.entity))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_entities_diff(body: &str) -> Result<IfcEntitiesDiff, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("entities diff: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_u64).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (id, rest) = entry.split_once(':').ok_or_else(|| format!("entity modified: bad entry {entry:?}"))?;
            Ok(IfcEntityModified { id: parse_u64(id)?, diff: dec_entity_diff(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("entity added: bad entry {entry:?}"))?;
            Ok(IfcEntityAdded { index: parse_usize(idx)?, entity: dec_entity(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(IfcEntitiesDiff { removed, modified, added })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_ifc_diff(d: &IfcDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = &d.file_description {
        tokens.push(format!("file-description={}", enc_ifc_value_list(v)));
    }
    if let Some(v) = &d.file_name {
        tokens.push(format!("file-name={}", enc_ifc_value_list(v)));
    }
    if let Some(v) = &d.file_schema {
        tokens.push(format!("file-schema={}", enc_ifc_value_list(v)));
    }
    if let Some(v) = &d.entities {
        tokens.push(format!("entities={}", enc_entities_diff(v)));
    }
    tokens.join(" ")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_ifc_diff(line: &str) -> Result<IfcDiff, String> {
    let mut d = IfcDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("file-description=") {
            d.file_description = Some(dec_ifc_value_list(rest)?);
        } else if let Some(rest) = token.strip_prefix("file-name=") {
            d.file_name = Some(dec_ifc_value_list(rest)?);
        } else if let Some(rest) = token.strip_prefix("file-schema=") {
            d.file_schema = Some(dec_ifc_value_list(rest)?);
        } else if let Some(rest) = token.strip_prefix("entities=") {
            d.entities = Some(dec_entities_diff(rest)?);
        } else {
            return Err(format!("ifc diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl protocol::DiffText for IfcDiff {
fn print_diff(&self) -> String {
    print_ifc_diff(self)
}
fn parse_diff(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
    parse_ifc_diff(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
}
}

}
pub use diff_codec::*;
