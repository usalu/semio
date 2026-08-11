//! 🔺️ GifDiff (89a) — sparse per-field diff, handcrafted per the ticket's recipe. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: **DELETES the prior
//! op-slot shape wholesale** (`snapshot: Option<GifSnapshot>` full-replace slot PLUS one
//! `Option<T>` per mutation kind, with a known LWW-loses-coalesced-inserts absorb bug) and
//! replaces it with three independent index-keyed collection triples (`frames`, `comments`,
//! `app_extensions`) plus a sparse scalar slot per screen-level field. `frames` is the strong,
//! per-field-diffable collection (`GifFrameDiff` covers every `GifFrame` field, incl. tri-state
//! `lct`/`transparent_index`/`plain_text`); `comments`/`app_extensions` are weak/value collections
//! whose "diff" IS the whole new item (per the recipe's strong/weak split — a `String` or
//! `GifAppExtension` has no further sub-structure worth diffing).

use crate::artifacts::gif::standards::v89a::subsets::any::schema::snapshot::{
    GifAppExtension, GifColorTable, GifDisposal, GifFrame, GifPlainText, GifRgb, GifSnapshot,
};
use protocol::MutationDiff;
use protocol::os_spr::command::DiffAlgebra;
#[cfg(test)]
use protocol::DiffCodec;
use serde::{Deserialize, Serialize};
use schema::ArtifactSchema;

//#region 🔖️IndexTransport
/// 📐️ Shared rank/unrank arithmetic for index-keyed collection diffs (`between`/`absorb`/
/// `inverse`) — see `🧬️schema-design.md` §Absorb and the top-level plan's "Absorb" section for the
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

//#region 🔖️FrameDiff
/// 🔺️ Sparse per-field diff for one [`GifFrame`] — a strong entity, per the recipe.
/// 🧪️ F6-PILOT FINDING: `#[derive(dsl::DslRecord)]`/`#[derive(dsl::DslDiff)]` CANNOT be used on
/// this struct — it has tri-state `Option<Option<T>>` fields (`lct`, `transparent_index`,
/// `plain_text`), which the derive's `classify_field` cannot bind: it peels exactly ONE `Option<..>`
/// layer via `inner_of(ty, "Option")`, leaving the REMAINING type as `Option<T>` itself, which
/// then needs `Option<T>: DslField` — a blanket impl that does not exist anywhere in the `dsl`
/// crate (confirmed empirically: `cargo check` gives `the trait bound
/// std::option::Option<GifColorTable>: DslField is not satisfied`, ditto `Option<u8>`,
/// `Option<GifPlainText>`). Since tri-state IS the plan's own normative representation for every
/// nullable snapshot field (`🧬️schema-design.md` / top-level plan's Diff recipe), this blocks the
/// derive far more broadly than the documented "enum node" restriction — see the ticket's
/// `f6-recon-report.md` for the full decision rule. `DiffCodec` for `GifDiff` is hand-rolled below
/// instead (this struct itself needs no `dsl` derive at all; it's a plain leaf type consumed by the
/// hand-rolled `print_diff`/`parse_diff`/`encode_diff`/`decode_diff`).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GifFrameDiff {
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delay_cs: Option<u16>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disposal: Option<GifDisposal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transparent_index: Option<Option<u8>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_input: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plain_text: Option<Option<GifPlainText>>,
}

impl GifFrameDiff {
    pub fn is_empty(&self) -> bool {
        self.left.is_none() && self.top.is_none() && self.width.is_none() && self.height.is_none()
            && self.interlace.is_none() && self.lct.is_none() && self.indices.is_none()
            && self.delay_cs.is_none() && self.disposal.is_none() && self.transparent_index.is_none()
            && self.user_input.is_none() && self.plain_text.is_none()
    }

    pub fn between(base: &GifFrame, other: &GifFrame) -> Self {
        Self {
            left: (base.left != other.left).then_some(other.left),
            top: (base.top != other.top).then_some(other.top),
            width: (base.width != other.width).then_some(other.width),
            height: (base.height != other.height).then_some(other.height),
            interlace: (base.interlace != other.interlace).then_some(other.interlace),
            lct: (base.lct != other.lct).then_some(other.lct.clone()),
            indices: (base.indices != other.indices).then_some(other.indices.clone()),
            delay_cs: (base.delay_cs != other.delay_cs).then_some(other.delay_cs),
            disposal: (base.disposal != other.disposal).then_some(other.disposal),
            transparent_index: (base.transparent_index != other.transparent_index).then_some(other.transparent_index),
            user_input: (base.user_input != other.user_input).then_some(other.user_input),
            plain_text: (base.plain_text != other.plain_text).then_some(other.plain_text.clone()),
        }
    }

    pub fn apply(&self, base: &GifFrame) -> GifFrame {
        let mut next = base.clone();
        if let Some(v) = self.left { next.left = v; }
        if let Some(v) = self.top { next.top = v; }
        if let Some(v) = self.width { next.width = v; }
        if let Some(v) = self.height { next.height = v; }
        if let Some(v) = self.interlace { next.interlace = v; }
        if let Some(v) = &self.lct { next.lct = v.clone(); }
        if let Some(v) = &self.indices { next.indices = v.clone(); }
        if let Some(v) = self.delay_cs { next.delay_cs = v; }
        if let Some(v) = self.disposal { next.disposal = v; }
        if let Some(v) = self.transparent_index { next.transparent_index = v; }
        if let Some(v) = self.user_input { next.user_input = v; }
        if let Some(v) = &self.plain_text { next.plain_text = v.clone(); }
        next
    }

    pub fn inverse(&self, base: &GifFrame) -> Self {
        Self {
            left: self.left.map(|_| base.left),
            top: self.top.map(|_| base.top),
            width: self.width.map(|_| base.width),
            height: self.height.map(|_| base.height),
            interlace: self.interlace.map(|_| base.interlace),
            lct: self.lct.as_ref().map(|_| base.lct.clone()),
            indices: self.indices.as_ref().map(|_| base.indices.clone()),
            delay_cs: self.delay_cs.map(|_| base.delay_cs),
            disposal: self.disposal.map(|_| base.disposal),
            transparent_index: self.transparent_index.map(|_| base.transparent_index),
            user_input: self.user_input.map(|_| base.user_input),
            plain_text: self.plain_text.as_ref().map(|_| base.plain_text.clone()),
        }
    }

    fn absorb(&mut self, other: Self) {
        if other.left.is_some() { self.left = other.left; }
        if other.top.is_some() { self.top = other.top; }
        if other.width.is_some() { self.width = other.width; }
        if other.height.is_some() { self.height = other.height; }
        if other.interlace.is_some() { self.interlace = other.interlace; }
        if other.lct.is_some() { self.lct = other.lct; }
        if other.indices.is_some() { self.indices = other.indices; }
        if other.delay_cs.is_some() { self.delay_cs = other.delay_cs; }
        if other.disposal.is_some() { self.disposal = other.disposal; }
        if other.transparent_index.is_some() { self.transparent_index = other.transparent_index; }
        if other.user_input.is_some() { self.user_input = other.user_input; }
        if other.plain_text.is_some() { self.plain_text = other.plain_text; }
    }
}
//#endregion 🔖️FrameDiff

//#region 🔖️FramesDiff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GifFrameModified {
    pub index: usize,
    pub diff: GifFrameDiff,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GifFrameAdded {
    pub index: usize,
    pub frame: GifFrame,
}

/// 🔺️ Index-keyed collection triple for `GifSnapshot::frames`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GifFramesDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<GifFrameModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<GifFrameAdded>,
}

impl GifFramesDiff {
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }

    pub fn between(base: &[GifFrame], other: &[GifFrame]) -> Self {
        let min = base.len().min(other.len());
        let mut modified = Vec::new();
        for i in 0..min {
            let d = GifFrameDiff::between(&base[i], &other[i]);
            if !d.is_empty() {
                modified.push(GifFrameModified { index: i, diff: d });
            }
        }
        let removed: Vec<usize> = (min..base.len()).collect();
        let added: Vec<GifFrameAdded> = (min..other.len())
            .map(|i| GifFrameAdded { index: i, frame: other[i].clone() })
            .collect();
        Self { removed, modified, added }
    }

    pub fn apply(&self, base: &[GifFrame]) -> Vec<GifFrame> {
        let mut next: Vec<Option<GifFrame>> = base.iter().cloned().map(Some).collect();
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
        let mut out: Vec<GifFrame> = next.into_iter().flatten().collect();
        let mut added_sorted = self.added.clone();
        added_sorted.sort_by_key(|a| a.index);
        for a in added_sorted {
            let at = a.index.min(out.len());
            out.insert(at, a.frame);
        }
        out
    }

    fn absorb(&mut self, other: Self) {
        let (removed, modified, added) = absorb_indexed_collection(
            std::mem::take(&mut self.removed),
            std::mem::take(&mut self.modified).into_iter().map(|m| (m.index, m.diff)).collect(),
            std::mem::take(&mut self.added).into_iter().map(|a| (a.index, a.frame)).collect(),
            other.removed,
            other.modified.into_iter().map(|m| (m.index, m.diff)).collect(),
            other.added.into_iter().map(|a| (a.index, a.frame)).collect(),
            |d, o| d.absorb(o),
            |d, item| d.apply(item),
        );
        self.removed = removed;
        self.modified = modified.into_iter().map(|(index, diff)| GifFrameModified { index, diff }).collect();
        self.added = added.into_iter().map(|(index, frame)| GifFrameAdded { index, frame }).collect();
    }

    fn inverse(&self, base_frames: &[GifFrame]) -> Self {
        let (removed, modified, added) = inverse_indexed_collection(
            &self.removed,
            &self.modified.iter().map(|m| (m.index, m.diff.clone())).collect::<Vec<_>>(),
            &self.added.iter().map(|a| (a.index, a.frame.clone())).collect::<Vec<_>>(),
            base_frames,
            |d, item| d.inverse(item),
        );
        Self {
            removed,
            modified: modified.into_iter().map(|(index, diff)| GifFrameModified { index, diff }).collect(),
            added: added.into_iter().map(|(index, frame)| GifFrameAdded { index, frame }).collect(),
        }
    }
}
//#endregion 🔖️FramesDiff

//#region 🔖️WeakCollectionDiffs
/// 🧩️ Macro-free, hand-duplicated (small, two instantiations) index-keyed collection triple for a
/// WEAK/value collection item — the "diff" IS the whole new value, per the recipe's strong/weak
/// split (no further sub-diffing of a `String` or a `GifAppExtension`).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GifCommentModified {
    pub index: usize,
    pub text: String,
}
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GifCommentAdded {
    pub index: usize,
    pub text: String,
}
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GifCommentsDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<GifCommentModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<GifCommentAdded>,
}

impl GifCommentsDiff {
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
    pub fn between(base: &[String], other: &[String]) -> Self {
        let min = base.len().min(other.len());
        let modified = (0..min).filter(|&i| base[i] != other[i]).map(|i| GifCommentModified { index: i, text: other[i].clone() }).collect();
        let removed: Vec<usize> = (min..base.len()).collect();
        let added: Vec<GifCommentAdded> = (min..other.len()).map(|i| GifCommentAdded { index: i, text: other[i].clone() }).collect();
        Self { removed, modified, added }
    }
    pub fn apply(&self, base: &[String]) -> Vec<String> {
        let mut next: Vec<Option<String>> = base.iter().cloned().map(Some).collect();
        for m in &self.modified {
            if let Some(slot) = next.get_mut(m.index) { *slot = Some(m.text.clone()); }
        }
        let mut removed_sorted = self.removed.clone();
        removed_sorted.sort_unstable();
        removed_sorted.reverse();
        for &r in &removed_sorted { if r < next.len() { next.remove(r); } }
        let mut out: Vec<String> = next.into_iter().flatten().collect();
        let mut added_sorted = self.added.clone();
        added_sorted.sort_by_key(|a| a.index);
        for a in added_sorted { let at = a.index.min(out.len()); out.insert(at, a.text); }
        out
    }
    fn absorb(&mut self, other: Self) {
        let (removed, modified, added) = absorb_indexed_collection(
            std::mem::take(&mut self.removed),
            std::mem::take(&mut self.modified).into_iter().map(|m| (m.index, m.text)).collect(),
            std::mem::take(&mut self.added).into_iter().map(|a| (a.index, a.text)).collect(),
            other.removed,
            other.modified.into_iter().map(|m| (m.index, m.text)).collect(),
            other.added.into_iter().map(|a| (a.index, a.text)).collect(),
            |d, o| *d = o,
            |d, _item| d.clone(),
        );
        self.removed = removed;
        self.modified = modified.into_iter().map(|(index, text)| GifCommentModified { index, text }).collect();
        self.added = added.into_iter().map(|(index, text)| GifCommentAdded { index, text }).collect();
    }
    fn inverse(&self, base_comments: &[String]) -> Self {
        let (removed, modified, added) = inverse_indexed_collection(
            &self.removed,
            &self.modified.iter().map(|m| (m.index, m.text.clone())).collect::<Vec<_>>(),
            &self.added.iter().map(|a| (a.index, a.text.clone())).collect::<Vec<_>>(),
            base_comments,
            |_d, item| item.clone(),
        );
        Self {
            removed,
            modified: modified.into_iter().map(|(index, text)| GifCommentModified { index, text }).collect(),
            added: added.into_iter().map(|(index, text)| GifCommentAdded { index, text }).collect(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GifAppExtensionModified {
    pub index: usize,
    pub extension: GifAppExtension,
}
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GifAppExtensionAdded {
    pub index: usize,
    pub extension: GifAppExtension,
}
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GifAppExtensionsDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<GifAppExtensionModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<GifAppExtensionAdded>,
}

impl GifAppExtensionsDiff {
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
    pub fn between(base: &[GifAppExtension], other: &[GifAppExtension]) -> Self {
        let min = base.len().min(other.len());
        let modified = (0..min).filter(|&i| base[i] != other[i]).map(|i| GifAppExtensionModified { index: i, extension: other[i].clone() }).collect();
        let removed: Vec<usize> = (min..base.len()).collect();
        let added: Vec<GifAppExtensionAdded> = (min..other.len()).map(|i| GifAppExtensionAdded { index: i, extension: other[i].clone() }).collect();
        Self { removed, modified, added }
    }
    pub fn apply(&self, base: &[GifAppExtension]) -> Vec<GifAppExtension> {
        let mut next: Vec<Option<GifAppExtension>> = base.iter().cloned().map(Some).collect();
        for m in &self.modified {
            if let Some(slot) = next.get_mut(m.index) { *slot = Some(m.extension.clone()); }
        }
        let mut removed_sorted = self.removed.clone();
        removed_sorted.sort_unstable();
        removed_sorted.reverse();
        for &r in &removed_sorted { if r < next.len() { next.remove(r); } }
        let mut out: Vec<GifAppExtension> = next.into_iter().flatten().collect();
        let mut added_sorted = self.added.clone();
        added_sorted.sort_by_key(|a| a.index);
        for a in added_sorted { let at = a.index.min(out.len()); out.insert(at, a.extension); }
        out
    }
    fn absorb(&mut self, other: Self) {
        let (removed, modified, added) = absorb_indexed_collection(
            std::mem::take(&mut self.removed),
            std::mem::take(&mut self.modified).into_iter().map(|m| (m.index, m.extension)).collect(),
            std::mem::take(&mut self.added).into_iter().map(|a| (a.index, a.extension)).collect(),
            other.removed,
            other.modified.into_iter().map(|m| (m.index, m.extension)).collect(),
            other.added.into_iter().map(|a| (a.index, a.extension)).collect(),
            |d, o| *d = o,
            |d, _item| d.clone(),
        );
        self.removed = removed;
        self.modified = modified.into_iter().map(|(index, extension)| GifAppExtensionModified { index, extension }).collect();
        self.added = added.into_iter().map(|(index, extension)| GifAppExtensionAdded { index, extension }).collect();
    }
    fn inverse(&self, base_exts: &[GifAppExtension]) -> Self {
        let (removed, modified, added) = inverse_indexed_collection(
            &self.removed,
            &self.modified.iter().map(|m| (m.index, m.extension.clone())).collect::<Vec<_>>(),
            &self.added.iter().map(|a| (a.index, a.extension.clone())).collect::<Vec<_>>(),
            base_exts,
            |_d, item| item.clone(),
        );
        Self {
            removed,
            modified: modified.into_iter().map(|(index, extension)| GifAppExtensionModified { index, extension }).collect(),
            added: added.into_iter().map(|(index, extension)| GifAppExtensionAdded { index, extension }).collect(),
        }
    }
}
//#endregion 🔖️WeakCollectionDiffs

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.gif.89a`. No `snapshot: Option<GifSnapshot>` full-replace slot anywhere.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.gif.89a.diff")]
pub struct GifDiff {
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gct: Option<Option<GifColorTable>>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background_color_index: Option<u8>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pixel_aspect_ratio: Option<u8>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub loop_count: Option<Option<u16>>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frames: Option<GifFramesDiff>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comments: Option<GifCommentsDiff>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_extensions: Option<GifAppExtensionsDiff>,
}

impl GifDiff {
    pub fn is_empty_diff(&self) -> bool {
        self.width.is_none() && self.height.is_none() && self.gct.is_none()
            && self.background_color_index.is_none() && self.pixel_aspect_ratio.is_none()
            && self.loop_count.is_none()
            && self.frames.as_ref().map(GifFramesDiff::is_empty).unwrap_or(true)
            && self.comments.as_ref().map(GifCommentsDiff::is_empty).unwrap_or(true)
            && self.app_extensions.as_ref().map(GifAppExtensionsDiff::is_empty).unwrap_or(true)
    }
}

impl MutationDiff<GifSnapshot> for GifDiff {
    fn apply(&self, base: &GifSnapshot) -> GifSnapshot {
        let mut next = base.clone();
        if let Some(v) = self.width { next.width = v; }
        if let Some(v) = self.height { next.height = v; }
        if let Some(v) = &self.gct { next.gct = v.clone(); }
        if let Some(v) = self.background_color_index { next.background_color_index = v; }
        if let Some(v) = self.pixel_aspect_ratio { next.pixel_aspect_ratio = v; }
        if let Some(v) = self.loop_count { next.loop_count = v; }
        if let Some(d) = &self.frames { next.frames = d.apply(&next.frames); }
        if let Some(d) = &self.comments { next.comments = d.apply(&next.comments); }
        if let Some(d) = &self.app_extensions { next.app_extensions = d.apply(&next.app_extensions); }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.width.is_some() { self.width = other.width; }
        if other.height.is_some() { self.height = other.height; }
        if other.gct.is_some() { self.gct = other.gct; }
        if other.background_color_index.is_some() { self.background_color_index = other.background_color_index; }
        if other.pixel_aspect_ratio.is_some() { self.pixel_aspect_ratio = other.pixel_aspect_ratio; }
        if other.loop_count.is_some() { self.loop_count = other.loop_count; }
        match (&mut self.frames, other.frames) {
            (Some(mine), Some(theirs)) => mine.absorb(theirs),
            (slot @ None, Some(theirs)) => *slot = Some(theirs),
            _ => {}
        }
        match (&mut self.comments, other.comments) {
            (Some(mine), Some(theirs)) => mine.absorb(theirs),
            (slot @ None, Some(theirs)) => *slot = Some(theirs),
            _ => {}
        }
        match (&mut self.app_extensions, other.app_extensions) {
            (Some(mine), Some(theirs)) => mine.absorb(theirs),
            (slot @ None, Some(theirs)) => *slot = Some(theirs),
            _ => {}
        }
    }
}

impl DiffAlgebra<GifSnapshot> for GifDiff {
    fn inverse(&self, base: &GifSnapshot) -> Self {
        Self {
            width: self.width.map(|_| base.width),
            height: self.height.map(|_| base.height),
            gct: self.gct.as_ref().map(|_| base.gct.clone()),
            background_color_index: self.background_color_index.map(|_| base.background_color_index),
            pixel_aspect_ratio: self.pixel_aspect_ratio.map(|_| base.pixel_aspect_ratio),
            loop_count: self.loop_count.map(|_| base.loop_count),
            frames: self.frames.as_ref().map(|d| d.inverse(&base.frames)),
            comments: self.comments.as_ref().map(|d| d.inverse(&base.comments)),
            app_extensions: self.app_extensions.as_ref().map(|d| d.inverse(&base.app_extensions)),
        }
    }

    fn between(base: &GifSnapshot, other: &GifSnapshot) -> Self {
        let frames_diff = GifFramesDiff::between(&base.frames, &other.frames);
        let comments_diff = GifCommentsDiff::between(&base.comments, &other.comments);
        let app_extensions_diff = GifAppExtensionsDiff::between(&base.app_extensions, &other.app_extensions);
        Self {
            width: (base.width != other.width).then_some(other.width),
            height: (base.height != other.height).then_some(other.height),
            gct: (base.gct != other.gct).then_some(other.gct.clone()),
            background_color_index: (base.background_color_index != other.background_color_index).then_some(other.background_color_index),
            pixel_aspect_ratio: (base.pixel_aspect_ratio != other.pixel_aspect_ratio).then_some(other.pixel_aspect_ratio),
            loop_count: (base.loop_count != other.loop_count).then_some(other.loop_count),
            frames: (!frames_diff.is_empty()).then_some(frames_diff),
            comments: (!comments_diff.is_empty()).then_some(comments_diff),
            app_extensions: (!app_extensions_diff.is_empty()).then_some(app_extensions_diff),
        }
    }

    fn is_empty(&self) -> bool {
        self.is_empty_diff()
    }
}

/// 🧩 Builds a set-snapshot diff — sparse field-by-field, never a full-replace slot.
pub fn diff_set_snapshot(base: &GifSnapshot, snapshot: &GifSnapshot) -> GifDiff {
    <GifDiff as DiffAlgebra<GifSnapshot>>::between(base, snapshot)
}

/// 🧪️ P2-FG2: representative `GifDiff` (89a) cases for `diff_grammar_conformance_law`/
/// `protocol_walk_law` (`../../../../⚙️engine/🦀️component.rs`'s `conformance_laws` module) —
/// the empty diff, plus a real `between()` result exercising every scalar field, both
/// tri-states (`gct`, `loop_count`), `GifFrameDiff`'s own THREE nested tri-states
/// (`lct`/`transparent_index`/`plain_text`), and all three collection triples (`frames`,
/// `comments`, `app_extensions`) at once (mirrors 87a's own `demo_diff_cases()`).
pub(crate) fn demo_diff_cases() -> Vec<GifDiff> {
    let f = |seed: u8, w: u32, h: u32| GifFrame {
        left: 0, top: 0, width: w, height: h,
        interlace: false,
        lct: Some(GifColorTable { sorted: false, colors: vec![GifRgb { r: seed, g: seed, b: seed }; 2] }),
        indices: vec![0u8; (w * h) as usize],
        delay_cs: 10,
        disposal: GifDisposal::DoNotDispose,
        transparent_index: Some(0),
        user_input: false,
        plain_text: None,
    };
    let a = GifSnapshot {
        width: 4, height: 4,
        gct: Some(GifColorTable { sorted: false, colors: vec![GifRgb { r: 1, g: 2, b: 3 }; 2] }),
        loop_count: Some(0),
        frames: vec![f(1, 2, 2), f(2, 2, 2)],
        comments: vec!["hello".into()],
        app_extensions: vec![GifAppExtension { identifier: *b"NETSCAPE", auth_code: *b"2.0", data: vec![1, 0, 0] }],
        ..GifSnapshot::default()
    };
    let mut fb0 = f(1, 2, 2);
    fb0.interlace = true;
    fb0.lct = None;
    fb0.transparent_index = None;
    fb0.plain_text = Some(GifPlainText { left: 0, top: 0, width: 4, height: 1, cell_width: 4, cell_height: 8, fg_color_index: 0, bg_color_index: 1, text: "hi".into() });
    let b = GifSnapshot {
        width: 8, height: 8,
        gct: None,
        background_color_index: 3,
        pixel_aspect_ratio: 5,
        loop_count: None,
        frames: vec![fb0, f(6, 3, 3), f(7, 3, 3)],
        comments: vec![],
        app_extensions: vec![],
        ..GifSnapshot::default()
    };
    vec![
        GifDiff::default(),
        diff_set_snapshot(&a, &b),
        diff_set_snapshot(&b, &a),
    ]
}
//#endregion 🔖️Diff

//#region 🔖️HandcraftedDiffCodec
/// 🧪️ F6-PILOT: **hand-rolled** `protocol::DiffCodec` for `GifDiff` — the derive path
/// (`#[derive(dsl::DslDiff)]`) is NOT usable here: `GifDiff` (and `GifFrameDiff` nested inside its
/// `frames` collection) both carry tri-state `Option<Option<T>>` fields (`gct`, `loop_count`; per
/// `GifFrameDiff`: `lct`, `transparent_index`, `plain_text`), which the derive cannot bind (see the
/// doc comment on `GifFrameDiff` above and `f6-recon-report.md` for the confirmed compile error).
/// This is the SAME hand-rolled path `SvgDiff`'s mutations/🔺️diff impl uses for its enum-node
/// reason — two independent reasons land an artifact on the same "hand-roll it" path.
///
/// **Grammar** (real, not `serde_json`): one space-separated `name=value` token per changed
/// top-level field (a field absent from the line = unchanged); the three collections print as
/// `name{[removed];[modified];[added]}` sections. Bytes/strings are lowercase hex (this artifact's
/// own `ArtifactDsl` impl above already uses hex for the same reason: no external base64 dep, no
/// escaping needed). `Option<T>` values (both real optional snapshot fields AND diff tri-states)
/// use a uniform `[0]`=None / `[1,<T>]`=Some(T) tag. Structs are positional `[f1,f2,...]` tuples.
/// `GifFrameDiff`'s own sparse fields print as single-letter `tag:value` pairs
/// (`L`/`T`/`W`/`H`/`I`/`C`/`X`/`D`/`S`/`P`/`U`/`Q`) inside its own `[...]`.
///
/// Worked example (see `f6-recon-report.md` for the literal printed strings captured from a real
/// test run): `width=10 frames{[0];[1:[S:b]];[2:[0,0,2,2,0,[0],0a0b,10,u,[0],0,[0]]]}`.
//#region 🔖️Primitives
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
fn parse_u8(s: &str) -> Result<u8, String> { s.parse().map_err(|e: std::num::ParseIntError| e.to_string()) }
fn parse_u16(s: &str) -> Result<u16, String> { s.parse().map_err(|e: std::num::ParseIntError| e.to_string()) }
fn parse_u32(s: &str) -> Result<u32, String> { s.parse().map_err(|e: std::num::ParseIntError| e.to_string()) }
fn parse_usize(s: &str) -> Result<usize, String> { s.parse().map_err(|e: std::num::ParseIntError| e.to_string()) }

/// 🧭️ Bracket-depth-aware split (tracks `[`/`]` only): a top-level `sep` inside nested brackets is
/// never mistaken for a field separator — the whole hand-rolled grammar's parsing primitive.
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
fn strip_brackets(s: &str) -> Result<&str, String> {
    s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))
}
fn encode_option<T>(opt: &Option<T>, enc: impl Fn(&T) -> String) -> String {
    match opt {
        None => "[0]".to_string(),
        Some(v) => format!("[1,{}]", enc(v)),
    }
}
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
fn enc_rgb(c: &GifRgb) -> String {
    format!("[{},{},{}]", c.r, c.g, c.b)
}
fn dec_rgb(s: &str) -> Result<GifRgb, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [r, g, b] = parts.as_slice() else { return Err(format!("rgb: expected 3 fields, got {}", parts.len())) };
    Ok(GifRgb { r: parse_u8(r)?, g: parse_u8(g)?, b: parse_u8(b)? })
}
fn enc_color_table(t: &GifColorTable) -> String {
    let colors = t.colors.iter().map(enc_rgb).collect::<Vec<_>>().join(",");
    format!("[{},[{}]]", if t.sorted { 1 } else { 0 }, colors)
}
fn dec_color_table(s: &str) -> Result<GifColorTable, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [sorted, colors] = parts.as_slice() else { return Err(format!("color table: expected 2 fields, got {}", parts.len())) };
    let colors = split_top_level(strip_brackets(colors)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_rgb).collect::<Result<Vec<_>, String>>()?;
    Ok(GifColorTable { sorted: *sorted == "1", colors })
}
fn enc_plain_text(p: &GifPlainText) -> String {
    format!(
        "[{},{},{},{},{},{},{},{},{}]",
        p.left, p.top, p.width, p.height, p.cell_width, p.cell_height, p.fg_color_index, p.bg_color_index, hex_encode(p.text.as_bytes())
    )
}
fn dec_plain_text(s: &str) -> Result<GifPlainText, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [left, top, width, height, cw, ch, fg, bg, text] = parts.as_slice() else {
        return Err(format!("plain text: expected 9 fields, got {}", parts.len()));
    };
    Ok(GifPlainText {
        left: parse_u32(left)?, top: parse_u32(top)?, width: parse_u32(width)?, height: parse_u32(height)?,
        cell_width: parse_u8(cw)?, cell_height: parse_u8(ch)?, fg_color_index: parse_u8(fg)?, bg_color_index: parse_u8(bg)?,
        text: String::from_utf8(hex_decode(text)?).map_err(|e| e.to_string())?,
    })
}
fn enc_disposal(d: GifDisposal) -> char {
    match d {
        GifDisposal::Unspecified => 'u',
        GifDisposal::DoNotDispose => 'd',
        GifDisposal::RestoreToBackground => 'b',
        GifDisposal::RestoreToPrevious => 'p',
    }
}
fn dec_disposal(s: &str) -> Result<GifDisposal, String> {
    match s {
        "u" => Ok(GifDisposal::Unspecified),
        "d" => Ok(GifDisposal::DoNotDispose),
        "b" => Ok(GifDisposal::RestoreToBackground),
        "p" => Ok(GifDisposal::RestoreToPrevious),
        other => Err(format!("bad disposal {other:?}")),
    }
}
fn enc_frame(f: &GifFrame) -> String {
    format!(
        "[{},{},{},{},{},{},{},{},{},{},{},{}]",
        f.left, f.top, f.width, f.height, if f.interlace { 1 } else { 0 },
        encode_option(&f.lct, enc_color_table), hex_encode(&f.indices), f.delay_cs, enc_disposal(f.disposal),
        encode_option(&f.transparent_index, |v| v.to_string()), if f.user_input { 1 } else { 0 },
        encode_option(&f.plain_text, enc_plain_text),
    )
}
fn dec_frame(s: &str) -> Result<GifFrame, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [left, top, width, height, interlace, lct, indices, delay_cs, disposal, transparent_index, user_input, plain_text] = parts.as_slice() else {
        return Err(format!("frame: expected 12 fields, got {}", parts.len()));
    };
    Ok(GifFrame {
        left: parse_u32(left)?, top: parse_u32(top)?, width: parse_u32(width)?, height: parse_u32(height)?,
        interlace: *interlace == "1",
        lct: decode_option(lct, dec_color_table)?,
        indices: hex_decode(indices)?,
        delay_cs: parse_u16(delay_cs)?,
        disposal: dec_disposal(disposal)?,
        transparent_index: decode_option(transparent_index, parse_u8)?,
        user_input: *user_input == "1",
        plain_text: decode_option(plain_text, dec_plain_text)?,
    })
}
fn enc_app_extension(e: &GifAppExtension) -> String {
    format!("[{},{},{}]", hex_encode(&e.identifier), hex_encode(&e.auth_code), hex_encode(&e.data))
}
fn dec_app_extension(s: &str) -> Result<GifAppExtension, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id_hex, auth_hex, data_hex] = parts.as_slice() else { return Err(format!("app extension: expected 3 fields, got {}", parts.len())) };
    let identifier: [u8; 8] = hex_decode(id_hex)?.try_into().map_err(|_| "app extension: identifier must be 8 bytes".to_string())?;
    let auth_code: [u8; 3] = hex_decode(auth_hex)?.try_into().map_err(|_| "app extension: auth_code must be 3 bytes".to_string())?;
    Ok(GifAppExtension { identifier, auth_code, data: hex_decode(data_hex)? })
}
//#endregion 🔖️ValueCodecs

//#region 🔖️DiffValueCodecs
fn enc_frame_diff(d: &GifFrameDiff) -> String {
    let mut parts = Vec::new();
    if let Some(v) = d.left { parts.push(format!("L:{v}")); }
    if let Some(v) = d.top { parts.push(format!("T:{v}")); }
    if let Some(v) = d.width { parts.push(format!("W:{v}")); }
    if let Some(v) = d.height { parts.push(format!("H:{v}")); }
    if let Some(v) = d.interlace { parts.push(format!("I:{}", if v { 1 } else { 0 })); }
    if let Some(v) = &d.lct { parts.push(format!("C:{}", encode_option(v, enc_color_table))); }
    if let Some(v) = &d.indices { parts.push(format!("X:{}", hex_encode(v))); }
    if let Some(v) = d.delay_cs { parts.push(format!("D:{v}")); }
    if let Some(v) = d.disposal { parts.push(format!("S:{}", enc_disposal(v))); }
    if let Some(v) = d.transparent_index { parts.push(format!("P:{}", encode_option(&v, |x| x.to_string()))); }
    if let Some(v) = d.user_input { parts.push(format!("U:{}", if v { 1 } else { 0 })); }
    if let Some(v) = &d.plain_text { parts.push(format!("Q:{}", encode_option(v, enc_plain_text))); }
    format!("[{}]", parts.join(","))
}
fn dec_frame_diff(s: &str) -> Result<GifFrameDiff, String> {
    let inner = strip_brackets(s)?;
    let mut d = GifFrameDiff::default();
    for entry in split_top_level(inner, ',') {
        if entry.is_empty() { continue; }
        let (tag, val) = entry.split_once(':').ok_or_else(|| format!("frame diff: bad entry {entry:?}"))?;
        match tag {
            "L" => d.left = Some(parse_u32(val)?),
            "T" => d.top = Some(parse_u32(val)?),
            "W" => d.width = Some(parse_u32(val)?),
            "H" => d.height = Some(parse_u32(val)?),
            "I" => d.interlace = Some(val == "1"),
            "C" => d.lct = Some(decode_option(val, dec_color_table)?),
            "X" => d.indices = Some(hex_decode(val)?),
            "D" => d.delay_cs = Some(parse_u16(val)?),
            "S" => d.disposal = Some(dec_disposal(val)?),
            "P" => d.transparent_index = Some(decode_option(val, parse_u8)?),
            "U" => d.user_input = Some(val == "1"),
            "Q" => d.plain_text = Some(decode_option(val, dec_plain_text)?),
            other => return Err(format!("frame diff: unknown tag {other:?}")),
        }
    }
    Ok(d)
}

/// 🧭️ Generic-shaped 3-section `[removed];[modified];[added]` collection-triple printer/parser,
/// hand-instantiated per weak/strong item type (mirrors `absorb_indexed_collection`'s genericity
/// above, but for text rendering instead of algebra).
fn enc_collection_triple(name: &str, removed: &[usize], modified: &[(usize, String)], added: &[(usize, String)]) -> String {
    let removed = removed.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let modified = modified.iter().map(|(i, v)| format!("{i}:{v}")).collect::<Vec<_>>().join(",");
    let added = added.iter().map(|(i, v)| format!("{i}:{v}")).collect::<Vec<_>>().join(",");
    format!("{name}{{[{removed}];[{modified}];[{added}]}}")
}
fn dec_collection_triple(body: &str) -> Result<(Vec<usize>, Vec<(usize, String)>, Vec<(usize, String)>), String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("collection: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_usize).collect::<Result<Vec<_>, String>>()?;
    let parse_entries = |s: &str| -> Result<Vec<(usize, String)>, String> {
        split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("collection entry: bad entry {entry:?}"))?;
            Ok((parse_usize(idx)?, rest.to_string()))
        }).collect()
    };
    Ok((removed, parse_entries(modified_s)?, parse_entries(added_s)?))
}

fn enc_frames_diff(d: &GifFramesDiff) -> String {
    enc_collection_triple(
        "frames",
        &d.removed,
        &d.modified.iter().map(|m| (m.index, enc_frame_diff(&m.diff))).collect::<Vec<_>>(),
        &d.added.iter().map(|a| (a.index, enc_frame(&a.frame))).collect::<Vec<_>>(),
    )
}
fn dec_frames_diff(body: &str) -> Result<GifFramesDiff, String> {
    let (removed, modified, added) = dec_collection_triple(body)?;
    Ok(GifFramesDiff {
        removed,
        modified: modified.into_iter().map(|(index, enc)| Ok(GifFrameModified { index, diff: dec_frame_diff(&enc)? })).collect::<Result<Vec<_>, String>>()?,
        added: added.into_iter().map(|(index, enc)| Ok(GifFrameAdded { index, frame: dec_frame(&enc)? })).collect::<Result<Vec<_>, String>>()?,
    })
}
fn enc_comments_diff(d: &GifCommentsDiff) -> String {
    enc_collection_triple(
        "comments",
        &d.removed,
        &d.modified.iter().map(|m| (m.index, hex_encode(m.text.as_bytes()))).collect::<Vec<_>>(),
        &d.added.iter().map(|a| (a.index, hex_encode(a.text.as_bytes()))).collect::<Vec<_>>(),
    )
}
fn dec_comments_diff(body: &str) -> Result<GifCommentsDiff, String> {
    let (removed, modified, added) = dec_collection_triple(body)?;
    let text_of = |hex: &str| -> Result<String, String> { String::from_utf8(hex_decode(hex)?).map_err(|e| e.to_string()) };
    Ok(GifCommentsDiff {
        removed,
        modified: modified.into_iter().map(|(index, hex)| Ok(GifCommentModified { index, text: text_of(&hex)? })).collect::<Result<Vec<_>, String>>()?,
        added: added.into_iter().map(|(index, hex)| Ok(GifCommentAdded { index, text: text_of(&hex)? })).collect::<Result<Vec<_>, String>>()?,
    })
}
fn enc_app_extensions_diff(d: &GifAppExtensionsDiff) -> String {
    enc_collection_triple(
        "appext",
        &d.removed,
        &d.modified.iter().map(|m| (m.index, enc_app_extension(&m.extension))).collect::<Vec<_>>(),
        &d.added.iter().map(|a| (a.index, enc_app_extension(&a.extension))).collect::<Vec<_>>(),
    )
}
fn dec_app_extensions_diff(body: &str) -> Result<GifAppExtensionsDiff, String> {
    let (removed, modified, added) = dec_collection_triple(body)?;
    Ok(GifAppExtensionsDiff {
        removed,
        modified: modified.into_iter().map(|(index, enc)| Ok(GifAppExtensionModified { index, extension: dec_app_extension(&enc)? })).collect::<Result<Vec<_>, String>>()?,
        added: added.into_iter().map(|(index, enc)| Ok(GifAppExtensionAdded { index, extension: dec_app_extension(&enc)? })).collect::<Result<Vec<_>, String>>()?,
    })
}
//#endregion 🔖️DiffValueCodecs

//#region 🔖️RealBinaryPrimitives
/// 🧪️ P2-FG2: real binary value codecs for `GifDiff` (89a)'s nested types — mirrors the text
/// codecs above field-for-field, using `dsl::ByteWriter`/`dsl::ByteReader` (the same real
/// LEB128-varint/length-prefixed framework primitives png's own upgraded `PngDiff` binary frame
/// uses, `📷️png/…/🔺️diff/🦀️component.rs`'s `RealBinaryPrimitives`/`RealBinaryDiffFrame`
/// regions, and 87a's own sibling upgrade — `dsl`/`store`/`protocol` all alias the same kernel
/// crate root, reachable with no `use` needed beyond the absolute path).
fn write_bin_rgb(w: &mut dsl::ByteWriter, c: &GifRgb) {
    w.write_u8(c.r);
    w.write_u8(c.g);
    w.write_u8(c.b);
}
fn read_bin_rgb(r: &mut dsl::ByteReader) -> Result<GifRgb, dsl::PackError> {
    Ok(GifRgb { r: r.read_u8()?, g: r.read_u8()?, b: r.read_u8()? })
}
fn write_bin_color_table(w: &mut dsl::ByteWriter, t: &GifColorTable) {
    w.write_u8(if t.sorted { 1 } else { 0 });
    write_bin_vec(w, &t.colors, write_bin_rgb);
}
fn read_bin_color_table(r: &mut dsl::ByteReader) -> Result<GifColorTable, dsl::PackError> {
    let sorted = r.read_u8()? != 0;
    let colors = read_bin_vec(r, read_bin_rgb)?;
    Ok(GifColorTable { sorted, colors })
}
fn write_bin_blob(w: &mut dsl::ByteWriter, bytes: &[u8]) {
    w.write_varint_u64(bytes.len() as u64);
    w.write_bytes(bytes);
}
fn read_bin_blob(r: &mut dsl::ByteReader) -> Result<Vec<u8>, dsl::PackError> {
    let len = r.read_varint_u64()? as usize;
    Ok(r.read_bytes(len)?.to_vec())
}
fn write_bin_str(w: &mut dsl::ByteWriter, s: &str) {
    write_bin_blob(w, s.as_bytes());
}
fn read_bin_str(r: &mut dsl::ByteReader) -> Result<String, dsl::PackError> {
    let bytes = read_bin_blob(r)?;
    String::from_utf8(bytes).map_err(|e| dsl::PackError::Malformed { what: "gif89a binary utf8 string", offset: 0, detail: e.to_string() })
}
fn write_bin_disposal(w: &mut dsl::ByteWriter, d: GifDisposal) {
    w.write_u8(d.to_bits());
}
fn read_bin_disposal(r: &mut dsl::ByteReader) -> Result<GifDisposal, dsl::PackError> {
    Ok(GifDisposal::from_bits(r.read_u8()?))
}
fn write_bin_plain_text(w: &mut dsl::ByteWriter, p: &GifPlainText) {
    w.write_u32_le(p.left);
    w.write_u32_le(p.top);
    w.write_u32_le(p.width);
    w.write_u32_le(p.height);
    w.write_u8(p.cell_width);
    w.write_u8(p.cell_height);
    w.write_u8(p.fg_color_index);
    w.write_u8(p.bg_color_index);
    write_bin_str(w, &p.text);
}
fn read_bin_plain_text(r: &mut dsl::ByteReader) -> Result<GifPlainText, dsl::PackError> {
    Ok(GifPlainText {
        left: r.read_u32_le()?, top: r.read_u32_le()?, width: r.read_u32_le()?, height: r.read_u32_le()?,
        cell_width: r.read_u8()?, cell_height: r.read_u8()?, fg_color_index: r.read_u8()?, bg_color_index: r.read_u8()?,
        text: read_bin_str(r)?,
    })
}
fn write_bin_frame(w: &mut dsl::ByteWriter, f: &GifFrame) {
    w.write_u32_le(f.left);
    w.write_u32_le(f.top);
    w.write_u32_le(f.width);
    w.write_u32_le(f.height);
    w.write_u8(if f.interlace { 1 } else { 0 });
    write_bin_option(w, &f.lct, write_bin_color_table);
    write_bin_blob(w, &f.indices);
    w.write_u16_le(f.delay_cs);
    write_bin_disposal(w, f.disposal);
    write_bin_option(w, &f.transparent_index, |w, v| w.write_u8(*v));
    w.write_u8(if f.user_input { 1 } else { 0 });
    write_bin_option(w, &f.plain_text, write_bin_plain_text);
}
fn read_bin_frame(r: &mut dsl::ByteReader) -> Result<GifFrame, dsl::PackError> {
    Ok(GifFrame {
        left: r.read_u32_le()?, top: r.read_u32_le()?, width: r.read_u32_le()?, height: r.read_u32_le()?,
        interlace: r.read_u8()? != 0,
        lct: read_bin_option(r, read_bin_color_table)?,
        indices: read_bin_blob(r)?,
        delay_cs: r.read_u16_le()?,
        disposal: read_bin_disposal(r)?,
        transparent_index: read_bin_option(r, |r| r.read_u8())?,
        user_input: r.read_u8()? != 0,
        plain_text: read_bin_option(r, read_bin_plain_text)?,
    })
}
fn write_bin_app_extension(w: &mut dsl::ByteWriter, e: &GifAppExtension) {
    w.write_bytes(&e.identifier);
    w.write_bytes(&e.auth_code);
    write_bin_blob(w, &e.data);
}
fn read_bin_app_extension(r: &mut dsl::ByteReader) -> Result<GifAppExtension, dsl::PackError> {
    let identifier: [u8; 8] = r.read_bytes(8)?.try_into().map_err(|_| dsl::PackError::Malformed { what: "gif89a app extension identifier", offset: 0, detail: "expected 8 bytes".into() })?;
    let auth_code: [u8; 3] = r.read_bytes(3)?.try_into().map_err(|_| dsl::PackError::Malformed { what: "gif89a app extension auth_code", offset: 0, detail: "expected 3 bytes".into() })?;
    Ok(GifAppExtension { identifier, auth_code, data: read_bin_blob(r)? })
}
/// 🧩 2-way presence flag (`0`=None, `1`=Some) — shared by every plain `Option<T>` field.
fn write_bin_option<T>(w: &mut dsl::ByteWriter, v: &Option<T>, write_value: impl FnOnce(&mut dsl::ByteWriter, &T)) {
    match v {
        None => w.write_u8(0),
        Some(val) => { w.write_u8(1); write_value(w, val); }
    }
}
fn read_bin_option<T>(r: &mut dsl::ByteReader, read_value: impl FnOnce(&mut dsl::ByteReader) -> Result<T, dsl::PackError>) -> Result<Option<T>, dsl::PackError> {
    match r.read_u8()? {
        0 => Ok(None),
        1 => Ok(Some(read_value(r)?)),
        other => Err(dsl::PackError::Malformed { what: "gif89a binary option tag", offset: 0, detail: format!("unknown tag {other}") }),
    }
}
fn write_bin_vec<T>(w: &mut dsl::ByteWriter, items: &[T], write_item: impl Fn(&mut dsl::ByteWriter, &T)) {
    w.write_varint_u64(items.len() as u64);
    for item in items {
        write_item(w, item);
    }
}
fn read_bin_vec<T>(r: &mut dsl::ByteReader, mut read_item: impl FnMut(&mut dsl::ByteReader) -> Result<T, dsl::PackError>) -> Result<Vec<T>, dsl::PackError> {
    let n = r.read_varint_u64()? as usize;
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        out.push(read_item(r)?);
    }
    Ok(out)
}
/// 🧩 3-way flag (`0`=unchanged, `1`=cleared-to-`None`, `2`=set-to-`Some(value)`) for every
/// TRI-STATE `Option<Option<T>>` field — same shape as png's own doc comment (this avoids
/// chaining two `if`-guarded conditional fields at the protocol-description level; the Rust
/// codec here has no such limitation but keeps the same 3-way-flag SHAPE for parity).
fn write_bin_tri_flag<T>(w: &mut dsl::ByteWriter, v: &Option<Option<T>>, write_value: impl FnOnce(&mut dsl::ByteWriter, &T)) {
    match v {
        None => w.write_u8(0),
        Some(None) => w.write_u8(1),
        Some(Some(val)) => { w.write_u8(2); write_value(w, val); }
    }
}
fn read_bin_tri_flag<T>(r: &mut dsl::ByteReader, read_value: impl FnOnce(&mut dsl::ByteReader) -> Result<T, dsl::PackError>) -> Result<Option<Option<T>>, dsl::PackError> {
    match r.read_u8()? {
        0 => Ok(None),
        1 => Ok(Some(None)),
        2 => Ok(Some(Some(read_value(r)?))),
        other => Err(dsl::PackError::Malformed { what: "gif89a diff tri-flag", offset: 0, detail: format!("unknown flag {other}") }),
    }
}
fn diff_pack_err(e: dsl::PackError) -> protocol::ProtocolError {
    protocol::ProtocolError::Malformed { what: "gif89a diff binary", offset: 0, detail: e.to_string() }
}
//#endregion 🔖️RealBinaryPrimitives

//#region 🔖️RealBinaryDiffFrame
/// 🧪️ P2-FG2: real binary encodings for the three collection-triple diff types (`frames`,
/// `comments`, `app_extensions`) — each produces one opaque `Vec<u8>` blob matching
/// `../💾️binary/📡️component.protocol.semio`'s `Array(u8, Field(<name>_len))` fields exactly
/// (the blob's OWN internal removed/modified/added shape isn't further protocol-walkable, see
/// that file's own doc comment); the Rust codec here IS genuinely, fully structured (real
/// varint counts, real per-item recursive encoding), never text-as-bytes.
fn write_bin_frame_diff(w: &mut dsl::ByteWriter, d: &GifFrameDiff) {
    write_bin_option(w, &d.left, |w, v| w.write_u32_le(*v));
    write_bin_option(w, &d.top, |w, v| w.write_u32_le(*v));
    write_bin_option(w, &d.width, |w, v| w.write_u32_le(*v));
    write_bin_option(w, &d.height, |w, v| w.write_u32_le(*v));
    write_bin_option(w, &d.interlace, |w, v| w.write_u8(if *v { 1 } else { 0 }));
    write_bin_tri_flag(w, &d.lct, write_bin_color_table);
    write_bin_option(w, &d.indices, |w, v| write_bin_blob(w, v));
    write_bin_option(w, &d.delay_cs, |w, v| w.write_u16_le(*v));
    write_bin_option(w, &d.disposal, |w, v| write_bin_disposal(w, *v));
    write_bin_tri_flag(w, &d.transparent_index, |w, v| w.write_u8(*v));
    write_bin_option(w, &d.user_input, |w, v| w.write_u8(if *v { 1 } else { 0 }));
    write_bin_tri_flag(w, &d.plain_text, write_bin_plain_text);
}
fn read_bin_frame_diff(r: &mut dsl::ByteReader) -> Result<GifFrameDiff, dsl::PackError> {
    Ok(GifFrameDiff {
        left: read_bin_option(r, |r| r.read_u32_le())?,
        top: read_bin_option(r, |r| r.read_u32_le())?,
        width: read_bin_option(r, |r| r.read_u32_le())?,
        height: read_bin_option(r, |r| r.read_u32_le())?,
        interlace: read_bin_option(r, |r| Ok(r.read_u8()? != 0))?,
        lct: read_bin_tri_flag(r, read_bin_color_table)?,
        indices: read_bin_option(r, read_bin_blob)?,
        delay_cs: read_bin_option(r, |r| r.read_u16_le())?,
        disposal: read_bin_option(r, read_bin_disposal)?,
        transparent_index: read_bin_tri_flag(r, |r| r.read_u8())?,
        user_input: read_bin_option(r, |r| Ok(r.read_u8()? != 0))?,
        plain_text: read_bin_tri_flag(r, read_bin_plain_text)?,
    })
}
fn enc_frames_diff_bin(d: &GifFramesDiff) -> Vec<u8> {
    let mut w = dsl::ByteWriter::new();
    write_bin_vec(&mut w, &d.removed, |w, v: &usize| w.write_varint_u64(*v as u64));
    write_bin_vec(&mut w, &d.modified, |w, m: &GifFrameModified| { w.write_varint_u64(m.index as u64); write_bin_frame_diff(w, &m.diff); });
    write_bin_vec(&mut w, &d.added, |w, a: &GifFrameAdded| { w.write_varint_u64(a.index as u64); write_bin_frame(w, &a.frame); });
    w.into_bytes()
}
fn dec_frames_diff_bin(bytes: &[u8]) -> Result<GifFramesDiff, dsl::PackError> {
    let mut r = dsl::ByteReader::new(bytes);
    let removed = read_bin_vec(&mut r, |r| Ok(r.read_varint_u64()? as usize))?;
    let modified = read_bin_vec(&mut r, |r| { let index = r.read_varint_u64()? as usize; let diff = read_bin_frame_diff(r)?; Ok(GifFrameModified { index, diff }) })?;
    let added = read_bin_vec(&mut r, |r| { let index = r.read_varint_u64()? as usize; let frame = read_bin_frame(r)?; Ok(GifFrameAdded { index, frame }) })?;
    Ok(GifFramesDiff { removed, modified, added })
}
fn enc_comments_diff_bin(d: &GifCommentsDiff) -> Vec<u8> {
    let mut w = dsl::ByteWriter::new();
    write_bin_vec(&mut w, &d.removed, |w, v: &usize| w.write_varint_u64(*v as u64));
    write_bin_vec(&mut w, &d.modified, |w, m: &GifCommentModified| { w.write_varint_u64(m.index as u64); write_bin_str(w, &m.text); });
    write_bin_vec(&mut w, &d.added, |w, a: &GifCommentAdded| { w.write_varint_u64(a.index as u64); write_bin_str(w, &a.text); });
    w.into_bytes()
}
fn dec_comments_diff_bin(bytes: &[u8]) -> Result<GifCommentsDiff, dsl::PackError> {
    let mut r = dsl::ByteReader::new(bytes);
    let removed = read_bin_vec(&mut r, |r| Ok(r.read_varint_u64()? as usize))?;
    let modified = read_bin_vec(&mut r, |r| { let index = r.read_varint_u64()? as usize; let text = read_bin_str(r)?; Ok(GifCommentModified { index, text }) })?;
    let added = read_bin_vec(&mut r, |r| { let index = r.read_varint_u64()? as usize; let text = read_bin_str(r)?; Ok(GifCommentAdded { index, text }) })?;
    Ok(GifCommentsDiff { removed, modified, added })
}
fn enc_app_extensions_diff_bin(d: &GifAppExtensionsDiff) -> Vec<u8> {
    let mut w = dsl::ByteWriter::new();
    write_bin_vec(&mut w, &d.removed, |w, v: &usize| w.write_varint_u64(*v as u64));
    write_bin_vec(&mut w, &d.modified, |w, m: &GifAppExtensionModified| { w.write_varint_u64(m.index as u64); write_bin_app_extension(w, &m.extension); });
    write_bin_vec(&mut w, &d.added, |w, a: &GifAppExtensionAdded| { w.write_varint_u64(a.index as u64); write_bin_app_extension(w, &a.extension); });
    w.into_bytes()
}
fn dec_app_extensions_diff_bin(bytes: &[u8]) -> Result<GifAppExtensionsDiff, dsl::PackError> {
    let mut r = dsl::ByteReader::new(bytes);
    let removed = read_bin_vec(&mut r, |r| Ok(r.read_varint_u64()? as usize))?;
    let modified = read_bin_vec(&mut r, |r| { let index = r.read_varint_u64()? as usize; let extension = read_bin_app_extension(r)?; Ok(GifAppExtensionModified { index, extension }) })?;
    let added = read_bin_vec(&mut r, |r| { let index = r.read_varint_u64()? as usize; let extension = read_bin_app_extension(r)?; Ok(GifAppExtensionAdded { index, extension }) })?;
    Ok(GifAppExtensionsDiff { removed, modified, added })
}
//#endregion 🔖️RealBinaryDiffFrame

//#region 🔖️TopLevel
fn print_gif_diff(d: &GifDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = d.width { tokens.push(format!("width={v}")); }
    if let Some(v) = d.height { tokens.push(format!("height={v}")); }
    if let Some(v) = &d.gct { tokens.push(format!("gct={}", encode_option(v, enc_color_table))); }
    if let Some(v) = d.background_color_index { tokens.push(format!("bg={v}")); }
    if let Some(v) = d.pixel_aspect_ratio { tokens.push(format!("par={v}")); }
    if let Some(v) = d.loop_count { tokens.push(format!("loop={}", encode_option(&v, |x| x.to_string()))); }
    if let Some(v) = &d.frames { tokens.push(enc_frames_diff(v)); }
    if let Some(v) = &d.comments { tokens.push(enc_comments_diff(v)); }
    if let Some(v) = &d.app_extensions { tokens.push(enc_app_extensions_diff(v)); }
    tokens.join(" ")
}
fn parse_gif_diff(line: &str) -> Result<GifDiff, String> {
    let mut d = GifDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("width=") { d.width = Some(parse_u32(rest)?); }
        else if let Some(rest) = token.strip_prefix("height=") { d.height = Some(parse_u32(rest)?); }
        else if let Some(rest) = token.strip_prefix("gct=") { d.gct = Some(decode_option(rest, dec_color_table)?); }
        else if let Some(rest) = token.strip_prefix("bg=") { d.background_color_index = Some(parse_u8(rest)?); }
        else if let Some(rest) = token.strip_prefix("par=") { d.pixel_aspect_ratio = Some(parse_u8(rest)?); }
        else if let Some(rest) = token.strip_prefix("loop=") { d.loop_count = Some(decode_option(rest, parse_u16)?); }
        else if let Some(rest) = token.strip_prefix("frames{") { d.frames = Some(dec_frames_diff(rest.strip_suffix('}').ok_or_else(|| "frames: missing closing brace".to_string())?)?); }
        else if let Some(rest) = token.strip_prefix("comments{") { d.comments = Some(dec_comments_diff(rest.strip_suffix('}').ok_or_else(|| "comments: missing closing brace".to_string())?)?); }
        else if let Some(rest) = token.strip_prefix("appext{") { d.app_extensions = Some(dec_app_extensions_diff(rest.strip_suffix('}').ok_or_else(|| "appext: missing closing brace".to_string())?)?); }
        else { return Err(format!("gif diff: unknown token {token:?}")); }
    }
    Ok(d)
}

impl protocol::DiffCodec for GifDiff {
    fn print_diff(&self) -> String {
        print_gif_diff(self)
    }
    fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        parse_gif_diff(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    /// ⚡️ P2-FG2: real binary diff-frame — upgraded from the F6-era `print_diff().into_bytes()`
    /// text-as-binary shortcut (100% of stdio's `DiffCodec` impls were still on that shortcut
    /// per the P2-W0 census; the FG1 wave's own closer report flagged leaving this un-upgraded
    /// as a real defect to not repeat). Matches `../💾️binary/📡️component.protocol.semio`'s real
    /// flag-per-field layout exactly, field for field, in struct order (2-way flag for plain
    /// `Option<T>` fields, 3-way flag for the tri-state `gct`/`loop_count` fields).
    fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let mut w = dsl::ByteWriter::new();
        write_bin_option(&mut w, &self.width, |w, v| w.write_u32_le(*v));
        write_bin_option(&mut w, &self.height, |w, v| w.write_u32_le(*v));
        write_bin_tri_flag(&mut w, &self.gct, |w, v| {
            let mut inner = dsl::ByteWriter::new();
            write_bin_color_table(&mut inner, v);
            write_bin_blob(w, &inner.into_bytes());
        });
        write_bin_option(&mut w, &self.background_color_index, |w, v| w.write_u8(*v));
        write_bin_option(&mut w, &self.pixel_aspect_ratio, |w, v| w.write_u8(*v));
        write_bin_tri_flag(&mut w, &self.loop_count, |w, v| w.write_u16_le(*v));
        write_bin_option(&mut w, &self.frames, |w, v| write_bin_blob(w, &enc_frames_diff_bin(v)));
        write_bin_option(&mut w, &self.comments, |w, v| write_bin_blob(w, &enc_comments_diff_bin(v)));
        write_bin_option(&mut w, &self.app_extensions, |w, v| write_bin_blob(w, &enc_app_extensions_diff_bin(v)));
        Ok(w.into_bytes())
    }
    fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut r = dsl::ByteReader::new(bytes);
        let width = read_bin_option(&mut r, |r| r.read_u32_le()).map_err(diff_pack_err)?;
        let height = read_bin_option(&mut r, |r| r.read_u32_le()).map_err(diff_pack_err)?;
        let gct = read_bin_tri_flag(&mut r, |r| {
            let blob = read_bin_blob(r)?;
            let mut inner = dsl::ByteReader::new(&blob);
            read_bin_color_table(&mut inner)
        }).map_err(diff_pack_err)?;
        let background_color_index = read_bin_option(&mut r, |r| r.read_u8()).map_err(diff_pack_err)?;
        let pixel_aspect_ratio = read_bin_option(&mut r, |r| r.read_u8()).map_err(diff_pack_err)?;
        let loop_count = read_bin_tri_flag(&mut r, |r| r.read_u16_le()).map_err(diff_pack_err)?;
        let frames = read_bin_option(&mut r, |r| dec_frames_diff_bin(&read_bin_blob(r)?)).map_err(diff_pack_err)?;
        let comments = read_bin_option(&mut r, |r| dec_comments_diff_bin(&read_bin_blob(r)?)).map_err(diff_pack_err)?;
        let app_extensions = read_bin_option(&mut r, |r| dec_app_extensions_diff_bin(&read_bin_blob(r)?)).map_err(diff_pack_err)?;
        Ok(GifDiff { width, height, gct, background_color_index, pixel_aspect_ratio, loop_count, frames, comments, app_extensions })
    }
}
//#endregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec

//#region Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::gif::standards::v89a::subsets::any::schema::snapshot::{GifRgb, STDIO_GIF89A_DOCUMENT_SCHEMA};

    fn frame(seed: u8, w: u32, h: u32) -> GifFrame {
        GifFrame {
            left: 0, top: 0, width: w, height: h,
            interlace: false,
            lct: Some(GifColorTable { sorted: false, colors: vec![GifRgb { r: seed, g: seed, b: seed }; 2] }),
            indices: vec![0u8; (w * h) as usize],
            delay_cs: 10,
            disposal: GifDisposal::DoNotDispose,
            transparent_index: None,
            user_input: false,
            plain_text: None,
        }
    }

    /// 🧪️ Canonical absorb case 1: `InsertFrame(2,f)` then `RemoveFrame(0)` →
    /// `{removed:[0], added:[(1,f)]}`.
    #[test]
    fn absorb_insert_then_remove_before_shifts_index() {
        let f = frame(9, 2, 2);
        let mut d1 = GifFramesDiff { added: vec![GifFrameAdded { index: 2, frame: f.clone() }], ..Default::default() };
        let d2 = GifFramesDiff { removed: vec![0], ..Default::default() };
        d1.absorb(d2);
        assert_eq!(d1.removed, vec![0]);
        assert_eq!(d1.added, vec![GifFrameAdded { index: 1, frame: f }]);
        assert!(d1.modified.is_empty());
    }

    /// 🧪️ Canonical absorb case 2: `InsertFrame(2,f)` then `InsertFrame(2,g)` → BOTH survive as
    /// `added:[(2,g),(3,f)]` — the exact LWW-slot bug this recipe replaces.
    #[test]
    fn absorb_insert_insert_same_index_both_survive() {
        let f = frame(1, 2, 2);
        let g = frame(2, 2, 2);
        let mut d1 = GifFramesDiff { added: vec![GifFrameAdded { index: 2, frame: f.clone() }], ..Default::default() };
        let d2 = GifFramesDiff { added: vec![GifFrameAdded { index: 2, frame: g.clone() }], ..Default::default() };
        d1.absorb(d2);
        assert_eq!(d1.added, vec![
            GifFrameAdded { index: 2, frame: g },
            GifFrameAdded { index: 3, frame: f },
        ]);
    }

    /// 🧪️ Canonical absorb case 3: `InsertFrame(1,f)` then `SetFrameDelay(1,42)` patches INTO the
    /// added payload — merged has only `added`, no separate `modified` entry.
    #[test]
    fn absorb_insert_then_set_field_patches_into_added() {
        let f = frame(1, 2, 2);
        let mut d1 = GifFramesDiff { added: vec![GifFrameAdded { index: 1, frame: f.clone() }], ..Default::default() };
        let d2 = GifFramesDiff {
            modified: vec![GifFrameModified { index: 1, diff: GifFrameDiff { delay_cs: Some(42), ..Default::default() } }],
            ..Default::default()
        };
        d1.absorb(d2);
        assert!(d1.modified.is_empty());
        assert_eq!(d1.added.len(), 1);
        assert_eq!(d1.added[0].frame.delay_cs, 42);
        assert_eq!(d1.added[0].index, 1);
    }

    #[test]
    fn absorb_law_holds_over_curated_ops() {
        let base = GifSnapshot { frames: vec![frame(1, 2, 2), frame(2, 2, 2), frame(3, 2, 2)], loop_count: Some(0), ..GifSnapshot::default() };
        let mid = {
            let mut s = base.clone();
            s.frames.insert(1, frame(9, 2, 2));
            s.frames.remove(0);
            s.comments.push("hello".into());
            s
        };
        let after = {
            let mut s = mid.clone();
            s.frames[0].disposal = GifDisposal::RestoreToBackground;
            s.frames.push(frame(5, 2, 2));
            s.app_extensions.push(GifAppExtension { identifier: *b"XMP Data", auth_code: *b"XMP", data: vec![1, 2, 3] });
            s
        };
        let mut d1 = <GifDiff as DiffAlgebra<GifSnapshot>>::between(&base, &mid);
        let d2 = <GifDiff as DiffAlgebra<GifSnapshot>>::between(&mid, &after);
        d1.absorb(d2);
        assert_eq!(d1.apply(&base), after);
    }

    #[test]
    fn between_roundtrip_law() {
        let a = GifSnapshot { width: 4, height: 4, frames: vec![frame(1, 4, 4)], ..GifSnapshot::default() };
        let b = GifSnapshot { width: 4, height: 4, frames: vec![frame(1, 4, 4), frame(2, 2, 2)], loop_count: Some(0), ..GifSnapshot::default() };
        let ab = <GifDiff as DiffAlgebra<GifSnapshot>>::between(&a, &b);
        assert_eq!(ab.apply(&a), b);
        let ba = <GifDiff as DiffAlgebra<GifSnapshot>>::between(&b, &a);
        assert_eq!(ba.apply(&b), a);
        assert!(<GifDiff as DiffAlgebra<GifSnapshot>>::between(&a, &a).is_empty());
    }

    #[test]
    fn inverse_law() {
        let base = GifSnapshot { frames: vec![frame(1, 2, 2), frame(2, 2, 2)], loop_count: Some(0), ..GifSnapshot::default() };
        let next = {
            let mut s = base.clone();
            s.frames[0].disposal = GifDisposal::RestoreToPrevious;
            s.frames.remove(1);
            s.frames.push(frame(7, 3, 3));
            s.background_color_index = 5;
            s.loop_count = None;
            s.comments.push("hi".into());
            s
        };
        let d = <GifDiff as DiffAlgebra<GifSnapshot>>::between(&base, &next);
        let mutated = d.apply(&base);
        let inv = d.inverse(&base);
        assert_eq!(inv.apply(&mutated), base);
    }

    /// 🧪️ Field sweep — the acceptance criterion: `sweep_a`/`sweep_b` differ in EVERY mutable
    /// field, incl. every tri-state exercising `Some(None)`, with asymmetric collection lengths
    /// (F1's structural trap: a single index-keyed `between()` call can show `removed` XOR
    /// `added`, never both — split across both directions, per `f1-closer-report.md` §4.4).
    #[test]
    fn field_sweep_covers_every_mutable_field() {
        let mut fa = frame(1, 2, 2);
        fa.transparent_index = Some(0);
        let sweep_a = GifSnapshot {
            schema: STDIO_GIF89A_DOCUMENT_SCHEMA.into(),
            width: 10, height: 8,
            gct: Some(GifColorTable { sorted: false, colors: vec![GifRgb { r: 1, g: 2, b: 3 }; 2] }),
            background_color_index: 0,
            pixel_aspect_ratio: 0,
            loop_count: None,
            frames: vec![fa, frame(2, 2, 2)],
            comments: vec!["first".into()],
            app_extensions: vec![GifAppExtension { identifier: *b"NETSCAPX", auth_code: *b"2.0", data: vec![9] }],
        };
        let mut fb0 = frame(1, 2, 2);
        fb0.disposal = GifDisposal::RestoreToPrevious;
        fb0.interlace = true;
        fb0.transparent_index = None;
        fb0.plain_text = Some(GifPlainText { left: 1, top: 1, width: 2, height: 2, cell_width: 4, cell_height: 4, fg_color_index: 0, bg_color_index: 1, text: "hi".into() });
        let sweep_b = GifSnapshot {
            schema: STDIO_GIF89A_DOCUMENT_SCHEMA.into(),
            width: 20, height: 16,
            gct: None,
            background_color_index: 3,
            pixel_aspect_ratio: 7,
            loop_count: Some(5),
            frames: vec![fb0, frame(6, 3, 3), frame(7, 3, 3)],
            comments: vec![],
            app_extensions: vec![],
        };

        let ab = <GifDiff as DiffAlgebra<GifSnapshot>>::between(&sweep_a, &sweep_b);
        assert_eq!(ab.apply(&sweep_a), sweep_b);
        assert!(ab.width.is_some());
        assert!(ab.height.is_some());
        assert_eq!(ab.gct, Some(None), "gct going Some->None must be tri-state Some(None)");
        assert!(ab.background_color_index.is_some());
        assert!(ab.pixel_aspect_ratio.is_some());
        assert_eq!(ab.loop_count, Some(Some(5)));
        let frames_ab = ab.frames.as_ref().expect("frames must differ");
        assert!(!frames_ab.modified.is_empty(), "sweep must exercise a modified frame");
        assert!(!frames_ab.added.is_empty(), "sweep must exercise an added frame (b is longer)");
        let modified_diff = &frames_ab.modified[0].diff;
        assert!(modified_diff.disposal.is_some());
        assert!(modified_diff.interlace.is_some());
        assert_eq!(modified_diff.transparent_index, Some(None), "transparent_index Some->None must be tri-state Some(None)");
        assert!(modified_diff.plain_text.is_some(), "plain_text None->Some must be captured");
        let comments_ab = ab.comments.as_ref().expect("comments must differ");
        assert!(!comments_ab.removed.is_empty(), "sweep must exercise a removed comment (b has none)");
        let app_ext_ab = ab.app_extensions.as_ref().expect("app_extensions must differ");
        assert!(!app_ext_ab.removed.is_empty(), "sweep must exercise a removed app extension (b has none)");

        let ba = <GifDiff as DiffAlgebra<GifSnapshot>>::between(&sweep_b, &sweep_a);
        assert_eq!(ba.apply(&sweep_b), sweep_a);
        let frames_ba = ba.frames.as_ref().expect("frames must differ");
        assert!(!frames_ba.removed.is_empty(), "reverse direction must exercise a removed frame (a is shorter)");
        let comments_ba = ba.comments.as_ref().expect("comments must differ");
        assert!(!comments_ba.added.is_empty(), "reverse direction must exercise an added comment");
        let app_ext_ba = ba.app_extensions.as_ref().expect("app_extensions must differ");
        assert!(!app_ext_ba.added.is_empty(), "reverse direction must exercise an added app extension");

        assert!(<GifDiff as DiffAlgebra<GifSnapshot>>::between(&sweep_a, &sweep_a).is_empty());
    }

    /// 🧪️ F6-PILOT: `DiffCodec` round-trip laws for the hand-rolled `GifDiff` text/binary grammar
    /// — exercises scalars, both tri-states (`gct`/`loop_count` at the top level, `lct`/
    /// `transparent_index`/`plain_text` inside a modified frame), and all three collection triples
    /// (`removed`/`modified`/`added`) simultaneously via a real `between()` result.
    #[test]
    fn diff_codec_text_binary_roundtrip_law() {
        let mut fa = frame(1, 2, 2);
        fa.transparent_index = Some(0);
        let a = GifSnapshot {
            width: 10, height: 8,
            gct: Some(GifColorTable { sorted: false, colors: vec![GifRgb { r: 1, g: 2, b: 3 }; 2] }),
            loop_count: None,
            frames: vec![fa, frame(2, 2, 2)],
            comments: vec!["first".into()],
            app_extensions: vec![GifAppExtension { identifier: *b"NETSCAPX", auth_code: *b"2.0", data: vec![9] }],
            ..GifSnapshot::default()
        };
        let mut fb0 = frame(1, 2, 2);
        fb0.disposal = GifDisposal::RestoreToPrevious;
        fb0.transparent_index = None;
        fb0.plain_text = Some(GifPlainText { left: 1, top: 1, width: 2, height: 2, cell_width: 4, cell_height: 4, fg_color_index: 0, bg_color_index: 1, text: "hi".into() });
        let b = GifSnapshot {
            width: 20, height: 16,
            gct: None,
            loop_count: Some(5),
            frames: vec![fb0, frame(6, 3, 3), frame(7, 3, 3)],
            comments: vec![],
            app_extensions: vec![],
            ..GifSnapshot::default()
        };
        let cases = vec![
            GifDiff::default(),
            <GifDiff as DiffAlgebra<GifSnapshot>>::between(&a, &b),
            <GifDiff as DiffAlgebra<GifSnapshot>>::between(&b, &a),
        ];
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
