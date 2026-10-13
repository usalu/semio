//! 🔤️ Text codec primitives of the semio family: bracketed lists and options, hex strings, the indexed and named
//! diff triples, and the geometry records in their native (exact round-trip) spelling; `plain` holds the Rust
//! `Display` spelling the subset diff codecs use.

use crate::schema::triples::*;
use crate::schema::geometry::native::{NativeF32, NativeF64};
use crate::schema::geometry::{SemioPoint2, SemioPoint3, SemioQuaternion, SemioRgba, SemioTransform};

/// 📐️ Bracket-depth-aware split — a `sep` inside `[...]` never splits (so a modified/added
/// entry's own nested `[...]` payload survives the outer `;`/`,` split intact).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn split_top_level(s: &str, sep: char) -> Vec<&str> {
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
pub fn strip_brackets(s: &str) -> Result<&str, String> {
    s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))
}

/// 🧮️ Optional sparse-diff field: `_` when absent, otherwise `s` followed by the field's own encoding.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_opt<T>(value: Option<&T>, enc: impl Fn(&T) -> String) -> String {
    value.map_or_else(|| "_".to_string(), |v| format!("s{}", enc(v)))
}

/// 🧮️ Inverse of [`enc_opt`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_opt<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Option<T>, String> {
    match s {
        "_" => Ok(None),
        other => other.strip_prefix('s').ok_or_else(|| format!("optional field: bad token {other:?}")).and_then(dec).map(Some),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_indexed_triple<D, T>(diff: &IndexedTripleDiff<D, T>, enc_d: impl Fn(&D) -> String, enc_t: impl Fn(&T) -> String) -> String {
    let removed = diff.removed.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let modified = diff.modified.iter().map(|m| format!("{}:{}", m.index, enc_d(&m.diff))).collect::<Vec<_>>().join(",");
    let added = diff.added.iter().map(|a| format!("{}:{}", a.index, enc_t(&a.item))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_indexed_triple<D, T>(body: &str, dec_d: impl Fn(&str) -> Result<D, String>, dec_t: impl Fn(&str) -> Result<T, String>) -> Result<IndexedTripleDiff<D, T>, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("indexed triple: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|s| s.parse::<usize>().map_err(|e: std::num::ParseIntError| e.to_string())).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("indexed modified: bad entry {entry:?}"))?;
            Ok(IndexModified { index: idx.parse::<usize>().map_err(|e: std::num::ParseIntError| e.to_string())?, diff: dec_d(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("indexed added: bad entry {entry:?}"))?;
            Ok(IndexAdded { index: idx.parse::<usize>().map_err(|e: std::num::ParseIntError| e.to_string())?, item: dec_t(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(IndexedTripleDiff { removed, modified, added })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_named_triple<K, D, T>(triple: &NamedTripleDiff<K, D, T>, enc_k: impl Fn(&K) -> String, enc_d: impl Fn(&D) -> String, enc_t: impl Fn(&T) -> String) -> String {
    let removed = triple.removed.iter().map(&enc_k).collect::<Vec<_>>().join(",");
    let modified = triple.modified.iter().map(|m| format!("{}:{}", enc_k(&m.key), enc_d(&m.diff))).collect::<Vec<_>>().join(",");
    let added = triple.added.iter().map(enc_t).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_named_triple<K, D, T>(s: &str, dec_k: impl Fn(&str) -> Result<K, String>, dec_d: impl Fn(&str) -> Result<D, String>, dec_t: impl Fn(&str) -> Result<T, String>) -> Result<NamedTripleDiff<K, D, T>, String> {
    let three = split_top_level(s, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("named triple: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(&dec_k).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (k, rest) = entry.split_once(':').ok_or_else(|| format!("named triple modified: bad entry {entry:?}"))?;
            Ok(NamedModified { key: dec_k(k)?, diff: dec_d(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_t).collect::<Result<Vec<_>, String>>()?;
    Ok(NamedTripleDiff { removed, modified, added })
}

/// 🧷 `NamedAdded<T>`-wrapping encode/decode pair (`index:item`, same `idx:rest` shape
/// `enc_indexed_triple`/`dec_indexed_triple` already use for `IndexAdded<T>` above) — for a
/// consumer instantiating `NamedTripleDiff<K, D, NamedAdded<T>>`, pass
/// `|a| enc_named_added(a, enc_t)`/`|s| dec_named_added(s, dec_t)` as `enc_named_triple`'s/
/// `dec_named_triple`'s own `enc_t`/`dec_t` argument.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_named_added<T>(a: &NamedAdded<T>, enc_t: impl Fn(&T) -> String) -> String {
    format!("{}:{}", a.index, enc_t(&a.item))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_named_added<T>(s: &str, dec_t: impl Fn(&str) -> Result<T, String>) -> Result<NamedAdded<T>, String> {
    let (idx, rest) = s.split_once(':').ok_or_else(|| format!("named added: bad entry {s:?}"))?;
    Ok(NamedAdded { index: idx.parse::<usize>().map_err(|e: std::num::ParseIntError| e.to_string())?, item: dec_t(rest)? })
}

/// 🧪️ ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION drawing wave (following the
/// flow pilot's proven template, `ws-codec-workflow-report.md`, and brep's own tagged-enum
/// precedent, `ws-codec-brep-report.md`): real hex/bracket-encoded value primitives backing the
/// hand-rolled `ArtifactDsl` below — replaces the old hex-of-`serde_json` passthrough.
///
/// 🧩️ The `#[derive(dsl::DslArtifact)]` path was reconsidered now that the 6 shared
/// `⚙️engine/🧮️geometry` value types derive `dsl::DslRecord`. Still blocked here: `PathSegment` and
/// `DrawNode` are data-carrying TAGGED ENUMS whose variants hold different field sets (matching
/// brep's `BrepCurve`/`BrepSurface` blocker exactly), and `DrawNode` is additionally RECURSIVE
/// (`Group.children: Vec<DrawNode>`) — no `DslEnum`-over-heterogeneous-recursive-payload mechanism
/// exists. Hand-rolled instead, single-letter tag prefix per variant (same convention brep's
/// `enc_curve`/`enc_surface` established), reused verbatim by the sibling `🔺️diff`/`🧬️mutations`
/// facets (`pub ` below) rather than re-derived three times.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if !s.len().is_multiple_of(2) {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_f64(s: &str) -> Result<f64, String> {
    crate::schema::geometry::native::parse(s)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_f32(s: &str) -> Result<f32, String> {
    crate::schema::geometry::native::parse32(s)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_bool(b: bool) -> &'static str {
    if b {
        "1"
    } else {
        "0"
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_bool(s: &str) -> Result<bool, String> {
    match s {
        "1" => Ok(true),
        "0" => Ok(false),
        other => Err(format!("bad bool {other:?}")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_list<T>(items: &[T], enc: impl Fn(&T) -> String) -> String {
    format!("[{}]", items.iter().map(enc).collect::<Vec<_>>().join(","))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_list<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Vec<T>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec).collect()
}
/// 🏳️ Single-state option: `[0]` = `None`, `[1,<value>]` = `Some(value)` — used by snapshot-level
/// `Option<T>` fields (never tri-state; tri-state `Option<Option<T>>` is a `🔺️diff`-only concept).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_option<T>(opt: &Option<T>, enc: impl Fn(&T) -> String) -> String {
    match opt {
        None => "[0]".to_string(),
        Some(v) => format!("[1,{}]", enc(v)),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_option<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Option<T>, String> {
    let inner = strip_brackets(s)?;
    match split_top_level(inner, ',').as_slice() {
        ["0"] => Ok(None),
        [tag, value] if *tag == "1" => Ok(Some(dec(value)?)),
        other => Err(format!("option decode: bad shape {other:?}")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_point2(p: &SemioPoint2) -> String {
    format!("[{},{}]", NativeF64(p.x), NativeF64(p.y))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_point2(s: &str) -> Result<SemioPoint2, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [x, y] = parts.as_slice() else { return Err(format!("point2: expected 2 fields, got {}", parts.len())) };
    Ok(SemioPoint2 { x: parse_f64(x)?, y: parse_f64(y)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_point3(p: &SemioPoint3) -> String {
    format!("[{},{},{}]", NativeF64(p.x), NativeF64(p.y), NativeF64(p.z))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_point3(s: &str) -> Result<SemioPoint3, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [x, y, z] = parts.as_slice() else { return Err(format!("point3: expected 3 fields, got {}", parts.len())) };
    Ok(SemioPoint3 { x: parse_f64(x)?, y: parse_f64(y)?, z: parse_f64(z)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_quaternion(q: &SemioQuaternion) -> String {
    format!("[{},{},{},{}]", NativeF64(q.x), NativeF64(q.y), NativeF64(q.z), NativeF64(q.w))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_quaternion(s: &str) -> Result<SemioQuaternion, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [x, y, z, w] = parts.as_slice() else { return Err(format!("quaternion: expected 4 fields, got {}", parts.len())) };
    Ok(SemioQuaternion { x: parse_f64(x)?, y: parse_f64(y)?, z: parse_f64(z)?, w: parse_f64(w)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_transform(t: &SemioTransform) -> String {
    format!("[{},{},{}]", enc_point3(&t.translation), enc_quaternion(&t.rotation), enc_point3(&t.scale))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_transform(s: &str) -> Result<SemioTransform, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [translation, rotation, scale] = parts.as_slice() else { return Err(format!("transform: expected 3 fields, got {}", parts.len())) };
    Ok(SemioTransform { translation: dec_point3(translation)?, rotation: dec_quaternion(rotation)?, scale: dec_point3(scale)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_rgba(c: &SemioRgba) -> String {
    format!("[{},{},{},{}]", NativeF32(c.r), NativeF32(c.g), NativeF32(c.b), NativeF32(c.a))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_rgba(s: &str) -> Result<SemioRgba, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [r, g, b, a] = parts.as_slice() else { return Err(format!("rgba: expected 4 fields, got {}", parts.len())) };
    Ok(SemioRgba { r: parse_f32(r)?, g: parse_f32(g)?, b: parse_f32(b)?, a: parse_f32(a)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_f64(v: f64) -> String {
    if v.is_nan(){format!("nan64_{:016x}",v.to_bits())}else{v.to_string()}
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_f64(s: &str) -> Result<f64, String> {
    if let Some(word)=s.strip_prefix("nan64_"){if word.len()!=16||!word.bytes().all(|byte|byte.is_ascii_hexdigit()){return Err("invalid Flow binary64 NaN word".into())}let bits=u64::from_str_radix(word,16).map_err(|error|error.to_string())?;let value=f64::from_bits(bits);if !value.is_nan(){return Err("Flow binary64 NaN word has a non-NaN class".into())}return Ok(value)}
    s.parse().map_err(|e: std::num::ParseFloatError| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_bool(s: &str) -> Result<bool, String> {
    match s {
        "1" => Ok(true),
        "0" => Ok(false),
        other => Err(format!("bool: bad value {other:?}")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_u32(s: &str) -> Result<u32, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_usize(s: &str) -> Result<usize, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_ref(r: &semio_framework_artifact_reference::ArtifactRef) -> String {
    format!("[{},{},{},{}]",enc_str(&r.artifact_id),enc_str(&r.dialect.artifact_kind),enc_str(&r.dialect.standard),enc_str(&r.dialect.subset))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_ref(s: &str) -> Result<semio_framework_artifact_reference::ArtifactRef, String> {
    let parts=split_top_level(strip_brackets(s)?,',');
    let[id,kind,standard,subset]=parts.as_slice()else{return Err("reference requires four literal fields".into())};
    Ok(semio_framework_artifact_reference::ArtifactRef{artifact_id:dec_str(id)?,dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:dec_str(kind)?,standard:dec_str(standard)?,subset:dec_str(subset)?}})
}
/// 🪪️ One literal child identity and four literal reference fields.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_child<S>(c: &store::os_store::artifact_child::ArtifactChild<S>) -> String {
    format!("[{},{}]", enc_str(&c.child_id), enc_ref(&c.target))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_child<S>(s: &str) -> Result<store::os_store::artifact_child::ArtifactChild<S>, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [child_id, target] = parts.as_slice() else { return Err(format!("child handle: expected 2 fields, got {}", parts.len())) };
    Ok(store::os_store::artifact_child::ArtifactChild::new(dec_str(child_id)?, dec_ref(target)?))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_child_opt<S>(c: &Option<store::os_store::artifact_child::ArtifactChild<S>>) -> String {
    match c {
        Some(c) => enc_child(c),
        None => "[]".to_string(),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_child_opt<S>(s: &str) -> Result<Option<store::os_store::artifact_child::ArtifactChild<S>>, String> {
    if s == "[]" {
        return Ok(None);
    }
    Ok(Some(dec_child(s)?))
}

/// ➖️ The `Display` spelling of the geometry records and scalars, as the subset diff codecs print and parse them.
pub mod plain {
use super::{split_top_level, strip_brackets};
use crate::schema::geometry::{SemioPoint2, SemioPoint3, SemioQuaternion, SemioRgba, SemioTransform};

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_f64(s: &str) -> Result<f64, String> {
    s.parse().map_err(|e: std::num::ParseFloatError| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_point2(p: &SemioPoint2) -> String {
    format!("[{},{}]", p.x, p.y)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_point2(s: &str) -> Result<SemioPoint2, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [x, y] = parts.as_slice() else { return Err(format!("point2: expected 2 fields, got {}", parts.len())) };
    Ok(SemioPoint2 { x: parse_f64(x)?, y: parse_f64(y)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_f32(s: &str) -> Result<f32, String> {
    s.parse().map_err(|e: std::num::ParseFloatError| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_rgba(c: &SemioRgba) -> String {
    format!("[{},{},{},{}]", c.r, c.g, c.b, c.a)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_rgba(s: &str) -> Result<SemioRgba, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [r, g, b, a] = parts.as_slice() else { return Err(format!("rgba: expected 4 fields, got {}", parts.len())) };
    Ok(SemioRgba { r: parse_f32(r)?, g: parse_f32(g)?, b: parse_f32(b)?, a: parse_f32(a)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_point3(p: &SemioPoint3) -> String {
    format!("[{},{},{}]", p.x, p.y, p.z)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_point3(s: &str) -> Result<SemioPoint3, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [x, y, z] = parts.as_slice() else { return Err(format!("point3: expected 3 fields, got {}", parts.len())) };
    Ok(SemioPoint3 { x: parse_f64(x)?, y: parse_f64(y)?, z: parse_f64(z)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_quat(q: &SemioQuaternion) -> String {
    format!("[{},{},{},{}]", q.x, q.y, q.z, q.w)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_quat(s: &str) -> Result<SemioQuaternion, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [x, y, z, w] = parts.as_slice() else { return Err(format!("quaternion: expected 4 fields, got {}", parts.len())) };
    Ok(SemioQuaternion { x: parse_f64(x)?, y: parse_f64(y)?, z: parse_f64(z)?, w: parse_f64(w)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_transform(t: &SemioTransform) -> String {
    format!("[{},{},{}]", enc_point3(&t.translation), enc_quat(&t.rotation), enc_point3(&t.scale))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_transform(s: &str) -> Result<SemioTransform, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [translation, rotation, scale] = parts.as_slice() else { return Err(format!("transform: expected 3 fields, got {}", parts.len())) };
    Ok(SemioTransform { translation: dec_point3(translation)?, rotation: dec_quat(rotation)?, scale: dec_point3(scale)? })
}
}
