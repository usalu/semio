//! 🔺️ GifDiff (87a) — sparse per-field diff, handcrafted per the ticket's recipe. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: replaces the prior
//! `{snapshot: Option<GifSnapshot>}` full-replace stub outright — no `snapshot` slot survives.
//! `images` is a strong, index-keyed collection (`GifImagesDiff{removed,modified,added}`); `gct`
//! and every scalar screen field get their own sparse slot; `GifColorTable`/`GifImage` (the weak
//! leaf pieces) are whole-value replaced, never sub-diffed further, per the recipe's strong/weak
//! split.

/// 🧩 Ordered removed keys, modified values, and inserted items.
pub(crate) type IndexedDiffParts<D, T> = (Vec<usize>, Vec<(usize, D)>, Vec<(usize, T)>);

use crate::standards::v87a::subsets::any::schema::snapshot::{GifColorTable, GifImage, GifRgb, GifSnapshot};
use framework_schema::ArtifactSchema;
use protocol::os_spr::command::DiffAlgebra;
use protocol::{DiffCodec};
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};

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
//#endregion 🔖️GenericCollectionAlgebra

//#region 🔖️ImageDiff
/// 🔺️ Sparse per-field diff for one [`GifImage`].
/// 🧪️ F6 FINDING: `#[derive(dsl::DslRecord)]`/`#[derive(dsl::)]` CANNOT be used on this
/// struct — it has a tri-state `Option<Option<T>>` field (`lct`), which the derive's
/// `classify_field` cannot bind: it peels exactly ONE `Option<..>` layer via `inner_of(ty,
/// "Option")`, leaving the REMAINING type as `Option<GifColorTable>` itself, which then needs
/// `Option<GifColorTable>: DslField` — a blanket impl that does not exist anywhere in the `dsl`
/// crate (confirmed empirically: `cargo check` gives `the trait bound
/// std::option::Option<v87a::...::GifColorTable>: DslField is not satisfied`, matching gif89a's
/// `GifFrameDiff` finding exactly — see `f6-recon-report.md` §3b). `DiffCodec` for `GifDiff` is
/// hand-rolled below instead (this struct itself needs no `dsl` derive at all; it's a plain leaf
/// type consumed by the hand-rolled `print_diff`/`parse_diff`/`encode_diff`/`decode_diff`).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GifImageDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub left: Option<u32>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub top: Option<u32>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub interlace: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub lct: Option<Option<GifColorTable>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
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
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GifImageModified {
    pub index: usize,
    pub diff: GifImageDiff,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GifImageAdded {
    pub index: usize,
    pub image: GifImage,
}

/// 🔺️ Index-keyed collection triple for `GifSnapshot::images`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GifImagesDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<GifImageModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
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
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.gif.diff")]
pub struct GifDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub gct: Option<Option<GifColorTable>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub background_color_index: Option<u8>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub pixel_aspect_ratio: Option<u8>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub images: Option<GifImagesDiff>,
}

impl GifDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty_diff(&self) -> bool {
        self.width.is_none() && self.height.is_none() && self.gct.is_none() && self.background_color_index.is_none() && self.pixel_aspect_ratio.is_none() && self.images.as_ref().is_none_or(GifImagesDiff::is_empty)
    }
}

impl MutationDiff<GifSnapshot> for GifDiff {
    fn apply(&self, base: &GifSnapshot) -> MutationApplyResult<GifSnapshot> {
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

    fn absorb(&mut self, other: Self) {
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
    fn inverse(&self, base: &GifSnapshot) -> Self {
        Self {
            width: self.width.map(|_| base.width),
            height: self.height.map(|_| base.height),
            gct: self.gct.as_ref().map(|_| base.gct.clone()),
            background_color_index: self.background_color_index.map(|_| base.background_color_index),
            pixel_aspect_ratio: self.pixel_aspect_ratio.map(|_| base.pixel_aspect_ratio),
            images: self.images.as_ref().map(|d| d.inverse(&base.images)),
        }
    }

    fn between(base: &GifSnapshot, other: &GifSnapshot) -> Self {
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

    fn is_empty(&self) -> bool {
        self.is_empty_diff()
    }
}

/// 🧩 Builds a set-snapshot diff — sparse field-by-field, never a full-replace slot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_snapshot(base: &GifSnapshot, snapshot: &GifSnapshot) -> GifDiff {
    <GifDiff as DiffAlgebra<GifSnapshot>>::between(base, snapshot)
}

/// 🧪️ P2-FG2: representative `GifDiff` cases for `diff_grammar_conformance_law`/
/// `protocol_walk_law` (`../../../../⚙️engine/🦀️.rs`'s `conformance_laws` module) —
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
// 🚫️aaaaa�️aaaregion 🔖️Primitives

//#region 🔖️ValueCodecs
// 🚫️aaaaaaregion 🔖️ValueCodecs

//#region 🔖️DiffValueCodecs
// 🚫️aa�️a️aaregion 🔖️DiffValueCodecs

//#region 🔖️RealBinaryPrimitives

//#endregion 🔖️RealBinaryPrimitives

//#region 🔖️RealBinaryDiffFrame
/// 🧪️aaaregion 🔖️RealBinaryDiffFrame

//#region 🔖️TopLevel
// 🚫️aaDiregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec

//#region Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion Tests

#[cfg(test)]
use protocol::{DiffBinary,DiffText};
