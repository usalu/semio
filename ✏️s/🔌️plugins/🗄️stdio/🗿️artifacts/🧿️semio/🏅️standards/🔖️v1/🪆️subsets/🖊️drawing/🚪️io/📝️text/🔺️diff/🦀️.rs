//! 📝️ Text representation codec surface for `stdio.semio.drawing` (diff). The real parse/print
//! is `SemioDrawingDiff`'s hand-rolled `protocol::DiffCodec` impl (../🦀️.rs) -- this
//! module exposes the grammar source for tooling/introspection.

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::drawing::schema::diff::*;
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint2, SemioRgba, SemioTransform};
use crate::standards::v1::subsets::base::schema::triples::{dec_indexed_triple, dec_named_triple, enc_indexed_triple, enc_named_triple, IndexAdded, IndexModified, NamedModified};
use crate::audio::io::text::diff::{strip_brackets};
use crate::audio::io::text::diff::{split_top_level};
use crate::standards::v1::subsets::drawing::schema::snapshot::{DrawCanvas, DrawLayer, DrawNode, DrawStyle, PathSegment, SemioDrawingSnapshot};
use crate::document::io::text::diff::{dec_style};
use crate::document::io::text::diff::{enc_style};
use crate::mesh::io::text::diff::{dec_rgba};
use crate::mesh::io::text::diff::{enc_rgba};
use crate::cad::io::text::diff::{dec_layer};
use crate::cad::io::text::diff::{enc_layer};
use crate::flow::io::text::diff::{dec_node};
use crate::flow::io::text::diff::{enc_node};
use crate::cad::io::text::diff::{dec_point2};
use crate::cad::io::text::diff::{enc_point2};
use crate::model::io::text::diff::{dec_transform};
use crate::model::io::text::diff::{enc_transform};
use crate::model::io::text::diff::{dec_list};
use crate::model::io::text::diff::{enc_list};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{dec_path_segment};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{enc_path_segment};
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use crate::standards::v1::subsets::base::schema::triples::IndexedTripleDiff;
use crate::standards::v1::subsets::base::schema::triples::NamedTripleDiff;
use crate::standards::v1::subsets::mesh::io::text::snapshot::enc_str;
use crate::standards::v1::subsets::mesh::io::text::snapshot::dec_str;
use crate::standards::v1::subsets::drawing::io::text::snapshot::enc_canvas;
use crate::standards::v1::subsets::drawing::io::text::snapshot::dec_canvas;
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_drawing_diff(d: &SemioDrawingDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = &d.canvas {
        tokens.push(format!("canvas={}", enc_canvas(v)));
    }
    if let Some(v) = &d.styles {
        tokens.push(format!("styles={}", enc_named_triple(v, |k: &String| enc_str(k), enc_style_diff, enc_style)));
    }
    if let Some(v) = &d.layers {
        tokens.push(format!("layers={}", enc_indexed_triple(v, enc_layer_diff, enc_layer)));
    }
    tokens.join(" ")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_drawing_diff(line: &str) -> Result<SemioDrawingDiff, String> {
    let mut d = SemioDrawingDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("canvas=") {
            d.canvas = Some(dec_canvas(rest)?);
        } else if let Some(rest) = token.strip_prefix("styles=") {
            d.styles = Some(dec_named_triple(rest, dec_str, dec_style_diff, dec_style)?);
        } else if let Some(rest) = token.strip_prefix("layers=") {
            d.layers = Some(dec_indexed_triple(rest, dec_layer_diff, dec_layer)?);
        } else {
            return Err(format!("drawing diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl protocol::DiffText for SemioDrawingDiff {
fn print_diff(&self) -> String {
    print_drawing_diff(self)
}
fn parse_diff(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
    parse_drawing_diff(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
}
}
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
pub(crate) fn enc_node_diff(d: &DrawNodeDiff) -> String {
    match d {
        DrawNodeDiff::Path(p) => format!("P[{},{}]", encode_option(&p.segments, |v| enc_list(v, enc_path_segment)), encode_option(&p.style, |v| encode_option(v, |s| enc_str(s)))),
        DrawNodeDiff::Text(t) => format!("T[{},{},{}]", encode_option(&t.value, |v| enc_str(v)), encode_option(&t.at, enc_point2), encode_option(&t.style, |v| encode_option(v, |s| enc_str(s)))),
        DrawNodeDiff::Group(g) => format!(
            "G[{},{}]",
            encode_option(&g.transform, enc_transform),
            match &g.children {
                Some(c) => format!("[1,{}]", enc_indexed_triple(c, enc_node_diff, enc_node)),
                None => "[0]".to_string(),
            }
        ),
        DrawNodeDiff::Image(i) => {
            format!("I[{},{},{},{},{}]", encode_option(&i.at, enc_point2), encode_option(&i.width, |v| v.to_string()), encode_option(&i.height, |v| v.to_string()), encode_option(&i.mime, |v| enc_str(v)), encode_option(&i.bytes, |v| hex_encode(v)),)
        }
        DrawNodeDiff::Replace { node } => format!("R[{}]", enc_node(node)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_node_diff(s: &str) -> Result<DrawNodeDiff, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "P" => {
            let parts = split_top_level(inner, ',');
            let [segments, style] = parts.as_slice() else { return Err(format!("path diff: expected 2 fields, got {}", parts.len())) };
            Ok(DrawNodeDiff::Path(DrawPathDiff { segments: decode_option(segments, |v| dec_list(v, dec_path_segment))?, style: decode_option(style, |v| decode_option(v, dec_str))? }))
        }
        "T" => {
            let parts = split_top_level(inner, ',');
            let [value, at, style] = parts.as_slice() else { return Err(format!("text diff: expected 3 fields, got {}", parts.len())) };
            Ok(DrawNodeDiff::Text(DrawTextDiff { value: decode_option(value, dec_str)?, at: decode_option(at, dec_point2)?, style: decode_option(style, |v| decode_option(v, dec_str))? }))
        }
        "G" => {
            let parts = split_top_level(inner, ',');
            let [transform_s, children_s] = parts.as_slice() else { return Err(format!("group diff: expected 2 fields, got {}", parts.len())) };
            let transform = decode_option(transform_s, dec_transform)?;
            let children = match split_top_level(strip_brackets(children_s)?, ',').as_slice() {
                ["0"] => None,
                [tag, rest @ ..] if *tag == "1" => Some(dec_indexed_triple(&rest.join(","), dec_node_diff, dec_node)?),
                other => return Err(format!("group children: bad shape {other:?}")),
            };
            Ok(DrawNodeDiff::Group(DrawGroupDiff { transform, children }))
        }
        "I" => {
            let parts = split_top_level(inner, ',');
            let [at, width, height, mime, bytes] = parts.as_slice() else { return Err(format!("image diff: expected 5 fields, got {}", parts.len())) };
            Ok(DrawNodeDiff::Image(DrawImageDiff {
                at: decode_option(at, dec_point2)?,
                width: decode_option(width, |v| v.parse::<f64>().map_err(|e: std::num::ParseFloatError| e.to_string()))?,
                height: decode_option(height, |v| v.parse::<f64>().map_err(|e: std::num::ParseFloatError| e.to_string()))?,
                mime: decode_option(mime, dec_str)?,
                bytes: decode_option(bytes, hex_decode)?,
            }))
        }
        "R" => Ok(DrawNodeDiff::Replace { node: dec_node(inner)? }),
        other => Err(format!("node diff: unknown tag {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_canvas(c: &DrawCanvasDiff) -> String {
    format!("[{},{},{}]", encode_option(&c.width, |v| v.to_string()), encode_option(&c.height, |v| v.to_string()), encode_option(&c.background, |v| encode_option(v, enc_rgba)))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_canvas(s: &str) -> Result<DrawCanvasDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [width, height, background] = parts.as_slice() else { return Err(format!("canvas diff: expected 3 fields, got {}", parts.len())) };
    Ok(DrawCanvasDiff {
        width: decode_option(width, |v| v.parse::<f64>().map_err(|e: std::num::ParseFloatError| e.to_string()))?,
        height: decode_option(height, |v| v.parse::<f64>().map_err(|e: std::num::ParseFloatError| e.to_string()))?,
        background: decode_option(background, |v| decode_option(v, dec_rgba))?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_style_diff(d: &DrawStyleDiff) -> String {
    format!(
        "[{},{},{},{}]",
        encode_option(&d.fill, |v| encode_option(v, enc_rgba)),
        encode_option(&d.stroke, |v| encode_option(v, enc_rgba)),
        encode_option(&d.stroke_width, |v| encode_option(v, |x| x.to_string())),
        encode_option(&d.opacity, |v| encode_option(v, |x| x.to_string())),
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_style_diff(s: &str) -> Result<DrawStyleDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [fill, stroke, stroke_width, opacity] = parts.as_slice() else { return Err(format!("style diff: expected 4 fields, got {}", parts.len())) };
    Ok(DrawStyleDiff {
        fill: decode_option(fill, |v| decode_option(v, dec_rgba))?,
        stroke: decode_option(stroke, |v| decode_option(v, dec_rgba))?,
        stroke_width: decode_option(stroke_width, |v| decode_option(v, |x| x.parse::<f64>().map_err(|e: std::num::ParseFloatError| e.to_string())))?,
        opacity: decode_option(opacity, |v| decode_option(v, |x| x.parse::<f32>().map_err(|e: std::num::ParseFloatError| e.to_string())))?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_layer_diff(d: &DrawLayerDiff) -> String {
    format!("[{},{},{},{}]", encode_option(&d.id, |v| enc_str(v)), encode_option(&d.name, |v| enc_str(v)), encode_option(&d.visible, |v| if *v { "1".to_string() } else { "0".to_string() }), encode_option(&d.root, enc_node_diff))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_layer_diff(s: &str) -> Result<DrawLayerDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, name, visible, root] = parts.as_slice() else { return Err(format!("layer diff: expected 4 fields, got {}", parts.len())) };
    Ok(DrawLayerDiff { id: decode_option(id, dec_str)?, name: decode_option(name, dec_str)?, visible: decode_option(visible, |v| Ok(v == "1"))?, root: decode_option(root, dec_node_diff)? })
}
}
pub use diff_codec::*;
