//! 🔺️ AviDiff — handcrafted sparse per-field diff (schema-design.md recipe): `main_header`
//! whole-value replaced (weak entity), `streams`/`chunks`/`unknown_chunks` index-keyed collection
//! triples (`removed`/`modified`/`added`, strong-like entities). Hand-rolled throughout — NOT
//! `#[derive(dsl::DslOps)]` (f6-final-summary.md §4.4 gap), following the SAME local
//! `IndexedDiff<T,D>` shape mp4's diff uses (duplicated per-artifact, not shared — avi is its
//! own top-level format artifact) and the SAME rank/unrank absorb algorithm, adapted from gif
//! 89a's `GifDiff` absorb (`🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`).

use crate::artifacts::avi::standards::v1_0::subsets::any::schema::snapshot::{AviChunk, AviMainHeader, AviSnapshot, AviStream, AviStreamFormat, AviStreamHeader, RiffChunk};
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use serde::{Deserialize, Serialize};

//#region 🔖️IndexedTriple
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_indexed<T, D>(base: &[T], diff: &IndexedDiff<T, D>, validate_item: impl Fn(&T, &D) -> MutationApplyResult<()>) -> MutationApplyResult<()> {
    let mut removed = std::collections::HashSet::new();
    for &index in &diff.removed {
        if index >= base.len() {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "indexed removal target does not exist"));
        }
        if !removed.insert(index) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "indexed removal target is repeated"));
        }
    }
    let mut modified = std::collections::HashSet::new();
    for entry in &diff.modified {
        if entry.index >= base.len() {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "indexed modification target does not exist"));
        }
        if removed.contains(&entry.index) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "indexed modification targets a removed item"));
        }
        if !modified.insert(entry.index) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "indexed modification target is repeated"));
        }
        validate_item(&base[entry.index], &entry.diff).map_err(|error| error.under(vec!["modified".to_string(), entry.index.to_string()]))?;
    }
    let final_len = base.len() - removed.len() + diff.added.len();
    let mut added = std::collections::HashSet::new();
    for entry in &diff.added {
        if entry.index > final_len {
            return Err(MutationApplyError::new("mutation.apply.invalid-index", "indexed addition is outside the final collection"));
        }
        if !added.insert(entry.index) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "indexed addition occupies a repeated final position"));
        }
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

/// ➕️ Structural, total, base-free absorb — see mp4's `absorb_indexed` for the full derivation
/// (identical algorithm, adapted from gif 89a's `absorb_indexed_collection`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

    d1.removed = merged_removed_base;
    d1.modified = modified_map.into_iter().map(|(index, diff)| IndexedModified { index, diff }).collect();
    d1.added = merged_added_final;
}
//#endregion 🔖️IndexedTriple

//#region 🔖️Chunk
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AviChunkDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<u8>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keyframe: Option<bool>,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_chunk_diff(base: &AviChunk, d: &AviChunkDiff) -> AviChunk {
    AviChunk { fourcc: base.fourcc.clone(), data: d.data.clone().unwrap_or_else(|| base.data.clone()), keyframe: d.keyframe.unwrap_or(base.keyframe) }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_chunk_diff_mut(item: &mut AviChunk, d: &AviChunkDiff) {
    *item = apply_chunk_diff(item, d);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_chunk(a: &AviChunk, b: &AviChunk) -> AviChunkDiff {
    AviChunkDiff { data: (a.data != b.data).then(|| b.data.clone()), keyframe: (a.keyframe != b.keyframe).then_some(b.keyframe) }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn chunk_diff_is_empty(d: &AviChunkDiff) -> bool {
    d.data.is_none() && d.keyframe.is_none()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_chunk_diff(a: &mut AviChunkDiff, b: AviChunkDiff) {
    if b.data.is_some() {
        a.data = b.data;
    }
    if b.keyframe.is_some() {
        a.keyframe = b.keyframe;
    }
}
//#endregion 🔖️Chunk

//#region 🔖️Stream
pub type AviChunksDiff = IndexedDiff<AviChunk, AviChunkDiff>;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AviStreamDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strh: Option<AviStreamHeader>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strf: Option<AviStreamFormat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chunks: Option<AviChunksDiff>,
    /// 📦️ Whole-value replace, same treatment as `strh`/`strf` — this stream's retained `strl`
    /// auxiliaries (`vprp`, `JUNK`, ...) have no addressable per-item mutation surface (see
    /// `AviMutation`'s module doc comment), so they only ever change as a unit.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strl_extra: Option<Vec<RiffChunk>>,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_stream_diff(base: &AviStream, d: &AviStreamDiff) -> AviStream {
    AviStream {
        strh: d.strh.clone().unwrap_or_else(|| base.strh.clone()),
        strf: d.strf.clone().unwrap_or_else(|| base.strf.clone()),
        chunks: d.chunks.as_ref().map_or_else(|| base.chunks.clone(), |cd| apply_indexed(&base.chunks, cd, apply_chunk_diff)),
        strl_extra: d.strl_extra.clone().unwrap_or_else(|| base.strl_extra.clone()),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_stream_diff_mut(item: &mut AviStream, d: &AviStreamDiff) {
    *item = apply_stream_diff(item, d);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_stream(a: &AviStream, b: &AviStream) -> AviStreamDiff {
    let chunks_diff = between_indexed(&a.chunks, &b.chunks, between_chunk, chunk_diff_is_empty);
    AviStreamDiff {
        strh: (a.strh != b.strh).then(|| b.strh.clone()),
        strf: (a.strf != b.strf).then(|| b.strf.clone()),
        chunks: (!chunks_diff.is_empty()).then_some(chunks_diff),
        strl_extra: (a.strl_extra != b.strl_extra).then(|| b.strl_extra.clone()),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn stream_diff_is_empty(d: &AviStreamDiff) -> bool {
    d.strh.is_none() && d.strf.is_none() && d.chunks.is_none() && d.strl_extra.is_none()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_stream_diff(a: &mut AviStreamDiff, b: AviStreamDiff) {
    if b.strh.is_some() {
        a.strh = b.strh;
    }
    if b.strf.is_some() {
        a.strf = b.strf;
    }
    if b.strl_extra.is_some() {
        a.strl_extra = b.strl_extra;
    }
    match (&mut a.chunks, b.chunks) {
        (Some(existing), Some(other)) => absorb_indexed(existing, other, absorb_chunk_diff, apply_chunk_diff_mut),
        (a_slot @ None, Some(other)) => *a_slot = Some(other),
        _ => {}
    }
}
//#endregion 🔖️Stream

//#region 🔖️Diff
pub type AviStreamsDiff = IndexedDiff<AviStream, AviStreamDiff>;
pub type AviUnknownChunksDiff = IndexedDiff<RiffChunk, RiffChunk>;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AviDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub main_header: Option<AviMainHeader>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub streams: Option<AviStreamsDiff>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idx1_present: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unknown_chunks: Option<AviUnknownChunksDiff>,
    /// 📦️ Whole-value replace, same treatment as `main_header` — the retained `hdrl` auxiliaries
    /// (`JUNK`, ...) have no addressable per-item mutation surface (see `AviMutation`'s module doc
    /// comment), so they only ever change as a unit.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hdrl_extra: Option<Vec<RiffChunk>>,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_riff_diff(base: &RiffChunk, d: &RiffChunk) -> RiffChunk {
    let _ = base;
    d.clone()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_riff_diff_mut(item: &mut RiffChunk, d: &RiffChunk) {
    *item = d.clone();
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_riff(a: &RiffChunk, b: &RiffChunk) -> RiffChunk {
    let _ = a;
    b.clone()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn riff_diff_is_empty(_d: &RiffChunk) -> bool {
    false
}

impl MutationDiff<AviSnapshot> for AviDiff {
    fn apply(&self, base: &AviSnapshot) -> MutationApplyResult<AviSnapshot> {
        if let Some(diff) = &self.streams {
            validate_indexed(&base.streams, diff, validate_stream_diff)?;
        }
        if let Some(diff) = &self.unknown_chunks {
            validate_indexed(&base.unknown_chunks, diff, |_, _| Ok(()))?;
        }
        Ok(AviSnapshot {
            schema: base.schema.clone(),
            main_header: self.main_header.clone().unwrap_or_else(|| base.main_header.clone()),
            streams: self.streams.as_ref().map_or_else(|| base.streams.clone(), |sd| apply_indexed(&base.streams, sd, apply_stream_diff)),
            idx1_present: self.idx1_present.unwrap_or(base.idx1_present),
            unknown_chunks: self.unknown_chunks.as_ref().map_or_else(|| base.unknown_chunks.clone(), |cd| apply_indexed(&base.unknown_chunks, cd, apply_riff_diff)),
            hdrl_extra: self.hdrl_extra.clone().unwrap_or_else(|| base.hdrl_extra.clone()),
        })
    }

    fn absorb(&mut self, other: Self) {
        if other.main_header.is_some() {
            self.main_header = other.main_header;
        }
        if other.idx1_present.is_some() {
            self.idx1_present = other.idx1_present;
        }
        if other.hdrl_extra.is_some() {
            self.hdrl_extra = other.hdrl_extra;
        }
        match (&mut self.streams, other.streams) {
            (Some(existing), Some(other_streams)) => absorb_indexed(existing, other_streams, absorb_stream_diff, apply_stream_diff_mut),
            (slot @ None, Some(other_streams)) => *slot = Some(other_streams),
            _ => {}
        }
        match (&mut self.unknown_chunks, other.unknown_chunks) {
            (Some(existing), Some(other_chunks)) => absorb_indexed(existing, other_chunks, |a: &mut RiffChunk, b: RiffChunk| *a = b, apply_riff_diff_mut),
            (slot @ None, Some(other_chunks)) => *slot = Some(other_chunks),
            _ => {}
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_stream_diff(base: &AviStream, diff: &AviStreamDiff) -> MutationApplyResult<()> {
    if let Some(chunks) = &diff.chunks {
        validate_indexed(&base.chunks, chunks, |_, _| Ok(()))?;
    }
    Ok(())
}

impl DiffAlgebra<AviSnapshot> for AviDiff {
    fn between(base: &AviSnapshot, other: &AviSnapshot) -> Self {
        let streams_diff = between_indexed(&base.streams, &other.streams, between_stream, stream_diff_is_empty);
        let chunks_diff = between_indexed(&base.unknown_chunks, &other.unknown_chunks, between_riff, riff_diff_is_empty);
        Self {
            main_header: (base.main_header != other.main_header).then(|| other.main_header.clone()),
            streams: (!streams_diff.is_empty()).then_some(streams_diff),
            idx1_present: (base.idx1_present != other.idx1_present).then_some(other.idx1_present),
            unknown_chunks: (!chunks_diff.is_empty()).then_some(chunks_diff),
            hdrl_extra: (base.hdrl_extra != other.hdrl_extra).then(|| other.hdrl_extra.clone()),
        }
    }
    fn inverse(&self, base: &AviSnapshot) -> Self {
        // 🔁️ Correct-by-construction (identical reasoning to mp4's `Mp4Diff::inverse`).
        let mut after = base.clone();
        if let Some(v) = &self.main_header {
            after.main_header = v.clone();
        }
        if let Some(v) = &self.streams {
            after.streams = apply_indexed(&base.streams, v, apply_stream_diff);
        }
        if let Some(v) = self.idx1_present {
            after.idx1_present = v;
        }
        if let Some(v) = &self.unknown_chunks {
            after.unknown_chunks = apply_indexed(&base.unknown_chunks, v, apply_riff_diff);
        }
        if let Some(v) = &self.hdrl_extra {
            after.hdrl_extra = v.clone();
        }
        Self::between(&after, base)
    }
    fn is_empty(&self) -> bool {
        self.main_header.is_none() && self.streams.is_none() && self.idx1_present.is_none() && self.unknown_chunks.is_none() && self.hdrl_extra.is_none()
    }
}

/// 🧩 Set-snapshot diff helper — used by the `📸️set-snapshot/🔺️diff` leaf.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_snapshot(base: &AviSnapshot, snapshot: &AviSnapshot) -> AviDiff {
    <AviDiff as DiffAlgebra<AviSnapshot>>::between(base, snapshot)
}
//#endregion 🔖️Diff

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::avi::standards::v1_0::subsets::any::schema::snapshot::STDIO_AVI_DOCUMENT_SCHEMA;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn chunk(n: u8) -> AviChunk {
        AviChunk { fourcc: "00dc".into(), data: vec![n], keyframe: n % 2 == 0 }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn stream(chunks: Vec<AviChunk>) -> AviStream {
        AviStream {
            strh: AviStreamHeader {
                fcc_type: "vids".into(),
                fcc_handler: "MJPG".into(),
                flags: 0,
                priority: 0,
                language: 0,
                initial_frames: 0,
                scale: 1,
                rate: 10,
                start: 0,
                length: chunks.len() as u32,
                suggested_buffer_size: 0,
                quality: -1,
                sample_size: 0,
                rc_frame_left: 0,
                rc_frame_top: 0,
                rc_frame_right: 16,
                rc_frame_bottom: 16,
                rc_frame_width: 16,
                strh_extra: vec![],
            },
            strf: AviStreamFormat::BitmapInfo { size: 40, width: 16, height: 16, planes: 1, bit_count: 24, compression: "MJPG".into(), size_image: 0, x_pels_per_meter: 0, y_pels_per_meter: 0, colors_used: 0, colors_important: 0 },
            chunks,
            strl_extra: vec![],
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn snap(streams: Vec<AviStream>) -> AviSnapshot {
        AviSnapshot {
            schema: STDIO_AVI_DOCUMENT_SCHEMA.into(),
            main_header: AviMainHeader {
                micro_sec_per_frame: 100_000,
                max_bytes_per_sec: 0,
                padding_granularity: 0,
                flags: 0x10,
                total_frames: 0,
                initial_frames: 0,
                streams: streams.len() as u32,
                suggested_buffer_size: 0,
                width: 16,
                height: 16,
                reserved: vec![0, 0, 0, 0],
            },
            streams,
            idx1_present: true,
            unknown_chunks: vec![],
            hdrl_extra: vec![],
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn field_sweep_covers_every_mutable_field() {
        let a = snap(vec![stream(vec![chunk(1), chunk(2)]), stream(vec![chunk(3)])]);
        let mut b = a.clone();
        b.main_header.width = 32;
        b.streams[0].chunks.remove(0);
        b.streams[0].chunks.push(chunk(9));
        b.streams[0].strl_extra.push(RiffChunk { fourcc: "vprp".into(), data: vec![7] });
        b.streams.remove(1);
        b.streams.push(stream(vec![chunk(5)]));
        b.idx1_present = false;
        b.unknown_chunks.push(RiffChunk { fourcc: "JUNK".into(), data: vec![1] });
        b.hdrl_extra.push(RiffChunk { fourcc: "JUNK".into(), data: vec![2] });

        let d = <AviDiff as DiffAlgebra<AviSnapshot>>::between(&a, &b);
        assert!(d.main_header.is_some());
        assert!(d.streams.is_some());
        assert!(d.idx1_present.is_some());
        assert!(d.unknown_chunks.is_some());
        assert!(d.hdrl_extra.is_some());
        assert_eq!(d.apply(&a).unwrap(), b);
        assert_eq!(<AviDiff as DiffAlgebra<AviSnapshot>>::between(&b, &a).apply(&b).unwrap(), a);
        assert!(<AviDiff as DiffAlgebra<AviSnapshot>>::between(&a, &a).is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn inverse_law_round_trips_through_apply() {
        let a = snap(vec![stream(vec![chunk(1), chunk(2)])]);
        let mut b = a.clone();
        b.streams[0].chunks[0].keyframe = !b.streams[0].chunks[0].keyframe;
        let d = <AviDiff as DiffAlgebra<AviSnapshot>>::between(&a, &b);
        let after = d.apply(&a).unwrap();
        assert_eq!(after, b);
        assert_eq!(d.inverse(&a).apply(&after).unwrap(), a);
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_insert_then_remove_before_matches_sequential() {
        let base: Vec<AviChunk> = vec![chunk(1), chunk(2)];
        let f = AviChunk { fourcc: "00dc".into(), data: vec![0xAA], keyframe: true };
        let mut d1: AviChunksDiff = IndexedDiff { removed: vec![], modified: vec![], added: vec![IndexedAdded { index: 2, item: f.clone() }] };
        let mid = apply_indexed(&base, &d1, apply_chunk_diff);
        let d2: AviChunksDiff = IndexedDiff { removed: vec![0], modified: vec![], added: vec![] };
        let after = apply_indexed(&mid, &d2, apply_chunk_diff);
        let sequential = after.clone();
        absorb_indexed(&mut d1, d2, absorb_chunk_diff, apply_chunk_diff_mut);
        assert_eq!(apply_indexed(&base, &d1, apply_chunk_diff), sequential);
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_insert_insert_same_index_both_survive() {
        let base: Vec<AviChunk> = vec![chunk(1)];
        let f = AviChunk { fourcc: "00dc".into(), data: vec![0xAA], keyframe: true };
        let g = AviChunk { fourcc: "00dc".into(), data: vec![0xBB], keyframe: false };
        let mut d1: AviChunksDiff = IndexedDiff { removed: vec![], modified: vec![], added: vec![IndexedAdded { index: 1, item: f }] };
        let mid = apply_indexed(&base, &d1, apply_chunk_diff);
        let d2: AviChunksDiff = IndexedDiff { removed: vec![], modified: vec![], added: vec![IndexedAdded { index: 1, item: g }] };
        let after = apply_indexed(&mid, &d2, apply_chunk_diff);
        let sequential = after.clone();
        absorb_indexed(&mut d1, d2, absorb_chunk_diff, apply_chunk_diff_mut);
        let combined = apply_indexed(&base, &d1, apply_chunk_diff);
        assert_eq!(combined, sequential);
        assert_eq!(combined.len(), 3);
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_associativity_over_three_diffs() {
        let a = snap(vec![stream(vec![chunk(1), chunk(2)])]);
        let mut mid1 = a.clone();
        mid1.streams[0].chunks[0].keyframe = false;
        let mut mid2 = mid1.clone();
        mid2.streams.push(stream(vec![chunk(5)]));
        let mut after = mid2.clone();
        after.main_header.width = 999;

        let d1 = <AviDiff as DiffAlgebra<AviSnapshot>>::between(&a, &mid1);
        let d2 = <AviDiff as DiffAlgebra<AviSnapshot>>::between(&mid1, &mid2);
        let d3 = <AviDiff as DiffAlgebra<AviSnapshot>>::between(&mid2, &after);

        let mut left = d1.clone();
        left.absorb(d2.clone());
        left.absorb(d3.clone());
        let mut d23 = d2;
        d23.absorb(d3);
        let mut right = d1;
        right.absorb(d23);

        assert_eq!(left.apply(&a).unwrap(), after);
        assert_eq!(right.apply(&a).unwrap(), after);
        assert_eq!(left.apply(&a).unwrap(), right.apply(&a).unwrap());
    }
}
//#endregion 🔖️Tests
