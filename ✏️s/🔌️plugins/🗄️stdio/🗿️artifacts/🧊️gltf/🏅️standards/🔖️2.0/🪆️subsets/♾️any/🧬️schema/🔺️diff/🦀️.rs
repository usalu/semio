//! 🔺️ GltfDiff — handcrafted sparse per-field diff for the fully typed glTF 2.0 document (ticket
//! ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION, F4). **DELETES the prior
//! `snapshot: Option<GltfSnapshot>` full-replace slot wholesale.** Index-keyed collection triples
//! for every one of the 14 top-level arrays (`scenes`/`nodes`/`meshes`/`accessors`/`bufferViews`/
//! `buffers`/`buffer` bytes/`materials`/`textures`/`images`/`samplers`/`skins`/`animations`/
//! `cameras`) plus sparse scalar slots for `asset`/`scene`/`extensionsUsed`/`extensionsRequired`/
//! `extensions`/`extras`/`sourceForm`. STRONG entities (scenes, nodes, meshes, accessors,
//! materials, buffers -- the recipe's explicitly prioritized highest-value arrays) get their own
//! per-field diff struct via the local [`ItemDiff`] trait; the remaining WEAK entities (bufferView,
//! buffer bytes, texture, image, sampler, skin, animation, camera -- real, undifferentiated glTF
//! objects whose "diff" is legitimately their whole new value per the recipe's strong/weak split)
//! reuse the SAME generic [`GltfCollectionDiff<T, D>`] wrapper via the blanket `ItemDiff<T> for T`
//! impl below (`D = T`). This is the general form of gif 89a's hand-duplicated
//! frames/comments/appExtensions triples -- one real generic collection algebra, instantiated per
//! entity, not a shortcut around per-entity semantics.

/// 🧩 Ordered removed keys, modified values, and inserted items.
pub(crate) type IndexedDiffParts<D, T> = (Vec<usize>, Vec<(usize, D)>, Vec<(usize, T)>, Vec<(usize, D)>);

use crate::standards::v2_0::subsets::any::schema::snapshot::{GltfAccessorType, GltfComponentType};
use crate::schema::snapshot::{
    GltfAccessor, GltfAlphaMode, GltfAnimation, GltfAnimationChannel, GltfAnimationChannelTarget, GltfAnimationPath, GltfAnimationSampler, GltfAsset, GltfBuffer, GltfBufferView, GltfCamera, GltfCameraProjection, GltfImage, GltfInterpolation,
    GltfJson, GltfMaterial, GltfMesh, GltfMorphTarget, GltfNode, GltfNormalTextureInfo, GltfOcclusionTextureInfo, GltfOrthographic, GltfPbrMetallicRoughness, GltfPerspective, GltfPrimitive, GltfSampler, GltfScene, GltfSkin, GltfSnapshot,
    GltfSourceForm, GltfSparseAccessor, GltfSparseIndices, GltfSparseValues, GltfTexture, GltfTextureInfo,
};
//#region 🔖️JsonPresence
/// 🈳️ Wire form for a diff slot holding `Option<Option<GltfJson>>`. Three states must stay
/// DISTINCT on the wire: "unchanged" (outer `None` — the key is absent, `skip_serializing_if`),
/// "cleared" (`Some(None)`) and "set to a JSON value" (`Some(Some(v))`). A bare `null` cannot
/// carry that, because [`GltfJson`] has its own `Null` variant: glTF 2.0 §3.2's `extras`/
/// `extensions` may legitimately hold JSON `null`, so `Some(Some(GltfJson::Null))` and
/// `Some(None)` both printed `null` and `Option`'s own `FromValue` read either one back as the
/// outer `None` — the change vanished from every JSON wire round trip (`GltfDiff` -> text ->
/// `GltfDiff`), taking the inverse mutation with it. The presence tag is the same `state`/`value`
/// vocabulary `GltfDataPresence` (the change-extras mutation payload) already spells.
/// https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html
pub mod json_presence {
    use super::GltfJson;

    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    pub fn to_value(value: &Option<Option<GltfJson>>) -> semio_framework_value::DslValue {
        match value {
            None => semio_framework_value::DslValue::Null,
            Some(None) => semio_framework_value::DslValue::object([("state".to_string(), semio_framework_value::DslValue::String("absent".to_string()))]),
            Some(Some(inner)) => semio_framework_value::DslValue::object([("state".to_string(), semio_framework_value::DslValue::String("present".to_string())), ("value".to_string(), semio_framework_value::ToValue::to_value(inner))]),
        }
    }

    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    pub fn from_value(value: semio_framework_value::DslValue) -> Result<Option<Option<GltfJson>>, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Object(entries) = &value else {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected a presence object {state,value}"));
        };
        match entries.iter().find(|(key, _)| key == "state").map(|(_, state)| state) {
            Some(semio_framework_value::DslValue::String(state)) if state == "absent" => Ok(Some(None)),
            Some(semio_framework_value::DslValue::String(state)) if state == "present" => {
                let inner = entries.iter().find(|(key, _)| key == "value").map(|(_, inner)| inner.clone()).unwrap_or(semio_framework_value::DslValue::Null);
                Ok(Some(Some(<GltfJson as semio_framework_value::FromValue>::from_value(inner)?)))
            }
            _ => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "presence object needs state = \"absent\" | \"present\"")),
        }
    }
}
//#endregion 🔖️JsonPresence

/// 🕳️ Tri-state decode of every non-JSON `Option<Option<T>>` slot (the shape the value derive's docs call
/// `deserialize_double_option`): the key is skipped when unchanged (`None`), and a PRESENT `null` is the clear
/// `Some(None)`, never the unchanged slot the blanket `Option<T>` decode would fold it into — so a restore diff that
/// clears `scale`/`mesh`/`name` survives every wire round trip. JSON slots use [`json_presence`] instead, since a
/// glTF JSON value may itself be `null`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn deserialize_double_option<T: semio_framework_value::FromValue>(value: semio_framework_value::DslValue) -> Result<Option<Option<T>>, semio_framework_value::ValueError> {
    <Option<T> as semio_framework_value::FromValue>::from_value(value).map(Some)
}

// 🧬️ `GltfDocument` is only reached through `mod tests`' `use super::*;` glob (its non-test uses
// below are all inside `#[cfg(test)]`), so — like the reactor/puzzle wasm-only imports elsewhere in
// this ticket — it must be gated to its actual consumer or it warns unused on the plain `lib` build.
#[cfg(test)]
use crate::schema::snapshot::GltfDocument;
use framework_schema::ArtifactSchema;
use protocol::os_spr::command::DiffAlgebra;
use protocol::MutationDiff;

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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(transparent)]
pub struct GltfRef(pub usize);

/// 🧷️ One attribute of a primitive or morph target, keyed by its semantic.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
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
fn unrefs(rows: Vec<GltfRef>) -> Vec<usize> {
    rows.into_iter().map(|reference| reference.0).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn attribute_rows(pairs: &[(String, usize)]) -> Vec<GltfAttribute> {
    pairs.iter().map(|(semantic, accessor)| GltfAttribute { semantic: semantic.clone(), accessor: *accessor }).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn attribute_pairs(rows: Vec<GltfAttribute>) -> Vec<(String, usize)> {
    rows.into_iter().map(|attribute| (attribute.semantic, attribute.accessor)).collect()
}
//#endregion 🔖️GenericCollectionDiff

//#region 🔖️AssetDiff
/// 🔺️ Sparse per-field diff for [`GltfAsset`] -- always present as a whole document (not a
/// collection item), so handled directly rather than through [`ItemDiff`].
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfAssetDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub generator: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub copyright: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub min_version: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", with = "json_presence")]
    pub extensions: Option<Option<GltfJson>>,
    #[value(default, skip_serializing_if = "Option::is_none", with = "json_presence")]
    pub extras: Option<Option<GltfJson>>,
}

impl GltfAssetDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.version.is_none() && self.generator.is_none() && self.copyright.is_none() && self.min_version.is_none() && self.extensions.is_none() && self.extras.is_none()
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn apply(&self, base: &GltfAsset, _capability: protocol::ApplyCapability) -> GltfAsset {
        self.apply_fields(base)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn apply_fields(&self, base: &GltfAsset) -> GltfAsset {
        let mut next = base.clone();
        if let Some(v) = &self.version {
            next.version = v.clone();
        }
        if let Some(v) = &self.generator {
            next.generator = v.clone();
        }
        if let Some(v) = &self.copyright {
            next.copyright = v.clone();
        }
        if let Some(v) = &self.min_version {
            next.min_version = v.clone();
        }
        if let Some(v) = &self.extensions {
            next.extensions = v.clone();
        }
        if let Some(v) = &self.extras {
            next.extras = v.clone();
        }
        next
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn inverse(&self, base: &GltfAsset) -> Self {
        Self {
            version: self.version.as_ref().map(|_| base.version.clone()),
            generator: self.generator.as_ref().map(|_| base.generator.clone()),
            copyright: self.copyright.as_ref().map(|_| base.copyright.clone()),
            min_version: self.min_version.as_ref().map(|_| base.min_version.clone()),
            extensions: self.extensions.as_ref().map(|_| base.extensions.clone()),
            extras: self.extras.as_ref().map(|_| base.extras.clone()),
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn absorb(&mut self, other: Self) {
        if other.version.is_some() {
            self.version = other.version;
        }
        if other.generator.is_some() {
            self.generator = other.generator;
        }
        if other.copyright.is_some() {
            self.copyright = other.copyright;
        }
        if other.min_version.is_some() {
            self.min_version = other.min_version;
        }
        if other.extensions.is_some() {
            self.extensions = other.extensions;
        }
        if other.extras.is_some() {
            self.extras = other.extras;
        }
    }
}
//#endregion 🔖️AssetDiff

//#region 🔖️SceneDiff
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfSceneDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub nodes: Option<GltfRefsDelta>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub name: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", with = "json_presence")]
    pub extensions: Option<Option<GltfJson>>,
    #[value(default, skip_serializing_if = "Option::is_none", with = "json_presence")]
    pub extras: Option<Option<GltfJson>>,
}

impl ItemDiff<GltfScene> for GltfSceneDiff {
    fn apply(&self, base: &GltfScene, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<GltfScene> {
        let mut next = base.clone();
        if let Some(v) = &self.nodes {
            next.nodes = unrefs(v.commit_onto(&refs(&base.nodes), capability).map_err(|error| error.under(["nodes"]))?);
        }
        if let Some(v) = &self.name {
            next.name = v.clone();
        }
        if let Some(v) = &self.extensions {
            next.extensions = v.clone();
        }
        if let Some(v) = &self.extras {
            next.extras = v.clone();
        }
        Ok(next)
    }
    fn inverse(&self, base: &GltfScene) -> Self {
        Self {
            nodes: self.nodes.as_ref().map(|delta| delta.inverse(&refs(&base.nodes))),
            name: self.name.as_ref().map(|_| base.name.clone()),
            extensions: self.extensions.as_ref().map(|_| base.extensions.clone()),
            extras: self.extras.as_ref().map(|_| base.extras.clone()),
        }
    }
    fn absorb_into(&mut self, other: Self) {
        absorb_slot!(self.nodes, other.nodes);
        if other.name.is_some() {
            self.name = other.name;
        }
        if other.extensions.is_some() {
            self.extensions = other.extensions;
        }
        if other.extras.is_some() {
            self.extras = other.extras;
        }
    }
}
//#endregion 🔖️SceneDiff

//#region 🔖️NodeDiff
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfNodeDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub children: Option<GltfRefsDelta>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub mesh: Option<Option<usize>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub camera: Option<Option<usize>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub skin: Option<Option<usize>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub matrix: Option<Option<[f64; 16]>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub translation: Option<Option<[f64; 3]>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub rotation: Option<Option<[f64; 4]>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub scale: Option<Option<[f64; 3]>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub weights: Option<Vec<f64>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub name: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", with = "json_presence")]
    pub extensions: Option<Option<GltfJson>>,
    #[value(default, skip_serializing_if = "Option::is_none", with = "json_presence")]
    pub extras: Option<Option<GltfJson>>,
}

impl ItemDiff<GltfNode> for GltfNodeDiff {
    fn apply(&self, base: &GltfNode, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<GltfNode> {
        let mut next = base.clone();
        if let Some(v) = &self.children {
            next.children = unrefs(v.commit_onto(&refs(&base.children), capability).map_err(|error| error.under(["children"]))?);
        }
        if let Some(v) = self.mesh {
            next.mesh = v;
        }
        if let Some(v) = self.camera {
            next.camera = v;
        }
        if let Some(v) = self.skin {
            next.skin = v;
        }
        if let Some(v) = self.matrix {
            next.matrix = v;
        }
        if let Some(v) = self.translation {
            next.translation = v;
        }
        if let Some(v) = self.rotation {
            next.rotation = v;
        }
        if let Some(v) = self.scale {
            next.scale = v;
        }
        if let Some(v) = &self.weights {
            next.weights = v.clone();
        }
        if let Some(v) = &self.name {
            next.name = v.clone();
        }
        if let Some(v) = &self.extensions {
            next.extensions = v.clone();
        }
        if let Some(v) = &self.extras {
            next.extras = v.clone();
        }
        Ok(next)
    }
    fn inverse(&self, base: &GltfNode) -> Self {
        Self {
            children: self.children.as_ref().map(|delta| delta.inverse(&refs(&base.children))),
            mesh: self.mesh.map(|_| base.mesh),
            camera: self.camera.map(|_| base.camera),
            skin: self.skin.map(|_| base.skin),
            matrix: self.matrix.map(|_| base.matrix),
            translation: self.translation.map(|_| base.translation),
            rotation: self.rotation.map(|_| base.rotation),
            scale: self.scale.map(|_| base.scale),
            weights: self.weights.as_ref().map(|_| base.weights.clone()),
            name: self.name.as_ref().map(|_| base.name.clone()),
            extensions: self.extensions.as_ref().map(|_| base.extensions.clone()),
            extras: self.extras.as_ref().map(|_| base.extras.clone()),
        }
    }
    fn absorb_into(&mut self, other: Self) {
        absorb_slot!(self.children, other.children);
        if other.mesh.is_some() {
            self.mesh = other.mesh;
        }
        if other.camera.is_some() {
            self.camera = other.camera;
        }
        if other.skin.is_some() {
            self.skin = other.skin;
        }
        if other.matrix.is_some() {
            self.matrix = other.matrix;
        }
        if other.translation.is_some() {
            self.translation = other.translation;
        }
        if other.rotation.is_some() {
            self.rotation = other.rotation;
        }
        if other.scale.is_some() {
            self.scale = other.scale;
        }
        if other.weights.is_some() {
            self.weights = other.weights;
        }
        if other.name.is_some() {
            self.name = other.name;
        }
        if other.extensions.is_some() {
            self.extensions = other.extensions;
        }
        if other.extras.is_some() {
            self.extras = other.extras;
        }
    }
}
//#endregion 🔖️NodeDiff

//#region 🔖️MeshDiff
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfMeshDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub primitives: Option<GltfPrimitivesDiff>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub weights: Option<Vec<f64>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub name: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", with = "json_presence")]
    pub extensions: Option<Option<GltfJson>>,
    #[value(default, skip_serializing_if = "Option::is_none", with = "json_presence")]
    pub extras: Option<Option<GltfJson>>,
}

impl ItemDiff<GltfMesh> for GltfMeshDiff {
    fn apply(&self, base: &GltfMesh, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<GltfMesh> {
        let mut next = base.clone();
        if let Some(v) = &self.primitives {
            next.primitives = v.apply(&base.primitives, "primitives", capability)?;
        }
        if let Some(v) = &self.weights {
            next.weights = v.clone();
        }
        if let Some(v) = &self.name {
            next.name = v.clone();
        }
        if let Some(v) = &self.extensions {
            next.extensions = v.clone();
        }
        if let Some(v) = &self.extras {
            next.extras = v.clone();
        }
        Ok(next)
    }
    fn inverse(&self, base: &GltfMesh) -> Self {
        Self {
            primitives: self.primitives.as_ref().map(|d| d.inverse(&base.primitives)),
            weights: self.weights.as_ref().map(|_| base.weights.clone()),
            name: self.name.as_ref().map(|_| base.name.clone()),
            extensions: self.extensions.as_ref().map(|_| base.extensions.clone()),
            extras: self.extras.as_ref().map(|_| base.extras.clone()),
        }
    }
    fn absorb_into(&mut self, other: Self) {
        match (&mut self.primitives, other.primitives) {
            (Some(mine), Some(theirs)) => mine.absorb(theirs),
            (slot @ None, Some(theirs)) => *slot = Some(theirs),
            _ => {}
        }
        if other.weights.is_some() {
            self.weights = other.weights;
        }
        if other.name.is_some() {
            self.name = other.name;
        }
        if other.extensions.is_some() {
            self.extensions = other.extensions;
        }
        if other.extras.is_some() {
            self.extras = other.extras;
        }
    }
}
//#endregion 🔖️MeshDiff

//#region 🔖️AccessorDiff
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfAccessorDiff {
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub buffer_view: Option<Option<usize>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub byte_offset: Option<usize>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub component_type: Option<GltfComponentType>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub normalized: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<usize>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<GltfAccessorType>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub max: Option<Option<Vec<f64>>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub min: Option<Option<Vec<f64>>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub sparse: Option<Option<GltfSparseAccessor>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub name: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", with = "json_presence")]
    pub extensions: Option<Option<GltfJson>>,
    #[value(default, skip_serializing_if = "Option::is_none", with = "json_presence")]
    pub extras: Option<Option<GltfJson>>,
}

impl ItemDiff<GltfAccessor> for GltfAccessorDiff {
    fn apply(&self, base: &GltfAccessor, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<GltfAccessor> {
        let mut next = base.clone();
        if let Some(v) = self.buffer_view {
            next.buffer_view = v;
        }
        if let Some(v) = self.byte_offset {
            next.byte_offset = v;
        }
        if let Some(v) = self.component_type {
            next.component_type = v;
        }
        if let Some(v) = self.normalized {
            next.normalized = v;
        }
        if let Some(v) = self.count {
            next.count = v;
        }
        if let Some(v) = self.kind {
            next.kind = v;
        }
        if let Some(v) = &self.max {
            next.max = v.clone();
        }
        if let Some(v) = &self.min {
            next.min = v.clone();
        }
        if let Some(v) = &self.sparse {
            next.sparse = v.clone();
        }
        if let Some(v) = &self.name {
            next.name = v.clone();
        }
        if let Some(v) = &self.extensions {
            next.extensions = v.clone();
        }
        if let Some(v) = &self.extras {
            next.extras = v.clone();
        }
        Ok(next)
    }
    fn inverse(&self, base: &GltfAccessor) -> Self {
        Self {
            buffer_view: self.buffer_view.map(|_| base.buffer_view),
            byte_offset: self.byte_offset.map(|_| base.byte_offset),
            component_type: self.component_type.map(|_| base.component_type),
            normalized: self.normalized.map(|_| base.normalized),
            count: self.count.map(|_| base.count),
            kind: self.kind.map(|_| base.kind),
            max: self.max.as_ref().map(|_| base.max.clone()),
            min: self.min.as_ref().map(|_| base.min.clone()),
            sparse: self.sparse.as_ref().map(|_| base.sparse.clone()),
            name: self.name.as_ref().map(|_| base.name.clone()),
            extensions: self.extensions.as_ref().map(|_| base.extensions.clone()),
            extras: self.extras.as_ref().map(|_| base.extras.clone()),
        }
    }
    fn absorb_into(&mut self, other: Self) {
        if other.buffer_view.is_some() {
            self.buffer_view = other.buffer_view;
        }
        if other.byte_offset.is_some() {
            self.byte_offset = other.byte_offset;
        }
        if other.component_type.is_some() {
            self.component_type = other.component_type;
        }
        if other.normalized.is_some() {
            self.normalized = other.normalized;
        }
        if other.count.is_some() {
            self.count = other.count;
        }
        if other.kind.is_some() {
            self.kind = other.kind;
        }
        if other.max.is_some() {
            self.max = other.max;
        }
        if other.min.is_some() {
            self.min = other.min;
        }
        if other.sparse.is_some() {
            self.sparse = other.sparse;
        }
        if other.name.is_some() {
            self.name = other.name;
        }
        if other.extensions.is_some() {
            self.extensions = other.extensions;
        }
        if other.extras.is_some() {
            self.extras = other.extras;
        }
    }
}
//#endregion 🔖️AccessorDiff

//#region 🔖️MaterialDiff
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfMaterialDiff {
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub name: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub pbr_metallic_roughness: Option<Option<GltfPbrMetallicRoughness>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub normal_texture: Option<Option<GltfNormalTextureInfo>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub occlusion_texture: Option<Option<GltfOcclusionTextureInfo>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub emissive_texture: Option<Option<GltfTextureInfo>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub emissive_factor: Option<[f64; 3]>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub alpha_mode: Option<GltfAlphaMode>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub alpha_cutoff: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub double_sided: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none", with = "json_presence")]
    pub extensions: Option<Option<GltfJson>>,
    #[value(default, skip_serializing_if = "Option::is_none", with = "json_presence")]
    pub extras: Option<Option<GltfJson>>,
}

impl ItemDiff<GltfMaterial> for GltfMaterialDiff {
    fn apply(&self, base: &GltfMaterial, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<GltfMaterial> {
        let mut next = base.clone();
        if let Some(v) = &self.name {
            next.name = v.clone();
        }
        if let Some(v) = &self.pbr_metallic_roughness {
            next.pbr_metallic_roughness = v.clone();
        }
        if let Some(v) = &self.normal_texture {
            next.normal_texture = v.clone();
        }
        if let Some(v) = &self.occlusion_texture {
            next.occlusion_texture = v.clone();
        }
        if let Some(v) = &self.emissive_texture {
            next.emissive_texture = v.clone();
        }
        if let Some(v) = self.emissive_factor {
            next.emissive_factor = v;
        }
        if let Some(v) = self.alpha_mode {
            next.alpha_mode = v;
        }
        if let Some(v) = self.alpha_cutoff {
            next.alpha_cutoff = v;
        }
        if let Some(v) = self.double_sided {
            next.double_sided = v;
        }
        if let Some(v) = &self.extensions {
            next.extensions = v.clone();
        }
        if let Some(v) = &self.extras {
            next.extras = v.clone();
        }
        Ok(next)
    }
    fn inverse(&self, base: &GltfMaterial) -> Self {
        Self {
            name: self.name.as_ref().map(|_| base.name.clone()),
            pbr_metallic_roughness: self.pbr_metallic_roughness.as_ref().map(|_| base.pbr_metallic_roughness.clone()),
            normal_texture: self.normal_texture.as_ref().map(|_| base.normal_texture.clone()),
            occlusion_texture: self.occlusion_texture.as_ref().map(|_| base.occlusion_texture.clone()),
            emissive_texture: self.emissive_texture.as_ref().map(|_| base.emissive_texture.clone()),
            emissive_factor: self.emissive_factor.map(|_| base.emissive_factor),
            alpha_mode: self.alpha_mode.map(|_| base.alpha_mode),
            alpha_cutoff: self.alpha_cutoff.map(|_| base.alpha_cutoff),
            double_sided: self.double_sided.map(|_| base.double_sided),
            extensions: self.extensions.as_ref().map(|_| base.extensions.clone()),
            extras: self.extras.as_ref().map(|_| base.extras.clone()),
        }
    }
    fn absorb_into(&mut self, other: Self) {
        if other.name.is_some() {
            self.name = other.name;
        }
        if other.pbr_metallic_roughness.is_some() {
            self.pbr_metallic_roughness = other.pbr_metallic_roughness;
        }
        if other.normal_texture.is_some() {
            self.normal_texture = other.normal_texture;
        }
        if other.occlusion_texture.is_some() {
            self.occlusion_texture = other.occlusion_texture;
        }
        if other.emissive_texture.is_some() {
            self.emissive_texture = other.emissive_texture;
        }
        if other.emissive_factor.is_some() {
            self.emissive_factor = other.emissive_factor;
        }
        if other.alpha_mode.is_some() {
            self.alpha_mode = other.alpha_mode;
        }
        if other.alpha_cutoff.is_some() {
            self.alpha_cutoff = other.alpha_cutoff;
        }
        if other.double_sided.is_some() {
            self.double_sided = other.double_sided;
        }
        if other.extensions.is_some() {
            self.extensions = other.extensions;
        }
        if other.extras.is_some() {
            self.extras = other.extras;
        }
    }
}
//#endregion 🔖️MaterialDiff

//#region 🔖️BufferDiff
/// 🔺️ Diff for `document.buffers[i]` METADATA (byteLength/uri/name/ext). The parallel raw-byte
/// payload lives in `GltfSnapshot::buffers` and is diffed separately by
/// [`GltfBufferBytesDiff`]/`buffer_bytes` (WEAK, whole-`Vec<u8>` replace) — see the recipe's
/// explicit "buffers: Vec<Vec<u8>> stays as-is" instruction: two index-aligned collections, not
/// one combined entity, matching the existing snapshot shape exactly.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfBufferDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub byte_length: Option<usize>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub uri: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub name: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", with = "json_presence")]
    pub extensions: Option<Option<GltfJson>>,
    #[value(default, skip_serializing_if = "Option::is_none", with = "json_presence")]
    pub extras: Option<Option<GltfJson>>,
}

impl ItemDiff<GltfBuffer> for GltfBufferDiff {
    fn apply(&self, base: &GltfBuffer, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<GltfBuffer> {
        let mut next = base.clone();
        if let Some(v) = self.byte_length {
            next.byte_length = v;
        }
        if let Some(v) = &self.uri {
            next.uri = v.clone();
        }
        if let Some(v) = &self.name {
            next.name = v.clone();
        }
        if let Some(v) = &self.extensions {
            next.extensions = v.clone();
        }
        if let Some(v) = &self.extras {
            next.extras = v.clone();
        }
        Ok(next)
    }
    fn inverse(&self, base: &GltfBuffer) -> Self {
        Self {
            byte_length: self.byte_length.map(|_| base.byte_length),
            uri: self.uri.as_ref().map(|_| base.uri.clone()),
            name: self.name.as_ref().map(|_| base.name.clone()),
            extensions: self.extensions.as_ref().map(|_| base.extensions.clone()),
            extras: self.extras.as_ref().map(|_| base.extras.clone()),
        }
    }
    fn absorb_into(&mut self, other: Self) {
        if other.byte_length.is_some() {
            self.byte_length = other.byte_length;
        }
        if other.uri.is_some() {
            self.uri = other.uri;
        }
        if other.name.is_some() {
            self.name = other.name;
        }
        if other.extensions.is_some() {
            self.extensions = other.extensions;
        }
        if other.extras.is_some() {
            self.extras = other.extras;
        }
    }
}
//#endregion 🔖️BufferDiff

//#region 🔖️PrimitiveDiff
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfPrimitiveDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<GltfAttributesDelta>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub indices: Option<Option<usize>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub material: Option<Option<usize>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub mode: Option<Option<u64>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub targets: Option<GltfTargetsDelta>,
    #[value(default, skip_serializing_if = "Option::is_none", with = "json_presence")]
    pub extensions: Option<Option<GltfJson>>,
    #[value(default, skip_serializing_if = "Option::is_none", with = "json_presence")]
    pub extras: Option<Option<GltfJson>>,
}

impl ItemDiff<GltfPrimitive> for GltfPrimitiveDiff {
    fn apply(&self, base: &GltfPrimitive, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<GltfPrimitive> {
        let mut next = base.clone();
        if let Some(v) = &self.attributes {
            next.attributes = attribute_pairs(v.commit_onto(&attribute_rows(&base.attributes), capability).map_err(|error| error.under(["attributes"]))?);
        }
        if let Some(v) = &self.indices {
            next.indices = v.clone();
        }
        if let Some(v) = &self.material {
            next.material = v.clone();
        }
        if let Some(v) = &self.mode {
            next.mode = v.clone();
        }
        if let Some(v) = &self.targets {
            next.targets = v.commit_onto(&base.targets, capability).map_err(|error| error.under(["targets"]))?;
        }
        if let Some(v) = &self.extensions {
            next.extensions = v.clone();
        }
        if let Some(v) = &self.extras {
            next.extras = v.clone();
        }
        Ok(next)
    }
    fn inverse(&self, base: &GltfPrimitive) -> Self {
        Self {
            attributes: self.attributes.as_ref().map(|delta| delta.inverse(&attribute_rows(&base.attributes))),
            indices: self.indices.as_ref().map(|_| base.indices.clone()),
            material: self.material.as_ref().map(|_| base.material.clone()),
            mode: self.mode.as_ref().map(|_| base.mode.clone()),
            targets: self.targets.as_ref().map(|delta| delta.inverse(&base.targets)),
            extensions: self.extensions.as_ref().map(|_| base.extensions.clone()),
            extras: self.extras.as_ref().map(|_| base.extras.clone()),
        }
    }
    fn absorb_into(&mut self, other: Self) {
        absorb_slot!(self.attributes, other.attributes);
        if other.indices.is_some() {
            self.indices = other.indices;
        }
        if other.material.is_some() {
            self.material = other.material;
        }
        if other.mode.is_some() {
            self.mode = other.mode;
        }
        absorb_slot!(self.targets, other.targets);
        if other.extensions.is_some() {
            self.extensions = other.extensions;
        }
        if other.extras.is_some() {
            self.extras = other.extras;
        }
    }
}
//#endregion 🔖️PrimitiveDiff

//#region 🔖️TextureDiff
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfTextureDiff {
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub sampler: Option<Option<usize>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub source: Option<Option<usize>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub name: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", with = "json_presence")]
    pub extensions: Option<Option<GltfJson>>,
    #[value(default, skip_serializing_if = "Option::is_none", with = "json_presence")]
    pub extras: Option<Option<GltfJson>>,
}

impl ItemDiff<GltfTexture> for GltfTextureDiff {
    fn apply(&self, base: &GltfTexture, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<GltfTexture> {
        let mut next = base.clone();
        if let Some(v) = &self.sampler {
            next.sampler = v.clone();
        }
        if let Some(v) = &self.source {
            next.source = v.clone();
        }
        if let Some(v) = &self.name {
            next.name = v.clone();
        }
        if let Some(v) = &self.extensions {
            next.extensions = v.clone();
        }
        if let Some(v) = &self.extras {
            next.extras = v.clone();
        }
        Ok(next)
    }
    fn inverse(&self, base: &GltfTexture) -> Self {
        Self {
            sampler: self.sampler.as_ref().map(|_| base.sampler.clone()),
            source: self.source.as_ref().map(|_| base.source.clone()),
            name: self.name.as_ref().map(|_| base.name.clone()),
            extensions: self.extensions.as_ref().map(|_| base.extensions.clone()),
            extras: self.extras.as_ref().map(|_| base.extras.clone()),
        }
    }
    fn absorb_into(&mut self, other: Self) {
        if other.sampler.is_some() {
            self.sampler = other.sampler;
        }
        if other.source.is_some() {
            self.source = other.source;
        }
        if other.name.is_some() {
            self.name = other.name;
        }
        if other.extensions.is_some() {
            self.extensions = other.extensions;
        }
        if other.extras.is_some() {
            self.extras = other.extras;
        }
    }
}
//#endregion 🔖️TextureDiff

//#region 🔖️ImageDiff
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfImageDiff {
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub uri: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub mime_type: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub buffer_view: Option<Option<usize>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub name: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", with = "json_presence")]
    pub extensions: Option<Option<GltfJson>>,
    #[value(default, skip_serializing_if = "Option::is_none", with = "json_presence")]
    pub extras: Option<Option<GltfJson>>,
}

impl ItemDiff<GltfImage> for GltfImageDiff {
    fn apply(&self, base: &GltfImage, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<GltfImage> {
        let mut next = base.clone();
        if let Some(v) = &self.uri {
            next.uri = v.clone();
        }
        if let Some(v) = &self.mime_type {
            next.mime_type = v.clone();
        }
        if let Some(v) = &self.buffer_view {
            next.buffer_view = v.clone();
        }
        if let Some(v) = &self.name {
            next.name = v.clone();
        }
        if let Some(v) = &self.extensions {
            next.extensions = v.clone();
        }
        if let Some(v) = &self.extras {
            next.extras = v.clone();
        }
        Ok(next)
    }
    fn inverse(&self, base: &GltfImage) -> Self {
        Self {
            uri: self.uri.as_ref().map(|_| base.uri.clone()),
            mime_type: self.mime_type.as_ref().map(|_| base.mime_type.clone()),
            buffer_view: self.buffer_view.as_ref().map(|_| base.buffer_view.clone()),
            name: self.name.as_ref().map(|_| base.name.clone()),
            extensions: self.extensions.as_ref().map(|_| base.extensions.clone()),
            extras: self.extras.as_ref().map(|_| base.extras.clone()),
        }
    }
    fn absorb_into(&mut self, other: Self) {
        if other.uri.is_some() {
            self.uri = other.uri;
        }
        if other.mime_type.is_some() {
            self.mime_type = other.mime_type;
        }
        if other.buffer_view.is_some() {
            self.buffer_view = other.buffer_view;
        }
        if other.name.is_some() {
            self.name = other.name;
        }
        if other.extensions.is_some() {
            self.extensions = other.extensions;
        }
        if other.extras.is_some() {
            self.extras = other.extras;
        }
    }
}
//#endregion 🔖️ImageDiff

//#region 🔖️BufferViewDiff
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfBufferViewDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub buffer: Option<usize>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub byte_offset: Option<usize>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub byte_length: Option<usize>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub byte_stride: Option<Option<usize>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub target: Option<Option<u64>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub name: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", with = "json_presence")]
    pub extensions: Option<Option<GltfJson>>,
    #[value(default, skip_serializing_if = "Option::is_none", with = "json_presence")]
    pub extras: Option<Option<GltfJson>>,
}

impl ItemDiff<GltfBufferView> for GltfBufferViewDiff {
    fn apply(&self, base: &GltfBufferView, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<GltfBufferView> {
        let mut next = base.clone();
        if let Some(v) = &self.buffer {
            next.buffer = v.clone();
        }
        if let Some(v) = &self.byte_offset {
            next.byte_offset = v.clone();
        }
        if let Some(v) = &self.byte_length {
            next.byte_length = v.clone();
        }
        if let Some(v) = &self.byte_stride {
            next.byte_stride = v.clone();
        }
        if let Some(v) = &self.target {
            next.target = v.clone();
        }
        if let Some(v) = &self.name {
            next.name = v.clone();
        }
        if let Some(v) = &self.extensions {
            next.extensions = v.clone();
        }
        if let Some(v) = &self.extras {
            next.extras = v.clone();
        }
        Ok(next)
    }
    fn inverse(&self, base: &GltfBufferView) -> Self {
        Self {
            buffer: self.buffer.as_ref().map(|_| base.buffer.clone()),
            byte_offset: self.byte_offset.as_ref().map(|_| base.byte_offset.clone()),
            byte_length: self.byte_length.as_ref().map(|_| base.byte_length.clone()),
            byte_stride: self.byte_stride.as_ref().map(|_| base.byte_stride.clone()),
            target: self.target.as_ref().map(|_| base.target.clone()),
            name: self.name.as_ref().map(|_| base.name.clone()),
            extensions: self.extensions.as_ref().map(|_| base.extensions.clone()),
            extras: self.extras.as_ref().map(|_| base.extras.clone()),
        }
    }
    fn absorb_into(&mut self, other: Self) {
        if other.buffer.is_some() {
            self.buffer = other.buffer;
        }
        if other.byte_offset.is_some() {
            self.byte_offset = other.byte_offset;
        }
        if other.byte_length.is_some() {
            self.byte_length = other.byte_length;
        }
        if other.byte_stride.is_some() {
            self.byte_stride = other.byte_stride;
        }
        if other.target.is_some() {
            self.target = other.target;
        }
        if other.name.is_some() {
            self.name = other.name;
        }
        if other.extensions.is_some() {
            self.extensions = other.extensions;
        }
        if other.extras.is_some() {
            self.extras = other.extras;
        }
    }
}
//#endregion 🔖️BufferViewDiff

//#region 🔖️SkinDiff
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfSkinDiff {
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub inverse_bind_matrices: Option<Option<usize>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub skeleton: Option<Option<usize>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub joints: Option<GltfRefsDelta>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub name: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", with = "json_presence")]
    pub extensions: Option<Option<GltfJson>>,
    #[value(default, skip_serializing_if = "Option::is_none", with = "json_presence")]
    pub extras: Option<Option<GltfJson>>,
}

impl ItemDiff<GltfSkin> for GltfSkinDiff {
    fn apply(&self, base: &GltfSkin, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<GltfSkin> {
        let mut next = base.clone();
        if let Some(v) = &self.inverse_bind_matrices {
            next.inverse_bind_matrices = v.clone();
        }
        if let Some(v) = &self.skeleton {
            next.skeleton = v.clone();
        }
        if let Some(v) = &self.joints {
            next.joints = unrefs(v.commit_onto(&refs(&base.joints), capability).map_err(|error| error.under(["joints"]))?);
        }
        if let Some(v) = &self.name {
            next.name = v.clone();
        }
        if let Some(v) = &self.extensions {
            next.extensions = v.clone();
        }
        if let Some(v) = &self.extras {
            next.extras = v.clone();
        }
        Ok(next)
    }
    fn inverse(&self, base: &GltfSkin) -> Self {
        Self {
            inverse_bind_matrices: self.inverse_bind_matrices.as_ref().map(|_| base.inverse_bind_matrices.clone()),
            skeleton: self.skeleton.as_ref().map(|_| base.skeleton.clone()),
            joints: self.joints.as_ref().map(|delta| delta.inverse(&refs(&base.joints))),
            name: self.name.as_ref().map(|_| base.name.clone()),
            extensions: self.extensions.as_ref().map(|_| base.extensions.clone()),
            extras: self.extras.as_ref().map(|_| base.extras.clone()),
        }
    }
    fn absorb_into(&mut self, other: Self) {
        if other.inverse_bind_matrices.is_some() {
            self.inverse_bind_matrices = other.inverse_bind_matrices;
        }
        if other.skeleton.is_some() {
            self.skeleton = other.skeleton;
        }
        absorb_slot!(self.joints, other.joints);
        if other.name.is_some() {
            self.name = other.name;
        }
        if other.extensions.is_some() {
            self.extensions = other.extensions;
        }
        if other.extras.is_some() {
            self.extras = other.extras;
        }
    }
}
//#endregion 🔖️SkinDiff

//#region 🔖️AnimationDiff
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfAnimationDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub channels: Option<GltfChannelsDelta>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub samplers: Option<GltfAnimationSamplersDelta>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub name: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", with = "json_presence")]
    pub extensions: Option<Option<GltfJson>>,
    #[value(default, skip_serializing_if = "Option::is_none", with = "json_presence")]
    pub extras: Option<Option<GltfJson>>,
}

impl ItemDiff<GltfAnimation> for GltfAnimationDiff {
    fn apply(&self, base: &GltfAnimation, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<GltfAnimation> {
        let mut next = base.clone();
        if let Some(v) = &self.channels {
            next.channels = v.commit_onto(&base.channels, capability).map_err(|error| error.under(["channels"]))?;
        }
        if let Some(v) = &self.samplers {
            next.samplers = v.commit_onto(&base.samplers, capability).map_err(|error| error.under(["samplers"]))?;
        }
        if let Some(v) = &self.name {
            next.name = v.clone();
        }
        if let Some(v) = &self.extensions {
            next.extensions = v.clone();
        }
        if let Some(v) = &self.extras {
            next.extras = v.clone();
        }
        Ok(next)
    }
    fn inverse(&self, base: &GltfAnimation) -> Self {
        Self {
            channels: self.channels.as_ref().map(|delta| delta.inverse(&base.channels)),
            samplers: self.samplers.as_ref().map(|delta| delta.inverse(&base.samplers)),
            name: self.name.as_ref().map(|_| base.name.clone()),
            extensions: self.extensions.as_ref().map(|_| base.extensions.clone()),
            extras: self.extras.as_ref().map(|_| base.extras.clone()),
        }
    }
    fn absorb_into(&mut self, other: Self) {
        absorb_slot!(self.channels, other.channels);
        absorb_slot!(self.samplers, other.samplers);
        if other.name.is_some() {
            self.name = other.name;
        }
        if other.extensions.is_some() {
            self.extensions = other.extensions;
        }
        if other.extras.is_some() {
            self.extras = other.extras;
        }
    }
}
//#endregion 🔖️AnimationDiff

//#region 🔖️CollectionTypeAliases
pub type GltfScenesDiff = GltfCollectionDiff<GltfScene, GltfSceneDiff>;
pub type GltfNodesDiff = GltfCollectionDiff<GltfNode, GltfNodeDiff>;
pub type GltfMeshesDiff = GltfCollectionDiff<GltfMesh, GltfMeshDiff>;
pub type GltfAccessorsDiff = GltfCollectionDiff<GltfAccessor, GltfAccessorDiff>;
pub type GltfMaterialsDiff = GltfCollectionDiff<GltfMaterial, GltfMaterialDiff>;
pub type GltfBuffersDiff = GltfCollectionDiff<GltfBuffer, GltfBufferDiff>;
pub type GltfBufferViewsDiff = GltfCollectionDiff<GltfBufferView, GltfBufferViewDiff>;
pub type GltfPrimitivesDiff = GltfCollectionDiff<GltfPrimitive, GltfPrimitiveDiff>;
pub type GltfBufferBytesDiff = GltfWeakCollectionDiff<Vec<u8>>;
pub type GltfTexturesDiff = GltfCollectionDiff<GltfTexture, GltfTextureDiff>;
pub type GltfImagesDiff = GltfCollectionDiff<GltfImage, GltfImageDiff>;
pub type GltfSamplersDiff = GltfWeakCollectionDiff<GltfSampler>;
pub type GltfSkinsDiff = GltfCollectionDiff<GltfSkin, GltfSkinDiff>;
pub type GltfAnimationsDiff = GltfCollectionDiff<GltfAnimation, GltfAnimationDiff>;
pub type GltfCamerasDiff = GltfWeakCollectionDiff<GltfCamera>;
//#endregion 🔖️CollectionTypeAliases

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.gltf`. No `snapshot: Option<GltfSnapshot>` full-replace slot anywhere.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.gltf.diff")]
pub struct GltfDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub asset: Option<GltfAssetDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub scene: Option<Option<usize>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub scenes: Option<GltfScenesDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub nodes: Option<GltfNodesDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub meshes: Option<GltfMeshesDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub accessors: Option<GltfAccessorsDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub buffer_views: Option<GltfBufferViewsDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub buffers: Option<GltfBuffersDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub buffer_bytes: Option<GltfBufferBytesDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub materials: Option<GltfMaterialsDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub textures: Option<GltfTexturesDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub images: Option<GltfImagesDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub samplers: Option<GltfSamplersDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub skins: Option<GltfSkinsDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub animations: Option<GltfAnimationsDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub cameras: Option<GltfCamerasDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extensions_used: Option<GltfStringsDelta>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extensions_required: Option<GltfStringsDelta>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none", with = "json_presence")]
    pub extensions: Option<Option<GltfJson>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none", with = "json_presence")]
    pub extras: Option<Option<GltfJson>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub source_form: Option<GltfSourceForm>,
}

impl GltfDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty_diff(&self) -> bool {
        self.asset.as_ref().is_none_or(GltfAssetDiff::is_empty)
            && self.scene.is_none()
            && self.scenes.as_ref().is_none_or(GltfScenesDiff::is_empty)
            && self.nodes.as_ref().is_none_or(GltfNodesDiff::is_empty)
            && self.meshes.as_ref().is_none_or(GltfMeshesDiff::is_empty)
            && self.accessors.as_ref().is_none_or(GltfAccessorsDiff::is_empty)
            && self.buffer_views.as_ref().is_none_or(GltfBufferViewsDiff::is_empty)
            && self.buffers.as_ref().is_none_or(GltfBuffersDiff::is_empty)
            && self.buffer_bytes.as_ref().is_none_or(GltfBufferBytesDiff::is_empty)
            && self.materials.as_ref().is_none_or(GltfMaterialsDiff::is_empty)
            && self.textures.as_ref().is_none_or(GltfTexturesDiff::is_empty)
            && self.images.as_ref().is_none_or(GltfImagesDiff::is_empty)
            && self.samplers.as_ref().is_none_or(GltfSamplersDiff::is_empty)
            && self.skins.as_ref().is_none_or(GltfSkinsDiff::is_empty)
            && self.animations.as_ref().is_none_or(GltfAnimationsDiff::is_empty)
            && self.cameras.as_ref().is_none_or(GltfCamerasDiff::is_empty)
            && self.extensions_used.as_ref().is_none_or(|delta| delta.is_empty())
            && self.extensions_required.as_ref().is_none_or(|delta| delta.is_empty())
            && self.extensions.is_none()
            && self.extras.is_none()
            && self.source_form.is_none()
    }
}

//#region 🗺️TouchedRegions
impl protocol::DiffRegions for GltfDiff {
    fn touches(&self) -> protocol::TouchedPaths {
        let mut paths = Vec::new();
        if self.asset.as_ref().is_some_and(|diff| !diff.is_empty()) {
            paths.push("document/asset".to_string());
        }
        if self.scene.is_some() {
            paths.push("document/scene".to_string());
        }
        macro_rules! collection_paths {
            ($field:ident, $name:literal) => {
                if let Some(diff) = &self.$field {
                    if !diff.removed.is_empty() || !diff.added.is_empty() {
                        paths.push(concat!("document/", $name).to_string());
                    } else {
                        paths.extend(diff.modified.iter().map(|entry| format!(concat!("document/", $name, "/{}"), entry.index)));
                    }
                }
            };
        }
        collection_paths!(scenes, "scenes");
        if let Some(diff) = &self.nodes {
            if !diff.removed.is_empty() || !diff.added.is_empty() {
                paths.push("document/nodes".to_string());
            } else {
                for entry in &diff.modified {
                    let root = format!("document/nodes/{}", entry.index);
                    if entry.diff.children.is_some() {
                        paths.push(format!("{root}/hierarchy"));
                    }
                    if entry.diff.matrix.is_some() || entry.diff.translation.is_some() || entry.diff.rotation.is_some() || entry.diff.scale.is_some() {
                        paths.push(format!("{root}/transform"));
                    }
                    if entry.diff.mesh.is_some() {
                        paths.push(format!("{root}/mesh"));
                    }
                    if entry.diff.skin.is_some() {
                        paths.push(format!("{root}/skin"));
                    }
                    if entry.diff.weights.is_some() {
                        paths.push(format!("{root}/weights"));
                    }
                    if entry.diff.camera.is_some() {
                        paths.push(format!("{root}/camera"));
                    }
                    if entry.diff.name.is_some() {
                        paths.push(format!("{root}/name"));
                    }
                    if entry.diff.extensions.is_some() {
                        paths.push(format!("{root}/extensions"));
                    }
                    if entry.diff.extras.is_some() {
                        paths.push(format!("{root}/extras"));
                    }
                }
            }
        }
        if let Some(diff) = &self.meshes {
            if !diff.removed.is_empty() || !diff.added.is_empty() {
                paths.push("document/meshes".to_string());
            } else {
                for entry in &diff.modified {
                    let root = format!("document/meshes/{}", entry.index);
                    if entry.diff.primitives.is_some() {
                        paths.push(format!("{root}/primitives"));
                    }
                    if entry.diff.weights.is_some() {
                        paths.push(format!("{root}/weights"));
                    }
                    if entry.diff.name.is_some() {
                        paths.push(format!("{root}/name"));
                    }
                    if entry.diff.extensions.is_some() {
                        paths.push(format!("{root}/extensions"));
                    }
                    if entry.diff.extras.is_some() {
                        paths.push(format!("{root}/extras"));
                    }
                }
            }
        }
        collection_paths!(accessors, "accessors");
        collection_paths!(buffer_views, "bufferViews");
        collection_paths!(buffers, "buffers");
        if let Some(diff) = &self.buffer_bytes {
            if !diff.removed.is_empty() || !diff.added.is_empty() {
                paths.push("buffers".to_string());
            } else {
                paths.extend(diff.modified.iter().map(|entry| format!("buffers/{}", entry.index)));
            }
        }
        collection_paths!(materials, "materials");
        collection_paths!(textures, "textures");
        collection_paths!(images, "images");
        collection_paths!(samplers, "samplers");
        collection_paths!(skins, "skins");
        collection_paths!(animations, "animations");
        collection_paths!(cameras, "cameras");
        if self.extensions_used.is_some() {
            paths.push("document/extensionsUsed".to_string());
        }
        if self.extensions_required.is_some() {
            paths.push("document/extensionsRequired".to_string());
        }
        if self.extensions.is_some() {
            paths.push("document/extensions".to_string());
        }
        if self.extras.is_some() {
            paths.push("document/extras".to_string());
        }
        if self.source_form.is_some() {
            paths.push("sourceForm".to_string());
        }
        paths.sort();
        paths.dedup();
        protocol::TouchedPaths { paths }
    }
}
//#endregion 🗺️TouchedRegions

impl MutationDiff<GltfSnapshot> for GltfDiff {
    fn apply(&self, base: &GltfSnapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<GltfSnapshot> {
        let mut next = base.clone();
        let doc = &mut next.document;
        if let Some(d) = &self.asset {
            doc.asset = d.apply(&doc.asset, capability);
        }
        if let Some(v) = self.scene {
            doc.scene = v;
        }
        if let Some(d) = &self.scenes {
            doc.scenes = d.apply(&doc.scenes, "document/scenes", capability)?;
        }
        if let Some(d) = &self.nodes {
            doc.nodes = d.apply(&doc.nodes, "document/nodes", capability)?;
        }
        if let Some(d) = &self.meshes {
            doc.meshes = d.apply(&doc.meshes, "document/meshes", capability)?;
        }
        if let Some(d) = &self.accessors {
            doc.accessors = d.apply(&doc.accessors, "document/accessors", capability)?;
        }
        if let Some(d) = &self.buffer_views {
            doc.buffer_views = d.apply(&doc.buffer_views, "document/bufferViews", capability)?;
        }
        if let Some(d) = &self.buffers {
            doc.buffers = d.apply(&doc.buffers, "document/buffers", capability)?;
        }
        if let Some(d) = &self.buffer_bytes {
            next.buffers = d.apply(&next.buffers, "buffers", capability)?;
        }
        if let Some(d) = &self.materials {
            doc.materials = d.apply(&doc.materials, "document/materials", capability)?;
        }
        if let Some(d) = &self.textures {
            doc.textures = d.apply(&doc.textures, "document/textures", capability)?;
        }
        if let Some(d) = &self.images {
            doc.images = d.apply(&doc.images, "document/images", capability)?;
        }
        if let Some(d) = &self.samplers {
            doc.samplers = d.apply(&doc.samplers, "document/samplers", capability)?;
        }
        if let Some(d) = &self.skins {
            doc.skins = d.apply(&doc.skins, "document/skins", capability)?;
        }
        if let Some(d) = &self.animations {
            doc.animations = d.apply(&doc.animations, "document/animations", capability)?;
        }
        if let Some(d) = &self.cameras {
            doc.cameras = d.apply(&doc.cameras, "document/cameras", capability)?;
        }
        if let Some(d) = &self.extensions_used {
            doc.extensions_used = d.commit_onto(&doc.extensions_used, capability).map_err(|error| error.under(["document/extensionsUsed"]))?;
        }
        if let Some(d) = &self.extensions_required {
            doc.extensions_required = d.commit_onto(&doc.extensions_required, capability).map_err(|error| error.under(["document/extensionsRequired"]))?;
        }
        if let Some(v) = &self.extensions {
            doc.extensions = v.clone();
        }
        if let Some(v) = &self.extras {
            doc.extras = v.clone();
        }
        if let Some(v) = self.source_form {
            next.source_form = v;
        }
        if let Some(scene) = next.document.scene {
            if scene >= next.document.scenes.len() {
                return Err(protocol::MutationApplyError::new("mutation.apply.invalid-reference", format!("default scene index {scene} does not address a scene")).at(["document/scene"]));
            }
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        match (&mut self.asset, other.asset) {
            (Some(mine), Some(theirs)) => mine.absorb(theirs),
            (slot @ None, Some(theirs)) => *slot = Some(theirs),
            _ => {}
        }
        if other.scene.is_some() {
            self.scene = other.scene;
        }
        macro_rules! absorb_collection {
            ($field:ident) => {
                match (&mut self.$field, other.$field) {
                    (Some(mine), Some(theirs)) => mine.absorb(theirs),
                    (slot @ None, Some(theirs)) => *slot = Some(theirs),
                    _ => {}
                }
            };
        }
        absorb_collection!(scenes);
        absorb_collection!(nodes);
        absorb_collection!(meshes);
        absorb_collection!(accessors);
        absorb_collection!(buffer_views);
        absorb_collection!(buffers);
        absorb_collection!(buffer_bytes);
        absorb_collection!(materials);
        absorb_collection!(textures);
        absorb_collection!(images);
        absorb_collection!(samplers);
        absorb_collection!(skins);
        absorb_collection!(animations);
        absorb_collection!(cameras);
        absorb_slot!(self.extensions_used, other.extensions_used);
        absorb_slot!(self.extensions_required, other.extensions_required);
        if other.extensions.is_some() {
            self.extensions = other.extensions;
        }
        if other.extras.is_some() {
            self.extras = other.extras;
        }
        if other.source_form.is_some() {
            self.source_form = other.source_form;
        }
    }
}

impl DiffAlgebra<GltfSnapshot> for GltfDiff {
    fn inverse(&self, base: &GltfSnapshot) -> Self {
        let doc = &base.document;
        Self {
            asset: self.asset.as_ref().map(|d| d.inverse(&doc.asset)),
            scene: self.scene.map(|_| doc.scene),
            scenes: self.scenes.as_ref().map(|d| d.inverse(&doc.scenes)),
            nodes: self.nodes.as_ref().map(|d| d.inverse(&doc.nodes)),
            meshes: self.meshes.as_ref().map(|d| d.inverse(&doc.meshes)),
            accessors: self.accessors.as_ref().map(|d| d.inverse(&doc.accessors)),
            buffer_views: self.buffer_views.as_ref().map(|d| d.inverse(&doc.buffer_views)),
            buffers: self.buffers.as_ref().map(|d| d.inverse(&doc.buffers)),
            buffer_bytes: self.buffer_bytes.as_ref().map(|d| d.inverse(&base.buffers)),
            materials: self.materials.as_ref().map(|d| d.inverse(&doc.materials)),
            textures: self.textures.as_ref().map(|d| d.inverse(&doc.textures)),
            images: self.images.as_ref().map(|d| d.inverse(&doc.images)),
            samplers: self.samplers.as_ref().map(|d| d.inverse(&doc.samplers)),
            skins: self.skins.as_ref().map(|d| d.inverse(&doc.skins)),
            animations: self.animations.as_ref().map(|d| d.inverse(&doc.animations)),
            cameras: self.cameras.as_ref().map(|d| d.inverse(&doc.cameras)),
            extensions_used: self.extensions_used.as_ref().map(|delta| delta.inverse(&doc.extensions_used)),
            extensions_required: self.extensions_required.as_ref().map(|delta| delta.inverse(&doc.extensions_required)),
            extensions: self.extensions.as_ref().map(|_| doc.extensions.clone()),
            extras: self.extras.as_ref().map(|_| doc.extras.clone()),
            source_form: self.source_form.map(|_| base.source_form),
        }
    }

    fn is_empty(&self) -> bool {
        self.is_empty_diff()
    }
}

/// 🧪️ P2-FG3: representative `GltfDiff` cases — the empty (`None`-everywhere) diff PLUS one rich diff that names a row in every one
/// of `GltfDiff`'s top-level clauses and one positional list row under a scene, a node and the used-extension list, declared
/// row by row against the lengths of `demo_gltf_snapshot()` — used by this artifact's own
/// `diff_grammar_conformance_law`/`protocol_walk_law` conformance tests (⚙️engine/component.rs).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_diff_cases() -> Vec<GltfDiff> {
    let base = crate::engine::demo_gltf_snapshot();
    let document = &base.document;
    fn appended<T, D>(len: usize, item: T) -> GltfCollectionDiff<T, D> {
        GltfCollectionDiff { removed: Vec::new(), modified: Vec::new(), added: vec![GltfAdded { index: len, item }], amended: Vec::new() }
    }
    let rich = GltfDiff {
        asset: Some(GltfAssetDiff { generator: Some(Some("semio-fg3".into())), ..Default::default() }),
        scene: Some(Some(1)),
        scenes: Some(appended(document.scenes.len(), GltfScene { nodes: vec![], name: Some("second-scene".into()), ..Default::default() })),
        nodes: Some(GltfNodesDiff {
            modified: vec![GltfModified {
                index: 0,
                diff: GltfNodeDiff { name: Some(Some("renamed-node".into())), children: Some(GltfRefsDelta::insertion(0, GltfRef(0))), ..Default::default() },
            }],
            ..Default::default()
        }),
        meshes: Some(appended(document.meshes.len(), GltfMesh::default())),
        accessors: Some(GltfAccessorsDiff { modified: vec![GltfModified { index: 0, diff: GltfAccessorDiff { count: Some(6), ..Default::default() } }], ..Default::default() }),
        buffer_views: Some(appended(document.buffer_views.len(), GltfBufferView { buffer: 0, byte_offset: 0, byte_length: 12, byte_stride: None, target: None, name: None, extensions: None, extras: None })),
        buffers: Some(appended(document.buffers.len(), GltfBuffer { byte_length: 4, uri: None, name: Some("extra".into()), extensions: None, extras: None })),
        buffer_bytes: Some(appended(base.buffers.len(), vec![1, 2, 3, 4])),
        materials: Some(GltfMaterialsDiff { modified: vec![GltfModified { index: 0, diff: GltfMaterialDiff { double_sided: Some(true), ..Default::default() } }], ..Default::default() }),
        textures: Some(appended(document.textures.len(), GltfTexture { sampler: None, source: None, name: Some("tex2".into()), extensions: None, extras: None })),
        images: Some(appended(document.images.len(), GltfImage { uri: Some("second.png".into()), ..Default::default() })),
        samplers: Some(appended(document.samplers.len(), GltfSampler::default())),
        skins: Some(appended(document.skins.len(), GltfSkin { joints: vec![0], ..Default::default() })),
        animations: Some(appended(document.animations.len(), GltfAnimation::default())),
        cameras: Some(appended(
            document.cameras.len(),
            GltfCamera {
                projection: GltfCameraProjection::Orthographic(GltfOrthographic { xmag: 1.0, ymag: 1.0, zfar: 10.0, znear: 0.1, extensions: None, extras: None }),
                name: Some("ortho-cam".into()),
                extensions: None,
                extras: None,
            },
        )),
        extensions_used: Some(GltfStringsDelta::insertion(document.extensions_used.len(), "KHR_texture_transform".to_string())),
        extensions_required: Some(GltfStringsDelta::insertion(document.extensions_required.len(), "KHR_texture_transform".to_string())),
        extensions: Some(Some(GltfJson::Bool(true))),
        extras: Some(None),
        source_form: Some(GltfSourceForm::Glb),
    };
    vec![GltfDiff::default(), rich]
}

//#endregion 🔖️Diff

//#region 🔖️HandcraftedDiffCodec
/// 🧪️ F6: **hand-rolled** `protocol::DiffCodec` for `GltfDiff` — CONFIRMED (not just per the
/// recon sweep's guess) by two independent real `cargo check -p semio-s-plugin-stdio --lib`
/// failures with `#[derive(dsl::DslDiff)]` temporarily added to this struct (captured verbatim in
/// `f6-gltf-recon-check1.txt`/`f6-gltf-diff-derive-check1.txt` in the ticket folder, then reverted
/// — 77 `E0277` errors): (1) every one of the 14 top-level arrays is typed through the GENERIC
/// [`GltfCollectionDiff<T, D>`] wrapper (e.g. `GltfCollectionDiff<GltfScene, GltfSceneDiff>`,
/// `GltfCollectionDiff<GltfCamera, GltfCamera>`) — `DslField` has no blanket impl for ANY
/// user-defined generic struct (only `Vec<T>`/`BTreeMap<String,T>`/`[T;N]` have such blanket impls
/// in the `dsl` crate), so the derive fails on EVERY collection field regardless of enum/tri-state
/// content, a blocker beyond both 3a and 3b from `f6-recon-report.md` §3; (2) `Option<GltfJson>` —
/// `GltfJson` is a real data-carrying enum (`Null`/`Bool`/`Number`/`String`/`Array`/`Object`), the
/// artifact's OWN local JSON value type for `extras`/`extensions` (F4's `GltfJson`, confirmed here
/// to in fact be diff-reachable via 20+ `Option<Option<GltfJson>>` fields, resolving the recon's
/// open question at classification row #23: 0 enums does NOT hold) — same 3a shape as `SvgNodeDiff`/
/// `XmlNode`. `GltfCameraProjection` (`Perspective`/`Orthographic`, inside `GltfCamera`, itself a
/// WEAK-collection item type) is a second data-carrying enum in the tree, reachable via the
/// `cameras` field. Every `Option<Option<T>>` tri-state field (42 per the recon sweep, e.g.
/// `GltfNodeDiff::mesh`/`matrix`/`translation`, `GltfAccessorDiff::sparse`, `GltfMaterialDiff::
/// pbr_metallic_roughness`) hits 3b independently on top. `#[derive(dsl::DslOps)]` on
/// `GltfMutation` (🧬️mutations/component.rs) fails the same way for the identical structural
/// reason (33 `E0277` errors, `f6-gltf-mutation-derive-check1.txt`): `SetSnapshot{snapshot:
/// GltfSnapshot}` (since removed) recursively requires `DslField` on `GltfAsset`/`GltfScene`/`GltfNode`/.../
/// `GltfSnapshot` itself, none of which are `DslRecord`-derived, and even if they all were, the
/// `GltfJson`/`GltfCameraProjection` enums nested inside would still block it (3a).
///
/// Grammar follows the same style as `GifDiff`/`SvgDiff`'s hand-rolled codecs (bracket-depth-aware
/// `split_top_level`, hex for strings/bytes, `[0]`/`[1,x]` for `Option<T>`, tag-prefix for
/// data-carrying enums) — this file re-derives its own copies of the small primitives (no shared
/// "hand-roll helpers" module exists yet, per `f6-recon-report.md` §5's "known duplication" note).
/// Given the sheer breadth of this artifact's fully-typed 2.0 model (by far the largest hand-roll
/// in the F6 program per the recon's own sizing), the value codecs below are grouped by field
/// GROUP (asset/scene/node; mesh/accessor/material; buffer family; texture/image/sampler/skin;
/// animation; camera) rather than one monolithic function, per the recon's own suggested structure.
//#region 🔖️Primitives











//#endregion 🔖️Primitives

//#region 🔖️ScalarCodecs















//#endregion 🔖️ScalarCodecs

//#region 🔖️GltfJsonCodec


//#endregion 🔖️GltfJsonCodec

//#region 🔖️UnitEnumCodecs












//#endregion 🔖️UnitEnumCodecs

//#region 🔖️AssetSceneNodeGroupCodecs












//#endregion 🔖️AssetSceneNodeGroupCodecs

//#region 🔖️MeshAccessorMaterialGroupCodecs
































//#endregion 🔖️MeshAccessorMaterialGroupCodecs

//#region 🔖️BufferGroupCodecs








//#endregion 🔖️BufferGroupCodecs

//#region 🔖️TextureImageSamplerSkinGroupCodecs








//#endregion 🔖️TextureImageSamplerSkinGroupCodecs

//#region 🔖️AnimationGroupCodecs








//#endregion 🔖️AnimationGroupCodecs

//#region 🔖️CameraGroupCodecs








//#endregion 🔖️CameraGroupCodecs

//#region 🔖️GenericCollectionCodec


//#endregion 🔖️GenericCollectionCodec

//#region 🔖️RealBinaryPrimitives





















//#endregion 🔖️RealBinaryPrimitives

//#region 🔖️RealBinaryJsonCodec




//#endregion 🔖️RealBinaryJsonCodec

//#region 🔖️RealBinaryUnitEnumCodecs












//#endregion 🔖️RealBinaryUnitEnumCodecs

//#region 🔖️RealBinaryAssetSceneNodeGroupCodecs










//#endregion 🔖️RealBinaryAssetSceneNodeGroupCodecs

//#region 🔖️RealBinaryMeshAccessorMaterialGroupCodecs






























//#endregion 🔖️RealBinaryMeshAccessorMaterialGroupCodecs

//#region 🔖️RealBinaryBufferGroupCodecs






//#endregion 🔖️RealBinaryBufferGroupCodecs

//#region 🔖️RealBinaryTextureImageSamplerSkinGroupCodecs








//#endregion 🔖️RealBinaryTextureImageSamplerSkinGroupCodecs

//#region 🔖️RealBinaryAnimationGroupCodecs








//#endregion 🔖️RealBinaryAnimationGroupCodecs

//#region 🔖️RealBinaryCameraGroupCodecs








//#endregion 🔖️RealBinaryCameraGroupCodecs

//#region 🔖️RealBinaryGenericCollectionCodec




//#endregion 🔖️RealBinaryGenericCollectionCodec

//#region 🔖️TopLevel




//#endregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🧪️HandcraftedDiffCodecTests
#[cfg(test)]
#[path = "🧪️tests/🔬️handcrafted-diff-codec/🦀️.rs"]
mod handcrafted_diff_codec_tests;
//#endregion 🧪️HandcraftedDiffCodecTests




















