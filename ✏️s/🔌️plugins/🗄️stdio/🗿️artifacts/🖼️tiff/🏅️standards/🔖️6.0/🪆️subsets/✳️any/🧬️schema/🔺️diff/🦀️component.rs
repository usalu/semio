//! 🔺️ TiffDiff — handcrafted sparse diff. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: replaces the old
//! `TiffDiff{snapshot: Option<TiffSnapshot>}` full-replace template. `ifds` is an INDEX-keyed
//! `removed`/`modified`/`added` triple (TIFF's own IFD chain is positional); within each IFD,
//! `entries` is a TAG-ID-keyed triple (`tag: u16`, not array index — tag SETS can differ in
//! size between two IFDs, and tags are TIFF's own natural stable identity, unlike an ordinal
//! position). A `TiffTag` is a weak value (`kind`/`values` move together atomically), so a
//! tag-triple's `modified`/`added` payload carries the whole new tag, never a nested diff.

use crate::artifacts::tiff::schema::snapshot::{TiffByteOrder, TiffFieldType, TiffIfd, TiffTag, TiffValues};
use crate::artifacts::tiff::TiffSnapshot;
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap};

//#region 🔖️TagsTriple
/// 🏷️ One `entries.modified[]`/`.added[]` entity — `TiffTag` is a weak value, so both carry
/// the entry's NEW `kind`/`values` directly (never a nested per-field diff).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TiffTagModified {
    pub tag: u16,
    pub kind: TiffFieldType,
    pub values: TiffValues,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TiffTagAdded {
    pub tag: u16,
    pub kind: TiffFieldType,
    pub values: TiffValues,
}

/// 🔺️ Tag-id-keyed `entries` triple for one IFD.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TiffTagsDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<u16>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<TiffTagModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<TiffTagAdded>,
}

impl TiffTagsDiff {
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
}

/// ▶️ Applies a tag-id-keyed triple to one IFD's entries. TIFF6 §2 requires ascending-tag-
/// order within an IFD — `apply` re-sorts on every call, keeping that invariant regardless of
/// the triple's own insertion order.
fn apply_tags(base: &[TiffTag], d: &TiffTagsDiff) -> Vec<TiffTag> {
    let mut items: Vec<TiffTag> = base.iter().filter(|t| !d.removed.contains(&t.tag)).cloned().collect();
    for m in &d.modified {
        if let Some(it) = items.iter_mut().find(|t| t.tag == m.tag) {
            it.kind = m.kind;
            it.values = m.values.clone();
        }
    }
    for a in &d.added {
        if let Some(it) = items.iter_mut().find(|t| t.tag == a.tag) {
            it.kind = a.kind;
            it.values = a.values.clone();
        } else {
            items.push(TiffTag { tag: a.tag, kind: a.kind, values: a.values.clone() });
        }
    }
    items.sort_by_key(|t| t.tag);
    items
}

fn between_tags(a: &[TiffTag], b: &[TiffTag]) -> Option<TiffTagsDiff> {
    let a_map: BTreeMap<u16, &TiffTag> = a.iter().map(|t| (t.tag, t)).collect();
    let b_map: BTreeMap<u16, &TiffTag> = b.iter().map(|t| (t.tag, t)).collect();
    let mut removed = Vec::new();
    let mut modified = Vec::new();
    let mut added = Vec::new();
    for (tag, at) in &a_map {
        match b_map.get(tag) {
            None => removed.push(*tag),
            Some(bt) => {
                if at.kind != bt.kind || at.values != bt.values {
                    modified.push(TiffTagModified { tag: *tag, kind: bt.kind, values: bt.values.clone() });
                }
            }
        }
    }
    for (tag, bt) in &b_map {
        if !a_map.contains_key(tag) {
            added.push(TiffTagAdded { tag: *tag, kind: bt.kind, values: bt.values.clone() });
        }
    }
    if removed.is_empty() && modified.is_empty() && added.is_empty() {
        None
    } else {
        Some(TiffTagsDiff { removed, modified, added })
    }
}

/// ➕️ Structural, total, base-free absorb for a TAG-ID-keyed triple. Simpler than an
/// index-keyed collection's transport: tag ids are stable identity, never renumbered by
/// insert/remove, so no position-simulation is needed — a plain keyed union/override algebra.
fn absorb_tags(d1: TiffTagsDiff, d2: TiffTagsDiff) -> TiffTagsDiff {
    let mut removed: BTreeSet<u16> = d1.removed.into_iter().collect();
    let mut modified: BTreeMap<u16, TiffTagModified> = d1.modified.into_iter().map(|m| (m.tag, m)).collect();
    let mut added: BTreeMap<u16, TiffTagAdded> = d1.added.into_iter().map(|a| (a.tag, a)).collect();

    for r in d2.removed {
        if added.remove(&r).is_some() {
            // A d2-removal of a d1-added tag annihilates the add (recipe's canonical case).
        } else {
            modified.remove(&r);
            removed.insert(r);
        }
    }
    for m in d2.modified {
        if let Some(a) = added.get_mut(&m.tag) {
            // d2 patch on a d1-added tag patches INTO the still-pending added payload.
            a.kind = m.kind;
            a.values = m.values;
        } else if !removed.contains(&m.tag) {
            modified.insert(m.tag, m);
        }
        // modified-of-removed (by d1) is illegal per the apply contract — ignored here too.
    }
    for a in d2.added {
        removed.remove(&a.tag);
        added.insert(a.tag, a);
    }

    TiffTagsDiff { removed: removed.into_iter().collect(), modified: modified.into_values().collect(), added: added.into_values().collect() }
}
//#endregion 🔖️TagsTriple

//#region 🔖️IfdsTriple
/// 🗂️ One `ifds.modified[]` entity — the recursive per-IFD tag-triple.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TiffIfdModified {
    pub index: usize,
    pub diff: TiffTagsDiff,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TiffIfdAdded {
    pub index: usize,
    pub ifd: TiffIfd,
}

/// 🔺️ Index-keyed `ifds` triple (TIFF's IFD chain is positional).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TiffIfdsDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<TiffIfdModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<TiffIfdAdded>,
}

//#region 🔖️IndexTransport
// 🧮 Base-free index transport for `ifds`' absorb — the same position-simulation shape as
// PNG's `text_chunks`/csv's `records` (`simulate_slots`/`base_len_hint`), since `ifds`'
// `modified` payload IS a nested diff (needs field-aware absorb, not plain LWW).
#[derive(Clone, Copy, Debug)]
enum Slot {
    Base(usize),
    Added(usize),
}

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

fn base_len_hint(removed: &[usize], modified_indices: impl Iterator<Item = usize>, added_indices: impl Iterator<Item = usize>) -> usize {
    removed.iter().copied().chain(modified_indices).chain(added_indices).max().map(|m| m + 1).unwrap_or(0)
}

fn absorb_ifds(d1: TiffIfdsDiff, d2: TiffIfdsDiff) -> TiffIfdsDiff {
    let d1_added_indices: Vec<usize> = d1.added.iter().map(|a| a.index).collect();
    let removed_count = {
        let mut r = d1.removed.clone();
        r.sort_unstable();
        r.dedup();
        r.len()
    };
    let needed_mid_len = d2.removed.iter().copied().chain(d2.modified.iter().map(|m| m.index)).max().map(|m| m + 1).unwrap_or(0);
    let base_len = base_len_hint(&d1.removed, d1.modified.iter().map(|m| m.index), d1_added_indices.iter().copied())
        .max((needed_mid_len + removed_count).saturating_sub(d1.added.len()));
    let mid_slots = simulate_slots(base_len, &d1.removed, &d1_added_indices);

    let mut final_removed: Vec<usize> = d1.removed;
    let mut modified_map: BTreeMap<usize, TiffTagsDiff> = d1.modified.into_iter().map(|m| (m.index, m.diff)).collect();
    let mut added_alive: Vec<Option<TiffIfdAdded>> = d1.added.into_iter().map(Some).collect();

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
                let entry = modified_map.entry(*b).or_default();
                *entry = absorb_tags(entry.clone(), m2.diff.clone());
            }
            Some(Slot::Added(ai)) => {
                if let Some(a) = added_alive[*ai].as_mut() {
                    a.ifd.entries = apply_tags(&a.ifd.entries, &m2.diff);
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
    let mut final_modified: Vec<TiffIfdModified> =
        modified_map.into_iter().filter(|(_, d)| !d.is_empty()).map(|(index, diff)| TiffIfdModified { index, diff }).collect();
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
    let mid_len = d2
        .removed
        .iter()
        .copied()
        .chain(d2.modified.iter().map(|m| m.index))
        .chain(alive_mid_positions.iter().copied())
        .chain(d2_added_indices.iter().copied())
        .max()
        .map(|m| m + 1)
        .unwrap_or(0);
    let after_slots = simulate_slots(mid_len, &d2.removed, &d2_added_indices);
    let mut mid_to_after: HashMap<usize, usize> = HashMap::new();
    for (pos, slot) in after_slots.iter().enumerate() {
        if let Slot::Base(m) = slot {
            mid_to_after.insert(*m, pos);
        }
    }

    let mut final_added: Vec<TiffIfdAdded> = Vec::new();
    for (ai, alive) in added_alive.into_iter().enumerate() {
        if let Some(added) = alive {
            let mid_pos = mid_slots.iter().position(|s| matches!(s, Slot::Added(idx) if *idx == ai)).expect("added_alive index always has a corresponding mid slot");
            if let Some(after_pos) = mid_to_after.get(&mid_pos) {
                final_added.push(TiffIfdAdded { index: *after_pos, ifd: added.ifd });
            }
        }
    }
    for a2 in d2.added {
        final_added.push(a2);
    }
    final_added.sort_by_key(|a| a.index);

    TiffIfdsDiff { removed: final_removed, modified: final_modified, added: final_added }
}
//#endregion 🔖️IndexTransport

fn apply_ifds(base: &[TiffIfd], d: &TiffIfdsDiff) -> Vec<TiffIfd> {
    let mut items = base.to_vec();
    for m in &d.modified {
        if let Some(it) = items.get_mut(m.index) {
            it.entries = apply_tags(&it.entries, &m.diff);
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
        items.insert(at, a.ifd);
    }
    items
}

fn between_ifds(a: &[TiffIfd], b: &[TiffIfd]) -> Option<TiffIfdsDiff> {
    let min = a.len().min(b.len());
    let mut modified = Vec::new();
    for i in 0..min {
        if let Some(d) = between_tags(&a[i].entries, &b[i].entries) {
            modified.push(TiffIfdModified { index: i, diff: d });
        }
    }
    let removed: Vec<usize> = (min..a.len()).collect();
    let added: Vec<TiffIfdAdded> = (min..b.len()).map(|i| TiffIfdAdded { index: i, ifd: b[i].clone() }).collect();
    if removed.is_empty() && modified.is_empty() && added.is_empty() { None } else { Some(TiffIfdsDiff { removed, modified, added }) }
}

fn absorb_ifds_opt(base: &mut Option<TiffIfdsDiff>, other: Option<TiffIfdsDiff>) {
    match (base.take(), other) {
        (None, o) => *base = o,
        (Some(b), None) => *base = Some(b),
        (Some(b), Some(o)) => *base = Some(absorb_ifds(b, o)),
    }
}
//#endregion 🔖️IfdsTriple

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.tiff`. No `snapshot: Option<TiffSnapshot>` full-replace slot — even
/// `SetSnapshot`'s diff is `TiffDiff::between(base, next)`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.tiff.diff")]
pub struct TiffDiff {
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub byte_order: Option<TiffByteOrder>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ifds: Option<TiffIfdsDiff>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pixels: Option<Vec<u8>>,
}

impl MutationDiff<TiffSnapshot> for TiffDiff {
    fn apply(&self, base: &TiffSnapshot) -> TiffSnapshot {
        let mut next = base.clone();
        if let Some(v) = self.byte_order {
            next.byte_order = v;
        }
        if let Some(d) = &self.ifds {
            next.ifds = apply_ifds(&next.ifds, d);
        }
        if let Some(v) = &self.pixels {
            next.pixels = v.clone();
        }
        next
    }

    /// ➕️ Structural, total, base-free sequential-coalesce (`## Absorb` contract). `byte_order`/
    /// `pixels`: LWW. `ifds`: index-transported merge with the nested tag-id-keyed merge for
    /// `modified` entries.
    fn absorb(&mut self, other: Self) {
        if other.byte_order.is_some() {
            self.byte_order = other.byte_order;
        }
        absorb_ifds_opt(&mut self.ifds, other.ifds);
        if other.pixels.is_some() {
            self.pixels = other.pixels;
        }
    }
}

impl DiffAlgebra<TiffSnapshot> for TiffDiff {
    /// 🔁️ Diff-level undo, derived generically (correct by construction): the state delta
    /// from `self.apply(base)` back to `base`.
    fn inverse(&self, base: &TiffSnapshot) -> Self {
        let mutated = self.apply(base);
        Self::between(&mutated, base)
    }

    /// 🧭️ State delta (compose `GetXDiff`): index-keyed pairwise `0..min(len)` matching for
    /// `ifds`, recursive tag-id-keyed matching within each surviving IFD pair.
    fn between(base: &TiffSnapshot, other: &TiffSnapshot) -> Self {
        Self {
            byte_order: (base.byte_order != other.byte_order).then_some(other.byte_order),
            ifds: between_ifds(&base.ifds, &other.ifds),
            pixels: (base.pixels != other.pixels).then(|| other.pixels.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.byte_order.is_none() && self.ifds.is_none() && self.pixels.is_none()
    }
}

/// 🧩 Builds a set-snapshot diff (sparse field-by-field delta, never a full-replace slot).
pub fn diff_set_snapshot(base: &TiffSnapshot, next: &TiffSnapshot) -> TiffDiff {
    TiffDiff::between(base, next)
}
//#endregion 🔖️Diff

//#region 🔖️MutationDiffBuilders
// 🧩 One handcrafted builder per `schema::mutations::TiffMutation` variant (excluding
// `NoMutation`/`SetSnapshot`, covered above).

pub fn diff_set_byte_order(base: &TiffSnapshot, byte_order: TiffByteOrder) -> TiffDiff {
    TiffDiff { byte_order: (base.byte_order != byte_order).then_some(byte_order), ..Default::default() }
}

pub fn diff_insert_ifd(base: &TiffSnapshot, index: usize, ifd: TiffIfd) -> TiffDiff {
    let at = index.min(base.ifds.len());
    TiffDiff { ifds: Some(TiffIfdsDiff { removed: vec![], modified: vec![], added: vec![TiffIfdAdded { index: at, ifd }] }), ..Default::default() }
}

pub fn diff_remove_ifd(base: &TiffSnapshot, index: usize) -> TiffDiff {
    if index >= base.ifds.len() {
        return TiffDiff::default();
    }
    TiffDiff { ifds: Some(TiffIfdsDiff { removed: vec![index], modified: vec![], added: vec![] }), ..Default::default() }
}

pub fn diff_set_tag(base: &TiffSnapshot, ifd_index: usize, tag: u16, kind: TiffFieldType, values: TiffValues) -> TiffDiff {
    let Some(ifd) = base.ifds.get(ifd_index) else { return TiffDiff::default() };
    let already = ifd.entries.iter().find(|t| t.tag == tag);
    if let Some(existing) = already {
        if existing.kind == kind && existing.values == values {
            return TiffDiff::default();
        }
        TiffDiff {
            ifds: Some(TiffIfdsDiff {
                removed: vec![],
                modified: vec![TiffIfdModified { index: ifd_index, diff: TiffTagsDiff { removed: vec![], modified: vec![TiffTagModified { tag, kind, values }], added: vec![] } }],
                added: vec![],
            }),
            ..Default::default()
        }
    } else {
        TiffDiff {
            ifds: Some(TiffIfdsDiff {
                removed: vec![],
                modified: vec![TiffIfdModified { index: ifd_index, diff: TiffTagsDiff { removed: vec![], modified: vec![], added: vec![TiffTagAdded { tag, kind, values }] } }],
                added: vec![],
            }),
            ..Default::default()
        }
    }
}

pub fn diff_remove_tag(base: &TiffSnapshot, ifd_index: usize, tag: u16) -> TiffDiff {
    let Some(ifd) = base.ifds.get(ifd_index) else { return TiffDiff::default() };
    if !ifd.entries.iter().any(|t| t.tag == tag) {
        return TiffDiff::default();
    }
    TiffDiff {
        ifds: Some(TiffIfdsDiff {
            removed: vec![],
            modified: vec![TiffIfdModified { index: ifd_index, diff: TiffTagsDiff { removed: vec![tag], modified: vec![], added: vec![] } }],
            added: vec![],
        }),
        ..Default::default()
    }
}

pub fn diff_set_pixels(base: &TiffSnapshot, pixels: Vec<u8>) -> TiffDiff {
    TiffDiff { pixels: (base.pixels != pixels).then_some(pixels), ..Default::default() }
}
//#endregion 🔖️MutationDiffBuilders
