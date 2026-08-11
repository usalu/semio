//! 🔺️ JpgDiff — handcrafted sparse diff. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: replaces the old
//! `JpgDiff{snapshot: Option<JpgSnapshot>}` full-replace template. JFIF header fields are
//! top-level scalars; `frame` is a `Modify`/`Replace` change (mirrors xml's `XmlNodeDiff::Replace`
//! fallback — a decode-status transition (`None`<->`Some`) is a "kind change", everything else is
//! a field-level `Modify`); `quant_tables`/`huffman_tables`/`frame.components` are id-keyed
//! `removed`/`modified`/`added` triples (stable identity, no index-transport needed — see
//! `absorb_id_keyed_*`); `other_segments` is an index-keyed triple (position-transported absorb,
//! mirrors png's `text_chunks`).

use crate::artifacts::jpg::schema::snapshot::{
    JfifDensityUnits, JfifThumbnail, JpgFrameComponent, JpgFrameHeader, JpgHuffmanClass,
    JpgHuffmanTable, JpgQuantTable, JpgSegment,
};
use crate::artifacts::jpg::JpgSnapshot;
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

//#region 🔖️ComponentsDiff
/// 🧩️ Sparse per-field patch for one `JpgFrameComponent`. `id` is the identity, never diffed.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JpgComponentDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub h_sampling: Option<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub v_sampling: Option<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quant_table_id: Option<u8>,
}

impl JpgComponentDiff {
    fn is_empty(&self) -> bool { self == &Self::default() }
    fn apply(&self, base: &JpgFrameComponent) -> JpgFrameComponent {
        JpgFrameComponent {
            id: base.id,
            h_sampling: self.h_sampling.unwrap_or(base.h_sampling),
            v_sampling: self.v_sampling.unwrap_or(base.v_sampling),
            quant_table_id: self.quant_table_id.unwrap_or(base.quant_table_id),
        }
    }
    fn between(a: &JpgFrameComponent, b: &JpgFrameComponent) -> Self {
        Self {
            h_sampling: (a.h_sampling != b.h_sampling).then_some(b.h_sampling),
            v_sampling: (a.v_sampling != b.v_sampling).then_some(b.v_sampling),
            quant_table_id: (a.quant_table_id != b.quant_table_id).then_some(b.quant_table_id),
        }
    }
    fn absorb(&mut self, other: Self) {
        if other.h_sampling.is_some() { self.h_sampling = other.h_sampling; }
        if other.v_sampling.is_some() { self.v_sampling = other.v_sampling; }
        if other.quant_table_id.is_some() { self.quant_table_id = other.quant_table_id; }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JpgComponentModified { pub id: u8, pub diff: JpgComponentDiff }
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JpgComponentAdded { pub index: usize, pub item: JpgFrameComponent }

/// 🔺️ Id-keyed `frame.components` triple.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JpgComponentsDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<u8>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<JpgComponentModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<JpgComponentAdded>,
}
impl JpgComponentsDiff {
    fn is_empty(&self) -> bool { self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty() }
}
//#endregion 🔖️ComponentsDiff

//#region 🔖️FrameDiff
/// 🖼️ Sparse per-field patch for one `JpgFrameHeader`, used when BOTH base and next have
/// `Some(frame)` (see `JpgFrameChange::Modify`).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JpgFrameFieldsDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub precision: Option<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<u16>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<u16>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub components: Option<JpgComponentsDiff>,
}
impl JpgFrameFieldsDiff {
    fn is_empty(&self) -> bool { self == &Self::default() }
}

/// 🌲️ `frame`'s change shape: `Modify` when both base/next have a frame (field-level patch,
/// including the id-keyed `components` triple); `Replace` on a decode-status "kind change"
/// (`None`<->`Some`) — mirrors xml's `XmlNodeDiff::Replace` fallback for exactly this situation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "change", rename_all = "camelCase")]
pub enum JpgFrameChange {
    Modify(JpgFrameFieldsDiff),
    Replace { frame: Option<JpgFrameHeader> },
}
//#endregion 🔖️FrameDiff

//#region 🔖️QuantTablesDiff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JpgQuantTableDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub precision: Option<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none", with = "opt_quant_values")]
    pub values: Option<[u16; 64]>,
}

/// 🧮️ `Option<[u16; 64]>` counterpart of `snapshot::quant_values` (see its doc — serde's manual
/// array impls stop at 32 elements).
mod opt_quant_values {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    pub fn serialize<S: Serializer>(v: &Option<[u16; 64]>, s: S) -> Result<S::Ok, S::Error> {
        v.map(|a| a.to_vec()).serialize(s)
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Option<[u16; 64]>, D::Error> {
        let v: Option<Vec<u16>> = Option::deserialize(d)?;
        match v {
            None => Ok(None),
            Some(vec) => <[u16; 64]>::try_from(vec).map(Some).map_err(|v: Vec<u16>| serde::de::Error::custom(format!("expected 64 values, got {}", v.len()))),
        }
    }
}
impl JpgQuantTableDiff {
    fn is_empty(&self) -> bool { self == &Self::default() }
    fn apply(&self, base: &JpgQuantTable) -> JpgQuantTable {
        JpgQuantTable {
            id: base.id,
            precision: self.precision.unwrap_or(base.precision),
            values: self.values.unwrap_or(base.values),
        }
    }
    fn between(a: &JpgQuantTable, b: &JpgQuantTable) -> Self {
        Self {
            precision: (a.precision != b.precision).then_some(b.precision),
            values: (a.values != b.values).then_some(b.values),
        }
    }
    fn absorb(&mut self, other: Self) {
        if other.precision.is_some() { self.precision = other.precision; }
        if other.values.is_some() { self.values = other.values; }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JpgQuantTableModified { pub id: u8, pub diff: JpgQuantTableDiff }
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JpgQuantTableAdded { pub index: usize, pub item: JpgQuantTable }

/// 🔺️ Id-keyed `quant_tables` (DQT) triple.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JpgQuantTablesDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<u8>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<JpgQuantTableModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<JpgQuantTableAdded>,
}
impl JpgQuantTablesDiff {
    fn is_empty(&self) -> bool { self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty() }
}
//#endregion 🔖️QuantTablesDiff

//#region 🔖️HuffmanTablesDiff
/// 🔑️ Compound identity for `huffman_tables` — DC id=0 and AC id=0 are different tables.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JpgHuffmanTableKey {
    pub class: JpgHuffmanClass,
    pub id: u8,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JpgHuffmanTableDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bits: Option<[u8; 16]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<u8>>,
}
impl JpgHuffmanTableDiff {
    fn is_empty(&self) -> bool { self == &Self::default() }
    fn apply(&self, base: &JpgHuffmanTable) -> JpgHuffmanTable {
        JpgHuffmanTable {
            id: base.id,
            class: base.class,
            bits: self.bits.unwrap_or(base.bits),
            values: self.values.clone().unwrap_or_else(|| base.values.clone()),
        }
    }
    fn between(a: &JpgHuffmanTable, b: &JpgHuffmanTable) -> Self {
        Self {
            bits: (a.bits != b.bits).then_some(b.bits),
            values: (a.values != b.values).then(|| b.values.clone()),
        }
    }
    fn absorb(&mut self, other: Self) {
        if other.bits.is_some() { self.bits = other.bits; }
        if other.values.is_some() { self.values = other.values; }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JpgHuffmanTableModified { pub key: JpgHuffmanTableKey, pub diff: JpgHuffmanTableDiff }
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JpgHuffmanTableAdded { pub index: usize, pub item: JpgHuffmanTable }

/// 🔺️ `(class, id)`-keyed `huffman_tables` (DHT) triple.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JpgHuffmanTablesDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<JpgHuffmanTableKey>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<JpgHuffmanTableModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<JpgHuffmanTableAdded>,
}
impl JpgHuffmanTablesDiff {
    fn is_empty(&self) -> bool { self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty() }
}
fn huffman_key(t: &JpgHuffmanTable) -> JpgHuffmanTableKey { JpgHuffmanTableKey { class: t.class, id: t.id } }
//#endregion 🔖️HuffmanTablesDiff

//#region 🔖️OtherSegmentsDiff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JpgSegmentDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<u8>>,
}
impl JpgSegmentDiff {
    fn is_empty(&self) -> bool { self == &Self::default() }
    fn apply(&self, base: &JpgSegment) -> JpgSegment {
        JpgSegment {
            marker: self.marker.unwrap_or(base.marker),
            data: self.data.clone().unwrap_or_else(|| base.data.clone()),
        }
    }
    fn between(a: &JpgSegment, b: &JpgSegment) -> Self {
        Self {
            marker: (a.marker != b.marker).then_some(b.marker),
            data: (a.data != b.data).then(|| b.data.clone()),
        }
    }
    fn absorb(&mut self, other: Self) {
        if other.marker.is_some() { self.marker = other.marker; }
        if other.data.is_some() { self.data = other.data; }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JpgSegmentModified { pub index: usize, pub diff: JpgSegmentDiff }
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JpgSegmentAdded { pub index: usize, pub item: JpgSegment }

/// 🔺️ Index-keyed `other_segments` triple (position-transported absorb — duplicate markers are
/// legal, so identity is position, not the marker byte).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JpgOtherSegmentsDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<JpgSegmentModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<JpgSegmentAdded>,
}
impl JpgOtherSegmentsDiff {
    fn is_empty(&self) -> bool { self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty() }
}
//#endregion 🔖️OtherSegmentsDiff

//#region 🔖️IndexTransport
// 🧮 Base-free index transport for `other_segments`' absorb — ported verbatim from png's
// `simulate_slots`/`base_len_hint`/`absorb_text_chunks` shape (position-keyed, field-aware
// modified payload), retargeted to `JpgSegment`/`JpgSegmentDiff`.
#[derive(Clone, Copy, Debug)]
enum Slot { Base(usize), Added(usize) }

fn simulate_slots(len: usize, removed: &[usize], added_indices: &[usize]) -> Vec<Slot> {
    let mut slots: Vec<Slot> = (0..len).map(Slot::Base).collect();
    let mut removed_desc = removed.to_vec();
    removed_desc.sort_unstable_by(|a, b| b.cmp(a));
    removed_desc.dedup();
    for r in removed_desc {
        if r < slots.len() { slots.remove(r); }
    }
    let mut order: Vec<usize> = (0..added_indices.len()).collect();
    order.sort_by_key(|&i| added_indices[i]);
    for i in order {
        let at = added_indices[i].min(slots.len());
        slots.insert(at, Slot::Added(i));
    }
    slots
}

fn base_len_hint(removed: &[usize], modified_indices: impl Iterator<Item = usize>, added_indices: impl Iterator<Item = usize>) -> usize {
    removed.iter().copied().chain(modified_indices).chain(added_indices).max().map(|m| m + 1).unwrap_or(0)
}

fn absorb_other_segments(d1: JpgOtherSegmentsDiff, d2: JpgOtherSegmentsDiff) -> JpgOtherSegmentsDiff {
    let d1_added_indices: Vec<usize> = d1.added.iter().map(|a| a.index).collect();
    let removed_count = { let mut r = d1.removed.clone(); r.sort_unstable(); r.dedup(); r.len() };
    let needed_mid_len = d2.removed.iter().copied().chain(d2.modified.iter().map(|m| m.index)).max().map(|m| m + 1).unwrap_or(0);
    let base_len = base_len_hint(&d1.removed, d1.modified.iter().map(|m| m.index), d1_added_indices.iter().copied())
        .max((needed_mid_len + removed_count).saturating_sub(d1.added.len()));
    let mid_slots = simulate_slots(base_len, &d1.removed, &d1_added_indices);

    let mut final_removed: Vec<usize> = d1.removed;
    let mut modified_map: BTreeMap<usize, JpgSegmentDiff> = d1.modified.into_iter().map(|m| (m.index, m.diff)).collect();
    let mut added_alive: Vec<Option<JpgSegmentAdded>> = d1.added.into_iter().map(Some).collect();

    for mid_idx in &d2.removed {
        match mid_slots.get(*mid_idx) {
            Some(Slot::Base(b)) => { final_removed.push(*b); modified_map.remove(b); }
            Some(Slot::Added(ai)) => { added_alive[*ai] = None; }
            None => {}
        }
    }
    for m2 in &d2.modified {
        match mid_slots.get(m2.index) {
            Some(Slot::Base(b)) => { modified_map.entry(*b).or_default().absorb(m2.diff.clone()); }
            Some(Slot::Added(ai)) => {
                if let Some(a) = added_alive[*ai].as_mut() { a.item = m2.diff.apply(&a.item); }
            }
            None => {}
        }
    }

    final_removed.sort_unstable();
    final_removed.dedup();
    for r in &final_removed { modified_map.remove(r); }
    let mut final_modified: Vec<JpgSegmentModified> = modified_map
        .into_iter()
        .filter(|(_, d)| !d.is_empty())
        .map(|(index, diff)| JpgSegmentModified { index, diff })
        .collect();
    final_modified.sort_by_key(|m| m.index);

    let alive_mid_positions: Vec<usize> = mid_slots.iter().enumerate()
        .filter_map(|(pos, slot)| match slot { Slot::Added(ai) if added_alive[*ai].is_some() => Some(pos), _ => None })
        .collect();
    let d2_added_indices: Vec<usize> = d2.added.iter().map(|a| a.index).collect();
    let mid_len = d2.removed.iter().copied()
        .chain(d2.modified.iter().map(|m| m.index))
        .chain(alive_mid_positions.iter().copied())
        .chain(d2_added_indices.iter().copied())
        .max().map(|m| m + 1).unwrap_or(0);
    let after_slots = simulate_slots(mid_len, &d2.removed, &d2_added_indices);
    let mut mid_to_after: HashMap<usize, usize> = HashMap::new();
    for (pos, slot) in after_slots.iter().enumerate() {
        if let Slot::Base(m) = slot { mid_to_after.insert(*m, pos); }
    }

    let mut final_added: Vec<JpgSegmentAdded> = Vec::new();
    for (ai, alive) in added_alive.into_iter().enumerate() {
        if let Some(added) = alive {
            let mid_pos = mid_slots.iter().position(|s| matches!(s, Slot::Added(idx) if *idx == ai))
                .expect("added_alive index always has a corresponding mid slot");
            if let Some(after_pos) = mid_to_after.get(&mid_pos) {
                final_added.push(JpgSegmentAdded { index: *after_pos, item: added.item });
            }
        }
    }
    for a2 in d2.added { final_added.push(a2); }
    final_added.sort_by_key(|a| a.index);

    JpgOtherSegmentsDiff { removed: final_removed, modified: final_modified, added: final_added }
}

fn absorb_other_segments_opt(base: &mut Option<JpgOtherSegmentsDiff>, other: Option<JpgOtherSegmentsDiff>) {
    match (base.take(), other) {
        (None, o) => *base = o,
        (Some(b), None) => *base = Some(b),
        (Some(b), Some(o)) => *base = Some(absorb_other_segments(b, o)),
    }
}

fn apply_other_segments(base: &[JpgSegment], d: &JpgOtherSegmentsDiff) -> Vec<JpgSegment> {
    let mut items = base.to_vec();
    for m in &d.modified {
        if let Some(it) = items.get_mut(m.index) { *it = m.diff.apply(it); }
    }
    let mut removed_desc = d.removed.clone();
    removed_desc.sort_unstable_by(|a, b| b.cmp(a));
    removed_desc.dedup();
    for idx in removed_desc { if idx < items.len() { items.remove(idx); } }
    let mut adds = d.added.clone();
    adds.sort_by_key(|a| a.index);
    for a in adds { let at = a.index.min(items.len()); items.insert(at, a.item); }
    items
}

fn between_other_segments(a: &[JpgSegment], b: &[JpgSegment]) -> Option<JpgOtherSegmentsDiff> {
    let min = a.len().min(b.len());
    let mut modified = Vec::new();
    for i in 0..min {
        if a[i] != b[i] {
            let d = JpgSegmentDiff::between(&a[i], &b[i]);
            if !d.is_empty() { modified.push(JpgSegmentModified { index: i, diff: d }); }
        }
    }
    let removed: Vec<usize> = (min..a.len()).collect();
    let added: Vec<JpgSegmentAdded> = (min..b.len()).map(|i| JpgSegmentAdded { index: i, item: b[i].clone() }).collect();
    if removed.is_empty() && modified.is_empty() && added.is_empty() { None } else { Some(JpgOtherSegmentsDiff { removed, modified, added }) }
}
//#endregion 🔖️IndexTransport

//#region 🔖️IdKeyedTransport
// 🧮 Stable-key absorb for `quant_tables`/`huffman_tables`/`frame.components` — id/key identity
// doesn't shift with position, so (unlike `other_segments`) NO index-transport simulation is
// needed at all; mirrors zip's `absorb_entries` (name-keyed, no-rename-tracking-needed case:
// jpg has no id-renaming mutation, so the rename map zip carries is simply omitted here). `index`
// bookkeeping on surviving `added` entries uses the same documented best-effort shift zip does:
// exact when d2's genuine (non-annihilating) removals sit before the add.

fn absorb_quant_tables(mut d1: JpgQuantTablesDiff, d2: JpgQuantTablesDiff) -> JpgQuantTablesDiff {
    let added_ids: std::collections::HashSet<u8> = d1.added.iter().map(|a| a.item.id).collect();
    let mut removed_shift = 0usize;
    for id in &d2.removed {
        if added_ids.contains(id) {
            d1.added.retain(|a| a.item.id != *id);
        } else {
            removed_shift += 1;
            if !d1.removed.contains(id) { d1.removed.push(*id); }
            d1.modified.retain(|m| m.id != *id);
        }
    }
    let mut merged_added: Vec<JpgQuantTableAdded> = d1.added.into_iter()
        .map(|mut a| { a.index = a.index.saturating_sub(removed_shift); a })
        .collect();
    let mut merged_modified = d1.modified;
    for dm in &d2.modified {
        if let Some(a) = merged_added.iter_mut().find(|a| a.item.id == dm.id) {
            a.item = dm.diff.apply(&a.item);
        } else if d1.removed.contains(&dm.id) {
            continue;
        } else if let Some(existing) = merged_modified.iter_mut().find(|m| m.id == dm.id) {
            existing.diff.absorb(dm.diff.clone());
        } else {
            merged_modified.push(JpgQuantTableModified { id: dm.id, diff: dm.diff.clone() });
        }
    }
    merged_added.extend(d2.added);
    JpgQuantTablesDiff { removed: d1.removed, modified: merged_modified, added: merged_added }
}

fn absorb_huffman_tables(mut d1: JpgHuffmanTablesDiff, d2: JpgHuffmanTablesDiff) -> JpgHuffmanTablesDiff {
    let added_keys: std::collections::HashSet<JpgHuffmanTableKey> = d1.added.iter().map(|a| huffman_key(&a.item)).collect();
    let mut removed_shift = 0usize;
    for key in &d2.removed {
        if added_keys.contains(key) {
            d1.added.retain(|a| huffman_key(&a.item) != *key);
        } else {
            removed_shift += 1;
            if !d1.removed.contains(key) { d1.removed.push(*key); }
            d1.modified.retain(|m| m.key != *key);
        }
    }
    let mut merged_added: Vec<JpgHuffmanTableAdded> = d1.added.into_iter()
        .map(|mut a| { a.index = a.index.saturating_sub(removed_shift); a })
        .collect();
    let mut merged_modified = d1.modified;
    for dm in &d2.modified {
        if let Some(a) = merged_added.iter_mut().find(|a| huffman_key(&a.item) == dm.key) {
            a.item = dm.diff.apply(&a.item);
        } else if d1.removed.contains(&dm.key) {
            continue;
        } else if let Some(existing) = merged_modified.iter_mut().find(|m| m.key == dm.key) {
            existing.diff.absorb(dm.diff.clone());
        } else {
            merged_modified.push(JpgHuffmanTableModified { key: dm.key, diff: dm.diff.clone() });
        }
    }
    merged_added.extend(d2.added);
    JpgHuffmanTablesDiff { removed: d1.removed, modified: merged_modified, added: merged_added }
}

fn absorb_components(mut d1: JpgComponentsDiff, d2: JpgComponentsDiff) -> JpgComponentsDiff {
    let added_ids: std::collections::HashSet<u8> = d1.added.iter().map(|a| a.item.id).collect();
    let mut removed_shift = 0usize;
    for id in &d2.removed {
        if added_ids.contains(id) {
            d1.added.retain(|a| a.item.id != *id);
        } else {
            removed_shift += 1;
            if !d1.removed.contains(id) { d1.removed.push(*id); }
            d1.modified.retain(|m| m.id != *id);
        }
    }
    let mut merged_added: Vec<JpgComponentAdded> = d1.added.into_iter()
        .map(|mut a| { a.index = a.index.saturating_sub(removed_shift); a })
        .collect();
    let mut merged_modified = d1.modified;
    for dm in &d2.modified {
        if let Some(a) = merged_added.iter_mut().find(|a| a.item.id == dm.id) {
            a.item = dm.diff.apply(&a.item);
        } else if d1.removed.contains(&dm.id) {
            continue;
        } else if let Some(existing) = merged_modified.iter_mut().find(|m| m.id == dm.id) {
            existing.diff.absorb(dm.diff.clone());
        } else {
            merged_modified.push(JpgComponentModified { id: dm.id, diff: dm.diff.clone() });
        }
    }
    merged_added.extend(d2.added);
    JpgComponentsDiff { removed: d1.removed, modified: merged_modified, added: merged_added }
}

fn between_components(a: &[JpgFrameComponent], b: &[JpgFrameComponent]) -> JpgComponentsDiff {
    let mut removed = Vec::new();
    let mut modified = Vec::new();
    for ac in a {
        match b.iter().find(|bc| bc.id == ac.id) {
            Some(bc) => {
                let d = JpgComponentDiff::between(ac, bc);
                if !d.is_empty() { modified.push(JpgComponentModified { id: ac.id, diff: d }); }
            }
            None => removed.push(ac.id),
        }
    }
    let added: Vec<JpgComponentAdded> = b.iter().enumerate()
        .filter(|(_, bc)| !a.iter().any(|ac| ac.id == bc.id))
        .map(|(index, bc)| JpgComponentAdded { index, item: *bc })
        .collect();
    JpgComponentsDiff { removed, modified, added }
}

fn apply_components(base: &[JpgFrameComponent], d: &JpgComponentsDiff) -> Vec<JpgFrameComponent> {
    let mut items: Vec<JpgFrameComponent> = base.iter().filter(|c| !d.removed.contains(&c.id)).copied().collect();
    for m in &d.modified {
        if let Some(item) = items.iter_mut().find(|c| c.id == m.id) { *item = m.diff.apply(item); }
    }
    let mut adds = d.added.clone();
    adds.sort_by_key(|a| a.index);
    for a in adds { let at = a.index.min(items.len()); items.insert(at, a.item); }
    items
}

fn between_quant_tables(a: &[JpgQuantTable], b: &[JpgQuantTable]) -> JpgQuantTablesDiff {
    let mut removed = Vec::new();
    let mut modified = Vec::new();
    for at_ in a {
        match b.iter().find(|bt| bt.id == at_.id) {
            Some(bt) => {
                let d = JpgQuantTableDiff::between(at_, bt);
                if !d.is_empty() { modified.push(JpgQuantTableModified { id: at_.id, diff: d }); }
            }
            None => removed.push(at_.id),
        }
    }
    let added: Vec<JpgQuantTableAdded> = b.iter().enumerate()
        .filter(|(_, bt)| !a.iter().any(|at_| at_.id == bt.id))
        .map(|(index, bt)| JpgQuantTableAdded { index, item: bt.clone() })
        .collect();
    JpgQuantTablesDiff { removed, modified, added }
}

fn apply_quant_tables(base: &[JpgQuantTable], d: &JpgQuantTablesDiff) -> Vec<JpgQuantTable> {
    let mut items: Vec<JpgQuantTable> = base.iter().filter(|t| !d.removed.contains(&t.id)).cloned().collect();
    for m in &d.modified {
        if let Some(item) = items.iter_mut().find(|t| t.id == m.id) { *item = m.diff.apply(item); }
    }
    let mut adds = d.added.clone();
    adds.sort_by_key(|a| a.index);
    for a in adds { let at = a.index.min(items.len()); items.insert(at, a.item); }
    items
}

fn between_huffman_tables(a: &[JpgHuffmanTable], b: &[JpgHuffmanTable]) -> JpgHuffmanTablesDiff {
    let mut removed = Vec::new();
    let mut modified = Vec::new();
    for at_ in a {
        let k = huffman_key(at_);
        match b.iter().find(|bt| huffman_key(bt) == k) {
            Some(bt) => {
                let d = JpgHuffmanTableDiff::between(at_, bt);
                if !d.is_empty() { modified.push(JpgHuffmanTableModified { key: k, diff: d }); }
            }
            None => removed.push(k),
        }
    }
    let added: Vec<JpgHuffmanTableAdded> = b.iter().enumerate()
        .filter(|(_, bt)| !a.iter().any(|at_| huffman_key(at_) == huffman_key(bt)))
        .map(|(index, bt)| JpgHuffmanTableAdded { index, item: bt.clone() })
        .collect();
    JpgHuffmanTablesDiff { removed, modified, added }
}

fn apply_huffman_tables(base: &[JpgHuffmanTable], d: &JpgHuffmanTablesDiff) -> Vec<JpgHuffmanTable> {
    let mut items: Vec<JpgHuffmanTable> = base.iter().filter(|t| !d.removed.contains(&huffman_key(t))).cloned().collect();
    for m in &d.modified {
        if let Some(item) = items.iter_mut().find(|t| huffman_key(t) == m.key) { *item = m.diff.apply(item); }
    }
    let mut adds = d.added.clone();
    adds.sort_by_key(|a| a.index);
    for a in adds { let at = a.index.min(items.len()); items.insert(at, a.item); }
    items
}
//#endregion 🔖️IdKeyedTransport

//#region 🔖️FrameHelpers
fn apply_frame(base: &Option<JpgFrameHeader>, change: &JpgFrameChange) -> Option<JpgFrameHeader> {
    match change {
        JpgFrameChange::Replace { frame } => frame.clone(),
        JpgFrameChange::Modify(fd) => {
            let mut f = base.clone().unwrap_or(JpgFrameHeader { precision: 8, width: 0, height: 0, components: Vec::new() });
            if let Some(p) = fd.precision { f.precision = p; }
            if let Some(w) = fd.width { f.width = w; }
            if let Some(h) = fd.height { f.height = h; }
            if let Some(cd) = &fd.components { f.components = apply_components(&f.components, cd); }
            Some(f)
        }
    }
}

fn between_frame(a: &Option<JpgFrameHeader>, b: &Option<JpgFrameHeader>) -> Option<JpgFrameChange> {
    if a == b { return None; }
    match (a, b) {
        (Some(af), Some(bf)) => {
            let mut fd = JpgFrameFieldsDiff::default();
            if af.precision != bf.precision { fd.precision = Some(bf.precision); }
            if af.width != bf.width { fd.width = Some(bf.width); }
            if af.height != bf.height { fd.height = Some(bf.height); }
            let cd = between_components(&af.components, &bf.components);
            if !cd.is_empty() { fd.components = Some(cd); }
            Some(JpgFrameChange::Modify(fd))
        }
        _ => Some(JpgFrameChange::Replace { frame: b.clone() }),
    }
}

fn absorb_frame(base: &mut Option<JpgFrameChange>, other: Option<JpgFrameChange>) {
    let Some(other) = other else { return };
    match other {
        JpgFrameChange::Replace { .. } => { *base = Some(other); }
        JpgFrameChange::Modify(fd2) => match base.take() {
            None => { *base = Some(JpgFrameChange::Modify(fd2)); }
            Some(JpgFrameChange::Replace { frame }) => {
                // 🩹 A d1 `Replace` already committed the whole new frame value; folding a d2
                // field-patch means applying it directly to that carried value (documented
                // patch-into-replace, the `Replace`-shape analogue of "patch into added").
                let patched = frame.map(|mut f| {
                    if let Some(p) = fd2.precision { f.precision = p; }
                    if let Some(w) = fd2.width { f.width = w; }
                    if let Some(h) = fd2.height { f.height = h; }
                    if let Some(cd) = &fd2.components { f.components = apply_components(&f.components, cd); }
                    f
                });
                *base = Some(JpgFrameChange::Replace { frame: patched });
            }
            Some(JpgFrameChange::Modify(mut fd1)) => {
                if fd2.precision.is_some() { fd1.precision = fd2.precision; }
                if fd2.width.is_some() { fd1.width = fd2.width; }
                if fd2.height.is_some() { fd1.height = fd2.height; }
                if let Some(cd2) = fd2.components {
                    fd1.components = Some(match fd1.components.take() {
                        Some(cd1) => absorb_components(cd1, cd2),
                        None => cd2,
                    });
                }
                *base = Some(JpgFrameChange::Modify(fd1));
            }
        },
    }
}
//#endregion 🔖️FrameHelpers

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.jpg`. No `snapshot: Option<JpgSnapshot>` full-replace slot — even
/// `SetSnapshot`'s diff is `JpgDiff::between(base, next)`.
/// 🧪️ F6 CONFIRMED HAND-ROLL: `#[derive(dsl::DslDiff)]` on this struct fails to compile with TWO
/// independent, simultaneous reasons (both captured verbatim by actually adding the derive this
/// session, real `cargo check` output, see `f6-jpg-report.md`): (1) `frame: Option<JpgFrameChange>`
/// — `JpgFrameChange` is a genuine data-carrying enum (`Modify(JpgFrameFieldsDiff)` /
/// `Replace{frame}`), and `DslField` has no impl for it (only `DslRecord`-derived structs and
/// `DslScalar`-derived UNIT-only enums implement `DslField`):
/// ```text
/// error[E0277]: the trait bound `JpgFrameChange: DslField` is not satisfied
///   --> .../🔺️diff/🦀️component.rs:738:23   (pub frame: Option<JpgFrameChange>)
/// ```
/// (2) `re_encode_quality`/`jfif_thumbnail`/`restart_interval` are tri-state `Option<Option<T>>`
/// fields — same blocker as `GifDiff`/`SvgDiff` (no `impl<T: DslField> DslField for Option<T>`
/// anywhere in `dsl`):
/// ```text
/// error[E0277]: the trait bound `std::option::Option<u8>: DslField` is not satisfied
///   --> .../🔺️diff/🦀️component.rs:720:35   (pub re_encode_quality: Option<Option<u8>>)
/// error[E0277]: the trait bound `Option<JfifThumbnail>: DslField` is not satisfied
///   --> .../🔺️diff/🦀️component.rs:735:32   (pub jfif_thumbnail: Option<Option<JfifThumbnail>>)
/// error[E0277]: the trait bound `std::option::Option<u16>: DslField` is not satisfied
///   --> .../🔺️diff/🦀️component.rs:753:34   (pub restart_interval: Option<Option<u16>>)
/// ```
/// `DiffCodec` is hand-rolled below (`#region 🔖️HandcraftedDiffCodec`).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.jpg.diff")]
pub struct JpgDiff {
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pixels: Option<Vec<u8>>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub re_encode_quality: Option<Option<u8>>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jfif_version: Option<(u8, u8)>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jfif_density_units: Option<JfifDensityUnits>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jfif_x_density: Option<u16>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jfif_y_density: Option<u16>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jfif_thumbnail: Option<Option<JfifThumbnail>>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frame: Option<JpgFrameChange>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sof_marker: Option<u8>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arithmetic: Option<bool>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quant_tables: Option<JpgQuantTablesDiff>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub huffman_tables: Option<JpgHuffmanTablesDiff>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restart_interval: Option<Option<u16>>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub other_segments: Option<JpgOtherSegmentsDiff>,
}

impl MutationDiff<JpgSnapshot> for JpgDiff {
    fn apply(&self, base: &JpgSnapshot) -> JpgSnapshot {
        let mut next = base.clone();
        if let Some(v) = self.width { next.width = v; }
        if let Some(v) = self.height { next.height = v; }
        if let Some(v) = &self.pixels { next.pixels = v.clone(); }
        if let Some(v) = &self.re_encode_quality { next.re_encode_quality = *v; }
        if let Some(v) = self.jfif_version { next.jfif_version = v; }
        if let Some(v) = self.jfif_density_units { next.jfif_density_units = v; }
        if let Some(v) = self.jfif_x_density { next.jfif_x_density = v; }
        if let Some(v) = self.jfif_y_density { next.jfif_y_density = v; }
        if let Some(v) = &self.jfif_thumbnail { next.jfif_thumbnail = v.clone(); }
        if let Some(change) = &self.frame { next.frame = apply_frame(&next.frame, change); }
        if let Some(v) = self.sof_marker { next.sof_marker = v; }
        if let Some(v) = self.arithmetic { next.arithmetic = v; }
        if let Some(qd) = &self.quant_tables { next.quant_tables = apply_quant_tables(&next.quant_tables, qd); }
        if let Some(hd) = &self.huffman_tables { next.huffman_tables = apply_huffman_tables(&next.huffman_tables, hd); }
        if let Some(v) = &self.restart_interval { next.restart_interval = *v; }
        if let Some(od) = &self.other_segments { next.other_segments = apply_other_segments(&next.other_segments, od); }
        next
    }

    /// ➕️ Structural, total, base-free sequential-coalesce (`## Absorb` contract). Scalars
    /// (incl. every tri-state): LWW. `frame`: `absorb_frame`. `quant_tables`/`huffman_tables`:
    /// stable-key merge, no index-transport. `other_segments`: position-transported merge.
    fn absorb(&mut self, other: Self) {
        if other.width.is_some() { self.width = other.width; }
        if other.height.is_some() { self.height = other.height; }
        if other.pixels.is_some() { self.pixels = other.pixels; }
        if other.re_encode_quality.is_some() { self.re_encode_quality = other.re_encode_quality; }
        if other.jfif_version.is_some() { self.jfif_version = other.jfif_version; }
        if other.jfif_density_units.is_some() { self.jfif_density_units = other.jfif_density_units; }
        if other.jfif_x_density.is_some() { self.jfif_x_density = other.jfif_x_density; }
        if other.jfif_y_density.is_some() { self.jfif_y_density = other.jfif_y_density; }
        if other.jfif_thumbnail.is_some() { self.jfif_thumbnail = other.jfif_thumbnail; }
        absorb_frame(&mut self.frame, other.frame);
        if other.sof_marker.is_some() { self.sof_marker = other.sof_marker; }
        if other.arithmetic.is_some() { self.arithmetic = other.arithmetic; }
        self.quant_tables = match (self.quant_tables.take(), other.quant_tables) {
            (None, o) => o,
            (Some(b), None) => Some(b),
            (Some(b), Some(o)) => Some(absorb_quant_tables(b, o)),
        };
        self.huffman_tables = match (self.huffman_tables.take(), other.huffman_tables) {
            (None, o) => o,
            (Some(b), None) => Some(b),
            (Some(b), Some(o)) => Some(absorb_huffman_tables(b, o)),
        };
        if other.restart_interval.is_some() { self.restart_interval = other.restart_interval; }
        absorb_other_segments_opt(&mut self.other_segments, other.other_segments);
    }
}

impl DiffAlgebra<JpgSnapshot> for JpgDiff {
    /// 🔁️ Diff-level undo, derived generically (correct by construction) exactly like zip's/
    /// png's: the state delta from `self.apply(base)` back to `base`.
    fn inverse(&self, base: &JpgSnapshot) -> Self {
        let mutated = self.apply(base);
        Self::between(&mutated, base)
    }

    /// 🧭️ State delta (compose `GetXDiff`): id-keyed matching for `quant_tables`/
    /// `huffman_tables`/`frame.components`, position-pairwise `0..min(len)` for
    /// `other_segments`, tri-state comparison for every optional scalar.
    fn between(base: &JpgSnapshot, other: &JpgSnapshot) -> Self {
        let qd = between_quant_tables(&base.quant_tables, &other.quant_tables);
        let hd = between_huffman_tables(&base.huffman_tables, &other.huffman_tables);
        Self {
            width: (base.width != other.width).then_some(other.width),
            height: (base.height != other.height).then_some(other.height),
            pixels: (base.pixels != other.pixels).then(|| other.pixels.clone()),
            re_encode_quality: (base.re_encode_quality != other.re_encode_quality).then_some(other.re_encode_quality),
            jfif_version: (base.jfif_version != other.jfif_version).then_some(other.jfif_version),
            jfif_density_units: (base.jfif_density_units != other.jfif_density_units).then_some(other.jfif_density_units),
            jfif_x_density: (base.jfif_x_density != other.jfif_x_density).then_some(other.jfif_x_density),
            jfif_y_density: (base.jfif_y_density != other.jfif_y_density).then_some(other.jfif_y_density),
            jfif_thumbnail: (base.jfif_thumbnail != other.jfif_thumbnail).then(|| other.jfif_thumbnail.clone()),
            frame: between_frame(&base.frame, &other.frame),
            sof_marker: (base.sof_marker != other.sof_marker).then_some(other.sof_marker),
            arithmetic: (base.arithmetic != other.arithmetic).then_some(other.arithmetic),
            quant_tables: (!qd.is_empty()).then_some(qd),
            huffman_tables: (!hd.is_empty()).then_some(hd),
            restart_interval: (base.restart_interval != other.restart_interval).then_some(other.restart_interval),
            other_segments: between_other_segments(&base.other_segments, &other.other_segments),
        }
    }

    fn is_empty(&self) -> bool { self == &Self::default() }
}

/// 🧩 Builds a set-snapshot diff (sparse field-by-field delta, never a full-replace slot).
pub fn diff_set_snapshot(base: &JpgSnapshot, next: &JpgSnapshot) -> JpgDiff {
    JpgDiff::between(base, next)
}
//#endregion 🔖️Diff

//#region 🔖️MutationDiffBuilders
pub fn diff_set_jfif_header(base: &JpgSnapshot, version: (u8, u8), density_units: JfifDensityUnits, x_density: u16, y_density: u16, thumbnail: Option<JfifThumbnail>) -> JpgDiff {
    JpgDiff {
        jfif_version: (base.jfif_version != version).then_some(version),
        jfif_density_units: (base.jfif_density_units != density_units).then_some(density_units),
        jfif_x_density: (base.jfif_x_density != x_density).then_some(x_density),
        jfif_y_density: (base.jfif_y_density != y_density).then_some(y_density),
        jfif_thumbnail: (base.jfif_thumbnail != thumbnail).then_some(thumbnail),
        ..Default::default()
    }
}

pub fn diff_set_quant_table(base: &JpgSnapshot, table: JpgQuantTable) -> JpgDiff {
    let d = match base.quant_tables.iter().position(|t| t.id == table.id) {
        Some(_) => {
            let existing = base.quant_tables.iter().find(|t| t.id == table.id).unwrap();
            let fd = JpgQuantTableDiff::between(existing, &table);
            if fd.is_empty() { JpgQuantTablesDiff::default() } else { JpgQuantTablesDiff { removed: vec![], modified: vec![JpgQuantTableModified { id: table.id, diff: fd }], added: vec![] } }
        }
        None => JpgQuantTablesDiff { removed: vec![], modified: vec![], added: vec![JpgQuantTableAdded { index: base.quant_tables.len(), item: table }] },
    };
    JpgDiff { quant_tables: (!d.is_empty()).then_some(d), ..Default::default() }
}

pub fn diff_remove_quant_table(base: &JpgSnapshot, id: u8) -> JpgDiff {
    if !base.quant_tables.iter().any(|t| t.id == id) { return JpgDiff::default(); }
    JpgDiff { quant_tables: Some(JpgQuantTablesDiff { removed: vec![id], modified: vec![], added: vec![] }), ..Default::default() }
}

pub fn diff_set_huffman_table(base: &JpgSnapshot, table: JpgHuffmanTable) -> JpgDiff {
    let key = huffman_key(&table);
    let d = match base.huffman_tables.iter().find(|t| huffman_key(t) == key) {
        Some(existing) => {
            let fd = JpgHuffmanTableDiff::between(existing, &table);
            if fd.is_empty() { JpgHuffmanTablesDiff::default() } else { JpgHuffmanTablesDiff { removed: vec![], modified: vec![JpgHuffmanTableModified { key, diff: fd }], added: vec![] } }
        }
        None => JpgHuffmanTablesDiff { removed: vec![], modified: vec![], added: vec![JpgHuffmanTableAdded { index: base.huffman_tables.len(), item: table }] },
    };
    JpgDiff { huffman_tables: (!d.is_empty()).then_some(d), ..Default::default() }
}

pub fn diff_remove_huffman_table(base: &JpgSnapshot, key: JpgHuffmanTableKey) -> JpgDiff {
    if !base.huffman_tables.iter().any(|t| huffman_key(t) == key) { return JpgDiff::default(); }
    JpgDiff { huffman_tables: Some(JpgHuffmanTablesDiff { removed: vec![key], modified: vec![], added: vec![] }), ..Default::default() }
}

pub fn diff_set_restart_interval(base: &JpgSnapshot, restart_interval: Option<u16>) -> JpgDiff {
    JpgDiff { restart_interval: (base.restart_interval != restart_interval).then_some(restart_interval), ..Default::default() }
}

pub fn diff_insert_other_segment(base: &JpgSnapshot, index: usize, segment: JpgSegment) -> JpgDiff {
    let at = index.min(base.other_segments.len());
    JpgDiff { other_segments: Some(JpgOtherSegmentsDiff { removed: vec![], modified: vec![], added: vec![JpgSegmentAdded { index: at, item: segment }] }), ..Default::default() }
}

pub fn diff_remove_other_segment(base: &JpgSnapshot, index: usize) -> JpgDiff {
    if index >= base.other_segments.len() { return JpgDiff::default(); }
    JpgDiff { other_segments: Some(JpgOtherSegmentsDiff { removed: vec![index], modified: vec![], added: vec![] }), ..Default::default() }
}

pub fn diff_set_pixels(base: &JpgSnapshot, pixels: Vec<u8>) -> JpgDiff {
    JpgDiff { pixels: (base.pixels != pixels).then_some(pixels), ..Default::default() }
}

pub fn diff_set_re_encode_quality(base: &JpgSnapshot, quality: Option<u8>) -> JpgDiff {
    JpgDiff { re_encode_quality: (base.re_encode_quality != quality).then_some(quality), ..Default::default() }
}
//#endregion 🔖️MutationDiffBuilders

//#region 🔖️HandcraftedDiffCodec
/// 🧪️ F6: hand-rolled `protocol::DiffCodec` for `JpgDiff` — see `JpgDiff`'s doc comment for the
/// confirmed compile-error citations. Same grammar style `GifDiff`/`SvgDiff`'s hand-rolled codecs
/// use (bracket-depth-aware split, hex for strings/bytes, `[0]`/`[1,x]` for `Option<T>`,
/// `name{[removed];[modified];[added]}`-shaped collection triples) — primitives re-declared here
/// (no shared "hand-roll helpers" module exists yet, per f6-recon-report.md §5's "known
/// duplication" note); several made `pub(crate)` so `JpgMutation`'s `OpText`/`OpBinary` can reuse
/// them without a third copy.
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
pub(crate) fn split_top_level(s: &str, sep: char) -> Vec<&str> {
    if s.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut depth = 0i32;
    let mut start = 0usize;
    for (i, c) in s.char_indices() {
        match c {
            '[' => depth += 1,
            ']' => depth -= 1,
            c if c == sep && depth == 0 => {
                out.push(&s[start..i]);
                start = i + c.len_utf8();
            }
            _ => {}
        }
    }
    out.push(&s[start..]);
    out
}
pub(crate) fn strip_brackets(s: &str) -> Result<&str, String> {
    s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))
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
pub(crate) fn parse_u8(s: &str) -> Result<u8, String> { s.parse().map_err(|e: std::num::ParseIntError| e.to_string()) }
pub(crate) fn parse_u16(s: &str) -> Result<u16, String> { s.parse().map_err(|e: std::num::ParseIntError| e.to_string()) }
pub(crate) fn parse_u32(s: &str) -> Result<u32, String> { s.parse().map_err(|e: std::num::ParseIntError| e.to_string()) }
pub(crate) fn parse_usize(s: &str) -> Result<usize, String> { s.parse().map_err(|e: std::num::ParseIntError| e.to_string()) }
pub(crate) fn parse_bool(s: &str) -> Result<bool, String> {
    match s {
        "0" => Ok(false),
        "1" => Ok(true),
        other => Err(format!("bad bool {other:?}")),
    }
}
//#endregion 🔖️Primitives

//#region 🔖️ValueCodecs
pub(crate) fn enc_density_units(u: &JfifDensityUnits) -> String { u.to_u8().to_string() }
pub(crate) fn dec_density_units(s: &str) -> Result<JfifDensityUnits, String> { JfifDensityUnits::from_u8(parse_u8(s)?) }
pub(crate) fn enc_huffman_class(c: &JpgHuffmanClass) -> String { c.to_u8().to_string() }
pub(crate) fn dec_huffman_class(s: &str) -> Result<JpgHuffmanClass, String> { JpgHuffmanClass::from_u8(parse_u8(s)?) }

pub(crate) fn enc_version(v: &(u8, u8)) -> String { format!("[{},{}]", v.0, v.1) }
pub(crate) fn dec_version(s: &str) -> Result<(u8, u8), String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [a, b] = parts.as_slice() else { return Err(format!("jfif version: expected 2 fields, got {}", parts.len())) };
    Ok((parse_u8(a)?, parse_u8(b)?))
}

fn enc_quant_values(v: &[u16; 64]) -> String {
    format!("[{}]", v.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","))
}
fn dec_quant_values(s: &str) -> Result<[u16; 64], String> {
    let values: Vec<u16> = split_top_level(strip_brackets(s)?, ',').into_iter().map(parse_u16).collect::<Result<_, _>>()?;
    <[u16; 64]>::try_from(values).map_err(|v: Vec<u16>| format!("quant values: expected 64, got {}", v.len()))
}

fn enc_bits16(b: &[u8; 16]) -> String { hex_encode(b) }
fn dec_bits16(s: &str) -> Result<[u8; 16], String> {
    <[u8; 16]>::try_from(hex_decode(s)?).map_err(|v: Vec<u8>| format!("huffman bits: expected 16, got {}", v.len()))
}

pub(crate) fn enc_thumbnail(t: &JfifThumbnail) -> String {
    format!("[{},{},{}]", t.width, t.height, hex_encode(&t.rgb_data))
}
pub(crate) fn dec_thumbnail(s: &str) -> Result<JfifThumbnail, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [width, height, rgb_data] = parts.as_slice() else { return Err(format!("thumbnail: expected 3 fields, got {}", parts.len())) };
    Ok(JfifThumbnail { width: parse_u8(width)?, height: parse_u8(height)?, rgb_data: hex_decode(rgb_data)? })
}

fn enc_frame_component(c: &JpgFrameComponent) -> String {
    format!("[{},{},{},{}]", c.id, c.h_sampling, c.v_sampling, c.quant_table_id)
}
fn dec_frame_component(s: &str) -> Result<JpgFrameComponent, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, h, v, q] = parts.as_slice() else { return Err(format!("frame component: expected 4 fields, got {}", parts.len())) };
    Ok(JpgFrameComponent { id: parse_u8(id)?, h_sampling: parse_u8(h)?, v_sampling: parse_u8(v)?, quant_table_id: parse_u8(q)? })
}

pub(crate) fn enc_frame_header(f: &JpgFrameHeader) -> String {
    let comps = f.components.iter().map(enc_frame_component).collect::<Vec<_>>().join(",");
    format!("[{},{},{},[{}]]", f.precision, f.width, f.height, comps)
}
pub(crate) fn dec_frame_header(s: &str) -> Result<JpgFrameHeader, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [precision, width, height, components] = parts.as_slice() else { return Err(format!("frame header: expected 4 fields, got {}", parts.len())) };
    let components = split_top_level(strip_brackets(components)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_frame_component).collect::<Result<Vec<_>, String>>()?;
    Ok(JpgFrameHeader { precision: parse_u8(precision)?, width: parse_u16(width)?, height: parse_u16(height)?, components })
}

pub(crate) fn enc_quant_table(t: &JpgQuantTable) -> String {
    format!("[{},{},{}]", t.id, t.precision, enc_quant_values(&t.values))
}
pub(crate) fn dec_quant_table(s: &str) -> Result<JpgQuantTable, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, precision, values] = parts.as_slice() else { return Err(format!("quant table: expected 3 fields, got {}", parts.len())) };
    Ok(JpgQuantTable { id: parse_u8(id)?, precision: parse_u8(precision)?, values: dec_quant_values(values)? })
}

pub(crate) fn enc_huffman_table(t: &JpgHuffmanTable) -> String {
    format!("[{},{},{},{}]", t.id, enc_huffman_class(&t.class), enc_bits16(&t.bits), hex_encode(&t.values))
}
pub(crate) fn dec_huffman_table(s: &str) -> Result<JpgHuffmanTable, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, class, bits, values] = parts.as_slice() else { return Err(format!("huffman table: expected 4 fields, got {}", parts.len())) };
    Ok(JpgHuffmanTable { id: parse_u8(id)?, class: dec_huffman_class(class)?, bits: dec_bits16(bits)?, values: hex_decode(values)? })
}

pub(crate) fn enc_huffman_key(k: &JpgHuffmanTableKey) -> String {
    format!("[{},{}]", enc_huffman_class(&k.class), k.id)
}
pub(crate) fn dec_huffman_key(s: &str) -> Result<JpgHuffmanTableKey, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [class, id] = parts.as_slice() else { return Err(format!("huffman key: expected 2 fields, got {}", parts.len())) };
    Ok(JpgHuffmanTableKey { class: dec_huffman_class(class)?, id: parse_u8(id)? })
}

pub(crate) fn enc_segment(s: &JpgSegment) -> String {
    format!("[{},{}]", s.marker, hex_encode(&s.data))
}
pub(crate) fn dec_segment(s: &str) -> Result<JpgSegment, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [marker, data] = parts.as_slice() else { return Err(format!("segment: expected 2 fields, got {}", parts.len())) };
    Ok(JpgSegment { marker: parse_u8(marker)?, data: hex_decode(data)? })
}
//#endregion 🔖️ValueCodecs

//#region 🔖️DiffValueCodecs
fn enc_component_diff(d: &JpgComponentDiff) -> String {
    format!(
        "[{},{},{}]",
        encode_option(&d.h_sampling, |v| v.to_string()),
        encode_option(&d.v_sampling, |v| v.to_string()),
        encode_option(&d.quant_table_id, |v| v.to_string()),
    )
}
fn dec_component_diff(s: &str) -> Result<JpgComponentDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [h, v, q] = parts.as_slice() else { return Err(format!("component diff: expected 3 fields, got {}", parts.len())) };
    Ok(JpgComponentDiff { h_sampling: decode_option(h, parse_u8)?, v_sampling: decode_option(v, parse_u8)?, quant_table_id: decode_option(q, parse_u8)? })
}
fn enc_components_diff(d: &JpgComponentsDiff) -> String {
    let removed = d.removed.iter().map(|id| id.to_string()).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:{}", m.id, enc_component_diff(&m.diff))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}", a.index, enc_frame_component(&a.item))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}
fn dec_components_diff(body: &str) -> Result<JpgComponentsDiff, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("components diff: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_u8).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|entry| {
        let (id, rest) = entry.split_once(':').ok_or_else(|| format!("component modified: bad entry {entry:?}"))?;
        Ok(JpgComponentModified { id: parse_u8(id)?, diff: dec_component_diff(rest)? })
    }).collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|entry| {
        let (index, rest) = entry.split_once(':').ok_or_else(|| format!("component added: bad entry {entry:?}"))?;
        Ok(JpgComponentAdded { index: parse_usize(index)?, item: dec_frame_component(rest)? })
    }).collect::<Result<Vec<_>, String>>()?;
    Ok(JpgComponentsDiff { removed, modified, added })
}

fn enc_quant_table_diff(d: &JpgQuantTableDiff) -> String {
    format!("[{},{}]", encode_option(&d.precision, |v| v.to_string()), encode_option(&d.values, enc_quant_values))
}
fn dec_quant_table_diff(s: &str) -> Result<JpgQuantTableDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [precision, values] = parts.as_slice() else { return Err(format!("quant table diff: expected 2 fields, got {}", parts.len())) };
    Ok(JpgQuantTableDiff { precision: decode_option(precision, parse_u8)?, values: decode_option(values, dec_quant_values)? })
}
fn enc_quant_tables_diff(d: &JpgQuantTablesDiff) -> String {
    let removed = d.removed.iter().map(|id| id.to_string()).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:{}", m.id, enc_quant_table_diff(&m.diff))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}", a.index, enc_quant_table(&a.item))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}
fn dec_quant_tables_diff(body: &str) -> Result<JpgQuantTablesDiff, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("quant tables diff: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_u8).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|entry| {
        let (id, rest) = entry.split_once(':').ok_or_else(|| format!("quant table modified: bad entry {entry:?}"))?;
        Ok(JpgQuantTableModified { id: parse_u8(id)?, diff: dec_quant_table_diff(rest)? })
    }).collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|entry| {
        let (index, rest) = entry.split_once(':').ok_or_else(|| format!("quant table added: bad entry {entry:?}"))?;
        Ok(JpgQuantTableAdded { index: parse_usize(index)?, item: dec_quant_table(rest)? })
    }).collect::<Result<Vec<_>, String>>()?;
    Ok(JpgQuantTablesDiff { removed, modified, added })
}

fn enc_huffman_table_diff(d: &JpgHuffmanTableDiff) -> String {
    format!("[{},{}]", encode_option(&d.bits, enc_bits16), encode_option(&d.values, |v| hex_encode(v)))
}
fn dec_huffman_table_diff(s: &str) -> Result<JpgHuffmanTableDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [bits, values] = parts.as_slice() else { return Err(format!("huffman table diff: expected 2 fields, got {}", parts.len())) };
    Ok(JpgHuffmanTableDiff { bits: decode_option(bits, dec_bits16)?, values: decode_option(values, hex_decode)? })
}
fn enc_huffman_tables_diff(d: &JpgHuffmanTablesDiff) -> String {
    let removed = d.removed.iter().map(enc_huffman_key).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:{}", enc_huffman_key(&m.key), enc_huffman_table_diff(&m.diff))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}", a.index, enc_huffman_table(&a.item))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}
fn dec_huffman_tables_diff(body: &str) -> Result<JpgHuffmanTablesDiff, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("huffman tables diff: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_huffman_key).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|entry| {
        let (key, rest) = entry.split_once(':').ok_or_else(|| format!("huffman table modified: bad entry {entry:?}"))?;
        Ok(JpgHuffmanTableModified { key: dec_huffman_key(key)?, diff: dec_huffman_table_diff(rest)? })
    }).collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|entry| {
        let (index, rest) = entry.split_once(':').ok_or_else(|| format!("huffman table added: bad entry {entry:?}"))?;
        Ok(JpgHuffmanTableAdded { index: parse_usize(index)?, item: dec_huffman_table(rest)? })
    }).collect::<Result<Vec<_>, String>>()?;
    Ok(JpgHuffmanTablesDiff { removed, modified, added })
}

fn enc_segment_diff(d: &JpgSegmentDiff) -> String {
    format!("[{},{}]", encode_option(&d.marker, |v| v.to_string()), encode_option(&d.data, |v| hex_encode(v)))
}
fn dec_segment_diff(s: &str) -> Result<JpgSegmentDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [marker, data] = parts.as_slice() else { return Err(format!("segment diff: expected 2 fields, got {}", parts.len())) };
    Ok(JpgSegmentDiff { marker: decode_option(marker, parse_u8)?, data: decode_option(data, hex_decode)? })
}
fn enc_other_segments_diff(d: &JpgOtherSegmentsDiff) -> String {
    let removed = d.removed.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:{}", m.index, enc_segment_diff(&m.diff))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}", a.index, enc_segment(&a.item))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}
fn dec_other_segments_diff(body: &str) -> Result<JpgOtherSegmentsDiff, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("other segments diff: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_usize).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|entry| {
        let (index, rest) = entry.split_once(':').ok_or_else(|| format!("segment modified: bad entry {entry:?}"))?;
        Ok(JpgSegmentModified { index: parse_usize(index)?, diff: dec_segment_diff(rest)? })
    }).collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|entry| {
        let (index, rest) = entry.split_once(':').ok_or_else(|| format!("segment added: bad entry {entry:?}"))?;
        Ok(JpgSegmentAdded { index: parse_usize(index)?, item: dec_segment(rest)? })
    }).collect::<Result<Vec<_>, String>>()?;
    Ok(JpgOtherSegmentsDiff { removed, modified, added })
}

/// 🌲 `JpgFrameChange`'s tag prefix: `M[fields-diff]` (Modify) / `R[frame-opt]` (Replace) — mirrors
/// `enc_xml_node`/`enc_node_diff`'s single-letter-tag convention (svg/gif precedent).
pub(crate) fn enc_frame_change(fc: &JpgFrameChange) -> String {
    match fc {
        JpgFrameChange::Modify(fd) => format!("M[{}]", enc_frame_fields_diff(fd)),
        JpgFrameChange::Replace { frame } => format!("R[{}]", encode_option(frame, enc_frame_header)),
    }
}
pub(crate) fn dec_frame_change(s: &str) -> Result<JpgFrameChange, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "M" => Ok(JpgFrameChange::Modify(dec_frame_fields_diff(inner)?)),
        "R" => Ok(JpgFrameChange::Replace { frame: decode_option(inner, dec_frame_header)? }),
        other => Err(format!("frame change: unknown tag {other:?}")),
    }
}
fn enc_frame_fields_diff(fd: &JpgFrameFieldsDiff) -> String {
    format!(
        "[{},{},{},{}]",
        encode_option(&fd.precision, |v| v.to_string()),
        encode_option(&fd.width, |v| v.to_string()),
        encode_option(&fd.height, |v| v.to_string()),
        encode_option(&fd.components, enc_components_diff),
    )
}
fn dec_frame_fields_diff(s: &str) -> Result<JpgFrameFieldsDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [precision, width, height, components] = parts.as_slice() else { return Err(format!("frame fields diff: expected 4 fields, got {}", parts.len())) };
    Ok(JpgFrameFieldsDiff {
        precision: decode_option(precision, parse_u8)?,
        width: decode_option(width, parse_u16)?,
        height: decode_option(height, parse_u16)?,
        components: decode_option(components, dec_components_diff)?,
    })
}
//#endregion 🔖️DiffValueCodecs

//#region 🔖️TopLevel
/// 🧾 Top-level line: space-separated `name=value` tokens, one per changed field, absent token =
/// unchanged (recipe convention). Tri-state fields (`re-encode-quality`/`jfif-thumbnail`/
/// `restart-interval`) additionally wrap their value in `[0]`/`[1,x]` since the token's presence
/// alone only means "the tri-state slot changed", not which of {cleared, set} it changed to.
fn print_jpg_diff(d: &JpgDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = d.width { tokens.push(format!("width={v}")); }
    if let Some(v) = d.height { tokens.push(format!("height={v}")); }
    if let Some(v) = &d.pixels { tokens.push(format!("pixels={}", hex_encode(v))); }
    if let Some(v) = &d.re_encode_quality { tokens.push(format!("re-encode-quality={}", encode_option(v, |q| q.to_string()))); }
    if let Some(v) = d.jfif_version { tokens.push(format!("jfif-version={}", enc_version(&v))); }
    if let Some(v) = d.jfif_density_units { tokens.push(format!("jfif-density-units={}", enc_density_units(&v))); }
    if let Some(v) = d.jfif_x_density { tokens.push(format!("jfif-x-density={v}")); }
    if let Some(v) = d.jfif_y_density { tokens.push(format!("jfif-y-density={v}")); }
    if let Some(v) = &d.jfif_thumbnail { tokens.push(format!("jfif-thumbnail={}", encode_option(v, enc_thumbnail))); }
    if let Some(v) = &d.frame { tokens.push(format!("frame={}", enc_frame_change(v))); }
    if let Some(v) = d.sof_marker { tokens.push(format!("sof-marker={v}")); }
    if let Some(v) = d.arithmetic { tokens.push(format!("arithmetic={}", if v { 1 } else { 0 })); }
    if let Some(v) = &d.quant_tables { tokens.push(format!("quant-tables={}", enc_quant_tables_diff(v))); }
    if let Some(v) = &d.huffman_tables { tokens.push(format!("huffman-tables={}", enc_huffman_tables_diff(v))); }
    if let Some(v) = &d.restart_interval { tokens.push(format!("restart-interval={}", encode_option(v, |ri| ri.to_string()))); }
    if let Some(v) = &d.other_segments { tokens.push(format!("other-segments={}", enc_other_segments_diff(v))); }
    tokens.join(" ")
}
fn parse_jpg_diff(line: &str) -> Result<JpgDiff, String> {
    let mut d = JpgDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("width=") { d.width = Some(parse_u32(rest)?); }
        else if let Some(rest) = token.strip_prefix("height=") { d.height = Some(parse_u32(rest)?); }
        else if let Some(rest) = token.strip_prefix("pixels=") { d.pixels = Some(hex_decode(rest)?); }
        else if let Some(rest) = token.strip_prefix("re-encode-quality=") { d.re_encode_quality = Some(decode_option(rest, parse_u8)?); }
        else if let Some(rest) = token.strip_prefix("jfif-version=") { d.jfif_version = Some(dec_version(rest)?); }
        else if let Some(rest) = token.strip_prefix("jfif-density-units=") { d.jfif_density_units = Some(dec_density_units(rest)?); }
        else if let Some(rest) = token.strip_prefix("jfif-x-density=") { d.jfif_x_density = Some(parse_u16(rest)?); }
        else if let Some(rest) = token.strip_prefix("jfif-y-density=") { d.jfif_y_density = Some(parse_u16(rest)?); }
        else if let Some(rest) = token.strip_prefix("jfif-thumbnail=") { d.jfif_thumbnail = Some(decode_option(rest, dec_thumbnail)?); }
        else if let Some(rest) = token.strip_prefix("frame=") { d.frame = Some(dec_frame_change(rest)?); }
        else if let Some(rest) = token.strip_prefix("sof-marker=") { d.sof_marker = Some(parse_u8(rest)?); }
        else if let Some(rest) = token.strip_prefix("arithmetic=") { d.arithmetic = Some(parse_bool(rest)?); }
        else if let Some(rest) = token.strip_prefix("quant-tables=") { d.quant_tables = Some(dec_quant_tables_diff(rest)?); }
        else if let Some(rest) = token.strip_prefix("huffman-tables=") { d.huffman_tables = Some(dec_huffman_tables_diff(rest)?); }
        else if let Some(rest) = token.strip_prefix("restart-interval=") { d.restart_interval = Some(decode_option(rest, parse_u16)?); }
        else if let Some(rest) = token.strip_prefix("other-segments=") { d.other_segments = Some(dec_other_segments_diff(rest)?); }
        else { return Err(format!("jpg diff: unknown token {token:?}")); }
    }
    Ok(d)
}

impl protocol::DiffCodec for JpgDiff {
    fn print_diff(&self) -> String {
        print_jpg_diff(self)
    }
    fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        parse_jpg_diff(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    /// ⚡️ Binary = the text bytes verbatim, same simplification `GifDiff`/`SvgDiff`/`WriterDiff`
    /// use — satisfies every `DiffCodec` law without inventing a second wire format.
    fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(self.print_diff().into_bytes())
    }
    fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let line = std::str::from_utf8(bytes).map_err(|e| protocol::ProtocolError::Malformed { what: "diff utf8", offset: 0, detail: e.to_string() })?;
        Self::parse_diff(line).map_err(|e| protocol::ProtocolError::Malformed { what: "diff text", offset: 0, detail: e.to_string() })
    }
}
//#endregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec

//#region 🧪️Tests
#[cfg(test)]
mod handcrafted_diff_codec_tests {
    use super::*;
    use protocol::command::DiffAlgebra;
    use protocol::DiffCodec;

    fn quant(id: u8, seed: u16) -> JpgQuantTable { JpgQuantTable { id, precision: 0, values: [seed; 64] } }
    fn huffman(class: JpgHuffmanClass, id: u8, seed: u8) -> JpgHuffmanTable { JpgHuffmanTable { id, class, bits: [seed; 16], values: vec![seed, seed.wrapping_add(1)] } }
    fn segment(marker: u8, data: Vec<u8>) -> JpgSegment { JpgSegment { marker, data } }

    /// 🌱 `snap_a`/`snap_b` differ in EVERY diffable field (both directions exercise removed XOR
    /// added per id-keyed/index-keyed collection, the recipe's documented workaround — see
    /// `f6-recon-report.md`'s `field_sweep` precedent). `snap_c` has `frame: None`, exercising
    /// `JpgFrameChange::Replace` against both `a` and `c`.
    fn snap_a() -> JpgSnapshot {
        JpgSnapshot {
            schema: "stdio.jpg".into(),
            width: 4,
            height: 4,
            pixels: vec![0u8; 16],
            re_encode_quality: Some(80),
            jfif_version: (1, 1),
            jfif_density_units: JfifDensityUnits::PixelsPerInch,
            jfif_x_density: 72,
            jfif_y_density: 72,
            jfif_thumbnail: Some(JfifThumbnail { width: 2, height: 1, rgb_data: vec![1, 2, 3, 4, 5, 6] }),
            frame: Some(JpgFrameHeader {
                precision: 8,
                width: 4,
                height: 4,
                components: vec![
                    JpgFrameComponent { id: 1, h_sampling: 2, v_sampling: 2, quant_table_id: 0 },
                    JpgFrameComponent { id: 9, h_sampling: 1, v_sampling: 1, quant_table_id: 1 },
                ],
            }),
            sof_marker: 0xC0,
            arithmetic: false,
            quant_tables: vec![quant(0, 10), quant(9, 20)],
            huffman_tables: vec![huffman(JpgHuffmanClass::Dc, 0, 1), huffman(JpgHuffmanClass::Ac, 9, 2)],
            restart_interval: Some(8),
            other_segments: vec![segment(0xFE, vec![1, 2, 3]), segment(0xE1, vec![9, 9])],
        }
    }
    fn snap_b() -> JpgSnapshot {
        JpgSnapshot {
            schema: "stdio.jpg".into(),
            width: 8,
            height: 6,
            pixels: vec![9u8; 12],
            re_encode_quality: None,
            jfif_version: (1, 2),
            jfif_density_units: JfifDensityUnits::Aspect,
            jfif_x_density: 1,
            jfif_y_density: 1,
            jfif_thumbnail: None,
            frame: Some(JpgFrameHeader {
                precision: 8,
                width: 8,
                height: 6,
                components: vec![JpgFrameComponent { id: 1, h_sampling: 1, v_sampling: 1, quant_table_id: 5 }],
            }),
            sof_marker: 0xC2,
            arithmetic: true,
            quant_tables: vec![quant(0, 99)],
            huffman_tables: vec![huffman(JpgHuffmanClass::Dc, 0, 7)],
            restart_interval: None,
            other_segments: vec![segment(0xFE, vec![4, 5, 6])],
        }
    }
    fn snap_c() -> JpgSnapshot { JpgSnapshot { frame: None, ..JpgSnapshot::default() } }

    /// 🧪️ F6: `DiffCodec` round-trip laws over the hand-rolled `JpgDiff` grammar — exercises the
    /// `JpgFrameChange` enum (both `Modify` and `Replace`), all three tri-state scalars in both
    /// transition directions, and all three id/index-keyed collection triples with removed,
    /// modified, AND added entries all populated across the two `between()` directions.
    #[test]
    fn diff_codec_text_binary_roundtrip_law() {
        let a = snap_a();
        let b = snap_b();
        let c = snap_c();
        let cases = vec![
            JpgDiff::default(),
            JpgDiff::between(&a, &b),
            JpgDiff::between(&b, &a),
            JpgDiff::between(&a, &c),
            JpgDiff::between(&c, &a),
        ];
        for d in cases {
            let printed = d.print_diff();
            assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
            let parsed = JpgDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
            assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

            let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
            let decoded = JpgDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
            assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
        }
    }
}
//#endregion 🧪️Tests
//#endregion 🔖️HandcraftedDiffCodec
