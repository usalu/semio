//! 🔺️ PlyDiff — handcrafted sparse diff. Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL: replaces the
//! old `PlyDiff{snapshot: Option<PlySnapshot>}` full-replace template with a real per-field
//! patch — `format` + an index-keyed `comments` triple + a name-keyed `elements` triple,
//! each modified element carrying its own `properties` (weak, whole-vec replace) and an
//! index-keyed `rows` triple, each modified row carrying a name-keyed sparse per-property patch.
//! Two collection levels nest (elements → rows), matching the recipe's "trees nest" rule.
//!
//! 🧪️ F6 CONFIRMED (ticket `f6-recon-report.md` §9 STEP 1, real `cargo check`, not guessed):
//! `#[derive(dsl::DslDiff)]` on `PlyDiff` fails —
//! `error[E0277]: the trait bound `PlyProperty: DslField` is not satisfied` at
//! `pub properties: Option<Vec<PlyProperty>>` (this file), because `PlyProperty` (`Scalar{..}` /
//! `List{..}`, both data-carrying variants) has no `DslField` impl and none is derivable (it is
//! not unit-variant-only, so `#[derive(dsl::DslScalar)]` does not apply either) — the classic 3a
//! "enum-in-tree" blocker (`PlyValue` is the same shape and would block equally via
//! `PlyRowFieldChange::value`). `DiffBinary,DiffCodec,DiffText` for `PlyDiff` is hand-rolled below instead, following
//! the ticket's §5 grammar template (verbatim primitives from the gif89a/svg pilots).





/// 🧩 Ordered removed keys, modified values, and inserted items.
pub(crate) type IndexedDiffParts<D, T> = (Vec<usize>, Vec<(usize, D)>, Vec<(usize, T)>);

use crate::schema::snapshot::{PlyElement, PlyFormat, PlyProperty, PlyRow, PlyScalarType};
use crate::PlySnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{DiffCodec};
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use std::collections::{BTreeMap, BTreeSet, HashSet};

//#region 🔖️CommentsTriple
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct IndexedDiff<T, D> {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<IndexedModified<D>>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<IndexedAdded<T>>,
}

impl<T, D> IndexedDiff<T, D> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct IndexedModified<D> {
    pub index: usize,
    pub diff: D,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct IndexedAdded<T> {
    pub index: usize,
    pub item: T,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_indexed<T: Clone, D>(base: &[T], diff: &IndexedDiff<T, D>, apply_item: impl Fn(&T, &D) -> T) -> Vec<T> {
    let mut kept: Vec<(usize, T)> = base.iter().enumerate().filter(|(i, _)| !diff.removed.contains(i)).map(|(i, t)| (i, t.clone())).collect();
    for m in &diff.modified {
        if let Some(entry) = kept.iter_mut().find(|(i, _)| *i == m.index) {
            entry.1 = apply_item(&entry.1, &m.diff);
        }
    }
    let mut result: Vec<T> = kept.into_iter().map(|(_, t)| t).collect();
    let mut adds = diff.added.clone();
    adds.sort_by_key(|a| a.index);
    for a in adds {
        let idx = a.index.min(result.len());
        result.insert(idx, a.item);
    }
    result
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_indexed<T, D>(base: &[T], diff: &IndexedDiff<T, D>, validate_item: impl Fn(&T, &D) -> MutationApplyResult<()>) -> MutationApplyResult<()> {
    let mut removed = std::collections::HashSet::new();
    for &index in &diff.removed {
        if index >= base.len() {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "indexed removal target does not exist"));
        }
        if !removed.insert(index) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "indexed removal target is repeated"));
        }
    }
    let mut modified = std::collections::HashSet::new();
    for entry in &diff.modified {
        if entry.index >= base.len() {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "indexed modification target does not exist"));
        }
        if removed.contains(&entry.index) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "indexed modification targets a removed item"));
        }
        if !modified.insert(entry.index) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "indexed modification target is repeated"));
        }
        validate_item(&base[entry.index], &entry.diff).map_err(|error| error.under(vec!["modified".to_string(), entry.index.to_string()]))?;
    }
    let final_len = base.len() - removed.len() + diff.added.len();
    let mut added = std::collections::HashSet::new();
    for entry in &diff.added {
        if entry.index > final_len {
            return Err(MutationApplyError::new("mutation.apply.invalid-index", "indexed addition is outside the final collection"));
        }
        if !added.insert(entry.index) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "indexed addition occupies a repeated final position"));
        }
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn count_le(sorted: &[usize], x: usize) -> usize {
    sorted.partition_point(|&v| v <= x)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn rank_excluding(pos: usize, excluded_sorted: &[usize]) -> usize {
    pos - count_le(excluded_sorted, pos)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn unrank_excluding(rank: usize, excluded_sorted: &[usize]) -> usize {
    let mut candidate = rank;
    loop {
        let next = rank + count_le(excluded_sorted, candidate);
        if next == candidate {
            return candidate;
        }
        candidate = next;
    }
}

/// ➕️ Structural, total, base-free absorb — see mp4's `absorb_indexed` for the full derivation
/// (identical algorithm, adapted from gif 89a's `absorb_indexed_collection`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn absorb_indexed<T: Clone, D: Clone>(d1: &mut IndexedDiff<T, D>, d2: IndexedDiff<T, D>, absorb_item: impl Fn(&mut D, D), apply_item_diff: impl Fn(&mut T, &D)) {
    let removed1_sorted = semio_s_artifact_stdio_contract::ordered(&d1.removed);
    let mut added1_index_sorted: Vec<usize> = d1.added.iter().map(|a| a.index).collect();
    added1_index_sorted.sort_unstable();
    let removed2_sorted = semio_s_artifact_stdio_contract::ordered(&d2.removed);
    let mut added2_index_sorted: Vec<usize> = d2.added.iter().map(|a| a.index).collect();
    added2_index_sorted.sort_unstable();

    let mut merged_added: Vec<IndexedAdded<T>> = std::mem::take(&mut d1.added);
    let mut annihilated: std::collections::HashSet<usize> = Default::default();

    let mut merged_removed_base: Vec<usize> = removed1_sorted.clone();
    for &r2 in &removed2_sorted {
        if added1_index_sorted.binary_search(&r2).is_ok() {
            annihilated.insert(r2);
            merged_added.retain(|a| a.index != r2);
        } else {
            let post_remove_rank = rank_excluding(r2, &added1_index_sorted);
            let base_index = unrank_excluding(post_remove_rank, &removed1_sorted);
            merged_removed_base.push(base_index);
        }
    }
    merged_removed_base.sort_unstable();
    merged_removed_base.dedup();

    let mut modified_map: std::collections::BTreeMap<usize, D> = std::mem::take(&mut d1.modified).into_iter().map(|m| (m.index, m.diff)).collect();
    for base_index in &merged_removed_base {
        modified_map.remove(base_index);
    }
    for m2 in d2.modified {
        if annihilated.contains(&m2.index) {
            continue;
        }
        if added1_index_sorted.binary_search(&m2.index).is_ok() {
            if let Some(entry) = merged_added.iter_mut().find(|a| a.index == m2.index) {
                apply_item_diff(&mut entry.item, &m2.diff);
            }
        } else {
            let post_remove_rank = rank_excluding(m2.index, &added1_index_sorted);
            let base_index = unrank_excluding(post_remove_rank, &removed1_sorted);
            if merged_removed_base.binary_search(&base_index).is_ok() {
                continue;
            }
            match modified_map.get_mut(&base_index) {
                Some(existing) => absorb_item(existing, m2.diff),
                None => {
                    modified_map.insert(base_index, m2.diff);
                }
            }
        }
    }

    let mut merged_added_final: Vec<IndexedAdded<T>> = merged_added
        .into_iter()
        .map(|a| {
            let after_pos = if removed2_sorted.binary_search(&a.index).is_ok() {
                a.index
            } else {
                let post_remove_rank = rank_excluding(a.index, &removed2_sorted);
                unrank_excluding(post_remove_rank, &added2_index_sorted)
            };
            IndexedAdded { index: after_pos, item: a.item }
        })
        .collect();
    merged_added_final.extend(d2.added);
    merged_added_final.sort_by_key(|a| a.index);

    d1.removed = merged_removed_base;
    d1.modified = modified_map.into_iter().map(|(index, diff)| IndexedModified { index, diff }).collect();
    d1.added = merged_added_final;
}

/// ↩️ Negative rows for an indexed collection triple against its BASE items: added rows become removals at their final index,
/// removed rows return at their base index, and each modified row restores its base value at the index the row has after the
/// diff. Every list comes back ascending, the normal form [`absorb_indexed`] emits.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse_indexed<T: Clone, D>(diff: &IndexedDiff<T, D>, base: &[T], inverse_item: impl Fn(&D, &T) -> D) -> IndexedDiff<T, D> {
    let removed_sorted = semio_s_artifact_stdio_contract::ordered_unique(&diff.removed);
    let mut added_final: Vec<usize> = diff.added.iter().map(|added| added.index).collect();
    added_final.sort_unstable();
    let after_index = |index: usize| {
        let survivor = index - removed_sorted.iter().filter(|dropped| **dropped < index).count();
        added_final.iter().fold(survivor, |position, inserted| if *inserted <= position { position + 1 } else { position })
    };
    let mut modified: Vec<IndexedModified<D>> = diff.modified.iter().filter_map(|row| base.get(row.index).map(|item| IndexedModified { index: after_index(row.index), diff: inverse_item(&row.diff, item) })).collect();
    modified.sort_by_key(|row| row.index);
    let added = removed_sorted.iter().filter_map(|index| base.get(*index).map(|item| IndexedAdded { index: *index, item: item.clone() })).collect();
    IndexedDiff { removed: added_final, modified, added }
}
//#endregion 🔖️CommentsTriple

//#region 🔖️RowFieldDiff
/// 🔣️ One changed cell inside a row's sparse patch, keyed by the owning element's property
/// NAME (stable per-element schema — see module doc; positions can shift if `properties`
/// itself is replaced, names don't).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct PlyRowFieldChange {
    pub name: String,
    pub value: PlyValue,
}

/// 🔺️ Sparse per-property patch for one [`PlyRow`] — only changed cells appear.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct PlyRowDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<PlyRowFieldChange>,
}

impl PlyRowDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }
    /// ➕️ LWW per-field-name upsert absorb.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn absorb(&mut self, other: Self) {
        for change in other.fields {
            if let Some(existing) = self.fields.iter_mut().find(|f| f.name == change.name) {
                existing.value = change.value;
            } else {
                self.fields.push(change);
            }
        }
    }
}

/// ▶️ Applies a row patch in place, resolving each change's property name against `properties`
/// (the OWNING element's declared column order) to find the cell index.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_row_diff(properties: &[PlyProperty], row: &mut PlyRow, diff: &PlyRowDiff) {
    for change in &diff.fields {
        for (index, property) in properties.iter().enumerate() {
            if property.name() == change.name {
                row.values[index] = change.value.clone();
            }
        }
    }
}

//#endregion 🔖️RowFieldDiff

//#region 🔖️RowsTriple
/// 📦️ One `rows.modified[]` entity — `index` is the row's position in BASE.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct PlyRowModified {
    pub index: usize,
    pub diff: PlyRowDiff,
}

/// 📦️ One `rows.added[]` entity — `index` is the row's position in the FINAL sequence.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct PlyRowAdded {
    pub index: usize,
    pub row: PlyRow,
}

/// 🔺️ Index-keyed removed/modified/added triple over one element's `rows` (PLY rows have no
/// stable identity beyond position, same rationale as csv's `records` triple).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct PlyRowsDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<PlyRowModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<PlyRowAdded>,
}

impl PlyRowsDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
}

/// ▶️ Applies a rows patch in place: modified (BASE indices, applied first) → removed
/// (descending, so earlier removals never shift a later one still pending) → added (FINAL
/// indices, ascending, clamped) — apply-order contract from the recipe's `## Diff`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_rows_diff(properties: &[PlyProperty], rows: &mut Vec<PlyRow>, diff: &PlyRowsDiff) {
    for m in &diff.modified {
        apply_row_diff(properties, &mut rows[m.index], &m.diff);
    }
    let mut removed_desc = diff.removed.clone();
    removed_desc.sort_unstable_by(|a, b| b.cmp(a));
    for idx in removed_desc {
        rows.remove(idx);
    }
    let mut adds: Vec<&PlyRowAdded> = diff.added.iter().collect();
    adds.sort_by_key(|a| a.index);
    for a in adds {
        rows.insert(a.index, a.row.clone());
    }
}

//#region 🔖️RowsAbsorb
/// 🎰 One slot of a simulated post-removal/insertion row array (index-transport for absorb,
/// mirrors csv's `records` absorb — duplicated locally per-artifact, not shared, per the
/// recipe's anti-generic-code rule).
#[derive(Clone, Copy, Debug)]
enum RowSlot {
    Base(usize),
    Added(usize),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn row_simulate_slots(len: usize, removed: &[usize], added_indices: &[usize]) -> Vec<RowSlot> {
    let mut slots: Vec<RowSlot> = (0..len).map(RowSlot::Base).collect();
    let removed_desc = semio_s_artifact_stdio_contract::ordered_unique_descending(&removed);
    for r in removed_desc {
        if r < slots.len() {
            slots.remove(r);
        }
    }
    let mut order: Vec<usize> = (0..added_indices.len()).collect();
    order.sort_by_key(|&i| added_indices[i]);
    for i in order {
        let at = added_indices[i].min(slots.len());
        slots.insert(at, RowSlot::Added(i));
    }
    slots
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn row_base_len_hint(removed: &[usize], modified_indices: impl Iterator<Item = usize>, added_indices: impl Iterator<Item = usize>) -> usize {
    removed.iter().copied().chain(modified_indices).chain(added_indices).max().map_or(0, |m| m + 1)
}

/// ➕️ Structural, total, base-free absorb of two `rows` triples belonging to the SAME element
/// (`## Absorb` contract) — index-transport twin of `absorb_elements` below, one nesting level
/// deeper.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_rows(d1: PlyRowsDiff, d2: PlyRowsDiff) -> PlyRowsDiff {
    let d1_added_indices: Vec<usize> = d1.added.iter().map(|a| a.index).collect();
    let removed_count = {
        let r = semio_s_artifact_stdio_contract::ordered_unique(&d1.removed);
        r.len()
    };
    let needed_mid_len = d2.removed.iter().copied().chain(d2.modified.iter().map(|m| m.index)).max().map_or(0, |m| m + 1);
    let base_len = row_base_len_hint(&d1.removed, d1.modified.iter().map(|m| m.index), d1_added_indices.iter().copied()).max((needed_mid_len + removed_count).saturating_sub(d1.added.len()));
    let mid_slots = row_simulate_slots(base_len, &d1.removed, &d1_added_indices);

    let mut final_removed: Vec<usize> = d1.removed.clone();
    let mut modified_map: BTreeMap<usize, PlyRowDiff> = d1.modified.into_iter().map(|m| (m.index, m.diff)).collect();
    let mut added_alive: Vec<Option<PlyRowAdded>> = d1.added.into_iter().map(Some).collect();

    for mid_idx in &d2.removed {
        match mid_slots.get(*mid_idx) {
            Some(RowSlot::Base(b)) => {
                final_removed.push(*b);
                modified_map.remove(b);
            }
            Some(RowSlot::Added(ai)) => {
                added_alive[*ai] = None;
            }
            None => {}
        }
    }
    for m2 in &d2.modified {
        match mid_slots.get(m2.index) {
            Some(RowSlot::Base(b)) => {
                modified_map.entry(*b).or_default().absorb(m2.diff.clone());
            }
            Some(RowSlot::Added(ai)) => {
                if let Some(added) = added_alive[*ai].as_mut() {
                    apply_row_field_changes_by_position_fallback(&mut added.row, &m2.diff);
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
    let mut final_modified: Vec<PlyRowModified> = modified_map.into_iter().filter(|(_, d)| !d.is_empty()).map(|(index, diff)| PlyRowModified { index, diff }).collect();
    final_modified.sort_by_key(|m| m.index);

    let alive_mid_positions: Vec<usize> = mid_slots
        .iter()
        .enumerate()
        .filter_map(|(pos, slot)| match slot {
            RowSlot::Added(ai) if added_alive[*ai].is_some() => Some(pos),
            _ => None,
        })
        .collect();
    let d2_added_indices: Vec<usize> = d2.added.iter().map(|a| a.index).collect();
    let mid_len = d2.removed.iter().copied().chain(d2.modified.iter().map(|m| m.index)).chain(alive_mid_positions.iter().copied()).chain(d2_added_indices.iter().copied()).max().map_or(0, |m| m + 1);
    let after_slots = row_simulate_slots(mid_len, &d2.removed, &d2_added_indices);
    let mut mid_to_after: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
    for (pos, slot) in after_slots.iter().enumerate() {
        if let RowSlot::Base(m) = slot {
            mid_to_after.insert(*m, pos);
        }
    }

    let mut final_added: Vec<PlyRowAdded> = Vec::new();
    for (ai, alive) in added_alive.into_iter().enumerate() {
        if let Some(added) = alive {
            if let Some(mid_pos) = mid_slots.iter().position(|s| matches!(s, RowSlot::Added(idx) if *idx == ai)) {
                if let Some(after_pos) = mid_to_after.get(&mid_pos) {
                    final_added.push(PlyRowAdded { index: *after_pos, row: added.row });
                }
            }
        }
    }
    for a2 in d2.added {
        final_added.push(a2);
    }
    final_added.sort_by_key(|a| a.index);

    PlyRowsDiff { removed: final_removed, modified: final_modified, added: final_added }
}

/// ➕️ Scope cut (see `deviations`): patching a carried `added` ROW's cells by property NAME
/// requires the owning element's `properties` (name→position) — but row-level absorb is
/// base-free (no snapshot, no element context) per the `## Absorb` contract, so that anchor
/// isn't available here. Safe no-op fallback: a `SetRowProperty` absorbed onto a not-yet-applied
/// `InsertRow` of a DIFFERENT diff (both targeting the same still-uncommitted row, within an
/// EXISTING element) drops the patch rather than guessing a position — never corrupts data. The
/// canonical, tested "Add+SetField" case — `AddElement` (whole element, real `properties`
/// attached) followed by `SetRowProperty` on that same still-pending row — is unaffected: it
/// flows through `absorb_elements`' `apply_element_diff`-into-added path instead, which DOES
/// carry real `properties` and resolves correctly.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_row_field_changes_by_position_fallback(row: &mut PlyRow, diff: &PlyRowDiff) {
    let _ = (row, diff);
}
//#endregion 🔖️RowsAbsorb
//#endregion 🔖️RowsTriple

//#region 🔖️ElementDiff
/// 🔺️ Sparse per-field patch for one [`PlyElement`]. `properties` is a weak value-list —
/// whole-vec replaced, never sub-diffed (recipe's weak-entity rule) — because a property-schema
/// change invalidates positional row data anyway (see `element_between`'s scope-cut note).
/// 🧪️ F6: this struct is the exact real blocker cited in the module doc comment
/// (`properties: Option<Vec<PlyProperty>>` — `PlyProperty: DslField` unsatisfied); it needs no
/// `dsl` derive at all, it's a plain leaf type consumed by the hand-rolled `print_diff`/
/// `parse_diff`/`encode_diff`/`decode_diff` below.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct PlyElementDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<u64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<PlyProperty>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub rows: Option<PlyRowsDiff>,
}

impl PlyElementDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn is_empty(&self) -> bool {
        self.count.is_none() && self.properties.is_none() && self.rows.as_ref().is_none_or(PlyRowsDiff::is_empty)
    }
}

/// ▶️ Applies independently owned declaration metadata and occurrence changes.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_element_diff(element: &mut PlyElement, diff: &PlyElementDiff) {
    if let Some(count) = diff.count { element.count=count; }
    if let Some(props) = &diff.properties {
        element.properties = props.clone();
    }
    if let Some(rd) = &diff.rows {
        apply_rows_diff(&element.properties, &mut element.rows, rd);
    }
}

/// ➕️ Recursive per-field absorb of one element's patch into another.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_element_rows(base: &mut PlyElementDiff, other: PlyElementDiff) {
    if other.count.is_some() { base.count=other.count; }
    if other.properties.is_some() {
        base.properties = other.properties;
    }
    base.rows = match (base.rows.take(), other.rows) {
        (None, None) => None,
        (Some(d1), None) => Some(d1),
        (None, Some(d2)) => Some(d2),
        (Some(d1), Some(d2)) => Some(absorb_rows(d1, d2)),
    };
}

//#endregion 🔖️ElementDiff

//#region 🔖️ElementsTriple
/// 📦️ One `elements.modified[]` entity — `name` is the element's identity.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct PlyElementModified {
    pub name: String,
    pub diff: PlyElementDiff,
}

/// 📦️ One `elements.added[]` entity — `index` is the element's position in the FINAL sequence.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct PlyElementAdded {
    pub index: usize,
    pub element: PlyElement,
}

/// 🔺️ Sparse name-keyed `elements` triple.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct PlyElementsDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<String>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<PlyElementModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<PlyElementAdded>,
}

impl PlyElementsDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
}

/// ➕️ Name-keyed absorb (mirrors zip's `entries` absorb — no rename support for elements since
/// there is no `RenameElement` mutation, which simplifies the key-transport map to identity).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_elements(d1: Option<PlyElementsDiff>, d2: Option<PlyElementsDiff>) -> Option<PlyElementsDiff> {
    let (mut d1, d2) = match (d1, d2) {
        (None, None) => return None,
        (Some(d1), None) => return Some(d1),
        (None, Some(d2)) => return Some(d2),
        (Some(d1), Some(d2)) => (d1, d2),
    };

    let added_names: HashSet<String> = d1.added.iter().map(|a| a.element.name.clone()).collect();
    let mut merged_removed: Vec<String> = d1.removed;
    let mut annihilated: HashSet<String> = HashSet::new();
    let mut removed_shift = 0usize;
    for name in &d2.removed {
        if added_names.contains(name) {
            annihilated.insert(name.clone());
        } else {
            removed_shift += 1;
            if !merged_removed.contains(name) {
                merged_removed.push(name.clone());
            }
            d1.modified.retain(|m| &m.name != name);
        }
    }

    let mut merged_modified: Vec<PlyElementModified> = d1.modified;
    let mut merged_added: Vec<PlyElementAdded> = d1
        .added
        .into_iter()
        .filter(|a| !annihilated.contains(&a.element.name))
        .map(|mut a| {
            a.index = a.index.saturating_sub(removed_shift);
            a
        })
        .collect();

    for dm in &d2.modified {
        if added_names.contains(&dm.name) {
            if annihilated.contains(&dm.name) {
                continue;
            }
            if let Some(a) = merged_added.iter_mut().find(|a| a.element.name == dm.name) {
                apply_element_diff(&mut a.element, &dm.diff);
            }
        } else {
            if merged_removed.contains(&dm.name) {
                continue;
            }
            if let Some(existing) = merged_modified.iter_mut().find(|m| m.name == dm.name) {
                absorb_element_rows(&mut existing.diff, dm.diff.clone());
            } else {
                merged_modified.push(PlyElementModified { name: dm.name.clone(), diff: dm.diff.clone() });
            }
        }
    }

    merged_added.extend(d2.added);
    let merged = PlyElementsDiff { removed: merged_removed, modified: merged_modified, added: merged_added };
    if merged.is_empty() {
        None
    } else {
        Some(merged)
    }
}
//#endregion 🔖️ElementsTriple

/// 💬 Index-keyed comments triple: removed base indices, replaced comment texts, inserted comments at their final index.
pub type PlyCommentsDiff = IndexedDiff<String, String>;

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.ply`. `schema` is an identity field and never appears here.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.ply.diff")]
pub struct PlyDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<PlyFormat>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub comments: Option<PlyCommentsDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub elements: Option<PlyElementsDiff>,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn target_error(code: &'static str, message: &'static str, target: Vec<String>) -> MutationApplyError {
    MutationApplyError::new(code, message).at(target)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_rows_diff(properties: &[PlyProperty], rows: &[PlyRow], diff: &PlyRowsDiff, prefix: &[String]) -> MutationApplyResult<()> {
    let mut property_positions = BTreeMap::new();
    for (index, property) in properties.iter().enumerate() {
        if property_positions.insert(property.name(), index).is_some() {
            let target = [prefix, &["properties".to_string(), property.name().to_string()][..]].concat();
            return Err(target_error("mutation.apply.duplicate-base-target", "property names must be unique", target));
        }
    }
    let mut removed = BTreeSet::new();
    for &index in &diff.removed {
        let target = [prefix, &["rows".to_string(), index.to_string()][..]].concat();
        if index >= rows.len() || !removed.insert(index) {
            return Err(target_error("mutation.apply.invalid-remove-index", "row removal target must exist exactly once", target));
        }
    }
    let mut modified = BTreeSet::new();
    for entry in &diff.modified {
        let row_target = [prefix, &["rows".to_string(), entry.index.to_string()][..]].concat();
        if entry.index >= rows.len() || removed.contains(&entry.index) || !modified.insert(entry.index) {
            return Err(target_error("mutation.apply.invalid-modify-index", "row modification target must exist exactly once and remain present", row_target));
        }
        let mut fields = BTreeSet::new();
        for field in &entry.diff.fields {
            let target = [prefix, &["rows".to_string(), entry.index.to_string(), "fields".to_string(), field.name.clone()][..]].concat();
            let position = property_positions.get(field.name.as_str()).copied();
            if !fields.insert(field.name.as_str()) || position.is_none() || position.is_some_and(|value| value >= rows[entry.index].values.len()) {
                return Err(target_error("invalid-field-target", "row field target must be unique and resolve to an existing cell", target));
            }
        }
    }
    let mut additions: Vec<usize> = diff.added.iter().map(|entry| entry.index).collect();
    additions.sort_unstable();
    let mut previous = None;
    for (length, index) in (rows.len() - removed.len()..).zip(additions) {
        let target = [prefix, &["rows".to_string(), index.to_string()][..]].concat();
        if index > length || previous == Some(index) {
            return Err(target_error("mutation.apply.invalid-add-index", "row addition target must be unique and within the evolving sequence", target));
        }
        previous = Some(index);
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_elements_diff(base: &[PlyElement], diff: &PlyElementsDiff) -> MutationApplyResult<()> {
    let mut base_by_name = BTreeMap::new();
    for element in base {
        if base_by_name.insert(element.name.as_str(), element).is_some() {
            return Err(target_error("mutation.apply.duplicate-base-target", "base element names must be unique", vec!["elements".to_string(), element.name.clone()]));
        }
    }
    let mut removed = BTreeSet::new();
    for name in &diff.removed {
        if !base_by_name.contains_key(name.as_str()) || !removed.insert(name.as_str()) {
            return Err(target_error("mutation.apply.invalid-remove-target", "element removal target must exist exactly once", vec!["elements".to_string(), name.clone()]));
        }
    }
    let mut modified = BTreeSet::new();
    for entry in &diff.modified {
        let base_element = base_by_name.get(entry.name.as_str()).copied();
        if base_element.is_none() || removed.contains(entry.name.as_str()) || !modified.insert(entry.name.as_str()) {
            return Err(target_error("mutation.apply.invalid-modify-target", "element modification target must exist exactly once and remain present", vec!["elements".to_string(), entry.name.clone()]));
        }
        if let (Some(rows), Some(element)) = (&entry.diff.rows, base_element) {
            let properties = entry.diff.properties.as_deref().unwrap_or(&element.properties);
            validate_rows_diff(properties, &element.rows, rows, &["elements".to_string(), entry.name.clone()])?;
        }
    }
    let mut additions: Vec<&PlyElementAdded> = diff.added.iter().collect();
    additions.sort_by_key(|entry| entry.index);
    let mut added_names = BTreeSet::new();
    let mut previous = None;
    for (length, entry) in (base.len() - removed.len()..).zip(additions) {
        if base_by_name.contains_key(entry.element.name.as_str()) || !added_names.insert(entry.element.name.as_str()) || entry.index > length || previous == Some(entry.index) {
            return Err(target_error("mutation.apply.invalid-add-target", "element name and position must be unique and valid", vec!["elements".to_string(), entry.element.name.clone()]));
        }
        previous = Some(entry.index);
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_ply_diff_unchecked(diff: &PlyDiff, base: &PlySnapshot) -> PlySnapshot {
    let mut next = base.clone();
    if let Some(format) = diff.format {
        next.format = format;
    }
    if let Some(comments) = &diff.comments {
        next.comments = apply_indexed(&base.comments, comments, |_, replacement: &String| replacement.clone());
    }
    if let Some(elements) = &diff.elements {
        for modified in &elements.modified {
            for element in &mut next.elements {
                if element.name == modified.name {
                    apply_element_diff(element, &modified.diff);
                }
            }
        }
        let removed: HashSet<&str> = elements.removed.iter().map(String::as_str).collect();
        next.elements.retain(|element| !removed.contains(element.name.as_str()));
        let mut additions: Vec<&PlyElementAdded> = elements.added.iter().collect();
        additions.sort_by_key(|entry| entry.index);
        for entry in additions {
            next.elements.insert(entry.index, entry.element.clone());
        }
    }
    next
}

impl MutationDiff<PlySnapshot> for PlyDiff {
    fn apply(&self, base: &PlySnapshot, _capability: protocol::ApplyCapability) -> MutationApplyResult<PlySnapshot> {
        if let Some(diff) = &self.comments {
            validate_indexed(&base.comments, diff, |_, _| Ok(()))?;
        }
        if let Some(diff) = &self.elements {
            validate_elements_diff(&base.elements, diff)?;
        }
        Ok(apply_ply_diff_unchecked(self, base))
    }

    /// ➕️ Structural, total, base-free sequential-coalesce (`## Absorb` contract). Scalars: LWW.
    /// `elements`: name-keyed transport (no renames — `AddElement`/`RemoveElement` only), one
    /// nested `rows` absorb per surviving modified element.
    fn absorb(&mut self, other: Self) {
        if other.format.is_some() {
            self.format = other.format;
        }
        match (&mut self.comments, other.comments) {
            (Some(existing), Some(other_comments)) => absorb_indexed(existing, other_comments, |a: &mut String, b: String| *a = b, |t: &mut String, d: &String| *t = d.clone()),
            (slot @ None, Some(other_comments)) => *slot = Some(other_comments),
            _ => {}
        }
        self.elements = absorb_elements(self.elements.take(), other.elements);
    }
}

/// ↩️ Negative rows for one row triple against its BASE rows: added rows become removals at their final index, removed rows return
/// at their base index, and each modified row restores its base cells at the index the row has after the diff.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_rows(properties: &[PlyProperty], diff: &PlyRowsDiff, base: &[PlyRow]) -> PlyRowsDiff {
    let removed_sorted = semio_s_artifact_stdio_contract::ordered_unique(&diff.removed);
    let mut added_final: Vec<usize> = diff.added.iter().map(|added| added.index).collect();
    added_final.sort_unstable();
    let after_index = |index: usize| {
        let survivor = index - removed_sorted.iter().filter(|dropped| **dropped < index).count();
        added_final.iter().fold(survivor, |position, inserted| if *inserted <= position { position + 1 } else { position })
    };
    let restore = |row_diff: &PlyRowDiff, row: &PlyRow| PlyRowDiff {
        fields: row_diff.fields.iter().filter_map(|change| properties.iter().position(|property| property.name() == change.name).and_then(|cell| row.values.get(cell)).map(|value| PlyRowFieldChange { name: change.name.clone(), value: value.clone() })).collect(),
    };
    let mut modified: Vec<PlyRowModified> = diff.modified.iter().filter_map(|row| base.get(row.index).map(|base_row| PlyRowModified { index: after_index(row.index), diff: restore(&row.diff, base_row) })).collect();
    modified.sort_by_key(|row| row.index);
    let added = removed_sorted.iter().filter_map(|index| base.get(*index).map(|row| PlyRowAdded { index: *index, row: row.clone() })).collect();
    PlyRowsDiff { removed: added_final, modified, added }
}

/// ↩️ The element patch restoring exactly what `diff` replaces in the BASE element.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_element_diff(diff: &PlyElementDiff, base: &PlyElement) -> PlyElementDiff {
    PlyElementDiff {
        count: diff.count.map(|_| base.count),
        properties: diff.properties.as_ref().map(|_| base.properties.clone()),
        rows: diff.rows.as_ref().map(|rows| inverse_rows(&base.properties, rows, &base.rows)).filter(|rows| !rows.is_empty()),
    }
}

/// ↩️ Negative rows for the elements triple against its BASE elements; removed elements return at their base index.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_elements(diff: &PlyElementsDiff, base: &[PlyElement]) -> PlyElementsDiff {
    let mut added_final: Vec<&PlyElementAdded> = diff.added.iter().collect();
    added_final.sort_by_key(|added| added.index);
    let removed = added_final.iter().map(|added| added.element.name.clone()).collect();
    let modified = diff.modified.iter().filter_map(|row| base.iter().find(|element| element.name == row.name).map(|element| PlyElementModified { name: row.name.clone(), diff: inverse_element_diff(&row.diff, element) })).collect();
    let mut restored: Vec<PlyElementAdded> = diff.removed.iter().filter_map(|name| base.iter().position(|element| &element.name == name).map(|index| PlyElementAdded { index, element: base[index].clone() })).collect();
    restored.sort_by_key(|added| added.index);
    PlyElementsDiff { removed, modified, added: restored }
}

impl DiffAlgebra<PlySnapshot> for PlyDiff {
    /// ↩️ Concrete diff-level undo: the format and comments return to their base values, and the elements triple turns into its
    /// negative rows (added elements become removals, removed elements return at their base index, modified elements restore
    /// their base declaration and rows).
    fn inverse(&self, base: &PlySnapshot) -> Self {
        PlyDiff {
            format: self.format.map(|_| base.format),
            comments: self.comments.as_ref().map(|comments| inverse_indexed(comments, &base.comments, |_, text: &String| text.clone())).filter(|comments| !comments.is_empty()),
            elements: self.elements.as_ref().map(|elements| inverse_elements(elements, &base.elements)).filter(|elements| !elements.is_empty()),
        }
    }

    fn is_empty(&self) -> bool {
        self.format.is_none() && self.comments.is_none() && self.elements.as_ref().is_none_or(PlyElementsDiff::is_empty)
    }
}
//#endregion 🔖️Diff

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_format(format: PlyFormat) -> PlyDiff {
    PlyDiff { format: Some(format), comments: None, elements: None }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_comment(index: usize, comment: String) -> PlyDiff {
    PlyDiff { format: None, comments: Some(PlyCommentsDiff { removed: vec![], modified: vec![], added: vec![IndexedAdded { index, item: comment }] }), elements: None }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_comment(index: usize) -> PlyDiff {
    PlyDiff { format: None, comments: Some(PlyCommentsDiff { removed: vec![index], modified: vec![], added: vec![] }), elements: None }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_add_element(index: usize, element: PlyElement) -> PlyDiff {
    PlyDiff { format: None, comments: None, elements: Some(PlyElementsDiff { removed: vec![], modified: vec![], added: vec![PlyElementAdded { index, element }] }) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_element(name: &str) -> PlyDiff {
    PlyDiff { format: None, comments: None, elements: Some(PlyElementsDiff { removed: vec![name.to_string()], modified: vec![], added: vec![] }) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_element_field(name: &str, diff: PlyElementDiff) -> PlyDiff {
    PlyDiff { format: None, comments: None, elements: Some(PlyElementsDiff { removed: vec![], modified: vec![PlyElementModified { name: name.to_string(), diff }], added: vec![] }) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_row(element_name: &str, index: usize, row: PlyRow) -> PlyDiff {
    diff_element_field(element_name, PlyElementDiff { count: None, properties: None, rows: Some(PlyRowsDiff { removed: vec![], modified: vec![], added: vec![PlyRowAdded { index, row }] }) })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_row(element_name: &str, index: usize) -> PlyDiff {
    diff_element_field(element_name, PlyElementDiff { count: None, properties: None, rows: Some(PlyRowsDiff { removed: vec![index], modified: vec![], added: vec![] }) })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_row_property(element_name: &str, row_index: usize, property_name: &str, value: PlyValue) -> PlyDiff {
    diff_element_field(
        element_name,
        PlyElementDiff {
            count: None,
            properties: None,
            rows: Some(PlyRowsDiff { removed: vec![], modified: vec![PlyRowModified { index: row_index, diff: PlyRowDiff { fields: vec![PlyRowFieldChange { name: property_name.to_string(), value }] } }], added: vec![] }),
        },
    )
}
//#endregion 🔖️MutationDiffBuilders

//#region 🔖️HandcraftedDiffCodec
/// 🧪️ F6: **hand-rolled** `protocol::DiffCodec` for `PlyDiff` — `#[derive(dsl::DslDiff)]` is not
/// usable (see the module doc comment for the real `cargo check` citation: `PlyProperty`/
/// `PlyValue` are data-carrying enums reachable from `PlyElementDiff::properties` and
/// `PlyRowFieldChange::value`, the 3a "enum-in-tree" blocker per `f6-recon-report.md` §3a).
///
/// **Grammar** (real, not `serde_json`), following the ticket's §5 template verbatim: one
/// space-separated `name=value` token per changed top-level field (absent token = unchanged).
/// Bytes/strings are lowercase hex (`enc_str`/`dec_str` — no external base64 dep, no escaping).
/// `Option<T>` values use the uniform `[0]`=None / `[1,<T>]`=Some(T) tag. Structs are positional
/// `[f1,f2,...]` tuples. Data-carrying enums (`PlyProperty`, `PlyValue`) use a single-uppercase
/// (or, for `PlyValue`'s eight scalar kinds, single-lowercase) tag prefix immediately followed by
/// the bracketed payload. Collection triples print as `{[removed];[modified];[added]}` — for the
/// index-keyed `rows` triple, `removed`/`modified` are index-keyed; for the name-keyed `elements`
/// triple, `removed`/`modified` are NAME-keyed (hex) while `added` stays index-keyed (matches
/// `PlyElementsDiff`'s own real shape — see `f6-ply-report.md` for why this deliberately deviates
/// from gif89a's uniform-index-keyed triple helper).
///
/// Worked example (captured from a real test run, see `diff_codec_text_binary_roundtrip_law`):
/// `format=6c comments=[68656c6c6f] elements={[666163e5];[];[76657274657865:[P:[...],R:{...}]]}`
/// (illustrative shape only — see the test for the literal printed string).
//#region 🔖️Primitives






//#endregion 🔖️Primitives

//#region 🔖️ValueCodecs




















//#endregion 🔖️ValueCodecs

//#region 🔖️DiffValueCodecs















//#endregion 🔖️DiffValueCodecs

//#region 🔖️RealBinaryPrimitives























//#endregion 🔖️RealBinaryPrimitives

//#region 🔖️RealBinaryDiffFrame










//#endregion 🔖️RealBinaryDiffFrame

//#region 🔖️TopLevel




//#endregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec

//#region 🔖️DemoDiffCases
/// ✅️ Every representative `PlyDiff` shape (empty, plus a real `between()` result in BOTH
/// directions over `sweep_a()`/`sweep_b()`) — the single case list `diff_codec_text_binary_
/// roundtrip_law` (this file) AND `diff_grammar_conformance_law`/`protocol_walk_law`
/// (`⚙️engine/🦀️.rs`) all exercise. Covers every scalar field, the name-keyed `elements`
/// triple in all three flavors (removed/modified/added) simultaneously, the nested index-keyed
/// `rows` triple, the weak `properties` replace, and both `PlyProperty`/`PlyValue` enum tag
/// families (incl. `PlyValue::List`'s recursion).
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn sweep_a() -> PlySnapshot {
    PlySnapshot {
        schema: crate::STDIO_PLY_DOCUMENT_SCHEMA.into(),
        format: PlyFormat::Ascii,
        comments: vec!["a".into()],
        elements: vec![
            PlyElement {
                name: "vertex".into(),
                count: 2,
                properties: vec![PlyProperty::Scalar { name: "x".into(), kind: PlyScalarType::Float }, PlyProperty::Scalar { name: "y".into(), kind: PlyScalarType::Float }],
                rows: vec![PlyRow { values: vec![PlyValue::Float(0.0), PlyValue::Float(0.0)] }, PlyRow { values: vec![PlyValue::Float(1.0), PlyValue::Float(1.0)] }],
            },
            PlyElement {
                name: "face".into(),
                count: 1,
                properties: vec![PlyProperty::List { name: "vertex_indices".into(), count_kind: PlyScalarType::UChar, value_kind: PlyScalarType::Int }],
                rows: vec![PlyRow { values: vec![PlyValue::List(vec![PlyValue::Int(0), PlyValue::Int(1), PlyValue::Int(2)])] }],
            },
        ],
    }
}

#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn sweep_b() -> PlySnapshot {
    PlySnapshot {
        schema: crate::STDIO_PLY_DOCUMENT_SCHEMA.into(),
        format: PlyFormat::BinaryLittleEndian,
        comments: vec!["a".into(), "b".into()],
        elements: vec![
            PlyElement {
                name: "vertex".into(),
                count: 1,
                properties: vec![PlyProperty::Scalar { name: "nx".into(), kind: PlyScalarType::Double }, PlyProperty::Scalar { name: "ny".into(), kind: PlyScalarType::Double }],
                rows: vec![PlyRow { values: vec![PlyValue::Double(9.0), PlyValue::Double(-9.5)] }],
            },
            PlyElement { name: "edge".into(), count: 1, properties: vec![PlyProperty::Scalar { name: "weight".into(), kind: PlyScalarType::Double }], rows: vec![PlyRow { values: vec![PlyValue::Double(3.5)] }] },
        ],
    }
}

#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<PlyDiff> {
    vec![
        PlyDiff::default(),
        PlyDiff {
            format: Some(PlyFormat::BinaryLittleEndian),
            comments: Some(PlyCommentsDiff {
                removed: vec![0],
                modified: vec![IndexedModified { index: 1, diff: "changed".to_string() }],
                added: vec![IndexedAdded { index: 0, item: "added".to_string() }],
            }),
            elements: Some(PlyElementsDiff { removed: vec!["face".into()], ..Default::default() }),
        },
        PlyDiff { format: Some(PlyFormat::Ascii), ..Default::default() },
    ]
}
//#endregion 🔖️DemoDiffCases

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️codec/🦀️.rs"]
mod codec_tests;
//#endregion 🧪️Tests

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::schema::snapshot::PlyValue;
//#endregion 🔁️Re-exports

#[cfg(test)]
use protocol::{DiffBinary,DiffText};
