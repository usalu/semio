//! 🔺️ SemioVideoDiff — handcrafted sparse diff over `SemioVideoSnapshot`. No
//! `snapshot: Option<SemioVideoSnapshot>` full-replace slot — even `SetSnapshot`'s diff is the
//! sparse field-by-field `SemioVideoDiff::between(base, next)`.
//!
//! Both collections this subset owns (`streams`, and within a stream its `samples`) are plain
//! ORDERED lists with no natural key (the master plan's own spec — `streams{kind,codec,width,
//! height,rate,samples{pts,key,data}}` — never proposes an id), so both are diffed via the shared
//! generic `engine::triples::IndexedTripleDiff<D, T>` (the index-keyed sibling of the
//! `NamedTripleDiff<K, D, T>` mesh/cad already reuse for their own id-keyed collections) — reusing
//! the SAME struct docx/bcf hand-rolled their own copy of (f6-final-summary.md §4.4: no `DslField`
//! bridge exists for generic collection-diff wrappers, so every subset hand-writes its own
//! `between`/`apply`/`inverse`/`absorb` algorithm over the shared struct; see
//! `w1b-type-ownership.md`'s "🧰️triples" entry). `#[derive(dsl::DslDiff)]` is not attempted here at
//! all — per this ticket's own instruction ("hand-roll all diff/op codecs — do not fight the
//! derive"), following the f6 recon's own finding that a `Vec<T>`-of-struct field inside a
//! `Vec<T>`-of-struct field (streams→samples) plus this file's own generic collection-triple
//! wrapper both individually block the derive macro.

use crate::standards::v1::subsets::base::schema::triples::{IndexAdded, IndexModified, IndexedTripleDiff};



use crate::standards::v1::subsets::video::schema::snapshot::{SemioRational, SemioVideoSample, SemioVideoSnapshot, SemioVideoStream, SemioVideoStreamKind};
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;

//#region 🔖️CollectionDiffTypes
pub type SemioVideoStreamsDiff = IndexedTripleDiff<SemioVideoStreamDiff, SemioVideoStream>;
pub type SemioVideoSamplesDiff = IndexedTripleDiff<SemioVideoSampleDiff, SemioVideoSample>;

/// 🎯️ Per-sample sparse diff — every field of `SemioVideoSample` is a plain scalar/opaque-bytes
/// value (no nested enum/collection), so this is a flat `Option<T>` bag, no tri-state needed
/// (the spec never marks any of `pts`/`key`/`data` as nullable).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SemioVideoSampleDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub pts: Option<u64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<bool>,
    /// 🗄️ Opaque payload replacement — honest boundary, whole-value only (never sub-diffed).
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<u8>>,
}

/// 🎞️ Per-stream sparse diff.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SemioVideoStreamDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<SemioVideoStreamKind>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub codec: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub rate: Option<SemioRational>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub samples: Option<SemioVideoSamplesDiff>,
}
//#endregion 🔖️CollectionDiffTypes

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SemioVideoDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub streams: Option<SemioVideoStreamsDiff>,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_indexed<T, D>(items: &mut Vec<T>, diff: &IndexedTripleDiff<D, T>, apply_item: impl Fn(&mut T, &D))
where
    T: Clone,
{
    for m in &diff.modified {
        if let Some(item) = items.get_mut(m.index) {
            apply_item(item, &m.diff);
        }
    }
    let mut removed_sorted = diff.removed.clone();
    removed_sorted.sort_unstable_by(|a, b| b.cmp(a));
    removed_sorted.dedup();
    for idx in removed_sorted {
        if idx < items.len() {
            items.remove(idx);
        }
    }
    let mut additions: Vec<&IndexAdded<T>> = diff.added.iter().collect();
    additions.sort_by_key(|a| a.index);
    for add in additions {
        let at = add.index.min(items.len());
        items.insert(at, add.item.clone());
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_indexed<T, D>(base_items: &[T], diff: &IndexedTripleDiff<D, T>, inverse_item: impl Fn(&T, &D) -> D) -> IndexedTripleDiff<D, T>
where
    T: Clone,
{
    let removed: Vec<usize> = diff.added.iter().map(|a| a.index).collect();
    let mut modified = Vec::new();
    for m in &diff.modified {
        if let Some(original) = base_items.get(m.index) {
            let next_index = transform_index(m.index, &diff.removed, &diff.added);
            modified.push(IndexModified { index: next_index, diff: inverse_item(original, &m.diff) });
        }
    }
    let mut added = Vec::new();
    for &idx in &diff.removed {
        if let Some(original) = base_items.get(idx) {
            added.push(IndexAdded { index: idx, item: original.clone() });
        }
    }
    added.sort_by_key(|a| a.index);
    IndexedTripleDiff { removed, modified, added }
}

/// 🧮️ Maps a base-side index through a diff's OWN removed/added to the position it ends up at once
/// that diff has been applied (svg `SvgDiff`'s `transform_index` precedent, generalized).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn transform_index<T>(idx: usize, removed: &[usize], added: &[IndexAdded<T>]) -> usize {
    let removed_before = removed.iter().filter(|&&r| r < idx).count();
    let pos = idx - removed_before;
    let mut order: Vec<usize> = added.iter().map(|a| a.index).collect();
    order.sort_unstable();
    let mut shift = 0usize;
    for target in order {
        if target <= pos + shift {
            shift += 1;
        } else {
            break;
        }
    }
    pos + shift
}

enum ItemOrigin {
    Base(usize),
    Added(usize),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn simulate_mid_origins<T>(base_len: usize, removed: &[usize], added: &[IndexAdded<T>]) -> Vec<ItemOrigin> {
    let mut mid: Vec<ItemOrigin> = (0..base_len).filter(|i| !removed.contains(i)).map(ItemOrigin::Base).collect();
    let mut order: Vec<(usize, usize)> = added.iter().enumerate().map(|(k, a)| (a.index, k)).collect();
    order.sort_by_key(|(idx, _)| *idx);
    for (idx, k) in order {
        let at = idx.min(mid.len());
        mid.insert(at, ItemOrigin::Added(k));
    }
    mid
}

/// 🧮️ Sequential-coalesce absorb per the recipe's normative algorithm: `absorb_item` recursively
/// absorbs two per-field diffs of the SAME item; `apply_item` patches a `D` onto a `T` (needed
/// when `d2` modifies an item `d1` just added).
#[allow(clippy::too_many_arguments)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_indexed<T, D>(d1: IndexedTripleDiff<D, T>, d2: &IndexedTripleDiff<D, T>, absorb_item: impl Fn(D, D) -> D, apply_item: impl Fn(&T, &D) -> T) -> IndexedTripleDiff<D, T>
where
    T: Clone,
    D: Clone,
{
    let d1_ref_max = d1.removed.iter().copied().chain(d1.modified.iter().map(|m| m.index)).max();
    let mut base_len = d1_ref_max.map_or(0, |m| m + 1);
    let mid_len_needed_by_d1 = d1.added.iter().map(|a| a.index + 1).max().unwrap_or(0);
    while base_len.saturating_sub(d1.removed.len()) + d1.added.len() < mid_len_needed_by_d1 {
        base_len += 1;
    }
    let d2_ref_max = d2.removed.iter().copied().chain(d2.modified.iter().map(|m| m.index)).max();
    let required_mid_len = d2_ref_max.map_or(0, |m| m + 1);
    while base_len.saturating_sub(d1.removed.len()) + d1.added.len() < required_mid_len {
        base_len += 1;
    }

    let mid = simulate_mid_origins(base_len, &d1.removed, &d1.added);

    let mut removed = d1.removed.clone();
    let mut modified = d1.modified;
    let mut working_added = d1.added;
    let mut annihilated: std::collections::HashSet<usize> = std::collections::HashSet::new();

    for &r2 in &d2.removed {
        match mid.get(r2) {
            Some(ItemOrigin::Base(bi)) => {
                if !removed.contains(bi) {
                    removed.push(*bi);
                }
                modified.retain(|m| &m.index != bi);
            }
            Some(ItemOrigin::Added(k)) => {
                annihilated.insert(*k);
            }
            None => {}
        }
    }
    for m2 in &d2.modified {
        match mid.get(m2.index) {
            Some(ItemOrigin::Base(bi)) => {
                if removed.contains(bi) {
                    continue;
                }
                match modified.iter_mut().find(|m| &m.index == bi) {
                    Some(existing) => existing.diff = absorb_item(existing.diff.clone(), m2.diff.clone()),
                    None => modified.push(IndexModified { index: *bi, diff: m2.diff.clone() }),
                }
            }
            Some(ItemOrigin::Added(k)) => {
                if annihilated.contains(k) {
                    continue;
                }
                if let Some(add) = working_added.get_mut(*k) {
                    add.item = apply_item(&add.item, &m2.diff);
                }
            }
            None => {}
        }
    }

    let mut added = Vec::new();
    for (k, add) in working_added.into_iter().enumerate() {
        if annihilated.contains(&k) {
            continue;
        }
        let final_index = transform_index(add.index, &d2.removed, &d2.added);
        added.push(IndexAdded { index: final_index, item: add.item });
    }
    for a2 in &d2.added {
        added.push(a2.clone());
    }
    added.sort_by_key(|a| a.index);

    IndexedTripleDiff { removed, modified, added }
}
//#endregion 🔖️GenericIndexedEngine

//#region 🔖️VideoDiffLogic
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_sample(old: &SemioVideoSample, new: &SemioVideoSample) -> Option<SemioVideoSampleDiff> {
    if old == new {
        return None;
    }
    Some(SemioVideoSampleDiff { pts: (old.pts != new.pts).then_some(new.pts), key: (old.key != new.key).then_some(new.key), data: (old.data != new.data).then(|| new.data.clone()) })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_sample(sample: &mut SemioVideoSample, diff: &SemioVideoSampleDiff) {
    if let Some(v) = diff.pts {
        sample.pts = v;
    }
    if let Some(v) = diff.key {
        sample.key = v;
    }
    if let Some(v) = &diff.data {
        sample.data = v.clone();
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_stream(stream: &mut SemioVideoStream, diff: &SemioVideoStreamDiff) {
    if let Some(v) = diff.kind {
        stream.kind = v;
    }
    if let Some(v) = &diff.codec {
        stream.codec = v.clone();
    }
    if let Some(v) = diff.width {
        stream.width = v;
    }
    if let Some(v) = diff.height {
        stream.height = v;
    }
    if let Some(v) = diff.rate {
        stream.rate = v;
    }
    if let Some(sd) = &diff.samples {
        apply_indexed(&mut stream.samples, sd, apply_sample);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_sample_to_copy(sample: &SemioVideoSample, diff: &SemioVideoSampleDiff) -> SemioVideoSample {
    let mut out = sample.clone();
    apply_sample(&mut out, diff);
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_stream_to_copy(stream: &SemioVideoStream, diff: &SemioVideoStreamDiff) -> SemioVideoStream {
    let mut out = stream.clone();
    apply_stream(&mut out, diff);
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_sample(base: &SemioVideoSample, diff: &SemioVideoSampleDiff) -> SemioVideoSampleDiff {
    SemioVideoSampleDiff { pts: diff.pts.map(|_| base.pts), key: diff.key.map(|_| base.key), data: diff.data.as_ref().map(|_| base.data.clone()) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_stream(base: &SemioVideoStream, diff: &SemioVideoStreamDiff) -> SemioVideoStreamDiff {
    SemioVideoStreamDiff {
        kind: diff.kind.map(|_| base.kind),
        codec: diff.codec.as_ref().map(|_| base.codec.clone()),
        width: diff.width.map(|_| base.width),
        height: diff.height.map(|_| base.height),
        rate: diff.rate.map(|_| base.rate),
        samples: diff.samples.as_ref().map(|sd| inverse_indexed(&base.samples, sd, inverse_sample)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_sample_diff(mut a: SemioVideoSampleDiff, b: SemioVideoSampleDiff) -> SemioVideoSampleDiff {
    if b.pts.is_some() {
        a.pts = b.pts;
    }
    if b.key.is_some() {
        a.key = b.key;
    }
    if b.data.is_some() {
        a.data = b.data;
    }
    a
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_stream_diff(mut a: SemioVideoStreamDiff, b: SemioVideoStreamDiff) -> SemioVideoStreamDiff {
    if b.kind.is_some() {
        a.kind = b.kind;
    }
    if b.codec.is_some() {
        a.codec = b.codec;
    }
    if b.width.is_some() {
        a.width = b.width;
    }
    if b.height.is_some() {
        a.height = b.height;
    }
    if b.rate.is_some() {
        a.rate = b.rate;
    }
    a.samples = match (a.samples.take(), b.samples) {
        (None, x) => x,
        (x, None) => x,
        (Some(sa), Some(sb)) => Some(absorb_indexed(sa, &sb, absorb_sample_diff, apply_sample_to_copy)),
    };
    a
}
//#endregion 🔖️VideoDiffLogic

//#region 🔖️Apply
impl MutationDiff<SemioVideoSnapshot> for SemioVideoDiff {
    fn apply(&self, base: &SemioVideoSnapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<SemioVideoSnapshot> {
        let mut next = base.clone();
        if let Some(d) = &self.streams {
            crate::standards::v1::subsets::base::schema::triples::validate_indexed_triple(d, next.streams.len(), ["streams"])?;
            apply_indexed(&mut next.streams, d, apply_stream);
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        self.streams = match (self.streams.take(), other.streams) {
            (None, x) => x,
            (x, None) => x,
            (Some(a), Some(b)) => Some(absorb_indexed(a, &b, absorb_stream_diff, apply_stream_to_copy)),
        };
    }
}
//#endregion 🔖️Apply

//#region 🔖️DiffAlgebra
impl DiffAlgebra<SemioVideoSnapshot> for SemioVideoDiff {
    fn inverse(&self, base: &SemioVideoSnapshot) -> Self {
        SemioVideoDiff { streams: self.streams.as_ref().map(|d| inverse_indexed(&base.streams, d, inverse_stream)) }
    }

    fn is_empty(&self) -> bool {
        self.streams.is_none()
    }
}
//#endregion 🔖️DiffAlgebra



/// 🧩 Builds the diff for inserting `stream` at `index` (FINAL state).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_stream(index: usize, stream: SemioVideoStream) -> SemioVideoDiff {
    SemioVideoDiff { streams: Some(SemioVideoStreamsDiff { added: vec![IndexAdded { index, item: stream }], ..Default::default() }) }
}

/// 🧩 Builds the diff for removing the stream at `index` (BASE-state index).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_stream(index: usize) -> SemioVideoDiff {
    SemioVideoDiff { streams: Some(SemioVideoStreamsDiff { removed: vec![index], ..Default::default() }) }
}

/// 🧩 Builds the diff for setting a stream's container-level metadata, via a real field-by-field
/// comparison against `old` (never full-replace).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_stream_meta(old: &SemioVideoStream, index: usize, kind: SemioVideoStreamKind, codec: &str, width: u32, height: u32, rate: SemioRational) -> SemioVideoDiff {
    let sd = SemioVideoStreamDiff {
        kind: (old.kind != kind).then_some(kind),
        codec: (old.codec != codec).then(|| codec.to_string()),
        width: (old.width != width).then_some(width),
        height: (old.height != height).then_some(height),
        rate: (old.rate != rate).then_some(rate),
        samples: None,
    };
    if sd.kind.is_none() && sd.codec.is_none() && sd.width.is_none() && sd.height.is_none() && sd.rate.is_none() {
        return SemioVideoDiff::default();
    }
    wrap_stream_diff(index, sd)
}

/// 🧩 Builds the diff for inserting `sample` at `index` within stream `stream_index` (FINAL state).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_sample(stream_index: usize, index: usize, sample: SemioVideoSample) -> SemioVideoDiff {
    let samples = SemioVideoSamplesDiff { added: vec![IndexAdded { index, item: sample }], ..Default::default() };
    wrap_stream_diff(stream_index, SemioVideoStreamDiff { samples: Some(samples), ..Default::default() })
}

/// 🧩 Builds the diff for removing the sample at `index` (BASE-state index) within `stream_index`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_sample(stream_index: usize, index: usize) -> SemioVideoDiff {
    let samples = SemioVideoSamplesDiff { removed: vec![index], ..Default::default() };
    wrap_stream_diff(stream_index, SemioVideoStreamDiff { samples: Some(samples), ..Default::default() })
}

/// 🧩 Builds the diff for replacing one sample's opaque `data` payload.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_sample_data(old: &SemioVideoSample, stream_index: usize, index: usize, data: Vec<u8>) -> SemioVideoDiff {
    if old.data == data {
        return SemioVideoDiff::default();
    }
    let sample_diff = SemioVideoSampleDiff { pts: None, key: None, data: Some(data) };
    let samples = SemioVideoSamplesDiff { modified: vec![IndexModified { index, diff: sample_diff }], ..Default::default() };
    wrap_stream_diff(stream_index, SemioVideoStreamDiff { samples: Some(samples), ..Default::default() })
}

/// 🧩 Builds the diff for setting a sample's `pts`/`key` flags.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_sample_flags(old: &SemioVideoSample, stream_index: usize, index: usize, pts: u64, key: bool) -> SemioVideoDiff {
    let sample_diff = SemioVideoSampleDiff { pts: (old.pts != pts).then_some(pts), key: (old.key != key).then_some(key), data: None };
    if sample_diff.pts.is_none() && sample_diff.key.is_none() {
        return SemioVideoDiff::default();
    }
    let samples = SemioVideoSamplesDiff { modified: vec![IndexModified { index, diff: sample_diff }], ..Default::default() };
    wrap_stream_diff(stream_index, SemioVideoStreamDiff { samples: Some(samples), ..Default::default() })
}

/// 🧭️ Wraps a single stream-level diff into a full `SemioVideoDiff`, addressing it as `modified`
/// at `stream_index`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn wrap_stream_diff(stream_index: usize, diff: SemioVideoStreamDiff) -> SemioVideoDiff {
    SemioVideoDiff { streams: Some(SemioVideoStreamsDiff { modified: vec![IndexModified { index: stream_index, diff }], ..Default::default() }) }
}
//#region 🔖️Demo
/// 🌱 Representative `SemioVideoDiff` cases built declaratively (empty/no-op and a removed stream row) — single source of truth for `diff_grammar_conformance_law`/`protocol_walk_law` in
/// `🎹️composer/🦀️.rs`.
#[cfg(all(test, feature = "conversion-video"))]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<SemioVideoDiff> {
    vec![SemioVideoDiff::default(), SemioVideoDiff { streams: Some(IndexedTripleDiff { removed: vec![0], ..Default::default() }) }]
}
//#endregion 🔖️Demo
//#endregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️handcrafted-diff-codec/🦀️.rs"]
mod handcrafted_diff_codec_tests;
//#endregion 🧪️Tests
