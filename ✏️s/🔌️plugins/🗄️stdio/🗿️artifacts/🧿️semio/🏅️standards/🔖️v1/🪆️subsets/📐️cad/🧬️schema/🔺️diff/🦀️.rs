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
use crate::standards::v1::subsets::base::schema::triples::{NamedModified, NamedTripleDiff};



use crate::standards::v1::subsets::cad::schema::snapshot::{CadBlock, CadEntity, CadEntityRecord, CadLayer, SemioCadSnapshot};
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;

//#region 🔖️DiffTypes
pub type CadLayersDiff = NamedTripleDiff<String, CadLayerDiff, CadLayer>;
pub type CadBlocksDiff = NamedTripleDiff<String, CadBlockDiff, CadBlock>;
pub type CadEntitiesDiff = NamedTripleDiff<String, CadEntityRecordDiff, CadEntityRecord>;

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

/// 🧮️ Key-identity absorb (not position) — a `d2`-removal of a `d1`-added key annihilates the add;
/// a `d2`-modify of a `d1`-added key patches into the carried payload; everything else composes on
/// the shared key space. Mirrors bcf's `absorb_named` (same canonical cases, B-R7).
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
    fn apply(&self, base: &SemioCadSnapshot) -> protocol::MutationApplyResult<SemioCadSnapshot> {
        let mut next = base.clone();
        if let Some(ld) = &self.layers {
            crate::standards::v1::subsets::base::schema::triples::validate_named_triple(&next.layers, ld, |layer| layer.name.clone(), |layer| layer.name.clone(), ["layers"])?;
            apply_named(&mut next.layers, ld, |l| l.name.clone(), apply_layer);
        }
        if let Some(bd) = &self.blocks {
            crate::standards::v1::subsets::base::schema::triples::validate_named_triple(&next.blocks, bd, |block| block.name.clone(), |block| block.name.clone(), ["blocks"])?;
            apply_named(&mut next.blocks, bd, |b| b.name.clone(), apply_block);
        }
        if let Some(ed) = &self.entities {
            crate::standards::v1::subsets::base::schema::triples::validate_named_triple(&next.entities, ed, |entity| entity.handle.clone(), |entity| entity.handle.clone(), ["entities"])?;
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

    fn between(base: &SemioCadSnapshot, other: &SemioCadSnapshot) -> Self {
        SemioCadDiff {
            layers: between_named(&base.layers, &other.layers, |l| l.name.clone(), between_layer),
            blocks: between_named(&base.blocks, &other.blocks, |b| b.name.clone(), between_block),
            entities: between_named(&base.entities, &other.entities, |e| e.handle.clone(), between_entity_record),
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_layer(base: &CadLayer, other: &CadLayer) -> Option<CadLayerDiff> {
    let color_index = if base.color_index != other.color_index { Some(other.color_index) } else { None };
    let line_type = if base.line_type != other.line_type { Some(other.line_type.clone()) } else { None };
    let visible = if base.visible != other.visible { Some(other.visible) } else { None };
    if color_index.is_none() && line_type.is_none() && visible.is_none() {
        None
    } else {
        Some(CadLayerDiff { color_index, line_type, visible })
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_block(base: &CadBlock, other: &CadBlock) -> Option<CadBlockDiff> {
    let base_point = if base.base_point != other.base_point { Some(other.base_point) } else { None };
    let entities = between_named(&base.entities, &other.entities, |e| e.handle.clone(), between_entity_record);
    if base_point.is_none() && entities.is_none() {
        None
    } else {
        Some(CadBlockDiff { base_point, entities })
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_entity_record(base: &CadEntityRecord, other: &CadEntityRecord) -> Option<CadEntityRecordDiff> {
    let layer = if base.layer != other.layer { Some(other.layer.clone()) } else { None };
    let entity = if base.entity != other.entity { Some(other.entity.clone()) } else { None };
    if layer.is_none() && entity.is_none() {
        None
    } else {
        Some(CadEntityRecordDiff { layer, entity })
    }
}
//#endregion 🔖️DiffAlgebra

//#region 🔖️SetSnapshot
/// 🧩️ Builds the sparse field-by-field diff for a `SetSnapshot` mutation. No
/// `snapshot: Option<SemioCadSnapshot>` full-replace slot -- this IS `SemioCadDiff::between`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_snapshot(base: &SemioCadSnapshot, next: &SemioCadSnapshot) -> SemioCadDiff {
    SemioCadDiff::between(base, next)
}
//#endregion 🔖️SetSnapshot

//#region 🔖️HandcraftedDiffCodec
/// 🎙️ Hand-rolled `protocol::DiffCodec` — no `dsl::DslDiff` derive attempted: `CadEntity` is a
/// data-carrying enum reached through `entities`/`blocks[].entities` (§3a family), and
/// `CadLayersDiff`/`CadBlocksDiff`/`CadEntitiesDiff` are all instances of the generic
/// `NamedTripleDiff<K,D,T>` (§4.4 family, `dsl` has no `DslField` bridge for generic collection
/// wrappers — f6-final-summary.md §4.4). Grammar: bracket-depth-aware split, hex for strings,
/// `[0]`/`[1,x]` for `Option<T>`, single-letter tag prefix for `CadEntity`'s 9-variant `xs:choice`
/// — same primitive set gif/svg/bcf established.
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
/// 🌱 Representative `SemioCadDiff` cases (empty/no-op, a full removed/modified/added sweep both
/// directions across every collection incl. the nested `blocks[].entities`, exercising 7 of the 9
/// `CadEntity` variants) — single source of truth for `diff_grammar_conformance_law`/
/// `protocol_walk_law` in `🎹️composer/🦀️.rs`. Self-contained (does not reach into
/// `#[cfg(test)] mod tests`'s own private `sweep_a`/`sweep_b`, since a private item of a child
/// module is not visible to its parent).
#[cfg(all(test, feature = "conversion-cad"))]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<SemioCadDiff> {
    let a = SemioCadSnapshot {
        schema: crate::standards::v1::subsets::cad::schema::snapshot::STDIO_SEMIOCAD_DOCUMENT_SCHEMA.into(),
        layers: vec![CadLayer { name: "keep".into(), color_index: 1, line_type: "CONTINUOUS".into(), visible: true }, CadLayer { name: "layer-removed".into(), color_index: 2, line_type: "DASHED".into(), visible: false }],
        blocks: vec![CadBlock {
            name: "keep-block".into(),
            base_point: SemioPoint2 { x: 0.0, y: 0.0 },
            entities: vec![CadEntityRecord { handle: "be1".into(), layer: "keep".into(), entity: CadEntity::Line { a: SemioPoint2 { x: 0.0, y: 0.0 }, b: SemioPoint2 { x: 1.0, y: 1.0 } } }],
        }],
        entities: vec![
            CadEntityRecord { handle: "e1".into(), layer: "keep".into(), entity: CadEntity::Circle { center: SemioPoint2 { x: 0.0, y: 0.0 }, radius: 1.0 } },
            CadEntityRecord { handle: "e-removed".into(), layer: "keep".into(), entity: CadEntity::Polyline { vertices: vec![SemioPoint2 { x: 0.0, y: 0.0 }], closed: false } },
        ],
    };
    let b = SemioCadSnapshot {
        schema: crate::standards::v1::subsets::cad::schema::snapshot::STDIO_SEMIOCAD_DOCUMENT_SCHEMA.into(),
        layers: vec![CadLayer { name: "keep".into(), color_index: 9, line_type: "DASHDOT".into(), visible: false }, CadLayer { name: "layer-added".into(), color_index: 4, line_type: "HIDDEN".into(), visible: true }],
        blocks: vec![CadBlock {
            name: "keep-block".into(),
            base_point: SemioPoint2 { x: 5.0, y: 5.0 },
            entities: vec![
                CadEntityRecord { handle: "be1".into(), layer: "layer-added".into(), entity: CadEntity::Arc { center: SemioPoint2 { x: 0.0, y: 0.0 }, radius: 1.0, start_angle: 0.0, end_angle: 90.0 } },
                CadEntityRecord { handle: "be-added".into(), layer: "keep".into(), entity: CadEntity::Dimension { def_point: SemioPoint2 { x: 0.0, y: 0.0 }, text_position: SemioPoint2 { x: 1.0, y: 1.0 }, measurement: 3.3, text: "3.3m".into() } },
            ],
        }],
        entities: vec![
            CadEntityRecord { handle: "e1".into(), layer: "layer-added".into(), entity: CadEntity::Ellipse { center: SemioPoint2 { x: 0.0, y: 0.0 }, major_axis_end: SemioPoint2 { x: 1.0, y: 0.0 }, ratio: 0.5, start_param: 0.0, end_param: 6.28 } },
            CadEntityRecord { handle: "e-added".into(), layer: "keep".into(), entity: CadEntity::Insert { block_name: "keep-block".into(), insertion_point: SemioPoint2 { x: 0.0, y: 0.0 }, scale: SemioPoint2 { x: 1.0, y: 1.0 }, rotation: 0.0 } },
        ],
    };

    vec![SemioCadDiff::default(), <SemioCadDiff as DiffAlgebra<SemioCadSnapshot>>::between(&a, &b), <SemioCadDiff as DiffAlgebra<SemioCadSnapshot>>::between(&b, &a)]
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
