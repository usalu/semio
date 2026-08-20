//! 🔺️ SemioImageDiff — sparse per-field diff, handcrafted per `🧬️schema-design.md`'s recipe.
//! `frames` (strong entity, per-field diffable) and `metadata` (weak/name-keyed entity, whole-
//! value diffed) are both index/name-keyed collection triples built on the SHARED
//! `standards::v1::subsets::any::schema::triples` module (`IndexedTripleDiff`/`NamedTripleDiff` +
//! `enc_indexed_triple`/`enc_named_triple`) — no per-subset reinvention of that wire shape, per
//! `w1b-type-ownership.md`. The between/apply/absorb/inverse ALGEBRA over those triple types is
//! hand-rolled locally below (the shared module only owns the wire codec, not the algebra — every
//! subset's collection shape differs enough that a shared generic algebra isn't the right cut),
//! following the docx/gif precedent (`f6-docx-ecma-376-report.md`, `f6-final-summary.md` §4.4). No
//! `snapshot: Option<SemioImageSnapshot>` full-replace slot anywhere.

use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::{
    dec_indexed_triple, dec_named_triple, enc_indexed_triple, enc_named_triple, split_top_level, strip_brackets, IndexAdded, IndexModified, IndexedTripleDiff, NamedModified, NamedTripleDiff,
};
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::{SemioColorspace, SemioImageFrame, SemioImageMetadataEntry, SemioImageSnapshot};
use protocol::command::DiffAlgebra;
/// 🔧️ Unconditional — the `#[cfg(test)] mod tests` block below calls `print_diff`/`parse_diff`/
/// `encode_diff`/`decode_diff` via method syntax on `SemioImageDiff`, which needs `DiffCodec` in
/// scope (the `impl protocol::DiffCodec for SemioImageDiff` block itself compiles fine unqualified,
/// but callers using method syntax do not get the trait for free) (W2b closer fix).
use protocol::DiffCodec;
use protocol::MutationDiff;
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
    pub async fn is_empty(&self) -> bool {
        self.delay_ms.is_none() && self.rgba8.is_none()
    }
    pub async fn between(base: &SemioImageFrame, other: &SemioImageFrame) -> Self {
        Self { delay_ms: (base.delay_ms != other.delay_ms).then_some(other.delay_ms), rgba8: (base.rgba8 != other.rgba8).then_some(other.rgba8.clone()) }
    }
    pub async fn apply(&self, base: &SemioImageFrame) -> SemioImageFrame {
        let mut next = base.clone();
        if let Some(v) = self.delay_ms {
            next.delay_ms = v;
        }
        if let Some(v) = &self.rgba8 {
            next.rgba8 = v.clone();
        }
        next
    }
    pub async fn inverse(&self, base: &SemioImageFrame) -> Self {
        Self { delay_ms: self.delay_ms.map(|_| base.delay_ms), rgba8: self.rgba8.as_ref().map(|_| base.rgba8.clone()) }
    }
    pub async fn absorb(&mut self, other: Self) {
        if other.delay_ms.is_some() {
            self.delay_ms = other.delay_ms;
        }
        if other.rgba8.is_some() {
            self.rgba8 = other.rgba8;
        }
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
async fn between_indexed<T: Clone + PartialEq, D>(base: &[T], other: &[T], diff_item: impl Fn(&T, &T) -> Option<D>) -> Option<IndexedTripleDiff<D, T>> {
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

async fn apply_indexed<T: Clone, D>(items: &mut Vec<T>, diff: &IndexedTripleDiff<D, T>, apply_item: impl Fn(&T, &D) -> T) {
    for m in &diff.modified {
        if let Some(item) = items.get_mut(m.index) {
            *item = apply_item(item, &m.diff);
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

/// 🧮️ Maps a base-side index through a diff's own removed/added to its position once applied.
async fn transform_index<T>(idx: usize, removed: &[usize], added: &[IndexAdded<T>]) -> usize {
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

async fn inverse_indexed<T: Clone, D>(base_items: &[T], diff: &IndexedTripleDiff<D, T>, inverse_item: impl Fn(&T, &D) -> D) -> IndexedTripleDiff<D, T> {
    let removed: Vec<usize> = diff.added.iter().map(|a| a.index).collect();
    let mut modified = Vec::new();
    for m in &diff.modified {
        if let Some(original) = base_items.get(m.index) {
            let next_index = transform_index(m.index, &diff.removed, &diff.added);
            modified.push(IndexModified { index: next_index.await, diff: inverse_item(original, &m.diff) });
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

enum ItemOrigin {
    Base(usize),
    Added(usize),
}

async fn simulate_mid_origins<T>(base_len: usize, removed: &[usize], added: &[IndexAdded<T>]) -> Vec<ItemOrigin> {
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
async fn absorb_indexed<T: Clone, D: Clone>(d1: IndexedTripleDiff<D, T>, d2: IndexedTripleDiff<D, T>, absorb_item: impl Fn(D, D) -> D, apply_item: impl Fn(&T, &D) -> T) -> IndexedTripleDiff<D, T> {
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

    let mid = simulate_mid_origins(base_len, &d1.removed, &d1.added).await;

    let mut removed = d1.removed.clone();
    let mut modified = d1.modified;
    let mut working_added = d1.added;
    let mut annihilated: std::collections::HashSet<usize> = Default::default();

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
        added.push(IndexAdded { index: final_index.await, item: add.item });
    }
    for a2 in &d2.added {
        added.push(a2.clone());
    }
    added.sort_by_key(|a| a.index);

    IndexedTripleDiff { removed, modified, added }
}
//#endregion 🔖️GenericIndexedAlgebra

//#region 🔖️GenericNamedAlgebra
async fn between_named<K: PartialEq + Clone, T: Clone + PartialEq, D>(base: &[T], other: &[T], key_of: impl Fn(&T) -> K, diff_item: impl Fn(&T, &T) -> Option<D>) -> Option<NamedTripleDiff<K, D, T>> {
    let mut removed = Vec::new();
    let mut modified = Vec::new();
    for b in base {
        let bk = key_of(b);
        match other.iter().find(|o| key_of(o) == bk) {
            None => removed.push(bk),
            Some(o) if o != b => {
                if let Some(d) = diff_item(b, o) {
                    modified.push(NamedModified { key: bk, diff: d });
                }
            }
            Some(_) => {}
        }
    }
    let mut added = Vec::new();
    for o in other {
        let ok = key_of(o);
        if !base.iter().any(|b| key_of(b) == ok) {
            added.push(o.clone());
        }
    }
    if removed.is_empty() && modified.is_empty() && added.is_empty() {
        None
    } else {
        Some(NamedTripleDiff { removed, modified, added })
    }
}

async fn apply_named<K: PartialEq + Clone, T: Clone, D>(items: &mut Vec<T>, diff: &NamedTripleDiff<K, D, T>, key_of: impl Fn(&T) -> K, apply_item: impl Fn(&mut T, &D)) {
    items.retain(|i| !diff.removed.contains(&key_of(i)));
    for m in &diff.modified {
        if let Some(item) = items.iter_mut().find(|i| key_of(i) == m.key) {
            apply_item(item, &m.diff);
        }
    }
    for item in &diff.added {
        items.push(item.clone());
    }
}

async fn inverse_named<K: PartialEq + Clone, T: Clone, D>(base_items: &[T], diff: &NamedTripleDiff<K, D, T>, key_of: impl Fn(&T) -> K, inverse_item: impl Fn(&T, &D) -> D) -> NamedTripleDiff<K, D, T> {
    let removed: Vec<K> = diff.added.iter().map(&key_of).collect();
    let mut modified = Vec::new();
    for m in &diff.modified {
        if let Some(original) = base_items.iter().find(|i| key_of(i) == m.key) {
            modified.push(NamedModified { key: m.key.clone(), diff: inverse_item(original, &m.diff) });
        }
    }
    let mut added = Vec::new();
    for k in &diff.removed {
        if let Some(original) = base_items.iter().find(|i| &key_of(i) == k) {
            added.push(original.clone());
        }
    }
    NamedTripleDiff { removed, modified, added }
}

/// 🧮️ Name-keyed absorb — identity is the KEY, so no index transport is needed.
async fn absorb_named<K: PartialEq + Clone, T: Clone, D: Clone>(d1: NamedTripleDiff<K, D, T>, d2: NamedTripleDiff<K, D, T>, key_of: impl Fn(&T) -> K, absorb_item: impl Fn(D, D) -> D, apply_item: impl Fn(&mut T, &D)) -> NamedTripleDiff<K, D, T> {
    let d1_added_keys: Vec<K> = d1.added.iter().map(&key_of).collect();
    let mut removed = d1.removed.clone();
    let mut annihilated: Vec<K> = Vec::new();
    for k in &d2.removed {
        if d1_added_keys.contains(k) {
            annihilated.push(k.clone());
        } else if !removed.contains(k) {
            removed.push(k.clone());
        }
    }
    let mut working_added: Vec<T> = d1.added.into_iter().filter(|a| !annihilated.contains(&key_of(a))).collect();
    let mut modified: Vec<NamedModified<K, D>> = d1.modified.into_iter().filter(|m| !removed.contains(&m.key)).collect();
    for m2 in &d2.modified {
        if let Some(added) = working_added.iter_mut().find(|a| key_of(a) == m2.key) {
            apply_item(added, &m2.diff);
            continue;
        }
        if removed.contains(&m2.key) {
            continue;
        }
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
async fn frames_between(base: &[SemioImageFrame], other: &[SemioImageFrame]) -> Option<SemioImageFramesDiff> {
    between_indexed(base, other, |a, b| {
        let d = semio_framework_plugin::resolve_ready(SemioImageFrameDiff::between(a, b));
        (!semio_framework_plugin::resolve_ready(d.is_empty())).then_some(d)
    }).await
}
async fn frames_apply(items: &mut Vec<SemioImageFrame>, diff: &SemioImageFramesDiff) {
    apply_indexed(items, diff, |item, d| d.apply(item));
}
async fn frames_inverse(base: &[SemioImageFrame], diff: &SemioImageFramesDiff) -> SemioImageFramesDiff {
    inverse_indexed(base, diff, |item, d| d.inverse(item)).await
}
async fn frames_absorb(d1: SemioImageFramesDiff, d2: SemioImageFramesDiff) -> SemioImageFramesDiff {
    absorb_indexed(
        d1,
        d2,
        |mut a, b| {
            a.absorb(b);
            a
        },
        |item, d| d.apply(item),
    ).await
}

async fn metadata_key(e: &SemioImageMetadataEntry) -> String {
    e.key.clone()
}
async fn metadata_between(base: &[SemioImageMetadataEntry], other: &[SemioImageMetadataEntry]) -> Option<SemioImageMetadataDiff> {
    between_named(base, other, metadata_key, |a, b| (a.value != b.value).then(|| b.value.clone()))
}
async fn metadata_apply(items: &mut Vec<SemioImageMetadataEntry>, diff: &SemioImageMetadataDiff) {
    apply_named(items, diff, metadata_key, |item, d| item.value = d.clone());
}
async fn metadata_inverse(base: &[SemioImageMetadataEntry], diff: &SemioImageMetadataDiff) -> SemioImageMetadataDiff {
    inverse_named(base, diff, metadata_key, |item, _d| item.value.clone()).await
}
async fn metadata_absorb(d1: SemioImageMetadataDiff, d2: SemioImageMetadataDiff) -> SemioImageMetadataDiff {
    absorb_named(d1, d2, metadata_key, |_old, new| new, |item, d| item.value = d.clone()).await
}
//#endregion 🔖️CollectionWrappers

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.image.diff")]
pub struct SemioImageDiff {
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub colorspace: Option<SemioColorspace>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bit_depth: Option<u8>,
    /// 🎨️ Tri-state: `None` = unchanged, `Some(None)` = ICC removed, `Some(Some(bytes))` = set.
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icc: Option<Option<Vec<u8>>>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frames: Option<SemioImageFramesDiff>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<SemioImageMetadataDiff>,
}

impl SemioImageDiff {
    pub async fn is_empty_diff(&self) -> bool {
        self.width.is_none() && self.height.is_none() && self.colorspace.is_none() && self.bit_depth.is_none() && self.icc.is_none() && self.frames.is_none() && self.metadata.is_none()
    }
}

impl MutationDiff<SemioImageSnapshot> for SemioImageDiff {
    async fn apply(&self, base: &SemioImageSnapshot) -> protocol::MutationApplyResult<SemioImageSnapshot> {
        let mut next = base.clone();
        if let Some(v) = self.width {
            next.width = v;
        }
        if let Some(v) = self.height {
            next.height = v;
        }
        if let Some(v) = self.colorspace {
            next.colorspace = v;
        }
        if let Some(v) = self.bit_depth {
            next.bit_depth = v;
        }
        if let Some(v) = &self.icc {
            next.icc = v.clone();
        }
        if let Some(d) = &self.frames {
            crate::artifacts::semio::standards::v1::subsets::any::schema::triples::validate_indexed_triple(d, next.frames.len(), ["frames"]).await?;
            frames_apply(&mut next.frames, d);
        }
        if let Some(d) = &self.metadata {
            crate::artifacts::semio::standards::v1::subsets::any::schema::triples::validate_named_triple(&next.metadata, d, |item| item.key.clone(), |item| item.key.clone(), ["metadata"]).await?;
            metadata_apply(&mut next.metadata, d);
        }
        Ok(next)
    }

    async fn absorb(&mut self, other: Self) {
        if other.width.is_some() {
            self.width = other.width;
        }
        if other.height.is_some() {
            self.height = other.height;
        }
        if other.colorspace.is_some() {
            self.colorspace = other.colorspace;
        }
        if other.bit_depth.is_some() {
            self.bit_depth = other.bit_depth;
        }
        if other.icc.is_some() {
            self.icc = other.icc;
        }
        self.frames = match (self.frames.take(), other.frames) {
            (Some(mine), Some(theirs)) => Some(frames_absorb(mine, theirs).await),
            (mine, theirs) => mine.or(theirs),
        };
        self.metadata = match (self.metadata.take(), other.metadata) {
            (Some(mine), Some(theirs)) => Some(metadata_absorb(mine, theirs).await),
            (mine, theirs) => mine.or(theirs),
        };
    }
}

impl DiffAlgebra<SemioImageSnapshot> for SemioImageDiff {
    async fn inverse(&self, base: &SemioImageSnapshot) -> Self {
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

    async fn between(base: &SemioImageSnapshot, other: &SemioImageSnapshot) -> Self {
        Self {
            width: (base.width != other.width).then_some(other.width),
            height: (base.height != other.height).then_some(other.height),
            colorspace: (base.colorspace != other.colorspace).then_some(other.colorspace),
            bit_depth: (base.bit_depth != other.bit_depth).then_some(other.bit_depth),
            icc: (base.icc != other.icc).then_some(other.icc.clone()),
            frames: frames_between(&base.frames, &other.frames).await,
            metadata: metadata_between(&base.metadata, &other.metadata).await,
        }
    }

    async fn is_empty(&self) -> bool {
        self.is_empty_diff().await
    }
}

/// 🧩 Builds a set-snapshot diff — sparse field-by-field, never a full-replace slot.
pub async fn diff_set_snapshot(base: &SemioImageSnapshot, snapshot: &SemioImageSnapshot) -> SemioImageDiff {
    <SemioImageDiff as DiffAlgebra<SemioImageSnapshot>>::between(base, snapshot).await
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
async fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
async fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
async fn hex_encode_str(s: &str) -> String {
    hex_encode(s.as_bytes()).await
}
async fn hex_decode_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s).await?).map_err(|e| e.to_string())
}
async fn parse_u8(s: &str) -> Result<u8, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}
async fn parse_u32(s: &str) -> Result<u32, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}
/// 🧪️ Real LEB128-varint-length-prefixed binary primitives (`store::pack_rt::write_varint_u64` /
/// `store::ByteReader`, same helpers this subset's own `📸️snapshot` facet's `ArtifactPack` uses)
/// backing the real `DiffCodec::encode_diff`/`decode_diff` below.
async fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    store::pack_rt::write_varint_u64(out, s.len() as u64);
    out.extend_from_slice(s.as_bytes());
}
async fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    let len = reader.read_varint_u64().await.map_err(|e| e.to_string())? as usize;
    let bytes = reader.read_bytes(len).await.map_err(|e| e.to_string())?.to_vec();
    String::from_utf8(bytes).map_err(|e| e.to_string())
}

pub(crate) async fn encode_option<T>(opt: &Option<T>, enc: impl Fn(&T) -> String) -> String {
    match opt {
        None => "[0]".to_string(),
        Some(v) => format!("[1,{}]", enc(v)),
    }
}
pub(crate) async fn decode_option<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Option<T>, String> {
    let inner = strip_brackets(s).await?;
    match split_top_level(inner, ',').await.as_slice() {
        ["0"] => Ok(None),
        [tag, value] if *tag == "1" => Ok(Some(dec(value)?)),
        other => Err(format!("option decode: bad shape {other:?}")),
    }
}
//#endregion 🔖️Primitives

//#region 🔖️ValueCodecs
pub(crate) async fn enc_colorspace(c: SemioColorspace) -> char {
    match c {
        SemioColorspace::Rgb => 'r',
        SemioColorspace::Rgba => 'a',
        SemioColorspace::Grayscale => 'g',
        SemioColorspace::GrayscaleAlpha => 'y',
        SemioColorspace::Indexed => 'i',
    }
}
pub(crate) async fn dec_colorspace(s: &str) -> Result<SemioColorspace, String> {
    match s {
        "r" => Ok(SemioColorspace::Rgb),
        "a" => Ok(SemioColorspace::Rgba),
        "g" => Ok(SemioColorspace::Grayscale),
        "y" => Ok(SemioColorspace::GrayscaleAlpha),
        "i" => Ok(SemioColorspace::Indexed),
        other => Err(format!("bad colorspace {other:?}")),
    }
}
pub(crate) async fn enc_frame(f: &SemioImageFrame) -> String {
    format!("[{},{}]", f.delay_ms, hex_encode(&f.rgba8))
}
pub(crate) async fn dec_frame(s: &str) -> Result<SemioImageFrame, String> {
    let parts = split_top_level(strip_brackets(s).await?, ',').await;
    let [delay, rgba] = parts.as_slice() else { return Err(format!("frame: expected 2 fields, got {}", parts.len())) };
    Ok(SemioImageFrame { delay_ms: parse_u32(delay).await?, rgba8: hex_decode(rgba).await? })
}
async fn enc_frame_diff(d: &SemioImageFrameDiff) -> String {
    let mut parts = Vec::new();
    if let Some(v) = d.delay_ms {
        parts.push(format!("D:{v}"));
    }
    if let Some(v) = &d.rgba8 {
        parts.push(format!("X:{}", hex_encode(v)));
    }
    format!("[{}]", parts.join(","))
}
async fn dec_frame_diff(s: &str) -> Result<SemioImageFrameDiff, String> {
    let inner = strip_brackets(s).await?;
    let mut d = SemioImageFrameDiff::default();
    for entry in split_top_level(inner, ',') {
        if entry.is_empty() {
            continue;
        }
        let (tag, val) = entry.split_once(':').ok_or_else(|| format!("frame diff: bad entry {entry:?}"))?;
        match tag {
            "D" => d.delay_ms = Some(parse_u32(val).await?),
            "X" => d.rgba8 = Some(hex_decode(val).await?),
            other => return Err(format!("frame diff: unknown tag {other:?}")),
        }
    }
    Ok(d)
}
pub(crate) async fn enc_metadata_entry(e: &SemioImageMetadataEntry) -> String {
    format!("[{},{}]", hex_encode_str(&e.key), hex_encode_str(&e.value))
}
pub(crate) async fn dec_metadata_entry(s: &str) -> Result<SemioImageMetadataEntry, String> {
    let parts = split_top_level(strip_brackets(s).await?, ',').await;
    let [key, value] = parts.as_slice() else { return Err(format!("metadata entry: expected 2 fields, got {}", parts.len())) };
    Ok(SemioImageMetadataEntry { key: hex_decode_str(key).await?, value: hex_decode_str(value).await? })
}
//#endregion 🔖️ValueCodecs

//#region 🔖️CollectionCodecs
async fn enc_frames_diff(d: &SemioImageFramesDiff) -> String {
    enc_indexed_triple(d, enc_frame_diff, enc_frame).await
}
async fn dec_frames_diff(s: &str) -> Result<SemioImageFramesDiff, String> {
    dec_indexed_triple(s, dec_frame_diff, dec_frame).await
}
async fn enc_metadata_diff(d: &SemioImageMetadataDiff) -> String {
    enc_named_triple(d, |k: &String| hex_encode_str(k), |v: &String| hex_encode_str(v), enc_metadata_entry).await
}
async fn dec_metadata_diff(s: &str) -> Result<SemioImageMetadataDiff, String> {
    dec_named_triple(s, hex_decode_str, hex_decode_str, dec_metadata_entry).await
}
//#endregion 🔖️CollectionCodecs

//#region 🔖️TopLevel
async fn print_image_diff(d: &SemioImageDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = d.width {
        tokens.push(format!("width={v}"));
    }
    if let Some(v) = d.height {
        tokens.push(format!("height={v}"));
    }
    if let Some(v) = d.colorspace {
        tokens.push(format!("colorspace={}", enc_colorspace(v)));
    }
    if let Some(v) = d.bit_depth {
        tokens.push(format!("bitDepth={v}"));
    }
    if let Some(v) = &d.icc {
        tokens.push(format!("icc={}", encode_option(v, |b| hex_encode(b))));
    }
    if let Some(v) = &d.frames {
        tokens.push(format!("frames{{{}}}", enc_frames_diff(v)));
    }
    if let Some(v) = &d.metadata {
        tokens.push(format!("metadata{{{}}}", enc_metadata_diff(v)));
    }
    tokens.join(" ")
}
async fn parse_image_diff(line: &str) -> Result<SemioImageDiff, String> {
    let mut d = SemioImageDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("width=") {
            d.width = Some(parse_u32(rest).await?);
        } else if let Some(rest) = token.strip_prefix("height=") {
            d.height = Some(parse_u32(rest).await?);
        } else if let Some(rest) = token.strip_prefix("colorspace=") {
            d.colorspace = Some(dec_colorspace(rest).await?);
        } else if let Some(rest) = token.strip_prefix("bitDepth=") {
            d.bit_depth = Some(parse_u8(rest).await?);
        } else if let Some(rest) = token.strip_prefix("icc=") {
            d.icc = Some(decode_option(rest, hex_decode).await?);
        } else if let Some(rest) = token.strip_prefix("frames{") {
            d.frames = Some(dec_frames_diff(rest.strip_suffix('}').ok_or_else(|| "frames: missing closing brace".to_string())?).await?);
        } else if let Some(rest) = token.strip_prefix("metadata{") {
            d.metadata = Some(dec_metadata_diff(rest.strip_suffix('}').ok_or_else(|| "metadata: missing closing brace".to_string())?).await?);
        } else {
            return Err(format!("image diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl DiffCodec for SemioImageDiff {
    async fn print_diff(&self) -> String {
        print_image_diff(self).await
    }
    async fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        parse_image_diff(line).await.map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    /// ⚡️ Real binary diff frame, replacing the old `print_diff().into_bytes()` text-as-binary
    /// shortcut. `format u8` + `presence u8` (bit0=`width` bit1=`height` bit2=`colorspace`
    /// bit3=`bitDepth` bit4=`icc` bit5=`frames` bit6=`metadata`) are two REAL fixed fields; each
    /// present field then follows as its own varint-length-prefixed opaque text blob (the same
    /// per-field `enc_*`/`enc_frames_diff`/`enc_metadata_diff` text `print_diff` already produces)
    /// — independently-delimited segments rather than one bare trailing `bytes` because there can
    /// be 0-7 of them (chaining a `Cond` per-segment hits the `protocol-cond-cannot-chain` gap: a
    /// second `if`-guard on a field that was itself only conditionally decoded hard-errors
    /// `eval_cond` — see `✳️flow`'s/`✳️mesh`'s pilot reports).
    async fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const DIFF_BINARY_FORMAT: u8 = 1;
        let mut presence = 0u8;
        if self.width.is_some() {
            presence |= 0b0000_0001;
        }
        if self.height.is_some() {
            presence |= 0b0000_0010;
        }
        if self.colorspace.is_some() {
            presence |= 0b0000_0100;
        }
        if self.bit_depth.is_some() {
            presence |= 0b0000_1000;
        }
        if self.icc.is_some() {
            presence |= 0b0001_0000;
        }
        if self.frames.is_some() {
            presence |= 0b0010_0000;
        }
        if self.metadata.is_some() {
            presence |= 0b0100_0000;
        }
        let mut out = vec![DIFF_BINARY_FORMAT, presence];
        if let Some(v) = self.width {
            write_str_lp(&mut out, &v.to_string());
        }
        if let Some(v) = self.height {
            write_str_lp(&mut out, &v.to_string());
        }
        if let Some(v) = self.colorspace {
            write_str_lp(&mut out, &enc_colorspace(v).to_string());
        }
        if let Some(v) = self.bit_depth {
            write_str_lp(&mut out, &v.to_string());
        }
        if let Some(v) = &self.icc {
            write_str_lp(&mut out, &encode_option(v, |b| hex_encode(b)));
        }
        if let Some(v) = &self.frames {
            write_str_lp(&mut out, &enc_frames_diff(v));
        }
        if let Some(v) = &self.metadata {
            write_str_lp(&mut out, &enc_metadata_diff(v));
        }
        Ok(out)
    }
    async fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const DIFF_BINARY_FORMAT: u8 = 1;
        if bytes.len() < 2 {
            return Err(protocol::ProtocolError::Malformed { what: "diff header", offset: 0, detail: "truncated (need format+presence)".to_string() });
        }
        if bytes[0] != DIFF_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "diff format", offset: 0, detail: format!("unsupported diff format {}", bytes[0]) });
        }
        let presence = bytes[1];
        let mut reader = store::ByteReader::new(&bytes[2..]);
        let width = if presence & 0b0000_0001 != 0 {
            let text = read_str_lp(&mut reader).await.map_err(|e| protocol::ProtocolError::Malformed { what: "diff width blob", offset: 2, detail: e })?;
            Some(parse_u32(&text).await.map_err(|e| protocol::ProtocolError::Malformed { what: "diff width text", offset: 2, detail: e })?)
        } else {
            None
        };
        let height = if presence & 0b0000_0010 != 0 {
            let text = read_str_lp(&mut reader).await.map_err(|e| protocol::ProtocolError::Malformed { what: "diff height blob", offset: 2, detail: e })?;
            Some(parse_u32(&text).await.map_err(|e| protocol::ProtocolError::Malformed { what: "diff height text", offset: 2, detail: e })?)
        } else {
            None
        };
        let colorspace = if presence & 0b0000_0100 != 0 {
            let text = read_str_lp(&mut reader).await.map_err(|e| protocol::ProtocolError::Malformed { what: "diff colorspace blob", offset: 2, detail: e })?;
            Some(dec_colorspace(&text).await.map_err(|e| protocol::ProtocolError::Malformed { what: "diff colorspace text", offset: 2, detail: e })?)
        } else {
            None
        };
        let bit_depth = if presence & 0b0000_1000 != 0 {
            let text = read_str_lp(&mut reader).await.map_err(|e| protocol::ProtocolError::Malformed { what: "diff bit_depth blob", offset: 2, detail: e })?;
            Some(parse_u8(&text).await.map_err(|e| protocol::ProtocolError::Malformed { what: "diff bit_depth text", offset: 2, detail: e })?)
        } else {
            None
        };
        let icc = if presence & 0b0001_0000 != 0 {
            let text = read_str_lp(&mut reader).await.map_err(|e| protocol::ProtocolError::Malformed { what: "diff icc blob", offset: 2, detail: e })?;
            Some(decode_option(&text, hex_decode).await.map_err(|e| protocol::ProtocolError::Malformed { what: "diff icc text", offset: 2, detail: e })?)
        } else {
            None
        };
        let frames = if presence & 0b0010_0000 != 0 {
            let text = read_str_lp(&mut reader).await.map_err(|e| protocol::ProtocolError::Malformed { what: "diff frames blob", offset: 2, detail: e })?;
            Some(dec_frames_diff(&text).await.map_err(|e| protocol::ProtocolError::Malformed { what: "diff frames text", offset: 2, detail: e })?)
        } else {
            None
        };
        let metadata = if presence & 0b0100_0000 != 0 {
            let text = read_str_lp(&mut reader).await.map_err(|e| protocol::ProtocolError::Malformed { what: "diff metadata blob", offset: 2, detail: e })?;
            Some(dec_metadata_diff(&text).await.map_err(|e| protocol::ProtocolError::Malformed { what: "diff metadata text", offset: 2, detail: e })?)
        } else {
            None
        };
        Ok(SemioImageDiff { width, height, colorspace, bit_depth, icc, frames, metadata })
    }
}
//#endregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec

//#region 🔖️Demo
/// 🌱 Representative `SemioImageDiff` cases (empty/no-op, a full field sweep both directions incl.
/// the `icc` tri-state and both collection triples, a bare frame/metadata insert) — single source
/// of truth for `diff_grammar_conformance_law`/`protocol_walk_law` in `🎹️composer/🦀️component.rs`.
#[cfg(test)]
pub(crate) async fn demo_diff_cases() -> Vec<SemioImageDiff> {
    async fn frame(seed: u8, len: usize) -> SemioImageFrame {
        SemioImageFrame { delay_ms: 100, rgba8: vec![seed; len] }
    }
    let a = SemioImageSnapshot {
        width: 10,
        height: 8,
        colorspace: SemioColorspace::Rgb,
        bit_depth: 8,
        frames: vec![frame(1, 4), frame(2, 4)],
        icc: Some(vec![1, 2, 3]),
        metadata: vec![SemioImageMetadataEntry { key: "keep".into(), value: "old".into() }],
        ..SemioImageSnapshot::default()
    };
    let b = SemioImageSnapshot {
        width: 20,
        height: 16,
        colorspace: SemioColorspace::GrayscaleAlpha,
        bit_depth: 16,
        frames: vec![
            {
                let mut f = frame(1, 4);
                f.delay_ms = 500;
                f
            },
            frame(6, 9),
        ],
        icc: None,
        metadata: vec![SemioImageMetadataEntry { key: "keep".into(), value: "new".into() }, SemioImageMetadataEntry { key: "fresh".into(), value: "hi".into() }],
        ..SemioImageSnapshot::default()
    };
    vec![SemioImageDiff::default(), <SemioImageDiff as DiffAlgebra<SemioImageSnapshot>>::between(&a, &b), <SemioImageDiff as DiffAlgebra<SemioImageSnapshot>>::between(&b, &a)]
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA;

    async fn frame(seed: u8, len: usize) -> SemioImageFrame {
        SemioImageFrame { delay_ms: 100, rgba8: vec![seed; len] }
    }

    /// 🧪️ Canonical absorb case 1: `InsertFrame(2,f)` then `RemoveFrame(0)` → `{removed:[0],
    /// added:[(1,f)]}`.
    #[semio_framework_async_macros::async_test]
    async fn absorb_insert_then_remove_before_shifts_index() {
        let f = frame(9, 4);
        let d1 = SemioImageFramesDiff { added: vec![IndexAdded { index: 2, item: f.clone() }], ..Default::default() };
        let d2 = SemioImageFramesDiff { removed: vec![0], ..Default::default() };
        let absorbed = frames_absorb(d1, d2);
        assert_eq!(absorbed.removed, vec![0]);
        assert_eq!(absorbed.added, vec![IndexAdded { index: 1, item: f }]);
        assert!(absorbed.modified.is_empty());
    }

    /// 🧪️ Canonical absorb case 2: `InsertFrame(2,f)` then `InsertFrame(2,g)` → both survive.
    #[semio_framework_async_macros::async_test]
    async fn absorb_insert_insert_same_index_both_survive() {
        let f = frame(1, 4);
        let g = frame(2, 4);
        let d1 = SemioImageFramesDiff { added: vec![IndexAdded { index: 2, item: f.clone() }], ..Default::default() };
        let d2 = SemioImageFramesDiff { added: vec![IndexAdded { index: 2, item: g.clone() }], ..Default::default() };
        let absorbed = frames_absorb(d1, d2);
        assert_eq!(absorbed.added, vec![IndexAdded { index: 2, item: g }, IndexAdded { index: 3, item: f }]);
    }

    /// 🧪️ Canonical absorb case 3: `InsertFrame(1,f)` then `SetFrameDelay(1,42)` patches INTO the
    /// added payload.
    #[semio_framework_async_macros::async_test]
    async fn absorb_insert_then_set_field_patches_into_added() {
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
    #[semio_framework_async_macros::async_test]
    async fn absorb_modify_then_remove_drops_modify() {
        let base = SemioImageSnapshot { frames: vec![frame(1, 4), frame(2, 4)], ..SemioImageSnapshot::default() };
        let mid = {
            let mut s = base.clone();
            s.frames[1].delay_ms = 50;
            s
        };
        let after = {
            let mut s = mid.clone();
            s.frames.remove(1);
            s
        };
        let d1 = frames_between(&base.frames, &mid.frames).expect("modify diff");
        let d2 = frames_between(&mid.frames, &after.frames).expect("remove diff");
        let absorbed = frames_absorb(d1, d2);
        assert!(absorbed.modified.is_empty());
        assert_eq!(absorbed.removed, vec![1]);
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_law_holds_over_curated_ops() {
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
        assert_eq!(d1.apply(&base).expect("apply must succeed for a well-formed fixture"), after);
    }

    #[semio_framework_async_macros::async_test]
    async fn between_roundtrip_law() {
        let a = SemioImageSnapshot { width: 4, height: 4, frames: vec![frame(1, 16)], ..SemioImageSnapshot::default() };
        let b = SemioImageSnapshot { width: 4, height: 4, frames: vec![frame(1, 16), frame(2, 4)], colorspace: SemioColorspace::Grayscale, ..SemioImageSnapshot::default() };
        let ab = <SemioImageDiff as DiffAlgebra<SemioImageSnapshot>>::between(&a, &b);
        assert_eq!(ab.apply(&a).expect("apply must succeed for a well-formed fixture"), b);
        let ba = <SemioImageDiff as DiffAlgebra<SemioImageSnapshot>>::between(&b, &a);
        assert_eq!(ba.apply(&b).expect("apply must succeed for a well-formed fixture"), a);
        assert!(<SemioImageDiff as DiffAlgebra<SemioImageSnapshot>>::between(&a, &a).is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn inverse_law() {
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
        let mutated = d.apply(&base).expect("apply must succeed for a well-formed fixture");
        let inv = d.inverse(&base);
        assert_eq!(inv.apply(&mutated).expect("apply must succeed for a well-formed fixture"), base);
    }

    /// 🧪️ field_sweep — THE acceptance criterion: `sweep_a`/`sweep_b` differ in every mutable
    /// field, including the `icc` tri-state exercising BOTH `Some(Some(_))` and `Some(None)`, and
    /// asymmetric collection lengths (a single same-direction `between()` shows removed XOR
    /// added, never both — split across both directions).
    #[semio_framework_async_macros::async_test]
    async fn field_sweep() {
        let sweep_a = SemioImageSnapshot {
            schema: STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA.into(),
            width: 10,
            height: 8,
            colorspace: SemioColorspace::Rgb,
            bit_depth: 8,
            frames: vec![frame(1, 4), frame(2, 4)],
            icc: Some(vec![1, 2, 3]),
            metadata: vec![SemioImageMetadataEntry { key: "keep".into(), value: "old".into() }, SemioImageMetadataEntry { key: "gone".into(), value: "bye".into() }],
        };
        let sweep_b = SemioImageSnapshot {
            schema: STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA.into(),
            width: 20,
            height: 16,
            colorspace: SemioColorspace::GrayscaleAlpha,
            bit_depth: 16,
            frames: vec![
                {
                    let mut f = frame(1, 4);
                    f.delay_ms = 500;
                    f
                },
                frame(6, 9),
                frame(7, 9),
            ],
            icc: None,
            metadata: vec![SemioImageMetadataEntry { key: "keep".into(), value: "new".into() }, SemioImageMetadataEntry { key: "fresh".into(), value: "hi".into() }],
        };

        let ab = <SemioImageDiff as DiffAlgebra<SemioImageSnapshot>>::between(&sweep_a, &sweep_b);
        assert_eq!(ab.apply(&sweep_a).expect("apply must succeed for a well-formed fixture"), sweep_b);
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
        assert_eq!(ba.apply(&sweep_b).expect("apply must succeed for a well-formed fixture"), sweep_a);
        assert_eq!(ba.icc, Some(Some(vec![1, 2, 3])), "icc None->Some must be tri-state Some(Some(_))");
        let frames_ba = ba.frames.as_ref().expect("frames must differ");
        assert!(!frames_ba.removed.is_empty(), "reverse direction must exercise a removed frame (a is shorter)");

        assert!(<SemioImageDiff as DiffAlgebra<SemioImageSnapshot>>::between(&sweep_a, &sweep_a).is_empty());
    }

    /// 🧪️ `DiffCodec` round-trip laws for the hand-rolled `SemioImageDiff` text/binary grammar —
    /// scalars, the `icc` tri-state, and both collection triples simultaneously via a real
    /// `between()` result.
    #[semio_framework_async_macros::async_test]
    async fn diff_codec_text_binary_roundtrip_law() {
        // 🌱 Reuses `demo_diff_cases()` (single source of truth, also feeds
        // `diff_grammar_conformance_law`/`protocol_walk_law` in `🎹️composer/🦀️component.rs`)
        // rather than an independent copy of the same base/other fixture pair.
        for d in demo_diff_cases() {
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
