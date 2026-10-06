//! 🔺️ JpgDiff — handcrafted sparse diff. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: replaces the old
//! `JpgDiff{snapshot: Option<JpgSnapshot>}` full-replace template. JFIF header fields are
//! top-level scalars; `frame` is a `Modify`/`Replace` change (mirrors xml's `XmlNodeDiff::Replace`
//! fallback — a decode-status transition (`None`<->`Some`) is a "kind change", everything else is
//! a field-level `Modify`); `quant_tables`/`huffman_tables`/`frame.components` are id-keyed
//! `removed`/`modified`/`added` triples (stable identity, no index-transport needed — see
//! `absorb_id_keyed_*`); `other_segments` is an index-keyed triple (position-transported absorb,
//! mirrors png's `text_chunks`).

use crate::schema::snapshot::{JfifDensityUnits, JfifThumbnail, JpgFrameComponent, JpgFrameHeader, JpgHuffmanClass, JpgHuffmanTable, JpgQuantTable, JpgSegment};
use crate::JpgSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
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
    fn absorb(&mut self, other: &Self) {
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
    fn absorb(&mut self, other: &Self) {
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
    removed.iter().copied().chain(modified_indices).chain(added_indices).max().map_or(0, |m| m + 1)
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
    let needed_mid_len = d2.removed.iter().copied().chain(d2.modified.iter().map(|m| m.index)).max().map_or(0, |m| m + 1);
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
    let mid_len = d2.removed.iter().copied().chain(d2.modified.iter().map(|m| m.index)).chain(alive_mid_positions.iter().copied()).chain(d2_added_indices.iter().copied()).max().map_or(0, |m| m + 1);
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
            existing.diff.absorb(&dm.diff);
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
            existing.diff.absorb(&dm.diff);
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
/// 🧪️ F6 CONFIRMED HAND-ROLL: `#[derive(dsl::)]` on this struct fails to compile with TWO
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
// 🚫️aaaaaaaaaaaregion 🔖️Primitives

//#region 🔖️BinaryPrimitives
/// 🧪️a️aregion 🔖️BinaryPrimitives

//#region 🔖️ValueBinaryCodecs
/// 🧪️aaaaaaaaaaaaaaaaaaaregion 🔖️ValueBinaryCodecs

//#region 🔖️ValueCodecs
// 🚫️aaaa️aa️aa️aa️aa️aa️aa️aa️aa️aa️aaregion 🔖️ValueCodecs

//#region 🔖️DiffValueCodecs
// 🚫️aaaa️aaaa️aaaa️aaaa� aaaregion 🔖️DiffValueCodecs

//#region 🔖️DiffValueBinaryCodecs
/// 🧪️aaa️aaaa️aaaa️aaaa� aaaregion 🔖️DiffValueBinaryCodecs

//#region 🔖️TopLevel
/// 🧾 aprregion 🔖️TopLevel
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
#[path = "🧪️tests/🔬️handcrafted-diff-codec/🦀️.rs"]
mod handcrafted_diff_codec_tests;
//#endregion 🧪️Tests
//#endregion 🔖️HandcraftedDiffCodec
