//! 🔺️ TiffDiff — handcrafted sparse diff. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: replaces the old
//! `TiffDiff{snapshot: Option<TiffSnapshot>}` full-replace template. `ifds` is an INDEX-keyed
//! `removed`/`modified`/`added` triple (TIFF's own IFD chain is positional); within each IFD,
//! `entries` is a TAG-ID-keyed triple (`tag: u16`, not array index — tag SETS can differ in
//! size between two IFDs, and tags are TIFF's own natural stable identity, unlike an ordinal
//! position). A `TiffTag` is a weak value (`kind`/`values` move together atomically), so a
//! tag-triple's `modified`/`added` payload carries the whole new tag, never a nested diff.


use crate::schema::snapshot::{TiffFieldType, TiffIfd, TiffSampleBlock, TiffWord64, TiffTag, TiffValues};
use crate::TiffSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use std::collections::{BTreeMap, BTreeSet, HashMap};

//#region 🔖️TagsTriple
/// 🏷️ One `entries.modified[]`/`.added[]` entity — `TiffTag` is a weak value, so both carry
/// the entry's NEW `kind`/`values` directly (never a nested per-field diff).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct TiffTagModified {
    pub tag: u16,
    pub values: TiffValues,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct TiffTagAdded {
    pub tag: u16,
    pub values: TiffValues,
}

/// 🔺️ Tag-id-keyed `entries` triple for one IFD.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct TiffTagsDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<u16>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<TiffTagModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<TiffTagAdded>,
}

impl TiffTagsDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
}

/// ▶️ Applies a tag-id-keyed triple to one IFD's entries. TIFF6 §2 requires ascending-tag-
/// order within an IFD — `apply` re-sorts on every call, keeping that invariant regardless of
/// the triple's own insertion order.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_tags(base: &[TiffTag], d: &TiffTagsDiff) -> Vec<TiffTag> {
    let mut items: Vec<TiffTag> = base.iter().filter(|t| !d.removed.contains(&t.tag)).cloned().collect();
    for m in &d.modified {
        if let Some(it) = items.iter_mut().find(|t| t.tag == m.tag) {
            it.values = m.values.clone();
        }
    }
    for a in &d.added {
        if let Some(it) = items.iter_mut().find(|t| t.tag == a.tag) {
            it.values = a.values.clone();
        } else {
            items.push(TiffTag { tag: a.tag, values: a.values.clone() });
        }
    }
    items.sort_by_key(|t| t.tag);
    items
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
                if at.values != bt.values {
                    modified.push(TiffTagModified { tag: *tag, values: bt.values.clone() });
                }
            }
        }
    }
    for (tag, bt) in &b_map {
        if !a_map.contains_key(tag) {
            added.push(TiffTagAdded { tag: *tag, values: bt.values.clone() });
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

//#region 🔖️SampleRuns
/// 🧱️ One contiguous run of replacement words inside block `block` of an IFD, starting at word `offset` of the block's pixel-major sample list.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct TiffSampleRun {
    pub block: usize,
    pub offset: usize,
    pub samples: Vec<TiffWord64>,
}

/// 🔍️ The maximal runs of `next` that differ from `current` over the same span, positioned from `offset` inside `block`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn differing_runs(block: usize, offset: usize, current: &[TiffWord64], next: &[TiffWord64]) -> Vec<TiffSampleRun> {
    let mut runs = Vec::new();
    let mut start = None;
    for index in 0..=current.len().min(next.len()) {
        let differs = index < current.len().min(next.len()) && current[index] != next[index];
        match (start, differs) {
            (None, true) => start = Some(index),
            (Some(from), false) => {
                runs.push(TiffSampleRun { block, offset: offset + from, samples: next[from..index].to_vec() });
                start = None;
            }
            _ => {}
        }
    }
    runs
}

/// 📐️ The normal form of a run list: ordered by block then offset, with touching runs fused into one.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn normalize_runs(mut runs: Vec<TiffSampleRun>) -> Vec<TiffSampleRun> {
    runs.retain(|run| !run.samples.is_empty());
    runs.sort_by_key(|run| (run.block, run.offset));
    let mut fused: Vec<TiffSampleRun> = Vec::with_capacity(runs.len());
    for run in runs {
        match fused.last_mut() {
            Some(last) if last.block == run.block && last.offset + last.samples.len() == run.offset => last.samples.extend(run.samples),
            _ => fused.push(run),
        }
    }
    fused
}

/// 🧩️ `second` written over `first`: the words of `first` that `second` covers are cut away before both are normalized together.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn overlay_runs(first: Vec<TiffSampleRun>, second: Vec<TiffSampleRun>) -> Vec<TiffSampleRun> {
    let mut pieces = first;
    for run in &second {
        let (from, to) = (run.offset, run.offset + run.samples.len());
        pieces = pieces
            .into_iter()
            .flat_map(|piece| {
                let (start, end) = (piece.offset, piece.offset + piece.samples.len());
                if piece.block != run.block || end <= from || start >= to {
                    return vec![piece];
                }
                let mut parts = Vec::new();
                if start < from {
                    parts.push(TiffSampleRun { block: piece.block, offset: start, samples: piece.samples[..from - start].to_vec() });
                }
                if end > to {
                    parts.push(TiffSampleRun { block: piece.block, offset: to, samples: piece.samples[to - start..].to_vec() });
                }
                parts
            })
            .collect();
    }
    pieces.extend(second);
    normalize_runs(pieces)
}

/// ▶️ Writes the runs into `blocks`, refusing a run that leaves its block.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_runs(blocks: &mut [TiffSampleBlock], runs: &[TiffSampleRun]) -> bool {
    runs.iter().all(|run| match blocks.get_mut(run.block).and_then(|block| block.samples.get_mut(run.offset..run.offset + run.samples.len())) {
        Some(span) => {
            span.copy_from_slice(&run.samples);
            true
        }
        None => false,
    })
}

/// ↩️ The runs that put `blocks` back: for every word a run really changes, the word `blocks` carries, fused into maximal runs.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_runs(runs: &[TiffSampleRun], blocks: &[TiffSampleBlock]) -> Vec<TiffSampleRun> {
    let restored = runs.iter().filter_map(|run| {
        let base = blocks.get(run.block)?.samples.get(run.offset..run.offset + run.samples.len())?;
        Some(differing_runs(run.block, run.offset, &run.samples, base))
    });
    normalize_runs(restored.flatten().collect())
}
//#endregion 🔖️SampleRuns

//#region 🔖️IfdsTriple
/// 🗂️ The per-IFD delta: the recursive tag-triple, a whole-value slot for that directory's raw block list (`TiffIfd::blocks`,
/// replaced wholesale when the block geometry itself changes) and the sparse sample runs that rewrite words inside the
/// surviving blocks (applied after the block list, so a run always addresses the block list the diff leaves behind).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct TiffIfdDiff {
    #[value(default, skip_serializing_if = "TiffTagsDiff::is_empty")]
    pub entries: TiffTagsDiff,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub blocks: Option<Vec<TiffSampleBlock>>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub runs: Vec<TiffSampleRun>,
}

impl TiffIfdDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty() && self.blocks.is_none() && self.runs.is_empty()
    }
}

/// 🗂️ One `ifds.modified[]` entity — the recursive per-IFD delta.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct TiffIfdModified {
    pub index: usize,
    pub diff: TiffIfdDiff,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct TiffIfdAdded {
    pub index: usize,
    pub ifd: TiffIfd,
}

/// 🔺️ Index-keyed `ifds` triple (TIFF's IFD chain is positional).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct TiffIfdsDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<TiffIfdModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
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
fn absorb_ifds(d1: TiffIfdsDiff, d2: TiffIfdsDiff) -> TiffIfdsDiff {
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
    let mut modified_map: BTreeMap<usize, TiffIfdDiff> = d1.modified.into_iter().map(|m| (m.index, m.diff)).collect();
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
                entry.entries = absorb_tags(entry.entries.clone(), m2.diff.entries.clone());
                absorb_blocks(&mut entry.blocks, &mut entry.runs, &m2.diff);
            }
            Some(Slot::Added(ai)) => {
                if let Some(a) = added_alive[*ai].as_mut() {
                    a.ifd.entries = apply_tags(&a.ifd.entries, &m2.diff.entries);
                    if let Some(storage) = &m2.diff.blocks {
                        a.ifd.blocks = storage.clone();
                    }
                    write_runs(&mut a.ifd.blocks, &m2.diff.runs);
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
    let mut final_modified: Vec<TiffIfdModified> = modified_map.into_iter().filter(|(_, d)| !d.is_empty()).map(|(index, diff)| TiffIfdModified { index, diff }).collect();
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_ifds(base: &[TiffIfd], d: &TiffIfdsDiff) -> Vec<TiffIfd> {
    let mut items = base.to_vec();
    for m in &d.modified {
        if let Some(it) = items.get_mut(m.index) {
            it.entries = apply_tags(&it.entries, &m.diff.entries);
            if let Some(storage) = &m.diff.blocks {
                it.blocks = storage.clone();
            }
            write_runs(&mut it.blocks, &m.diff.runs);
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

/// 📐️ Whether `b` keeps the block geometry of `a` (count, rectangles, channels, word counts), so that only sample words can differ.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn same_block_shape(a: &[TiffSampleBlock], b: &[TiffSampleBlock]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(x, y)| (x.x, x.y, x.width, x.height, x.channels, x.samples.len()) == (y.x, y.y, y.width, y.height, y.channels, y.samples.len()))
}

/// 🧭️ The block delta from `a` to `b`: sparse runs when the block geometry is untouched, the whole new block list otherwise.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_blocks(a: &[TiffSampleBlock], b: &[TiffSampleBlock]) -> (Option<Vec<TiffSampleBlock>>, Vec<TiffSampleRun>) {
    if a == b {
        return (None, Vec::new());
    }
    if !same_block_shape(a, b) {
        return (Some(b.to_vec()), Vec::new());
    }
    (None, normalize_runs(a.iter().zip(b).enumerate().flat_map(|(index, (x, y))| differing_runs(index, 0, &x.samples, &y.samples)).collect()))
}

/// ➕️ Folds `later`'s block delta into an earlier one: a replacement block list wins and discards earlier runs, runs land in an owned block list or overlay earlier runs.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_blocks(blocks: &mut Option<Vec<TiffSampleBlock>>, runs: &mut Vec<TiffSampleRun>, later: &TiffIfdDiff) {
    if let Some(replacement) = &later.blocks {
        *blocks = Some(replacement.clone());
        runs.clear();
    }
    match blocks {
        Some(owned) => {
            write_runs(owned, &later.runs);
        }
        None => *runs = overlay_runs(std::mem::take(runs), later.runs.clone()),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_ifds(a: &[TiffIfd], b: &[TiffIfd]) -> Option<TiffIfdsDiff> {
    let min = a.len().min(b.len());
    let mut modified = Vec::new();
    for i in 0..min {
        let (blocks, runs) = between_blocks(&a[i].blocks, &b[i].blocks);
        let diff = TiffIfdDiff { entries: between_tags(&a[i].entries, &b[i].entries).unwrap_or_default(), blocks, runs };
        if !diff.is_empty() {
            modified.push(TiffIfdModified { index: i, diff });
        }
    }
    let removed: Vec<usize> = (min..a.len()).collect();
    let added: Vec<TiffIfdAdded> = (min..b.len()).map(|i| TiffIfdAdded { index: i, ifd: b[i].clone() }).collect();
    if removed.is_empty() && modified.is_empty() && added.is_empty() {
        None
    } else {
        Some(TiffIfdsDiff { removed, modified, added })
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_tags(diff: &TiffTagsDiff, base: &[TiffTag]) -> TiffTagsDiff {
    let find = |tag: &u16| base.iter().find(|entry| entry.tag == *tag);
    let mut removed: Vec<u16> = diff.added.iter().map(|added| added.tag).collect();
    removed.sort_unstable();
    let mut modified: Vec<TiffTagModified> = diff.modified.iter().filter_map(|row| find(&row.tag).filter(|entry| entry.values != row.values).map(|entry| TiffTagModified { tag: row.tag, values: entry.values.clone() })).collect();
    modified.sort_by_key(|row| row.tag);
    let mut added: Vec<TiffTagAdded> = diff.removed.iter().filter_map(|tag| find(tag).map(|entry| TiffTagAdded { tag: *tag, values: entry.values.clone() })).collect();
    added.sort_by_key(|row| row.tag);
    TiffTagsDiff { removed, modified, added }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_ifd_diff(diff: &TiffIfdDiff, base: &TiffIfd) -> TiffIfdDiff {
    let entries = inverse_tags(&diff.entries, &base.entries);
    match diff.blocks.as_ref().filter(|blocks| **blocks != base.blocks) {
        Some(_) => TiffIfdDiff { entries, blocks: Some(base.blocks.clone()), runs: Vec::new() },
        None => TiffIfdDiff { entries, blocks: None, runs: inverse_runs(&diff.runs, &base.blocks) },
    }
}

/// ↩️ Negative rows for the IFD triple against its BASE directories: added rows become removals at their final index, removed rows return at their
/// base index, and each modified row restores its base value at the index the row has after the diff. Every list comes back ascending.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_ifds(diff: &TiffIfdsDiff, base: &[TiffIfd]) -> TiffIfdsDiff {
    let mut removed_sorted = diff.removed.clone();
    removed_sorted.sort_unstable();
    removed_sorted.dedup();
    let mut added_final: Vec<usize> = diff.added.iter().map(|added| added.index).collect();
    added_final.sort_unstable();
    let after_index = |index: usize| {
        let survivor = index - removed_sorted.iter().filter(|dropped| **dropped < index).count();
        added_final.iter().fold(survivor, |position, inserted| if *inserted <= position { position + 1 } else { position })
    };
    let mut modified: Vec<TiffIfdModified> = diff.modified.iter().filter_map(|row| base.get(row.index).map(|ifd| TiffIfdModified { index: after_index(row.index), diff: inverse_ifd_diff(&row.diff, ifd) })).filter(|row| !row.diff.is_empty()).collect();
    modified.sort_by_key(|row| row.index);
    let added = removed_sorted.iter().filter_map(|index| base.get(*index).map(|ifd| TiffIfdAdded { index: *index, ifd: ifd.clone() })).collect();
    TiffIfdsDiff { removed: added_final, modified, added }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_ifds_opt(base: &mut Option<TiffIfdsDiff>, other: Option<TiffIfdsDiff>) {
    match (base.take(), other) {
        (None, o) => *base = o,
        (Some(b), None) => *base = Some(b),
        (Some(b), Some(o)) => *base = Some(absorb_ifds(b, o)),
    }
}
//#endregion 🔖️IfdsTriple

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.tiff`: the index-keyed IFD triple. `TiffValues` is a data-carrying enum that `DslField` cannot express, so the
/// text and binary diff codecs frame the value tree directly instead of deriving a field grammar.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[artifact_schema(id = "s.stdio.tiff.diff")]
pub struct TiffDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub ifds: Option<TiffIfdsDiff>,
}

impl MutationDiff<TiffSnapshot> for TiffDiff {
    fn apply(&self, base: &TiffSnapshot, _capability: protocol::ApplyCapability) -> MutationApplyResult<TiffSnapshot> {
        if let Some(ifds) = &self.ifds {
            validate_tiff_ifds(&base.ifds, ifds)?;
        }
        let mut next = base.clone();
        if let Some(d) = &self.ifds {
            next.ifds = apply_ifds(&next.ifds, d);
        }
        next.validate().map_err(|message| protocol::MutationApplyError::new("mutation.apply.invalid-samples", message).at(["ifds"]))?;
        Ok(next)
    }

    /// ➕️ Structural, total, base-free sequential-coalesce (`## Absorb` contract).
    /// `byte_order` is LWW; `ifds` uses an index-transported merge with nested tag-id-keyed and
    /// canonical-storage replacement for modified entries.
    fn absorb(&mut self, other: Self) {
        absorb_ifds_opt(&mut self.ifds, other.ifds);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_tiff_ifds(base: &[TiffIfd], diff: &TiffIfdsDiff) -> MutationApplyResult<()> {
    let mut removed = std::collections::HashSet::new();
    for &index in &diff.removed {
        if index >= base.len() || !removed.insert(index) {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "TIFF IFD removal is missing or duplicated").at(["ifds", "removed"]));
        }
    }
    let mut modified = std::collections::HashSet::new();
    for entry in &diff.modified {
        if entry.index >= base.len() || !modified.insert(entry.index) || removed.contains(&entry.index) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "TIFF IFD modification is missing, duplicated, or removed").at(["ifds", "modified"]));
        }
        validate_tiff_tags(&base[entry.index].entries, &entry.diff.entries)?;
        validate_tiff_runs(entry.diff.blocks.as_deref().unwrap_or(&base[entry.index].blocks), &entry.diff.runs)?;
    }
    let final_len = base.len().saturating_sub(diff.removed.len()).saturating_add(diff.added.len());
    let mut added = std::collections::HashSet::new();
    for entry in &diff.added {
        if entry.index > final_len || !added.insert(entry.index) {
            return Err(MutationApplyError::new("mutation.apply.invalid-index", "TIFF IFD addition index is invalid or duplicated").at(["ifds", "added"]));
        }
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_tiff_runs(blocks: &[TiffSampleBlock], runs: &[TiffSampleRun]) -> MutationApplyResult<()> {
    match runs.iter().all(|run| blocks.get(run.block).is_some_and(|block| run.offset.checked_add(run.samples.len()).is_some_and(|end| end <= block.samples.len()))) {
        true => Ok(()),
        false => Err(MutationApplyError::new("mutation.apply.invalid-run", "TIFF sample run leaves its block").at(["ifds", "modified", "runs"])),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_tiff_tags(base: &[TiffTag], diff: &TiffTagsDiff) -> MutationApplyResult<()> {
    let base_tags: std::collections::HashSet<u16> = base.iter().map(|tag| tag.tag).collect();
    let removed: std::collections::HashSet<u16> = diff.removed.iter().copied().collect();
    if removed.len() != diff.removed.len() || diff.removed.iter().any(|tag| !base_tags.contains(tag)) {
        return Err(MutationApplyError::new("mutation.apply.missing-target", "TIFF tag removal is missing or duplicated").at(["entries", "removed"]));
    }
    let mut modified = std::collections::HashSet::new();
    for entry in &diff.modified {
        if !base_tags.contains(&entry.tag) || !modified.insert(entry.tag) || removed.contains(&entry.tag) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "TIFF tag modification is missing, duplicated, or removed").at(["entries", "modified"]));
        }
    }
    let mut added = std::collections::HashSet::new();
    for entry in &diff.added {
        if base_tags.contains(&entry.tag) || !added.insert(entry.tag) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "TIFF tag addition conflicts with the target state or has an invalid value kind").at(["entries", "added"]));
        }
    }
    Ok(())
}

impl DiffAlgebra<TiffSnapshot> for TiffDiff {
    /// 🔁️ Concrete diff-level undo: removed directories come back at their base index, added directories go away again, and every
    /// modified directory restores the tags, block list and sample words `base` carries.
    fn inverse(&self, base: &TiffSnapshot) -> Self {
        Self { ifds: self.ifds.as_ref().map(|diff| inverse_ifds(diff, &base.ifds)).filter(|diff| !diff.removed.is_empty() || !diff.modified.is_empty() || !diff.added.is_empty()) }
    }

    /// 🧭️ State delta (compose `GetXDiff`): index-keyed pairwise `0..min(len)` matching for
    /// `ifds`, recursive tag-id-keyed matching within each surviving IFD pair.
    fn between(base: &TiffSnapshot, other: &TiffSnapshot) -> Self {
        Self { ifds: between_ifds(&base.ifds, &other.ifds) }
    }

    fn is_empty(&self) -> bool {
        self.ifds.is_none()
    }
}

//#endregion 🔖️Diff

//#region 🔖️DemoCases
/// 🧪️ P2-FG2: representative `TiffDiff` values (byte order, IFD tags, and storage exercised; IFD-level
/// index-keyed removed/modified/added AND nested tag-id-keyed removed/modified/added, every
/// `TiffValues` field-type family) — the single source of truth reused by
/// `diff_grammar_conformance_law`/`protocol_walk_law` below (`⚙️engine/🦀️.rs`).
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases()->Vec<TiffDiff>{let a=crate::schema::demo_tiff_snapshot();let mut b=a.clone();b.ifds[0].blocks[0].samples[0].lo=9;b.ifds[0].entries.push(TiffTag{tag:50000,values:TiffValues::Double(vec![TiffWord64{lo:17,hi:0x7ff80000}])});let c=TiffSnapshot::default();vec![TiffDiff::default(),TiffDiff::between(&a,&b),TiffDiff::between(&b,&a),TiffDiff::between(&a,&c),TiffDiff::between(&c,&a)]}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️handcrafted-diff-codec/🦀️.rs"]
mod handcrafted_diff_codec_tests;
//#endregion 🧪️Tests
