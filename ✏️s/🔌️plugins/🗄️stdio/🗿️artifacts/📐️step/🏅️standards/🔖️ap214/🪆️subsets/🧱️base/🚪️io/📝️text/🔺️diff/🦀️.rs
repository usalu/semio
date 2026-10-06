//! 📝️ Text representation codec surface for `stdio.step` (diff).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type StepDiffText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v_ap214::subsets::base::schema::diff::*;
use protocol::{DiffText,DiffBinary};
use std::collections::{BTreeMap, BTreeSet, HashSet};
use crate::StepSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use crate::schema::snapshot::StepComplexType;
use crate::schema::snapshot::StepEntity;
use crate::schema::snapshot::StepFileDescription;
use crate::schema::snapshot::StepFileName;
use crate::schema::snapshot::StepFileSchema;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use crate::schema::snapshot::StepValue;

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

/// 🔤️ `StepValue` — single-uppercase-letter tag prefix like `enc_xml_node`, one per variant: `U`
/// Unset, `D` Derived, `I` Integer, `R` Real, `S` String, `E` Enum, `F` reFerence (`R` taken by
/// Real), `A` Aggregate (recursive list), `T` TypedValue (recursive, name + one wrapped value).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_value(v: &StepValue) -> String {
    match v {
        StepValue::Unset => "U[]".to_string(),
        StepValue::Derived => "D[]".to_string(),
        StepValue::Integer(i) => format!("I[{i}]"),
        StepValue::Real(r) => format!("R[{r}]"),
        StepValue::String(s) => format!("S[{}]", enc_str(s)),
        StepValue::Enum(s) => format!("E[{}]", enc_str(s)),
        StepValue::Reference(id) => format!("F[{id}]"),
        StepValue::Aggregate(items) => format!("A[{}]", items.iter().map(enc_value).collect::<Vec<_>>().join(",")),
        StepValue::TypedValue { type_name, value } => format!("T[{},{}]", enc_str(type_name), enc_value(value)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_value(s: &str) -> Result<StepValue, String> {
    if s.len() < 3 {
        return Err(format!("step value: too short {s:?}"));
    }
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "U" => Ok(StepValue::Unset),
        "D" => Ok(StepValue::Derived),
        "I" => Ok(StepValue::Integer(inner.parse().map_err(|e: std::num::ParseIntError| e.to_string())?)),
        "R" => Ok(StepValue::Real(inner.parse().map_err(|e: std::num::ParseFloatError| e.to_string())?)),
        "S" => Ok(StepValue::String(dec_str(inner)?)),
        "E" => Ok(StepValue::Enum(dec_str(inner)?)),
        "F" => Ok(StepValue::Reference(inner.parse().map_err(|e: std::num::ParseIntError| e.to_string())?)),
        "A" => {
            let items = split_top_level(inner, ',').into_iter().filter(|s| !s.is_empty()).map(dec_value).collect::<Result<Vec<_>, String>>()?;
            Ok(StepValue::Aggregate(items))
        }
        "T" => {
            let parts = split_top_level(inner, ',');
            let [type_name, value] = parts.as_slice() else { return Err(format!("typed value: expected 2 fields, got {}", parts.len())) };
            Ok(StepValue::TypedValue { type_name: dec_str(type_name)?, value: Box::new(dec_value(value)?) })
        }
        other => Err(format!("step value: unknown tag {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_complex(c: &StepComplexType) -> String {
    format!("[{},[{}]]", enc_str(&c.name), c.args.iter().map(enc_value).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_complex(s: &str) -> Result<StepComplexType, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, args] = parts.as_slice() else { return Err(format!("complex type: expected 2 fields, got {}", parts.len())) };
    let args = split_top_level(strip_brackets(args)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_value).collect::<Result<Vec<_>, String>>()?;
    Ok(StepComplexType { name: dec_str(name)?, args })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_entity(e: &StepEntity) -> String {
    format!("[{},{},[{}],[{}]]", e.id, enc_str(&e.name), e.args.iter().map(enc_value).collect::<Vec<_>>().join(","), e.complex.iter().map(enc_complex).collect::<Vec<_>>().join(","),)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_entity(s: &str) -> Result<StepEntity, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, name, args, complex] = parts.as_slice() else { return Err(format!("entity: expected 4 fields, got {}", parts.len())) };
    Ok(StepEntity {
        id: parse_u64(id)?,
        name: dec_str(name)?,
        args: split_top_level(strip_brackets(args)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_value).collect::<Result<Vec<_>, String>>()?,
        complex: split_top_level(strip_brackets(complex)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_complex).collect::<Result<Vec<_>, String>>()?,
    })
}

/// 📜️ One `LIST[1:?] OF STRING` header slot (`description`, `author`, `organization`, `schemas`).
/// ISO 10303-21 §8.2's lower bound of one is a POPULATION constraint (see `📸️snapshot`'s own
/// `unpopulated_string_list`): `()` is not a legal spelling of "nothing to say", `('')` is. The
/// wire spells both the empty list and the single empty string `[]` — `enc_str("")` IS the empty
/// hex run — so this decoder reads that one spelling as the standard's own conformant minimum
/// rather than as a list the exchange structure cannot carry. Without it, printing a header whose
/// `description` is `[""]` (which is what `StepFileDescription::default()` builds) and parsing it
/// back yielded `[]`, and `op_text_binary_roundtrip_law` measured the loss.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_populated_str_list(s: &str) -> Result<Vec<String>, String> {
    let inner = strip_brackets(s)?;
    if inner.is_empty() {
        return Ok(crate::schema::snapshot::unpopulated_string_list());
    }
    split_top_level(inner, ',').into_iter().map(dec_str).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_file_description(d: &StepFileDescription) -> String {
    format!("[[{}],{}]", d.description.iter().map(|s| enc_str(s)).collect::<Vec<_>>().join(","), enc_str(&d.implementation_level))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_file_description(s: &str) -> Result<StepFileDescription, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [description, implementation_level] = parts.as_slice() else { return Err(format!("file description: expected 2 fields, got {}", parts.len())) };
    Ok(StepFileDescription { description: dec_populated_str_list(description)?, implementation_level: dec_str(implementation_level)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_file_name(f: &StepFileName) -> String {
    format!(
        "[{},{},[{}],[{}],{},{},{}]",
        enc_str(&f.name),
        enc_str(&f.timestamp),
        f.author.iter().map(|s| enc_str(s)).collect::<Vec<_>>().join(","),
        f.organization.iter().map(|s| enc_str(s)).collect::<Vec<_>>().join(","),
        enc_str(&f.preprocessor_version),
        enc_str(&f.originating_system),
        enc_str(&f.authorization),
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_file_name(s: &str) -> Result<StepFileName, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, timestamp, author, organization, preprocessor_version, originating_system, authorization] = parts.as_slice() else {
        return Err(format!("file name: expected 7 fields, got {}", parts.len()));
    };
    Ok(StepFileName {
        name: dec_str(name)?,
        timestamp: dec_str(timestamp)?,
        author: dec_populated_str_list(author)?,
        organization: dec_populated_str_list(organization)?,
        preprocessor_version: dec_str(preprocessor_version)?,
        originating_system: dec_str(originating_system)?,
        authorization: dec_str(authorization)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_file_schema(s: &StepFileSchema) -> String {
    format!("[{}]", s.schemas.iter().map(|x| enc_str(x)).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_file_schema(s: &str) -> Result<StepFileSchema, String> {
    let schemas = dec_populated_str_list(s)?;
    Ok(StepFileSchema { schemas })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_args_diff(d: &StepArgsDiff) -> String {
    let removed = d.removed.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:{}", m.index, enc_value(&m.value))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}", a.index, enc_value(&a.value))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_args_diff(body: &str) -> Result<StepArgsDiff, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("args diff: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_usize).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("arg modified: bad entry {entry:?}"))?;
            Ok(StepArgModified { index: parse_usize(idx)?, value: dec_value(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("arg added: bad entry {entry:?}"))?;
            Ok(StepArgAdded { index: parse_usize(idx)?, value: dec_value(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(StepArgsDiff { removed, modified, added })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_entity_diff(d: &StepEntityDiff) -> String {
    format!("[{},{},{}]", encode_option(&d.name, |v| enc_str(v)), encode_option(&d.args, enc_args_diff), encode_option(&d.complex, |v| format!("[{}]", v.iter().map(enc_complex).collect::<Vec<_>>().join(","))),)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_entity_diff(s: &str) -> Result<StepEntityDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, args, complex] = parts.as_slice() else { return Err(format!("entity diff: expected 3 fields, got {}", parts.len())) };
    Ok(StepEntityDiff {
        name: decode_option(name, dec_str)?,
        args: decode_option(args, dec_args_diff)?,
        complex: decode_option(complex, |v| split_top_level(strip_brackets(v)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_complex).collect::<Result<Vec<_>, String>>())?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_entities_diff(d: &StepEntitiesDiff) -> String {
    let removed = d.removed.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:{}", m.id, enc_entity_diff(&m.diff))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}", a.index, enc_entity(&a.entity))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_entities_diff(body: &str) -> Result<StepEntitiesDiff, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("entities diff: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_u64).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (id, rest) = entry.split_once(':').ok_or_else(|| format!("entity modified: bad entry {entry:?}"))?;
            Ok(StepEntityModified { id: parse_u64(id)?, diff: dec_entity_diff(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("entity added: bad entry {entry:?}"))?;
            Ok(StepEntityAdded { index: parse_usize(idx)?, entity: dec_entity(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(StepEntitiesDiff { removed, modified, added })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_step_diff(d: &StepDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = &d.file_description {
        tokens.push(format!("file-description={}", enc_file_description(v)));
    }
    if let Some(v) = &d.file_name {
        tokens.push(format!("file-name={}", enc_file_name(v)));
    }
    if let Some(v) = &d.file_schema {
        tokens.push(format!("file-schema={}", enc_file_schema(v)));
    }
    if let Some(v) = &d.entities {
        tokens.push(format!("entities={}", enc_entities_diff(v)));
    }
    tokens.join(" ")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_step_diff(line: &str) -> Result<StepDiff, String> {
    let mut d = StepDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("file-description=") {
            d.file_description = Some(dec_file_description(rest)?);
        } else if let Some(rest) = token.strip_prefix("file-name=") {
            d.file_name = Some(dec_file_name(rest)?);
        } else if let Some(rest) = token.strip_prefix("file-schema=") {
            d.file_schema = Some(dec_file_schema(rest)?);
        } else if let Some(rest) = token.strip_prefix("entities=") {
            d.entities = Some(dec_entities_diff(rest)?);
        } else {
            return Err(format!("step diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl protocol::DiffText for StepDiff {
fn print_diff(&self) -> String {
    print_step_diff(self)
}
fn parse_diff(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
    parse_step_diff(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
}
}
}
pub use diff_codec::*;
