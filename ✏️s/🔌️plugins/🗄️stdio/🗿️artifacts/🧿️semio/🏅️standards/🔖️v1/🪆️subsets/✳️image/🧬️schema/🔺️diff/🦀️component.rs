//! 🔺️ SemioImageDiff — sparse per-field diff, handcrafted per `🧬️schema-design.md`'s recipe.
//! `frames` (strong entity, per-field diffable) and `metadata` (weak/name-keyed entity, whole-
//! value diffed) are both index/name-keyed collection triples built on the SHARED
//! `standards::v1::engine::triples` module (`IndexedTripleDiff`/`NamedTripleDiff` +
//! `enc_indexed_triple`/`enc_named_triple`) — no per-subset reinvention of that wire shape, per
//! `w1b-type-ownership.md`. The between/apply/absorb/inverse ALGEBRA over those triple types is
//! hand-rolled locally below (the shared module only owns the wire codec, not the algebra — every
//! subset's collection shape differs enough that a shared generic algebra isn't the right cut),
//! following the docx/gif precedent (`f6-docx-ecma-376-report.md`, `f6-final-summary.md` §4.4). No
//! `snapshot: Option<SemioImageSnapshot>` full-replace slot anywhere.

use crate::artifacts::semio::standards::v1::engine::triples::{
    IndexAdded, IndexModified, IndexedTripleDiff, NamedModified, NamedTripleDiff,
    dec_indexed_triple, dec_named_triple, enc_indexed_triple, enc_named_triple, split_top_level, strip_brackets,
};
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::{
    SemioColorspace, SemioImageFrame, SemioImageMetadataEntry, SemioImageSnapshot,
};
use protocol::MutationDiff;
/// 🔧️ Unconditional — the `#[cfg(test)] mod tests` block below calls `print_diff`/`parse_diff`/
/// `encode_diff`/`decode_diff` via method syntax on `SemioImageDiff`, which needs `DiffCodec` in
/// scope (the `impl protocol::DiffCodec for SemioImageDiff` block itself compiles fine unqualified,
/// but callers using method syntax do not get the trait for free) (W2b closer fix).
use protocol::DiffCodec;
use protocol::command::DiffAlgebra;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️FrameDiff
/// 🔺️ Sparse per-field diff for one [`SemioImageFrame`] — a strong entity, per the recipe.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemioImageFrameDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delay_ms: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rgba8: Option<Vec<u8>>,
}

impl SemioImageFrameDiff {
    pub fn is_empty(&self) -> bool {
        self.delay_ms.is_none() && self.rgba8.is_none()
    }
    pub fn between(base: &SemioImageFrame, other: &SemioImageFrame) -> Self {
        Self {
            delay_ms: (base.delay_ms != other.delay_ms).then_some(other.delay_ms),
            rgba8: (base.rgba8 != other.rgba8).then_some(other.rgba8.clone()),
        }
    }
    pub fn apply(&self, base: &SemioImageFrame) -> SemioImageFrame {
        let mut next = base.clone();
        if let Some(v) = self.delay_ms { next.delay_ms = v; }
        if let Some(v) = &self.rgba8 { next.rgba8 = v.clone(); }
        next
    }
    pub fn inverse(&self, base: &SemioImageFrame) -> Self {
        Self {
            delay_ms: self.delay_ms.map(|_| base.delay_ms),
            rgba8: self.rgba8.as_ref().map(|_| base.rgba8.clone()),
        }
    }
    pub fn absorb(&mut self, other: Self) {
        if other.delay_ms.is_some() { self.delay_ms = other.delay_ms; }
        if other.rgba8.is_some() { self.rgba8 = other.rgba8; }
    }
}
//#endregion 🔖️FrameDiff

//#region 🔖️CollectionTypeAliases
pub type SemioImageFramesDiff = IndexedTripleDiff<SemioImageFrameDiff, SemioImageFrame>;
/// 🏷️ Weak/name-keyed collection: `D = String` (the whole new value — no sub-diffing a scalar).
pub type SemioImageMetadataDiff = NamedTripleDiff<String, String, SemioImageMetadataEntry>;
//#endregion 🔖️CollectionTypeAliases

//#region 🔖️GenericIndexedAlgebra
/// 🧮️ Between (positional, per the recipe's index-keyed matching rule): pairwise-compares
/// `0..min(base,other)` as `modified`, base tail as `removed`, other tail as `added`.
fn between_indexed<T: Clone + PartialEq, D>(base: &[T], other: &[T], diff_item: impl Fn(&T, &T) -> Option<D>) -> Option<IndexedTripleDiff<D, T>> {
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
    if modified.is_empty() && removed.is_empty() && added.is_empty() { None } else { Some(IndexedTripleDiff { removed, modified, added }) }
}

fn apply_indexed<T: Clone, D>(items: &mut Vec<T>, diff: &IndexedTripleDiff<D, T>, apply_item: impl Fn(&T, &D) -> T) {
    for m in &diff.modified {
        if let Some(item) = items.get_mut(m.index) {
            *item = apply_item(item, &m.diff);
        }
    }
    let mut removed_sorted = diff.removed.clone();
    removed_sorted.sort_unstable_by(|a, b| b.cmp(a));
    removed_sorted.dedup();
    for idx in removed_sorted {
        if idx < items.len() { items.remove(idx); }
    }
    let mut additions: Vec<&IndexAdded<T>> = diff.added.iter().collect();
    additions.sort_by_key(|a| a.index);
    for add in additions {
        let at = add.index.min(items.len());
        items.insert(at, add.item.clone());
    }
}

/// 🧮️ Maps a base-side index through a diff's own removed/added to its position once applied.
fn transform_index<T>(idx: usize, removed: &[usize], added: &[IndexAdded<T>]) -> usize {
    let removed_before = removed.iter().filter(|&&r| r < idx).count();
    let pos = idx - removed_before;
    let mut order: Vec<usize> = added.iter().map(|a| a.index).collect();
    order.sort_unstable();
    let mut shift = 0usize;
    for target in order {
        if target <= pos + shift { shift += 1; } else { break; }
    }
    pos + shift
}

fn inverse_indexed<T: Clone, D>(base_items: &[T], diff: &IndexedTripleDiff<D, T>, inverse_item: impl Fn(&T, &D) -> D) -> IndexedTripleDiff<D, T> {
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

enum ItemOrigin { Base(usize), Added(usize) }

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

/// 🧮️ Sequential-coalesce absorb per the recipe's normative algorithm (gif/docx precedent).
fn absorb_indexed<T: Clone, D: Clone>(
    d1: IndexedTripleDiff<D, T>,
    d2: IndexedTripleDiff<D, T>,
    absorb_item: impl Fn(D, D) -> D,
    apply_item: impl Fn(&T, &D) -> T,
) -> IndexedTripleDiff<D, T> {
    let d1_ref_max = d1.removed.iter().copied().chain(d1.modified.iter().map(|m| m.index)).max();
    let mut base_len = d1_ref_max.map(|m| m + 1).unwrap_or(0);
    let mid_len_needed_by_d1 = d1.added.iter().map(|a| a.index + 1).max().unwrap_or(0);
    while base_len.saturating_sub(d1.removed.len()) + d1.added.len() < mid_len_needed_by_d1 { base_len += 1; }
    let d2_ref_max = d2.removed.iter().copied().chain(d2.modified.iter().map(|m| m.index)).max();
    let required_mid_len = d2_ref_max.map(|m| m + 1).unwrap_or(0);
    while base_len.saturating_sub(d1.removed.len()) + d1.added.len() < required_mid_len { base_len += 1; }

    let mid = simulate_mid_origins(base_len, &d1.removed, &d1.added);

    let mut removed = d1.removed.clone();
    let mut modified = d1.modified;
    let mut working_added = d1.added;
    let mut annihilated: std::collections::HashSet<usize> = Default::default();

    for &r2 in &d2.removed {
        match mid.get(r2) {
            Some(ItemOrigin::Base(bi)) => {
                if !removed.contains(bi) { removed.push(*bi); }
                modified.retain(|m| &m.index != bi);
            }
            Some(ItemOrigin::Added(k)) => { annihilated.insert(*k); }
            None => {}
        }
    }
    for m2 in &d2.modified {
        match mid.get(m2.index) {
            Some(ItemOrigin::Base(bi)) => {
                if removed.contains(bi) { continue; }
                match modified.iter_mut().find(|m| &m.index == bi) {
                    Some(existing) => existing.diff = absorb_item(existing.diff.clone(), m2.diff.clone()),
                    None => modified.push(IndexModified { index: *bi, diff: m2.diff.clone() }),
                }
            }
            Some(ItemOrigin::Added(k)) => {
                if annihilated.contains(k) { continue; }
                if let Some(add) = working_added.get_mut(*k) {
                    add.item = apply_item(&add.item, &m2.diff);
                }
            }
            None => {}
        }
    }

    let mut added = Vec::new();
    for (k, add) in working_added.into_iter().enumerate() {
        if annihilated.contains(&k) { continue; }
        let final_index = transform_index(add.index, &d2.removed, &d2.added);
        added.push(IndexAdded { index: final_index, item: add.item });
    }
    for a2 in &d2.added { added.push(a2.clone()); }
    added.sort_by_key(|a| a.index);

    IndexedTripleDiff { removed, modified, added }
}
//#endregion 🔖️GenericIndexedAlgebra

//#region 🔖️GenericNamedAlgebra
fn between_named<K: PartialEq + Clone, T: Clone + PartialEq, D>(base: &[T], other: &[T], key_of: impl Fn(&T) -> K, diff_item: impl Fn(&T, &T) -> Option<D>) -> Option<NamedTripleDiff<K, D, T>> {
    let mut removed = Vec::new();
    let mut modified = Vec::new();
    for b in base {
        let bk = key_of(b);
        match other.iter().find(|o| key_of(o) == bk) {
            None => removed.push(bk),
            Some(o) if o != b => { if let Some(d) = diff_item(b, o) { modified.push(NamedModified { key: bk, diff: d }); } }
            Some(_) => {}
        }
    }
    let mut added = Vec::new();
    for o in other {
        let ok = key_of(o);
        if !base.iter().any(|b| key_of(b) == ok) { added.push(o.clone()); }
    }
    if removed.is_empty() && modified.is_empty() && added.is_empty() { None } else { Some(NamedTripleDiff { removed, modified, added }) }
}

fn apply_named<K: PartialEq + Clone, T: Clone, D>(items: &mut Vec<T>, diff: &NamedTripleDiff<K, D, T>, key_of: impl Fn(&T) -> K, apply_item: impl Fn(&mut T, &D)) {
    items.retain(|i| !diff.removed.contains(&key_of(i)));
    for m in &diff.modified {
        if let Some(item) = items.iter_mut().find(|i| key_of(i) == m.key) { apply_item(item, &m.diff); }
    }
    for item in &diff.added { items.push(item.clone()); }
}

fn inverse_named<K: PartialEq + Clone, T: Clone, D>(base_items: &[T], diff: &NamedTripleDiff<K, D, T>, key_of: impl Fn(&T) -> K, inverse_item: impl Fn(&T, &D) -> D) -> NamedTripleDiff<K, D, T> {
    let removed: Vec<K> = diff.added.iter().map(&key_of).collect();
    let mut modified = Vec::new();
    for m in &diff.modified {
        if let Some(original) = base_items.iter().find(|i| key_of(i) == m.key) {
            modified.push(NamedModified { key: m.key.clone(), diff: inverse_item(original, &m.diff) });
        }
    }
    let mut added = Vec::new();
    for k in &diff.removed {
        if let Some(original) = base_items.iter().find(|i| &key_of(i) == k) { added.push(original.clone()); }
    }
    NamedTripleDiff { removed, modified, added }
}

/// 🧮️ Name-keyed absorb — identity is the KEY, so no index transport is needed.
fn absorb_named<K: PartialEq + Clone, T: Clone, D: Clone>(
    d1: NamedTripleDiff<K, D, T>,
    d2: NamedTripleDiff<K, D, T>,
    key_of: impl Fn(&T) -> K,
    absorb_item: impl Fn(D, D) -> D,
    apply_item: impl Fn(&mut T, &D),
) -> NamedTripleDiff<K, D, T> {
    let d1_added_keys: Vec<K> = d1.added.iter().map(&key_of).collect();
    let mut removed = d1.removed.clone();
    let mut annihilated: Vec<K> = Vec::new();
    for k in &d2.removed {
        if d1_added_keys.contains(k) { annihilated.push(k.clone()); } else if !removed.contains(k) { removed.push(k.clone()); }
    }
    let mut working_added: Vec<T> = d1.added.into_iter().filter(|a| !annihilated.contains(&key_of(a))).collect();
    let mut modified: Vec<NamedModified<K, D>> = d1.modified.into_iter().filter(|m| !removed.contains(&m.key)).collect();
    for m2 in &d2.modified {
        if let Some(added) = working_added.iter_mut().find(|a| key_of(a) == m2.key) { apply_item(added, &m2.diff); continue; }
        if removed.contains(&m2.key) { continue; }
        match modified.iter_mut().find(|m| m.key == m2.key) {
            Some(existing) => existing.diff = absorb_item(existing.diff.clone(), m2.diff.clone()),
            None => modified.push(NamedModified { key: m2.key.clone(), diff: m2.diff.clone() }),
        }
    }
    for a2 in &d2.added {
        let k2 = key_of(a2);
        match working_added.iter_mut().find(|a| key_of(a) == k2) {
            Some(existing) => *existing = a2.clone(),
            None => working_added.push(a2.clone()),
        }
    }
    NamedTripleDiff { removed, modified, added: working_added }
}
//#endregion 🔖️GenericNamedAlgebra

//#region 🔖️CollectionWrappers
fn frames_between(base: &[SemioImageFrame], other: &[SemioImageFrame]) -> Option<SemioImageFramesDiff> {
    between_indexed(base, other, |a, b| { let d = SemioImageFrameDiff::between(a, b); (!d.is_empty()).then_some(d) })
}
fn frames_apply(items: &mut Vec<SemioImageFrame>, diff: &SemioImageFramesDiff) { apply_indexed(items, diff, |item, d| d.apply(item)); }
fn frames_inverse(base: &[SemioImageFrame], diff: &SemioImageFramesDiff) -> SemioImageFramesDiff { inverse_indexed(base, diff, |item, d| d.inverse(item)) }
fn frames_absorb(d1: SemioImageFramesDiff, d2: SemioImageFramesDiff) -> SemioImageFramesDiff {
    absorb_indexed(d1, d2, |mut a, b| { a.absorb(b); a }, |item, d| d.apply(item))
}

fn metadata_key(e: &SemioImageMetadataEntry) -> String { e.key.clone() }
fn metadata_between(base: &[SemioImageMetadataEntry], other: &[SemioImageMetadataEntry]) -> Option<SemioImageMetadataDiff> {
    between_named(base, other, metadata_key, |a, b| (a.value != b.value).then(|| b.value.clone()))
}
fn metadata_apply(items: &mut Vec<SemioImageMetadataEntry>, diff: &SemioImageMetadataDiff) {
    apply_named(items, diff, metadata_key, |item, d| item.value = d.clone());
}
fn metadata_inverse(base: &[SemioImageMetadataEntry], diff: &SemioImageMetadataDiff) -> SemioImageMetadataDiff {
    inverse_named(base, diff, metadata_key, |item, _d| item.value.clone())
}
fn metadata_absorb(d1: SemioImageMetadataDiff, d2: SemioImageMetadataDiff) -> SemioImageMetadataDiff {
    absorb_named(d1, d2, metadata_key, |_old, new| new, |item, d| item.value = d.clone())
}
//#endregion 🔖️CollectionWrappers

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.image.diff")]
pub struct SemioImageDiff {
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub colorspace: Option<SemioColorspace>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bit_depth: Option<u8>,
    /// 🎨️ Tri-state: `None` = unchanged, `Some(None)` = ICC removed, `Some(Some(bytes))` = set.
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icc: Option<Option<Vec<u8>>>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frames: Option<SemioImageFramesDiff>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<SemioImageMetadataDiff>,
}

impl SemioImageDiff {
    pub fn is_empty_diff(&self) -> bool {
        self.width.is_none() && self.height.is_none() && self.colorspace.is_none() && self.bit_depth.is_none()
            && self.icc.is_none() && self.frames.is_none() && self.metadata.is_none()
    }
}

impl MutationDiff<SemioImageSnapshot> for SemioImageDiff {
    fn apply(&self, base: &SemioImageSnapshot) -> SemioImageSnapshot {
        let mut next = base.clone();
        if let Some(v) = self.width { next.width = v; }
        if let Some(v) = self.height { next.height = v; }
        if let Some(v) = self.colorspace { next.colorspace = v; }
        if let Some(v) = self.bit_depth { next.bit_depth = v; }
        if let Some(v) = &self.icc { next.icc = v.clone(); }
        if let Some(d) = &self.frames { frames_apply(&mut next.frames, d); }
        if let Some(d) = &self.metadata { metadata_apply(&mut next.metadata, d); }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.width.is_some() { self.width = other.width; }
        if other.height.is_some() { self.height = other.height; }
        if other.colorspace.is_some() { self.colorspace = other.colorspace; }
        if other.bit_depth.is_some() { self.bit_depth = other.bit_depth; }
        if other.icc.is_some() { self.icc = other.icc; }
        self.frames = match (self.frames.take(), other.frames) {
            (Some(mine), Some(theirs)) => Some(frames_absorb(mine, theirs)),
            (mine, theirs) => mine.or(theirs),
        };
        self.metadata = match (self.metadata.take(), other.metadata) {
            (Some(mine), Some(theirs)) => Some(metadata_absorb(mine, theirs)),
            (mine, theirs) => mine.or(theirs),
        };
    }
}

impl DiffAlgebra<SemioImageSnapshot> for SemioImageDiff {
    fn inverse(&self, base: &SemioImageSnapshot) -> Self {
        Self {
            width: self.width.map(|_| base.width),
            height: self.height.map(|_| base.height),
            colorspace: self.colorspace.map(|_| base.colorspace),
            bit_depth: self.bit_depth.map(|_| base.bit_depth),
            icc: self.icc.as_ref().map(|_| base.icc.clone()),
            frames: self.frames.as_ref().map(|d| frames_inverse(&base.frames, d)),
            metadata: self.metadata.as_ref().map(|d| metadata_inverse(&base.metadata, d)),
        }
    }

    fn between(base: &SemioImageSnapshot, other: &SemioImageSnapshot) -> Self {
        Self {
            width: (base.width != other.width).then_some(other.width),
            height: (base.height != other.height).then_some(other.height),
            colorspace: (base.colorspace != other.colorspace).then_some(other.colorspace),
            bit_depth: (base.bit_depth != other.bit_depth).then_some(other.bit_depth),
            icc: (base.icc != other.icc).then_some(other.icc.clone()),
            frames: frames_between(&base.frames, &other.frames),
            metadata: metadata_between(&base.metadata, &other.metadata),
        }
    }

    fn is_empty(&self) -> bool {
        self.is_empty_diff()
    }
}

/// 🧩 Builds a set-snapshot diff — sparse field-by-field, never a full-replace slot.
pub fn diff_set_snapshot(base: &SemioImageSnapshot, snapshot: &SemioImageSnapshot) -> SemioImageDiff {
    <SemioImageDiff as DiffAlgebra<SemioImageSnapshot>>::between(base, snapshot)
}
//#endregion 🔖️Diff

//#region 🔖️HandcraftedDiffCodec
/// 🧪️ Hand-rolled `protocol::DiffCodec` — `SemioImageDiff` carries a tri-state `Option<Option<T>>`
/// field (`icc`), the same shape gif's `GifDiff`/docx's `DocxDiff` document as blocking the
/// `#[derive(dsl::DslDiff)]` path (f6-final-summary.md §4.3/§4.4; `dsl` has no blanket
/// `Option<T>: DslField` impl). Grammar: one space-separated `name=value` token per changed
/// top-level field; the two collections print as `name{[removed];[modified];[added]}` via the
/// SHARED `enc_indexed_triple`/`enc_named_triple` (see module doc comment). Bytes/strings are
/// lowercase hex — no external base64 dep, no escaping needed. `Option<T>` uses a uniform
/// `[0]`=None / `[1,<T>]`=Some(T) tag.
//#region 🔖️Primitives
fn hex_encode(bytes: &[u8]) -> String { bytes.iter().map(|b| format!("{b:02x}")).collect() }
fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 { return Err(format!("odd hex length: {s:?}")); }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
fn hex_encode_str(s: &str) -> String { hex_encode(s.as_bytes()) }
fn hex_decode_str(s: &str) -> Result<String, String> { String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string()) }
fn parse_u8(s: &str) -> Result<u8, String> { s.parse().map_err(|e: std::num::ParseIntError| e.to_string()) }
fn parse_u32(s: &str) -> Result<u32, String> { s.parse().map_err(|e: std::num::ParseIntError| e.to_string()) }
fn parse_usize(s: &str) -> Result<usize, String> { s.parse().map_err(|e: std::num::ParseIntError| e.to_string()) }

pub(crate) fn encode_option<T>(opt: &Option<T>, enc: impl Fn(&T) -> String) -> String {
    match opt { None => "[0]".to_string(), Some(v) => format!("[1,{}]", enc(v)) }
}
pub(crate) fn decode_option<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Option<T>, String> {
    let inner = strip_brackets(s)?;
    match split_top_level(inner, ',').as_slice() {
        ["0"] => Ok(None),
        [tag, value] if *tag == "1" => Ok(Some(dec(value)?)),
        other => Err(format!("option decode: bad shape {other:?}")),
    }
}
//#endregion 🔖️Primitives

//#region 🔖️ValueCodecs
pub(crate) fn enc_colorspace(c: SemioColorspace) -> char {
    match c { SemioColorspace::Rgb => 'r', SemioColorspace::Rgba => 'a', SemioColorspace::Grayscale => 'g', SemioColorspace::GrayscaleAlpha => 'y', SemioColorspace::Indexed => 'i' }
}
pub(crate) fn dec_colorspace(s: &str) -> Result<SemioColorspace, String> {
    match s {
        "r" => Ok(SemioColorspace::Rgb), "a" => Ok(SemioColorspace::Rgba), "g" => Ok(SemioColorspace::Grayscale),
        "y" => Ok(SemioColorspace::GrayscaleAlpha), "i" => Ok(SemioColorspace::Indexed),
        other => Err(format!("bad colorspace {other:?}")),
    }
}
pub(crate) fn enc_frame(f: &SemioImageFrame) -> String { format!("[{},{}]", f.delay_ms, hex_encode(&f.rgba8)) }
pub(crate) fn dec_frame(s: &str) -> Result<SemioImageFrame, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [delay, rgba] = parts.as_slice() else { return Err(format!("frame: expected 2 fields, got {}", parts.len())) };
    Ok(SemioImageFrame { delay_ms: parse_u32(delay)?, rgba8: hex_decode(rgba)? })
}
fn enc_frame_diff(d: &SemioImageFrameDiff) -> String {
    let mut parts = Vec::new();
    if let Some(v) = d.delay_ms { parts.push(format!("D:{v}")); }
    if let Some(v) = &d.rgba8 { parts.push(format!("X:{}", hex_encode(v))); }
    format!("[{}]", parts.join(","))
}
fn dec_frame_diff(s: &str) -> Result<SemioImageFrameDiff, String> {
    let inner = strip_brackets(s)?;
    let mut d = SemioImageFrameDiff::default();
    for entry in split_top_level(inner, ',') {
        if entry.is_empty() { continue; }
        let (tag, val) = entry.split_once(':').ok_or_else(|| format!("frame diff: bad entry {entry:?}"))?;
        match tag {
            "D" => d.delay_ms = Some(parse_u32(val)?),
            "X" => d.rgba8 = Some(hex_decode(val)?),
            other => return Err(format!("frame diff: unknown tag {other:?}")),
        }
    }
    Ok(d)
}
pub(crate) fn enc_metadata_entry(e: &SemioImageMetadataEntry) -> String { format!("[{},{}]", hex_encode_str(&e.key), hex_encode_str(&e.value)) }
pub(crate) fn dec_metadata_entry(s: &str) -> Result<SemioImageMetadataEntry, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [key, value] = parts.as_slice() else { return Err(format!("metadata entry: expected 2 fields, got {}", parts.len())) };
    Ok(SemioImageMetadataEntry { key: hex_decode_str(key)?, value: hex_decode_str(value)? })
}
//#endregion 🔖️ValueCodecs

//#region 🔖️CollectionCodecs
fn enc_frames_diff(d: &SemioImageFramesDiff) -> String { enc_indexed_triple(d, enc_frame_diff, enc_frame) }
fn dec_frames_diff(s: &str) -> Result<SemioImageFramesDiff, String> { dec_indexed_triple(s, dec_frame_diff, dec_frame) }
fn enc_metadata_diff(d: &SemioImageMetadataDiff) -> String { enc_named_triple(d, |k: &String| hex_encode_str(k), |v: &String| hex_encode_str(v), enc_metadata_entry) }
fn dec_metadata_diff(s: &str) -> Result<SemioImageMetadataDiff, String> { dec_named_triple(s, hex_decode_str, hex_decode_str, dec_metadata_entry) }
//#endregion 🔖️CollectionCodecs

//#region 🔖️TopLevel
fn print_image_diff(d: &SemioImageDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = d.width { tokens.push(format!("width={v}")); }
    if let Some(v) = d.height { tokens.push(format!("height={v}")); }
    if let Some(v) = d.colorspace { tokens.push(format!("colorspace={}", enc_colorspace(v))); }
    if let Some(v) = d.bit_depth { tokens.push(format!("bitDepth={v}")); }
    if let Some(v) = &d.icc { tokens.push(format!("icc={}", encode_option(v, |b| hex_encode(b)))); }
    if let Some(v) = &d.frames { tokens.push(format!("frames{{{}}}", enc_frames_diff(v))); }
    if let Some(v) = &d.metadata { tokens.push(format!("metadata{{{}}}", enc_metadata_diff(v))); }
    tokens.join(" ")
}
fn parse_image_diff(line: &str) -> Result<SemioImageDiff, String> {
    let mut d = SemioImageDiff::default();
    if line.is_empty() { return Ok(d); }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("width=") { d.width = Some(parse_u32(rest)?); }
        else if let Some(rest) = token.strip_prefix("height=") { d.height = Some(parse_u32(rest)?); }
        else if let Some(rest) = token.strip_prefix("colorspace=") { d.colorspace = Some(dec_colorspace(rest)?); }
        else if let Some(rest) = token.strip_prefix("bitDepth=") { d.bit_depth = Some(parse_u8(rest)?); }
        else if let Some(rest) = token.strip_prefix("icc=") { d.icc = Some(decode_option(rest, hex_decode)?); }
        else if let Some(rest) = token.strip_prefix("frames{") { d.frames = Some(dec_frames_diff(rest.strip_suffix('}').ok_or_else(|| "frames: missing closing brace".to_string())?)?); }
        else if let Some(rest) = token.strip_prefix("metadata{") { d.metadata = Some(dec_metadata_diff(rest.strip_suffix('}').ok_or_else(|| "metadata: missing closing brace".to_string())?)?); }
        else { return Err(format!("image diff: unknown token {token:?}")); }
    }
    Ok(d)
}

impl protocol::DiffCodec for SemioImageDiff {
    fn print_diff(&self) -> String { print_image_diff(self) }
    fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        parse_image_diff(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    /// ⚡️ Binary = the text bytes verbatim — same simplification gif's/`WriterDiff`'s hand-rolled
    /// `DiffCodec` uses: satisfies every law (round-trips, deterministic) without a second, denser
    /// wire format this wave's scope didn't call for.
    fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(self.print_diff().into_bytes())
    }
    fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let line = std::str::from_utf8(bytes).map_err(|e| protocol::ProtocolError::Malformed { what: "diff utf8", offset: 0, detail: e.to_string() })?;
        Self::parse_diff(line).map_err(|e| protocol::ProtocolError::Malformed { what: "diff text", offset: 0, detail: e.to_string() })
    }
}
//#endregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA;

    fn frame(seed: u8, len: usize) -> SemioImageFrame {
        SemioImageFrame { delay_ms: 100, rgba8: vec![seed; len] }
    }

    /// 🧪️ Canonical absorb case 1: `InsertFrame(2,f)` then `RemoveFrame(0)` → `{removed:[0],
    /// added:[(1,f)]}`.
    #[test]
    fn absorb_insert_then_remove_before_shifts_index() {
        let f = frame(9, 4);
        let d1 = SemioImageFramesDiff { added: vec![IndexAdded { index: 2, item: f.clone() }], ..Default::default() };
        let d2 = SemioImageFramesDiff { removed: vec![0], ..Default::default() };
        let absorbed = frames_absorb(d1, d2);
        assert_eq!(absorbed.removed, vec![0]);
        assert_eq!(absorbed.added, vec![IndexAdded { index: 1, item: f }]);
        assert!(absorbed.modified.is_empty());
    }

    /// 🧪️ Canonical absorb case 2: `InsertFrame(2,f)` then `InsertFrame(2,g)` → both survive.
    #[test]
    fn absorb_insert_insert_same_index_both_survive() {
        let f = frame(1, 4);
        let g = frame(2, 4);
        let d1 = SemioImageFramesDiff { added: vec![IndexAdded { index: 2, item: f.clone() }], ..Default::default() };
        let d2 = SemioImageFramesDiff { added: vec![IndexAdded { index: 2, item: g.clone() }], ..Default::default() };
        let absorbed = frames_absorb(d1, d2);
        assert_eq!(absorbed.added, vec![IndexAdded { index: 2, item: g }, IndexAdded { index: 3, item: f }]);
    }

    /// 🧪️ Canonical absorb case 3: `InsertFrame(1,f)` then `SetFrameDelay(1,42)` patches INTO the
    /// added payload.
    #[test]
    fn absorb_insert_then_set_field_patches_into_added() {
        let f = frame(1, 4);
        let d1 = SemioImageFramesDiff { added: vec![IndexAdded { index: 1, item: f.clone() }], ..Default::default() };
        let d2 = SemioImageFramesDiff { modified: vec![IndexModified { index: 1, diff: SemioImageFrameDiff { delay_ms: Some(42), rgba8: None } }], ..Default::default() };
        let absorbed = frames_absorb(d1, d2);
        assert!(absorbed.modified.is_empty());
        assert_eq!(absorbed.added.len(), 1);
        assert_eq!(absorbed.added[0].item.delay_ms, 42);
        assert_eq!(absorbed.added[0].index, 1);
    }

    /// 🧪️ Canonical absorb case 4: Modify+Remove annihilates the modify.
    #[test]
    fn absorb_modify_then_remove_drops_modify() {
        let base = SemioImageSnapshot { frames: vec![frame(1, 4), frame(2, 4)], ..SemioImageSnapshot::default() };
        let mid = { let mut s = base.clone(); s.frames[1].delay_ms = 50; s };
        let after = { let mut s = mid.clone(); s.frames.remove(1); s };
        let d1 = frames_between(&base.frames, &mid.frames).expect("modify diff");
        let d2 = frames_between(&mid.frames, &after.frames).expect("remove diff");
        let absorbed = frames_absorb(d1, d2);
        assert!(absorbed.modified.is_empty());
        assert_eq!(absorbed.removed, vec![1]);
    }

    #[test]
    fn absorb_law_holds_over_curated_ops() {
        let base = SemioImageSnapshot { frames: vec![frame(1, 4), frame(2, 4), frame(3, 4)], metadata: vec![SemioImageMetadataEntry { key: "a".into(), value: "1".into() }], ..SemioImageSnapshot::default() };
        let mid = {
            let mut s = base.clone();
            s.frames.insert(1, frame(9, 4));
            s.frames.remove(0);
            s.metadata.push(SemioImageMetadataEntry { key: "b".into(), value: "2".into() });
            s
        };
        let after = {
            let mut s = mid.clone();
            s.frames[0].delay_ms = 250;
            s.frames.push(frame(5, 4));
            s.metadata[0].value = "1-changed".into();
            s
        };
        let mut d1 = <SemioImageDiff as DiffAlgebra<SemioImageSnapshot>>::between(&base, &mid);
        let d2 = <SemioImageDiff as DiffAlgebra<SemioImageSnapshot>>::between(&mid, &after);
        d1.absorb(d2);
        assert_eq!(d1.apply(&base), after);
    }

    #[test]
    fn between_roundtrip_law() {
        let a = SemioImageSnapshot { width: 4, height: 4, frames: vec![frame(1, 16)], ..SemioImageSnapshot::default() };
        let b = SemioImageSnapshot { width: 4, height: 4, frames: vec![frame(1, 16), frame(2, 4)], colorspace: SemioColorspace::Grayscale, ..SemioImageSnapshot::default() };
        let ab = <SemioImageDiff as DiffAlgebra<SemioImageSnapshot>>::between(&a, &b);
        assert_eq!(ab.apply(&a), b);
        let ba = <SemioImageDiff as DiffAlgebra<SemioImageSnapshot>>::between(&b, &a);
        assert_eq!(ba.apply(&b), a);
        assert!(<SemioImageDiff as DiffAlgebra<SemioImageSnapshot>>::between(&a, &a).is_empty());
    }

    #[test]
    fn inverse_law() {
        let base = SemioImageSnapshot { frames: vec![frame(1, 4), frame(2, 4)], icc: Some(vec![1, 2]), ..SemioImageSnapshot::default() };
        let next = {
            let mut s = base.clone();
            s.frames[0].delay_ms = 400;
            s.frames.remove(1);
            s.frames.push(frame(7, 9));
            s.icc = None;
            s.metadata.push(SemioImageMetadataEntry { key: "k".into(), value: "v".into() });
            s
        };
        let d = <SemioImageDiff as DiffAlgebra<SemioImageSnapshot>>::between(&base, &next);
        let mutated = d.apply(&base);
        let inv = d.inverse(&base);
        assert_eq!(inv.apply(&mutated), base);
    }

    /// 🧪️ field_sweep — THE acceptance criterion: `sweep_a`/`sweep_b` differ in every mutable
    /// field, including the `icc` tri-state exercising BOTH `Some(Some(_))` and `Some(None)`, and
    /// asymmetric collection lengths (a single same-direction `between()` shows removed XOR
    /// added, never both — split across both directions).
    #[test]
    fn field_sweep() {
        let sweep_a = SemioImageSnapshot {
            schema: STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA.into(),
            width: 10, height: 8,
            colorspace: SemioColorspace::Rgb,
            bit_depth: 8,
            frames: vec![frame(1, 4), frame(2, 4)],
            icc: Some(vec![1, 2, 3]),
            metadata: vec![
                SemioImageMetadataEntry { key: "keep".into(), value: "old".into() },
                SemioImageMetadataEntry { key: "gone".into(), value: "bye".into() },
            ],
        };
        let sweep_b = SemioImageSnapshot {
            schema: STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA.into(),
            width: 20, height: 16,
            colorspace: SemioColorspace::GrayscaleAlpha,
            bit_depth: 16,
            frames: vec![
                { let mut f = frame(1, 4); f.delay_ms = 500; f },
                frame(6, 9),
                frame(7, 9),
            ],
            icc: None,
            metadata: vec![
                SemioImageMetadataEntry { key: "keep".into(), value: "new".into() },
                SemioImageMetadataEntry { key: "fresh".into(), value: "hi".into() },
            ],
        };

        let ab = <SemioImageDiff as DiffAlgebra<SemioImageSnapshot>>::between(&sweep_a, &sweep_b);
        assert_eq!(ab.apply(&sweep_a), sweep_b);
        assert!(ab.width.is_some());
        assert!(ab.height.is_some());
        assert!(ab.colorspace.is_some());
        assert!(ab.bit_depth.is_some());
        assert_eq!(ab.icc, Some(None), "icc Some->None must be tri-state Some(None)");
        let frames_ab = ab.frames.as_ref().expect("frames must differ");
        assert!(!frames_ab.modified.is_empty(), "sweep must exercise a modified frame");
        assert!(!frames_ab.added.is_empty(), "sweep must exercise an added frame (b is longer)");
        assert!(frames_ab.modified[0].diff.delay_ms.is_some());
        let metadata_ab = ab.metadata.as_ref().expect("metadata must differ");
        assert!(!metadata_ab.modified.is_empty(), "metadata: modified not exercised");
        assert!(!metadata_ab.removed.is_empty(), "metadata: removed not exercised");
        assert!(!metadata_ab.added.is_empty(), "metadata: added not exercised");

        let ba = <SemioImageDiff as DiffAlgebra<SemioImageSnapshot>>::between(&sweep_b, &sweep_a);
        assert_eq!(ba.apply(&sweep_b), sweep_a);
        assert_eq!(ba.icc, Some(Some(vec![1, 2, 3])), "icc None->Some must be tri-state Some(Some(_))");
        let frames_ba = ba.frames.as_ref().expect("frames must differ");
        assert!(!frames_ba.removed.is_empty(), "reverse direction must exercise a removed frame (a is shorter)");

        assert!(<SemioImageDiff as DiffAlgebra<SemioImageSnapshot>>::between(&sweep_a, &sweep_a).is_empty());
    }

    /// 🧪️ `DiffCodec` round-trip laws for the hand-rolled `SemioImageDiff` text/binary grammar —
    /// scalars, the `icc` tri-state, and both collection triples simultaneously via a real
    /// `between()` result.
    #[test]
    fn diff_codec_text_binary_roundtrip_law() {
        let a = SemioImageSnapshot {
            width: 10, height: 8, colorspace: SemioColorspace::Rgb, bit_depth: 8,
            frames: vec![frame(1, 4), frame(2, 4)],
            icc: Some(vec![1, 2, 3]),
            metadata: vec![SemioImageMetadataEntry { key: "keep".into(), value: "old".into() }],
            ..SemioImageSnapshot::default()
        };
        let b = SemioImageSnapshot {
            width: 20, height: 16, colorspace: SemioColorspace::GrayscaleAlpha, bit_depth: 16,
            frames: vec![{ let mut f = frame(1, 4); f.delay_ms = 500; f }, frame(6, 9)],
            icc: None,
            metadata: vec![SemioImageMetadataEntry { key: "keep".into(), value: "new".into() }, SemioImageMetadataEntry { key: "fresh".into(), value: "hi".into() }],
            ..SemioImageSnapshot::default()
        };
        let cases = vec![
            SemioImageDiff::default(),
            <SemioImageDiff as DiffAlgebra<SemioImageSnapshot>>::between(&a, &b),
            <SemioImageDiff as DiffAlgebra<SemioImageSnapshot>>::between(&b, &a),
        ];
        for d in cases {
            let printed = d.print_diff();
            assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
            let parsed = SemioImageDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
            assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

            let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
            let decoded = SemioImageDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
            assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
        }
    }
}
//#endregion 🔖️Tests
