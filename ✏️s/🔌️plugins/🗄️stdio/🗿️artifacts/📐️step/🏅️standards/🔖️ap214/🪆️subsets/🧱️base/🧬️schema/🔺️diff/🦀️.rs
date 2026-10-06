//! 🔺️ StepDiff — handcrafted sparse diff. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: replaces the old
//! `StepDiff{snapshot: Option<StepSnapshot>}` full-replace template with a real per-field patch —
//! three scalar HEADER-record slots (`file_description`/`file_name`/`file_schema`, each a weak
//! value struct per the recipe's strong/weak split, whole-value replaced) plus an id-keyed
//! `entities` triple. `entities.modified[].diff.args` is a SEPARATE index-keyed triple (Part-21
//! entity argument lists are positional, not named) whose items (`StepValue`) are themselves weak
//! — "the diff IS the whole new value", same pattern as gif's `GifCommentsDiff`/`String`.

/// 🧩 Ordered removed keys, modified values, and inserted items.
pub(crate) type IndexedDiffParts<D, T> = (Vec<usize>, Vec<(usize, D)>, Vec<(usize, T)>);

use std::collections::{BTreeMap, BTreeSet, HashSet};

use crate::StepSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};

//#region 🔖️IndexTransport
/// 📐️ Shared rank/unrank arithmetic for index-keyed collection diffs (`between`/`absorb`/
/// `inverse`) — see `🧬️schema-design.md` §Absorb. `excluded_sorted` must be sorted ascending.
/// Own copy (not imported from gif) per the recipe's specific-code mandate — small and
/// self-contained enough that duplicating it per artifact is the honest choice, not a defect.
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn transport_forward(index: usize, removed_sorted: &[usize], added_index_sorted: &[usize]) -> usize {
    unrank_excluding(rank_excluding(index, removed_sorted), added_index_sorted)
}
//#endregion 🔖️IndexTransport

//#region 🔖️GenericIndexedCollectionAlgebra
/// 🧮️ Sequential-coalesce absorb for an index-keyed collection triple, generic over item `T` and
/// per-item diff `D` (here always `T == D == StepValue`, a weak collection whose "diff" is the
/// whole new value). Canonical correctness verified against the plan's 3 mandated cases in this
/// module's tests.
#[allow(clippy::too_many_arguments)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_indexed_collection<T: Clone, D: Clone>(
    removed1: Vec<usize>,
    modified1: Vec<(usize, D)>,
    added1: Vec<(usize, T)>,
    removed2: Vec<usize>,
    modified2: Vec<(usize, D)>,
    added2: Vec<(usize, T)>,
    mut absorb_diff: impl FnMut(&mut D, D),
    apply_diff_to_item: impl Fn(&D, &T) -> T,
) -> IndexedDiffParts<D, T> {
    let mut removed1_sorted = removed1;
    removed1_sorted.sort_unstable();
    let mut added1_index_sorted: Vec<usize> = added1.iter().map(|(i, _)| *i).collect();
    added1_index_sorted.sort_unstable();
    let mut removed2_sorted = removed2;
    removed2_sorted.sort_unstable();
    let mut added2_index_sorted: Vec<usize> = added2.iter().map(|(i, _)| *i).collect();
    added2_index_sorted.sort_unstable();

    let mut merged_added: Vec<(usize, T)> = added1;
    let mut annihilated: HashSet<usize> = Default::default();

    let mut merged_removed_base: Vec<usize> = removed1_sorted.clone();
    for &r2 in &removed2_sorted {
        if added1_index_sorted.binary_search(&r2).is_ok() {
            annihilated.insert(r2);
            merged_added.retain(|(i, _)| *i != r2);
        } else {
            let post_remove_rank = rank_excluding(r2, &added1_index_sorted);
            let base_index = unrank_excluding(post_remove_rank, &removed1_sorted);
            merged_removed_base.push(base_index);
        }
    }
    merged_removed_base.sort_unstable();
    merged_removed_base.dedup();

    let mut modified_map: BTreeMap<usize, D> = modified1.into_iter().collect();
    for base_index in &merged_removed_base {
        modified_map.remove(base_index);
    }
    for (mp, dd2) in modified2 {
        if annihilated.contains(&mp) {
            continue;
        }
        if added1_index_sorted.binary_search(&mp).is_ok() {
            if let Some(entry) = merged_added.iter_mut().find(|(i, _)| *i == mp) {
                entry.1 = apply_diff_to_item(&dd2, &entry.1);
            }
        } else {
            let post_remove_rank = rank_excluding(mp, &added1_index_sorted);
            let base_index = unrank_excluding(post_remove_rank, &removed1_sorted);
            if merged_removed_base.binary_search(&base_index).is_ok() {
                continue;
            }
            modified_map.entry(base_index).and_modify(|d| absorb_diff(d, dd2.clone())).or_insert(dd2);
        }
    }
    let merged_modified: Vec<(usize, D)> = modified_map.into_iter().collect();

    let mut merged_added_final: Vec<(usize, T)> = merged_added
        .into_iter()
        .map(|(mp, item)| {
            let after_pos = if removed2_sorted.binary_search(&mp).is_ok() {
                mp
            } else {
                let post_remove_rank = rank_excluding(mp, &removed2_sorted);
                unrank_excluding(post_remove_rank, &added2_index_sorted)
            };
            (after_pos, item)
        })
        .collect();
    merged_added_final.extend(added2);
    merged_added_final.sort_by_key(|(i, _)| *i);

    (merged_removed_base, merged_modified, merged_added_final)
}

/// ↩️ Diff-level inverse for an index-keyed collection triple, given the ORIGINAL base items.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_indexed_collection<T: Clone, D: Clone>(removed: &[usize], modified: &[(usize, D)], added: &[(usize, T)], base_items: &[T], diff_inverse: impl Fn(&D, &T) -> D) -> IndexedDiffParts<D, T> {
    let mut removed_sorted = removed.to_vec();
    removed_sorted.sort_unstable();
    let mut added_index_sorted: Vec<usize> = added.iter().map(|(i, _)| *i).collect();
    added_index_sorted.sort_unstable();

    let mut inv_removed: Vec<usize> = added.iter().map(|(i, _)| *i).collect();
    let mut inv_modified: Vec<(usize, D)> = Vec::new();
    for (base_index, d) in modified {
        if let Some(orig) = base_items.get(*base_index) {
            let after_index = transport_forward(*base_index, &removed_sorted, &added_index_sorted);
            inv_modified.push((after_index, diff_inverse(d, orig)));
        }
    }
    let mut inv_added: Vec<(usize, T)> = Vec::new();
    for &r in removed {
        if let Some(orig) = base_items.get(r) {
            inv_added.push((r, orig.clone()));
        }
    }
    inv_removed.sort_unstable();
    inv_added.sort_by_key(|(i, _)| *i);
    (inv_removed, inv_modified, inv_added)
}
//#endregion 🔖️GenericIndexedCollectionAlgebra

//#region 🔖️ArgsDiff
/// 🔺️ One `entities.modified[].diff.args.modified[]` entry — `StepValue` is weak (no further
/// sub-structure worth diffing), so the "diff" IS the whole new value.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct StepArgModified {
    pub index: usize,
    pub value: StepValue,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct StepArgAdded {
    pub index: usize,
    pub value: StepValue,
}

/// 🔺️ Index-keyed collection triple for `StepEntity::args` — Part-21 entity argument lists are
/// positional, never named.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct StepArgsDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<StepArgModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<StepArgAdded>,
}

impl StepArgsDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn between(base: &[StepValue], other: &[StepValue]) -> Self {
        let min = base.len().min(other.len());
        let mut modified = Vec::new();
        for i in 0..min {
            if base[i] != other[i] {
                modified.push(StepArgModified { index: i, value: other[i].clone() });
            }
        }
        let removed: Vec<usize> = (min..base.len()).collect();
        let added: Vec<StepArgAdded> = (min..other.len()).map(|i| StepArgAdded { index: i, value: other[i].clone() }).collect();
        Self { removed, modified, added }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn apply(&self, base: &[StepValue]) -> Vec<StepValue> {
        let mut next = base.to_vec();
        for m in &self.modified {
            next[m.index] = m.value.clone();
        }
        let mut removed_sorted = self.removed.clone();
        removed_sorted.sort_unstable();
        removed_sorted.reverse();
        for &r in &removed_sorted {
            next.remove(r);
        }
        let mut added_sorted = self.added.clone();
        added_sorted.sort_by_key(|a| a.index);
        for a in added_sorted {
            next.insert(a.index, a.value);
        }
        next
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn absorb(&mut self, other: Self) {
        let (removed, modified, added) = absorb_indexed_collection(
            std::mem::take(&mut self.removed),
            std::mem::take(&mut self.modified).into_iter().map(|m| (m.index, m.value)).collect(),
            std::mem::take(&mut self.added).into_iter().map(|a| (a.index, a.value)).collect(),
            other.removed,
            other.modified.into_iter().map(|m| (m.index, m.value)).collect(),
            other.added.into_iter().map(|a| (a.index, a.value)).collect(),
            |d: &mut StepValue, o: StepValue| *d = o,
            |d: &StepValue, _item: &StepValue| d.clone(),
        );
        self.removed = removed;
        self.modified = modified.into_iter().map(|(index, value)| StepArgModified { index, value }).collect();
        self.added = added.into_iter().map(|(index, value)| StepArgAdded { index, value }).collect();
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn inverse(&self, base_args: &[StepValue]) -> Self {
        let (removed, modified, added) = inverse_indexed_collection(
            &self.removed,
            &self.modified.iter().map(|m| (m.index, m.value.clone())).collect::<Vec<_>>(),
            &self.added.iter().map(|a| (a.index, a.value.clone())).collect::<Vec<_>>(),
            base_args,
            |_d: &StepValue, item: &StepValue| item.clone(),
        );
        Self { removed, modified: modified.into_iter().map(|(index, value)| StepArgModified { index, value }).collect(), added: added.into_iter().map(|(index, value)| StepArgAdded { index, value }).collect() }
    }
}
//#endregion 🔖️ArgsDiff

//#region 🔖️EntityDiff
/// 🔺️ Sparse per-field diff for one [`StepEntity`] — a strong entity, per the recipe. `complex`
/// (the rare multi-type-instance extension) is a weak value list, whole-vec replaced.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct StepEntityDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<StepArgsDiff>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub complex: Option<Vec<StepComplexType>>,
}

impl StepEntityDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.name.is_none() && self.args.is_none() && self.complex.is_none()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn between(base: &StepEntity, other: &StepEntity) -> Self {
        let args_diff = StepArgsDiff::between(&base.args, &other.args);
        Self { name: (base.name != other.name).then(|| other.name.clone()), args: (!args_diff.is_empty()).then_some(args_diff), complex: (base.complex != other.complex).then(|| other.complex.clone()) }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn apply(&self, base: &StepEntity) -> StepEntity {
        let mut next = base.clone();
        if let Some(v) = &self.name {
            next.name = v.clone();
        }
        if let Some(d) = &self.args {
            next.args = d.apply(&next.args);
        }
        if let Some(v) = &self.complex {
            next.complex = v.clone();
        }
        next
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn inverse(&self, base: &StepEntity) -> Self {
        Self { name: self.name.as_ref().map(|_| base.name.clone()), args: self.args.as_ref().map(|d| d.inverse(&base.args)), complex: self.complex.as_ref().map(|_| base.complex.clone()) }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn absorb(&mut self, other: Self) {
        if other.name.is_some() {
            self.name = other.name;
        }
        match (&mut self.args, other.args) {
            (Some(mine), Some(theirs)) => mine.absorb(theirs),
            (slot @ None, Some(theirs)) => *slot = Some(theirs),
            _ => {}
        }
        if other.complex.is_some() {
            self.complex = other.complex;
        }
    }
}
//#endregion 🔖️EntityDiff

//#region 🔖️EntitiesTriple
/// 📦️ One `entities.modified[]` entity — `id` is stable Part-21 instance-number identity (never
/// renumbered by a mutation in this recipe; unlike zip's names, no rename tracking is needed).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct StepEntityModified {
    pub id: u64,
    pub diff: StepEntityDiff,
}

/// 📦️ One `entities.added[]` entity — `index` is the entity's position in the FINAL sequence.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct StepEntityAdded {
    pub index: usize,
    pub entity: StepEntity,
}

/// 📦️ Sparse id-keyed `entities` triple.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct StepEntitiesDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<u64>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<StepEntityModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<StepEntityAdded>,
}

impl StepEntitiesDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn between(base: &[StepEntity], other: &[StepEntity]) -> Self {
        let base_ids: HashSet<u64> = base.iter().map(|e| e.id).collect();
        let other_ids: HashSet<u64> = other.iter().map(|e| e.id).collect();

        let removed: Vec<u64> = base.iter().filter(|e| !other_ids.contains(&e.id)).map(|e| e.id).collect();

        let mut modified = Vec::new();
        for be in base {
            if let Some(oe) = other.iter().find(|o| o.id == be.id) {
                let d = StepEntityDiff::between(be, oe);
                if !d.is_empty() {
                    modified.push(StepEntityModified { id: be.id, diff: d });
                }
            }
        }

        let added: Vec<StepEntityAdded> = other.iter().enumerate().filter(|(_, e)| !base_ids.contains(&e.id)).map(|(index, e)| StepEntityAdded { index, entity: e.clone() }).collect();

        Self { removed, modified, added }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn apply(&self, base: &[StepEntity]) -> Vec<StepEntity> {
        let mut entities: Vec<StepEntity> = base.to_vec();
        if !self.removed.is_empty() {
            let removed: HashSet<u64> = self.removed.iter().copied().collect();
            entities.retain(|e| !removed.contains(&e.id));
        }
        for m in &self.modified {
            for entity in &mut entities {
                if entity.id == m.id {
                    *entity = m.diff.apply(entity);
                }
            }
        }
        let mut adds: Vec<&StepEntityAdded> = self.added.iter().collect();
        adds.sort_by_key(|a| a.index);
        for a in adds {
            entities.insert(a.index, a.entity.clone());
        }
        entities
    }
}

/// ➕️ Free-function core of `entities` absorb — id-keyed, no rename transport needed (unlike
/// zip's names, a `#123` instance number is never reassigned by this recipe's mutation
/// vocabulary). Structural, total, base-free, sequential-coalesce, same shape as zip's
/// `absorb_entries` minus the rename machinery.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_entities(d1: Option<StepEntitiesDiff>, d2: Option<StepEntitiesDiff>) -> Option<StepEntitiesDiff> {
    let (mut d1, d2) = match (d1, d2) {
        (None, None) => return None,
        (Some(d1), None) => return Some(d1),
        (None, Some(d2)) => return Some(d2),
        (Some(d1), Some(d2)) => (d1, d2),
    };

    let added_ids: HashSet<u64> = d1.added.iter().map(|a| a.entity.id).collect();
    let mut merged_removed: Vec<u64> = std::mem::take(&mut d1.removed);
    let mut annihilated: HashSet<u64> = HashSet::new();
    let mut removed_shift_count = 0usize;

    for id in &d2.removed {
        if added_ids.contains(id) {
            annihilated.insert(*id);
        } else {
            removed_shift_count += 1;
            if !merged_removed.contains(id) {
                merged_removed.push(*id);
            }
            d1.modified.retain(|m| m.id != *id);
        }
    }

    let mut merged_modified: Vec<StepEntityModified> = d1.modified;
    let mut merged_added: Vec<StepEntityAdded> = d1
        .added
        .into_iter()
        .filter(|a| !annihilated.contains(&a.entity.id))
        .map(|mut a| {
            a.index = a.index.saturating_sub(removed_shift_count);
            a
        })
        .collect();

    for dm in d2.modified {
        if added_ids.contains(&dm.id) {
            if annihilated.contains(&dm.id) {
                continue; // modified-of-annihilated-add: moot.
            }
            if let Some(a) = merged_added.iter_mut().find(|a| a.entity.id == dm.id) {
                a.entity = dm.diff.apply(&a.entity);
            }
        } else {
            if merged_removed.contains(&dm.id) {
                continue; // modified-of-removed: illegal, ignored (matches apply()'s no-op rule).
            }
            if let Some(existing) = merged_modified.iter_mut().find(|m| m.id == dm.id) {
                existing.diff.absorb(dm.diff.clone());
            } else {
                merged_modified.push(StepEntityModified { id: dm.id, diff: dm.diff.clone() });
            }
        }
    }

    merged_added.extend(d2.added);

    let merged = StepEntitiesDiff { removed: merged_removed, modified: merged_modified, added: merged_added };
    if merged.is_empty() {
        None
    } else {
        Some(merged)
    }
}
//#endregion 🔖️EntitiesTriple

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.step`. No `snapshot: Option<StepSnapshot>` full-replace slot anywhere.
/// `schema` is an identity field and never appears here. The three HEADER records are scalar
/// weak-value slots (never sub-diffed) per the recipe.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.step.diff")]
pub struct StepDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub file_description: Option<StepFileDescription>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<StepFileName>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub file_schema: Option<StepFileSchema>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub entities: Option<StepEntitiesDiff>,
}

impl StepDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty_diff(&self) -> bool {
        self.file_description.is_none() && self.file_name.is_none() && self.file_schema.is_none() && self.entities.as_ref().is_none_or(StepEntitiesDiff::is_empty)
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn target_error(code: &'static str, message: &'static str, target: Vec<String>) -> MutationApplyError {
    MutationApplyError::new(code, message).at(target)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_args_diff(base_len: usize, diff: &StepArgsDiff, prefix: &[String]) -> MutationApplyResult<()> {
    let mut removed = BTreeSet::new();
    for &index in &diff.removed {
        let mut target = prefix.to_vec();
        target.extend(["args".to_string(), index.to_string()]);
        if index >= base_len || !removed.insert(index) {
            return Err(target_error("mutation.apply.invalid-remove-index", "argument removal target must exist exactly once", target));
        }
    }
    let mut modified = BTreeSet::new();
    for entry in &diff.modified {
        let mut target = prefix.to_vec();
        target.extend(["args".to_string(), entry.index.to_string()]);
        if entry.index >= base_len || removed.contains(&entry.index) || !modified.insert(entry.index) {
            return Err(target_error("mutation.apply.invalid-modify-index", "argument modification target must exist exactly once and remain present", target));
        }
    }
    let mut additions: Vec<usize> = diff.added.iter().map(|entry| entry.index).collect();
    additions.sort_unstable();
    let mut previous = None;
    for (length, index) in (base_len - removed.len()..).zip(additions) {
        let mut target = prefix.to_vec();
        target.extend(["args".to_string(), index.to_string()]);
        if index > length || previous == Some(index) {
            return Err(target_error("mutation.apply.invalid-add-index", "argument addition target must be unique and within the evolving sequence", target));
        }
        previous = Some(index);
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_entities_diff(base: &[StepEntity], diff: &StepEntitiesDiff) -> MutationApplyResult<()> {
    let mut base_by_id = BTreeMap::new();
    for entity in base {
        if base_by_id.insert(entity.id, entity).is_some() {
            return Err(target_error("mutation.apply.duplicate-base-target", "base entity ids must be unique", vec!["entities".to_string(), entity.id.to_string()]));
        }
    }
    let mut removed = BTreeSet::new();
    for &id in &diff.removed {
        if !base_by_id.contains_key(&id) || !removed.insert(id) {
            return Err(target_error("mutation.apply.invalid-remove-target", "entity removal target must exist exactly once", vec!["entities".to_string(), id.to_string()]));
        }
    }
    let mut modified = BTreeSet::new();
    for entry in &diff.modified {
        let base_entity = base_by_id.get(&entry.id);
        if base_entity.is_none() || removed.contains(&entry.id) || !modified.insert(entry.id) {
            return Err(target_error("mutation.apply.invalid-modify-target", "entity modification target must exist exactly once and remain present", vec!["entities".to_string(), entry.id.to_string()]));
        }
        if let Some(args) = &entry.diff.args {
            validate_args_diff(base_entity.map(|entity| entity.args.len()).unwrap_or_default(), args, &["entities".to_string(), entry.id.to_string()])?;
        }
    }
    let mut additions: Vec<&StepEntityAdded> = diff.added.iter().collect();
    additions.sort_by_key(|entry| entry.index);
    let mut added_ids = BTreeSet::new();
    let mut previous = None;
    for (length, entry) in (base.len() - removed.len()..).zip(additions) {
        if base_by_id.contains_key(&entry.entity.id) || !added_ids.insert(entry.entity.id) || entry.index > length || previous == Some(entry.index) {
            return Err(target_error("mutation.apply.invalid-add-target", "entity id and position must be unique and valid", vec!["entities".to_string(), entry.entity.id.to_string()]));
        }
        previous = Some(entry.index);
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_step_diff_unchecked(diff: &StepDiff, base: &StepSnapshot) -> StepSnapshot {
    let mut next = base.clone();
    if let Some(value) = &diff.file_description {
        next.header.file_description = value.clone();
    }
    if let Some(value) = &diff.file_name {
        next.header.file_name = value.clone();
    }
    if let Some(value) = &diff.file_schema {
        next.header.file_schema = value.clone();
    }
    if let Some(value) = &diff.entities {
        next.entities = value.apply(&next.entities);
    }
    next
}

impl MutationDiff<StepSnapshot> for StepDiff {
    fn apply(&self, base: &StepSnapshot) -> MutationApplyResult<StepSnapshot> {
        if let Some(diff) = &self.entities {
            validate_entities_diff(&base.entities, diff)?;
        }
        Ok(apply_step_diff_unchecked(self, base))
    }

    fn absorb(&mut self, other: Self) {
        if other.file_description.is_some() {
            self.file_description = other.file_description;
        }
        if other.file_name.is_some() {
            self.file_name = other.file_name;
        }
        if other.file_schema.is_some() {
            self.file_schema = other.file_schema;
        }
        self.entities = absorb_entities(self.entities.take(), other.entities);
    }
}

impl DiffAlgebra<StepSnapshot> for StepDiff {
    /// 🔁️ Diff-level undo, derived generically (correct by construction): the state delta from
    /// `self.apply(base)` back to `base` — `between` is the single source of truth for turning a
    /// state pair into a diff.
    fn inverse(&self, base: &StepSnapshot) -> Self {
        let mutated = apply_step_diff_unchecked(self, base);
        Self::between(&mutated, base)
    }

    fn between(base: &StepSnapshot, other: &StepSnapshot) -> Self {
        let entities_diff = StepEntitiesDiff::between(&base.entities, &other.entities);
        Self {
            file_description: (base.header.file_description != other.header.file_description).then(|| other.header.file_description.clone()),
            file_name: (base.header.file_name != other.header.file_name).then(|| other.header.file_name.clone()),
            file_schema: (base.header.file_schema != other.header.file_schema).then(|| other.header.file_schema.clone()),
            entities: (!entities_diff.is_empty()).then_some(entities_diff),
        }
    }

    fn is_empty(&self) -> bool {
        self.is_empty_diff()
    }
}

/// 🧩 Builds a set-snapshot diff — sparse field-by-field, never a full-replace slot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_snapshot(base: &StepSnapshot, next: &StepSnapshot) -> StepDiff {
    <StepDiff as DiffAlgebra<StepSnapshot>>::between(base, next)
}
//#endregion 🔖️Diff

//#region 🔖️HandcraftedDiffCodec
/// 🧪️ F6: **hand-rolled** `protocol::DiffCodec` for `StepDiff` — real `cargo check` confirms 3a
/// (`#[derive(dsl::DslDiff)]` fails: `StepEntitiesDiff: DslField` unsatisfied, cascading from
/// `StepEntityDiff.args: Option<StepArgsDiff>` -> `StepArgsDiff.modified/added` ->
/// `StepValue: DslField` unsatisfied — `StepValue` is a genuine data-carrying enum, no `DslField`
/// impl derivable for it, same root cause as `SvgNodeDiff`/`XmlNode`). No tri-state
/// `Option<Option<_>>` anywhere in this diff (3b does not apply here) — every `StepDiff` field is a
/// plain `Option<T>` ("weak value, whole-replaced"), so the grammar below needs no `[0]`/`[1,x]`
/// tri-state wrapper at the TOP level (absent token = unchanged is already unambiguous); the
/// wrapper IS still needed for genuinely nested `Option<T>` sub-fields (`StepEntityDiff.name`/
/// `.args`/`.complex`). Same primitive set + grammar conventions as `GifDiff`/`SvgDiff`'s
/// hand-rolled codecs (bracket-depth-aware split, hex for strings, `idx:payload`/`id:payload` for
/// collection-triple entries) — own copy per the recipe's specific-code mandate (see this file's
/// `IndexTransport` region doc comment for the same rationale), reused by `StepMutation`'s
/// `OpText`/`OpBinary` via `pub(crate)`.
//#region 🔖️Primitives











//#endregion 🔖️Primitives

//#region 🔖️BinaryPrimitives








//#endregion 🔖️BinaryPrimitives

//#region 🔖️ValueCodecs























//#endregion 🔖️ValueCodecs

//#region 🔖️ValueBinaryCodecs




















//#endregion 🔖️ValueBinaryCodecs

//#region 🔖️DiffValueCodecs








//#endregion 🔖️DiffValueCodecs

//#region 🔖️DiffValueBinaryCodecs








//#endregion 🔖️DiffValueBinaryCodecs

//#region 🔖️TopLevel




//#endregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec

//#region 🔖️DemoCases
/// 🧪️ P2-FG1: representative `StepDiff` cases — real `print_diff()`-conformance-law fodder
/// (`diff_grammar_conformance_law`) and `protocol_walk_law` fodder — the empty diff, a genuine
/// `between()` result exercising every top-level field plus all three `entities`/`args`
/// collection-triple flavors and `StepEntityDiff.complex`, and its reverse direction.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<StepDiff> {
    let a = crate::engine::demo_step_snapshot();
    let mut b = a.clone();
    b.header.file_schema.schemas.push("CONFIG_CONTROL_DESIGN".into());
    b.header.file_name.originating_system = "changed".into();
    if let Some(first) = b.entities.first_mut() {
        first.name = "RENAMED_POINT".into();
        first.args.push(StepValue::Aggregate(vec![StepValue::Integer(1), StepValue::Integer(2)]));
        first.complex.push(StepComplexType { name: "EXTRA_TYPE".into(), args: vec![StepValue::Real(1.5), StepValue::String("hi".into())] });
    }
    b.entities.push(StepEntity { id: 99, name: "ADDED_WITH_COMPLEX".into(), args: vec![StepValue::Unset], complex: vec![StepComplexType { name: "ANOTHER_TYPE".into(), args: vec![StepValue::Reference(42)] }] });
    vec![StepDiff::default(), StepDiff::between(&a, &b), StepDiff::between(&b, &a)]
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️handcrafted-diff-codec/🦀️.rs"]
mod handcrafted_diff_codec_tests;
//#endregion 🧪️Tests

//#region 🔁️Re-exports
pub use crate::schema::snapshot::StepComplexType;
pub use crate::schema::snapshot::StepEntity;
pub use crate::schema::snapshot::StepFileDescription;
pub use crate::schema::snapshot::StepFileName;
pub use crate::schema::snapshot::StepFileSchema;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::schema::snapshot::StepValue;
//#endregion 🔁️Re-exports
