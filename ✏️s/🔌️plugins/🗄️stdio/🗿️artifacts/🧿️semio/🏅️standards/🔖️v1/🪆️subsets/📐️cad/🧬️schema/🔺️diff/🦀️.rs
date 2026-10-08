//! 🔺️ SemioCadDiff — handcrafted sparse diff over `SemioCadSnapshot`. No
//! `replacement: Option<SemioCadSnapshot>` full-replace slot — even `SetSnapshot`'s diff is the
//! sparse field-by-field `SemioCadDiff::between(base, next)`.
//!
//! `layers`/`blocks`/`entities` (name/handle-keyed) are diffed via the SHARED
//! `engine::triples::NamedTripleDiff<K,D,T>` (per the w1b type-ownership brief — do NOT redefine
//! this per-subset, unlike bcf/docx which predate the shared module). `blocks[].entities` reuses
//! the SAME `CadEntitiesDiff` alias one level deeper (a block is just a second id-keyed entity
//! collection, same shape as the top-level one). `CadEntity` itself is a WEAK value (whole-value
//! replaced, never sub-diffed — same treatment `BcfCamera`/`XlsxCellValue` get), so
//! `CadEntityRecordDiff.entity` is a plain `Option<CadEntity>`, not a nested diff type.

use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use crate::standards::v1::subsets::base::schema::triples::{NamedModified, NamedTripleDiff, NamedAdded};



use crate::standards::v1::subsets::cad::schema::snapshot::{CadBlock, CadEntity, CadEntityRecord, CadLayer, SemioCadSnapshot};
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;

//#region 🔖️DiffTypes
pub type CadLayersDiff = NamedTripleDiff<String, CadLayerDiff, NamedAdded<CadLayer>>;
pub type CadBlocksDiff = NamedTripleDiff<String, CadBlockDiff, NamedAdded<CadBlock>>;
pub type CadEntitiesDiff = NamedTripleDiff<String, CadEntityRecordDiff, NamedAdded<CadEntityRecord>>;

/// 🔺️ Per-layer sparse diff — all 3 mutable fields.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct CadLayerDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub color_index: Option<i32>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub line_type: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
}

/// 🔺️ Per-block sparse diff — `base_point` scalar; `entities` a nested id-keyed triple.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct CadBlockDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub base_point: Option<SemioPoint2>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub entities: Option<CadEntitiesDiff>,
}

/// 🔺️ Per-entity-record sparse diff — `layer` is a scalar patch; `entity` is whole-value replaced
/// (weak value struct, per this module's doc comment).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct CadEntityRecordDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub layer: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub entity: Option<CadEntity>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SemioCadDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub layers: Option<CadLayersDiff>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub blocks: Option<CadBlocksDiff>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub entities: Option<CadEntitiesDiff>,
}
//#endregion 🔖️DiffTypes

//#region 🔖️GenericNamedEngine
/// 🏷️ Name/id-keyed collection algebra (apply/between/inverse/absorb), generic over key `K`, item
/// `T`, per-field diff `D`. Operates on the SHARED `engine::triples::NamedTripleDiff` type — this
/// artifact's own copy of the algorithm (cross-artifact algorithm imports would be architecturally
/// wrong, same rationale bcf's own copy documents), not a re-definition of the data shape.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_named<K, T, D>(items: &mut Vec<T>, diff: &NamedTripleDiff<K, D, NamedAdded<T>>, key_of: impl Fn(&T) -> K, apply_item: impl Fn(&mut T, &D))
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
    let mut ascending: Vec<&NamedAdded<T>> = diff.added.iter().collect();
    ascending.sort_by_key(|a| a.index);
    for a in ascending {
        let at = a.index.min(items.len());
        items.insert(at, a.item.clone());
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn reproduces_order<K, T>(base: &[T], other: &[T], removed: &[K], added: &[NamedAdded<T>], key_of: &impl Fn(&T) -> K) -> bool
where
    K: PartialEq,
{
    let mut keys: Vec<K> = base.iter().map(key_of).filter(|k| !removed.contains(k)).collect();
    let mut ascending: Vec<&NamedAdded<T>> = added.iter().collect();
    ascending.sort_by_key(|a| a.index);
    for a in ascending {
        let at = a.index.min(keys.len());
        keys.insert(at, key_of(&a.item));
    }
    keys.len() == other.len() && keys.iter().zip(other.iter().map(key_of)).all(|(produced, expected)| *produced == expected)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_named<K, T, D>(base_items: &[T], diff: &NamedTripleDiff<K, D, NamedAdded<T>>, key_of: impl Fn(&T) -> K, inverse_item: impl Fn(&T, &D) -> D) -> NamedTripleDiff<K, D, NamedAdded<T>>
where
    K: PartialEq + Clone,
    T: Clone,
{
    let removed: Vec<K> = diff.added.iter().map(|a| key_of(&a.item)).collect();
    let mut modified = Vec::new();
    for m in &diff.modified {
        if let Some(original) = base_items.iter().find(|i| key_of(i) == m.key) {
            modified.push(NamedModified { key: m.key.clone(), diff: inverse_item(original, &m.diff) });
        }
    }
    let mut added: Vec<NamedAdded<T>> = diff.removed.iter().filter_map(|k| base_items.iter().position(|i| &key_of(i) == k).map(|index| NamedAdded { index, item: base_items[index].clone() })).collect();
    added.sort_by_key(|a| a.index);
    NamedTripleDiff { removed, modified, added }
}

/// 🧮️ Key-identity absorb (not position) — a `d2`-removal of a `d1`-added key annihilates the add;
/// a `d2`-modify of a `d1`-added key patches into the carried payload; everything else composes on
/// the shared key space. Mirrors bcf's `absorb_named` (same canonical cases, B-R7).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_named<K, T, D>(d1: NamedTripleDiff<K, D, NamedAdded<T>>, d2: &NamedTripleDiff<K, D, NamedAdded<T>>, key_of: impl Fn(&T) -> K, absorb_item: impl Fn(D, D) -> D, apply_item: impl Fn(&mut T, &D)) -> NamedTripleDiff<K, D, NamedAdded<T>>
where
    K: PartialEq + Clone,
    T: Clone,
    D: Clone,
{
    let d1_added_keys: Vec<K> = d1.added.iter().map(|a| key_of(&a.item)).collect();
    let mut removed = d1.removed.clone();
    let mut annihilated: Vec<K> = Vec::new();
    for k in &d2.removed {
        if d1_added_keys.contains(k) {
            annihilated.push(k.clone());
        } else if !removed.contains(k) {
            removed.push(k.clone());
        }
    }
    let mut working_added: Vec<NamedAdded<T>> = d1.added.into_iter().filter(|a| !annihilated.contains(&key_of(&a.item))).collect();
    let mut modified: Vec<NamedModified<K, D>> = d1.modified.into_iter().filter(|m| !removed.contains(&m.key)).collect();
    for m2 in &d2.modified {
        if let Some(added) = working_added.iter_mut().find(|a| key_of(&a.item) == m2.key) {
            apply_item(&mut added.item, &m2.diff);
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
        let k2 = key_of(&a2.item);
        match working_added.iter_mut().find(|a| key_of(&a.item) == k2) {
            Some(existing) => *existing = a2.clone(),
            None => working_added.push(a2.clone()),
        }
    }
    NamedTripleDiff { removed, modified, added: working_added }
}
//#endregion 🔖️GenericNamedEngine

//#region 🔖️WrapHelpers
/// 🧭️ Lowers a per-layer leaf diff into a full `SemioCadDiff` (mirrors bcf's `wrap_topic_diff`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn wrap_layer_diff(name: &str, diff: CadLayerDiff) -> SemioCadDiff {
    SemioCadDiff { layers: Some(CadLayersDiff { removed: Vec::new(), modified: vec![NamedModified { key: name.to_string(), diff }], added: Vec::new() }), blocks: None, entities: None }
}

/// 🧭️ Lowers a per-block leaf diff into a full `SemioCadDiff`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn wrap_block_diff(name: &str, diff: CadBlockDiff) -> SemioCadDiff {
    SemioCadDiff { layers: None, blocks: Some(CadBlocksDiff { removed: Vec::new(), modified: vec![NamedModified { key: name.to_string(), diff }], added: Vec::new() }), entities: None }
}

/// 🧭️ Lowers a per-top-level-entity leaf diff into a full `SemioCadDiff`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn wrap_entity_diff(handle: &str, diff: CadEntityRecordDiff) -> SemioCadDiff {
    SemioCadDiff { layers: None, blocks: None, entities: Some(CadEntitiesDiff { removed: Vec::new(), modified: vec![NamedModified { key: handle.to_string(), diff }], added: Vec::new() }) }
}

/// 🧭️ Lowers a per-block-entity leaf diff (inside block `block_name`) into a full `SemioCadDiff`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn wrap_block_entity_diff(block_name: &str, handle: &str, diff: CadEntityRecordDiff) -> SemioCadDiff {
    wrap_block_diff(block_name, CadBlockDiff { base_point: None, entities: Some(CadEntitiesDiff { removed: Vec::new(), modified: vec![NamedModified { key: handle.to_string(), diff }], added: Vec::new() }) })
}
//#endregion 🔖️WrapHelpers

//#region 🔖️Apply
impl MutationDiff<SemioCadSnapshot> for SemioCadDiff {
    fn apply(&self, base: &SemioCadSnapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<SemioCadSnapshot> {
        let mut next = base.clone();
        if let Some(ld) = &self.layers {
            crate::standards::v1::subsets::base::schema::triples::validate_named_triple(&next.layers, ld, |layer| layer.name.clone(), |added| added.item.name.clone(), ["layers"])?;
            apply_named(&mut next.layers, ld, |l| l.name.clone(), apply_layer);
        }
        if let Some(bd) = &self.blocks {
            crate::standards::v1::subsets::base::schema::triples::validate_named_triple(&next.blocks, bd, |block| block.name.clone(), |added| added.item.name.clone(), ["blocks"])?;
            apply_named(&mut next.blocks, bd, |b| b.name.clone(), apply_block);
        }
        if let Some(ed) = &self.entities {
            crate::standards::v1::subsets::base::schema::triples::validate_named_triple(&next.entities, ed, |entity| entity.handle.clone(), |added| added.item.handle.clone(), ["entities"])?;
            apply_named(&mut next.entities, ed, |e| e.handle.clone(), apply_entity_record);
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        self.layers = match (self.layers.take(), other.layers) {
            (None, b) => b,
            (a, None) => a,
            (Some(a), Some(b)) => Some(absorb_named(a, &b, |l| l.name.clone(), absorb_layer_diff, apply_layer)),
        };
        self.blocks = match (self.blocks.take(), other.blocks) {
            (None, b) => b,
            (a, None) => a,
            (Some(a), Some(b)) => Some(absorb_named(a, &b, |bl| bl.name.clone(), absorb_block_diff, apply_block)),
        };
        self.entities = match (self.entities.take(), other.entities) {
            (None, b) => b,
            (a, None) => a,
            (Some(a), Some(b)) => Some(absorb_named(a, &b, |e| e.handle.clone(), absorb_entity_record_diff, apply_entity_record)),
        };
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_layer(layer: &mut CadLayer, diff: &CadLayerDiff) {
    if let Some(v) = &diff.color_index {
        layer.color_index = *v;
    }
    if let Some(v) = &diff.line_type {
        layer.line_type = v.clone();
    }
    if let Some(v) = &diff.visible {
        layer.visible = *v;
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_block(block: &mut CadBlock, diff: &CadBlockDiff) {
    if let Some(v) = &diff.base_point {
        block.base_point = *v;
    }
    if let Some(ed) = &diff.entities {
        apply_named(&mut block.entities, ed, |e| e.handle.clone(), apply_entity_record);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_entity_record(rec: &mut CadEntityRecord, diff: &CadEntityRecordDiff) {
    if let Some(v) = &diff.layer {
        rec.layer = v.clone();
    }
    if let Some(v) = &diff.entity {
        rec.entity = v.clone();
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_layer_diff(mut a: CadLayerDiff, b: CadLayerDiff) -> CadLayerDiff {
    if b.color_index.is_some() {
        a.color_index = b.color_index;
    }
    if b.line_type.is_some() {
        a.line_type = b.line_type;
    }
    if b.visible.is_some() {
        a.visible = b.visible;
    }
    a
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_block_diff(mut a: CadBlockDiff, b: CadBlockDiff) -> CadBlockDiff {
    if b.base_point.is_some() {
        a.base_point = b.base_point;
    }
    a.entities = match (a.entities.take(), b.entities) {
        (None, x) => x,
        (x, None) => x,
        (Some(x), Some(y)) => Some(absorb_named(x, &y, |e| e.handle.clone(), absorb_entity_record_diff, apply_entity_record)),
    };
    a
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_entity_record_diff(mut a: CadEntityRecordDiff, b: CadEntityRecordDiff) -> CadEntityRecordDiff {
    if b.layer.is_some() {
        a.layer = b.layer;
    }
    if b.entity.is_some() {
        a.entity = b.entity;
    }
    a
}
//#endregion 🔖️Apply

//#region 🔖️DiffAlgebra
impl DiffAlgebra<SemioCadSnapshot> for SemioCadDiff {
    fn inverse(&self, base: &SemioCadSnapshot) -> Self {
        SemioCadDiff {
            layers: self.layers.as_ref().map(|d| inverse_named(&base.layers, d, |l| l.name.clone(), inverse_layer)),
            blocks: self.blocks.as_ref().map(|d| inverse_named(&base.blocks, d, |b| b.name.clone(), inverse_block)),
            entities: self.entities.as_ref().map(|d| inverse_named(&base.entities, d, |e| e.handle.clone(), inverse_entity_record)),
        }
    }

    fn is_empty(&self) -> bool {
        self.layers.is_none() && self.blocks.is_none() && self.entities.is_none()
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_layer(base: &CadLayer, diff: &CadLayerDiff) -> CadLayerDiff {
    CadLayerDiff { color_index: diff.color_index.as_ref().map(|_| base.color_index), line_type: diff.line_type.as_ref().map(|_| base.line_type.clone()), visible: diff.visible.as_ref().map(|_| base.visible) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_block(base: &CadBlock, diff: &CadBlockDiff) -> CadBlockDiff {
    CadBlockDiff { base_point: diff.base_point.as_ref().map(|_| base.base_point), entities: diff.entities.as_ref().map(|d| inverse_named(&base.entities, d, |e| e.handle.clone(), inverse_entity_record)) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_entity_record(base: &CadEntityRecord, diff: &CadEntityRecordDiff) -> CadEntityRecordDiff {
    CadEntityRecordDiff { layer: diff.layer.as_ref().map(|_| base.layer.clone()), entity: diff.entity.as_ref().map(|_| base.entity.clone()) }
}

//#region 🔖️Demo
/// 🌱 Representative `SemioCadDiff` cases built declaratively (empty/no-op and an empty-but-present row triple per collection) — single source of truth for `diff_grammar_conformance_law`/`protocol_walk_law` in
/// `🎹️composer/🦀️.rs`.
#[cfg(all(test, feature = "conversion-cad"))]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<SemioCadDiff> {
    vec![SemioCadDiff::default(), SemioCadDiff { layers: Some(Default::default()), blocks: Some(Default::default()), entities: Some(Default::default()) }]
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
