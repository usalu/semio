//! 📝️ Text representation codec surface for `stdio.semio.model` (snapshot).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::model::schema::snapshot::*;
use crate::standards::v1::subsets::base::schema::geometry::native;
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint3, SemioQuaternion, SemioTransform};
use crate::standards::v1::subsets::base::schema::triples::{split_top_level, strip_brackets};
use framework_schema::ArtifactSchema;

/// 🧪️ P2 pilot (model): real hex/bracket-encoded value primitives backing the hand-rolled
/// `ArtifactDsl` below — same style as this subset's own `🔺️diff`/`🧬️mutations` facets
/// (`GifDiff`/`SvgDiff`/`DocxDiff`'s established hand-rolled convention), duplicated here (not
/// imported from `schema::diff`) to keep `snapshot` — the base type `diff`/`mutations` both depend
/// ON — free of a reverse dependency on either sibling facet (same rationale
/// `stdio.semio.flow`'s own snapshot module documents).
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
pub(crate) fn enc_f64(v: f64) -> String {
    native::NativeF64(v).to_string()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_f64(s: &str) -> Result<f64, String> {
    native::parse(s)
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
pub(crate) fn enc_list<T>(items: &[T], enc: impl Fn(&T) -> String) -> String {
    format!("[{}]", items.iter().map(enc).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_list<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Vec<T>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_point3(p: &SemioPoint3) -> String {
    format!("[{},{},{}]", enc_f64(p.x), enc_f64(p.y), enc_f64(p.z))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_point3(s: &str) -> Result<SemioPoint3, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [x, y, z] = parts.as_slice() else { return Err(format!("point3: expected 3 fields, got {}", parts.len())) };
    Ok(SemioPoint3 { x: dec_f64(x)?, y: dec_f64(y)?, z: dec_f64(z)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_quat(q: &SemioQuaternion) -> String {
    format!("[{},{},{},{}]", enc_f64(q.x), enc_f64(q.y), enc_f64(q.z), enc_f64(q.w))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_quat(s: &str) -> Result<SemioQuaternion, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [x, y, z, w] = parts.as_slice() else { return Err(format!("quaternion: expected 4 fields, got {}", parts.len())) };
    Ok(SemioQuaternion { x: dec_f64(x)?, y: dec_f64(y)?, z: dec_f64(z)?, w: dec_f64(w)? })
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
        PsetValue::Number { value } => format!("N[{}]",native::NativeF64(*value)),
        PsetValue::Boolean { value } => format!("B[{}]", if *value { "1" } else { "0" }),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_pset_value(s: &str) -> Result<PsetValue, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "T" => Ok(PsetValue::Text { value: dec_str(inner)? }),
        "N" => Ok(PsetValue::Number { value: dec_f64(inner)? }),
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

/// 📄️ The real structured text body: four lines — `schema=<hex>`, `spatial=[<node>,...]`,
/// `elements=[<element>,...]`, `relations=[<relation>,...]` — matching the grammar's `document =
/// artifact-mark schema-line spatial-line elements-line relations-line`. Newlines are pure lexer
/// trivia in the shared dialect, so this is genuinely recognizable by `dsl::Recognizer`, not merely
/// readable.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_semio_model_snapshot_body(s: &SemioModelSnapshot) -> String {
    format!(
        "schema={}\nspatial=[{}]\nelements=[{}]\nrelations=[{}]",
        enc_str(&s.schema),
        s.spatial.iter().map(enc_spatial_node).collect::<Vec<_>>().join(","),
        s.elements.iter().map(enc_element).collect::<Vec<_>>().join(","),
        s.relations.iter().map(enc_relation).collect::<Vec<_>>().join(","),
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_semio_model_snapshot_body(body: &str) -> Result<SemioModelSnapshot, String> {
    let mut schema = None;
    let mut spatial = Vec::new();
    let mut elements = Vec::new();
    let mut relations = Vec::new();
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("schema=") {
            schema = Some(dec_str(rest)?);
        } else if let Some(rest) = line.strip_prefix("spatial=") {
            let inner = strip_brackets(rest)?;
            spatial = split_top_level(inner, ',').into_iter().filter(|s| !s.is_empty()).map(dec_spatial_node).collect::<Result<Vec<_>, String>>()?;
        } else if let Some(rest) = line.strip_prefix("elements=") {
            let inner = strip_brackets(rest)?;
            elements = split_top_level(inner, ',').into_iter().filter(|s| !s.is_empty()).map(dec_element).collect::<Result<Vec<_>, String>>()?;
        } else if let Some(rest) = line.strip_prefix("relations=") {
            let inner = strip_brackets(rest)?;
            relations = split_top_level(inner, ',').into_iter().filter(|s| !s.is_empty()).map(dec_relation).collect::<Result<Vec<_>, String>>()?;
        } else {
            return Err(format!("model snapshot: unknown line {line:?}"));
        }
    }
    let schema = schema.ok_or_else(|| "model snapshot: missing schema line".to_string())?;
    Ok(SemioModelSnapshot { schema, spatial, elements, relations })
}

/// 🎁 Real structured text/binary codecs (P2 pilot — model subset upgraded off the old
/// hex-dump-of-`serde_json` shortcut, following `stdio.semio.flow`'s proven pattern). Wrapped
/// in the repo-wide `store::semio_format` envelope, unchanged.
impl store::ArtifactDsl for SemioModelSnapshot {
    const EXTENSION: &'static str = "semio";
    fn envelope_id() -> &'static str {
        STDIO_SEMIOMODEL_DOCUMENT_SCHEMA
    }

    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_semio_model_snapshot_body(body).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        let body = print_semio_model_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 🚪️ The four codec entry points above, reachable from OUTSIDE this crate. `store` is a private
/// `extern crate semio_framework_os_kernel as store` alias in `🦀️.rs`, so an external caller —
/// an owner-root test adapter is exactly that — can neither bring `store::ArtifactDsl`/
/// `store::ArtifactPack` into scope nor name `store::TextError`/`store::PackError` in a signature.
/// These four wrappers carry the error across as a plain `String` so the subset's own text and
/// binary envelopes stay drivable end to end (`kit`'s precedent for the same structural gap).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_semio_model_dsl(text: &str) -> Result<SemioModelSnapshot, String> {
    <SemioModelSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|e| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn print_semio_model_dsl(snapshot: &SemioModelSnapshot) -> String {
    <SemioModelSnapshot as store::ArtifactDsl>::print_dsl(snapshot)
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod snapshot_wire_codec {
use super::*;
use crate::standards::v1::subsets::model::schema::snapshot::*;
use crate::standards::v1::subsets::base::schema::geometry::native;
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint3, SemioQuaternion, SemioTransform};
use crate::audio::io::text::diff::{strip_brackets};
use crate::audio::io::text::diff::{split_top_level};
use framework_schema::ArtifactSchema;




}
pub use snapshot_wire_codec::*;
