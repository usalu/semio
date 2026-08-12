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

use crate::artifacts::semio::standards::v1::engine::triples::{self, IndexAdded, IndexModified, IndexedTripleDiff};
use crate::artifacts::semio::standards::v1::subsets::audio::schema::snapshot::{
    SemioAudioChannel, SemioAudioFormat, SemioAudioSnapshot, SemioAudioTag,
};
use protocol::MutationDiff;
use protocol::command::DiffAlgebra;
/// 🔧️ Unconditional — `impl protocol::DiffCodec for SemioAudioDiff` below's `encode_diff`/
/// `decode_diff` are now real production code (binary upgrade, this wave), not test-only.
use protocol::DiffCodec;
use serde::{Deserialize, Serialize};

//#region 🔖️IndexTransport
/// 📐️ Shared rank/unrank arithmetic for index-keyed collection diffs — see
/// `🧬️schema-design.md` §Absorb; ported verbatim from gif 89a's own diff module (the reference
/// implementation for this arithmetic), generalized here to operate on the shared
/// `IndexedTripleDiff<D,T>` shape instead of a bespoke per-artifact triple struct.
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

//#region 🔖️GenericIndexedCollectionOps
/// 🕳️ Emptiness for the shared `IndexedTripleDiff<D,T>` (the engine type itself carries no
/// `is_empty` — this subset supplies the semantics, per the recipe: the engine is data+text-codec
/// only, apply/between/absorb/inverse are each collection's own job).
fn indexed_is_empty<D, T>(t: &IndexedTripleDiff<D, T>) -> bool {
    t.removed.is_empty() && t.modified.is_empty() && t.added.is_empty()
}

fn indexed_between<T: Clone + PartialEq, D>(
    base: &[T],
    other: &[T],
    diff_between: impl Fn(&T, &T) -> Option<D>,
) -> IndexedTripleDiff<D, T> {
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

fn indexed_apply<T: Clone, D>(triple: &IndexedTripleDiff<D, T>, base: &[T], diff_apply: impl Fn(&D, &T) -> T) -> Vec<T> {
    let mut next: Vec<Option<T>> = base.iter().cloned().map(Some).collect();
    for m in &triple.modified {
        if let Some(slot) = next.get_mut(m.index) {
            if let Some(item) = slot {
                *item = diff_apply(&m.diff, item);
            }
        }
    }
    let mut removed_sorted = triple.removed.clone();
    removed_sorted.sort_unstable();
    removed_sorted.reverse();
    for &r in &removed_sorted {
        if r < next.len() { next.remove(r); }
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
fn indexed_absorb<T: Clone, D: Clone>(
    mine: &mut IndexedTripleDiff<D, T>,
    other: IndexedTripleDiff<D, T>,
    mut absorb_diff: impl FnMut(&mut D, D),
    apply_diff_to_item: impl Fn(&D, &T) -> T,
) {
    let removed1 = std::mem::take(&mut mine.removed);
    let modified1: Vec<(usize, D)> = std::mem::take(&mut mine.modified).into_iter().map(|m| (m.index, m.diff)).collect();
    let added1: Vec<(usize, T)> = std::mem::take(&mut mine.added).into_iter().map(|a| (a.index, a.item)).collect();
    let removed2 = other.removed;
    let modified2: Vec<(usize, D)> = other.modified.into_iter().map(|m| (m.index, m.diff)).collect();
    let added2: Vec<(usize, T)> = other.added.into_iter().map(|a| (a.index, a.item)).collect();

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

    mine.removed = merged_removed_base;
    mine.modified = merged_modified.into_iter().map(|(index, diff)| IndexModified { index, diff }).collect();
    mine.added = merged_added_final.into_iter().map(|(index, item)| IndexAdded { index, item }).collect();
}

/// ↩️ Diff-level inverse for a generic index-keyed collection triple, given the ORIGINAL base
/// items — generalized from gif's `inverse_indexed_collection`.
fn indexed_inverse<T: Clone, D>(
    triple: &IndexedTripleDiff<D, T>,
    base_items: &[T],
    diff_inverse: impl Fn(&D, &T) -> D,
) -> IndexedTripleDiff<D, T> {
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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemioAudioChannelDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub samples: Option<Vec<f32>>,
}

impl SemioAudioChannelDiff {
    pub fn is_empty(&self) -> bool { self.samples.is_none() }
    pub fn between(base: &SemioAudioChannel, other: &SemioAudioChannel) -> Self {
        Self { samples: (base.samples != other.samples).then_some(other.samples.clone()) }
    }
    pub fn apply(&self, base: &SemioAudioChannel) -> SemioAudioChannel {
        let mut next = base.clone();
        if let Some(v) = &self.samples { next.samples = v.clone(); }
        next
    }
    pub fn inverse(&self, base: &SemioAudioChannel) -> Self {
        Self { samples: self.samples.as_ref().map(|_| base.samples.clone()) }
    }
    fn absorb(&mut self, other: Self) {
        if other.samples.is_some() { self.samples = other.samples; }
    }
}

pub type SemioAudioChannelsDiff = IndexedTripleDiff<SemioAudioChannelDiff, SemioAudioChannel>;

fn channels_between(base: &[SemioAudioChannel], other: &[SemioAudioChannel]) -> SemioAudioChannelsDiff {
    indexed_between(base, other, |a, b| {
        let d = SemioAudioChannelDiff::between(a, b);
        (!d.is_empty()).then_some(d)
    })
}
fn channels_apply(d: &SemioAudioChannelsDiff, base: &[SemioAudioChannel]) -> Vec<SemioAudioChannel> {
    indexed_apply(d, base, |diff, item| diff.apply(item))
}
fn channels_absorb(mine: &mut SemioAudioChannelsDiff, other: SemioAudioChannelsDiff) {
    indexed_absorb(mine, other, |d, o| d.absorb(o), |diff, item| diff.apply(item))
}
fn channels_inverse(d: &SemioAudioChannelsDiff, base_items: &[SemioAudioChannel]) -> SemioAudioChannelsDiff {
    indexed_inverse(d, base_items, |diff, item| diff.inverse(item))
}
//#endregion 🔖️ChannelDiff

//#region 🔖️TagsDiff
/// 🏷️ `tags` is a WEAK/value collection per the recipe: its "diff" IS the whole new
/// [`SemioAudioTag`] (`D = T = SemioAudioTag`), no further sub-diffing of a key/value pair.
pub type SemioAudioTagsDiff = IndexedTripleDiff<SemioAudioTag, SemioAudioTag>;

fn tags_between(base: &[SemioAudioTag], other: &[SemioAudioTag]) -> SemioAudioTagsDiff {
    indexed_between(base, other, |a, b| (a != b).then_some(b.clone()))
}
fn tags_apply(d: &SemioAudioTagsDiff, base: &[SemioAudioTag]) -> Vec<SemioAudioTag> {
    indexed_apply(d, base, |diff, _item| diff.clone())
}
fn tags_absorb(mine: &mut SemioAudioTagsDiff, other: SemioAudioTagsDiff) {
    indexed_absorb(mine, other, |d, o| *d = o, |diff, _item| diff.clone())
}
fn tags_inverse(d: &SemioAudioTagsDiff, base_items: &[SemioAudioTag]) -> SemioAudioTagsDiff {
    indexed_inverse(d, base_items, |_diff, item| item.clone())
}
//#endregion 🔖️TagsDiff

//#region 🔖️Diff
/// 🔺️ Diff for `s.stdio.semio.audio`. No `snapshot: Option<SemioAudioSnapshot>` full-replace
/// slot anywhere — every field is sparse.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemioAudioDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<SemioAudioFormat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channels: Option<SemioAudioChannelsDiff>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<SemioAudioTagsDiff>,
}

impl SemioAudioDiff {
    pub fn is_empty_diff(&self) -> bool {
        self.sample_rate.is_none()
            && self.format.is_none()
            && self.channels.as_ref().map(indexed_is_empty).unwrap_or(true)
            && self.tags.as_ref().map(indexed_is_empty).unwrap_or(true)
    }
}

impl MutationDiff<SemioAudioSnapshot> for SemioAudioDiff {
    fn apply(&self, base: &SemioAudioSnapshot) -> SemioAudioSnapshot {
        let mut next = base.clone();
        if let Some(v) = self.sample_rate { next.sample_rate = v; }
        if let Some(v) = self.format { next.format = v; }
        if let Some(d) = &self.channels { next.channels = channels_apply(d, &next.channels); }
        if let Some(d) = &self.tags { next.tags = tags_apply(d, &next.tags); }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.sample_rate.is_some() { self.sample_rate = other.sample_rate; }
        if other.format.is_some() { self.format = other.format; }
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

    fn is_empty(&self) -> bool { self.is_empty_diff() }
}

/// 🧩️ Builds a set-snapshot diff — sparse field-by-field, never a full-replace slot.
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
pub(crate) fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
pub(crate) fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
pub(crate) fn hex_decode_string(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
pub(crate) fn parse_u32(s: &str) -> Result<u32, String> { s.parse().map_err(|e: std::num::ParseIntError| e.to_string()) }
pub(crate) fn parse_usize(s: &str) -> Result<usize, String> { s.parse().map_err(|e: std::num::ParseIntError| e.to_string()) }

pub(crate) fn split_top_level(s: &str, sep: char) -> Vec<&str> { triples::split_top_level(s, sep) }
pub(crate) fn strip_brackets(s: &str) -> Result<&str, String> { triples::strip_brackets(s) }

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
pub(crate) fn enc_format(f: SemioAudioFormat) -> &'static str {
    match f {
        SemioAudioFormat::Pcm8 => "pcm8",
        SemioAudioFormat::Pcm16 => "pcm16",
        SemioAudioFormat::Pcm24 => "pcm24",
        SemioAudioFormat::Pcm32 => "pcm32",
        SemioAudioFormat::Float32 => "f32",
        SemioAudioFormat::Float64 => "f64",
    }
}
pub(crate) fn dec_format(s: &str) -> Result<SemioAudioFormat, String> {
    match s {
        "pcm8" => Ok(SemioAudioFormat::Pcm8),
        "pcm16" => Ok(SemioAudioFormat::Pcm16),
        "pcm24" => Ok(SemioAudioFormat::Pcm24),
        "pcm32" => Ok(SemioAudioFormat::Pcm32),
        "f32" => Ok(SemioAudioFormat::Float32),
        "f64" => Ok(SemioAudioFormat::Float64),
        other => Err(format!("bad audio format {other:?}")),
    }
}

/// 🔢️ Exact-round-trip `f32` list — `to_bits()` hex tokens inside a bracket, never decimal
/// text (sidesteps float-formatting precision loss and NaN/-0.0 print-ambiguity entirely).
pub(crate) fn enc_f32_list(v: &[f32]) -> String {
    format!("[{}]", v.iter().map(|f| format!("{:08x}", f.to_bits())).collect::<Vec<_>>().join(","))
}
pub(crate) fn dec_f32_list(s: &str) -> Result<Vec<f32>, String> {
    let inner = strip_brackets(s)?;
    if inner.is_empty() { return Ok(Vec::new()); }
    split_top_level(inner, ',').into_iter().map(|tok| u32::from_str_radix(tok, 16).map(f32::from_bits).map_err(|e| e.to_string())).collect()
}

pub(crate) fn enc_channel(c: &SemioAudioChannel) -> String { enc_f32_list(&c.samples) }
pub(crate) fn dec_channel(s: &str) -> Result<SemioAudioChannel, String> { Ok(SemioAudioChannel { samples: dec_f32_list(s)? }) }

fn enc_channel_diff(d: &SemioAudioChannelDiff) -> String { encode_option(&d.samples, |v| enc_f32_list(v)) }
fn dec_channel_diff(s: &str) -> Result<SemioAudioChannelDiff, String> { Ok(SemioAudioChannelDiff { samples: decode_option(s, dec_f32_list)? }) }

pub(crate) fn enc_tag(t: &SemioAudioTag) -> String {
    format!("[{},{}]", hex_encode(t.key.as_bytes()), hex_encode(t.value.as_bytes()))
}
pub(crate) fn dec_tag(s: &str) -> Result<SemioAudioTag, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [key, value] = parts.as_slice() else { return Err(format!("tag: expected 2 fields, got {}", parts.len())) };
    Ok(SemioAudioTag { key: hex_decode_string(key)?, value: hex_decode_string(value)? })
}

/// 🧩️ Full bracket encoding of a snapshot — used both by [`protocol::DiffCodec`]'s `SetSnapshot`
/// payload (via the mutations module) and directly nowhere else; kept here alongside its sibling
/// value codecs.
pub(crate) fn enc_snapshot(s: &SemioAudioSnapshot) -> String {
    format!(
        "[{},{},{},[{}],[{}]]",
        hex_encode(s.schema.as_bytes()),
        s.sample_rate,
        enc_format(s.format),
        s.channels.iter().map(enc_channel).collect::<Vec<_>>().join(","),
        s.tags.iter().map(enc_tag).collect::<Vec<_>>().join(","),
    )
}
pub(crate) fn dec_snapshot(s: &str) -> Result<SemioAudioSnapshot, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [schema_hex, sample_rate, format, channels_s, tags_s] = parts.as_slice() else {
        return Err(format!("snapshot: expected 5 fields, got {}", parts.len()));
    };
    let channels = split_top_level(strip_brackets(channels_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_channel).collect::<Result<Vec<_>, String>>()?;
    let tags = split_top_level(strip_brackets(tags_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_tag).collect::<Result<Vec<_>, String>>()?;
    Ok(SemioAudioSnapshot {
        schema: hex_decode_string(schema_hex)?,
        sample_rate: parse_u32(sample_rate)?,
        format: dec_format(format)?,
        channels,
        tags,
    })
}
//#endregion 🔖️ValueCodecs

//#region 🔖️TopLevel
fn print_audio_diff(d: &SemioAudioDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = d.sample_rate { tokens.push(format!("rate={v}")); }
    if let Some(v) = d.format { tokens.push(format!("format={}", enc_format(v))); }
    if let Some(v) = &d.channels {
        tokens.push(format!("channels{{{}}}", triples::enc_indexed_triple(v, enc_channel_diff, enc_channel)));
    }
    if let Some(v) = &d.tags {
        tokens.push(format!("tags{{{}}}", triples::enc_indexed_triple(v, enc_tag, enc_tag)));
    }
    tokens.join(" ")
}
fn parse_audio_diff(line: &str) -> Result<SemioAudioDiff, String> {
    let mut d = SemioAudioDiff::default();
    if line.is_empty() { return Ok(d); }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("rate=") { d.sample_rate = Some(parse_u32(rest)?); }
        else if let Some(rest) = token.strip_prefix("format=") { d.format = Some(dec_format(rest)?); }
        else if let Some(rest) = token.strip_prefix("channels{") {
            let body = rest.strip_suffix('}').ok_or_else(|| "channels: missing closing brace".to_string())?;
            d.channels = Some(triples::dec_indexed_triple(body, dec_channel_diff, dec_channel)?);
        }
        else if let Some(rest) = token.strip_prefix("tags{") {
            let body = rest.strip_suffix('}').ok_or_else(|| "tags: missing closing brace".to_string())?;
            d.tags = Some(triples::dec_indexed_triple(body, dec_tag, dec_tag)?);
        }
        else { return Err(format!("audio diff: unknown token {token:?}")); }
    }
    Ok(d)
}

/// 🧪️ Real LEB128-varint-length-prefixed binary primitives (`store::pack_rt::write_varint_u64` /
/// `store::ByteReader`, same helpers this subset's own `📸️snapshot` facet's `ArtifactPack` uses)
/// backing the real `DiffCodec::encode_diff`/`decode_diff` below.
fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    store::pack_rt::write_varint_u64(out, s.len() as u64);
    out.extend_from_slice(s.as_bytes());
}
fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    let bytes = reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec();
    String::from_utf8(bytes).map_err(|e| e.to_string())
}

impl protocol::DiffCodec for SemioAudioDiff {
    fn print_diff(&self) -> String { print_audio_diff(self) }
    fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        parse_audio_diff(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    /// ⚡️ Real binary diff frame, replacing the old `print_diff().into_bytes()` text-as-binary
    /// shortcut. `format u8` + `presence u8` (bit0=`sample_rate` bit1=`format` bit2=`channels`
    /// bit3=`tags`) are two REAL fixed fields; each present field then follows as its own
    /// varint-length-prefixed opaque text blob (the same per-field `rate=`/`format=`/
    /// `enc_indexed_triple`-based text `print_diff` already produces) — independently-delimited
    /// segments rather than one bare trailing `bytes` because there can be 0-4 of them (chaining a
    /// `Cond` per-segment hits the `protocol-cond-cannot-chain` gap: a second `if`-guard on a field
    /// that was itself only conditionally decoded hard-errors `eval_cond` — see `✳️workflow`'s/
    /// `✳️image`'s pilot reports).
    fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const DIFF_BINARY_FORMAT: u8 = 1;
        let mut presence = 0u8;
        if self.sample_rate.is_some() { presence |= 0b0001; }
        if self.format.is_some() { presence |= 0b0010; }
        if self.channels.is_some() { presence |= 0b0100; }
        if self.tags.is_some() { presence |= 0b1000; }
        let mut out = vec![DIFF_BINARY_FORMAT, presence];
        if let Some(v) = self.sample_rate { write_str_lp(&mut out, &v.to_string()); }
        if let Some(v) = self.format { write_str_lp(&mut out, enc_format(v)); }
        if let Some(v) = &self.channels { write_str_lp(&mut out, &triples::enc_indexed_triple(v, enc_channel_diff, enc_channel)); }
        if let Some(v) = &self.tags { write_str_lp(&mut out, &triples::enc_indexed_triple(v, enc_tag, enc_tag)); }
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
        let sample_rate = if presence & 0b0001 != 0 {
            let text = read_str_lp(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff sample_rate blob", offset: 2, detail: e })?;
            Some(parse_u32(&text).map_err(|e| protocol::ProtocolError::Malformed { what: "diff sample_rate text", offset: 2, detail: e })?)
        } else { None };
        let format = if presence & 0b0010 != 0 {
            let text = read_str_lp(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff format blob", offset: 2, detail: e })?;
            Some(dec_format(&text).map_err(|e| protocol::ProtocolError::Malformed { what: "diff format text", offset: 2, detail: e })?)
        } else { None };
        let channels = if presence & 0b0100 != 0 {
            let text = read_str_lp(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff channels blob", offset: 2, detail: e })?;
            Some(triples::dec_indexed_triple(&text, dec_channel_diff, dec_channel).map_err(|e| protocol::ProtocolError::Malformed { what: "diff channels text", offset: 2, detail: e })?)
        } else { None };
        let tags = if presence & 0b1000 != 0 {
            let text = read_str_lp(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff tags blob", offset: 2, detail: e })?;
            Some(triples::dec_indexed_triple(&text, dec_tag, dec_tag).map_err(|e| protocol::ProtocolError::Malformed { what: "diff tags text", offset: 2, detail: e })?)
        } else { None };
        Ok(SemioAudioDiff { sample_rate, format, channels, tags })
    }
}
//#endregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec

//#region 🔖️Demo
/// 🌱 Representative `SemioAudioDiff` cases (empty/no-op, a full field sweep both directions incl.
/// both collection triples) — single source of truth for `diff_grammar_conformance_law`/
/// `protocol_walk_law` in `🎹️composer/🦀️component.rs`.
#[cfg(test)]
pub(crate) fn demo_diff_cases() -> Vec<SemioAudioDiff> {
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
    vec![
        SemioAudioDiff::default(),
        <SemioAudioDiff as DiffAlgebra<SemioAudioSnapshot>>::between(&a, &b),
        <SemioAudioDiff as DiffAlgebra<SemioAudioSnapshot>>::between(&b, &a),
    ]
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn channel(seed: f32, len: usize) -> SemioAudioChannel {
        SemioAudioChannel { samples: (0..len).map(|i| seed + i as f32 * 0.1).collect() }
    }

    fn base_snapshot() -> SemioAudioSnapshot {
        SemioAudioSnapshot {
            sample_rate: 44_100,
            format: SemioAudioFormat::Pcm16,
            channels: vec![channel(0.0, 4), channel(1.0, 4), channel(2.0, 4)],
            tags: vec![SemioAudioTag { key: "title".into(), value: "one".into() }],
            ..SemioAudioSnapshot::default()
        }
    }

    /// 🧪️ Canonical absorb case 1: `InsertChannel(2,c)` then `RemoveChannel(0)` →
    /// `{removed:[0], added:[(1,c)]}`.
    #[test]
    fn absorb_insert_then_remove_before_shifts_index() {
        let c = channel(9.0, 2);
        let mut d1: SemioAudioChannelsDiff = IndexedTripleDiff { added: vec![IndexAdded { index: 2, item: c.clone() }], ..Default::default() };
        let d2: SemioAudioChannelsDiff = IndexedTripleDiff { removed: vec![0], ..Default::default() };
        channels_absorb(&mut d1, d2);
        assert_eq!(d1.removed, vec![0]);
        assert_eq!(d1.added, vec![IndexAdded { index: 1, item: c }]);
        assert!(d1.modified.is_empty());
    }

    /// 🧪️ Canonical absorb case 2: `InsertChannel(2,c)` then `InsertChannel(2,d)` → BOTH survive.
    #[test]
    fn absorb_insert_insert_same_index_both_survive() {
        let c = channel(1.0, 2);
        let d = channel(2.0, 2);
        let mut d1: SemioAudioChannelsDiff = IndexedTripleDiff { added: vec![IndexAdded { index: 2, item: c.clone() }], ..Default::default() };
        let d2: SemioAudioChannelsDiff = IndexedTripleDiff { added: vec![IndexAdded { index: 2, item: d.clone() }], ..Default::default() };
        channels_absorb(&mut d1, d2);
        assert_eq!(d1.added, vec![IndexAdded { index: 2, item: d }, IndexAdded { index: 3, item: c }]);
    }

    /// 🧪️ Canonical absorb case 3: `InsertChannel(1,c)` then `SetChannelSamples(1,..)` patches
    /// INTO the added payload — merged has only `added`, no separate `modified` entry.
    #[test]
    fn absorb_insert_then_set_field_patches_into_added() {
        let c = channel(1.0, 2);
        let mut d1: SemioAudioChannelsDiff = IndexedTripleDiff { added: vec![IndexAdded { index: 1, item: c.clone() }], ..Default::default() };
        let d2: SemioAudioChannelsDiff = IndexedTripleDiff {
            modified: vec![IndexModified { index: 1, diff: SemioAudioChannelDiff { samples: Some(vec![9.0, 9.0]) } }],
            ..Default::default()
        };
        channels_absorb(&mut d1, d2);
        assert!(d1.modified.is_empty());
        assert_eq!(d1.added.len(), 1);
        assert_eq!(d1.added[0].item.samples, vec![9.0, 9.0]);
        assert_eq!(d1.added[0].index, 1);
    }

    #[test]
    fn absorb_law_holds_over_curated_ops() {
        let base = base_snapshot();
        let mid = {
            let mut s = base.clone();
            s.channels.insert(1, channel(9.0, 4));
            s.channels.remove(0);
            s.tags.push(SemioAudioTag { key: "artist".into(), value: "a".into() });
            s
        };
        let after = {
            let mut s = mid.clone();
            s.channels[0].samples = vec![5.0, 5.0, 5.0, 5.0];
            s.channels.push(channel(5.0, 4));
            s.tags[0].value = "changed".into();
            s
        };
        let mut d1 = <SemioAudioDiff as DiffAlgebra<SemioAudioSnapshot>>::between(&base, &mid);
        let d2 = <SemioAudioDiff as DiffAlgebra<SemioAudioSnapshot>>::between(&mid, &after);
        d1.absorb(d2);
        assert_eq!(d1.apply(&base), after);
    }

    #[test]
    fn between_roundtrip_law() {
        let a = base_snapshot();
        let mut b = base_snapshot();
        b.sample_rate = 48_000;
        b.channels.push(channel(3.0, 4));
        let ab = <SemioAudioDiff as DiffAlgebra<SemioAudioSnapshot>>::between(&a, &b);
        assert_eq!(ab.apply(&a), b);
        let ba = <SemioAudioDiff as DiffAlgebra<SemioAudioSnapshot>>::between(&b, &a);
        assert_eq!(ba.apply(&b), a);
        assert!(<SemioAudioDiff as DiffAlgebra<SemioAudioSnapshot>>::between(&a, &a).is_empty());
    }

    #[test]
    fn inverse_law() {
        let base = base_snapshot();
        let next = {
            let mut s = base.clone();
            s.channels[0].samples = vec![7.0, 7.0, 7.0, 7.0];
            s.channels.remove(1);
            s.channels.push(channel(6.0, 4));
            s.sample_rate = 22_050;
            s.tags.clear();
            s
        };
        let d = <SemioAudioDiff as DiffAlgebra<SemioAudioSnapshot>>::between(&base, &next);
        let mutated = d.apply(&base);
        let inv = d.inverse(&base);
        assert_eq!(inv.apply(&mutated), base);
    }

    /// 🧪️ Field sweep — the acceptance criterion: `sweep_a`/`sweep_b` differ in EVERY mutable
    /// field, with asymmetric collection lengths so both `removed` and `added` get exercised
    /// (split across both directions, matching the recipe's own guidance).
    #[test]
    fn field_sweep_covers_every_mutable_field() {
        let sweep_a = SemioAudioSnapshot {
            sample_rate: 44_100,
            format: SemioAudioFormat::Pcm16,
            channels: vec![channel(0.0, 4), channel(1.0, 4)],
            tags: vec![SemioAudioTag { key: "title".into(), value: "first".into() }],
            ..SemioAudioSnapshot::default()
        };
        let sweep_b = SemioAudioSnapshot {
            sample_rate: 96_000,
            format: SemioAudioFormat::Float64,
            channels: vec![channel(9.0, 4), channel(1.0, 4), channel(2.0, 4)],
            tags: vec![],
            ..SemioAudioSnapshot::default()
        };

        let ab = <SemioAudioDiff as DiffAlgebra<SemioAudioSnapshot>>::between(&sweep_a, &sweep_b);
        assert_eq!(ab.apply(&sweep_a), sweep_b);
        assert!(ab.sample_rate.is_some());
        assert!(ab.format.is_some());
        let channels_ab = ab.channels.as_ref().expect("channels must differ");
        assert!(!channels_ab.modified.is_empty(), "sweep must exercise a modified channel");
        assert!(!channels_ab.added.is_empty(), "sweep must exercise an added channel (b is longer)");
        let tags_ab = ab.tags.as_ref().expect("tags must differ");
        assert!(!tags_ab.removed.is_empty(), "sweep must exercise a removed tag (b has none)");

        let ba = <SemioAudioDiff as DiffAlgebra<SemioAudioSnapshot>>::between(&sweep_b, &sweep_a);
        assert_eq!(ba.apply(&sweep_b), sweep_a);
        let channels_ba = ba.channels.as_ref().expect("channels must differ");
        assert!(!channels_ba.removed.is_empty(), "reverse direction must exercise a removed channel (a is shorter)");
        let tags_ba = ba.tags.as_ref().expect("tags must differ");
        assert!(!tags_ba.added.is_empty(), "reverse direction must exercise an added tag");

        assert!(<SemioAudioDiff as DiffAlgebra<SemioAudioSnapshot>>::between(&sweep_a, &sweep_a).is_empty());
    }

    /// 🧪️ `DiffCodec` text/binary round-trip law — exercises scalars and both collection triples
    /// (`removed`/`modified`/`added`) simultaneously via a real `between()` result.
    #[test]
    fn diff_codec_text_binary_roundtrip_law() {
        let a = base_snapshot();
        let mut b = base_snapshot();
        b.sample_rate = 48_000;
        b.format = SemioAudioFormat::Float32;
        b.channels[0].samples = vec![9.9, 8.8];
        b.channels.remove(1);
        b.channels.push(channel(4.0, 3));
        b.tags.push(SemioAudioTag { key: "artist".into(), value: "someone".into() });

        let cases = vec![
            SemioAudioDiff::default(),
            <SemioAudioDiff as DiffAlgebra<SemioAudioSnapshot>>::between(&a, &b),
            <SemioAudioDiff as DiffAlgebra<SemioAudioSnapshot>>::between(&b, &a),
        ];
        for d in cases {
            let printed = d.print_diff();
            assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
            let parsed = SemioAudioDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
            assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

            let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
            let decoded = SemioAudioDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
            assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
        }
    }

    #[test]
    fn snapshot_bracket_codec_round_trips() {
        let s = base_snapshot();
        let encoded = enc_snapshot(&s);
        let decoded = dec_snapshot(&encoded).expect("decode");
        assert_eq!(decoded, s);
    }
}
//#endregion 🔖️Tests
