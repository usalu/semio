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

use crate::artifacts::semio::standards::v1::subsets::base::schema::triples::{dec_indexed_triple, enc_indexed_triple, split_top_level, strip_brackets, IndexAdded, IndexModified, IndexedTripleDiff};
use crate::artifacts::semio::standards::v1::subsets::video::schema::snapshot::{SemioRational, SemioVideoSample, SemioVideoSnapshot, SemioVideoStream, SemioVideoStreamKind};
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
//#endregion 🔖️Diff

//#region 🔖️GenericIndexedEngine
/// 🧮️ `between`/`apply`/`inverse`/`absorb` over `IndexedTripleDiff<D,T>`, generic over item `T`
/// and per-field diff `D` — ported verbatim (same algorithm) from docx's own hand-rolled indexed
/// engine, this subset's own instance since no shared generic ALGORITHM exists yet (only the
/// shared struct does). Reused twice below: once for `streams`, once (nested, inside a modified
/// stream) for `samples`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_indexed<T, D>(base: &[T], other: &[T], diff_item: impl Fn(&T, &T) -> Option<D>) -> Option<IndexedTripleDiff<D, T>>
where
    T: Clone + PartialEq,
{
    let min_len = base.len().min(other.len());
    let mut modified = Vec::new();
    for i in 0..min_len {
        if base[i] != other[i] {
            if let Some(d) = diff_item(&base[i], &other[i]) {
                modified.push(IndexModified { index: i, diff: d });
            }
        }
    }
    let removed: Vec<usize> = (other.len()..base.len()).collect();
    let added: Vec<IndexAdded<T>> = (min_len..other.len()).map(|i| IndexAdded { index: i, item: other[i].clone() }).collect();
    if modified.is_empty() && removed.is_empty() && added.is_empty() {
        None
    } else {
        Some(IndexedTripleDiff { removed, modified, added })
    }
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
fn absorb_indexed<T, D>(d1: IndexedTripleDiff<D, T>, d2: IndexedTripleDiff<D, T>, absorb_item: impl Fn(D, D) -> D, apply_item: impl Fn(&T, &D) -> T) -> IndexedTripleDiff<D, T>
where
    T: Clone,
    D: Clone,
{
    let d1_ref_max = d1.removed.iter().copied().chain(d1.modified.iter().map(|m| m.index)).max();
    let mut base_len = d1_ref_max.map(|m| m + 1).unwrap_or(0);
    let mid_len_needed_by_d1 = d1.added.iter().map(|a| a.index + 1).max().unwrap_or(0);
    while base_len.saturating_sub(d1.removed.len()) + d1.added.len() < mid_len_needed_by_d1 {
        base_len += 1;
    }
    let d2_ref_max = d2.removed.iter().copied().chain(d2.modified.iter().map(|m| m.index)).max();
    let required_mid_len = d2_ref_max.map(|m| m + 1).unwrap_or(0);
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
fn diff_stream(old: &SemioVideoStream, new: &SemioVideoStream) -> Option<SemioVideoStreamDiff> {
    if old == new {
        return None;
    }
    let samples = between_indexed(&old.samples, &new.samples, diff_sample);
    Some(SemioVideoStreamDiff {
        kind: (old.kind != new.kind).then_some(new.kind),
        codec: (old.codec != new.codec).then(|| new.codec.clone()),
        width: (old.width != new.width).then_some(new.width),
        height: (old.height != new.height).then_some(new.height),
        rate: (old.rate != new.rate).then_some(new.rate),
        samples,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_video(base: &SemioVideoSnapshot, other: &SemioVideoSnapshot) -> SemioVideoDiff {
    SemioVideoDiff { streams: between_indexed(&base.streams, &other.streams, diff_stream) }
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
fn sample_with_diff_applied(sample: &SemioVideoSample, diff: &SemioVideoSampleDiff) -> SemioVideoSample {
    let mut out = sample.clone();
    apply_sample(&mut out, diff);
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn stream_with_diff_applied(stream: &SemioVideoStream, diff: &SemioVideoStreamDiff) -> SemioVideoStream {
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
        (Some(sa), Some(sb)) => Some(absorb_indexed(sa, sb, absorb_sample_diff, sample_with_diff_applied)),
    };
    a
}
//#endregion 🔖️VideoDiffLogic

//#region 🔖️Apply
impl MutationDiff<SemioVideoSnapshot> for SemioVideoDiff {
    fn apply(&self, base: &SemioVideoSnapshot) -> protocol::MutationApplyResult<SemioVideoSnapshot> {
        let mut next = base.clone();
        if let Some(d) = &self.streams {
            crate::artifacts::semio::standards::v1::subsets::base::schema::triples::validate_indexed_triple(d, next.streams.len(), ["streams"])?;
            apply_indexed(&mut next.streams, d, apply_stream);
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        self.streams = match (self.streams.take(), other.streams) {
            (None, x) => x,
            (x, None) => x,
            (Some(a), Some(b)) => Some(absorb_indexed(a, b, absorb_stream_diff, stream_with_diff_applied)),
        };
    }
}
//#endregion 🔖️Apply

//#region 🔖️DiffAlgebra
impl DiffAlgebra<SemioVideoSnapshot> for SemioVideoDiff {
    fn inverse(&self, base: &SemioVideoSnapshot) -> Self {
        SemioVideoDiff { streams: self.streams.as_ref().map(|d| inverse_indexed(&base.streams, d, inverse_stream)) }
    }

    fn between(base: &SemioVideoSnapshot, other: &SemioVideoSnapshot) -> Self {
        diff_video(base, other)
    }

    fn is_empty(&self) -> bool {
        self.streams.is_none()
    }
}
//#endregion 🔖️DiffAlgebra

//#region 🔖️SetSnapshot
/// 🧩 Builds the sparse field-by-field diff for a `SetSnapshot` mutation. No `snapshot:
/// Option<SemioVideoSnapshot>` full-replace slot -- this IS `SemioVideoDiff::between`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_snapshot(base: &SemioVideoSnapshot, next: &SemioVideoSnapshot) -> SemioVideoDiff {
    SemioVideoDiff::between(base, next)
}

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
//#endregion 🔖️SetSnapshot

//#region 🔖️HandcraftedDiffCodec
/// 🎙️ Hand-rolled `protocol::DiffCodec` — same grammar style `GifDiff`/`SvgDiff`/`DocxDiff`'s
/// hand-rolled codecs use (bracket-depth-aware split via the shared `engine::triples` helpers,
/// hex for strings/bytes, `[0]`/`[1,x]` for `Option<T>`). This file's own `enc_indexed_triple`/
/// `dec_indexed_triple` (reused from `engine::triples`, NOT redefined) let the codec stay generic
/// over BOTH nesting levels (`streams`, and within a modified stream, `samples`) — one generic
/// pair, reused twice, instead of two bespoke per-collection encoders.
//#region 🔖️Primitives
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_bool(b: &bool) -> String {
    if *b {
        "1".to_string()
    } else {
        "0".to_string()
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_bool(s: &str) -> Result<bool, String> {
    match s {
        "1" => Ok(true),
        "0" => Ok(false),
        other => Err(format!("bool: bad value {other:?}")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_usize(s: &str) -> Result<usize, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn encode_option<T>(opt: &Option<T>, enc: impl Fn(&T) -> String) -> String {
    match opt {
        None => "[0]".to_string(),
        Some(v) => format!("[1,{}]", enc(v)),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn decode_option<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Option<T>, String> {
    let inner = strip_brackets(s)?;
    match split_top_level(inner, ',').as_slice() {
        ["0"] => Ok(None),
        [tag, value] if *tag == "1" => Ok(Some(dec(value)?)),
        other => Err(format!("option decode: bad shape {other:?}")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_list<T>(items: &[T], enc: impl Fn(&T) -> String) -> String {
    format!("[{}]", items.iter().map(|i| enc(i)).collect::<Vec<_>>().join(","))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_list<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Vec<T>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec).collect()
}
//#endregion 🔖️Primitives

//#region 🔖️ValueCodecs
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_kind(k: &SemioVideoStreamKind) -> String {
    match k {
        SemioVideoStreamKind::Video => "V",
        SemioVideoStreamKind::Audio => "A",
        SemioVideoStreamKind::Subtitle => "S",
    }
    .to_string()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_kind(s: &str) -> Result<SemioVideoStreamKind, String> {
    match s {
        "V" => Ok(SemioVideoStreamKind::Video),
        "A" => Ok(SemioVideoStreamKind::Audio),
        "S" => Ok(SemioVideoStreamKind::Subtitle),
        other => Err(format!("stream kind: bad value {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_rational(r: &SemioRational) -> String {
    format!("[{},{}]", r.num, r.den)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_rational(s: &str) -> Result<SemioRational, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [num, den] = parts.as_slice() else { return Err(format!("rational: expected 2 fields, got {}", parts.len())) };
    Ok(SemioRational { num: num.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, den: den.parse().map_err(|e: std::num::ParseIntError| e.to_string())? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_sample(s: &SemioVideoSample) -> String {
    format!("[{},{},{}]", s.pts, enc_bool(&s.key), hex_encode(&s.data))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_sample(s: &str) -> Result<SemioVideoSample, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [pts, key, data] = parts.as_slice() else { return Err(format!("sample: expected 3 fields, got {}", parts.len())) };
    Ok(SemioVideoSample { pts: pts.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, key: dec_bool(key)?, data: hex_decode(data)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_stream(s: &SemioVideoStream) -> String {
    format!("[{},{},{},{},{},{}]", enc_kind(&s.kind), enc_str(&s.codec), s.width, s.height, enc_rational(&s.rate), enc_list(&s.samples, enc_sample))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_stream(s: &str) -> Result<SemioVideoStream, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [kind, codec, width, height, rate, samples] = parts.as_slice() else { return Err(format!("stream: expected 6 fields, got {}", parts.len())) };
    Ok(SemioVideoStream {
        kind: dec_kind(kind)?,
        codec: dec_str(codec)?,
        width: width.parse().map_err(|e: std::num::ParseIntError| e.to_string())?,
        height: height.parse().map_err(|e: std::num::ParseIntError| e.to_string())?,
        rate: dec_rational(rate)?,
        samples: dec_list(samples, dec_sample)?,
    })
}
//#endregion 🔖️ValueCodecs

//#region 🔖️DiffValueCodecs
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_sample_diff(d: &SemioVideoSampleDiff) -> String {
    format!("[{},{},{}]", encode_option(&d.pts, |v| v.to_string()), encode_option(&d.key, |v| enc_bool(v)), encode_option(&d.data, |v| hex_encode(v)))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_sample_diff(s: &str) -> Result<SemioVideoSampleDiff, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [pts, key, data] = parts.as_slice() else { return Err(format!("sample diff: expected 3 fields, got {}", parts.len())) };
    Ok(SemioVideoSampleDiff { pts: decode_option(pts, |v| v.parse().map_err(|e: std::num::ParseIntError| e.to_string()))?, key: decode_option(key, dec_bool)?, data: decode_option(data, hex_decode)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_samples_diff(d: &SemioVideoSamplesDiff) -> String {
    enc_indexed_triple(d, enc_sample_diff, enc_sample)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_samples_diff(s: &str) -> Result<SemioVideoSamplesDiff, String> {
    dec_indexed_triple(s, dec_sample_diff, dec_sample)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_stream_diff(d: &SemioVideoStreamDiff) -> String {
    format!(
        "[{},{},{},{},{},{}]",
        encode_option(&d.kind, |v| enc_kind(v)),
        encode_option(&d.codec, |v| enc_str(v)),
        encode_option(&d.width, |v| v.to_string()),
        encode_option(&d.height, |v| v.to_string()),
        encode_option(&d.rate, |v| enc_rational(v)),
        encode_option(&d.samples, |v| enc_samples_diff(v)),
    )
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_stream_diff(s: &str) -> Result<SemioVideoStreamDiff, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [kind, codec, width, height, rate, samples] = parts.as_slice() else { return Err(format!("stream diff: expected 6 fields, got {}", parts.len())) };
    Ok(SemioVideoStreamDiff {
        kind: decode_option(kind, dec_kind)?,
        codec: decode_option(codec, dec_str)?,
        width: decode_option(width, |v| v.parse().map_err(|e: std::num::ParseIntError| e.to_string()))?,
        height: decode_option(height, |v| v.parse().map_err(|e: std::num::ParseIntError| e.to_string()))?,
        rate: decode_option(rate, dec_rational)?,
        samples: decode_option(samples, dec_samples_diff)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_streams_diff(d: &SemioVideoStreamsDiff) -> String {
    enc_indexed_triple(d, enc_stream_diff, enc_stream)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_streams_diff(s: &str) -> Result<SemioVideoStreamsDiff, String> {
    dec_indexed_triple(s, dec_stream_diff, dec_stream)
}
//#endregion 🔖️DiffValueCodecs

//#region 🔖️TopLevel
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_semio_video_diff(d: &SemioVideoDiff) -> String {
    match &d.streams {
        Some(v) => format!("streams={}", enc_streams_diff(v)),
        None => String::new(),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_semio_video_diff(line: &str) -> Result<SemioVideoDiff, String> {
    if line.is_empty() {
        return Ok(SemioVideoDiff::default());
    }
    let rest = line.strip_prefix("streams=").ok_or_else(|| format!("semio video diff: unknown token {line:?}"))?;
    Ok(SemioVideoDiff { streams: Some(dec_streams_diff(rest)?) })
}

/// 🧪️ Real LEB128-varint-length-prefixed binary primitives (`store::pack_rt::write_varint_u64` /
/// `store::ByteReader`, same helpers flow's/mesh's upgraded diff facets reuse) backing the
/// real `DiffCodec::encode_diff`/`decode_diff` below.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    store::pack_rt::write_varint_u64(out, s.len() as u64);
    out.extend_from_slice(s.as_bytes());
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    String::from_utf8(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec()).map_err(|e| e.to_string())
}

impl protocol::DiffCodec for SemioVideoDiff {
    fn print_diff(&self) -> String {
        print_semio_video_diff(self)
    }
    fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        parse_semio_video_diff(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    /// ⚡️ Real binary diff frame, replacing the old `print_diff().into_bytes()` text-as-binary
    /// shortcut (same treatment flow's/mesh's own upgraded diff facets use). `format u8` +
    /// `presence u8` (bit0 = `streams` present) are two REAL fixed fields; when present, `streams`
    /// follows as one varint-length-prefixed opaque blob (the same `enc_streams_diff` bracket/hex
    /// text `print_diff` already emits) — a length-prefixed segment rather than a bare trailing
    /// `bytes` chain so the shape stays uniform with flow's/mesh's multi-field diff frames
    /// (`protocol-cond-cannot-chain`: a second `if`-guard on a field that's itself only
    /// conditionally decoded hard-errors `eval_cond` — see this wave's report).
    fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const DIFF_BINARY_FORMAT: u8 = 1;
        let presence: u8 = if self.streams.is_some() { 0b01 } else { 0b00 };
        let mut out = vec![DIFF_BINARY_FORMAT, presence];
        if let Some(v) = &self.streams {
            write_str_lp(&mut out, &enc_streams_diff(v));
        }
        Ok(out)
    }
    fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const DIFF_BINARY_FORMAT: u8 = 1;
        if bytes.len() < 2 {
            return Err(protocol::ProtocolError::Malformed { what: "diff header", offset: 0, detail: "truncated (need format+presence)".to_string() });
        }
        if bytes[0] != DIFF_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "diff format", offset: 0, detail: format!("unsupported diff format {}", bytes[0]) });
        }
        let presence = bytes[1];
        let mut reader = store::ByteReader::new(&bytes[2..]);
        let streams = if presence & 0b01 != 0 {
            let text = read_str_lp(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff streams blob", offset: 2, detail: e })?;
            Some(dec_streams_diff(&text).map_err(|e| protocol::ProtocolError::Malformed { what: "diff streams text", offset: 2, detail: e })?)
        } else {
            None
        };
        Ok(SemioVideoDiff { streams })
    }
}

//#region 🔖️Demo
/// 🌱 Representative `SemioVideoDiff` cases — the empty (no-op) diff, `a→b`, and `b→a` over
/// `snapshot_a()`/`snapshot_b()` — covering `streams.removed`/`.modified`/`.added` AND, within a
/// modified stream, nested `samples.removed`/`.modified`/`.added` (both directions combined).
/// `pub(crate)` module-scope so `🎹️composer/🦀️.rs`'s conformance-law tests can reuse it —
/// same convention flow's/mesh's own `demo_diff_cases()` use.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<SemioVideoDiff> {
    let a = handcrafted_diff_codec_tests::snapshot_a();
    let b = handcrafted_diff_codec_tests::snapshot_b();
    vec![SemioVideoDiff::default(), SemioVideoDiff::between(&a, &b), SemioVideoDiff::between(&b, &a)]
}
//#endregion 🔖️Demo
//#endregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec

//#region 🧪️Tests
#[cfg(test)]
mod handcrafted_diff_codec_tests {
    use super::*;
    use protocol::DiffCodec;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn snapshot_a() -> SemioVideoSnapshot {
        SemioVideoSnapshot {
            schema: crate::artifacts::semio::standards::v1::subsets::video::schema::snapshot::STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA.into(),
            streams: vec![
                SemioVideoStream {
                    kind: SemioVideoStreamKind::Video,
                    codec: "avc-old".into(),
                    width: 640,
                    height: 480,
                    rate: SemioRational { num: 24, den: 1 },
                    samples: vec![SemioVideoSample { pts: 0, key: true, data: vec![9] }, SemioVideoSample { pts: 1, key: false, data: vec![8] }, SemioVideoSample { pts: 2, key: true, data: vec![7] }],
                },
                SemioVideoStream { kind: SemioVideoStreamKind::Audio, codec: "aac".into(), width: 0, height: 0, rate: SemioRational { num: 1, den: 1 }, samples: Vec::new() },
                SemioVideoStream { kind: SemioVideoStreamKind::Subtitle, codec: "srt".into(), width: 0, height: 0, rate: SemioRational { num: 1, den: 1 }, samples: Vec::new() },
            ],
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn snapshot_b() -> SemioVideoSnapshot {
        SemioVideoSnapshot {
            schema: crate::artifacts::semio::standards::v1::subsets::video::schema::snapshot::STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA.into(),
            streams: vec![
                SemioVideoStream {
                    kind: SemioVideoStreamKind::Audio,
                    codec: "new-codec".into(),
                    width: 1280,
                    height: 720,
                    rate: SemioRational { num: 30, den: 1 },
                    samples: vec![SemioVideoSample { pts: 0, key: true, data: vec![9] }, SemioVideoSample { pts: 22, key: true, data: vec![80] }],
                },
                SemioVideoStream { kind: SemioVideoStreamKind::Audio, codec: "aac".into(), width: 0, height: 0, rate: SemioRational { num: 1, den: 1 }, samples: Vec::new() },
            ],
        }
    }

    /// 🧪️ `diff_codec_text_binary_roundtrip_law`: print/parse and encode/decode round-trip over
    /// the hand-rolled `SemioVideoDiff` grammar — exercises `streams.removed`/`.modified`/`.added`
    /// AND, within the same modified stream, nested `samples.removed`/`.modified` (reverse
    /// direction additionally exercises nested `samples.added`), a `SemioVideoStreamKind` enum
    /// change, and every stream-level scalar field.
    #[semio_framework_async_macros::async_test]
    async fn diff_codec_text_binary_roundtrip_law() {
        let a = snapshot_a();
        let b = snapshot_b();
        let cases = vec![SemioVideoDiff::default(), SemioVideoDiff::between(&a, &b), SemioVideoDiff::between(&b, &a), SemioVideoDiff::between(&a, &a)];
        for d in cases {
            let printed = d.print_diff();
            assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
            let parsed = SemioVideoDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
            assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

            let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
            let decoded = SemioVideoDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
            assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
        }

        // Field sweep proof: confirm every collection flavor actually got exercised above.
        let diff_ab = SemioVideoDiff::between(&a, &b);
        let streams_diff = diff_ab.streams.as_ref().expect("streams diff present");
        assert!(!streams_diff.removed.is_empty(), "streams: removed not exercised");
        assert_eq!(streams_diff.modified.len(), 1);
        let stream_mod = &streams_diff.modified[0].diff;
        assert!(stream_mod.kind.is_some() && stream_mod.codec.is_some() && stream_mod.width.is_some() && stream_mod.height.is_some() && stream_mod.rate.is_some(), "modified stream: not every scalar field exercised");
        let samples_diff = stream_mod.samples.as_ref().expect("nested samples diff present");
        assert!(!samples_diff.removed.is_empty(), "samples: removed not exercised");
        assert!(!samples_diff.modified.is_empty(), "samples: modified not exercised");

        let diff_ba = SemioVideoDiff::between(&b, &a);
        let streams_diff_ba = diff_ba.streams.as_ref().expect("streams diff (b->a) present");
        assert!(!streams_diff_ba.added.is_empty(), "streams (b->a): added not exercised");
        let stream_mod_ba = &streams_diff_ba.modified[0].diff;
        let samples_diff_ba = stream_mod_ba.samples.as_ref().expect("nested samples diff (b->a) present");
        assert!(!samples_diff_ba.added.is_empty(), "samples (b->a): added not exercised");
    }
}
//#endregion 🧪️Tests
