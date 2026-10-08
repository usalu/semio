//! 🔺️ IfcDiff — handcrafted sparse diff, replacing the prior `IfcDiff{snapshot: Option<IfcSnapshot>}`
//! full-replace template (ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION).
//! Two independent collection triples: `entities` (id-keyed — ids never shift/rename, so absorb
//! needs no key-transport map, unlike zip's name-keyed entries) and, per modified entity, `args`
//! (index-keyed — positions DO shift on insert/remove, so its absorb needs the same rank/unrank
//! index-transport arithmetic as gif 89a's frame collection). HEADER fields are three sparse
//! scalar slots. `schema` is identity and never appears here.

/// 🧩 Ordered removed keys, modified values, and inserted items.
pub(crate) type IndexedDiffParts<D, T> = (Vec<usize>, Vec<(usize, D)>, Vec<(usize, T)>);

use std::collections::{BTreeMap, BTreeSet, HashSet};

use crate::schema::snapshot::{IfcComplexType, IfcEntity, IfcValue};
use crate::IfcSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};

//#region 🔖️IndexTransport
/// 📐️ Own local copy (per the recipe's "hand-duplicated, macro-free" convention — never shared
/// cross-artifact) of the rank/unrank arithmetic for index-keyed collection diffs, used by
/// `IfcArgsDiff::{absorb,inverse}`. `excluded_sorted` must be sorted ascending. See
/// `🧬️schema-design.md` §Absorb / gif 89a's diff module for the derivation this mirrors.
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

/// 🧮️ Sequential-coalesce absorb for an index-keyed collection triple, generic over item `T` and
/// its diff `D` — own local copy (see module doc).
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
fn rewind_indexed_collection<T: Clone, D: Clone>(removed: &[usize], modified: &[(usize, D)], added: &[(usize, T)], base_items: &[T], rewind_item: impl Fn(&D, &T) -> D) -> IndexedDiffParts<D, T> {
    let removed_sorted: Vec<usize> = removed.iter().copied().collect::<std::collections::BTreeSet<usize>>().into_iter().collect();
    let mut added_index_sorted: Vec<usize> = added.iter().map(|(i, _)| *i).collect();
    added_index_sorted.sort_unstable();

    let mut inv_removed: Vec<usize> = added.iter().map(|(i, _)| *i).collect();
    let mut inv_modified: Vec<(usize, D)> = Vec::new();
    for (base_index, d) in modified {
        if let Some(orig) = base_items.get(*base_index) {
            let after_index = transport_forward(*base_index, &removed_sorted, &added_index_sorted);
            inv_modified.push((after_index, rewind_item(d, orig)));
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
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct IfcArgModified {
    pub index: usize,
    pub value: IfcValue,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct IfcArgAdded {
    pub index: usize,
    pub value: IfcValue,
}

/// 🔺️ Index-keyed collection triple for one [`IfcEntity::args`] — positional per the EXPRESS
/// attribute order, so indices shift on insert/remove exactly like gif's frames.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct IfcArgsDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<IfcArgModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<IfcArgAdded>,
}

impl IfcArgsDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn inverse(&self, base_args: &[IfcValue]) -> Self {
        let (removed, modified, added) =
            rewind_indexed_collection(&self.removed, &self.modified.iter().map(|m| (m.index, m.value.clone())).collect::<Vec<_>>(), &self.added.iter().map(|a| (a.index, a.value.clone())).collect::<Vec<_>>(), base_args, |_d, item| item.clone());
        Self { removed, modified: modified.into_iter().map(|(index, value)| IfcArgModified { index, value }).collect(), added: added.into_iter().map(|(index, value)| IfcArgAdded { index, value }).collect() }
    }
}
//#endregion 🔖️ArgsDiff

//#region 🔖️EntityDiff
/// 🔺️ Sparse per-field diff for one [`IfcEntity`] — a strong entity. `id` is identity, never
/// diffed. `complex` (real IFC4 COMPLEX-instance extra type members) is a weak value-list —
/// whole-vec replaced, never sub-diffed, matching `complex`'s rarity/edge-case role.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct IfcEntityDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<IfcArgsDiff>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub complex: Option<Vec<IfcComplexType>>,
}

impl IfcEntityDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.name.is_none() && self.args.is_none() && self.complex.is_none()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn inverse(&self, base: &IfcEntity) -> Self {
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

//#region 🔖️EntitiesDiff
/// 📦️ One `entities.modified[]` entity — keyed by `id` (stable forever; unlike zip's name-keyed
/// entries, an IFC entity's `id` is never itself a mutable field, so no rename-transport map is
/// needed anywhere in this collection's absorb).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct IfcEntityModified {
    pub id: u64,
    pub diff: IfcEntityDiff,
}

/// 📦️ One `entities.added[]` entity — `index` is the FINAL position (apply semantics: ascending
/// `insert(min(index, len))`; see the recipe's `## Absorb`/`## Diff` apply-semantics note).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct IfcEntityAdded {
    pub index: usize,
    pub entity: IfcEntity,
}

/// 📦️ Sparse id-keyed `entities` triple.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct IfcEntitiesDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<u64>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<IfcEntityModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<IfcEntityAdded>,
}

impl IfcEntitiesDiff {
    /// 🔁️ The rows that undo this diff against `base`: added entities are removed, removed entities return at their base position, and modified
    /// entities invert field by field.
    pub fn inverse(&self, base: &[IfcEntity]) -> Self {
        let mut removed: Vec<(usize, u64)> = self.added.iter().map(|entry| (entry.index, entry.entity.id)).collect();
        removed.sort_unstable();
        let modified = self.modified.iter().filter_map(|entry| base.iter().find(|entity| entity.id == entry.id).map(|entity| IfcEntityModified { id: entry.id, diff: entry.diff.inverse(entity) })).collect();
        let mut added: Vec<IfcEntityAdded> = self.removed.iter().filter_map(|id| base.iter().position(|entity| entity.id == *id).map(|index| IfcEntityAdded { index, entity: base[index].clone() })).collect();
        added.sort_by_key(|entry| entry.index);
        Self { removed: removed.into_iter().map(|(_, id)| id).collect(), modified, added }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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

/// ➕️ Structural, total, base-free sequential-coalesce absorb of the `entities` triple (`##
/// Absorb` contract). Simpler than zip's name-keyed entries: `id` is never itself a diffable
/// field, so no rename-transport map is needed — only the `added[].index` final-position
/// bookkeeping (shifted by the count of `other`'s genuine, non-annihilating removals), mirroring
/// zip's own documented best-effort position adjustment for the same reason (this key kind's
/// diffs don't carry full base-position information for untouched survivors).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

    // 🧮️ `d1`'s adds carry positions in the INTERMEDIATE list `d2` was diffed against; `d2`'s own
    // adds occupy their positions in the FINAL list. Transporting the former through the latter
    // (the index-keyed sibling `absorb_indexed_collection` already does this via
    // `unrank_excluding`) is what keeps two inserts at one position from collapsing onto the same
    // final index — which `apply`'s own `invalid-add-target` guard rejects outright.
    let mut added2_index_sorted: Vec<usize> = d2.added.iter().map(|entry| entry.index).collect();
    added2_index_sorted.sort_unstable();
    for entry in &mut merged_added {
        entry.index = transport_forward(entry.index, &[], &added2_index_sorted);
    }

    for dm in d2.modified {
        if added_ids.contains(&dm.id) {
            if annihilated.contains(&dm.id) {
                continue; // modified-of-annihilated-add: moot.
            }
            if let Some(a) = merged_added.iter_mut().find(|a| a.entity.id == dm.id) {
                a.entity = apply_entity_diff_to_added(&dm.diff, &a.entity);
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
/// 🔺️ Diff for `stdio.ifc`. No `snapshot: Option<IfcSnapshot>` full-replace slot anywhere — the diff is sparse
/// field by field.
/// 🧪️ F6 CONFIRMED: `#[derive(dsl::)]` on this struct fails to compile (ticket
/// 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION, real `cargo check -p
/// semio-s-plugin-stdio --lib` output, verbatim):
/// ```text
/// error[E0277]: the trait bound `v4::subsets::any::schema::snapshot::component::IfcValue: DslField` is not satisfied
///    --> …/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:483:34
///     |
/// 483 |     pub file_description: Option<Vec<IfcValue>>,
///     |                                  ^^^^^^^^^^^^^ unsatisfied trait bound
/// error[E0277]: the trait bound `IfcEntitiesDiff: DslField` is not satisfied
///    --> …/🔺️diff/🦀️.rs:492:26   (pub entities: Option<IfcEntitiesDiff>)
/// ```
/// Root cause is §3a of `f6-recon-report.md`: [`IfcValue`] is a genuine data-carrying enum
/// (`Integer`/`Real`/`String`/`Enum`/`Reference`/`Aggregate`/`TypedValue`, all with fields) reachable
/// from `file_description`/`file_name`/`file_schema` directly and from `entities` transitively
/// (`IfcEntitiesDiff` -> `IfcEntityDiff` -> `IfcArgsDiff` -> `IfcArgModified.value: IfcValue`).
/// `DslField` has no impl for `IfcValue` (only `DslRecord`-derived structs and `DslScalar`-derived
/// UNIT-only enums implement `DslField`), so nothing downstream of it can derive either. `DiffBinary,DiffCodec,DiffText`
/// is hand-rolled below (`#[derive(dsl::)]` intentionally NOT present on this struct).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.ifc.diff")]
pub struct IfcDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub file_description: Option<Vec<IfcValue>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<Vec<IfcValue>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub file_schema: Option<Vec<IfcValue>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub entities: Option<IfcEntitiesDiff>,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn target_error(code: &'static str, message: &'static str, target: Vec<String>) -> MutationApplyError {
    MutationApplyError::new(code, message).at(target)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_args_diff(base_len: usize, diff: &IfcArgsDiff, prefix: &[String]) -> MutationApplyResult<()> {
    let mut removed = BTreeSet::new();
    for &index in &diff.removed {
        let target: Vec<String> = prefix.iter().cloned().chain(["args".to_string(), index.to_string()]).collect();
        if index >= base_len || !removed.insert(index) {
            return Err(target_error("mutation.apply.invalid-remove-index", "argument removal target must exist exactly once", target));
        }
    }
    let mut modified = BTreeSet::new();
    for entry in &diff.modified {
        let target: Vec<String> = prefix.iter().cloned().chain(["args".to_string(), entry.index.to_string()]).collect();
        if entry.index >= base_len || removed.contains(&entry.index) || !modified.insert(entry.index) {
            return Err(target_error("mutation.apply.invalid-modify-index", "argument modification target must exist exactly once and remain present", target));
        }
    }
    let mut additions: Vec<usize> = diff.added.iter().map(|entry| entry.index).collect();
    additions.sort_unstable();
    let mut previous = None;
    for (length, index) in (base_len - removed.len()..).zip(additions) {
        let target: Vec<String> = prefix.iter().cloned().chain(["args".to_string(), index.to_string()]).collect();
        if index > length || previous == Some(index) {
            return Err(target_error("mutation.apply.invalid-add-index", "argument addition target must be unique and within the evolving sequence", target));
        }
        previous = Some(index);
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_entities_diff(base: &[IfcEntity], diff: &IfcEntitiesDiff) -> MutationApplyResult<()> {
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
    let mut additions: Vec<&IfcEntityAdded> = diff.added.iter().collect();
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
    fn apply(&self, base: &IfcSnapshot, _capability: protocol::ApplyCapability) -> MutationApplyResult<IfcSnapshot> {
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
    /// 🔁️ Diff-level undo: every header field the diff sets returns to its base value, and the entity rows invert key by key.
    fn inverse(&self, base: &IfcSnapshot) -> Self {
        Self {
            file_description: self.file_description.as_ref().map(|_| base.header.file_description.clone()),
            file_name: self.file_name.as_ref().map(|_| base.header.file_name.clone()),
            file_schema: self.file_schema.as_ref().map(|_| base.header.file_schema.clone()),
            entities: self.entities.as_ref().map(|diff| diff.inverse(&base.entities)),
        }
    }

    fn is_empty(&self) -> bool {
        self.file_description.is_none() && self.file_name.is_none() && self.file_schema.is_none() && self.entities.as_ref().is_none_or(IfcEntitiesDiff::is_empty)
    }
}

//#region 🔖️MutationDiffBuilders
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_file_description(values: Vec<IfcValue>) -> IfcDiff {
    IfcDiff { file_description: Some(values), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_file_name(values: Vec<IfcValue>) -> IfcDiff {
    IfcDiff { file_name: Some(values), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_file_schema(values: Vec<IfcValue>) -> IfcDiff {
    IfcDiff { file_schema: Some(values), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_entity(index: usize, entity: IfcEntity) -> IfcDiff {
    IfcDiff { entities: Some(IfcEntitiesDiff { added: vec![IfcEntityAdded { index, entity }], ..Default::default() }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_entity(id: u64) -> IfcDiff {
    IfcDiff { entities: Some(IfcEntitiesDiff { removed: vec![id], ..Default::default() }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_entity_field(id: u64, field: IfcEntityDiff) -> IfcDiff {
    IfcDiff { entities: Some(IfcEntitiesDiff { modified: vec![IfcEntityModified { id, diff: field }], ..Default::default() }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_entity_name(id: u64, name: &str) -> IfcDiff {
    diff_entity_field(id, IfcEntityDiff { name: Some(name.to_string()), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_entity_arg(id: u64, index: usize, value: IfcValue) -> IfcDiff {
    diff_entity_field(id, IfcEntityDiff { args: Some(IfcArgsDiff { modified: vec![IfcArgModified { index, value }], ..Default::default() }), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_entity_arg(id: u64, index: usize, value: IfcValue) -> IfcDiff {
    diff_entity_field(id, IfcEntityDiff { args: Some(IfcArgsDiff { added: vec![IfcArgAdded { index, value }], ..Default::default() }), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_entity_arg(id: u64, index: usize) -> IfcDiff {
    diff_entity_field(id, IfcEntityDiff { args: Some(IfcArgsDiff { removed: vec![index], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️MutationDiffBuilders
//#endregion 🔖️Diff

//#region 🔖️DemoCases
/// 🧪️ Representative `IfcDiff` cases built declaratively — `diff_grammar_conformance_law` and `protocol_walk_law` fodder: the empty
/// diff, header lanes with an entity row triple (removed/modified args/added), and a header-only diff.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<IfcDiff> {
    let entities = IfcEntitiesDiff {
        removed: vec![2],
        modified: vec![IfcEntityModified { id: 1, diff: IfcEntityDiff { name: Some("IFCQUANTITYVOLUME".into()), args: Some(IfcArgsDiff { removed: vec![0], ..Default::default() }), complex: Some(Vec::new()) } }],
        added: vec![IfcEntityAdded { index: 1, entity: IfcEntity { id: 300, name: "IFCBUILDINGSTOREY".into(), args: vec![IfcValue::Aggregate(vec![IfcValue::Integer(1), IfcValue::Integer(2)])], complex: vec![] } }],
    };
    vec![
        IfcDiff::default(),
        IfcDiff { file_name: Some(vec![IfcValue::String("changed.ifc".into())]), entities: Some(entities), ..Default::default() },
        IfcDiff { file_description: Some(vec![IfcValue::TypedValue { name: "IFCLENGTHMEASURE".into(), items: vec![IfcValue::Real(3000.0)] }]), ..Default::default() },
    ]
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️handcrafted-diff-codec/🦀️.rs"]
mod handcrafted_diff_codec_tests;
//#endregion 🧪️Tests

/// 🧩️ The carried value of an entity an earlier diff added, once a later diff modified it (absorb).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_entity_diff_to_added(diff: &IfcEntityDiff, added: &IfcEntity) -> IfcEntity {
    diff.apply(added)
}
