//! 📝️ Text representation codec surface for `stdio.semio.drawing` (snapshot). The real parse/
//! print lives on `SemioDrawingSnapshot`'s `store::ArtifactDsl` impl (📸️snapshot/🦀️.rs)
//! -- this module exposes the grammar source for tooling/introspection, matching svg's own
//! `📝️text/🦀️.rs` convention.

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::drawing::schema::snapshot::*;
use crate::standards::v1::subsets::base::schema::geometry::native::{NativeF64,NativeF32};
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint3, SemioQuaternion, SemioRgba, SemioTransform};
use crate::standards::v1::subsets::base::schema::triples::{split_top_level, strip_brackets};
use framework_schema::ArtifactSchema;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;

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
pub(crate) fn parse_f64(s: &str) -> Result<f64, String> {
    crate::standards::v1::subsets::base::schema::geometry::native::parse(s)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_f32(s: &str) -> Result<f32, String> {
    crate::standards::v1::subsets::base::schema::geometry::native::parse32(s)
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
    format!("[{}]", items.iter().map(enc).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_list<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Vec<T>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec).collect()
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
    format!("[{},{}]", NativeF64(p.x), NativeF64(p.y))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_point2(s: &str) -> Result<SemioPoint2, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [x, y] = parts.as_slice() else { return Err(format!("point2: expected 2 fields, got {}", parts.len())) };
    Ok(SemioPoint2 { x: parse_f64(x)?, y: parse_f64(y)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_point3(p: &SemioPoint3) -> String {
    format!("[{},{},{}]", NativeF64(p.x), NativeF64(p.y), NativeF64(p.z))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_point3(s: &str) -> Result<SemioPoint3, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [x, y, z] = parts.as_slice() else { return Err(format!("point3: expected 3 fields, got {}", parts.len())) };
    Ok(SemioPoint3 { x: parse_f64(x)?, y: parse_f64(y)?, z: parse_f64(z)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_quaternion(q: &SemioQuaternion) -> String {
    format!("[{},{},{},{}]", NativeF64(q.x), NativeF64(q.y), NativeF64(q.z), NativeF64(q.w))
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
    format!("[{},{},{},{}]", NativeF32(c.r), NativeF32(c.g), NativeF32(c.b), NativeF32(c.a))
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
            format!("A[{},{},{},{},{},{}]", NativeF64(*rx), NativeF64(*ry), NativeF64(*x_rotation), enc_bool(*large_arc), enc_bool(*sweep), enc_point2(to))
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
        DrawNode::Group { transform, children } => format!("G[{},{}]", enc_transform(transform), enc_list(children, enc_node)),
        DrawNode::Image { at, width, height, mime, bytes } => format!("I[{},{},{},{},{}]", enc_point2(at), NativeF64(*width), NativeF64(*height), enc_str(mime), hex_encode(bytes)),
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
    format!("[{},{},{},{},{}]", enc_str(&s.name), encode_option(&s.fill, enc_rgba), encode_option(&s.stroke, enc_rgba), encode_option(&s.stroke_width, |v: &f64| NativeF64(*v).to_string()), encode_option(&s.opacity, |v: &f32| NativeF32(*v).to_string()),)
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
    format!("[{},{},{}]", NativeF64(c.width), NativeF64(c.height), encode_option(&c.background, enc_rgba))
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
pub(crate) fn print_drawing_snapshot_body(s: &SemioDrawingSnapshot) -> String {
    format!("schema={}\ncanvas={}\nstyles=[{}]\nlayers=[{}]", enc_str(&s.schema), enc_canvas(&s.canvas), s.styles.iter().map(enc_style).collect::<Vec<_>>().join(","), s.layers.iter().map(enc_layer).collect::<Vec<_>>().join(","),)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_drawing_snapshot_body(body: &str) -> Result<SemioDrawingSnapshot, String> {
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

/// 🎁 Real structured text/binary codecs (drawing wave — off the old hex-dump-of-`serde_json`
/// shortcut, following the flow/brep waves' proven template). Wrapped in the repo-wide
/// `store::semio_format` envelope, unchanged.
impl store::ArtifactDsl for SemioDrawingSnapshot {
    const EXTENSION: &'static str = "semio";
    fn envelope_id() -> &'static str {
        STDIO_SEMIODRAWING_DOCUMENT_SCHEMA
    }

    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_drawing_snapshot_body(body).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        let body = print_drawing_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📤️ This subset's own `#[value(rename_all = "camelCase")]` structural JSON projection of
/// `s.stdio.semio.drawing` — the shape `🖊️mutate-semio-drawing` compares under `ordered-json-v1`,
/// derived from the snapshot type itself rather than hand-written a second time in the adapter. The
/// projection is deeply discriminated: `DrawNode` and `PathSegment` are both `#[value(tag = "kind",
/// rename_all = "camelCase")]` enums nested to arbitrary depth, so a hand-written adapter
/// projection would have to reproduce every discriminator at every level. A thin
/// `pack::to_json_string` wrapper (over `ToValue`/`FromValue`, first-party, per this ticket's
/// serde→value conversion).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_semio_drawing_snapshot_json(snapshot: &SemioDrawingSnapshot) -> String {
    semio_framework_pack_json::to_json_string(snapshot)
}

/// 📥️ The `pack::from_json_str` inverse of [`encode_semio_drawing_snapshot_json`] — decodes the
/// committed `../🧬️mutations/<kind>/🧪️tests/<fixture>/📸️snapshot/{⬅️before,➡️after}/🔣️.json`
/// specification vectors into real [`SemioDrawingSnapshot`] values, so `🖊️mutate-semio-drawing`'s
/// adapter reads the committed fixture instead of re-declaring a recursive scene graph as a Rust
/// literal beside it. Reaching `pack` from that adapter is impossible — the generated test host
/// links only this crate — which is why the bridge belongs here rather than there.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_drawing_snapshot_json(text: &str) -> Result<SemioDrawingSnapshot, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

/// 📝️ Parses `s.stdio.semio.drawing` DSL text into a [`SemioDrawingSnapshot`] — a named
/// pass-through of this snapshot's own `store::ArtifactDsl` impl above, whose trait and error type
/// are both unnameable outside this crate, so `🖊️mutate-semio-drawing`'s `identity-round-trip`
/// scenario reaches the real committed sketch artifact through this instead.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_semio_drawing_dsl(text: &str) -> Result<SemioDrawingSnapshot, String> {
    <SemioDrawingSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| error.to_string())
}

/// 📝️ Renders a [`SemioDrawingSnapshot`] back as DSL text — the inverse of
/// [`parse_semio_drawing_dsl`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn print_semio_drawing_dsl(snapshot: &SemioDrawingSnapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod snapshot_wire_codec {
use super::*;
use crate::standards::v1::subsets::drawing::schema::snapshot::*;
use crate::standards::v1::subsets::base::schema::geometry::native::{NativeF64,NativeF32};
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint3, SemioQuaternion, SemioRgba, SemioTransform};
use crate::audio::io::text::diff::{strip_brackets};
use crate::audio::io::text::diff::{split_top_level};
use framework_schema::ArtifactSchema;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;




}
pub use snapshot_wire_codec::*;

#[allow(unused_imports)]
mod snapshot_wire2_codec {
use crate::standards::v1::subsets::drawing::schema::snapshot::component::component::nodes::*;
use crate::standards::v1::subsets::drawing::schema::snapshot::component::component::crate::standards::v1::subsets::drawing::schema::snapshot::component::component::{DrawNode,PathSegment,SemioPoint2,SemioTransform};
use semio_framework_value::{DslValue,ToValue,FromValue,ValueError,ValueRefusalKind,DecodedValue,NativeDecodeControl,NativeEncodeControl};

pub fn retire_node(value:DrawNode){drop(crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding::Owned::new(value));}

pub(crate) fn retire_nodes(values:Vec<DrawNode>){for value in values{retire_node(value);}}

pub(crate) fn retire_values(values:Vec<DslValue>){<Vec<DslValue> as FromValue>::retire_decoded(values);}

pub(crate) fn encode_push<T>(items:&mut Vec<T>,value:T,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{
    if items.len()==items.capacity(){let additional=items.capacity().max(1);control.charge(additional.checked_mul(std::mem::size_of::<T>()).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Drawing frontier overflow"))?)?;items.try_reserve_exact(additional).map_err(|_|ValueError::new(ValueRefusalKind::AllocationFailed,"Drawing frontier allocation"))?;}
    items.push(value);Ok(())
}

pub(crate) fn decode_push<T>(items:&mut Vec<T>,value:T,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{
    if items.len()==items.capacity(){let additional=items.capacity().max(1);control.charge(additional.checked_mul(std::mem::size_of::<T>()).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Drawing frontier overflow"))?)?;items.try_reserve_exact(additional).map_err(|_|ValueError::new(ValueRefusalKind::AllocationFailed,"Drawing frontier allocation"))?;}
    items.push(value);Ok(())
}

pub(crate) fn entries<'a>(value:&'a DslValue,control:&mut NativeDecodeControl<'_>)->Result<&'a[(String,DslValue)],ValueError>{value.object_controlled(control)}

pub(crate) fn required<'a>(values:&'a[(String,DslValue)],key:&str,control:&mut NativeDecodeControl<'_>)->Result<&'a DslValue,ValueError>{DslValue::field_controlled(values,key,control)?.ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,format!("missing Drawing field {key}")))}

pub(crate) fn decode<T:FromValue>(values:&[(String,DslValue)],key:&str,control:&mut NativeDecodeControl<'_>)->Result<T,ValueError>{T::from_value_controlled(required(values,key,control)?,control).map_err(|error|error.under(key))}

pub(crate) fn optional<T:FromValue>(values:&[(String,DslValue)],key:&str,control:&mut NativeDecodeControl<'_>)->Result<Option<T>,ValueError>{match DslValue::field_controlled(values,key,control)?{Some(value)=>Option::<T>::from_value_controlled(value,control).map_err(|error|error.under(key)),None=>Ok(None)}}

pub(crate) fn child_values<'a>(values:&'a[(String,DslValue)],control:&mut NativeDecodeControl<'_>)->Result<&'a[DslValue],ValueError>{match DslValue::field_controlled(values,"children",control)?{None=>Ok(&[]),Some(DslValue::Array(values))=>Ok(values),Some(_)=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"Drawing children must be an array"))}}

pub(crate) enum Input<'a>{Node(&'a DslValue),Group(SemioTransform,usize)}

pub fn decode_node(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<DrawNode,ValueError>{
    control.scoped_stage(|control|{
        control.begin_stage(0)?;
        let mut pending=Vec::new();decode_push(&mut pending,Input::Node(value),control)?;
        let mut built=DecodedValue::new(Vec::<DrawNode>::new(),retire_nodes);
        while let Some(task)=pending.pop(){
            match task{
                Input::Node(value)=>{
                    let fields=entries(value,control)?;
                    let DslValue::String(kind)=required(fields,"kind",control)? else{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Drawing kind must be text"))};
                    let node=match kind.as_str(){
                        "path"=>DrawNode::Path{segments:decode::<Vec<PathSegment>>(fields,"segments",control)?,style:optional(fields,"style",control)?},
                        "text"=>DrawNode::Text{value:decode(fields,"value",control)?,at:decode::<SemioPoint2>(fields,"at",control)?,style:optional(fields,"style",control)?},
                        "image"=>DrawNode::Image{at:decode(fields,"at",control)?,width:decode(fields,"width",control)?,height:decode(fields,"height",control)?,mime:decode(fields,"mime",control)?,bytes:decode(fields,"bytes",control)?},
                        "group"=>{let transform=decode::<SemioTransform>(fields,"transform",control)?;let children=child_values(fields,control)?;decode_push(&mut pending,Input::Group(transform,children.len()),control)?;for child in children.iter().rev(){decode_push(&mut pending,Input::Node(child),control)?;control.step()?;}control.step()?;continue;},
                        _=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unknown Drawing kind")),
                    };
                    let owner=DecodedValue::new(node,retire_node);
                    if built.get().len()==built.get().capacity(){let additional=built.get().capacity().max(1);control.charge(additional.checked_mul(std::mem::size_of::<DrawNode>()).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Drawing forest overflow"))?)?;built.get_mut().try_reserve_exact(additional).map_err(|_|ValueError::new(ValueRefusalKind::AllocationFailed,"Drawing forest allocation"))?;}
                    built.get_mut().push(owner.take());
                },
                Input::Group(transform,count)=>{
                    let start=built.get().len().checked_sub(count).ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"Drawing child construction"))?;
                    let mut children=DecodedValue::new(control.allocate_vec::<DrawNode>(count)?,retire_nodes);
                    for node in built.get_mut().drain(start..){children.get_mut().push(node);}
                    let owner=DecodedValue::new(DrawNode::Group{transform,children:children.take()},retire_node);
                    if built.get().len()==built.get().capacity(){control.charge(std::mem::size_of::<DrawNode>())?;built.get_mut().try_reserve_exact(1).map_err(|_|ValueError::new(ValueRefusalKind::AllocationFailed,"Drawing group allocation"))?;}
                    built.get_mut().push(owner.take());
                },
            }
            control.step()?;
        }
        if built.get().len()!=1{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"Drawing root construction"))}
        Ok(built.get_mut().pop().unwrap())
    })
}

pub(crate) fn object(count:usize,control:&mut NativeEncodeControl<'_>)->Result<DecodedValue<DslValue>,ValueError>{Ok(DecodedValue::new(DslValue::Object(control.allocate_vec(count)?),<DslValue as FromValue>::retire_decoded))}

pub(crate) fn add<T:ToValue+?Sized>(value:&mut DslValue,key:&str,field:&T,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{
    let key=control.copy_text(key)?;let child=field.to_value_controlled(control)?;
    let DslValue::Object(fields)=value else{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"Drawing object construction"))};fields.push((key,child));Ok(())
}

pub(crate) fn add_owned(value:&mut DslValue,key:&str,child:DecodedValue<DslValue>,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{let key=control.copy_text(key)?;let DslValue::Object(fields)=value else{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"Drawing object construction"))};fields.push((key,child.take()));Ok(())}

pub(crate) enum Output<'a>{Node(&'a DrawNode),Group(&'a SemioTransform,usize)}

pub fn encode_node(value:&DrawNode,control:&mut NativeEncodeControl<'_>)->Result<DslValue,ValueError>{
    control.scoped_stage(|control|{
        control.begin_stage(0)?;
        let mut pending=Vec::new();encode_push(&mut pending,Output::Node(value),control)?;
        let mut built=DecodedValue::new(Vec::<DslValue>::new(),retire_values);
        while let Some(task)=pending.pop(){
            let output=match task{
                Output::Node(DrawNode::Path{segments,style})=>{let mut v=object(2+usize::from(style.is_some()),control)?;add(v.get_mut(),"kind",&"path",control)?;add(v.get_mut(),"segments",segments,control)?;if let Some(style)=style{add(v.get_mut(),"style",style,control)?;}v},
                Output::Node(DrawNode::Text{value,at,style})=>{let mut v=object(3+usize::from(style.is_some()),control)?;add(v.get_mut(),"kind",&"text",control)?;add(v.get_mut(),"value",value,control)?;add(v.get_mut(),"at",at,control)?;if let Some(style)=style{add(v.get_mut(),"style",style,control)?;}v},
                Output::Node(DrawNode::Image{at,width,height,mime,bytes})=>{let mut v=object(6,control)?;add(v.get_mut(),"kind",&"image",control)?;add(v.get_mut(),"at",at,control)?;add(v.get_mut(),"width",width,control)?;add(v.get_mut(),"height",height,control)?;add(v.get_mut(),"mime",mime,control)?;add(v.get_mut(),"bytes",bytes,control)?;v},
                Output::Node(DrawNode::Group{transform,children})=>{encode_push(&mut pending,Output::Group(transform,children.len()),control)?;for child in children.iter().rev(){encode_push(&mut pending,Output::Node(child),control)?;control.step()?;}control.step()?;continue;},
                Output::Group(transform,count)=>{let start=built.get().len().checked_sub(count).ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"Drawing projected children"))?;let mut children=DecodedValue::new(control.allocate_vec::<DslValue>(count)?,retire_values);for child in built.get_mut().drain(start..){children.get_mut().push(child);}let mut v=object(3,control)?;add(v.get_mut(),"kind",&"group",control)?;add(v.get_mut(),"transform",transform,control)?;add_owned(v.get_mut(),"children",DecodedValue::new(DslValue::Array(children.take()),<DslValue as FromValue>::retire_decoded),control)?;v},
            };
            if built.get().len()==built.get().capacity(){let additional=built.get().capacity().max(1);control.charge(additional.checked_mul(std::mem::size_of::<DslValue>()).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Drawing projected forest overflow"))?)?;built.get_mut().try_reserve_exact(additional).map_err(|_|ValueError::new(ValueRefusalKind::AllocationFailed,"Drawing projected forest allocation"))?;}
            built.get_mut().push(output.take());control.step()?;
        }
        if built.get().len()!=1{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"Drawing projected root"))}
        Ok(built.get_mut().pop().unwrap())
    })
}
}
pub use snapshot_wire2_codec::*;
