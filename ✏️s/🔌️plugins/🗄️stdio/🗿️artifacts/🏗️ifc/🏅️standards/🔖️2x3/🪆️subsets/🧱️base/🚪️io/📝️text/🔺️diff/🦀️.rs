//! 📝️ Text representation codec surface for `stdio.ifc.2x3` (diff).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type Ifc2x3DiffText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v2x3::subsets::base::schema::diff::*;
use protocol::{DiffText,DiffBinary};
use crate::standards::v2x3::subsets::base::schema::snapshot::{Ifc2x3EdmPreamble, Ifc2x3Snapshot};
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use semio_s_artifact_stdio_contract::part21::{Part21Decimal, Part21Header, Part21Instance, Part21Value};
use framework_schema::ArtifactSchema;
use protocol::os_spr::command::DiffAlgebra;
use std::collections::{BTreeSet, HashSet};
use std::fmt::Write as _;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_encode(bytes: &[u8]) -> String {
    let mut encoded = String::with_capacity(bytes.len() * 2);
    hex_encode_into(bytes, &mut encoded);
    encoded
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_encode_into(bytes: &[u8], encoded: &mut String) {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    for byte in bytes {
        encoded.push(HEX[(byte >> 4) as usize] as char);
        encoded.push(HEX[(byte & 0x0f) as usize] as char);
    }
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
pub(crate) fn enc_edm_preamble(preamble: &Ifc2x3EdmPreamble) -> String {
    format!(
        "[{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}]",
        enc_str(&preamble.producer),
        enc_str(&preamble.module),
        enc_str(&preamble.creation_date),
        enc_str(&preamble.host),
        enc_str(&preamble.database),
        enc_str(&preamble.database_version),
        enc_str(&preamble.database_creation_date),
        enc_str(&preamble.schema),
        enc_str(&preamble.model),
        enc_str(&preamble.model_creation_date),
        enc_str(&preamble.header_model),
        enc_str(&preamble.header_model_creation_date),
        enc_str(&preamble.user),
        enc_str(&preamble.group),
        enc_str(&preamble.license),
        enc_str(&preamble.options)
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_edm_preamble(s: &str) -> Result<Ifc2x3EdmPreamble, String> {
    let fields = split_top_level(strip_brackets(s)?, ',');
    let [producer, module, creation_date, host, database, database_version, database_creation_date, schema, model, model_creation_date, header_model, header_model_creation_date, user, group, license, options] = fields.as_slice() else {
        return Err(format!("EDM preamble: expected 16 fields, got {}", fields.len()));
    };
    Ok(Ifc2x3EdmPreamble {
        producer: dec_str(producer)?,
        module: dec_str(module)?,
        creation_date: dec_str(creation_date)?,
        host: dec_str(host)?,
        database: dec_str(database)?,
        database_version: dec_str(database_version)?,
        database_creation_date: dec_str(database_creation_date)?,
        schema: dec_str(schema)?,
        model: dec_str(model)?,
        model_creation_date: dec_str(model_creation_date)?,
        header_model: dec_str(header_model)?,
        header_model_creation_date: dec_str(header_model_creation_date)?,
        user: dec_str(user)?,
        group: dec_str(group)?,
        license: dec_str(license)?,
        options: dec_str(options)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_optional_edm_preamble(preamble: &Option<Ifc2x3EdmPreamble>) -> String {
    preamble.as_ref().map_or_else(|| "[0]".into(), |value| format!("[1,{}]", enc_edm_preamble(value)))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_optional_edm_preamble(s: &str) -> Result<Option<Ifc2x3EdmPreamble>, String> {
    match split_top_level(strip_brackets(s)?, ',').as_slice() {
        ["0"] => Ok(None),
        ["1", value] => Ok(Some(dec_edm_preamble(value)?)),
        _ => Err(format!("optional EDM preamble: invalid payload {s:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_u64(s: &str) -> Result<u64, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn split_top_level(s: &str, sep: char) -> Vec<&str> {
    split_top_level_iter(s, sep).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn split_top_level_iter(s: &str, sep: char) -> impl Iterator<Item = &str> {
    let separator = sep as u8;
    let mut depth = 0i32;
    let mut start = 0usize;
    let mut cursor = 0usize;
    std::iter::from_fn(move || {
        if s.is_empty() || cursor > s.len() {
            return None;
        }
        while cursor < s.len() {
            match s.as_bytes()[cursor] {
                b'[' => depth += 1,
                b']' => depth -= 1,
                byte if byte == separator && depth == 0 => {
                    let item = &s[start..cursor];
                    cursor += 1;
                    start = cursor;
                    return Some(item);
                }
                _ => {}
            }
            cursor += 1;
        }
        cursor += 1;
        Some(&s[start..])
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn strip_brackets(s: &str) -> Result<&str, String> {
    s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))
}

/// 🔤️ `Part21Value`'s tag scheme, single uppercase letter + bracketed positional payload
/// (payload-free variants `Unset`/`Derived` are the bare letter, no brackets — never ambiguous
/// since every token boundary is whitespace/`,`/`;`, matching `4`'s own `IfcValue` convention
/// exactly, same isomorphic 9-variant shape): `U`=Unset, `D`=Derived, `I[n]`=Int, `R[n]`=Real
/// (Rust's `Display`/`FromStr` for `f64` round-trip exactly), `S[hex]`=Str, `E[hex]`=Enum,
/// `F[n]`=Ref, `A[v,v,...]`=List, `T[hex,[v,v,...]]`=Typed.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_part21_value_into(v: &Part21Value, out: &mut String) {
    match v {
        Part21Value::Unset => out.push('U'),
        Part21Value::Derived => out.push('D'),
        Part21Value::Int(i) => {
            write!(out, "I[{i}]").expect("writing to String");
        }
        Part21Value::Real(r) => {
            write!(out, "R[{r}]").expect("writing to String");
        }
        Part21Value::Str(s) => {
            out.push_str("S[");
            hex_encode_into(s.as_bytes(), out);
            out.push(']');
        }
        Part21Value::Enum(s) => {
            out.push_str("E[");
            hex_encode_into(s.as_bytes(), out);
            out.push(']');
        }
        Part21Value::Ref(id) => {
            write!(out, "F[{id}]").expect("writing to String");
        }
        Part21Value::List(items) => {
            out.push_str("A[");
            enc_part21_values_into(items, out);
            out.push(']');
        }
        Part21Value::Typed { name, items } => {
            out.push_str("T[");
            hex_encode_into(name.as_bytes(), out);
            out.push_str(",[");
            enc_part21_values_into(items, out);
            out.push_str("]]");
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_part21_values_into(values: &[Part21Value], out: &mut String) {
    for (index, value) in values.iter().enumerate() {
        if index != 0 {
            out.push(',');
        }
        enc_part21_value_into(value, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_part21_value(s: &str) -> Result<Part21Value, String> {
    if s == "U" {
        return Ok(Part21Value::Unset);
    }
    if s == "D" {
        return Ok(Part21Value::Derived);
    }
    if s.is_empty() {
        return Err("part21 value: empty token".to_string());
    }
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "I" => Ok(Part21Value::Int(inner.parse().map_err(|e: std::num::ParseIntError| e.to_string())?)),
        "R" => Ok(Part21Value::Real(Part21Decimal::parse(inner)?)),
        "S" => Ok(Part21Value::Str(dec_str(inner)?)),
        "E" => Ok(Part21Value::Enum(dec_str(inner)?)),
        "F" => Ok(Part21Value::Ref(inner.parse().map_err(|e: std::num::ParseIntError| e.to_string())?)),
        "A" => {
            let items = split_top_level(inner, ',').into_iter().filter(|s| !s.is_empty()).map(dec_part21_value).collect::<Result<Vec<_>, String>>()?;
            Ok(Part21Value::List(items))
        }
        "T" => {
            let parts = split_top_level(inner, ',');
            let [name, items_s] = parts.as_slice() else { return Err(format!("typed value: expected 2 fields, got {}", parts.len())) };
            let items = split_top_level(strip_brackets(items_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_part21_value).collect::<Result<Vec<_>, String>>()?;
            Ok(Part21Value::Typed { name: dec_str(name)?, items })
        }
        other => Err(format!("part21 value: unknown tag {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_part21_value_list(vs: &[Part21Value]) -> String {
    let mut out = String::new();
    enc_part21_value_list_into(vs, &mut out);
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_part21_value_list_into(vs: &[Part21Value], out: &mut String) {
    out.push('[');
    enc_part21_values_into(vs, out);
    out.push(']');
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_part21_value_list(s: &str) -> Result<Vec<Part21Value>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_part21_value).collect()
}

/// 📦️ `[fileDescriptionList,fileNameList,fileSchemaList]` — three self-bracketed
/// `part21-value-list`s, matching `4`'s own `enc_ifc_header` positional shape exactly (both
/// standards' HEADER record is the same 3-tuple-of-raw-value-list Part-21 shape).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_part21_header(h: &Part21Header) -> String {
    format!("[{},{},{}]", enc_part21_value_list(&h.file_description), enc_part21_value_list(&h.file_name), enc_part21_value_list(&h.file_schema))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_part21_header(s: &str) -> Result<Part21Header, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [fd, fname, fs] = parts.as_slice() else { return Err(format!("part21 header: expected 3 fields, got {}", parts.len())) };
    Ok(Part21Header { file_description: dec_part21_value_list(fd)?, file_name: dec_part21_value_list(fname)?, file_schema: dec_part21_value_list(fs)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_part21_instance_into(inst: &Part21Instance, out: &mut String) {
    out.push('[');
    write!(out, "{}", inst.id).expect("writing to String");
    out.push_str(",[");
    for (index, (name, args)) in inst.entities.iter().enumerate() {
        if index != 0 {
            out.push(',');
        }
        out.push('[');
        hex_encode_into(name.as_bytes(), out);
        out.push(',');
        enc_part21_value_list_into(args, out);
        out.push(']');
    }
    out.push_str("]]");
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_part21_instance(s: &str) -> Result<Part21Instance, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id_s, entities_s] = parts.as_slice() else {
        return Err(format!("part21 instance: expected 2 fields, got {}", parts.len()));
    };
    let entities_inner = strip_brackets(entities_s)?;
    let entities = split_top_level_iter(entities_inner, ',')
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let e = split_top_level(strip_brackets(entry)?, ',');
            let [name, args] = e.as_slice() else {
                return Err(format!("part21 entity: expected 2 fields, got {}", e.len()));
            };
            Ok((dec_str(name)?, dec_part21_value_list(args)?))
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(Part21Instance { id: parse_u64(id_s)?, entities })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_instance_list_into(list: &[Part21Instance], out: &mut String) {
    out.push('[');
    for (index, instance) in list.iter().enumerate() {
        if index != 0 {
            out.push(',');
        }
        enc_part21_instance_into(instance, out);
    }
    out.push(']');
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_instance_list(s: &str) -> Result<Vec<Part21Instance>, String> {
    split_top_level_iter(strip_brackets(s)?, ',').filter(|s| !s.is_empty()).map(dec_part21_instance).collect()
}

/// 🔖️ One line of space-separated `key=value` tokens, only the CHANGED top-level fields present,
/// in declared field order (`schema`/`header`/`removed`/`upserted`) — matching `4`'s own
/// `print_ifc_diff` shape exactly.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_ifc2x3_diff(d: &Ifc2x3Diff) -> String {
    let mut out = String::new();
    let separate = |out: &mut String| {
        if !out.is_empty() {
            out.push(' ');
        }
    };
    if let Some(s) = &d.schema {
        separate(&mut out);
        out.push_str("schema=");
        out.push_str(&enc_str(s));
    }
    if let Some(h) = &d.header {
        separate(&mut out);
        out.push_str("header=");
        out.push_str(&enc_part21_header(h));
    }
    if !d.removed_instances.is_empty() {
        separate(&mut out);
        out.push_str("removed=[");
        for (index, id) in d.removed_instances.iter().enumerate() {
            if index != 0 {
                out.push(',');
            }
            out.push_str(&id.to_string());
        }
        out.push(']');
    }
    if !d.upserted_instances.is_empty() {
        separate(&mut out);
        out.push_str("upserted=");
        enc_instance_list_into(&d.upserted_instances, &mut out);
    }
    if let Some(preamble) = &d.edm_preamble {
        separate(&mut out);
        out.push_str("edm-preamble=");
        out.push_str(&enc_optional_edm_preamble(preamble));
    }
    if let Some(order) = &d.instance_order {
        separate(&mut out);
        out.push_str("instance-order=[");
        for (index, id) in order.iter().enumerate() {
            if index != 0 {
                out.push(',');
            }
            out.push_str(&id.to_string());
        }
        out.push(']');
    }
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_ifc2x3_diff(line: &str) -> Result<Ifc2x3Diff, String> {
    let mut d = Ifc2x3Diff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("schema=") {
            d.schema = Some(dec_str(rest)?);
        } else if let Some(rest) = token.strip_prefix("header=") {
            d.header = Some(dec_part21_header(rest)?);
        } else if let Some(rest) = token.strip_prefix("removed=") {
            d.removed_instances = split_top_level(strip_brackets(rest)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_u64).collect::<Result<Vec<_>, String>>()?;
        } else if let Some(rest) = token.strip_prefix("upserted=") {
            d.upserted_instances = dec_instance_list(rest)?;
        } else if let Some(rest) = token.strip_prefix("edm-preamble=") {
            d.edm_preamble = Some(dec_optional_edm_preamble(rest)?);
        } else if let Some(rest) = token.strip_prefix("instance-order=") {
            d.instance_order = Some(split_top_level(strip_brackets(rest)?, ',').into_iter().filter(|value| !value.is_empty()).map(parse_u64).collect::<Result<Vec<_>, _>>()?);
        } else {
            return Err(format!("ifc2x3 diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl protocol::DiffText for Ifc2x3Diff {
fn print_diff(&self) -> String {
    print_ifc2x3_diff(self)
}
fn parse_diff(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
    parse_ifc2x3_diff(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
}
}
}
pub use diff_codec::*;

#[allow(unused_imports)]
mod diff_wire_codec {
use super::*;
use crate::standards::v2x3::subsets::base::schema::diff::*;
use crate::standards::v2x3::subsets::base::schema::snapshot::{Ifc2x3EdmPreamble, Ifc2x3Snapshot};
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use semio_s_artifact_stdio_contract::part21::{Part21Decimal, Part21Header, Part21Instance, Part21Value};
use framework_schema::ArtifactSchema;
use protocol::os_spr::command::DiffAlgebra;
use std::collections::{BTreeSet, HashSet};
use std::fmt::Write as _;

/// 📦️ `[id,[entity,entity,...]]` — a `Part21Instance`'s `entities: Vec<(String,Vec<Part21Value>)>`
/// list has 1 entry for a simple instance, 2+ for a real spec-legal COMPLEX instance (ISO
/// 10303-21 §4.2) — same shape `4`'s own snapshot grammar's `instance-body = entity-record |
/// "(" entity-record+ ")"` recognizes at the exchange-file level, restated here for the diff/op
/// wire's own positional codec. Each entity is `[hexname,[args]]`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_part21_instance(inst: &Part21Instance) -> String {
    let mut out = String::new();
    enc_part21_instance_into(inst, &mut out);
    out
}
}
pub use diff_wire_codec::*;
