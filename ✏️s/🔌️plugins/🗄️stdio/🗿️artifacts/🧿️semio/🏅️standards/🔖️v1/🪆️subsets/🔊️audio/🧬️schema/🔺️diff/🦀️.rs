//! 🔺️ SemioAudioDiff — sparse, handcrafted per-field diff replacing the W1b full-replace
//! scaffold. `sample_rate`/`format` are plain scalar slots; `channels` (strong, per-field
//! diffable — today one field, `samples`) and `tags` (weak — the diff IS the whole new pair) are
//! both index-keyed collection triples built DIRECTLY on the shared
//! `engine::triples::IndexedTripleDiff<D,T>` type (per the ticket's mandate to reuse `🧰️triples`
//! rather than hand-duplicating gif's bespoke `GifFramesDiff`/`GifCommentsDiff` per collection —
//! the docx precedent of "one generic codec pair, N instantiations"). No tri-state
//! `Option<Option<T>>` fields exist in this shape (nothing here is individually nullable), so —
//! unlike gif's `GifDiff` — the ONLY reason this is hand-rolled rather than
//! `#[derive(dsl::DslDiff)]` is the ticket's own instruction to hand-roll every op/diff codec
//! outright rather than risk the generic-collection `DslField` gap (f6-final-summary.md §4.4,
//! independently hit by gltf/pptx/docx/bcf/xlsx) — `IndexedTripleDiff<D,T>` is a bare generic with
//! no `DslField` impl of its own.

use crate::standards::v1::subsets::audio::schema::snapshot::{SemioAudioChannel, SemioAudioFormat, SemioAudioSnapshot, SemioAudioTag};
use crate::standards::v1::subsets::base::schema::triples::{self, IndexAdded, IndexModified, IndexedTripleDiff};
use protocol::command::DiffAlgebra;
/// 🔧️ Unconditional — `impl protocol::DiffCodec for SemioAudioDiff` below's `encode_diff`/
/// `decode_diff` are now real production code (binary upgrade, this wave), not test-only.
use protocol::{DiffCodec};
use protocol::MutationDiff;

//#region 🔖️IndexTransport
/// 📐️ Shared rank/unrank arithmetic for index-keyed collection diffs — see
/// `🧬️schema-design.md` §Absorb; ported verbatim from gif 89a's own diff module (the reference
/// implementation for this arithmetic), generalized here to operate on the shared
/// `IndexedTripleDiff<D,T>` shape instead of a bespoke per-artifact triple struct.
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

//#region 🔖️GenericIndexedCollectionOps
/// 🕳️ Emptiness for the shared `IndexedTripleDiff<D,T>` (the engine type itself carries no
/// `is_empty` — this subset supplies the semantics, per the recipe: the engine is data+text-codec
/// only, apply/between/absorb/inverse are each collection's own job).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn indexed_is_empty<D, T>(t: &IndexedTripleDiff<D, T>) -> bool {
    t.removed.is_empty() && t.modified.is_empty() && t.added.is_empty()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn indexed_between<T: Clone + PartialEq, D>(base: &[T], other: &[T], diff_between: impl Fn(&T, &T) -> Option<D>) -> IndexedTripleDiff<D, T> {
    let min = base.len().min(other.len());
    let mut modified = Vec::new();
    for i in 0..min {
        if let Some(d) = diff_between(&base[i], &other[i]) {
            modified.push(IndexModified { index: i, diff: d });
        }
    }
    let removed: Vec<usize> = (min..base.len()).collect();
    let added: Vec<IndexAdded<T>> = (min..other.len()).map(|i| IndexAdded { index: i, item: other[i].clone() }).collect();
    IndexedTripleDiff { removed, modified, added }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn indexed_apply<T: Clone, D>(triple: &IndexedTripleDiff<D, T>, base: &[T], diff_apply: impl Fn(&D, &T) -> T) -> Vec<T> {
    let mut next: Vec<Option<T>> = base.iter().cloned().map(Some).collect();
    for m in &triple.modified {
        if let Some(Some(item)) = next.get_mut(m.index) {
            *item = diff_apply(&m.diff, item);
        }
    }
    let mut removed_sorted = triple.removed.clone();
    removed_sorted.sort_unstable();
    removed_sorted.reverse();
    for &r in &removed_sorted {
        if r < next.len() {
            next.remove(r);
        }
    }
    let mut out: Vec<T> = next.into_iter().flatten().collect();
    let mut added_sorted = triple.added.clone();
    added_sorted.sort_by_key(|a| a.index);
    for a in added_sorted {
        let at = a.index.min(out.len());
        out.insert(at, a.item);
    }
    out
}

/// 🧮️ Sequential-coalesce absorb, generalized from gif's `absorb_indexed_collection` (see that
/// module's doc comment for the derivation and the plan's 3 mandated canonical cases, all
/// re-verified for this generic form in this file's own tests below).
#[allow(clippy::too_many_arguments)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn indexed_absorb<T: Clone, D: Clone>(mine: &mut IndexedTripleDiff<D, T>, other: IndexedTripleDiff<D, T>, mut absorb_diff: impl FnMut(&mut D, D), apply_diff_to_item: impl Fn(&D, &T) -> T) {
    let removed1 = std::mem::take(&mut mine.removed);
    let modified1: Vec<(usize, D)> = std::mem::take(&mut mine.modified).into_iter().map(|m| (m.index, m.diff)).collect();
    let added1: Vec<(usize, T)> = std::mem::take(&mut mine.added).into_iter().map(|a| (a.index, a.item)).collect();
    let removed2 = other.removed;
    let modified2: Vec<(usize, D)> = other.modified.into_iter().map(|m| (m.index, m.diff)).collect();
    let added2: Vec<(usize, T)> = other.added.into_iter().map(|a| (a.index, a.item)).collect();

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

    mine.removed = merged_removed_base;
    mine.modified = merged_modified.into_iter().map(|(index, diff)| IndexModified { index, diff }).collect();
    mine.added = merged_added_final.into_iter().map(|(index, item)| IndexAdded { index, item }).collect();
}

/// ↩️ Diff-level inverse for a generic index-keyed collection triple, given the ORIGINAL base
/// items — generalized from gif's `inverse_indexed_collection`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn indexed_inverse<T: Clone, D>(triple: &IndexedTripleDiff<D, T>, base_items: &[T], diff_inverse: impl Fn(&D, &T) -> D) -> IndexedTripleDiff<D, T> {
    let mut removed_sorted: Vec<usize> = triple.removed.clone();
    removed_sorted.sort_unstable();
    let mut added_index_sorted: Vec<usize> = triple.added.iter().map(|a| a.index).collect();
    added_index_sorted.sort_unstable();

    let mut inv_removed: Vec<usize> = triple.added.iter().map(|a| a.index).collect();
    let mut inv_modified: Vec<IndexModified<D>> = Vec::new();
    for m in &triple.modified {
        if let Some(orig) = base_items.get(m.index) {
            let after_index = transport_forward(m.index, &removed_sorted, &added_index_sorted);
            inv_modified.push(IndexModified { index: after_index, diff: diff_inverse(&m.diff, orig) });
        }
    }
    let mut inv_added: Vec<IndexAdded<T>> = Vec::new();
    for &r in &triple.removed {
        if let Some(orig) = base_items.get(r) {
            inv_added.push(IndexAdded { index: r, item: orig.clone() });
        }
    }
    inv_removed.sort_unstable();
    inv_added.sort_by_key(|a| a.index);
    IndexedTripleDiff { removed: inv_removed, modified: inv_modified, added: inv_added }
}
//#endregion 🔖️GenericIndexedCollectionOps

//#region 🔖️ChannelDiff
/// 🔺️ Sparse diff for one [`SemioAudioChannel`] — a strong entity per the recipe. One field
/// today (`samples`); kept as its own type (rather than folding into the collection triple
/// directly) so a future per-channel field slots in without reshaping `channels`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SemioAudioChannelDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub samples: Option<Vec<f32>>,
}

impl SemioAudioChannelDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.samples.is_none()
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn between(base: &SemioAudioChannel, other: &SemioAudioChannel) -> Self {
        Self { samples: (base.samples != other.samples).then_some(other.samples.clone()) }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn apply(&self, base: &SemioAudioChannel) -> SemioAudioChannel {
        let mut next = base.clone();
        if let Some(v) = &self.samples {
            next.samples = v.clone();
        }
        next
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn inverse(&self, base: &SemioAudioChannel) -> Self {
        Self { samples: self.samples.as_ref().map(|_| base.samples.clone()) }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn absorb(&mut self, other: Self) {
        if other.samples.is_some() {
            self.samples = other.samples;
        }
    }
}

pub type SemioAudioChannelsDiff = IndexedTripleDiff<SemioAudioChannelDiff, SemioAudioChannel>;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn channels_between(base: &[SemioAudioChannel], other: &[SemioAudioChannel]) -> SemioAudioChannelsDiff {
    indexed_between(base, other, |a, b| {
        let d = SemioAudioChannelDiff::between(a, b);
        (!d.is_empty()).then_some(d)
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn channels_apply(d: &SemioAudioChannelsDiff, base: &[SemioAudioChannel]) -> Vec<SemioAudioChannel> {
    indexed_apply(d, base, |diff, item| diff.apply(item))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn channels_absorb(mine: &mut SemioAudioChannelsDiff, other: SemioAudioChannelsDiff) {
    indexed_absorb(mine, other, |d, o| d.absorb(o), |diff, item| diff.apply(item));
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn channels_inverse(d: &SemioAudioChannelsDiff, base_items: &[SemioAudioChannel]) -> SemioAudioChannelsDiff {
    indexed_inverse(d, base_items, |diff, item| diff.inverse(item))
}
//#endregion 🔖️ChannelDiff

//#region 🔖️TagsDiff
/// 🏷️ `tags` is a WEAK/value collection per the recipe: its "diff" IS the whole new
/// [`SemioAudioTag`] (`D = T = SemioAudioTag`), no further sub-diffing of a key/value pair.
pub type SemioAudioTagsDiff = IndexedTripleDiff<SemioAudioTag, SemioAudioTag>;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn tags_between(base: &[SemioAudioTag], other: &[SemioAudioTag]) -> SemioAudioTagsDiff {
    indexed_between(base, other, |a, b| (a != b).then_some(b.clone()))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn tags_apply(d: &SemioAudioTagsDiff, base: &[SemioAudioTag]) -> Vec<SemioAudioTag> {
    indexed_apply(d, base, |diff, _item| diff.clone())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn tags_absorb(mine: &mut SemioAudioTagsDiff, other: SemioAudioTagsDiff) {
    indexed_absorb(mine, other, |d, o| *d = o, |diff, _item| diff.clone());
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn tags_inverse(d: &SemioAudioTagsDiff, base_items: &[SemioAudioTag]) -> SemioAudioTagsDiff {
    indexed_inverse(d, base_items, |_diff, item| item.clone())
}
//#endregion 🔖️TagsDiff

//#region 🔖️Diff
/// 🔺️ Diff for `s.stdio.semio.audio`. No `snapshot: Option<SemioAudioSnapshot>` full-replace
/// slot anywhere — every field is sparse.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SemioAudioDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<u32>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<SemioAudioFormat>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub channels: Option<SemioAudioChannelsDiff>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<SemioAudioTagsDiff>,
}

impl SemioAudioDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty_diff(&self) -> bool {
        self.sample_rate.is_none() && self.format.is_none() && self.channels.as_ref().is_none_or(indexed_is_empty) && self.tags.as_ref().is_none_or(indexed_is_empty)
    }
}

impl MutationDiff<SemioAudioSnapshot> for SemioAudioDiff {
    fn apply(&self, base: &SemioAudioSnapshot) -> protocol::MutationApplyResult<SemioAudioSnapshot> {
        let mut next = base.clone();
        if let Some(v) = self.sample_rate {
            next.sample_rate = v;
        }
        if let Some(v) = self.format {
            next.format = v;
        }
        if let Some(d) = &self.channels {
            triples::validate_indexed_triple(d, next.channels.len(), ["channels"])?;
            next.channels = channels_apply(d, &next.channels);
        }
        if let Some(d) = &self.tags {
            triples::validate_indexed_triple(d, next.tags.len(), ["tags"])?;
            next.tags = tags_apply(d, &next.tags);
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.sample_rate.is_some() {
            self.sample_rate = other.sample_rate;
        }
        if other.format.is_some() {
            self.format = other.format;
        }
        match (&mut self.channels, other.channels) {
            (Some(mine), Some(theirs)) => channels_absorb(mine, theirs),
            (slot @ None, Some(theirs)) => *slot = Some(theirs),
            _ => {}
        }
        match (&mut self.tags, other.tags) {
            (Some(mine), Some(theirs)) => tags_absorb(mine, theirs),
            (slot @ None, Some(theirs)) => *slot = Some(theirs),
            _ => {}
        }
    }
}

impl DiffAlgebra<SemioAudioSnapshot> for SemioAudioDiff {
    fn inverse(&self, base: &SemioAudioSnapshot) -> Self {
        Self {
            sample_rate: self.sample_rate.map(|_| base.sample_rate),
            format: self.format.map(|_| base.format),
            channels: self.channels.as_ref().map(|d| channels_inverse(d, &base.channels)),
            tags: self.tags.as_ref().map(|d| tags_inverse(d, &base.tags)),
        }
    }

    fn between(base: &SemioAudioSnapshot, other: &SemioAudioSnapshot) -> Self {
        let channels_diff = channels_between(&base.channels, &other.channels);
        let tags_diff = tags_between(&base.tags, &other.tags);
        Self {
            sample_rate: (base.sample_rate != other.sample_rate).then_some(other.sample_rate),
            format: (base.format != other.format).then_some(other.format),
            channels: (!indexed_is_empty(&channels_diff)).then_some(channels_diff),
            tags: (!indexed_is_empty(&tags_diff)).then_some(tags_diff),
        }
    }

    fn is_empty(&self) -> bool {
        self.is_empty_diff()
    }
}

/// 🧩️ Builds a set-snapshot diff — sparse field-by-field, never a full-replace slot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_snapshot(base: &SemioAudioSnapshot, snapshot: &SemioAudioSnapshot) -> SemioAudioDiff {
    <SemioAudioDiff as DiffAlgebra<SemioAudioSnapshot>>::between(base, snapshot)
}
//#endregion 🔖️Diff

//#region 🔖️HandcraftedDiffCodec
/// 🧪️ Hand-rolled `protocol::DiffCodec` per the ticket's blanket instruction (never fight the
/// derive — see this module's doc comment for the generic-collection `DslField` gap that would
/// otherwise block `#[derive(dsl::DslDiff)]` on the `channels`/`tags` fields).
///
/// **Grammar** (real, not `serde_json`): one space-separated `name=value` token per changed
/// top-level scalar field; the two collections print as `name{<🧰️triples enc_indexed_triple
/// output>}` sections, reusing the SHARED engine codec directly (no per-collection hand-duplicated
/// bracket printer, unlike gif's `enc_frames_diff`/`enc_comments_diff` — the docx-precedent
/// simplification this ticket calls for). `f32` samples print as `to_bits()` hex tokens (exact
/// round trip, no float-formatting precision loss, no NaN/–0.0 ambiguity). Strings are lowercase
/// hex. Worked example: `rate=44100 format=f32 channels{[];[1:[1,[3f800000]]];[]}
/// tags{[0];[];[0:[74697465,6669727374]]}`.
//#region 🔖️Primitives











//#endregion 🔖️Primitives

//#region 🔖️ValueCodecs

















//#endregion 🔖️ValueCodecs

//#region 🔖️TopLevel







//#endregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec

//#region 🔖️Demo
/// 🌱 Representative `SemioAudioDiff` cases (empty/no-op, a full field sweep both directions incl.
/// both collection triples) — single source of truth for `diff_grammar_conformance_law`/
/// `protocol_walk_law` in `🎹️composer/🦀️.rs`.
#[cfg(all(test, feature = "conversion-audio"))]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<SemioAudioDiff> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn channel(seed: f32, len: usize) -> SemioAudioChannel {
        SemioAudioChannel { samples: (0..len).map(|i| seed + i as f32 * 0.1).collect() }
    }
    let a = SemioAudioSnapshot {
        sample_rate: 44_100,
        format: SemioAudioFormat::Pcm16,
        channels: vec![channel(0.0, 4), channel(1.0, 4), channel(2.0, 4)],
        tags: vec![SemioAudioTag { key: "title".into(), value: "one".into() }],
        ..SemioAudioSnapshot::default()
    };
    let b = SemioAudioSnapshot {
        sample_rate: 48_000,
        format: SemioAudioFormat::Float32,
        channels: vec![channel(9.0, 2), channel(1.0, 4)],
        tags: vec![SemioAudioTag { key: "title".into(), value: "two".into() }, SemioAudioTag { key: "artist".into(), value: "someone".into() }],
        ..SemioAudioSnapshot::default()
    };
    vec![SemioAudioDiff::default(), <SemioAudioDiff as DiffAlgebra<SemioAudioSnapshot>>::between(&a, &b), <SemioAudioDiff as DiffAlgebra<SemioAudioSnapshot>>::between(&b, &a)]
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests

#[cfg(test)]
use protocol::{DiffBinary,DiffText};
