//! 🔺️ StepDiff — handcrafted sparse diff. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: replaces the old
//! `StepDiff{snapshot: Option<StepSnapshot>}` full-replace template with a real per-field patch —
//! three scalar HEADER-record slots (`file_description`/`file_name`/`file_schema`, each a weak
//! value struct per the recipe's strong/weak split, whole-value replaced) plus an id-keyed
//! `entities` triple. `entities.modified[].diff.args` is a SEPARATE index-keyed triple (Part-21
//! entity argument lists are positional, not named) whose items (`StepValue`) are themselves weak
//! — "the diff IS the whole new value", same pattern as gif's `GifCommentsDiff`/`String`.

use std::collections::HashSet;

use crate::artifacts::step::schema::snapshot::{StepComplexType, StepEntity, StepFileDescription, StepFileName, StepFileSchema, StepValue};
use crate::artifacts::step::StepSnapshot;
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️IndexTransport
/// 📐️ Shared rank/unrank arithmetic for index-keyed collection diffs (`between`/`absorb`/
/// `inverse`) — see `🧬️schema-design.md` §Absorb. `excluded_sorted` must be sorted ascending.
/// Own copy (not imported from gif) per the recipe's specific-code mandate — small and
/// self-contained enough that duplicating it per artifact is the honest choice, not a defect.
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
//#endregion 🔖️IndexTransport

//#region 🔖️GenericIndexedCollectionAlgebra
/// 🧮️ Sequential-coalesce absorb for an index-keyed collection triple, generic over item `T` and
/// per-item diff `D` (here always `T == D == StepValue`, a weak collection whose "diff" is the
/// whole new value). Canonical correctness verified against the plan's 3 mandated cases in this
/// module's tests.
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

    let mut modified_map: std::collections::BTreeMap<usize, D> = modified1.into_iter().collect();
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
fn inverse_indexed_collection<T: Clone, D: Clone>(
    removed: &[usize],
    modified: &[(usize, D)],
    added: &[(usize, T)],
    base_items: &[T],
    diff_inverse: impl Fn(&D, &T) -> D,
) -> (Vec<usize>, Vec<(usize, D)>, Vec<(usize, T)>) {
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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StepArgModified {
    pub index: usize,
    pub value: StepValue,
}
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StepArgAdded {
    pub index: usize,
    pub value: StepValue,
}

/// 🔺️ Index-keyed collection triple for `StepEntity::args` — Part-21 entity argument lists are
/// positional, never named.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StepArgsDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<StepArgModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<StepArgAdded>,
}

impl StepArgsDiff {
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }

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

    pub fn apply(&self, base: &[StepValue]) -> Vec<StepValue> {
        let mut next: Vec<Option<StepValue>> = base.iter().cloned().map(Some).collect();
        for m in &self.modified {
            if let Some(slot) = next.get_mut(m.index) {
                *slot = Some(m.value.clone());
            }
        }
        let mut removed_sorted = self.removed.clone();
        removed_sorted.sort_unstable();
        removed_sorted.reverse();
        for &r in &removed_sorted {
            if r < next.len() {
                next.remove(r);
            }
        }
        let mut out: Vec<StepValue> = next.into_iter().flatten().collect();
        let mut added_sorted = self.added.clone();
        added_sorted.sort_by_key(|a| a.index);
        for a in added_sorted {
            let at = a.index.min(out.len());
            out.insert(at, a.value);
        }
        out
    }

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

    fn inverse(&self, base_args: &[StepValue]) -> Self {
        let (removed, modified, added) = inverse_indexed_collection(
            &self.removed,
            &self.modified.iter().map(|m| (m.index, m.value.clone())).collect::<Vec<_>>(),
            &self.added.iter().map(|a| (a.index, a.value.clone())).collect::<Vec<_>>(),
            base_args,
            |_d: &StepValue, item: &StepValue| item.clone(),
        );
        Self {
            removed,
            modified: modified.into_iter().map(|(index, value)| StepArgModified { index, value }).collect(),
            added: added.into_iter().map(|(index, value)| StepArgAdded { index, value }).collect(),
        }
    }
}
//#endregion 🔖️ArgsDiff

//#region 🔖️EntityDiff
/// 🔺️ Sparse per-field diff for one [`StepEntity`] — a strong entity, per the recipe. `complex`
/// (the rare multi-type-instance extension) is a weak value list, whole-vec replaced.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StepEntityDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<StepArgsDiff>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub complex: Option<Vec<StepComplexType>>,
}

impl StepEntityDiff {
    pub fn is_empty(&self) -> bool {
        self.name.is_none() && self.args.is_none() && self.complex.is_none()
    }

    pub fn between(base: &StepEntity, other: &StepEntity) -> Self {
        let args_diff = StepArgsDiff::between(&base.args, &other.args);
        Self {
            name: (base.name != other.name).then(|| other.name.clone()),
            args: (!args_diff.is_empty()).then_some(args_diff),
            complex: (base.complex != other.complex).then(|| other.complex.clone()),
        }
    }

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

    pub fn inverse(&self, base: &StepEntity) -> Self {
        Self {
            name: self.name.as_ref().map(|_| base.name.clone()),
            args: self.args.as_ref().map(|d| d.inverse(&base.args)),
            complex: self.complex.as_ref().map(|_| base.complex.clone()),
        }
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

//#region 🔖️EntitiesTriple
/// 📦️ One `entities.modified[]` entity — `id` is stable Part-21 instance-number identity (never
/// renumbered by a mutation in this recipe; unlike zip's names, no rename tracking is needed).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StepEntityModified {
    pub id: u64,
    pub diff: StepEntityDiff,
}

/// 📦️ One `entities.added[]` entity — `index` is the entity's position in the FINAL sequence.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StepEntityAdded {
    pub index: usize,
    pub entity: StepEntity,
}

/// 📦️ Sparse id-keyed `entities` triple.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StepEntitiesDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<u64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<StepEntityModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<StepEntityAdded>,
}

impl StepEntitiesDiff {
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }

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

        let added: Vec<StepEntityAdded> = other
            .iter()
            .enumerate()
            .filter(|(_, e)| !base_ids.contains(&e.id))
            .map(|(index, e)| StepEntityAdded { index, entity: e.clone() })
            .collect();

        Self { removed, modified, added }
    }

    pub fn apply(&self, base: &[StepEntity]) -> Vec<StepEntity> {
        let mut entities: Vec<StepEntity> = base.to_vec();
        if !self.removed.is_empty() {
            let removed: HashSet<u64> = self.removed.iter().copied().collect();
            entities.retain(|e| !removed.contains(&e.id));
        }
        for m in &self.modified {
            if let Some(e) = entities.iter_mut().find(|e| e.id == m.id) {
                *e = m.diff.apply(e);
            }
        }
        let mut adds: Vec<&StepEntityAdded> = self.added.iter().collect();
        adds.sort_by_key(|a| a.index);
        for a in adds {
            let at = a.index.min(entities.len());
            entities.insert(at, a.entity.clone());
        }
        entities
    }

    /// ➕️ In-place absorb on a bare (non-`Option`-wrapped) triple — delegates to the
    /// `Option`-wrapped free function `absorb_entities` (the real logic; kept there so
    /// `StepDiff::absorb`'s `self.entities.take()`/`other.entities` shape needs no unwrapping).
    fn absorb(&mut self, other: Self) {
        let merged = absorb_entities(Some(std::mem::take(self)), Some(other));
        *self = merged.unwrap_or_default();
    }
}

/// ➕️ Free-function core of `entities` absorb — id-keyed, no rename transport needed (unlike
/// zip's names, a `#123` instance number is never reassigned by this recipe's mutation
/// vocabulary). Structural, total, base-free, sequential-coalesce, same shape as zip's
/// `absorb_entries` minus the rename machinery.
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
    if merged.is_empty() { None } else { Some(merged) }
}
//#endregion 🔖️EntitiesTriple

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.step`. No `snapshot: Option<StepSnapshot>` full-replace slot anywhere.
/// `schema` is an identity field and never appears here. The three HEADER records are scalar
/// weak-value slots (never sub-diffed) per the recipe.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.step.diff")]
pub struct StepDiff {
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_description: Option<StepFileDescription>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<StepFileName>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_schema: Option<StepFileSchema>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entities: Option<StepEntitiesDiff>,
}

impl StepDiff {
    pub fn is_empty_diff(&self) -> bool {
        self.file_description.is_none()
            && self.file_name.is_none()
            && self.file_schema.is_none()
            && self.entities.as_ref().map(StepEntitiesDiff::is_empty).unwrap_or(true)
    }
}

impl MutationDiff<StepSnapshot> for StepDiff {
    fn apply(&self, base: &StepSnapshot) -> StepSnapshot {
        let mut next = base.clone();
        if let Some(v) = &self.file_description {
            next.header.file_description = v.clone();
        }
        if let Some(v) = &self.file_name {
            next.header.file_name = v.clone();
        }
        if let Some(v) = &self.file_schema {
            next.header.file_schema = v.clone();
        }
        if let Some(d) = &self.entities {
            next.entities = d.apply(&next.entities);
        }
        next
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
        let mutated = self.apply(base);
        Self::between(&mutated, base)
    }

    fn between(base: &StepSnapshot, other: &StepSnapshot) -> Self {
        let entities_diff = StepEntitiesDiff::between(&base.entities, &other.entities);
        Self {
            file_description: (base.header.file_description != other.header.file_description)
                .then(|| other.header.file_description.clone()),
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
pub(crate) fn parse_usize(s: &str) -> Result<usize, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}
pub(crate) fn parse_u64(s: &str) -> Result<u64, String> {
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

//#region 🔖️ValueCodecs
/// 🔤️ `StepValue` — single-uppercase-letter tag prefix like `enc_xml_node`, one per variant: `U`
/// Unset, `D` Derived, `I` Integer, `R` Real, `S` String, `E` Enum, `F` reFerence (`R` taken by
/// Real), `A` Aggregate (recursive list), `T` TypedValue (recursive, name + one wrapped value).
pub(crate) fn enc_value(v: &StepValue) -> String {
    match v {
        StepValue::Unset => "U[]".to_string(),
        StepValue::Derived => "D[]".to_string(),
        StepValue::Integer(i) => format!("I[{i}]"),
        StepValue::Real(r) => format!("R[{r}]"),
        StepValue::String(s) => format!("S[{}]", enc_str(s)),
        StepValue::Enum(s) => format!("E[{}]", enc_str(s)),
        StepValue::Reference(id) => format!("F[{id}]"),
        StepValue::Aggregate(items) => format!("A[{}]", items.iter().map(enc_value).collect::<Vec<_>>().join(",")),
        StepValue::TypedValue { type_name, value } => format!("T[{},{}]", enc_str(type_name), enc_value(value)),
    }
}
pub(crate) fn dec_value(s: &str) -> Result<StepValue, String> {
    if s.len() < 3 {
        return Err(format!("step value: too short {s:?}"));
    }
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "U" => Ok(StepValue::Unset),
        "D" => Ok(StepValue::Derived),
        "I" => Ok(StepValue::Integer(inner.parse().map_err(|e: std::num::ParseIntError| e.to_string())?)),
        "R" => Ok(StepValue::Real(inner.parse().map_err(|e: std::num::ParseFloatError| e.to_string())?)),
        "S" => Ok(StepValue::String(dec_str(inner)?)),
        "E" => Ok(StepValue::Enum(dec_str(inner)?)),
        "F" => Ok(StepValue::Reference(inner.parse().map_err(|e: std::num::ParseIntError| e.to_string())?)),
        "A" => {
            let items = split_top_level(inner, ',').into_iter().filter(|s| !s.is_empty()).map(dec_value).collect::<Result<Vec<_>, String>>()?;
            Ok(StepValue::Aggregate(items))
        }
        "T" => {
            let parts = split_top_level(inner, ',');
            let [type_name, value] = parts.as_slice() else { return Err(format!("typed value: expected 2 fields, got {}", parts.len())) };
            Ok(StepValue::TypedValue { type_name: dec_str(type_name)?, value: Box::new(dec_value(value)?) })
        }
        other => Err(format!("step value: unknown tag {other:?}")),
    }
}

fn enc_complex(c: &StepComplexType) -> String {
    format!("[{},[{}]]", enc_str(&c.name), c.args.iter().map(enc_value).collect::<Vec<_>>().join(","))
}
fn dec_complex(s: &str) -> Result<StepComplexType, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, args] = parts.as_slice() else { return Err(format!("complex type: expected 2 fields, got {}", parts.len())) };
    let args = split_top_level(strip_brackets(args)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_value).collect::<Result<Vec<_>, String>>()?;
    Ok(StepComplexType { name: dec_str(name)?, args })
}

pub(crate) fn enc_entity(e: &StepEntity) -> String {
    format!(
        "[{},{},[{}],[{}]]",
        e.id,
        enc_str(&e.name),
        e.args.iter().map(enc_value).collect::<Vec<_>>().join(","),
        e.complex.iter().map(enc_complex).collect::<Vec<_>>().join(","),
    )
}
pub(crate) fn dec_entity(s: &str) -> Result<StepEntity, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, name, args, complex] = parts.as_slice() else { return Err(format!("entity: expected 4 fields, got {}", parts.len())) };
    Ok(StepEntity {
        id: parse_u64(id)?,
        name: dec_str(name)?,
        args: split_top_level(strip_brackets(args)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_value).collect::<Result<Vec<_>, String>>()?,
        complex: split_top_level(strip_brackets(complex)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_complex).collect::<Result<Vec<_>, String>>()?,
    })
}

pub(crate) fn enc_file_description(d: &StepFileDescription) -> String {
    format!("[[{}],{}]", d.description.iter().map(|s| enc_str(s)).collect::<Vec<_>>().join(","), enc_str(&d.implementation_level))
}
pub(crate) fn dec_file_description(s: &str) -> Result<StepFileDescription, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [description, implementation_level] = parts.as_slice() else { return Err(format!("file description: expected 2 fields, got {}", parts.len())) };
    Ok(StepFileDescription {
        description: split_top_level(strip_brackets(description)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_str).collect::<Result<Vec<_>, String>>()?,
        implementation_level: dec_str(implementation_level)?,
    })
}

pub(crate) fn enc_file_name(f: &StepFileName) -> String {
    format!(
        "[{},{},[{}],[{}],{},{},{}]",
        enc_str(&f.name),
        enc_str(&f.timestamp),
        f.author.iter().map(|s| enc_str(s)).collect::<Vec<_>>().join(","),
        f.organization.iter().map(|s| enc_str(s)).collect::<Vec<_>>().join(","),
        enc_str(&f.preprocessor_version),
        enc_str(&f.originating_system),
        enc_str(&f.authorization),
    )
}
pub(crate) fn dec_file_name(s: &str) -> Result<StepFileName, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, timestamp, author, organization, preprocessor_version, originating_system, authorization] = parts.as_slice() else {
        return Err(format!("file name: expected 7 fields, got {}", parts.len()));
    };
    Ok(StepFileName {
        name: dec_str(name)?,
        timestamp: dec_str(timestamp)?,
        author: split_top_level(strip_brackets(author)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_str).collect::<Result<Vec<_>, String>>()?,
        organization: split_top_level(strip_brackets(organization)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_str).collect::<Result<Vec<_>, String>>()?,
        preprocessor_version: dec_str(preprocessor_version)?,
        originating_system: dec_str(originating_system)?,
        authorization: dec_str(authorization)?,
    })
}

pub(crate) fn enc_file_schema(s: &StepFileSchema) -> String {
    format!("[{}]", s.schemas.iter().map(|x| enc_str(x)).collect::<Vec<_>>().join(","))
}
pub(crate) fn dec_file_schema(s: &str) -> Result<StepFileSchema, String> {
    let schemas = split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_str).collect::<Result<Vec<_>, String>>()?;
    Ok(StepFileSchema { schemas })
}

/// 📸️ Full `StepSnapshot` codec — needed by `SetSnapshot`'s `OpText`/`OpBinary` (mutations file
/// imports this `pub(crate)`), never by `StepDiff` itself (no `snapshot: Option<StepSnapshot>`
/// full-replace slot exists on the diff).
pub(crate) fn enc_step_snapshot(s: &StepSnapshot) -> String {
    format!(
        "[{},{},{},{},[{}]]",
        enc_str(&s.schema),
        enc_file_description(&s.header.file_description),
        enc_file_name(&s.header.file_name),
        enc_file_schema(&s.header.file_schema),
        s.entities.iter().map(enc_entity).collect::<Vec<_>>().join(","),
    )
}
pub(crate) fn dec_step_snapshot(s: &str) -> Result<StepSnapshot, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [schema, file_description, file_name, file_schema, entities] = parts.as_slice() else {
        return Err(format!("step snapshot: expected 5 fields, got {}", parts.len()));
    };
    Ok(StepSnapshot {
        schema: dec_str(schema)?,
        header: crate::artifacts::step::schema::snapshot::StepHeader {
            file_description: dec_file_description(file_description)?,
            file_name: dec_file_name(file_name)?,
            file_schema: dec_file_schema(file_schema)?,
        },
        entities: split_top_level(strip_brackets(entities)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_entity).collect::<Result<Vec<_>, String>>()?,
    })
}
//#endregion 🔖️ValueCodecs

//#region 🔖️DiffValueCodecs
fn enc_args_diff(d: &StepArgsDiff) -> String {
    let removed = d.removed.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:{}", m.index, enc_value(&m.value))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}", a.index, enc_value(&a.value))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}
fn dec_args_diff(body: &str) -> Result<StepArgsDiff, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("args diff: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_usize).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("arg modified: bad entry {entry:?}"))?;
            Ok(StepArgModified { index: parse_usize(idx)?, value: dec_value(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("arg added: bad entry {entry:?}"))?;
            Ok(StepArgAdded { index: parse_usize(idx)?, value: dec_value(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(StepArgsDiff { removed, modified, added })
}

fn enc_entity_diff(d: &StepEntityDiff) -> String {
    format!(
        "[{},{},{}]",
        encode_option(&d.name, |v| enc_str(v)),
        encode_option(&d.args, enc_args_diff),
        encode_option(&d.complex, |v| format!("[{}]", v.iter().map(enc_complex).collect::<Vec<_>>().join(","))),
    )
}
fn dec_entity_diff(s: &str) -> Result<StepEntityDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, args, complex] = parts.as_slice() else { return Err(format!("entity diff: expected 3 fields, got {}", parts.len())) };
    Ok(StepEntityDiff {
        name: decode_option(name, dec_str)?,
        args: decode_option(args, dec_args_diff)?,
        complex: decode_option(complex, |v| {
            split_top_level(strip_brackets(v)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_complex).collect::<Result<Vec<_>, String>>()
        })?,
    })
}

fn enc_entities_diff(d: &StepEntitiesDiff) -> String {
    let removed = d.removed.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:{}", m.id, enc_entity_diff(&m.diff))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}", a.index, enc_entity(&a.entity))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}
fn dec_entities_diff(body: &str) -> Result<StepEntitiesDiff, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("entities diff: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_u64).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (id, rest) = entry.split_once(':').ok_or_else(|| format!("entity modified: bad entry {entry:?}"))?;
            Ok(StepEntityModified { id: parse_u64(id)?, diff: dec_entity_diff(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("entity added: bad entry {entry:?}"))?;
            Ok(StepEntityAdded { index: parse_usize(idx)?, entity: dec_entity(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(StepEntitiesDiff { removed, modified, added })
}
//#endregion 🔖️DiffValueCodecs

//#region 🔖️TopLevel
fn print_step_diff(d: &StepDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = &d.file_description { tokens.push(format!("file-description={}", enc_file_description(v))); }
    if let Some(v) = &d.file_name { tokens.push(format!("file-name={}", enc_file_name(v))); }
    if let Some(v) = &d.file_schema { tokens.push(format!("file-schema={}", enc_file_schema(v))); }
    if let Some(v) = &d.entities { tokens.push(format!("entities={}", enc_entities_diff(v))); }
    tokens.join(" ")
}
fn parse_step_diff(line: &str) -> Result<StepDiff, String> {
    let mut d = StepDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("file-description=") { d.file_description = Some(dec_file_description(rest)?); }
        else if let Some(rest) = token.strip_prefix("file-name=") { d.file_name = Some(dec_file_name(rest)?); }
        else if let Some(rest) = token.strip_prefix("file-schema=") { d.file_schema = Some(dec_file_schema(rest)?); }
        else if let Some(rest) = token.strip_prefix("entities=") { d.entities = Some(dec_entities_diff(rest)?); }
        else { return Err(format!("step diff: unknown token {token:?}")); }
    }
    Ok(d)
}

impl protocol::DiffCodec for StepDiff {
    fn print_diff(&self) -> String {
        print_step_diff(self)
    }
    fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        parse_step_diff(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    /// ⚡️ Binary = the text bytes verbatim, same simplification `GifDiff`/`SvgDiff`/`WriterDiff`
    /// use — satisfies every `DiffCodec` law without inventing a second wire format.
    fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(self.print_diff().into_bytes())
    }
    fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let line = std::str::from_utf8(bytes).map_err(|e| protocol::ProtocolError::Malformed { what: "diff utf8", offset: 0, detail: e.to_string() })?;
        Self::parse_diff(line).map_err(|e| protocol::ProtocolError::Malformed { what: "diff text", offset: 0, detail: e.to_string() })
    }
}
//#endregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::step::schema::snapshot::{StepFileDescription, StepFileName, StepFileSchema, StepHeader};
    use crate::artifacts::step::STDIO_STEP_DOCUMENT_SCHEMA;

    fn entity(id: u64, name: &str, args: Vec<StepValue>) -> StepEntity {
        StepEntity { id, name: name.into(), args, complex: Vec::new() }
    }

    fn base_snapshot() -> StepSnapshot {
        StepSnapshot {
            schema: STDIO_STEP_DOCUMENT_SCHEMA.into(),
            header: StepHeader {
                file_description: StepFileDescription { description: vec!["".into()], implementation_level: "2;1".into() },
                file_name: StepFileName {
                    name: "a.step".into(),
                    timestamp: "2026-08-10T00:00:00".into(),
                    author: vec!["Ueli".into()],
                    organization: vec!["semio".into()],
                    preprocessor_version: "semio".into(),
                    originating_system: "".into(),
                    authorization: "".into(),
                },
                file_schema: StepFileSchema { schemas: vec!["AUTOMOTIVE_DESIGN".into()] },
            },
            entities: vec![
                entity(1, "CARTESIAN_POINT", vec![StepValue::String("".into()), StepValue::Aggregate(vec![StepValue::Real(0.0), StepValue::Real(0.0), StepValue::Real(0.0)])]),
                entity(2, "CARTESIAN_POINT", vec![StepValue::String("".into()), StepValue::Aggregate(vec![StepValue::Real(1.0), StepValue::Real(0.0), StepValue::Real(0.0)])]),
                entity(3, "DIRECTION", vec![StepValue::String("".into()), StepValue::Reference(99)]),
            ],
        }
    }

    /// 🧪️ Canonical absorb case 1: `InsertEntity(2,e)` then `RemoveEntity(base-id-at-0)` →
    /// removed base id survives, added index shifts down by one.
    #[test]
    fn absorb_insert_then_remove_before_shifts_index() {
        let e = entity(50, "THING", vec![]);
        let mut d1 = StepEntitiesDiff { added: vec![StepEntityAdded { index: 2, entity: e.clone() }], ..Default::default() };
        let d2 = StepEntitiesDiff { removed: vec![1], ..Default::default() };
        d1.absorb(d2);
        assert_eq!(d1.removed, vec![1]);
        assert_eq!(d1.added, vec![StepEntityAdded { index: 1, entity: e }]);
        assert!(d1.modified.is_empty());
    }

    /// 🧪️ Canonical absorb case 2: `InsertEntity(2,e)` then `InsertEntity(2,f)` → BOTH survive.
    /// Id-keyed `entities` (like zip's name-keyed `entries`) does not renumber colliding `added`
    /// indices in the merged diff itself — `apply()`'s stable sort-by-index + sequential
    /// `insert(at, ..)` is what resolves the collision: d1's entry (listed first) inserts at 2,
    /// then d2's entry (listed second) also inserts at 2, pushing d1's entry to 3. Applying the
    /// merged diff proves both survive at the right FINAL positions even though the stored
    /// `index` fields are both still `2`.
    #[test]
    fn absorb_insert_insert_same_index_both_survive() {
        let e = entity(50, "A", vec![]);
        let f = entity(51, "B", vec![]);
        let mut d1 = StepEntitiesDiff { added: vec![StepEntityAdded { index: 2, entity: e.clone() }], ..Default::default() };
        let d2 = StepEntitiesDiff { added: vec![StepEntityAdded { index: 2, entity: f.clone() }], ..Default::default() };
        d1.absorb(d2);
        assert_eq!(d1.added, vec![StepEntityAdded { index: 2, entity: e.clone() }, StepEntityAdded { index: 2, entity: f.clone() }]);
        let base = vec![entity(1, "BASE0", vec![]), entity(2, "BASE1", vec![])];
        let applied = d1.apply(&base);
        let pos_e = applied.iter().position(|x| x.id == 50).expect("e survives");
        let pos_f = applied.iter().position(|x| x.id == 51).expect("f survives");
        assert_eq!(pos_f, 2, "later-absorbed insert (f) lands at the target index");
        assert_eq!(pos_e, 3, "earlier insert (e) is pushed one position later by f");
    }

    /// 🧪️ Canonical absorb case 3: `InsertEntity(1,e)` then `SetEntityName(e.id, "X")` patches
    /// INTO the added payload.
    #[test]
    fn absorb_insert_then_set_field_patches_into_added() {
        let e = entity(50, "A", vec![]);
        let mut d1 = StepEntitiesDiff { added: vec![StepEntityAdded { index: 1, entity: e.clone() }], ..Default::default() };
        let d2 = StepEntitiesDiff {
            modified: vec![StepEntityModified { id: 50, diff: StepEntityDiff { name: Some("X".into()), ..Default::default() } }],
            ..Default::default()
        };
        d1.absorb(d2);
        assert!(d1.modified.is_empty());
        assert_eq!(d1.added.len(), 1);
        assert_eq!(d1.added[0].entity.name, "X");
        assert_eq!(d1.added[0].index, 1);
    }

    #[test]
    fn absorb_law_holds_over_curated_ops() {
        let base = base_snapshot();
        let mid = {
            let mut s = base.clone();
            s.entities.insert(1, entity(60, "NEW", vec![StepValue::Unset]));
            s.entities.retain(|e| e.id != 1);
            s
        };
        let after = {
            let mut s = mid.clone();
            if let Some(e) = s.entities.iter_mut().find(|e| e.id == 60) {
                e.args.push(StepValue::Integer(7));
            }
            s.entities.push(entity(70, "MORE", vec![]));
            s
        };
        let mut d1 = <StepDiff as DiffAlgebra<StepSnapshot>>::between(&base, &mid);
        let d2 = <StepDiff as DiffAlgebra<StepSnapshot>>::between(&mid, &after);
        d1.absorb(d2);
        assert_eq!(d1.apply(&base), after);
    }

    #[test]
    fn between_roundtrip_law() {
        let a = base_snapshot();
        let mut b = a.clone();
        b.entities.push(entity(4, "EXTRA", vec![StepValue::Enum("T".into())]));
        b.header.file_schema.schemas.push("CONFIG_CONTROL_DESIGN".into());
        let ab = <StepDiff as DiffAlgebra<StepSnapshot>>::between(&a, &b);
        assert_eq!(ab.apply(&a), b);
        let ba = <StepDiff as DiffAlgebra<StepSnapshot>>::between(&b, &a);
        assert_eq!(ba.apply(&b), a);
        assert!(<StepDiff as DiffAlgebra<StepSnapshot>>::between(&a, &a).is_empty());
    }

    #[test]
    fn inverse_law() {
        let base = base_snapshot();
        let next = {
            let mut s = base.clone();
            s.entities[0].name = "RENAMED_POINT".into();
            s.entities.remove(2);
            s.entities.push(entity(9, "NEWTHING", vec![StepValue::Derived]));
            s.header.file_name.originating_system = "semio".into();
            s
        };
        let d = <StepDiff as DiffAlgebra<StepSnapshot>>::between(&base, &next);
        let mutated = d.apply(&base);
        let inv = d.inverse(&base);
        assert_eq!(inv.apply(&mutated), base);
    }

    /// 🧪️ Field sweep — the acceptance criterion: `sweep_a`/`sweep_b` differ in EVERY mutable
    /// field, with asymmetric collection lengths split across both `between()` directions (F1's
    /// structural trap — a single index/id-keyed `between()` call can show `removed` XOR `added`,
    /// never both, from one direction alone).
    #[test]
    fn field_sweep_covers_every_mutable_field() {
        let sweep_a = StepSnapshot {
            schema: STDIO_STEP_DOCUMENT_SCHEMA.into(),
            header: StepHeader {
                file_description: StepFileDescription { description: vec!["a".into()], implementation_level: "2;1".into() },
                file_name: StepFileName {
                    name: "a.step".into(),
                    timestamp: "2026-01-01T00:00:00".into(),
                    author: vec!["A".into()],
                    organization: vec!["OrgA".into()],
                    preprocessor_version: "pvA".into(),
                    originating_system: "sysA".into(),
                    authorization: "authA".into(),
                },
                file_schema: StepFileSchema { schemas: vec!["AUTOMOTIVE_DESIGN".into()] },
            },
            entities: vec![
                entity(1, "CARTESIAN_POINT", vec![StepValue::String("p1".into()), StepValue::Real(1.0)]),
                entity(2, "TO_REMOVE", vec![StepValue::Unset]),
            ],
        };
        let sweep_b = StepSnapshot {
            schema: STDIO_STEP_DOCUMENT_SCHEMA.into(),
            header: StepHeader {
                file_description: StepFileDescription { description: vec!["b".into(), "b2".into()], implementation_level: "2;2".into() },
                file_name: StepFileName {
                    name: "b.step".into(),
                    timestamp: "2026-02-02T00:00:00".into(),
                    author: vec!["B".into()],
                    organization: vec!["OrgB".into()],
                    preprocessor_version: "pvB".into(),
                    originating_system: "sysB".into(),
                    authorization: "authB".into(),
                },
                file_schema: StepFileSchema { schemas: vec!["CONFIG_CONTROL_DESIGN".into()] },
            },
            entities: vec![
                entity(1, "RENAMED_POINT", vec![StepValue::String("p1changed".into()), StepValue::Real(2.0), StepValue::Enum("T".into())]),
                entity(3, "ADDED_ENTITY", vec![StepValue::Reference(1)]),
                entity(4, "ANOTHER_ADDED", vec![]),
            ],
        };

        let ab = <StepDiff as DiffAlgebra<StepSnapshot>>::between(&sweep_a, &sweep_b);
        assert_eq!(ab.apply(&sweep_a), sweep_b);
        assert!(ab.file_description.is_some());
        assert!(ab.file_name.is_some());
        assert!(ab.file_schema.is_some());
        let entities_ab = ab.entities.as_ref().expect("entities must differ");
        assert!(!entities_ab.removed.is_empty(), "sweep must exercise a removed entity (id 2 absent from b)");
        assert!(!entities_ab.modified.is_empty(), "sweep must exercise a modified entity (id 1 changed)");
        assert!(!entities_ab.added.is_empty(), "sweep must exercise an added entity (b has ids 3,4)");
        let e1_diff = &entities_ab.modified.iter().find(|m| m.id == 1).expect("id 1 modified").diff;
        assert!(e1_diff.name.is_some());
        let args_diff = e1_diff.args.as_ref().expect("args must differ");
        assert!(!args_diff.modified.is_empty(), "arg 0/1 changed value");
        assert!(!args_diff.added.is_empty(), "arg 2 added (b's entity 1 has 3 args, a's has 2)");

        let ba = <StepDiff as DiffAlgebra<StepSnapshot>>::between(&sweep_b, &sweep_a);
        assert_eq!(ba.apply(&sweep_b), sweep_a);
        let entities_ba = ba.entities.as_ref().expect("entities must differ");
        assert!(!entities_ba.removed.is_empty(), "reverse direction must exercise removed (ids 3,4 absent from a)");
        assert!(!entities_ba.added.is_empty(), "reverse direction must exercise added (id 2 absent from b)");
        let e1_diff_ba = &entities_ba.modified.iter().find(|m| m.id == 1).expect("id 1 modified").diff;
        let args_diff_ba = e1_diff_ba.args.as_ref().expect("args must differ");
        assert!(!args_diff_ba.removed.is_empty(), "reverse direction must exercise a removed arg");

        assert!(<StepDiff as DiffAlgebra<StepSnapshot>>::between(&sweep_a, &sweep_a).is_empty());
    }
}

#[cfg(test)]
mod handcrafted_diff_codec_tests {
    use super::*;
    use protocol::DiffCodec;
    use crate::artifacts::step::schema::snapshot::{StepFileDescription, StepFileName, StepFileSchema, StepHeader};
    use crate::artifacts::step::STDIO_STEP_DOCUMENT_SCHEMA;

    fn entity(id: u64, name: &str, args: Vec<StepValue>) -> StepEntity {
        StepEntity { id, name: name.into(), args, complex: Vec::new() }
    }

    fn snapshot() -> StepSnapshot {
        StepSnapshot {
            schema: STDIO_STEP_DOCUMENT_SCHEMA.into(),
            header: StepHeader {
                file_description: StepFileDescription { description: vec!["a".into()], implementation_level: "2;1".into() },
                file_name: StepFileName { name: "a.step".into(), timestamp: "t".into(), author: vec!["Ueli".into()], organization: vec!["semio".into()], preprocessor_version: "pv".into(), originating_system: "sys".into(), authorization: "auth".into() },
                file_schema: StepFileSchema { schemas: vec!["AUTOMOTIVE_DESIGN".into()] },
            },
            entities: vec![
                entity(1, "CARTESIAN_POINT", vec![StepValue::String("".into()), StepValue::Aggregate(vec![StepValue::Real(0.0), StepValue::Real(1.5), StepValue::Integer(-3)])]),
                entity(2, "COMPLEX", vec![StepValue::Unset, StepValue::Derived, StepValue::Reference(7), StepValue::Enum("T".into()), StepValue::TypedValue { type_name: "IFCLENGTHMEASURE".into(), value: Box::new(StepValue::Real(3000.0)) }]),
            ],
        }
    }

    /// 🧪️ `diff_codec_text_binary_roundtrip_law`: exercises `StepValue`'s every variant (incl. the
    /// recursive `Aggregate`/`TypedValue` cases), `StepComplexType`, and all three `entities`
    /// collection-triple flavors (removed/modified/added) at once via a real `between()` result.
    #[test]
    fn diff_codec_text_binary_roundtrip_law() {
        let a = snapshot();
        let mut b = a.clone();
        b.header.file_schema.schemas.push("CONFIG_CONTROL_DESIGN".into());
        b.header.file_name.originating_system = "changed".into();
        b.entities[0].name = "RENAMED_POINT".into();
        b.entities[0].args.push(StepValue::Aggregate(vec![StepValue::Integer(1), StepValue::Integer(2)]));
        // Exercises `StepEntityDiff.complex: Option<Vec<StepComplexType>>` directly on a MODIFIED
        // entity (not just a freshly-added one) so `enc_entity_diff`'s `complex` field is `Some`.
        b.entities[0].complex.push(StepComplexType { name: "EXTRA_TYPE".into(), args: vec![StepValue::Real(1.5), StepValue::String("hi".into())] });
        b.entities.remove(1);
        b.entities.push(entity(3, "ADDED_WITH_COMPLEX", vec![StepValue::Unset]));
        b.entities[1].complex.push(StepComplexType { name: "ANOTHER_TYPE".into(), args: vec![StepValue::Reference(42)] });

        let cases = vec![
            StepDiff::default(),
            StepDiff::between(&a, &b),
            StepDiff::between(&b, &a),
            StepDiff::between(&a, &a),
        ];
        for d in cases {
            let printed = d.print_diff();
            assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
            let parsed = StepDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
            assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

            let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
            let decoded = StepDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
            assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
        }
    }
}
//#endregion 🧪️Tests
