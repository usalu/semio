//! 🧬️ SemioModelSnapshot — complete per the master plan's `model` row: a flat, id-keyed spatial
//! hierarchy (site/building/storey/space, parent-pointer graph — mirrors ifc/4's
//! `IfcRelAggregates`/`IfcRelContainedInSpatialStructure` shape rather than a recursive tree type)
//! + elements (typed class enum, placement, a BY-ID `GeometryRef` into the sibling `brep`/`mesh`
//! subsets, and named property sets) + relations (typed kind enum, from/to id endpoints). Owned by
//! `model` (w1b-type-ownership.md): `SemioModelElement`, `GeometryRef`, plus this file's own
//! `SpatialNode`/`ModelRelation`/`ElementClass`/`PropertySet`/`Property`/`PsetValue`/
//! `RelationKind`/`SpatialKind`. `model` never inlines brep/mesh geometry data — `GeometryRef`
//! resolves by id into those subsets' own snapshots (spec-mandated cross-reuse, master plan
//! Architecture section); referential integrity of THAT cross-subset link is out of this
//! snapshot's own scope (it is not decodable from `model` alone), but every reference WITHIN this
//! subset's own collections (spatial parent pointers, element→spatial containment, relation
//! endpoints) is checked by the composer's `SemioModelValidator`.

use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::{SemioPoint3, SemioQuaternion, SemioTransform};
use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::{split_top_level, strip_brackets};
use schema::ArtifactSchema;

//#region 🔖️Ids
pub const STDIO_SEMIOMODEL_DOCUMENT_SCHEMA: &str = "stdio.semio.model";
//#endregion 🔖️Ids

//#region 🔖️Spatial
/// 🏢️ ifc/4 spatial-structure levels this subset targets (`IfcSite`/`IfcBuilding`/
/// `IfcBuildingStorey`/`IfcSpace`) — the master plan's exact "(site/building/storey/space)" list.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum SpatialKind {
    #[default]
    Site,
    Building,
    Storey,
    Space,
}

/// 🌳️ One spatial-tree node. The tree itself is expressed as a flat id-keyed collection with a
/// `parent_id` pointer (not a recursive struct) — matches how ifc's own spatial containment is a
/// graph of `IfcRelAggregates` edges over flat entities, not a nested Rust type.
/// 🧪️ `Default` is a technical workaround, never a meaningful "empty node" -- a known
/// `serde_derive` limitation (`#[value(default)]` on the shared `🧰️triples::NamedTripleDiff`'s
/// `added: Vec<T>` field spuriously infers `T: Default`, same root cause bcf's own diff module
/// documents) means every strong-entity type reachable through a `NamedTripleDiff<K,D,T>` needs
/// `Default` purely to satisfy that derive, not because any real code constructs a default one.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SpatialNode {
    pub id: String,
    pub kind: SpatialKind,
    pub name: String,
    #[value(default)]
    pub parent_id: Option<String>,
    #[value(default)]
    pub placement: SemioTransform,
}
//#endregion 🔖️Spatial

//#region 🔖️Element
/// 🧱️ Real, named IFC-style element classes plus an honest `Other{name}` catch-all for a class
/// this subset hasn't named yet — carries the REAL class name rather than silently collapsing it
/// (never a lying black-hole variant).
/// 🧪️ `Default` (first variant, `Wall`) is the same `serde_derive` technical workaround as
/// `SpatialKind`'s -- see `SpatialNode`'s doc comment.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum ElementClass {
    #[default]
    Wall,
    Slab,
    Column,
    Beam,
    Door,
    Window,
    Roof,
    Stair,
    Furniture,
    Other {
        name: String,
    },
}

/// 📐️ Owned by `model`: geometry reference resolved BY ID into a sibling subset's own snapshot
/// (`brep`/`mesh`) — never inline duplication (w1b-type-ownership.md cross-reuse summary). Named
/// variants throughout, never a bare tuple (f6-final-summary.md §4.3).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum GeometryRef {
    #[default]
    None,
    Brep {
        brep_id: String,
    },
    Mesh {
        mesh_id: String,
    },
}

/// 🏷️ IFC property-set value — weak value type, whole-value replaced in diffs, never sub-diffed
/// (schema-design.md's strong/weak entity split).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum PsetValue {
    Text { value: String },
    Number { value: f64 },
    Boolean { value: bool },
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Property {
    pub key: String,
    pub value: PsetValue,
}

/// 📦️ IFC "Pset_*"-shaped named property bag.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PropertySet {
    pub name: String,
    #[value(default)]
    pub properties: Vec<Property>,
}

/// 🏛️ Owned by `model`: one spatial/physical element — the master plan's
/// "elements{class enum, placement, GeometryRef{Brep|Mesh|None}, psets}". `Default` is the same
/// `serde_derive` technical workaround as `SpatialNode`'s (see that struct's doc comment).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SemioModelElement {
    pub id: String,
    pub class: ElementClass,
    #[value(default)]
    pub placement: SemioTransform,
    #[value(default)]
    pub geometry: GeometryRef,
    /// 🗺️ Which `SpatialNode` (by id) contains this element — `None` = not yet placed in the
    /// spatial tree. Checked for dangling references by `SemioModelValidator`.
    #[value(default)]
    pub spatial_id: Option<String>,
    #[value(default)]
    pub psets: Vec<PropertySet>,
}
//#endregion 🔖️Element

//#region 🔖️Relation
/// 🔗️ IFC-style relationship kinds between two ids (elements and/or spatial nodes) plus an honest
/// `Other{label}` catch-all, same rationale as `ElementClass::Other`.
/// 🧪️ `Default` (first variant, `Aggregates`) is the same `serde_derive` technical workaround as
/// `SpatialKind`'s -- see `SpatialNode`'s doc comment.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum RelationKind {
    #[default]
    Aggregates,
    ContainedIn,
    ConnectsTo,
    FillsVoid,
    VoidsElement,
    Other {
        label: String,
    },
}

/// 🔗️ Owned by `model`: the master plan's "relations{kind enum, from, to}" — `id` is this
/// subset's own synthesized edge key (needed to diff relations as a keyed collection via the
/// shared `🧰️triples` engine; the master plan's 3-field description is the payload this key
/// wraps, not a rejection of having one). `Default` is the same `serde_derive` technical
/// workaround as `SpatialNode`'s.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct ModelRelation {
    pub id: String,
    pub kind: RelationKind,
    pub from: String,
    pub to: String,
}
//#endregion 🔖️Relation

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.model")]
pub struct SemioModelSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub spatial: Vec<SpatialNode>,
    #[state(artifact)]
    #[value(default)]
    pub elements: Vec<SemioModelElement>,
    #[state(artifact)]
    #[value(default)]
    pub relations: Vec<ModelRelation>,
}

impl Default for SemioModelSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_SEMIOMODEL_DOCUMENT_SCHEMA.into(), spatial: Vec::new(), elements: Vec::new(), relations: Vec::new() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️TextPrimitives
/// 🧪️ P2 pilot (model): real hex/bracket-encoded value primitives backing the hand-rolled
/// `ArtifactDsl` below — same style as this subset's own `🔺️diff`/`🧬️mutations` facets
/// (`GifDiff`/`SvgDiff`/`DocxDiff`'s established hand-rolled convention), duplicated here (not
/// imported from `schema::diff`) to keep `snapshot` — the base type `diff`/`mutations` both depend
/// ON — free of a reverse dependency on either sibling facet (same rationale
/// `stdio.semio.flow`'s own snapshot module documents).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_f64(v: f64) -> String {
    format!("{v}")
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_f64(s: &str) -> Result<f64, String> {
    s.parse().map_err(|e: std::num::ParseFloatError| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_option<T>(opt: &Option<T>, enc: impl Fn(&T) -> String) -> String {
    match opt {
        None => "[0]".to_string(),
        Some(v) => format!("[1,{}]", enc(v)),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_option<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Option<T>, String> {
    let inner = strip_brackets(s)?;
    match split_top_level(inner, ',').as_slice() {
        ["0"] => Ok(None),
        [tag, value] if *tag == "1" => Ok(Some(dec(value)?)),
        other => Err(format!("option decode: bad shape {other:?}")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_list<T>(items: &[T], enc: impl Fn(&T) -> String) -> String {
    format!("[{}]", items.iter().map(|it| enc(it)).collect::<Vec<_>>().join(","))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_list<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Vec<T>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|entry| dec(entry)).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_point3(p: &SemioPoint3) -> String {
    format!("[{},{},{}]", enc_f64(p.x), enc_f64(p.y), enc_f64(p.z))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_point3(s: &str) -> Result<SemioPoint3, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [x, y, z] = parts.as_slice() else { return Err(format!("point3: expected 3 fields, got {}", parts.len())) };
    Ok(SemioPoint3 { x: dec_f64(x)?, y: dec_f64(y)?, z: dec_f64(z)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_quat(q: &SemioQuaternion) -> String {
    format!("[{},{},{},{}]", enc_f64(q.x), enc_f64(q.y), enc_f64(q.z), enc_f64(q.w))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_quat(s: &str) -> Result<SemioQuaternion, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [x, y, z, w] = parts.as_slice() else { return Err(format!("quaternion: expected 4 fields, got {}", parts.len())) };
    Ok(SemioQuaternion { x: dec_f64(x)?, y: dec_f64(y)?, z: dec_f64(z)?, w: dec_f64(w)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_transform(t: &SemioTransform) -> String {
    format!("[{},{},{}]", enc_point3(&t.translation), enc_quat(&t.rotation), enc_point3(&t.scale))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_transform(s: &str) -> Result<SemioTransform, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [translation, rotation, scale] = parts.as_slice() else { return Err(format!("transform: expected 3 fields, got {}", parts.len())) };
    Ok(SemioTransform { translation: dec_point3(translation)?, rotation: dec_quat(rotation)?, scale: dec_point3(scale)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_spatial_kind(k: &SpatialKind) -> &'static str {
    match k {
        SpatialKind::Site => "S",
        SpatialKind::Building => "B",
        SpatialKind::Storey => "T",
        SpatialKind::Space => "P",
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_spatial_kind(s: &str) -> Result<SpatialKind, String> {
    match s {
        "S" => Ok(SpatialKind::Site),
        "B" => Ok(SpatialKind::Building),
        "T" => Ok(SpatialKind::Storey),
        "P" => Ok(SpatialKind::Space),
        other => Err(format!("spatial kind: unknown tag {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_element_class(c: &ElementClass) -> String {
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
fn dec_element_class(s: &str) -> Result<ElementClass, String> {
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
fn enc_geometry_ref(g: &GeometryRef) -> String {
    match g {
        GeometryRef::None => "N".to_string(),
        GeometryRef::Brep { brep_id } => format!("B[{}]", enc_str(brep_id)),
        GeometryRef::Mesh { mesh_id } => format!("M[{}]", enc_str(mesh_id)),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_geometry_ref(s: &str) -> Result<GeometryRef, String> {
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
fn enc_pset_value(v: &PsetValue) -> String {
    match v {
        PsetValue::Text { value } => format!("T[{}]", enc_str(value)),
        PsetValue::Number { value } => format!("N[{value}]"),
        PsetValue::Boolean { value } => format!("B[{}]", if *value { "1" } else { "0" }),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_pset_value(s: &str) -> Result<PsetValue, String> {
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
fn enc_property(p: &Property) -> String {
    format!("[{},{}]", enc_str(&p.key), enc_pset_value(&p.value))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_property(s: &str) -> Result<Property, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [key, value] = parts.as_slice() else { return Err(format!("property: expected 2 fields, got {}", parts.len())) };
    Ok(Property { key: dec_str(key)?, value: dec_pset_value(value)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_property_set(ps: &PropertySet) -> String {
    format!("[{},{}]", enc_str(&ps.name), enc_list(&ps.properties, enc_property))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_property_set(s: &str) -> Result<PropertySet, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, properties] = parts.as_slice() else { return Err(format!("property set: expected 2 fields, got {}", parts.len())) };
    Ok(PropertySet { name: dec_str(name)?, properties: dec_list(properties, dec_property)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_spatial_node(n: &SpatialNode) -> String {
    format!("[{},{},{},{},{}]", enc_str(&n.id), enc_spatial_kind(&n.kind), enc_str(&n.name), encode_option(&n.parent_id, |v: &String| enc_str(v)), enc_transform(&n.placement))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_spatial_node(s: &str) -> Result<SpatialNode, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, kind, name, parent_id, placement] = parts.as_slice() else { return Err(format!("spatial node: expected 5 fields, got {}", parts.len())) };
    Ok(SpatialNode { id: dec_str(id)?, kind: dec_spatial_kind(kind)?, name: dec_str(name)?, parent_id: decode_option(parent_id, dec_str)?, placement: dec_transform(placement)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_element(e: &SemioModelElement) -> String {
    format!("[{},{},{},{},{},{}]", enc_str(&e.id), enc_element_class(&e.class), enc_transform(&e.placement), enc_geometry_ref(&e.geometry), encode_option(&e.spatial_id, |v: &String| enc_str(v)), enc_list(&e.psets, enc_property_set),)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_element(s: &str) -> Result<SemioModelElement, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, class, placement, geometry, spatial_id, psets] = parts.as_slice() else { return Err(format!("element: expected 6 fields, got {}", parts.len())) };
    Ok(SemioModelElement { id: dec_str(id)?, class: dec_element_class(class)?, placement: dec_transform(placement)?, geometry: dec_geometry_ref(geometry)?, spatial_id: decode_option(spatial_id, dec_str)?, psets: dec_list(psets, dec_property_set)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_relation_kind(k: &RelationKind) -> String {
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
fn dec_relation_kind(s: &str) -> Result<RelationKind, String> {
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
fn enc_relation(r: &ModelRelation) -> String {
    format!("[{},{},{},{}]", enc_str(&r.id), enc_relation_kind(&r.kind), enc_str(&r.from), enc_str(&r.to))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_relation(s: &str) -> Result<ModelRelation, String> {
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
fn print_semio_model_snapshot_body(s: &SemioModelSnapshot) -> String {
    format!(
        "schema={}\nspatial=[{}]\nelements=[{}]\nrelations=[{}]",
        enc_str(&s.schema),
        s.spatial.iter().map(enc_spatial_node).collect::<Vec<_>>().join(","),
        s.elements.iter().map(enc_element).collect::<Vec<_>>().join(","),
        s.relations.iter().map(enc_relation).collect::<Vec<_>>().join(","),
    )
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_semio_model_snapshot_body(body: &str) -> Result<SemioModelSnapshot, String> {
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
//#endregion 🔖️TextPrimitives

//#region 🔖️BinaryPrimitives
/// 🧪️ P2 pilot (model): real LEB128-varint-length-prefixed binary primitives (`store::pack_rt::
/// write_varint_u64` / `store::ByteReader`, same helpers `stdio.semio.flow`'s upgraded
/// `ArtifactPack` reuses) backing the real `ArtifactPack` below — replaces the old
/// `serde_json::to_vec`-in-envelope shortcut entirely.
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
fn write_option_str(out: &mut Vec<u8>, opt: &Option<String>) {
    match opt {
        None => out.push(0),
        Some(v) => {
            out.push(1);
            write_str_lp(out, v);
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_option_str(reader: &mut store::ByteReader<'_>) -> Result<Option<String>, String> {
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => Ok(None),
        1 => Ok(Some(read_str_lp(reader)?)),
        other => Err(format!("option<str>: unknown presence tag {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_transform(out: &mut Vec<u8>, t: &SemioTransform) {
    out.extend_from_slice(&t.translation.x.to_le_bytes());
    out.extend_from_slice(&t.translation.y.to_le_bytes());
    out.extend_from_slice(&t.translation.z.to_le_bytes());
    out.extend_from_slice(&t.rotation.x.to_le_bytes());
    out.extend_from_slice(&t.rotation.y.to_le_bytes());
    out.extend_from_slice(&t.rotation.z.to_le_bytes());
    out.extend_from_slice(&t.rotation.w.to_le_bytes());
    out.extend_from_slice(&t.scale.x.to_le_bytes());
    out.extend_from_slice(&t.scale.y.to_le_bytes());
    out.extend_from_slice(&t.scale.z.to_le_bytes());
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_transform(reader: &mut store::ByteReader<'_>) -> Result<SemioTransform, String> {
    let mut next = || reader.read_f64_le().map_err(|e| e.to_string());
    let translation = SemioPoint3 { x: next()?, y: next()?, z: next()? };
    let rotation = SemioQuaternion { x: next()?, y: next()?, z: next()?, w: next()? };
    let scale = SemioPoint3 { x: next()?, y: next()?, z: next()? };
    Ok(SemioTransform { translation, rotation, scale })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_spatial_kind(out: &mut Vec<u8>, k: &SpatialKind) {
    out.push(match k {
        SpatialKind::Site => 0,
        SpatialKind::Building => 1,
        SpatialKind::Storey => 2,
        SpatialKind::Space => 3,
    });
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_spatial_kind(reader: &mut store::ByteReader<'_>) -> Result<SpatialKind, String> {
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => Ok(SpatialKind::Site),
        1 => Ok(SpatialKind::Building),
        2 => Ok(SpatialKind::Storey),
        3 => Ok(SpatialKind::Space),
        other => Err(format!("spatial kind: unknown tag {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_element_class(out: &mut Vec<u8>, c: &ElementClass) {
    match c {
        ElementClass::Wall => out.push(0),
        ElementClass::Slab => out.push(1),
        ElementClass::Column => out.push(2),
        ElementClass::Beam => out.push(3),
        ElementClass::Door => out.push(4),
        ElementClass::Window => out.push(5),
        ElementClass::Roof => out.push(6),
        ElementClass::Stair => out.push(7),
        ElementClass::Furniture => out.push(8),
        ElementClass::Other { name } => {
            out.push(9);
            write_str_lp(out, name);
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_element_class(reader: &mut store::ByteReader<'_>) -> Result<ElementClass, String> {
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => Ok(ElementClass::Wall),
        1 => Ok(ElementClass::Slab),
        2 => Ok(ElementClass::Column),
        3 => Ok(ElementClass::Beam),
        4 => Ok(ElementClass::Door),
        5 => Ok(ElementClass::Window),
        6 => Ok(ElementClass::Roof),
        7 => Ok(ElementClass::Stair),
        8 => Ok(ElementClass::Furniture),
        9 => Ok(ElementClass::Other { name: read_str_lp(reader)? }),
        other => Err(format!("element class: unknown tag {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_geometry_ref(out: &mut Vec<u8>, g: &GeometryRef) {
    match g {
        GeometryRef::None => out.push(0),
        GeometryRef::Brep { brep_id } => {
            out.push(1);
            write_str_lp(out, brep_id);
        }
        GeometryRef::Mesh { mesh_id } => {
            out.push(2);
            write_str_lp(out, mesh_id);
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_geometry_ref(reader: &mut store::ByteReader<'_>) -> Result<GeometryRef, String> {
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => Ok(GeometryRef::None),
        1 => Ok(GeometryRef::Brep { brep_id: read_str_lp(reader)? }),
        2 => Ok(GeometryRef::Mesh { mesh_id: read_str_lp(reader)? }),
        other => Err(format!("geometry ref: unknown tag {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_pset_value(out: &mut Vec<u8>, v: &PsetValue) {
    match v {
        PsetValue::Text { value } => {
            out.push(0);
            write_str_lp(out, value);
        }
        PsetValue::Number { value } => {
            out.push(1);
            out.extend_from_slice(&value.to_le_bytes());
        }
        PsetValue::Boolean { value } => {
            out.push(2);
            out.push(if *value { 1 } else { 0 });
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_pset_value(reader: &mut store::ByteReader<'_>) -> Result<PsetValue, String> {
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => Ok(PsetValue::Text { value: read_str_lp(reader)? }),
        1 => Ok(PsetValue::Number { value: reader.read_f64_le().map_err(|e| e.to_string())? }),
        2 => Ok(PsetValue::Boolean { value: reader.read_u8().map_err(|e| e.to_string())? != 0 }),
        other => Err(format!("pset value: unknown tag {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_property(out: &mut Vec<u8>, p: &Property) {
    write_str_lp(out, &p.key);
    write_pset_value(out, &p.value);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_property(reader: &mut store::ByteReader<'_>) -> Result<Property, String> {
    let key = read_str_lp(reader)?;
    let value = read_pset_value(reader)?;
    Ok(Property { key, value })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_property_set(out: &mut Vec<u8>, ps: &PropertySet) {
    write_str_lp(out, &ps.name);
    store::pack_rt::write_varint_u64(out, ps.properties.len() as u64);
    for p in &ps.properties {
        write_property(out, p);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_property_set(reader: &mut store::ByteReader<'_>) -> Result<PropertySet, String> {
    let name = read_str_lp(reader)?;
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut properties = Vec::with_capacity(count as usize);
    for _ in 0..count {
        properties.push(read_property(reader)?);
    }
    Ok(PropertySet { name, properties })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_spatial_node(out: &mut Vec<u8>, n: &SpatialNode) {
    write_str_lp(out, &n.id);
    write_spatial_kind(out, &n.kind);
    write_str_lp(out, &n.name);
    write_option_str(out, &n.parent_id);
    write_transform(out, &n.placement);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_spatial_node(reader: &mut store::ByteReader<'_>) -> Result<SpatialNode, String> {
    let id = read_str_lp(reader)?;
    let kind = read_spatial_kind(reader)?;
    let name = read_str_lp(reader)?;
    let parent_id = read_option_str(reader)?;
    let placement = read_transform(reader)?;
    Ok(SpatialNode { id, kind, name, parent_id, placement })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_element(out: &mut Vec<u8>, e: &SemioModelElement) {
    write_str_lp(out, &e.id);
    write_element_class(out, &e.class);
    write_transform(out, &e.placement);
    write_geometry_ref(out, &e.geometry);
    write_option_str(out, &e.spatial_id);
    store::pack_rt::write_varint_u64(out, e.psets.len() as u64);
    for ps in &e.psets {
        write_property_set(out, ps);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_element(reader: &mut store::ByteReader<'_>) -> Result<SemioModelElement, String> {
    let id = read_str_lp(reader)?;
    let class = read_element_class(reader)?;
    let placement = read_transform(reader)?;
    let geometry = read_geometry_ref(reader)?;
    let spatial_id = read_option_str(reader)?;
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut psets = Vec::with_capacity(count as usize);
    for _ in 0..count {
        psets.push(read_property_set(reader)?);
    }
    Ok(SemioModelElement { id, class, placement, geometry, spatial_id, psets })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_relation_kind(out: &mut Vec<u8>, k: &RelationKind) {
    match k {
        RelationKind::Aggregates => out.push(0),
        RelationKind::ContainedIn => out.push(1),
        RelationKind::ConnectsTo => out.push(2),
        RelationKind::FillsVoid => out.push(3),
        RelationKind::VoidsElement => out.push(4),
        RelationKind::Other { label } => {
            out.push(5);
            write_str_lp(out, label);
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_relation_kind(reader: &mut store::ByteReader<'_>) -> Result<RelationKind, String> {
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => Ok(RelationKind::Aggregates),
        1 => Ok(RelationKind::ContainedIn),
        2 => Ok(RelationKind::ConnectsTo),
        3 => Ok(RelationKind::FillsVoid),
        4 => Ok(RelationKind::VoidsElement),
        5 => Ok(RelationKind::Other { label: read_str_lp(reader)? }),
        other => Err(format!("relation kind: unknown tag {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_relation(out: &mut Vec<u8>, r: &ModelRelation) {
    write_str_lp(out, &r.id);
    write_relation_kind(out, &r.kind);
    write_str_lp(out, &r.from);
    write_str_lp(out, &r.to);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_relation(reader: &mut store::ByteReader<'_>) -> Result<ModelRelation, String> {
    let id = read_str_lp(reader)?;
    let kind = read_relation_kind(reader)?;
    let from = read_str_lp(reader)?;
    let to = read_str_lp(reader)?;
    Ok(ModelRelation { id, kind, from, to })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_model_snapshot_binary(s: &SemioModelSnapshot) -> Vec<u8> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut out = Vec::new();
    out.push(PACK_BINARY_FORMAT);
    write_str_lp(&mut out, &s.schema);
    store::pack_rt::write_varint_u64(&mut out, s.spatial.len() as u64);
    for n in &s.spatial {
        write_spatial_node(&mut out, n);
    }
    store::pack_rt::write_varint_u64(&mut out, s.elements.len() as u64);
    for e in &s.elements {
        write_element(&mut out, e);
    }
    store::pack_rt::write_varint_u64(&mut out, s.relations.len() as u64);
    for r in &s.relations {
        write_relation(&mut out, r);
    }
    out
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_model_snapshot_binary(bytes: &[u8]) -> Result<SemioModelSnapshot, String> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut reader = store::ByteReader::new(bytes);
    let format = reader.read_u8().map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT {
        return Err(format!("unsupported pack format {format}"));
    }
    let schema = read_str_lp(&mut reader)?;
    let spatial_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut spatial = Vec::with_capacity(spatial_count as usize);
    for _ in 0..spatial_count {
        spatial.push(read_spatial_node(&mut reader)?);
    }
    let element_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut elements = Vec::with_capacity(element_count as usize);
    for _ in 0..element_count {
        elements.push(read_element(&mut reader)?);
    }
    let relation_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut relations = Vec::with_capacity(relation_count as usize);
    for _ in 0..relation_count {
        relations.push(read_relation(&mut reader)?);
    }
    Ok(SemioModelSnapshot { schema, spatial, elements, relations })
}
//#endregion 🔖️BinaryPrimitives

//#region 🔖️HandcraftedArtifactCodecs
/// 🎁 Real structured text/binary codecs (P2 pilot — model subset upgraded off the old
/// hex-dump-of-`serde_json` shortcut, following `stdio.semio.flow`'s proven pattern). Wrapped
/// in the repo-wide `store::semio_format` envelope, unchanged.
impl store::ArtifactDsl for SemioModelSnapshot {
    const EXTENSION: &'static str = "semio";
    fn envelope_id() -> &'static str {
        STDIO_SEMIOMODEL_DOCUMENT_SCHEMA
    }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_semio_model_snapshot_body(body).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        let body = print_semio_model_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for SemioModelSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_model_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let _ = options;
        decode_model_snapshot_binary(&inner).map_err(store::PackError::Schema)
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️ReachableCodecs
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_semio_model_pack(snapshot: &SemioModelSnapshot) -> Vec<u8> {
    <SemioModelSnapshot as store::ArtifactPack>::encode_pack(snapshot)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_model_pack(bytes: &[u8]) -> Result<SemioModelSnapshot, String> {
    <SemioModelSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|e| e.to_string())
}
//#endregion 🔖️ReachableCodecs

//#region 🔖️Demo
/// 🌱 The demo `s.stdio.semio.model` document — a fully-populated snapshot exercising every
/// collection/leaf shape at least once. Single source of truth for
/// `📚️examples/🏢️building/🖼️assets/🗣️.dsl.semio`/`🎒️.pack.semio` and for the
/// conformance-law tests in `🎹️composer/🦀️.rs`.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_semio_model_snapshot() -> SemioModelSnapshot {
    SemioModelSnapshot {
        schema: STDIO_SEMIOMODEL_DOCUMENT_SCHEMA.into(),
        spatial: vec![
            SpatialNode { id: "site-1".into(), kind: SpatialKind::Site, name: "Site One".into(), parent_id: None, placement: SemioTransform::identity() },
            SpatialNode {
                id: "storey-1".into(),
                kind: SpatialKind::Storey,
                name: "Ground Floor".into(),
                parent_id: Some("site-1".into()),
                placement: SemioTransform { translation: SemioPoint3 { x: 0.0, y: 0.0, z: 3.0 }, rotation: SemioQuaternion::default(), scale: SemioPoint3 { x: 1.0, y: 1.0, z: 1.0 } },
            },
        ],
        elements: vec![SemioModelElement {
            id: "wall-1".into(),
            class: ElementClass::Wall,
            placement: SemioTransform::identity(),
            geometry: GeometryRef::Brep { brep_id: "brep-1".into() },
            spatial_id: Some("storey-1".into()),
            psets: vec![PropertySet {
                name: "Pset_WallCommon".into(),
                properties: vec![
                    Property { key: "IsExternal".into(), value: PsetValue::Boolean { value: true } },
                    Property { key: "FireRating".into(), value: PsetValue::Text { value: "REI60".into() } },
                    Property { key: "ThermalTransmittance".into(), value: PsetValue::Number { value: 0.24 } },
                ],
            }],
        }],
        relations: vec![ModelRelation { id: "rel-1".into(), kind: RelationKind::ContainedIn, from: "wall-1".into(), to: "storey-1".into() }],
    }
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn json_pack_round_trips() {
        let snap = SemioModelSnapshot::default();
        let bytes = <SemioModelSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <SemioModelSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(snap, back);
    }

    #[semio_framework_async_macros::async_test]
    async fn dsl_text_round_trips() {
        let snap = SemioModelSnapshot::default();
        let text = <SemioModelSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        let back = <SemioModelSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(snap, back);
    }

    /// 🧪️ codec_retention_law: decode(encode(x)) == x for a fully-populated snapshot (every
    /// collection non-empty, every field set), through both the pack (binary) and DSL (text) codecs.
    #[semio_framework_async_macros::async_test]
    async fn codec_retention_law() {
        let snap = demo_semio_model_snapshot();
        let packed = <SemioModelSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let unpacked = <SemioModelSnapshot as store::ArtifactPack>::decode_pack(&packed).expect("decode pack");
        assert_eq!(snap, unpacked);

        let text = <SemioModelSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        let reparsed = <SemioModelSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse dsl");
        assert_eq!(snap, reparsed);
    }
}
//#endregion 🔖️Tests
