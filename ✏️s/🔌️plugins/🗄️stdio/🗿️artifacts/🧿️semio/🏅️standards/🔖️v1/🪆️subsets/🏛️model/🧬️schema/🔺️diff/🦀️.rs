//! 🔺️ SemioModelDiff — handcrafted sparse diff over `SemioModelSnapshot`. No
//! `snapshot: Option<SemioModelSnapshot>` full-replace slot — even `SetSnapshot`'s diff is the
//! sparse field-by-field `SemioModelDiff::between(base, next)`.
//!
//! `spatial`/`elements`/`relations` (all id-keyed) are diffed via the SHARED
//! `crate::standards::v1::subsets::base::schema::triples::NamedTripleDiff<K,D,T>` engine
//! (w1b-type-ownership.md: "Use `🧰️triples`... instead of reinventing it") — this file does NOT
//! redefine that container or its wire codec, only the generic apply/between/inverse/absorb glue
//! functions a specific artifact's collections need (mirrors bcf's own local generic engine,
//! `f6-final-summary.md` §4.4, minus the container type itself, which now has one shared home).

use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint3, SemioQuaternion, SemioTransform};
use crate::standards::v1::subsets::base::schema::triples::{dec_named_triple, enc_named_triple, NamedModified, NamedTripleDiff};


use crate::standards::v1::subsets::model::schema::snapshot::{ElementClass, GeometryRef, ModelRelation, Property, PropertySet, PsetValue, RelationKind, SemioModelElement, SemioModelSnapshot, SpatialKind, SpatialNode};
use protocol::command::DiffAlgebra;
use protocol::{DiffBinary,DiffCodec,DiffText, MutationDiff};

//#region 🔖️GenericNamedEngine
/// 🧮️ Generic name/id-keyed collection glue — `between`/`apply`/`inverse`/`absorb` over the
/// shared `NamedTripleDiff<K,D,T>` container, written once and instantiated per collection below
/// (mirrors bcf's own local copy, `💬️bcf/…/🔺️diff/🦀️.rs` §GenericNamedEngine).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
    let faithful = reproduces_order(base, other, &removed, &added, &key_of);
    if !faithful {
        return Some(NamedTripleDiff { removed: base.iter().map(&key_of).collect(), modified: Vec::new(), added: other.to_vec() });
    }
    if removed.is_empty() && modified.is_empty() && added.is_empty() {
        return None;
    }
    Some(NamedTripleDiff { removed, modified, added })
}

/// 🧮️ Whether the sparse triple can reproduce `other`'s ORDER. [`apply_named`] keeps every surviving
/// member where it already stood and pushes `added` onto the tail, so the key sequence it produces is
/// exactly `survivors(base order) ++ added(other order)`. When `other` orders its members any other
/// way — a member that survives but moved, or a new member that belongs before an old one — the
/// sparse triple is not a faithful description of the transition and [`between_named`] degrades to a
/// full replacement instead, which the same [`apply_named`] reproduces exactly.
///
/// 🐛️ Same defect `🌊️flow`'s own `between_named` carries this guard for, measured here on the real
/// artifact: `set-snapshot` replacing the capsule tower's `[site, building, storey]` with
/// `[storey, site]` produced `[site, storey]`, because both survivors kept the positions they held in
/// the base while the building was dropped. `set-snapshot` means the snapshot BECOMES the named one,
/// order included, and now it does. Reached here through `diff_set_snapshot`, which is why every
/// spelling of a whole-collection replacement is covered rather than only the reordering one.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn reproduces_order<K, T>(base: &[T], other: &[T], removed: &[K], added: &[T], key_of: &impl Fn(&T) -> K) -> bool
where
    K: PartialEq,
{
    let produced = base.iter().map(key_of).filter(|k| !removed.contains(k)).chain(added.iter().map(key_of));
    let mut target = other.iter().map(key_of);
    for key in produced {
        match target.next() {
            Some(expected) if expected == key => {}
            _ => return false,
        }
    }
    target.next().is_none()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_named<K, T, D>(d1: NamedTripleDiff<K, D, T>, d2: &NamedTripleDiff<K, D, T>, key_of: impl Fn(&T) -> K, absorb_item: impl Fn(D, D) -> D, apply_item: impl Fn(&mut T, &D)) -> NamedTripleDiff<K, D, T>
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
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SpatialNodeDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<SpatialKind>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub placement: Option<SemioTransform>,
}

/// 🔺️ Per-element sparse diff. `spatial_id` is tri-state (`Some(None)` = removed from the
/// spatial tree). `psets` is whole-value replaced (weak entity, never sub-diffed per the recipe).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SemioModelElementDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub class: Option<ElementClass>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub placement: Option<SemioTransform>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub geometry: Option<GeometryRef>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub spatial_id: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub psets: Option<Vec<PropertySet>>,
}

/// 🔺️ Per-relation sparse diff — `id` is the key, so only `kind`/`from`/`to` can change.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct ModelRelationDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<RelationKind>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}

/// 🔺️ Diff for `s.stdio.semio.model`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SemioModelDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub spatial: Option<SpatialDiff>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub elements: Option<ElementsDiff>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub relations: Option<RelationsDiff>,
}
//#endregion 🔖️DiffTypes

//#region 🔖️Apply
impl MutationDiff<SemioModelSnapshot> for SemioModelDiff {
    fn apply(&self, base: &SemioModelSnapshot) -> protocol::MutationApplyResult<SemioModelSnapshot> {
        let mut next = base.clone();
        if let Some(d) = &self.spatial {
            crate::standards::v1::subsets::base::schema::triples::validate_named_triple(&next.spatial, d, |item| item.id.clone(), |item| item.id.clone(), ["spatial"])?;
            apply_named(&mut next.spatial, d, |n: &SpatialNode| n.id.clone(), apply_spatial);
        }
        if let Some(d) = &self.elements {
            crate::standards::v1::subsets::base::schema::triples::validate_named_triple(&next.elements, d, |item| item.id.clone(), |item| item.id.clone(), ["elements"])?;
            apply_named(&mut next.elements, d, |e: &SemioModelElement| e.id.clone(), apply_element);
        }
        if let Some(d) = &self.relations {
            crate::standards::v1::subsets::base::schema::triples::validate_named_triple(&next.relations, d, |item| item.id.clone(), |item| item.id.clone(), ["relations"])?;
            apply_named(&mut next.relations, d, |r: &ModelRelation| r.id.clone(), apply_relation);
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        self.spatial = match (self.spatial.take(), other.spatial) {
            (None, b) => b,
            (a, None) => a,
            (Some(a), Some(b)) => Some(absorb_named(a, &b, |n: &SpatialNode| n.id.clone(), absorb_spatial_diff, apply_spatial)),
        };
        self.elements = match (self.elements.take(), other.elements) {
            (None, b) => b,
            (a, None) => a,
            (Some(a), Some(b)) => Some(absorb_named(a, &b, |e: &SemioModelElement| e.id.clone(), absorb_element_diff, apply_element)),
        };
        self.relations = match (self.relations.take(), other.relations) {
            (None, b) => b,
            (a, None) => a,
            (Some(a), Some(b)) => Some(absorb_named(a, &b, |r: &ModelRelation| r.id.clone(), absorb_relation_diff, apply_relation)),
        };
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_spatial(base: &SpatialNode, diff: &SpatialNodeDiff) -> SpatialNodeDiff {
    SpatialNodeDiff { kind: diff.kind.as_ref().map(|_| base.kind), name: diff.name.as_ref().map(|_| base.name.clone()), parent_id: diff.parent_id.as_ref().map(|_| base.parent_id.clone()), placement: diff.placement.as_ref().map(|_| base.placement) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_element(base: &SemioModelElement, diff: &SemioModelElementDiff) -> SemioModelElementDiff {
    SemioModelElementDiff {
        class: diff.class.as_ref().map(|_| base.class.clone()),
        placement: diff.placement.as_ref().map(|_| base.placement),
        geometry: diff.geometry.as_ref().map(|_| base.geometry.clone()),
        spatial_id: diff.spatial_id.as_ref().map(|_| base.spatial_id.clone()),
        psets: diff.psets.as_ref().map(|_| base.psets.clone()),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_relation(base: &ModelRelation, diff: &ModelRelationDiff) -> ModelRelationDiff {
    ModelRelationDiff { kind: diff.kind.as_ref().map(|_| base.kind.clone()), from: diff.from.as_ref().map(|_| base.from.clone()), to: diff.to.as_ref().map(|_| base.to.clone()) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_snapshot(base: &SemioModelSnapshot, next: &SemioModelSnapshot) -> SemioModelDiff {
    SemioModelDiff::between(base, next)
}
//#endregion 🔖️SetSnapshot

//#region 🔖️HandcraftedDiffCodec
/// 🎙️ Hand-rolled `protocol::DiffCodec` — same bracket-depth-aware token grammar bcf/gif/svg use
/// (see `crate::standards::v1::subsets::base::schema::triples` for the shared
/// `split_top_level`/`strip_brackets` primitives this reuses rather than redefining). This
/// artifact's own copy of the small hex/option/list primitive set (each artifact writes its own,
/// per bcf's own module doc rationale -- cross-artifact imports would be architecturally wrong).
//#region 🔖️Primitives














//#endregion 🔖️Primitives

//#region 🔖️ValueCodecs




































//#endregion 🔖️ValueCodecs

//#region 🔖️DiffValueCodecs








//#endregion 🔖️DiffValueCodecs

//#region 🔖️TopLevel











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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn moved_transform(x: f64) -> SemioTransform {
    SemioTransform { translation: SemioPoint3 { x, y: 0.0, z: 0.0 }, rotation: SemioQuaternion::default(), scale: SemioPoint3 { x: 1.0, y: 1.0, z: 1.0 } }
}
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
/// `🎹️composer/🦀️.rs`.
#[cfg(all(test, feature = "conversion-model"))]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
