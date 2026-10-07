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

/// 🧩 Ordered removed keys, modified values, and inserted items.
pub(crate) type IndexedDiffParts<D, T> = (Vec<usize>, Vec<(usize, D)>, Vec<(usize, T)>);

use crate::standards::v89a::subsets::any::schema::snapshot::{GifAppExtension, GifColorTable, GifDisposal, GifFrame, GifPlainText, GifRgb, GifSnapshot};
use framework_schema::ArtifactSchema;
use protocol::os_spr::command::DiffAlgebra;
use protocol::{DiffCodec};
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};

//#region 🔖️IndexTransport
/// 📐️ Shared rank/unrank arithmetic for index-keyed collection diffs (`between`/`absorb`/
/// `inverse`) — see `🧬️schema-design.md` §Absorb and the top-level plan's "Absorb" section for the
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

/// ↩️ Diff-level inverse for an index-keyed collection triple, given the ORIGINAL base items.
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

//#region 🔖️FrameDiff
/// 🔺️ Sparse per-field diff for one [`GifFrame`] — a strong entity, per the recipe.
/// 🧪️ F6-PILOT FINDING: `#[derive(dsl::DslRecord)]`/`#[derive(dsl::)]` CANNOT be used on
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
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GifFrameDiff {
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
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub delay_cs: Option<u16>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub disposal: Option<GifDisposal>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub transparent_index: Option<Option<u8>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub user_input: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub plain_text: Option<Option<GifPlainText>>,
}

impl GifFrameDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.left.is_none()
            && self.top.is_none()
            && self.width.is_none()
            && self.height.is_none()
            && self.interlace.is_none()
            && self.lct.is_none()
            && self.indices.is_none()
            && self.delay_cs.is_none()
            && self.disposal.is_none()
            && self.transparent_index.is_none()
            && self.user_input.is_none()
            && self.plain_text.is_none()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn apply(&self, base: &GifFrame) -> GifFrame {
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
        if let Some(v) = self.delay_cs {
            next.delay_cs = v;
        }
        if let Some(v) = self.disposal {
            next.disposal = v;
        }
        if let Some(v) = self.transparent_index {
            next.transparent_index = v;
        }
        if let Some(v) = self.user_input {
            next.user_input = v;
        }
        if let Some(v) = &self.plain_text {
            next.plain_text = v.clone();
        }
        next
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
        if other.delay_cs.is_some() {
            self.delay_cs = other.delay_cs;
        }
        if other.disposal.is_some() {
            self.disposal = other.disposal;
        }
        if other.transparent_index.is_some() {
            self.transparent_index = other.transparent_index;
        }
        if other.user_input.is_some() {
            self.user_input = other.user_input;
        }
        if other.plain_text.is_some() {
            self.plain_text = other.plain_text;
        }
    }
}
//#endregion 🔖️FrameDiff

//#region 🔖️FramesDiff
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GifFrameModified {
    pub index: usize,
    pub diff: GifFrameDiff,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GifFrameAdded {
    pub index: usize,
    pub frame: GifFrame,
}

/// 🔺️ Index-keyed collection triple for `GifSnapshot::frames`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GifFramesDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<GifFrameModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<GifFrameAdded>,
}

impl GifFramesDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
        let added: Vec<GifFrameAdded> = (min..other.len()).map(|i| GifFrameAdded { index: i, frame: other[i].clone() }).collect();
        Self { removed, modified, added }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn apply(&self, base: &[GifFrame]) -> Vec<GifFrame> {
        let mut next: Vec<Option<GifFrame>> = base.iter().cloned().map(Some).collect();
        for m in &self.modified {
            if let Some(Some(item)) = next.get_mut(m.index) {
                *item = m.diff.apply(item);
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
        let mut out: Vec<GifFrame> = next.into_iter().flatten().collect();
        let mut added_sorted = self.added.clone();
        added_sorted.sort_by_key(|a| a.index);
        for a in added_sorted {
            let at = a.index.min(out.len());
            out.insert(at, a.frame);
        }
        out
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn inverse(&self, base_frames: &[GifFrame]) -> Self {
        let (removed, modified, added) =
            inverse_indexed_collection(&self.removed, &self.modified.iter().map(|m| (m.index, m.diff.clone())).collect::<Vec<_>>(), &self.added.iter().map(|a| (a.index, a.frame.clone())).collect::<Vec<_>>(), base_frames, |d, item| d.inverse(item));
        Self { removed, modified: modified.into_iter().map(|(index, diff)| GifFrameModified { index, diff }).collect(), added: added.into_iter().map(|(index, frame)| GifFrameAdded { index, frame }).collect() }
    }
}
//#endregion 🔖️FramesDiff

//#region 🔖️WeakCollectionDiffs
/// 🧩️ Macro-free, hand-duplicated (small, two instantiations) index-keyed collection triple for a
/// WEAK/value collection item — the "diff" IS the whole new value, per the recipe's strong/weak
/// split (no further sub-diffing of a `String` or a `GifAppExtension`).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GifCommentModified {
    pub index: usize,
    pub text: String,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GifCommentAdded {
    pub index: usize,
    pub text: String,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GifCommentsDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<GifCommentModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<GifCommentAdded>,
}

impl GifCommentsDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn between(base: &[String], other: &[String]) -> Self {
        let min = base.len().min(other.len());
        let modified = (0..min).filter(|&i| base[i] != other[i]).map(|i| GifCommentModified { index: i, text: other[i].clone() }).collect();
        let removed: Vec<usize> = (min..base.len()).collect();
        let added: Vec<GifCommentAdded> = (min..other.len()).map(|i| GifCommentAdded { index: i, text: other[i].clone() }).collect();
        Self { removed, modified, added }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn apply(&self, base: &[String]) -> Vec<String> {
        let mut next: Vec<Option<String>> = base.iter().cloned().map(Some).collect();
        for m in &self.modified {
            if let Some(slot) = next.get_mut(m.index) {
                *slot = Some(m.text.clone());
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
        let mut out: Vec<String> = next.into_iter().flatten().collect();
        let mut added_sorted = self.added.clone();
        added_sorted.sort_by_key(|a| a.index);
        for a in added_sorted {
            let at = a.index.min(out.len());
            out.insert(at, a.text);
        }
        out
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn inverse(&self, base_comments: &[String]) -> Self {
        let (removed, modified, added) =
            inverse_indexed_collection(&self.removed, &self.modified.iter().map(|m| (m.index, m.text.clone())).collect::<Vec<_>>(), &self.added.iter().map(|a| (a.index, a.text.clone())).collect::<Vec<_>>(), base_comments, |_d, item| item.clone());
        Self { removed, modified: modified.into_iter().map(|(index, text)| GifCommentModified { index, text }).collect(), added: added.into_iter().map(|(index, text)| GifCommentAdded { index, text }).collect() }
    }
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GifAppExtensionModified {
    pub index: usize,
    pub extension: GifAppExtension,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GifAppExtensionAdded {
    pub index: usize,
    pub extension: GifAppExtension,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GifAppExtensionsDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<GifAppExtensionModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<GifAppExtensionAdded>,
}

impl GifAppExtensionsDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn between(base: &[GifAppExtension], other: &[GifAppExtension]) -> Self {
        let min = base.len().min(other.len());
        let modified = (0..min).filter(|&i| base[i] != other[i]).map(|i| GifAppExtensionModified { index: i, extension: other[i].clone() }).collect();
        let removed: Vec<usize> = (min..base.len()).collect();
        let added: Vec<GifAppExtensionAdded> = (min..other.len()).map(|i| GifAppExtensionAdded { index: i, extension: other[i].clone() }).collect();
        Self { removed, modified, added }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn apply(&self, base: &[GifAppExtension]) -> Vec<GifAppExtension> {
        let mut next: Vec<Option<GifAppExtension>> = base.iter().cloned().map(Some).collect();
        for m in &self.modified {
            if let Some(slot) = next.get_mut(m.index) {
                *slot = Some(m.extension.clone());
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
        let mut out: Vec<GifAppExtension> = next.into_iter().flatten().collect();
        let mut added_sorted = self.added.clone();
        added_sorted.sort_by_key(|a| a.index);
        for a in added_sorted {
            let at = a.index.min(out.len());
            out.insert(at, a.extension);
        }
        out
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn inverse(&self, base_exts: &[GifAppExtension]) -> Self {
        let (removed, modified, added) =
            inverse_indexed_collection(&self.removed, &self.modified.iter().map(|m| (m.index, m.extension.clone())).collect::<Vec<_>>(), &self.added.iter().map(|a| (a.index, a.extension.clone())).collect::<Vec<_>>(), base_exts, |_d, item| {
                item.clone()
            });
        Self { removed, modified: modified.into_iter().map(|(index, extension)| GifAppExtensionModified { index, extension }).collect(), added: added.into_iter().map(|(index, extension)| GifAppExtensionAdded { index, extension }).collect() }
    }
}
//#endregion 🔖️WeakCollectionDiffs

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.gif.89a`. No `snapshot: Option<GifSnapshot>` full-replace slot anywhere.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.gif.89a.diff")]
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
    pub loop_count: Option<Option<u16>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub frames: Option<GifFramesDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub comments: Option<GifCommentsDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub app_extensions: Option<GifAppExtensionsDiff>,
}

impl GifDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty_diff(&self) -> bool {
        self.width.is_none()
            && self.height.is_none()
            && self.gct.is_none()
            && self.background_color_index.is_none()
            && self.pixel_aspect_ratio.is_none()
            && self.loop_count.is_none()
            && self.frames.as_ref().is_none_or(GifFramesDiff::is_empty)
            && self.comments.as_ref().is_none_or(GifCommentsDiff::is_empty)
            && self.app_extensions.as_ref().is_none_or(GifAppExtensionsDiff::is_empty)
    }
}

impl MutationDiff<GifSnapshot> for GifDiff {
    fn apply(&self, base: &GifSnapshot) -> MutationApplyResult<GifSnapshot> {
        if let Some(frames) = &self.frames {
            validate_gif_triple(base.frames.len(), frames.removed.as_slice(), frames.modified.iter().map(|entry| entry.index), frames.added.iter().map(|entry| entry.index), ["frames"])?;
        }
        if let Some(comments) = &self.comments {
            validate_gif_triple(base.comments.len(), comments.removed.as_slice(), comments.modified.iter().map(|entry| entry.index), comments.added.iter().map(|entry| entry.index), ["comments"])?;
        }
        if let Some(extensions) = &self.app_extensions {
            validate_gif_triple(base.app_extensions.len(), extensions.removed.as_slice(), extensions.modified.iter().map(|entry| entry.index), extensions.added.iter().map(|entry| entry.index), ["appExtensions"])?;
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
        if let Some(v) = self.loop_count {
            next.loop_count = v;
        }
        if let Some(d) = &self.frames {
            next.frames = d.apply(&next.frames);
        }
        if let Some(d) = &self.comments {
            next.comments = d.apply(&next.comments);
        }
        if let Some(d) = &self.app_extensions {
            next.app_extensions = d.apply(&next.app_extensions);
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
        if other.loop_count.is_some() {
            self.loop_count = other.loop_count;
        }
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_gif_triple<I, J, K>(base_len: usize, removed: &[usize], modified: I, added: J, path: K) -> MutationApplyResult<()>
where
    I: IntoIterator<Item = usize>,
    J: IntoIterator<Item = usize>,
    K: IntoIterator,
    K::Item: AsRef<str>,
{
    let path: Vec<String> = path.into_iter().map(|part| part.as_ref().to_owned()).collect();
    let mut removed_set = std::collections::HashSet::new();
    for &index in removed {
        if index >= base_len || !removed_set.insert(index) {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "GIF collection removal is missing or duplicated").at(path.iter().map(String::as_str)));
        }
    }
    let mut modified_set = std::collections::HashSet::new();
    for index in modified {
        if index >= base_len || !modified_set.insert(index) || removed_set.contains(&index) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "GIF collection modification is missing, duplicated, or removed").at(path.iter().map(String::as_str)));
        }
    }
    let added: Vec<usize> = added.into_iter().collect();
    let final_len = base_len.saturating_sub(removed.len()).saturating_add(added.len());
    let mut added_set = std::collections::HashSet::new();
    for index in added {
        if index > final_len || !added_set.insert(index) {
            return Err(MutationApplyError::new("mutation.apply.invalid-index", "GIF collection addition index is invalid or duplicated").at(path.iter().map(String::as_str)));
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_snapshot(base: &GifSnapshot, snapshot: &GifSnapshot) -> GifDiff {
    <GifDiff as DiffAlgebra<GifSnapshot>>::between(base, snapshot)
}

/// 🧪️ P2-FG2: representative `GifDiff` (89a) cases for `diff_grammar_conformance_law`/
/// `protocol_walk_law` (`../../../../⚙️engine/🦀️.rs`'s `conformance_laws` module) —
/// the empty diff, plus a real `between()` result exercising every scalar field, both
/// tri-states (`gct`, `loop_count`), `GifFrameDiff`'s own THREE nested tri-states
/// (`lct`/`transparent_index`/`plain_text`), and all three collection triples (`frames`,
/// `comments`, `app_extensions`) at once (mirrors 87a's own `demo_diff_cases()`).
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<GifDiff> {
    let f = |seed: u8, w: u32, h: u32| GifFrame {
        left: 0,
        top: 0,
        width: w,
        height: h,
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
        width: 4,
        height: 4,
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
    let b = GifSnapshot { width: 8, height: 8, gct: None, background_color_index: 3, pixel_aspect_ratio: 5, loop_count: None, frames: vec![fb0, f(6, 3, 3), f(7, 3, 3)], comments: vec![], app_extensions: vec![], ..GifSnapshot::default() };
    vec![GifDiff::default(), diff_set_snapshot(&a, &b), diff_set_snapshot(&b, &a)]
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
// 🚫️aaaaaa�️aaaregion 🔖️Primitives

//#region 🔖️ValueCodecs
// 🚫️aaaaaaaaaaaaregion 🔖️ValueCodecs

//#region 🔖️DiffValueCodecs
// 🚫️aa�️a️aaaaaaregion 🔖️DiffValueCodecs

//#region 🔖️RealBinaryPrimitives

//#endregion 🔖️RealBinaryPrimitives

//#region 🔖️RealBinaryDiffFrame
/// 🧪️aaaaaaaregion 🔖️RealBinaryDiffFrame

//#region 🔖️TopLevel
// 🚫️aaDiregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec

//#region Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion Tests

// 🚫️a️a️a
#[cfg(test)]
use protocol::{DiffBinary,DiffText};
