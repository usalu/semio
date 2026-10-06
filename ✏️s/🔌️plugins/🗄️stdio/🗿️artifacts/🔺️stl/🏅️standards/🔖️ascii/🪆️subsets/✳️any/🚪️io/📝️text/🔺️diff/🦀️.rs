//! 📝️ Text representation codec surface for `stdio.stl` (diff).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type StlDiffText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v_ascii::subsets::any::schema::diff::*;
use crate::schema::snapshot::StlTriangle;
use crate::StlSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use std::collections::{BTreeSet, HashMap, HashSet};

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
pub(crate) fn hex_encode_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_decode_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_f64(s: &str) -> Result<f64, String> {
    s.parse().map_err(|e: std::num::ParseFloatError| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_usize(s: &str) -> Result<usize, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}

/// 🧭️ Bracket-depth-aware split (tracks `[`/`]` only): a top-level `sep` inside nested brackets is
/// never mistaken for a field separator — the whole hand-rolled grammar's parsing primitive (same
/// technique `gif`89a's/`svg`'s hand-rolled `DiffCodec` use).
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

/// 📐️ One `[f64; 3]` level — the depth marker `dsl`'s own `Shape::Tuple` printer is missing.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_vec3(v: &[f64; 3]) -> String {
    format!("[{},{},{}]", v[0], v[1], v[2])
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_vec3(s: &str) -> Result<[f64; 3], String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [x, y, z] = parts.as_slice() else { return Err(format!("vec3: expected 3 fields, got {}", parts.len())) };
    Ok([parse_f64(x)?, parse_f64(y)?, parse_f64(z)?])
}

/// 📐️ The outer `[[f64; 3]; 3]` level — 3 `enc_vec3`-bracketed vertices inside one more `[...]`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_vertices(vs: &[[f64; 3]; 3]) -> String {
    format!("[{}]", vs.iter().map(enc_vec3).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_vertices(s: &str) -> Result<[[f64; 3]; 3], String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [v0, v1, v2] = parts.as_slice() else { return Err(format!("vertices: expected 3 fields, got {}", parts.len())) };
    Ok([dec_vec3(v0)?, dec_vec3(v1)?, dec_vec3(v2)?])
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_triangle(t: &StlTriangle) -> String {
    format!("[{},{}]", enc_vec3(&t.normal), enc_vertices(&t.vertices))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_triangle(s: &str) -> Result<StlTriangle, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [normal, vertices] = parts.as_slice() else { return Err(format!("triangle: expected 2 fields, got {}", parts.len())) };
    Ok(StlTriangle { normal: dec_vec3(normal)?, vertices: dec_vertices(vertices)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_triangle_diff(d: &StlTriangleDiff) -> String {
    let mut parts = Vec::new();
    if let Some(v) = &d.normal {
        parts.push(format!("N:{}", enc_vec3(v)));
    }
    if let Some(v) = &d.vertices {
        parts.push(format!("V:{}", enc_vertices(v)));
    }
    format!("[{}]", parts.join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_triangle_diff(s: &str) -> Result<StlTriangleDiff, String> {
    let inner = strip_brackets(s)?;
    let mut d = StlTriangleDiff::default();
    for entry in split_top_level(inner, ',') {
        if entry.is_empty() {
            continue;
        }
        let (tag, val) = entry.split_once(':').ok_or_else(|| format!("triangle diff: bad entry {entry:?}"))?;
        match tag {
            "N" => d.normal = Some(dec_vec3(val)?),
            "V" => d.vertices = Some(dec_vertices(val)?),
            other => return Err(format!("triangle diff: unknown tag {other:?}")),
        }
    }
    Ok(d)
}

/// 🧭️ Generic-shaped 3-section `[removed];[modified];[added]` collection-triple printer/parser
/// (same shape `gif`89a's hand-roll uses, ported here for `triangles`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_collection_triple(name: &str, removed: &[usize], modified: &[(usize, String)], added: &[(usize, String)]) -> String {
    let removed = removed.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let modified = modified.iter().map(|(i, v)| format!("{i}:{v}")).collect::<Vec<_>>().join(",");
    let added = added.iter().map(|(i, v)| format!("{i}:{v}")).collect::<Vec<_>>().join(",");
    format!("{name}{{[{removed}];[{modified}];[{added}]}}")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_collection_triple(body: &str) -> Result<IndexedDiffParts<String, String>, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("collection: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_usize).collect::<Result<Vec<_>, String>>()?;
    let parse_entries = |s: &str| -> Result<Vec<(usize, String)>, String> {
        split_top_level(strip_brackets(s)?, ',')
            .into_iter()
            .filter(|s| !s.is_empty())
            .map(|entry| {
                let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("collection entry: bad entry {entry:?}"))?;
                Ok((parse_usize(idx)?, rest.to_string()))
            })
            .collect()
    };
    Ok((removed, parse_entries(modified_s)?, parse_entries(added_s)?))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_triangles_diff(d: &StlTrianglesDiff) -> String {
    enc_collection_triple("triangles", &d.removed, &d.modified.iter().map(|m| (m.index, enc_triangle_diff(&m.diff))).collect::<Vec<_>>(), &d.added.iter().map(|a| (a.index, enc_triangle(&a.triangle))).collect::<Vec<_>>())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_triangles_diff(body: &str) -> Result<StlTrianglesDiff, String> {
    let (removed, modified, added) = dec_collection_triple(body)?;
    Ok(StlTrianglesDiff {
        removed,
        modified: modified.into_iter().map(|(index, enc)| Ok(StlTriangleModified { index, diff: dec_triangle_diff(&enc)? })).collect::<Result<Vec<_>, String>>()?,
        added: added.into_iter().map(|(index, enc)| Ok(StlTriangleAdded { index, triangle: dec_triangle(&enc)? })).collect::<Result<Vec<_>, String>>()?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_stl_diff(d: &StlDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = &d.solid_name {
        tokens.push(format!("solid-name={}", hex_encode_str(v)));
    }
    if let Some(v) = &d.triangles {
        tokens.push(enc_triangles_diff(v));
    }
    tokens.join(" ")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_stl_diff(line: &str) -> Result<StlDiff, String> {
    let mut d = StlDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("solid-name=") {
            d.solid_name = Some(hex_decode_str(rest)?);
        } else if let Some(rest) = token.strip_prefix("triangles{") {
            d.triangles = Some(dec_triangles_diff(rest.strip_suffix('}').ok_or_else(|| "triangles: missing closing brace".to_string())?)?);
        } else {
            return Err(format!("stl diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl protocol::DiffText for StlDiff {
fn print_diff(&self) -> String {
    print_stl_diff(self)
}
fn parse_diff(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
    parse_stl_diff(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
}
}
}
pub use diff_codec::*;
