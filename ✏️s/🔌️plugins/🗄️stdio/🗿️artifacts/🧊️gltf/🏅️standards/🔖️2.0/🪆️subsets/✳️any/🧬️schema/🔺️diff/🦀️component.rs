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

use crate::artifacts::gltf::schema::snapshot::{
    GltfAccessor, GltfAlphaMode, GltfAnimation, GltfAnimationChannel, GltfAnimationChannelTarget, GltfAnimationPath,
    GltfAnimationSampler, GltfAsset, GltfBuffer, GltfBufferView, GltfCamera, GltfCameraProjection, GltfDocument,
    GltfImage, GltfInterpolation, GltfJson, GltfMaterial, GltfMesh, GltfNode, GltfNormalTextureInfo,
    GltfOcclusionTextureInfo, GltfOrthographic, GltfPbrMetallicRoughness, GltfPerspective, GltfPrimitive,
    GltfSampler, GltfScene, GltfSkin, GltfSnapshot, GltfSourceForm, GltfSparseAccessor, GltfSparseIndices,
    GltfSparseValues, GltfTexture, GltfTextureInfo,
};
use crate::artifacts::gltf::engine::{GltfAccessorType, GltfComponentType};
use protocol::MutationDiff;
use protocol::os_spr::command::DiffAlgebra;
use serde::{Deserialize, Serialize};
use schema::ArtifactSchema;

//#region 🔖️IndexTransport
/// 📐️ Shared rank/unrank arithmetic for index-keyed collection diffs (`between`/`absorb`/
/// `inverse`) — see `🧬️schema-design.md` §Absorb and the plan's "Absorb" section for the
/// derivation. `excluded_sorted` must be sorted ascending.
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

//#region 🔖️GenericCollectionAlgebra
/// 🧮️ Sequential-coalesce absorb for an index-keyed collection triple, generic over the item type
/// `T` and its per-item diff type `D`. Canonical correctness verified against the plan's 3
/// mandated cases in this module's tests. See `🧬️schema-design.md` §Absorb.
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
            modified_map
                .entry(base_index)
                .and_modify(|d| absorb_diff(d, dd2.clone()))
                .or_insert(dd2);
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
//#endregion 🔖️GenericCollectionAlgebra

//#region 🔖️ItemDiffTrait
/// 🧩️ A per-item diff for collection element type `T` -- implemented by real per-field diff
/// structs for STRONG entities (`GltfNodeDiff`, `GltfMeshDiff`, …), and by the blanket `T for T`
/// impl below for WEAK entities (the "diff" IS the whole new value).
pub(crate) trait ItemDiff<T>: Clone + PartialEq {
    fn between(base: &T, other: &T) -> Self;
    fn apply(&self, base: &T) -> T;
    fn inverse(&self, base: &T) -> Self;
    fn absorb_into(&mut self, other: Self);
}

/// 🍃️ WEAK entities: the diff type IS the item type (whole-value replace), per the recipe's
/// strong/weak split -- no further sub-structure worth diffing.
impl<T: Clone + PartialEq> ItemDiff<T> for T {
    fn between(_base: &T, other: &T) -> Self { other.clone() }
    fn apply(&self, _base: &T) -> T { self.clone() }
    fn inverse(&self, base: &T) -> Self { base.clone() }
    fn absorb_into(&mut self, other: Self) { *self = other; }
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
#[serde(bound(
    serialize = "T: Serialize, D: Serialize",
    deserialize = "T: Deserialize<'de>, D: Deserialize<'de>"
))]
pub struct GltfCollectionDiff<T, D> {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<GltfModified<D>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<GltfAdded<T>>,
}

impl<T, D> Default for GltfCollectionDiff<T, D> {
    fn default() -> Self { Self { removed: Vec::new(), modified: Vec::new(), added: Vec::new() } }
}

impl<T: Clone + PartialEq, D: ItemDiff<T>> GltfCollectionDiff<T, D> {
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }

    pub fn between(base: &[T], other: &[T]) -> Self {
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

    pub fn apply(&self, base: &[T]) -> Vec<T> {
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
            if r < next.len() { next.remove(r); }
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

    pub fn absorb(&mut self, other: Self) {
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

    pub fn inverse(&self, base_items: &[T]) -> Self {
        let (removed, modified, added) = inverse_indexed_collection(
            &self.removed,
            &self.modified.iter().map(|m| (m.index, m.diff.clone())).collect::<Vec<_>>(),
            &self.added.iter().map(|a| (a.index, a.item.clone())).collect::<Vec<_>>(),
            base_items,
            |d, item| d.inverse(item),
        );
        Self {
            removed,
            modified: modified.into_iter().map(|(index, diff)| GltfModified { index, diff }).collect(),
            added: added.into_iter().map(|(index, item)| GltfAdded { index, item }).collect(),
        }
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
    pub fn is_empty(&self) -> bool {
        self.version.is_none() && self.generator.is_none() && self.copyright.is_none()
            && self.min_version.is_none() && self.extensions.is_none() && self.extras.is_none()
    }
    pub fn between(base: &GltfAsset, other: &GltfAsset) -> Self {
        Self {
            version: (base.version != other.version).then(|| other.version.clone()),
            generator: (base.generator != other.generator).then(|| other.generator.clone()),
            copyright: (base.copyright != other.copyright).then(|| other.copyright.clone()),
            min_version: (base.min_version != other.min_version).then(|| other.min_version.clone()),
            extensions: (base.extensions != other.extensions).then(|| other.extensions.clone()),
            extras: (base.extras != other.extras).then(|| other.extras.clone()),
        }
    }
    pub fn apply(&self, base: &GltfAsset) -> GltfAsset {
        let mut next = base.clone();
        if let Some(v) = &self.version { next.version = v.clone(); }
        if let Some(v) = &self.generator { next.generator = v.clone(); }
        if let Some(v) = &self.copyright { next.copyright = v.clone(); }
        if let Some(v) = &self.min_version { next.min_version = v.clone(); }
        if let Some(v) = &self.extensions { next.extensions = v.clone(); }
        if let Some(v) = &self.extras { next.extras = v.clone(); }
        next
    }
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
    pub fn absorb(&mut self, other: Self) {
        if other.version.is_some() { self.version = other.version; }
        if other.generator.is_some() { self.generator = other.generator; }
        if other.copyright.is_some() { self.copyright = other.copyright; }
        if other.min_version.is_some() { self.min_version = other.min_version; }
        if other.extensions.is_some() { self.extensions = other.extensions; }
        if other.extras.is_some() { self.extras = other.extras; }
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
    fn between(base: &GltfScene, other: &GltfScene) -> Self {
        Self {
            nodes: (base.nodes != other.nodes).then(|| other.nodes.clone()),
            name: (base.name != other.name).then(|| other.name.clone()),
            extensions: (base.extensions != other.extensions).then(|| other.extensions.clone()),
            extras: (base.extras != other.extras).then(|| other.extras.clone()),
        }
    }
    fn apply(&self, base: &GltfScene) -> GltfScene {
        let mut next = base.clone();
        if let Some(v) = &self.nodes { next.nodes = v.clone(); }
        if let Some(v) = &self.name { next.name = v.clone(); }
        if let Some(v) = &self.extensions { next.extensions = v.clone(); }
        if let Some(v) = &self.extras { next.extras = v.clone(); }
        next
    }
    fn inverse(&self, base: &GltfScene) -> Self {
        Self {
            nodes: self.nodes.as_ref().map(|_| base.nodes.clone()),
            name: self.name.as_ref().map(|_| base.name.clone()),
            extensions: self.extensions.as_ref().map(|_| base.extensions.clone()),
            extras: self.extras.as_ref().map(|_| base.extras.clone()),
        }
    }
    fn absorb_into(&mut self, other: Self) {
        if other.nodes.is_some() { self.nodes = other.nodes; }
        if other.name.is_some() { self.name = other.name; }
        if other.extensions.is_some() { self.extensions = other.extensions; }
        if other.extras.is_some() { self.extras = other.extras; }
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
    fn between(base: &GltfNode, other: &GltfNode) -> Self {
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
    fn apply(&self, base: &GltfNode) -> GltfNode {
        let mut next = base.clone();
        if let Some(v) = &self.children { next.children = v.clone(); }
        if let Some(v) = self.mesh { next.mesh = v; }
        if let Some(v) = self.camera { next.camera = v; }
        if let Some(v) = self.skin { next.skin = v; }
        if let Some(v) = self.matrix { next.matrix = v; }
        if let Some(v) = self.translation { next.translation = v; }
        if let Some(v) = self.rotation { next.rotation = v; }
        if let Some(v) = self.scale { next.scale = v; }
        if let Some(v) = &self.weights { next.weights = v.clone(); }
        if let Some(v) = &self.name { next.name = v.clone(); }
        if let Some(v) = &self.extensions { next.extensions = v.clone(); }
        if let Some(v) = &self.extras { next.extras = v.clone(); }
        next
    }
    fn inverse(&self, base: &GltfNode) -> Self {
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
    fn absorb_into(&mut self, other: Self) {
        if other.children.is_some() { self.children = other.children; }
        if other.mesh.is_some() { self.mesh = other.mesh; }
        if other.camera.is_some() { self.camera = other.camera; }
        if other.skin.is_some() { self.skin = other.skin; }
        if other.matrix.is_some() { self.matrix = other.matrix; }
        if other.translation.is_some() { self.translation = other.translation; }
        if other.rotation.is_some() { self.rotation = other.rotation; }
        if other.scale.is_some() { self.scale = other.scale; }
        if other.weights.is_some() { self.weights = other.weights; }
        if other.name.is_some() { self.name = other.name; }
        if other.extensions.is_some() { self.extensions = other.extensions; }
        if other.extras.is_some() { self.extras = other.extras; }
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
    fn between(base: &GltfMesh, other: &GltfMesh) -> Self {
        Self {
            primitives: (base.primitives != other.primitives).then(|| other.primitives.clone()),
            weights: (base.weights != other.weights).then(|| other.weights.clone()),
            name: (base.name != other.name).then(|| other.name.clone()),
            extensions: (base.extensions != other.extensions).then(|| other.extensions.clone()),
            extras: (base.extras != other.extras).then(|| other.extras.clone()),
        }
    }
    fn apply(&self, base: &GltfMesh) -> GltfMesh {
        let mut next = base.clone();
        if let Some(v) = &self.primitives { next.primitives = v.clone(); }
        if let Some(v) = &self.weights { next.weights = v.clone(); }
        if let Some(v) = &self.name { next.name = v.clone(); }
        if let Some(v) = &self.extensions { next.extensions = v.clone(); }
        if let Some(v) = &self.extras { next.extras = v.clone(); }
        next
    }
    fn inverse(&self, base: &GltfMesh) -> Self {
        Self {
            primitives: self.primitives.as_ref().map(|_| base.primitives.clone()),
            weights: self.weights.as_ref().map(|_| base.weights.clone()),
            name: self.name.as_ref().map(|_| base.name.clone()),
            extensions: self.extensions.as_ref().map(|_| base.extensions.clone()),
            extras: self.extras.as_ref().map(|_| base.extras.clone()),
        }
    }
    fn absorb_into(&mut self, other: Self) {
        if other.primitives.is_some() { self.primitives = other.primitives; }
        if other.weights.is_some() { self.weights = other.weights; }
        if other.name.is_some() { self.name = other.name; }
        if other.extensions.is_some() { self.extensions = other.extensions; }
        if other.extras.is_some() { self.extras = other.extras; }
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
    fn between(base: &GltfAccessor, other: &GltfAccessor) -> Self {
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
    fn apply(&self, base: &GltfAccessor) -> GltfAccessor {
        let mut next = base.clone();
        if let Some(v) = self.buffer_view { next.buffer_view = v; }
        if let Some(v) = self.byte_offset { next.byte_offset = v; }
        if let Some(v) = self.component_type { next.component_type = v; }
        if let Some(v) = self.normalized { next.normalized = v; }
        if let Some(v) = self.count { next.count = v; }
        if let Some(v) = self.kind { next.kind = v; }
        if let Some(v) = &self.max { next.max = v.clone(); }
        if let Some(v) = &self.min { next.min = v.clone(); }
        if let Some(v) = &self.sparse { next.sparse = v.clone(); }
        if let Some(v) = &self.name { next.name = v.clone(); }
        if let Some(v) = &self.extensions { next.extensions = v.clone(); }
        if let Some(v) = &self.extras { next.extras = v.clone(); }
        next
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
        if other.buffer_view.is_some() { self.buffer_view = other.buffer_view; }
        if other.byte_offset.is_some() { self.byte_offset = other.byte_offset; }
        if other.component_type.is_some() { self.component_type = other.component_type; }
        if other.normalized.is_some() { self.normalized = other.normalized; }
        if other.count.is_some() { self.count = other.count; }
        if other.kind.is_some() { self.kind = other.kind; }
        if other.max.is_some() { self.max = other.max; }
        if other.min.is_some() { self.min = other.min; }
        if other.sparse.is_some() { self.sparse = other.sparse; }
        if other.name.is_some() { self.name = other.name; }
        if other.extensions.is_some() { self.extensions = other.extensions; }
        if other.extras.is_some() { self.extras = other.extras; }
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
    pub pbr_metallic_roughness: Option<Option<crate::artifacts::gltf::schema::snapshot::GltfPbrMetallicRoughness>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub normal_texture: Option<Option<crate::artifacts::gltf::schema::snapshot::GltfNormalTextureInfo>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub occlusion_texture: Option<Option<crate::artifacts::gltf::schema::snapshot::GltfOcclusionTextureInfo>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emissive_texture: Option<Option<crate::artifacts::gltf::schema::snapshot::GltfTextureInfo>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emissive_factor: Option<[f64; 3]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alpha_mode: Option<crate::artifacts::gltf::schema::snapshot::GltfAlphaMode>,
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
    fn between(base: &GltfMaterial, other: &GltfMaterial) -> Self {
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
    fn apply(&self, base: &GltfMaterial) -> GltfMaterial {
        let mut next = base.clone();
        if let Some(v) = &self.name { next.name = v.clone(); }
        if let Some(v) = &self.pbr_metallic_roughness { next.pbr_metallic_roughness = v.clone(); }
        if let Some(v) = &self.normal_texture { next.normal_texture = v.clone(); }
        if let Some(v) = &self.occlusion_texture { next.occlusion_texture = v.clone(); }
        if let Some(v) = &self.emissive_texture { next.emissive_texture = v.clone(); }
        if let Some(v) = self.emissive_factor { next.emissive_factor = v; }
        if let Some(v) = self.alpha_mode { next.alpha_mode = v; }
        if let Some(v) = self.alpha_cutoff { next.alpha_cutoff = v; }
        if let Some(v) = self.double_sided { next.double_sided = v; }
        if let Some(v) = &self.extensions { next.extensions = v.clone(); }
        if let Some(v) = &self.extras { next.extras = v.clone(); }
        next
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
        if other.name.is_some() { self.name = other.name; }
        if other.pbr_metallic_roughness.is_some() { self.pbr_metallic_roughness = other.pbr_metallic_roughness; }
        if other.normal_texture.is_some() { self.normal_texture = other.normal_texture; }
        if other.occlusion_texture.is_some() { self.occlusion_texture = other.occlusion_texture; }
        if other.emissive_texture.is_some() { self.emissive_texture = other.emissive_texture; }
        if other.emissive_factor.is_some() { self.emissive_factor = other.emissive_factor; }
        if other.alpha_mode.is_some() { self.alpha_mode = other.alpha_mode; }
        if other.alpha_cutoff.is_some() { self.alpha_cutoff = other.alpha_cutoff; }
        if other.double_sided.is_some() { self.double_sided = other.double_sided; }
        if other.extensions.is_some() { self.extensions = other.extensions; }
        if other.extras.is_some() { self.extras = other.extras; }
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
    fn between(base: &GltfBuffer, other: &GltfBuffer) -> Self {
        Self {
            byte_length: (base.byte_length != other.byte_length).then_some(other.byte_length),
            uri: (base.uri != other.uri).then(|| other.uri.clone()),
            name: (base.name != other.name).then(|| other.name.clone()),
            extensions: (base.extensions != other.extensions).then(|| other.extensions.clone()),
            extras: (base.extras != other.extras).then(|| other.extras.clone()),
        }
    }
    fn apply(&self, base: &GltfBuffer) -> GltfBuffer {
        let mut next = base.clone();
        if let Some(v) = self.byte_length { next.byte_length = v; }
        if let Some(v) = &self.uri { next.uri = v.clone(); }
        if let Some(v) = &self.name { next.name = v.clone(); }
        if let Some(v) = &self.extensions { next.extensions = v.clone(); }
        if let Some(v) = &self.extras { next.extras = v.clone(); }
        next
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
        if other.byte_length.is_some() { self.byte_length = other.byte_length; }
        if other.uri.is_some() { self.uri = other.uri; }
        if other.name.is_some() { self.name = other.name; }
        if other.extensions.is_some() { self.extensions = other.extensions; }
        if other.extras.is_some() { self.extras = other.extras; }
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
pub type GltfImagesDiff = GltfWeakCollectionDiff<crate::artifacts::gltf::schema::snapshot::GltfImage>;
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
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub asset: Option<GltfAssetDiff>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scene: Option<Option<usize>>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scenes: Option<GltfScenesDiff>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nodes: Option<GltfNodesDiff>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meshes: Option<GltfMeshesDiff>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accessors: Option<GltfAccessorsDiff>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buffer_views: Option<GltfBufferViewsDiff>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buffers: Option<GltfBuffersDiff>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buffer_bytes: Option<GltfBufferBytesDiff>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub materials: Option<GltfMaterialsDiff>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub textures: Option<GltfTexturesDiff>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub images: Option<GltfImagesDiff>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub samplers: Option<GltfSamplersDiff>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skins: Option<GltfSkinsDiff>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub animations: Option<GltfAnimationsDiff>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cameras: Option<GltfCamerasDiff>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions_used: Option<Vec<String>>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions_required: Option<Vec<String>>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Option<GltfJson>>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extras: Option<Option<GltfJson>>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_form: Option<GltfSourceForm>,
}

impl GltfDiff {
    pub fn is_empty_diff(&self) -> bool {
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

impl MutationDiff<GltfSnapshot> for GltfDiff {
    fn apply(&self, base: &GltfSnapshot) -> GltfSnapshot {
        let mut next = base.clone();
        let doc = &mut next.document;
        if let Some(d) = &self.asset { doc.asset = d.apply(&doc.asset); }
        if let Some(v) = self.scene { doc.scene = v; }
        if let Some(d) = &self.scenes { doc.scenes = d.apply(&doc.scenes); }
        if let Some(d) = &self.nodes { doc.nodes = d.apply(&doc.nodes); }
        if let Some(d) = &self.meshes { doc.meshes = d.apply(&doc.meshes); }
        if let Some(d) = &self.accessors { doc.accessors = d.apply(&doc.accessors); }
        if let Some(d) = &self.buffer_views { doc.buffer_views = d.apply(&doc.buffer_views); }
        if let Some(d) = &self.buffers { doc.buffers = d.apply(&doc.buffers); }
        if let Some(d) = &self.buffer_bytes { next.buffers = d.apply(&next.buffers); }
        if let Some(d) = &self.materials { doc.materials = d.apply(&doc.materials); }
        if let Some(d) = &self.textures { doc.textures = d.apply(&doc.textures); }
        if let Some(d) = &self.images { doc.images = d.apply(&doc.images); }
        if let Some(d) = &self.samplers { doc.samplers = d.apply(&doc.samplers); }
        if let Some(d) = &self.skins { doc.skins = d.apply(&doc.skins); }
        if let Some(d) = &self.animations { doc.animations = d.apply(&doc.animations); }
        if let Some(d) = &self.cameras { doc.cameras = d.apply(&doc.cameras); }
        if let Some(v) = &self.extensions_used { doc.extensions_used = v.clone(); }
        if let Some(v) = &self.extensions_required { doc.extensions_required = v.clone(); }
        if let Some(v) = &self.extensions { doc.extensions = v.clone(); }
        if let Some(v) = &self.extras { doc.extras = v.clone(); }
        if let Some(v) = self.source_form { next.source_form = v; }
        next
    }

    fn absorb(&mut self, other: Self) {
        match (&mut self.asset, other.asset) {
            (Some(mine), Some(theirs)) => mine.absorb(theirs),
            (slot @ None, Some(theirs)) => *slot = Some(theirs),
            _ => {}
        }
        if other.scene.is_some() { self.scene = other.scene; }
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
        if other.extensions_used.is_some() { self.extensions_used = other.extensions_used; }
        if other.extensions_required.is_some() { self.extensions_required = other.extensions_required; }
        if other.extensions.is_some() { self.extensions = other.extensions; }
        if other.extras.is_some() { self.extras = other.extras; }
        if other.source_form.is_some() { self.source_form = other.source_form; }
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
            extensions_used: self.extensions_used.as_ref().map(|_| doc.extensions_used.clone()),
            extensions_required: self.extensions_required.as_ref().map(|_| doc.extensions_required.clone()),
            extensions: self.extensions.as_ref().map(|_| doc.extensions.clone()),
            extras: self.extras.as_ref().map(|_| doc.extras.clone()),
            source_form: self.source_form.map(|_| base.source_form),
        }
    }

    fn between(base: &GltfSnapshot, other: &GltfSnapshot) -> Self {
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

    fn is_empty(&self) -> bool {
        self.is_empty_diff()
    }
}

/// 🧩 Builds a set-snapshot diff — sparse field-by-field, never a full-replace slot.
pub fn diff_set_snapshot(base: &GltfSnapshot, snapshot: &GltfSnapshot) -> GltfDiff {
    <GltfDiff as DiffAlgebra<GltfSnapshot>>::between(base, snapshot)
}
//#endregion 🔖️Diff

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::gltf::STDIO_GLTF_DOCUMENT_SCHEMA;

    //#region 🔖️Fixtures
    fn scene(seed: usize) -> GltfScene {
        GltfScene { nodes: vec![seed, seed + 1], name: Some(format!("scene{seed}")), extensions: None, extras: None }
    }
    fn node(seed: usize) -> GltfNode {
        GltfNode { children: vec![seed], mesh: Some(seed), name: Some(format!("node{seed}")), ..GltfNode::default() }
    }
    fn mesh(seed: usize) -> GltfMesh {
        GltfMesh {
            primitives: vec![GltfPrimitive { attributes: vec![("POSITION".into(), seed)], material: Some(seed), mode: Some(4), ..GltfPrimitive::default() }],
            name: Some(format!("mesh{seed}")),
            ..GltfMesh::default()
        }
    }
    fn accessor(seed: usize) -> GltfAccessor {
        GltfAccessor {
            buffer_view: Some(seed), byte_offset: 0, component_type: GltfComponentType::Float, normalized: false,
            count: seed + 1, kind: GltfAccessorType::Vec3, max: Some(vec![1.0]), min: Some(vec![0.0]), sparse: None,
            name: Some(format!("acc{seed}")), extensions: None, extras: None,
        }
    }
    fn material(seed: usize) -> GltfMaterial {
        GltfMaterial { name: Some(format!("mat{seed}")), double_sided: seed % 2 == 0, ..GltfMaterial::default() }
    }
    fn buffer_meta(seed: usize) -> GltfBuffer {
        GltfBuffer { byte_length: seed * 4, uri: None, name: Some(format!("buf{seed}")), extensions: None, extras: None }
    }
    fn animation(seed: usize) -> GltfAnimation {
        GltfAnimation { name: Some(format!("anim{seed}")), ..GltfAnimation::default() }
    }

    fn base_snapshot() -> GltfSnapshot {
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
    #[test]
    fn absorb_law_insert_then_remove_before_shifts_index() {
        let n = node(9);
        let mut d1 = GltfNodesDiff { added: vec![GltfAdded { index: 2, item: n.clone() }], ..Default::default() };
        let d2 = GltfNodesDiff { removed: vec![0], ..Default::default() };
        d1.absorb(d2);
        assert_eq!(d1.removed, vec![0]);
        assert_eq!(d1.added, vec![GltfAdded { index: 1, item: n }]);
        assert!(d1.modified.is_empty());
    }

    /// 🧪️ Canonical absorb case 2: `Insert(2,f)` then `Insert(2,g)` → BOTH survive.
    #[test]
    fn absorb_law_insert_insert_same_index_both_survive() {
        let f = node(1);
        let g = node(2);
        let mut d1 = GltfNodesDiff { added: vec![GltfAdded { index: 2, item: f.clone() }], ..Default::default() };
        let d2 = GltfNodesDiff { added: vec![GltfAdded { index: 2, item: g.clone() }], ..Default::default() };
        d1.absorb(d2);
        assert_eq!(d1.added, vec![GltfAdded { index: 2, item: g }, GltfAdded { index: 3, item: f }]);
    }

    /// 🧪️ Canonical absorb case 3: `Insert(1,f)` then `SetField(1,name)` patches INTO the added
    /// payload -- merged has only `added`, no separate `modified` entry.
    #[test]
    fn absorb_law_insert_then_set_field_patches_into_added() {
        let f = node(1);
        let mut d1 = GltfNodesDiff { added: vec![GltfAdded { index: 1, item: f.clone() }], ..Default::default() };
        let d2 = GltfNodesDiff {
            modified: vec![GltfModified { index: 1, diff: GltfNodeDiff { name: Some(Some("renamed".into())), ..Default::default() } }],
            ..Default::default()
        };
        d1.absorb(d2);
        assert!(d1.modified.is_empty());
        assert_eq!(d1.added.len(), 1);
        assert_eq!(d1.added[0].item.name, Some("renamed".to_string()));
        assert_eq!(d1.added[0].index, 1);
    }

    /// 🧪️ Canonical absorb case 4 (id-keyed-collection-analog / modify-of-removed): `Remove(0)`
    /// then `Modify(0)` (post-remove index 0 refers to a DIFFERENT surviving base item) must NOT
    /// corrupt the removed item and must attach the d2 patch to the correct transported base index.
    #[test]
    fn absorb_law_remove_then_modify_transports_to_correct_surviving_item() {
        let mut d1 = GltfNodesDiff { removed: vec![0], ..Default::default() };
        // after d1, base[1] is now at position 0 -- d2 modifies position 0 (== base[1]).
        let d2 = GltfNodesDiff { modified: vec![GltfModified { index: 0, diff: GltfNodeDiff { name: Some(Some("x".into())), ..Default::default() } }], ..Default::default() };
        d1.absorb(d2);
        assert_eq!(d1.removed, vec![0]);
        assert_eq!(d1.modified.len(), 1);
        assert_eq!(d1.modified[0].index, 1, "d2's patch at post-remove position 0 must transport to BASE index 1");
    }

    #[test]
    fn absorb_law_holds_over_curated_ops() {
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
        assert_eq!(protocol::MutationDiff::apply(&d1, &base), after);
    }
    //#endregion 🔖️AbsorbCanonicalCases

    //#region 🔖️BetweenRoundtripLaw
    #[test]
    fn between_roundtrip_law_holds_on_synthetic_fixture() {
        let a = base_snapshot();
        let mut b = a.clone();
        b.document.nodes.push(node(5));
        b.document.asset.generator = Some("other-tool".into());
        b.source_form = GltfSourceForm::Glb;
        let ab = <GltfDiff as DiffAlgebra<GltfSnapshot>>::between(&a, &b);
        assert_eq!(protocol::MutationDiff::apply(&ab, &a), b);
        let ba = <GltfDiff as DiffAlgebra<GltfSnapshot>>::between(&b, &a);
        assert_eq!(protocol::MutationDiff::apply(&ba, &b), a);
        assert!(<GltfDiff as DiffAlgebra<GltfSnapshot>>::between(&a, &a).is_empty());
    }
    //#endregion 🔖️BetweenRoundtripLaw

    //#region 🔖️InverseLaw
    #[test]
    fn inverse_law_diff_level_round_trips() {
        let base = base_snapshot();
        let next = {
            let mut s = base.clone();
            s.document.nodes[0].mesh = None;
            s.document.nodes.remove(1);
            s.document.nodes.push(node(8));
            s.document.extensions_used.clear();
            s.document.materials[0].alpha_mode = crate::artifacts::gltf::schema::snapshot::GltfAlphaMode::Blend;
            s
        };
        let d = <GltfDiff as DiffAlgebra<GltfSnapshot>>::between(&base, &next);
        let mutated = protocol::MutationDiff::apply(&d, &base);
        let inv = <GltfDiff as DiffAlgebra<GltfSnapshot>>::inverse(&d, &base);
        assert_eq!(protocol::MutationDiff::apply(&inv, &mutated), base);
    }
    //#endregion 🔖️InverseLaw

    //#region 🔖️FieldSweep
    /// 🧪️ Field sweep — the acceptance criterion: `sweep_a`/`sweep_b` differ in EVERY mutable
    /// field, incl. every tri-state exercising `Some(None)`, with asymmetric collection lengths
    /// split across both `between()` directions (F1's structural trap).
    #[test]
    fn field_sweep_covers_every_mutable_field() {
        let sweep_a = GltfSnapshot {
            schema: STDIO_GLTF_DOCUMENT_SCHEMA.into(),
            document: GltfDocument {
                asset: GltfAsset { version: "2.0".into(), generator: Some("a-tool".into()), copyright: Some("(c) a".into()), min_version: Some("2.0".into()), extensions: Some(GltfJson::Bool(true)), extras: Some(GltfJson::String("a".into())) },
                scene: Some(0),
                scenes: vec![scene(0), scene(1)],
                nodes: vec![node(0), node(1)],
                meshes: vec![mesh(0), mesh(1)],
                accessors: vec![accessor(0), accessor(1)],
                buffer_views: vec![crate::artifacts::gltf::schema::snapshot::GltfBufferView { buffer: 0, byte_offset: 0, byte_length: 4, byte_stride: None, target: None, name: None, extensions: None, extras: None }],
                buffers: vec![buffer_meta(0), buffer_meta(1)],
                materials: vec![material(0), material(1)],
                textures: vec![crate::artifacts::gltf::schema::snapshot::GltfTexture { sampler: Some(0), source: Some(0), name: None, extensions: None, extras: None }],
                images: vec![crate::artifacts::gltf::schema::snapshot::GltfImage { uri: Some("a.png".into()), ..Default::default() }],
                samplers: vec![crate::artifacts::gltf::schema::snapshot::GltfSampler::default()],
                skins: vec![crate::artifacts::gltf::schema::snapshot::GltfSkin { joints: vec![0, 1], ..Default::default() }],
                animations: vec![animation(0), animation(1)],
                cameras: vec![],
                extensions_used: vec!["KHR_a".into()],
                extensions_required: vec!["KHR_a".into()],
                extensions: Some(GltfJson::Object(vec![("KHR_a".into(), GltfJson::Object(vec![]))])),
                extras: Some(GltfJson::String("sweep-a-extras".into())),
            },
            buffers: vec![vec![1, 2], vec![3, 4]],
            source_form: GltfSourceForm::Json,
        };
        let sweep_b = GltfSnapshot {
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
                cameras: vec![crate::artifacts::gltf::schema::snapshot::GltfCamera {
                    projection: crate::artifacts::gltf::schema::snapshot::GltfCameraProjection::Perspective(crate::artifacts::gltf::schema::snapshot::GltfPerspective { aspect_ratio: Some(1.5), yfov: 0.8, zfar: Some(100.0), znear: 0.1, extensions: None, extras: None }),
                    name: Some("cam".into()), extensions: None, extras: None,
                }],
                extensions_used: vec![],
                extensions_required: vec![],
                extensions: None,
                extras: None,
            },
            buffers: vec![vec![9]],
            source_form: GltfSourceForm::Glb,
        };

        let ab = <GltfDiff as DiffAlgebra<GltfSnapshot>>::between(&sweep_a, &sweep_b);
        assert_eq!(protocol::MutationDiff::apply(&ab, &sweep_a), sweep_b);
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
        assert_eq!(protocol::MutationDiff::apply(&ba, &sweep_b), sweep_a);
        let nodes_ba = ba.nodes.as_ref().unwrap();
        assert!(!nodes_ba.removed.is_empty(), "reverse direction must exercise a removed node (a is shorter, b is longer)");
        let cameras_ba = ba.cameras.as_ref().unwrap();
        assert!(!cameras_ba.removed.is_empty(), "reverse direction must exercise a removed camera");

        assert!(<GltfDiff as DiffAlgebra<GltfSnapshot>>::between(&sweep_a, &sweep_a).is_empty());
    }
    //#endregion 🔖️FieldSweep
}
//#endregion 🧪️Tests
