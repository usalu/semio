//! 📝️ Text representation codec surface for `stdio.semio.model` (diff).

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::model::schema::diff::*;
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint3, SemioQuaternion, SemioTransform};
use crate::standards::v1::subsets::base::schema::triples::{dec_named_triple, enc_named_triple, NamedModified, NamedTripleDiff};
use crate::audio::io::text::diff::{strip_brackets};
use crate::audio::io::text::diff::{split_top_level};
use crate::standards::v1::subsets::model::schema::snapshot::{ElementClass, GeometryRef, ModelRelation, Property, PropertySet, PsetValue, RelationKind, SemioModelElement, SemioModelSnapshot, SpatialKind, SpatialNode};
use protocol::command::DiffAlgebra;
use protocol::{DiffBinary,DiffCodec,DiffText, MutationDiff};
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_semio_model_diff(d: &SemioModelDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = &d.spatial {
        tokens.push(format!("spatial={}", enc_spatial_diff(v)));
    }
    if let Some(v) = &d.elements {
        tokens.push(format!("elements={}", enc_elements_diff(v)));
    }
    if let Some(v) = &d.relations {
        tokens.push(format!("relations={}", enc_relations_diff(v)));
    }
    tokens.join(" ")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_semio_model_diff(line: &str) -> Result<SemioModelDiff, String> {
    let mut d = SemioModelDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("spatial=") {
            d.spatial = Some(dec_spatial_diff(rest)?);
        } else if let Some(rest) = token.strip_prefix("elements=") {
            d.elements = Some(dec_elements_diff(rest)?);
        } else if let Some(rest) = token.strip_prefix("relations=") {
            d.relations = Some(dec_relations_diff(rest)?);
        } else {
            return Err(format!("semio model diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl protocol::DiffText for SemioModelDiff {
fn print_diff(&self) -> String {
    print_semio_model_diff(self)
}
fn parse_diff(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
    parse_semio_model_diff(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
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
pub(crate) fn enc_list<T>(items: &[T], enc: impl Fn(&T) -> String) -> String {
    format!("[{}]", items.iter().map(enc).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_list<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Vec<T>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec).collect()
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
pub(crate) fn enc_quat(q: &SemioQuaternion) -> String {
    format!("[{},{},{},{}]", q.x, q.y, q.z, q.w)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_quat(s: &str) -> Result<SemioQuaternion, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [x, y, z, w] = parts.as_slice() else { return Err(format!("quaternion: expected 4 fields, got {}", parts.len())) };
    Ok(SemioQuaternion { x: parse_f64(x)?, y: parse_f64(y)?, z: parse_f64(z)?, w: parse_f64(w)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_transform(t: &SemioTransform) -> String {
    format!("[{},{},{}]", enc_point3(&t.translation), enc_quat(&t.rotation), enc_point3(&t.scale))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_transform(s: &str) -> Result<SemioTransform, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [translation, rotation, scale] = parts.as_slice() else { return Err(format!("transform: expected 3 fields, got {}", parts.len())) };
    Ok(SemioTransform { translation: dec_point3(translation)?, rotation: dec_quat(rotation)?, scale: dec_point3(scale)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_spatial_kind(k: &SpatialKind) -> &'static str {
    match k {
        SpatialKind::Site => "S",
        SpatialKind::Building => "B",
        SpatialKind::Storey => "T",
        SpatialKind::Space => "P",
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_spatial_kind(s: &str) -> Result<SpatialKind, String> {
    match s {
        "S" => Ok(SpatialKind::Site),
        "B" => Ok(SpatialKind::Building),
        "T" => Ok(SpatialKind::Storey),
        "P" => Ok(SpatialKind::Space),
        other => Err(format!("spatial kind: unknown tag {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_element_class(c: &ElementClass) -> String {
    match c {
        ElementClass::Wall => "WA".to_string(),
        ElementClass::Slab => "SL".to_string(),
        ElementClass::Column => "CO".to_string(),
        ElementClass::Beam => "BE".to_string(),
        ElementClass::Door => "DO".to_string(),
        ElementClass::Window => "WI".to_string(),
        ElementClass::Roof => "RO".to_string(),
        ElementClass::Stair => "ST".to_string(),
        ElementClass::Furniture => "FU".to_string(),
        ElementClass::Other { name } => format!("OT[{}]", enc_str(name)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_element_class(s: &str) -> Result<ElementClass, String> {
    match s {
        "WA" => Ok(ElementClass::Wall),
        "SL" => Ok(ElementClass::Slab),
        "CO" => Ok(ElementClass::Column),
        "BE" => Ok(ElementClass::Beam),
        "DO" => Ok(ElementClass::Door),
        "WI" => Ok(ElementClass::Window),
        "RO" => Ok(ElementClass::Roof),
        "ST" => Ok(ElementClass::Stair),
        "FU" => Ok(ElementClass::Furniture),
        other if other.starts_with("OT[") => Ok(ElementClass::Other { name: dec_str(strip_brackets(&other[2..])?)? }),
        other => Err(format!("element class: unknown tag {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_geometry_ref(g: &GeometryRef) -> String {
    match g {
        GeometryRef::None => "N".to_string(),
        GeometryRef::Brep { brep_id } => format!("B[{}]", enc_str(brep_id)),
        GeometryRef::Mesh { mesh_id } => format!("M[{}]", enc_str(mesh_id)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_geometry_ref(s: &str) -> Result<GeometryRef, String> {
    if s == "N" {
        return Ok(GeometryRef::None);
    }
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "B" => Ok(GeometryRef::Brep { brep_id: dec_str(inner)? }),
        "M" => Ok(GeometryRef::Mesh { mesh_id: dec_str(inner)? }),
        other => Err(format!("geometry ref: unknown tag {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_pset_value(v: &PsetValue) -> String {
    match v {
        PsetValue::Text { value } => format!("T[{}]", enc_str(value)),
        PsetValue::Number { value } => format!("N[{value}]"),
        PsetValue::Boolean { value } => format!("B[{}]", if *value { "1" } else { "0" }),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_pset_value(s: &str) -> Result<PsetValue, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "T" => Ok(PsetValue::Text { value: dec_str(inner)? }),
        "N" => Ok(PsetValue::Number { value: parse_f64(inner)? }),
        "B" => Ok(PsetValue::Boolean { value: inner == "1" }),
        other => Err(format!("pset value: unknown tag {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_property(p: &Property) -> String {
    format!("[{},{}]", enc_str(&p.key), enc_pset_value(&p.value))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_property(s: &str) -> Result<Property, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [key, value] = parts.as_slice() else { return Err(format!("property: expected 2 fields, got {}", parts.len())) };
    Ok(Property { key: dec_str(key)?, value: dec_pset_value(value)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_property_set(ps: &PropertySet) -> String {
    format!("[{},{}]", enc_str(&ps.name), enc_list(&ps.properties, enc_property))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_property_set(s: &str) -> Result<PropertySet, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, properties] = parts.as_slice() else { return Err(format!("property set: expected 2 fields, got {}", parts.len())) };
    Ok(PropertySet { name: dec_str(name)?, properties: dec_list(properties, dec_property)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_spatial_node(n: &SpatialNode) -> String {
    format!("[{},{},{},{},{}]", enc_str(&n.id), enc_spatial_kind(&n.kind), enc_str(&n.name), encode_option(&n.parent_id, |v: &String| enc_str(v)), enc_transform(&n.placement))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_spatial_node(s: &str) -> Result<SpatialNode, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, kind, name, parent_id, placement] = parts.as_slice() else { return Err(format!("spatial node: expected 5 fields, got {}", parts.len())) };
    Ok(SpatialNode { id: dec_str(id)?, kind: dec_spatial_kind(kind)?, name: dec_str(name)?, parent_id: decode_option(parent_id, dec_str)?, placement: dec_transform(placement)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_element(e: &SemioModelElement) -> String {
    format!("[{},{},{},{},{},{}]", enc_str(&e.id), enc_element_class(&e.class), enc_transform(&e.placement), enc_geometry_ref(&e.geometry), encode_option(&e.spatial_id, |v: &String| enc_str(v)), enc_list(&e.psets, enc_property_set),)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_element(s: &str) -> Result<SemioModelElement, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, class, placement, geometry, spatial_id, psets] = parts.as_slice() else { return Err(format!("element: expected 6 fields, got {}", parts.len())) };
    Ok(SemioModelElement { id: dec_str(id)?, class: dec_element_class(class)?, placement: dec_transform(placement)?, geometry: dec_geometry_ref(geometry)?, spatial_id: decode_option(spatial_id, dec_str)?, psets: dec_list(psets, dec_property_set)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_relation_kind(k: &RelationKind) -> String {
    match k {
        RelationKind::Aggregates => "AG".to_string(),
        RelationKind::ContainedIn => "CI".to_string(),
        RelationKind::ConnectsTo => "CN".to_string(),
        RelationKind::FillsVoid => "FV".to_string(),
        RelationKind::VoidsElement => "VE".to_string(),
        RelationKind::Other { label } => format!("OT[{}]", enc_str(label)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_relation_kind(s: &str) -> Result<RelationKind, String> {
    match s {
        "AG" => Ok(RelationKind::Aggregates),
        "CI" => Ok(RelationKind::ContainedIn),
        "CN" => Ok(RelationKind::ConnectsTo),
        "FV" => Ok(RelationKind::FillsVoid),
        "VE" => Ok(RelationKind::VoidsElement),
        other if other.starts_with("OT[") => Ok(RelationKind::Other { label: dec_str(strip_brackets(&other[2..])?)? }),
        other => Err(format!("relation kind: unknown tag {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_relation(r: &ModelRelation) -> String {
    format!("[{},{},{},{}]", enc_str(&r.id), enc_relation_kind(&r.kind), enc_str(&r.from), enc_str(&r.to))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_relation(s: &str) -> Result<ModelRelation, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, kind, from, to] = parts.as_slice() else { return Err(format!("relation: expected 4 fields, got {}", parts.len())) };
    Ok(ModelRelation { id: dec_str(id)?, kind: dec_relation_kind(kind)?, from: dec_str(from)?, to: dec_str(to)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_spatial_node_diff(d: &SpatialNodeDiff) -> String {
    format!(
        "[{},{},{},{}]",
        encode_option(&d.kind, |v: &SpatialKind| enc_spatial_kind(v).to_string()),
        encode_option(&d.name, |v: &String| enc_str(v)),
        encode_option(&d.parent_id, |inner: &Option<String>| encode_option(inner, |v: &String| enc_str(v))),
        encode_option(&d.placement, enc_transform),
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_spatial_node_diff(s: &str) -> Result<SpatialNodeDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [kind, name, parent_id, placement] = parts.as_slice() else { return Err(format!("spatial node diff: expected 4 fields, got {}", parts.len())) };
    Ok(SpatialNodeDiff { kind: decode_option(kind, dec_spatial_kind)?, name: decode_option(name, dec_str)?, parent_id: decode_option(parent_id, |s| decode_option(s, dec_str))?, placement: decode_option(placement, dec_transform)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_element_diff(d: &SemioModelElementDiff) -> String {
    format!(
        "[{},{},{},{},{}]",
        encode_option(&d.class, enc_element_class),
        encode_option(&d.placement, enc_transform),
        encode_option(&d.geometry, enc_geometry_ref),
        encode_option(&d.spatial_id, |inner: &Option<String>| encode_option(inner, |v: &String| enc_str(v))),
        encode_option(&d.psets, |v: &Vec<PropertySet>| enc_list(v, enc_property_set)),
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_element_diff(s: &str) -> Result<SemioModelElementDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [class, placement, geometry, spatial_id, psets] = parts.as_slice() else { return Err(format!("element diff: expected 5 fields, got {}", parts.len())) };
    Ok(SemioModelElementDiff {
        class: decode_option(class, dec_element_class)?,
        placement: decode_option(placement, dec_transform)?,
        geometry: decode_option(geometry, dec_geometry_ref)?,
        spatial_id: decode_option(spatial_id, |s| decode_option(s, dec_str))?,
        psets: decode_option(psets, |s| dec_list(s, dec_property_set))?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_relation_diff(d: &ModelRelationDiff) -> String {
    format!("[{},{},{}]", encode_option(&d.kind, enc_relation_kind), encode_option(&d.from, |v: &String| enc_str(v)), encode_option(&d.to, |v: &String| enc_str(v)))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_relation_diff(s: &str) -> Result<ModelRelationDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [kind, from, to] = parts.as_slice() else { return Err(format!("relation diff: expected 3 fields, got {}", parts.len())) };
    Ok(ModelRelationDiff { kind: decode_option(kind, dec_relation_kind)?, from: decode_option(from, dec_str)?, to: decode_option(to, dec_str)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_spatial_diff(d: &SpatialDiff) -> String {
    enc_named_triple(d, |k: &String| enc_str(k), enc_spatial_node_diff, enc_spatial_node)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_spatial_diff(s: &str) -> Result<SpatialDiff, String> {
    dec_named_triple(s, dec_str, dec_spatial_node_diff, dec_spatial_node)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_elements_diff(d: &ElementsDiff) -> String {
    enc_named_triple(d, |k: &String| enc_str(k), enc_element_diff, enc_element)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_elements_diff(s: &str) -> Result<ElementsDiff, String> {
    dec_named_triple(s, dec_str, dec_element_diff, dec_element)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_relations_diff(d: &RelationsDiff) -> String {
    enc_named_triple(d, |k: &String| enc_str(k), enc_relation_diff, enc_relation)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_relations_diff(s: &str) -> Result<RelationsDiff, String> {
    dec_named_triple(s, dec_str, dec_relation_diff, dec_relation)
}
}
pub use diff_codec::*;
