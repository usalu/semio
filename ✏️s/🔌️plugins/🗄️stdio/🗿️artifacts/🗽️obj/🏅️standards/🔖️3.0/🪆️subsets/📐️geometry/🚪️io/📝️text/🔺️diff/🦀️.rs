//! 📝️ Text representation codec surface for `stdio.obj` (diff).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type ObjDiffText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v3_0::subsets::any::schema::diff::*;
use crate::schema::snapshot::{ObjFace, ObjGroup, ObjNormal, ObjObject, ObjTexCoord, ObjVertex};
use crate::ObjSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{DiffBinary,DiffCodec,DiffText};
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use std::collections::{BTreeSet, HashMap, HashSet};
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use crate::schema::snapshot::ObjFaceVertex;
use crate::schema::snapshot::ObjSmoothingRange;
use crate::schema::snapshot::ObjUnknownStatement;
use crate::schema::snapshot::ObjUsemtlRange;

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
pub(crate) fn fmt_f64(v: f64) -> String {
    v.to_string()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_f64(s: &str) -> Result<f64, String> {
    s.parse().map_err(|e: std::num::ParseFloatError| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_u32(s: &str) -> Result<u32, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
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
pub(crate) fn enc_vertex(v: &ObjVertex) -> String {
    format!("[{},{},{},{}]", fmt_f64(v.x), fmt_f64(v.y), fmt_f64(v.z), encode_option(&v.w, |w| fmt_f64(*w)))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_vertex(s: &str) -> Result<ObjVertex, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [x, y, z, w] = parts.as_slice() else { return Err(format!("vertex: expected 4 fields, got {}", parts.len())) };
    Ok(ObjVertex { x: parse_f64(x)?, y: parse_f64(y)?, z: parse_f64(z)?, w: decode_option(w, parse_f64)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_texcoord(t: &ObjTexCoord) -> String {
    format!("[{},{},{}]", fmt_f64(t.u), fmt_f64(t.v), encode_option(&t.w, |w| fmt_f64(*w)))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_texcoord(s: &str) -> Result<ObjTexCoord, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [u, v, w] = parts.as_slice() else { return Err(format!("texcoord: expected 3 fields, got {}", parts.len())) };
    Ok(ObjTexCoord { u: parse_f64(u)?, v: parse_f64(v)?, w: decode_option(w, parse_f64)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_normal(n: &ObjNormal) -> String {
    format!("[{},{},{}]", fmt_f64(n.x), fmt_f64(n.y), fmt_f64(n.z))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_normal(s: &str) -> Result<ObjNormal, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [x, y, z] = parts.as_slice() else { return Err(format!("normal: expected 3 fields, got {}", parts.len())) };
    Ok(ObjNormal { x: parse_f64(x)?, y: parse_f64(y)?, z: parse_f64(z)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_face_vertex(fv: &ObjFaceVertex) -> String {
    format!("[{},{},{}]", fv.vertex, encode_option(&fv.texcoord, |v| v.to_string()), encode_option(&fv.normal, |v| v.to_string()))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_face_vertex(s: &str) -> Result<ObjFaceVertex, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [vertex, texcoord, normal] = parts.as_slice() else { return Err(format!("face vertex: expected 3 fields, got {}", parts.len())) };
    Ok(ObjFaceVertex { vertex: parse_u32(vertex)?, texcoord: decode_option(texcoord, parse_u32)?, normal: decode_option(normal, parse_u32)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_face(f: &ObjFace) -> String {
    format!("[{}]", f.vertices.iter().map(enc_face_vertex).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_face(s: &str) -> Result<ObjFace, String> {
    let inner = strip_brackets(s)?;
    let vertices = split_top_level(inner, ',').into_iter().filter(|s| !s.is_empty()).map(dec_face_vertex).collect::<Result<Vec<_>, String>>()?;
    Ok(ObjFace { vertices })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_group(g: &ObjGroup) -> String {
    format!("[{},[{}]]", hex_encode_str(&g.name), g.faces.iter().map(|f| f.to_string()).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_group(s: &str) -> Result<ObjGroup, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name_hex, faces_s] = parts.as_slice() else { return Err(format!("group: expected 2 fields, got {}", parts.len())) };
    let faces = split_top_level(strip_brackets(faces_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_u64).collect::<Result<Vec<_>, String>>()?;
    Ok(ObjGroup { name: hex_decode_str(name_hex)?, faces })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_object(o: &ObjObject) -> String {
    format!("[{},[{}]]", hex_encode_str(&o.name), o.faces.iter().map(|f| f.to_string()).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_object(s: &str) -> Result<ObjObject, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name_hex, faces_s] = parts.as_slice() else { return Err(format!("object: expected 2 fields, got {}", parts.len())) };
    let faces = split_top_level(strip_brackets(faces_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_u64).collect::<Result<Vec<_>, String>>()?;
    Ok(ObjObject { name: hex_decode_str(name_hex)?, faces })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_usemtl(u: &ObjUsemtlRange) -> String {
    format!("[{},{}]", u.face_index_from, hex_encode_str(&u.material))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_usemtl(s: &str) -> Result<ObjUsemtlRange, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [idx, mat] = parts.as_slice() else { return Err(format!("usemtl: expected 2 fields, got {}", parts.len())) };
    Ok(ObjUsemtlRange { face_index_from: parse_u64(idx)?, material: hex_decode_str(mat)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_smoothing(sg: &ObjSmoothingRange) -> String {
    format!("[{},{}]", sg.face_index_from, encode_option(&sg.group, |g| g.to_string()))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_smoothing(s: &str) -> Result<ObjSmoothingRange, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [idx, grp] = parts.as_slice() else { return Err(format!("smoothing: expected 2 fields, got {}", parts.len())) };
    Ok(ObjSmoothingRange { face_index_from: parse_u64(idx)?, group: decode_option(grp, parse_u32)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_unknown(u: &ObjUnknownStatement) -> String {
    format!("[{},{}]", u.line_index, hex_encode_str(&u.raw))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_unknown(s: &str) -> Result<ObjUnknownStatement, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [idx, raw] = parts.as_slice() else { return Err(format!("unknown: expected 2 fields, got {}", parts.len())) };
    Ok(ObjUnknownStatement { line_index: idx.parse::<u64>().map_err(|error| error.to_string())?, raw: hex_decode_str(raw)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_vertex_diff(d: &ObjVertexDiff) -> String {
    let mut parts = Vec::new();
    if let Some(v) = d.x {
        parts.push(format!("X:{}", fmt_f64(v)));
    }
    if let Some(v) = d.y {
        parts.push(format!("Y:{}", fmt_f64(v)));
    }
    if let Some(v) = d.z {
        parts.push(format!("Z:{}", fmt_f64(v)));
    }
    if let Some(v) = d.w {
        parts.push(format!("W:{}", encode_option(&v, |w| fmt_f64(*w))));
    }
    format!("[{}]", parts.join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_vertex_diff(s: &str) -> Result<ObjVertexDiff, String> {
    let inner = strip_brackets(s)?;
    let mut d = ObjVertexDiff::default();
    for entry in split_top_level(inner, ',') {
        if entry.is_empty() {
            continue;
        }
        let (tag, val) = entry.split_once(':').ok_or_else(|| format!("vertex diff: bad entry {entry:?}"))?;
        match tag {
            "X" => d.x = Some(parse_f64(val)?),
            "Y" => d.y = Some(parse_f64(val)?),
            "Z" => d.z = Some(parse_f64(val)?),
            "W" => d.w = Some(decode_option(val, parse_f64)?),
            other => return Err(format!("vertex diff: unknown tag {other:?}")),
        }
    }
    Ok(d)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_texcoord_diff(d: &ObjTexCoordDiff) -> String {
    let mut parts = Vec::new();
    if let Some(v) = d.u {
        parts.push(format!("U:{}", fmt_f64(v)));
    }
    if let Some(v) = d.v {
        parts.push(format!("V:{}", fmt_f64(v)));
    }
    if let Some(v) = d.w {
        parts.push(format!("W:{}", encode_option(&v, |w| fmt_f64(*w))));
    }
    format!("[{}]", parts.join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_texcoord_diff(s: &str) -> Result<ObjTexCoordDiff, String> {
    let inner = strip_brackets(s)?;
    let mut d = ObjTexCoordDiff::default();
    for entry in split_top_level(inner, ',') {
        if entry.is_empty() {
            continue;
        }
        let (tag, val) = entry.split_once(':').ok_or_else(|| format!("texcoord diff: bad entry {entry:?}"))?;
        match tag {
            "U" => d.u = Some(parse_f64(val)?),
            "V" => d.v = Some(parse_f64(val)?),
            "W" => d.w = Some(decode_option(val, parse_f64)?),
            other => return Err(format!("texcoord diff: unknown tag {other:?}")),
        }
    }
    Ok(d)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_normal_diff(d: &ObjNormalDiff) -> String {
    let mut parts = Vec::new();
    if let Some(v) = d.x {
        parts.push(format!("X:{}", fmt_f64(v)));
    }
    if let Some(v) = d.y {
        parts.push(format!("Y:{}", fmt_f64(v)));
    }
    if let Some(v) = d.z {
        parts.push(format!("Z:{}", fmt_f64(v)));
    }
    format!("[{}]", parts.join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_normal_diff(s: &str) -> Result<ObjNormalDiff, String> {
    let inner = strip_brackets(s)?;
    let mut d = ObjNormalDiff::default();
    for entry in split_top_level(inner, ',') {
        if entry.is_empty() {
            continue;
        }
        let (tag, val) = entry.split_once(':').ok_or_else(|| format!("normal diff: bad entry {entry:?}"))?;
        match tag {
            "X" => d.x = Some(parse_f64(val)?),
            "Y" => d.y = Some(parse_f64(val)?),
            "Z" => d.z = Some(parse_f64(val)?),
            other => return Err(format!("normal diff: unknown tag {other:?}")),
        }
    }
    Ok(d)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_face_diff(d: &ObjFaceDiff) -> String {
    let mut parts = Vec::new();
    if let Some(v) = &d.vertices {
        parts.push(format!("V:[{}]", v.iter().map(enc_face_vertex).collect::<Vec<_>>().join(",")));
    }
    format!("[{}]", parts.join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_face_diff(s: &str) -> Result<ObjFaceDiff, String> {
    let inner = strip_brackets(s)?;
    let mut d = ObjFaceDiff::default();
    for entry in split_top_level(inner, ',') {
        if entry.is_empty() {
            continue;
        }
        let (tag, val) = entry.split_once(':').ok_or_else(|| format!("face diff: bad entry {entry:?}"))?;
        match tag {
            "V" => {
                let items = split_top_level(strip_brackets(val)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_face_vertex).collect::<Result<Vec<_>, String>>()?;
                d.vertices = Some(items);
            }
            other => return Err(format!("face diff: unknown tag {other:?}")),
        }
    }
    Ok(d)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_group_diff(d: &ObjGroupDiff) -> String {
    let mut parts = Vec::new();
    if let Some(v) = &d.faces {
        parts.push(format!("F:[{}]", v.iter().map(|f| f.to_string()).collect::<Vec<_>>().join(",")));
    }
    format!("[{}]", parts.join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_group_diff(s: &str) -> Result<ObjGroupDiff, String> {
    let inner = strip_brackets(s)?;
    let mut d = ObjGroupDiff::default();
    for entry in split_top_level(inner, ',') {
        if entry.is_empty() {
            continue;
        }
        let (tag, val) = entry.split_once(':').ok_or_else(|| format!("group diff: bad entry {entry:?}"))?;
        match tag {
            "F" => {
                let faces = split_top_level(strip_brackets(val)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_u64).collect::<Result<Vec<_>, String>>()?;
                d.faces = Some(faces);
            }
            other => return Err(format!("group diff: unknown tag {other:?}")),
        }
    }
    Ok(d)
}

/// 🧭️ Generic-shaped 3-section `[removed];[modified];[added]` index-keyed collection-triple
/// printer/parser (mirrors gif89a's `enc_collection_triple`/`dec_collection_triple`), hand-
/// instantiated per item type below.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_index_triple(name: &str, removed: &[usize], modified: &[(usize, String)], added: &[(usize, String)]) -> String {
    let removed = removed.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let modified = modified.iter().map(|(i, v)| format!("{i}:{v}")).collect::<Vec<_>>().join(",");
    let added = added.iter().map(|(i, v)| format!("{i}:{v}")).collect::<Vec<_>>().join(",");
    format!("{name}{{[{removed}];[{modified}];[{added}]}}")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_index_triple(body: &str) -> Result<IndexedDiffParts<String, String>, String> {
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

/// 🧭️ Same shape, name-keyed (`removed: Vec<String>`) — for `groups`/`objects`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_named_triple(name: &str, removed: &[String], modified: &[(String, String)], added: &[(usize, String)]) -> String {
    let removed = removed.iter().map(|n| hex_encode_str(n)).collect::<Vec<_>>().join(",");
    let modified = modified.iter().map(|(n, v)| format!("{}:{v}", hex_encode_str(n))).collect::<Vec<_>>().join(",");
    let added = added.iter().map(|(i, v)| format!("{i}:{v}")).collect::<Vec<_>>().join(",");
    format!("{name}{{[{removed}];[{modified}];[{added}]}}")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_named_triple(body: &str) -> Result<NamedDiffParts<String, String>, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("named collection: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(hex_decode_str).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (name_hex, rest) = entry.split_once(':').ok_or_else(|| format!("named collection modified: bad entry {entry:?}"))?;
            Ok((hex_decode_str(name_hex)?, rest.to_string()))
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("named collection added: bad entry {entry:?}"))?;
            Ok((parse_usize(idx)?, rest.to_string()))
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok((removed, modified, added))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_vertices_diff(d: &ObjVerticesDiff) -> String {
    enc_index_triple("vertices", &d.removed, &d.modified.iter().map(|m| (m.index, enc_vertex_diff(&m.diff))).collect::<Vec<_>>(), &d.added.iter().map(|a| (a.index, enc_vertex(&a.vertex))).collect::<Vec<_>>())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_vertices_diff(body: &str) -> Result<ObjVerticesDiff, String> {
    let (removed, modified, added) = dec_index_triple(body)?;
    Ok(ObjVerticesDiff {
        removed,
        modified: modified.into_iter().map(|(index, enc)| Ok(ObjVertexModified { index, diff: dec_vertex_diff(&enc)? })).collect::<Result<Vec<_>, String>>()?,
        added: added.into_iter().map(|(index, enc)| Ok(ObjVertexAdded { index, vertex: dec_vertex(&enc)? })).collect::<Result<Vec<_>, String>>()?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_texcoords_diff(d: &ObjTexCoordsDiff) -> String {
    enc_index_triple("texcoords", &d.removed, &d.modified.iter().map(|m| (m.index, enc_texcoord_diff(&m.diff))).collect::<Vec<_>>(), &d.added.iter().map(|a| (a.index, enc_texcoord(&a.texcoord))).collect::<Vec<_>>())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_texcoords_diff(body: &str) -> Result<ObjTexCoordsDiff, String> {
    let (removed, modified, added) = dec_index_triple(body)?;
    Ok(ObjTexCoordsDiff {
        removed,
        modified: modified.into_iter().map(|(index, enc)| Ok(ObjTexCoordModified { index, diff: dec_texcoord_diff(&enc)? })).collect::<Result<Vec<_>, String>>()?,
        added: added.into_iter().map(|(index, enc)| Ok(ObjTexCoordAdded { index, texcoord: dec_texcoord(&enc)? })).collect::<Result<Vec<_>, String>>()?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_normals_diff(d: &ObjNormalsDiff) -> String {
    enc_index_triple("normals", &d.removed, &d.modified.iter().map(|m| (m.index, enc_normal_diff(&m.diff))).collect::<Vec<_>>(), &d.added.iter().map(|a| (a.index, enc_normal(&a.normal))).collect::<Vec<_>>())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_normals_diff(body: &str) -> Result<ObjNormalsDiff, String> {
    let (removed, modified, added) = dec_index_triple(body)?;
    Ok(ObjNormalsDiff {
        removed,
        modified: modified.into_iter().map(|(index, enc)| Ok(ObjNormalModified { index, diff: dec_normal_diff(&enc)? })).collect::<Result<Vec<_>, String>>()?,
        added: added.into_iter().map(|(index, enc)| Ok(ObjNormalAdded { index, normal: dec_normal(&enc)? })).collect::<Result<Vec<_>, String>>()?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_faces_diff(d: &ObjFacesDiff) -> String {
    enc_index_triple("faces", &d.removed, &d.modified.iter().map(|m| (m.index, enc_face_diff(&m.diff))).collect::<Vec<_>>(), &d.added.iter().map(|a| (a.index, enc_face(&a.face))).collect::<Vec<_>>())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_faces_diff(body: &str) -> Result<ObjFacesDiff, String> {
    let (removed, modified, added) = dec_index_triple(body)?;
    Ok(ObjFacesDiff {
        removed,
        modified: modified.into_iter().map(|(index, enc)| Ok(ObjFaceModified { index, diff: dec_face_diff(&enc)? })).collect::<Result<Vec<_>, String>>()?,
        added: added.into_iter().map(|(index, enc)| Ok(ObjFaceAdded { index, face: dec_face(&enc)? })).collect::<Result<Vec<_>, String>>()?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_groups_diff(d: &ObjGroupsDiff) -> String {
    enc_named_triple("groups", &d.removed, &d.modified.iter().map(|m| (m.name.clone(), enc_group_diff(&m.diff))).collect::<Vec<_>>(), &d.added.iter().map(|a| (a.index, enc_group(&a.group))).collect::<Vec<_>>())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_groups_diff(body: &str) -> Result<ObjGroupsDiff, String> {
    let (removed, modified, added) = dec_named_triple(body)?;
    Ok(ObjGroupsDiff {
        removed,
        modified: modified.into_iter().map(|(name, enc)| Ok(ObjGroupModified { name, diff: dec_group_diff(&enc)? })).collect::<Result<Vec<_>, String>>()?,
        added: added.into_iter().map(|(index, enc)| Ok(ObjGroupAdded { index, group: dec_group(&enc)? })).collect::<Result<Vec<_>, String>>()?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_objects_diff(d: &ObjObjectsDiff) -> String {
    enc_named_triple("objects", &d.removed, &d.modified.iter().map(|m| (m.name.clone(), enc_group_diff(&m.diff))).collect::<Vec<_>>(), &d.added.iter().map(|a| (a.index, enc_object(&a.object))).collect::<Vec<_>>())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_objects_diff(body: &str) -> Result<ObjObjectsDiff, String> {
    let (removed, modified, added) = dec_named_triple(body)?;
    Ok(ObjObjectsDiff {
        removed,
        modified: modified.into_iter().map(|(name, enc)| Ok(ObjGroupModified { name, diff: dec_group_diff(&enc)? })).collect::<Result<Vec<_>, String>>()?,
        added: added.into_iter().map(|(index, enc)| Ok(ObjObjectAdded { index, object: dec_object(&enc)? })).collect::<Result<Vec<_>, String>>()?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_obj_diff(d: &ObjDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = &d.vertices {
        tokens.push(enc_vertices_diff(v));
    }
    if let Some(v) = &d.texcoords {
        tokens.push(enc_texcoords_diff(v));
    }
    if let Some(v) = &d.normals {
        tokens.push(enc_normals_diff(v));
    }
    if let Some(v) = &d.faces {
        tokens.push(enc_faces_diff(v));
    }
    if let Some(v) = &d.groups {
        tokens.push(enc_groups_diff(v));
    }
    if let Some(v) = &d.objects {
        tokens.push(enc_objects_diff(v));
    }
    if let Some(v) = &d.mtllib {
        tokens.push(format!("mtllib={}", encode_option(v, |s| hex_encode_str(s))));
    }
    if let Some(v) = &d.usemtl {
        tokens.push(format!("usemtl=[{}]", v.iter().map(enc_usemtl).collect::<Vec<_>>().join(",")));
    }
    if let Some(v) = &d.smoothing_groups {
        tokens.push(format!("smoothing=[{}]", v.iter().map(enc_smoothing).collect::<Vec<_>>().join(",")));
    }
    if let Some(v) = &d.unknown_statements {
        tokens.push(format!("unknown=[{}]", v.iter().map(enc_unknown).collect::<Vec<_>>().join(",")));
    }
    tokens.join(" ")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_obj_diff(line: &str) -> Result<ObjDiff, String> {
    let mut d = ObjDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("vertices{") {
            d.vertices = Some(dec_vertices_diff(rest.strip_suffix('}').ok_or_else(|| "vertices: missing closing brace".to_string())?)?);
        } else if let Some(rest) = token.strip_prefix("texcoords{") {
            d.texcoords = Some(dec_texcoords_diff(rest.strip_suffix('}').ok_or_else(|| "texcoords: missing closing brace".to_string())?)?);
        } else if let Some(rest) = token.strip_prefix("normals{") {
            d.normals = Some(dec_normals_diff(rest.strip_suffix('}').ok_or_else(|| "normals: missing closing brace".to_string())?)?);
        } else if let Some(rest) = token.strip_prefix("faces{") {
            d.faces = Some(dec_faces_diff(rest.strip_suffix('}').ok_or_else(|| "faces: missing closing brace".to_string())?)?);
        } else if let Some(rest) = token.strip_prefix("groups{") {
            d.groups = Some(dec_groups_diff(rest.strip_suffix('}').ok_or_else(|| "groups: missing closing brace".to_string())?)?);
        } else if let Some(rest) = token.strip_prefix("objects{") {
            d.objects = Some(dec_objects_diff(rest.strip_suffix('}').ok_or_else(|| "objects: missing closing brace".to_string())?)?);
        } else if let Some(rest) = token.strip_prefix("mtllib=") {
            d.mtllib = Some(decode_option(rest, hex_decode_str)?);
        } else if let Some(rest) = token.strip_prefix("usemtl=") {
            d.usemtl = Some(split_top_level(strip_brackets(rest)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_usemtl).collect::<Result<Vec<_>, String>>()?);
        } else if let Some(rest) = token.strip_prefix("smoothing=") {
            d.smoothing_groups = Some(split_top_level(strip_brackets(rest)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_smoothing).collect::<Result<Vec<_>, String>>()?);
        } else if let Some(rest) = token.strip_prefix("unknown=") {
            d.unknown_statements = Some(split_top_level(strip_brackets(rest)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_unknown).collect::<Result<Vec<_>, String>>()?);
        } else {
            return Err(format!("obj diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

pub(crate) fn parse_u64(value:&str)->Result<u64,String>{value.parse::<u64>().map_err(|error|error.to_string())}

impl protocol::DiffText for ObjDiff {
fn print_diff(&self) -> String {
    print_obj_diff(self)
}
fn parse_diff(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
    parse_obj_diff(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
}
}
}
pub use diff_codec::*;
