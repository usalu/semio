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
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use schema::ArtifactSchema;
use std::collections::{BTreeMap, BTreeSet, HashMap};

//#region 🔖️TagsTriple
/// 🏷️ One `entries.modified[]`/`.added[]` entity — `TiffTag` is a weak value, so both carry
/// the entry's NEW `kind`/`values` directly (never a nested per-field diff).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct TiffTagModified {
    pub tag: u16,
    pub kind: TiffFieldType,
    pub values: TiffValues,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct TiffTagAdded {
    pub tag: u16,
    pub kind: TiffFieldType,
    pub values: TiffValues,
}

/// 🔺️ Tag-id-keyed `entries` triple for one IFD.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
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
/// 🗂️ The per-IFD delta: the recursive tag-triple plus a whole-value slot for that directory's own
/// raw strip payload (`TiffIfd::pixels` — a weak value, replaced wholesale, never sub-diffed, the
/// same treatment `TiffDiff::pixels` gives the primary raster).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct TiffIfdDiff {
    #[value(default, skip_serializing_if = "TiffTagsDiff::is_empty")]
    pub entries: TiffTagsDiff,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub pixels: Option<Vec<u8>>,
}

impl TiffIfdDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty() && self.pixels.is_none()
    }
}

/// 🗂️ One `ifds.modified[]` entity — the recursive per-IFD delta.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct TiffIfdModified {
    pub index: usize,
    pub diff: TiffIfdDiff,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct TiffIfdAdded {
    pub index: usize,
    pub ifd: TiffIfd,
}

/// 🔺️ Index-keyed `ifds` triple (TIFF's IFD chain is positional).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
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
    removed.iter().copied().chain(modified_indices).chain(added_indices).max().map(|m| m + 1).unwrap_or(0)
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
    let needed_mid_len = d2.removed.iter().copied().chain(d2.modified.iter().map(|m| m.index)).max().map(|m| m + 1).unwrap_or(0);
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
                if m2.diff.pixels.is_some() {
                    entry.pixels = m2.diff.pixels.clone();
                }
            }
            Some(Slot::Added(ai)) => {
                if let Some(a) = added_alive[*ai].as_mut() {
                    a.ifd.entries = apply_tags(&a.ifd.entries, &m2.diff.entries);
                    if let Some(pixels) = &m2.diff.pixels {
                        a.ifd.pixels = pixels.clone();
                    }
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
    let mid_len = d2.removed.iter().copied().chain(d2.modified.iter().map(|m| m.index)).chain(alive_mid_positions.iter().copied()).chain(d2_added_indices.iter().copied()).max().map(|m| m + 1).unwrap_or(0);
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
            if let Some(pixels) = &m.diff.pixels {
                it.pixels = pixels.clone();
            }
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_ifds(a: &[TiffIfd], b: &[TiffIfd]) -> Option<TiffIfdsDiff> {
    let min = a.len().min(b.len());
    let mut modified = Vec::new();
    for i in 0..min {
        let diff = TiffIfdDiff { entries: between_tags(&a[i].entries, &b[i].entries).unwrap_or_default(), pixels: (a[i].pixels != b[i].pixels).then(|| b[i].pixels.clone()) };
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
/// 🧪️ F6 CONFIRMED (real `cargo check`, ticket
/// 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION): adding
/// `#[derive(dsl::DslDiff)]` here fails — `TiffValues` (12 non-unit variants: `Byte(Vec<u8>)`,
/// `Ascii(String)`, `Short(Vec<u16>)`, … `Double(Vec<f64>)`) is a genuine data-carrying enum
/// reachable through `ifds: Option<TiffIfdsDiff>` -> `TiffIfdModified.diff.modified[].values` /
/// `.added[].values`, and `DslField` has no impl for it (only `DslRecord`-derived structs and
/// `DslScalar`-derived UNIT-only enums implement `DslField` — recon report §3a): `error[E0277]:
/// the trait bound v6_0::…::TiffValues: DslField is not satisfied`. Same root cause independently
/// requires a direct typed codec for `ReplaceTagMutation.values`, which reaches the same
/// `TiffValues`. `DiffCodec` is hand-rolled below (see `HandcraftedDiffCodec`).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.tiff.diff")]
pub struct TiffDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub byte_order: Option<TiffByteOrder>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub ifds: Option<TiffIfdsDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub pixels: Option<Vec<u8>>,
}

impl MutationDiff<TiffSnapshot> for TiffDiff {
    fn apply(&self, base: &TiffSnapshot) -> MutationApplyResult<TiffSnapshot> {
        if let Some(ifds) = &self.ifds {
            validate_tiff_ifds(&base.ifds, ifds)?;
        }
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
        Ok(next)
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
fn validate_tiff_tags(base: &[TiffTag], diff: &TiffTagsDiff) -> MutationApplyResult<()> {
    let base_tags: std::collections::HashSet<u16> = base.iter().map(|tag| tag.tag).collect();
    let removed: std::collections::HashSet<u16> = diff.removed.iter().copied().collect();
    if removed.len() != diff.removed.len() || diff.removed.iter().any(|tag| !base_tags.contains(tag)) {
        return Err(MutationApplyError::new("mutation.apply.missing-target", "TIFF tag removal is missing or duplicated").at(["entries", "removed"]));
    }
    let mut modified = std::collections::HashSet::new();
    for entry in &diff.modified {
        if !base_tags.contains(&entry.tag) || !modified.insert(entry.tag) || removed.contains(&entry.tag) || entry.kind != entry.values.kind() {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "TIFF tag modification is missing, duplicated, or removed").at(["entries", "modified"]));
        }
    }
    let mut added = std::collections::HashSet::new();
    for entry in &diff.added {
        if base_tags.contains(&entry.tag) || !added.insert(entry.tag) || entry.kind != entry.values.kind() {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "TIFF tag addition conflicts with the target state or has an invalid value kind").at(["entries", "added"]));
        }
    }
    Ok(())
}

impl DiffAlgebra<TiffSnapshot> for TiffDiff {
    /// 🔁️ Diff-level undo, derived generically (correct by construction): the state delta
    /// from `self.apply(base)` back to `base`.
    fn inverse(&self, base: &TiffSnapshot) -> Self {
        let mutated = self.apply(base).unwrap();
        Self::between(&mutated, base)
    }

    /// 🧭️ State delta (compose `GetXDiff`): index-keyed pairwise `0..min(len)` matching for
    /// `ifds`, recursive tag-id-keyed matching within each surviving IFD pair.
    fn between(base: &TiffSnapshot, other: &TiffSnapshot) -> Self {
        Self { byte_order: (base.byte_order != other.byte_order).then_some(other.byte_order), ifds: between_ifds(&base.ifds, &other.ifds), pixels: (base.pixels != other.pixels).then(|| other.pixels.clone()) }
    }

    fn is_empty(&self) -> bool {
        self.byte_order.is_none() && self.ifds.is_none() && self.pixels.is_none()
    }
}

/// 🧩 Builds a set-snapshot diff (sparse field-by-field delta, never a full-replace slot).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_snapshot(base: &TiffSnapshot, next: &TiffSnapshot) -> TiffDiff {
    TiffDiff::between(base, next)
}
//#endregion 🔖️Diff

//#region 🔖️MutationDiffBuilders
// 🧩 One handcrafted builder per `schema::mutations::TiffMutation` variant (excluding
// `NoMutation`/`SetSnapshot`, covered above).

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9


// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9


// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9


// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9


// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9


// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9

//#endregion 🔖️MutationDiffBuilders

//#region 🔖️HandcraftedDiffCodec
/// 🧪️ F6: **hand-rolled** `protocol::DiffCodec` for `TiffDiff` — `TiffValues` (a genuine
/// data-carrying enum, real compile error captured on the `TiffDiff` doc comment above) rules out
/// `#[derive(dsl::DslDiff)]`. Same grammar style `GifDiff`/`SvgDiff`'s hand-rolled codecs use
/// (bracket-depth-aware split, hex for strings/bytes, single-letter tag prefix for enums,
/// `[removed];[modified];[added]` for collection triples) — see `f6-recon-report.md` §5 for the
/// primitive rationale; this file re-derives its own copies of the small helper functions (no
/// shared "hand-roll helpers" module exists yet). No `Option<T>`/tri-state wrapping is needed
/// here — every `TiffDiff`/`TiffMutation` field is a required value, so `encode_option`/
/// `decode_option` (present in `GifDiff`/`SvgDiff`) are deliberately omitted as dead code.
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
/// 🔢️ Generic numeric-token parser (`u8`/`u16`/`u32`/`i8`/`i16`/`i32`/`f32`/`f64`/`usize`, every
/// scalar this grammar carries) — `f32`/`f64`'s `Display`/`FromStr` round-trip exactly for every
/// finite value this codec ever produces (same assumption `svg`'s `ViewBox` float fields make).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_num<T: std::str::FromStr>(s: &str) -> Result<T, String>
where
    T::Err: std::fmt::Display,
{
    s.parse::<T>().map_err(|e| e.to_string())
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
/// 📃️ Generic bracketed comma list (`[e1,e2,...]`) — every `Vec<T>` in this grammar (IFD entries,
/// an IFD list, a numeric value list) uses this same shape.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_list<T>(items: &[T], enc: impl Fn(&T) -> String) -> String {
    format!("[{}]", items.iter().map(|x| enc(x)).collect::<Vec<_>>().join(","))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_list<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Vec<T>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec).collect()
}

//#region 🔖️BinaryPrimitives
/// 🧪️ P2-FG2: real LEB128-varint-framed binary primitives (length-prefixed bytes/utf8) backing
/// the upgraded `DiffCodec`/`OpBinary` frames below (and, via re-export, `../🧬️mutations/
/// 🦀️component.rs`'s own upgraded `OpBinary`) — reuses `store::pack_rt::write_varint_u64`/
/// `store::ByteReader` rather than reinventing varint encode/decode, same shape `xml`'s own
/// `write_str_lp`/`read_str_lp` uses.
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}
//#endregion 🔖️BinaryPrimitives
//#endregion 🔖️Primitives

//#region 🔖️ValueCodecs
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_byte_order(b: TiffByteOrder) -> String {
    match b {
        TiffByteOrder::LittleEndian => "0".to_string(),
        TiffByteOrder::BigEndian => "1".to_string(),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_byte_order(s: &str) -> Result<TiffByteOrder, String> {
    match s {
        "0" => Ok(TiffByteOrder::LittleEndian),
        "1" => Ok(TiffByteOrder::BigEndian),
        other => Err(format!("byte order: unknown code {other:?}")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_field_type(k: TiffFieldType) -> String {
    k.to_u16().to_string()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_field_type(s: &str) -> Result<TiffFieldType, String> {
    TiffFieldType::from_u16(parse_num::<u16>(s)?)
}
/// 📦️ `TiffValues` — single-uppercase-letter tag prefix immediately followed by the bracketed
/// positional payload (same convention `svg`'s `enc_xml_node`/gif's enum codecs use): `B`=Byte,
/// `A`=Ascii, `S`=Short, `L`=Long, `R`=Rational, `E`=SByte, `U`=Undefined, `H`=SShort, `G`=SLong,
/// `Q`=SRational, `F`=Float, `D`=Double. `Byte`/`Undefined` (raw octets) and `Ascii` (text) are hex;
/// every numeric list is decimal comma-separated; `Rational`/`SRational` pairs nest as `[n,d]`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_values(v: &TiffValues) -> String {
    match v {
        TiffValues::Byte(b) => format!("B[{}]", hex_encode(b)),
        TiffValues::Ascii(s) => format!("A[{}]", enc_str(s)),
        TiffValues::Short(v) => format!("S{}", enc_list(v, |x| x.to_string())),
        TiffValues::Long(v) => format!("L{}", enc_list(v, |x| x.to_string())),
        TiffValues::Rational(v) => format!("R{}", enc_list(v, |(n, d)| format!("[{n},{d}]"))),
        TiffValues::SByte(v) => format!("E{}", enc_list(v, |x| x.to_string())),
        TiffValues::Undefined(b) => format!("U[{}]", hex_encode(b)),
        TiffValues::SShort(v) => format!("H{}", enc_list(v, |x| x.to_string())),
        TiffValues::SLong(v) => format!("G{}", enc_list(v, |x| x.to_string())),
        TiffValues::SRational(v) => format!("Q{}", enc_list(v, |(n, d)| format!("[{n},{d}]"))),
        TiffValues::Float(v) => format!("F{}", enc_list(v, |x| x.to_string())),
        TiffValues::Double(v) => format!("D{}", enc_list(v, |x| x.to_string())),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_values(s: &str) -> Result<TiffValues, String> {
    let (tag, rest) = s.split_at(1);
    let pair = |s: &str| -> Result<(u32, u32), String> {
        let parts = split_top_level(strip_brackets(s)?, ',');
        let [n, d] = parts.as_slice() else { return Err(format!("rational: expected 2 fields, got {}", parts.len())) };
        Ok((parse_num::<u32>(n)?, parse_num::<u32>(d)?))
    };
    let spair = |s: &str| -> Result<(i32, i32), String> {
        let parts = split_top_level(strip_brackets(s)?, ',');
        let [n, d] = parts.as_slice() else { return Err(format!("srational: expected 2 fields, got {}", parts.len())) };
        Ok((parse_num::<i32>(n)?, parse_num::<i32>(d)?))
    };
    match tag {
        "B" => Ok(TiffValues::Byte(hex_decode(strip_brackets(rest)?)?)),
        "A" => Ok(TiffValues::Ascii(dec_str(strip_brackets(rest)?)?)),
        "S" => Ok(TiffValues::Short(dec_list(rest, parse_num::<u16>)?)),
        "L" => Ok(TiffValues::Long(dec_list(rest, parse_num::<u32>)?)),
        "R" => Ok(TiffValues::Rational(dec_list(rest, pair)?)),
        "E" => Ok(TiffValues::SByte(dec_list(rest, parse_num::<i8>)?)),
        "U" => Ok(TiffValues::Undefined(hex_decode(strip_brackets(rest)?)?)),
        "H" => Ok(TiffValues::SShort(dec_list(rest, parse_num::<i16>)?)),
        "G" => Ok(TiffValues::SLong(dec_list(rest, parse_num::<i32>)?)),
        "Q" => Ok(TiffValues::SRational(dec_list(rest, spair)?)),
        "F" => Ok(TiffValues::Float(dec_list(rest, parse_num::<f32>)?)),
        "D" => Ok(TiffValues::Double(dec_list(rest, parse_num::<f64>)?)),
        other => Err(format!("tiff values: unknown tag {other:?}")),
    }
}
/// 🏷️ One IFD entry: `[tag,kind,values]` positional triple.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_tag(t: &TiffTag) -> String {
    format!("[{},{},{}]", t.tag, enc_field_type(t.kind), enc_values(&t.values))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_tag(s: &str) -> Result<TiffTag, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [tag, kind, values] = parts.as_slice() else { return Err(format!("tag: expected 3 fields, got {}", parts.len())) };
    Ok(TiffTag { tag: parse_num::<u16>(tag)?, kind: dec_field_type(kind)?, values: dec_values(values)? })
}
/// 🗂️ One IFD: `[<entries-list>,<pixels-hex>]` — the bracketed list of `enc_tag` entries followed
/// by this directory's own raw strip bytes as hex (empty for a metadata-only directory and for
/// IFD 0, whose raster is the snapshot's own `pixels`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_ifd(ifd: &TiffIfd) -> String {
    format!("[{},{}]", enc_list(&ifd.entries, enc_tag), hex_encode(&ifd.pixels))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_ifd(s: &str) -> Result<TiffIfd, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [entries, pixels] = parts.as_slice() else { return Err(format!("ifd: expected 2 fields, got {}", parts.len())) };
    Ok(TiffIfd { entries: dec_list(entries, dec_tag)?, pixels: hex_decode(pixels)? })
}
//#endregion 🔖️ValueCodecs

//#region 🔖️ValueBinaryCodecs
/// 🧪️ P2-FG2: real recursive binary twins of [`enc_tag`]/[`dec_tag`]/[`enc_ifd`]/[`dec_ifd`]/
/// [`enc_values`]/[`dec_values`] above — a 1-byte kind tag (`0`=Byte/`1`=Ascii/`2`=Short/`3`=Long/
/// `4`=Rational/`5`=SByte/`6`=Undefined/`7`=SShort/`8`=SLong/`9`=SRational/`10`=Float/`11`=Double,
/// distinct numbering from the text codec's letter tags) followed by the real typed payload
/// (varint-length-prefixed bytes for `Byte`/`Ascii`/`Undefined`, a varint COUNT then that many
/// fixed-width LE elements for every numeric list, `Rational`/`SRational` pairs as two consecutive
/// fixed-width elements) — genuinely typed binary, NOT text-as-bytes. Backs the upgraded
/// `DiffCodec`/`OpBinary` frames below (`../🧬️mutations/🦀️component.rs` reuses these via its own
/// `pub(crate)` re-export, same intra-artifact reuse convention `xml`'s `enc_xml_node_bin` uses).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_values_bin(v: &TiffValues, out: &mut Vec<u8>) {
    match v {
        TiffValues::Byte(b) => {
            out.push(0);
            write_bytes_lp(out, b);
        }
        TiffValues::Ascii(s) => {
            out.push(1);
            write_str_lp(out, s);
        }
        TiffValues::Short(v) => {
            out.push(2);
            store::pack_rt::write_varint_u64(out, v.len() as u64);
            v.iter().for_each(|&x| out.extend_from_slice(&x.to_le_bytes()));
        }
        TiffValues::Long(v) => {
            out.push(3);
            store::pack_rt::write_varint_u64(out, v.len() as u64);
            v.iter().for_each(|&x| out.extend_from_slice(&x.to_le_bytes()));
        }
        TiffValues::Rational(v) => {
            out.push(4);
            store::pack_rt::write_varint_u64(out, v.len() as u64);
            v.iter().for_each(|&(n, d)| {
                out.extend_from_slice(&n.to_le_bytes());
                out.extend_from_slice(&d.to_le_bytes());
            });
        }
        TiffValues::SByte(v) => {
            out.push(5);
            store::pack_rt::write_varint_u64(out, v.len() as u64);
            v.iter().for_each(|&x| out.push(x as u8));
        }
        TiffValues::Undefined(b) => {
            out.push(6);
            write_bytes_lp(out, b);
        }
        TiffValues::SShort(v) => {
            out.push(7);
            store::pack_rt::write_varint_u64(out, v.len() as u64);
            v.iter().for_each(|&x| out.extend_from_slice(&x.to_le_bytes()));
        }
        TiffValues::SLong(v) => {
            out.push(8);
            store::pack_rt::write_varint_u64(out, v.len() as u64);
            v.iter().for_each(|&x| out.extend_from_slice(&x.to_le_bytes()));
        }
        TiffValues::SRational(v) => {
            out.push(9);
            store::pack_rt::write_varint_u64(out, v.len() as u64);
            v.iter().for_each(|&(n, d)| {
                out.extend_from_slice(&n.to_le_bytes());
                out.extend_from_slice(&d.to_le_bytes());
            });
        }
        TiffValues::Float(v) => {
            out.push(10);
            store::pack_rt::write_varint_u64(out, v.len() as u64);
            v.iter().for_each(|&x| out.extend_from_slice(&x.to_le_bytes()));
        }
        TiffValues::Double(v) => {
            out.push(11);
            store::pack_rt::write_varint_u64(out, v.len() as u64);
            v.iter().for_each(|&x| out.extend_from_slice(&x.to_le_bytes()));
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_values_bin(reader: &mut store::ByteReader<'_>) -> Result<TiffValues, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    let count = |reader: &mut store::ByteReader<'_>| -> Result<u64, String> { reader.read_varint_u64().map_err(|e| e.to_string()) };
    match tag {
        0 => Ok(TiffValues::Byte(read_bytes_lp(reader)?)),
        1 => Ok(TiffValues::Ascii(read_str_lp(reader)?)),
        2 => {
            let n = count(reader)?;
            (0..n).map(|_| reader.read_u16_le().map_err(|e| e.to_string())).collect::<Result<Vec<_>, _>>().map(TiffValues::Short)
        }
        3 => {
            let n = count(reader)?;
            (0..n).map(|_| reader.read_u32_le().map_err(|e| e.to_string())).collect::<Result<Vec<_>, _>>().map(TiffValues::Long)
        }
        4 => {
            let n = count(reader)?;
            (0..n)
                .map(|_| {
                    let a = reader.read_u32_le().map_err(|e| e.to_string())?;
                    let b = reader.read_u32_le().map_err(|e| e.to_string())?;
                    Ok((a, b))
                })
                .collect::<Result<Vec<_>, String>>()
                .map(TiffValues::Rational)
        }
        5 => {
            let n = count(reader)?;
            (0..n).map(|_| reader.read_u8().map_err(|e| e.to_string()).map(|b| b as i8)).collect::<Result<Vec<_>, _>>().map(TiffValues::SByte)
        }
        6 => Ok(TiffValues::Undefined(read_bytes_lp(reader)?)),
        7 => {
            let n = count(reader)?;
            (0..n).map(|_| reader.read_u16_le().map_err(|e| e.to_string()).map(|x| x as i16)).collect::<Result<Vec<_>, _>>().map(TiffValues::SShort)
        }
        8 => {
            let n = count(reader)?;
            (0..n).map(|_| reader.read_u32_le().map_err(|e| e.to_string()).map(|x| x as i32)).collect::<Result<Vec<_>, _>>().map(TiffValues::SLong)
        }
        9 => {
            let n = count(reader)?;
            (0..n)
                .map(|_| {
                    let a = reader.read_u32_le().map_err(|e| e.to_string())? as i32;
                    let b = reader.read_u32_le().map_err(|e| e.to_string())? as i32;
                    Ok((a, b))
                })
                .collect::<Result<Vec<_>, String>>()
                .map(TiffValues::SRational)
        }
        10 => {
            let n = count(reader)?;
            (0..n).map(|_| reader.read_bytes(4).map_err(|e| e.to_string()).map(|b| f32::from_le_bytes(b.try_into().expect("4 bytes")))).collect::<Result<Vec<_>, _>>().map(TiffValues::Float)
        }
        11 => {
            let n = count(reader)?;
            (0..n).map(|_| reader.read_f64_le().map_err(|e| e.to_string())).collect::<Result<Vec<_>, _>>().map(TiffValues::Double)
        }
        other => Err(format!("tiff values binary: unknown tag {other}")),
    }
}
/// 🏷️ Binary twin of [`enc_tag`]/[`dec_tag`] — `tag:u16le, kind:u8 (TIFF field-type code 1-12,
/// always fits one byte), values:enc_values_bin`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_tag_bin(t: &TiffTag, out: &mut Vec<u8>) {
    out.extend_from_slice(&t.tag.to_le_bytes());
    out.push(t.kind.to_u16() as u8);
    enc_values_bin(&t.values, out);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_tag_bin(reader: &mut store::ByteReader<'_>) -> Result<TiffTag, String> {
    let tag = reader.read_u16_le().map_err(|e| e.to_string())?;
    let kind = TiffFieldType::from_u16(reader.read_u8().map_err(|e| e.to_string())? as u16)?;
    let values = dec_values_bin(reader)?;
    Ok(TiffTag { tag, kind, values })
}
/// 🗂️ Binary twin of [`enc_ifd`]/[`dec_ifd`] — varint entry count, then that many [`enc_tag_bin`]
/// entries.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_ifd_bin(ifd: &TiffIfd, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, ifd.entries.len() as u64);
    ifd.entries.iter().for_each(|t| enc_tag_bin(t, out));
    write_bytes_lp(out, &ifd.pixels);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_ifd_bin(reader: &mut store::ByteReader<'_>) -> Result<TiffIfd, String> {
    let n = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut entries = Vec::with_capacity(n as usize);
    for _ in 0..n {
        entries.push(dec_tag_bin(reader)?);
    }
    let pixels = read_bytes_lp(reader)?;
    Ok(TiffIfd { entries, pixels })
}
//#endregion 🔖️ValueBinaryCodecs

//#region 🔖️DiffValueCodecs
/// 🔺️ Tag-id-keyed `entries` triple: `[removed];[modified];[added]`, `modified`/`added` entries
/// are `tag:kind:values` (colon-separated — safe since `kind` is bare decimal and `values` never
/// contains a literal `:`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_tags_diff(d: &TiffTagsDiff) -> String {
    let removed = d.removed.iter().map(|t| t.to_string()).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:{}:{}", m.tag, enc_field_type(m.kind), enc_values(&m.values))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}:{}", a.tag, enc_field_type(a.kind), enc_values(&a.values))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_tags_diff(body: &str) -> Result<TiffTagsDiff, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("tags diff: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_num::<u16>).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (tag_s, rest) = entry.split_once(':').ok_or_else(|| format!("tag modified: bad entry {entry:?}"))?;
            let (kind_s, values_s) = rest.split_once(':').ok_or_else(|| format!("tag modified: bad entry {entry:?}"))?;
            Ok(TiffTagModified { tag: parse_num::<u16>(tag_s)?, kind: dec_field_type(kind_s)?, values: dec_values(values_s)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (tag_s, rest) = entry.split_once(':').ok_or_else(|| format!("tag added: bad entry {entry:?}"))?;
            let (kind_s, values_s) = rest.split_once(':').ok_or_else(|| format!("tag added: bad entry {entry:?}"))?;
            Ok(TiffTagAdded { tag: parse_num::<u16>(tag_s)?, kind: dec_field_type(kind_s)?, values: dec_values(values_s)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(TiffTagsDiff { removed, modified, added })
}

/// 🔺️ One IFD's own delta: `[<tags-triple>];<pixels>` — the bracketed (so `split_top_level(_, ';')`
/// sees it as ONE section) tag triple, then `-` for "strip bytes unchanged" or the new bytes as hex.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_ifd_diff(d: &TiffIfdDiff) -> String {
    let pixels = match &d.pixels {
        Some(bytes) => format!("#{}", hex_encode(bytes)),
        None => "-".to_string(),
    };
    format!("[{}];{pixels}", enc_tags_diff(&d.entries))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_ifd_diff(body: &str) -> Result<TiffIfdDiff, String> {
    let two = split_top_level(body, ';');
    let [entries_s, pixels_s] = two.as_slice() else { return Err(format!("ifd diff: expected 2 sections, got {}", two.len())) };
    let pixels = match *pixels_s {
        "-" => None,
        other => Some(hex_decode(other.strip_prefix('#').ok_or_else(|| format!("ifd diff: bad pixels slot {other:?}"))?)?),
    };
    Ok(TiffIfdDiff { entries: dec_tags_diff(strip_brackets(entries_s)?)?, pixels })
}

/// 🗂️ Index-keyed `ifds` triple: `[removed];[modified];[added]`, `modified` entries are
/// `index:<ifd-diff>` (recursive), `added` entries are `index:<ifd>`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_ifds_diff(d: &TiffIfdsDiff) -> String {
    let removed = d.removed.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:{}", m.index, enc_ifd_diff(&m.diff))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}", a.index, enc_ifd(&a.ifd))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_ifds_diff(body: &str) -> Result<TiffIfdsDiff, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("ifds diff: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_num::<usize>).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("ifd modified: bad entry {entry:?}"))?;
            Ok(TiffIfdModified { index: parse_num::<usize>(idx)?, diff: dec_ifd_diff(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("ifd added: bad entry {entry:?}"))?;
            Ok(TiffIfdAdded { index: parse_num::<usize>(idx)?, ifd: dec_ifd(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(TiffIfdsDiff { removed, modified, added })
}
//#endregion 🔖️DiffValueCodecs

//#region 🔖️DiffValueBinaryCodecs
/// 🧪️ P2-FG2: real recursive binary twins of [`enc_tags_diff`]/[`dec_tags_diff`]/
/// [`enc_ifds_diff`]/[`dec_ifds_diff`] — every `removed`/`modified`/`added` triple becomes a
/// varint-counted, recursively-encoded list (same shape XML's `enc_children_diff_bin`/
/// `enc_attrs_diff_bin` use), backing the upgraded `DiffCodec::encode_diff`/`decode_diff` below.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_tags_diff_bin(d: &TiffTagsDiff, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, d.removed.len() as u64);
    d.removed.iter().for_each(|&t| out.extend_from_slice(&t.to_le_bytes()));
    store::pack_rt::write_varint_u64(out, d.modified.len() as u64);
    for m in &d.modified {
        out.extend_from_slice(&m.tag.to_le_bytes());
        out.push(m.kind.to_u16() as u8);
        enc_values_bin(&m.values, out);
    }
    store::pack_rt::write_varint_u64(out, d.added.len() as u64);
    for a in &d.added {
        out.extend_from_slice(&a.tag.to_le_bytes());
        out.push(a.kind.to_u16() as u8);
        enc_values_bin(&a.values, out);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_tags_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<TiffTagsDiff, String> {
    let rn = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut removed = Vec::with_capacity(rn as usize);
    for _ in 0..rn {
        removed.push(reader.read_u16_le().map_err(|e| e.to_string())?);
    }
    let mn = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(mn as usize);
    for _ in 0..mn {
        let tag = reader.read_u16_le().map_err(|e| e.to_string())?;
        let kind = TiffFieldType::from_u16(reader.read_u8().map_err(|e| e.to_string())? as u16)?;
        let values = dec_values_bin(reader)?;
        modified.push(TiffTagModified { tag, kind, values });
    }
    let an = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(an as usize);
    for _ in 0..an {
        let tag = reader.read_u16_le().map_err(|e| e.to_string())?;
        let kind = TiffFieldType::from_u16(reader.read_u8().map_err(|e| e.to_string())? as u16)?;
        let values = dec_values_bin(reader)?;
        added.push(TiffTagAdded { tag, kind, values });
    }
    Ok(TiffTagsDiff { removed, modified, added })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_ifds_diff_bin(d: &TiffIfdsDiff, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, d.removed.len() as u64);
    d.removed.iter().for_each(|&i| store::pack_rt::write_varint_u64(out, i as u64));
    store::pack_rt::write_varint_u64(out, d.modified.len() as u64);
    for m in &d.modified {
        store::pack_rt::write_varint_u64(out, m.index as u64);
        enc_tags_diff_bin(&m.diff.entries, out);
        match &m.diff.pixels {
            Some(bytes) => {
                out.push(1);
                write_bytes_lp(out, bytes);
            }
            None => out.push(0),
        }
    }
    store::pack_rt::write_varint_u64(out, d.added.len() as u64);
    for a in &d.added {
        store::pack_rt::write_varint_u64(out, a.index as u64);
        enc_ifd_bin(&a.ifd, out);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_ifds_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<TiffIfdsDiff, String> {
    let rn = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut removed = Vec::with_capacity(rn as usize);
    for _ in 0..rn {
        removed.push(reader.read_varint_u64().map_err(|e| e.to_string())? as usize);
    }
    let mn = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(mn as usize);
    for _ in 0..mn {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let entries = dec_tags_diff_bin(reader)?;
        let pixels = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(read_bytes_lp(reader)?) } else { None };
        modified.push(TiffIfdModified { index, diff: TiffIfdDiff { entries, pixels } });
    }
    let an = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(an as usize);
    for _ in 0..an {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let ifd = dec_ifd_bin(reader)?;
        added.push(TiffIfdAdded { index, ifd });
    }
    Ok(TiffIfdsDiff { removed, modified, added })
}
//#endregion 🔖️DiffValueBinaryCodecs

//#region 🔖️TopLevel
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_tiff_diff(d: &TiffDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = d.byte_order {
        tokens.push(format!("byte-order={}", enc_byte_order(v)));
    }
    if let Some(v) = &d.ifds {
        tokens.push(format!("ifds={}", enc_ifds_diff(v)));
    }
    if let Some(v) = &d.pixels {
        tokens.push(format!("pixels={}", hex_encode(v)));
    }
    tokens.join(" ")
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_tiff_diff(line: &str) -> Result<TiffDiff, String> {
    let mut d = TiffDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("byte-order=") {
            d.byte_order = Some(dec_byte_order(rest)?);
        } else if let Some(rest) = token.strip_prefix("ifds=") {
            d.ifds = Some(dec_ifds_diff(rest)?);
        } else if let Some(rest) = token.strip_prefix("pixels=") {
            d.pixels = Some(hex_decode(rest)?);
        } else {
            return Err(format!("tiff diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl protocol::DiffCodec for TiffDiff {
    fn print_diff(&self) -> String {
        print_tiff_diff(self)
    }
    fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        parse_tiff_diff(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    /// 🧪️ P2-FG2: REAL binary frame (`format u8 | flags u8 | [byte_order][ifds][pixels]`),
    /// matching `../💾️binary/📡️component.protocol.semio`'s `header fixed 2` + `chain payload
    /// bytes` shape — upgraded from F6's `print_diff().into_bytes()` text-as-binary shortcut (100%
    /// of stdio's `DiffCodec` impls were still on that shortcut per the P2-W0 census). `flags` bits
    /// 0/1/2 mark `byte_order`/`ifds`/`pixels` presence; each present field's own real typed
    /// payload follows in that fixed order (`ifds` recurses through [`enc_ifds_diff_bin`] into the
    /// tag-id-keyed triples and the 12-variant `TiffValues` union, genuinely structured all the
    /// way down, never text-as-bytes).
    fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let mut flags: u8 = 0;
        if self.byte_order.is_some() {
            flags |= 0b001;
        }
        if self.ifds.is_some() {
            flags |= 0b010;
        }
        if self.pixels.is_some() {
            flags |= 0b100;
        }
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, flags];
        if let Some(v) = self.byte_order {
            out.push(match v {
                TiffByteOrder::LittleEndian => 0,
                TiffByteOrder::BigEndian => 1,
            });
        }
        if let Some(d) = &self.ifds {
            enc_ifds_diff_bin(d, &mut out);
        }
        if let Some(p) = &self.pixels {
            write_bytes_lp(&mut out, p);
        }
        Ok(out)
    }
    fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let _format = reader.read_u8().map_err(|e| malformed("diff format", 0, e.to_string()))?;
        let flags = reader.read_u8().map_err(|e| malformed("diff flags", 1, e.to_string()))?;
        let byte_order = if flags & 0b001 != 0 {
            let v = reader.read_u8().map_err(|e| malformed("diff byte_order", reader.position(), e.to_string()))?;
            Some(if v == 0 { TiffByteOrder::LittleEndian } else { TiffByteOrder::BigEndian })
        } else {
            None
        };
        let ifds = if flags & 0b010 != 0 { Some(dec_ifds_diff_bin(&mut reader).map_err(|e| malformed("diff ifds", reader.position(), e))?) } else { None };
        let pixels = if flags & 0b100 != 0 { Some(read_bytes_lp(&mut reader).map_err(|e| malformed("diff pixels", reader.position(), e))?) } else { None };
        Ok(TiffDiff { byte_order, ifds, pixels })
    }
}
//#endregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec

//#region 🔖️DemoCases
/// 🧪️ P2-FG2: representative `TiffDiff` values (byte_order/ifds/pixels all exercised, IFD-level
/// index-keyed removed/modified/added AND nested tag-id-keyed removed/modified/added, every
/// `TiffValues` field-type family) — the single source of truth reused by
/// `diff_grammar_conformance_law`/`protocol_walk_law` below (`⚙️engine/🦀️component.rs`).
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<TiffDiff> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn tag(id: u16, kind: TiffFieldType, values: TiffValues) -> TiffTag {
        TiffTag { tag: id, kind, values }
    }
    let a = TiffSnapshot {
        schema: "stdio.tiff".into(),
        byte_order: TiffByteOrder::LittleEndian,
        ifds: vec![TiffIfd { pixels: Vec::new(),
            entries: vec![
                tag(256, TiffFieldType::Long, TiffValues::Long(vec![4])),
                tag(258, TiffFieldType::Short, TiffValues::Short(vec![8, 8, 8])),
                tag(315, TiffFieldType::Ascii, TiffValues::Ascii("An Author".into())),
                tag(282, TiffFieldType::Rational, TiffValues::Rational(vec![(72, 1)])),
            ],
        }],
        pixels: vec![0u8; 16],
    };
    let mut b = a.clone();
    b.byte_order = TiffByteOrder::BigEndian;
    b.ifds[0].entries.retain(|t| t.tag != 258); // remove
    b.ifds[0].entries.iter_mut().find(|t| t.tag == 315).unwrap().values = TiffValues::Ascii("New Author".into()); // modify
    b.ifds[0].entries.push(tag(37380, TiffFieldType::SRational, TiffValues::SRational(vec![(-3, 10)]))); // add
    b.ifds[0].entries.push(tag(50003, TiffFieldType::Float, TiffValues::Float(vec![1.5, -2.25])));
    b.ifds.push(TiffIfd { pixels: Vec::new(), entries: vec![tag(2, TiffFieldType::Long, TiffValues::Long(vec![9]))] }); // whole IFD added
    b.pixels = vec![9u8; 16];
    let c = TiffSnapshot { schema: "stdio.tiff".into(), byte_order: TiffByteOrder::LittleEndian, ifds: vec![], pixels: vec![] };
    vec![TiffDiff::default(), TiffDiff::between(&a, &b), TiffDiff::between(&b, &a), TiffDiff::between(&a, &c), TiffDiff::between(&c, &a)]
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
mod handcrafted_diff_codec_tests {
    use super::*;
    use protocol::DiffCodec;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn tag(id: u16, kind: TiffFieldType, values: TiffValues) -> TiffTag {
        TiffTag { tag: id, kind, values }
    }

    /// 🧪️ `DiffCodec` round-trip laws over the hand-rolled `TiffDiff` grammar — exercises every
    /// `TiffValues` variant (incl. `Rational`/`SRational` pair lists and `Ascii`/`Byte` hex),
    /// both IFD-level (index-keyed) and tag-level (id-keyed) removed/modified/added, and the
    /// scalar `byte_order`/`pixels` tokens.
    #[semio_framework_async_macros::async_test]
    async fn diff_codec_text_binary_roundtrip_law() {
        let a = TiffSnapshot {
            schema: "stdio.tiff".into(),
            byte_order: TiffByteOrder::LittleEndian,
            ifds: vec![
                TiffIfd { pixels: Vec::new(),
                    entries: vec![
                        tag(256, TiffFieldType::Long, TiffValues::Long(vec![4])),
                        tag(258, TiffFieldType::Short, TiffValues::Short(vec![8, 8, 8])),
                        tag(315, TiffFieldType::Ascii, TiffValues::Ascii("An Author".into())),
                        tag(282, TiffFieldType::Rational, TiffValues::Rational(vec![(72, 1)])),
                        tag(700, TiffFieldType::Undefined, TiffValues::Undefined(vec![0xde, 0xad])),
                    ],
                },
                TiffIfd { pixels: Vec::new(), entries: vec![tag(1, TiffFieldType::Byte, TiffValues::Byte(vec![1, 2, 3]))] },
            ],
            pixels: vec![0u8; 16],
        };
        let mut b = a.clone();
        b.byte_order = TiffByteOrder::BigEndian;
        b.ifds[0].entries.retain(|t| t.tag != 258); // remove
        b.ifds[0].entries.iter_mut().find(|t| t.tag == 315).unwrap().values = TiffValues::Ascii("New Author".into()); // modify
        b.ifds[0].entries.push(tag(37380, TiffFieldType::SRational, TiffValues::SRational(vec![(-3, 10), (0, 1)]))); // add
        b.ifds[0].entries.push(tag(50000, TiffFieldType::SByte, TiffValues::SByte(vec![-1, -2])));
        b.ifds[0].entries.push(tag(50001, TiffFieldType::SShort, TiffValues::SShort(vec![-100])));
        b.ifds[0].entries.push(tag(50002, TiffFieldType::SLong, TiffValues::SLong(vec![-100000])));
        b.ifds[0].entries.push(tag(50003, TiffFieldType::Float, TiffValues::Float(vec![1.5, -2.25])));
        b.ifds[0].entries.push(tag(50004, TiffFieldType::Double, TiffValues::Double(vec![3.14159265358979])));
        b.ifds.push(TiffIfd { pixels: Vec::new(), entries: vec![tag(2, TiffFieldType::Long, TiffValues::Long(vec![9]))] }); // whole IFD added
        b.pixels = vec![9u8; 16];
        let c = TiffSnapshot { schema: "stdio.tiff".into(), byte_order: TiffByteOrder::LittleEndian, ifds: vec![], pixels: vec![] };

        let cases = vec![TiffDiff::default(), TiffDiff::between(&a, &b), TiffDiff::between(&b, &a), TiffDiff::between(&a, &c), TiffDiff::between(&c, &a)];
        for d in cases {
            let printed = d.print_diff();
            assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
            let parsed = TiffDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
            assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

            let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
            let decoded = TiffDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
            assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
        }
    }
}
//#endregion 🧪️Tests
