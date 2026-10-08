//! 📝️ Text representation codec surface for `stdio.bcf` (diff).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type BcfDiffText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v2_1::subsets::any::schema::diff::*;
use protocol::{DiffText,DiffBinary};
use crate::schema::snapshot::{BcfCamera, BcfColoring, BcfComment, BcfComponents, BcfPoint3, BcfRawPart, BcfTopic, BcfViewpoint, BcfVisibility};
use crate::BcfSnapshot;
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
pub(crate) fn enc_bytes(b: &[u8]) -> String {
    hex_encode(b)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_bytes(s: &str) -> Result<Vec<u8>, String> {
    hex_decode(s)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_f64(s: &str) -> Result<f64, String> {
    s.parse().map_err(|e: std::num::ParseFloatError| e.to_string())
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

/// 📋️ Bracketed comma-joined list -- the un-keyed sibling of `IndexedDiff`'s codec below, for
/// plain `Vec<T>` fields (`labels`, `exceptions`, `selection`, `coloring`, ...) that are
/// whole-value replaced rather than key-diffed.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_list<T>(items: &[T], enc: impl Fn(&T) -> String) -> String {
    format!("[{}]", items.iter().map(enc).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_list<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Vec<T>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec).collect()
}

/// 🏷️ Generic codec for the `IndexedDiff<T,D>` engine (this file's own §IndexedTriple region) --
/// `[removed];[modified];[added]`, semicolon-separated sections, each a comma-separated list;
/// `removed` entries are base indices, `modified` entries `index:diff` and `added` entries
/// `index:item` (colon-separated, unambiguous because the index is decimal digits). Written once,
/// generically, and instantiated per collection (`topics`/`comments`/`viewpoints`/`parts`) below.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_indexed_triple<T, D>(triple: &IndexedDiff<T, D>, enc_d: impl Fn(&D) -> String, enc_t: impl Fn(&T) -> String) -> String {
    let removed = triple.removed.iter().map(|index| index.to_string()).collect::<Vec<_>>().join(",");
    let modified = triple.modified.iter().map(|m| format!("{}:{}", m.index, enc_d(&m.diff))).collect::<Vec<_>>().join(",");
    let added = triple.added.iter().map(|a| format!("{}:{}", a.index, enc_t(&a.item))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_indexed_triple<T, D>(s: &str, dec_d: impl Fn(&str) -> Result<D, String>, dec_t: impl Fn(&str) -> Result<T, String>) -> Result<IndexedDiff<T, D>, String> {
    let three = split_top_level(s, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("indexed triple: expected 3 sections, got {}", three.len())) };
    let index_of = |text: &str| text.parse::<usize>().map_err(|e| format!("indexed triple: bad index {text:?}: {e}"));
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|text| index_of(&text)).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (index, rest) = entry.split_once(':').ok_or_else(|| format!("indexed triple modified: bad entry {entry:?}"))?;
            Ok(IndexedModified { index: index_of(index)?, diff: dec_d(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (index, rest) = entry.split_once(':').ok_or_else(|| format!("indexed triple added: bad entry {entry:?}"))?;
            Ok(IndexedAdded { index: index_of(index)?, item: dec_t(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(IndexedDiff { removed, modified, added })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_point3(p: &BcfPoint3) -> String {
    format!("[{},{},{}]", p.x, p.y, p.z)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_point3(s: &str) -> Result<BcfPoint3, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [x, y, z] = parts.as_slice() else { return Err(format!("point3: expected 3 fields, got {}", parts.len())) };
    Ok(BcfPoint3 { x: parse_f64(x)?, y: parse_f64(y)?, z: parse_f64(z)? })
}

/// 📷 `P[view_point,direction,up_vector,field_of_view]` (Perspective) / `O[...,view_to_world_scale]`
/// (Orthogonal) -- single-letter tag prefix, the `xs:choice` made concrete (same convention as
/// `enc_xml_node`'s `E`/`T`/`D`/`M`/`P` tags in svg's hand-rolled codec).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_camera(c: &BcfCamera) -> String {
    match c {
        BcfCamera::Perspective { view_point, direction, up_vector, field_of_view } => {
            format!("P[{},{},{},{}]", enc_point3(view_point), enc_point3(direction), enc_point3(up_vector), field_of_view)
        }
        BcfCamera::Orthogonal { view_point, direction, up_vector, view_to_world_scale } => {
            format!("O[{},{},{},{}]", enc_point3(view_point), enc_point3(direction), enc_point3(up_vector), view_to_world_scale)
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_camera(s: &str) -> Result<BcfCamera, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    let parts = split_top_level(inner, ',');
    let [view_point, direction, up_vector, last] = parts.as_slice() else { return Err(format!("camera: expected 4 fields, got {}", parts.len())) };
    match tag {
        "P" => Ok(BcfCamera::Perspective { view_point: dec_point3(view_point)?, direction: dec_point3(direction)?, up_vector: dec_point3(up_vector)?, field_of_view: parse_f64(last)? }),
        "O" => Ok(BcfCamera::Orthogonal { view_point: dec_point3(view_point)?, direction: dec_point3(direction)?, up_vector: dec_point3(up_vector)?, view_to_world_scale: parse_f64(last)? }),
        other => Err(format!("camera: unknown tag {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_visibility(v: &BcfVisibility) -> String {
    format!("[{},{}]", if v.default_visibility { "1" } else { "0" }, enc_list(&v.exceptions, |s: &String| enc_str(s)))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_visibility(s: &str) -> Result<BcfVisibility, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [default_visibility, exceptions] = parts.as_slice() else { return Err(format!("visibility: expected 2 fields, got {}", parts.len())) };
    Ok(BcfVisibility { default_visibility: *default_visibility == "1", exceptions: dec_list(exceptions, dec_str)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_coloring(c: &BcfColoring) -> String {
    format!("[{},{}]", enc_str(&c.color), enc_list(&c.components, |s: &String| enc_str(s)))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_coloring(s: &str) -> Result<BcfColoring, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [color, components] = parts.as_slice() else { return Err(format!("coloring: expected 2 fields, got {}", parts.len())) };
    Ok(BcfColoring { color: dec_str(color)?, components: dec_list(components, dec_str)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_components(c: &BcfComponents) -> String {
    format!("[{},{},{}]", enc_list(&c.selection, |s: &String| enc_str(s)), enc_visibility(&c.visibility), enc_list(&c.coloring, enc_coloring))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_components(s: &str) -> Result<BcfComponents, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [selection, visibility, coloring] = parts.as_slice() else { return Err(format!("components: expected 3 fields, got {}", parts.len())) };
    Ok(BcfComponents { selection: dec_list(selection, dec_str)?, visibility: dec_visibility(visibility)?, coloring: dec_list(coloring, dec_coloring)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_comment(c: &BcfComment) -> String {
    format!("[{},{},{},{},{}]", enc_str(&c.guid), enc_str(&c.date), enc_str(&c.author), enc_str(&c.text), encode_option(&c.viewpoint_ref, |v: &String| enc_str(v)),)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_comment(s: &str) -> Result<BcfComment, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [guid, date, author, text, viewpoint_ref] = parts.as_slice() else { return Err(format!("comment: expected 5 fields, got {}", parts.len())) };
    Ok(BcfComment { guid: dec_str(guid)?, date: dec_str(date)?, author: dec_str(author)?, text: dec_str(text)?, viewpoint_ref: decode_option(viewpoint_ref, dec_str)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_viewpoint(v: &BcfViewpoint) -> String {
    format!("[{},{},{},{}]", enc_str(&v.guid), encode_option(&v.camera, enc_camera), encode_option(&v.components, enc_components), encode_option(&v.snapshot, |b: &Vec<u8>| enc_bytes(b)),)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_viewpoint(s: &str) -> Result<BcfViewpoint, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [guid, camera, components, snapshot] = parts.as_slice() else { return Err(format!("viewpoint: expected 4 fields, got {}", parts.len())) };
    Ok(BcfViewpoint { guid: dec_str(guid)?, camera: decode_option(camera, dec_camera)?, components: decode_option(components, dec_components)?, snapshot: decode_option(snapshot, dec_bytes)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_topic(t: &BcfTopic) -> String {
    format!(
        "[{},{},{},{},{},{},{},{},{},{}]",
        enc_str(&t.guid),
        enc_str(&t.title),
        enc_str(&t.description),
        enc_str(&t.status),
        enc_str(&t.priority),
        enc_list(&t.labels, |s: &String| enc_str(s)),
        enc_str(&t.creation_date),
        enc_str(&t.creation_author),
        enc_list(&t.comments, enc_comment),
        enc_list(&t.viewpoints, enc_viewpoint),
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_topic(s: &str) -> Result<BcfTopic, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [guid, title, description, status, priority, labels, creation_date, creation_author, comments, viewpoints] = parts.as_slice() else {
        return Err(format!("topic: expected 10 fields, got {}", parts.len()));
    };
    Ok(BcfTopic {
        guid: dec_str(guid)?,
        title: dec_str(title)?,
        description: dec_str(description)?,
        status: dec_str(status)?,
        priority: dec_str(priority)?,
        labels: dec_list(labels, dec_str)?,
        creation_date: dec_str(creation_date)?,
        creation_author: dec_str(creation_author)?,
        comments: dec_list(comments, dec_comment)?,
        viewpoints: dec_list(viewpoints, dec_viewpoint)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_part(p: &BcfRawPart) -> String {
    format!("[{},{}]", enc_str(&p.name), enc_bytes(&p.data))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_part(s: &str) -> Result<BcfRawPart, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, data] = parts.as_slice() else { return Err(format!("part: expected 2 fields, got {}", parts.len())) };
    Ok(BcfRawPart { name: dec_str(name)?, data: dec_bytes(data)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_comment_diff(d: &BcfCommentDiff) -> String {
    format!(
        "[{},{},{},{}]",
        encode_option(&d.date, |v: &String| enc_str(v)),
        encode_option(&d.author, |v: &String| enc_str(v)),
        encode_option(&d.text, |v: &String| enc_str(v)),
        encode_option(&d.viewpoint_ref, |inner: &Option<String>| encode_option(inner, |v: &String| enc_str(v))),
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_comment_diff(s: &str) -> Result<BcfCommentDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [date, author, text, viewpoint_ref] = parts.as_slice() else { return Err(format!("comment diff: expected 4 fields, got {}", parts.len())) };
    Ok(BcfCommentDiff { date: decode_option(date, dec_str)?, author: decode_option(author, dec_str)?, text: decode_option(text, dec_str)?, viewpoint_ref: decode_option(viewpoint_ref, |s| decode_option(s, dec_str))? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_viewpoint_diff(d: &BcfViewpointDiff) -> String {
    format!(
        "[{},{},{}]",
        encode_option(&d.camera, |inner: &Option<BcfCamera>| encode_option(inner, enc_camera)),
        encode_option(&d.components, |inner: &Option<BcfComponents>| encode_option(inner, enc_components)),
        encode_option(&d.snapshot, |inner: &Option<Vec<u8>>| encode_option(inner, |b: &Vec<u8>| enc_bytes(b))),
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_viewpoint_diff(s: &str) -> Result<BcfViewpointDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [camera, components, snapshot] = parts.as_slice() else { return Err(format!("viewpoint diff: expected 3 fields, got {}", parts.len())) };
    Ok(BcfViewpointDiff { camera: decode_option(camera, |s| decode_option(s, dec_camera))?, components: decode_option(components, |s| decode_option(s, dec_components))?, snapshot: decode_option(snapshot, |s| decode_option(s, dec_bytes))? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_part_diff(d: &BcfPartDiff) -> String {
    format!("[{}]", encode_option(&d.data, |b: &Vec<u8>| enc_bytes(b)))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_part_diff(s: &str) -> Result<BcfPartDiff, String> {
    let inner = strip_brackets(s)?;
    Ok(BcfPartDiff { data: decode_option(inner, dec_bytes)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_topic_diff(d: &BcfTopicDiff) -> String {
    format!(
        "[{},{},{},{},{},{},{},{},{}]",
        encode_option(&d.title, |v: &String| enc_str(v)),
        encode_option(&d.description, |v: &String| enc_str(v)),
        encode_option(&d.status, |v: &String| enc_str(v)),
        encode_option(&d.priority, |v: &String| enc_str(v)),
        encode_option(&d.labels, |v: &Vec<String>| enc_list(v, |s| enc_str(s))),
        encode_option(&d.creation_date, |v: &String| enc_str(v)),
        encode_option(&d.creation_author, |v: &String| enc_str(v)),
        encode_option(&d.comments, |v: &BcfCommentsDiff| enc_indexed_triple(v, enc_comment_diff, enc_comment)),
        encode_option(&d.viewpoints, |v: &BcfViewpointsDiff| enc_indexed_triple(v, enc_viewpoint_diff, enc_viewpoint)),
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_topic_diff(s: &str) -> Result<BcfTopicDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [title, description, status, priority, labels, creation_date, creation_author, comments, viewpoints] = parts.as_slice() else {
        return Err(format!("topic diff: expected 9 fields, got {}", parts.len()));
    };
    Ok(BcfTopicDiff {
        title: decode_option(title, dec_str)?,
        description: decode_option(description, dec_str)?,
        status: decode_option(status, dec_str)?,
        priority: decode_option(priority, dec_str)?,
        labels: decode_option(labels, |s| dec_list(s, dec_str))?,
        creation_date: decode_option(creation_date, dec_str)?,
        creation_author: decode_option(creation_author, dec_str)?,
        comments: decode_option(comments, |s| dec_indexed_triple(s, dec_comment_diff, dec_comment))?,
        viewpoints: decode_option(viewpoints, |s| dec_indexed_triple(s, dec_viewpoint_diff, dec_viewpoint))?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_bcf_diff(d: &BcfDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = &d.version {
        tokens.push(format!("version={}", enc_str(v)));
    }
    if let Some(t) = &d.topics {
        tokens.push(format!("topics={}", enc_indexed_triple(t, enc_topic_diff, enc_topic)));
    }
    if let Some(p) = &d.parts {
        tokens.push(format!("parts={}", enc_indexed_triple(p, enc_part_diff, enc_part)));
    }
    tokens.join(" ")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_bcf_diff(line: &str) -> Result<BcfDiff, String> {
    let mut d = BcfDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("version=") {
            d.version = Some(dec_str(rest)?);
        } else if let Some(rest) = token.strip_prefix("topics=") {
            d.topics = Some(dec_indexed_triple(rest, dec_topic_diff, dec_topic)?);
        } else if let Some(rest) = token.strip_prefix("parts=") {
            d.parts = Some(dec_indexed_triple(rest, dec_part_diff, dec_part)?);
        } else {
            return Err(format!("bcf diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl protocol::DiffText for BcfDiff {
fn print_diff(&self) -> String {
    print_bcf_diff(self)
}
fn parse_diff(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
    parse_bcf_diff(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
}
}
}
pub use diff_codec::*;
