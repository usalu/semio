//! 📝️ Text representation codec surface for `stdio.semio.drawing` (snapshot). The real parse/
//! print lives on `SemioDrawingSnapshot`'s `pack::snapshot::ArtifactDsl` impl (📸️snapshot/🦀️.rs)
//! -- this module exposes the grammar source for tooling/introspection, matching svg's own
//! `📝️text/🦀️.rs` convention.

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::schema::snapshot::*;
use semio_s_artifact_stdio_semio_base::schema::geometry::native::NativeF64;
use semio_s_artifact_stdio_semio_base::schema::geometry::native::NativeF32;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint3;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioQuaternion;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioRgba;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioTransform;
use semio_s_artifact_stdio_semio_base::io::text::codec::split_top_level;
use semio_s_artifact_stdio_semio_base::io::text::codec::strip_brackets;
use framework_schema::ArtifactSchema;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint2;

use semio_s_artifact_stdio_semio_base::io::text::codec::hex_encode;
use semio_s_artifact_stdio_semio_base::io::text::codec::hex_decode;
use semio_s_artifact_stdio_semio_base::io::text::codec::enc_str;
use semio_s_artifact_stdio_semio_base::io::text::codec::dec_str;
use semio_s_artifact_stdio_semio_base::io::text::codec::parse_f64;
use semio_s_artifact_stdio_semio_base::io::text::codec::parse_f32;
use semio_s_artifact_stdio_semio_base::io::text::codec::enc_bool;
use semio_s_artifact_stdio_semio_base::io::text::codec::parse_bool;
use semio_s_artifact_stdio_semio_base::io::text::codec::enc_list;
use semio_s_artifact_stdio_semio_base::io::text::codec::dec_list;
use semio_s_artifact_stdio_semio_base::io::text::codec::encode_option;
use semio_s_artifact_stdio_semio_base::io::text::codec::decode_option;
use semio_s_artifact_stdio_semio_base::io::text::codec::enc_point2;
use semio_s_artifact_stdio_semio_base::io::text::codec::dec_point2;
use semio_s_artifact_stdio_semio_base::io::text::codec::enc_transform;
use semio_s_artifact_stdio_semio_base::io::text::codec::dec_transform;
use semio_s_artifact_stdio_semio_base::io::text::codec::enc_rgba;
use semio_s_artifact_stdio_semio_base::io::text::codec::dec_rgba;

/// 📐️ `M[to]` (MoveTo) / `L[to]` (LineTo) / `C[c1,c2,to]` (CubicTo) / `Q[c,to]` (QuadTo) /
/// `A[rx,ry,xRotation,largeArc,sweep,to]` (ArcTo) / `Z` (Close, no payload).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_path_segment(seg: &PathSegment) -> String {
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
pub fn dec_path_segment(s: &str) -> Result<PathSegment, String> {
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
pub fn enc_node(n: &DrawNode) -> String {
    match n {
        DrawNode::Path { segments, style } => format!("P[{},{}]", enc_list(segments, enc_path_segment), encode_option(style, |s| enc_str(s))),
        DrawNode::Text { value, at, style } => format!("T[{},{},{}]", enc_str(value), enc_point2(at), encode_option(style, |s| enc_str(s))),
        DrawNode::Group { transform, children } => format!("G[{},{}]", enc_transform(transform), enc_list(children, enc_node)),
        DrawNode::Image { at, width, height, mime, bytes } => format!("I[{},{},{},{},{}]", enc_point2(at), NativeF64(*width), NativeF64(*height), enc_str(mime), hex_encode(bytes)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_node(s: &str) -> Result<DrawNode, String> {
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
pub fn enc_style(s: &DrawStyle) -> String {
    format!("[{},{},{},{},{}]", enc_str(&s.name), encode_option(&s.fill, enc_rgba), encode_option(&s.stroke, enc_rgba), encode_option(&s.stroke_width, |v: &f64| NativeF64(*v).to_string()), encode_option(&s.opacity, |v: &f32| NativeF32(*v).to_string()),)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_style(s: &str) -> Result<DrawStyle, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, fill, stroke, stroke_width, opacity] = parts.as_slice() else { return Err(format!("style: expected 5 fields, got {}", parts.len())) };
    Ok(DrawStyle { name: dec_str(name)?, fill: decode_option(fill, dec_rgba)?, stroke: decode_option(stroke, dec_rgba)?, stroke_width: decode_option(stroke_width, parse_f64)?, opacity: decode_option(opacity, parse_f32)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_layer(l: &DrawLayer) -> String {
    format!("[{},{},{},{}]", enc_str(&l.id), enc_str(&l.name), enc_bool(l.visible), enc_node(&l.root))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_layer(s: &str) -> Result<DrawLayer, String> {
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
impl pack::snapshot::ArtifactDsl for SemioDrawingSnapshot {
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
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
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
/// pass-through of this snapshot's own `pack::snapshot::ArtifactDsl` impl above, whose trait and error type
/// are both unnameable outside this crate, so `🖊️mutate-semio-drawing`'s `identity-round-trip`
/// scenario reaches the real committed sketch artifact through this instead.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_semio_drawing_dsl(text: &str) -> Result<SemioDrawingSnapshot, String> {
    <SemioDrawingSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text).map_err(|error| error.to_string())
}

/// 📝️ Renders a [`SemioDrawingSnapshot`] back as DSL text — the inverse of
/// [`parse_semio_drawing_dsl`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn print_semio_drawing_dsl(snapshot: &SemioDrawingSnapshot) -> String {
    pack::snapshot::ArtifactDsl::print_dsl(snapshot)
}
}
pub use snapshot_codec::*;

