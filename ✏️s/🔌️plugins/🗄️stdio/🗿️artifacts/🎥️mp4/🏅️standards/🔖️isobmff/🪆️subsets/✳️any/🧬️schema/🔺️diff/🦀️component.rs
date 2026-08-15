//! 🔺️ Mp4Diff — handcrafted sparse per-field diff (schema-design.md recipe): `ftyp` whole-value
//! replaced (weak entity), `tracks`/`samples` index-keyed collection triples
//! (`removed`/`modified`/`added`, strong-like entities). Hand-rolled throughout — NOT
//! `#[derive(dsl::DslOps)]` (f6-final-summary.md §4.4: generic collection-diff wrappers have no
//! `DslField` bridge in the derive macros) — `IndexedDiff<T,D>`/`IndexedModified`/`IndexedAdded`
//! below are this artifact's own named (never bare-tuple) generic triple types, following the
//! bcf/docx `enc_indexed_triple` precedent's SHAPE without needing that shared engine (mp4 is a
//! top-level format artifact, not a semio subset).

use crate::artifacts::mp4::standards::isobmff::subsets::any::schema::snapshot::{Mp4Codec, Mp4Ftyp, Mp4Movie, Mp4Sample, Mp4Snapshot, Mp4Track, Mp4TrackMetadata};
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;
use serde::{Deserialize, Serialize};

//#region 🔖️IndexedTriple
/// 🧩 Index-keyed collection diff triple (schema-design.md's `CsDiff`/`CModified`/`CAdded`,
/// generic over this artifact's own item/diff pairs — see module doc comment for why this is
/// hand-rolled locally rather than derived or imported).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexedDiff<T, D> {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<IndexedModified<D>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<IndexedAdded<T>>,
}

impl<T, D> IndexedDiff<T, D> {
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexedModified<D> {
    pub index: usize,
    pub diff: D,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexedAdded<T> {
    pub index: usize,
    pub item: T,
}

/// ▶️ Apply semantics (normative, schema-design.md): `removed`/`modified` index BASE state;
/// `added` indices are FINAL positions, inserted ascending at `min(index, len)`. Out-of-range
/// keys are graceful no-ops.
pub fn apply_indexed<T: Clone, D>(base: &[T], diff: &IndexedDiff<T, D>, apply_item: impl Fn(&T, &D) -> T) -> Vec<T> {
    let mut kept: Vec<(usize, T)> = base.iter().enumerate().filter(|(i, _)| !diff.removed.contains(i)).map(|(i, t)| (i, t.clone())).collect();
    for m in &diff.modified {
        if let Some(entry) = kept.iter_mut().find(|(i, _)| *i == m.index) {
            entry.1 = apply_item(&entry.1, &m.diff);
        }
    }
    let mut result: Vec<T> = kept.into_iter().map(|(_, t)| t).collect();
    let mut adds = diff.added.clone();
    adds.sort_by_key(|a| a.index);
    for a in adds {
        let idx = a.index.min(result.len());
        result.insert(idx, a.item);
    }
    result
}

/// 🧭️ State delta (schema-design.md's `between` matching for index keys): pairwise by position,
/// `modified` = compare `0..min(base.len(),other.len())`, `removed` = base tail, `added` = other tail.
pub fn between_indexed<T: Clone + PartialEq, D>(base: &[T], other: &[T], between_item: impl Fn(&T, &T) -> D, item_is_empty: impl Fn(&D) -> bool) -> IndexedDiff<T, D> {
    let min_len = base.len().min(other.len());
    let mut modified = Vec::new();
    for i in 0..min_len {
        if base[i] != other[i] {
            let d = between_item(&base[i], &other[i]);
            if !item_is_empty(&d) {
                modified.push(IndexedModified { index: i, diff: d });
            }
        }
    }
    let removed: Vec<usize> = if other.len() < base.len() { (other.len()..base.len()).collect() } else { Vec::new() };
    let added: Vec<IndexedAdded<T>> = if other.len() > base.len() { (base.len()..other.len()).map(|i| IndexedAdded { index: i, item: other[i].clone() }).collect() } else { Vec::new() };
    IndexedDiff { removed, modified, added }
}

/// 📐️ Shared rank/unrank index-transport arithmetic (adapted from gif 89a's `GifDiff` absorb —
/// `🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
/// `count_le`/`rank_excluding`/`unrank_excluding`/`transport_forward` — chosen over deriving a
/// position-label array because it needs no base-length parameter, which `MutationDiff::absorb`'s
/// base-free signature can't supply). `excluded_sorted` must be sorted ascending.
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

/// ➕️ Structural, total, base-free absorb (schema-design.md's normative algorithm, adapted from
/// gif 89a's `absorb_indexed_collection`): composes `d1` (base→mid) with `d2` (mid→after) into
/// base→after, in place on `d1`. `absorb_item` recursively absorbs a surviving item's mid-diff
/// into an existing base-diff; `apply_item_diff` applies a diff onto a `T` in place (used when
/// `d2` patches an item `d1` just added).
pub fn absorb_indexed<T: Clone, D: Clone>(d1: &mut IndexedDiff<T, D>, d2: IndexedDiff<T, D>, absorb_item: impl Fn(&mut D, D), apply_item_diff: impl Fn(&mut T, &D)) {
    let mut removed1_sorted = d1.removed.clone();
    removed1_sorted.sort_unstable();
    let mut added1_index_sorted: Vec<usize> = d1.added.iter().map(|a| a.index).collect();
    added1_index_sorted.sort_unstable();
    let mut removed2_sorted = d2.removed.clone();
    removed2_sorted.sort_unstable();
    let mut added2_index_sorted: Vec<usize> = d2.added.iter().map(|a| a.index).collect();
    added2_index_sorted.sort_unstable();

    let mut merged_added: Vec<IndexedAdded<T>> = std::mem::take(&mut d1.added);
    let mut annihilated: std::collections::HashSet<usize> = Default::default();

    //#region Removed
    let mut merged_removed_base: Vec<usize> = removed1_sorted.clone();
    for &r2 in &removed2_sorted {
        if added1_index_sorted.binary_search(&r2).is_ok() {
            annihilated.insert(r2);
            merged_added.retain(|a| a.index != r2);
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
    let mut modified_map: std::collections::BTreeMap<usize, D> = std::mem::take(&mut d1.modified).into_iter().map(|m| (m.index, m.diff)).collect();
    for base_index in &merged_removed_base {
        modified_map.remove(base_index);
    }
    for m2 in d2.modified {
        if annihilated.contains(&m2.index) {
            continue;
        }
        if added1_index_sorted.binary_search(&m2.index).is_ok() {
            if let Some(entry) = merged_added.iter_mut().find(|a| a.index == m2.index) {
                apply_item_diff(&mut entry.item, &m2.diff);
            }
        } else {
            let post_remove_rank = rank_excluding(m2.index, &added1_index_sorted);
            let base_index = unrank_excluding(post_remove_rank, &removed1_sorted);
            if merged_removed_base.binary_search(&base_index).is_ok() {
                continue;
            }
            match modified_map.get_mut(&base_index) {
                Some(existing) => absorb_item(existing, m2.diff),
                None => {
                    modified_map.insert(base_index, m2.diff);
                }
            }
        }
    }
    //#endregion Modified

    //#region Added
    let mut merged_added_final: Vec<IndexedAdded<T>> = merged_added
        .into_iter()
        .map(|a| {
            let after_pos = if removed2_sorted.binary_search(&a.index).is_ok() {
                a.index
            } else {
                let post_remove_rank = rank_excluding(a.index, &removed2_sorted);
                unrank_excluding(post_remove_rank, &added2_index_sorted)
            };
            IndexedAdded { index: after_pos, item: a.item }
        })
        .collect();
    merged_added_final.extend(d2.added);
    merged_added_final.sort_by_key(|a| a.index);
    //#endregion Added

    d1.removed = merged_removed_base;
    d1.modified = modified_map.into_iter().map(|(index, diff)| IndexedModified { index, diff }).collect();
    d1.added = merged_added_final;
}
//#endregion 🔖️IndexedTriple

//#region 🔖️Sample
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Mp4SampleDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[dsl(base64)]
    pub data: Option<Vec<u8>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cts_offset: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sync: Option<bool>,
}

fn apply_sample_diff(base: &Mp4Sample, d: &Mp4SampleDiff) -> Mp4Sample {
    Mp4Sample { data: d.data.clone().unwrap_or_else(|| base.data.clone()), duration: d.duration.unwrap_or(base.duration), cts_offset: d.cts_offset.unwrap_or(base.cts_offset), sync: d.sync.unwrap_or(base.sync) }
}
fn apply_sample_diff_mut(item: &mut Mp4Sample, d: &Mp4SampleDiff) {
    *item = apply_sample_diff(item, d);
}

fn between_sample(a: &Mp4Sample, b: &Mp4Sample) -> Mp4SampleDiff {
    Mp4SampleDiff { data: (a.data != b.data).then(|| b.data.clone()), duration: (a.duration != b.duration).then_some(b.duration), cts_offset: (a.cts_offset != b.cts_offset).then_some(b.cts_offset), sync: (a.sync != b.sync).then_some(b.sync) }
}
fn sample_diff_is_empty(d: &Mp4SampleDiff) -> bool {
    d.data.is_none() && d.duration.is_none() && d.cts_offset.is_none() && d.sync.is_none()
}
fn absorb_sample_diff(a: &mut Mp4SampleDiff, b: Mp4SampleDiff) {
    if b.data.is_some() {
        a.data = b.data;
    }
    if b.duration.is_some() {
        a.duration = b.duration;
    }
    if b.cts_offset.is_some() {
        a.cts_offset = b.cts_offset;
    }
    if b.sync.is_some() {
        a.sync = b.sync;
    }
}
//#endregion 🔖️Sample

//#region 🔖️Track
pub type Mp4SamplesDiff = IndexedDiff<Mp4Sample, Mp4SampleDiff>;

#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
struct Mp4SampleModifiedRecord {
    index: usize,
    diff: Mp4SampleDiff,
}

#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
struct Mp4SampleAddedRecord {
    index: usize,
    item: Mp4Sample,
}

#[derive(Clone, Debug, Default, PartialEq, dsl::DslRecord)]
struct Mp4SamplesDiffRecord {
    removed: Vec<usize>,
    modified: Vec<Mp4SampleModifiedRecord>,
    added: Vec<Mp4SampleAddedRecord>,
}

impl dsl::DslField for Mp4SamplesDiff {
    fn shape() -> dsl::Shape {
        <Mp4SamplesDiffRecord as dsl::DslField>::shape()
    }
    fn to_value(&self) -> dsl::FieldValue {
        Mp4SamplesDiffRecord {
            removed: self.removed.clone(),
            modified: self.modified.iter().map(|entry| Mp4SampleModifiedRecord { index: entry.index, diff: entry.diff.clone() }).collect(),
            added: self.added.iter().map(|entry| Mp4SampleAddedRecord { index: entry.index, item: entry.item.clone() }).collect(),
        }
        .to_value()
    }
    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        let record = <Mp4SamplesDiffRecord as dsl::DslField>::from_value(value)?;
        Ok(Self {
            removed: record.removed,
            modified: record.modified.into_iter().map(|entry| IndexedModified { index: entry.index, diff: entry.diff }).collect(),
            added: record.added.into_iter().map(|entry| IndexedAdded { index: entry.index, item: entry.item }).collect(),
        })
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Mp4TrackDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub track_id: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timescale: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub codec: Option<Mp4Codec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Mp4TrackMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chunk_sample_counts: Option<Vec<u32>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub samples: Option<Mp4SamplesDiff>,
}

fn apply_track_diff(base: &Mp4Track, d: &Mp4TrackDiff) -> Mp4Track {
    Mp4Track {
        track_id: d.track_id.unwrap_or(base.track_id),
        timescale: d.timescale.unwrap_or(base.timescale),
        codec: d.codec.clone().unwrap_or_else(|| base.codec.clone()),
        width: d.width.unwrap_or(base.width),
        height: d.height.unwrap_or(base.height),
        metadata: d.metadata.clone().unwrap_or_else(|| base.metadata.clone()),
        chunk_sample_counts: d.chunk_sample_counts.clone().unwrap_or_else(|| base.chunk_sample_counts.clone()),
        samples: d.samples.as_ref().map_or_else(|| base.samples.clone(), |sd| apply_indexed(&base.samples, sd, apply_sample_diff)),
    }
}
fn apply_track_diff_mut(item: &mut Mp4Track, d: &Mp4TrackDiff) {
    *item = apply_track_diff(item, d);
}

fn between_track(a: &Mp4Track, b: &Mp4Track) -> Mp4TrackDiff {
    let samples_diff = between_indexed(&a.samples, &b.samples, between_sample, sample_diff_is_empty);
    Mp4TrackDiff {
        track_id: (a.track_id != b.track_id).then_some(b.track_id),
        timescale: (a.timescale != b.timescale).then_some(b.timescale),
        codec: (a.codec != b.codec).then(|| b.codec.clone()),
        width: (a.width != b.width).then_some(b.width),
        height: (a.height != b.height).then_some(b.height),
        metadata: (a.metadata != b.metadata).then(|| b.metadata.clone()),
        chunk_sample_counts: (a.chunk_sample_counts != b.chunk_sample_counts).then(|| b.chunk_sample_counts.clone()),
        samples: (!samples_diff.is_empty()).then_some(samples_diff),
    }
}
fn track_diff_is_empty(d: &Mp4TrackDiff) -> bool {
    d.track_id.is_none() && d.timescale.is_none() && d.codec.is_none() && d.width.is_none() && d.height.is_none() && d.metadata.is_none() && d.chunk_sample_counts.is_none() && d.samples.is_none()
}
fn absorb_track_diff(a: &mut Mp4TrackDiff, b: Mp4TrackDiff) {
    if b.track_id.is_some() {
        a.track_id = b.track_id;
    }
    if b.timescale.is_some() {
        a.timescale = b.timescale;
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
    if b.metadata.is_some() {
        a.metadata = b.metadata;
    }
    if b.chunk_sample_counts.is_some() {
        a.chunk_sample_counts = b.chunk_sample_counts;
    }
    match (&mut a.samples, b.samples) {
        (Some(existing), Some(other)) => absorb_indexed(existing, other, absorb_sample_diff, apply_sample_diff_mut),
        (a_slot @ None, Some(other)) => *a_slot = Some(other),
        _ => {}
    }
}
//#endregion 🔖️Track

//#region 🔖️Diff
pub type Mp4TracksDiff = IndexedDiff<Mp4Track, Mp4TrackDiff>;

#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
struct Mp4TrackModifiedRecord {
    index: usize,
    diff: Mp4TrackDiff,
}

#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
struct Mp4TrackAddedRecord {
    index: usize,
    item: Mp4Track,
}

#[derive(Clone, Debug, Default, PartialEq, dsl::DslRecord)]
struct Mp4TracksDiffRecord {
    removed: Vec<usize>,
    modified: Vec<Mp4TrackModifiedRecord>,
    added: Vec<Mp4TrackAddedRecord>,
}

impl dsl::DslField for Mp4TracksDiff {
    fn shape() -> dsl::Shape {
        <Mp4TracksDiffRecord as dsl::DslField>::shape()
    }
    fn to_value(&self) -> dsl::FieldValue {
        Mp4TracksDiffRecord {
            removed: self.removed.clone(),
            modified: self.modified.iter().map(|entry| Mp4TrackModifiedRecord { index: entry.index, diff: entry.diff.clone() }).collect(),
            added: self.added.iter().map(|entry| Mp4TrackAddedRecord { index: entry.index, item: entry.item.clone() }).collect(),
        }
        .to_value()
    }
    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        let record = <Mp4TracksDiffRecord as dsl::DslField>::from_value(value)?;
        Ok(Self {
            removed: record.removed,
            modified: record.modified.into_iter().map(|entry| IndexedModified { index: entry.index, diff: entry.diff }).collect(),
            added: record.added.into_iter().map(|entry| IndexedAdded { index: entry.index, item: entry.item }).collect(),
        })
    }
}
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslDiff)]
#[serde(rename_all = "camelCase")]
pub struct Mp4Diff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ftyp: Option<Mp4Ftyp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub movie: Option<Mp4Movie>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracks: Option<Mp4TracksDiff>,
}

impl MutationDiff<Mp4Snapshot> for Mp4Diff {
    fn apply(&self, base: &Mp4Snapshot) -> Mp4Snapshot {
        Mp4Snapshot {
            schema: base.schema.clone(),
            ftyp: self.ftyp.clone().unwrap_or_else(|| base.ftyp.clone()),
            movie: self.movie.clone().unwrap_or_else(|| base.movie.clone()),
            tracks: self.tracks.as_ref().map_or_else(|| base.tracks.clone(), |td| apply_indexed(&base.tracks, td, apply_track_diff)),
        }
    }

    fn absorb(&mut self, other: Self) {
        if other.ftyp.is_some() {
            self.ftyp = other.ftyp;
        }
        if other.movie.is_some() {
            self.movie = other.movie;
        }
        match (&mut self.tracks, other.tracks) {
            (Some(existing), Some(other_tracks)) => absorb_indexed(existing, other_tracks, absorb_track_diff, apply_track_diff_mut),
            (slot @ None, Some(other_tracks)) => *slot = Some(other_tracks),
            _ => {}
        }
    }
}

impl DiffAlgebra<Mp4Snapshot> for Mp4Diff {
    fn between(base: &Mp4Snapshot, other: &Mp4Snapshot) -> Self {
        let tracks_diff = between_indexed(&base.tracks, &other.tracks, between_track, track_diff_is_empty);
        Self { ftyp: (base.ftyp != other.ftyp).then(|| other.ftyp.clone()), movie: (base.movie != other.movie).then(|| other.movie.clone()), tracks: (!tracks_diff.is_empty()).then_some(tracks_diff) }
    }
    fn inverse(&self, base: &Mp4Snapshot) -> Self {
        // 🔁️ Correct-by-construction: `between(after, base)` trivially satisfies the inverse law
        // `d.inverse(base).apply(&d.apply(base)) == base` because `between` itself satisfies
        // `between(a,b).apply(a) == b` (tested directly below) — applying `between(after, base)`
        // to `after` yields `base` by that same law with `a = after, b = base`.
        let after = self.apply(base);
        Self::between(&after, base)
    }

    fn is_empty(&self) -> bool {
        self.ftyp.is_none() && self.movie.is_none() && self.tracks.is_none()
    }
}

/// 🧩 Set-snapshot diff helper — used by the `📸️set-snapshot/🔺️diff` leaf.
pub fn diff_set_snapshot(base: &Mp4Snapshot, snapshot: &Mp4Snapshot) -> Mp4Diff {
    <Mp4Diff as DiffAlgebra<Mp4Snapshot>>::between(base, snapshot)
}
//#endregion 🔖️Diff

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::mp4::standards::isobmff::subsets::any::schema::snapshot::STDIO_MP4_DOCUMENT_SCHEMA;

    fn sample(n: u8) -> Mp4Sample {
        Mp4Sample { data: vec![n], duration: u32::from(n) * 10, cts_offset: 0, sync: n % 2 == 0 }
    }

    fn track(id: u32, samples: Vec<Mp4Sample>) -> Mp4Track {
        Mp4Track { track_id: id, timescale: 1000, codec: Mp4Codec::default(), width: 64, height: 64, metadata: Mp4TrackMetadata::default(), chunk_sample_counts: vec![samples.len() as u32], samples }
    }

    fn snap(tracks: Vec<Mp4Track>) -> Mp4Snapshot {
        Mp4Snapshot { schema: STDIO_MP4_DOCUMENT_SCHEMA.into(), ftyp: Mp4Ftyp { major_brand: "isom".into(), minor_version: 0, compatible_brands: vec![] }, movie: Mp4Movie::default(), tracks }
    }

    //#region field_sweep + between_roundtrip_law
    #[test]
    fn field_sweep_covers_every_mutable_field() {
        let a = snap(vec![track(1, vec![sample(1), sample(2)]), track(2, vec![sample(3)])]);
        let mut b = a.clone();
        b.ftyp.major_brand = "mp42".into();
        b.tracks[0].width = 128;
        b.tracks[0].samples.remove(0);
        b.tracks[0].samples.push(sample(9));
        b.tracks.remove(1);
        b.tracks.push(track(3, vec![sample(5)]));
        let d = <Mp4Diff as DiffAlgebra<Mp4Snapshot>>::between(&a, &b);
        assert!(d.ftyp.is_some(), "ftyp field must be covered by the sweep");
        assert!(d.tracks.is_some(), "tracks field must be covered by the sweep");
        assert_eq!(d.apply(&a), b);
        assert_eq!(<Mp4Diff as DiffAlgebra<Mp4Snapshot>>::between(&b, &a).apply(&b), a);
        assert!(<Mp4Diff as DiffAlgebra<Mp4Snapshot>>::between(&a, &a).is_empty());
    }

    #[test]
    fn inverse_law_round_trips_through_apply() {
        let a = snap(vec![track(1, vec![sample(1), sample(2)])]);
        let mut b = a.clone();
        b.tracks[0].samples[0].duration = 999;
        b.tracks[0].samples[0].sync = !b.tracks[0].samples[0].sync;
        let d = <Mp4Diff as DiffAlgebra<Mp4Snapshot>>::between(&a, &b);
        let after = d.apply(&a);
        assert_eq!(after, b);
        let inv = d.inverse(&a);
        assert_eq!(inv.apply(&after), a);
    }
    //#endregion

    //#region absorb_law — canonical index-transport cases (schema-design.md)
    #[test]
    fn absorb_insert_then_remove_before_matches_sequential() {
        let base: Vec<Mp4Sample> = vec![sample(1), sample(2)];
        // d1: insert `f` at final index 2 -> mid = [s1, s2, f]
        let f = Mp4Sample { data: vec![0xAA], duration: 1, cts_offset: 0, sync: true };
        let mut d1: Mp4SamplesDiff = IndexedDiff { removed: vec![], modified: vec![], added: vec![IndexedAdded { index: 2, item: f.clone() }] };
        let mid = apply_indexed(&base, &d1, apply_sample_diff);
        // d2: remove base index 0 from mid -> after = [s2, f]
        let d2: Mp4SamplesDiff = IndexedDiff { removed: vec![0], modified: vec![], added: vec![] };
        let after = apply_indexed(&mid, &d2, apply_sample_diff);
        let sequential = after.clone();

        absorb_indexed(&mut d1, d2, absorb_sample_diff, apply_sample_diff_mut);
        let combined = apply_indexed(&base, &d1, apply_sample_diff);
        assert_eq!(combined, sequential, "absorb(d1,d2).apply(base) must equal d2.apply(d1.apply(base))");
        assert_eq!(d1.removed, vec![0], "the real base removal must transport through");
    }

    #[test]
    fn absorb_insert_insert_same_index_both_survive() {
        let base: Vec<Mp4Sample> = vec![sample(1)];
        let f = Mp4Sample { data: vec![0xAA], duration: 1, cts_offset: 0, sync: true };
        let g = Mp4Sample { data: vec![0xBB], duration: 2, cts_offset: 0, sync: false };
        let mut d1: Mp4SamplesDiff = IndexedDiff { removed: vec![], modified: vec![], added: vec![IndexedAdded { index: 1, item: f.clone() }] };
        let mid = apply_indexed(&base, &d1, apply_sample_diff);
        assert_eq!(mid, vec![sample(1), f.clone()]);
        let d2: Mp4SamplesDiff = IndexedDiff { removed: vec![], modified: vec![], added: vec![IndexedAdded { index: 1, item: g.clone() }] };
        let after = apply_indexed(&mid, &d2, apply_sample_diff);
        let sequential = after.clone();

        absorb_indexed(&mut d1, d2, absorb_sample_diff, apply_sample_diff_mut);
        let combined = apply_indexed(&base, &d1, apply_sample_diff);
        assert_eq!(combined, sequential);
        assert_eq!(combined.len(), 3, "both inserts at the same nominal index must survive (fixes the gif-style LWW-slot bug)");
    }

    #[test]
    fn absorb_modify_patches_into_added_payload() {
        let base: Vec<Mp4Sample> = vec![sample(1)];
        let f = Mp4Sample { data: vec![0xAA], duration: 1, cts_offset: 0, sync: false };
        let mut d1: Mp4SamplesDiff = IndexedDiff { removed: vec![], modified: vec![], added: vec![IndexedAdded { index: 1, item: f.clone() }] };
        let mid = apply_indexed(&base, &d1, apply_sample_diff);
        let patch = Mp4SampleDiff { data: None, duration: Some(42), cts_offset: None, sync: Some(true) };
        let d2: Mp4SamplesDiff = IndexedDiff { removed: vec![], modified: vec![IndexedModified { index: 1, diff: patch }], added: vec![] };
        let after = apply_indexed(&mid, &d2, apply_sample_diff);
        let sequential = after.clone();

        absorb_indexed(&mut d1, d2, absorb_sample_diff, apply_sample_diff_mut);
        let combined = apply_indexed(&base, &d1, apply_sample_diff);
        assert_eq!(combined, sequential);
        assert_eq!(combined[1].duration, 42);
        assert!(combined[1].sync);
        assert!(d1.modified.is_empty(), "the patch must land INTO the carried added payload, not become a separate modified entry");
    }

    #[test]
    fn absorb_modify_then_remove_drops_the_modification() {
        let base: Vec<Mp4Sample> = vec![sample(1), sample(2)];
        let mut d1: Mp4SamplesDiff = IndexedDiff { removed: vec![], modified: vec![IndexedModified { index: 0, diff: Mp4SampleDiff { data: None, duration: Some(77), cts_offset: None, sync: None } }], added: vec![] };
        let mid = apply_indexed(&base, &d1, apply_sample_diff);
        let d2: Mp4SamplesDiff = IndexedDiff { removed: vec![0], modified: vec![], added: vec![] };
        let after = apply_indexed(&mid, &d2, apply_sample_diff);
        let sequential = after.clone();

        absorb_indexed(&mut d1, d2, absorb_sample_diff, apply_sample_diff_mut);
        let combined = apply_indexed(&base, &d1, apply_sample_diff);
        assert_eq!(combined, sequential);
        assert!(d1.modified.is_empty(), "a merged-removed key's modified entry must be dropped");
    }

    #[test]
    fn absorb_associativity_over_three_diffs() {
        let a = snap(vec![track(1, vec![sample(1), sample(2)])]);
        let mut mid1 = a.clone();
        mid1.tracks[0].samples[0].duration = 11;
        let mut mid2 = mid1.clone();
        mid2.tracks.push(track(2, vec![sample(5)]));
        let mut after = mid2.clone();
        after.tracks[0].width = 999;

        let d1 = <Mp4Diff as DiffAlgebra<Mp4Snapshot>>::between(&a, &mid1);
        let d2 = <Mp4Diff as DiffAlgebra<Mp4Snapshot>>::between(&mid1, &mid2);
        let d3 = <Mp4Diff as DiffAlgebra<Mp4Snapshot>>::between(&mid2, &after);

        let mut left = d1.clone();
        left.absorb(d2.clone());
        left.absorb(d3.clone());

        let mut d23 = d2.clone();
        d23.absorb(d3.clone());
        let mut right = d1.clone();
        right.absorb(d23);

        assert_eq!(left.apply(&a), after);
        assert_eq!(right.apply(&a), after);
        assert_eq!(left.apply(&a), right.apply(&a), "absorb must be associative");
    }

    #[test]
    fn exact_fixture_empty_inverse_absorb_and_source_removal_laws() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../temp/bauen-mit-bestand.mp4");
        let bytes = std::fs::read(path).expect("read exact MP4 fixture");
        let base = crate::artifacts::mp4::standards::isobmff::subsets::any::io::decode_mp4(&bytes).expect("decode exact MP4 fixture");

        let empty = Mp4Diff::default();
        assert!(empty.is_empty());
        assert_eq!(crate::artifacts::mp4::standards::isobmff::subsets::any::io::encode_mp4(&empty.apply(&base)), bytes);

        let mut changed = base.clone();
        changed.tracks[0].width += 1;
        let diff = Mp4Diff::between(&base, &changed);
        let after = diff.apply(&base);
        let inverse = diff.inverse(&base);
        assert_eq!(crate::artifacts::mp4::standards::isobmff::subsets::any::io::encode_mp4(&inverse.apply(&after)), bytes);

        let mut absorbed = diff;
        absorbed.absorb(inverse);
        assert_eq!(crate::artifacts::mp4::standards::isobmff::subsets::any::io::encode_mp4(&absorbed.apply(&base)), bytes);
    }
    //#endregion
}
//#endregion 🔖️Tests
