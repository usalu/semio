//! 📝️ Text representation codec surface for `stdio.semio.value` (diff).

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v1::subsets::value::schema::diff::*;
use protocol::{DiffText,DiffBinary};
use crate::standards::v1::subsets::base::schema::triples::{IndexAdded, IndexModified, IndexedTripleDiff, NamedModified, NamedTripleDiff};
use crate::standards::v1::subsets::base::io::text::snapshot::{dec_indexed_triple, dec_named_triple, enc_indexed_triple, enc_named_triple};
use crate::standards::v1::subsets::audio::io::text::diff::{strip_brackets};
use crate::standards::v1::subsets::audio::io::text::diff::{split_top_level};
use crate::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot;
use crate::standards::v1::subsets::value::schema::snapshot::{SemioValue, SemioValueEntry, SemioValueNode, ValueId};
use framework_schema::ArtifactSchema;
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_value_id(id: &ValueId) -> String {
    enc_str(&id.value)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_value_id(s: &str) -> Result<ValueId, String> {
    Ok(ValueId::new(dec_str(s)?))
}

/// 🌳 Tag-prefixed like `json`'s `enc_json_value`: `Z` (null, no payload, no brackets) / `B[0|1]`
/// / `I[hex(lexeme)]` / `F[hex(lexeme)]` / `S[hex(value)]` / `Y[hex(bytes)]` / `L[v1,v2,...]` /
/// `M[hexkey1:v1,hexkey2:v2,...]` / `R[hex(id)]` — member insertion order preserved by
/// construction (a list, never re-sorted).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_semio_value(v: &SemioValue) -> String {
    match v {
        SemioValue::Null => "Z".to_string(),
        SemioValue::Bool { value } => format!("B[{}]", if *value { "1" } else { "0" }),
        SemioValue::Int { lexeme } => format!("I[{}]", enc_str(lexeme)),
        SemioValue::Float { lexeme } => format!("F[{}]", enc_str(lexeme)),
        SemioValue::Str { value } => format!("S[{}]", enc_str(value)),
        SemioValue::Bytes { value } => format!("Y[{}]", hex_encode(value)),
        SemioValue::List { items } => format!("L[{}]", items.iter().map(enc_semio_value).collect::<Vec<_>>().join(",")),
        SemioValue::Map { entries } => format!("M[{}]", entries.iter().map(|e| format!("{}:{}", enc_str(&e.key), enc_semio_value(&e.value))).collect::<Vec<_>>().join(",")),
        SemioValue::Ref { id } => format!("R[{}]", enc_value_id(id)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_semio_value(s: &str) -> Result<SemioValue, String> {
    if s == "Z" {
        return Ok(SemioValue::Null);
    }
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "B" => Ok(SemioValue::Bool { value: inner == "1" }),
        "I" => Ok(SemioValue::Int { lexeme: dec_str(inner)? }),
        "F" => Ok(SemioValue::Float { lexeme: dec_str(inner)? }),
        "S" => Ok(SemioValue::Str { value: dec_str(inner)? }),
        "Y" => Ok(SemioValue::Bytes { value: hex_decode(inner)? }),
        "L" => Ok(SemioValue::List { items: split_top_level(inner, ',').into_iter().filter(|s| !s.is_empty()).map(dec_semio_value).collect::<Result<Vec<_>, String>>()? }),
        "M" => {
            let entries = split_top_level(inner, ',')
                .into_iter()
                .filter(|s| !s.is_empty())
                .map(|entry| {
                    let (key, value) = entry.split_once(':').ok_or_else(|| format!("map entry: bad entry {entry:?}"))?;
                    Ok(SemioValueEntry { key: dec_str(key)?, value: dec_semio_value(value)? })
                })
                .collect::<Result<Vec<_>, String>>()?;
            Ok(SemioValue::Map { entries })
        }
        "R" => Ok(SemioValue::Ref { id: dec_value_id(inner)? }),
        other => Err(format!("semio value: unknown tag {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_semio_value_entry(e: &SemioValueEntry) -> String {
    format!("{}:{}", enc_str(&e.key), enc_semio_value(&e.value))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_semio_value_entry(s: &str) -> Result<SemioValueEntry, String> {
    let (key, value) = s.split_once(':').ok_or_else(|| format!("value entry: bad entry {s:?}"))?;
    Ok(SemioValueEntry { key: dec_str(key)?, value: dec_semio_value(value)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_semio_value_node(n: &SemioValueNode) -> String {
    format!("{}:{}", enc_value_id(&n.id), enc_semio_value(&n.value))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_semio_value_node(s: &str) -> Result<SemioValueNode, String> {
    let (id, value) = s.split_once(':').ok_or_else(|| format!("value node: bad entry {s:?}"))?;
    Ok(SemioValueNode { id: dec_value_id(id)?, value: dec_semio_value(value)? })
}

/// 🧷 `NamedAdded<T>`-wrapping variants of the two encoders above — `index:` prefixed — used ONLY
/// for a diff's own `added` list (see [`NamedAdded`]'s doc comment); the plain (unwrapped)
/// encoders above stay the ones `🧬️mutations`' snapshot-level `nodes` list encoding uses.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_named_added_entry(a: &NamedAdded<SemioValueEntry>) -> String {
    format!("{}:{}", a.index, enc_semio_value_entry(&a.item))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_named_added_entry(s: &str) -> Result<NamedAdded<SemioValueEntry>, String> {
    let (idx, rest) = s.split_once(':').ok_or_else(|| format!("named added entry: bad entry {s:?}"))?;
    Ok(NamedAdded { index: idx.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, item: dec_semio_value_entry(rest)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_named_added_node(a: &NamedAdded<SemioValueNode>) -> String {
    format!("{}:{}", a.index, enc_semio_value_node(&a.item))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_named_added_node(s: &str) -> Result<NamedAdded<SemioValueNode>, String> {
    let (idx, rest) = s.split_once(':').ok_or_else(|| format!("named added node: bad entry {s:?}"))?;
    Ok(NamedAdded { index: idx.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, item: dec_semio_value_node(rest)? })
}

/// 🌳 `SemioValueDiff` itself needs a tag (`P`=rePlace, `B`=Bool, `I`=Int, `F`=Float, `S`=Str,
/// `Y`=Bytes, `L`=List, `M`=Map, `R`=Ref) since, unlike a plain [`SemioValue`], it appears
/// standalone (not always inside a bracketed container) at the top-level `root=`/`nodes=` token
/// position.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_value_diff(d: &SemioValueDiff) -> String {
    match d {
        SemioValueDiff::Replace { value } => format!("P[{}]", enc_semio_value(value)),
        SemioValueDiff::Bool { value } => format!("B[{}]", if *value { "1" } else { "0" }),
        SemioValueDiff::Int { lexeme } => format!("I[{}]", enc_str(lexeme)),
        SemioValueDiff::Float { lexeme } => format!("F[{}]", enc_str(lexeme)),
        SemioValueDiff::Str { value } => format!("S[{}]", enc_str(value)),
        SemioValueDiff::Bytes { value } => format!("Y[{}]", hex_encode(value)),
        SemioValueDiff::List { diff } => format!("L[{}]", enc_indexed_triple(diff, enc_value_diff, enc_semio_value)),
        SemioValueDiff::Map { diff } => format!("M[{}]", enc_named_triple(diff, |k: &String| enc_str(k), enc_value_diff, enc_named_added_entry)),
        SemioValueDiff::Ref { id } => format!("R[{}]", enc_value_id(id)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_value_diff(s: &str) -> Result<SemioValueDiff, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "P" => Ok(SemioValueDiff::Replace { value: dec_semio_value(inner)? }),
        "B" => Ok(SemioValueDiff::Bool { value: inner == "1" }),
        "I" => Ok(SemioValueDiff::Int { lexeme: dec_str(inner)? }),
        "F" => Ok(SemioValueDiff::Float { lexeme: dec_str(inner)? }),
        "S" => Ok(SemioValueDiff::Str { value: dec_str(inner)? }),
        "Y" => Ok(SemioValueDiff::Bytes { value: hex_decode(inner)? }),
        "L" => Ok(SemioValueDiff::List { diff: dec_indexed_triple(inner, dec_value_diff, dec_semio_value)? }),
        "M" => Ok(SemioValueDiff::Map { diff: dec_named_triple(inner, dec_str, dec_value_diff, dec_named_added_entry)? }),
        "R" => Ok(SemioValueDiff::Ref { id: dec_value_id(inner)? }),
        other => Err(format!("semio value diff: unknown tag {other:?}")),
    }
}

/// 🧭️ Two-field top level (`root=<enc>` / `nodes=<enc>`, either absent = unchanged).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_value_tree_diff(d: &SemioValueTreeDiff) -> String {
    let mut tokens = Vec::new();
    if let Some(v) = &d.root {
        tokens.push(format!("root={}", enc_value_diff(v)));
    }
    if let Some(o) = &d.nodes {
        tokens.push(format!("nodes={}", enc_named_triple(o, enc_value_id, enc_value_diff, enc_named_added_node)));
    }
    tokens.join(" ")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_value_tree_diff(line: &str) -> Result<SemioValueTreeDiff, String> {
    let mut d = SemioValueTreeDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("root=") {
            d.root = Some(dec_value_diff(rest)?);
        } else if let Some(rest) = token.strip_prefix("nodes=") {
            d.nodes = Some(dec_named_triple(rest, dec_value_id, dec_value_diff, dec_named_added_node)?);
        } else {
            return Err(format!("semio value diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl protocol::DiffText for SemioValueTreeDiff {
fn print_diff(&self) -> String {
    print_value_tree_diff(self)
}
fn parse_diff(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
    parse_value_tree_diff(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
}
}
}
pub use diff_codec::*;
