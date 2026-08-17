//! 🔺️ SemioModelDiff — handcrafted sparse diff over `SemioModelSnapshot`. No
//! `snapshot: Option<SemioModelSnapshot>` full-replace slot — even `SetSnapshot`'s diff is the
//! sparse field-by-field `SemioModelDiff::between(base, next)`.
//!
//! `spatial`/`elements`/`relations` (all id-keyed) are diffed via the SHARED
//! `crate::artifacts::semio::standards::v1::subsets::any::schema::triples::NamedTripleDiff<K,D,T>` engine
//! (w1b-type-ownership.md: "Use `🧰️triples`... instead of reinventing it") — this file does NOT
//! redefine that container or its wire codec, only the generic apply/between/inverse/absorb glue
//! functions a specific artifact's collections need (mirrors bcf's own local generic engine,
//! `f6-final-summary.md` §4.4, minus the container type itself, which now has one shared home).

use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::{SemioPoint3, SemioQuaternion, SemioTransform};
use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::{dec_named_triple, enc_named_triple, split_top_level, strip_brackets, NamedModified, NamedTripleDiff};
use crate::artifacts::semio::standards::v1::subsets::model::schema::snapshot::{ElementClass, GeometryRef, ModelRelation, Property, PropertySet, PsetValue, RelationKind, SemioModelElement, SemioModelSnapshot, SpatialKind, SpatialNode};
use protocol::command::DiffAlgebra;
use protocol::{DiffCodec, MutationDiff};
use serde::{Deserialize, Serialize};

//#region 🔖️GenericNamedEngine
/// 🧮️ Generic name/id-keyed collection glue — `between`/`apply`/`inverse`/`absorb` over the
/// shared `NamedTripleDiff<K,D,T>` container, written once and instantiated per collection below
/// (mirrors bcf's own local copy, `💬️bcf/…/🔺️diff/🦀️component.rs` §GenericNamedEngine).
fn between_named<K, T, D>(base: &[T], other: &[T], key_of: impl Fn(&T) -> K, diff_item: impl Fn(&T, &T) -> Option<D>) -> Option<NamedTripleDiff<K, D, T>>
where
    K: PartialEq + Clone,
    T: Clone + PartialEq,
{
    let mut removed = Vec::new();
    let mut modified = Vec::new();
    for b in base {
        let bk = key_of(b);
        match other.iter().find(|o| key_of(o) == bk) {
            None => removed.push(bk),
            Some(o) if o != b => {
                if let Some(d) = diff_item(b, o) {
                    modified.push(NamedModified { key: bk, diff: d });
                }
            }
            Some(_) => {}
        }
    }
    let mut added = Vec::new();
    for o in other {
        let ok = key_of(o);
        if !base.iter().any(|b| key_of(b) == ok) {
            added.push(o.clone());
        }
    }
    if removed.is_empty() && modified.is_empty() && added.is_empty() {
        None
    } else {
        Some(NamedTripleDiff { removed, modified, added })
    }
}

fn apply_named<K, T, D>(items: &mut Vec<T>, diff: &NamedTripleDiff<K, D, T>, key_of: impl Fn(&T) -> K, apply_item: impl Fn(&mut T, &D))
where
    K: PartialEq + Clone,
    T: Clone,
{
    items.retain(|i| !diff.removed.contains(&key_of(i)));
    for m in &diff.modified {
        if let Some(item) = items.iter_mut().find(|i| key_of(i) == m.key) {
            apply_item(item, &m.diff);
        }
    }
    for item in &diff.added {
        items.push(item.clone());
    }
}

fn inverse_named<K, T, D>(base_items: &[T], diff: &NamedTripleDiff<K, D, T>, key_of: impl Fn(&T) -> K, inverse_item: impl Fn(&T, &D) -> D) -> NamedTripleDiff<K, D, T>
where
    K: PartialEq + Clone,
    T: Clone,
{
    let removed: Vec<K> = diff.added.iter().map(&key_of).collect();
    let mut modified = Vec::new();
    for m in &diff.modified {
        if let Some(original) = base_items.iter().find(|i| key_of(i) == m.key) {
            modified.push(NamedModified { key: m.key.clone(), diff: inverse_item(original, &m.diff) });
        }
    }
    let mut added = Vec::new();
    for k in &diff.removed {
        if let Some(original) = base_items.iter().find(|i| &key_of(i) == k) {
            added.push(original.clone());
        }
    }
    NamedTripleDiff { removed, modified, added }
}

/// 🧮️ Name/id-keyed absorb — identity is the KEY (not position): a `d2`-removal of a `d1`-added
/// key annihilates the add; a `d2`-modify of a `d1`-added key patches into the carried payload.
fn absorb_named<K, T, D>(d1: NamedTripleDiff<K, D, T>, d2: NamedTripleDiff<K, D, T>, key_of: impl Fn(&T) -> K, absorb_item: impl Fn(D, D) -> D, apply_item: impl Fn(&mut T, &D)) -> NamedTripleDiff<K, D, T>
where
    K: PartialEq + Clone,
    T: Clone,
    D: Clone,
{
    let d1_added_keys: Vec<K> = d1.added.iter().map(&key_of).collect();
    let mut removed = d1.removed.clone();
    let mut annihilated: Vec<K> = Vec::new();
    for k in &d2.removed {
        if d1_added_keys.contains(k) {
            annihilated.push(k.clone());
        } else if !removed.contains(k) {
            removed.push(k.clone());
        }
    }
    let mut working_added: Vec<T> = d1.added.into_iter().filter(|a| !annihilated.contains(&key_of(a))).collect();
    let mut modified: Vec<NamedModified<K, D>> = d1.modified.into_iter().filter(|m| !removed.contains(&m.key)).collect();
    for m2 in &d2.modified {
        if let Some(added) = working_added.iter_mut().find(|a| key_of(a) == m2.key) {
            apply_item(added, &m2.diff);
            continue;
        }
        if removed.contains(&m2.key) {
            continue;
        }
        match modified.iter_mut().find(|m| m.key == m2.key) {
            Some(existing) => existing.diff = absorb_item(existing.diff.clone(), m2.diff.clone()),
            None => modified.push(NamedModified { key: m2.key.clone(), diff: m2.diff.clone() }),
        }
    }
    for a2 in &d2.added {
        let k2 = key_of(a2);
        match working_added.iter_mut().find(|a| key_of(a) == k2) {
            Some(existing) => *existing = a2.clone(),
            None => working_added.push(a2.clone()),
        }
    }
    NamedTripleDiff { removed, modified, added: working_added }
}
//#endregion 🔖️GenericNamedEngine

//#region 🔖️DiffTypes
pub type SpatialDiff = NamedTripleDiff<String, SpatialNodeDiff, SpatialNode>;
pub type ElementsDiff = NamedTripleDiff<String, SemioModelElementDiff, SemioModelElement>;
pub type RelationsDiff = NamedTripleDiff<String, ModelRelationDiff, ModelRelation>;

/// 🔺️ Per-spatial-node sparse diff. `parent_id` is tri-state (`Some(None)` = detached from its
/// parent, becomes a root).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpatialNodeDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<SpatialKind>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<Option<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placement: Option<SemioTransform>,
}

/// 🔺️ Per-element sparse diff. `spatial_id` is tri-state (`Some(None)` = removed from the
/// spatial tree). `psets` is whole-value replaced (weak entity, never sub-diffed per the recipe).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemioModelElementDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub class: Option<ElementClass>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placement: Option<SemioTransform>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub geometry: Option<GeometryRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spatial_id: Option<Option<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub psets: Option<Vec<PropertySet>>,
}

/// 🔺️ Per-relation sparse diff — `id` is the key, so only `kind`/`from`/`to` can change.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelRelationDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<RelationKind>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}

/// 🔺️ Diff for `s.stdio.semio.model`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemioModelDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spatial: Option<SpatialDiff>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub elements: Option<ElementsDiff>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relations: Option<RelationsDiff>,
}
//#endregion 🔖️DiffTypes

//#region 🔖️Apply
impl MutationDiff<SemioModelSnapshot> for SemioModelDiff {
    fn apply(&self, base: &SemioModelSnapshot) -> protocol::MutationApplyResult<SemioModelSnapshot> {
        let mut next = base.clone();
        if let Some(d) = &self.spatial {
            crate::artifacts::semio::standards::v1::subsets::any::schema::triples::validate_named_triple(&next.spatial, d, |item| item.id.clone(), |item| item.id.clone(), ["spatial"])?;
            apply_named(&mut next.spatial, d, |n: &SpatialNode| n.id.clone(), apply_spatial);
        }
        if let Some(d) = &self.elements {
            crate::artifacts::semio::standards::v1::subsets::any::schema::triples::validate_named_triple(&next.elements, d, |item| item.id.clone(), |item| item.id.clone(), ["elements"])?;
            apply_named(&mut next.elements, d, |e: &SemioModelElement| e.id.clone(), apply_element);
        }
        if let Some(d) = &self.relations {
            crate::artifacts::semio::standards::v1::subsets::any::schema::triples::validate_named_triple(&next.relations, d, |item| item.id.clone(), |item| item.id.clone(), ["relations"])?;
            apply_named(&mut next.relations, d, |r: &ModelRelation| r.id.clone(), apply_relation);
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        self.spatial = match (self.spatial.take(), other.spatial) {
            (None, b) => b,
            (a, None) => a,
            (Some(a), Some(b)) => Some(absorb_named(a, b, |n: &SpatialNode| n.id.clone(), absorb_spatial_diff, apply_spatial)),
        };
        self.elements = match (self.elements.take(), other.elements) {
            (None, b) => b,
            (a, None) => a,
            (Some(a), Some(b)) => Some(absorb_named(a, b, |e: &SemioModelElement| e.id.clone(), absorb_element_diff, apply_element)),
        };
        self.relations = match (self.relations.take(), other.relations) {
            (None, b) => b,
            (a, None) => a,
            (Some(a), Some(b)) => Some(absorb_named(a, b, |r: &ModelRelation| r.id.clone(), absorb_relation_diff, apply_relation)),
        };
    }
}

fn apply_spatial(node: &mut SpatialNode, diff: &SpatialNodeDiff) {
    if let Some(v) = &diff.kind {
        node.kind = *v;
    }
    if let Some(v) = &diff.name {
        node.name = v.clone();
    }
    if let Some(v) = &diff.parent_id {
        node.parent_id = v.clone();
    }
    if let Some(v) = &diff.placement {
        node.placement = *v;
    }
}

fn apply_element(element: &mut SemioModelElement, diff: &SemioModelElementDiff) {
    if let Some(v) = &diff.class {
        element.class = v.clone();
    }
    if let Some(v) = &diff.placement {
        element.placement = *v;
    }
    if let Some(v) = &diff.geometry {
        element.geometry = v.clone();
    }
    if let Some(v) = &diff.spatial_id {
        element.spatial_id = v.clone();
    }
    if let Some(v) = &diff.psets {
        element.psets = v.clone();
    }
}

fn apply_relation(relation: &mut ModelRelation, diff: &ModelRelationDiff) {
    if let Some(v) = &diff.kind {
        relation.kind = v.clone();
    }
    if let Some(v) = &diff.from {
        relation.from = v.clone();
    }
    if let Some(v) = &diff.to {
        relation.to = v.clone();
    }
}
//#endregion 🔖️Apply

//#region 🔖️DiffAlgebra
impl DiffAlgebra<SemioModelSnapshot> for SemioModelDiff {
    fn inverse(&self, base: &SemioModelSnapshot) -> Self {
        SemioModelDiff {
            spatial: self.spatial.as_ref().map(|d| inverse_named(&base.spatial, d, |n: &SpatialNode| n.id.clone(), inverse_spatial)),
            elements: self.elements.as_ref().map(|d| inverse_named(&base.elements, d, |e: &SemioModelElement| e.id.clone(), inverse_element)),
            relations: self.relations.as_ref().map(|d| inverse_named(&base.relations, d, |r: &ModelRelation| r.id.clone(), inverse_relation)),
        }
    }

    fn between(base: &SemioModelSnapshot, other: &SemioModelSnapshot) -> Self {
        SemioModelDiff {
            spatial: between_named(&base.spatial, &other.spatial, |n: &SpatialNode| n.id.clone(), between_spatial),
            elements: between_named(&base.elements, &other.elements, |e: &SemioModelElement| e.id.clone(), between_element),
            relations: between_named(&base.relations, &other.relations, |r: &ModelRelation| r.id.clone(), between_relation),
        }
    }

    fn is_empty(&self) -> bool {
        self.spatial.is_none() && self.elements.is_none() && self.relations.is_none()
    }
}

fn inverse_spatial(base: &SpatialNode, diff: &SpatialNodeDiff) -> SpatialNodeDiff {
    SpatialNodeDiff { kind: diff.kind.as_ref().map(|_| base.kind), name: diff.name.as_ref().map(|_| base.name.clone()), parent_id: diff.parent_id.as_ref().map(|_| base.parent_id.clone()), placement: diff.placement.as_ref().map(|_| base.placement) }
}

fn inverse_element(base: &SemioModelElement, diff: &SemioModelElementDiff) -> SemioModelElementDiff {
    SemioModelElementDiff {
        class: diff.class.as_ref().map(|_| base.class.clone()),
        placement: diff.placement.as_ref().map(|_| base.placement),
        geometry: diff.geometry.as_ref().map(|_| base.geometry.clone()),
        spatial_id: diff.spatial_id.as_ref().map(|_| base.spatial_id.clone()),
        psets: diff.psets.as_ref().map(|_| base.psets.clone()),
    }
}

fn inverse_relation(base: &ModelRelation, diff: &ModelRelationDiff) -> ModelRelationDiff {
    ModelRelationDiff { kind: diff.kind.as_ref().map(|_| base.kind.clone()), from: diff.from.as_ref().map(|_| base.from.clone()), to: diff.to.as_ref().map(|_| base.to.clone()) }
}

fn between_spatial(base: &SpatialNode, other: &SpatialNode) -> Option<SpatialNodeDiff> {
    let kind = if base.kind != other.kind { Some(other.kind) } else { None };
    let name = if base.name != other.name { Some(other.name.clone()) } else { None };
    let parent_id = if base.parent_id != other.parent_id { Some(other.parent_id.clone()) } else { None };
    let placement = if base.placement != other.placement { Some(other.placement) } else { None };
    if kind.is_none() && name.is_none() && parent_id.is_none() && placement.is_none() {
        None
    } else {
        Some(SpatialNodeDiff { kind, name, parent_id, placement })
    }
}

fn between_element(base: &SemioModelElement, other: &SemioModelElement) -> Option<SemioModelElementDiff> {
    let class = if base.class != other.class { Some(other.class.clone()) } else { None };
    let placement = if base.placement != other.placement { Some(other.placement) } else { None };
    let geometry = if base.geometry != other.geometry { Some(other.geometry.clone()) } else { None };
    let spatial_id = if base.spatial_id != other.spatial_id { Some(other.spatial_id.clone()) } else { None };
    let psets = if base.psets != other.psets { Some(other.psets.clone()) } else { None };
    if class.is_none() && placement.is_none() && geometry.is_none() && spatial_id.is_none() && psets.is_none() {
        None
    } else {
        Some(SemioModelElementDiff { class, placement, geometry, spatial_id, psets })
    }
}

fn between_relation(base: &ModelRelation, other: &ModelRelation) -> Option<ModelRelationDiff> {
    let kind = if base.kind != other.kind { Some(other.kind.clone()) } else { None };
    let from = if base.from != other.from { Some(other.from.clone()) } else { None };
    let to = if base.to != other.to { Some(other.to.clone()) } else { None };
    if kind.is_none() && from.is_none() && to.is_none() {
        None
    } else {
        Some(ModelRelationDiff { kind, from, to })
    }
}

fn absorb_spatial_diff(mut a: SpatialNodeDiff, b: SpatialNodeDiff) -> SpatialNodeDiff {
    if b.kind.is_some() {
        a.kind = b.kind;
    }
    if b.name.is_some() {
        a.name = b.name;
    }
    if b.parent_id.is_some() {
        a.parent_id = b.parent_id;
    }
    if b.placement.is_some() {
        a.placement = b.placement;
    }
    a
}

fn absorb_element_diff(mut a: SemioModelElementDiff, b: SemioModelElementDiff) -> SemioModelElementDiff {
    if b.class.is_some() {
        a.class = b.class;
    }
    if b.placement.is_some() {
        a.placement = b.placement;
    }
    if b.geometry.is_some() {
        a.geometry = b.geometry;
    }
    if b.spatial_id.is_some() {
        a.spatial_id = b.spatial_id;
    }
    if b.psets.is_some() {
        a.psets = b.psets;
    }
    a
}

fn absorb_relation_diff(mut a: ModelRelationDiff, b: ModelRelationDiff) -> ModelRelationDiff {
    if b.kind.is_some() {
        a.kind = b.kind;
    }
    if b.from.is_some() {
        a.from = b.from;
    }
    if b.to.is_some() {
        a.to = b.to;
    }
    a
}
//#endregion 🔖️DiffAlgebra

//#region 🔖️SetSnapshot
/// 🧩 Builds the sparse field-by-field diff for a `SetSnapshot` mutation. No
/// `snapshot: Option<SemioModelSnapshot>` full-replace slot -- this IS `SemioModelDiff::between`.
pub fn diff_set_snapshot(base: &SemioModelSnapshot, next: &SemioModelSnapshot) -> SemioModelDiff {
    SemioModelDiff::between(base, next)
}
//#endregion 🔖️SetSnapshot

//#region 🔖️HandcraftedDiffCodec
/// 🎙️ Hand-rolled `protocol::DiffCodec` — same bracket-depth-aware token grammar bcf/gif/svg use
/// (see `crate::artifacts::semio::standards::v1::subsets::any::schema::triples` for the shared
/// `split_top_level`/`strip_brackets` primitives this reuses rather than redefining). This
/// artifact's own copy of the small hex/option/list primitive set (each artifact writes its own,
/// per bcf's own module doc rationale -- cross-artifact imports would be architecturally wrong).
//#region 🔖️Primitives
pub(crate) fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
pub(crate) fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
pub(crate) fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
pub(crate) fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
pub(crate) fn parse_f64(s: &str) -> Result<f64, String> {
    s.parse().map_err(|e: std::num::ParseFloatError| e.to_string())
}
pub(crate) fn encode_option<T>(opt: &Option<T>, enc: impl Fn(&T) -> String) -> String {
    match opt {
        None => "[0]".to_string(),
        Some(v) => format!("[1,{}]", enc(v)),
    }
}
pub(crate) fn decode_option<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Option<T>, String> {
    let inner = strip_brackets(s)?;
    match split_top_level(inner, ',').as_slice() {
        ["0"] => Ok(None),
        [tag, value] if *tag == "1" => Ok(Some(dec(value)?)),
        other => Err(format!("option decode: bad shape {other:?}")),
    }
}
pub(crate) fn enc_list<T>(items: &[T], enc: impl Fn(&T) -> String) -> String {
    format!("[{}]", items.iter().map(|it| enc(it)).collect::<Vec<_>>().join(","))
}
pub(crate) fn dec_list<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Vec<T>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|entry| dec(entry)).collect()
}

/// 🧪️ P2 pilot (model): real LEB128-varint-length-prefixed binary primitives (`store::pack_rt::
/// write_varint_u64` / `store::ByteReader`, same helpers `stdio.flow`'s upgraded `DiffCodec`
/// reuses) backing the real `DiffCodec::encode_diff`/`decode_diff` below.
pub(crate) fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}
pub(crate) fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}
pub(crate) fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}
pub(crate) fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}
//#endregion 🔖️Primitives

//#region 🔖️ValueCodecs
pub(crate) fn enc_point3(p: &SemioPoint3) -> String {
    format!("[{},{},{}]", p.x, p.y, p.z)
}
pub(crate) fn dec_point3(s: &str) -> Result<SemioPoint3, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [x, y, z] = parts.as_slice() else { return Err(format!("point3: expected 3 fields, got {}", parts.len())) };
    Ok(SemioPoint3 { x: parse_f64(x)?, y: parse_f64(y)?, z: parse_f64(z)? })
}
pub(crate) fn enc_quat(q: &SemioQuaternion) -> String {
    format!("[{},{},{},{}]", q.x, q.y, q.z, q.w)
}
pub(crate) fn dec_quat(s: &str) -> Result<SemioQuaternion, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [x, y, z, w] = parts.as_slice() else { return Err(format!("quaternion: expected 4 fields, got {}", parts.len())) };
    Ok(SemioQuaternion { x: parse_f64(x)?, y: parse_f64(y)?, z: parse_f64(z)?, w: parse_f64(w)? })
}
pub(crate) fn enc_transform(t: &SemioTransform) -> String {
    format!("[{},{},{}]", enc_point3(&t.translation), enc_quat(&t.rotation), enc_point3(&t.scale))
}
pub(crate) fn dec_transform(s: &str) -> Result<SemioTransform, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [translation, rotation, scale] = parts.as_slice() else { return Err(format!("transform: expected 3 fields, got {}", parts.len())) };
    Ok(SemioTransform { translation: dec_point3(translation)?, rotation: dec_quat(rotation)?, scale: dec_point3(scale)? })
}

pub(crate) fn enc_spatial_kind(k: &SpatialKind) -> &'static str {
    match k {
        SpatialKind::Site => "S",
        SpatialKind::Building => "B",
        SpatialKind::Storey => "T",
        SpatialKind::Space => "P",
    }
}
pub(crate) fn dec_spatial_kind(s: &str) -> Result<SpatialKind, String> {
    match s {
        "S" => Ok(SpatialKind::Site),
        "B" => Ok(SpatialKind::Building),
        "T" => Ok(SpatialKind::Storey),
        "P" => Ok(SpatialKind::Space),
        other => Err(format!("spatial kind: unknown tag {other:?}")),
    }
}

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

pub(crate) fn enc_geometry_ref(g: &GeometryRef) -> String {
    match g {
        GeometryRef::None => "N".to_string(),
        GeometryRef::Brep { brep_id } => format!("B[{}]", enc_str(brep_id)),
        GeometryRef::Mesh { mesh_id } => format!("M[{}]", enc_str(mesh_id)),
    }
}
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

pub(crate) fn enc_pset_value(v: &PsetValue) -> String {
    match v {
        PsetValue::Text { value } => format!("T[{}]", enc_str(value)),
        PsetValue::Number { value } => format!("N[{value}]"),
        PsetValue::Boolean { value } => format!("B[{}]", if *value { "1" } else { "0" }),
    }
}
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

pub(crate) fn enc_property(p: &Property) -> String {
    format!("[{},{}]", enc_str(&p.key), enc_pset_value(&p.value))
}
pub(crate) fn dec_property(s: &str) -> Result<Property, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [key, value] = parts.as_slice() else { return Err(format!("property: expected 2 fields, got {}", parts.len())) };
    Ok(Property { key: dec_str(key)?, value: dec_pset_value(value)? })
}

pub(crate) fn enc_property_set(ps: &PropertySet) -> String {
    format!("[{},{}]", enc_str(&ps.name), enc_list(&ps.properties, enc_property))
}
pub(crate) fn dec_property_set(s: &str) -> Result<PropertySet, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, properties] = parts.as_slice() else { return Err(format!("property set: expected 2 fields, got {}", parts.len())) };
    Ok(PropertySet { name: dec_str(name)?, properties: dec_list(properties, dec_property)? })
}

pub(crate) fn enc_spatial_node(n: &SpatialNode) -> String {
    format!("[{},{},{},{},{}]", enc_str(&n.id), enc_spatial_kind(&n.kind), enc_str(&n.name), encode_option(&n.parent_id, |v: &String| enc_str(v)), enc_transform(&n.placement))
}
pub(crate) fn dec_spatial_node(s: &str) -> Result<SpatialNode, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, kind, name, parent_id, placement] = parts.as_slice() else { return Err(format!("spatial node: expected 5 fields, got {}", parts.len())) };
    Ok(SpatialNode { id: dec_str(id)?, kind: dec_spatial_kind(kind)?, name: dec_str(name)?, parent_id: decode_option(parent_id, dec_str)?, placement: dec_transform(placement)? })
}

pub(crate) fn enc_element(e: &SemioModelElement) -> String {
    format!("[{},{},{},{},{},{}]", enc_str(&e.id), enc_element_class(&e.class), enc_transform(&e.placement), enc_geometry_ref(&e.geometry), encode_option(&e.spatial_id, |v: &String| enc_str(v)), enc_list(&e.psets, enc_property_set),)
}
pub(crate) fn dec_element(s: &str) -> Result<SemioModelElement, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, class, placement, geometry, spatial_id, psets] = parts.as_slice() else { return Err(format!("element: expected 6 fields, got {}", parts.len())) };
    Ok(SemioModelElement { id: dec_str(id)?, class: dec_element_class(class)?, placement: dec_transform(placement)?, geometry: dec_geometry_ref(geometry)?, spatial_id: decode_option(spatial_id, dec_str)?, psets: dec_list(psets, dec_property_set)? })
}

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

pub(crate) fn enc_relation(r: &ModelRelation) -> String {
    format!("[{},{},{},{}]", enc_str(&r.id), enc_relation_kind(&r.kind), enc_str(&r.from), enc_str(&r.to))
}
pub(crate) fn dec_relation(s: &str) -> Result<ModelRelation, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, kind, from, to] = parts.as_slice() else { return Err(format!("relation: expected 4 fields, got {}", parts.len())) };
    Ok(ModelRelation { id: dec_str(id)?, kind: dec_relation_kind(kind)?, from: dec_str(from)?, to: dec_str(to)? })
}
//#endregion 🔖️ValueCodecs

//#region 🔖️DiffValueCodecs
fn enc_spatial_node_diff(d: &SpatialNodeDiff) -> String {
    format!(
        "[{},{},{},{}]",
        encode_option(&d.kind, |v: &SpatialKind| enc_spatial_kind(v).to_string()),
        encode_option(&d.name, |v: &String| enc_str(v)),
        encode_option(&d.parent_id, |inner: &Option<String>| encode_option(inner, |v: &String| enc_str(v))),
        encode_option(&d.placement, enc_transform),
    )
}
fn dec_spatial_node_diff(s: &str) -> Result<SpatialNodeDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [kind, name, parent_id, placement] = parts.as_slice() else { return Err(format!("spatial node diff: expected 4 fields, got {}", parts.len())) };
    Ok(SpatialNodeDiff { kind: decode_option(kind, dec_spatial_kind)?, name: decode_option(name, dec_str)?, parent_id: decode_option(parent_id, |s| decode_option(s, dec_str))?, placement: decode_option(placement, dec_transform)? })
}

fn enc_element_diff(d: &SemioModelElementDiff) -> String {
    format!(
        "[{},{},{},{},{}]",
        encode_option(&d.class, enc_element_class),
        encode_option(&d.placement, enc_transform),
        encode_option(&d.geometry, enc_geometry_ref),
        encode_option(&d.spatial_id, |inner: &Option<String>| encode_option(inner, |v: &String| enc_str(v))),
        encode_option(&d.psets, |v: &Vec<PropertySet>| enc_list(v, enc_property_set)),
    )
}
fn dec_element_diff(s: &str) -> Result<SemioModelElementDiff, String> {
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

fn enc_relation_diff(d: &ModelRelationDiff) -> String {
    format!("[{},{},{}]", encode_option(&d.kind, enc_relation_kind), encode_option(&d.from, |v: &String| enc_str(v)), encode_option(&d.to, |v: &String| enc_str(v)))
}
fn dec_relation_diff(s: &str) -> Result<ModelRelationDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [kind, from, to] = parts.as_slice() else { return Err(format!("relation diff: expected 3 fields, got {}", parts.len())) };
    Ok(ModelRelationDiff { kind: decode_option(kind, dec_relation_kind)?, from: decode_option(from, dec_str)?, to: decode_option(to, dec_str)? })
}
//#endregion 🔖️DiffValueCodecs

//#region 🔖️TopLevel
pub(crate) fn enc_spatial_diff(d: &SpatialDiff) -> String {
    enc_named_triple(d, |k: &String| enc_str(k), enc_spatial_node_diff, enc_spatial_node)
}
pub(crate) fn dec_spatial_diff(s: &str) -> Result<SpatialDiff, String> {
    dec_named_triple(s, dec_str, dec_spatial_node_diff, dec_spatial_node)
}
pub(crate) fn enc_elements_diff(d: &ElementsDiff) -> String {
    enc_named_triple(d, |k: &String| enc_str(k), enc_element_diff, enc_element)
}
pub(crate) fn dec_elements_diff(s: &str) -> Result<ElementsDiff, String> {
    dec_named_triple(s, dec_str, dec_element_diff, dec_element)
}
pub(crate) fn enc_relations_diff(d: &RelationsDiff) -> String {
    enc_named_triple(d, |k: &String| enc_str(k), enc_relation_diff, enc_relation)
}
pub(crate) fn dec_relations_diff(s: &str) -> Result<RelationsDiff, String> {
    dec_named_triple(s, dec_str, dec_relation_diff, dec_relation)
}

fn print_semio_model_diff(d: &SemioModelDiff) -> String {
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
fn parse_semio_model_diff(line: &str) -> Result<SemioModelDiff, String> {
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

impl DiffCodec for SemioModelDiff {
    fn print_diff(&self) -> String {
        print_semio_model_diff(self)
    }
    fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        parse_semio_model_diff(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    /// ⚡️ P2 pilot (model): real binary diff frame, replacing the old `print_diff().into_bytes()`
    /// text-as-binary shortcut. `format u8` + `presence u8` (bit0=`spatial`, bit1=`elements`,
    /// bit2=`relations`) are two REAL fixed fields; each present collection then follows as its own
    /// varint-length-prefixed opaque blob (the same `enc_spatial_diff`/`enc_elements_diff`/
    /// `enc_relations_diff` bracket/hex text this type's `print_diff` already produces) — three
    /// independently-delimited segments rather than one bare trailing `bytes` because there can be
    /// 0-3 of them (chaining a `Cond` per-segment hits the `protocol-cond-cannot-chain` gap: a
    /// second `if`-guard on a field that was itself only conditionally decoded hard-errors
    /// `eval_cond` — same gap `stdio.semio.flow`'s own diff facet hit first).
    fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const DIFF_BINARY_FORMAT: u8 = 1;
        let mut presence = 0u8;
        if self.spatial.is_some() {
            presence |= 0b001;
        }
        if self.elements.is_some() {
            presence |= 0b010;
        }
        if self.relations.is_some() {
            presence |= 0b100;
        }
        let mut out = vec![DIFF_BINARY_FORMAT, presence];
        if let Some(v) = &self.spatial {
            write_str_lp(&mut out, &enc_spatial_diff(v));
        }
        if let Some(v) = &self.elements {
            write_str_lp(&mut out, &enc_elements_diff(v));
        }
        if let Some(v) = &self.relations {
            write_str_lp(&mut out, &enc_relations_diff(v));
        }
        Ok(out)
    }
    fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const DIFF_BINARY_FORMAT: u8 = 1;
        if bytes.len() < 2 {
            return Err(protocol::ProtocolError::Malformed { what: "diff header", offset: 0, detail: "truncated (need format+presence)".to_string() });
        }
        if bytes[0] != DIFF_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "diff format", offset: 0, detail: format!("unsupported diff format {}", bytes[0]) });
        }
        let presence = bytes[1];
        let mut reader = store::ByteReader::new(&bytes[2..]);
        let spatial = if presence & 0b001 != 0 {
            let text = read_str_lp(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff spatial blob", offset: 2, detail: e })?;
            Some(dec_spatial_diff(&text).map_err(|e| protocol::ProtocolError::Malformed { what: "diff spatial text", offset: 2, detail: e })?)
        } else {
            None
        };
        let elements = if presence & 0b010 != 0 {
            let text = read_str_lp(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff elements blob", offset: 2, detail: e })?;
            Some(dec_elements_diff(&text).map_err(|e| protocol::ProtocolError::Malformed { what: "diff elements text", offset: 2, detail: e })?)
        } else {
            None
        };
        let relations = if presence & 0b100 != 0 {
            let text = read_str_lp(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff relations blob", offset: 2, detail: e })?;
            Some(dec_relations_diff(&text).map_err(|e| protocol::ProtocolError::Malformed { what: "diff relations text", offset: 2, detail: e })?)
        } else {
            None
        };
        Ok(SemioModelDiff { spatial, elements, relations })
    }
}
//#endregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec

//#region 🔖️Demo
/// 🏗️ Sweep base -- one node/element/relation that survives (and gets modified in every field),
/// one that gets removed. `keep-spatial`'s `parent_id` starts `Some(..)` so `sweep_b` can exercise
/// the `Some(None)` tri-state transition; `keep-element`'s `spatial_id` starts `None` so `sweep_b`
/// exercises the opposite transition (None -> Some). Module-scope (not nested in `mod tests`) so
/// `demo_diff_cases` below and the composer's `conformance_laws` can both reuse it — single source
/// of truth, same convention `stdio.semio.flow`'s own diff facet demo cases use.
#[cfg(test)]
pub(crate) fn moved_transform(x: f64) -> SemioTransform {
    SemioTransform { translation: SemioPoint3 { x, y: 0.0, z: 0.0 }, rotation: SemioQuaternion::default(), scale: SemioPoint3 { x: 1.0, y: 1.0, z: 1.0 } }
}
#[cfg(test)]
pub(crate) fn sweep_a() -> SemioModelSnapshot {
    SemioModelSnapshot {
        schema: SemioModelSnapshot::default().schema,
        spatial: vec![
            SpatialNode { id: "keep-spatial".into(), kind: SpatialKind::Site, name: "Alpha".into(), parent_id: Some("orphan-parent".into()), placement: SemioTransform::identity() },
            SpatialNode { id: "gone-spatial".into(), kind: SpatialKind::Building, name: "ToRemove".into(), parent_id: None, placement: SemioTransform::identity() },
        ],
        elements: vec![
            SemioModelElement {
                id: "keep-element".into(),
                class: ElementClass::Wall,
                placement: SemioTransform::identity(),
                geometry: GeometryRef::None,
                spatial_id: None,
                psets: vec![PropertySet { name: "Pset_A".into(), properties: vec![Property { key: "k".into(), value: PsetValue::Boolean { value: false } }] }],
            },
            SemioModelElement { id: "gone-element".into(), class: ElementClass::Door, placement: SemioTransform::identity(), geometry: GeometryRef::None, spatial_id: None, psets: vec![] },
        ],
        relations: vec![ModelRelation { id: "keep-relation".into(), kind: RelationKind::Aggregates, from: "a".into(), to: "b".into() }, ModelRelation { id: "gone-relation".into(), kind: RelationKind::ConnectsTo, from: "x".into(), to: "y".into() }],
    }
}
#[cfg(test)]
pub(crate) fn sweep_b() -> SemioModelSnapshot {
    SemioModelSnapshot {
        schema: SemioModelSnapshot::default().schema,
        spatial: vec![
            SpatialNode { id: "keep-spatial".into(), kind: SpatialKind::Building, name: "Alpha Renamed".into(), parent_id: None, placement: moved_transform(9.0) },
            SpatialNode { id: "new-spatial".into(), kind: SpatialKind::Storey, name: "Fresh".into(), parent_id: Some("keep-spatial".into()), placement: SemioTransform::identity() },
        ],
        elements: vec![
            SemioModelElement {
                id: "keep-element".into(),
                class: ElementClass::Slab,
                placement: moved_transform(4.0),
                geometry: GeometryRef::Brep { brep_id: "b1".into() },
                spatial_id: Some("keep-spatial".into()),
                psets: vec![PropertySet { name: "Pset_B".into(), properties: vec![Property { key: "k2".into(), value: PsetValue::Number { value: 3.5 } }] }],
            },
            SemioModelElement { id: "new-element".into(), class: ElementClass::Column, placement: SemioTransform::identity(), geometry: GeometryRef::Mesh { mesh_id: "m1".into() }, spatial_id: None, psets: vec![] },
        ],
        relations: vec![ModelRelation { id: "keep-relation".into(), kind: RelationKind::ContainedIn, from: "c".into(), to: "d".into() }, ModelRelation { id: "new-relation".into(), kind: RelationKind::FillsVoid, from: "e".into(), to: "f".into() }],
    }
}

/// 🌱 Representative `SemioModelDiff` cases (empty/no-op, a full spatial+element+relation sweep
/// both directions, a bare spatial-node insert, a bare element insert, a bare relation insert) —
/// single source of truth for `grammar_conformance_law`/`protocol_walk_law` in
/// `🎹️composer/🦀️component.rs`.
#[cfg(test)]
pub(crate) fn demo_diff_cases() -> Vec<SemioModelDiff> {
    let a = sweep_a();
    let b = sweep_b();
    let mut cases = vec![SemioModelDiff::default(), <SemioModelDiff as DiffAlgebra<SemioModelSnapshot>>::between(&a, &b), <SemioModelDiff as DiffAlgebra<SemioModelSnapshot>>::between(&b, &a)];
    cases.push(SemioModelDiff {
        spatial: Some(NamedTripleDiff { added: vec![SpatialNode { id: "demo-spatial".into(), kind: SpatialKind::Space, name: "Demo".into(), parent_id: None, placement: SemioTransform::identity() }], ..Default::default() }),
        elements: None,
        relations: None,
    });
    cases.push(SemioModelDiff {
        spatial: None,
        elements: Some(NamedTripleDiff {
            added: vec![SemioModelElement { id: "demo-element".into(), class: ElementClass::Beam, placement: SemioTransform::identity(), geometry: GeometryRef::None, spatial_id: None, psets: vec![] }],
            ..Default::default()
        }),
        relations: None,
    });
    cases.push(SemioModelDiff { spatial: None, elements: None, relations: Some(NamedTripleDiff { added: vec![ModelRelation { id: "demo-relation".into(), kind: RelationKind::ConnectsTo, from: "a".into(), to: "b".into() }], ..Default::default() }) });
    cases
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// 🧪️ field_sweep: `sweep_a`/`sweep_b` differ in EVERY mutable field across all three
    /// collections (one removed, one modified-in-every-field, one added each), and exercise the
    /// `parent_id`/`spatial_id` tri-states in both directions (Some->None on spatial, None->Some
    /// on elements).
    #[test]
    fn field_sweep() {
        let a = sweep_a();
        let b = sweep_b();
        let d = SemioModelDiff::between(&a, &b);

        let spatial = d.spatial.as_ref().expect("spatial diff present");
        assert_eq!(spatial.removed, vec!["gone-spatial".to_string()]);
        assert_eq!(spatial.added.len(), 1);
        let keep_spatial = &spatial.modified.iter().find(|m| m.key == "keep-spatial").expect("keep-spatial modified").diff;
        assert!(keep_spatial.kind.is_some() && keep_spatial.name.is_some() && keep_spatial.placement.is_some());
        assert_eq!(keep_spatial.parent_id, Some(None), "Some->None parent_id tri-state must surface as Some(None)");

        let elements = d.elements.as_ref().expect("elements diff present");
        assert_eq!(elements.removed, vec!["gone-element".to_string()]);
        assert_eq!(elements.added.len(), 1);
        let keep_element = &elements.modified.iter().find(|m| m.key == "keep-element").expect("keep-element modified").diff;
        assert!(keep_element.class.is_some() && keep_element.placement.is_some() && keep_element.geometry.is_some() && keep_element.psets.is_some());
        assert_eq!(keep_element.spatial_id, Some(Some("keep-spatial".to_string())), "None->Some spatial_id tri-state must surface");

        let relations = d.relations.as_ref().expect("relations diff present");
        assert_eq!(relations.removed, vec!["gone-relation".to_string()]);
        assert_eq!(relations.added.len(), 1);
        let keep_relation = &relations.modified.iter().find(|m| m.key == "keep-relation").expect("keep-relation modified").diff;
        assert!(keep_relation.kind.is_some() && keep_relation.from.is_some() && keep_relation.to.is_some());

        assert_eq!(d.apply(&a).expect("apply must succeed for a well-formed fixture"), b);
        assert!(SemioModelDiff::between(&a, &a).is_empty());
    }

    /// 🧪️ between_roundtrip_law: `between(a,b).apply(a) == b` and the symmetric direction.
    #[test]
    fn between_roundtrip_law() {
        let a = sweep_a();
        let b = sweep_b();
        assert_eq!(SemioModelDiff::between(&a, &b).apply(&a).expect("apply must succeed for a well-formed fixture"), b);
        assert_eq!(SemioModelDiff::between(&b, &a).apply(&b).expect("apply must succeed for a well-formed fixture"), a);
    }

    /// 🧪️ inverse_law: `d.inverse(base).apply(&d.apply(base)) == base`.
    #[test]
    fn inverse_law() {
        let a = sweep_a();
        let b = sweep_b();
        let d = SemioModelDiff::between(&a, &b);
        let applied = d.apply(&a).expect("apply must succeed for a well-formed fixture");
        let inv = d.inverse(&a);
        assert_eq!(inv.apply(&applied).expect("apply must succeed for a well-formed fixture"), a);
    }

    /// 🧪️ absorb_law: `absorb(d1,d2).apply(base) == d2.apply(&d1.apply(base))`, including the
    /// canonical add-then-remove-before / add-then-set-field cases from schema-design.md.
    #[test]
    fn absorb_law() {
        let base = sweep_a();
        let mid = sweep_b();
        let mut after = sweep_b();
        after.elements.push(SemioModelElement { id: "third-element".into(), class: ElementClass::Beam, placement: SemioTransform::identity(), geometry: GeometryRef::None, spatial_id: None, psets: vec![] });
        after.relations.retain(|r| r.id != "new-relation");

        let d1 = SemioModelDiff::between(&base, &mid);
        let d2 = SemioModelDiff::between(&mid, &after);
        let sequential = d2.apply(&d1.apply(&base).expect("apply must succeed for a well-formed fixture")).expect("apply must succeed for a well-formed fixture");

        let mut absorbed = d1.clone();
        absorbed.absorb(d2.clone());
        assert_eq!(absorbed.apply(&base).expect("apply must succeed for a well-formed fixture"), sequential);
        assert_eq!(absorbed.apply(&base).expect("apply must succeed for a well-formed fixture"), after);

        // Canonical case: Insert(X) absorbed with Remove(X) annihilates the add.
        let mut with_add = SemioModelDiff::default();
        with_add.elements = Some(NamedTripleDiff {
            removed: vec![],
            modified: vec![],
            added: vec![SemioModelElement { id: "temp".into(), class: ElementClass::Wall, placement: SemioTransform::identity(), geometry: GeometryRef::None, spatial_id: None, psets: vec![] }],
        });
        let mut with_remove = SemioModelDiff::default();
        with_remove.elements = Some(NamedTripleDiff { removed: vec!["temp".to_string()], modified: vec![], added: vec![] });
        let mut annihilated = with_add.clone();
        annihilated.absorb(with_remove);
        let elements_diff = annihilated.elements.as_ref().expect("elements diff present after annihilation");
        assert!(elements_diff.added.is_empty() && elements_diff.removed.is_empty(), "add-then-remove-before must annihilate cleanly, got {elements_diff:?}");

        // Canonical case: Insert(X) absorbed with Set(X.field) patches into the carried payload.
        let mut with_set = SemioModelDiff::default();
        with_set.elements = Some(NamedTripleDiff { removed: vec![], modified: vec![NamedModified { key: "temp".to_string(), diff: SemioModelElementDiff { class: Some(ElementClass::Door), ..Default::default() } }], added: vec![] });
        let mut patched_add = with_add;
        patched_add.absorb(with_set);
        let patched = patched_add.elements.as_ref().expect("elements diff present after patch-into-added");
        assert_eq!(patched.added.len(), 1);
        assert_eq!(patched.added[0].class, ElementClass::Door, "add-then-set-field must patch INTO the carried added payload");
    }

    /// 🧪️ diff_codec_text_binary_roundtrip_law: hand-rolled `DiffCodec` text+binary round trip.
    #[test]
    fn diff_codec_text_binary_roundtrip_law() {
        let a = sweep_a();
        let b = sweep_b();
        let d = SemioModelDiff::between(&a, &b);

        let printed = d.print_diff();
        assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
        let parsed = SemioModelDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
        assert_eq!(parsed, d);

        let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
        let decoded = SemioModelDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
        assert_eq!(decoded, d);

        // Empty diff also round-trips (the common "no change" case every artifact's codec hits).
        let empty = SemioModelDiff::default();
        assert_eq!(empty.print_diff(), "");
        assert_eq!(SemioModelDiff::parse_diff("").unwrap(), empty);
    }
}
//#endregion 🔖️Tests
