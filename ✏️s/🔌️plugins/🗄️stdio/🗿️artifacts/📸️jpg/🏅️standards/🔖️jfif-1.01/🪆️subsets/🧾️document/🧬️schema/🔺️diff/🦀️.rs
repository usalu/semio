//! 🔺️ JpgDiff — handcrafted sparse diff. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: replaces the old
//! `JpgDiff{snapshot: Option<JpgSnapshot>}` full-replace template. JFIF header fields are
//! top-level scalars; `frame` is a `Modify`/`Replace` change (mirrors xml's `XmlNodeDiff::Replace`
//! fallback — a decode-status transition (`None`<->`Some`) is a "kind change", everything else is
//! a field-level `Modify`); `quant_tables`/`huffman_tables`/`frame.components` are id-keyed
//! `removed`/`modified`/`added` triples (stable identity, no index-transport needed — see
//! `absorb_id_keyed_*`); `other_segments` is an index-keyed triple (position-transported absorb,
//! mirrors png's `text_chunks`).

use crate::artifacts::jpg::schema::snapshot::{JfifDensityUnits, JfifThumbnail, JpgFrameComponent, JpgFrameHeader, JpgHuffmanClass, JpgHuffmanTable, JpgQuantTable, JpgSegment};
use crate::artifacts::jpg::JpgSnapshot;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use schema::ArtifactSchema;
use std::collections::{BTreeMap, HashMap};

//#region 🔖️ComponentsDiff
/// 🧩️ Sparse per-field patch for one `JpgFrameComponent`. `id` is the identity, never diffed.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct JpgComponentDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub h_sampling: Option<u8>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub v_sampling: Option<u8>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub quant_table_id: Option<u8>,
}

impl JpgComponentDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn apply(&self, base: &JpgFrameComponent) -> JpgFrameComponent {
        JpgFrameComponent { id: base.id, h_sampling: self.h_sampling.unwrap_or(base.h_sampling), v_sampling: self.v_sampling.unwrap_or(base.v_sampling), quant_table_id: self.quant_table_id.unwrap_or(base.quant_table_id) }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn between(a: &JpgFrameComponent, b: &JpgFrameComponent) -> Self {
        Self { h_sampling: (a.h_sampling != b.h_sampling).then_some(b.h_sampling), v_sampling: (a.v_sampling != b.v_sampling).then_some(b.v_sampling), quant_table_id: (a.quant_table_id != b.quant_table_id).then_some(b.quant_table_id) }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn absorb(&mut self, other: Self) {
        if other.h_sampling.is_some() {
            self.h_sampling = other.h_sampling;
        }
        if other.v_sampling.is_some() {
            self.v_sampling = other.v_sampling;
        }
        if other.quant_table_id.is_some() {
            self.quant_table_id = other.quant_table_id;
        }
    }
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct JpgComponentModified {
    pub id: u8,
    pub diff: JpgComponentDiff,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct JpgComponentAdded {
    pub index: usize,
    pub item: JpgFrameComponent,
}

/// 🔺️ Id-keyed `frame.components` triple.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct JpgComponentsDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<u8>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<JpgComponentModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<JpgComponentAdded>,
}
impl JpgComponentsDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
}
//#endregion 🔖️ComponentsDiff

//#region 🔖️FrameDiff
/// 🖼️ Sparse per-field patch for one `JpgFrameHeader`, used when BOTH base and next have
/// `Some(frame)` (see `JpgFrameChange::Modify`).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct JpgFrameFieldsDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub precision: Option<u8>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<u16>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<u16>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub components: Option<JpgComponentsDiff>,
}
/// 🌲️ `frame`'s change shape: `Modify` when both base/next have a frame (field-level patch,
/// including the id-keyed `components` triple); `Replace` on a decode-status "kind change"
/// (`None`<->`Some`) — mirrors xml's `XmlNodeDiff::Replace` fallback for exactly this situation.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "change", rename_all = "camelCase")]
pub enum JpgFrameChange {
    Modify(JpgFrameFieldsDiff),
    Replace { frame: Option<JpgFrameHeader> },
}
//#endregion 🔖️FrameDiff

//#region 🔖️QuantTablesDiff
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct JpgQuantTableDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub precision: Option<u8>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<[u16; 64]>,
}
impl JpgQuantTableDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(in super::super) fn is_empty(&self) -> bool {
        self == &Self::default()
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn apply(&self, base: &JpgQuantTable) -> JpgQuantTable {
        JpgQuantTable { id: base.id, precision: self.precision.unwrap_or(base.precision), values: self.values.unwrap_or(base.values) }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(in super::super) fn between(a: &JpgQuantTable, b: &JpgQuantTable) -> Self {
        Self { precision: (a.precision != b.precision).then_some(b.precision), values: (a.values != b.values).then_some(b.values) }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn absorb(&mut self, other: Self) {
        if other.precision.is_some() {
            self.precision = other.precision;
        }
        if other.values.is_some() {
            self.values = other.values;
        }
    }
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct JpgQuantTableModified {
    pub id: u8,
    pub diff: JpgQuantTableDiff,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct JpgQuantTableAdded {
    pub index: usize,
    pub item: JpgQuantTable,
}

/// 🔺️ Id-keyed `quant_tables` (DQT) triple.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct JpgQuantTablesDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<u8>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<JpgQuantTableModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<JpgQuantTableAdded>,
}
impl JpgQuantTablesDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(in super::super) fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
}
//#endregion 🔖️QuantTablesDiff

//#region 🔖️HuffmanTablesDiff
/// 🔑️ Compound identity for `huffman_tables` — DC id=0 and AC id=0 are different tables.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct JpgHuffmanTableKey {
    pub class: JpgHuffmanClass,
    pub id: u8,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct JpgHuffmanTableDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub bits: Option<[u8; 16]>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<u8>>,
}
impl JpgHuffmanTableDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(in super::super) fn is_empty(&self) -> bool {
        self == &Self::default()
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn apply(&self, base: &JpgHuffmanTable) -> JpgHuffmanTable {
        JpgHuffmanTable { id: base.id, class: base.class, bits: self.bits.unwrap_or(base.bits), values: self.values.clone().unwrap_or_else(|| base.values.clone()) }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(in super::super) fn between(a: &JpgHuffmanTable, b: &JpgHuffmanTable) -> Self {
        Self { bits: (a.bits != b.bits).then_some(b.bits), values: (a.values != b.values).then(|| b.values.clone()) }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn absorb(&mut self, other: Self) {
        if other.bits.is_some() {
            self.bits = other.bits;
        }
        if other.values.is_some() {
            self.values = other.values;
        }
    }
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct JpgHuffmanTableModified {
    pub key: JpgHuffmanTableKey,
    pub diff: JpgHuffmanTableDiff,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct JpgHuffmanTableAdded {
    pub index: usize,
    pub item: JpgHuffmanTable,
}

/// 🔺️ `(class, id)`-keyed `huffman_tables` (DHT) triple.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct JpgHuffmanTablesDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<JpgHuffmanTableKey>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<JpgHuffmanTableModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<JpgHuffmanTableAdded>,
}
impl JpgHuffmanTablesDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(in super::super) fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn huffman_key(t: &JpgHuffmanTable) -> JpgHuffmanTableKey {
    JpgHuffmanTableKey { class: t.class, id: t.id }
}
//#endregion 🔖️HuffmanTablesDiff

//#region 🔖️OtherSegmentsDiff
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct JpgSegmentDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<u8>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<u8>>,
}
impl JpgSegmentDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn apply(&self, base: &JpgSegment) -> JpgSegment {
        JpgSegment { marker: self.marker.unwrap_or(base.marker), data: self.data.clone().unwrap_or_else(|| base.data.clone()) }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn between(a: &JpgSegment, b: &JpgSegment) -> Self {
        Self { marker: (a.marker != b.marker).then_some(b.marker), data: (a.data != b.data).then(|| b.data.clone()) }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn absorb(&mut self, other: Self) {
        if other.marker.is_some() {
            self.marker = other.marker;
        }
        if other.data.is_some() {
            self.data = other.data;
        }
    }
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct JpgSegmentModified {
    pub index: usize,
    pub diff: JpgSegmentDiff,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct JpgSegmentAdded {
    pub index: usize,
    pub item: JpgSegment,
}

/// 🔺️ Index-keyed `other_segments` triple (position-transported absorb — duplicate markers are
/// legal, so identity is position, not the marker byte).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct JpgOtherSegmentsDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<JpgSegmentModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<JpgSegmentAdded>,
}
//#endregion 🔖️OtherSegmentsDiff

//#region 🔖️IndexTransport
// 🧮 Base-free index transport for `other_segments`' absorb — ported verbatim from png's
// `simulate_slots`/`base_len_hint`/`absorb_text_chunks` shape (position-keyed, field-aware
// modified payload), retargeted to `JpgSegment`/`JpgSegmentDiff`.
#[derive(Clone, Copy, Debug)]
enum Slot {
    Base(usize),
    Added(usize),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn simulate_slots(len: usize, removed: &[usize], added_indices: &[usize]) -> Vec<Slot> {
    let mut slots: Vec<Slot> = (0..len).map(Slot::Base).collect();
    let mut removed_desc = removed.to_vec();
    removed_desc.sort_unstable_by(|a, b| b.cmp(a));
    removed_desc.dedup();
    for r in removed_desc {
        if r < slots.len() {
            slots.remove(r);
        }
    }
    let mut order: Vec<usize> = (0..added_indices.len()).collect();
    order.sort_by_key(|&i| added_indices[i]);
    for i in order {
        let at = added_indices[i].min(slots.len());
        slots.insert(at, Slot::Added(i));
    }
    slots
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn base_len_hint(removed: &[usize], modified_indices: impl Iterator<Item = usize>, added_indices: impl Iterator<Item = usize>) -> usize {
    removed.iter().copied().chain(modified_indices).chain(added_indices).max().map(|m| m + 1).unwrap_or(0)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_other_segments(d1: JpgOtherSegmentsDiff, d2: JpgOtherSegmentsDiff) -> JpgOtherSegmentsDiff {
    let d1_added_indices: Vec<usize> = d1.added.iter().map(|a| a.index).collect();
    let removed_count = {
        let mut r = d1.removed.clone();
        r.sort_unstable();
        r.dedup();
        r.len()
    };
    let needed_mid_len = d2.removed.iter().copied().chain(d2.modified.iter().map(|m| m.index)).max().map(|m| m + 1).unwrap_or(0);
    let base_len = base_len_hint(&d1.removed, d1.modified.iter().map(|m| m.index), d1_added_indices.iter().copied()).max((needed_mid_len + removed_count).saturating_sub(d1.added.len()));
    let mid_slots = simulate_slots(base_len, &d1.removed, &d1_added_indices);

    let mut final_removed: Vec<usize> = d1.removed;
    let mut modified_map: BTreeMap<usize, JpgSegmentDiff> = d1.modified.into_iter().map(|m| (m.index, m.diff)).collect();
    let mut added_alive: Vec<Option<JpgSegmentAdded>> = d1.added.into_iter().map(Some).collect();

    for mid_idx in &d2.removed {
        match mid_slots.get(*mid_idx) {
            Some(Slot::Base(b)) => {
                final_removed.push(*b);
                modified_map.remove(b);
            }
            Some(Slot::Added(ai)) => {
                added_alive[*ai] = None;
            }
            None => {}
        }
    }
    for m2 in &d2.modified {
        match mid_slots.get(m2.index) {
            Some(Slot::Base(b)) => {
                modified_map.entry(*b).or_default().absorb(m2.diff.clone());
            }
            Some(Slot::Added(ai)) => {
                if let Some(a) = added_alive[*ai].as_mut() {
                    a.item = m2.diff.apply(&a.item);
                }
            }
            None => {}
        }
    }

    final_removed.sort_unstable();
    final_removed.dedup();
    for r in &final_removed {
        modified_map.remove(r);
    }
    let mut final_modified: Vec<JpgSegmentModified> = modified_map.into_iter().filter(|(_, d)| !d.is_empty()).map(|(index, diff)| JpgSegmentModified { index, diff }).collect();
    final_modified.sort_by_key(|m| m.index);

    let alive_mid_positions: Vec<usize> = mid_slots
        .iter()
        .enumerate()
        .filter_map(|(pos, slot)| match slot {
            Slot::Added(ai) if added_alive[*ai].is_some() => Some(pos),
            _ => None,
        })
        .collect();
    let d2_added_indices: Vec<usize> = d2.added.iter().map(|a| a.index).collect();
    let mid_len = d2.removed.iter().copied().chain(d2.modified.iter().map(|m| m.index)).chain(alive_mid_positions.iter().copied()).chain(d2_added_indices.iter().copied()).max().map(|m| m + 1).unwrap_or(0);
    let after_slots = simulate_slots(mid_len, &d2.removed, &d2_added_indices);
    let mut mid_to_after: HashMap<usize, usize> = HashMap::new();
    for (pos, slot) in after_slots.iter().enumerate() {
        if let Slot::Base(m) = slot {
            mid_to_after.insert(*m, pos);
        }
    }

    let mut final_added: Vec<JpgSegmentAdded> = Vec::new();
    for (ai, alive) in added_alive.into_iter().enumerate() {
        if let Some(added) = alive {
            let mid_pos = mid_slots.iter().position(|s| matches!(s, Slot::Added(idx) if *idx == ai)).expect("added_alive index always has a corresponding mid slot");
            if let Some(after_pos) = mid_to_after.get(&mid_pos) {
                final_added.push(JpgSegmentAdded { index: *after_pos, item: added.item });
            }
        }
    }
    for a2 in d2.added {
        final_added.push(a2);
    }
    final_added.sort_by_key(|a| a.index);

    JpgOtherSegmentsDiff { removed: final_removed, modified: final_modified, added: final_added }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_other_segments_opt(base: &mut Option<JpgOtherSegmentsDiff>, other: Option<JpgOtherSegmentsDiff>) {
    match (base.take(), other) {
        (None, o) => *base = o,
        (Some(b), None) => *base = Some(b),
        (Some(b), Some(o)) => *base = Some(absorb_other_segments(b, o)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_other_segments(base: &[JpgSegment], d: &JpgOtherSegmentsDiff) -> Vec<JpgSegment> {
    let mut items = base.to_vec();
    for m in &d.modified {
        if let Some(it) = items.get_mut(m.index) {
            *it = m.diff.apply(it);
        }
    }
    let mut removed_desc = d.removed.clone();
    removed_desc.sort_unstable_by(|a, b| b.cmp(a));
    removed_desc.dedup();
    for idx in removed_desc {
        if idx < items.len() {
            items.remove(idx);
        }
    }
    let mut adds = d.added.clone();
    adds.sort_by_key(|a| a.index);
    for a in adds {
        let at = a.index.min(items.len());
        items.insert(at, a.item);
    }
    items
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_other_segments(a: &[JpgSegment], b: &[JpgSegment]) -> Option<JpgOtherSegmentsDiff> {
    let min = a.len().min(b.len());
    let mut modified = Vec::new();
    for i in 0..min {
        if a[i] != b[i] {
            let d = JpgSegmentDiff::between(&a[i], &b[i]);
            if !d.is_empty() {
                modified.push(JpgSegmentModified { index: i, diff: d });
            }
        }
    }
    let removed: Vec<usize> = (min..a.len()).collect();
    let added: Vec<JpgSegmentAdded> = (min..b.len()).map(|i| JpgSegmentAdded { index: i, item: b[i].clone() }).collect();
    if removed.is_empty() && modified.is_empty() && added.is_empty() {
        None
    } else {
        Some(JpgOtherSegmentsDiff { removed, modified, added })
    }
}
//#endregion 🔖️IndexTransport

//#region 🔖️IdKeyedTransport
// 🧮 Stable-key absorb for `quant_tables`/`huffman_tables`/`frame.components` — id/key identity
// doesn't shift with position, so (unlike `other_segments`) NO index-transport simulation is
// needed at all; mirrors zip's `absorb_entries` (name-keyed, no-rename-tracking-needed case:
// jpg has no id-renaming mutation, so the rename map zip carries is simply omitted here). `index`
// bookkeeping on surviving `added` entries uses the same documented best-effort shift zip does:
// exact when d2's genuine (non-annihilating) removals sit before the add.

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_quant_tables(mut d1: JpgQuantTablesDiff, d2: JpgQuantTablesDiff) -> JpgQuantTablesDiff {
    let added_ids: std::collections::HashSet<u8> = d1.added.iter().map(|a| a.item.id).collect();
    let mut removed_shift = 0usize;
    for id in &d2.removed {
        if added_ids.contains(id) {
            d1.added.retain(|a| a.item.id != *id);
        } else {
            removed_shift += 1;
            if !d1.removed.contains(id) {
                d1.removed.push(*id);
            }
            d1.modified.retain(|m| m.id != *id);
        }
    }
    let mut merged_added: Vec<JpgQuantTableAdded> = d1
        .added
        .into_iter()
        .map(|mut a| {
            a.index = a.index.saturating_sub(removed_shift);
            a
        })
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_huffman_tables(mut d1: JpgHuffmanTablesDiff, d2: JpgHuffmanTablesDiff) -> JpgHuffmanTablesDiff {
    let added_keys: std::collections::HashSet<JpgHuffmanTableKey> = d1.added.iter().map(|a| huffman_key(&a.item)).collect();
    let mut removed_shift = 0usize;
    for key in &d2.removed {
        if added_keys.contains(key) {
            d1.added.retain(|a| huffman_key(&a.item) != *key);
        } else {
            removed_shift += 1;
            if !d1.removed.contains(key) {
                d1.removed.push(*key);
            }
            d1.modified.retain(|m| m.key != *key);
        }
    }
    let mut merged_added: Vec<JpgHuffmanTableAdded> = d1
        .added
        .into_iter()
        .map(|mut a| {
            a.index = a.index.saturating_sub(removed_shift);
            a
        })
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_components(mut d1: JpgComponentsDiff, d2: JpgComponentsDiff) -> JpgComponentsDiff {
    let added_ids: std::collections::HashSet<u8> = d1.added.iter().map(|a| a.item.id).collect();
    let mut removed_shift = 0usize;
    for id in &d2.removed {
        if added_ids.contains(id) {
            d1.added.retain(|a| a.item.id != *id);
        } else {
            removed_shift += 1;
            if !d1.removed.contains(id) {
                d1.removed.push(*id);
            }
            d1.modified.retain(|m| m.id != *id);
        }
    }
    let mut merged_added: Vec<JpgComponentAdded> = d1
        .added
        .into_iter()
        .map(|mut a| {
            a.index = a.index.saturating_sub(removed_shift);
            a
        })
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_components(a: &[JpgFrameComponent], b: &[JpgFrameComponent]) -> JpgComponentsDiff {
    let mut removed = Vec::new();
    let mut modified = Vec::new();
    for ac in a {
        match b.iter().find(|bc| bc.id == ac.id) {
            Some(bc) => {
                let d = JpgComponentDiff::between(ac, bc);
                if !d.is_empty() {
                    modified.push(JpgComponentModified { id: ac.id, diff: d });
                }
            }
            None => removed.push(ac.id),
        }
    }
    let added: Vec<JpgComponentAdded> = b.iter().enumerate().filter(|(_, bc)| !a.iter().any(|ac| ac.id == bc.id)).map(|(index, bc)| JpgComponentAdded { index, item: *bc }).collect();
    JpgComponentsDiff { removed, modified, added }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_components(base: &[JpgFrameComponent], d: &JpgComponentsDiff) -> Vec<JpgFrameComponent> {
    let mut items: Vec<JpgFrameComponent> = base.iter().filter(|c| !d.removed.contains(&c.id)).copied().collect();
    for m in &d.modified {
        if let Some(item) = items.iter_mut().find(|c| c.id == m.id) {
            *item = m.diff.apply(item);
        }
    }
    let mut adds = d.added.clone();
    adds.sort_by_key(|a| a.index);
    for a in adds {
        let at = a.index.min(items.len());
        items.insert(at, a.item);
    }
    items
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_quant_tables(a: &[JpgQuantTable], b: &[JpgQuantTable]) -> JpgQuantTablesDiff {
    let mut removed = Vec::new();
    let mut modified = Vec::new();
    for at_ in a {
        match b.iter().find(|bt| bt.id == at_.id) {
            Some(bt) => {
                let d = JpgQuantTableDiff::between(at_, bt);
                if !d.is_empty() {
                    modified.push(JpgQuantTableModified { id: at_.id, diff: d });
                }
            }
            None => removed.push(at_.id),
        }
    }
    let added: Vec<JpgQuantTableAdded> = b.iter().enumerate().filter(|(_, bt)| !a.iter().any(|at_| at_.id == bt.id)).map(|(index, bt)| JpgQuantTableAdded { index, item: bt.clone() }).collect();
    JpgQuantTablesDiff { removed, modified, added }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_quant_tables(base: &[JpgQuantTable], d: &JpgQuantTablesDiff) -> Vec<JpgQuantTable> {
    let mut items: Vec<JpgQuantTable> = base.iter().filter(|t| !d.removed.contains(&t.id)).cloned().collect();
    for m in &d.modified {
        if let Some(item) = items.iter_mut().find(|t| t.id == m.id) {
            *item = m.diff.apply(item);
        }
    }
    let mut adds = d.added.clone();
    adds.sort_by_key(|a| a.index);
    for a in adds {
        let at = a.index.min(items.len());
        items.insert(at, a.item);
    }
    items
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_huffman_tables(a: &[JpgHuffmanTable], b: &[JpgHuffmanTable]) -> JpgHuffmanTablesDiff {
    let mut removed = Vec::new();
    let mut modified = Vec::new();
    for at_ in a {
        let k = huffman_key(at_);
        match b.iter().find(|bt| huffman_key(bt) == k) {
            Some(bt) => {
                let d = JpgHuffmanTableDiff::between(at_, bt);
                if !d.is_empty() {
                    modified.push(JpgHuffmanTableModified { key: k, diff: d });
                }
            }
            None => removed.push(k),
        }
    }
    let added: Vec<JpgHuffmanTableAdded> = b.iter().enumerate().filter(|(_, bt)| !a.iter().any(|at_| huffman_key(at_) == huffman_key(bt))).map(|(index, bt)| JpgHuffmanTableAdded { index, item: bt.clone() }).collect();
    JpgHuffmanTablesDiff { removed, modified, added }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_huffman_tables(base: &[JpgHuffmanTable], d: &JpgHuffmanTablesDiff) -> Vec<JpgHuffmanTable> {
    let mut items: Vec<JpgHuffmanTable> = base.iter().filter(|t| !d.removed.contains(&huffman_key(t))).cloned().collect();
    for m in &d.modified {
        if let Some(item) = items.iter_mut().find(|t| huffman_key(t) == m.key) {
            *item = m.diff.apply(item);
        }
    }
    let mut adds = d.added.clone();
    adds.sort_by_key(|a| a.index);
    for a in adds {
        let at = a.index.min(items.len());
        items.insert(at, a.item);
    }
    items
}
//#endregion 🔖️IdKeyedTransport

//#region 🔖️FrameHelpers
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_frame(base: &Option<JpgFrameHeader>, change: &JpgFrameChange) -> Option<JpgFrameHeader> {
    match change {
        JpgFrameChange::Replace { frame } => frame.clone(),
        JpgFrameChange::Modify(fd) => {
            let mut f = base.clone().unwrap_or(JpgFrameHeader { precision: 8, width: 0, height: 0, components: Vec::new() });
            if let Some(p) = fd.precision {
                f.precision = p;
            }
            if let Some(w) = fd.width {
                f.width = w;
            }
            if let Some(h) = fd.height {
                f.height = h;
            }
            if let Some(cd) = &fd.components {
                f.components = apply_components(&f.components, cd);
            }
            Some(f)
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_frame(a: &Option<JpgFrameHeader>, b: &Option<JpgFrameHeader>) -> Option<JpgFrameChange> {
    if a == b {
        return None;
    }
    match (a, b) {
        (Some(af), Some(bf)) => {
            let mut fd = JpgFrameFieldsDiff::default();
            if af.precision != bf.precision {
                fd.precision = Some(bf.precision);
            }
            if af.width != bf.width {
                fd.width = Some(bf.width);
            }
            if af.height != bf.height {
                fd.height = Some(bf.height);
            }
            let cd = between_components(&af.components, &bf.components);
            if !cd.is_empty() {
                fd.components = Some(cd);
            }
            Some(JpgFrameChange::Modify(fd))
        }
        _ => Some(JpgFrameChange::Replace { frame: b.clone() }),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_frame(base: &mut Option<JpgFrameChange>, other: Option<JpgFrameChange>) {
    let Some(other) = other else { return };
    match other {
        JpgFrameChange::Replace { .. } => {
            *base = Some(other);
        }
        JpgFrameChange::Modify(fd2) => match base.take() {
            None => {
                *base = Some(JpgFrameChange::Modify(fd2));
            }
            Some(JpgFrameChange::Replace { frame }) => {
                // 🩹 A d1 `Replace` already committed the whole new frame value; folding a d2
                // field-patch means applying it directly to that carried value (documented
                // patch-into-replace, the `Replace`-shape analogue of "patch into added").
                let patched = frame.map(|mut f| {
                    if let Some(p) = fd2.precision {
                        f.precision = p;
                    }
                    if let Some(w) = fd2.width {
                        f.width = w;
                    }
                    if let Some(h) = fd2.height {
                        f.height = h;
                    }
                    if let Some(cd) = &fd2.components {
                        f.components = apply_components(&f.components, cd);
                    }
                    f
                });
                *base = Some(JpgFrameChange::Replace { frame: patched });
            }
            Some(JpgFrameChange::Modify(mut fd1)) => {
                if fd2.precision.is_some() {
                    fd1.precision = fd2.precision;
                }
                if fd2.width.is_some() {
                    fd1.width = fd2.width;
                }
                if fd2.height.is_some() {
                    fd1.height = fd2.height;
                }
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
///   --> .../🔺️diff/🦀️.rs:738:23   (pub frame: Option<JpgFrameChange>)
/// ```
/// (2) `re_encode_quality`/`jfif_thumbnail`/`restart_interval` are tri-state `Option<Option<T>>`
/// fields — same blocker as `GifDiff`/`SvgDiff` (no `impl<T: DslField> DslField for Option<T>`
/// anywhere in `dsl`):
/// ```text
/// error[E0277]: the trait bound `std::option::Option<u8>: DslField` is not satisfied
///   --> .../🔺️diff/🦀️.rs:720:35   (pub re_encode_quality: Option<Option<u8>>)
/// error[E0277]: the trait bound `Option<JfifThumbnail>: DslField` is not satisfied
///   --> .../🔺️diff/🦀️.rs:735:32   (pub jfif_thumbnail: Option<Option<JfifThumbnail>>)
/// error[E0277]: the trait bound `std::option::Option<u16>: DslField` is not satisfied
///   --> .../🔺️diff/🦀️.rs:753:34   (pub restart_interval: Option<Option<u16>>)
/// ```
/// `DiffCodec` is hand-rolled below (`#region 🔖️HandcraftedDiffCodec`).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.jpg.diff")]
pub struct JpgDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub pixels: Option<Vec<u8>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub re_encode_quality: Option<Option<u8>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub jfif_version: Option<(u8, u8)>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub jfif_density_units: Option<JfifDensityUnits>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub jfif_x_density: Option<u16>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub jfif_y_density: Option<u16>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub jfif_thumbnail: Option<Option<JfifThumbnail>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub frame: Option<JpgFrameChange>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub sof_marker: Option<u8>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub arithmetic: Option<bool>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub quant_tables: Option<JpgQuantTablesDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub huffman_tables: Option<JpgHuffmanTablesDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub restart_interval: Option<Option<u16>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub other_segments: Option<JpgOtherSegmentsDiff>,
}

impl MutationDiff<JpgSnapshot> for JpgDiff {
    fn apply(&self, base: &JpgSnapshot) -> MutationApplyResult<JpgSnapshot> {
        validate_jpg_frame(base.frame.as_ref(), self.frame.as_ref())?;
        if let Some(quant) = &self.quant_tables {
            validate_jpg_quant_tables(&base.quant_tables, quant)?;
        }
        if let Some(huffman) = &self.huffman_tables {
            validate_jpg_huffman_tables(&base.huffman_tables, huffman)?;
        }
        if let Some(segments) = &self.other_segments {
            validate_jpg_indexed(base.other_segments.len(), &segments.removed, segments.modified.iter().map(|entry| entry.index), segments.added.iter().map(|entry| entry.index), ["otherSegments"])?;
        }
        let mut next = base.clone();
        if let Some(v) = self.width {
            next.width = v;
        }
        if let Some(v) = self.height {
            next.height = v;
        }
        if let Some(v) = &self.pixels {
            next.pixels = v.clone();
        }
        if let Some(v) = &self.re_encode_quality {
            next.re_encode_quality = *v;
        }
        if let Some(v) = self.jfif_version {
            next.jfif_version = v;
        }
        if let Some(v) = self.jfif_density_units {
            next.jfif_density_units = v;
        }
        if let Some(v) = self.jfif_x_density {
            next.jfif_x_density = v;
        }
        if let Some(v) = self.jfif_y_density {
            next.jfif_y_density = v;
        }
        if let Some(v) = &self.jfif_thumbnail {
            next.jfif_thumbnail = v.clone();
        }
        if let Some(change) = &self.frame {
            next.frame = apply_frame(&next.frame, change);
        }
        if let Some(v) = self.sof_marker {
            next.sof_marker = v;
        }
        if let Some(v) = self.arithmetic {
            next.arithmetic = v;
        }
        if let Some(qd) = &self.quant_tables {
            next.quant_tables = apply_quant_tables(&next.quant_tables, qd);
        }
        if let Some(hd) = &self.huffman_tables {
            next.huffman_tables = apply_huffman_tables(&next.huffman_tables, hd);
        }
        if let Some(v) = &self.restart_interval {
            next.restart_interval = *v;
        }
        if let Some(od) = &self.other_segments {
            next.other_segments = apply_other_segments(&next.other_segments, od);
        }
        Ok(next)
    }

    /// ➕️ Structural, total, base-free sequential-coalesce (`## Absorb` contract). Scalars
    /// (incl. every tri-state): LWW. `frame`: `absorb_frame`. `quant_tables`/`huffman_tables`:
    /// stable-key merge, no index-transport. `other_segments`: position-transported merge.
    fn absorb(&mut self, other: Self) {
        if other.width.is_some() {
            self.width = other.width;
        }
        if other.height.is_some() {
            self.height = other.height;
        }
        if other.pixels.is_some() {
            self.pixels = other.pixels;
        }
        if other.re_encode_quality.is_some() {
            self.re_encode_quality = other.re_encode_quality;
        }
        if other.jfif_version.is_some() {
            self.jfif_version = other.jfif_version;
        }
        if other.jfif_density_units.is_some() {
            self.jfif_density_units = other.jfif_density_units;
        }
        if other.jfif_x_density.is_some() {
            self.jfif_x_density = other.jfif_x_density;
        }
        if other.jfif_y_density.is_some() {
            self.jfif_y_density = other.jfif_y_density;
        }
        if other.jfif_thumbnail.is_some() {
            self.jfif_thumbnail = other.jfif_thumbnail;
        }
        absorb_frame(&mut self.frame, other.frame);
        if other.sof_marker.is_some() {
            self.sof_marker = other.sof_marker;
        }
        if other.arithmetic.is_some() {
            self.arithmetic = other.arithmetic;
        }
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
        if other.restart_interval.is_some() {
            self.restart_interval = other.restart_interval;
        }
        absorb_other_segments_opt(&mut self.other_segments, other.other_segments);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_jpg_frame(base: Option<&JpgFrameHeader>, change: Option<&JpgFrameChange>) -> MutationApplyResult<()> {
    let Some(change) = change else { return Ok(()) };
    match change {
        JpgFrameChange::Replace { .. } => Ok(()),
        JpgFrameChange::Modify(fields) => {
            let Some(base) = base else {
                return Err(MutationApplyError::new("mutation.apply.missing-target", "JPEG frame modification requires an existing frame").at(["frame"]));
            };
            let Some(components) = &fields.components else { return Ok(()) };
            let removed: std::collections::HashSet<u8> = components.removed.iter().copied().collect();
            if removed.len() != components.removed.len() || components.removed.iter().any(|id| base.components.iter().all(|component| component.id != *id)) {
                return Err(MutationApplyError::new("mutation.apply.missing-target", "JPEG frame component removal is missing or duplicated").at(["frame", "components"]));
            }
            let mut modified = std::collections::HashSet::new();
            for entry in &components.modified {
                if base.components.iter().all(|component| component.id != entry.id) || !modified.insert(entry.id) || removed.contains(&entry.id) {
                    return Err(MutationApplyError::new("mutation.apply.conflicting-target", "JPEG frame component modification is missing, duplicated, or removed").at(["frame", "components"]));
                }
            }
            let final_len = base.components.len().saturating_sub(components.removed.len()).saturating_add(components.added.len());
            let mut added_ids = std::collections::HashSet::new();
            for entry in &components.added {
                if entry.index > final_len || !added_ids.insert(entry.item.id) || base.components.iter().any(|component| component.id == entry.item.id) {
                    return Err(MutationApplyError::new("mutation.apply.duplicate-target", "JPEG frame component addition conflicts with the target state").at(["frame", "components"]));
                }
            }
            Ok(())
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_jpg_quant_tables(base: &[JpgQuantTable], diff: &JpgQuantTablesDiff) -> MutationApplyResult<()> {
    let base_ids: std::collections::HashSet<u8> = base.iter().map(|table| table.id).collect();
    let removed: std::collections::HashSet<u8> = diff.removed.iter().copied().collect();
    if removed.len() != diff.removed.len() || diff.removed.iter().any(|id| !base_ids.contains(id)) {
        return Err(MutationApplyError::new("mutation.apply.missing-target", "JPEG quantization-table removal is missing or duplicated").at(["quantTables"]));
    }
    let mut modified = std::collections::HashSet::new();
    for entry in &diff.modified {
        if !base_ids.contains(&entry.id) || !modified.insert(entry.id) || removed.contains(&entry.id) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "JPEG quantization-table modification is missing, duplicated, or removed").at(["quantTables"]));
        }
    }
    let mut added_ids = std::collections::HashSet::new();
    for entry in &diff.added {
        if base_ids.contains(&entry.item.id) || !added_ids.insert(entry.item.id) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "JPEG quantization-table addition conflicts with the target state").at(["quantTables", "added"]));
        }
    }
    validate_jpg_additions(base.len(), diff.removed.len(), diff.added.iter().map(|entry| entry.index), ["quantTables", "added"])
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_jpg_huffman_tables(base: &[JpgHuffmanTable], diff: &JpgHuffmanTablesDiff) -> MutationApplyResult<()> {
    let base_keys: std::collections::HashSet<JpgHuffmanTableKey> = base.iter().map(huffman_key).collect();
    let removed: std::collections::HashSet<JpgHuffmanTableKey> = diff.removed.iter().copied().collect();
    if removed.len() != diff.removed.len() || diff.removed.iter().any(|key| !base_keys.contains(key)) {
        return Err(MutationApplyError::new("mutation.apply.missing-target", "JPEG Huffman-table removal is missing or duplicated").at(["huffmanTables"]));
    }
    let mut modified = std::collections::HashSet::new();
    for entry in &diff.modified {
        if !base_keys.contains(&entry.key) || !modified.insert(entry.key) || removed.contains(&entry.key) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "JPEG Huffman-table modification is missing, duplicated, or removed").at(["huffmanTables"]));
        }
    }
    let mut added_keys = std::collections::HashSet::new();
    for entry in &diff.added {
        let key = huffman_key(&entry.item);
        if base_keys.contains(&key) || !added_keys.insert(key) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "JPEG Huffman-table addition conflicts with the target state").at(["huffmanTables", "added"]));
        }
    }
    validate_jpg_additions(base.len(), diff.removed.len(), diff.added.iter().map(|entry| entry.index), ["huffmanTables", "added"])
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_jpg_additions<I, K>(base_len: usize, removed_len: usize, added: I, path: K) -> MutationApplyResult<()>
where
    I: IntoIterator<Item = usize>,
    K: IntoIterator,
    K::Item: AsRef<str>,
{
    let path: Vec<String> = path.into_iter().map(|part| part.as_ref().to_owned()).collect();
    let added: Vec<usize> = added.into_iter().collect();
    let final_len = base_len.saturating_sub(removed_len).saturating_add(added.len());
    let mut added_set = std::collections::HashSet::new();
    for index in added {
        if index > final_len || !added_set.insert(index) {
            return Err(MutationApplyError::new("mutation.apply.invalid-index", "JPEG collection addition index is invalid or duplicated").at(path.iter().map(String::as_str)));
        }
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_jpg_indexed<I, J, K>(base_len: usize, removed: &[usize], modified: I, added: J, path: K) -> MutationApplyResult<()>
where
    I: IntoIterator<Item = usize>,
    J: IntoIterator<Item = usize>,
    K: IntoIterator,
    K::Item: AsRef<str>,
{
    let path: Vec<String> = path.into_iter().map(|part| part.as_ref().to_owned()).collect();
    let mut removed_set = std::collections::HashSet::new();
    for &index in removed {
        if index >= base_len || !removed_set.insert(index) {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "JPEG collection removal is missing or duplicated").at(path.iter().map(String::as_str)));
        }
    }
    let mut modified_set = std::collections::HashSet::new();
    for index in modified {
        if index >= base_len || !modified_set.insert(index) || removed_set.contains(&index) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "JPEG collection modification is missing, duplicated, or removed").at(path.iter().map(String::as_str)));
        }
    }
    let added: Vec<usize> = added.into_iter().collect();
    let final_len = base_len.saturating_sub(removed.len()).saturating_add(added.len());
    let mut added_set = std::collections::HashSet::new();
    for index in added {
        if index > final_len || !added_set.insert(index) {
            return Err(MutationApplyError::new("mutation.apply.invalid-index", "JPEG collection addition index is invalid or duplicated").at(path.iter().map(String::as_str)));
        }
    }
    Ok(())
}

impl DiffAlgebra<JpgSnapshot> for JpgDiff {
    /// 🔁️ Diff-level undo, derived generically (correct by construction) exactly like zip's/
    /// png's: the state delta from `self.apply(base)` back to `base`.
    fn inverse(&self, base: &JpgSnapshot) -> Self {
        let mutated = self.apply(base).unwrap();
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

    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

/// 🧩 Builds a set-snapshot diff (sparse field-by-field delta, never a full-replace slot).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_snapshot(base: &JpgSnapshot, next: &JpgSnapshot) -> JpgDiff {
    JpgDiff::between(base, next)
}
//#endregion 🔖️Diff

//#region 🔖️MutationDiffBuilders
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9


// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9


// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9


// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9


// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9


// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9


// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9


// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9


// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9


// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9

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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn strip_brackets(s: &str) -> Result<&str, String> {
    s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))
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
pub(crate) fn parse_u8(s: &str) -> Result<u8, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_u16(s: &str) -> Result<u16, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_u32(s: &str) -> Result<u32, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_usize(s: &str) -> Result<usize, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_bool(s: &str) -> Result<bool, String> {
    match s {
        "0" => Ok(false),
        "1" => Ok(true),
        other => Err(format!("bad bool {other:?}")),
    }
}
//#endregion 🔖️Primitives

//#region 🔖️BinaryPrimitives
/// 🧪️ P2-FG2: real LEB128-varint-framed binary primitives backing the upgraded `DiffCodec`
/// (below) and, via `pub(crate)`, `../🧬️mutations/🦀️.rs`'s upgraded `OpBinary` — mirrors
/// `📰️xml`'s own `write_bytes_lp`/`read_bytes_lp` shape (`📖️grammar-recipe.md` §2.5), reusing
/// `store::pack_rt::write_varint_u64`/`store::ByteReader` rather than reinventing varint codecs.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}
/// 🏳️ Generic `Option<T>` presence-byte codec (`0`/`1` + payload) — the binary twin of the text
/// side's `encode_option`/`decode_option`, used for every tri-state/plain-optional field below.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_opt<T>(out: &mut Vec<u8>, opt: &Option<T>, enc: impl FnOnce(&T, &mut Vec<u8>)) {
    out.push(if opt.is_some() { 1 } else { 0 });
    if let Some(v) = opt {
        enc(v, out);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_opt<T>(reader: &mut store::ByteReader<'_>, dec: impl FnOnce(&mut store::ByteReader<'_>) -> Result<T, String>) -> Result<Option<T>, String> {
    let has = reader.read_u8().map_err(|e| e.to_string())?;
    if has != 0 {
        Ok(Some(dec(reader)?))
    } else {
        Ok(None)
    }
}
//#endregion 🔖️BinaryPrimitives

//#region 🔖️ValueBinaryCodecs
/// 🧪️ P2-FG2: real recursive-free binary twins of `§ValueCodecs` above — every type here is a
/// bounded, non-recursive record/collection (unlike xml's self-recursive `XmlNode`), so every
/// field is genuinely, individually written/read; no opaque payload anywhere in this region.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_version_bin(v: &(u8, u8), out: &mut Vec<u8>) {
    out.push(v.0);
    out.push(v.1);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_version_bin(reader: &mut store::ByteReader<'_>) -> Result<(u8, u8), String> {
    Ok((reader.read_u8().map_err(|e| e.to_string())?, reader.read_u8().map_err(|e| e.to_string())?))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_density_units_bin(u: &JfifDensityUnits, out: &mut Vec<u8>) {
    out.push(u.to_u8());
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_density_units_bin(reader: &mut store::ByteReader<'_>) -> Result<JfifDensityUnits, String> {
    JfifDensityUnits::from_u8(reader.read_u8().map_err(|e| e.to_string())?)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_huffman_class_bin(c: &JpgHuffmanClass, out: &mut Vec<u8>) {
    out.push(c.to_u8());
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_huffman_class_bin(reader: &mut store::ByteReader<'_>) -> Result<JpgHuffmanClass, String> {
    JpgHuffmanClass::from_u8(reader.read_u8().map_err(|e| e.to_string())?)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_thumbnail_bin(t: &JfifThumbnail, out: &mut Vec<u8>) {
    out.push(t.width);
    out.push(t.height);
    write_bytes_lp(out, &t.rgb_data);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_thumbnail_bin(reader: &mut store::ByteReader<'_>) -> Result<JfifThumbnail, String> {
    let width = reader.read_u8().map_err(|e| e.to_string())?;
    let height = reader.read_u8().map_err(|e| e.to_string())?;
    let rgb_data = read_bytes_lp(reader)?;
    Ok(JfifThumbnail { width, height, rgb_data })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_frame_component_bin(c: &JpgFrameComponent, out: &mut Vec<u8>) {
    out.push(c.id);
    out.push(c.h_sampling);
    out.push(c.v_sampling);
    out.push(c.quant_table_id);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_frame_component_bin(reader: &mut store::ByteReader<'_>) -> Result<JpgFrameComponent, String> {
    Ok(JpgFrameComponent {
        id: reader.read_u8().map_err(|e| e.to_string())?,
        h_sampling: reader.read_u8().map_err(|e| e.to_string())?,
        v_sampling: reader.read_u8().map_err(|e| e.to_string())?,
        quant_table_id: reader.read_u8().map_err(|e| e.to_string())?,
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_frame_header_bin(f: &JpgFrameHeader, out: &mut Vec<u8>) {
    out.push(f.precision);
    store::pack_rt::write_varint_u64(out, f.width as u64);
    store::pack_rt::write_varint_u64(out, f.height as u64);
    store::pack_rt::write_varint_u64(out, f.components.len() as u64);
    for c in &f.components {
        enc_frame_component_bin(c, out);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_frame_header_bin(reader: &mut store::ByteReader<'_>) -> Result<JpgFrameHeader, String> {
    let precision = reader.read_u8().map_err(|e| e.to_string())?;
    let width = reader.read_varint_u64().map_err(|e| e.to_string())? as u16;
    let height = reader.read_varint_u64().map_err(|e| e.to_string())? as u16;
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut components = Vec::with_capacity(count as usize);
    for _ in 0..count {
        components.push(dec_frame_component_bin(reader)?);
    }
    Ok(JpgFrameHeader { precision, width, height, components })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_quant_table_bin(t: &JpgQuantTable, out: &mut Vec<u8>) {
    out.push(t.id);
    out.push(t.precision);
    for v in t.values.iter() {
        out.extend_from_slice(&v.to_le_bytes());
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_quant_table_bin(reader: &mut store::ByteReader<'_>) -> Result<JpgQuantTable, String> {
    let id = reader.read_u8().map_err(|e| e.to_string())?;
    let precision = reader.read_u8().map_err(|e| e.to_string())?;
    let mut values = [0u16; 64];
    for v in values.iter_mut() {
        *v = reader.read_u16_le().map_err(|e| e.to_string())?;
    }
    Ok(JpgQuantTable { id, precision, values })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_huffman_table_bin(t: &JpgHuffmanTable, out: &mut Vec<u8>) {
    out.push(t.id);
    enc_huffman_class_bin(&t.class, out);
    out.extend_from_slice(&t.bits);
    write_bytes_lp(out, &t.values);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_huffman_table_bin(reader: &mut store::ByteReader<'_>) -> Result<JpgHuffmanTable, String> {
    let id = reader.read_u8().map_err(|e| e.to_string())?;
    let class = dec_huffman_class_bin(reader)?;
    let bits_vec = reader.read_bytes(16).map_err(|e| e.to_string())?.to_vec();
    let bits: [u8; 16] = bits_vec.try_into().map_err(|_| "huffman bits: expected 16 bytes".to_string())?;
    let values = read_bytes_lp(reader)?;
    Ok(JpgHuffmanTable { id, class, bits, values })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_huffman_key_bin(k: &JpgHuffmanTableKey, out: &mut Vec<u8>) {
    enc_huffman_class_bin(&k.class, out);
    out.push(k.id);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_huffman_key_bin(reader: &mut store::ByteReader<'_>) -> Result<JpgHuffmanTableKey, String> {
    let class = dec_huffman_class_bin(reader)?;
    let id = reader.read_u8().map_err(|e| e.to_string())?;
    Ok(JpgHuffmanTableKey { class, id })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_segment_bin(s: &JpgSegment, out: &mut Vec<u8>) {
    out.push(s.marker);
    write_bytes_lp(out, &s.data);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_segment_bin(reader: &mut store::ByteReader<'_>) -> Result<JpgSegment, String> {
    let marker = reader.read_u8().map_err(|e| e.to_string())?;
    let data = read_bytes_lp(reader)?;
    Ok(JpgSegment { marker, data })
}
//#endregion 🔖️ValueBinaryCodecs

//#region 🔖️ValueCodecs
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_density_units(u: &JfifDensityUnits) -> String {
    u.to_u8().to_string()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_density_units(s: &str) -> Result<JfifDensityUnits, String> {
    JfifDensityUnits::from_u8(parse_u8(s)?)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_huffman_class(c: &JpgHuffmanClass) -> String {
    c.to_u8().to_string()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_huffman_class(s: &str) -> Result<JpgHuffmanClass, String> {
    JpgHuffmanClass::from_u8(parse_u8(s)?)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_version(v: &(u8, u8)) -> String {
    format!("[{},{}]", v.0, v.1)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_version(s: &str) -> Result<(u8, u8), String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [a, b] = parts.as_slice() else { return Err(format!("jfif version: expected 2 fields, got {}", parts.len())) };
    Ok((parse_u8(a)?, parse_u8(b)?))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_quant_values(v: &[u16; 64]) -> String {
    format!("[{}]", v.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_quant_values(s: &str) -> Result<[u16; 64], String> {
    let values: Vec<u16> = split_top_level(strip_brackets(s)?, ',').into_iter().map(parse_u16).collect::<Result<_, _>>()?;
    <[u16; 64]>::try_from(values).map_err(|v: Vec<u16>| format!("quant values: expected 64, got {}", v.len()))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_bits16(b: &[u8; 16]) -> String {
    hex_encode(b)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_bits16(s: &str) -> Result<[u8; 16], String> {
    <[u8; 16]>::try_from(hex_decode(s)?).map_err(|v: Vec<u8>| format!("huffman bits: expected 16, got {}", v.len()))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_thumbnail(t: &JfifThumbnail) -> String {
    format!("[{},{},{}]", t.width, t.height, hex_encode(&t.rgb_data))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_thumbnail(s: &str) -> Result<JfifThumbnail, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [width, height, rgb_data] = parts.as_slice() else { return Err(format!("thumbnail: expected 3 fields, got {}", parts.len())) };
    Ok(JfifThumbnail { width: parse_u8(width)?, height: parse_u8(height)?, rgb_data: hex_decode(rgb_data)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_frame_component(c: &JpgFrameComponent) -> String {
    format!("[{},{},{},{}]", c.id, c.h_sampling, c.v_sampling, c.quant_table_id)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_frame_component(s: &str) -> Result<JpgFrameComponent, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, h, v, q] = parts.as_slice() else { return Err(format!("frame component: expected 4 fields, got {}", parts.len())) };
    Ok(JpgFrameComponent { id: parse_u8(id)?, h_sampling: parse_u8(h)?, v_sampling: parse_u8(v)?, quant_table_id: parse_u8(q)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_frame_header(f: &JpgFrameHeader) -> String {
    let comps = f.components.iter().map(enc_frame_component).collect::<Vec<_>>().join(",");
    format!("[{},{},{},[{}]]", f.precision, f.width, f.height, comps)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_frame_header(s: &str) -> Result<JpgFrameHeader, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [precision, width, height, components] = parts.as_slice() else { return Err(format!("frame header: expected 4 fields, got {}", parts.len())) };
    let components = split_top_level(strip_brackets(components)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_frame_component).collect::<Result<Vec<_>, String>>()?;
    Ok(JpgFrameHeader { precision: parse_u8(precision)?, width: parse_u16(width)?, height: parse_u16(height)?, components })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_quant_table(t: &JpgQuantTable) -> String {
    format!("[{},{},{}]", t.id, t.precision, enc_quant_values(&t.values))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_quant_table(s: &str) -> Result<JpgQuantTable, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, precision, values] = parts.as_slice() else { return Err(format!("quant table: expected 3 fields, got {}", parts.len())) };
    Ok(JpgQuantTable { id: parse_u8(id)?, precision: parse_u8(precision)?, values: dec_quant_values(values)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_huffman_table(t: &JpgHuffmanTable) -> String {
    format!("[{},{},{},{}]", t.id, enc_huffman_class(&t.class), enc_bits16(&t.bits), hex_encode(&t.values))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_huffman_table(s: &str) -> Result<JpgHuffmanTable, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, class, bits, values] = parts.as_slice() else { return Err(format!("huffman table: expected 4 fields, got {}", parts.len())) };
    Ok(JpgHuffmanTable { id: parse_u8(id)?, class: dec_huffman_class(class)?, bits: dec_bits16(bits)?, values: hex_decode(values)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_huffman_key(k: &JpgHuffmanTableKey) -> String {
    format!("[{},{}]", enc_huffman_class(&k.class), k.id)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_huffman_key(s: &str) -> Result<JpgHuffmanTableKey, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [class, id] = parts.as_slice() else { return Err(format!("huffman key: expected 2 fields, got {}", parts.len())) };
    Ok(JpgHuffmanTableKey { class: dec_huffman_class(class)?, id: parse_u8(id)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_segment(s: &JpgSegment) -> String {
    format!("[{},{}]", s.marker, hex_encode(&s.data))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_segment(s: &str) -> Result<JpgSegment, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [marker, data] = parts.as_slice() else { return Err(format!("segment: expected 2 fields, got {}", parts.len())) };
    Ok(JpgSegment { marker: parse_u8(marker)?, data: hex_decode(data)? })
}
//#endregion 🔖️ValueCodecs

//#region 🔖️DiffValueCodecs
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_component_diff(d: &JpgComponentDiff) -> String {
    format!("[{},{},{}]", encode_option(&d.h_sampling, |v| v.to_string()), encode_option(&d.v_sampling, |v| v.to_string()), encode_option(&d.quant_table_id, |v| v.to_string()),)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_component_diff(s: &str) -> Result<JpgComponentDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [h, v, q] = parts.as_slice() else { return Err(format!("component diff: expected 3 fields, got {}", parts.len())) };
    Ok(JpgComponentDiff { h_sampling: decode_option(h, parse_u8)?, v_sampling: decode_option(v, parse_u8)?, quant_table_id: decode_option(q, parse_u8)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_components_diff(d: &JpgComponentsDiff) -> String {
    let removed = d.removed.iter().map(|id| id.to_string()).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:{}", m.id, enc_component_diff(&m.diff))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}", a.index, enc_frame_component(&a.item))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_components_diff(body: &str) -> Result<JpgComponentsDiff, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("components diff: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_u8).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (id, rest) = entry.split_once(':').ok_or_else(|| format!("component modified: bad entry {entry:?}"))?;
            Ok(JpgComponentModified { id: parse_u8(id)?, diff: dec_component_diff(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (index, rest) = entry.split_once(':').ok_or_else(|| format!("component added: bad entry {entry:?}"))?;
            Ok(JpgComponentAdded { index: parse_usize(index)?, item: dec_frame_component(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(JpgComponentsDiff { removed, modified, added })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_quant_table_diff(d: &JpgQuantTableDiff) -> String {
    format!("[{},{}]", encode_option(&d.precision, |v| v.to_string()), encode_option(&d.values, enc_quant_values))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_quant_table_diff(s: &str) -> Result<JpgQuantTableDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [precision, values] = parts.as_slice() else { return Err(format!("quant table diff: expected 2 fields, got {}", parts.len())) };
    Ok(JpgQuantTableDiff { precision: decode_option(precision, parse_u8)?, values: decode_option(values, dec_quant_values)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_quant_tables_diff(d: &JpgQuantTablesDiff) -> String {
    let removed = d.removed.iter().map(|id| id.to_string()).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:{}", m.id, enc_quant_table_diff(&m.diff))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}", a.index, enc_quant_table(&a.item))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_quant_tables_diff(body: &str) -> Result<JpgQuantTablesDiff, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("quant tables diff: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_u8).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (id, rest) = entry.split_once(':').ok_or_else(|| format!("quant table modified: bad entry {entry:?}"))?;
            Ok(JpgQuantTableModified { id: parse_u8(id)?, diff: dec_quant_table_diff(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (index, rest) = entry.split_once(':').ok_or_else(|| format!("quant table added: bad entry {entry:?}"))?;
            Ok(JpgQuantTableAdded { index: parse_usize(index)?, item: dec_quant_table(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(JpgQuantTablesDiff { removed, modified, added })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_huffman_table_diff(d: &JpgHuffmanTableDiff) -> String {
    format!("[{},{}]", encode_option(&d.bits, enc_bits16), encode_option(&d.values, |v| hex_encode(v)))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_huffman_table_diff(s: &str) -> Result<JpgHuffmanTableDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [bits, values] = parts.as_slice() else { return Err(format!("huffman table diff: expected 2 fields, got {}", parts.len())) };
    Ok(JpgHuffmanTableDiff { bits: decode_option(bits, dec_bits16)?, values: decode_option(values, hex_decode)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_huffman_tables_diff(d: &JpgHuffmanTablesDiff) -> String {
    let removed = d.removed.iter().map(enc_huffman_key).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:{}", enc_huffman_key(&m.key), enc_huffman_table_diff(&m.diff))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}", a.index, enc_huffman_table(&a.item))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_huffman_tables_diff(body: &str) -> Result<JpgHuffmanTablesDiff, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("huffman tables diff: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_huffman_key).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (key, rest) = entry.split_once(':').ok_or_else(|| format!("huffman table modified: bad entry {entry:?}"))?;
            Ok(JpgHuffmanTableModified { key: dec_huffman_key(key)?, diff: dec_huffman_table_diff(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (index, rest) = entry.split_once(':').ok_or_else(|| format!("huffman table added: bad entry {entry:?}"))?;
            Ok(JpgHuffmanTableAdded { index: parse_usize(index)?, item: dec_huffman_table(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(JpgHuffmanTablesDiff { removed, modified, added })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_segment_diff(d: &JpgSegmentDiff) -> String {
    format!("[{},{}]", encode_option(&d.marker, |v| v.to_string()), encode_option(&d.data, |v| hex_encode(v)))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_segment_diff(s: &str) -> Result<JpgSegmentDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [marker, data] = parts.as_slice() else { return Err(format!("segment diff: expected 2 fields, got {}", parts.len())) };
    Ok(JpgSegmentDiff { marker: decode_option(marker, parse_u8)?, data: decode_option(data, hex_decode)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_other_segments_diff(d: &JpgOtherSegmentsDiff) -> String {
    let removed = d.removed.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:{}", m.index, enc_segment_diff(&m.diff))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}", a.index, enc_segment(&a.item))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_other_segments_diff(body: &str) -> Result<JpgOtherSegmentsDiff, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("other segments diff: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_usize).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (index, rest) = entry.split_once(':').ok_or_else(|| format!("segment modified: bad entry {entry:?}"))?;
            Ok(JpgSegmentModified { index: parse_usize(index)?, diff: dec_segment_diff(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (index, rest) = entry.split_once(':').ok_or_else(|| format!("segment added: bad entry {entry:?}"))?;
            Ok(JpgSegmentAdded { index: parse_usize(index)?, item: dec_segment(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(JpgOtherSegmentsDiff { removed, modified, added })
}

/// 🌲 `JpgFrameChange`'s tag prefix: `M[fields-diff]` (Modify) / `R[frame-opt]` (Replace) — mirrors
/// `enc_xml_node`/`enc_node_diff`'s single-letter-tag convention (svg/gif precedent).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_frame_change(fc: &JpgFrameChange) -> String {
    match fc {
        JpgFrameChange::Modify(fd) => format!("M[{}]", enc_frame_fields_diff(fd)),
        JpgFrameChange::Replace { frame } => format!("R[{}]", encode_option(frame, enc_frame_header)),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_frame_change(s: &str) -> Result<JpgFrameChange, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "M" => Ok(JpgFrameChange::Modify(dec_frame_fields_diff(inner)?)),
        "R" => Ok(JpgFrameChange::Replace { frame: decode_option(inner, dec_frame_header)? }),
        other => Err(format!("frame change: unknown tag {other:?}")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_frame_fields_diff(fd: &JpgFrameFieldsDiff) -> String {
    format!("[{},{},{},{}]", encode_option(&fd.precision, |v| v.to_string()), encode_option(&fd.width, |v| v.to_string()), encode_option(&fd.height, |v| v.to_string()), encode_option(&fd.components, enc_components_diff),)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_frame_fields_diff(s: &str) -> Result<JpgFrameFieldsDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [precision, width, height, components] = parts.as_slice() else { return Err(format!("frame fields diff: expected 4 fields, got {}", parts.len())) };
    Ok(JpgFrameFieldsDiff { precision: decode_option(precision, parse_u8)?, width: decode_option(width, parse_u16)?, height: decode_option(height, parse_u16)?, components: decode_option(components, dec_components_diff)? })
}
//#endregion 🔖️DiffValueCodecs

//#region 🔖️DiffValueBinaryCodecs
/// 🧪️ P2-FG2: real binary twins of `§DiffValueCodecs` above — every collection triple below is
/// `varint-count + real-item` (removed/modified/added, matching the recipe's §1.4 shape but
/// binary-framed rather than bracket-text-framed); no opaque payload anywhere in this region since
/// none of jpg's diff types are self-recursive (unlike xml's `XmlNodeDiff`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_component_diff_bin(d: &JpgComponentDiff, out: &mut Vec<u8>) {
    write_opt(out, &d.h_sampling, |v, out| out.push(*v));
    write_opt(out, &d.v_sampling, |v, out| out.push(*v));
    write_opt(out, &d.quant_table_id, |v, out| out.push(*v));
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_component_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<JpgComponentDiff, String> {
    Ok(JpgComponentDiff {
        h_sampling: read_opt(reader, |r| r.read_u8().map_err(|e| e.to_string()))?,
        v_sampling: read_opt(reader, |r| r.read_u8().map_err(|e| e.to_string()))?,
        quant_table_id: read_opt(reader, |r| r.read_u8().map_err(|e| e.to_string()))?,
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_components_diff_bin(d: &JpgComponentsDiff, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, d.removed.len() as u64);
    for id in &d.removed {
        out.push(*id);
    }
    store::pack_rt::write_varint_u64(out, d.modified.len() as u64);
    for m in &d.modified {
        out.push(m.id);
        enc_component_diff_bin(&m.diff, out);
    }
    store::pack_rt::write_varint_u64(out, d.added.len() as u64);
    for a in &d.added {
        store::pack_rt::write_varint_u64(out, a.index as u64);
        enc_frame_component_bin(&a.item, out);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_components_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<JpgComponentsDiff, String> {
    let rc = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut removed = Vec::with_capacity(rc as usize);
    for _ in 0..rc {
        removed.push(reader.read_u8().map_err(|e| e.to_string())?);
    }
    let mc = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(mc as usize);
    for _ in 0..mc {
        let id = reader.read_u8().map_err(|e| e.to_string())?;
        modified.push(JpgComponentModified { id, diff: dec_component_diff_bin(reader)? });
    }
    let ac = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(ac as usize);
    for _ in 0..ac {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        added.push(JpgComponentAdded { index, item: dec_frame_component_bin(reader)? });
    }
    Ok(JpgComponentsDiff { removed, modified, added })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_quant_table_diff_bin(d: &JpgQuantTableDiff, out: &mut Vec<u8>) {
    write_opt(out, &d.precision, |v, out| out.push(*v));
    write_opt(out, &d.values, |v, out| {
        for x in v.iter() {
            out.extend_from_slice(&x.to_le_bytes());
        }
    });
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_quant_table_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<JpgQuantTableDiff, String> {
    Ok(JpgQuantTableDiff {
        precision: read_opt(reader, |r| r.read_u8().map_err(|e| e.to_string()))?,
        values: read_opt(reader, |r| {
            let mut values = [0u16; 64];
            for v in values.iter_mut() {
                *v = r.read_u16_le().map_err(|e| e.to_string())?;
            }
            Ok(values)
        })?,
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_quant_tables_diff_bin(d: &JpgQuantTablesDiff, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, d.removed.len() as u64);
    for id in &d.removed {
        out.push(*id);
    }
    store::pack_rt::write_varint_u64(out, d.modified.len() as u64);
    for m in &d.modified {
        out.push(m.id);
        enc_quant_table_diff_bin(&m.diff, out);
    }
    store::pack_rt::write_varint_u64(out, d.added.len() as u64);
    for a in &d.added {
        store::pack_rt::write_varint_u64(out, a.index as u64);
        enc_quant_table_bin(&a.item, out);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_quant_tables_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<JpgQuantTablesDiff, String> {
    let rc = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut removed = Vec::with_capacity(rc as usize);
    for _ in 0..rc {
        removed.push(reader.read_u8().map_err(|e| e.to_string())?);
    }
    let mc = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(mc as usize);
    for _ in 0..mc {
        let id = reader.read_u8().map_err(|e| e.to_string())?;
        modified.push(JpgQuantTableModified { id, diff: dec_quant_table_diff_bin(reader)? });
    }
    let ac = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(ac as usize);
    for _ in 0..ac {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        added.push(JpgQuantTableAdded { index, item: dec_quant_table_bin(reader)? });
    }
    Ok(JpgQuantTablesDiff { removed, modified, added })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_huffman_table_diff_bin(d: &JpgHuffmanTableDiff, out: &mut Vec<u8>) {
    write_opt(out, &d.bits, |v, out| out.extend_from_slice(v));
    write_opt(out, &d.values, |v, out| write_bytes_lp(out, v));
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_huffman_table_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<JpgHuffmanTableDiff, String> {
    Ok(JpgHuffmanTableDiff {
        bits: read_opt(reader, |r| {
            let v = r.read_bytes(16).map_err(|e| e.to_string())?.to_vec();
            v.try_into().map_err(|_| "huffman table diff bits: expected 16 bytes".to_string())
        })?,
        values: read_opt(reader, read_bytes_lp)?,
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_huffman_tables_diff_bin(d: &JpgHuffmanTablesDiff, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, d.removed.len() as u64);
    for k in &d.removed {
        enc_huffman_key_bin(k, out);
    }
    store::pack_rt::write_varint_u64(out, d.modified.len() as u64);
    for m in &d.modified {
        enc_huffman_key_bin(&m.key, out);
        enc_huffman_table_diff_bin(&m.diff, out);
    }
    store::pack_rt::write_varint_u64(out, d.added.len() as u64);
    for a in &d.added {
        store::pack_rt::write_varint_u64(out, a.index as u64);
        enc_huffman_table_bin(&a.item, out);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_huffman_tables_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<JpgHuffmanTablesDiff, String> {
    let rc = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut removed = Vec::with_capacity(rc as usize);
    for _ in 0..rc {
        removed.push(dec_huffman_key_bin(reader)?);
    }
    let mc = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(mc as usize);
    for _ in 0..mc {
        let key = dec_huffman_key_bin(reader)?;
        modified.push(JpgHuffmanTableModified { key, diff: dec_huffman_table_diff_bin(reader)? });
    }
    let ac = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(ac as usize);
    for _ in 0..ac {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        added.push(JpgHuffmanTableAdded { index, item: dec_huffman_table_bin(reader)? });
    }
    Ok(JpgHuffmanTablesDiff { removed, modified, added })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_segment_diff_bin(d: &JpgSegmentDiff, out: &mut Vec<u8>) {
    write_opt(out, &d.marker, |v, out| out.push(*v));
    write_opt(out, &d.data, |v, out| write_bytes_lp(out, v));
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_segment_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<JpgSegmentDiff, String> {
    Ok(JpgSegmentDiff { marker: read_opt(reader, |r| r.read_u8().map_err(|e| e.to_string()))?, data: read_opt(reader, read_bytes_lp)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_other_segments_diff_bin(d: &JpgOtherSegmentsDiff, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, d.removed.len() as u64);
    for i in &d.removed {
        store::pack_rt::write_varint_u64(out, *i as u64);
    }
    store::pack_rt::write_varint_u64(out, d.modified.len() as u64);
    for m in &d.modified {
        store::pack_rt::write_varint_u64(out, m.index as u64);
        enc_segment_diff_bin(&m.diff, out);
    }
    store::pack_rt::write_varint_u64(out, d.added.len() as u64);
    for a in &d.added {
        store::pack_rt::write_varint_u64(out, a.index as u64);
        enc_segment_bin(&a.item, out);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_other_segments_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<JpgOtherSegmentsDiff, String> {
    let rc = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut removed = Vec::with_capacity(rc as usize);
    for _ in 0..rc {
        removed.push(reader.read_varint_u64().map_err(|e| e.to_string())? as usize);
    }
    let mc = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(mc as usize);
    for _ in 0..mc {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        modified.push(JpgSegmentModified { index, diff: dec_segment_diff_bin(reader)? });
    }
    let ac = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(ac as usize);
    for _ in 0..ac {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        added.push(JpgSegmentAdded { index, item: dec_segment_bin(reader)? });
    }
    Ok(JpgOtherSegmentsDiff { removed, modified, added })
}

/// 🌲 `JpgFrameChange`'s binary tag: `0`=Modify(frame-fields-diff) / `1`=Replace(opt-frame-header)
/// — same tag numbering convention as `xml`'s `enc_node_diff_bin` (0/1/2 by declaration order).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_frame_change_bin(fc: &JpgFrameChange, out: &mut Vec<u8>) {
    match fc {
        JpgFrameChange::Modify(fd) => {
            out.push(0);
            enc_frame_fields_diff_bin(fd, out);
        }
        JpgFrameChange::Replace { frame } => {
            out.push(1);
            write_opt(out, frame, |f, out| enc_frame_header_bin(f, out));
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_frame_change_bin(reader: &mut store::ByteReader<'_>) -> Result<JpgFrameChange, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => Ok(JpgFrameChange::Modify(dec_frame_fields_diff_bin(reader)?)),
        1 => Ok(JpgFrameChange::Replace { frame: read_opt(reader, dec_frame_header_bin)? }),
        other => Err(format!("frame change binary: unknown tag {other}")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_frame_fields_diff_bin(fd: &JpgFrameFieldsDiff, out: &mut Vec<u8>) {
    write_opt(out, &fd.precision, |v, out| out.push(*v));
    write_opt(out, &fd.width, |v, out| store::pack_rt::write_varint_u64(out, *v as u64));
    write_opt(out, &fd.height, |v, out| store::pack_rt::write_varint_u64(out, *v as u64));
    write_opt(out, &fd.components, enc_components_diff_bin);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_frame_fields_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<JpgFrameFieldsDiff, String> {
    Ok(JpgFrameFieldsDiff {
        precision: read_opt(reader, |r| r.read_u8().map_err(|e| e.to_string()))?,
        width: read_opt(reader, |r| Ok(r.read_varint_u64().map_err(|e| e.to_string())? as u16))?,
        height: read_opt(reader, |r| Ok(r.read_varint_u64().map_err(|e| e.to_string())? as u16))?,
        components: read_opt(reader, dec_components_diff_bin)?,
    })
}
//#endregion 🔖️DiffValueBinaryCodecs

//#region 🔖️TopLevel
/// 🧾 Top-level line: space-separated `name=value` tokens, one per changed field, absent token =
/// unchanged (recipe convention). Tri-state fields (`re-encode-quality`/`jfif-thumbnail`/
/// `restart-interval`) additionally wrap their value in `[0]`/`[1,x]` since the token's presence
/// alone only means "the tri-state slot changed", not which of {cleared, set} it changed to.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_jpg_diff(d: &JpgDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = d.width {
        tokens.push(format!("width={v}"));
    }
    if let Some(v) = d.height {
        tokens.push(format!("height={v}"));
    }
    if let Some(v) = &d.pixels {
        tokens.push(format!("pixels={}", hex_encode(v)));
    }
    if let Some(v) = &d.re_encode_quality {
        tokens.push(format!("re-encode-quality={}", encode_option(v, |q| q.to_string())));
    }
    if let Some(v) = d.jfif_version {
        tokens.push(format!("jfif-version={}", enc_version(&v)));
    }
    if let Some(v) = d.jfif_density_units {
        tokens.push(format!("jfif-density-units={}", enc_density_units(&v)));
    }
    if let Some(v) = d.jfif_x_density {
        tokens.push(format!("jfif-x-density={v}"));
    }
    if let Some(v) = d.jfif_y_density {
        tokens.push(format!("jfif-y-density={v}"));
    }
    if let Some(v) = &d.jfif_thumbnail {
        tokens.push(format!("jfif-thumbnail={}", encode_option(v, enc_thumbnail)));
    }
    if let Some(v) = &d.frame {
        tokens.push(format!("frame={}", enc_frame_change(v)));
    }
    if let Some(v) = d.sof_marker {
        tokens.push(format!("sof-marker={v}"));
    }
    if let Some(v) = d.arithmetic {
        tokens.push(format!("arithmetic={}", if v { 1 } else { 0 }));
    }
    if let Some(v) = &d.quant_tables {
        tokens.push(format!("quant-tables={}", enc_quant_tables_diff(v)));
    }
    if let Some(v) = &d.huffman_tables {
        tokens.push(format!("huffman-tables={}", enc_huffman_tables_diff(v)));
    }
    if let Some(v) = &d.restart_interval {
        tokens.push(format!("restart-interval={}", encode_option(v, |ri| ri.to_string())));
    }
    if let Some(v) = &d.other_segments {
        tokens.push(format!("other-segments={}", enc_other_segments_diff(v)));
    }
    tokens.join(" ")
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_jpg_diff(line: &str) -> Result<JpgDiff, String> {
    let mut d = JpgDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("width=") {
            d.width = Some(parse_u32(rest)?);
        } else if let Some(rest) = token.strip_prefix("height=") {
            d.height = Some(parse_u32(rest)?);
        } else if let Some(rest) = token.strip_prefix("pixels=") {
            d.pixels = Some(hex_decode(rest)?);
        } else if let Some(rest) = token.strip_prefix("re-encode-quality=") {
            d.re_encode_quality = Some(decode_option(rest, parse_u8)?);
        } else if let Some(rest) = token.strip_prefix("jfif-version=") {
            d.jfif_version = Some(dec_version(rest)?);
        } else if let Some(rest) = token.strip_prefix("jfif-density-units=") {
            d.jfif_density_units = Some(dec_density_units(rest)?);
        } else if let Some(rest) = token.strip_prefix("jfif-x-density=") {
            d.jfif_x_density = Some(parse_u16(rest)?);
        } else if let Some(rest) = token.strip_prefix("jfif-y-density=") {
            d.jfif_y_density = Some(parse_u16(rest)?);
        } else if let Some(rest) = token.strip_prefix("jfif-thumbnail=") {
            d.jfif_thumbnail = Some(decode_option(rest, dec_thumbnail)?);
        } else if let Some(rest) = token.strip_prefix("frame=") {
            d.frame = Some(dec_frame_change(rest)?);
        } else if let Some(rest) = token.strip_prefix("sof-marker=") {
            d.sof_marker = Some(parse_u8(rest)?);
        } else if let Some(rest) = token.strip_prefix("arithmetic=") {
            d.arithmetic = Some(parse_bool(rest)?);
        } else if let Some(rest) = token.strip_prefix("quant-tables=") {
            d.quant_tables = Some(dec_quant_tables_diff(rest)?);
        } else if let Some(rest) = token.strip_prefix("huffman-tables=") {
            d.huffman_tables = Some(dec_huffman_tables_diff(rest)?);
        } else if let Some(rest) = token.strip_prefix("restart-interval=") {
            d.restart_interval = Some(decode_option(rest, parse_u16)?);
        } else if let Some(rest) = token.strip_prefix("other-segments=") {
            d.other_segments = Some(dec_other_segments_diff(rest)?);
        } else {
            return Err(format!("jpg diff: unknown token {token:?}"));
        }
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
    /// 🧪️ P2-FG2: REAL binary frame (`format u8 | flags u16le | <present fields, in declaration
    /// order>`), matching `../💾️binary/📡️.protocol.semio`'s `header fixed 3` + `chain
    /// payload bytes` shape — upgraded from F6's `print_diff().into_bytes()` text-as-binary
    /// shortcut (100% of stdio's `DiffCodec` impls were still on that shortcut per the P2-W0
    /// census). `flags` bit `i` (LSB first) marks the field at declaration position `i` present
    /// (`width`=0 .. `other_segments`=15 — 16 fields, hence `u16` rather than xml's `u8`); each
    /// present field's own (possibly tri-state) binary payload follows in that fixed order, using
    /// the real, non-recursive binary codecs in `§ValueBinaryCodecs`/`§DiffValueBinaryCodecs`
    /// above — no opaque tail anywhere in THIS frame (unlike xml's `XmlNodeDiff`, none of jpg's
    /// diff payloads are self-recursive).
    fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let mut flags: u16 = 0;
        if self.width.is_some() {
            flags |= 1 << 0;
        }
        if self.height.is_some() {
            flags |= 1 << 1;
        }
        if self.pixels.is_some() {
            flags |= 1 << 2;
        }
        if self.re_encode_quality.is_some() {
            flags |= 1 << 3;
        }
        if self.jfif_version.is_some() {
            flags |= 1 << 4;
        }
        if self.jfif_density_units.is_some() {
            flags |= 1 << 5;
        }
        if self.jfif_x_density.is_some() {
            flags |= 1 << 6;
        }
        if self.jfif_y_density.is_some() {
            flags |= 1 << 7;
        }
        if self.jfif_thumbnail.is_some() {
            flags |= 1 << 8;
        }
        if self.frame.is_some() {
            flags |= 1 << 9;
        }
        if self.sof_marker.is_some() {
            flags |= 1 << 10;
        }
        if self.arithmetic.is_some() {
            flags |= 1 << 11;
        }
        if self.quant_tables.is_some() {
            flags |= 1 << 12;
        }
        if self.huffman_tables.is_some() {
            flags |= 1 << 13;
        }
        if self.restart_interval.is_some() {
            flags |= 1 << 14;
        }
        if self.other_segments.is_some() {
            flags |= 1 << 15;
        }

        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT];
        out.extend_from_slice(&flags.to_le_bytes());
        if let Some(v) = self.width {
            store::pack_rt::write_varint_u64(&mut out, v as u64);
        }
        if let Some(v) = self.height {
            store::pack_rt::write_varint_u64(&mut out, v as u64);
        }
        if let Some(v) = &self.pixels {
            write_bytes_lp(&mut out, v);
        }
        if let Some(v) = &self.re_encode_quality {
            write_opt(&mut out, v, |q, out| out.push(*q));
        }
        if let Some(v) = self.jfif_version {
            enc_version_bin(&v, &mut out);
        }
        if let Some(v) = self.jfif_density_units {
            enc_density_units_bin(&v, &mut out);
        }
        if let Some(v) = self.jfif_x_density {
            store::pack_rt::write_varint_u64(&mut out, v as u64);
        }
        if let Some(v) = self.jfif_y_density {
            store::pack_rt::write_varint_u64(&mut out, v as u64);
        }
        if let Some(v) = &self.jfif_thumbnail {
            write_opt(&mut out, v, |t, out| enc_thumbnail_bin(t, out));
        }
        if let Some(v) = &self.frame {
            enc_frame_change_bin(v, &mut out);
        }
        if let Some(v) = self.sof_marker {
            out.push(v);
        }
        if let Some(v) = self.arithmetic {
            out.push(if v { 1 } else { 0 });
        }
        if let Some(v) = &self.quant_tables {
            enc_quant_tables_diff_bin(v, &mut out);
        }
        if let Some(v) = &self.huffman_tables {
            enc_huffman_tables_diff_bin(v, &mut out);
        }
        if let Some(v) = &self.restart_interval {
            write_opt(&mut out, v, |ri, out| store::pack_rt::write_varint_u64(out, *ri as u64));
        }
        if let Some(v) = &self.other_segments {
            enc_other_segments_diff_bin(v, &mut out);
        }
        Ok(out)
    }
    fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let _format = reader.read_u8().map_err(|e| malformed("diff format", 0, e.to_string()))?;
        let flags = reader.read_u16_le().map_err(|e| malformed("diff flags", 1, e.to_string()))?;

        let width = if flags & (1 << 0) != 0 { Some(reader.read_varint_u64().map_err(|e| malformed("diff width", reader.position(), e.to_string()))? as u32) } else { None };
        let height = if flags & (1 << 1) != 0 { Some(reader.read_varint_u64().map_err(|e| malformed("diff height", reader.position(), e.to_string()))? as u32) } else { None };
        let pixels = if flags & (1 << 2) != 0 { Some(read_bytes_lp(&mut reader).map_err(|e| malformed("diff pixels", reader.position(), e))?) } else { None };
        let re_encode_quality = if flags & (1 << 3) != 0 { Some(read_opt(&mut reader, |r| r.read_u8().map_err(|e| e.to_string())).map_err(|e| malformed("diff re-encode-quality", reader.position(), e))?) } else { None };
        let jfif_version = if flags & (1 << 4) != 0 { Some(dec_version_bin(&mut reader).map_err(|e| malformed("diff jfif-version", reader.position(), e))?) } else { None };
        let jfif_density_units = if flags & (1 << 5) != 0 { Some(dec_density_units_bin(&mut reader).map_err(|e| malformed("diff jfif-density-units", reader.position(), e))?) } else { None };
        let jfif_x_density = if flags & (1 << 6) != 0 { Some(reader.read_varint_u64().map_err(|e| malformed("diff jfif-x-density", reader.position(), e.to_string()))? as u16) } else { None };
        let jfif_y_density = if flags & (1 << 7) != 0 { Some(reader.read_varint_u64().map_err(|e| malformed("diff jfif-y-density", reader.position(), e.to_string()))? as u16) } else { None };
        let jfif_thumbnail = if flags & (1 << 8) != 0 { Some(read_opt(&mut reader, dec_thumbnail_bin).map_err(|e| malformed("diff jfif-thumbnail", reader.position(), e))?) } else { None };
        let frame = if flags & (1 << 9) != 0 { Some(dec_frame_change_bin(&mut reader).map_err(|e| malformed("diff frame", reader.position(), e))?) } else { None };
        let sof_marker = if flags & (1 << 10) != 0 { Some(reader.read_u8().map_err(|e| malformed("diff sof-marker", reader.position(), e.to_string()))?) } else { None };
        let arithmetic = if flags & (1 << 11) != 0 { Some(reader.read_u8().map_err(|e| malformed("diff arithmetic", reader.position(), e.to_string()))? != 0) } else { None };
        let quant_tables = if flags & (1 << 12) != 0 { Some(dec_quant_tables_diff_bin(&mut reader).map_err(|e| malformed("diff quant-tables", reader.position(), e))?) } else { None };
        let huffman_tables = if flags & (1 << 13) != 0 { Some(dec_huffman_tables_diff_bin(&mut reader).map_err(|e| malformed("diff huffman-tables", reader.position(), e))?) } else { None };
        let restart_interval = if flags & (1 << 14) != 0 { Some(read_opt(&mut reader, |r| Ok(r.read_varint_u64().map_err(|e| e.to_string())? as u16)).map_err(|e| malformed("diff restart-interval", reader.position(), e))?) } else { None };
        let other_segments = if flags & (1 << 15) != 0 { Some(dec_other_segments_diff_bin(&mut reader).map_err(|e| malformed("diff other-segments", reader.position(), e))?) } else { None };

        Ok(JpgDiff { width, height, pixels, re_encode_quality, jfif_version, jfif_density_units, jfif_x_density, jfif_y_density, jfif_thumbnail, frame, sof_marker, arithmetic, quant_tables, huffman_tables, restart_interval, other_segments })
    }
}
//#endregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec

//#region 🔖️DemoCases
/// 🧪️ P2-FG2: representative `JpgDiff` values (both `JpgFrameChange` variants, every tri-state
/// scalar in both transition directions, all three id/index-keyed collection triples with
/// removed/modified/added all populated across two `between()` directions) — the single source of
/// truth reused by `handcrafted_diff_codec_tests::diff_codec_text_binary_roundtrip_law` below AND
/// by `⚙️engine/🦀️.rs`'s `diff_grammar_conformance_law`/`protocol_walk_law` conformance
/// tests. Mirrors `handcrafted_diff_codec_tests`'s own `snap_a`/`snap_b`/`snap_c` fixtures exactly
/// (kept `pub(crate)` here instead of `#[cfg(test)]`-gated so the engine's non-test conformance
/// module can reuse it too — matches png's own `demo_diff_cases()` visibility).
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<JpgDiff> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn quant(id: u8, seed: u16) -> JpgQuantTable {
        JpgQuantTable { id, precision: 0, values: [seed; 64] }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn huffman(class: JpgHuffmanClass, id: u8, seed: u8) -> JpgHuffmanTable {
        JpgHuffmanTable { id, class, bits: [seed; 16], values: vec![seed, seed.wrapping_add(1)] }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn segment(marker: u8, data: Vec<u8>) -> JpgSegment {
        JpgSegment { marker, data }
    }

    let a = JpgSnapshot {
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
        frame: Some(JpgFrameHeader { precision: 8, width: 4, height: 4, components: vec![JpgFrameComponent { id: 1, h_sampling: 2, v_sampling: 2, quant_table_id: 0 }, JpgFrameComponent { id: 9, h_sampling: 1, v_sampling: 1, quant_table_id: 1 }] }),
        sof_marker: 0xC0,
        arithmetic: false,
        quant_tables: vec![quant(0, 10), quant(9, 20)],
        huffman_tables: vec![huffman(JpgHuffmanClass::Dc, 0, 1), huffman(JpgHuffmanClass::Ac, 9, 2)],
        restart_interval: Some(8),
        other_segments: vec![segment(0xFE, vec![1, 2, 3]), segment(0xE1, vec![9, 9])],
    };
    let b = JpgSnapshot {
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
        frame: Some(JpgFrameHeader { precision: 8, width: 8, height: 6, components: vec![JpgFrameComponent { id: 1, h_sampling: 1, v_sampling: 1, quant_table_id: 5 }] }),
        sof_marker: 0xC2,
        arithmetic: true,
        quant_tables: vec![quant(0, 99)],
        huffman_tables: vec![huffman(JpgHuffmanClass::Dc, 0, 7)],
        restart_interval: None,
        other_segments: vec![segment(0xFE, vec![4, 5, 6])],
    };
    let c = JpgSnapshot { frame: None, ..JpgSnapshot::default() };

    vec![JpgDiff::default(), JpgDiff::between(&a, &b), JpgDiff::between(&b, &a), JpgDiff::between(&a, &c), JpgDiff::between(&c, &a)]
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
mod handcrafted_diff_codec_tests {
    use super::*;
    use protocol::command::DiffAlgebra;
    use protocol::DiffCodec;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn quant(id: u8, seed: u16) -> JpgQuantTable {
        JpgQuantTable { id, precision: 0, values: [seed; 64] }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn huffman(class: JpgHuffmanClass, id: u8, seed: u8) -> JpgHuffmanTable {
        JpgHuffmanTable { id, class, bits: [seed; 16], values: vec![seed, seed.wrapping_add(1)] }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn segment(marker: u8, data: Vec<u8>) -> JpgSegment {
        JpgSegment { marker, data }
    }

    /// 🌱 `snap_a`/`snap_b` differ in EVERY diffable field (both directions exercise removed XOR
    /// added per id-keyed/index-keyed collection, the recipe's documented workaround — see
    /// `f6-recon-report.md`'s `field_sweep` precedent). `snap_c` has `frame: None`, exercising
    /// `JpgFrameChange::Replace` against both `a` and `c`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
                components: vec![JpgFrameComponent { id: 1, h_sampling: 2, v_sampling: 2, quant_table_id: 0 }, JpgFrameComponent { id: 9, h_sampling: 1, v_sampling: 1, quant_table_id: 1 }],
            }),
            sof_marker: 0xC0,
            arithmetic: false,
            quant_tables: vec![quant(0, 10), quant(9, 20)],
            huffman_tables: vec![huffman(JpgHuffmanClass::Dc, 0, 1), huffman(JpgHuffmanClass::Ac, 9, 2)],
            restart_interval: Some(8),
            other_segments: vec![segment(0xFE, vec![1, 2, 3]), segment(0xE1, vec![9, 9])],
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
            frame: Some(JpgFrameHeader { precision: 8, width: 8, height: 6, components: vec![JpgFrameComponent { id: 1, h_sampling: 1, v_sampling: 1, quant_table_id: 5 }] }),
            sof_marker: 0xC2,
            arithmetic: true,
            quant_tables: vec![quant(0, 99)],
            huffman_tables: vec![huffman(JpgHuffmanClass::Dc, 0, 7)],
            restart_interval: None,
            other_segments: vec![segment(0xFE, vec![4, 5, 6])],
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn snap_c() -> JpgSnapshot {
        JpgSnapshot { frame: None, ..JpgSnapshot::default() }
    }

    /// 🧪️ F6: `DiffCodec` round-trip laws over the hand-rolled `JpgDiff` grammar — exercises the
    /// `JpgFrameChange` enum (both `Modify` and `Replace`), all three tri-state scalars in both
    /// transition directions, and all three id/index-keyed collection triples with removed,
    /// modified, AND added entries all populated across the two `between()` directions.
    #[semio_framework_async_macros::async_test]
    async fn diff_codec_text_binary_roundtrip_law() {
        let a = snap_a();
        let b = snap_b();
        let c = snap_c();
        let cases = vec![JpgDiff::default(), JpgDiff::between(&a, &b), JpgDiff::between(&b, &a), JpgDiff::between(&a, &c), JpgDiff::between(&c, &a)];
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
