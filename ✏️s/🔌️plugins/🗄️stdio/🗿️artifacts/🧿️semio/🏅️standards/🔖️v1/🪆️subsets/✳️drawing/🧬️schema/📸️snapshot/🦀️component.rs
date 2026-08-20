//! 🧬️ SemioDrawingSnapshot — canvas + name-keyed styles + ordered layers, each a recursive
//! `DrawNode`{Path{segments}/Text/Group{transform,children}/Image} scene graph — from svg;
//! replaces DwgDrawing-as-neutral. Real, complete-per-spec-row shape (master plan "drawing" row):
//! no `serde_json::Value`, no bare tuples/nested fixed arrays (geometry fields reuse
//! `engine::geometry`'s named structs throughout).

use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::{SemioPoint2, SemioPoint3, SemioQuaternion, SemioRgba, SemioTransform};
use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::{split_top_level, strip_brackets};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️PathSegment
/// ✏️ A single SVG-style path command — the honest, complete production set for `Path.segments`
/// (no `*OCTET`/size-eos catch-all: every field a real drawn quantity).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum PathSegment {
    MoveTo {
        to: SemioPoint2,
    },
    LineTo {
        to: SemioPoint2,
    },
    CubicTo {
        c1: SemioPoint2,
        c2: SemioPoint2,
        to: SemioPoint2,
    },
    QuadTo {
        c: SemioPoint2,
        to: SemioPoint2,
    },
    /// 🌙️ Elliptical arc, SVG `A rx ry x-rotation large-arc sweep x y` shape.
    ArcTo {
        rx: f64,
        ry: f64,
        x_rotation: f64,
        large_arc: bool,
        sweep: bool,
        to: SemioPoint2,
    },
    Close,
}
//#endregion 🔖️PathSegment

//#region 🔖️DrawNode
/// 🖍️ Owned by the `drawing` subset: the recursive scene-graph node, matching svg's
/// `SvgNodeDiff` recursive-diff template per the master plan. `style` fields are a referential
/// `Option<String>` into `SemioDrawingSnapshot.styles` by name (checked by `SemioDrawingValidator`
/// — dangling references are a real referential-invariant breach, not silently tolerated).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum DrawNode {
    Path {
        segments: Vec<PathSegment>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        style: Option<String>,
    },
    Text {
        value: String,
        at: SemioPoint2,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        style: Option<String>,
    },
    Group {
        transform: SemioTransform,
        #[serde(default)]
        children: Vec<DrawNode>,
    },
    /// 🖼️ Raster payload embedded verbatim (typed raw retention — real bytes, not a lie).
    Image { at: SemioPoint2, width: f64, height: f64, mime: String, bytes: Vec<u8> },
}

impl Default for DrawNode {
    fn default() -> Self {
        DrawNode::Group { transform: SemioTransform::identity(), children: Vec::new() }
    }
}
//#endregion 🔖️DrawNode

//#region 🔖️Style
/// 🎨️ A named presentation style, referenced by `DrawNode::Path`/`Text.style`. Name-keyed
/// (`NamedTripleDiff<String, DrawStyleDiff, DrawStyle>` in the diff facet).
/// 🩹 `Default` derived (not just decoration) — required transitively as the `T` of
/// `triples::NamedTripleDiff<String, DrawStyleDiff, DrawStyle>`'s generated `Deserialize` impl
/// (serde-derive's bound inference for `#[serde(default)]` fields on a generic container reaches
/// every type parameter, not just the immediately-defaulted field's own type).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawStyle {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fill: Option<SemioRgba>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stroke: Option<SemioRgba>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stroke_width: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub opacity: Option<f32>,
}
//#endregion 🔖️Style

//#region 🔖️Layer
/// 🗂️ One ordered layer (index-keyed z-order, `IndexedTripleDiff<DrawLayerDiff, DrawLayer>` in
/// the diff facet — mirrors gif-frame ordering precedent).
/// 🩹 `Default` derived for the same reason as `DrawStyle` above (needed as the `T` of
/// `triples::IndexedTripleDiff<DrawLayerDiff, DrawLayer>`).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawLayer {
    pub id: String,
    pub name: String,
    pub visible: bool,
    pub root: DrawNode,
}
//#endregion 🔖️Layer

//#region 🔖️Canvas
/// 🖼️ Document-level viewport/backdrop.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawCanvas {
    pub width: f64,
    pub height: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background: Option<SemioRgba>,
}

impl Default for DrawCanvas {
    fn default() -> Self {
        Self { width: 0.0, height: 0.0, background: None }
    }
}
//#endregion 🔖️Canvas

//#region 🔖️Ids
pub const STDIO_SEMIODRAWING_DOCUMENT_SCHEMA: &str = "stdio.semio.drawing";
//#endregion 🔖️Ids

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.drawing")]
pub struct SemioDrawingSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub canvas: DrawCanvas,
    #[state(artifact)]
    #[serde(default)]
    pub styles: Vec<DrawStyle>,
    #[state(artifact)]
    #[serde(default)]
    pub layers: Vec<DrawLayer>,
}

impl Default for SemioDrawingSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_SEMIODRAWING_DOCUMENT_SCHEMA.into(), canvas: DrawCanvas::default(), styles: Vec::new(), layers: Vec::new() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️TextPrimitives
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
/// facets (`pub(crate)` below) rather than re-derived three times.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
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
pub(crate) fn parse_f64(s: &str) -> Result<f64, String> {
    s.parse().map_err(|e: std::num::ParseFloatError| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_f32(s: &str) -> Result<f32, String> {
    s.parse().map_err(|e: std::num::ParseFloatError| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_bool(b: bool) -> &'static str {
    if b {
        "1"
    } else {
        "0"
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_bool(s: &str) -> Result<bool, String> {
    match s {
        "1" => Ok(true),
        "0" => Ok(false),
        other => Err(format!("bad bool {other:?}")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_list<T>(items: &[T], enc: impl Fn(&T) -> String) -> String {
    format!("[{}]", items.iter().map(|it| enc(it)).collect::<Vec<_>>().join(","))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_list<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Vec<T>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|entry| dec(entry)).collect()
}
/// 🏳️ Single-state option: `[0]` = `None`, `[1,<value>]` = `Some(value)` — used by snapshot-level
/// `Option<T>` fields (never tri-state; tri-state `Option<Option<T>>` is a `🔺️diff`-only concept).
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
pub(crate) fn enc_point2(p: &SemioPoint2) -> String {
    format!("[{},{}]", p.x, p.y)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_point2(s: &str) -> Result<SemioPoint2, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [x, y] = parts.as_slice() else { return Err(format!("point2: expected 2 fields, got {}", parts.len())) };
    Ok(SemioPoint2 { x: parse_f64(x)?, y: parse_f64(y)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_point3(p: &SemioPoint3) -> String {
    format!("[{},{},{}]", p.x, p.y, p.z)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_point3(s: &str) -> Result<SemioPoint3, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [x, y, z] = parts.as_slice() else { return Err(format!("point3: expected 3 fields, got {}", parts.len())) };
    Ok(SemioPoint3 { x: parse_f64(x)?, y: parse_f64(y)?, z: parse_f64(z)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_quaternion(q: &SemioQuaternion) -> String {
    format!("[{},{},{},{}]", q.x, q.y, q.z, q.w)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_quaternion(s: &str) -> Result<SemioQuaternion, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [x, y, z, w] = parts.as_slice() else { return Err(format!("quaternion: expected 4 fields, got {}", parts.len())) };
    Ok(SemioQuaternion { x: parse_f64(x)?, y: parse_f64(y)?, z: parse_f64(z)?, w: parse_f64(w)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_transform(t: &SemioTransform) -> String {
    format!("[{},{},{}]", enc_point3(&t.translation), enc_quaternion(&t.rotation), enc_point3(&t.scale))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_transform(s: &str) -> Result<SemioTransform, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [translation, rotation, scale] = parts.as_slice() else { return Err(format!("transform: expected 3 fields, got {}", parts.len())) };
    Ok(SemioTransform { translation: dec_point3(translation)?, rotation: dec_quaternion(rotation)?, scale: dec_point3(scale)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_rgba(c: &SemioRgba) -> String {
    format!("[{},{},{},{}]", c.r, c.g, c.b, c.a)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_rgba(s: &str) -> Result<SemioRgba, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [r, g, b, a] = parts.as_slice() else { return Err(format!("rgba: expected 4 fields, got {}", parts.len())) };
    Ok(SemioRgba { r: parse_f32(r)?, g: parse_f32(g)?, b: parse_f32(b)?, a: parse_f32(a)? })
}

/// 📐️ `M[to]` (MoveTo) / `L[to]` (LineTo) / `C[c1,c2,to]` (CubicTo) / `Q[c,to]` (QuadTo) /
/// `A[rx,ry,xRotation,largeArc,sweep,to]` (ArcTo) / `Z` (Close, no payload).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_path_segment(seg: &PathSegment) -> String {
    match seg {
        PathSegment::MoveTo { to } => format!("M[{}]", enc_point2(to)),
        PathSegment::LineTo { to } => format!("L[{}]", enc_point2(to)),
        PathSegment::CubicTo { c1, c2, to } => format!("C[{},{},{}]", enc_point2(c1), enc_point2(c2), enc_point2(to)),
        PathSegment::QuadTo { c, to } => format!("Q[{},{}]", enc_point2(c), enc_point2(to)),
        PathSegment::ArcTo { rx, ry, x_rotation, large_arc, sweep, to } => {
            format!("A[{},{},{},{},{},{}]", rx, ry, x_rotation, enc_bool(*large_arc), enc_bool(*sweep), enc_point2(to))
        }
        PathSegment::Close => "Z".to_string(),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_path_segment(s: &str) -> Result<PathSegment, String> {
    if s == "Z" {
        return Ok(PathSegment::Close);
    }
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    let parts = split_top_level(inner, ',');
    match tag {
        "M" => {
            let [to] = parts.as_slice() else { return Err(format!("moveTo: expected 1 field, got {}", parts.len())) };
            Ok(PathSegment::MoveTo { to: dec_point2(to)? })
        }
        "L" => {
            let [to] = parts.as_slice() else { return Err(format!("lineTo: expected 1 field, got {}", parts.len())) };
            Ok(PathSegment::LineTo { to: dec_point2(to)? })
        }
        "C" => {
            let [c1, c2, to] = parts.as_slice() else { return Err(format!("cubicTo: expected 3 fields, got {}", parts.len())) };
            Ok(PathSegment::CubicTo { c1: dec_point2(c1)?, c2: dec_point2(c2)?, to: dec_point2(to)? })
        }
        "Q" => {
            let [c, to] = parts.as_slice() else { return Err(format!("quadTo: expected 2 fields, got {}", parts.len())) };
            Ok(PathSegment::QuadTo { c: dec_point2(c)?, to: dec_point2(to)? })
        }
        "A" => {
            let [rx, ry, x_rotation, large_arc, sweep, to] = parts.as_slice() else { return Err(format!("arcTo: expected 6 fields, got {}", parts.len())) };
            Ok(PathSegment::ArcTo { rx: parse_f64(rx)?, ry: parse_f64(ry)?, x_rotation: parse_f64(x_rotation)?, large_arc: parse_bool(large_arc)?, sweep: parse_bool(sweep)?, to: dec_point2(to)? })
        }
        other => Err(format!("path segment: unknown tag {other:?}")),
    }
}

/// 🌳️ `P[segments,style]` (Path) / `T[value,at,style]` (Text) / `G[transform,children]` (Group,
/// `children` genuinely RECURSIVE) / `I[at,width,height,mime,bytes]` (Image, `bytes` hex-encoded).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_node(n: &DrawNode) -> String {
    match n {
        DrawNode::Path { segments, style } => format!("P[{},{}]", enc_list(segments, enc_path_segment), encode_option(style, |s| enc_str(s))),
        DrawNode::Text { value, at, style } => format!("T[{},{},{}]", enc_str(value), enc_point2(at), encode_option(style, |s| enc_str(s))),
        DrawNode::Group { transform, children } => format!("G[{},{}.await]", enc_transform(transform), enc_list(children, enc_node)),
        DrawNode::Image { at, width, height, mime, bytes } => format!("I[{},{},{},{},{}]", enc_point2(at), width, height, enc_str(mime), hex_encode(bytes)),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_node(s: &str) -> Result<DrawNode, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "P" => {
            let parts = split_top_level(inner, ',');
            let [segments, style] = parts.as_slice() else { return Err(format!("path node: expected 2 fields, got {}", parts.len())) };
            Ok(DrawNode::Path { segments: dec_list(segments, dec_path_segment)?, style: decode_option(style, dec_str)? })
        }
        "T" => {
            let parts = split_top_level(inner, ',');
            let [value, at, style] = parts.as_slice() else { return Err(format!("text node: expected 3 fields, got {}", parts.len())) };
            Ok(DrawNode::Text { value: dec_str(value)?, at: dec_point2(at)?, style: decode_option(style, dec_str)? })
        }
        "G" => {
            let parts = split_top_level(inner, ',');
            let [transform, children] = parts.as_slice() else { return Err(format!("group node: expected 2 fields, got {}", parts.len())) };
            Ok(DrawNode::Group { transform: dec_transform(transform)?, children: dec_list(children, dec_node)? })
        }
        "I" => {
            let parts = split_top_level(inner, ',');
            let [at, width, height, mime, bytes] = parts.as_slice() else { return Err(format!("image node: expected 5 fields, got {}", parts.len())) };
            Ok(DrawNode::Image { at: dec_point2(at)?, width: parse_f64(width)?, height: parse_f64(height)?, mime: dec_str(mime)?, bytes: hex_decode(bytes)? })
        }
        other => Err(format!("node: unknown tag {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_style(s: &DrawStyle) -> String {
    format!("[{},{},{},{},{}]", enc_str(&s.name), encode_option(&s.fill, enc_rgba), encode_option(&s.stroke, enc_rgba), encode_option(&s.stroke_width, |v: &f64| v.to_string()), encode_option(&s.opacity, |v: &f32| v.to_string()),)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_style(s: &str) -> Result<DrawStyle, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, fill, stroke, stroke_width, opacity] = parts.as_slice() else { return Err(format!("style: expected 5 fields, got {}", parts.len())) };
    Ok(DrawStyle { name: dec_str(name)?, fill: decode_option(fill, dec_rgba)?, stroke: decode_option(stroke, dec_rgba)?, stroke_width: decode_option(stroke_width, parse_f64)?, opacity: decode_option(opacity, parse_f32)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_layer(l: &DrawLayer) -> String {
    format!("[{},{},{},{}]", enc_str(&l.id), enc_str(&l.name), enc_bool(l.visible), enc_node(&l.root))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_layer(s: &str) -> Result<DrawLayer, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, name, visible, root] = parts.as_slice() else { return Err(format!("layer: expected 4 fields, got {}", parts.len())) };
    Ok(DrawLayer { id: dec_str(id)?, name: dec_str(name)?, visible: parse_bool(visible)?, root: dec_node(root)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_canvas(c: &DrawCanvas) -> String {
    format!("[{},{},{}]", c.width, c.height, encode_option(&c.background, enc_rgba))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_canvas(s: &str) -> Result<DrawCanvas, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [width, height, background] = parts.as_slice() else { return Err(format!("canvas: expected 3 fields, got {}", parts.len())) };
    Ok(DrawCanvas { width: parse_f64(width)?, height: parse_f64(height)?, background: decode_option(background, dec_rgba)? })
}

/// 📄️ The real structured text body: four lines — `schema=<hex>`, `canvas=<canvas>`,
/// `styles=[...]`, `layers=[...]` — matching the grammar's `document = artifact-mark schema-line
/// canvas-line styles-line layers-line`. Newlines are pure lexer trivia in the shared dialect, so
/// this is genuinely recognizable by `dsl::Recognizer`, not merely readable.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_drawing_snapshot_body(s: &SemioDrawingSnapshot) -> String {
    format!("schema={}\ncanvas={}\nstyles=[{}]\nlayers=[{}]", enc_str(&s.schema), enc_canvas(&s.canvas), s.styles.iter().map(enc_style).collect::<Vec<_>>().join(","), s.layers.iter().map(enc_layer).collect::<Vec<_>>().join(","),)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_drawing_snapshot_body(body: &str) -> Result<SemioDrawingSnapshot, String> {
    let mut schema = None;
    let mut canvas = None;
    let mut styles = Vec::new();
    let mut layers = Vec::new();
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("schema=") {
            schema = Some(dec_str(rest)?);
        } else if let Some(rest) = line.strip_prefix("canvas=") {
            canvas = Some(dec_canvas(rest)?);
        } else if let Some(rest) = line.strip_prefix("styles=") {
            styles = dec_list(rest, dec_style)?;
        } else if let Some(rest) = line.strip_prefix("layers=") {
            layers = dec_list(rest, dec_layer)?;
        } else {
            return Err(format!("drawing snapshot: unknown line {line:?}"));
        }
    }
    let schema = schema.ok_or_else(|| "drawing snapshot: missing schema line".to_string())?;
    let canvas = canvas.ok_or_else(|| "drawing snapshot: missing canvas line".to_string())?;
    Ok(SemioDrawingSnapshot { schema, canvas, styles, layers })
}
//#endregion 🔖️TextPrimitives

//#region 🔖️BinaryPrimitives
/// 🧪️ Real LEB128-varint-length-prefixed binary primitives (`store::pack_rt::write_varint_u64` /
/// `store::ByteReader`, same helpers `stdio.semio.flow`'s/`stdio.semio.brep`'s upgraded
/// `ArtifactPack` reuse) backing the real `ArtifactPack` below — replaces the old
/// `serde_json::to_vec`-in-envelope shortcut.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_point2(out: &mut Vec<u8>, p: &SemioPoint2) {
    out.extend_from_slice(&p.x.to_le_bytes());
    out.extend_from_slice(&p.y.to_le_bytes());
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_point2(reader: &mut store::ByteReader<'_>) -> Result<SemioPoint2, String> {
    Ok(SemioPoint2 { x: reader.read_f64_le().map_err(|e| e.to_string())?, y: reader.read_f64_le().map_err(|e| e.to_string())? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_point3(out: &mut Vec<u8>, p: &SemioPoint3) {
    out.extend_from_slice(&p.x.to_le_bytes());
    out.extend_from_slice(&p.y.to_le_bytes());
    out.extend_from_slice(&p.z.to_le_bytes());
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_point3(reader: &mut store::ByteReader<'_>) -> Result<SemioPoint3, String> {
    Ok(SemioPoint3 { x: reader.read_f64_le().map_err(|e| e.to_string())?, y: reader.read_f64_le().map_err(|e| e.to_string())?, z: reader.read_f64_le().map_err(|e| e.to_string())? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_quaternion(out: &mut Vec<u8>, q: &SemioQuaternion) {
    out.extend_from_slice(&q.x.to_le_bytes());
    out.extend_from_slice(&q.y.to_le_bytes());
    out.extend_from_slice(&q.z.to_le_bytes());
    out.extend_from_slice(&q.w.to_le_bytes());
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_quaternion(reader: &mut store::ByteReader<'_>) -> Result<SemioQuaternion, String> {
    Ok(SemioQuaternion { x: reader.read_f64_le().map_err(|e| e.to_string())?, y: reader.read_f64_le().map_err(|e| e.to_string())?, z: reader.read_f64_le().map_err(|e| e.to_string())?, w: reader.read_f64_le().map_err(|e| e.to_string())? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_transform(out: &mut Vec<u8>, t: &SemioTransform) {
    write_point3(out, &t.translation);
    write_quaternion(out, &t.rotation);
    write_point3(out, &t.scale);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_transform(reader: &mut store::ByteReader<'_>) -> Result<SemioTransform, String> {
    Ok(SemioTransform { translation: read_point3(reader)?, rotation: read_quaternion(reader)?, scale: read_point3(reader)? })
}
/// 🩹️ `store::ByteReader` has no native `f32` reader (only `f64_le`) — read 4 raw bytes instead.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_f32_le(reader: &mut store::ByteReader<'_>) -> Result<f32, String> {
    let bytes = reader.read_bytes(4).map_err(|e| e.to_string())?;
    let arr: [u8; 4] = bytes.try_into().map_err(|_| "f32 read: truncated".to_string())?;
    Ok(f32::from_le_bytes(arr))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_rgba(out: &mut Vec<u8>, c: &SemioRgba) {
    out.extend_from_slice(&c.r.to_le_bytes());
    out.extend_from_slice(&c.g.to_le_bytes());
    out.extend_from_slice(&c.b.to_le_bytes());
    out.extend_from_slice(&c.a.to_le_bytes());
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_rgba(reader: &mut store::ByteReader<'_>) -> Result<SemioRgba, String> {
    Ok(SemioRgba { r: read_f32_le(reader)?, g: read_f32_le(reader)?, b: read_f32_le(reader)?, a: read_f32_le(reader)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_bool(out: &mut Vec<u8>, b: bool) {
    out.push(if b { 1 } else { 0 });
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_bool(reader: &mut store::ByteReader<'_>) -> Result<bool, String> {
    Ok(reader.read_u8().map_err(|e| e.to_string())? != 0)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_option<T>(out: &mut Vec<u8>, opt: &Option<T>, write: impl Fn(&mut Vec<u8>, &T)) {
    match opt {
        None => out.push(0),
        Some(v) => {
            out.push(1);
            write(out, v);
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_option<T>(reader: &mut store::ByteReader<'_>, read: impl Fn(&mut store::ByteReader<'_>) -> Result<T, String>) -> Result<Option<T>, String> {
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => Ok(None),
        1 => Ok(Some(read(reader)?)),
        other => Err(format!("option: bad tag byte {other}")),
    }
}

/// 🏷️ `PathSegment` variant tags — 0=MoveTo, 1=LineTo, 2=CubicTo, 3=QuadTo, 4=ArcTo, 5=Close.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_path_segment(out: &mut Vec<u8>, seg: &PathSegment) {
    match seg {
        PathSegment::MoveTo { to } => {
            out.push(0);
            write_point2(out, to);
        }
        PathSegment::LineTo { to } => {
            out.push(1);
            write_point2(out, to);
        }
        PathSegment::CubicTo { c1, c2, to } => {
            out.push(2);
            write_point2(out, c1);
            write_point2(out, c2);
            write_point2(out, to);
        }
        PathSegment::QuadTo { c, to } => {
            out.push(3);
            write_point2(out, c);
            write_point2(out, to);
        }
        PathSegment::ArcTo { rx, ry, x_rotation, large_arc, sweep, to } => {
            out.push(4);
            out.extend_from_slice(&rx.to_le_bytes());
            out.extend_from_slice(&ry.to_le_bytes());
            out.extend_from_slice(&x_rotation.to_le_bytes());
            write_bool(out, *large_arc);
            write_bool(out, *sweep);
            write_point2(out, to);
        }
        PathSegment::Close => out.push(5),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_path_segment(reader: &mut store::ByteReader<'_>) -> Result<PathSegment, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => Ok(PathSegment::MoveTo { to: read_point2(reader)? }),
        1 => Ok(PathSegment::LineTo { to: read_point2(reader)? }),
        2 => Ok(PathSegment::CubicTo { c1: read_point2(reader)?, c2: read_point2(reader)?, to: read_point2(reader)? }),
        3 => Ok(PathSegment::QuadTo { c: read_point2(reader)?, to: read_point2(reader)? }),
        4 => Ok(PathSegment::ArcTo {
            rx: reader.read_f64_le().map_err(|e| e.to_string())?,
            ry: reader.read_f64_le().map_err(|e| e.to_string())?,
            x_rotation: reader.read_f64_le().map_err(|e| e.to_string())?,
            large_arc: read_bool(reader)?,
            sweep: read_bool(reader)?,
            to: read_point2(reader)?,
        }),
        5 => Ok(PathSegment::Close),
        other => Err(format!("path segment: unknown binary tag {other}")),
    }
}

/// 🏷️ `DrawNode` variant tags — 0=Path, 1=Text, 2=Group (RECURSIVE `children`), 3=Image.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_node(out: &mut Vec<u8>, n: &DrawNode) {
    match n {
        DrawNode::Path { segments, style } => {
            out.push(0);
            store::pack_rt::write_varint_u64(out, segments.len() as u64);
            for seg in segments {
                write_path_segment(out, seg);
            }
            write_option(out, style, |out, s| write_str_lp(out, s));
        }
        DrawNode::Text { value, at, style } => {
            out.push(1);
            write_str_lp(out, value);
            write_point2(out, at);
            write_option(out, style, |out, s| write_str_lp(out, s));
        }
        DrawNode::Group { transform, children } => {
            out.push(2);
            write_transform(out, transform);
            store::pack_rt::write_varint_u64(out, children.len() as u64);
            for child in children {
                write_node(out, child);
            }
        }
        DrawNode::Image { at, width, height, mime, bytes } => {
            out.push(3);
            write_point2(out, at);
            out.extend_from_slice(&width.to_le_bytes());
            out.extend_from_slice(&height.to_le_bytes());
            write_str_lp(out, mime);
            write_bytes_lp(out, bytes);
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_node(reader: &mut store::ByteReader<'_>) -> Result<DrawNode, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => {
            let n = reader.read_varint_u64().map_err(|e| e.to_string())?;
            let mut segments = Vec::with_capacity(n as usize);
            for _ in 0..n {
                segments.push(read_path_segment(reader)?);
            }
            let style = read_option(reader, read_str_lp)?;
            Ok(DrawNode::Path { segments, style })
        }
        1 => {
            let value = read_str_lp(reader)?;
            let at = read_point2(reader)?;
            let style = read_option(reader, read_str_lp)?;
            Ok(DrawNode::Text { value, at, style })
        }
        2 => {
            let transform = read_transform(reader)?;
            let n = reader.read_varint_u64().map_err(|e| e.to_string())?;
            let mut children = Vec::with_capacity(n as usize);
            for _ in 0..n {
                children.push(read_node(reader)?);
            }
            Ok(DrawNode::Group { transform, children })
        }
        3 => {
            let at = read_point2(reader)?;
            let width = reader.read_f64_le().map_err(|e| e.to_string())?;
            let height = reader.read_f64_le().map_err(|e| e.to_string())?;
            let mime = read_str_lp(reader)?;
            let bytes = read_bytes_lp(reader)?;
            Ok(DrawNode::Image { at, width, height, mime, bytes })
        }
        other => Err(format!("node: unknown binary tag {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_style(out: &mut Vec<u8>, s: &DrawStyle) {
    write_str_lp(out, &s.name);
    write_option(out, &s.fill, write_rgba);
    write_option(out, &s.stroke, write_rgba);
    write_option(out, &s.stroke_width, |out, v| out.extend_from_slice(&v.to_le_bytes()));
    write_option(out, &s.opacity, |out, v| out.extend_from_slice(&v.to_le_bytes()));
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_style(reader: &mut store::ByteReader<'_>) -> Result<DrawStyle, String> {
    let name = read_str_lp(reader)?;
    let fill = read_option(reader, read_rgba)?;
    let stroke = read_option(reader, read_rgba)?;
    let stroke_width = read_option(reader, |r| semio_framework_plugin::resolve_ready(r.read_f64_le()).map_err(|e| e.to_string()))?;
    let opacity = read_option(reader, read_f32_le)?;
    Ok(DrawStyle { name, fill, stroke, stroke_width, opacity })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_layer(out: &mut Vec<u8>, l: &DrawLayer) {
    write_str_lp(out, &l.id);
    write_str_lp(out, &l.name);
    write_bool(out, l.visible);
    write_node(out, &l.root);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_layer(reader: &mut store::ByteReader<'_>) -> Result<DrawLayer, String> {
    Ok(DrawLayer { id: read_str_lp(reader)?, name: read_str_lp(reader)?, visible: read_bool(reader)?, root: read_node(reader)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_canvas(out: &mut Vec<u8>, c: &DrawCanvas) {
    out.extend_from_slice(&c.width.to_le_bytes());
    out.extend_from_slice(&c.height.to_le_bytes());
    write_option(out, &c.background, write_rgba);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_canvas(reader: &mut store::ByteReader<'_>) -> Result<DrawCanvas, String> {
    let width = reader.read_f64_le().map_err(|e| e.to_string())?;
    let height = reader.read_f64_le().map_err(|e| e.to_string())?;
    let background = read_option(reader, read_rgba)?;
    Ok(DrawCanvas { width, height, background })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_drawing_snapshot_binary(s: &SemioDrawingSnapshot) -> Vec<u8> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut out = Vec::new();
    out.push(PACK_BINARY_FORMAT);
    write_str_lp(&mut out, &s.schema);
    write_canvas(&mut out, &s.canvas);
    store::pack_rt::write_varint_u64(&mut out, s.styles.len() as u64);
    for style in &s.styles {
        write_style(&mut out, style);
    }
    store::pack_rt::write_varint_u64(&mut out, s.layers.len() as u64);
    for layer in &s.layers {
        write_layer(&mut out, layer);
    }
    out
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_drawing_snapshot_binary(bytes: &[u8]) -> Result<SemioDrawingSnapshot, String> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut reader = semio_framework_plugin::resolve_ready(store::ByteReader::new(bytes));
    let format = reader.read_u8().map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT {
        return Err(format!("unsupported pack format {format}"));
    }
    let schema = read_str_lp(&mut reader)?;
    let canvas = read_canvas(&mut reader)?;
    let style_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut styles = Vec::with_capacity(style_count as usize);
    for _ in 0..style_count {
        styles.push(read_style(&mut reader)?);
    }
    let layer_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut layers = Vec::with_capacity(layer_count as usize);
    for _ in 0..layer_count {
        layers.push(read_layer(&mut reader)?);
    }
    Ok(SemioDrawingSnapshot { schema, canvas, styles, layers })
}
//#endregion 🔖️BinaryPrimitives

//#region 🔖️HandcraftedArtifactCodecs
/// 🎁 Real structured text/binary codecs (drawing wave — off the old hex-dump-of-`serde_json`
/// shortcut, following the flow/brep waves' proven template). Wrapped in the repo-wide
/// `store::semio_format` envelope, unchanged.
impl store::ArtifactDsl for SemioDrawingSnapshot {
    const EXTENSION: &'static str = "semio";
    async fn envelope_id() -> &'static str {
        STDIO_SEMIODRAWING_DOCUMENT_SCHEMA
    }

    async fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_drawing_snapshot_body(body).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }

    async fn print_dsl(&self) -> String {
        let body = print_drawing_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id().await, store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for SemioDrawingSnapshot {
    async fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_drawing_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id().await, store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    async fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let _ = options;
        decode_drawing_snapshot_binary(&inner).map_err(store::PackError::Schema)
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️Demo
/// 🌱 The demo `s.stdio.semio.drawing` document — exercises every `PathSegment`/`DrawNode` variant
/// at least once (incl. nested `Group.children` recursion) plus every `Option<T>` field non-`None`.
/// Single source of truth for `📚️examples/🖍️sketch/🖼️assets/🗣️example.dsl.semio`/`🎒️example.pack.semio`
/// and for the conformance-law tests in `🎹️composer/🦀️component.rs`.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_drawing_snapshot() -> SemioDrawingSnapshot {
    SemioDrawingSnapshot {
        schema: STDIO_SEMIODRAWING_DOCUMENT_SCHEMA.into(),
        canvas: DrawCanvas { width: 100.0, height: 50.0, background: Some(SemioRgba { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }) },
        styles: vec![DrawStyle { name: "s1".into(), fill: Some(SemioRgba { r: 1.0, g: 0.0, b: 0.0, a: 1.0 }), stroke: None, stroke_width: Some(2.0), opacity: None }],
        layers: vec![DrawLayer {
            id: "l0".into(),
            name: "base".into(),
            visible: true,
            root: DrawNode::Group {
                transform: SemioTransform::identity(),
                children: vec![
                    DrawNode::Path {
                        segments: vec![
                            PathSegment::MoveTo { to: SemioPoint2 { x: 0.0, y: 0.0 } },
                            PathSegment::LineTo { to: SemioPoint2 { x: 10.0, y: 10.0 } },
                            PathSegment::CubicTo { c1: SemioPoint2 { x: 1.0, y: 1.0 }, c2: SemioPoint2 { x: 2.0, y: 2.0 }, to: SemioPoint2 { x: 3.0, y: 3.0 } },
                            PathSegment::QuadTo { c: SemioPoint2 { x: 4.0, y: 4.0 }, to: SemioPoint2 { x: 5.0, y: 5.0 } },
                            PathSegment::ArcTo { rx: 1.0, ry: 2.0, x_rotation: 0.0, large_arc: true, sweep: false, to: SemioPoint2 { x: 6.0, y: 6.0 } },
                            PathSegment::Close,
                        ],
                        style: Some("s1".into()),
                    },
                    DrawNode::Text { value: "hi".into(), at: SemioPoint2 { x: 5.0, y: 5.0 }, style: None },
                    DrawNode::Image { at: SemioPoint2 { x: 0.0, y: 0.0 }, width: 8.0, height: 8.0, mime: "image/png".into(), bytes: vec![1, 2, 3] },
                    DrawNode::Group { transform: SemioTransform::identity(), children: Vec::new() },
                ],
            },
        }],
    }
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sample() -> SemioDrawingSnapshot {
        SemioDrawingSnapshot {
            schema: STDIO_SEMIODRAWING_DOCUMENT_SCHEMA.into(),
            canvas: DrawCanvas { width: 100.0, height: 50.0, background: Some(SemioRgba { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }) },
            styles: vec![DrawStyle { name: "s1".into(), fill: Some(SemioRgba { r: 1.0, g: 0.0, b: 0.0, a: 1.0 }), stroke: None, stroke_width: Some(2.0), opacity: None }],
            layers: vec![DrawLayer {
                id: "l0".into(),
                name: "base".into(),
                visible: true,
                root: DrawNode::Group {
                    transform: SemioTransform::identity(),
                    children: vec![
                        DrawNode::Path { segments: vec![PathSegment::MoveTo { to: SemioPoint2 { x: 0.0, y: 0.0 } }, PathSegment::LineTo { to: SemioPoint2 { x: 10.0, y: 10.0 } }, PathSegment::Close], style: Some("s1".into()) },
                        DrawNode::Text { value: "hi".into(), at: SemioPoint2 { x: 5.0, y: 5.0 }, style: None },
                        DrawNode::Image { at: SemioPoint2 { x: 0.0, y: 0.0 }, width: 8.0, height: 8.0, mime: "image/png".into(), bytes: vec![1, 2, 3] },
                    ],
                },
            }],
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn json_pack_round_trips() {
        let snap = sample();
        let bytes = <SemioDrawingSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <SemioDrawingSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(snap, back);
    }

    #[semio_framework_async_macros::async_test]
    async fn dsl_text_round_trips() {
        let snap = sample();
        let text = <SemioDrawingSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        let back = <SemioDrawingSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(snap, back);
    }

    #[semio_framework_async_macros::async_test]
    async fn default_snapshot_round_trips() {
        let snap = SemioDrawingSnapshot::default();
        let bytes = <SemioDrawingSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <SemioDrawingSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(snap, back);
    }

    /// 🧪️ Every `PathSegment`/`DrawNode` variant (incl. nested `Group.children` recursion) round-
    /// trips through both the pack binary and the dsl text codec — the demo fixture used by the
    /// fixture-honesty conformance law.
    #[semio_framework_async_macros::async_test]
    async fn demo_snapshot_round_trips_pack_and_dsl() {
        let demo = demo_drawing_snapshot();
        let packed = <SemioDrawingSnapshot as store::ArtifactPack>::encode_pack(&demo);
        assert_eq!(<SemioDrawingSnapshot as store::ArtifactPack>::decode_pack(&packed).expect("decode"), demo);
        let text = <SemioDrawingSnapshot as store::ArtifactDsl>::print_dsl(&demo);
        assert_eq!(<SemioDrawingSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse"), demo);
    }
}
//#endregion 🔖️Tests
