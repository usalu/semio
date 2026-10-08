//! 🔺️ AviDiff — handcrafted sparse per-field diff (schema-design.md recipe): `main_header`
//! whole-value replaced (weak entity), `streams`/`chunks`/`unknown_chunks` index-keyed collection
//! triples (`removed`/`modified`/`added`, strong-like entities). Hand-rolled throughout — NOT
//! `#[derive(dsl::DslOps)]` (f6-final-summary.md §4.4 gap), following the SAME local
//! `IndexedDiff<T,D>` shape mp4's diff uses (duplicated per-artifact, not shared — avi is its
//! own top-level format artifact) and the SAME rank/unrank absorb algorithm, adapted from gif
//! 89a's `GifDiff` absorb (`🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🦀️.rs`).

use crate::standards::v1_0::subsets::any::schema::snapshot::{AviChunk, AviMainHeader, AviSnapshot, AviStream, AviStreamFormat, AviStreamHeader, RiffChunk};
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};

//#region 🔖️IndexedTriple
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct IndexedDiff<T, D> {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<IndexedModified<D>>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<IndexedAdded<T>>,
}

impl<T, D> IndexedDiff<T, D> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct IndexedModified<D> {
    pub index: usize,
    pub diff: D,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
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
    let removed1_sorted = semio_s_artifact_stdio_contract::ordered(&d1.removed);
    let mut added1_index_sorted: Vec<usize> = d1.added.iter().map(|a| a.index).collect();
    added1_index_sorted.sort_unstable();
    let removed2_sorted = semio_s_artifact_stdio_contract::ordered(&d2.removed);
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

/// ↩️ Negative rows for an indexed collection triple against its BASE items: added rows become removals at their final index,
/// removed rows return at their base index, and each modified row restores its base value at the index the row has after the
/// diff. Every list comes back ascending, the normal form [`absorb_indexed`] emits.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse_indexed<T: Clone, D>(diff: &IndexedDiff<T, D>, base: &[T], inverse_item: impl Fn(&D, &T) -> D) -> IndexedDiff<T, D> {
    let removed_sorted = semio_s_artifact_stdio_contract::ordered_unique(&diff.removed);
    let mut added_final: Vec<usize> = diff.added.iter().map(|added| added.index).collect();
    added_final.sort_unstable();
    let after_index = |index: usize| {
        let survivor = index - removed_sorted.iter().filter(|dropped| **dropped < index).count();
        added_final.iter().fold(survivor, |position, inserted| if *inserted <= position { position + 1 } else { position })
    };
    let mut modified: Vec<IndexedModified<D>> = diff.modified.iter().filter_map(|row| base.get(row.index).map(|item| IndexedModified { index: after_index(row.index), diff: inverse_item(&row.diff, item) })).collect();
    modified.sort_by_key(|row| row.index);
    let added = removed_sorted.iter().filter_map(|index| base.get(*index).map(|item| IndexedAdded { index: *index, item: item.clone() })).collect();
    IndexedDiff { removed: added_final, modified, added }
}
//#endregion 🔖️IndexedTriple

//#region 🔖️Chunk
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct AviChunkDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<u8>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
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
fn inverse_chunk_diff(d: &AviChunkDiff, base: &AviChunk) -> AviChunkDiff {
    AviChunkDiff { data: d.data.as_ref().map(|_| base.data.clone()), keyframe: d.keyframe.map(|_| base.keyframe) }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn chunk_diff_is_empty(d: &AviChunkDiff) -> bool {
    d.data.is_none() && d.keyframe.is_none()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_chunk_rows(a: &mut AviChunkDiff, b: AviChunkDiff) {
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

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct AviStreamDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub strh: Option<AviStreamHeader>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub strf: Option<AviStreamFormat>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub chunks: Option<AviChunksDiff>,
    /// 📦️ Whole-value replace, same treatment as `strh`/`strf` — this stream's retained `strl`
    /// auxiliaries (`vprp`, `JUNK`, ...) have no addressable per-item mutation surface (see
    /// `AviMutation`'s module doc comment), so they only ever change as a unit.
    #[value(default, skip_serializing_if = "Option::is_none")]
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
fn inverse_stream_diff(d: &AviStreamDiff, base: &AviStream) -> AviStreamDiff {
    AviStreamDiff {
        strh: d.strh.as_ref().map(|_| base.strh.clone()),
        strf: d.strf.as_ref().map(|_| base.strf.clone()),
        chunks: d.chunks.as_ref().map(|chunks| inverse_indexed(chunks, &base.chunks, inverse_chunk_diff)).filter(|chunks| !chunks.is_empty()),
        strl_extra: d.strl_extra.as_ref().map(|_| base.strl_extra.clone()),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn stream_diff_is_empty(d: &AviStreamDiff) -> bool {
    d.strh.is_none() && d.strf.is_none() && d.chunks.is_none() && d.strl_extra.is_none()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_stream_rows(a: &mut AviStreamDiff, b: AviStreamDiff) {
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
        (Some(existing), Some(other)) => absorb_indexed(existing, other, absorb_chunk_rows, apply_chunk_diff_mut),
        (a_slot @ None, Some(other)) => *a_slot = Some(other),
        _ => {}
    }
}
//#endregion 🔖️Stream

//#region 🔖️Diff
pub type AviStreamsDiff = IndexedDiff<AviStream, AviStreamDiff>;
pub type AviUnknownChunksDiff = IndexedDiff<RiffChunk, RiffChunk>;

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct AviDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub main_header: Option<AviMainHeader>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub streams: Option<AviStreamsDiff>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub idx1_present: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub unknown_chunks: Option<AviUnknownChunksDiff>,
    /// 📦️ Whole-value replace, same treatment as `main_header` — the retained `hdrl` auxiliaries
    /// (`JUNK`, ...) have no addressable per-item mutation surface (see `AviMutation`'s module doc
    /// comment), so they only ever change as a unit.
    #[value(default, skip_serializing_if = "Option::is_none")]
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
fn riff_diff_is_empty(_d: &RiffChunk) -> bool {
    false
}

impl MutationDiff<AviSnapshot> for AviDiff {
    fn apply(&self, base: &AviSnapshot, _capability: protocol::ApplyCapability) -> MutationApplyResult<AviSnapshot> {
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
            (Some(existing), Some(other_streams)) => absorb_indexed(existing, other_streams, absorb_stream_rows, apply_stream_diff_mut),
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
    fn inverse(&self, base: &AviSnapshot) -> Self {
        Self {
            main_header: self.main_header.as_ref().map(|_| base.main_header.clone()),
            streams: self.streams.as_ref().map(|streams| inverse_indexed(streams, &base.streams, inverse_stream_diff)).filter(|streams| !streams.is_empty()),
            idx1_present: self.idx1_present.map(|_| base.idx1_present),
            unknown_chunks: self.unknown_chunks.as_ref().map(|chunks| inverse_indexed(chunks, &base.unknown_chunks, |_, chunk: &RiffChunk| chunk.clone())).filter(|chunks| !chunks.is_empty()),
            hdrl_extra: self.hdrl_extra.as_ref().map(|_| base.hdrl_extra.clone()),
        }
    }
    fn is_empty(&self) -> bool {
        self.main_header.is_none() && self.streams.is_none() && self.idx1_present.is_none() && self.unknown_chunks.is_none() && self.hdrl_extra.is_none()
    }
}

//#endregion 🔖️Diff

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
