//! 🔺️ IfcDiff — handcrafted sparse diff, replacing the prior `IfcDiff{snapshot: Option<IfcSnapshot>}`
//! full-replace template (ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION).
//! Two independent collection triples: `entities` (id-keyed — ids never shift/rename, so absorb
//! needs no key-transport map, unlike zip's name-keyed entries) and, per modified entity, `args`
//! (index-keyed — positions DO shift on insert/remove, so its absorb needs the same rank/unrank
//! index-transport arithmetic as gif 89a's frame collection). HEADER fields are three sparse
//! scalar slots. `schema` is identity and never appears here.

use std::collections::{BTreeMap, BTreeSet, HashSet};

use crate::artifacts::ifc::schema::snapshot::{IfcComplexType, IfcEntity, IfcValue};
use crate::artifacts::ifc::IfcSnapshot;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️IndexTransport
/// 📐️ Own local copy (per the recipe's "hand-duplicated, macro-free" convention — never shared
/// cross-artifact) of the rank/unrank arithmetic for index-keyed collection diffs, used by
/// `IfcArgsDiff::{absorb,inverse}`. `excluded_sorted` must be sorted ascending. See
/// `🧬️schema-design.md` §Absorb / gif 89a's diff module for the derivation this mirrors.
fn count_le(sorted: &[usize], x: usize) -> usize {
    sorted.partition_point(|&v| v <= x)
}
fn rank_excluding(pos: usize, excluded_sorted: &[usize]) -> usize {
    pos - count_le(excluded_sorted, pos)
}
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
fn transport_forward(index: usize, removed_sorted: &[usize], added_index_sorted: &[usize]) -> usize {
    unrank_excluding(rank_excluding(index, removed_sorted), added_index_sorted)
}

/// 🧮️ Sequential-coalesce absorb for an index-keyed collection triple, generic over item `T` and
/// its diff `D` — own local copy (see module doc).
#[allow(clippy::too_many_arguments)]
fn absorb_indexed_collection<T: Clone, D: Clone>(
    removed1: Vec<usize>,
    modified1: Vec<(usize, D)>,
    added1: Vec<(usize, T)>,
    removed2: Vec<usize>,
    modified2: Vec<(usize, D)>,
    added2: Vec<(usize, T)>,
    mut absorb_diff: impl FnMut(&mut D, D),
    apply_diff_to_item: impl Fn(&D, &T) -> T,
) -> (Vec<usize>, Vec<(usize, D)>, Vec<(usize, T)>) {
    let mut removed1_sorted = removed1.clone();
    removed1_sorted.sort_unstable();
    let mut added1_index_sorted: Vec<usize> = added1.iter().map(|(i, _)| *i).collect();
    added1_index_sorted.sort_unstable();
    let mut removed2_sorted = removed2.clone();
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
fn inverse_indexed_collection<T: Clone, D: Clone>(removed: &[usize], modified: &[(usize, D)], added: &[(usize, T)], base_items: &[T], diff_inverse: impl Fn(&D, &T) -> D) -> (Vec<usize>, Vec<(usize, D)>, Vec<(usize, T)>) {
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
//#endregion 🔖️IndexTransport

//#region 🔖️ArgsDiff
/// 🔺️ One `args.modified[]`/`added[]` entry — `IfcValue` is a weak/value leaf (per the recipe's
/// strong/weak split), so the "diff" for a changed argument IS the whole new value.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IfcArgModified {
    pub index: usize,
    pub value: IfcValue,
}
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IfcArgAdded {
    pub index: usize,
    pub value: IfcValue,
}

/// 🔺️ Index-keyed collection triple for one [`IfcEntity::args`] — positional per the EXPRESS
/// attribute order, so indices shift on insert/remove exactly like gif's frames.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IfcArgsDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<IfcArgModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<IfcArgAdded>,
}

impl IfcArgsDiff {
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }

    pub fn between(base: &[IfcValue], other: &[IfcValue]) -> Self {
        let min = base.len().min(other.len());
        let modified = (0..min).filter(|&i| base[i] != other[i]).map(|i| IfcArgModified { index: i, value: other[i].clone() }).collect();
        let removed: Vec<usize> = (min..base.len()).collect();
        let added: Vec<IfcArgAdded> = (min..other.len()).map(|i| IfcArgAdded { index: i, value: other[i].clone() }).collect();
        Self { removed, modified, added }
    }

    pub fn apply(&self, base: &[IfcValue]) -> Vec<IfcValue> {
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

    fn absorb(&mut self, other: Self) {
        let (removed, modified, added) = absorb_indexed_collection(
            std::mem::take(&mut self.removed),
            std::mem::take(&mut self.modified).into_iter().map(|m| (m.index, m.value)).collect(),
            std::mem::take(&mut self.added).into_iter().map(|a| (a.index, a.value)).collect(),
            other.removed,
            other.modified.into_iter().map(|m| (m.index, m.value)).collect(),
            other.added.into_iter().map(|a| (a.index, a.value)).collect(),
            |d, o| *d = o,
            |d, _item| d.clone(),
        );
        self.removed = removed;
        self.modified = modified.into_iter().map(|(index, value)| IfcArgModified { index, value }).collect();
        self.added = added.into_iter().map(|(index, value)| IfcArgAdded { index, value }).collect();
    }

    fn inverse(&self, base_args: &[IfcValue]) -> Self {
        let (removed, modified, added) =
            inverse_indexed_collection(&self.removed, &self.modified.iter().map(|m| (m.index, m.value.clone())).collect::<Vec<_>>(), &self.added.iter().map(|a| (a.index, a.value.clone())).collect::<Vec<_>>(), base_args, |_d, item| item.clone());
        Self { removed, modified: modified.into_iter().map(|(index, value)| IfcArgModified { index, value }).collect(), added: added.into_iter().map(|(index, value)| IfcArgAdded { index, value }).collect() }
    }
}
//#endregion 🔖️ArgsDiff

//#region 🔖️EntityDiff
/// 🔺️ Sparse per-field diff for one [`IfcEntity`] — a strong entity. `id` is identity, never
/// diffed. `complex` (real IFC4 COMPLEX-instance extra type members) is a weak value-list —
/// whole-vec replaced, never sub-diffed, matching `complex`'s rarity/edge-case role.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IfcEntityDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<IfcArgsDiff>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub complex: Option<Vec<IfcComplexType>>,
}

impl IfcEntityDiff {
    pub fn is_empty(&self) -> bool {
        self.name.is_none() && self.args.is_none() && self.complex.is_none()
    }

    pub fn between(base: &IfcEntity, other: &IfcEntity) -> Self {
        let args_diff = IfcArgsDiff::between(&base.args, &other.args);
        Self { name: (base.name != other.name).then(|| other.name.clone()), args: (!args_diff.is_empty()).then_some(args_diff), complex: (base.complex != other.complex).then(|| other.complex.clone()) }
    }

    pub fn apply(&self, base: &IfcEntity) -> IfcEntity {
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

    pub fn inverse(&self, base: &IfcEntity) -> Self {
        Self { name: self.name.as_ref().map(|_| base.name.clone()), args: self.args.as_ref().map(|d| d.inverse(&base.args)), complex: self.complex.as_ref().map(|_| base.complex.clone()) }
    }

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

//#region 🔖️EntitiesDiff
/// 📦️ One `entities.modified[]` entity — keyed by `id` (stable forever; unlike zip's name-keyed
/// entries, an IFC entity's `id` is never itself a mutable field, so no rename-transport map is
/// needed anywhere in this collection's absorb).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IfcEntityModified {
    pub id: u64,
    pub diff: IfcEntityDiff,
}

/// 📦️ One `entities.added[]` entity — `index` is the FINAL position (apply semantics: ascending
/// `insert(min(index, len))`; see the recipe's `## Absorb`/`## Diff` apply-semantics note).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IfcEntityAdded {
    pub index: usize,
    pub entity: IfcEntity,
}

/// 📦️ Sparse id-keyed `entities` triple.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IfcEntitiesDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<u64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<IfcEntityModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<IfcEntityAdded>,
}

impl IfcEntitiesDiff {
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }

    pub fn apply(&self, base: &[IfcEntity]) -> Vec<IfcEntity> {
        let mut entities: Vec<IfcEntity> = base.to_vec();
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
        let mut adds: Vec<&IfcEntityAdded> = self.added.iter().collect();
        adds.sort_by_key(|a| a.index);
        for a in adds {
            entities.insert(a.index, a.entity.clone());
        }
        entities
    }
}

/// 🧭️ State delta (compose `GetXDiff`): id-keyed matching — every base/other entity pair sharing
/// an `id` is compared field-by-field via [`IfcEntityDiff::between`]; ids present only in `base`
/// are `removed`, only in `other` are `added` at their final position.
fn entities_between(base: &[IfcEntity], other: &[IfcEntity]) -> Option<IfcEntitiesDiff> {
    if base == other {
        return None;
    }
    let base_ids: HashSet<u64> = base.iter().map(|e| e.id).collect();
    let other_ids: HashSet<u64> = other.iter().map(|e| e.id).collect();

    let removed: Vec<u64> = base.iter().filter(|e| !other_ids.contains(&e.id)).map(|e| e.id).collect();

    let mut modified = Vec::new();
    for be in base {
        if let Some(oe) = other.iter().find(|o| o.id == be.id) {
            let d = IfcEntityDiff::between(be, oe);
            if !d.is_empty() {
                modified.push(IfcEntityModified { id: be.id, diff: d });
            }
        }
    }

    let added: Vec<IfcEntityAdded> = other.iter().enumerate().filter(|(_, e)| !base_ids.contains(&e.id)).map(|(index, e)| IfcEntityAdded { index, entity: e.clone() }).collect();

    let d = IfcEntitiesDiff { removed, modified, added };
    if d.is_empty() {
        None
    } else {
        Some(d)
    }
}

/// ➕️ Structural, total, base-free sequential-coalesce absorb of the `entities` triple (`##
/// Absorb` contract). Simpler than zip's name-keyed entries: `id` is never itself a diffable
/// field, so no rename-transport map is needed — only the `added[].index` final-position
/// bookkeeping (shifted by the count of `other`'s genuine, non-annihilating removals), mirroring
/// zip's own documented best-effort position adjustment for the same reason (this key kind's
/// diffs don't carry full base-position information for untouched survivors).
fn absorb_entities(d1: Option<IfcEntitiesDiff>, d2: Option<IfcEntitiesDiff>) -> Option<IfcEntitiesDiff> {
    let (mut d1, d2) = match (d1, d2) {
        (None, None) => return None,
        (Some(d1), None) => return Some(d1),
        (None, Some(d2)) => return Some(d2),
        (Some(d1), Some(d2)) => (d1, d2),
    };

    let added_ids: HashSet<u64> = d1.added.iter().map(|a| a.entity.id).collect();
    let mut merged_removed: Vec<u64> = d1.removed;
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
            d1.modified.retain(|m| &m.id != id);
        }
    }

    let mut merged_modified: Vec<IfcEntityModified> = d1.modified;
    let mut merged_added: Vec<IfcEntityAdded> = d1
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
                merged_modified.push(IfcEntityModified { id: dm.id, diff: dm.diff.clone() });
            }
        }
    }

    merged_added.extend(d2.added);

    let merged = IfcEntitiesDiff { removed: merged_removed, modified: merged_modified, added: merged_added };
    if merged.is_empty() {
        None
    } else {
        Some(merged)
    }
}
//#endregion 🔖️EntitiesDiff

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.ifc`. No `snapshot: Option<IfcSnapshot>` full-replace slot anywhere — even
/// `SetSnapshot`'s diff is the sparse field-by-field [`IfcDiff::between`].
/// 🧪️ F6 CONFIRMED: `#[derive(dsl::DslDiff)]` on this struct fails to compile (ticket
/// 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION, real `cargo check -p
/// semio-s-plugin-stdio --lib` output, verbatim):
/// ```text
/// error[E0277]: the trait bound `v4::subsets::any::schema::snapshot::component::IfcValue: DslField` is not satisfied
///    --> …/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs:483:34
///     |
/// 483 |     pub file_description: Option<Vec<IfcValue>>,
///     |                                  ^^^^^^^^^^^^^ unsatisfied trait bound
/// error[E0277]: the trait bound `IfcEntitiesDiff: DslField` is not satisfied
///    --> …/🔺️diff/🦀️component.rs:492:26   (pub entities: Option<IfcEntitiesDiff>)
/// ```
/// Root cause is §3a of `f6-recon-report.md`: [`IfcValue`] is a genuine data-carrying enum
/// (`Integer`/`Real`/`String`/`Enum`/`Reference`/`Aggregate`/`TypedValue`, all with fields) reachable
/// from `file_description`/`file_name`/`file_schema` directly and from `entities` transitively
/// (`IfcEntitiesDiff` -> `IfcEntityDiff` -> `IfcArgsDiff` -> `IfcArgModified.value: IfcValue`).
/// `DslField` has no impl for `IfcValue` (only `DslRecord`-derived structs and `DslScalar`-derived
/// UNIT-only enums implement `DslField`), so nothing downstream of it can derive either. `DiffCodec`
/// is hand-rolled below (`#[derive(dsl::DslDiff)]` intentionally NOT present on this struct).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.ifc.diff")]
pub struct IfcDiff {
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_description: Option<Vec<IfcValue>>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<Vec<IfcValue>>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_schema: Option<Vec<IfcValue>>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entities: Option<IfcEntitiesDiff>,
}

fn target_error(code: &'static str, message: &'static str, target: Vec<String>) -> MutationApplyError {
    MutationApplyError::new(code, message).at(target)
}

fn validate_args_diff(base_len: usize, diff: &IfcArgsDiff, prefix: &[String]) -> MutationApplyResult<()> {
    let mut removed = BTreeSet::new();
    for &index in &diff.removed {
        let mut target = prefix.to_vec();
        target.extend(["args".to_string(), index.to_string()]);
        if index >= base_len || !removed.insert(index) {
            return Err(target_error("invalid-remove-index", "argument removal target must exist exactly once", target));
        }
    }
    let mut modified = BTreeSet::new();
    for entry in &diff.modified {
        let mut target = prefix.to_vec();
        target.extend(["args".to_string(), entry.index.to_string()]);
        if entry.index >= base_len || removed.contains(&entry.index) || !modified.insert(entry.index) {
            return Err(target_error("invalid-modify-index", "argument modification target must exist exactly once and remain present", target));
        }
    }
    let mut length = base_len - removed.len();
    let mut additions: Vec<usize> = diff.added.iter().map(|entry| entry.index).collect();
    additions.sort_unstable();
    let mut previous = None;
    for index in additions {
        let mut target = prefix.to_vec();
        target.extend(["args".to_string(), index.to_string()]);
        if index > length || previous == Some(index) {
            return Err(target_error("invalid-add-index", "argument addition target must be unique and within the evolving sequence", target));
        }
        previous = Some(index);
        length += 1;
    }
    Ok(())
}

fn validate_entities_diff(base: &[IfcEntity], diff: &IfcEntitiesDiff) -> MutationApplyResult<()> {
    let mut base_by_id = BTreeMap::new();
    for entity in base {
        if base_by_id.insert(entity.id, entity).is_some() {
            return Err(target_error("duplicate-base-target", "base entity ids must be unique", vec!["entities".to_string(), entity.id.to_string()]));
        }
    }
    let mut removed = BTreeSet::new();
    for &id in &diff.removed {
        if !base_by_id.contains_key(&id) || !removed.insert(id) {
            return Err(target_error("invalid-remove-target", "entity removal target must exist exactly once", vec!["entities".to_string(), id.to_string()]));
        }
    }
    let mut modified = BTreeSet::new();
    for entry in &diff.modified {
        let base_entity = base_by_id.get(&entry.id);
        if base_entity.is_none() || removed.contains(&entry.id) || !modified.insert(entry.id) {
            return Err(target_error("invalid-modify-target", "entity modification target must exist exactly once and remain present", vec!["entities".to_string(), entry.id.to_string()]));
        }
        if let Some(args) = &entry.diff.args {
            validate_args_diff(base_entity.map(|entity| entity.args.len()).unwrap_or_default(), args, &["entities".to_string(), entry.id.to_string()])?;
        }
    }
    let mut length = base.len() - removed.len();
    let mut additions: Vec<&IfcEntityAdded> = diff.added.iter().collect();
    additions.sort_by_key(|entry| entry.index);
    let mut added_ids = BTreeSet::new();
    let mut previous = None;
    for entry in additions {
        if base_by_id.contains_key(&entry.entity.id) || !added_ids.insert(entry.entity.id) || entry.index > length || previous == Some(entry.index) {
            return Err(target_error("invalid-add-target", "entity id and position must be unique and valid", vec!["entities".to_string(), entry.entity.id.to_string()]));
        }
        previous = Some(entry.index);
        length += 1;
    }
    Ok(())
}

fn apply_ifc_diff_unchecked(diff: &IfcDiff, base: &IfcSnapshot) -> IfcSnapshot {
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

impl MutationDiff<IfcSnapshot> for IfcDiff {
    fn apply(&self, base: &IfcSnapshot) -> MutationApplyResult<IfcSnapshot> {
        if let Some(diff) = &self.entities {
            validate_entities_diff(&base.entities, diff)?;
        }
        Ok(apply_ifc_diff_unchecked(self, base))
    }

    /// ➕️ Structural, total, base-free sequential-coalesce (`## Absorb` contract). Scalars: LWW.
    /// `entities`: see [`absorb_entities`].
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

impl DiffAlgebra<IfcSnapshot> for IfcDiff {
    /// 🔁️ Diff-level undo, derived generically (correct by construction, per zip's precedent):
    /// the state delta from `self.apply(base)` back to `base`.
    fn inverse(&self, base: &IfcSnapshot) -> Self {
        let mutated = apply_ifc_diff_unchecked(self, base);
        Self::between(&mutated, base)
    }

    /// 🧭️ State delta (compose `GetXDiff`).
    fn between(base: &IfcSnapshot, other: &IfcSnapshot) -> Self {
        Self {
            file_description: (base.header.file_description != other.header.file_description).then(|| other.header.file_description.clone()),
            file_name: (base.header.file_name != other.header.file_name).then(|| other.header.file_name.clone()),
            file_schema: (base.header.file_schema != other.header.file_schema).then(|| other.header.file_schema.clone()),
            entities: entities_between(&base.entities, &other.entities),
        }
    }

    fn is_empty(&self) -> bool {
        self.file_description.is_none() && self.file_name.is_none() && self.file_schema.is_none() && self.entities.as_ref().map_or(true, IfcEntitiesDiff::is_empty)
    }
}

//#region 🔖️MutationDiffBuilders
/// 🧩 `SetSnapshot`'s diff is the sparse field-by-field `between(base, next)` — no full-replace
/// slot exists on `IfcDiff` to short-circuit into.
pub fn diff_set_snapshot(base: &IfcSnapshot, next: &IfcSnapshot) -> IfcDiff {
    IfcDiff::between(base, next)
}
pub fn diff_set_file_description(values: Vec<IfcValue>) -> IfcDiff {
    IfcDiff { file_description: Some(values), ..Default::default() }
}
pub fn diff_set_file_name(values: Vec<IfcValue>) -> IfcDiff {
    IfcDiff { file_name: Some(values), ..Default::default() }
}
pub fn diff_set_file_schema(values: Vec<IfcValue>) -> IfcDiff {
    IfcDiff { file_schema: Some(values), ..Default::default() }
}
pub fn diff_insert_entity(index: usize, entity: IfcEntity) -> IfcDiff {
    IfcDiff { entities: Some(IfcEntitiesDiff { added: vec![IfcEntityAdded { index, entity }], ..Default::default() }), ..Default::default() }
}
pub fn diff_remove_entity(id: u64) -> IfcDiff {
    IfcDiff { entities: Some(IfcEntitiesDiff { removed: vec![id], ..Default::default() }), ..Default::default() }
}
fn diff_entity_field(id: u64, field: IfcEntityDiff) -> IfcDiff {
    IfcDiff { entities: Some(IfcEntitiesDiff { modified: vec![IfcEntityModified { id, diff: field }], ..Default::default() }), ..Default::default() }
}
pub fn diff_set_entity_name(id: u64, name: &str) -> IfcDiff {
    diff_entity_field(id, IfcEntityDiff { name: Some(name.to_string()), ..Default::default() })
}
pub fn diff_set_entity_arg(id: u64, index: usize, value: IfcValue) -> IfcDiff {
    diff_entity_field(id, IfcEntityDiff { args: Some(IfcArgsDiff { modified: vec![IfcArgModified { index, value }], ..Default::default() }), ..Default::default() })
}
pub fn diff_insert_entity_arg(id: u64, index: usize, value: IfcValue) -> IfcDiff {
    diff_entity_field(id, IfcEntityDiff { args: Some(IfcArgsDiff { added: vec![IfcArgAdded { index, value }], ..Default::default() }), ..Default::default() })
}
pub fn diff_remove_entity_arg(id: u64, index: usize) -> IfcDiff {
    diff_entity_field(id, IfcEntityDiff { args: Some(IfcArgsDiff { removed: vec![index], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️MutationDiffBuilders
//#endregion 🔖️Diff

//#region 🔖️HandcraftedDiffCodec
/// 🧪️ F6: **hand-rolled** `protocol::DiffCodec` for `IfcDiff` (see the compile-error citation on
/// [`IfcDiff`] itself). Same grammar style `GifDiff`/`SvgDiff`'s hand-rolled codecs use
/// (bracket-depth-aware split, hex for strings, `[0]`/`[1,x]` for `Option<T>`, single-uppercase-
/// letter tag prefix for data-carrying enum variants) — see `f6-recon-report.md` §5 for the
/// primitive rationale; this file re-derives its own copies since no shared "hand-roll helpers"
/// module exists yet (flagged there as a future extraction once ≥3 artifacts hand-roll, already the
/// case — not done here, out of this ticket's single-artifact ownership boundary).
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
pub(crate) fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
pub(crate) fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
fn parse_usize(s: &str) -> Result<usize, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}
fn parse_u64(s: &str) -> Result<u64, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
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
//#endregion 🔖️Primitives

//#region 🔖️BinaryPrimitives
/// 🧪️ P2-FG1: real varint/length-prefixed binary primitives (own local copy — never shared
/// cross-artifact, same convention the text primitives above use) backing the upgraded
/// `DiffCodec::encode_diff`/`decode_diff` frame below and `OpBinary::encode_op`/`decode_op`
/// (`../../🧬️mutations/🦀️component.rs`) — reuses `store::pack_rt::write_varint_u64`/
/// `store::ByteReader` rather than reinventing varint encode/decode. `pub(crate)` so the
/// mutations sibling can reuse these rather than duplicating them a second time in that file (same
/// intra-artifact-reuse split the TEXT codec primitives above already use).
pub(crate) fn write_str_bin(out: &mut Vec<u8>, s: &str) {
    store::pack_rt::write_varint_u64(out, s.len() as u64);
    out.extend_from_slice(s.as_bytes());
}
pub(crate) fn read_str_bin(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    String::from_utf8(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec()).map_err(|e| e.to_string())
}
pub(crate) fn write_option_bin<T>(out: &mut Vec<u8>, opt: &Option<T>, enc: impl FnOnce(&T, &mut Vec<u8>)) {
    match opt {
        None => out.push(0),
        Some(v) => {
            out.push(1);
            enc(v, out);
        }
    }
}
pub(crate) fn read_option_bin<T>(reader: &mut store::ByteReader<'_>, dec: impl FnOnce(&mut store::ByteReader<'_>) -> Result<T, String>) -> Result<Option<T>, String> {
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => Ok(None),
        1 => Ok(Some(dec(reader)?)),
        other => Err(format!("option binary: unknown tag {other}")),
    }
}
/// ➡️ Zigzag-encodes `value` into `store::pack_rt::write_varint_u64`'s unsigned domain — own local
/// copy (`store::pack_rt` only ships the unsigned writer; the read side's zigzag decode is already
/// built into `store::ByteReader::read_varint_i64`), same convention `zip`'s own diff module uses.
fn write_varint_i64(out: &mut Vec<u8>, value: i64) {
    let zigzag = ((value << 1) ^ (value >> 63)) as u64;
    store::pack_rt::write_varint_u64(out, zigzag);
}
//#endregion 🔖️BinaryPrimitives

//#region 🔖️IfcValueCodecs
/// 🔤️ `IfcValue` tag scheme, single uppercase letter + bracketed positional payload (payload-free
/// variants `Unset`/`Derived` are the bare letter, no brackets — never ambiguous with a bracketed
/// payload since every token boundary is either whitespace, `,`, or `;`, never a bare letter
/// followed directly by more letters): `U`=Unset, `D`=Derived, `I[n]`=Integer, `R[n]`=Real (Rust's
/// `Display`/`FromStr` for `f64` round-trip exactly, the shortest decimal that parses back), `S[hex]`
/// =String, `E[hex]`=Enum, `F[n]`=Reference, `A[v,v,...]`=Aggregate, `T[hex,[v,v,...]]`=TypedValue.
pub(crate) fn enc_ifc_value(v: &IfcValue) -> String {
    match v {
        IfcValue::Unset => "U".to_string(),
        IfcValue::Derived => "D".to_string(),
        IfcValue::Integer(i) => format!("I[{i}]"),
        IfcValue::Real(r) => format!("R[{r}]"),
        IfcValue::String(s) => format!("S[{}]", enc_str(s)),
        IfcValue::Enum(s) => format!("E[{}]", enc_str(s)),
        IfcValue::Reference(id) => format!("F[{id}]"),
        IfcValue::Aggregate(items) => format!("A[{}]", items.iter().map(enc_ifc_value).collect::<Vec<_>>().join(",")),
        IfcValue::TypedValue(name, items) => {
            format!("T[{},[{}]]", enc_str(name), items.iter().map(enc_ifc_value).collect::<Vec<_>>().join(","))
        }
    }
}
pub(crate) fn dec_ifc_value(s: &str) -> Result<IfcValue, String> {
    if s == "U" {
        return Ok(IfcValue::Unset);
    }
    if s == "D" {
        return Ok(IfcValue::Derived);
    }
    if s.is_empty() {
        return Err("ifc value: empty token".to_string());
    }
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "I" => Ok(IfcValue::Integer(inner.parse().map_err(|e: std::num::ParseIntError| e.to_string())?)),
        "R" => Ok(IfcValue::Real(inner.parse().map_err(|e: std::num::ParseFloatError| e.to_string())?)),
        "S" => Ok(IfcValue::String(dec_str(inner)?)),
        "E" => Ok(IfcValue::Enum(dec_str(inner)?)),
        "F" => Ok(IfcValue::Reference(inner.parse().map_err(|e: std::num::ParseIntError| e.to_string())?)),
        "A" => {
            let items = split_top_level(inner, ',').into_iter().filter(|s| !s.is_empty()).map(dec_ifc_value).collect::<Result<Vec<_>, String>>()?;
            Ok(IfcValue::Aggregate(items))
        }
        "T" => {
            let parts = split_top_level(inner, ',');
            let [name, items_s] = parts.as_slice() else { return Err(format!("typed value: expected 2 fields, got {}", parts.len())) };
            let items = split_top_level(strip_brackets(items_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_ifc_value).collect::<Result<Vec<_>, String>>()?;
            Ok(IfcValue::TypedValue(dec_str(name)?, items))
        }
        other => Err(format!("ifc value: unknown tag {other:?}")),
    }
}
pub(crate) fn enc_ifc_value_list(vs: &[IfcValue]) -> String {
    format!("[{}]", vs.iter().map(enc_ifc_value).collect::<Vec<_>>().join(","))
}
pub(crate) fn dec_ifc_value_list(s: &str) -> Result<Vec<IfcValue>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_ifc_value).collect()
}

//#region 🔖️IfcValueBinaryCodecs
/// 🧪️ P2-FG1: real recursive binary twin of [`enc_ifc_value`]/[`dec_ifc_value`] above — same 0-8
/// ordinal order as the text codec's `U`-`T` tag range, backing the upgraded `DiffCodec`
/// (below)/`OpBinary` (`../../🧬️mutations/🦀️component.rs`) frames. `Aggregate`/`TypedValue` recurse
/// into [`enc_ifc_value_list_bin`] exactly like their text-codec twins recurse into
/// `enc_ifc_value_list` — genuine field-by-field binary all the way down, no opaque tail needed
/// here (unlike the top-level `IfcDiff`/`IfcMutation` frames, `IfcValue` itself is fully flat/
/// spec-expressible per variant).
pub(crate) fn enc_ifc_value_bin(v: &IfcValue, out: &mut Vec<u8>) {
    match v {
        IfcValue::Unset => out.push(0),
        IfcValue::Derived => out.push(1),
        IfcValue::Integer(i) => {
            out.push(2);
            write_varint_i64(out, *i);
        }
        IfcValue::Real(r) => {
            out.push(3);
            out.extend_from_slice(&r.to_le_bytes());
        }
        IfcValue::String(s) => {
            out.push(4);
            write_str_bin(out, s);
        }
        IfcValue::Enum(s) => {
            out.push(5);
            write_str_bin(out, s);
        }
        IfcValue::Reference(id) => {
            out.push(6);
            store::pack_rt::write_varint_u64(out, *id);
        }
        IfcValue::Aggregate(items) => {
            out.push(7);
            enc_ifc_value_list_bin(items, out);
        }
        IfcValue::TypedValue(name, items) => {
            out.push(8);
            write_str_bin(out, name);
            enc_ifc_value_list_bin(items, out);
        }
    }
}
pub(crate) fn dec_ifc_value_bin(reader: &mut store::ByteReader<'_>) -> Result<IfcValue, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => Ok(IfcValue::Unset),
        1 => Ok(IfcValue::Derived),
        2 => Ok(IfcValue::Integer(reader.read_varint_i64().map_err(|e| e.to_string())?)),
        3 => Ok(IfcValue::Real(reader.read_f64_le().map_err(|e| e.to_string())?)),
        4 => Ok(IfcValue::String(read_str_bin(reader)?)),
        5 => Ok(IfcValue::Enum(read_str_bin(reader)?)),
        6 => Ok(IfcValue::Reference(reader.read_varint_u64().map_err(|e| e.to_string())?)),
        7 => Ok(IfcValue::Aggregate(dec_ifc_value_list_bin(reader)?)),
        8 => {
            let name = read_str_bin(reader)?;
            let items = dec_ifc_value_list_bin(reader)?;
            Ok(IfcValue::TypedValue(name, items))
        }
        other => Err(format!("ifc value binary: unknown tag {other}")),
    }
}
pub(crate) fn enc_ifc_value_list_bin(vs: &[IfcValue], out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, vs.len() as u64);
    for v in vs {
        enc_ifc_value_bin(v, out);
    }
}
pub(crate) fn dec_ifc_value_list_bin(reader: &mut store::ByteReader<'_>) -> Result<Vec<IfcValue>, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    (0..count).map(|_| dec_ifc_value_bin(reader)).collect()
}
//#endregion 🔖️IfcValueBinaryCodecs
//#endregion 🔖️IfcValueCodecs

//#region 🔖️EntityCodecs
fn enc_complex_type(c: &IfcComplexType) -> String {
    format!("[{},{}]", enc_str(&c.name), enc_ifc_value_list(&c.args))
}
fn dec_complex_type(s: &str) -> Result<IfcComplexType, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, args] = parts.as_slice() else { return Err(format!("complex type: expected 2 fields, got {}", parts.len())) };
    Ok(IfcComplexType { name: dec_str(name)?, args: dec_ifc_value_list(args)? })
}
fn enc_complex_list(list: &[IfcComplexType]) -> String {
    format!("[{}]", list.iter().map(enc_complex_type).collect::<Vec<_>>().join(","))
}
fn dec_complex_list(s: &str) -> Result<Vec<IfcComplexType>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_complex_type).collect()
}
/// 📦️ `[id,hexname,[args],[complex]]` — positional, mirrors [`IfcEntity`]'s own field order.
pub(crate) fn enc_entity(e: &IfcEntity) -> String {
    format!("[{},{},{},{}]", e.id, enc_str(&e.name), enc_ifc_value_list(&e.args), enc_complex_list(&e.complex))
}
pub(crate) fn dec_entity(s: &str) -> Result<IfcEntity, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, name, args, complex] = parts.as_slice() else { return Err(format!("entity: expected 4 fields, got {}", parts.len())) };
    Ok(IfcEntity { id: parse_u64(id)?, name: dec_str(name)?, args: dec_ifc_value_list(args)?, complex: dec_complex_list(complex)? })
}

//#region 🔖️EntityBinaryCodecs
/// 🧪️ P2-FG1: real binary twin of [`enc_complex_type`]/[`dec_complex_type`] and
/// [`enc_entity`]/[`dec_entity`] above — `id | name | args | complex` field-by-field, `args`/
/// `complex` each length-prefixed lists of the recursive `enc_ifc_value_bin`/`enc_complex_type_bin`
/// shape. `pub(crate)` (entity + list variants) so the mutations sibling can reuse these for its
/// own `InsertEntity`/`SetSnapshot` payloads, same intra-artifact-reuse split the TEXT codec uses.
fn enc_complex_type_bin(c: &IfcComplexType, out: &mut Vec<u8>) {
    write_str_bin(out, &c.name);
    enc_ifc_value_list_bin(&c.args, out);
}
fn dec_complex_type_bin(reader: &mut store::ByteReader<'_>) -> Result<IfcComplexType, String> {
    let name = read_str_bin(reader)?;
    let args = dec_ifc_value_list_bin(reader)?;
    Ok(IfcComplexType { name, args })
}
pub(crate) fn enc_complex_list_bin(list: &[IfcComplexType], out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, list.len() as u64);
    for c in list {
        enc_complex_type_bin(c, out);
    }
}
pub(crate) fn dec_complex_list_bin(reader: &mut store::ByteReader<'_>) -> Result<Vec<IfcComplexType>, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    (0..count).map(|_| dec_complex_type_bin(reader)).collect()
}
/// 📦️ `id | name | args | complex` — positional, mirrors [`enc_entity`]'s own field order.
pub(crate) fn enc_entity_bin(e: &IfcEntity, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, e.id);
    write_str_bin(out, &e.name);
    enc_ifc_value_list_bin(&e.args, out);
    enc_complex_list_bin(&e.complex, out);
}
pub(crate) fn dec_entity_bin(reader: &mut store::ByteReader<'_>) -> Result<IfcEntity, String> {
    let id = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let name = read_str_bin(reader)?;
    let args = dec_ifc_value_list_bin(reader)?;
    let complex = dec_complex_list_bin(reader)?;
    Ok(IfcEntity { id, name, args, complex })
}
pub(crate) fn enc_entity_list_bin(list: &[IfcEntity], out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, list.len() as u64);
    for e in list {
        enc_entity_bin(e, out);
    }
}
pub(crate) fn dec_entity_list_bin(reader: &mut store::ByteReader<'_>) -> Result<Vec<IfcEntity>, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    (0..count).map(|_| dec_entity_bin(reader)).collect()
}
//#endregion 🔖️EntityBinaryCodecs
//#endregion 🔖️EntityCodecs

//#region 🔖️DiffValueCodecs
fn enc_args_diff(d: &IfcArgsDiff) -> String {
    let removed = d.removed.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:{}", m.index, enc_ifc_value(&m.value))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}", a.index, enc_ifc_value(&a.value))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}
fn dec_args_diff(body: &str) -> Result<IfcArgsDiff, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("args diff: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_usize).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("arg modified: bad entry {entry:?}"))?;
            Ok(IfcArgModified { index: parse_usize(idx)?, value: dec_ifc_value(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("arg added: bad entry {entry:?}"))?;
            Ok(IfcArgAdded { index: parse_usize(idx)?, value: dec_ifc_value(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(IfcArgsDiff { removed, modified, added })
}

/// 🔖️ `[nameOpt,argsOpt,complexOpt]` — positional triple, each field individually `Option`-tagged.
fn enc_entity_diff(d: &IfcEntityDiff) -> String {
    format!(
        "[{},{},{}]",
        encode_option(&d.name, |v| enc_str(v)),
        match &d.args {
            Some(a) => format!("[1,{}]", enc_args_diff(a)),
            None => "[0]".to_string(),
        },
        match &d.complex {
            Some(c) => format!("[1,{}]", enc_complex_list(c)),
            None => "[0]".to_string(),
        },
    )
}
fn dec_entity_diff(s: &str) -> Result<IfcEntityDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, args, complex] = parts.as_slice() else { return Err(format!("entity diff: expected 3 fields, got {}", parts.len())) };
    let args = match split_top_level(strip_brackets(args)?, ',').as_slice() {
        ["0"] => None,
        [tag, rest @ ..] if *tag == "1" => Some(dec_args_diff(&rest.join(","))?),
        other => return Err(format!("entity diff args: bad shape {other:?}")),
    };
    let complex = match split_top_level(strip_brackets(complex)?, ',').as_slice() {
        ["0"] => None,
        [tag, rest @ ..] if *tag == "1" => Some(dec_complex_list(&rest.join(","))?),
        other => return Err(format!("entity diff complex: bad shape {other:?}")),
    };
    Ok(IfcEntityDiff { name: decode_option(name, dec_str)?, args, complex })
}

fn enc_entities_diff(d: &IfcEntitiesDiff) -> String {
    let removed = d.removed.iter().map(|id| id.to_string()).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:{}", m.id, enc_entity_diff(&m.diff))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}", a.index, enc_entity(&a.entity))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}
fn dec_entities_diff(body: &str) -> Result<IfcEntitiesDiff, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("entities diff: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_u64).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (id, rest) = entry.split_once(':').ok_or_else(|| format!("entity modified: bad entry {entry:?}"))?;
            Ok(IfcEntityModified { id: parse_u64(id)?, diff: dec_entity_diff(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("entity added: bad entry {entry:?}"))?;
            Ok(IfcEntityAdded { index: parse_usize(idx)?, entity: dec_entity(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(IfcEntitiesDiff { removed, modified, added })
}
//#endregion 🔖️DiffValueCodecs

//#region 🔖️DiffValueBinaryCodecs
/// 🧪️ P2-FG1: real binary twin of [`enc_args_diff`]/[`dec_args_diff`],
/// [`enc_entity_diff`]/[`dec_entity_diff`], [`enc_entities_diff`]/[`dec_entities_diff`] above — each
/// collection triple becomes three varint-counted lists (removed/modified/added), the recursive
/// per-entry `value`/`diff`/`entity` payload the flat/recursive `IfcValue`/`IfcEntityDiff`/
/// `IfcEntity` binary shape already defined above — backing the upgraded
/// `DiffCodec::encode_diff`/`decode_diff` below.
fn enc_args_diff_bin(d: &IfcArgsDiff, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, d.removed.len() as u64);
    for idx in &d.removed {
        store::pack_rt::write_varint_u64(out, *idx as u64);
    }
    store::pack_rt::write_varint_u64(out, d.modified.len() as u64);
    for m in &d.modified {
        store::pack_rt::write_varint_u64(out, m.index as u64);
        enc_ifc_value_bin(&m.value, out);
    }
    store::pack_rt::write_varint_u64(out, d.added.len() as u64);
    for a in &d.added {
        store::pack_rt::write_varint_u64(out, a.index as u64);
        enc_ifc_value_bin(&a.value, out);
    }
}
fn dec_args_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<IfcArgsDiff, String> {
    let removed_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut removed = Vec::with_capacity(removed_count as usize);
    for _ in 0..removed_count {
        removed.push(reader.read_varint_u64().map_err(|e| e.to_string())? as usize);
    }
    let modified_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(modified_count as usize);
    for _ in 0..modified_count {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let value = dec_ifc_value_bin(reader)?;
        modified.push(IfcArgModified { index, value });
    }
    let added_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(added_count as usize);
    for _ in 0..added_count {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let value = dec_ifc_value_bin(reader)?;
        added.push(IfcArgAdded { index, value });
    }
    Ok(IfcArgsDiff { removed, modified, added })
}

fn enc_entity_diff_bin(d: &IfcEntityDiff, out: &mut Vec<u8>) {
    write_option_bin(out, &d.name, |v, o| write_str_bin(o, v));
    write_option_bin(out, &d.args, |v, o| enc_args_diff_bin(v, o));
    write_option_bin(out, &d.complex, |v, o| enc_complex_list_bin(v, o));
}
fn dec_entity_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<IfcEntityDiff, String> {
    let name = read_option_bin(reader, read_str_bin)?;
    let args = read_option_bin(reader, dec_args_diff_bin)?;
    let complex = read_option_bin(reader, dec_complex_list_bin)?;
    Ok(IfcEntityDiff { name, args, complex })
}

fn enc_entities_diff_bin(d: &IfcEntitiesDiff, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, d.removed.len() as u64);
    for id in &d.removed {
        store::pack_rt::write_varint_u64(out, *id);
    }
    store::pack_rt::write_varint_u64(out, d.modified.len() as u64);
    for m in &d.modified {
        store::pack_rt::write_varint_u64(out, m.id);
        enc_entity_diff_bin(&m.diff, out);
    }
    store::pack_rt::write_varint_u64(out, d.added.len() as u64);
    for a in &d.added {
        store::pack_rt::write_varint_u64(out, a.index as u64);
        enc_entity_bin(&a.entity, out);
    }
}
fn dec_entities_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<IfcEntitiesDiff, String> {
    let removed_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut removed = Vec::with_capacity(removed_count as usize);
    for _ in 0..removed_count {
        removed.push(reader.read_varint_u64().map_err(|e| e.to_string())?);
    }
    let modified_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(modified_count as usize);
    for _ in 0..modified_count {
        let id = reader.read_varint_u64().map_err(|e| e.to_string())?;
        let diff = dec_entity_diff_bin(reader)?;
        modified.push(IfcEntityModified { id, diff });
    }
    let added_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(added_count as usize);
    for _ in 0..added_count {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let entity = dec_entity_bin(reader)?;
        added.push(IfcEntityAdded { index, entity });
    }
    Ok(IfcEntitiesDiff { removed, modified, added })
}
//#endregion 🔖️DiffValueBinaryCodecs

//#region 🔖️TopLevel
fn print_ifc_diff(d: &IfcDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = &d.file_description {
        tokens.push(format!("file-description={}", enc_ifc_value_list(v)));
    }
    if let Some(v) = &d.file_name {
        tokens.push(format!("file-name={}", enc_ifc_value_list(v)));
    }
    if let Some(v) = &d.file_schema {
        tokens.push(format!("file-schema={}", enc_ifc_value_list(v)));
    }
    if let Some(v) = &d.entities {
        tokens.push(format!("entities={}", enc_entities_diff(v)));
    }
    tokens.join(" ")
}
fn parse_ifc_diff(line: &str) -> Result<IfcDiff, String> {
    let mut d = IfcDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("file-description=") {
            d.file_description = Some(dec_ifc_value_list(rest)?);
        } else if let Some(rest) = token.strip_prefix("file-name=") {
            d.file_name = Some(dec_ifc_value_list(rest)?);
        } else if let Some(rest) = token.strip_prefix("file-schema=") {
            d.file_schema = Some(dec_ifc_value_list(rest)?);
        } else if let Some(rest) = token.strip_prefix("entities=") {
            d.entities = Some(dec_entities_diff(rest)?);
        } else {
            return Err(format!("ifc diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl protocol::DiffCodec for IfcDiff {
    fn print_diff(&self) -> String {
        print_ifc_diff(self)
    }
    fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        parse_ifc_diff(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    /// 🧪️ P2-FG1: REAL binary frame (`format u8 | flags u8 | field payloads...`), matching
    /// `../💾️binary/📡️component.protocol.semio`'s `header fixed 2` + `chain payload bytes` shape —
    /// upgraded from F6's `print_diff().into_bytes()` text-as-binary shortcut (`IfcDiff` was one of
    /// 4 of stdio's 7 FG1 standards still on that shortcut per this wave's own P2-FG1 census).
    /// `flags` bit0..3 = `file_description`/`file_name`/`file_schema`/`entities` presence (same
    /// order the text grammar's token list uses); each present field's payload follows immediately
    /// in that order, real field-by-field binary all the way down for `file_description`/
    /// `file_name`/`file_schema` (flat `Vec<IfcValue>`) and for `entities`' own flat `id`/`name`/
    /// `index` fields — only the innermost recursive `IfcValue::Aggregate`/`TypedValue` payload
    /// bottoms out via `enc_ifc_value_bin`'s own recursive call (not an opaque tail: `IfcValue` is
    /// fully spec-expressible per variant, see that fn's own doc comment).
    fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let flags: u8 = (self.file_description.is_some() as u8) | ((self.file_name.is_some() as u8) << 1) | ((self.file_schema.is_some() as u8) << 2) | ((self.entities.is_some() as u8) << 3);
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, flags];
        if let Some(v) = &self.file_description {
            enc_ifc_value_list_bin(v, &mut out);
        }
        if let Some(v) = &self.file_name {
            enc_ifc_value_list_bin(v, &mut out);
        }
        if let Some(v) = &self.file_schema {
            enc_ifc_value_list_bin(v, &mut out);
        }
        if let Some(v) = &self.entities {
            enc_entities_diff_bin(v, &mut out);
        }
        Ok(out)
    }
    fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let _format = reader.read_u8().map_err(|e| malformed("diff format", 0, e.to_string()))?;
        let flags = reader.read_u8().map_err(|e| malformed("diff flags", 1, e.to_string()))?;
        let file_description = if flags & 1 != 0 { Some(dec_ifc_value_list_bin(&mut reader).map_err(|e| malformed("diff file_description", reader.position(), e))?) } else { None };
        let file_name = if flags & 2 != 0 { Some(dec_ifc_value_list_bin(&mut reader).map_err(|e| malformed("diff file_name", reader.position(), e))?) } else { None };
        let file_schema = if flags & 4 != 0 { Some(dec_ifc_value_list_bin(&mut reader).map_err(|e| malformed("diff file_schema", reader.position(), e))?) } else { None };
        let entities = if flags & 8 != 0 { Some(dec_entities_diff_bin(&mut reader).map_err(|e| malformed("diff entities", reader.position(), e))?) } else { None };
        Ok(IfcDiff { file_description, file_name, file_schema, entities })
    }
}
//#endregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec

//#region 🔖️DemoCases
/// 🧪️ P2-FG1: representative `IfcDiff` cases — real `print_diff()`-conformance-law fodder
/// (`diff_grammar_conformance_law`) and `protocol_walk_law` fodder — the empty diff, a genuine
/// `between()` result exercising every top-level field plus all three `entities`/`args`
/// collection-triple flavors and `IfcEntityDiff.complex`, and its reverse direction.
pub(crate) fn demo_diff_cases() -> Vec<IfcDiff> {
    let a = crate::artifacts::ifc::engine::demo_ifc_snapshot();
    let mut b = a.clone();
    b.header.file_name = vec![IfcValue::String("changed.ifc".into())];
    if let Some(first) = b.entities.first_mut() {
        first.name = "IFCQUANTITYVOLUME".into();
        first.args = vec![IfcValue::TypedValue("IFCLENGTHMEASURE".into(), vec![IfcValue::Real(3000.0)])];
        first.complex = vec![];
    }
    b.entities.push(IfcEntity { id: 300, name: "IFCBUILDINGSTOREY".into(), args: vec![IfcValue::Aggregate(vec![IfcValue::Integer(1), IfcValue::Integer(2)])], complex: vec![] });
    vec![IfcDiff::default(), IfcDiff::between(&a, &b), IfcDiff::between(&b, &a)]
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
mod handcrafted_diff_codec_tests {
    use super::*;
    use protocol::DiffCodec;

    fn entity(id: u64, name: &str, args: Vec<IfcValue>) -> IfcEntity {
        IfcEntity { id, name: name.into(), args, complex: vec![] }
    }

    fn base() -> IfcSnapshot {
        IfcSnapshot {
            schema: "stdio.ifc".into(),
            header: crate::artifacts::ifc::schema::snapshot::IfcHeader {
                file_description: vec![IfcValue::String("d".into())],
                file_name: vec![IfcValue::String("n".into())],
                file_schema: vec![IfcValue::Aggregate(vec![IfcValue::String("IFC4".into())])],
            },
            entities: vec![
                entity(1, "IFCPROJECT", vec![IfcValue::String("gid".into()), IfcValue::Reference(2), IfcValue::Unset, IfcValue::Derived]),
                IfcEntity {
                    id: 2,
                    name: "IFCQUANTITYAREA".into(),
                    args: vec![IfcValue::Real(10.5), IfcValue::Integer(-3), IfcValue::Enum("EDGE".into())],
                    complex: vec![IfcComplexType { name: "IFCPHYSICALSIMPLEQUANTITY".into(), args: vec![IfcValue::Unset] }],
                },
            ],
        }
    }

    /// 🧪️ F6: `DiffCodec` round-trip laws over the hand-rolled `IfcDiff` grammar — exercises every
    /// `IfcValue` variant (incl. `Aggregate`/`TypedValue` recursion), the `entities` collection
    /// triple, and the nested per-entity `args` collection triple + `complex` weak-list replace.
    #[test]
    fn diff_codec_text_binary_roundtrip_law() {
        let a = base();
        let mut b = base();
        b.header.file_name = vec![IfcValue::String("changed".into())];
        b.entities.remove(0); // remove id 1
        b.entities[0].name = "IFCQUANTITYVOLUME".into(); // modify id 2
        b.entities[0].args = vec![IfcValue::TypedValue("IFCLENGTHMEASURE".into(), vec![IfcValue::Real(3000.0)])];
        b.entities[0].complex = vec![];
        b.entities.push(entity(300, "IFCBUILDINGSTOREY", vec![IfcValue::Aggregate(vec![IfcValue::Integer(1), IfcValue::Integer(2)])]));

        let cases = vec![IfcDiff::default(), IfcDiff::between(&a, &b), IfcDiff::between(&b, &a), IfcDiff::between(&a, &a)];
        for d in cases {
            let printed = d.print_diff();
            assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
            let parsed = IfcDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
            assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

            let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
            let decoded = IfcDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
            assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
        }
    }
}
//#endregion 🧪️Tests
