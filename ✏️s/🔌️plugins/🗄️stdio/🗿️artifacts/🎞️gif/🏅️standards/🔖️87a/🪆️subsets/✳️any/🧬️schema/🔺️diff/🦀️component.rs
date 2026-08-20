//! 🔺️ GifDiff (87a) — sparse per-field diff, handcrafted per the ticket's recipe. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: replaces the prior
//! `{snapshot: Option<GifSnapshot>}` full-replace stub outright — no `snapshot` slot survives.
//! `images` is a strong, index-keyed collection (`GifImagesDiff{removed,modified,added}`); `gct`
//! and every scalar screen field get their own sparse slot; `GifColorTable`/`GifImage` (the weak
//! leaf pieces) are whole-value replaced, never sub-diffed further, per the recipe's strong/weak
//! split.

use crate::artifacts::gif::standards::v87a::subsets::any::schema::snapshot::{GifColorTable, GifImage, GifRgb, GifSnapshot};
use protocol::os_spr::command::DiffAlgebra;
use protocol::DiffCodec;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️IndexTransport
/// 📐️ Shared rank/unrank arithmetic for index-keyed collection diffs (`between`/`absorb`/
/// `inverse` all need it) — see `🧬️schema-design.md` §Absorb and the top-level plan's "Absorb"
/// section for the derivation. `excluded_sorted` must be sorted ascending.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn count_le(sorted: &[usize], x: usize) -> usize {
    sorted.partition_point(|&v| v <= x)
}
/// 🔁️ Rank (0-indexed) of `pos` among non-negative integers NOT in `excluded_sorted` — `pos`
/// itself must not be in `excluded_sorted`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn rank_excluding(pos: usize, excluded_sorted: &[usize]) -> usize {
    pos - count_le(excluded_sorted, pos)
}
/// 🔁️ Inverse of [`rank_excluding`]: the `rank`-th (0-indexed) non-negative integer not in
/// `excluded_sorted`. Converges because `excluded_sorted` is finite.
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
/// 🧭️ One-shot base→after index transport for a SINGLE diff's own `removed`/`added` index sets
/// (used by `inverse`, where there is only one diff to transport through).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn transport_forward(index: usize, removed_sorted: &[usize], added_index_sorted: &[usize]) -> usize {
    unrank_excluding(rank_excluding(index, removed_sorted), added_index_sorted)
}
//#endregion 🔖️IndexTransport

//#region 🔖️GenericCollectionAlgebra
/// 🧮️ Sequential-coalesce absorb for an index-keyed collection triple, generic over the item type
/// `T` and its per-item diff type `Diff` (which may equal `T` itself for weak/whole-value-replaced
/// items). `absorb_diff` recursively absorbs two per-item diffs; `apply_diff_to_item` patches an
/// item's current value with an incoming diff (used when a d2 modify targets a d1-added item —
/// "patch into added"). Canonical correctness verified against the plan's 3 mandated cases in this
/// module's tests. See `🧬️schema-design.md` §Absorb.
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

/// ↩️ Diff-level inverse for an index-keyed collection triple, given the ORIGINAL base items (to
/// recover values for re-inserting removed entries and to compute per-item inverses for modified
/// entries). `diff_inverse` inverts one item's per-field diff against that item's base value.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
//#endregion 🔖️GenericCollectionAlgebra

//#region 🔖️ImageDiff
/// 🔺️ Sparse per-field diff for one [`GifImage`].
/// 🧪️ F6 FINDING: `#[derive(dsl::DslRecord)]`/`#[derive(dsl::DslDiff)]` CANNOT be used on this
/// struct — it has a tri-state `Option<Option<T>>` field (`lct`), which the derive's
/// `classify_field` cannot bind: it peels exactly ONE `Option<..>` layer via `inner_of(ty,
/// "Option")`, leaving the REMAINING type as `Option<GifColorTable>` itself, which then needs
/// `Option<GifColorTable>: DslField` — a blanket impl that does not exist anywhere in the `dsl`
/// crate (confirmed empirically: `cargo check` gives `the trait bound
/// std::option::Option<v87a::...::GifColorTable>: DslField is not satisfied`, matching gif89a's
/// `GifFrameDiff` finding exactly — see `f6-recon-report.md` §3b). `DiffCodec` for `GifDiff` is
/// hand-rolled below instead (this struct itself needs no `dsl` derive at all; it's a plain leaf
/// type consumed by the hand-rolled `print_diff`/`parse_diff`/`encode_diff`/`decode_diff`).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GifImageDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub left: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interlace: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lct: Option<Option<GifColorTable>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indices: Option<Vec<u8>>,
}

impl GifImageDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.left.is_none() && self.top.is_none() && self.width.is_none() && self.height.is_none() && self.interlace.is_none() && self.lct.is_none() && self.indices.is_none()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn between(base: &GifImage, other: &GifImage) -> Self {
        Self {
            left: (base.left != other.left).then_some(other.left),
            top: (base.top != other.top).then_some(other.top),
            width: (base.width != other.width).then_some(other.width),
            height: (base.height != other.height).then_some(other.height),
            interlace: (base.interlace != other.interlace).then_some(other.interlace),
            lct: (base.lct != other.lct).then_some(other.lct.clone()),
            indices: (base.indices != other.indices).then_some(other.indices.clone()),
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn apply(&self, base: &GifImage) -> GifImage {
        let mut next = base.clone();
        if let Some(v) = self.left {
            next.left = v;
        }
        if let Some(v) = self.top {
            next.top = v;
        }
        if let Some(v) = self.width {
            next.width = v;
        }
        if let Some(v) = self.height {
            next.height = v;
        }
        if let Some(v) = self.interlace {
            next.interlace = v;
        }
        if let Some(v) = &self.lct {
            next.lct = v.clone();
        }
        if let Some(v) = &self.indices {
            next.indices = v.clone();
        }
        next
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn inverse(&self, base: &GifImage) -> Self {
        Self {
            left: self.left.map(|_| base.left),
            top: self.top.map(|_| base.top),
            width: self.width.map(|_| base.width),
            height: self.height.map(|_| base.height),
            interlace: self.interlace.map(|_| base.interlace),
            lct: self.lct.as_ref().map(|_| base.lct.clone()),
            indices: self.indices.as_ref().map(|_| base.indices.clone()),
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn absorb(&mut self, other: Self) {
        if other.left.is_some() {
            self.left = other.left;
        }
        if other.top.is_some() {
            self.top = other.top;
        }
        if other.width.is_some() {
            self.width = other.width;
        }
        if other.height.is_some() {
            self.height = other.height;
        }
        if other.interlace.is_some() {
            self.interlace = other.interlace;
        }
        if other.lct.is_some() {
            self.lct = other.lct;
        }
        if other.indices.is_some() {
            self.indices = other.indices;
        }
    }
}
//#endregion 🔖️ImageDiff

//#region 🔖️ImagesDiff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GifImageModified {
    pub index: usize,
    pub diff: GifImageDiff,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GifImageAdded {
    pub index: usize,
    pub image: GifImage,
}

/// 🔺️ Index-keyed collection triple for `GifSnapshot::images`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GifImagesDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<GifImageModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<GifImageAdded>,
}

impl GifImagesDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn between(base: &[GifImage], other: &[GifImage]) -> Self {
        let min = base.len().min(other.len());
        let mut modified = Vec::new();
        for i in 0..min {
            let d = GifImageDiff::between(&base[i], &other[i]);
            if !d.is_empty() {
                modified.push(GifImageModified { index: i, diff: d });
            }
        }
        let removed: Vec<usize> = (min..base.len()).collect();
        let added: Vec<GifImageAdded> = (min..other.len()).map(|i| GifImageAdded { index: i, image: other[i].clone() }).collect();
        Self { removed, modified, added }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn apply(&self, base: &[GifImage]) -> Vec<GifImage> {
        let mut next: Vec<Option<GifImage>> = base.iter().cloned().map(Some).collect();
        let mut removed_sorted = self.removed.clone();
        removed_sorted.sort_unstable();
        removed_sorted.reverse();
        for m in &self.modified {
            if let Some(Some(item)) = next.get(m.index).map(|o| o.as_ref().map(|i| m.diff.apply(i))) {
                if let Some(slot) = next.get_mut(m.index) {
                    *slot = Some(item);
                }
            }
        }
        for &r in &removed_sorted {
            if r < next.len() {
                next.remove(r);
            }
        }
        let mut out: Vec<GifImage> = next.into_iter().flatten().collect();
        let mut added_sorted = self.added.clone();
        added_sorted.sort_by_key(|a| a.index);
        for a in added_sorted {
            let at = a.index.min(out.len());
            out.insert(at, a.image);
        }
        out
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn absorb(&mut self, other: Self) {
        let (removed, modified, added) = absorb_indexed_collection(
            std::mem::take(&mut self.removed),
            std::mem::take(&mut self.modified).into_iter().map(|m| (m.index, m.diff)).collect(),
            std::mem::take(&mut self.added).into_iter().map(|a| (a.index, a.image)).collect(),
            other.removed,
            other.modified.into_iter().map(|m| (m.index, m.diff)).collect(),
            other.added.into_iter().map(|a| (a.index, a.image)).collect(),
            |d, o| d.absorb(o),
            |d, item| d.apply(item),
        );
        self.removed = removed;
        self.modified = modified.into_iter().map(|(index, diff)| GifImageModified { index, diff }).collect();
        self.added = added.into_iter().map(|(index, image)| GifImageAdded { index, image }).collect();
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn inverse(&self, base_images: &[GifImage]) -> Self {
        let (removed, modified, added) =
            inverse_indexed_collection(&self.removed, &self.modified.iter().map(|m| (m.index, m.diff.clone())).collect::<Vec<_>>(), &self.added.iter().map(|a| (a.index, a.image.clone())).collect::<Vec<_>>(), base_images, |d, item| d.inverse(item));
        Self { removed, modified: modified.into_iter().map(|(index, diff)| GifImageModified { index, diff }).collect(), added: added.into_iter().map(|(index, image)| GifImageAdded { index, image }).collect() }
    }
}
//#endregion 🔖️ImagesDiff

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.gif` (87a). No `snapshot: Option<GifSnapshot>` full-replace slot anywhere —
/// even `SetSnapshot`'s diff is the sparse field-by-field `between(base, next)`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.gif.diff")]
pub struct GifDiff {
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gct: Option<Option<GifColorTable>>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background_color_index: Option<u8>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pixel_aspect_ratio: Option<u8>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub images: Option<GifImagesDiff>,
}

impl GifDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty_diff(&self) -> bool {
        self.width.is_none() && self.height.is_none() && self.gct.is_none() && self.background_color_index.is_none() && self.pixel_aspect_ratio.is_none() && self.images.as_ref().map(GifImagesDiff::is_empty).unwrap_or(true)
    }
}

impl MutationDiff<GifSnapshot> for GifDiff {
    async fn apply(&self, base: &GifSnapshot) -> MutationApplyResult<GifSnapshot> {
        if let Some(images) = &self.images {
            validate_gif_images(base.images.len(), images)?;
        }
        let mut next = base.clone();
        if let Some(v) = self.width {
            next.width = v;
        }
        if let Some(v) = self.height {
            next.height = v;
        }
        if let Some(v) = &self.gct {
            next.gct = v.clone();
        }
        if let Some(v) = self.background_color_index {
            next.background_color_index = v;
        }
        if let Some(v) = self.pixel_aspect_ratio {
            next.pixel_aspect_ratio = v;
        }
        if let Some(images_diff) = &self.images {
            next.images = images_diff.apply(&next.images);
        }
        Ok(next)
    }

    async fn absorb(&mut self, other: Self) {
        if other.width.is_some() {
            self.width = other.width;
        }
        if other.height.is_some() {
            self.height = other.height;
        }
        if other.gct.is_some() {
            self.gct = other.gct;
        }
        if other.background_color_index.is_some() {
            self.background_color_index = other.background_color_index;
        }
        if other.pixel_aspect_ratio.is_some() {
            self.pixel_aspect_ratio = other.pixel_aspect_ratio;
        }
        match (&mut self.images, other.images) {
            (Some(mine), Some(theirs)) => mine.absorb(theirs),
            (slot @ None, Some(theirs)) => *slot = Some(theirs),
            _ => {}
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_gif_images(base_len: usize, diff: &GifImagesDiff) -> MutationApplyResult<()> {
    let mut removed = std::collections::HashSet::new();
    for &index in &diff.removed {
        if index >= base_len || !removed.insert(index) {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "GIF image removal is missing or duplicated").at(["images", "removed"]));
        }
    }
    let mut modified = std::collections::HashSet::new();
    for entry in &diff.modified {
        if entry.index >= base_len || !modified.insert(entry.index) || removed.contains(&entry.index) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "GIF image modification is missing, duplicated, or removed").at(["images", "modified"]));
        }
    }
    let final_len = base_len.saturating_sub(diff.removed.len()).saturating_add(diff.added.len());
    let mut added = std::collections::HashSet::new();
    for entry in &diff.added {
        if entry.index > final_len || !added.insert(entry.index) {
            return Err(MutationApplyError::new("mutation.apply.invalid-index", "GIF image addition index is invalid or duplicated").at(["images", "added"]));
        }
    }
    Ok(())
}

impl DiffAlgebra<GifSnapshot> for GifDiff {
    async fn inverse(&self, base: &GifSnapshot) -> Self {
        Self {
            width: self.width.map(|_| base.width),
            height: self.height.map(|_| base.height),
            gct: self.gct.as_ref().map(|_| base.gct.clone()),
            background_color_index: self.background_color_index.map(|_| base.background_color_index),
            pixel_aspect_ratio: self.pixel_aspect_ratio.map(|_| base.pixel_aspect_ratio),
            images: self.images.as_ref().map(|d| d.inverse(&base.images)),
        }
    }

    async fn between(base: &GifSnapshot, other: &GifSnapshot) -> Self {
        let images_diff = GifImagesDiff::between(&base.images, &other.images);
        Self {
            width: (base.width != other.width).then_some(other.width),
            height: (base.height != other.height).then_some(other.height),
            gct: (base.gct != other.gct).then_some(other.gct.clone()),
            background_color_index: (base.background_color_index != other.background_color_index).then_some(other.background_color_index),
            pixel_aspect_ratio: (base.pixel_aspect_ratio != other.pixel_aspect_ratio).then_some(other.pixel_aspect_ratio),
            images: (!images_diff.is_empty()).then_some(images_diff),
        }
    }

    async fn is_empty(&self) -> bool {
        self.is_empty_diff()
    }
}

/// 🧩 Builds a set-snapshot diff — sparse field-by-field, never a full-replace slot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_snapshot(base: &GifSnapshot, snapshot: &GifSnapshot) -> GifDiff {
    <GifDiff as DiffAlgebra<GifSnapshot>>::between(base, snapshot)
}

/// 🧪️ P2-FG2: representative `GifDiff` cases for `diff_grammar_conformance_law`/
/// `protocol_walk_law` (`../../../../⚙️engine/🦀️component.rs`'s `conformance_laws` module) —
/// the empty diff, plus a real `between()` result exercising every scalar field, the `gct`
/// tri-state (both `Some(Some(_))` and `Some(None)`), and the `images` collection triple's
/// `removed`/`modified`/`added` all at once (mirrors png's own `demo_diff_cases()`).
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<GifDiff> {
    let img = |seed: u8, w: u32, h: u32| GifImage { left: 0, top: 0, width: w, height: h, interlace: false, lct: Some(GifColorTable { sorted: false, colors: vec![GifRgb { r: seed, g: seed, b: seed }; 2] }), indices: vec![0u8; (w * h) as usize] };
    let a = GifSnapshot { width: 4, height: 4, gct: Some(GifColorTable { sorted: false, colors: vec![GifRgb { r: 1, g: 2, b: 3 }; 2] }), images: vec![img(1, 2, 2), img(2, 2, 2)], ..GifSnapshot::default() };
    let mut ib0 = img(1, 2, 2);
    ib0.interlace = true;
    ib0.lct = None;
    let b = GifSnapshot { width: 8, height: 8, gct: None, background_color_index: 3, pixel_aspect_ratio: 5, images: vec![ib0, img(6, 3, 3), img(7, 3, 3)], ..GifSnapshot::default() };
    vec![GifDiff::default(), diff_set_snapshot(&a, &b), diff_set_snapshot(&b, &a)]
}
//#endregion 🔖️Diff

//#region 🔖️HandcraftedDiffCodec
/// 🧪️ F6: **hand-rolled** `protocol::DiffCodec` for `GifDiff` — the derive path
/// (`#[derive(dsl::DslDiff)]`) is NOT usable here: `GifDiff` (and `GifImageDiff` nested inside its
/// `images` collection) both carry a tri-state `Option<Option<T>>` field (`gct`; `GifImageDiff`'s
/// `lct`), which the derive cannot bind (see the doc comment on `GifImageDiff` above, and
/// `f6-recon-report.md` for the confirmed compile error, reproduced identically here via real
/// `cargo check`). This is the SAME hand-rolled path gif89a's `GifDiff` uses, for the identical
/// tri-state reason — 87a is the row `f6-recon-report.md` §8 flagged as "same family/pattern as
/// 89a, simpler (no GCE)": one collection triple (`images`) instead of three, one tri-state field
/// (`gct`) at the top level plus one (`lct`) inside the collection's per-item diff, instead of
/// 89a's two top-level (`gct`/`loop_count`) plus three nested (`lct`/`transparent_index`/
/// `plain_text`).
///
/// **Grammar** (real, not `serde_json`) — identical conventions to gif89a's `GifDiff`, copied
/// directly per `f6-recon-report.md` §5/§9: one space-separated `name=value` token per changed
/// top-level field (a field absent from the line = unchanged); the collection prints as
/// `images{[removed];[modified];[added]}`. Bytes/strings are lowercase hex (this artifact's own
/// `ArtifactDsl` impl in the `📸️snapshot` module already uses hex for the same reason). `Option<T>`
/// values (both real optional snapshot fields AND diff tri-states) use a uniform `[0]`=None /
/// `[1,<T>]`=Some(T) tag. Structs are positional `[f1,f2,...]` tuples. `GifImageDiff`'s own sparse
/// fields print as single-letter `tag:value` pairs (`L`/`T`/`W`/`H`/`I`/`C`/`X`) inside its own
/// `[...]` — no `D`/`S`/`P`/`U`/`Q` tags (87a has no GCE-derived fields at all).
//#region 🔖️Primitives
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_u8(s: &str) -> Result<u8, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_u32(s: &str) -> Result<u32, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_usize(s: &str) -> Result<usize, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}

/// 🧭️ Bracket-depth-aware split (tracks `[`/`]` only): a top-level `sep` inside nested brackets is
/// never mistaken for a field separator — the whole hand-rolled grammar's parsing primitive.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn split_top_level(s: &str, sep: char) -> Vec<&str> {
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
fn strip_brackets(s: &str) -> Result<&str, String> {
    s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_option<T>(opt: &Option<T>, enc: impl Fn(&T) -> String) -> String {
    match opt {
        None => "[0]".to_string(),
        Some(v) => format!("[1,{}]", enc(v)),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_option<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Option<T>, String> {
    let inner = strip_brackets(s)?;
    match split_top_level(inner, ',').as_slice() {
        ["0"] => Ok(None),
        [tag, value] if *tag == "1" => Ok(Some(dec(value)?)),
        other => Err(format!("option decode: bad shape {other:?}")),
    }
}
//#endregion 🔖️Primitives

//#region 🔖️ValueCodecs
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_rgb(c: &GifRgb) -> String {
    format!("[{},{},{}]", c.r, c.g, c.b)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_rgb(s: &str) -> Result<GifRgb, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [r, g, b] = parts.as_slice() else { return Err(format!("rgb: expected 3 fields, got {}", parts.len())) };
    Ok(GifRgb { r: parse_u8(r)?, g: parse_u8(g)?, b: parse_u8(b)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_color_table(t: &GifColorTable) -> String {
    let colors = t.colors.iter().map(enc_rgb).collect::<Vec<_>>().join(",");
    format!("[{},[{}]]", if t.sorted { 1 } else { 0 }, colors)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_color_table(s: &str) -> Result<GifColorTable, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [sorted, colors] = parts.as_slice() else { return Err(format!("color table: expected 2 fields, got {}", parts.len())) };
    let colors = split_top_level(strip_brackets(colors)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_rgb).collect::<Result<Vec<_>, String>>()?;
    Ok(GifColorTable { sorted: *sorted == "1", colors })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_image(f: &GifImage) -> String {
    format!("[{},{},{},{},{},{},{}]", f.left, f.top, f.width, f.height, if f.interlace { 1 } else { 0 }, encode_option(&f.lct, enc_color_table), hex_encode(&f.indices),)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_image(s: &str) -> Result<GifImage, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [left, top, width, height, interlace, lct, indices] = parts.as_slice() else {
        return Err(format!("image: expected 7 fields, got {}", parts.len()));
    };
    Ok(GifImage { left: parse_u32(left)?, top: parse_u32(top)?, width: parse_u32(width)?, height: parse_u32(height)?, interlace: *interlace == "1", lct: decode_option(lct, dec_color_table)?, indices: hex_decode(indices)? })
}
//#endregion 🔖️ValueCodecs

//#region 🔖️DiffValueCodecs
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_image_diff(d: &GifImageDiff) -> String {
    let mut parts = Vec::new();
    if let Some(v) = d.left {
        parts.push(format!("L:{v}"));
    }
    if let Some(v) = d.top {
        parts.push(format!("T:{v}"));
    }
    if let Some(v) = d.width {
        parts.push(format!("W:{v}"));
    }
    if let Some(v) = d.height {
        parts.push(format!("H:{v}"));
    }
    if let Some(v) = d.interlace {
        parts.push(format!("I:{}", if v { 1 } else { 0 }));
    }
    if let Some(v) = &d.lct {
        parts.push(format!("C:{}", encode_option(v, enc_color_table)));
    }
    if let Some(v) = &d.indices {
        parts.push(format!("X:{}", hex_encode(v)));
    }
    format!("[{}]", parts.join(","))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_image_diff(s: &str) -> Result<GifImageDiff, String> {
    let inner = strip_brackets(s)?;
    let mut d = GifImageDiff::default();
    for entry in split_top_level(inner, ',') {
        if entry.is_empty() {
            continue;
        }
        let (tag, val) = entry.split_once(':').ok_or_else(|| format!("image diff: bad entry {entry:?}"))?;
        match tag {
            "L" => d.left = Some(parse_u32(val)?),
            "T" => d.top = Some(parse_u32(val)?),
            "W" => d.width = Some(parse_u32(val)?),
            "H" => d.height = Some(parse_u32(val)?),
            "I" => d.interlace = Some(val == "1"),
            "C" => d.lct = Some(decode_option(val, dec_color_table)?),
            "X" => d.indices = Some(hex_decode(val)?),
            other => return Err(format!("image diff: unknown tag {other:?}")),
        }
    }
    Ok(d)
}

/// 🧭️ Generic-shaped 3-section `[removed];[modified];[added]` collection-triple printer/parser —
/// identical shape to gif89a's `enc_collection_triple`/`dec_collection_triple`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_collection_triple(name: &str, removed: &[usize], modified: &[(usize, String)], added: &[(usize, String)]) -> String {
    let removed = removed.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let modified = modified.iter().map(|(i, v)| format!("{i}:{v}")).collect::<Vec<_>>().join(",");
    let added = added.iter().map(|(i, v)| format!("{i}:{v}")).collect::<Vec<_>>().join(",");
    format!("{name}{{[{removed}];[{modified}];[{added}]}}")
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_collection_triple(body: &str) -> Result<(Vec<usize>, Vec<(usize, String)>, Vec<(usize, String)>), String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("collection: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_usize).collect::<Result<Vec<_>, String>>()?;
    let parse_entries = |s: &str| -> Result<Vec<(usize, String)>, String> {
        split_top_level(strip_brackets(s)?, ',')
            .into_iter()
            .filter(|s| !s.is_empty())
            .map(|entry| {
                let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("collection entry: bad entry {entry:?}"))?;
                Ok((parse_usize(idx)?, rest.to_string()))
            })
            .collect()
    };
    Ok((removed, parse_entries(modified_s)?, parse_entries(added_s)?))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_images_diff(d: &GifImagesDiff) -> String {
    enc_collection_triple("images", &d.removed, &d.modified.iter().map(|m| (m.index, enc_image_diff(&m.diff))).collect::<Vec<_>>(), &d.added.iter().map(|a| (a.index, enc_image(&a.image))).collect::<Vec<_>>())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_images_diff(body: &str) -> Result<GifImagesDiff, String> {
    let (removed, modified, added) = dec_collection_triple(body)?;
    Ok(GifImagesDiff {
        removed,
        modified: modified.into_iter().map(|(index, enc)| Ok(GifImageModified { index, diff: dec_image_diff(&enc)? })).collect::<Result<Vec<_>, String>>()?,
        added: added.into_iter().map(|(index, enc)| Ok(GifImageAdded { index, image: dec_image(&enc)? })).collect::<Result<Vec<_>, String>>()?,
    })
}
//#endregion 🔖️DiffValueCodecs

//#region 🔖️RealBinaryPrimitives
/// 🧪️ P2-FG2: real binary value codecs for `GifDiff`'s nested types — mirrors the text codecs
/// above field-for-field, using `dsl::ByteWriter`/`dsl::ByteReader` (the same real
/// LEB128-varint/length-prefixed framework primitives png's own upgraded `PngDiff` binary frame
/// uses, `📷️png/…/🔺️diff/🦀️component.rs`'s `RealBinaryPrimitives`/`RealBinaryDiffFrame`
/// regions — `dsl`/`store`/`protocol` all alias the same kernel crate root, reachable with no
/// `use` needed beyond the absolute path).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_bin_rgb(w: &mut dsl::ByteWriter, c: &GifRgb) {
    w.write_u8(c.r);
    w.write_u8(c.g);
    w.write_u8(c.b);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_bin_rgb(r: &mut dsl::ByteReader<'_>) -> Result<GifRgb, dsl::PackError> {
    Ok(GifRgb { r: r.read_u8()?, g: r.read_u8()?, b: r.read_u8()? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_bin_color_table(w: &mut dsl::ByteWriter, t: &GifColorTable) {
    w.write_u8(if t.sorted { 1 } else { 0 });
    write_bin_vec(w, &t.colors, write_bin_rgb);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_bin_color_table(r: &mut dsl::ByteReader<'_>) -> Result<GifColorTable, dsl::PackError> {
    let sorted = r.read_u8()? != 0;
    let colors = read_bin_vec(r, read_bin_rgb)?;
    Ok(GifColorTable { sorted, colors })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_bin_blob(w: &mut dsl::ByteWriter, bytes: &[u8]) {
    w.write_varint_u64(bytes.len() as u64);
    w.write_bytes(bytes);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_bin_blob(r: &mut dsl::ByteReader<'_>) -> Result<Vec<u8>, dsl::PackError> {
    let len = r.read_varint_u64()? as usize;
    Ok(r.read_bytes(len)?.to_vec())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_bin_image(w: &mut dsl::ByteWriter, f: &GifImage) {
    w.write_u32_le(f.left);
    w.write_u32_le(f.top);
    w.write_u32_le(f.width);
    w.write_u32_le(f.height);
    w.write_u8(if f.interlace { 1 } else { 0 });
    write_bin_option(w, &f.lct, write_bin_color_table);
    write_bin_blob(w, &f.indices);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_bin_image(r: &mut dsl::ByteReader<'_>) -> Result<GifImage, dsl::PackError> {
    Ok(GifImage { left: r.read_u32_le()?, top: r.read_u32_le()?, width: r.read_u32_le()?, height: r.read_u32_le()?, interlace: r.read_u8()? != 0, lct: read_bin_option(r, read_bin_color_table)?, indices: read_bin_blob(r)? })
}
/// 🧩 2-way presence flag (`0`=None, `1`=Some) — shared by every plain `Option<T>` field.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_bin_option<T>(w: &mut dsl::ByteWriter, v: &Option<T>, write_value: impl FnOnce(&mut dsl::ByteWriter, &T)) {
    match v {
        None => w.write_u8(0),
        Some(val) => {
            w.write_u8(1);
            write_value(w, val);
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_bin_option<T>(r: &mut dsl::ByteReader<'_>, read_value: impl FnOnce(&mut dsl::ByteReader<'_>) -> Result<T, dsl::PackError>) -> Result<Option<T>, dsl::PackError> {
    match r.read_u8()? {
        0 => Ok(None),
        1 => Ok(Some(read_value(r)?)),
        other => Err(dsl::PackError::Malformed { what: "gif87a binary option tag", offset: 0, detail: format!("unknown tag {other}") }),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_bin_vec<T>(w: &mut dsl::ByteWriter, items: &[T], write_item: impl Fn(&mut dsl::ByteWriter, &T)) {
    w.write_varint_u64(items.len() as u64);
    for item in items {
        write_item(w, item);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_bin_vec<T>(r: &mut dsl::ByteReader<'_>, mut read_item: impl FnMut(&mut dsl::ByteReader<'_>) -> Result<T, dsl::PackError>) -> Result<Vec<T>, dsl::PackError> {
    let n = r.read_varint_u64()? as usize;
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        out.push(read_item(r)?);
    }
    Ok(out)
}
/// 🧩 3-way flag (`0`=unchanged, `1`=cleared-to-`None`, `2`=set-to-`Some(value)`) for every
/// TRI-STATE `Option<Option<T>>` field — matches png's own doc comment for why this avoids
/// chaining two `if`-guarded conditional fields at the protocol-description level (`Cond::eval`
/// errors on a field that was itself only conditionally decoded); the Rust codec itself has no
/// such limitation, but keeps the same 3-way-flag SHAPE for parity with the protocol file.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_bin_tri_flag<T>(w: &mut dsl::ByteWriter, v: &Option<Option<T>>, write_value: impl FnOnce(&mut dsl::ByteWriter, &T)) {
    match v {
        None => w.write_u8(0),
        Some(None) => w.write_u8(1),
        Some(Some(val)) => {
            w.write_u8(2);
            write_value(w, val);
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_bin_tri_flag<T>(r: &mut dsl::ByteReader<'_>, read_value: impl FnOnce(&mut dsl::ByteReader<'_>) -> Result<T, dsl::PackError>) -> Result<Option<Option<T>>, dsl::PackError> {
    match r.read_u8()? {
        0 => Ok(None),
        1 => Ok(Some(None)),
        2 => Ok(Some(Some(read_value(r)?))),
        other => Err(dsl::PackError::Malformed { what: "gif87a diff tri-flag", offset: 0, detail: format!("unknown flag {other}") }),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_pack_err(e: dsl::PackError) -> protocol::ProtocolError {
    protocol::ProtocolError::Malformed { what: "gif87a diff binary", offset: 0, detail: e.to_string() }
}
//#endregion 🔖️RealBinaryPrimitives

//#region 🔖️RealBinaryDiffFrame
/// 🧪️ P2-FG2: real binary encoding for `GifImageDiff`/`GifImagesDiff` — the collection triple
/// produces one opaque `Vec<u8>` blob matching `../💾️binary/📡️component.protocol.semio`'s
/// `Array(u8, Field(images_len))` field (the blob's own internal removed/modified/added shape
/// isn't further protocol-walkable — see that file's own doc comment); the Rust codec here IS
/// genuinely, fully structured (real varint counts, real per-item recursive encoding), never
/// text-as-bytes.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_bin_image_diff(w: &mut dsl::ByteWriter, d: &GifImageDiff) {
    write_bin_option(w, &d.left, |w, v| w.write_u32_le(*v));
    write_bin_option(w, &d.top, |w, v| w.write_u32_le(*v));
    write_bin_option(w, &d.width, |w, v| w.write_u32_le(*v));
    write_bin_option(w, &d.height, |w, v| w.write_u32_le(*v));
    write_bin_option(w, &d.interlace, |w, v| w.write_u8(if *v { 1 } else { 0 }));
    write_bin_tri_flag(w, &d.lct, write_bin_color_table);
    write_bin_option(w, &d.indices, |w, v| write_bin_blob(w, v));
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_bin_image_diff(r: &mut dsl::ByteReader<'_>) -> Result<GifImageDiff, dsl::PackError> {
    Ok(GifImageDiff {
        left: read_bin_option(r, |r| r.read_u32_le())?,
        top: read_bin_option(r, |r| r.read_u32_le())?,
        width: read_bin_option(r, |r| r.read_u32_le())?,
        height: read_bin_option(r, |r| r.read_u32_le())?,
        interlace: read_bin_option(r, |r| Ok(r.read_u8()? != 0))?,
        lct: read_bin_tri_flag(r, read_bin_color_table)?,
        indices: read_bin_option(r, read_bin_blob)?,
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_images_diff_bin(d: &GifImagesDiff) -> Vec<u8> {
    let mut w = semio_framework_plugin::resolve_ready(dsl::ByteWriter::new());
    write_bin_vec(&mut w, &d.removed, |w, v: &usize| w.write_varint_u64(*v as u64));
    write_bin_vec(&mut w, &d.modified, |w, m: &GifImageModified| {
        w.write_varint_u64(m.index as u64);
        write_bin_image_diff(w, &m.diff);
    });
    write_bin_vec(&mut w, &d.added, |w, a: &GifImageAdded| {
        w.write_varint_u64(a.index as u64);
        write_bin_image(w, &a.image);
    });
    w.into_bytes()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_images_diff_bin(bytes: &[u8]) -> Result<GifImagesDiff, dsl::PackError> {
    let mut r = semio_framework_plugin::resolve_ready(dsl::ByteReader::new(bytes));
    let removed = read_bin_vec(&mut r, |r| Ok(r.read_varint_u64()? as usize))?;
    let modified = read_bin_vec(&mut r, |r| {
        let index = r.read_varint_u64()? as usize;
        let diff = read_bin_image_diff(r)?;
        Ok(GifImageModified { index, diff })
    })?;
    let added = read_bin_vec(&mut r, |r| {
        let index = r.read_varint_u64()? as usize;
        let image = read_bin_image(r)?;
        Ok(GifImageAdded { index, image })
    })?;
    Ok(GifImagesDiff { removed, modified, added })
}
//#endregion 🔖️RealBinaryDiffFrame

//#region 🔖️TopLevel
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_gif_diff(d: &GifDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = d.width {
        tokens.push(format!("width={v}"));
    }
    if let Some(v) = d.height {
        tokens.push(format!("height={v}"));
    }
    if let Some(v) = &d.gct {
        tokens.push(format!("gct={}", encode_option(v, enc_color_table)));
    }
    if let Some(v) = d.background_color_index {
        tokens.push(format!("bg={v}"));
    }
    if let Some(v) = d.pixel_aspect_ratio {
        tokens.push(format!("par={v}"));
    }
    if let Some(v) = &d.images {
        tokens.push(enc_images_diff(v));
    }
    tokens.join(" ")
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_gif_diff(line: &str) -> Result<GifDiff, String> {
    let mut d = GifDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("width=") {
            d.width = Some(parse_u32(rest)?);
        } else if let Some(rest) = token.strip_prefix("height=") {
            d.height = Some(parse_u32(rest)?);
        } else if let Some(rest) = token.strip_prefix("gct=") {
            d.gct = Some(decode_option(rest, dec_color_table)?);
        } else if let Some(rest) = token.strip_prefix("bg=") {
            d.background_color_index = Some(parse_u8(rest)?);
        } else if let Some(rest) = token.strip_prefix("par=") {
            d.pixel_aspect_ratio = Some(parse_u8(rest)?);
        } else if let Some(rest) = token.strip_prefix("images{") {
            d.images = Some(dec_images_diff(rest.strip_suffix('}').ok_or_else(|| "images: missing closing brace".to_string())?)?);
        } else {
            return Err(format!("gif diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl DiffCodec for GifDiff {
    async fn print_diff(&self) -> String {
        print_gif_diff(self)
    }
    async fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        parse_gif_diff(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    /// ⚡️ P2-FG2: real binary diff-frame — upgraded from the F6-era `print_diff().into_bytes()`
    /// text-as-binary shortcut (100% of stdio's `DiffCodec` impls were still on that shortcut
    /// per the P2-W0 census; the FG1 wave's own closer report flagged leaving this un-upgraded
    /// as a real defect to not repeat). Matches `../💾️binary/📡️component.protocol.semio`'s real
    /// flag-per-field layout exactly, field for field, in struct order (2-way flag for plain
    /// `Option<T>` fields, 3-way flag for the tri-state `gct` field).
    async fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let mut w = dsl::ByteWriter::new().await;
        write_bin_option(&mut w, &self.width, |w, v| w.write_u32_le(*v));
        write_bin_option(&mut w, &self.height, |w, v| w.write_u32_le(*v));
        write_bin_tri_flag(&mut w, &self.gct, |w, v| {
            let mut inner = semio_framework_plugin::resolve_ready(dsl::ByteWriter::new());
            write_bin_color_table(&mut inner, v);
            write_bin_blob(w, &semio_framework_plugin::resolve_ready(inner.into_bytes()));
        });
        write_bin_option(&mut w, &self.background_color_index, |w, v| w.write_u8(*v));
        write_bin_option(&mut w, &self.pixel_aspect_ratio, |w, v| w.write_u8(*v));
        write_bin_option(&mut w, &self.images, |w, v| write_bin_blob(w, &enc_images_diff_bin(v)));
        Ok(w.into_bytes().await)
    }
    async fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut r = semio_framework_plugin::resolve_ready(dsl::ByteReader::new(bytes));
        let width = read_bin_option(&mut r, |r| r.read_u32_le()).map_err(diff_pack_err)?;
        let height = read_bin_option(&mut r, |r| r.read_u32_le()).map_err(diff_pack_err)?;
        let gct = read_bin_tri_flag(&mut r, |r| {
            let blob = read_bin_blob(r)?;
            let mut inner = semio_framework_plugin::resolve_ready(dsl::ByteReader::new(&blob));
            read_bin_color_table(&mut inner)
        })
        .map_err(diff_pack_err)?;
        let background_color_index = read_bin_option(&mut r, |r| r.read_u8()).map_err(diff_pack_err)?;
        let pixel_aspect_ratio = read_bin_option(&mut r, |r| r.read_u8()).map_err(diff_pack_err)?;
        let images = read_bin_option(&mut r, |r| dec_images_diff_bin(&read_bin_blob(r)?)).map_err(diff_pack_err)?;
        Ok(GifDiff { width, height, gct, background_color_index, pixel_aspect_ratio, images })
    }
}
//#endregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec

//#region Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::gif::standards::v87a::subsets::any::schema::snapshot::GifRgb;
    use crate::artifacts::gif::STDIO_GIF_DOCUMENT_SCHEMA;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn img(seed: u8, w: u32, h: u32) -> GifImage {
        GifImage { left: 0, top: 0, width: w, height: h, interlace: false, lct: Some(GifColorTable { sorted: false, colors: vec![GifRgb { r: seed, g: seed, b: seed }; 2] }), indices: vec![0u8; (w * h) as usize] }
    }

    /// 🧪️ Canonical absorb case 1: `Insert(2,f)` then `Remove(0)` → `{removed:[0], added:[(1,f)]}`.
    #[semio_framework_async_macros::async_test]
    async fn absorb_insert_then_remove_before_shifts_index() {
        let f = img(9, 2, 2);
        let mut d1 = GifImagesDiff { added: vec![GifImageAdded { index: 2, image: f.clone() }], ..Default::default() };
        let d2 = GifImagesDiff { removed: vec![0], ..Default::default() };
        d1.absorb(d2);
        assert_eq!(d1.removed, vec![0]);
        assert_eq!(d1.added, vec![GifImageAdded { index: 1, image: f }]);
        assert!(d1.modified.is_empty());
    }

    /// 🧪️ Canonical absorb case 2: `Insert(2,f)` then `Insert(2,g)` → BOTH survive as
    /// `added:[(2,g),(3,f)]` — the exact LWW-slot bug this recipe replaces.
    #[semio_framework_async_macros::async_test]
    async fn absorb_insert_insert_same_index_both_survive() {
        let f = img(1, 2, 2);
        let g = img(2, 2, 2);
        let mut d1 = GifImagesDiff { added: vec![GifImageAdded { index: 2, image: f.clone() }], ..Default::default() };
        let d2 = GifImagesDiff { added: vec![GifImageAdded { index: 2, image: g.clone() }], ..Default::default() };
        d1.absorb(d2);
        assert_eq!(d1.added, vec![GifImageAdded { index: 2, image: g }, GifImageAdded { index: 3, image: f },]);
    }

    /// 🧪️ Canonical absorb case 3: `Insert(1,f)` then `SetField(1,v)` patches INTO the added
    /// payload — merged has only `added`, no separate `modified` entry.
    #[semio_framework_async_macros::async_test]
    async fn absorb_insert_then_set_field_patches_into_added() {
        let f = img(1, 2, 2);
        let mut d1 = GifImagesDiff { added: vec![GifImageAdded { index: 1, image: f.clone() }], ..Default::default() };
        let d2 = GifImagesDiff { modified: vec![GifImageModified { index: 1, diff: GifImageDiff { interlace: Some(true), ..Default::default() } }], ..Default::default() };
        d1.absorb(d2);
        assert!(d1.modified.is_empty());
        assert_eq!(d1.added.len(), 1);
        assert!(d1.added[0].image.interlace);
        assert_eq!(d1.added[0].index, 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_law_holds_over_curated_ops() {
        let base = GifSnapshot { images: vec![img(1, 2, 2), img(2, 2, 2), img(3, 2, 2)], ..GifSnapshot::default() };
        let mid = {
            let mut s = base.clone();
            s.images.insert(1, img(9, 2, 2));
            s.images.remove(0);
            s
        };
        let after = {
            let mut s = mid.clone();
            s.images[0].interlace = true;
            s.images.push(img(5, 2, 2));
            s
        };
        let mut d1 = <GifDiff as DiffAlgebra<GifSnapshot>>::between(&base, &mid);
        let d2 = <GifDiff as DiffAlgebra<GifSnapshot>>::between(&mid, &after);
        d1.absorb(d2);
        assert_eq!(d1.apply(&base).unwrap(), after);
    }

    #[semio_framework_async_macros::async_test]
    async fn between_roundtrip_law() {
        let a = GifSnapshot { width: 4, height: 4, images: vec![img(1, 4, 4)], ..GifSnapshot::default() };
        let b = GifSnapshot { width: 4, height: 4, images: vec![img(1, 4, 4), img(2, 2, 2)], ..GifSnapshot::default() };
        let ab = <GifDiff as DiffAlgebra<GifSnapshot>>::between(&a, &b);
        assert_eq!(ab.apply(&a).unwrap(), b);
        let ba = <GifDiff as DiffAlgebra<GifSnapshot>>::between(&b, &a);
        assert_eq!(ba.apply(&b).unwrap(), a);
        assert!(<GifDiff as DiffAlgebra<GifSnapshot>>::between(&a, &a).is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn inverse_law() {
        let base = GifSnapshot { images: vec![img(1, 2, 2), img(2, 2, 2)], ..GifSnapshot::default() };
        let next = {
            let mut s = base.clone();
            s.images[0].interlace = true;
            s.images.remove(1);
            s.images.push(img(7, 3, 3));
            s.background_color_index = 5;
            s
        };
        let d = <GifDiff as DiffAlgebra<GifSnapshot>>::between(&base, &next);
        let mutated = d.apply(&base).unwrap();
        let inv = d.inverse(&base);
        assert_eq!(inv.apply(&mutated).unwrap(), base);
    }

    /// 🧪️ Field sweep — the acceptance criterion: `sweep_a`/`sweep_b` differ in EVERY mutable
    /// field, with asymmetric image-collection lengths (F1's structural trap: a single
    /// index-keyed `between()` call can show `removed` XOR `added`, never both — so assertions are
    /// split across both directions, per `f1-closer-report.md` §4.4).
    #[semio_framework_async_macros::async_test]
    async fn field_sweep_covers_every_mutable_field() {
        let sweep_a = GifSnapshot {
            schema: STDIO_GIF_DOCUMENT_SCHEMA.into(),
            width: 10,
            height: 8,
            gct: Some(GifColorTable { sorted: false, colors: vec![GifRgb { r: 1, g: 2, b: 3 }; 2] }),
            background_color_index: 0,
            pixel_aspect_ratio: 0,
            images: vec![img(1, 2, 2), img(2, 2, 2)],
        };
        let mut sweep_b = GifSnapshot {
            schema: STDIO_GIF_DOCUMENT_SCHEMA.into(),
            width: 20,
            height: 16,
            gct: Some(GifColorTable { sorted: true, colors: vec![GifRgb { r: 9, g: 9, b: 9 }; 4] }),
            background_color_index: 3,
            pixel_aspect_ratio: 7,
            images: vec![img(1, 2, 2)],
        };
        sweep_b.images[0].interlace = true;
        sweep_b.images.push(img(5, 3, 3));
        sweep_b.images.push(img(6, 3, 3));

        let ab = <GifDiff as DiffAlgebra<GifSnapshot>>::between(&sweep_a, &sweep_b);
        assert_eq!(ab.apply(&sweep_a).unwrap(), sweep_b);
        assert!(ab.width.is_some());
        assert!(ab.height.is_some());
        assert!(ab.gct.is_some());
        assert!(ab.background_color_index.is_some());
        assert!(ab.pixel_aspect_ratio.is_some());
        let images_ab = ab.images.as_ref().expect("images must differ");
        assert!(!images_ab.modified.is_empty(), "sweep must exercise a modified image");
        assert!(!images_ab.added.is_empty(), "sweep must exercise an added image (b is longer)");

        let ba = <GifDiff as DiffAlgebra<GifSnapshot>>::between(&sweep_b, &sweep_a);
        assert_eq!(ba.apply(&sweep_b).unwrap(), sweep_a);
        let images_ba = ba.images.as_ref().expect("images must differ");
        assert!(!images_ba.removed.is_empty(), "reverse direction must exercise a removed image (a is shorter)");

        assert!(<GifDiff as DiffAlgebra<GifSnapshot>>::between(&sweep_a, &sweep_a).is_empty());
    }

    /// 🧪️ Tri-state nullable field: `gct` going from `Some` to `None` must be `Some(None)`, not
    /// absent from the diff.
    #[semio_framework_async_macros::async_test]
    async fn gct_tristate_removal_is_some_none() {
        let a = GifSnapshot { gct: Some(GifColorTable { sorted: false, colors: vec![GifRgb::default(); 2] }), ..GifSnapshot::default() };
        let b = GifSnapshot { gct: None, ..GifSnapshot::default() };
        let d = <GifDiff as DiffAlgebra<GifSnapshot>>::between(&a, &b);
        assert_eq!(d.gct, Some(None));
        assert_eq!(d.apply(&a).unwrap(), b);
    }

    /// 🧪️ F6: `DiffCodec` round-trip laws for the hand-rolled `GifDiff` text/binary grammar —
    /// exercises scalars, both tri-states (`gct` at the top level, `lct` inside a modified image),
    /// and the `images` collection triple (`removed`/`modified`/`added`) simultaneously via a real
    /// `between()` result — mirrors gif89a's `diff_codec_text_binary_roundtrip_law`.
    #[semio_framework_async_macros::async_test]
    async fn diff_codec_text_binary_roundtrip_law() {
        let a = GifSnapshot { width: 10, height: 8, gct: Some(GifColorTable { sorted: false, colors: vec![GifRgb { r: 1, g: 2, b: 3 }; 2] }), images: vec![img(1, 2, 2), img(2, 2, 2)], ..GifSnapshot::default() };
        let mut ib0 = img(1, 2, 2);
        ib0.interlace = true;
        ib0.lct = None;
        let b = GifSnapshot { width: 20, height: 16, gct: None, images: vec![ib0, img(6, 3, 3), img(7, 3, 3)], ..GifSnapshot::default() };
        let cases = vec![GifDiff::default(), <GifDiff as DiffAlgebra<GifSnapshot>>::between(&a, &b), <GifDiff as DiffAlgebra<GifSnapshot>>::between(&b, &a)];
        for d in cases {
            let printed = d.print_diff();
            assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
            let parsed = GifDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
            assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

            let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
            let decoded = GifDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
            assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
        }
    }
}
//#endregion Tests
