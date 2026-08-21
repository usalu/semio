//! 🔺️ PngDiff — handcrafted sparse diff. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: replaces the old
//! `PngDiff{snapshot: Option<PngSnapshot>}` full-replace template. IHDR/tRNS/ancillary fields
//! are top-level scalars (plain `Option<T>` for always-present IHDR fields, tri-state
//! `Option<Option<T>>` for the genuinely optional ones); `plte`/`text_chunks`/`chunk_order`/
//! `unknown_chunks` are index-keyed `removed`/`modified`/`added` triples (`plte` additionally
//! nested inside a tri-state so "palette chunk removed entirely" and "palette entries changed"
//! are both expressible — see `PngPlteDiff` doc).
//!
//! 🧪️ F6 CONFIRMED (real `cargo check -p semio-s-plugin-stdio --lib` runs, output kept in the
//! ticket folder as `f6-png-diff-derive-check.txt`/`f6-png-mutation-derive-check.txt`):
//! `#[derive(dsl::DslDiff)]` on `PngDiff` fails for TWO independent reasons (per
//! `f6-recon-report.md` §3's decision rule), same as `SvgDiff`/`GifDiff`: (1) 8 top-level
//! tri-state `Option<Option<T>>` fields (`plte`, `trns`, `gama`, `chrm`, `srgb`, `phys`, `time`,
//! `bkgd`) — e.g. `error[E0277]: the trait bound Option<PngChromaticities>: DslField is not
//! satisfied`; (2) `PngTransparency`/`PngBackground` are genuine data-carrying enums
//! (`Indexed`/`Grayscale`/`Rgb` etc., each with fields) reachable through `trns`/`bkgd` — e.g.
//! `error[E0277]: the trait bound PngTransparency: DslField is not satisfied`. `PngMutation`'s
//! own `#[derive(dsl::DslOps)]` attempt fails for the SAME enum reason (`SetTransparency`/
//! `SetBackground` carry the enum directly; `SetSnapshot` carries it via `PngSnapshot`) —
//! independently confirmed with its own real `cargo check` (42 `DslField` errors). Both sides are
//! hand-rolled below/in `🧬️mutations/🦀️component.rs`, following the gif89a/svg template exactly
//! (`f6-recon-report.md` §5).

use crate::artifacts::png::schema::snapshot::{PngBackground, PngChromaticities, PngChunk, PngChunkMarker, PngColorType, PngPhysicalDims, PngRgb, PngSrgbIntent, PngTextChunk, PngTextKind, PngTimestamp, PngTransparency};
use crate::artifacts::png::PngSnapshot;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

//#region 🔖️PlteTriple
/// 🎨️ One `plte.modified[]`/`plte.added[]` entity — `PngRgb` is a weak value, so `modified`
/// carries the entry's NEW value directly (not a nested diff).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PngPlteEntryModified {
    pub index: usize,
    pub rgb: PngRgb,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PngPlteEntryAdded {
    pub index: usize,
    pub rgb: PngRgb,
}

/// 🔺️ Index-keyed `PLTE` entries triple. Nested inside `PngDiff::plte`'s OUTER
/// `Option<Option<_>>`: outer `None` = palette unchanged; outer `Some(None)` = the `PLTE`
/// chunk was removed entirely; outer `Some(Some(triple))` = the palette is present (whether
/// newly created — an all-`added` triple — or an existing palette's entries changed).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PngPlteDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<PngPlteEntryModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<PngPlteEntryAdded>,
}
//#endregion 🔖️PlteTriple

//#region 🔖️TextChunkDiff
/// 💬️ Sparse per-field patch for one [`PngTextChunk`].
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PngTextChunkDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keyword: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compressed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<PngTextKind>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language_tag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub translated_keyword: Option<String>,
}

impl PngTextChunkDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn apply(&self, base: &PngTextChunk) -> PngTextChunk {
        PngTextChunk {
            keyword: self.keyword.clone().unwrap_or_else(|| base.keyword.clone()),
            value: self.value.clone().unwrap_or_else(|| base.value.clone()),
            compressed: self.compressed.unwrap_or(base.compressed),
            kind: self.kind.unwrap_or(base.kind),
            language_tag: self.language_tag.clone().unwrap_or_else(|| base.language_tag.clone()),
            translated_keyword: self.translated_keyword.clone().unwrap_or_else(|| base.translated_keyword.clone()),
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn between(a: &PngTextChunk, b: &PngTextChunk) -> Self {
        Self {
            keyword: (a.keyword != b.keyword).then(|| b.keyword.clone()),
            value: (a.value != b.value).then(|| b.value.clone()),
            compressed: (a.compressed != b.compressed).then_some(b.compressed),
            kind: (a.kind != b.kind).then_some(b.kind),
            language_tag: (a.language_tag != b.language_tag).then(|| b.language_tag.clone()),
            translated_keyword: (a.translated_keyword != b.translated_keyword).then(|| b.translated_keyword.clone()),
        }
    }
    /// ➕️ LWW field-by-field absorb.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn absorb(&mut self, other: Self) {
        if other.keyword.is_some() {
            self.keyword = other.keyword;
        }
        if other.value.is_some() {
            self.value = other.value;
        }
        if other.compressed.is_some() {
            self.compressed = other.compressed;
        }
        if other.kind.is_some() {
            self.kind = other.kind;
        }
        if other.language_tag.is_some() {
            self.language_tag = other.language_tag;
        }
        if other.translated_keyword.is_some() {
            self.translated_keyword = other.translated_keyword;
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PngTextChunkModified {
    pub index: usize,
    pub diff: PngTextChunkDiff,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PngTextChunkAdded {
    pub index: usize,
    pub chunk: PngTextChunk,
}

/// 🔺️ Index-keyed `text_chunks` triple (see `PngTextChunk` doc for why index, not keyword).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PngTextChunksDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<PngTextChunkModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<PngTextChunkAdded>,
}
//#endregion 🔖️TextChunkDiff

//#region 🔖️UnknownChunksDiff
/// 🗃️ One `unknown_chunks.modified[]`/`.added[]` entity — `PngChunk` is a weak value.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PngUnknownChunkModified {
    pub index: usize,
    pub chunk: PngChunk,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PngUnknownChunkAdded {
    pub index: usize,
    pub chunk: PngChunk,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PngUnknownChunksDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<PngUnknownChunkModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<PngUnknownChunkAdded>,
}
//#endregion 🔖️UnknownChunksDiff

//#region 🔖️ChunkOrderDiff
/// 🧭️ One `chunk_order.modified[]`/`.added[]` entity — `PngChunkMarker` is a weak value.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PngChunkOrderModified {
    pub index: usize,
    pub marker: PngChunkMarker,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PngChunkOrderAdded {
    pub index: usize,
    pub marker: PngChunkMarker,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PngChunkOrderDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<PngChunkOrderModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<PngChunkOrderAdded>,
}
//#endregion 🔖️ChunkOrderDiff

//#region 🔖️IndexTransport
// 🧮 Base-free index transport for absorb — simulates the SAME removed(descending)/added
// (ascending, clamped) sequence `apply` performs, over a virtual index universe bounded
// tightly by what a diff's own removed/modified keys actually reference (matches the recipe's
// "structural, total, base-free" absorb contract; ported verbatim from csv's proven
// `simulate_slots`/`base_len_hint`, which are already fully type-agnostic).

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

/// ➕️ Structural, total, base-free absorb for an index-keyed collection of WEAK values (the
/// `modified` payload is the new value itself, not a nested diff — a doubly-modified position
/// is plain LWW). Shared by `plte`/`unknown_chunks`/`chunk_order` (`text_chunks` needs its own
/// field-aware variant, see `absorb_text_chunks`, since its modified payload IS a nested diff).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_weak_index_triple<T: Clone>(
    d1_removed: Vec<usize>,
    d1_modified: Vec<(usize, T)>,
    d1_added: Vec<(usize, T)>,
    d2_removed: Vec<usize>,
    d2_modified: Vec<(usize, T)>,
    d2_added: Vec<(usize, T)>,
) -> (Vec<usize>, Vec<(usize, T)>, Vec<(usize, T)>) {
    let d1_added_indices: Vec<usize> = d1_added.iter().map(|(i, _)| *i).collect();
    let removed_count = {
        let mut r = d1_removed.clone();
        r.sort_unstable();
        r.dedup();
        r.len()
    };
    let needed_mid_len = d2_removed.iter().copied().chain(d2_modified.iter().map(|(i, _)| *i)).max().map(|m| m + 1).unwrap_or(0);
    let base_len = base_len_hint(&d1_removed, d1_modified.iter().map(|(i, _)| *i), d1_added_indices.iter().copied()).max((needed_mid_len + removed_count).saturating_sub(d1_added.len()));
    let mid_slots = simulate_slots(base_len, &d1_removed, &d1_added_indices);

    let mut final_removed: Vec<usize> = d1_removed;
    let mut modified_map: BTreeMap<usize, T> = d1_modified.into_iter().collect();
    let mut added_alive: Vec<Option<(usize, T)>> = d1_added.into_iter().map(Some).collect();

    for mid_idx in &d2_removed {
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
    for (mid_idx, val) in &d2_modified {
        match mid_slots.get(*mid_idx) {
            Some(Slot::Base(b)) => {
                modified_map.insert(*b, val.clone());
            }
            Some(Slot::Added(ai)) => {
                if let Some(a) = added_alive[*ai].as_mut() {
                    a.1 = val.clone();
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
    let final_modified: Vec<(usize, T)> = modified_map.into_iter().collect();

    let alive_mid_positions: Vec<usize> = mid_slots
        .iter()
        .enumerate()
        .filter_map(|(pos, slot)| match slot {
            Slot::Added(ai) if added_alive[*ai].is_some() => Some(pos),
            _ => None,
        })
        .collect();
    let d2_added_indices: Vec<usize> = d2_added.iter().map(|(i, _)| *i).collect();
    let mid_len = d2_removed.iter().copied().chain(d2_modified.iter().map(|(i, _)| *i)).chain(alive_mid_positions.iter().copied()).chain(d2_added_indices.iter().copied()).max().map(|m| m + 1).unwrap_or(0);
    let after_slots = simulate_slots(mid_len, &d2_removed, &d2_added_indices);
    let mut mid_to_after: HashMap<usize, usize> = HashMap::new();
    for (pos, slot) in after_slots.iter().enumerate() {
        if let Slot::Base(m) = slot {
            mid_to_after.insert(*m, pos);
        }
    }

    let mut final_added: Vec<(usize, T)> = Vec::new();
    for (ai, alive) in added_alive.into_iter().enumerate() {
        if let Some((_, val)) = alive {
            let mid_pos = mid_slots.iter().position(|s| matches!(s, Slot::Added(idx) if *idx == ai)).expect("added_alive index always has a corresponding mid slot");
            if let Some(after_pos) = mid_to_after.get(&mid_pos) {
                final_added.push((*after_pos, val));
            }
        }
    }
    for (i, v) in d2_added {
        final_added.push((i, v));
    }
    final_added.sort_by_key(|(i, _)| *i);

    (final_removed, final_modified, final_added)
}

/// ➕️ Field-aware absorb for `text_chunks` (mirrors csv's `absorb_records` exactly, retargeted
/// to `PngTextChunk`/`PngTextChunkDiff`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_text_chunks(d1: PngTextChunksDiff, d2: PngTextChunksDiff) -> PngTextChunksDiff {
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
    let mut modified_map: BTreeMap<usize, PngTextChunkDiff> = d1.modified.into_iter().map(|m| (m.index, m.diff)).collect();
    let mut added_alive: Vec<Option<PngTextChunkAdded>> = d1.added.into_iter().map(Some).collect();

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
                    a.chunk = m2.diff.apply(&a.chunk);
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
    let mut final_modified: Vec<PngTextChunkModified> = modified_map.into_iter().filter(|(_, d)| !d.is_empty()).map(|(index, diff)| PngTextChunkModified { index, diff }).collect();
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

    let mut final_added: Vec<PngTextChunkAdded> = Vec::new();
    for (ai, alive) in added_alive.into_iter().enumerate() {
        if let Some(added) = alive {
            let mid_pos = mid_slots.iter().position(|s| matches!(s, Slot::Added(idx) if *idx == ai)).expect("added_alive index always has a corresponding mid slot");
            if let Some(after_pos) = mid_to_after.get(&mid_pos) {
                final_added.push(PngTextChunkAdded { index: *after_pos, chunk: added.chunk });
            }
        }
    }
    for a2 in d2.added {
        final_added.push(a2);
    }
    final_added.sort_by_key(|a| a.index);

    PngTextChunksDiff { removed: final_removed, modified: final_modified, added: final_added }
}
//#endregion 🔖️IndexTransport

//#region 🔖️CollectionApplyBetween
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_plte(base: &Option<Vec<PngRgb>>, d: &Option<PngPlteDiff>) -> Option<Vec<PngRgb>> {
    match d {
        None => None,
        Some(triple) => {
            let mut entries = base.clone().unwrap_or_default();
            let mut removed_desc = triple.removed.clone();
            removed_desc.sort_unstable_by(|a, b| b.cmp(a));
            removed_desc.dedup();
            for idx in removed_desc {
                if idx < entries.len() {
                    entries.remove(idx);
                }
            }
            for m in &triple.modified {
                if let Some(e) = entries.get_mut(m.index) {
                    *e = m.rgb;
                }
            }
            let mut adds = triple.added.clone();
            adds.sort_by_key(|a| a.index);
            for a in adds {
                let at = a.index.min(entries.len());
                entries.insert(at, a.rgb);
            }
            Some(entries)
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_plte(a: &Option<Vec<PngRgb>>, b: &Option<Vec<PngRgb>>) -> Option<Option<PngPlteDiff>> {
    if a == b {
        return None;
    }
    match b {
        None => Some(None),
        Some(bv) => {
            let av = a.clone().unwrap_or_default();
            let min = av.len().min(bv.len());
            let mut modified = Vec::new();
            for i in 0..min {
                if av[i] != bv[i] {
                    modified.push(PngPlteEntryModified { index: i, rgb: bv[i] });
                }
            }
            let removed: Vec<usize> = (min..av.len()).collect();
            let added: Vec<PngPlteEntryAdded> = (min..bv.len()).map(|i| PngPlteEntryAdded { index: i, rgb: bv[i] }).collect();
            Some(Some(PngPlteDiff { removed, modified, added }))
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_plte(base: &mut Option<Option<PngPlteDiff>>, other: Option<Option<PngPlteDiff>>) {
    let Some(other) = other else { return };
    match other {
        None => {
            *base = Some(None);
        }
        Some(t2) => match base.take() {
            None | Some(None) => {
                *base = Some(Some(t2));
            }
            Some(Some(t1)) => {
                let (removed, modified, added) = absorb_weak_index_triple(
                    t1.removed,
                    t1.modified.into_iter().map(|m| (m.index, m.rgb)).collect(),
                    t1.added.into_iter().map(|a| (a.index, a.rgb)).collect(),
                    t2.removed,
                    t2.modified.into_iter().map(|m| (m.index, m.rgb)).collect(),
                    t2.added.into_iter().map(|a| (a.index, a.rgb)).collect(),
                );
                *base = Some(Some(PngPlteDiff { removed, modified: modified.into_iter().map(|(index, rgb)| PngPlteEntryModified { index, rgb }).collect(), added: added.into_iter().map(|(index, rgb)| PngPlteEntryAdded { index, rgb }).collect() }));
            }
        },
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_text_chunks(base: &[PngTextChunk], d: &PngTextChunksDiff) -> Vec<PngTextChunk> {
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
        items.insert(at, a.chunk);
    }
    items
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_text_chunks(a: &[PngTextChunk], b: &[PngTextChunk]) -> Option<PngTextChunksDiff> {
    let min = a.len().min(b.len());
    let mut modified = Vec::new();
    for i in 0..min {
        if a[i] != b[i] {
            let d = PngTextChunkDiff::between(&a[i], &b[i]);
            if !d.is_empty() {
                modified.push(PngTextChunkModified { index: i, diff: d });
            }
        }
    }
    let removed: Vec<usize> = (min..a.len()).collect();
    let added: Vec<PngTextChunkAdded> = (min..b.len()).map(|i| PngTextChunkAdded { index: i, chunk: b[i].clone() }).collect();
    if removed.is_empty() && modified.is_empty() && added.is_empty() {
        None
    } else {
        Some(PngTextChunksDiff { removed, modified, added })
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_unknown_chunks(base: &[PngChunk], d: &PngUnknownChunksDiff) -> Vec<PngChunk> {
    let mut items = base.to_vec();
    for m in &d.modified {
        if let Some(it) = items.get_mut(m.index) {
            *it = m.chunk.clone();
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
        items.insert(at, a.chunk);
    }
    items
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_unknown_chunks(a: &[PngChunk], b: &[PngChunk]) -> Option<PngUnknownChunksDiff> {
    let min = a.len().min(b.len());
    let mut modified = Vec::new();
    for i in 0..min {
        if a[i] != b[i] {
            modified.push(PngUnknownChunkModified { index: i, chunk: b[i].clone() });
        }
    }
    let removed: Vec<usize> = (min..a.len()).collect();
    let added: Vec<PngUnknownChunkAdded> = (min..b.len()).map(|i| PngUnknownChunkAdded { index: i, chunk: b[i].clone() }).collect();
    if removed.is_empty() && modified.is_empty() && added.is_empty() {
        None
    } else {
        Some(PngUnknownChunksDiff { removed, modified, added })
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_chunk_order(base: &[PngChunkMarker], d: &PngChunkOrderDiff) -> Vec<PngChunkMarker> {
    let mut items = base.to_vec();
    for m in &d.modified {
        if let Some(it) = items.get_mut(m.index) {
            *it = m.marker.clone();
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
        items.insert(at, a.marker);
    }
    items
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_chunk_order(a: &[PngChunkMarker], b: &[PngChunkMarker]) -> Option<PngChunkOrderDiff> {
    let min = a.len().min(b.len());
    let mut modified = Vec::new();
    for i in 0..min {
        if a[i] != b[i] {
            modified.push(PngChunkOrderModified { index: i, marker: b[i].clone() });
        }
    }
    let removed: Vec<usize> = (min..a.len()).collect();
    let added: Vec<PngChunkOrderAdded> = (min..b.len()).map(|i| PngChunkOrderAdded { index: i, marker: b[i].clone() }).collect();
    if removed.is_empty() && modified.is_empty() && added.is_empty() {
        None
    } else {
        Some(PngChunkOrderDiff { removed, modified, added })
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_text_chunks_opt(base: &mut Option<PngTextChunksDiff>, other: Option<PngTextChunksDiff>) {
    match (base.take(), other) {
        (None, o) => *base = o,
        (Some(b), None) => *base = Some(b),
        (Some(b), Some(o)) => *base = Some(absorb_text_chunks(b, o)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_unknown_chunks_opt(base: &mut Option<PngUnknownChunksDiff>, other: Option<PngUnknownChunksDiff>) {
    match (base.take(), other) {
        (None, o) => *base = o,
        (Some(b), None) => *base = Some(b),
        (Some(b), Some(o)) => {
            let (removed, modified, added) = absorb_weak_index_triple(
                b.removed,
                b.modified.into_iter().map(|m| (m.index, m.chunk)).collect(),
                b.added.into_iter().map(|a| (a.index, a.chunk)).collect(),
                o.removed,
                o.modified.into_iter().map(|m| (m.index, m.chunk)).collect(),
                o.added.into_iter().map(|a| (a.index, a.chunk)).collect(),
            );
            *base = Some(PngUnknownChunksDiff {
                removed,
                modified: modified.into_iter().map(|(index, chunk)| PngUnknownChunkModified { index, chunk }).collect(),
                added: added.into_iter().map(|(index, chunk)| PngUnknownChunkAdded { index, chunk }).collect(),
            });
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_chunk_order_opt(base: &mut Option<PngChunkOrderDiff>, other: Option<PngChunkOrderDiff>) {
    match (base.take(), other) {
        (None, o) => *base = o,
        (Some(b), None) => *base = Some(b),
        (Some(b), Some(o)) => {
            let (removed, modified, added) = absorb_weak_index_triple(
                b.removed,
                b.modified.into_iter().map(|m| (m.index, m.marker)).collect(),
                b.added.into_iter().map(|a| (a.index, a.marker)).collect(),
                o.removed,
                o.modified.into_iter().map(|m| (m.index, m.marker)).collect(),
                o.added.into_iter().map(|a| (a.index, a.marker)).collect(),
            );
            *base = Some(PngChunkOrderDiff {
                removed,
                modified: modified.into_iter().map(|(index, marker)| PngChunkOrderModified { index, marker }).collect(),
                added: added.into_iter().map(|(index, marker)| PngChunkOrderAdded { index, marker }).collect(),
            });
        }
    }
}
//#endregion 🔖️CollectionApplyBetween

//#region 🔖️ChunkOrderMutationHelpers
// 🧩 Shared by `schema::mutations`' `diff()` builders — keeps every mutation that toggles an
// ancillary field's presence, or inserts/removes a `text_chunks`/`unknown_chunks` entry, ALSO
// producing the matching `chunk_order` delta (the recipe's "nothing real silently dropped"
// rule extends to ORDER: a scalar going None→Some without a chunk_order entry would silently
// desync the two).

/// 📍 Where a freshly created marker lands — always just before `Iend` (or at the very end if
/// there is none). Documented normalization: only a DECODED file's `chunk_order` carries the
/// real original position; anything created via mutation gets this deterministic default.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn chunk_order_insert_pos(order: &[PngChunkMarker]) -> usize {
    order.iter().position(|m| matches!(m, PngChunkMarker::Iend)).unwrap_or(order.len())
}

/// 🔀 Diff for a scalar ancillary field's marker toggling presence (`None` if presence didn't
/// change — e.g. a value-only change to an already-present field).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn chunk_order_presence_diff(order: &[PngChunkMarker], is_marker: fn(&PngChunkMarker) -> bool, marker: PngChunkMarker, was_present: bool, want_present: bool) -> Option<PngChunkOrderDiff> {
    if was_present == want_present {
        return None;
    }
    if want_present {
        let pos = chunk_order_insert_pos(order);
        Some(PngChunkOrderDiff { removed: vec![], modified: vec![], added: vec![PngChunkOrderAdded { index: pos, marker }] })
    } else {
        order.iter().position(|m| is_marker(m)).map(|idx| PngChunkOrderDiff { removed: vec![idx], modified: vec![], added: vec![] })
    }
}

/// ➕️ Diff for inserting a new `Text{index: at}` marker: renumbers every existing `Text`
/// marker whose embedded index is `>= at` (`modified`, same `chunk_order` position, bumped
/// payload) and appends the new marker just before `Iend` (`added`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn chunk_order_insert_text_diff(order: &[PngChunkMarker], at: usize) -> PngChunkOrderDiff {
    let modified = order
        .iter()
        .enumerate()
        .filter_map(|(pos, m)| match m {
            PngChunkMarker::Text { index } if *index >= at => Some(PngChunkOrderModified { index: pos, marker: PngChunkMarker::Text { index: index + 1 } }),
            _ => None,
        })
        .collect();
    let added = vec![PngChunkOrderAdded { index: chunk_order_insert_pos(order), marker: PngChunkMarker::Text { index: at } }];
    PngChunkOrderDiff { removed: vec![], modified, added }
}

/// ➖️ Diff for removing the `Text{index: at}` marker: drops it (`removed`) and renumbers every
/// `Text` marker with a HIGHER embedded index down by one (`modified`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn chunk_order_remove_text_diff(order: &[PngChunkMarker], at: usize) -> PngChunkOrderDiff {
    let removed: Vec<usize> = order.iter().position(|m| matches!(m, PngChunkMarker::Text { index } if *index == at)).into_iter().collect();
    let modified = order
        .iter()
        .enumerate()
        .filter_map(|(pos, m)| match m {
            PngChunkMarker::Text { index } if *index > at => Some(PngChunkOrderModified { index: pos, marker: PngChunkMarker::Text { index: index - 1 } }),
            _ => None,
        })
        .collect();
    PngChunkOrderDiff { removed, modified, added: vec![] }
}

/// ➕️ `Unknown{index}` analogue of [`chunk_order_insert_text_diff`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn chunk_order_insert_unknown_diff(order: &[PngChunkMarker], at: usize) -> PngChunkOrderDiff {
    let modified = order
        .iter()
        .enumerate()
        .filter_map(|(pos, m)| match m {
            PngChunkMarker::Unknown { index } if *index >= at => Some(PngChunkOrderModified { index: pos, marker: PngChunkMarker::Unknown { index: index + 1 } }),
            _ => None,
        })
        .collect();
    let added = vec![PngChunkOrderAdded { index: chunk_order_insert_pos(order), marker: PngChunkMarker::Unknown { index: at } }];
    PngChunkOrderDiff { removed: vec![], modified, added }
}

/// ➖️ `Unknown{index}` analogue of [`chunk_order_remove_text_diff`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn chunk_order_remove_unknown_diff(order: &[PngChunkMarker], at: usize) -> PngChunkOrderDiff {
    let removed: Vec<usize> = order.iter().position(|m| matches!(m, PngChunkMarker::Unknown { index } if *index == at)).into_iter().collect();
    let modified = order
        .iter()
        .enumerate()
        .filter_map(|(pos, m)| match m {
            PngChunkMarker::Unknown { index } if *index > at => Some(PngChunkOrderModified { index: pos, marker: PngChunkMarker::Unknown { index: index - 1 } }),
            _ => None,
        })
        .collect();
    PngChunkOrderDiff { removed, modified, added: vec![] }
}
//#endregion 🔖️ChunkOrderMutationHelpers

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.png`. No `snapshot: Option<PngSnapshot>` full-replace slot — even
/// `SetSnapshot`'s diff is `PngDiff::between(base, next)`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.png.diff")]
pub struct PngDiff {
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bit_depth: Option<u8>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color_type: Option<PngColorType>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interlace: Option<bool>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plte: Option<Option<PngPlteDiff>>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trns: Option<Option<PngTransparency>>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gama: Option<Option<u32>>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chrm: Option<Option<PngChromaticities>>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub srgb: Option<Option<PngSrgbIntent>>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phys: Option<Option<PngPhysicalDims>>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<Option<PngTimestamp>>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bkgd: Option<Option<PngBackground>>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_chunks: Option<PngTextChunksDiff>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pixels: Option<Vec<u8>>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chunk_order: Option<PngChunkOrderDiff>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unknown_chunks: Option<PngUnknownChunksDiff>,
}

impl MutationDiff<PngSnapshot> for PngDiff {
    async fn apply(&self, base: &PngSnapshot) -> MutationApplyResult<PngSnapshot> {
        if let Some(Some(plte)) = &self.plte {
            validate_png_triple(base.plte.as_ref().map_or(0, Vec::len), &plte.removed, plte.modified.iter().map(|entry| entry.index), plte.added.iter().map(|entry| entry.index), ["plte"])?;
        }
        if let Some(text) = &self.text_chunks {
            validate_png_triple(base.text_chunks.len(), &text.removed, text.modified.iter().map(|entry| entry.index), text.added.iter().map(|entry| entry.index), ["textChunks"])?;
        }
        if let Some(order) = &self.chunk_order {
            validate_png_triple(base.chunk_order.len(), &order.removed, order.modified.iter().map(|entry| entry.index), order.added.iter().map(|entry| entry.index), ["chunkOrder"])?;
        }
        if let Some(unknown) = &self.unknown_chunks {
            validate_png_triple(base.unknown_chunks.len(), &unknown.removed, unknown.modified.iter().map(|entry| entry.index), unknown.added.iter().map(|entry| entry.index), ["unknownChunks"])?;
        }
        let mut next = base.clone();
        if let Some(v) = self.width {
            next.width = v;
        }
        if let Some(v) = self.height {
            next.height = v;
        }
        if let Some(v) = self.bit_depth {
            next.bit_depth = v;
        }
        if let Some(v) = self.color_type {
            next.color_type = v;
        }
        if let Some(v) = self.interlace {
            next.interlace = v;
        }
        if let Some(d) = &self.plte {
            next.plte = apply_plte(&next.plte, d);
        }
        if let Some(v) = &self.trns {
            next.trns = v.clone();
        }
        if let Some(v) = self.gama {
            next.gama = v;
        }
        if let Some(v) = self.chrm {
            next.chrm = v;
        }
        if let Some(v) = self.srgb {
            next.srgb = v;
        }
        if let Some(v) = self.phys {
            next.phys = v;
        }
        if let Some(v) = self.time {
            next.time = v;
        }
        if let Some(v) = &self.bkgd {
            next.bkgd = v.clone();
        }
        if let Some(td) = &self.text_chunks {
            next.text_chunks = apply_text_chunks(&next.text_chunks, td);
        }
        if let Some(v) = &self.pixels {
            next.pixels = v.clone();
        }
        if let Some(od) = &self.chunk_order {
            next.chunk_order = apply_chunk_order(&next.chunk_order, od);
        }
        if let Some(ud) = &self.unknown_chunks {
            next.unknown_chunks = apply_unknown_chunks(&next.unknown_chunks, ud);
        }
        Ok(next)
    }

    /// ➕️ Structural, total, base-free sequential-coalesce (`## Absorb` contract). Scalars
    /// (incl. every tri-state, `plte` excepted): LWW. Collections: index-transported merge —
    /// `plte`/`unknown_chunks`/`chunk_order` via the shared weak-value transport,
    /// `text_chunks` via its own field-aware variant.
    async fn absorb(&mut self, other: Self) {
        if other.width.is_some() {
            self.width = other.width;
        }
        if other.height.is_some() {
            self.height = other.height;
        }
        if other.bit_depth.is_some() {
            self.bit_depth = other.bit_depth;
        }
        if other.color_type.is_some() {
            self.color_type = other.color_type;
        }
        if other.interlace.is_some() {
            self.interlace = other.interlace;
        }
        absorb_plte(&mut self.plte, other.plte);
        if other.trns.is_some() {
            self.trns = other.trns;
        }
        if other.gama.is_some() {
            self.gama = other.gama;
        }
        if other.chrm.is_some() {
            self.chrm = other.chrm;
        }
        if other.srgb.is_some() {
            self.srgb = other.srgb;
        }
        if other.phys.is_some() {
            self.phys = other.phys;
        }
        if other.time.is_some() {
            self.time = other.time;
        }
        if other.bkgd.is_some() {
            self.bkgd = other.bkgd;
        }
        absorb_text_chunks_opt(&mut self.text_chunks, other.text_chunks);
        if other.pixels.is_some() {
            self.pixels = other.pixels;
        }
        absorb_chunk_order_opt(&mut self.chunk_order, other.chunk_order);
        absorb_unknown_chunks_opt(&mut self.unknown_chunks, other.unknown_chunks);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_png_triple<I, J, K>(base_len: usize, removed: &[usize], modified: I, added: J, path: K) -> MutationApplyResult<()>
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
            return Err(MutationApplyError::new("mutation.apply.missing-target", "PNG collection removal is missing or duplicated").at(path.iter().map(String::as_str)));
        }
    }
    let mut modified_set = std::collections::HashSet::new();
    for index in modified {
        if index >= base_len || !modified_set.insert(index) || removed_set.contains(&index) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "PNG collection modification is missing, duplicated, or removed").at(path.iter().map(String::as_str)));
        }
    }
    let added: Vec<usize> = added.into_iter().collect();
    let final_len = base_len.saturating_sub(removed.len()).saturating_add(added.len());
    let mut added_set = std::collections::HashSet::new();
    for index in added {
        if index > final_len || !added_set.insert(index) {
            return Err(MutationApplyError::new("mutation.apply.invalid-index", "PNG collection addition index is invalid or duplicated").at(path.iter().map(String::as_str)));
        }
    }
    Ok(())
}

impl DiffAlgebra<PngSnapshot> for PngDiff {
    /// 🔁️ Diff-level undo, derived generically (correct by construction) exactly like zip's:
    /// the state delta from `self.apply(base)` back to `base`.
    async fn inverse(&self, base: &PngSnapshot) -> Self {
        let mutated = self.apply(base).await.unwrap();
        Self::between(&mutated, base).await
    }

    /// 🧭️ State delta (compose `GetXDiff`): index-keyed pairwise `0..min(len)` matching for
    /// every collection, tri-state comparison for every optional scalar/nested triple.
    async fn between(base: &PngSnapshot, other: &PngSnapshot) -> Self {
        Self {
            width: (base.width != other.width).then_some(other.width),
            height: (base.height != other.height).then_some(other.height),
            bit_depth: (base.bit_depth != other.bit_depth).then_some(other.bit_depth),
            color_type: (base.color_type != other.color_type).then_some(other.color_type),
            interlace: (base.interlace != other.interlace).then_some(other.interlace),
            plte: between_plte(&base.plte, &other.plte),
            trns: (base.trns != other.trns).then(|| other.trns.clone()),
            gama: (base.gama != other.gama).then_some(other.gama),
            chrm: (base.chrm != other.chrm).then_some(other.chrm),
            srgb: (base.srgb != other.srgb).then_some(other.srgb),
            phys: (base.phys != other.phys).then_some(other.phys),
            time: (base.time != other.time).then_some(other.time),
            bkgd: (base.bkgd != other.bkgd).then(|| other.bkgd.clone()),
            text_chunks: between_text_chunks(&base.text_chunks, &other.text_chunks),
            pixels: (base.pixels != other.pixels).then(|| other.pixels.clone()),
            chunk_order: between_chunk_order(&base.chunk_order, &other.chunk_order),
            unknown_chunks: between_unknown_chunks(&base.unknown_chunks, &other.unknown_chunks),
        }
    }

    async fn is_empty(&self) -> bool {
        self.width.is_none()
            && self.height.is_none()
            && self.bit_depth.is_none()
            && self.color_type.is_none()
            && self.interlace.is_none()
            && self.plte.is_none()
            && self.trns.is_none()
            && self.gama.is_none()
            && self.chrm.is_none()
            && self.srgb.is_none()
            && self.phys.is_none()
            && self.time.is_none()
            && self.bkgd.is_none()
            && self.text_chunks.is_none()
            && self.pixels.is_none()
            && self.chunk_order.is_none()
            && self.unknown_chunks.is_none()
    }
}

/// 🧩 Builds a set-snapshot diff (sparse field-by-field delta, never a full-replace slot).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_snapshot(base: &PngSnapshot, next: &PngSnapshot) -> PngDiff {
    PngDiff::between(base, next)
}
//#endregion 🔖️Diff

//#region 🔖️MutationDiffBuilders
// 🧩 One handcrafted builder per `schema::mutations::PngMutation` variant (excluding
// `NoMutation`/`SetSnapshot`, covered above) — each constructs the sparse `PngDiff` directly,
// including the matching `chunk_order` delta where the mutation changes chunk presence/order.

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_header(base: &PngSnapshot, width: u32, height: u32, bit_depth: u8, color_type: PngColorType, interlace: bool) -> PngDiff {
    PngDiff {
        width: (base.width != width).then_some(width),
        height: (base.height != height).then_some(height),
        bit_depth: (base.bit_depth != bit_depth).then_some(bit_depth),
        color_type: (base.color_type != color_type).then_some(color_type),
        interlace: (base.interlace != interlace).then_some(interlace),
        ..Default::default()
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_palette(base: &PngSnapshot, plte: &Option<Vec<PngRgb>>) -> PngDiff {
    PngDiff { plte: between_plte(&base.plte, plte), chunk_order: chunk_order_presence_diff(&base.chunk_order, |m| matches!(m, PngChunkMarker::Plte), PngChunkMarker::Plte, base.plte.is_some(), plte.is_some()), ..Default::default() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_transparency(base: &PngSnapshot, trns: &Option<PngTransparency>) -> PngDiff {
    PngDiff { trns: (base.trns != *trns).then(|| trns.clone()), chunk_order: chunk_order_presence_diff(&base.chunk_order, |m| matches!(m, PngChunkMarker::Trns), PngChunkMarker::Trns, base.trns.is_some(), trns.is_some()), ..Default::default() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_gamma(base: &PngSnapshot, gama: Option<u32>) -> PngDiff {
    PngDiff { gama: (base.gama != gama).then_some(gama), chunk_order: chunk_order_presence_diff(&base.chunk_order, |m| matches!(m, PngChunkMarker::Gama), PngChunkMarker::Gama, base.gama.is_some(), gama.is_some()), ..Default::default() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_chromaticities(base: &PngSnapshot, chrm: Option<PngChromaticities>) -> PngDiff {
    PngDiff { chrm: (base.chrm != chrm).then_some(chrm), chunk_order: chunk_order_presence_diff(&base.chunk_order, |m| matches!(m, PngChunkMarker::Chrm), PngChunkMarker::Chrm, base.chrm.is_some(), chrm.is_some()), ..Default::default() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_srgb_intent(base: &PngSnapshot, srgb: Option<PngSrgbIntent>) -> PngDiff {
    PngDiff { srgb: (base.srgb != srgb).then_some(srgb), chunk_order: chunk_order_presence_diff(&base.chunk_order, |m| matches!(m, PngChunkMarker::Srgb), PngChunkMarker::Srgb, base.srgb.is_some(), srgb.is_some()), ..Default::default() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_physical_dims(base: &PngSnapshot, phys: Option<PngPhysicalDims>) -> PngDiff {
    PngDiff { phys: (base.phys != phys).then_some(phys), chunk_order: chunk_order_presence_diff(&base.chunk_order, |m| matches!(m, PngChunkMarker::Phys), PngChunkMarker::Phys, base.phys.is_some(), phys.is_some()), ..Default::default() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_timestamp(base: &PngSnapshot, time: Option<PngTimestamp>) -> PngDiff {
    PngDiff { time: (base.time != time).then_some(time), chunk_order: chunk_order_presence_diff(&base.chunk_order, |m| matches!(m, PngChunkMarker::Time), PngChunkMarker::Time, base.time.is_some(), time.is_some()), ..Default::default() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_background(base: &PngSnapshot, bkgd: &Option<PngBackground>) -> PngDiff {
    PngDiff { bkgd: (base.bkgd != *bkgd).then(|| bkgd.clone()), chunk_order: chunk_order_presence_diff(&base.chunk_order, |m| matches!(m, PngChunkMarker::Bkgd), PngChunkMarker::Bkgd, base.bkgd.is_some(), bkgd.is_some()), ..Default::default() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_text_chunk(base: &PngSnapshot, index: usize, chunk: PngTextChunk) -> PngDiff {
    let at = index.min(base.text_chunks.len());
    PngDiff { text_chunks: Some(PngTextChunksDiff { removed: vec![], modified: vec![], added: vec![PngTextChunkAdded { index: at, chunk }] }), chunk_order: Some(chunk_order_insert_text_diff(&base.chunk_order, at)), ..Default::default() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_text_chunk(base: &PngSnapshot, index: usize) -> PngDiff {
    if index >= base.text_chunks.len() {
        return PngDiff::default();
    }
    PngDiff { text_chunks: Some(PngTextChunksDiff { removed: vec![index], modified: vec![], added: vec![] }), chunk_order: Some(chunk_order_remove_text_diff(&base.chunk_order, index)), ..Default::default() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_text_chunk(base: &PngSnapshot, index: usize, chunk: PngTextChunk) -> PngDiff {
    let existing = match base.text_chunks.get(index) {
        Some(c) => c,
        None => return PngDiff::default(),
    };
    let d = PngTextChunkDiff::between(existing, &chunk);
    if d.is_empty() {
        return PngDiff::default();
    }
    PngDiff { text_chunks: Some(PngTextChunksDiff { removed: vec![], modified: vec![PngTextChunkModified { index, diff: d }], added: vec![] }), ..Default::default() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_pixels(base: &PngSnapshot, pixels: Vec<u8>) -> PngDiff {
    PngDiff { pixels: (base.pixels != pixels).then_some(pixels), ..Default::default() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_unknown_chunk(base: &PngSnapshot, index: usize, chunk: PngChunk) -> PngDiff {
    let at = index.min(base.unknown_chunks.len());
    PngDiff { unknown_chunks: Some(PngUnknownChunksDiff { removed: vec![], modified: vec![], added: vec![PngUnknownChunkAdded { index: at, chunk }] }), chunk_order: Some(chunk_order_insert_unknown_diff(&base.chunk_order, at)), ..Default::default() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_unknown_chunk(base: &PngSnapshot, index: usize) -> PngDiff {
    if index >= base.unknown_chunks.len() {
        return PngDiff::default();
    }
    PngDiff { unknown_chunks: Some(PngUnknownChunksDiff { removed: vec![index], modified: vec![], added: vec![] }), chunk_order: Some(chunk_order_remove_unknown_diff(&base.chunk_order, index)), ..Default::default() }
}
//#endregion 🔖️MutationDiffBuilders

//#region 🔖️HandcraftedDiffCodec
/// 🧪️ F6: **hand-rolled** `protocol::DiffCodec` for `PngDiff` (see the file-header doc comment
/// for the two confirmed real compile-error reasons the derive is unavailable). **Grammar**
/// (real, not `serde_json`): one space-separated `name=value` token per changed top-level
/// scalar/tri-state field (a field absent from the line = unchanged); the three non-tri-state
/// collection triples (`text_chunks`/`chunk_order`/`unknown_chunks`) print as
/// `name{[removed];[modified];[added]}` (no `=`); `plte`'s OUTER tri-state wraps the SAME
/// triple-body shape bare inside `encode_option`'s uniform `[0]`=None/`[1,<T>]`=Some(T) tag —
/// nested brackets disambiguate cleanly since `split_top_level` only tracks `[`/`]` depth, never
/// confusing the body's internal `;`/`,` with the tag's own. Bytes/strings are lowercase hex (no
/// external base64 dep, matches gif89a/svg's own hand-rolled codecs). Structs are positional
/// `[f1,f2,...]` tuples; data-carrying enums (`PngTransparency`/`PngBackground`) use a
/// single-uppercase-letter tag prefix immediately followed by the bracketed payload; unit-only
/// enums (`PngColorType`/`PngSrgbIntent`/`PngTextKind`) print as a bare decimal ordinal (reusing
/// `PngColorType::to_u8`/`from_u8` and `PngSrgbIntent::to_u8`/`from_u8` where they already exist
/// on the type); `PngChunkMarker` (mixed unit + data variants) uses its own literal chunk-name
/// tag (`IHDR`/`PLTE`/.../`TEXT[idx]`/`UNKN[idx]`) since it needs to stay self-documenting inside
/// `chunk_order` triples. `encode_diff` = the text bytes verbatim, same simplification
/// `GifDiff`/`SvgDiff`/`WriterDiff` use. Every primitive and value codec below is `pub(crate)` so
/// `PngMutation`'s own hand-rolled `OpText`/`OpBinary` (`🧬️mutations/🦀️component.rs`) reuses them
/// instead of duplicating (same intra-artifact reuse pattern svg's diff/mutations pair uses).
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
pub(crate) fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
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

/// 🧭️ Bracket-depth-aware split (tracks `[`/`]` only) — the whole grammar's parsing primitive.
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
pub(crate) fn enc_list<T>(items: &[T], enc: impl Fn(&T) -> String) -> String {
    format!("[{}]", items.iter().map(|i| enc(i)).collect::<Vec<_>>().join(","))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_list<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Vec<T>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec).collect()
}
//#endregion 🔖️Primitives

//#region 🔖️ValueCodecs
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_color_type(c: PngColorType) -> String {
    c.to_u8().to_string()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_color_type(s: &str) -> Result<PngColorType, String> {
    PngColorType::from_u8(parse_u8(s)?)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_rgb(c: &PngRgb) -> String {
    format!("[{},{},{}]", c.r, c.g, c.b)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_rgb(s: &str) -> Result<PngRgb, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [r, g, b] = parts.as_slice() else { return Err(format!("rgb: expected 3 fields, got {}", parts.len())) };
    Ok(PngRgb { r: parse_u8(r)?, g: parse_u8(g)?, b: parse_u8(b)? })
}
/// 👁️ `I[<hex alpha bytes>]` (Indexed) / `G[<gray>]` (Grayscale) / `R[<r>,<g>,<b>]` (Rgb).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_transparency(t: &PngTransparency) -> String {
    match t {
        PngTransparency::Indexed { alpha } => format!("I[{}]", hex_encode(alpha)),
        PngTransparency::Grayscale { gray } => format!("G[{gray}]"),
        PngTransparency::Rgb { r, g, b } => format!("R[{r},{g},{b}]"),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_transparency(s: &str) -> Result<PngTransparency, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "I" => Ok(PngTransparency::Indexed { alpha: hex_decode(inner)? }),
        "G" => Ok(PngTransparency::Grayscale { gray: parse_u16(inner)? }),
        "R" => {
            let parts = split_top_level(inner, ',');
            let [r, g, b] = parts.as_slice() else { return Err(format!("transparency rgb: expected 3 fields, got {}", parts.len())) };
            Ok(PngTransparency::Rgb { r: parse_u16(r)?, g: parse_u16(g)?, b: parse_u16(b)? })
        }
        other => Err(format!("transparency: unknown tag {other:?}")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_chromaticities(c: &PngChromaticities) -> String {
    format!("[{},{},{},{},{},{},{},{}]", c.white_x, c.white_y, c.red_x, c.red_y, c.green_x, c.green_y, c.blue_x, c.blue_y)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_chromaticities(s: &str) -> Result<PngChromaticities, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [wx, wy, rx, ry, gx, gy, bx, by] = parts.as_slice() else { return Err(format!("chromaticities: expected 8 fields, got {}", parts.len())) };
    Ok(PngChromaticities { white_x: parse_u32(wx)?, white_y: parse_u32(wy)?, red_x: parse_u32(rx)?, red_y: parse_u32(ry)?, green_x: parse_u32(gx)?, green_y: parse_u32(gy)?, blue_x: parse_u32(bx)?, blue_y: parse_u32(by)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_srgb_intent(s: PngSrgbIntent) -> String {
    s.to_u8().to_string()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_srgb_intent(s: &str) -> Result<PngSrgbIntent, String> {
    PngSrgbIntent::from_u8(parse_u8(s)?)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_physical_dims(p: &PngPhysicalDims) -> String {
    format!("[{},{},{}]", p.ppu_x, p.ppu_y, if p.unit_is_meter { 1 } else { 0 })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_physical_dims(s: &str) -> Result<PngPhysicalDims, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [ppu_x, ppu_y, unit] = parts.as_slice() else { return Err(format!("physical dims: expected 3 fields, got {}", parts.len())) };
    Ok(PngPhysicalDims { ppu_x: parse_u32(ppu_x)?, ppu_y: parse_u32(ppu_y)?, unit_is_meter: *unit == "1" })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_timestamp(t: &PngTimestamp) -> String {
    format!("[{},{},{},{},{},{}]", t.year, t.month, t.day, t.hour, t.minute, t.second)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_timestamp(s: &str) -> Result<PngTimestamp, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [year, month, day, hour, minute, second] = parts.as_slice() else { return Err(format!("timestamp: expected 6 fields, got {}", parts.len())) };
    Ok(PngTimestamp { year: parse_u16(year)?, month: parse_u8(month)?, day: parse_u8(day)?, hour: parse_u8(hour)?, minute: parse_u8(minute)?, second: parse_u8(second)? })
}
/// 🖼️ `G[<gray>]` (Grayscale) / `R[<r>,<g>,<b>]` (Rgb) / `I[<index>]` (Indexed).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_background(b: &PngBackground) -> String {
    match b {
        PngBackground::Grayscale { gray } => format!("G[{gray}]"),
        PngBackground::Rgb { r, g, b } => format!("R[{r},{g},{b}]"),
        PngBackground::Indexed { index } => format!("I[{index}]"),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_background(s: &str) -> Result<PngBackground, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "G" => Ok(PngBackground::Grayscale { gray: parse_u16(inner)? }),
        "R" => {
            let parts = split_top_level(inner, ',');
            let [r, g, b] = parts.as_slice() else { return Err(format!("background rgb: expected 3 fields, got {}", parts.len())) };
            Ok(PngBackground::Rgb { r: parse_u16(r)?, g: parse_u16(g)?, b: parse_u16(b)? })
        }
        "I" => Ok(PngBackground::Indexed { index: parse_u8(inner)? }),
        other => Err(format!("background: unknown tag {other:?}")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_text_kind(k: PngTextKind) -> String {
    match k {
        PngTextKind::Text => "0".to_string(),
        PngTextKind::ZText => "1".to_string(),
        PngTextKind::IText => "2".to_string(),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_text_kind(s: &str) -> Result<PngTextKind, String> {
    match s {
        "0" => Ok(PngTextKind::Text),
        "1" => Ok(PngTextKind::ZText),
        "2" => Ok(PngTextKind::IText),
        other => Err(format!("text kind: unknown tag {other:?}")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_text_chunk(c: &PngTextChunk) -> String {
    format!("[{},{},{},{},{},{}]", enc_str(&c.keyword), enc_str(&c.value), if c.compressed { 1 } else { 0 }, enc_text_kind(c.kind), enc_str(&c.language_tag), enc_str(&c.translated_keyword),)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_text_chunk(s: &str) -> Result<PngTextChunk, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [keyword, value, compressed, kind, language_tag, translated_keyword] = parts.as_slice() else {
        return Err(format!("text chunk: expected 6 fields, got {}", parts.len()));
    };
    Ok(PngTextChunk { keyword: dec_str(keyword)?, value: dec_str(value)?, compressed: *compressed == "1", kind: dec_text_kind(kind)?, language_tag: dec_str(language_tag)?, translated_keyword: dec_str(translated_keyword)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_chunk(c: &PngChunk) -> String {
    format!("[{},{}]", hex_encode(&c.kind), hex_encode(&c.data))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_chunk(s: &str) -> Result<PngChunk, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [kind, data] = parts.as_slice() else { return Err(format!("chunk: expected 2 fields, got {}", parts.len())) };
    let kind_bytes = hex_decode(kind)?;
    let kind: [u8; 4] = kind_bytes.as_slice().try_into().map_err(|_| format!("chunk kind: expected 4 bytes, got {}", kind_bytes.len()))?;
    Ok(PngChunk { kind, data: hex_decode(data)? })
}
/// 🧭️ Unit markers print as their bare literal chunk-name tag; `Text`/`Unknown` as `TAG[index]`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_chunk_marker(m: &PngChunkMarker) -> String {
    match m {
        PngChunkMarker::Ihdr => "IHDR".to_string(),
        PngChunkMarker::Plte => "PLTE".to_string(),
        PngChunkMarker::Trns => "TRNS".to_string(),
        PngChunkMarker::Gama => "GAMA".to_string(),
        PngChunkMarker::Chrm => "CHRM".to_string(),
        PngChunkMarker::Srgb => "SRGB".to_string(),
        PngChunkMarker::Phys => "PHYS".to_string(),
        PngChunkMarker::Time => "TIME".to_string(),
        PngChunkMarker::Bkgd => "BKGD".to_string(),
        PngChunkMarker::Idat => "IDAT".to_string(),
        PngChunkMarker::Iend => "IEND".to_string(),
        PngChunkMarker::Text { index } => format!("TEXT[{index}]"),
        PngChunkMarker::Unknown { index } => format!("UNKN[{index}]"),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_chunk_marker(s: &str) -> Result<PngChunkMarker, String> {
    match s {
        "IHDR" => Ok(PngChunkMarker::Ihdr),
        "PLTE" => Ok(PngChunkMarker::Plte),
        "TRNS" => Ok(PngChunkMarker::Trns),
        "GAMA" => Ok(PngChunkMarker::Gama),
        "CHRM" => Ok(PngChunkMarker::Chrm),
        "SRGB" => Ok(PngChunkMarker::Srgb),
        "PHYS" => Ok(PngChunkMarker::Phys),
        "TIME" => Ok(PngChunkMarker::Time),
        "BKGD" => Ok(PngChunkMarker::Bkgd),
        "IDAT" => Ok(PngChunkMarker::Idat),
        "IEND" => Ok(PngChunkMarker::Iend),
        other => {
            if let Some(rest) = other.strip_prefix("TEXT[") {
                let inner = rest.strip_suffix(']').ok_or_else(|| format!("chunk marker: bad TEXT shape {other:?}"))?;
                Ok(PngChunkMarker::Text { index: parse_usize(inner)? })
            } else if let Some(rest) = other.strip_prefix("UNKN[") {
                let inner = rest.strip_suffix(']').ok_or_else(|| format!("chunk marker: bad UNKN shape {other:?}"))?;
                Ok(PngChunkMarker::Unknown { index: parse_usize(inner)? })
            } else {
                Err(format!("chunk marker: unknown tag {other:?}"))
            }
        }
    }
}
//#endregion 🔖️ValueCodecs

//#region 🔖️DiffValueCodecs
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_triple_body(removed: &[usize], modified: &[(usize, String)], added: &[(usize, String)]) -> String {
    let removed = removed.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let modified = modified.iter().map(|(i, v)| format!("{i}:{v}")).collect::<Vec<_>>().join(",");
    let added = added.iter().map(|(i, v)| format!("{i}:{v}")).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_triple_body(body: &str) -> Result<(Vec<usize>, Vec<(usize, String)>, Vec<(usize, String)>), String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("triple: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_usize).collect::<Result<Vec<_>, String>>()?;
    let parse_entries = |s: &str| -> Result<Vec<(usize, String)>, String> {
        split_top_level(strip_brackets(s)?, ',')
            .into_iter()
            .filter(|s| !s.is_empty())
            .map(|entry| {
                let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("triple entry: bad entry {entry:?}"))?;
                Ok((parse_usize(idx)?, rest.to_string()))
            })
            .collect()
    };
    Ok((removed, parse_entries(modified_s)?, parse_entries(added_s)?))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_plte_body(d: &PngPlteDiff) -> String {
    enc_triple_body(&d.removed, &d.modified.iter().map(|m| (m.index, enc_rgb(&m.rgb))).collect::<Vec<_>>(), &d.added.iter().map(|a| (a.index, enc_rgb(&a.rgb))).collect::<Vec<_>>())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_plte_body(body: &str) -> Result<PngPlteDiff, String> {
    let (removed, modified, added) = dec_triple_body(body)?;
    Ok(PngPlteDiff {
        removed,
        modified: modified.into_iter().map(|(index, v)| Ok(PngPlteEntryModified { index, rgb: dec_rgb(&v)? })).collect::<Result<Vec<_>, String>>()?,
        added: added.into_iter().map(|(index, v)| Ok(PngPlteEntryAdded { index, rgb: dec_rgb(&v)? })).collect::<Result<Vec<_>, String>>()?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_text_chunk_diff(d: &PngTextChunkDiff) -> String {
    format!(
        "[{},{},{},{},{},{}]",
        encode_option(&d.keyword, |v| enc_str(v)),
        encode_option(&d.value, |v| enc_str(v)),
        encode_option(&d.compressed, |v| if *v { "1".to_string() } else { "0".to_string() }),
        encode_option(&d.kind, |v| enc_text_kind(*v)),
        encode_option(&d.language_tag, |v| enc_str(v)),
        encode_option(&d.translated_keyword, |v| enc_str(v)),
    )
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_text_chunk_diff(s: &str) -> Result<PngTextChunkDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [keyword, value, compressed, kind, language_tag, translated_keyword] = parts.as_slice() else {
        return Err(format!("text chunk diff: expected 6 fields, got {}", parts.len()));
    };
    Ok(PngTextChunkDiff {
        keyword: decode_option(keyword, dec_str)?,
        value: decode_option(value, dec_str)?,
        compressed: decode_option(compressed, |v| Ok(v == "1"))?,
        kind: decode_option(kind, dec_text_kind)?,
        language_tag: decode_option(language_tag, dec_str)?,
        translated_keyword: decode_option(translated_keyword, dec_str)?,
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_text_chunks_diff(d: &PngTextChunksDiff) -> String {
    format!("text-chunks{{{}}}", enc_triple_body(&d.removed, &d.modified.iter().map(|m| (m.index, enc_text_chunk_diff(&m.diff))).collect::<Vec<_>>(), &d.added.iter().map(|a| (a.index, enc_text_chunk(&a.chunk))).collect::<Vec<_>>(),))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_text_chunks_diff(body: &str) -> Result<PngTextChunksDiff, String> {
    let (removed, modified, added) = dec_triple_body(body)?;
    Ok(PngTextChunksDiff {
        removed,
        modified: modified.into_iter().map(|(index, v)| Ok(PngTextChunkModified { index, diff: dec_text_chunk_diff(&v)? })).collect::<Result<Vec<_>, String>>()?,
        added: added.into_iter().map(|(index, v)| Ok(PngTextChunkAdded { index, chunk: dec_text_chunk(&v)? })).collect::<Result<Vec<_>, String>>()?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_chunk_order_diff(d: &PngChunkOrderDiff) -> String {
    format!("chunk-order{{{}}}", enc_triple_body(&d.removed, &d.modified.iter().map(|m| (m.index, enc_chunk_marker(&m.marker))).collect::<Vec<_>>(), &d.added.iter().map(|a| (a.index, enc_chunk_marker(&a.marker))).collect::<Vec<_>>(),))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_chunk_order_diff(body: &str) -> Result<PngChunkOrderDiff, String> {
    let (removed, modified, added) = dec_triple_body(body)?;
    Ok(PngChunkOrderDiff {
        removed,
        modified: modified.into_iter().map(|(index, v)| Ok(PngChunkOrderModified { index, marker: dec_chunk_marker(&v)? })).collect::<Result<Vec<_>, String>>()?,
        added: added.into_iter().map(|(index, v)| Ok(PngChunkOrderAdded { index, marker: dec_chunk_marker(&v)? })).collect::<Result<Vec<_>, String>>()?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_unknown_chunks_diff(d: &PngUnknownChunksDiff) -> String {
    format!("unknown-chunks{{{}}}", enc_triple_body(&d.removed, &d.modified.iter().map(|m| (m.index, enc_chunk(&m.chunk))).collect::<Vec<_>>(), &d.added.iter().map(|a| (a.index, enc_chunk(&a.chunk))).collect::<Vec<_>>(),))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_unknown_chunks_diff(body: &str) -> Result<PngUnknownChunksDiff, String> {
    let (removed, modified, added) = dec_triple_body(body)?;
    Ok(PngUnknownChunksDiff {
        removed,
        modified: modified.into_iter().map(|(index, v)| Ok(PngUnknownChunkModified { index, chunk: dec_chunk(&v)? })).collect::<Result<Vec<_>, String>>()?,
        added: added.into_iter().map(|(index, v)| Ok(PngUnknownChunkAdded { index, chunk: dec_chunk(&v)? })).collect::<Result<Vec<_>, String>>()?,
    })
}
//#endregion 🔖️DiffValueCodecs

//#region 🔖️RealBinaryPrimitives
// 🧪️ P2-P2: real binary value codecs for `PngDiff`/`PngMutation`'s shared nested types —
// mirrors the text codecs immediately above (`enc_str`/`enc_rgb`/…) field-for-field, using
// `dsl::ByteWriter`/`dsl::ByteReader` (the real framework LEB128-varint/length-prefixed
// primitives, `🧰️framework/…/🎒️pack/🧾️codec/🦀️component.rs`, reachable exactly like csv's own
// precedent — `dsl`/`store`/`protocol` all alias the same kernel crate root). `pub(crate)` so
// `🧬️mutations/🦀️component.rs`'s own `OpBinary` impl reuses these instead of duplicating (same
// intra-artifact reuse direction the text codecs above already establish).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_str(w: &mut dsl::ByteWriter, s: &str) {
    let bytes = s.as_bytes();
    w.write_varint_u64(bytes.len() as u64);
    w.write_bytes(bytes);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_str(r: &mut dsl::ByteReader<'_>) -> Result<String, dsl::PackError> {
    let len = r.read_varint_u64()? as usize;
    let bytes = r.read_bytes(len)?;
    String::from_utf8(bytes.to_vec()).map_err(|e| dsl::PackError::Malformed { what: "png binary utf8 string", offset: 0, detail: e.to_string() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_blob(w: &mut dsl::ByteWriter, bytes: &[u8]) {
    w.write_varint_u64(bytes.len() as u64);
    w.write_bytes(bytes);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_blob(r: &mut dsl::ByteReader<'_>) -> Result<Vec<u8>, dsl::PackError> {
    let len = r.read_varint_u64()? as usize;
    Ok(r.read_bytes(len)?.to_vec())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_rgb(w: &mut dsl::ByteWriter, c: &PngRgb) {
    w.write_u8(c.r);
    w.write_u8(c.g);
    w.write_u8(c.b);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_rgb(r: &mut dsl::ByteReader<'_>) -> Result<PngRgb, dsl::PackError> {
    Ok(PngRgb { r: r.read_u8()?, g: r.read_u8()?, b: r.read_u8()? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_transparency(w: &mut dsl::ByteWriter, t: &PngTransparency) {
    match t {
        PngTransparency::Indexed { alpha } => {
            w.write_u8(0);
            write_bin_blob(w, alpha);
        }
        PngTransparency::Grayscale { gray } => {
            w.write_u8(1);
            w.write_u16_le(*gray);
        }
        PngTransparency::Rgb { r, g, b } => {
            w.write_u8(2);
            w.write_u16_le(*r);
            w.write_u16_le(*g);
            w.write_u16_le(*b);
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_transparency(r: &mut dsl::ByteReader<'_>) -> Result<PngTransparency, dsl::PackError> {
    match r.read_u8()? {
        0 => Ok(PngTransparency::Indexed { alpha: read_bin_blob(r)? }),
        1 => Ok(PngTransparency::Grayscale { gray: r.read_u16_le()? }),
        2 => {
            let rr = r.read_u16_le()?;
            let g = r.read_u16_le()?;
            let b = r.read_u16_le()?;
            Ok(PngTransparency::Rgb { r: rr, g, b })
        }
        other => Err(dsl::PackError::Malformed { what: "png transparency tag", offset: 0, detail: format!("unknown tag {other}") }),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_chromaticities(w: &mut dsl::ByteWriter, c: &PngChromaticities) {
    for v in [c.white_x, c.white_y, c.red_x, c.red_y, c.green_x, c.green_y, c.blue_x, c.blue_y] {
        w.write_u32_le(v);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_chromaticities(r: &mut dsl::ByteReader<'_>) -> Result<PngChromaticities, dsl::PackError> {
    Ok(PngChromaticities { white_x: r.read_u32_le()?, white_y: r.read_u32_le()?, red_x: r.read_u32_le()?, red_y: r.read_u32_le()?, green_x: r.read_u32_le()?, green_y: r.read_u32_le()?, blue_x: r.read_u32_le()?, blue_y: r.read_u32_le()? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_physical_dims(w: &mut dsl::ByteWriter, p: &PngPhysicalDims) {
    w.write_u32_le(p.ppu_x);
    w.write_u32_le(p.ppu_y);
    w.write_u8(if p.unit_is_meter { 1 } else { 0 });
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_physical_dims(r: &mut dsl::ByteReader<'_>) -> Result<PngPhysicalDims, dsl::PackError> {
    Ok(PngPhysicalDims { ppu_x: r.read_u32_le()?, ppu_y: r.read_u32_le()?, unit_is_meter: r.read_u8()? != 0 })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_timestamp(w: &mut dsl::ByteWriter, t: &PngTimestamp) {
    w.write_u16_le(t.year);
    w.write_u8(t.month);
    w.write_u8(t.day);
    w.write_u8(t.hour);
    w.write_u8(t.minute);
    w.write_u8(t.second);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_timestamp(r: &mut dsl::ByteReader<'_>) -> Result<PngTimestamp, dsl::PackError> {
    Ok(PngTimestamp { year: r.read_u16_le()?, month: r.read_u8()?, day: r.read_u8()?, hour: r.read_u8()?, minute: r.read_u8()?, second: r.read_u8()? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_background(w: &mut dsl::ByteWriter, b: &PngBackground) {
    match b {
        PngBackground::Grayscale { gray } => {
            w.write_u8(0);
            w.write_u16_le(*gray);
        }
        PngBackground::Rgb { r, g, b } => {
            w.write_u8(1);
            w.write_u16_le(*r);
            w.write_u16_le(*g);
            w.write_u16_le(*b);
        }
        PngBackground::Indexed { index } => {
            w.write_u8(2);
            w.write_u8(*index);
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_background(r: &mut dsl::ByteReader<'_>) -> Result<PngBackground, dsl::PackError> {
    match r.read_u8()? {
        0 => Ok(PngBackground::Grayscale { gray: r.read_u16_le()? }),
        1 => {
            let rr = r.read_u16_le()?;
            let g = r.read_u16_le()?;
            let b = r.read_u16_le()?;
            Ok(PngBackground::Rgb { r: rr, g, b })
        }
        2 => Ok(PngBackground::Indexed { index: r.read_u8()? }),
        other => Err(dsl::PackError::Malformed { what: "png background tag", offset: 0, detail: format!("unknown tag {other}") }),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_text_kind_u8(k: PngTextKind) -> u8 {
    match k {
        PngTextKind::Text => 0,
        PngTextKind::ZText => 1,
        PngTextKind::IText => 2,
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_text_kind_u8(v: u8) -> Result<PngTextKind, dsl::PackError> {
    match v {
        0 => Ok(PngTextKind::Text),
        1 => Ok(PngTextKind::ZText),
        2 => Ok(PngTextKind::IText),
        other => Err(dsl::PackError::Malformed { what: "png text kind tag", offset: 0, detail: format!("unknown tag {other}") }),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_text_chunk(w: &mut dsl::ByteWriter, c: &PngTextChunk) {
    write_bin_str(w, &c.keyword);
    write_bin_str(w, &c.value);
    w.write_u8(if c.compressed { 1 } else { 0 });
    w.write_u8(enc_text_kind_u8(c.kind));
    write_bin_str(w, &c.language_tag);
    write_bin_str(w, &c.translated_keyword);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_text_chunk(r: &mut dsl::ByteReader<'_>) -> Result<PngTextChunk, dsl::PackError> {
    Ok(PngTextChunk { keyword: read_bin_str(r)?, value: read_bin_str(r)?, compressed: r.read_u8()? != 0, kind: dec_text_kind_u8(r.read_u8()?)?, language_tag: read_bin_str(r)?, translated_keyword: read_bin_str(r)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_chunk(w: &mut dsl::ByteWriter, c: &PngChunk) {
    w.write_bytes(&c.kind);
    write_bin_blob(w, &c.data);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_chunk(r: &mut dsl::ByteReader<'_>) -> Result<PngChunk, dsl::PackError> {
    let kind_bytes = r.read_bytes(4)?;
    let kind: [u8; 4] = kind_bytes.try_into().map_err(|_| dsl::PackError::Malformed { what: "png chunk kind", offset: 0, detail: "expected 4 bytes".into() })?;
    Ok(PngChunk { kind, data: read_bin_blob(r)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_chunk_marker(w: &mut dsl::ByteWriter, m: &PngChunkMarker) {
    match m {
        PngChunkMarker::Ihdr => w.write_u8(0),
        PngChunkMarker::Plte => w.write_u8(1),
        PngChunkMarker::Trns => w.write_u8(2),
        PngChunkMarker::Gama => w.write_u8(3),
        PngChunkMarker::Chrm => w.write_u8(4),
        PngChunkMarker::Srgb => w.write_u8(5),
        PngChunkMarker::Phys => w.write_u8(6),
        PngChunkMarker::Time => w.write_u8(7),
        PngChunkMarker::Bkgd => w.write_u8(8),
        PngChunkMarker::Idat => w.write_u8(9),
        PngChunkMarker::Iend => w.write_u8(10),
        PngChunkMarker::Text { index } => {
            w.write_u8(11);
            w.write_varint_u64(*index as u64);
        }
        PngChunkMarker::Unknown { index } => {
            w.write_u8(12);
            w.write_varint_u64(*index as u64);
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_chunk_marker(r: &mut dsl::ByteReader<'_>) -> Result<PngChunkMarker, dsl::PackError> {
    match r.read_u8()? {
        0 => Ok(PngChunkMarker::Ihdr),
        1 => Ok(PngChunkMarker::Plte),
        2 => Ok(PngChunkMarker::Trns),
        3 => Ok(PngChunkMarker::Gama),
        4 => Ok(PngChunkMarker::Chrm),
        5 => Ok(PngChunkMarker::Srgb),
        6 => Ok(PngChunkMarker::Phys),
        7 => Ok(PngChunkMarker::Time),
        8 => Ok(PngChunkMarker::Bkgd),
        9 => Ok(PngChunkMarker::Idat),
        10 => Ok(PngChunkMarker::Iend),
        11 => Ok(PngChunkMarker::Text { index: r.read_varint_u64()? as usize }),
        12 => Ok(PngChunkMarker::Unknown { index: r.read_varint_u64()? as usize }),
        other => Err(dsl::PackError::Malformed { what: "png chunk marker tag", offset: 0, detail: format!("unknown tag {other}") }),
    }
}
/// 🧩 2-way presence flag (`0`=None, `1`=Some) — shared by every plain `Option<T>` field.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_option<T>(w: &mut dsl::ByteWriter, v: &Option<T>, write_value: impl FnOnce(&mut dsl::ByteWriter, &T)) {
    match v {
        None => w.write_u8(0),
        Some(val) => {
            w.write_u8(1);
            write_value(w, val);
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_option<T>(r: &mut dsl::ByteReader<'_>, read_value: impl FnOnce(&mut dsl::ByteReader<'_>) -> Result<T, dsl::PackError>) -> Result<Option<T>, dsl::PackError> {
    match r.read_u8()? {
        0 => Ok(None),
        1 => Ok(Some(read_value(r)?)),
        other => Err(dsl::PackError::Malformed { what: "png binary option tag", offset: 0, detail: format!("unknown tag {other}") }),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_vec<T>(w: &mut dsl::ByteWriter, items: &[T], write_item: impl Fn(&mut dsl::ByteWriter, &T)) {
    w.write_varint_u64(items.len() as u64);
    for item in items {
        write_item(w, item);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_vec<T>(r: &mut dsl::ByteReader<'_>, mut read_item: impl FnMut(&mut dsl::ByteReader<'_>) -> Result<T, dsl::PackError>) -> Result<Vec<T>, dsl::PackError> {
    let n = r.read_varint_u64()? as usize;
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        out.push(read_item(r)?);
    }
    Ok(out)
}
/// 🧩 Whole-`PngSnapshot` real binary encoding — reused by `PngMutation::SetSnapshot`'s own
/// binary op arm (`🧬️mutations/🦀️component.rs`) so a full snapshot payload isn't hand-encoded
/// a second time.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_snapshot(w: &mut dsl::ByteWriter, s: &PngSnapshot) {
    write_bin_str(w, &s.schema);
    w.write_u32_le(s.width);
    w.write_u32_le(s.height);
    w.write_u8(s.bit_depth);
    w.write_u8(s.color_type.to_u8());
    w.write_u8(if s.interlace { 1 } else { 0 });
    write_bin_option(w, &s.plte, |w, v: &Vec<PngRgb>| write_bin_vec(w, v, write_bin_rgb));
    write_bin_option(w, &s.trns, write_bin_transparency);
    write_bin_option(w, &s.gama, |w, v: &u32| w.write_u32_le(*v));
    write_bin_option(w, &s.chrm, write_bin_chromaticities);
    write_bin_option(w, &s.srgb, |w, v: &PngSrgbIntent| w.write_u8(v.to_u8()));
    write_bin_option(w, &s.phys, write_bin_physical_dims);
    write_bin_option(w, &s.time, write_bin_timestamp);
    write_bin_option(w, &s.bkgd, write_bin_background);
    write_bin_vec(w, &s.text_chunks, write_bin_text_chunk);
    write_bin_blob(w, &s.pixels);
    write_bin_vec(w, &s.chunk_order, write_bin_chunk_marker);
    write_bin_vec(w, &s.unknown_chunks, write_bin_chunk);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_snapshot(r: &mut dsl::ByteReader<'_>) -> Result<PngSnapshot, dsl::PackError> {
    Ok(PngSnapshot {
        schema: read_bin_str(r)?,
        width: r.read_u32_le()?,
        height: r.read_u32_le()?,
        bit_depth: r.read_u8()?,
        color_type: PngColorType::from_u8(r.read_u8()?).map_err(|e| dsl::PackError::Malformed { what: "png color type", offset: 0, detail: e })?,
        interlace: r.read_u8()? != 0,
        plte: read_bin_option(r, |r| read_bin_vec(r, read_bin_rgb))?,
        trns: read_bin_option(r, read_bin_transparency)?,
        gama: read_bin_option(r, |r| r.read_u32_le())?,
        chrm: read_bin_option(r, read_bin_chromaticities)?,
        srgb: read_bin_option(r, |r| PngSrgbIntent::from_u8(r.read_u8()?).map_err(|e| dsl::PackError::Malformed { what: "png srgb intent", offset: 0, detail: e }))?,
        phys: read_bin_option(r, read_bin_physical_dims)?,
        time: read_bin_option(r, read_bin_timestamp)?,
        bkgd: read_bin_option(r, read_bin_background)?,
        text_chunks: read_bin_vec(r, read_bin_text_chunk)?,
        pixels: read_bin_blob(r)?,
        chunk_order: read_bin_vec(r, read_bin_chunk_marker)?,
        unknown_chunks: read_bin_vec(r, read_bin_chunk)?,
    })
}
//#endregion 🔖️RealBinaryPrimitives

//#region 🔖️RealBinaryDiffFrame
// 🧪️ P2-P2: real binary encodings for the three collection-triple diff types
// (`PngPlteDiff`/`PngTextChunksDiff`/`PngChunkOrderDiff`/`PngUnknownChunksDiff`) — each
// produces one opaque `Vec<u8>` blob matching `../💾️binary/📡️component.protocol.semio`'s
// `Array(u8, Field(<name>_len))` fields exactly (the blob's OWN internal removed/modified/
// added shape isn't further protocol-walkable, see that file's own doc comment).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_plte_diff_bin(d: &PngPlteDiff) -> Vec<u8> {
    let mut w = semio_framework_plugin::resolve_ready(dsl::ByteWriter::new());
    write_bin_vec(&mut w, &d.removed, |w, v: &usize| w.write_varint_u64(*v as u64));
    write_bin_vec(&mut w, &d.modified, |w, m: &PngPlteEntryModified| {
        w.write_varint_u64(m.index as u64);
        write_bin_rgb(w, &m.rgb);
    });
    write_bin_vec(&mut w, &d.added, |w, a: &PngPlteEntryAdded| {
        w.write_varint_u64(a.index as u64);
        write_bin_rgb(w, &a.rgb);
    });
    w.into_bytes()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_plte_diff_bin(bytes: &[u8]) -> Result<PngPlteDiff, dsl::PackError> {
    let mut r = semio_framework_plugin::resolve_ready(dsl::ByteReader::new(bytes));
    let removed = read_bin_vec(&mut r, |r| Ok(r.read_varint_u64()? as usize))?;
    let modified = read_bin_vec(&mut r, |r| {
        let index = r.read_varint_u64()? as usize;
        let rgb = read_bin_rgb(r)?;
        Ok(PngPlteEntryModified { index, rgb })
    })?;
    let added = read_bin_vec(&mut r, |r| {
        let index = r.read_varint_u64()? as usize;
        let rgb = read_bin_rgb(r)?;
        Ok(PngPlteEntryAdded { index, rgb })
    })?;
    Ok(PngPlteDiff { removed, modified, added })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_bin_text_chunk_diff(w: &mut dsl::ByteWriter, d: &PngTextChunkDiff) {
    write_bin_option(w, &d.keyword, |w, v: &String| write_bin_str(w, v));
    write_bin_option(w, &d.value, |w, v: &String| write_bin_str(w, v));
    write_bin_option(w, &d.compressed, |w, v: &bool| w.write_u8(if *v { 1 } else { 0 }));
    write_bin_option(w, &d.kind, |w, v: &PngTextKind| w.write_u8(enc_text_kind_u8(*v)));
    write_bin_option(w, &d.language_tag, |w, v: &String| write_bin_str(w, v));
    write_bin_option(w, &d.translated_keyword, |w, v: &String| write_bin_str(w, v));
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_bin_text_chunk_diff(r: &mut dsl::ByteReader<'_>) -> Result<PngTextChunkDiff, dsl::PackError> {
    Ok(PngTextChunkDiff {
        keyword: read_bin_option(r, read_bin_str)?,
        value: read_bin_option(r, read_bin_str)?,
        compressed: read_bin_option(r, |r| Ok(r.read_u8()? != 0))?,
        kind: read_bin_option(r, |r| dec_text_kind_u8(r.read_u8()?))?,
        language_tag: read_bin_option(r, read_bin_str)?,
        translated_keyword: read_bin_option(r, read_bin_str)?,
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_text_chunks_diff_bin(d: &PngTextChunksDiff) -> Vec<u8> {
    let mut w = semio_framework_plugin::resolve_ready(dsl::ByteWriter::new());
    write_bin_vec(&mut w, &d.removed, |w, v: &usize| w.write_varint_u64(*v as u64));
    write_bin_vec(&mut w, &d.modified, |w, m: &PngTextChunkModified| {
        w.write_varint_u64(m.index as u64);
        write_bin_text_chunk_diff(w, &m.diff);
    });
    write_bin_vec(&mut w, &d.added, |w, a: &PngTextChunkAdded| {
        w.write_varint_u64(a.index as u64);
        write_bin_text_chunk(w, &a.chunk);
    });
    w.into_bytes()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_text_chunks_diff_bin(bytes: &[u8]) -> Result<PngTextChunksDiff, dsl::PackError> {
    let mut r = semio_framework_plugin::resolve_ready(dsl::ByteReader::new(bytes));
    let removed = read_bin_vec(&mut r, |r| Ok(r.read_varint_u64()? as usize))?;
    let modified = read_bin_vec(&mut r, |r| {
        let index = r.read_varint_u64()? as usize;
        let diff = read_bin_text_chunk_diff(r)?;
        Ok(PngTextChunkModified { index, diff })
    })?;
    let added = read_bin_vec(&mut r, |r| {
        let index = r.read_varint_u64()? as usize;
        let chunk = read_bin_text_chunk(r)?;
        Ok(PngTextChunkAdded { index, chunk })
    })?;
    Ok(PngTextChunksDiff { removed, modified, added })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_chunk_order_diff_bin(d: &PngChunkOrderDiff) -> Vec<u8> {
    let mut w = semio_framework_plugin::resolve_ready(dsl::ByteWriter::new());
    write_bin_vec(&mut w, &d.removed, |w, v: &usize| w.write_varint_u64(*v as u64));
    write_bin_vec(&mut w, &d.modified, |w, m: &PngChunkOrderModified| {
        w.write_varint_u64(m.index as u64);
        write_bin_chunk_marker(w, &m.marker);
    });
    write_bin_vec(&mut w, &d.added, |w, a: &PngChunkOrderAdded| {
        w.write_varint_u64(a.index as u64);
        write_bin_chunk_marker(w, &a.marker);
    });
    w.into_bytes()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_chunk_order_diff_bin(bytes: &[u8]) -> Result<PngChunkOrderDiff, dsl::PackError> {
    let mut r = semio_framework_plugin::resolve_ready(dsl::ByteReader::new(bytes));
    let removed = read_bin_vec(&mut r, |r| Ok(r.read_varint_u64()? as usize))?;
    let modified = read_bin_vec(&mut r, |r| {
        let index = r.read_varint_u64()? as usize;
        let marker = read_bin_chunk_marker(r)?;
        Ok(PngChunkOrderModified { index, marker })
    })?;
    let added = read_bin_vec(&mut r, |r| {
        let index = r.read_varint_u64()? as usize;
        let marker = read_bin_chunk_marker(r)?;
        Ok(PngChunkOrderAdded { index, marker })
    })?;
    Ok(PngChunkOrderDiff { removed, modified, added })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_unknown_chunks_diff_bin(d: &PngUnknownChunksDiff) -> Vec<u8> {
    let mut w = semio_framework_plugin::resolve_ready(dsl::ByteWriter::new());
    write_bin_vec(&mut w, &d.removed, |w, v: &usize| w.write_varint_u64(*v as u64));
    write_bin_vec(&mut w, &d.modified, |w, m: &PngUnknownChunkModified| {
        w.write_varint_u64(m.index as u64);
        write_bin_chunk(w, &m.chunk);
    });
    write_bin_vec(&mut w, &d.added, |w, a: &PngUnknownChunkAdded| {
        w.write_varint_u64(a.index as u64);
        write_bin_chunk(w, &a.chunk);
    });
    w.into_bytes()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_unknown_chunks_diff_bin(bytes: &[u8]) -> Result<PngUnknownChunksDiff, dsl::PackError> {
    let mut r = semio_framework_plugin::resolve_ready(dsl::ByteReader::new(bytes));
    let removed = read_bin_vec(&mut r, |r| Ok(r.read_varint_u64()? as usize))?;
    let modified = read_bin_vec(&mut r, |r| {
        let index = r.read_varint_u64()? as usize;
        let chunk = read_bin_chunk(r)?;
        Ok(PngUnknownChunkModified { index, chunk })
    })?;
    let added = read_bin_vec(&mut r, |r| {
        let index = r.read_varint_u64()? as usize;
        let chunk = read_bin_chunk(r)?;
        Ok(PngUnknownChunkAdded { index, chunk })
    })?;
    Ok(PngUnknownChunksDiff { removed, modified, added })
}
/// 🧩 3-way flag (`0`=unchanged, `1`=cleared-to-`None`, `2`=set-to-`Some(value)`) for every
/// TRI-STATE `Option<Option<T>>` field — see the protocol file's own doc comment for why this
/// avoids chaining two `if`-guarded conditional fields (`Cond::eval` errors on a field that
/// was itself only conditionally decoded).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_bin_tri_flag<T>(w: &mut dsl::ByteWriter, v: &Option<Option<T>>, write_value: impl FnOnce(&mut dsl::ByteWriter, &T)) {
    match v {
        None => w.write_u8(0),
        Some(None) => w.write_u8(1),
        Some(Some(val)) => {
            w.write_u8(2);
            write_value(w, val);
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_bin_tri_flag<T>(r: &mut dsl::ByteReader<'_>, read_value: impl FnOnce(&mut dsl::ByteReader<'_>) -> Result<T, dsl::PackError>) -> Result<Option<Option<T>>, dsl::PackError> {
    match r.read_u8()? {
        0 => Ok(None),
        1 => Ok(Some(None)),
        2 => Ok(Some(Some(read_value(r)?))),
        other => Err(dsl::PackError::Malformed { what: "png diff tri-flag", offset: 0, detail: format!("unknown flag {other}") }),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_pack_err(e: dsl::PackError) -> protocol::ProtocolError {
    protocol::ProtocolError::Malformed { what: "png diff binary", offset: 0, detail: e.to_string() }
}
//#endregion 🔖️RealBinaryDiffFrame

//#region 🔖️TopLevel
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_png_diff(d: &PngDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = d.width {
        tokens.push(format!("width={v}"));
    }
    if let Some(v) = d.height {
        tokens.push(format!("height={v}"));
    }
    if let Some(v) = d.bit_depth {
        tokens.push(format!("bit-depth={v}"));
    }
    if let Some(v) = d.color_type {
        tokens.push(format!("color-type={}", enc_color_type(v)));
    }
    if let Some(v) = d.interlace {
        tokens.push(format!("interlace={}", if v { 1 } else { 0 }));
    }
    if let Some(v) = &d.plte {
        tokens.push(format!("plte={}", encode_option(v, enc_plte_body)));
    }
    if let Some(v) = &d.trns {
        tokens.push(format!("trns={}", encode_option(v, enc_transparency)));
    }
    if let Some(v) = &d.gama {
        tokens.push(format!("gama={}", encode_option(v, |x: &u32| x.to_string())));
    }
    if let Some(v) = &d.chrm {
        tokens.push(format!("chrm={}", encode_option(v, enc_chromaticities)));
    }
    if let Some(v) = &d.srgb {
        tokens.push(format!("srgb={}", encode_option(v, |s: &PngSrgbIntent| enc_srgb_intent(*s))));
    }
    if let Some(v) = &d.phys {
        tokens.push(format!("phys={}", encode_option(v, enc_physical_dims)));
    }
    if let Some(v) = &d.time {
        tokens.push(format!("time={}", encode_option(v, enc_timestamp)));
    }
    if let Some(v) = &d.bkgd {
        tokens.push(format!("bkgd={}", encode_option(v, enc_background)));
    }
    if let Some(v) = &d.text_chunks {
        tokens.push(enc_text_chunks_diff(v));
    }
    if let Some(v) = &d.pixels {
        tokens.push(format!("pixels={}", hex_encode(v)));
    }
    if let Some(v) = &d.chunk_order {
        tokens.push(enc_chunk_order_diff(v));
    }
    if let Some(v) = &d.unknown_chunks {
        tokens.push(enc_unknown_chunks_diff(v));
    }
    tokens.join(" ")
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_png_diff(line: &str) -> Result<PngDiff, String> {
    let mut d = PngDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("width=") {
            d.width = Some(parse_u32(rest)?);
        } else if let Some(rest) = token.strip_prefix("height=") {
            d.height = Some(parse_u32(rest)?);
        } else if let Some(rest) = token.strip_prefix("bit-depth=") {
            d.bit_depth = Some(parse_u8(rest)?);
        } else if let Some(rest) = token.strip_prefix("color-type=") {
            d.color_type = Some(dec_color_type(rest)?);
        } else if let Some(rest) = token.strip_prefix("interlace=") {
            d.interlace = Some(rest == "1");
        } else if let Some(rest) = token.strip_prefix("plte=") {
            d.plte = Some(decode_option(rest, dec_plte_body)?);
        } else if let Some(rest) = token.strip_prefix("trns=") {
            d.trns = Some(decode_option(rest, dec_transparency)?);
        } else if let Some(rest) = token.strip_prefix("gama=") {
            d.gama = Some(decode_option(rest, parse_u32)?);
        } else if let Some(rest) = token.strip_prefix("chrm=") {
            d.chrm = Some(decode_option(rest, dec_chromaticities)?);
        } else if let Some(rest) = token.strip_prefix("srgb=") {
            d.srgb = Some(decode_option(rest, dec_srgb_intent)?);
        } else if let Some(rest) = token.strip_prefix("phys=") {
            d.phys = Some(decode_option(rest, dec_physical_dims)?);
        } else if let Some(rest) = token.strip_prefix("time=") {
            d.time = Some(decode_option(rest, dec_timestamp)?);
        } else if let Some(rest) = token.strip_prefix("bkgd=") {
            d.bkgd = Some(decode_option(rest, dec_background)?);
        } else if let Some(rest) = token.strip_prefix("text-chunks{") {
            d.text_chunks = Some(dec_text_chunks_diff(rest.strip_suffix('}').ok_or_else(|| "text-chunks: missing closing brace".to_string())?)?);
        } else if let Some(rest) = token.strip_prefix("pixels=") {
            d.pixels = Some(hex_decode(rest)?);
        } else if let Some(rest) = token.strip_prefix("chunk-order{") {
            d.chunk_order = Some(dec_chunk_order_diff(rest.strip_suffix('}').ok_or_else(|| "chunk-order: missing closing brace".to_string())?)?);
        } else if let Some(rest) = token.strip_prefix("unknown-chunks{") {
            d.unknown_chunks = Some(dec_unknown_chunks_diff(rest.strip_suffix('}').ok_or_else(|| "unknown-chunks: missing closing brace".to_string())?)?);
        } else {
            return Err(format!("png diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl protocol::DiffCodec for PngDiff {
    async fn print_diff(&self) -> String {
        print_png_diff(self)
    }
    async fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        parse_png_diff(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }

    /// ⚡️ P2-P2: real binary diff-frame — upgraded from the F6-era `print_diff().into_bytes()`
    /// text-as-binary shortcut. Matches `../💾️binary/📡️component.protocol.semio`'s real
    /// flag-per-field layout exactly, field for field, in struct order (see that file's own
    /// doc comment for the 2-way/3-way flag design).
    async fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let mut w = dsl::ByteWriter::new().await;
        write_bin_option(&mut w, &self.width, |w, v| w.write_u32_le(*v));
        write_bin_option(&mut w, &self.height, |w, v| w.write_u32_le(*v));
        write_bin_option(&mut w, &self.bit_depth, |w, v| w.write_u8(*v));
        write_bin_option(&mut w, &self.color_type, |w, v| w.write_u8(v.to_u8()));
        write_bin_option(&mut w, &self.interlace, |w, v| w.write_u8(if *v { 1 } else { 0 }));

        write_bin_tri_flag(&mut w, &self.plte, |w, v| write_bin_blob(w, &enc_plte_diff_bin(v)));
        write_bin_tri_flag(&mut w, &self.trns, |w, v| {
            let mut inner = semio_framework_plugin::resolve_ready(dsl::ByteWriter::new());
            write_bin_transparency(&mut inner, v);
            write_bin_blob(w, &semio_framework_plugin::resolve_ready(inner.into_bytes()));
        });
        write_bin_tri_flag(&mut w, &self.gama, |w, v| w.write_u32_le(*v));
        write_bin_tri_flag(&mut w, &self.chrm, |w, v| write_bin_chromaticities(w, v));
        write_bin_tri_flag(&mut w, &self.srgb, |w, v| w.write_u8(v.to_u8()));
        write_bin_tri_flag(&mut w, &self.phys, |w, v| write_bin_physical_dims(w, v));
        write_bin_tri_flag(&mut w, &self.time, |w, v| write_bin_timestamp(w, v));
        write_bin_tri_flag(&mut w, &self.bkgd, |w, v| {
            let mut inner = semio_framework_plugin::resolve_ready(dsl::ByteWriter::new());
            write_bin_background(&mut inner, v);
            write_bin_blob(w, &semio_framework_plugin::resolve_ready(inner.into_bytes()));
        });

        write_bin_option(&mut w, &self.text_chunks, |w, v| write_bin_blob(w, &enc_text_chunks_diff_bin(v)));
        write_bin_option(&mut w, &self.pixels, |w, v| write_bin_blob(w, v));
        write_bin_option(&mut w, &self.chunk_order, |w, v| write_bin_blob(w, &enc_chunk_order_diff_bin(v)));
        write_bin_option(&mut w, &self.unknown_chunks, |w, v| write_bin_blob(w, &enc_unknown_chunks_diff_bin(v)));

        Ok(w.into_bytes().await)
    }
    async fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut r = semio_framework_plugin::resolve_ready(dsl::ByteReader::new(bytes));
        let width = read_bin_option(&mut r, |r| r.read_u32_le()).map_err(diff_pack_err)?;
        let height = read_bin_option(&mut r, |r| r.read_u32_le()).map_err(diff_pack_err)?;
        let bit_depth = read_bin_option(&mut r, |r| r.read_u8()).map_err(diff_pack_err)?;
        let color_type = read_bin_option(&mut r, |r| PngColorType::from_u8(r.read_u8()?).map_err(|e| dsl::PackError::Malformed { what: "png diff color type", offset: 0, detail: e })).map_err(diff_pack_err)?;
        let interlace = read_bin_option(&mut r, |r| Ok(r.read_u8()? != 0)).map_err(diff_pack_err)?;

        let plte = read_bin_tri_flag(&mut r, |r| dec_plte_diff_bin(&read_bin_blob(r)?)).map_err(diff_pack_err)?;
        let trns = read_bin_tri_flag(&mut r, |r| {
            let blob = read_bin_blob(r)?;
            let mut inner = semio_framework_plugin::resolve_ready(dsl::ByteReader::new(&blob));
            read_bin_transparency(&mut inner)
        })
        .map_err(diff_pack_err)?;
        let gama = read_bin_tri_flag(&mut r, |r| r.read_u32_le()).map_err(diff_pack_err)?;
        let chrm = read_bin_tri_flag(&mut r, read_bin_chromaticities).map_err(diff_pack_err)?;
        let srgb = read_bin_tri_flag(&mut r, |r| PngSrgbIntent::from_u8(r.read_u8()?).map_err(|e| dsl::PackError::Malformed { what: "png diff srgb intent", offset: 0, detail: e })).map_err(diff_pack_err)?;
        let phys = read_bin_tri_flag(&mut r, read_bin_physical_dims).map_err(diff_pack_err)?;
        let time = read_bin_tri_flag(&mut r, read_bin_timestamp).map_err(diff_pack_err)?;
        let bkgd = read_bin_tri_flag(&mut r, |r| {
            let blob = read_bin_blob(r)?;
            let mut inner = semio_framework_plugin::resolve_ready(dsl::ByteReader::new(&blob));
            read_bin_background(&mut inner)
        })
        .map_err(diff_pack_err)?;

        let text_chunks = read_bin_option(&mut r, |r| dec_text_chunks_diff_bin(&read_bin_blob(r)?)).map_err(diff_pack_err)?;
        let pixels = read_bin_option(&mut r, |r| read_bin_blob(r)).map_err(diff_pack_err)?;
        let chunk_order = read_bin_option(&mut r, |r| dec_chunk_order_diff_bin(&read_bin_blob(r)?)).map_err(diff_pack_err)?;
        let unknown_chunks = read_bin_option(&mut r, |r| dec_unknown_chunks_diff_bin(&read_bin_blob(r)?)).map_err(diff_pack_err)?;

        Ok(PngDiff { width, height, bit_depth, color_type, interlace, plte, trns, gama, chrm, srgb, phys, time, bkgd, text_chunks, pixels, chunk_order, unknown_chunks })
    }
}
//#endregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec

//#region 🔖️DemoDiffCases
/// 🧪️ P2-P2: shared demo diff fixtures — `diff_grammar_conformance_law`/`protocol_walk_law`
/// (`⚙️engine/🦀️component.rs`'s `conformance_laws` module) call this directly instead of
/// duplicating the literal case list; `handcrafted_diff_codec_tests::diff_codec_text_binary_
/// roundtrip_law` below now calls it too (single source of truth, per CLAUDE.md).
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_snap_a() -> PngSnapshot {
    PngSnapshot {
        schema: "stdio.png".into(),
        width: 10,
        height: 20,
        bit_depth: 8,
        color_type: PngColorType::Rgba,
        interlace: false,
        plte: Some(vec![PngRgb { r: 1, g: 1, b: 1 }, PngRgb { r: 2, g: 2, b: 2 }]),
        trns: Some(PngTransparency::Grayscale { gray: 5 }),
        gama: Some(45455),
        chrm: Some(PngChromaticities { white_x: 1, white_y: 2, red_x: 3, red_y: 4, green_x: 5, green_y: 6, blue_x: 7, blue_y: 8 }),
        srgb: Some(PngSrgbIntent::Perceptual),
        phys: Some(PngPhysicalDims { ppu_x: 100, ppu_y: 100, unit_is_meter: true }),
        time: Some(PngTimestamp { year: 2020, month: 1, day: 1, hour: 0, minute: 0, second: 0 }),
        bkgd: Some(PngBackground::Grayscale { gray: 255 }),
        text_chunks: vec![
            PngTextChunk { keyword: "Author".into(), value: "orig".into(), compressed: false, kind: PngTextKind::Text, language_tag: String::new(), translated_keyword: String::new() },
            PngTextChunk { keyword: "Trash".into(), value: "gone".into(), compressed: false, kind: PngTextKind::Text, language_tag: String::new(), translated_keyword: String::new() },
        ],
        pixels: vec![0u8, 0, 0, 255, 255, 255, 255, 255],
        chunk_order: vec![PngChunkMarker::Gama, PngChunkMarker::Chrm, PngChunkMarker::Text { index: 0 }],
        unknown_chunks: vec![PngChunk { kind: *b"prIV", data: vec![1, 2, 3] }, PngChunk { kind: *b"gone", data: vec![9, 9] }],
    }
}
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_snap_b() -> PngSnapshot {
    PngSnapshot {
        schema: "stdio.png".into(),
        width: 11,
        height: 21,
        bit_depth: 16,
        color_type: PngColorType::Palette,
        interlace: true,
        plte: Some(vec![PngRgb { r: 9, g: 9, b: 9 }]),
        trns: None,
        gama: None,
        chrm: None,
        srgb: Some(PngSrgbIntent::AbsoluteColorimetric),
        phys: None,
        time: None,
        bkgd: None,
        text_chunks: vec![PngTextChunk { keyword: "Creator".into(), value: "changed".into(), compressed: true, kind: PngTextKind::IText, language_tag: "en".into(), translated_keyword: "Auteur".into() }],
        pixels: vec![1u8, 1, 1, 255],
        chunk_order: vec![PngChunkMarker::Srgb, PngChunkMarker::Unknown { index: 0 }],
        unknown_chunks: vec![PngChunk { kind: *b"prIV", data: vec![4, 5, 6] }],
    }
}
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_empty_snap() -> PngSnapshot {
    PngSnapshot { schema: "stdio.png".into(), ..Default::default() }
}
/// ✅️ Representative `PngDiff` cases, incl. the empty (`None`) diff and the `Replace`-shaped
/// transitions to/from an all-defaults snapshot — exercises every scalar, every tri-state
/// (both `Some(Some) -> Some(None)` and `None -> Some(Some)`), and every collection triple's
/// removed/modified/added arms.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<PngDiff> {
    let a = demo_snap_a();
    let b = demo_snap_b();
    let c = demo_empty_snap();
    vec![PngDiff::default(), PngDiff::between(&a, &b), PngDiff::between(&b, &a), PngDiff::between(&a, &c), PngDiff::between(&c, &a)]
}
//#endregion 🔖️DemoDiffCases

//#region 🧪️Tests
#[cfg(test)]
mod handcrafted_diff_codec_tests {
    use super::*;
    use protocol::DiffCodec;

    /// 🧪️ `DiffCodec` round-trip laws over the hand-rolled `PngDiff` grammar AND the real binary
    /// frame (`demo_diff_cases()` above — `snap_a`/`snap_b` differ in every mutable field,
    /// plus transitions to/from an all-defaults snapshot) — exercises every scalar field, every
    /// tri-state `Some(None)`/`Some(Some(_))` transition (incl. `plte`'s tri-state-wrapping-a-
    /// triple shape), and every collection triple's removed/modified/added arms.
    #[semio_framework_async_macros::async_test]
    async fn diff_codec_text_binary_roundtrip_law() {
        for d in demo_diff_cases() {
            let printed = d.print_diff();
            assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
            let parsed = PngDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
            assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

            let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
            let decoded = PngDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
            assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
        }
    }
}
//#endregion 🧪️Tests
