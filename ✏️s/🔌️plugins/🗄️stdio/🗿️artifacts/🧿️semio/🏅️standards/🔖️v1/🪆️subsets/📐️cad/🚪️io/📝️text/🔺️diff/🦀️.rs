//! 📝️ Text representation grammar surface for `s.stdio.semio.cad.diff`.

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v1::subsets::cad::io::binary::diff::{encode_option, decode_option};
use crate::standards::v1::subsets::cad::schema::diff::*;
use protocol::{DiffText,DiffBinary};
use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use crate::standards::v1::subsets::base::schema::triples::{NamedModified, NamedTripleDiff};
use crate::standards::v1::subsets::base::io::text::snapshot::{dec_named_added, dec_named_triple, enc_named_added, enc_named_triple};
use crate::standards::v1::subsets::audio::io::text::diff::{strip_brackets};
use crate::standards::v1::subsets::audio::io::text::diff::{split_top_level};
use crate::standards::v1::subsets::cad::schema::snapshot::{CadBlock, CadEntity, CadEntityRecord, CadLayer, SemioCadSnapshot};
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_cad_diff(d: &SemioCadDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(l) = &d.layers {
        tokens.push(format!("layers={}", enc_named_triple(l, |k: &String| enc_str(k), enc_layer_diff, |a| enc_named_added(a, enc_layer))));
    }
    if let Some(b) = &d.blocks {
        tokens.push(format!("blocks={}", enc_named_triple(b, |k: &String| enc_str(k), enc_block_diff, |a| enc_named_added(a, enc_block))));
    }
    if let Some(e) = &d.entities {
        tokens.push(format!("entities={}", enc_named_triple(e, |k: &String| enc_str(k), enc_entity_record_diff, |a| enc_named_added(a, enc_entity_record))));
    }
    tokens.join(" ")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_cad_diff(line: &str) -> Result<SemioCadDiff, String> {
    let mut d = SemioCadDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("layers=") {
            d.layers = Some(dec_named_triple(rest, dec_str, dec_layer_diff, |t| dec_named_added(t, dec_layer))?);
        } else if let Some(rest) = token.strip_prefix("blocks=") {
            d.blocks = Some(dec_named_triple(rest, dec_str, dec_block_diff, |t| dec_named_added(t, dec_block))?);
        } else if let Some(rest) = token.strip_prefix("entities=") {
            d.entities = Some(dec_named_triple(rest, dec_str, dec_entity_record_diff, |t| dec_named_added(t, dec_entity_record))?);
        } else {
            return Err(format!("cad diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl protocol::DiffText for SemioCadDiff {
fn print_diff(&self) -> String {
    print_cad_diff(self)
}
fn parse_diff(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
    parse_cad_diff(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
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
pub(crate) fn parse_f64(s: &str) -> Result<f64, String> {
    s.parse().map_err(|e: std::num::ParseFloatError| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_i32(s: &str) -> Result<i32, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_list<T>(items: &[T], enc: impl Fn(&T) -> String) -> String {
    format!("[{}]", items.iter().map(enc).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_list<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Vec<T>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec).collect()
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

/// 📐️ `L`ine/`A`rc/`C`ircle/`E`llipse/`P`olyline/`T`ext/`I`nsert/`S`olid/`D`imension — the `dxf`
/// r12 `xs:choice`-equivalent made concrete (single-letter tag, same convention bcf's `enc_camera`
/// established).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_entity(e: &CadEntity) -> String {
    match e {
        CadEntity::Line { a, b } => format!("L[{},{}]", enc_point2(a), enc_point2(b)),
        CadEntity::Arc { center, radius, start_angle, end_angle } => format!("A[{},{},{},{}]", enc_point2(center), radius, start_angle, end_angle),
        CadEntity::Circle { center, radius } => format!("C[{},{}]", enc_point2(center), radius),
        CadEntity::Ellipse { center, major_axis_end, ratio, start_param, end_param } => {
            format!("E[{},{},{},{},{}]", enc_point2(center), enc_point2(major_axis_end), ratio, start_param, end_param)
        }
        CadEntity::Polyline { vertices, closed } => format!("P[{},{}]", enc_list(vertices, enc_point2), if *closed { "1" } else { "0" }),
        CadEntity::Text { position, height, rotation, content } => format!("T[{},{},{},{}]", enc_point2(position), height, rotation, enc_str(content)),
        CadEntity::Insert { block_name, insertion_point, scale, rotation } => format!("I[{},{},{},{}]", enc_str(block_name), enc_point2(insertion_point), enc_point2(scale), rotation),
        CadEntity::Solid { p1, p2, p3, p4 } => format!("S[{},{},{},{}]", enc_point2(p1), enc_point2(p2), enc_point2(p3), enc_point2(p4)),
        CadEntity::Dimension { def_point, text_position, measurement, text } => format!("D[{},{},{},{}]", enc_point2(def_point), enc_point2(text_position), measurement, enc_str(text)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_entity(s: &str) -> Result<CadEntity, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    let parts = split_top_level(inner, ',');
    match tag {
        "L" => {
            let [a, b] = parts.as_slice() else { return Err(format!("line: expected 2 fields, got {}", parts.len())) };
            Ok(CadEntity::Line { a: dec_point2(a)?, b: dec_point2(b)? })
        }
        "A" => {
            let [center, radius, start_angle, end_angle] = parts.as_slice() else { return Err(format!("arc: expected 4 fields, got {}", parts.len())) };
            Ok(CadEntity::Arc { center: dec_point2(center)?, radius: parse_f64(radius)?, start_angle: parse_f64(start_angle)?, end_angle: parse_f64(end_angle)? })
        }
        "C" => {
            let [center, radius] = parts.as_slice() else { return Err(format!("circle: expected 2 fields, got {}", parts.len())) };
            Ok(CadEntity::Circle { center: dec_point2(center)?, radius: parse_f64(radius)? })
        }
        "E" => {
            let [center, major_axis_end, ratio, start_param, end_param] = parts.as_slice() else { return Err(format!("ellipse: expected 5 fields, got {}", parts.len())) };
            Ok(CadEntity::Ellipse { center: dec_point2(center)?, major_axis_end: dec_point2(major_axis_end)?, ratio: parse_f64(ratio)?, start_param: parse_f64(start_param)?, end_param: parse_f64(end_param)? })
        }
        "P" => {
            let [vertices, closed] = parts.as_slice() else { return Err(format!("polyline: expected 2 fields, got {}", parts.len())) };
            Ok(CadEntity::Polyline { vertices: dec_list(vertices, dec_point2)?, closed: *closed == "1" })
        }
        "T" => {
            let [position, height, rotation, content] = parts.as_slice() else { return Err(format!("text: expected 4 fields, got {}", parts.len())) };
            Ok(CadEntity::Text { position: dec_point2(position)?, height: parse_f64(height)?, rotation: parse_f64(rotation)?, content: dec_str(content)? })
        }
        "I" => {
            let [block_name, insertion_point, scale, rotation] = parts.as_slice() else { return Err(format!("insert: expected 4 fields, got {}", parts.len())) };
            Ok(CadEntity::Insert { block_name: dec_str(block_name)?, insertion_point: dec_point2(insertion_point)?, scale: dec_point2(scale)?, rotation: parse_f64(rotation)? })
        }
        "S" => {
            let [p1, p2, p3, p4] = parts.as_slice() else { return Err(format!("solid: expected 4 fields, got {}", parts.len())) };
            Ok(CadEntity::Solid { p1: dec_point2(p1)?, p2: dec_point2(p2)?, p3: dec_point2(p3)?, p4: dec_point2(p4)? })
        }
        "D" => {
            let [def_point, text_position, measurement, text] = parts.as_slice() else { return Err(format!("dimension: expected 4 fields, got {}", parts.len())) };
            Ok(CadEntity::Dimension { def_point: dec_point2(def_point)?, text_position: dec_point2(text_position)?, measurement: parse_f64(measurement)?, text: dec_str(text)? })
        }
        other => Err(format!("entity: unknown tag {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_layer(l: &CadLayer) -> String {
    format!("[{},{},{},{}]", enc_str(&l.name), l.color_index, enc_str(&l.line_type), if l.visible { "1" } else { "0" })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_layer(s: &str) -> Result<CadLayer, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, color_index, line_type, visible] = parts.as_slice() else { return Err(format!("layer: expected 4 fields, got {}", parts.len())) };
    Ok(CadLayer { name: dec_str(name)?, color_index: parse_i32(color_index)?, line_type: dec_str(line_type)?, visible: *visible == "1" })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_entity_record(r: &CadEntityRecord) -> String {
    format!("[{},{},{}]", enc_str(&r.handle), enc_str(&r.layer), enc_entity(&r.entity))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_entity_record(s: &str) -> Result<CadEntityRecord, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [handle, layer, entity] = parts.as_slice() else { return Err(format!("entity record: expected 3 fields, got {}", parts.len())) };
    Ok(CadEntityRecord { handle: dec_str(handle)?, layer: dec_str(layer)?, entity: dec_entity(entity)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_block(b: &CadBlock) -> String {
    format!("[{},{},{}]", enc_str(&b.name), enc_point2(&b.base_point), enc_list(&b.entities, enc_entity_record))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_block(s: &str) -> Result<CadBlock, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, base_point, entities] = parts.as_slice() else { return Err(format!("block: expected 3 fields, got {}", parts.len())) };
    Ok(CadBlock { name: dec_str(name)?, base_point: dec_point2(base_point)?, entities: dec_list(entities, dec_entity_record)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_layer_diff(d: &CadLayerDiff) -> String {
    format!("[{},{},{}]", encode_option(&d.color_index, |v: &i32| v.to_string()), encode_option(&d.line_type, |v: &String| enc_str(v)), encode_option(&d.visible, |v: &bool| if *v { "1".to_string() } else { "0".to_string() }),)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_layer_diff(s: &str) -> Result<CadLayerDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [color_index, line_type, visible] = parts.as_slice() else { return Err(format!("layer diff: expected 3 fields, got {}", parts.len())) };
    Ok(CadLayerDiff { color_index: decode_option(color_index, parse_i32)?, line_type: decode_option(line_type, dec_str)?, visible: decode_option(visible, |v| Ok(v == "1"))? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_entity_record_diff(d: &CadEntityRecordDiff) -> String {
    format!("[{},{}]", encode_option(&d.layer, |v: &String| enc_str(v)), encode_option(&d.entity, enc_entity))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_entity_record_diff(s: &str) -> Result<CadEntityRecordDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [layer, entity] = parts.as_slice() else { return Err(format!("entity record diff: expected 2 fields, got {}", parts.len())) };
    Ok(CadEntityRecordDiff { layer: decode_option(layer, dec_str)?, entity: decode_option(entity, dec_entity)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_block_diff(d: &CadBlockDiff) -> String {
    format!("[{},{}]", encode_option(&d.base_point, |p: &SemioPoint2| enc_point2(p)), encode_option(&d.entities, |v: &CadEntitiesDiff| enc_named_triple(v, |k: &String| enc_str(k), enc_entity_record_diff, |a| enc_named_added(a, enc_entity_record))),)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_block_diff(s: &str) -> Result<CadBlockDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [base_point, entities] = parts.as_slice() else { return Err(format!("block diff: expected 2 fields, got {}", parts.len())) };
    Ok(CadBlockDiff { base_point: decode_option(base_point, dec_point2)?, entities: decode_option(entities, |v| dec_named_triple(v, dec_str, dec_entity_record_diff, |t| dec_named_added(t, dec_entity_record)))? })
}
}
pub use diff_codec::*;
