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

use crate::artifacts::gltf::engine::{GltfAccessorType, GltfComponentType};
use crate::artifacts::gltf::schema::snapshot::{
    GltfAccessor, GltfAlphaMode, GltfAnimation, GltfAnimationChannel, GltfAnimationChannelTarget, GltfAnimationPath, GltfAnimationSampler, GltfAsset, GltfBuffer, GltfBufferView, GltfCamera, GltfCameraProjection, GltfImage,
    GltfInterpolation, GltfJson, GltfMaterial, GltfMesh, GltfMorphTarget, GltfNode, GltfNormalTextureInfo, GltfOcclusionTextureInfo, GltfOrthographic, GltfPbrMetallicRoughness, GltfPerspective, GltfPrimitive, GltfSampler, GltfScene, GltfSkin,
    GltfSnapshot, GltfSourceForm, GltfSparseAccessor, GltfSparseIndices, GltfSparseValues, GltfTexture, GltfTextureInfo,
};
// 🧬️ `GltfDocument` is only reached through `mod tests`' `use super::*;` glob (its non-test uses
// below are all inside `#[cfg(test)]`), so — like the reactor/puzzle wasm-only imports elsewhere in
// this ticket — it must be gated to its actual consumer or it warns unused on the plain `lib` build.
#[cfg(test)]
use crate::artifacts::gltf::schema::snapshot::GltfDocument;
use protocol::os_spr::command::DiffAlgebra;
use protocol::MutationDiff;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️IndexTransport
/// 📐️ Shared rank/unrank arithmetic for index-keyed collection diffs (`between`/`absorb`/
/// `inverse`) — see `🧬️schema-design.md` §Absorb and the plan's "Absorb" section for the
/// derivation. `excluded_sorted` must be sorted ascending.
async fn count_le(sorted: &[usize], x: usize) -> usize {
    sorted.partition_point(|&v| v <= x)
}
async fn rank_excluding(pos: usize, excluded_sorted: &[usize]) -> usize {
    pos - count_le(excluded_sorted, pos)
}
async fn unrank_excluding(rank: usize, excluded_sorted: &[usize]) -> usize {
    let mut candidate = rank;
    loop {
        let next = rank + count_le(excluded_sorted, candidate);
        if next == candidate {
            return candidate;
        }
        candidate = next;
    }
}
async fn transport_forward(index: usize, removed_sorted: &[usize], added_index_sorted: &[usize]) -> usize {
    unrank_excluding(rank_excluding(index, removed_sorted), added_index_sorted)
}
//#endregion 🔖️IndexTransport

//#region 🔖️GenericCollectionAlgebra
/// 🧮️ Sequential-coalesce absorb for an index-keyed collection triple, generic over the item type
/// `T` and its per-item diff type `D`. Canonical correctness verified against the plan's 3
/// mandated cases in this module's tests. See `🧬️schema-design.md` §Absorb.
#[allow(clippy::too_many_arguments)]
async fn absorb_indexed_collection<T: Clone, D: Clone>(
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
    let mut annihilated: std::collections::HashSet<usize> = Default::default();

    //#region Removed
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
    //#endregion Modified

    //#region Added
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
    //#endregion Added

    (merged_removed_base, merged_modified, merged_added_final)
}

/// ↩️ Diff-level inverse for an index-keyed collection triple, given the ORIGINAL base items.
async fn inverse_indexed_collection<T: Clone, D: Clone>(removed: &[usize], modified: &[(usize, D)], added: &[(usize, T)], base_items: &[T], diff_inverse: impl Fn(&D, &T) -> D) -> (Vec<usize>, Vec<(usize, D)>, Vec<(usize, T)>) {
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
//#endregion 🔖️GenericCollectionAlgebra

//#region 🔖️ItemDiffTrait
/// 🧩️ A per-item diff for collection element type `T` -- implemented by real per-field diff
/// structs for STRONG entities (`GltfNodeDiff`, `GltfMeshDiff`, …), and by the blanket `T for T`
/// impl below for WEAK entities (the "diff" IS the whole new value).
pub trait ItemDiff<T>: Clone + PartialEq {
    async fn between(base: &T, other: &T) -> Self;
    async fn apply(&self, base: &T) -> T;
    async fn inverse(&self, base: &T) -> Self;
    async fn absorb_into(&mut self, other: Self);
}

/// 🍃️ WEAK entities: the diff type IS the item type (whole-value replace), per the recipe's
/// strong/weak split -- no further sub-structure worth diffing.
impl<T: Clone + PartialEq> ItemDiff<T> for T {
    async fn between(_base: &T, other: &T) -> Self {
        other.clone()
    }
    async fn apply(&self, _base: &T) -> T {
        self.clone()
    }
    async fn inverse(&self, base: &T) -> Self {
        base.clone()
    }
    async fn absorb_into(&mut self, other: Self) {
        *self = other;
    }
}
//#endregion 🔖️ItemDiffTrait

//#region 🔖️GenericCollectionDiff
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(bound(serialize = "D: Serialize", deserialize = "D: Deserialize<'de>"))]
pub struct GltfModified<D> {
    pub index: usize,
    pub diff: D,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(bound(serialize = "T: Serialize", deserialize = "T: Deserialize<'de>"))]
pub struct GltfAdded<T> {
    pub index: usize,
    pub item: T,
}

/// 🔺️ Generic index-keyed collection triple, instantiated once per top-level glTF array. `D =
/// item::Diff` for strong entities; `D = T` (via the blanket impl) for weak entities. The explicit
/// `#[serde(bound(...))]` overrides serde_derive's overly-conservative auto-inferred bounds (which
/// would otherwise additionally require `T: Default`/`D: Default` purely because `#[serde(default)]`
/// appears on a field whose type mentions the generic parameter — `Vec<_>` itself is unconditionally
/// `Default` regardless of its element type, so that extra bound is never actually needed).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(bound(serialize = "T: Serialize, D: Serialize", deserialize = "T: Deserialize<'de>, D: Deserialize<'de>"))]
pub struct GltfCollectionDiff<T, D> {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<GltfModified<D>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<GltfAdded<T>>,
}

impl<T, D> Default for GltfCollectionDiff<T, D> {
    fn default() -> Self {
        Self { removed: Vec::new(), modified: Vec::new(), added: Vec::new() }
    }
}

impl<T: Clone + PartialEq, D: ItemDiff<T>> GltfCollectionDiff<T, D> {
    pub async fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }

    pub async fn between(base: &[T], other: &[T]) -> Self {
        let min = base.len().min(other.len());
        let mut modified = Vec::new();
        for i in 0..min {
            if base[i] != other[i] {
                modified.push(GltfModified { index: i, diff: D::between(&base[i], &other[i]) });
            }
        }
        let removed: Vec<usize> = (min..base.len()).collect();
        let added: Vec<GltfAdded<T>> = (min..other.len()).map(|i| GltfAdded { index: i, item: other[i].clone() }).collect();
        Self { removed, modified, added }
    }

    pub async fn validate_apply(&self, base_len: usize, target: &str) -> protocol::MutationApplyResult<()> {
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
        let mut length = base_len - removed.len();
        let mut additions: Vec<usize> = self.added.iter().map(|entry| entry.index).collect();
        additions.sort_unstable();
        let mut previous = None;
        for index in additions {
            if index > length || previous == Some(index) {
                return Err(protocol::MutationApplyError::new("mutation.apply.invalid-add-index", format!("add index {index} is out of range or duplicated")).at([target]));
            }
            previous = Some(index);
            length += 1;
        }
        Ok(())
    }

    pub async fn apply(&self, base: &[T]) -> Vec<T> {
        let mut next: Vec<Option<T>> = base.iter().cloned().map(Some).collect();
        for m in &self.modified {
            if let Some(slot) = next.get_mut(m.index) {
                if let Some(item) = slot {
                    *item = m.diff.apply(item);
                }
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
        let mut out: Vec<T> = next.into_iter().flatten().collect();
        let mut added_sorted = self.added.clone();
        added_sorted.sort_by_key(|a| a.index);
        for a in added_sorted {
            let at = a.index.min(out.len());
            out.insert(at, a.item);
        }
        out
    }

    pub async fn absorb(&mut self, other: Self) {
        let (removed, modified, added) = absorb_indexed_collection(
            std::mem::take(&mut self.removed),
            std::mem::take(&mut self.modified).into_iter().map(|m| (m.index, m.diff)).collect(),
            std::mem::take(&mut self.added).into_iter().map(|a| (a.index, a.item)).collect(),
            other.removed,
            other.modified.into_iter().map(|m| (m.index, m.diff)).collect(),
            other.added.into_iter().map(|a| (a.index, a.item)).collect(),
            |d, o| d.absorb_into(o),
            |d, item| d.apply(item),
        );
        self.removed = removed;
        self.modified = modified.into_iter().map(|(index, diff)| GltfModified { index, diff }).collect();
        self.added = added.into_iter().map(|(index, item)| GltfAdded { index, item }).collect();
    }

    pub async fn inverse(&self, base_items: &[T]) -> Self {
        let (removed, modified, added) =
            inverse_indexed_collection(&self.removed, &self.modified.iter().map(|m| (m.index, m.diff.clone())).collect::<Vec<_>>(), &self.added.iter().map(|a| (a.index, a.item.clone())).collect::<Vec<_>>(), base_items, |d, item| d.inverse(item));
        Self { removed, modified: modified.into_iter().map(|(index, diff)| GltfModified { index, diff }).collect(), added: added.into_iter().map(|(index, item)| GltfAdded { index, item }).collect() }
    }
}

/// 🍃️ Type alias for a WEAK collection (diff = whole new item).
pub type GltfWeakCollectionDiff<T> = GltfCollectionDiff<T, T>;
//#endregion 🔖️GenericCollectionDiff

//#region 🔖️AssetDiff
/// 🔺️ Sparse per-field diff for [`GltfAsset`] -- always present as a whole document (not a
/// collection item), so handled directly rather than through [`ItemDiff`].
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfAssetDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generator: Option<Option<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copyright: Option<Option<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_version: Option<Option<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Option<GltfJson>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extras: Option<Option<GltfJson>>,
}

impl GltfAssetDiff {
    pub async fn is_empty(&self) -> bool {
        self.version.is_none() && self.generator.is_none() && self.copyright.is_none() && self.min_version.is_none() && self.extensions.is_none() && self.extras.is_none()
    }
    pub async fn between(base: &GltfAsset, other: &GltfAsset) -> Self {
        Self {
            version: (base.version != other.version).then(|| other.version.clone()),
            generator: (base.generator != other.generator).then(|| other.generator.clone()),
            copyright: (base.copyright != other.copyright).then(|| other.copyright.clone()),
            min_version: (base.min_version != other.min_version).then(|| other.min_version.clone()),
            extensions: (base.extensions != other.extensions).then(|| other.extensions.clone()),
            extras: (base.extras != other.extras).then(|| other.extras.clone()),
        }
    }
    pub async fn apply(&self, base: &GltfAsset) -> GltfAsset {
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
    pub async fn inverse(&self, base: &GltfAsset) -> Self {
        Self {
            version: self.version.as_ref().map(|_| base.version.clone()),
            generator: self.generator.as_ref().map(|_| base.generator.clone()),
            copyright: self.copyright.as_ref().map(|_| base.copyright.clone()),
            min_version: self.min_version.as_ref().map(|_| base.min_version.clone()),
            extensions: self.extensions.as_ref().map(|_| base.extensions.clone()),
            extras: self.extras.as_ref().map(|_| base.extras.clone()),
        }
    }
    pub async fn absorb(&mut self, other: Self) {
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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfSceneDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<usize>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Option<GltfJson>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extras: Option<Option<GltfJson>>,
}

impl ItemDiff<GltfScene> for GltfSceneDiff {
    async fn between(base: &GltfScene, other: &GltfScene) -> Self {
        Self {
            nodes: (base.nodes != other.nodes).then(|| other.nodes.clone()),
            name: (base.name != other.name).then(|| other.name.clone()),
            extensions: (base.extensions != other.extensions).then(|| other.extensions.clone()),
            extras: (base.extras != other.extras).then(|| other.extras.clone()),
        }
    }
    async fn apply(&self, base: &GltfScene) -> GltfScene {
        let mut next = base.clone();
        if let Some(v) = &self.nodes {
            next.nodes = v.clone();
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
        next
    }
    async fn inverse(&self, base: &GltfScene) -> Self {
        Self {
            nodes: self.nodes.as_ref().map(|_| base.nodes.clone()),
            name: self.name.as_ref().map(|_| base.name.clone()),
            extensions: self.extensions.as_ref().map(|_| base.extensions.clone()),
            extras: self.extras.as_ref().map(|_| base.extras.clone()),
        }
    }
    async fn absorb_into(&mut self, other: Self) {
        if other.nodes.is_some() {
            self.nodes = other.nodes;
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
//#endregion 🔖️SceneDiff

//#region 🔖️NodeDiff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfNodeDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<usize>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mesh: Option<Option<usize>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub camera: Option<Option<usize>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skin: Option<Option<usize>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matrix: Option<Option<[f64; 16]>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub translation: Option<Option<[f64; 3]>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rotation: Option<Option<[f64; 4]>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<Option<[f64; 3]>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weights: Option<Vec<f64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Option<GltfJson>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extras: Option<Option<GltfJson>>,
}

impl ItemDiff<GltfNode> for GltfNodeDiff {
    async fn between(base: &GltfNode, other: &GltfNode) -> Self {
        Self {
            children: (base.children != other.children).then(|| other.children.clone()),
            mesh: (base.mesh != other.mesh).then_some(other.mesh),
            camera: (base.camera != other.camera).then_some(other.camera),
            skin: (base.skin != other.skin).then_some(other.skin),
            matrix: (base.matrix != other.matrix).then_some(other.matrix),
            translation: (base.translation != other.translation).then_some(other.translation),
            rotation: (base.rotation != other.rotation).then_some(other.rotation),
            scale: (base.scale != other.scale).then_some(other.scale),
            weights: (base.weights != other.weights).then(|| other.weights.clone()),
            name: (base.name != other.name).then(|| other.name.clone()),
            extensions: (base.extensions != other.extensions).then(|| other.extensions.clone()),
            extras: (base.extras != other.extras).then(|| other.extras.clone()),
        }
    }
    async fn apply(&self, base: &GltfNode) -> GltfNode {
        let mut next = base.clone();
        if let Some(v) = &self.children {
            next.children = v.clone();
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
        next
    }
    async fn inverse(&self, base: &GltfNode) -> Self {
        Self {
            children: self.children.as_ref().map(|_| base.children.clone()),
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
    async fn absorb_into(&mut self, other: Self) {
        if other.children.is_some() {
            self.children = other.children;
        }
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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfMeshDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primitives: Option<Vec<GltfPrimitive>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weights: Option<Vec<f64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Option<GltfJson>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extras: Option<Option<GltfJson>>,
}

impl ItemDiff<GltfMesh> for GltfMeshDiff {
    async fn between(base: &GltfMesh, other: &GltfMesh) -> Self {
        Self {
            primitives: (base.primitives != other.primitives).then(|| other.primitives.clone()),
            weights: (base.weights != other.weights).then(|| other.weights.clone()),
            name: (base.name != other.name).then(|| other.name.clone()),
            extensions: (base.extensions != other.extensions).then(|| other.extensions.clone()),
            extras: (base.extras != other.extras).then(|| other.extras.clone()),
        }
    }
    async fn apply(&self, base: &GltfMesh) -> GltfMesh {
        let mut next = base.clone();
        if let Some(v) = &self.primitives {
            next.primitives = v.clone();
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
        next
    }
    async fn inverse(&self, base: &GltfMesh) -> Self {
        Self {
            primitives: self.primitives.as_ref().map(|_| base.primitives.clone()),
            weights: self.weights.as_ref().map(|_| base.weights.clone()),
            name: self.name.as_ref().map(|_| base.name.clone()),
            extensions: self.extensions.as_ref().map(|_| base.extensions.clone()),
            extras: self.extras.as_ref().map(|_| base.extras.clone()),
        }
    }
    async fn absorb_into(&mut self, other: Self) {
        if other.primitives.is_some() {
            self.primitives = other.primitives;
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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfAccessorDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buffer_view: Option<Option<usize>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub byte_offset: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub component_type: Option<GltfComponentType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub normalized: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<GltfAccessorType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<Option<Vec<f64>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<Option<Vec<f64>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sparse: Option<Option<GltfSparseAccessor>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Option<GltfJson>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extras: Option<Option<GltfJson>>,
}

impl ItemDiff<GltfAccessor> for GltfAccessorDiff {
    async fn between(base: &GltfAccessor, other: &GltfAccessor) -> Self {
        Self {
            buffer_view: (base.buffer_view != other.buffer_view).then_some(other.buffer_view),
            byte_offset: (base.byte_offset != other.byte_offset).then_some(other.byte_offset),
            component_type: (base.component_type != other.component_type).then_some(other.component_type),
            normalized: (base.normalized != other.normalized).then_some(other.normalized),
            count: (base.count != other.count).then_some(other.count),
            kind: (base.kind != other.kind).then_some(other.kind),
            max: (base.max != other.max).then(|| other.max.clone()),
            min: (base.min != other.min).then(|| other.min.clone()),
            sparse: (base.sparse != other.sparse).then(|| other.sparse.clone()),
            name: (base.name != other.name).then(|| other.name.clone()),
            extensions: (base.extensions != other.extensions).then(|| other.extensions.clone()),
            extras: (base.extras != other.extras).then(|| other.extras.clone()),
        }
    }
    async fn apply(&self, base: &GltfAccessor) -> GltfAccessor {
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
        next
    }
    async fn inverse(&self, base: &GltfAccessor) -> Self {
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
    async fn absorb_into(&mut self, other: Self) {
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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfMaterialDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pbr_metallic_roughness: Option<Option<GltfPbrMetallicRoughness>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub normal_texture: Option<Option<GltfNormalTextureInfo>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub occlusion_texture: Option<Option<GltfOcclusionTextureInfo>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emissive_texture: Option<Option<GltfTextureInfo>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emissive_factor: Option<[f64; 3]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alpha_mode: Option<GltfAlphaMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alpha_cutoff: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub double_sided: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Option<GltfJson>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extras: Option<Option<GltfJson>>,
}

impl ItemDiff<GltfMaterial> for GltfMaterialDiff {
    async fn between(base: &GltfMaterial, other: &GltfMaterial) -> Self {
        Self {
            name: (base.name != other.name).then(|| other.name.clone()),
            pbr_metallic_roughness: (base.pbr_metallic_roughness != other.pbr_metallic_roughness).then(|| other.pbr_metallic_roughness.clone()),
            normal_texture: (base.normal_texture != other.normal_texture).then(|| other.normal_texture.clone()),
            occlusion_texture: (base.occlusion_texture != other.occlusion_texture).then(|| other.occlusion_texture.clone()),
            emissive_texture: (base.emissive_texture != other.emissive_texture).then(|| other.emissive_texture.clone()),
            emissive_factor: (base.emissive_factor != other.emissive_factor).then_some(other.emissive_factor),
            alpha_mode: (base.alpha_mode != other.alpha_mode).then_some(other.alpha_mode),
            alpha_cutoff: (base.alpha_cutoff != other.alpha_cutoff).then_some(other.alpha_cutoff),
            double_sided: (base.double_sided != other.double_sided).then_some(other.double_sided),
            extensions: (base.extensions != other.extensions).then(|| other.extensions.clone()),
            extras: (base.extras != other.extras).then(|| other.extras.clone()),
        }
    }
    async fn apply(&self, base: &GltfMaterial) -> GltfMaterial {
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
        next
    }
    async fn inverse(&self, base: &GltfMaterial) -> Self {
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
    async fn absorb_into(&mut self, other: Self) {
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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfBufferDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub byte_length: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<Option<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Option<GltfJson>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extras: Option<Option<GltfJson>>,
}

impl ItemDiff<GltfBuffer> for GltfBufferDiff {
    async fn between(base: &GltfBuffer, other: &GltfBuffer) -> Self {
        Self {
            byte_length: (base.byte_length != other.byte_length).then_some(other.byte_length),
            uri: (base.uri != other.uri).then(|| other.uri.clone()),
            name: (base.name != other.name).then(|| other.name.clone()),
            extensions: (base.extensions != other.extensions).then(|| other.extensions.clone()),
            extras: (base.extras != other.extras).then(|| other.extras.clone()),
        }
    }
    async fn apply(&self, base: &GltfBuffer) -> GltfBuffer {
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
        next
    }
    async fn inverse(&self, base: &GltfBuffer) -> Self {
        Self {
            byte_length: self.byte_length.map(|_| base.byte_length),
            uri: self.uri.as_ref().map(|_| base.uri.clone()),
            name: self.name.as_ref().map(|_| base.name.clone()),
            extensions: self.extensions.as_ref().map(|_| base.extensions.clone()),
            extras: self.extras.as_ref().map(|_| base.extras.clone()),
        }
    }
    async fn absorb_into(&mut self, other: Self) {
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

//#region 🔖️CollectionTypeAliases
pub type GltfScenesDiff = GltfCollectionDiff<GltfScene, GltfSceneDiff>;
pub type GltfNodesDiff = GltfCollectionDiff<GltfNode, GltfNodeDiff>;
pub type GltfMeshesDiff = GltfCollectionDiff<GltfMesh, GltfMeshDiff>;
pub type GltfAccessorsDiff = GltfCollectionDiff<GltfAccessor, GltfAccessorDiff>;
pub type GltfMaterialsDiff = GltfCollectionDiff<GltfMaterial, GltfMaterialDiff>;
pub type GltfBuffersDiff = GltfCollectionDiff<GltfBuffer, GltfBufferDiff>;
pub type GltfBufferViewsDiff = GltfWeakCollectionDiff<GltfBufferView>;
pub type GltfBufferBytesDiff = GltfWeakCollectionDiff<Vec<u8>>;
pub type GltfTexturesDiff = GltfWeakCollectionDiff<GltfTexture>;
pub type GltfImagesDiff = GltfWeakCollectionDiff<GltfImage>;
pub type GltfSamplersDiff = GltfWeakCollectionDiff<GltfSampler>;
pub type GltfSkinsDiff = GltfWeakCollectionDiff<GltfSkin>;
pub type GltfAnimationsDiff = GltfWeakCollectionDiff<GltfAnimation>;
pub type GltfCamerasDiff = GltfWeakCollectionDiff<GltfCamera>;
//#endregion 🔖️CollectionTypeAliases

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.gltf`. No `snapshot: Option<GltfSnapshot>` full-replace slot anywhere.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.gltf.diff")]
pub struct GltfDiff {
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub asset: Option<GltfAssetDiff>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scene: Option<Option<usize>>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scenes: Option<GltfScenesDiff>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nodes: Option<GltfNodesDiff>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meshes: Option<GltfMeshesDiff>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accessors: Option<GltfAccessorsDiff>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buffer_views: Option<GltfBufferViewsDiff>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buffers: Option<GltfBuffersDiff>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buffer_bytes: Option<GltfBufferBytesDiff>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub materials: Option<GltfMaterialsDiff>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub textures: Option<GltfTexturesDiff>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub images: Option<GltfImagesDiff>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub samplers: Option<GltfSamplersDiff>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skins: Option<GltfSkinsDiff>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub animations: Option<GltfAnimationsDiff>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cameras: Option<GltfCamerasDiff>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions_used: Option<Vec<String>>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions_required: Option<Vec<String>>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Option<GltfJson>>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extras: Option<Option<GltfJson>>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_form: Option<GltfSourceForm>,
}

impl GltfDiff {
    pub async fn is_empty_diff(&self) -> bool {
        self.asset.as_ref().map(GltfAssetDiff::is_empty).unwrap_or(true)
            && self.scene.is_none()
            && self.scenes.as_ref().map(GltfScenesDiff::is_empty).unwrap_or(true)
            && self.nodes.as_ref().map(GltfNodesDiff::is_empty).unwrap_or(true)
            && self.meshes.as_ref().map(GltfMeshesDiff::is_empty).unwrap_or(true)
            && self.accessors.as_ref().map(GltfAccessorsDiff::is_empty).unwrap_or(true)
            && self.buffer_views.as_ref().map(GltfBufferViewsDiff::is_empty).unwrap_or(true)
            && self.buffers.as_ref().map(GltfBuffersDiff::is_empty).unwrap_or(true)
            && self.buffer_bytes.as_ref().map(GltfBufferBytesDiff::is_empty).unwrap_or(true)
            && self.materials.as_ref().map(GltfMaterialsDiff::is_empty).unwrap_or(true)
            && self.textures.as_ref().map(GltfTexturesDiff::is_empty).unwrap_or(true)
            && self.images.as_ref().map(GltfImagesDiff::is_empty).unwrap_or(true)
            && self.samplers.as_ref().map(GltfSamplersDiff::is_empty).unwrap_or(true)
            && self.skins.as_ref().map(GltfSkinsDiff::is_empty).unwrap_or(true)
            && self.animations.as_ref().map(GltfAnimationsDiff::is_empty).unwrap_or(true)
            && self.cameras.as_ref().map(GltfCamerasDiff::is_empty).unwrap_or(true)
            && self.extensions_used.is_none()
            && self.extensions_required.is_none()
            && self.extensions.is_none()
            && self.extras.is_none()
            && self.source_form.is_none()
    }
}

//#region 🗺️TouchedRegions
impl protocol::DiffRegions for GltfDiff {
    async fn touches(&self) -> protocol::TouchedPaths {
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
    async fn apply(&self, base: &GltfSnapshot) -> protocol::MutationApplyResult<GltfSnapshot> {
        macro_rules! validate_collection {
            ($field:ident, $base:expr, $target:literal) => {
                if let Some(diff) = &self.$field {
                    diff.validate_apply($base.len(), $target)?;
                }
            };
        }
        validate_collection!(scenes, base.document.scenes, "document/scenes");
        validate_collection!(nodes, base.document.nodes, "document/nodes");
        validate_collection!(meshes, base.document.meshes, "document/meshes");
        validate_collection!(accessors, base.document.accessors, "document/accessors");
        validate_collection!(buffer_views, base.document.buffer_views, "document/bufferViews");
        validate_collection!(buffers, base.document.buffers, "document/buffers");
        validate_collection!(buffer_bytes, base.buffers, "buffers");
        validate_collection!(materials, base.document.materials, "document/materials");
        validate_collection!(textures, base.document.textures, "document/textures");
        validate_collection!(images, base.document.images, "document/images");
        validate_collection!(samplers, base.document.samplers, "document/samplers");
        validate_collection!(skins, base.document.skins, "document/skins");
        validate_collection!(animations, base.document.animations, "document/animations");
        validate_collection!(cameras, base.document.cameras, "document/cameras");
        let mut next = base.clone();
        let doc = &mut next.document;
        if let Some(d) = &self.asset {
            doc.asset = d.apply(&doc.asset);
        }
        if let Some(v) = self.scene {
            doc.scene = v;
        }
        if let Some(d) = &self.scenes {
            doc.scenes = d.apply(&doc.scenes);
        }
        if let Some(d) = &self.nodes {
            doc.nodes = d.apply(&doc.nodes);
        }
        if let Some(d) = &self.meshes {
            doc.meshes = d.apply(&doc.meshes);
        }
        if let Some(d) = &self.accessors {
            doc.accessors = d.apply(&doc.accessors);
        }
        if let Some(d) = &self.buffer_views {
            doc.buffer_views = d.apply(&doc.buffer_views);
        }
        if let Some(d) = &self.buffers {
            doc.buffers = d.apply(&doc.buffers);
        }
        if let Some(d) = &self.buffer_bytes {
            next.buffers = d.apply(&next.buffers);
        }
        if let Some(d) = &self.materials {
            doc.materials = d.apply(&doc.materials);
        }
        if let Some(d) = &self.textures {
            doc.textures = d.apply(&doc.textures);
        }
        if let Some(d) = &self.images {
            doc.images = d.apply(&doc.images);
        }
        if let Some(d) = &self.samplers {
            doc.samplers = d.apply(&doc.samplers);
        }
        if let Some(d) = &self.skins {
            doc.skins = d.apply(&doc.skins);
        }
        if let Some(d) = &self.animations {
            doc.animations = d.apply(&doc.animations);
        }
        if let Some(d) = &self.cameras {
            doc.cameras = d.apply(&doc.cameras);
        }
        if let Some(v) = &self.extensions_used {
            doc.extensions_used = v.clone();
        }
        if let Some(v) = &self.extensions_required {
            doc.extensions_required = v.clone();
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

    async fn absorb(&mut self, other: Self) {
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
        if other.extensions_used.is_some() {
            self.extensions_used = other.extensions_used;
        }
        if other.extensions_required.is_some() {
            self.extensions_required = other.extensions_required;
        }
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
    async fn inverse(&self, base: &GltfSnapshot) -> Self {
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
            extensions_used: self.extensions_used.as_ref().map(|_| doc.extensions_used.clone()),
            extensions_required: self.extensions_required.as_ref().map(|_| doc.extensions_required.clone()),
            extensions: self.extensions.as_ref().map(|_| doc.extensions.clone()),
            extras: self.extras.as_ref().map(|_| doc.extras.clone()),
            source_form: self.source_form.map(|_| base.source_form),
        }
    }

    async fn between(base: &GltfSnapshot, other: &GltfSnapshot) -> Self {
        let (bd, od) = (&base.document, &other.document);
        let asset_diff = GltfAssetDiff::between(&bd.asset, &od.asset);
        let scenes_diff = GltfScenesDiff::between(&bd.scenes, &od.scenes);
        let nodes_diff = GltfNodesDiff::between(&bd.nodes, &od.nodes);
        let meshes_diff = GltfMeshesDiff::between(&bd.meshes, &od.meshes);
        let accessors_diff = GltfAccessorsDiff::between(&bd.accessors, &od.accessors);
        let buffer_views_diff = GltfBufferViewsDiff::between(&bd.buffer_views, &od.buffer_views);
        let buffers_diff = GltfBuffersDiff::between(&bd.buffers, &od.buffers);
        let buffer_bytes_diff = GltfBufferBytesDiff::between(&base.buffers, &other.buffers);
        let materials_diff = GltfMaterialsDiff::between(&bd.materials, &od.materials);
        let textures_diff = GltfTexturesDiff::between(&bd.textures, &od.textures);
        let images_diff = GltfImagesDiff::between(&bd.images, &od.images);
        let samplers_diff = GltfSamplersDiff::between(&bd.samplers, &od.samplers);
        let skins_diff = GltfSkinsDiff::between(&bd.skins, &od.skins);
        let animations_diff = GltfAnimationsDiff::between(&bd.animations, &od.animations);
        let cameras_diff = GltfCamerasDiff::between(&bd.cameras, &od.cameras);
        Self {
            asset: (!asset_diff.is_empty()).then_some(asset_diff),
            scene: (bd.scene != od.scene).then_some(od.scene),
            scenes: (!scenes_diff.is_empty()).then_some(scenes_diff),
            nodes: (!nodes_diff.is_empty()).then_some(nodes_diff),
            meshes: (!meshes_diff.is_empty()).then_some(meshes_diff),
            accessors: (!accessors_diff.is_empty()).then_some(accessors_diff),
            buffer_views: (!buffer_views_diff.is_empty()).then_some(buffer_views_diff),
            buffers: (!buffers_diff.is_empty()).then_some(buffers_diff),
            buffer_bytes: (!buffer_bytes_diff.is_empty()).then_some(buffer_bytes_diff),
            materials: (!materials_diff.is_empty()).then_some(materials_diff),
            textures: (!textures_diff.is_empty()).then_some(textures_diff),
            images: (!images_diff.is_empty()).then_some(images_diff),
            samplers: (!samplers_diff.is_empty()).then_some(samplers_diff),
            skins: (!skins_diff.is_empty()).then_some(skins_diff),
            animations: (!animations_diff.is_empty()).then_some(animations_diff),
            cameras: (!cameras_diff.is_empty()).then_some(cameras_diff),
            extensions_used: (bd.extensions_used != od.extensions_used).then(|| od.extensions_used.clone()),
            extensions_required: (bd.extensions_required != od.extensions_required).then(|| od.extensions_required.clone()),
            extensions: (bd.extensions != od.extensions).then(|| od.extensions.clone()),
            extras: (bd.extras != od.extras).then(|| od.extras.clone()),
            source_form: (base.source_form != other.source_form).then_some(other.source_form),
        }
    }

    async fn is_empty(&self) -> bool {
        self.is_empty_diff()
    }
}

/// 🧪️ P2-FG3: representative `GltfDiff` cases — the empty (`None`-everywhere) diff PLUS one
/// genuinely rich diff exercising every one of `GltfDiff`'s 21 top-level clauses at once (built
/// via the real `DiffAlgebra::between` over `demo_gltf_snapshot()` vs. a hand-tweaked variant, so
/// every collection's `added`/`modified` entries are real, not fabricated) — used by this
/// artifact's own `diff_grammar_conformance_law`/`protocol_walk_law` conformance tests
/// (⚙️engine/component.rs), mirroring json's own `demo_diff_cases()` role in its pilot report.
pub async fn demo_diff_cases() -> Vec<GltfDiff> {
    let base = crate::artifacts::gltf::engine::demo_gltf_snapshot();
    let mut other = base.clone();
    other.document.asset.generator = Some("semio-fg3".into());
    other.document.scene = Some(1);
    other.document.scenes.push(GltfScene { nodes: vec![], name: Some("second-scene".into()), ..Default::default() });
    other.document.nodes[0].name = Some("renamed-node".into());
    other.document.meshes.push(GltfMesh::default());
    other.document.accessors[0].count = 6;
    other.document.buffer_views.push(GltfBufferView { buffer: 0, byte_offset: 0, byte_length: 12, byte_stride: None, target: None, name: None, extensions: None, extras: None });
    other.document.buffers.push(GltfBuffer { byte_length: 4, uri: None, name: Some("extra".into()), extensions: None, extras: None });
    other.buffers.push(vec![1, 2, 3, 4]);
    other.document.materials[0].double_sided = true;
    other.document.textures.push(GltfTexture { sampler: None, source: None, name: Some("tex2".into()), extensions: None, extras: None });
    other.document.images.push(GltfImage { uri: Some("second.png".into()), ..Default::default() });
    other.document.samplers.push(GltfSampler::default());
    other.document.skins.push(GltfSkin { joints: vec![0], ..Default::default() });
    other.document.animations.push(GltfAnimation::default());
    other.document.cameras.push(GltfCamera {
        projection: GltfCameraProjection::Orthographic(GltfOrthographic { xmag: 1.0, ymag: 1.0, zfar: 10.0, znear: 0.1, extensions: None, extras: None }),
        name: Some("ortho-cam".into()),
        extensions: None,
        extras: None,
    });
    other.document.extensions_used.push("KHR_texture_transform".into());
    other.document.extensions = Some(GltfJson::Bool(true));
    other.document.extras = None;
    other.source_form = GltfSourceForm::Glb;
    let rich = <GltfDiff as DiffAlgebra<GltfSnapshot>>::between(&base, &other);
    vec![GltfDiff::default(), rich]
}

/// 🧩 Builds a set-snapshot diff — sparse field-by-field, never a full-replace slot.
pub async fn diff_set_snapshot(base: &GltfSnapshot, snapshot: &GltfSnapshot) -> GltfDiff {
    <GltfDiff as DiffAlgebra<GltfSnapshot>>::between(base, snapshot)
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
/// GltfSnapshot}` recursively requires `DslField` on `GltfAsset`/`GltfScene`/`GltfNode`/.../
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
pub(crate) async fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
pub(crate) async fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
pub(crate) async fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
pub(crate) async fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
pub(crate) async fn parse_usize(s: &str) -> Result<usize, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}
pub(crate) async fn split_top_level(s: &str, sep: char) -> Vec<&str> {
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
pub(crate) async fn strip_brackets(s: &str) -> Result<&str, String> {
    s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))
}
pub(crate) async fn encode_option<T>(opt: &Option<T>, enc: impl Fn(&T) -> String) -> String {
    match opt {
        None => "[0]".to_string(),
        Some(v) => format!("[1,{}]", enc(v)),
    }
}
pub(crate) async fn decode_option<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Option<T>, String> {
    let inner = strip_brackets(s)?;
    match split_top_level(inner, ',').as_slice() {
        ["0"] => Ok(None),
        [tag, value] if *tag == "1" => Ok(Some(dec(value)?)),
        other => Err(format!("option decode: bad shape {other:?}")),
    }
}
/// 🎲️ `Option<Option<T>>` tri-state -- outer layer is peeled by the CALLER (top-level line
/// tokenizing decides "token present or not" for a change slot); this helper handles the INNER
/// layer when the tri-state value itself is embedded as a single positional field inside a larger
/// bracketed tuple (e.g. one field of `GltfAssetDiff`/`GltfNodeDiff`), where both layers must be
/// explicit since there is no "absent token" to lean on.
pub(crate) async fn encode_option_option<T>(opt: &Option<Option<T>>, enc: impl Fn(&T) -> String) -> String {
    encode_option(opt, |inner: &Option<T>| encode_option(inner, &enc))
}
pub(crate) async fn decode_option_option<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Option<Option<T>>, String> {
    decode_option(s, |inner: &str| decode_option(inner, &dec))
}
//#endregion 🔖️Primitives

//#region 🔖️ScalarCodecs
pub(crate) async fn enc_f64(v: f64) -> String {
    v.to_string()
}
pub(crate) async fn dec_f64(s: &str) -> Result<f64, String> {
    s.parse::<f64>().map_err(|e: std::num::ParseFloatError| e.to_string())
}
pub(crate) async fn enc_u64(v: u64) -> String {
    v.to_string()
}
pub(crate) async fn dec_u64(s: &str) -> Result<u64, String> {
    s.parse::<u64>().map_err(|e: std::num::ParseIntError| e.to_string())
}
pub(crate) async fn enc_bool(v: bool) -> String {
    if v {
        "1".to_string()
    } else {
        "0".to_string()
    }
}
pub(crate) async fn dec_bool(s: &str) -> Result<bool, String> {
    match s {
        "0" => Ok(false),
        "1" => Ok(true),
        other => Err(format!("bool: expected 0/1, got {other:?}")),
    }
}
pub(crate) async fn enc_f64_slice(v: &[f64]) -> String {
    format!("[{}]", v.iter().map(|x| enc_f64(*x)).collect::<Vec<_>>().join(","))
}
pub(crate) async fn dec_f64_vec(s: &str) -> Result<Vec<f64>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_f64).collect()
}
pub(crate) async fn dec_f64_array<const N: usize>(s: &str) -> Result<[f64; N], String> {
    let v = dec_f64_vec(s)?;
    let len = v.len();
    v.try_into().map_err(|_| format!("expected {N} floats, got {len}"))
}
pub(crate) async fn enc_usize_vec(v: &[usize]) -> String {
    format!("[{}]", v.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","))
}
pub(crate) async fn dec_usize_vec(s: &str) -> Result<Vec<usize>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_usize).collect()
}
pub(crate) async fn enc_string_vec(v: &[String]) -> String {
    format!("[{}]", v.iter().map(|s| enc_str(s)).collect::<Vec<_>>().join(","))
}
pub(crate) async fn dec_string_vec(s: &str) -> Result<Vec<String>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_str).collect()
}
/// 🏷️ `GltfPrimitive::attributes` -- `Vec<(String, usize)>`, name-keyed and order-preserving.
pub(crate) async fn enc_attr_pairs(v: &[(String, usize)]) -> String {
    format!("[{}]", v.iter().map(|(k, idx)| format!("{}:{idx}", enc_str(k))).collect::<Vec<_>>().join(","))
}
pub(crate) async fn dec_attr_pairs(s: &str) -> Result<Vec<(String, usize)>, String> {
    split_top_level(strip_brackets(s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (k, v) = entry.split_once(':').ok_or_else(|| format!("attr pair: bad entry {entry:?}"))?;
            Ok((dec_str(k)?, parse_usize(v)?))
        })
        .collect()
}
//#endregion 🔖️ScalarCodecs

//#region 🔖️GltfJsonCodec
/// 🌳 `GltfJson` -- this artifact's own local `extras`/`extensions` value enum (F4). Tag prefix:
/// `Z`=Null (bare, no payload), `B[0|1]`=Bool, `F[<f64>]`=Number, `S[<hex>]`=String,
/// `A[v,v,...]`=Array, `O[k:v,k:v,...]`=Object (member order preserved, matching `GltfJson::
/// Object`'s own `Vec<(String,GltfJson)>` shape rather than a map).
pub(crate) async fn enc_json(v: &GltfJson) -> String {
    match v {
        GltfJson::Null => "Z".to_string(),
        GltfJson::Bool(b) => format!("B[{}]", enc_bool(*b)),
        GltfJson::Number(n) => format!("F[{}]", enc_f64(*n)),
        GltfJson::String(s) => format!("S[{}]", enc_str(s)),
        GltfJson::Array(items) => format!("A[{}]", items.iter().map(enc_json).collect::<Vec<_>>().join(",")),
        GltfJson::Object(members) => {
            format!("O[{}]", members.iter().map(|(k, v)| format!("{}:{}", enc_str(k), enc_json(v))).collect::<Vec<_>>().join(","))
        }
    }
}
pub(crate) async fn dec_json(s: &str) -> Result<GltfJson, String> {
    if s == "Z" {
        return Ok(GltfJson::Null);
    }
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "B" => Ok(GltfJson::Bool(dec_bool(inner)?)),
        "F" => Ok(GltfJson::Number(dec_f64(inner)?)),
        "S" => Ok(GltfJson::String(dec_str(inner)?)),
        "A" => Ok(GltfJson::Array(split_top_level(inner, ',').into_iter().filter(|s| !s.is_empty()).map(dec_json).collect::<Result<Vec<_>, String>>()?)),
        "O" => Ok(GltfJson::Object(
            split_top_level(inner, ',')
                .into_iter()
                .filter(|s| !s.is_empty())
                .map(|entry| {
                    let (k, v) = entry.split_once(':').ok_or_else(|| format!("json object entry: bad {entry:?}"))?;
                    Ok((dec_str(k)?, dec_json(v)?))
                })
                .collect::<Result<Vec<_>, String>>()?,
        )),
        other => Err(format!("json: unknown tag {other:?}")),
    }
}
//#endregion 🔖️GltfJsonCodec

//#region 🔖️UnitEnumCodecs
/// 🔢️ Wire code, not a word tag -- reuses [`GltfComponentType::code`]/`from_code` (the same spec
/// numeric code the JSON serde impl uses, `crate::artifacts::gltf::engine`).
pub(crate) async fn enc_component_type(t: GltfComponentType) -> String {
    t.code().to_string()
}
pub(crate) async fn dec_component_type(s: &str) -> Result<GltfComponentType, String> {
    GltfComponentType::from_code(dec_u64(s)?)
}
/// 🔤️ Word tag -- reuses [`GltfAccessorType::as_str`]/`from_str`.
pub(crate) async fn enc_accessor_type(t: GltfAccessorType) -> String {
    t.as_str().to_string()
}
pub(crate) async fn dec_accessor_type(s: &str) -> Result<GltfAccessorType, String> {
    GltfAccessorType::from_str(s)
}
pub(crate) async fn enc_alpha_mode(m: GltfAlphaMode) -> String {
    match m {
        GltfAlphaMode::Opaque => "OPAQUE",
        GltfAlphaMode::Mask => "MASK",
        GltfAlphaMode::Blend => "BLEND",
    }
    .to_string()
}
pub(crate) async fn dec_alpha_mode(s: &str) -> Result<GltfAlphaMode, String> {
    match s {
        "OPAQUE" => Ok(GltfAlphaMode::Opaque),
        "MASK" => Ok(GltfAlphaMode::Mask),
        "BLEND" => Ok(GltfAlphaMode::Blend),
        other => Err(format!("alpha mode: unknown {other:?}")),
    }
}
pub(crate) async fn enc_interpolation(i: GltfInterpolation) -> String {
    match i {
        GltfInterpolation::Linear => "LINEAR",
        GltfInterpolation::Step => "STEP",
        GltfInterpolation::CubicSpline => "CUBICSPLINE",
    }
    .to_string()
}
pub(crate) async fn dec_interpolation(s: &str) -> Result<GltfInterpolation, String> {
    match s {
        "LINEAR" => Ok(GltfInterpolation::Linear),
        "STEP" => Ok(GltfInterpolation::Step),
        "CUBICSPLINE" => Ok(GltfInterpolation::CubicSpline),
        other => Err(format!("interpolation: unknown {other:?}")),
    }
}
pub(crate) async fn enc_animation_path(p: GltfAnimationPath) -> String {
    match p {
        GltfAnimationPath::Translation => "translation",
        GltfAnimationPath::Rotation => "rotation",
        GltfAnimationPath::Scale => "scale",
        GltfAnimationPath::Weights => "weights",
    }
    .to_string()
}
pub(crate) async fn dec_animation_path(s: &str) -> Result<GltfAnimationPath, String> {
    match s {
        "translation" => Ok(GltfAnimationPath::Translation),
        "rotation" => Ok(GltfAnimationPath::Rotation),
        "scale" => Ok(GltfAnimationPath::Scale),
        "weights" => Ok(GltfAnimationPath::Weights),
        other => Err(format!("animation path: unknown {other:?}")),
    }
}
pub(crate) async fn enc_source_form(f: GltfSourceForm) -> String {
    match f {
        GltfSourceForm::Json => "json",
        GltfSourceForm::Glb => "glb",
    }
    .to_string()
}
pub(crate) async fn dec_source_form(s: &str) -> Result<GltfSourceForm, String> {
    match s {
        "json" => Ok(GltfSourceForm::Json),
        "glb" => Ok(GltfSourceForm::Glb),
        other => Err(format!("source form: unknown {other:?}")),
    }
}
//#endregion 🔖️UnitEnumCodecs

//#region 🔖️AssetSceneNodeGroupCodecs
pub(crate) async fn enc_asset_diff(d: &GltfAssetDiff) -> String {
    format!(
        "[{},{},{},{},{},{}]",
        encode_option(&d.version, |v| enc_str(v)),
        encode_option_option(&d.generator, |v| enc_str(v)),
        encode_option_option(&d.copyright, |v| enc_str(v)),
        encode_option_option(&d.min_version, |v| enc_str(v)),
        encode_option_option(&d.extensions, enc_json),
        encode_option_option(&d.extras, enc_json),
    )
}
pub(crate) async fn dec_asset_diff(s: &str) -> Result<GltfAssetDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [version, generator, copyright, min_version, extensions, extras] = parts.as_slice() else {
        return Err(format!("asset diff: expected 6 fields, got {}", parts.len()));
    };
    Ok(GltfAssetDiff {
        version: decode_option(version, dec_str)?,
        generator: decode_option_option(generator, dec_str)?,
        copyright: decode_option_option(copyright, dec_str)?,
        min_version: decode_option_option(min_version, dec_str)?,
        extensions: decode_option_option(extensions, dec_json)?,
        extras: decode_option_option(extras, dec_json)?,
    })
}

pub(crate) async fn enc_scene(sc: &GltfScene) -> String {
    format!("[{},{},{},{}]", enc_usize_vec(&sc.nodes), encode_option(&sc.name, |v| enc_str(v)), encode_option(&sc.extensions, enc_json), encode_option(&sc.extras, enc_json),)
}
pub(crate) async fn dec_scene(s: &str) -> Result<GltfScene, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [nodes, name, extensions, extras] = parts.as_slice() else { return Err(format!("scene: expected 4 fields, got {}", parts.len())) };
    Ok(GltfScene { nodes: dec_usize_vec(nodes)?, name: decode_option(name, dec_str)?, extensions: decode_option(extensions, dec_json)?, extras: decode_option(extras, dec_json)? })
}
pub(crate) async fn enc_scene_diff(d: &GltfSceneDiff) -> String {
    format!("[{},{},{},{}]", encode_option(&d.nodes, |v| enc_usize_vec(v)), encode_option_option(&d.name, |v| enc_str(v)), encode_option_option(&d.extensions, enc_json), encode_option_option(&d.extras, enc_json),)
}
pub(crate) async fn dec_scene_diff(s: &str) -> Result<GltfSceneDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [nodes, name, extensions, extras] = parts.as_slice() else { return Err(format!("scene diff: expected 4 fields, got {}", parts.len())) };
    Ok(GltfSceneDiff { nodes: decode_option(nodes, dec_usize_vec)?, name: decode_option_option(name, dec_str)?, extensions: decode_option_option(extensions, dec_json)?, extras: decode_option_option(extras, dec_json)? })
}

pub(crate) async fn enc_node(n: &GltfNode) -> String {
    format!(
        "[{},{},{},{},{},{},{},{},{},{},{},{}]",
        enc_usize_vec(&n.children),
        encode_option(&n.mesh, |v| v.to_string()),
        encode_option(&n.camera, |v| v.to_string()),
        encode_option(&n.skin, |v| v.to_string()),
        encode_option(&n.matrix, |v| enc_f64_slice(v)),
        encode_option(&n.translation, |v| enc_f64_slice(v)),
        encode_option(&n.rotation, |v| enc_f64_slice(v)),
        encode_option(&n.scale, |v| enc_f64_slice(v)),
        enc_f64_slice(&n.weights),
        encode_option(&n.name, |v| enc_str(v)),
        encode_option(&n.extensions, enc_json),
        encode_option(&n.extras, enc_json),
    )
}
pub(crate) async fn dec_node(s: &str) -> Result<GltfNode, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [children, mesh, camera, skin, matrix, translation, rotation, scale, weights, name, extensions, extras] = parts.as_slice() else {
        return Err(format!("node: expected 12 fields, got {}", parts.len()));
    };
    Ok(GltfNode {
        children: dec_usize_vec(children)?,
        mesh: decode_option(mesh, parse_usize)?,
        camera: decode_option(camera, parse_usize)?,
        skin: decode_option(skin, parse_usize)?,
        matrix: decode_option(matrix, dec_f64_array::<16>)?,
        translation: decode_option(translation, dec_f64_array::<3>)?,
        rotation: decode_option(rotation, dec_f64_array::<4>)?,
        scale: decode_option(scale, dec_f64_array::<3>)?,
        weights: dec_f64_vec(weights)?,
        name: decode_option(name, dec_str)?,
        extensions: decode_option(extensions, dec_json)?,
        extras: decode_option(extras, dec_json)?,
    })
}
pub(crate) async fn enc_node_diff(d: &GltfNodeDiff) -> String {
    format!(
        "[{},{},{},{},{},{},{},{},{},{},{},{}]",
        encode_option(&d.children, |v| enc_usize_vec(v)),
        encode_option_option(&d.mesh, |v| v.to_string()),
        encode_option_option(&d.camera, |v| v.to_string()),
        encode_option_option(&d.skin, |v| v.to_string()),
        encode_option_option(&d.matrix, |v| enc_f64_slice(v)),
        encode_option_option(&d.translation, |v| enc_f64_slice(v)),
        encode_option_option(&d.rotation, |v| enc_f64_slice(v)),
        encode_option_option(&d.scale, |v| enc_f64_slice(v)),
        encode_option(&d.weights, |v| enc_f64_slice(v)),
        encode_option_option(&d.name, |v| enc_str(v)),
        encode_option_option(&d.extensions, enc_json),
        encode_option_option(&d.extras, enc_json),
    )
}
pub(crate) async fn dec_node_diff(s: &str) -> Result<GltfNodeDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [children, mesh, camera, skin, matrix, translation, rotation, scale, weights, name, extensions, extras] = parts.as_slice() else {
        return Err(format!("node diff: expected 12 fields, got {}", parts.len()));
    };
    Ok(GltfNodeDiff {
        children: decode_option(children, dec_usize_vec)?,
        mesh: decode_option_option(mesh, parse_usize)?,
        camera: decode_option_option(camera, parse_usize)?,
        skin: decode_option_option(skin, parse_usize)?,
        matrix: decode_option_option(matrix, dec_f64_array::<16>)?,
        translation: decode_option_option(translation, dec_f64_array::<3>)?,
        rotation: decode_option_option(rotation, dec_f64_array::<4>)?,
        scale: decode_option_option(scale, dec_f64_array::<3>)?,
        weights: decode_option(weights, dec_f64_vec)?,
        name: decode_option_option(name, dec_str)?,
        extensions: decode_option_option(extensions, dec_json)?,
        extras: decode_option_option(extras, dec_json)?,
    })
}
//#endregion 🔖️AssetSceneNodeGroupCodecs

//#region 🔖️MeshAccessorMaterialGroupCodecs
pub(crate) async fn enc_primitive(p: &GltfPrimitive) -> String {
    format!(
        "[{},{},{},{},{},{},{}]",
        enc_attr_pairs(&p.attributes),
        encode_option(&p.indices, |v| v.to_string()),
        encode_option(&p.material, |v| v.to_string()),
        encode_option(&p.mode, |v| enc_u64(*v)),
        format!("[{}]", p.targets.iter().map(|target| enc_attr_pairs(&target.0)).collect::<Vec<_>>().join(",")),
        encode_option(&p.extensions, enc_json),
        encode_option(&p.extras, enc_json),
    )
}
pub(crate) async fn dec_primitive(s: &str) -> Result<GltfPrimitive, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [attributes, indices, material, mode, targets, extensions, extras] = parts.as_slice() else {
        return Err(format!("primitive: expected 7 fields, got {}", parts.len()));
    };
    Ok(GltfPrimitive {
        attributes: dec_attr_pairs(attributes)?,
        indices: decode_option(indices, parse_usize)?,
        material: decode_option(material, parse_usize)?,
        mode: decode_option(mode, dec_u64)?,
        targets: split_top_level(strip_brackets(targets)?, ',').into_iter().filter(|value| !value.is_empty()).map(|value| dec_attr_pairs(&value).map(GltfMorphTarget)).collect::<Result<Vec<_>, _>>()?,
        extensions: decode_option(extensions, dec_json)?,
        extras: decode_option(extras, dec_json)?,
    })
}
pub(crate) async fn enc_primitive_vec(v: &[GltfPrimitive]) -> String {
    format!("[{}]", v.iter().map(enc_primitive).collect::<Vec<_>>().join(","))
}
pub(crate) async fn dec_primitive_vec(s: &str) -> Result<Vec<GltfPrimitive>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_primitive).collect()
}
pub(crate) async fn enc_mesh(m: &GltfMesh) -> String {
    format!("[{},{},{},{},{}]", enc_primitive_vec(&m.primitives), enc_f64_slice(&m.weights), encode_option(&m.name, |v| enc_str(v)), encode_option(&m.extensions, enc_json), encode_option(&m.extras, enc_json),)
}
pub(crate) async fn dec_mesh(s: &str) -> Result<GltfMesh, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [primitives, weights, name, extensions, extras] = parts.as_slice() else { return Err(format!("mesh: expected 5 fields, got {}", parts.len())) };
    Ok(GltfMesh { primitives: dec_primitive_vec(primitives)?, weights: dec_f64_vec(weights)?, name: decode_option(name, dec_str)?, extensions: decode_option(extensions, dec_json)?, extras: decode_option(extras, dec_json)? })
}
pub(crate) async fn enc_mesh_diff(d: &GltfMeshDiff) -> String {
    format!(
        "[{},{},{},{},{}]",
        encode_option(&d.primitives, |v| enc_primitive_vec(v)),
        encode_option(&d.weights, |v| enc_f64_slice(v)),
        encode_option_option(&d.name, |v| enc_str(v)),
        encode_option_option(&d.extensions, enc_json),
        encode_option_option(&d.extras, enc_json),
    )
}
pub(crate) async fn dec_mesh_diff(s: &str) -> Result<GltfMeshDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [primitives, weights, name, extensions, extras] = parts.as_slice() else { return Err(format!("mesh diff: expected 5 fields, got {}", parts.len())) };
    Ok(GltfMeshDiff {
        primitives: decode_option(primitives, dec_primitive_vec)?,
        weights: decode_option(weights, dec_f64_vec)?,
        name: decode_option_option(name, dec_str)?,
        extensions: decode_option_option(extensions, dec_json)?,
        extras: decode_option_option(extras, dec_json)?,
    })
}

pub(crate) async fn enc_sparse_indices(v: &GltfSparseIndices) -> String {
    format!("[{},{},{}]", v.buffer_view, v.byte_offset, enc_component_type(v.component_type))
}
pub(crate) async fn dec_sparse_indices(s: &str) -> Result<GltfSparseIndices, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [buffer_view, byte_offset, component_type] = parts.as_slice() else { return Err(format!("sparse indices: expected 3 fields, got {}", parts.len())) };
    Ok(GltfSparseIndices { buffer_view: parse_usize(buffer_view)?, byte_offset: parse_usize(byte_offset)?, component_type: dec_component_type(component_type)? })
}
pub(crate) async fn enc_sparse_values(v: &GltfSparseValues) -> String {
    format!("[{},{}]", v.buffer_view, v.byte_offset)
}
pub(crate) async fn dec_sparse_values(s: &str) -> Result<GltfSparseValues, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [buffer_view, byte_offset] = parts.as_slice() else { return Err(format!("sparse values: expected 2 fields, got {}", parts.len())) };
    Ok(GltfSparseValues { buffer_view: parse_usize(buffer_view)?, byte_offset: parse_usize(byte_offset)? })
}
pub(crate) async fn enc_sparse_accessor(v: &GltfSparseAccessor) -> String {
    format!("[{},{},{}]", v.count, enc_sparse_indices(&v.indices), enc_sparse_values(&v.values))
}
pub(crate) async fn dec_sparse_accessor(s: &str) -> Result<GltfSparseAccessor, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [count, indices, values] = parts.as_slice() else { return Err(format!("sparse accessor: expected 3 fields, got {}", parts.len())) };
    Ok(GltfSparseAccessor { count: parse_usize(count)?, indices: dec_sparse_indices(indices)?, values: dec_sparse_values(values)? })
}
pub(crate) async fn enc_accessor(a: &GltfAccessor) -> String {
    format!(
        "[{},{},{},{},{},{},{},{},{},{},{},{}]",
        encode_option(&a.buffer_view, |v| v.to_string()),
        a.byte_offset,
        enc_component_type(a.component_type),
        enc_bool(a.normalized),
        a.count,
        enc_accessor_type(a.kind),
        encode_option(&a.max, |v| enc_f64_slice(v)),
        encode_option(&a.min, |v| enc_f64_slice(v)),
        encode_option(&a.sparse, enc_sparse_accessor),
        encode_option(&a.name, |v| enc_str(v)),
        encode_option(&a.extensions, enc_json),
        encode_option(&a.extras, enc_json),
    )
}
pub(crate) async fn dec_accessor(s: &str) -> Result<GltfAccessor, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [buffer_view, byte_offset, component_type, normalized, count, kind, max, min, sparse, name, extensions, extras] = parts.as_slice() else {
        return Err(format!("accessor: expected 12 fields, got {}", parts.len()));
    };
    Ok(GltfAccessor {
        buffer_view: decode_option(buffer_view, parse_usize)?,
        byte_offset: parse_usize(byte_offset)?,
        component_type: dec_component_type(component_type)?,
        normalized: dec_bool(normalized)?,
        count: parse_usize(count)?,
        kind: dec_accessor_type(kind)?,
        max: decode_option(max, dec_f64_vec)?,
        min: decode_option(min, dec_f64_vec)?,
        sparse: decode_option(sparse, dec_sparse_accessor)?,
        name: decode_option(name, dec_str)?,
        extensions: decode_option(extensions, dec_json)?,
        extras: decode_option(extras, dec_json)?,
    })
}
pub(crate) async fn enc_accessor_diff(d: &GltfAccessorDiff) -> String {
    format!(
        "[{},{},{},{},{},{},{},{},{},{},{},{}]",
        encode_option_option(&d.buffer_view, |v| v.to_string()),
        encode_option(&d.byte_offset, |v| v.to_string()),
        encode_option(&d.component_type, |v| enc_component_type(*v)),
        encode_option(&d.normalized, |v| enc_bool(*v)),
        encode_option(&d.count, |v| v.to_string()),
        encode_option(&d.kind, |v| enc_accessor_type(*v)),
        encode_option_option(&d.max, |v| enc_f64_slice(v)),
        encode_option_option(&d.min, |v| enc_f64_slice(v)),
        encode_option_option(&d.sparse, enc_sparse_accessor),
        encode_option_option(&d.name, |v| enc_str(v)),
        encode_option_option(&d.extensions, enc_json),
        encode_option_option(&d.extras, enc_json),
    )
}
pub(crate) async fn dec_accessor_diff(s: &str) -> Result<GltfAccessorDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [buffer_view, byte_offset, component_type, normalized, count, kind, max, min, sparse, name, extensions, extras] = parts.as_slice() else {
        return Err(format!("accessor diff: expected 12 fields, got {}", parts.len()));
    };
    Ok(GltfAccessorDiff {
        buffer_view: decode_option_option(buffer_view, parse_usize)?,
        byte_offset: decode_option(byte_offset, parse_usize)?,
        component_type: decode_option(component_type, dec_component_type)?,
        normalized: decode_option(normalized, dec_bool)?,
        count: decode_option(count, parse_usize)?,
        kind: decode_option(kind, dec_accessor_type)?,
        max: decode_option_option(max, dec_f64_vec)?,
        min: decode_option_option(min, dec_f64_vec)?,
        sparse: decode_option_option(sparse, dec_sparse_accessor)?,
        name: decode_option_option(name, dec_str)?,
        extensions: decode_option_option(extensions, dec_json)?,
        extras: decode_option_option(extras, dec_json)?,
    })
}

pub(crate) async fn enc_texture_info(v: &GltfTextureInfo) -> String {
    format!("[{},{},{},{}]", v.index, enc_u64(v.tex_coord), encode_option(&v.extensions, enc_json), encode_option(&v.extras, enc_json))
}
pub(crate) async fn dec_texture_info(s: &str) -> Result<GltfTextureInfo, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [index, tex_coord, extensions, extras] = parts.as_slice() else { return Err(format!("texture info: expected 4 fields, got {}", parts.len())) };
    Ok(GltfTextureInfo { index: parse_usize(index)?, tex_coord: dec_u64(tex_coord)?, extensions: decode_option(extensions, dec_json)?, extras: decode_option(extras, dec_json)? })
}
pub(crate) async fn enc_normal_texture_info(v: &GltfNormalTextureInfo) -> String {
    format!("[{},{},{},{},{}]", v.index, enc_u64(v.tex_coord), enc_f64(v.scale), encode_option(&v.extensions, enc_json), encode_option(&v.extras, enc_json))
}
pub(crate) async fn dec_normal_texture_info(s: &str) -> Result<GltfNormalTextureInfo, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [index, tex_coord, scale, extensions, extras] = parts.as_slice() else { return Err(format!("normal texture info: expected 5 fields, got {}", parts.len())) };
    Ok(GltfNormalTextureInfo { index: parse_usize(index)?, tex_coord: dec_u64(tex_coord)?, scale: dec_f64(scale)?, extensions: decode_option(extensions, dec_json)?, extras: decode_option(extras, dec_json)? })
}
pub(crate) async fn enc_occlusion_texture_info(v: &GltfOcclusionTextureInfo) -> String {
    format!("[{},{},{},{},{}]", v.index, enc_u64(v.tex_coord), enc_f64(v.strength), encode_option(&v.extensions, enc_json), encode_option(&v.extras, enc_json))
}
pub(crate) async fn dec_occlusion_texture_info(s: &str) -> Result<GltfOcclusionTextureInfo, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [index, tex_coord, strength, extensions, extras] = parts.as_slice() else { return Err(format!("occlusion texture info: expected 5 fields, got {}", parts.len())) };
    Ok(GltfOcclusionTextureInfo { index: parse_usize(index)?, tex_coord: dec_u64(tex_coord)?, strength: dec_f64(strength)?, extensions: decode_option(extensions, dec_json)?, extras: decode_option(extras, dec_json)? })
}
pub(crate) async fn enc_pbr(v: &GltfPbrMetallicRoughness) -> String {
    format!(
        "[{},{},{},{},{},{},{}]",
        enc_f64_slice(&v.base_color_factor),
        encode_option(&v.base_color_texture, enc_texture_info),
        enc_f64(v.metallic_factor),
        enc_f64(v.roughness_factor),
        encode_option(&v.metallic_roughness_texture, enc_texture_info),
        encode_option(&v.extensions, enc_json),
        encode_option(&v.extras, enc_json),
    )
}
pub(crate) async fn dec_pbr(s: &str) -> Result<GltfPbrMetallicRoughness, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [base_color_factor, base_color_texture, metallic_factor, roughness_factor, metallic_roughness_texture, extensions, extras] = parts.as_slice() else {
        return Err(format!("pbr: expected 7 fields, got {}", parts.len()));
    };
    Ok(GltfPbrMetallicRoughness {
        base_color_factor: dec_f64_array::<4>(base_color_factor)?,
        base_color_texture: decode_option(base_color_texture, dec_texture_info)?,
        metallic_factor: dec_f64(metallic_factor)?,
        roughness_factor: dec_f64(roughness_factor)?,
        metallic_roughness_texture: decode_option(metallic_roughness_texture, dec_texture_info)?,
        extensions: decode_option(extensions, dec_json)?,
        extras: decode_option(extras, dec_json)?,
    })
}
pub(crate) async fn enc_material(m: &GltfMaterial) -> String {
    format!(
        "[{},{},{},{},{},{},{},{},{},{},{}]",
        encode_option(&m.name, |v| enc_str(v)),
        encode_option(&m.pbr_metallic_roughness, enc_pbr),
        encode_option(&m.normal_texture, enc_normal_texture_info),
        encode_option(&m.occlusion_texture, enc_occlusion_texture_info),
        encode_option(&m.emissive_texture, enc_texture_info),
        enc_f64_slice(&m.emissive_factor),
        enc_alpha_mode(m.alpha_mode),
        enc_f64(m.alpha_cutoff),
        enc_bool(m.double_sided),
        encode_option(&m.extensions, enc_json),
        encode_option(&m.extras, enc_json),
    )
}
pub(crate) async fn dec_material(s: &str) -> Result<GltfMaterial, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, pbr, normal_texture, occlusion_texture, emissive_texture, emissive_factor, alpha_mode, alpha_cutoff, double_sided, extensions, extras] = parts.as_slice() else {
        return Err(format!("material: expected 11 fields, got {}", parts.len()));
    };
    Ok(GltfMaterial {
        name: decode_option(name, dec_str)?,
        pbr_metallic_roughness: decode_option(pbr, dec_pbr)?,
        normal_texture: decode_option(normal_texture, dec_normal_texture_info)?,
        occlusion_texture: decode_option(occlusion_texture, dec_occlusion_texture_info)?,
        emissive_texture: decode_option(emissive_texture, dec_texture_info)?,
        emissive_factor: dec_f64_array::<3>(emissive_factor)?,
        alpha_mode: dec_alpha_mode(alpha_mode)?,
        alpha_cutoff: dec_f64(alpha_cutoff)?,
        double_sided: dec_bool(double_sided)?,
        extensions: decode_option(extensions, dec_json)?,
        extras: decode_option(extras, dec_json)?,
    })
}
pub(crate) async fn enc_material_diff(d: &GltfMaterialDiff) -> String {
    format!(
        "[{},{},{},{},{},{},{},{},{},{},{}]",
        encode_option_option(&d.name, |v| enc_str(v)),
        encode_option_option(&d.pbr_metallic_roughness, enc_pbr),
        encode_option_option(&d.normal_texture, enc_normal_texture_info),
        encode_option_option(&d.occlusion_texture, enc_occlusion_texture_info),
        encode_option_option(&d.emissive_texture, enc_texture_info),
        encode_option(&d.emissive_factor, |v| enc_f64_slice(v)),
        encode_option(&d.alpha_mode, |v| enc_alpha_mode(*v)),
        encode_option(&d.alpha_cutoff, |v| enc_f64(*v)),
        encode_option(&d.double_sided, |v| enc_bool(*v)),
        encode_option_option(&d.extensions, enc_json),
        encode_option_option(&d.extras, enc_json),
    )
}
pub(crate) async fn dec_material_diff(s: &str) -> Result<GltfMaterialDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, pbr, normal_texture, occlusion_texture, emissive_texture, emissive_factor, alpha_mode, alpha_cutoff, double_sided, extensions, extras] = parts.as_slice() else {
        return Err(format!("material diff: expected 11 fields, got {}", parts.len()));
    };
    Ok(GltfMaterialDiff {
        name: decode_option_option(name, dec_str)?,
        pbr_metallic_roughness: decode_option_option(pbr, dec_pbr)?,
        normal_texture: decode_option_option(normal_texture, dec_normal_texture_info)?,
        occlusion_texture: decode_option_option(occlusion_texture, dec_occlusion_texture_info)?,
        emissive_texture: decode_option_option(emissive_texture, dec_texture_info)?,
        emissive_factor: decode_option(emissive_factor, dec_f64_array::<3>)?,
        alpha_mode: decode_option(alpha_mode, dec_alpha_mode)?,
        alpha_cutoff: decode_option(alpha_cutoff, dec_f64)?,
        double_sided: decode_option(double_sided, dec_bool)?,
        extensions: decode_option_option(extensions, dec_json)?,
        extras: decode_option_option(extras, dec_json)?,
    })
}
//#endregion 🔖️MeshAccessorMaterialGroupCodecs

//#region 🔖️BufferGroupCodecs
pub(crate) async fn enc_buffer(b: &GltfBuffer) -> String {
    format!("[{},{},{},{},{}]", b.byte_length, encode_option(&b.uri, |v| enc_str(v)), encode_option(&b.name, |v| enc_str(v)), encode_option(&b.extensions, enc_json), encode_option(&b.extras, enc_json))
}
pub(crate) async fn dec_buffer(s: &str) -> Result<GltfBuffer, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [byte_length, uri, name, extensions, extras] = parts.as_slice() else { return Err(format!("buffer: expected 5 fields, got {}", parts.len())) };
    Ok(GltfBuffer { byte_length: parse_usize(byte_length)?, uri: decode_option(uri, dec_str)?, name: decode_option(name, dec_str)?, extensions: decode_option(extensions, dec_json)?, extras: decode_option(extras, dec_json)? })
}
pub(crate) async fn enc_buffer_diff(d: &GltfBufferDiff) -> String {
    format!(
        "[{},{},{},{},{}]",
        encode_option(&d.byte_length, |v| v.to_string()),
        encode_option_option(&d.uri, |v| enc_str(v)),
        encode_option_option(&d.name, |v| enc_str(v)),
        encode_option_option(&d.extensions, enc_json),
        encode_option_option(&d.extras, enc_json),
    )
}
pub(crate) async fn dec_buffer_diff(s: &str) -> Result<GltfBufferDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [byte_length, uri, name, extensions, extras] = parts.as_slice() else { return Err(format!("buffer diff: expected 5 fields, got {}", parts.len())) };
    Ok(GltfBufferDiff {
        byte_length: decode_option(byte_length, parse_usize)?,
        uri: decode_option_option(uri, dec_str)?,
        name: decode_option_option(name, dec_str)?,
        extensions: decode_option_option(extensions, dec_json)?,
        extras: decode_option_option(extras, dec_json)?,
    })
}
pub(crate) async fn enc_buffer_view(v: &GltfBufferView) -> String {
    format!(
        "[{},{},{},{},{},{},{},{}]",
        v.buffer,
        v.byte_offset,
        v.byte_length,
        encode_option(&v.byte_stride, |x| x.to_string()),
        encode_option(&v.target, |x| enc_u64(*x)),
        encode_option(&v.name, |x| enc_str(x)),
        encode_option(&v.extensions, enc_json),
        encode_option(&v.extras, enc_json),
    )
}
pub(crate) async fn dec_buffer_view(s: &str) -> Result<GltfBufferView, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [buffer, byte_offset, byte_length, byte_stride, target, name, extensions, extras] = parts.as_slice() else {
        return Err(format!("buffer view: expected 8 fields, got {}", parts.len()));
    };
    Ok(GltfBufferView {
        buffer: parse_usize(buffer)?,
        byte_offset: parse_usize(byte_offset)?,
        byte_length: parse_usize(byte_length)?,
        byte_stride: decode_option(byte_stride, parse_usize)?,
        target: decode_option(target, dec_u64)?,
        name: decode_option(name, dec_str)?,
        extensions: decode_option(extensions, dec_json)?,
        extras: decode_option(extras, dec_json)?,
    })
}
/// 🧬️ Raw buffer bytes (`GltfSnapshot::buffers[i]`) -- hex, same as every other byte payload in
/// this grammar (no base64: no external dep, matching the family's own hex idiom, see
/// `f6-recon-report.md` §5).
pub(crate) async fn enc_bytes(v: &[u8]) -> String {
    hex_encode(v)
}
pub(crate) async fn dec_bytes(s: &str) -> Result<Vec<u8>, String> {
    hex_decode(s)
}
//#endregion 🔖️BufferGroupCodecs

//#region 🔖️TextureImageSamplerSkinGroupCodecs
pub(crate) async fn enc_texture(t: &GltfTexture) -> String {
    format!("[{},{},{},{},{}]", encode_option(&t.sampler, |v| v.to_string()), encode_option(&t.source, |v| v.to_string()), encode_option(&t.name, |v| enc_str(v)), encode_option(&t.extensions, enc_json), encode_option(&t.extras, enc_json),)
}
pub(crate) async fn dec_texture(s: &str) -> Result<GltfTexture, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [sampler, source, name, extensions, extras] = parts.as_slice() else { return Err(format!("texture: expected 5 fields, got {}", parts.len())) };
    Ok(GltfTexture { sampler: decode_option(sampler, parse_usize)?, source: decode_option(source, parse_usize)?, name: decode_option(name, dec_str)?, extensions: decode_option(extensions, dec_json)?, extras: decode_option(extras, dec_json)? })
}
pub(crate) async fn enc_image(i: &GltfImage) -> String {
    format!(
        "[{},{},{},{},{},{}]",
        encode_option(&i.uri, |v| enc_str(v)),
        encode_option(&i.mime_type, |v| enc_str(v)),
        encode_option(&i.buffer_view, |v| v.to_string()),
        encode_option(&i.name, |v| enc_str(v)),
        encode_option(&i.extensions, enc_json),
        encode_option(&i.extras, enc_json),
    )
}
pub(crate) async fn dec_image(s: &str) -> Result<GltfImage, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [uri, mime_type, buffer_view, name, extensions, extras] = parts.as_slice() else { return Err(format!("image: expected 6 fields, got {}", parts.len())) };
    Ok(GltfImage {
        uri: decode_option(uri, dec_str)?,
        mime_type: decode_option(mime_type, dec_str)?,
        buffer_view: decode_option(buffer_view, parse_usize)?,
        name: decode_option(name, dec_str)?,
        extensions: decode_option(extensions, dec_json)?,
        extras: decode_option(extras, dec_json)?,
    })
}
pub(crate) async fn enc_sampler(s: &GltfSampler) -> String {
    format!(
        "[{},{},{},{},{},{},{}]",
        encode_option(&s.mag_filter, |v| enc_u64(*v)),
        encode_option(&s.min_filter, |v| enc_u64(*v)),
        enc_u64(s.wrap_s),
        enc_u64(s.wrap_t),
        encode_option(&s.name, |v| enc_str(v)),
        encode_option(&s.extensions, enc_json),
        encode_option(&s.extras, enc_json),
    )
}
pub(crate) async fn dec_sampler(s: &str) -> Result<GltfSampler, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [mag_filter, min_filter, wrap_s, wrap_t, name, extensions, extras] = parts.as_slice() else {
        return Err(format!("sampler: expected 7 fields, got {}", parts.len()));
    };
    Ok(GltfSampler {
        mag_filter: decode_option(mag_filter, dec_u64)?,
        min_filter: decode_option(min_filter, dec_u64)?,
        wrap_s: dec_u64(wrap_s)?,
        wrap_t: dec_u64(wrap_t)?,
        name: decode_option(name, dec_str)?,
        extensions: decode_option(extensions, dec_json)?,
        extras: decode_option(extras, dec_json)?,
    })
}
pub(crate) async fn enc_skin(v: &GltfSkin) -> String {
    format!(
        "[{},{},{},{},{},{}]",
        encode_option(&v.inverse_bind_matrices, |x| x.to_string()),
        encode_option(&v.skeleton, |x| x.to_string()),
        enc_usize_vec(&v.joints),
        encode_option(&v.name, |x| enc_str(x)),
        encode_option(&v.extensions, enc_json),
        encode_option(&v.extras, enc_json),
    )
}
pub(crate) async fn dec_skin(s: &str) -> Result<GltfSkin, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [inverse_bind_matrices, skeleton, joints, name, extensions, extras] = parts.as_slice() else {
        return Err(format!("skin: expected 6 fields, got {}", parts.len()));
    };
    Ok(GltfSkin {
        inverse_bind_matrices: decode_option(inverse_bind_matrices, parse_usize)?,
        skeleton: decode_option(skeleton, parse_usize)?,
        joints: dec_usize_vec(joints)?,
        name: decode_option(name, dec_str)?,
        extensions: decode_option(extensions, dec_json)?,
        extras: decode_option(extras, dec_json)?,
    })
}
//#endregion 🔖️TextureImageSamplerSkinGroupCodecs

//#region 🔖️AnimationGroupCodecs
pub(crate) async fn enc_animation_channel_target(t: &GltfAnimationChannelTarget) -> String {
    format!("[{},{},{},{}]", encode_option(&t.node, |v| v.to_string()), enc_animation_path(t.path), encode_option(&t.extensions, enc_json), encode_option(&t.extras, enc_json))
}
pub(crate) async fn dec_animation_channel_target(s: &str) -> Result<GltfAnimationChannelTarget, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [node, path, extensions, extras] = parts.as_slice() else { return Err(format!("animation channel target: expected 4 fields, got {}", parts.len())) };
    Ok(GltfAnimationChannelTarget { node: decode_option(node, parse_usize)?, path: dec_animation_path(path)?, extensions: decode_option(extensions, dec_json)?, extras: decode_option(extras, dec_json)? })
}
pub(crate) async fn enc_animation_channel(c: &GltfAnimationChannel) -> String {
    format!("[{},{},{},{}]", c.sampler, enc_animation_channel_target(&c.target), encode_option(&c.extensions, enc_json), encode_option(&c.extras, enc_json))
}
pub(crate) async fn dec_animation_channel(s: &str) -> Result<GltfAnimationChannel, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [sampler, target, extensions, extras] = parts.as_slice() else { return Err(format!("animation channel: expected 4 fields, got {}", parts.len())) };
    Ok(GltfAnimationChannel { sampler: parse_usize(sampler)?, target: dec_animation_channel_target(target)?, extensions: decode_option(extensions, dec_json)?, extras: decode_option(extras, dec_json)? })
}
pub(crate) async fn enc_animation_sampler(s: &GltfAnimationSampler) -> String {
    format!("[{},{},{},{},{}]", s.input, enc_interpolation(s.interpolation), s.output, encode_option(&s.extensions, enc_json), encode_option(&s.extras, enc_json))
}
pub(crate) async fn dec_animation_sampler(s: &str) -> Result<GltfAnimationSampler, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [input, interpolation, output, extensions, extras] = parts.as_slice() else { return Err(format!("animation sampler: expected 5 fields, got {}", parts.len())) };
    Ok(GltfAnimationSampler { input: parse_usize(input)?, interpolation: dec_interpolation(interpolation)?, output: parse_usize(output)?, extensions: decode_option(extensions, dec_json)?, extras: decode_option(extras, dec_json)? })
}
pub(crate) async fn enc_animation(a: &GltfAnimation) -> String {
    format!(
        "[{},{},{},{},{}]",
        format!("[{}]", a.channels.iter().map(enc_animation_channel).collect::<Vec<_>>().join(",")),
        format!("[{}]", a.samplers.iter().map(enc_animation_sampler).collect::<Vec<_>>().join(",")),
        encode_option(&a.name, |v| enc_str(v)),
        encode_option(&a.extensions, enc_json),
        encode_option(&a.extras, enc_json),
    )
}
pub(crate) async fn dec_animation(s: &str) -> Result<GltfAnimation, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [channels, samplers, name, extensions, extras] = parts.as_slice() else { return Err(format!("animation: expected 5 fields, got {}", parts.len())) };
    Ok(GltfAnimation {
        channels: split_top_level(strip_brackets(channels)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_animation_channel).collect::<Result<Vec<_>, String>>()?,
        samplers: split_top_level(strip_brackets(samplers)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_animation_sampler).collect::<Result<Vec<_>, String>>()?,
        name: decode_option(name, dec_str)?,
        extensions: decode_option(extensions, dec_json)?,
        extras: decode_option(extras, dec_json)?,
    })
}
//#endregion 🔖️AnimationGroupCodecs

//#region 🔖️CameraGroupCodecs
pub(crate) async fn enc_perspective(p: &GltfPerspective) -> String {
    format!("[{},{},{},{},{},{}]", encode_option(&p.aspect_ratio, |v| enc_f64(*v)), enc_f64(p.yfov), encode_option(&p.zfar, |v| enc_f64(*v)), enc_f64(p.znear), encode_option(&p.extensions, enc_json), encode_option(&p.extras, enc_json),)
}
pub(crate) async fn dec_perspective(s: &str) -> Result<GltfPerspective, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [aspect_ratio, yfov, zfar, znear, extensions, extras] = parts.as_slice() else { return Err(format!("perspective: expected 6 fields, got {}", parts.len())) };
    Ok(GltfPerspective {
        aspect_ratio: decode_option(aspect_ratio, dec_f64)?,
        yfov: dec_f64(yfov)?,
        zfar: decode_option(zfar, dec_f64)?,
        znear: dec_f64(znear)?,
        extensions: decode_option(extensions, dec_json)?,
        extras: decode_option(extras, dec_json)?,
    })
}
pub(crate) async fn enc_orthographic(o: &GltfOrthographic) -> String {
    format!("[{},{},{},{},{},{}]", enc_f64(o.xmag), enc_f64(o.ymag), enc_f64(o.zfar), enc_f64(o.znear), encode_option(&o.extensions, enc_json), encode_option(&o.extras, enc_json))
}
pub(crate) async fn dec_orthographic(s: &str) -> Result<GltfOrthographic, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [xmag, ymag, zfar, znear, extensions, extras] = parts.as_slice() else { return Err(format!("orthographic: expected 6 fields, got {}", parts.len())) };
    Ok(GltfOrthographic { xmag: dec_f64(xmag)?, ymag: dec_f64(ymag)?, zfar: dec_f64(zfar)?, znear: dec_f64(znear)?, extensions: decode_option(extensions, dec_json)?, extras: decode_option(extras, dec_json)? })
}
/// 🔀️ `GltfCameraProjection` is a real data-carrying enum (§3a) -- tag prefix `P`=Perspective,
/// `O`=Orthographic.
pub(crate) async fn enc_camera_projection(p: &GltfCameraProjection) -> String {
    match p {
        GltfCameraProjection::Perspective(v) => format!("P{}", enc_perspective(v)),
        GltfCameraProjection::Orthographic(v) => format!("O{}", enc_orthographic(v)),
    }
}
pub(crate) async fn dec_camera_projection(s: &str) -> Result<GltfCameraProjection, String> {
    let (tag, rest) = s.split_at(1);
    match tag {
        "P" => Ok(GltfCameraProjection::Perspective(dec_perspective(rest)?)),
        "O" => Ok(GltfCameraProjection::Orthographic(dec_orthographic(rest)?)),
        other => Err(format!("camera projection: unknown tag {other:?}")),
    }
}
pub(crate) async fn enc_camera(c: &GltfCamera) -> String {
    format!("[{},{},{},{}]", enc_camera_projection(&c.projection), encode_option(&c.name, |v| enc_str(v)), encode_option(&c.extensions, enc_json), encode_option(&c.extras, enc_json))
}
pub(crate) async fn dec_camera(s: &str) -> Result<GltfCamera, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [projection, name, extensions, extras] = parts.as_slice() else { return Err(format!("camera: expected 4 fields, got {}", parts.len())) };
    Ok(GltfCamera { projection: dec_camera_projection(projection)?, name: decode_option(name, dec_str)?, extensions: decode_option(extensions, dec_json)?, extras: decode_option(extras, dec_json)? })
}
//#endregion 🔖️CameraGroupCodecs

//#region 🔖️GenericCollectionCodec
/// 🧮️ Generic index-keyed collection triple codec, shared by every one of the 14 top-level arrays
/// (STRONG entities pass a real per-item diff encoder; WEAK entities pass the same `enc_item`/
/// `dec_item` for both `enc_item`/`enc_diff` via `GltfWeakCollectionDiff<T> = GltfCollectionDiff<T,
/// T>`) -- one real generic codec, not 14 hand-duplicated ones.
pub(crate) async fn enc_collection<T, D>(c: &GltfCollectionDiff<T, D>, enc_item: impl Fn(&T) -> String, enc_diff: impl Fn(&D) -> String) -> String {
    let removed = c.removed.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let modified = c.modified.iter().map(|m| format!("{}:{}", m.index, enc_diff(&m.diff))).collect::<Vec<_>>().join(",");
    let added = c.added.iter().map(|a| format!("{}:{}", a.index, enc_item(&a.item))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}
pub(crate) async fn dec_collection<T, D>(s: &str, dec_item: impl Fn(&str) -> Result<T, String>, dec_diff: impl Fn(&str) -> Result<D, String>) -> Result<GltfCollectionDiff<T, D>, String> {
    let three = split_top_level(s, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("collection: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_usize).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("collection modified: bad entry {entry:?}"))?;
            Ok(GltfModified { index: parse_usize(idx)?, diff: dec_diff(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("collection added: bad entry {entry:?}"))?;
            Ok(GltfAdded { index: parse_usize(idx)?, item: dec_item(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(GltfCollectionDiff { removed, modified, added })
}
//#endregion 🔖️GenericCollectionCodec

//#region 🔖️RealBinaryPrimitives
/// 🧪️ P2-FG3: real binary value codecs for `GltfDiff`/`GltfMutation` — mirrors the text codecs
/// above field-for-field, using `dsl::ByteWriter`/`dsl::ByteReader` (the same real LEB128-varint/
/// length-prefixed framework primitives png's/gif89a's own upgraded binary frames use,
/// `🎞️gif/🏅️standards/🔖️89a/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`'s `RealBinaryPrimitives`/
/// `RealBinaryDiffFrame` regions — `dsl`/`store`/`protocol` all alias the same kernel crate root,
/// reachable with no `use` needed beyond the absolute path). `pub(crate)` so `🧬️mutations/
/// 🦀️component.rs`'s hand-rolled `OpBinary` can reuse every one of these the same way it already
/// reuses this module's TEXT `enc_*`/`dec_*` primitives.
pub(crate) async fn write_bin_blob(w: &mut dsl::ByteWriter, bytes: &[u8]) {
    w.write_varint_u64(bytes.len() as u64);
    w.write_bytes(bytes);
}
pub(crate) async fn read_bin_blob(r: &mut dsl::ByteReader<'_>) -> Result<Vec<u8>, dsl::PackError> {
    let len = r.read_varint_u64()? as usize;
    Ok(r.read_bytes(len)?.to_vec())
}
pub(crate) async fn write_bin_str(w: &mut dsl::ByteWriter, s: &str) {
    write_bin_blob(w, s.as_bytes());
}
pub(crate) async fn read_bin_str(r: &mut dsl::ByteReader<'_>) -> Result<String, dsl::PackError> {
    let bytes = read_bin_blob(r)?;
    String::from_utf8(bytes).map_err(|e| dsl::PackError::Malformed { what: "gltf binary utf8 string", offset: 0, detail: e.to_string() })
}
/// 🧩 2-way presence flag (`0`=None, `1`=Some) — shared by every plain `Option<T>` field.
pub(crate) async fn write_bin_option<T>(w: &mut dsl::ByteWriter, v: &Option<T>, write_value: impl FnOnce(&mut dsl::ByteWriter, &T)) {
    match v {
        None => w.write_u8(0),
        Some(val) => {
            w.write_u8(1);
            write_value(w, val);
        }
    }
}
pub(crate) async fn read_bin_option<T>(r: &mut dsl::ByteReader<'_>, read_value: impl FnOnce(&mut dsl::ByteReader<'_>) -> Result<T, dsl::PackError>) -> Result<Option<T>, dsl::PackError> {
    match r.read_u8()? {
        0 => Ok(None),
        1 => Ok(Some(read_value(r)?)),
        other => Err(dsl::PackError::Malformed { what: "gltf binary option tag", offset: 0, detail: format!("unknown tag {other}") }),
    }
}
/// 🧩 3-way flag (`0`=unchanged/absent, `1`=cleared-to-`None`, `2`=set-to-`Some(value)`) for every
/// TRI-STATE `Option<Option<T>>` field — same shape as png's/gif's own doc comment (avoids
/// chaining two `if`-guarded conditional fields at the PROTOCOL-DESCRIPTION level,
/// `protocol-cond-cannot-chain`; the Rust codec here has no such limitation but keeps the same
/// 3-way-flag SHAPE for parity with `../💾️binary/📡️component.protocol.semio`).
pub(crate) async fn write_bin_tri<T>(w: &mut dsl::ByteWriter, v: &Option<Option<T>>, write_value: impl FnOnce(&mut dsl::ByteWriter, &T)) {
    match v {
        None => w.write_u8(0),
        Some(None) => w.write_u8(1),
        Some(Some(val)) => {
            w.write_u8(2);
            write_value(w, val);
        }
    }
}
pub(crate) async fn read_bin_tri<T>(r: &mut dsl::ByteReader<'_>, read_value: impl FnOnce(&mut dsl::ByteReader<'_>) -> Result<T, dsl::PackError>) -> Result<Option<Option<T>>, dsl::PackError> {
    match r.read_u8()? {
        0 => Ok(None),
        1 => Ok(Some(None)),
        2 => Ok(Some(Some(read_value(r)?))),
        other => Err(dsl::PackError::Malformed { what: "gltf binary tri-flag", offset: 0, detail: format!("unknown flag {other}") }),
    }
}
pub(crate) async fn write_bin_vec<T>(w: &mut dsl::ByteWriter, items: &[T], write_item: impl Fn(&mut dsl::ByteWriter, &T)) {
    w.write_varint_u64(items.len() as u64);
    for item in items {
        write_item(w, item);
    }
}
pub(crate) async fn read_bin_vec<T>(r: &mut dsl::ByteReader<'_>, mut read_item: impl FnMut(&mut dsl::ByteReader<'_>) -> Result<T, dsl::PackError>) -> Result<Vec<T>, dsl::PackError> {
    let n = r.read_varint_u64()? as usize;
    let mut out = Vec::with_capacity(n.min(1 << 20));
    for _ in 0..n {
        out.push(read_item(r)?);
    }
    Ok(out)
}
pub(crate) async fn write_bin_f64_array<const N: usize>(w: &mut dsl::ByteWriter, v: &[f64; N]) {
    for x in v {
        w.write_f64_le(*x);
    }
}
pub(crate) async fn read_bin_f64_array<const N: usize>(r: &mut dsl::ByteReader<'_>) -> Result<[f64; N], dsl::PackError> {
    let mut out = [0.0f64; N];
    for slot in out.iter_mut() {
        *slot = r.read_f64_le()?;
    }
    Ok(out)
}
pub(crate) async fn write_bin_f64_vec(w: &mut dsl::ByteWriter, v: &[f64]) {
    write_bin_vec(w, v, |w, x| w.write_f64_le(*x));
}
pub(crate) async fn read_bin_f64_vec(r: &mut dsl::ByteReader<'_>) -> Result<Vec<f64>, dsl::PackError> {
    read_bin_vec(r, |r| r.read_f64_le())
}
pub(crate) async fn write_bin_usize_vec(w: &mut dsl::ByteWriter, v: &[usize]) {
    write_bin_vec(w, v, |w, x: &usize| w.write_varint_u64(*x as u64));
}
pub(crate) async fn read_bin_usize_vec(r: &mut dsl::ByteReader<'_>) -> Result<Vec<usize>, dsl::PackError> {
    read_bin_vec(r, |r| Ok(r.read_varint_u64()? as usize))
}
pub(crate) async fn write_bin_string_vec(w: &mut dsl::ByteWriter, v: &[String]) {
    write_bin_vec(w, v, |w, s: &String| write_bin_str(w, s));
}
pub(crate) async fn read_bin_string_vec(r: &mut dsl::ByteReader<'_>) -> Result<Vec<String>, dsl::PackError> {
    read_bin_vec(r, read_bin_str)
}
pub(crate) async fn write_bin_attr_pairs(w: &mut dsl::ByteWriter, v: &[(String, usize)]) {
    write_bin_vec(w, v, |w, (k, idx): &(String, usize)| {
        write_bin_str(w, k);
        w.write_varint_u64(*idx as u64);
    });
}
pub(crate) async fn read_bin_attr_pairs(r: &mut dsl::ByteReader<'_>) -> Result<Vec<(String, usize)>, dsl::PackError> {
    read_bin_vec(r, |r| Ok((read_bin_str(r)?, r.read_varint_u64()? as usize)))
}
pub(crate) async fn gltf_bin_err(e: dsl::PackError) -> protocol::ProtocolError {
    protocol::ProtocolError::Malformed { what: "gltf binary", offset: 0, detail: e.to_string() }
}
//#endregion 🔖️RealBinaryPrimitives

//#region 🔖️RealBinaryJsonCodec
/// 🌳 `GltfJson` -- genuinely recursive real binary: tag `u8` (0=Null,1=Bool,2=Number,3=String,
/// 4=Array,5=Object) then the payload, matching `enc_json`/`dec_json`'s own tag scheme.
pub(crate) async fn write_bin_json(w: &mut dsl::ByteWriter, v: &GltfJson) {
    match v {
        GltfJson::Null => w.write_u8(0),
        GltfJson::Bool(b) => {
            w.write_u8(1);
            w.write_u8(if *b { 1 } else { 0 });
        }
        GltfJson::Number(n) => {
            w.write_u8(2);
            w.write_f64_le(*n);
        }
        GltfJson::String(s) => {
            w.write_u8(3);
            write_bin_str(w, s);
        }
        GltfJson::Array(items) => {
            w.write_u8(4);
            write_bin_vec(w, items, write_bin_json);
        }
        GltfJson::Object(members) => {
            w.write_u8(5);
            write_bin_vec(w, members, |w, (k, v): &(String, GltfJson)| {
                write_bin_str(w, k);
                write_bin_json(w, v);
            });
        }
    }
}
pub(crate) async fn read_bin_json(r: &mut dsl::ByteReader<'_>) -> Result<GltfJson, dsl::PackError> {
    match r.read_u8()? {
        0 => Ok(GltfJson::Null),
        1 => Ok(GltfJson::Bool(r.read_u8()? != 0)),
        2 => Ok(GltfJson::Number(r.read_f64_le()?)),
        3 => Ok(GltfJson::String(read_bin_str(r)?)),
        4 => Ok(GltfJson::Array(read_bin_vec(r, read_bin_json)?)),
        5 => Ok(GltfJson::Object(read_bin_vec(r, |r| Ok((read_bin_str(r)?, read_bin_json(r)?)))?)),
        other => Err(dsl::PackError::Malformed { what: "gltf json binary tag", offset: 0, detail: format!("unknown tag {other}") }),
    }
}
pub(crate) async fn write_bin_json_opt(w: &mut dsl::ByteWriter, v: &Option<GltfJson>) {
    write_bin_option(w, v, write_bin_json);
}
pub(crate) async fn read_bin_json_opt(r: &mut dsl::ByteReader<'_>) -> Result<Option<GltfJson>, dsl::PackError> {
    read_bin_option(r, read_bin_json)
}
//#endregion 🔖️RealBinaryJsonCodec

//#region 🔖️RealBinaryUnitEnumCodecs
/// 🔢️ Real spec numeric code (5120..5126), matching `GltfComponentType::code`/`from_code` exactly
/// -- NOT a re-derived discriminant table (the spec code IS this enum's real wire value, same one
/// the artifact's own `serde` impl emits).
pub(crate) async fn write_bin_component_type(w: &mut dsl::ByteWriter, t: GltfComponentType) {
    w.write_u32_le(t.code() as u32);
}
pub(crate) async fn read_bin_component_type(r: &mut dsl::ByteReader<'_>) -> Result<GltfComponentType, dsl::PackError> {
    GltfComponentType::from_code(r.read_u32_le()? as u64).map_err(|e| dsl::PackError::Malformed { what: "gltf component_type", offset: 0, detail: e })
}
/// 🔢️ Compact `u8` discriminants for the remaining small unit-variant enums (real spec strings
/// only exist on the TEXT side; the binary frame is free to use its own dense encoding since
/// nothing outside this codec pair ever reads these bytes directly).
pub(crate) async fn write_bin_accessor_type(w: &mut dsl::ByteWriter, t: GltfAccessorType) {
    w.write_u8(match t {
        GltfAccessorType::Scalar => 0,
        GltfAccessorType::Vec2 => 1,
        GltfAccessorType::Vec3 => 2,
        GltfAccessorType::Vec4 => 3,
        GltfAccessorType::Mat2 => 4,
        GltfAccessorType::Mat3 => 5,
        GltfAccessorType::Mat4 => 6,
    });
}
pub(crate) async fn read_bin_accessor_type(r: &mut dsl::ByteReader<'_>) -> Result<GltfAccessorType, dsl::PackError> {
    Ok(match r.read_u8()? {
        0 => GltfAccessorType::Scalar,
        1 => GltfAccessorType::Vec2,
        2 => GltfAccessorType::Vec3,
        3 => GltfAccessorType::Vec4,
        4 => GltfAccessorType::Mat2,
        5 => GltfAccessorType::Mat3,
        6 => GltfAccessorType::Mat4,
        other => return Err(dsl::PackError::Malformed { what: "gltf accessor_type", offset: 0, detail: format!("unknown tag {other}") }),
    })
}
pub(crate) async fn write_bin_alpha_mode(w: &mut dsl::ByteWriter, m: GltfAlphaMode) {
    w.write_u8(match m {
        GltfAlphaMode::Opaque => 0,
        GltfAlphaMode::Mask => 1,
        GltfAlphaMode::Blend => 2,
    });
}
pub(crate) async fn read_bin_alpha_mode(r: &mut dsl::ByteReader<'_>) -> Result<GltfAlphaMode, dsl::PackError> {
    Ok(match r.read_u8()? {
        0 => GltfAlphaMode::Opaque,
        1 => GltfAlphaMode::Mask,
        2 => GltfAlphaMode::Blend,
        other => return Err(dsl::PackError::Malformed { what: "gltf alpha_mode", offset: 0, detail: format!("unknown tag {other}") }),
    })
}
pub(crate) async fn write_bin_interpolation(w: &mut dsl::ByteWriter, i: GltfInterpolation) {
    w.write_u8(match i {
        GltfInterpolation::Linear => 0,
        GltfInterpolation::Step => 1,
        GltfInterpolation::CubicSpline => 2,
    });
}
pub(crate) async fn read_bin_interpolation(r: &mut dsl::ByteReader<'_>) -> Result<GltfInterpolation, dsl::PackError> {
    Ok(match r.read_u8()? {
        0 => GltfInterpolation::Linear,
        1 => GltfInterpolation::Step,
        2 => GltfInterpolation::CubicSpline,
        other => return Err(dsl::PackError::Malformed { what: "gltf interpolation", offset: 0, detail: format!("unknown tag {other}") }),
    })
}
pub(crate) async fn write_bin_animation_path(w: &mut dsl::ByteWriter, p: GltfAnimationPath) {
    w.write_u8(match p {
        GltfAnimationPath::Translation => 0,
        GltfAnimationPath::Rotation => 1,
        GltfAnimationPath::Scale => 2,
        GltfAnimationPath::Weights => 3,
    });
}
pub(crate) async fn read_bin_animation_path(r: &mut dsl::ByteReader<'_>) -> Result<GltfAnimationPath, dsl::PackError> {
    Ok(match r.read_u8()? {
        0 => GltfAnimationPath::Translation,
        1 => GltfAnimationPath::Rotation,
        2 => GltfAnimationPath::Scale,
        3 => GltfAnimationPath::Weights,
        other => return Err(dsl::PackError::Malformed { what: "gltf animation_path", offset: 0, detail: format!("unknown tag {other}") }),
    })
}
pub(crate) async fn write_bin_source_form(w: &mut dsl::ByteWriter, f: GltfSourceForm) {
    w.write_u8(match f {
        GltfSourceForm::Json => 0,
        GltfSourceForm::Glb => 1,
    });
}
pub(crate) async fn read_bin_source_form(r: &mut dsl::ByteReader<'_>) -> Result<GltfSourceForm, dsl::PackError> {
    Ok(match r.read_u8()? {
        0 => GltfSourceForm::Json,
        1 => GltfSourceForm::Glb,
        other => return Err(dsl::PackError::Malformed { what: "gltf source_form", offset: 0, detail: format!("unknown tag {other}") }),
    })
}
//#endregion 🔖️RealBinaryUnitEnumCodecs

//#region 🔖️RealBinaryAssetSceneNodeGroupCodecs
pub(crate) async fn write_bin_asset_diff(w: &mut dsl::ByteWriter, d: &GltfAssetDiff) {
    write_bin_option(w, &d.version, |w, v| write_bin_str(w, v));
    write_bin_tri(w, &d.generator, |w, v| write_bin_str(w, v));
    write_bin_tri(w, &d.copyright, |w, v| write_bin_str(w, v));
    write_bin_tri(w, &d.min_version, |w, v| write_bin_str(w, v));
    write_bin_tri(w, &d.extensions, write_bin_json);
    write_bin_tri(w, &d.extras, write_bin_json);
}
pub(crate) async fn read_bin_asset_diff(r: &mut dsl::ByteReader<'_>) -> Result<GltfAssetDiff, dsl::PackError> {
    Ok(GltfAssetDiff {
        version: read_bin_option(r, read_bin_str)?,
        generator: read_bin_tri(r, read_bin_str)?,
        copyright: read_bin_tri(r, read_bin_str)?,
        min_version: read_bin_tri(r, read_bin_str)?,
        extensions: read_bin_tri(r, read_bin_json)?,
        extras: read_bin_tri(r, read_bin_json)?,
    })
}
pub(crate) async fn write_bin_scene(w: &mut dsl::ByteWriter, sc: &GltfScene) {
    write_bin_usize_vec(w, &sc.nodes);
    write_bin_option(w, &sc.name, |w, v| write_bin_str(w, v));
    write_bin_json_opt(w, &sc.extensions);
    write_bin_json_opt(w, &sc.extras);
}
pub(crate) async fn read_bin_scene(r: &mut dsl::ByteReader<'_>) -> Result<GltfScene, dsl::PackError> {
    Ok(GltfScene { nodes: read_bin_usize_vec(r)?, name: read_bin_option(r, read_bin_str)?, extensions: read_bin_json_opt(r)?, extras: read_bin_json_opt(r)? })
}
pub(crate) async fn write_bin_scene_diff(w: &mut dsl::ByteWriter, d: &GltfSceneDiff) {
    write_bin_option(w, &d.nodes, |w, v| write_bin_usize_vec(w, v));
    write_bin_tri(w, &d.name, |w, v| write_bin_str(w, v));
    write_bin_tri(w, &d.extensions, write_bin_json);
    write_bin_tri(w, &d.extras, write_bin_json);
}
pub(crate) async fn read_bin_scene_diff(r: &mut dsl::ByteReader<'_>) -> Result<GltfSceneDiff, dsl::PackError> {
    Ok(GltfSceneDiff { nodes: read_bin_option(r, read_bin_usize_vec)?, name: read_bin_tri(r, read_bin_str)?, extensions: read_bin_tri(r, read_bin_json)?, extras: read_bin_tri(r, read_bin_json)? })
}
pub(crate) async fn write_bin_node(w: &mut dsl::ByteWriter, n: &GltfNode) {
    write_bin_usize_vec(w, &n.children);
    write_bin_option(w, &n.mesh, |w, v| w.write_varint_u64(*v as u64));
    write_bin_option(w, &n.camera, |w, v| w.write_varint_u64(*v as u64));
    write_bin_option(w, &n.skin, |w, v| w.write_varint_u64(*v as u64));
    write_bin_option(w, &n.matrix, |w, v| write_bin_f64_array::<16>(w, v));
    write_bin_option(w, &n.translation, |w, v| write_bin_f64_array::<3>(w, v));
    write_bin_option(w, &n.rotation, |w, v| write_bin_f64_array::<4>(w, v));
    write_bin_option(w, &n.scale, |w, v| write_bin_f64_array::<3>(w, v));
    write_bin_f64_vec(w, &n.weights);
    write_bin_option(w, &n.name, |w, v| write_bin_str(w, v));
    write_bin_json_opt(w, &n.extensions);
    write_bin_json_opt(w, &n.extras);
}
pub(crate) async fn read_bin_node(r: &mut dsl::ByteReader<'_>) -> Result<GltfNode, dsl::PackError> {
    Ok(GltfNode {
        children: read_bin_usize_vec(r)?,
        mesh: read_bin_option(r, |r| Ok(r.read_varint_u64()? as usize))?,
        camera: read_bin_option(r, |r| Ok(r.read_varint_u64()? as usize))?,
        skin: read_bin_option(r, |r| Ok(r.read_varint_u64()? as usize))?,
        matrix: read_bin_option(r, read_bin_f64_array::<16>)?,
        translation: read_bin_option(r, read_bin_f64_array::<3>)?,
        rotation: read_bin_option(r, read_bin_f64_array::<4>)?,
        scale: read_bin_option(r, read_bin_f64_array::<3>)?,
        weights: read_bin_f64_vec(r)?,
        name: read_bin_option(r, read_bin_str)?,
        extensions: read_bin_json_opt(r)?,
        extras: read_bin_json_opt(r)?,
    })
}
pub(crate) async fn write_bin_node_diff(w: &mut dsl::ByteWriter, d: &GltfNodeDiff) {
    write_bin_option(w, &d.children, |w, v| write_bin_usize_vec(w, v));
    write_bin_tri(w, &d.mesh, |w, v| w.write_varint_u64(*v as u64));
    write_bin_tri(w, &d.camera, |w, v| w.write_varint_u64(*v as u64));
    write_bin_tri(w, &d.skin, |w, v| w.write_varint_u64(*v as u64));
    write_bin_tri(w, &d.matrix, |w, v| write_bin_f64_array::<16>(w, v));
    write_bin_tri(w, &d.translation, |w, v| write_bin_f64_array::<3>(w, v));
    write_bin_tri(w, &d.rotation, |w, v| write_bin_f64_array::<4>(w, v));
    write_bin_tri(w, &d.scale, |w, v| write_bin_f64_array::<3>(w, v));
    write_bin_option(w, &d.weights, |w, v| write_bin_f64_vec(w, v));
    write_bin_tri(w, &d.name, |w, v| write_bin_str(w, v));
    write_bin_tri(w, &d.extensions, write_bin_json);
    write_bin_tri(w, &d.extras, write_bin_json);
}
pub(crate) async fn read_bin_node_diff(r: &mut dsl::ByteReader<'_>) -> Result<GltfNodeDiff, dsl::PackError> {
    Ok(GltfNodeDiff {
        children: read_bin_option(r, read_bin_usize_vec)?,
        mesh: read_bin_tri(r, |r| Ok(r.read_varint_u64()? as usize))?,
        camera: read_bin_tri(r, |r| Ok(r.read_varint_u64()? as usize))?,
        skin: read_bin_tri(r, |r| Ok(r.read_varint_u64()? as usize))?,
        matrix: read_bin_tri(r, read_bin_f64_array::<16>)?,
        translation: read_bin_tri(r, read_bin_f64_array::<3>)?,
        rotation: read_bin_tri(r, read_bin_f64_array::<4>)?,
        scale: read_bin_tri(r, read_bin_f64_array::<3>)?,
        weights: read_bin_option(r, read_bin_f64_vec)?,
        name: read_bin_tri(r, read_bin_str)?,
        extensions: read_bin_tri(r, read_bin_json)?,
        extras: read_bin_tri(r, read_bin_json)?,
    })
}
//#endregion 🔖️RealBinaryAssetSceneNodeGroupCodecs

//#region 🔖️RealBinaryMeshAccessorMaterialGroupCodecs
pub(crate) async fn write_bin_primitive(w: &mut dsl::ByteWriter, p: &GltfPrimitive) {
    write_bin_attr_pairs(w, &p.attributes);
    write_bin_option(w, &p.indices, |w, v| w.write_varint_u64(*v as u64));
    write_bin_option(w, &p.material, |w, v| w.write_varint_u64(*v as u64));
    write_bin_option(w, &p.mode, |w, v| w.write_varint_u64(*v));
    write_bin_vec(w, &p.targets, |w, target| write_bin_attr_pairs(w, &target.0));
    write_bin_json_opt(w, &p.extensions);
    write_bin_json_opt(w, &p.extras);
}
pub(crate) async fn read_bin_primitive(r: &mut dsl::ByteReader<'_>) -> Result<GltfPrimitive, dsl::PackError> {
    Ok(GltfPrimitive {
        attributes: read_bin_attr_pairs(r)?,
        indices: read_bin_option(r, |r| Ok(r.read_varint_u64()? as usize))?,
        material: read_bin_option(r, |r| Ok(r.read_varint_u64()? as usize))?,
        mode: read_bin_option(r, |r| r.read_varint_u64())?,
        targets: read_bin_vec(r, |r| read_bin_attr_pairs(r).map(GltfMorphTarget))?,
        extensions: read_bin_json_opt(r)?,
        extras: read_bin_json_opt(r)?,
    })
}
pub(crate) async fn write_bin_primitive_vec(w: &mut dsl::ByteWriter, v: &[GltfPrimitive]) {
    write_bin_vec(w, v, write_bin_primitive);
}
pub(crate) async fn read_bin_primitive_vec(r: &mut dsl::ByteReader<'_>) -> Result<Vec<GltfPrimitive>, dsl::PackError> {
    read_bin_vec(r, read_bin_primitive)
}
pub(crate) async fn write_bin_mesh(w: &mut dsl::ByteWriter, m: &GltfMesh) {
    write_bin_primitive_vec(w, &m.primitives);
    write_bin_f64_vec(w, &m.weights);
    write_bin_option(w, &m.name, |w, v| write_bin_str(w, v));
    write_bin_json_opt(w, &m.extensions);
    write_bin_json_opt(w, &m.extras);
}
pub(crate) async fn read_bin_mesh(r: &mut dsl::ByteReader<'_>) -> Result<GltfMesh, dsl::PackError> {
    Ok(GltfMesh { primitives: read_bin_primitive_vec(r)?, weights: read_bin_f64_vec(r)?, name: read_bin_option(r, read_bin_str)?, extensions: read_bin_json_opt(r)?, extras: read_bin_json_opt(r)? })
}
pub(crate) async fn write_bin_mesh_diff(w: &mut dsl::ByteWriter, d: &GltfMeshDiff) {
    write_bin_option(w, &d.primitives, |w, v| write_bin_primitive_vec(w, v));
    write_bin_option(w, &d.weights, |w, v| write_bin_f64_vec(w, v));
    write_bin_tri(w, &d.name, |w, v| write_bin_str(w, v));
    write_bin_tri(w, &d.extensions, write_bin_json);
    write_bin_tri(w, &d.extras, write_bin_json);
}
pub(crate) async fn read_bin_mesh_diff(r: &mut dsl::ByteReader<'_>) -> Result<GltfMeshDiff, dsl::PackError> {
    Ok(GltfMeshDiff {
        primitives: read_bin_option(r, read_bin_primitive_vec)?,
        weights: read_bin_option(r, read_bin_f64_vec)?,
        name: read_bin_tri(r, read_bin_str)?,
        extensions: read_bin_tri(r, read_bin_json)?,
        extras: read_bin_tri(r, read_bin_json)?,
    })
}
pub(crate) async fn write_bin_sparse_indices(w: &mut dsl::ByteWriter, v: &GltfSparseIndices) {
    w.write_varint_u64(v.buffer_view as u64);
    w.write_varint_u64(v.byte_offset as u64);
    write_bin_component_type(w, v.component_type);
}
pub(crate) async fn read_bin_sparse_indices(r: &mut dsl::ByteReader<'_>) -> Result<GltfSparseIndices, dsl::PackError> {
    Ok(GltfSparseIndices { buffer_view: r.read_varint_u64()? as usize, byte_offset: r.read_varint_u64()? as usize, component_type: read_bin_component_type(r)? })
}
pub(crate) async fn write_bin_sparse_values(w: &mut dsl::ByteWriter, v: &GltfSparseValues) {
    w.write_varint_u64(v.buffer_view as u64);
    w.write_varint_u64(v.byte_offset as u64);
}
pub(crate) async fn read_bin_sparse_values(r: &mut dsl::ByteReader<'_>) -> Result<GltfSparseValues, dsl::PackError> {
    Ok(GltfSparseValues { buffer_view: r.read_varint_u64()? as usize, byte_offset: r.read_varint_u64()? as usize })
}
pub(crate) async fn write_bin_sparse_accessor(w: &mut dsl::ByteWriter, v: &GltfSparseAccessor) {
    w.write_varint_u64(v.count as u64);
    write_bin_sparse_indices(w, &v.indices);
    write_bin_sparse_values(w, &v.values);
}
pub(crate) async fn read_bin_sparse_accessor(r: &mut dsl::ByteReader<'_>) -> Result<GltfSparseAccessor, dsl::PackError> {
    Ok(GltfSparseAccessor { count: r.read_varint_u64()? as usize, indices: read_bin_sparse_indices(r)?, values: read_bin_sparse_values(r)? })
}
pub(crate) async fn write_bin_accessor(w: &mut dsl::ByteWriter, a: &GltfAccessor) {
    write_bin_option(w, &a.buffer_view, |w, v| w.write_varint_u64(*v as u64));
    w.write_varint_u64(a.byte_offset as u64);
    write_bin_component_type(w, a.component_type);
    w.write_u8(if a.normalized { 1 } else { 0 });
    w.write_varint_u64(a.count as u64);
    write_bin_accessor_type(w, a.kind);
    write_bin_option(w, &a.max, |w, v| write_bin_f64_vec(w, v));
    write_bin_option(w, &a.min, |w, v| write_bin_f64_vec(w, v));
    write_bin_option(w, &a.sparse, write_bin_sparse_accessor);
    write_bin_option(w, &a.name, |w, v| write_bin_str(w, v));
    write_bin_json_opt(w, &a.extensions);
    write_bin_json_opt(w, &a.extras);
}
pub(crate) async fn read_bin_accessor(r: &mut dsl::ByteReader<'_>) -> Result<GltfAccessor, dsl::PackError> {
    Ok(GltfAccessor {
        buffer_view: read_bin_option(r, |r| Ok(r.read_varint_u64()? as usize))?,
        byte_offset: r.read_varint_u64()? as usize,
        component_type: read_bin_component_type(r)?,
        normalized: r.read_u8()? != 0,
        count: r.read_varint_u64()? as usize,
        kind: read_bin_accessor_type(r)?,
        max: read_bin_option(r, read_bin_f64_vec)?,
        min: read_bin_option(r, read_bin_f64_vec)?,
        sparse: read_bin_option(r, read_bin_sparse_accessor)?,
        name: read_bin_option(r, read_bin_str)?,
        extensions: read_bin_json_opt(r)?,
        extras: read_bin_json_opt(r)?,
    })
}
pub(crate) async fn write_bin_accessor_diff(w: &mut dsl::ByteWriter, d: &GltfAccessorDiff) {
    write_bin_tri(w, &d.buffer_view, |w, v| w.write_varint_u64(*v as u64));
    write_bin_option(w, &d.byte_offset, |w, v| w.write_varint_u64(*v as u64));
    write_bin_option(w, &d.component_type, |w, v| write_bin_component_type(w, *v));
    write_bin_option(w, &d.normalized, |w, v| w.write_u8(if *v { 1 } else { 0 }));
    write_bin_option(w, &d.count, |w, v| w.write_varint_u64(*v as u64));
    write_bin_option(w, &d.kind, |w, v| write_bin_accessor_type(w, *v));
    write_bin_tri(w, &d.max, |w, v| write_bin_f64_vec(w, v));
    write_bin_tri(w, &d.min, |w, v| write_bin_f64_vec(w, v));
    write_bin_tri(w, &d.sparse, write_bin_sparse_accessor);
    write_bin_tri(w, &d.name, |w, v| write_bin_str(w, v));
    write_bin_tri(w, &d.extensions, write_bin_json);
    write_bin_tri(w, &d.extras, write_bin_json);
}
pub(crate) async fn read_bin_accessor_diff(r: &mut dsl::ByteReader<'_>) -> Result<GltfAccessorDiff, dsl::PackError> {
    Ok(GltfAccessorDiff {
        buffer_view: read_bin_tri(r, |r| Ok(r.read_varint_u64()? as usize))?,
        byte_offset: read_bin_option(r, |r| Ok(r.read_varint_u64()? as usize))?,
        component_type: read_bin_option(r, read_bin_component_type)?,
        normalized: read_bin_option(r, |r| Ok(r.read_u8()? != 0))?,
        count: read_bin_option(r, |r| Ok(r.read_varint_u64()? as usize))?,
        kind: read_bin_option(r, read_bin_accessor_type)?,
        max: read_bin_tri(r, read_bin_f64_vec)?,
        min: read_bin_tri(r, read_bin_f64_vec)?,
        sparse: read_bin_tri(r, read_bin_sparse_accessor)?,
        name: read_bin_tri(r, read_bin_str)?,
        extensions: read_bin_tri(r, read_bin_json)?,
        extras: read_bin_tri(r, read_bin_json)?,
    })
}
pub(crate) async fn write_bin_texture_info(w: &mut dsl::ByteWriter, v: &GltfTextureInfo) {
    w.write_varint_u64(v.index as u64);
    w.write_varint_u64(v.tex_coord);
    write_bin_json_opt(w, &v.extensions);
    write_bin_json_opt(w, &v.extras);
}
pub(crate) async fn read_bin_texture_info(r: &mut dsl::ByteReader<'_>) -> Result<GltfTextureInfo, dsl::PackError> {
    Ok(GltfTextureInfo { index: r.read_varint_u64()? as usize, tex_coord: r.read_varint_u64()?, extensions: read_bin_json_opt(r)?, extras: read_bin_json_opt(r)? })
}
pub(crate) async fn write_bin_normal_texture_info(w: &mut dsl::ByteWriter, v: &GltfNormalTextureInfo) {
    w.write_varint_u64(v.index as u64);
    w.write_varint_u64(v.tex_coord);
    w.write_f64_le(v.scale);
    write_bin_json_opt(w, &v.extensions);
    write_bin_json_opt(w, &v.extras);
}
pub(crate) async fn read_bin_normal_texture_info(r: &mut dsl::ByteReader<'_>) -> Result<GltfNormalTextureInfo, dsl::PackError> {
    Ok(GltfNormalTextureInfo { index: r.read_varint_u64()? as usize, tex_coord: r.read_varint_u64()?, scale: r.read_f64_le()?, extensions: read_bin_json_opt(r)?, extras: read_bin_json_opt(r)? })
}
pub(crate) async fn write_bin_occlusion_texture_info(w: &mut dsl::ByteWriter, v: &GltfOcclusionTextureInfo) {
    w.write_varint_u64(v.index as u64);
    w.write_varint_u64(v.tex_coord);
    w.write_f64_le(v.strength);
    write_bin_json_opt(w, &v.extensions);
    write_bin_json_opt(w, &v.extras);
}
pub(crate) async fn read_bin_occlusion_texture_info(r: &mut dsl::ByteReader<'_>) -> Result<GltfOcclusionTextureInfo, dsl::PackError> {
    Ok(GltfOcclusionTextureInfo { index: r.read_varint_u64()? as usize, tex_coord: r.read_varint_u64()?, strength: r.read_f64_le()?, extensions: read_bin_json_opt(r)?, extras: read_bin_json_opt(r)? })
}
pub(crate) async fn write_bin_pbr(w: &mut dsl::ByteWriter, v: &GltfPbrMetallicRoughness) {
    write_bin_f64_array::<4>(w, &v.base_color_factor);
    write_bin_option(w, &v.base_color_texture, write_bin_texture_info);
    w.write_f64_le(v.metallic_factor);
    w.write_f64_le(v.roughness_factor);
    write_bin_option(w, &v.metallic_roughness_texture, write_bin_texture_info);
    write_bin_json_opt(w, &v.extensions);
    write_bin_json_opt(w, &v.extras);
}
pub(crate) async fn read_bin_pbr(r: &mut dsl::ByteReader<'_>) -> Result<GltfPbrMetallicRoughness, dsl::PackError> {
    Ok(GltfPbrMetallicRoughness {
        base_color_factor: read_bin_f64_array::<4>(r)?,
        base_color_texture: read_bin_option(r, read_bin_texture_info)?,
        metallic_factor: r.read_f64_le()?,
        roughness_factor: r.read_f64_le()?,
        metallic_roughness_texture: read_bin_option(r, read_bin_texture_info)?,
        extensions: read_bin_json_opt(r)?,
        extras: read_bin_json_opt(r)?,
    })
}
pub(crate) async fn write_bin_material(w: &mut dsl::ByteWriter, m: &GltfMaterial) {
    write_bin_option(w, &m.name, |w, v| write_bin_str(w, v));
    write_bin_option(w, &m.pbr_metallic_roughness, write_bin_pbr);
    write_bin_option(w, &m.normal_texture, write_bin_normal_texture_info);
    write_bin_option(w, &m.occlusion_texture, write_bin_occlusion_texture_info);
    write_bin_option(w, &m.emissive_texture, write_bin_texture_info);
    write_bin_f64_array::<3>(w, &m.emissive_factor);
    write_bin_alpha_mode(w, m.alpha_mode);
    w.write_f64_le(m.alpha_cutoff);
    w.write_u8(if m.double_sided { 1 } else { 0 });
    write_bin_json_opt(w, &m.extensions);
    write_bin_json_opt(w, &m.extras);
}
pub(crate) async fn read_bin_material(r: &mut dsl::ByteReader<'_>) -> Result<GltfMaterial, dsl::PackError> {
    Ok(GltfMaterial {
        name: read_bin_option(r, read_bin_str)?,
        pbr_metallic_roughness: read_bin_option(r, read_bin_pbr)?,
        normal_texture: read_bin_option(r, read_bin_normal_texture_info)?,
        occlusion_texture: read_bin_option(r, read_bin_occlusion_texture_info)?,
        emissive_texture: read_bin_option(r, read_bin_texture_info)?,
        emissive_factor: read_bin_f64_array::<3>(r)?,
        alpha_mode: read_bin_alpha_mode(r)?,
        alpha_cutoff: r.read_f64_le()?,
        double_sided: r.read_u8()? != 0,
        extensions: read_bin_json_opt(r)?,
        extras: read_bin_json_opt(r)?,
    })
}
pub(crate) async fn write_bin_material_diff(w: &mut dsl::ByteWriter, d: &GltfMaterialDiff) {
    write_bin_tri(w, &d.name, |w, v| write_bin_str(w, v));
    write_bin_tri(w, &d.pbr_metallic_roughness, write_bin_pbr);
    write_bin_tri(w, &d.normal_texture, write_bin_normal_texture_info);
    write_bin_tri(w, &d.occlusion_texture, write_bin_occlusion_texture_info);
    write_bin_tri(w, &d.emissive_texture, write_bin_texture_info);
    write_bin_option(w, &d.emissive_factor, |w, v| write_bin_f64_array::<3>(w, v));
    write_bin_option(w, &d.alpha_mode, |w, v| write_bin_alpha_mode(w, *v));
    write_bin_option(w, &d.alpha_cutoff, |w, v| w.write_f64_le(*v));
    write_bin_option(w, &d.double_sided, |w, v| w.write_u8(if *v { 1 } else { 0 }));
    write_bin_tri(w, &d.extensions, write_bin_json);
    write_bin_tri(w, &d.extras, write_bin_json);
}
pub(crate) async fn read_bin_material_diff(r: &mut dsl::ByteReader<'_>) -> Result<GltfMaterialDiff, dsl::PackError> {
    Ok(GltfMaterialDiff {
        name: read_bin_tri(r, read_bin_str)?,
        pbr_metallic_roughness: read_bin_tri(r, read_bin_pbr)?,
        normal_texture: read_bin_tri(r, read_bin_normal_texture_info)?,
        occlusion_texture: read_bin_tri(r, read_bin_occlusion_texture_info)?,
        emissive_texture: read_bin_tri(r, read_bin_texture_info)?,
        emissive_factor: read_bin_option(r, read_bin_f64_array::<3>)?,
        alpha_mode: read_bin_option(r, read_bin_alpha_mode)?,
        alpha_cutoff: read_bin_option(r, |r| r.read_f64_le())?,
        double_sided: read_bin_option(r, |r| Ok(r.read_u8()? != 0))?,
        extensions: read_bin_tri(r, read_bin_json)?,
        extras: read_bin_tri(r, read_bin_json)?,
    })
}
//#endregion 🔖️RealBinaryMeshAccessorMaterialGroupCodecs

//#region 🔖️RealBinaryBufferGroupCodecs
pub(crate) async fn write_bin_buffer(w: &mut dsl::ByteWriter, b: &GltfBuffer) {
    w.write_varint_u64(b.byte_length as u64);
    write_bin_option(w, &b.uri, |w, v| write_bin_str(w, v));
    write_bin_option(w, &b.name, |w, v| write_bin_str(w, v));
    write_bin_json_opt(w, &b.extensions);
    write_bin_json_opt(w, &b.extras);
}
pub(crate) async fn read_bin_buffer(r: &mut dsl::ByteReader<'_>) -> Result<GltfBuffer, dsl::PackError> {
    Ok(GltfBuffer { byte_length: r.read_varint_u64()? as usize, uri: read_bin_option(r, read_bin_str)?, name: read_bin_option(r, read_bin_str)?, extensions: read_bin_json_opt(r)?, extras: read_bin_json_opt(r)? })
}
pub(crate) async fn write_bin_buffer_diff(w: &mut dsl::ByteWriter, d: &GltfBufferDiff) {
    write_bin_option(w, &d.byte_length, |w, v| w.write_varint_u64(*v as u64));
    write_bin_tri(w, &d.uri, |w, v| write_bin_str(w, v));
    write_bin_tri(w, &d.name, |w, v| write_bin_str(w, v));
    write_bin_tri(w, &d.extensions, write_bin_json);
    write_bin_tri(w, &d.extras, write_bin_json);
}
pub(crate) async fn read_bin_buffer_diff(r: &mut dsl::ByteReader<'_>) -> Result<GltfBufferDiff, dsl::PackError> {
    Ok(GltfBufferDiff {
        byte_length: read_bin_option(r, |r| Ok(r.read_varint_u64()? as usize))?,
        uri: read_bin_tri(r, read_bin_str)?,
        name: read_bin_tri(r, read_bin_str)?,
        extensions: read_bin_tri(r, read_bin_json)?,
        extras: read_bin_tri(r, read_bin_json)?,
    })
}
pub(crate) async fn write_bin_buffer_view(w: &mut dsl::ByteWriter, v: &GltfBufferView) {
    w.write_varint_u64(v.buffer as u64);
    w.write_varint_u64(v.byte_offset as u64);
    w.write_varint_u64(v.byte_length as u64);
    write_bin_option(w, &v.byte_stride, |w, x| w.write_varint_u64(*x as u64));
    write_bin_option(w, &v.target, |w, x| w.write_varint_u64(*x));
    write_bin_option(w, &v.name, |w, x| write_bin_str(w, x));
    write_bin_json_opt(w, &v.extensions);
    write_bin_json_opt(w, &v.extras);
}
pub(crate) async fn read_bin_buffer_view(r: &mut dsl::ByteReader<'_>) -> Result<GltfBufferView, dsl::PackError> {
    Ok(GltfBufferView {
        buffer: r.read_varint_u64()? as usize,
        byte_offset: r.read_varint_u64()? as usize,
        byte_length: r.read_varint_u64()? as usize,
        byte_stride: read_bin_option(r, |r| Ok(r.read_varint_u64()? as usize))?,
        target: read_bin_option(r, |r| r.read_varint_u64())?,
        name: read_bin_option(r, read_bin_str)?,
        extensions: read_bin_json_opt(r)?,
        extras: read_bin_json_opt(r)?,
    })
}
//#endregion 🔖️RealBinaryBufferGroupCodecs

//#region 🔖️RealBinaryTextureImageSamplerSkinGroupCodecs
pub(crate) async fn write_bin_texture(w: &mut dsl::ByteWriter, t: &GltfTexture) {
    write_bin_option(w, &t.sampler, |w, v| w.write_varint_u64(*v as u64));
    write_bin_option(w, &t.source, |w, v| w.write_varint_u64(*v as u64));
    write_bin_option(w, &t.name, |w, v| write_bin_str(w, v));
    write_bin_json_opt(w, &t.extensions);
    write_bin_json_opt(w, &t.extras);
}
pub(crate) async fn read_bin_texture(r: &mut dsl::ByteReader<'_>) -> Result<GltfTexture, dsl::PackError> {
    Ok(GltfTexture {
        sampler: read_bin_option(r, |r| Ok(r.read_varint_u64()? as usize))?,
        source: read_bin_option(r, |r| Ok(r.read_varint_u64()? as usize))?,
        name: read_bin_option(r, read_bin_str)?,
        extensions: read_bin_json_opt(r)?,
        extras: read_bin_json_opt(r)?,
    })
}
pub(crate) async fn write_bin_image(w: &mut dsl::ByteWriter, i: &GltfImage) {
    write_bin_option(w, &i.uri, |w, v| write_bin_str(w, v));
    write_bin_option(w, &i.mime_type, |w, v| write_bin_str(w, v));
    write_bin_option(w, &i.buffer_view, |w, v| w.write_varint_u64(*v as u64));
    write_bin_option(w, &i.name, |w, v| write_bin_str(w, v));
    write_bin_json_opt(w, &i.extensions);
    write_bin_json_opt(w, &i.extras);
}
pub(crate) async fn read_bin_image(r: &mut dsl::ByteReader<'_>) -> Result<GltfImage, dsl::PackError> {
    Ok(GltfImage {
        uri: read_bin_option(r, read_bin_str)?,
        mime_type: read_bin_option(r, read_bin_str)?,
        buffer_view: read_bin_option(r, |r| Ok(r.read_varint_u64()? as usize))?,
        name: read_bin_option(r, read_bin_str)?,
        extensions: read_bin_json_opt(r)?,
        extras: read_bin_json_opt(r)?,
    })
}
pub(crate) async fn write_bin_sampler(w: &mut dsl::ByteWriter, s: &GltfSampler) {
    write_bin_option(w, &s.mag_filter, |w, v| w.write_varint_u64(*v));
    write_bin_option(w, &s.min_filter, |w, v| w.write_varint_u64(*v));
    w.write_varint_u64(s.wrap_s);
    w.write_varint_u64(s.wrap_t);
    write_bin_option(w, &s.name, |w, v| write_bin_str(w, v));
    write_bin_json_opt(w, &s.extensions);
    write_bin_json_opt(w, &s.extras);
}
pub(crate) async fn read_bin_sampler(r: &mut dsl::ByteReader<'_>) -> Result<GltfSampler, dsl::PackError> {
    Ok(GltfSampler {
        mag_filter: read_bin_option(r, |r| r.read_varint_u64())?,
        min_filter: read_bin_option(r, |r| r.read_varint_u64())?,
        wrap_s: r.read_varint_u64()?,
        wrap_t: r.read_varint_u64()?,
        name: read_bin_option(r, read_bin_str)?,
        extensions: read_bin_json_opt(r)?,
        extras: read_bin_json_opt(r)?,
    })
}
pub(crate) async fn write_bin_skin(w: &mut dsl::ByteWriter, v: &GltfSkin) {
    write_bin_option(w, &v.inverse_bind_matrices, |w, x| w.write_varint_u64(*x as u64));
    write_bin_option(w, &v.skeleton, |w, x| w.write_varint_u64(*x as u64));
    write_bin_usize_vec(w, &v.joints);
    write_bin_option(w, &v.name, |w, x| write_bin_str(w, x));
    write_bin_json_opt(w, &v.extensions);
    write_bin_json_opt(w, &v.extras);
}
pub(crate) async fn read_bin_skin(r: &mut dsl::ByteReader<'_>) -> Result<GltfSkin, dsl::PackError> {
    Ok(GltfSkin {
        inverse_bind_matrices: read_bin_option(r, |r| Ok(r.read_varint_u64()? as usize))?,
        skeleton: read_bin_option(r, |r| Ok(r.read_varint_u64()? as usize))?,
        joints: read_bin_usize_vec(r)?,
        name: read_bin_option(r, read_bin_str)?,
        extensions: read_bin_json_opt(r)?,
        extras: read_bin_json_opt(r)?,
    })
}
//#endregion 🔖️RealBinaryTextureImageSamplerSkinGroupCodecs

//#region 🔖️RealBinaryAnimationGroupCodecs
pub(crate) async fn write_bin_animation_channel_target(w: &mut dsl::ByteWriter, t: &GltfAnimationChannelTarget) {
    write_bin_option(w, &t.node, |w, v| w.write_varint_u64(*v as u64));
    write_bin_animation_path(w, t.path);
    write_bin_json_opt(w, &t.extensions);
    write_bin_json_opt(w, &t.extras);
}
pub(crate) async fn read_bin_animation_channel_target(r: &mut dsl::ByteReader<'_>) -> Result<GltfAnimationChannelTarget, dsl::PackError> {
    Ok(GltfAnimationChannelTarget { node: read_bin_option(r, |r| Ok(r.read_varint_u64()? as usize))?, path: read_bin_animation_path(r)?, extensions: read_bin_json_opt(r)?, extras: read_bin_json_opt(r)? })
}
pub(crate) async fn write_bin_animation_channel(w: &mut dsl::ByteWriter, c: &GltfAnimationChannel) {
    w.write_varint_u64(c.sampler as u64);
    write_bin_animation_channel_target(w, &c.target);
    write_bin_json_opt(w, &c.extensions);
    write_bin_json_opt(w, &c.extras);
}
pub(crate) async fn read_bin_animation_channel(r: &mut dsl::ByteReader<'_>) -> Result<GltfAnimationChannel, dsl::PackError> {
    Ok(GltfAnimationChannel { sampler: r.read_varint_u64()? as usize, target: read_bin_animation_channel_target(r)?, extensions: read_bin_json_opt(r)?, extras: read_bin_json_opt(r)? })
}
pub(crate) async fn write_bin_animation_sampler(w: &mut dsl::ByteWriter, s: &GltfAnimationSampler) {
    w.write_varint_u64(s.input as u64);
    write_bin_interpolation(w, s.interpolation);
    w.write_varint_u64(s.output as u64);
    write_bin_json_opt(w, &s.extensions);
    write_bin_json_opt(w, &s.extras);
}
pub(crate) async fn read_bin_animation_sampler(r: &mut dsl::ByteReader<'_>) -> Result<GltfAnimationSampler, dsl::PackError> {
    Ok(GltfAnimationSampler { input: r.read_varint_u64()? as usize, interpolation: read_bin_interpolation(r)?, output: r.read_varint_u64()? as usize, extensions: read_bin_json_opt(r)?, extras: read_bin_json_opt(r)? })
}
pub(crate) async fn write_bin_animation(w: &mut dsl::ByteWriter, a: &GltfAnimation) {
    write_bin_vec(w, &a.channels, write_bin_animation_channel);
    write_bin_vec(w, &a.samplers, write_bin_animation_sampler);
    write_bin_option(w, &a.name, |w, v| write_bin_str(w, v));
    write_bin_json_opt(w, &a.extensions);
    write_bin_json_opt(w, &a.extras);
}
pub(crate) async fn read_bin_animation(r: &mut dsl::ByteReader<'_>) -> Result<GltfAnimation, dsl::PackError> {
    Ok(GltfAnimation { channels: read_bin_vec(r, read_bin_animation_channel)?, samplers: read_bin_vec(r, read_bin_animation_sampler)?, name: read_bin_option(r, read_bin_str)?, extensions: read_bin_json_opt(r)?, extras: read_bin_json_opt(r)? })
}
//#endregion 🔖️RealBinaryAnimationGroupCodecs

//#region 🔖️RealBinaryCameraGroupCodecs
pub(crate) async fn write_bin_perspective(w: &mut dsl::ByteWriter, p: &GltfPerspective) {
    write_bin_option(w, &p.aspect_ratio, |w, v| w.write_f64_le(*v));
    w.write_f64_le(p.yfov);
    write_bin_option(w, &p.zfar, |w, v| w.write_f64_le(*v));
    w.write_f64_le(p.znear);
    write_bin_json_opt(w, &p.extensions);
    write_bin_json_opt(w, &p.extras);
}
pub(crate) async fn read_bin_perspective(r: &mut dsl::ByteReader<'_>) -> Result<GltfPerspective, dsl::PackError> {
    Ok(GltfPerspective { aspect_ratio: read_bin_option(r, |r| r.read_f64_le())?, yfov: r.read_f64_le()?, zfar: read_bin_option(r, |r| r.read_f64_le())?, znear: r.read_f64_le()?, extensions: read_bin_json_opt(r)?, extras: read_bin_json_opt(r)? })
}
pub(crate) async fn write_bin_orthographic(w: &mut dsl::ByteWriter, o: &GltfOrthographic) {
    w.write_f64_le(o.xmag);
    w.write_f64_le(o.ymag);
    w.write_f64_le(o.zfar);
    w.write_f64_le(o.znear);
    write_bin_json_opt(w, &o.extensions);
    write_bin_json_opt(w, &o.extras);
}
pub(crate) async fn read_bin_orthographic(r: &mut dsl::ByteReader<'_>) -> Result<GltfOrthographic, dsl::PackError> {
    Ok(GltfOrthographic { xmag: r.read_f64_le()?, ymag: r.read_f64_le()?, zfar: r.read_f64_le()?, znear: r.read_f64_le()?, extensions: read_bin_json_opt(r)?, extras: read_bin_json_opt(r)? })
}
/// 🔀️ `GltfCameraProjection` real data-carrying enum -- tag `u8` (0=Perspective, 1=Orthographic).
pub(crate) async fn write_bin_camera_projection(w: &mut dsl::ByteWriter, p: &GltfCameraProjection) {
    match p {
        GltfCameraProjection::Perspective(v) => {
            w.write_u8(0);
            write_bin_perspective(w, v);
        }
        GltfCameraProjection::Orthographic(v) => {
            w.write_u8(1);
            write_bin_orthographic(w, v);
        }
    }
}
pub(crate) async fn read_bin_camera_projection(r: &mut dsl::ByteReader<'_>) -> Result<GltfCameraProjection, dsl::PackError> {
    match r.read_u8()? {
        0 => Ok(GltfCameraProjection::Perspective(read_bin_perspective(r)?)),
        1 => Ok(GltfCameraProjection::Orthographic(read_bin_orthographic(r)?)),
        other => Err(dsl::PackError::Malformed { what: "gltf camera_projection", offset: 0, detail: format!("unknown tag {other}") }),
    }
}
pub(crate) async fn write_bin_camera(w: &mut dsl::ByteWriter, c: &GltfCamera) {
    write_bin_camera_projection(w, &c.projection);
    write_bin_option(w, &c.name, |w, v| write_bin_str(w, v));
    write_bin_json_opt(w, &c.extensions);
    write_bin_json_opt(w, &c.extras);
}
pub(crate) async fn read_bin_camera(r: &mut dsl::ByteReader<'_>) -> Result<GltfCamera, dsl::PackError> {
    Ok(GltfCamera { projection: read_bin_camera_projection(r)?, name: read_bin_option(r, read_bin_str)?, extensions: read_bin_json_opt(r)?, extras: read_bin_json_opt(r)? })
}
//#endregion 🔖️RealBinaryCameraGroupCodecs

//#region 🔖️RealBinaryGenericCollectionCodec
/// 🧮️ Generic index-keyed collection triple real binary codec, shared by every one of the 14
/// top-level arrays -- mirrors `enc_collection`/`dec_collection`'s TEXT shape exactly, real varint
/// counts + real per-item recursive encoding (never text-as-bytes).
pub(crate) async fn write_bin_collection<T, D>(w: &mut dsl::ByteWriter, c: &GltfCollectionDiff<T, D>, write_item: impl Fn(&mut dsl::ByteWriter, &T), write_diff: impl Fn(&mut dsl::ByteWriter, &D)) {
    write_bin_vec(w, &c.removed, |w, v: &usize| w.write_varint_u64(*v as u64));
    write_bin_vec(w, &c.modified, |w, m: &GltfModified<D>| {
        w.write_varint_u64(m.index as u64);
        write_diff(w, &m.diff);
    });
    write_bin_vec(w, &c.added, |w, a: &GltfAdded<T>| {
        w.write_varint_u64(a.index as u64);
        write_item(w, &a.item);
    });
}
pub(crate) async fn read_bin_collection<T, D>(
    r: &mut dsl::ByteReader<'_>,
    read_item: impl Fn(&mut dsl::ByteReader<'_>) -> Result<T, dsl::PackError>,
    read_diff: impl Fn(&mut dsl::ByteReader<'_>) -> Result<D, dsl::PackError>,
) -> Result<GltfCollectionDiff<T, D>, dsl::PackError> {
    let removed = read_bin_usize_vec(r)?;
    let modified = read_bin_vec(r, |r| {
        let index = r.read_varint_u64()? as usize;
        let diff = read_diff(r)?;
        Ok(GltfModified { index, diff })
    })?;
    let added = read_bin_vec(r, |r| {
        let index = r.read_varint_u64()? as usize;
        let item = read_item(r)?;
        Ok(GltfAdded { index, item })
    })?;
    Ok(GltfCollectionDiff { removed, modified, added })
}
/// 🧵 A single opaque length-prefixed blob wrapping one collection's real binary encoding --
/// matches `../💾️binary/📡️component.protocol.semio`'s `Array(u8, Field(<name>_len))` fields (the
/// blob's OWN internal removed/modified/added shape isn't further protocol-walkable,
/// `protocol-prim-ref-recursion`/`protocol-array-of-records`, same documented limitation as every
/// other stdio pilot's own nested-payload field).
pub(crate) async fn write_bin_collection_blob<T, D>(c: &GltfCollectionDiff<T, D>, write_item: impl Fn(&mut dsl::ByteWriter, &T), write_diff: impl Fn(&mut dsl::ByteWriter, &D)) -> Vec<u8> {
    let mut inner = dsl::ByteWriter::new();
    write_bin_collection(&mut inner, c, write_item, write_diff);
    inner.into_bytes()
}
pub(crate) async fn read_bin_collection_blob<T, D>(
    bytes: &[u8],
    read_item: impl Fn(&mut dsl::ByteReader<'_>) -> Result<T, dsl::PackError>,
    read_diff: impl Fn(&mut dsl::ByteReader<'_>) -> Result<D, dsl::PackError>,
) -> Result<GltfCollectionDiff<T, D>, dsl::PackError> {
    let mut inner = dsl::ByteReader::new(bytes);
    read_bin_collection(&mut inner, read_item, read_diff)
}
//#endregion 🔖️RealBinaryGenericCollectionCodec

//#region 🔖️TopLevel
async fn print_gltf_diff(d: &GltfDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = &d.asset {
        tokens.push(format!("asset={}", enc_asset_diff(v)));
    }
    if let Some(v) = d.scene {
        tokens.push(format!("scene={}", encode_option(&v, |x| x.to_string())));
    }
    if let Some(v) = &d.scenes {
        tokens.push(format!("scenes={}", enc_collection(v, enc_scene, enc_scene_diff)));
    }
    if let Some(v) = &d.nodes {
        tokens.push(format!("nodes={}", enc_collection(v, enc_node, enc_node_diff)));
    }
    if let Some(v) = &d.meshes {
        tokens.push(format!("meshes={}", enc_collection(v, enc_mesh, enc_mesh_diff)));
    }
    if let Some(v) = &d.accessors {
        tokens.push(format!("accessors={}", enc_collection(v, enc_accessor, enc_accessor_diff)));
    }
    if let Some(v) = &d.buffer_views {
        tokens.push(format!("buffer-views={}", enc_collection(v, enc_buffer_view, enc_buffer_view)));
    }
    if let Some(v) = &d.buffers {
        tokens.push(format!("buffers={}", enc_collection(v, enc_buffer, enc_buffer_diff)));
    }
    if let Some(v) = &d.buffer_bytes {
        tokens.push(format!("buffer-bytes={}", enc_collection(v, |b: &Vec<u8>| enc_bytes(b), |b: &Vec<u8>| enc_bytes(b))));
    }
    if let Some(v) = &d.materials {
        tokens.push(format!("materials={}", enc_collection(v, enc_material, enc_material_diff)));
    }
    if let Some(v) = &d.textures {
        tokens.push(format!("textures={}", enc_collection(v, enc_texture, enc_texture)));
    }
    if let Some(v) = &d.images {
        tokens.push(format!("images={}", enc_collection(v, enc_image, enc_image)));
    }
    if let Some(v) = &d.samplers {
        tokens.push(format!("samplers={}", enc_collection(v, enc_sampler, enc_sampler)));
    }
    if let Some(v) = &d.skins {
        tokens.push(format!("skins={}", enc_collection(v, enc_skin, enc_skin)));
    }
    if let Some(v) = &d.animations {
        tokens.push(format!("animations={}", enc_collection(v, enc_animation, enc_animation)));
    }
    if let Some(v) = &d.cameras {
        tokens.push(format!("cameras={}", enc_collection(v, enc_camera, enc_camera)));
    }
    if let Some(v) = &d.extensions_used {
        tokens.push(format!("extensions-used={}", enc_string_vec(v)));
    }
    if let Some(v) = &d.extensions_required {
        tokens.push(format!("extensions-required={}", enc_string_vec(v)));
    }
    if let Some(v) = &d.extensions {
        tokens.push(format!("extensions={}", encode_option(v, enc_json)));
    }
    if let Some(v) = &d.extras {
        tokens.push(format!("extras={}", encode_option(v, enc_json)));
    }
    if let Some(v) = d.source_form {
        tokens.push(format!("source-form={}", enc_source_form(v)));
    }
    tokens.join(" ")
}
async fn parse_gltf_diff(line: &str) -> Result<GltfDiff, String> {
    let mut d = GltfDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("asset=") {
            d.asset = Some(dec_asset_diff(rest)?);
        } else if let Some(rest) = token.strip_prefix("scene=") {
            d.scene = Some(decode_option(rest, parse_usize)?);
        } else if let Some(rest) = token.strip_prefix("scenes=") {
            d.scenes = Some(dec_collection(rest, dec_scene, dec_scene_diff)?);
        } else if let Some(rest) = token.strip_prefix("nodes=") {
            d.nodes = Some(dec_collection(rest, dec_node, dec_node_diff)?);
        } else if let Some(rest) = token.strip_prefix("meshes=") {
            d.meshes = Some(dec_collection(rest, dec_mesh, dec_mesh_diff)?);
        } else if let Some(rest) = token.strip_prefix("accessors=") {
            d.accessors = Some(dec_collection(rest, dec_accessor, dec_accessor_diff)?);
        } else if let Some(rest) = token.strip_prefix("buffer-views=") {
            d.buffer_views = Some(dec_collection(rest, dec_buffer_view, dec_buffer_view)?);
        } else if let Some(rest) = token.strip_prefix("buffer-bytes=") {
            d.buffer_bytes = Some(dec_collection(rest, dec_bytes, dec_bytes)?);
        } else if let Some(rest) = token.strip_prefix("buffers=") {
            d.buffers = Some(dec_collection(rest, dec_buffer, dec_buffer_diff)?);
        } else if let Some(rest) = token.strip_prefix("materials=") {
            d.materials = Some(dec_collection(rest, dec_material, dec_material_diff)?);
        } else if let Some(rest) = token.strip_prefix("textures=") {
            d.textures = Some(dec_collection(rest, dec_texture, dec_texture)?);
        } else if let Some(rest) = token.strip_prefix("images=") {
            d.images = Some(dec_collection(rest, dec_image, dec_image)?);
        } else if let Some(rest) = token.strip_prefix("samplers=") {
            d.samplers = Some(dec_collection(rest, dec_sampler, dec_sampler)?);
        } else if let Some(rest) = token.strip_prefix("skins=") {
            d.skins = Some(dec_collection(rest, dec_skin, dec_skin)?);
        } else if let Some(rest) = token.strip_prefix("animations=") {
            d.animations = Some(dec_collection(rest, dec_animation, dec_animation)?);
        } else if let Some(rest) = token.strip_prefix("cameras=") {
            d.cameras = Some(dec_collection(rest, dec_camera, dec_camera)?);
        } else if let Some(rest) = token.strip_prefix("extensions-used=") {
            d.extensions_used = Some(dec_string_vec(rest)?);
        } else if let Some(rest) = token.strip_prefix("extensions-required=") {
            d.extensions_required = Some(dec_string_vec(rest)?);
        } else if let Some(rest) = token.strip_prefix("extensions=") {
            d.extensions = Some(decode_option(rest, dec_json)?);
        } else if let Some(rest) = token.strip_prefix("extras=") {
            d.extras = Some(decode_option(rest, dec_json)?);
        } else if let Some(rest) = token.strip_prefix("source-form=") {
            d.source_form = Some(dec_source_form(rest)?);
        } else {
            return Err(format!("gltf diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl protocol::DiffCodec for GltfDiff {
    async fn print_diff(&self) -> String {
        print_gltf_diff(self)
    }
    async fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        parse_gltf_diff(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    /// ⚡️ P2-FG3: real binary diff-frame — upgraded from the F6-era `print_diff().into_bytes()`
    /// text-as-binary shortcut (100% of stdio's `DiffCodec` impls were still on that shortcut per
    /// the P2-W0 census; the FG1 wave's own closer report flagged leaving this un-upgraded as a
    /// real defect to not repeat, and FG2's gif89a upgrade is this file's literal template).
    /// Matches `../💾️binary/📡️component.protocol.semio`'s real flag-per-field layout exactly,
    /// field for field, in `GltfDiff`'s own struct declaration order (2-way flag for plain
    /// `Option<T>` fields, 3-way flag for the 3 tri-state fields `scene`/`extensions`/`extras`).
    /// Every one of the 14 collection fields is one length-prefixed blob wrapping its own real
    /// binary `removed`/`modified`/`added` encoding (`write_bin_collection_blob`) — the blob's
    /// OWN internal shape isn't further protocol-walkable (`Prim::Ref` recursion gap), but this
    /// Rust side IS genuinely, fully structured real binary throughout, never text-as-bytes.
    async fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let mut w = dsl::ByteWriter::new();
        // `asset`/`extensions_used`/`extensions_required`/`extensions`/`extras` are each wrapped
        // in a length-prefixed blob (matching `../💾️binary/📡️component.protocol.semio`'s
        // `Array(u8, Field(<name>_len))` shape exactly) — NOT bare-inline like `scene`/
        // `source_form`'s fixed-width payloads — because they are NOT the last field in the frame
        // and their own internal shape has no fixed width `walk_protocol` could otherwise skip
        // past without knowing its byte length up front.
        write_bin_option(&mut w, &self.asset, |w, v| {
            write_bin_blob(w, &{
                let mut inner = dsl::ByteWriter::new();
                write_bin_asset_diff(&mut inner, v);
                inner.into_bytes()
            })
        });
        write_bin_tri(&mut w, &self.scene, |w, v| w.write_varint_u64(*v as u64));
        write_bin_option(&mut w, &self.scenes, |w, v| write_bin_blob(w, &write_bin_collection_blob(v, write_bin_scene, write_bin_scene_diff)));
        write_bin_option(&mut w, &self.nodes, |w, v| write_bin_blob(w, &write_bin_collection_blob(v, write_bin_node, write_bin_node_diff)));
        write_bin_option(&mut w, &self.meshes, |w, v| write_bin_blob(w, &write_bin_collection_blob(v, write_bin_mesh, write_bin_mesh_diff)));
        write_bin_option(&mut w, &self.accessors, |w, v| write_bin_blob(w, &write_bin_collection_blob(v, write_bin_accessor, write_bin_accessor_diff)));
        write_bin_option(&mut w, &self.buffer_views, |w, v| write_bin_blob(w, &write_bin_collection_blob(v, write_bin_buffer_view, write_bin_buffer_view)));
        write_bin_option(&mut w, &self.buffers, |w, v| write_bin_blob(w, &write_bin_collection_blob(v, write_bin_buffer, write_bin_buffer_diff)));
        write_bin_option(&mut w, &self.buffer_bytes, |w, v| write_bin_blob(w, &write_bin_collection_blob(v, |w, b: &Vec<u8>| write_bin_blob(w, b), |w, b: &Vec<u8>| write_bin_blob(w, b))));
        write_bin_option(&mut w, &self.materials, |w, v| write_bin_blob(w, &write_bin_collection_blob(v, write_bin_material, write_bin_material_diff)));
        write_bin_option(&mut w, &self.textures, |w, v| write_bin_blob(w, &write_bin_collection_blob(v, write_bin_texture, write_bin_texture)));
        write_bin_option(&mut w, &self.images, |w, v| write_bin_blob(w, &write_bin_collection_blob(v, write_bin_image, write_bin_image)));
        write_bin_option(&mut w, &self.samplers, |w, v| write_bin_blob(w, &write_bin_collection_blob(v, write_bin_sampler, write_bin_sampler)));
        write_bin_option(&mut w, &self.skins, |w, v| write_bin_blob(w, &write_bin_collection_blob(v, write_bin_skin, write_bin_skin)));
        write_bin_option(&mut w, &self.animations, |w, v| write_bin_blob(w, &write_bin_collection_blob(v, write_bin_animation, write_bin_animation)));
        write_bin_option(&mut w, &self.cameras, |w, v| write_bin_blob(w, &write_bin_collection_blob(v, write_bin_camera, write_bin_camera)));
        write_bin_option(&mut w, &self.extensions_used, |w, v| {
            write_bin_blob(w, &{
                let mut inner = dsl::ByteWriter::new();
                write_bin_string_vec(&mut inner, v);
                inner.into_bytes()
            })
        });
        write_bin_option(&mut w, &self.extensions_required, |w, v| {
            write_bin_blob(w, &{
                let mut inner = dsl::ByteWriter::new();
                write_bin_string_vec(&mut inner, v);
                inner.into_bytes()
            })
        });
        write_bin_tri(&mut w, &self.extensions, |w, v| {
            write_bin_blob(w, &{
                let mut inner = dsl::ByteWriter::new();
                write_bin_json(&mut inner, v);
                inner.into_bytes()
            })
        });
        write_bin_tri(&mut w, &self.extras, |w, v| {
            write_bin_blob(w, &{
                let mut inner = dsl::ByteWriter::new();
                write_bin_json(&mut inner, v);
                inner.into_bytes()
            })
        });
        write_bin_option(&mut w, &self.source_form, |w, v| write_bin_source_form(w, *v));
        Ok(w.into_bytes())
    }
    async fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut r = dsl::ByteReader::new(bytes);
        let asset = read_bin_option(&mut r, |r| {
            let b = read_bin_blob(r)?;
            let mut inner = dsl::ByteReader::new(&b);
            read_bin_asset_diff(&mut inner)
        })
        .map_err(gltf_bin_err)?;
        let scene = read_bin_tri(&mut r, |r| Ok(r.read_varint_u64()? as usize)).map_err(gltf_bin_err)?;
        let scenes = read_bin_option(&mut r, |r| {
            let b = read_bin_blob(r)?;
            read_bin_collection_blob(&b, read_bin_scene, read_bin_scene_diff)
        })
        .map_err(gltf_bin_err)?;
        let nodes = read_bin_option(&mut r, |r| {
            let b = read_bin_blob(r)?;
            read_bin_collection_blob(&b, read_bin_node, read_bin_node_diff)
        })
        .map_err(gltf_bin_err)?;
        let meshes = read_bin_option(&mut r, |r| {
            let b = read_bin_blob(r)?;
            read_bin_collection_blob(&b, read_bin_mesh, read_bin_mesh_diff)
        })
        .map_err(gltf_bin_err)?;
        let accessors = read_bin_option(&mut r, |r| {
            let b = read_bin_blob(r)?;
            read_bin_collection_blob(&b, read_bin_accessor, read_bin_accessor_diff)
        })
        .map_err(gltf_bin_err)?;
        let buffer_views = read_bin_option(&mut r, |r| {
            let b = read_bin_blob(r)?;
            read_bin_collection_blob(&b, read_bin_buffer_view, read_bin_buffer_view)
        })
        .map_err(gltf_bin_err)?;
        let buffers = read_bin_option(&mut r, |r| {
            let b = read_bin_blob(r)?;
            read_bin_collection_blob(&b, read_bin_buffer, read_bin_buffer_diff)
        })
        .map_err(gltf_bin_err)?;
        let buffer_bytes = read_bin_option(&mut r, |r| {
            let b = read_bin_blob(r)?;
            read_bin_collection_blob(&b, read_bin_blob, read_bin_blob)
        })
        .map_err(gltf_bin_err)?;
        let materials = read_bin_option(&mut r, |r| {
            let b = read_bin_blob(r)?;
            read_bin_collection_blob(&b, read_bin_material, read_bin_material_diff)
        })
        .map_err(gltf_bin_err)?;
        let textures = read_bin_option(&mut r, |r| {
            let b = read_bin_blob(r)?;
            read_bin_collection_blob(&b, read_bin_texture, read_bin_texture)
        })
        .map_err(gltf_bin_err)?;
        let images = read_bin_option(&mut r, |r| {
            let b = read_bin_blob(r)?;
            read_bin_collection_blob(&b, read_bin_image, read_bin_image)
        })
        .map_err(gltf_bin_err)?;
        let samplers = read_bin_option(&mut r, |r| {
            let b = read_bin_blob(r)?;
            read_bin_collection_blob(&b, read_bin_sampler, read_bin_sampler)
        })
        .map_err(gltf_bin_err)?;
        let skins = read_bin_option(&mut r, |r| {
            let b = read_bin_blob(r)?;
            read_bin_collection_blob(&b, read_bin_skin, read_bin_skin)
        })
        .map_err(gltf_bin_err)?;
        let animations = read_bin_option(&mut r, |r| {
            let b = read_bin_blob(r)?;
            read_bin_collection_blob(&b, read_bin_animation, read_bin_animation)
        })
        .map_err(gltf_bin_err)?;
        let cameras = read_bin_option(&mut r, |r| {
            let b = read_bin_blob(r)?;
            read_bin_collection_blob(&b, read_bin_camera, read_bin_camera)
        })
        .map_err(gltf_bin_err)?;
        let extensions_used = read_bin_option(&mut r, |r| {
            let b = read_bin_blob(r)?;
            let mut inner = dsl::ByteReader::new(&b);
            read_bin_string_vec(&mut inner)
        })
        .map_err(gltf_bin_err)?;
        let extensions_required = read_bin_option(&mut r, |r| {
            let b = read_bin_blob(r)?;
            let mut inner = dsl::ByteReader::new(&b);
            read_bin_string_vec(&mut inner)
        })
        .map_err(gltf_bin_err)?;
        let extensions = read_bin_tri(&mut r, |r| {
            let b = read_bin_blob(r)?;
            let mut inner = dsl::ByteReader::new(&b);
            read_bin_json(&mut inner)
        })
        .map_err(gltf_bin_err)?;
        let extras = read_bin_tri(&mut r, |r| {
            let b = read_bin_blob(r)?;
            let mut inner = dsl::ByteReader::new(&b);
            read_bin_json(&mut inner)
        })
        .map_err(gltf_bin_err)?;
        let source_form = read_bin_option(&mut r, read_bin_source_form).map_err(gltf_bin_err)?;
        Ok(GltfDiff { asset, scene, scenes, nodes, meshes, accessors, buffer_views, buffers, buffer_bytes, materials, textures, images, samplers, skins, animations, cameras, extensions_used, extensions_required, extensions, extras, source_form })
    }
}
//#endregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::gltf::STDIO_GLTF_DOCUMENT_SCHEMA;

    //#region 🔖️Fixtures
    async fn scene(seed: usize) -> GltfScene {
        GltfScene { nodes: vec![seed, seed + 1], name: Some(format!("scene{seed}")), extensions: None, extras: None }
    }
    async fn node(seed: usize) -> GltfNode {
        GltfNode { children: vec![seed], mesh: Some(seed), name: Some(format!("node{seed}")), ..GltfNode::default() }
    }
    async fn mesh(seed: usize) -> GltfMesh {
        GltfMesh { primitives: vec![GltfPrimitive { attributes: vec![("POSITION".into(), seed)], material: Some(seed), mode: Some(4), ..GltfPrimitive::default() }], name: Some(format!("mesh{seed}")), ..GltfMesh::default() }
    }
    async fn accessor(seed: usize) -> GltfAccessor {
        GltfAccessor {
            buffer_view: Some(seed),
            byte_offset: 0,
            component_type: GltfComponentType::Float,
            normalized: false,
            count: seed + 1,
            kind: GltfAccessorType::Vec3,
            max: Some(vec![1.0]),
            min: Some(vec![0.0]),
            sparse: None,
            name: Some(format!("acc{seed}")),
            extensions: None,
            extras: None,
        }
    }
    async fn material(seed: usize) -> GltfMaterial {
        GltfMaterial { name: Some(format!("mat{seed}")), double_sided: seed % 2 == 0, ..GltfMaterial::default() }
    }
    async fn buffer_meta(seed: usize) -> GltfBuffer {
        GltfBuffer { byte_length: seed * 4, uri: None, name: Some(format!("buf{seed}")), extensions: None, extras: None }
    }
    async fn animation(seed: usize) -> GltfAnimation {
        GltfAnimation { name: Some(format!("anim{seed}")), ..GltfAnimation::default() }
    }

    async fn base_snapshot() -> GltfSnapshot {
        GltfSnapshot {
            schema: STDIO_GLTF_DOCUMENT_SCHEMA.into(),
            document: GltfDocument {
                asset: GltfAsset { version: "2.0".into(), generator: Some("semio".into()), ..GltfAsset::default() },
                scene: Some(0),
                scenes: vec![scene(0), scene(1)],
                nodes: vec![node(0), node(1), node(2)],
                meshes: vec![mesh(0), mesh(1)],
                accessors: vec![accessor(0), accessor(1)],
                buffer_views: vec![],
                buffers: vec![buffer_meta(0)],
                materials: vec![material(0), material(1)],
                animations: vec![animation(0)],
                extensions_used: vec!["KHR_materials_unlit".into()],
                ..GltfDocument::default()
            },
            buffers: vec![vec![1, 2, 3, 4]],
            source_form: GltfSourceForm::Json,
        }
    }
    //#endregion 🔖️Fixtures

    //#region 🔖️AbsorbCanonicalCases
    /// 🧪️ Canonical absorb case 1: `Insert(2,x)` then `Remove(0)` → `{removed:[0], added:[(1,x)]}`.
    #[semio_framework_async_macros::async_test]
    async fn absorb_law_insert_then_remove_before_shifts_index() {
        let n = node(9);
        let mut d1 = GltfNodesDiff { added: vec![GltfAdded { index: 2, item: n.clone() }], ..Default::default() };
        let d2 = GltfNodesDiff { removed: vec![0], ..Default::default() };
        d1.absorb(d2);
        assert_eq!(d1.removed, vec![0]);
        assert_eq!(d1.added, vec![GltfAdded { index: 1, item: n }]);
        assert!(d1.modified.is_empty());
    }

    /// 🧪️ Canonical absorb case 2: `Insert(2,f)` then `Insert(2,g)` → BOTH survive.
    #[semio_framework_async_macros::async_test]
    async fn absorb_law_insert_insert_same_index_both_survive() {
        let f = node(1);
        let g = node(2);
        let mut d1 = GltfNodesDiff { added: vec![GltfAdded { index: 2, item: f.clone() }], ..Default::default() };
        let d2 = GltfNodesDiff { added: vec![GltfAdded { index: 2, item: g.clone() }], ..Default::default() };
        d1.absorb(d2);
        assert_eq!(d1.added, vec![GltfAdded { index: 2, item: g }, GltfAdded { index: 3, item: f }]);
    }

    /// 🧪️ Canonical absorb case 3: `Insert(1,f)` then `SetField(1,name)` patches INTO the added
    /// payload -- merged has only `added`, no separate `modified` entry.
    #[semio_framework_async_macros::async_test]
    async fn absorb_law_insert_then_set_field_patches_into_added() {
        let f = node(1);
        let mut d1 = GltfNodesDiff { added: vec![GltfAdded { index: 1, item: f.clone() }], ..Default::default() };
        let d2 = GltfNodesDiff { modified: vec![GltfModified { index: 1, diff: GltfNodeDiff { name: Some(Some("renamed".into())), ..Default::default() } }], ..Default::default() };
        d1.absorb(d2);
        assert!(d1.modified.is_empty());
        assert_eq!(d1.added.len(), 1);
        assert_eq!(d1.added[0].item.name, Some("renamed".to_string()));
        assert_eq!(d1.added[0].index, 1);
    }

    /// 🧪️ Canonical absorb case 4 (id-keyed-collection-analog / modify-of-removed): `Remove(0)`
    /// then `Modify(0)` (post-remove index 0 refers to a DIFFERENT surviving base item) must NOT
    /// corrupt the removed item and must attach the d2 patch to the correct transported base index.
    #[semio_framework_async_macros::async_test]
    async fn absorb_law_remove_then_modify_transports_to_correct_surviving_item() {
        let mut d1 = GltfNodesDiff { removed: vec![0], ..Default::default() };
        // after d1, base[1] is now at position 0 -- d2 modifies position 0 (== base[1]).
        let d2 = GltfNodesDiff { modified: vec![GltfModified { index: 0, diff: GltfNodeDiff { name: Some(Some("x".into())), ..Default::default() } }], ..Default::default() };
        d1.absorb(d2);
        assert_eq!(d1.removed, vec![0]);
        assert_eq!(d1.modified.len(), 1);
        assert_eq!(d1.modified[0].index, 1, "d2's patch at post-remove position 0 must transport to BASE index 1");
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_law_holds_over_curated_ops() {
        let base = base_snapshot();
        let mid = {
            let mut s = base.clone();
            s.document.nodes.insert(1, node(9));
            s.document.nodes.remove(0);
            s.document.materials.push(material(5));
            s
        };
        let after = {
            let mut s = mid.clone();
            s.document.nodes[0].name = Some("renamed-in-after".into());
            s.document.scenes.push(scene(7));
            s.document.materials.remove(0);
            s.buffers.push(vec![9, 9]);
            s.document.buffers.push(buffer_meta(9));
            s
        };
        let mut d1 = <GltfDiff as DiffAlgebra<GltfSnapshot>>::between(&base, &mid);
        let d2 = <GltfDiff as DiffAlgebra<GltfSnapshot>>::between(&mid, &after);
        d1.absorb(d2);
        assert_eq!(MutationDiff::apply(&d1, &base).expect("apply must succeed for a well-formed fixture"), after);
    }
    //#endregion 🔖️AbsorbCanonicalCases

    //#region 🔖️BetweenRoundtripLaw
    #[semio_framework_async_macros::async_test]
    async fn between_roundtrip_law_holds_on_synthetic_fixture() {
        let a = base_snapshot();
        let mut b = a.clone();
        b.document.nodes.push(node(5));
        b.document.asset.generator = Some("other-tool".into());
        b.source_form = GltfSourceForm::Glb;
        let ab = <GltfDiff as DiffAlgebra<GltfSnapshot>>::between(&a, &b);
        assert_eq!(MutationDiff::apply(&ab, &a).expect("apply must succeed for a well-formed fixture"), b);
        let ba = <GltfDiff as DiffAlgebra<GltfSnapshot>>::between(&b, &a);
        assert_eq!(MutationDiff::apply(&ba, &b).expect("apply must succeed for a well-formed fixture"), a);
        assert!(<GltfDiff as DiffAlgebra<GltfSnapshot>>::between(&a, &a).is_empty());
    }
    //#endregion 🔖️BetweenRoundtripLaw

    //#region 🔖️InverseLaw
    #[semio_framework_async_macros::async_test]
    async fn inverse_law_diff_level_round_trips() {
        let base = base_snapshot();
        let next = {
            let mut s = base.clone();
            s.document.nodes[0].mesh = None;
            s.document.nodes.remove(1);
            s.document.nodes.push(node(8));
            s.document.extensions_used.clear();
            s.document.materials[0].alpha_mode = GltfAlphaMode::Blend;
            s
        };
        let d = <GltfDiff as DiffAlgebra<GltfSnapshot>>::between(&base, &next);
        let mutated = MutationDiff::apply(&d, &base).expect("apply must succeed for a well-formed fixture");
        let inv = <GltfDiff as DiffAlgebra<GltfSnapshot>>::inverse(&d, &base);
        assert_eq!(MutationDiff::apply(&inv, &mutated).expect("apply must succeed for a well-formed fixture"), base);
    }
    //#endregion 🔖️InverseLaw

    //#region 🔖️FieldSweep
    /// 🎯️ Shared field-sweep fixture: `sweep_a`/`sweep_b` differ in EVERY mutable field, incl.
    /// every tri-state exercising `Some(None)`, with asymmetric collection lengths split across
    /// both `between()` directions (F1's structural trap). Factored out of
    /// `field_sweep_covers_every_mutable_field` (its original owner) so `diff_codec_text_binary_
    /// roundtrip_law` (`HandcraftedDiffCodec` tests, further down) can reuse the exact same
    /// comprehensive diff rather than re-deriving a second copy. `pub(super)` (not private) so the
    /// sibling `handcrafted_diff_codec_tests` module can reach it via `super::tests::sweep_a()`.
    pub(super) async fn sweep_a() -> GltfSnapshot {
        GltfSnapshot {
            schema: STDIO_GLTF_DOCUMENT_SCHEMA.into(),
            document: GltfDocument {
                asset: GltfAsset { version: "2.0".into(), generator: Some("a-tool".into()), copyright: Some("(c) a".into()), min_version: Some("2.0".into()), extensions: Some(GltfJson::Bool(true)), extras: Some(GltfJson::String("a".into())) },
                scene: Some(0),
                scenes: vec![scene(0), scene(1)],
                nodes: vec![node(0), node(1)],
                meshes: vec![mesh(0), mesh(1)],
                accessors: vec![accessor(0), accessor(1)],
                buffer_views: vec![GltfBufferView { buffer: 0, byte_offset: 0, byte_length: 4, byte_stride: None, target: None, name: None, extensions: None, extras: None }],
                buffers: vec![buffer_meta(0), buffer_meta(1)],
                materials: vec![material(0), material(1)],
                textures: vec![GltfTexture { sampler: Some(0), source: Some(0), name: None, extensions: None, extras: None }],
                images: vec![GltfImage { uri: Some("a.png".into()), ..Default::default() }],
                samplers: vec![GltfSampler::default()],
                skins: vec![GltfSkin { joints: vec![0, 1], ..Default::default() }],
                animations: vec![animation(0), animation(1)],
                cameras: vec![],
                extensions_used: vec!["KHR_a".into()],
                extensions_required: vec!["KHR_a".into()],
                extensions: Some(GltfJson::Object(vec![("KHR_a".into(), GltfJson::Object(vec![]))])),
                extras: Some(GltfJson::String("sweep-a-extras".into())),
            },
            buffers: vec![vec![1, 2], vec![3, 4]],
            source_form: GltfSourceForm::Json,
        }
    }
    pub(super) async fn sweep_b() -> GltfSnapshot {
        GltfSnapshot {
            schema: STDIO_GLTF_DOCUMENT_SCHEMA.into(),
            document: GltfDocument {
                asset: GltfAsset { version: "2.1".into(), generator: None, copyright: None, min_version: None, extensions: None, extras: None },
                scene: None,
                scenes: vec![scene(9)],
                nodes: vec![node(9), node(10), node(11)],
                meshes: vec![mesh(9)],
                accessors: vec![accessor(9)],
                buffer_views: vec![],
                buffers: vec![buffer_meta(9)],
                materials: vec![material(9), material(10), material(11)],
                textures: vec![],
                images: vec![],
                samplers: vec![],
                skins: vec![],
                animations: vec![],
                cameras: vec![GltfCamera {
                    projection: GltfCameraProjection::Perspective(GltfPerspective {
                        aspect_ratio: Some(1.5),
                        yfov: 0.8,
                        zfar: Some(100.0),
                        znear: 0.1,
                        extensions: None,
                        extras: None,
                    }),
                    name: Some("cam".into()),
                    extensions: None,
                    extras: None,
                }],
                extensions_used: vec![],
                extensions_required: vec![],
                extensions: None,
                extras: None,
            },
            buffers: vec![vec![9]],
            source_form: GltfSourceForm::Glb,
        }
    }

    /// 🧪️ Field sweep — the acceptance criterion: `sweep_a`/`sweep_b` differ in EVERY mutable
    /// field, incl. every tri-state exercising `Some(None)`, with asymmetric collection lengths
    /// split across both `between()` directions (F1's structural trap).
    #[semio_framework_async_macros::async_test]
    async fn field_sweep_covers_every_mutable_field() {
        let sweep_a = sweep_a();
        let sweep_b = sweep_b();

        let ab = <GltfDiff as DiffAlgebra<GltfSnapshot>>::between(&sweep_a, &sweep_b);
        assert_eq!(MutationDiff::apply(&ab, &sweep_a).expect("apply must succeed for a well-formed fixture"), sweep_b);
        assert!(ab.asset.is_some());
        assert_eq!(ab.scene, Some(None), "scene going Some->None must be tri-state Some(None)");
        assert!(ab.scenes.is_some());
        assert!(ab.nodes.is_some());
        assert!(ab.meshes.is_some());
        assert!(ab.accessors.is_some());
        assert!(ab.buffer_views.is_some());
        assert!(ab.buffers.is_some());
        assert!(ab.buffer_bytes.is_some());
        assert!(ab.materials.is_some());
        assert!(ab.textures.is_some());
        assert!(ab.images.is_some());
        assert!(ab.samplers.is_some());
        assert!(ab.skins.is_some());
        assert!(ab.animations.is_some());
        assert!(ab.cameras.is_some());
        assert!(ab.extensions_used.is_some());
        assert!(ab.extensions_required.is_some());
        assert!(ab.source_form.is_some());
        assert_eq!(ab.extensions, Some(None), "document.extensions going Some->None must be tri-state Some(None)");
        assert_eq!(ab.extras, Some(None), "document.extras going Some->None must be tri-state Some(None)");
        let nodes_ab = ab.nodes.as_ref().unwrap();
        assert!(!nodes_ab.modified.is_empty() || !nodes_ab.added.is_empty());
        assert!(!nodes_ab.added.is_empty(), "sweep must exercise an added node (b is longer)");

        let ba = <GltfDiff as DiffAlgebra<GltfSnapshot>>::between(&sweep_b, &sweep_a);
        assert_eq!(MutationDiff::apply(&ba, &sweep_b).expect("apply must succeed for a well-formed fixture"), sweep_a);
        let nodes_ba = ba.nodes.as_ref().unwrap();
        assert!(!nodes_ba.removed.is_empty(), "reverse direction must exercise a removed node (a is shorter, b is longer)");
        let cameras_ba = ba.cameras.as_ref().unwrap();
        assert!(!cameras_ba.removed.is_empty(), "reverse direction must exercise a removed camera");

        assert!(<GltfDiff as DiffAlgebra<GltfSnapshot>>::between(&sweep_a, &sweep_a).is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn touched_regions_are_stable_precise_for_modification_and_conservative_for_transport() {
        use protocol::DiffRegions as _;
        let modified = GltfDiff {
            nodes: Some(GltfNodesDiff { modified: vec![GltfModified { index: 3, diff: GltfNodeDiff { translation: Some(Some([1.0, 2.0, 3.0])), ..Default::default() } }], ..Default::default() }),
            buffer_bytes: Some(GltfBufferBytesDiff { modified: vec![GltfModified { index: 2, diff: vec![1, 2] }], ..Default::default() }),
            ..Default::default()
        };
        assert_eq!(modified.touches().paths, vec!["buffers/2", "document/nodes/3/transform"]);
        let structural = GltfDiff { nodes: Some(GltfNodesDiff { removed: vec![1], ..Default::default() }), ..Default::default() };
        assert_eq!(structural.touches().paths, vec!["document/nodes"]);
    }
    //#endregion 🔖️FieldSweep
}
//#endregion 🧪️Tests

//#region 🧪️HandcraftedDiffCodecTests
#[cfg(test)]
mod handcrafted_diff_codec_tests {
    use super::*;
    use crate::artifacts::gltf::STDIO_GLTF_DOCUMENT_SCHEMA;
    use protocol::DiffCodec;

    //#region 🔖️Fixtures
    async fn node_tristate_a() -> GltfNode {
        GltfNode {
            children: vec![0, 1],
            mesh: Some(0),
            camera: Some(0),
            skin: Some(0),
            matrix: Some([1.0; 16]),
            translation: None,
            rotation: None,
            scale: None,
            weights: vec![0.5, 0.5],
            name: Some("n-a".into()),
            extensions: Some(GltfJson::Bool(true)),
            extras: None,
        }
    }
    /// 🎯️ Every one of `node_tristate_a`'s nullable fields flips the OTHER way (`Some -> None` OR
    /// `None -> Some`), so `between()` exercises `Some(None)` on `mesh`/`camera`/`skin`/`matrix`/
    /// `extensions` AND `Some(Some(_))` on `translation`/`rotation`/`scale`/`extras` in one pair.
    async fn node_tristate_b() -> GltfNode {
        GltfNode {
            children: vec![2],
            mesh: None,
            camera: None,
            skin: None,
            matrix: None,
            translation: Some([1.0, 2.0, 3.0]),
            rotation: Some([0.0, 0.0, 0.0, 1.0]),
            scale: Some([2.0, 2.0, 2.0]),
            weights: vec![],
            name: None,
            extensions: None,
            extras: Some(GltfJson::Array(vec![GltfJson::Null, GltfJson::Number(-1.5), GltfJson::String("x".into())])),
        }
    }
    async fn accessor_sparse_a() -> GltfAccessor {
        GltfAccessor { buffer_view: Some(1), byte_offset: 0, component_type: GltfComponentType::UnsignedShort, normalized: true, count: 4, kind: GltfAccessorType::Vec2, max: None, min: None, sparse: None, name: None, extensions: None, extras: None }
    }
    /// 🎯️ `sparse` flips `None -> Some(GltfSparseAccessor{..})` -- the one accessor field not
    /// exercised by `sweep_a`/`sweep_b` above.
    async fn accessor_sparse_b() -> GltfAccessor {
        GltfAccessor {
            sparse: Some(GltfSparseAccessor { count: 2, indices: GltfSparseIndices { buffer_view: 2, byte_offset: 0, component_type: GltfComponentType::UnsignedByte }, values: GltfSparseValues { buffer_view: 3, byte_offset: 0 } }),
            max: Some(vec![1.0, 1.0]),
            ..accessor_sparse_a()
        }
    }
    async fn material_textures_a() -> GltfMaterial {
        GltfMaterial::default()
    }
    /// 🎯️ Every optional texture slot (`pbr_metallic_roughness`/`normal_texture`/
    /// `occlusion_texture`/`emissive_texture`) flips `None -> Some(_)` -- none of which
    /// `sweep_a`/`sweep_b` above touch (both leave `GltfMaterial::default()`'s texture slots at
    /// `None`).
    async fn material_textures_b() -> GltfMaterial {
        GltfMaterial {
            pbr_metallic_roughness: Some(GltfPbrMetallicRoughness { base_color_texture: Some(GltfTextureInfo { index: 0, tex_coord: 1, extensions: None, extras: None }), ..Default::default() }),
            normal_texture: Some(GltfNormalTextureInfo { index: 1, tex_coord: 0, scale: 2.0, extensions: None, extras: None }),
            occlusion_texture: Some(GltfOcclusionTextureInfo { index: 2, tex_coord: 0, strength: 0.5, extensions: None, extras: None }),
            emissive_texture: Some(GltfTextureInfo { index: 3, tex_coord: 0, extensions: None, extras: None }),
            alpha_mode: GltfAlphaMode::Mask,
            ..GltfMaterial::default()
        }
    }
    async fn buffer_uri_a() -> GltfBuffer {
        GltfBuffer { byte_length: 4, uri: Some("data:...".into()), name: None, extensions: None, extras: None }
    }
    async fn buffer_uri_b() -> GltfBuffer {
        GltfBuffer { uri: None, ..buffer_uri_a() }
    }
    async fn camera_orthographic() -> GltfCamera {
        GltfCamera {
            projection: GltfCameraProjection::Orthographic(GltfOrthographic { xmag: 1.0, ymag: 1.0, zfar: 10.0, znear: 0.1, extensions: None, extras: Some(GltfJson::Null) }),
            name: None,
            extensions: None,
            extras: None,
        }
    }

    async fn tristate_snapshot_a() -> GltfSnapshot {
        GltfSnapshot {
            schema: STDIO_GLTF_DOCUMENT_SCHEMA.into(),
            document: GltfDocument {
                asset: GltfAsset::default(),
                nodes: vec![node_tristate_a()],
                accessors: vec![accessor_sparse_a()],
                materials: vec![material_textures_a()],
                buffers: vec![buffer_uri_a()],
                cameras: vec![],
                ..GltfDocument::default()
            },
            buffers: vec![vec![1, 2, 3]],
            source_form: GltfSourceForm::Json,
        }
    }
    async fn tristate_snapshot_b() -> GltfSnapshot {
        GltfSnapshot {
            schema: STDIO_GLTF_DOCUMENT_SCHEMA.into(),
            document: GltfDocument {
                asset: GltfAsset::default(),
                nodes: vec![node_tristate_b()],
                accessors: vec![accessor_sparse_b()],
                materials: vec![material_textures_b()],
                buffers: vec![buffer_uri_b()],
                cameras: vec![camera_orthographic()],
                ..GltfDocument::default()
            },
            buffers: vec![vec![1, 2, 3]],
            source_form: GltfSourceForm::Json,
        }
    }
    //#endregion 🔖️Fixtures

    /// 🧪️ F6: `DiffCodec` round-trip laws over the hand-rolled `GltfDiff` grammar. Exercises a
    /// representative SUBSET of the 42 tri-state fields (not literally all 42, per the F6 brief):
    /// (1) `sweep_a()`/`sweep_b()` (this file's own `field_sweep_covers_every_mutable_field`
    /// fixture, reused verbatim) — every top-level `GltfDiff` field populated at least once
    /// (`asset`/`scene`/all 14 collections/`extensions_used`/`extensions_required`/`extensions`/
    /// `extras`/`source_form`), a `Perspective` camera, and every `GltfAssetDiff` tri-state field
    /// going `Some -> None`; (2) `tristate_snapshot_a/b` — the tri-state fields `sweep_a`/`sweep_b`
    /// do NOT touch: `GltfNodeDiff::mesh/camera/skin/matrix` going `Some(Some) -> Some(None)`,
    /// `translation/rotation/scale` going `Some(None) -> Some(Some)` (both tri-state directions on
    /// the SAME collection-modified entry), `GltfAccessorDiff::sparse` going `None -> Some`,
    /// `GltfMaterialDiff::pbr_metallic_roughness/normal_texture/occlusion_texture/emissive_texture`
    /// all going `None -> Some`, `GltfBufferDiff::uri` going `Some -> None`, an `Orthographic`
    /// camera (the OTHER `GltfCameraProjection` variant `sweep_b` doesn't use), and `GltfJson`'s
    /// `Null`/`Number`/`Array` variants (`sweep_a`/`sweep_b` only exercise `Bool`/`String`/
    /// `Object`) -- together (1)+(2) cover every `GltfJson` variant, both `GltfCameraProjection`
    /// variants, and at least one tri-state field per STRONG entity diff type
    /// (`Asset`/`Scene`/`Node`/`Mesh`/`Accessor`/`Material`/`Buffer`), which is the representative
    /// slice this law test commits to (documented here, not literally all 42 occurrences).
    #[semio_framework_async_macros::async_test]
    async fn diff_codec_text_binary_roundtrip_law() {
        let sweep_a = tests::sweep_a();
        let sweep_b = tests::sweep_b();
        let tri_a = tristate_snapshot_a();
        let tri_b = tristate_snapshot_b();

        let cases = vec![
            GltfDiff::default(),
            <GltfDiff as DiffAlgebra<GltfSnapshot>>::between(&sweep_a, &sweep_b),
            <GltfDiff as DiffAlgebra<GltfSnapshot>>::between(&sweep_b, &sweep_a),
            <GltfDiff as DiffAlgebra<GltfSnapshot>>::between(&tri_a, &tri_b),
            <GltfDiff as DiffAlgebra<GltfSnapshot>>::between(&tri_b, &tri_a),
        ];
        for d in cases {
            let printed = d.print_diff();
            assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
            let parsed = GltfDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
            assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

            let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
            let decoded = GltfDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
            assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
        }
    }
}
//#endregion 🧪️HandcraftedDiffCodecTests
