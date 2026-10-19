//! 🧮️ Index transport, the generic collection algebra and the generic collection diff with its list delta macros.

use crate::{standards::v2_0::subsets::any::schema::snapshot::GltfAnimationChannel, standards::v2_0::subsets::any::schema::snapshot::GltfAnimationSampler, standards::v2_0::subsets::any::schema::snapshot::GltfMorphTarget};
use crate::standards::v2_0::subsets::any::schema::diff::*;

//#region 🔖️IndexTransport
/// 📐️ Shared rank/unrank arithmetic for index-keyed collection diffs (`absorb`/
/// `inverse`) — see `🧬️schema-design.md` §Absorb and the plan's "Absorb" section for the
/// derivation. `excluded_sorted` must be sorted ascending.
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

//#region 🔖️GenericCollectionAlgebra
/// 🧮️ Sequential-coalesce absorb for an index-keyed collection triple, generic over the item type
/// `T` and its per-item diff type `D`. Canonical correctness verified against the plan's 3
/// mandated cases in this module's tests. See `🧬️schema-design.md` §Absorb.
#[allow(clippy::too_many_arguments)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_indexed_collection<T: Clone, D: Clone>(
    removed1: Vec<usize>,
    modified1: Vec<(usize, D)>,
    added1: Vec<(usize, T)>,
    amended1: Vec<(usize, D)>,
    removed2: Vec<usize>,
    modified2: Vec<(usize, D)>,
    added2: Vec<(usize, T)>,
    amended2: Vec<(usize, D)>,
    mut absorb_diff: impl FnMut(&mut D, D),
) -> IndexedDiffParts<D, T> {
    let mut removed1_sorted = removed1;
    removed1_sorted.sort_unstable();
    let mut added1_index_sorted: Vec<usize> = added1.iter().map(|(i, _)| *i).collect();
    added1_index_sorted.sort_unstable();
    let mut removed2_sorted = removed2;
    removed2_sorted.sort_unstable();
    let mut added2_index_sorted: Vec<usize> = added2.iter().map(|(i, _)| *i).collect();
    added2_index_sorted.sort_unstable();

    let mut merged_added: Vec<(usize, T, Option<D>)> = added1.into_iter().map(|(at, item)| (at, item, amended1.iter().find(|(index, _)| *index == at).map(|(_, patch)| patch.clone()))).collect();
    let mut annihilated: std::collections::HashSet<usize> = Default::default();

    //#region Removed
    let mut merged_removed_base: Vec<usize> = removed1_sorted.clone();
    for &r2 in &removed2_sorted {
        if added1_index_sorted.binary_search(&r2).is_ok() {
            annihilated.insert(r2);
            merged_added.retain(|(i, _, _)| *i != r2);
        } else {
            let post_remove_rank = rank_excluding(r2, &added1_index_sorted);
            let base_index = unrank_excluding(post_remove_rank, &removed1_sorted);
            merged_removed_base.push(base_index);
        }
    }
    merged_removed_base.sort_unstable();
    merged_removed_base.dedup();
    //#endregion Removed

    //#region Modified
    let mut modified_map: std::collections::BTreeMap<usize, D> = modified1.into_iter().collect();
    for base_index in &merged_removed_base {
        modified_map.remove(base_index);
    }
    for (mp, dd2) in modified2 {
        if annihilated.contains(&mp) {
            continue;
        }
        if added1_index_sorted.binary_search(&mp).is_ok() {
            if let Some(entry) = merged_added.iter_mut().find(|(i, _, _)| *i == mp) {
                entry.2 = Some(match entry.2.take() {
                    Some(mut existing) => {
                        absorb_diff(&mut existing, dd2);
                        existing
                    }
                    None => dd2,
                });
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
    //#endregion Modified

    //#region Added
    let mut merged_added_final: Vec<(usize, T, Option<D>)> = merged_added
        .into_iter()
        .map(|(mp, item, patch)| {
            let after_pos = if removed2_sorted.binary_search(&mp).is_ok() {
                mp
            } else {
                let post_remove_rank = rank_excluding(mp, &removed2_sorted);
                unrank_excluding(post_remove_rank, &added2_index_sorted)
            };
            (after_pos, item, patch)
        })
        .collect();
    merged_added_final.extend(added2.into_iter().map(|(at, item)| (at, item, amended2.iter().find(|(index, _)| *index == at).map(|(_, patch)| patch.clone()))));
    merged_added_final.sort_by_key(|(i, _, _)| *i);
    let merged_amended: Vec<(usize, D)> = merged_added_final.iter().filter_map(|(at, _, patch)| patch.clone().map(|patch| (*at, patch))).collect();
    let merged_added_rows: Vec<(usize, T)> = merged_added_final.into_iter().map(|(at, item, _)| (at, item)).collect();
    //#endregion Added

    (merged_removed_base, merged_modified, merged_added_rows, merged_amended)
}

/// ↩️ Diff-level inverse for an index-keyed collection triple, given the ORIGINAL base items.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_indexed_collection<T: Clone, D: Clone>(removed: &[usize], modified: &[(usize, D)], added: &[(usize, T)], base_items: &[T], diff_inverse: impl Fn(&D, &T) -> D) -> IndexedDiffParts<D, T> {
    let mut removed_sorted = Vec::with_capacity(removed.len());
    removed_sorted.extend_from_slice(removed);
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
    (inv_removed, inv_modified, inv_added, Vec::new())
}
//#endregion 🔖️GenericCollectionAlgebra

//#region 🔖️ItemDiffTrait
/// 🧩️ A per-item diff for collection element type `T` -- implemented by real per-field diff
/// structs for STRONG entities (`GltfNodeDiff`, `GltfMeshDiff`, …), and by the blanket `T for T`
/// impl below for WEAK entities (the "diff" IS the whole new value). `apply` runs only under the received capability, which a
/// diff forwards to the nested list deltas it owns.
pub trait ItemDiff<T>: Clone + PartialEq {
    /// ✍️ The row's record once the diff is committed onto `base`; every implementor overrides it, the default refuses.
    fn apply(&self, base: &T, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<T> {
        let _ = (base, capability);
        Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "this row diff has no apply"))
    }
    fn inverse(&self, base: &T) -> Self;
    fn absorb_into(&mut self, other: Self);
}

/// 🍃️ WEAK entities: the diff type IS the item type (whole-value replace), per the recipe's
/// strong/weak split -- no further sub-structure worth diffing.
impl<T: Clone + PartialEq> ItemDiff<T> for T {
    fn apply(&self, _base: &T, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<T> {
        Ok(self.clone())
    }
    fn inverse(&self, base: &T) -> Self {
        base.clone()
    }
    fn absorb_into(&mut self, other: Self) {
        *self = other;
    }
}
//#endregion 🔖️ItemDiffTrait

//#region 🔖️GenericCollectionDiff
// 🩹 `#[derive(ToValue, FromValue)]` synthesizes `D: ToValue + FromValue` automatically per own
// type parameter (see `🌱️value/✨️derive`'s module docs) — no explicit `#[value(bound = "...")]`
// override needed here.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfModified<D> {
    pub index: usize,
    pub diff: D,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfAdded<T> {
    pub index: usize,
    pub item: T,
}

/// 🔺️ Generic index-keyed collection, instantiated once per position-addressed glTF array (the entities other entries refer to by
/// index, so a position IS the identity): `removed` base positions, `modified` base position plus the row's diff, `added` final
/// position plus the item, and `amended` the diff of a row this diff itself adds (keyed by its final position, because no base
/// position exists to key it by). `D = item::Diff` for strong entities; `D = T` (via the blanket impl) for weak entities.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfCollectionDiff<T, D> {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<GltfModified<D>>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<GltfAdded<T>>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub amended: Vec<GltfModified<D>>,
}

impl<T, D> Default for GltfCollectionDiff<T, D> {
    fn default() -> Self {
        Self { removed: Vec::new(), modified: Vec::new(), added: Vec::new(), amended: Vec::new() }
    }
}

impl<T: Clone + PartialEq, D: ItemDiff<T>> GltfCollectionDiff<T, D> {
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }

    pub fn validate_apply(&self, base_len: usize, target: &str) -> protocol::MutationApplyResult<()> {
        let mut removed = std::collections::BTreeSet::new();
        for &index in &self.removed {
            if index >= base_len || !removed.insert(index) {
                return Err(protocol::MutationApplyError::new("mutation.apply.invalid-remove-index", format!("remove index {index} is absent or duplicated")).at([target]));
            }
        }
        let mut modified = std::collections::BTreeSet::new();
        for entry in &self.modified {
            if entry.index >= base_len || removed.contains(&entry.index) || !modified.insert(entry.index) {
                return Err(protocol::MutationApplyError::new("mutation.apply.invalid-modify-index", format!("modify index {} is absent, removed, or duplicated", entry.index)).at([target]));
            }
        }
        let mut additions: Vec<usize> = self.added.iter().map(|entry| entry.index).collect();
        additions.sort_unstable();
        let mut previous = None;
        for (length, index) in (base_len - removed.len()..).zip(additions.iter().copied()) {
            if index > length || previous == Some(index) {
                return Err(protocol::MutationApplyError::new("mutation.apply.invalid-add-index", format!("add index {index} is out of range or duplicated")).at([target]));
            }
            previous = Some(index);
        }
        let mut amended = std::collections::BTreeSet::new();
        for entry in &self.amended {
            if additions.binary_search(&entry.index).is_err() || !amended.insert(entry.index) {
                return Err(protocol::MutationApplyError::new("mutation.apply.invalid-modify-index", format!("amend index {} names no added row or is duplicated", entry.index)).at([target]));
            }
        }
        Ok(())
    }

    /// ✍️ The list this diff turns `base` into; reached only from a diff type's own `apply`, under the received capability.
    pub fn apply(&self, base: &[T], target: &str, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Vec<T>> {
        self.validate_apply(base.len(), target)?;
        let mut next: Vec<Option<T>> = base.iter().cloned().map(Some).collect();
        for entry in &self.modified {
            if let Some(item) = next[entry.index].take() {
                next[entry.index] = Some(entry.diff.apply(&item, capability).map_err(|error| error.under([target.to_string(), entry.index.to_string()]))?);
            }
        }
        let mut removed_sorted = Vec::with_capacity(self.removed.len());
        removed_sorted.extend_from_slice(&self.removed);
        removed_sorted.sort_unstable();
        for &index in removed_sorted.iter().rev() {
            next.remove(index);
        }
        let mut out: Vec<T> = next.into_iter().flatten().collect();
        let mut added_sorted = Vec::with_capacity(self.added.len());
        added_sorted.extend(self.added.iter());
        added_sorted.sort_by_key(|entry| entry.index);
        for entry in added_sorted {
            out.insert(entry.index, entry.item.clone());
        }
        for entry in &self.amended {
            let item = out[entry.index].clone();
            out[entry.index] = entry.diff.apply(&item, capability).map_err(|error| error.under([target.to_string(), entry.index.to_string()]))?;
        }
        Ok(out)
    }

    pub fn absorb(&mut self, other: Self) {
        let (removed, modified, added, amended) = absorb_indexed_collection(
            std::mem::take(&mut self.removed),
            std::mem::take(&mut self.modified).into_iter().map(|m| (m.index, m.diff)).collect(),
            std::mem::take(&mut self.added).into_iter().map(|a| (a.index, a.item)).collect(),
            std::mem::take(&mut self.amended).into_iter().map(|m| (m.index, m.diff)).collect(),
            other.removed,
            other.modified.into_iter().map(|m| (m.index, m.diff)).collect(),
            other.added.into_iter().map(|a| (a.index, a.item)).collect(),
            other.amended.into_iter().map(|m| (m.index, m.diff)).collect(),
            |d, o| {
                d.absorb_into(o);
            },
        );
        self.removed = removed;
        self.modified = modified.into_iter().map(|(index, diff)| GltfModified { index, diff }).collect();
        self.added = added.into_iter().map(|(index, item)| GltfAdded { index, item }).collect();
        self.amended = amended.into_iter().map(|(index, diff)| GltfModified { index, diff }).collect();
    }

    pub fn inverse(&self, base_items: &[T]) -> Self {
        let (removed, modified, added, amended) =
            inverse_indexed_collection(&self.removed, &self.modified.iter().map(|m| (m.index, m.diff.clone())).collect::<Vec<_>>(), &self.added.iter().map(|a| (a.index, a.item.clone())).collect::<Vec<_>>(), base_items, |d, orig| d.inverse(orig));
        Self {
            removed,
            modified: modified.into_iter().map(|(index, diff)| GltfModified { index, diff }).collect(),
            added: added.into_iter().map(|(index, item)| GltfAdded { index, item }).collect(),
            amended: amended.into_iter().map(|(index, diff)| GltfModified { index, diff }).collect(),
        }
    }
}

/// 🍃️ Type alias for a WEAK collection (diff = whole new item).
pub type GltfWeakCollectionDiff<T> = GltfCollectionDiff<T, T>;

/// 🧷️ A reference to a top-level entry (a node, a mesh, an accessor): its own key in the lists of references an entity owns.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(transparent)]
pub struct GltfRef(pub usize);

/// 🧷️ One attribute of a primitive or morph target, keyed by its semantic.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct GltfAttribute {
    pub semantic: String,
    pub accessor: usize,
}

protocol::plain_list_delta! { pub GltfRefsDelta { removal: GltfRefRemoval, insertion: GltfRefInsertion, relocation: GltfRefRelocation, row: GltfRef, list: Vec<GltfRef>, key: usize = |reference| reference.0, values_only } }
protocol::plain_list_delta! { pub GltfStringsDelta { removal: GltfStringRemoval, insertion: GltfStringInsertion, relocation: GltfStringRelocation, row: String, values_only } }
protocol::plain_list_delta! { pub GltfAttributesDelta { removal: GltfAttributeRemoval, insertion: GltfAttributeInsertion, relocation: GltfAttributeRelocation, row: GltfAttribute, list: Vec<GltfAttribute>, key: String = |attribute| attribute.semantic.clone(), values_only } }
protocol::plain_list_delta! { pub GltfTargetsDelta { removal: GltfTargetRemoval, insertion: GltfTargetInsertion, relocation: GltfTargetRelocation, row: GltfMorphTarget, list: Vec<GltfMorphTarget>, key: String = |target| format!("{target:?}"), values_only } }
protocol::plain_list_delta! { pub GltfChannelsDelta { removal: GltfChannelRemoval, insertion: GltfChannelInsertion, relocation: GltfChannelRelocation, row: GltfAnimationChannel, list: Vec<GltfAnimationChannel>, key: String = |channel| format!("{channel:?}"), values_only } }
protocol::plain_list_delta! { pub GltfAnimationSamplersDelta { removal: GltfAnimationSamplerRemoval, insertion: GltfAnimationSamplerInsertion, relocation: GltfAnimationSamplerRelocation, row: GltfAnimationSampler, list: Vec<GltfAnimationSampler>, key: String = |sampler| format!("{sampler:?}"), values_only } }

/// 🧷️ The plain-row constructor of a macro-generated list delta: removed `(id, base index)`, inserted `(after index, row)` and moved
/// `(id, base index, after index)`, each read from the base list by the leaf that raises the delta.
macro_rules! delta_rows {
    ($delta:ident, $row:ty) => {
        impl $delta {
            pub fn rows(removed: Vec<(<$row as protocol::list_delta::Keyed>::Key, usize)>, inserted: Vec<(usize, $row)>, moved: Vec<(<$row as protocol::list_delta::Keyed>::Key, usize, usize)>) -> Self {
                Self::from_parts(protocol::list_delta::Parts { removed, inserted, moved, modified: Vec::new() })
            }
        }
    };
}
delta_rows!(GltfRefsDelta, GltfRef);
delta_rows!(GltfStringsDelta, String);
delta_rows!(GltfAttributesDelta, GltfAttribute);
delta_rows!(GltfTargetsDelta, GltfMorphTarget);
delta_rows!(GltfChannelsDelta, GltfAnimationChannel);
delta_rows!(GltfAnimationSamplersDelta, GltfAnimationSampler);

/// 🧷️ Folds `later` into the list delta already held in a slot of a diff.
macro_rules! absorb_slot {
    ($slot:expr, $later:expr) => {
        match (&mut $slot, $later) {
            (Some(mine), Some(theirs)) => mine.absorb(theirs),
            (slot @ None, Some(theirs)) => *slot = Some(theirs),
            _ => {}
        }
    };
}

/// 🧷️ The keyed rows of a list of entry references.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn refs(values: &[usize]) -> Vec<GltfRef> {
    values.iter().copied().map(GltfRef).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn unrefs(rows: Vec<GltfRef>) -> Vec<usize> {
    rows.into_iter().map(|reference| reference.0).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn attribute_rows(pairs: &[(String, usize)]) -> Vec<GltfAttribute> {
    pairs.iter().map(|(semantic, accessor)| GltfAttribute { semantic: semantic.clone(), accessor: *accessor }).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn attribute_pairs(rows: Vec<GltfAttribute>) -> Vec<(String, usize)> {
    rows.into_iter().map(|attribute| (attribute.semantic, attribute.accessor)).collect()
}
//#endregion 🔖️GenericCollectionDiff
