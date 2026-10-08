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

use crate::standards::v1::subsets::base::schema::triples::{IndexAdded, IndexModified, IndexedTripleDiff, NamedModified, NamedTripleDiff, NamedAdded};



use crate::standards::v1::subsets::image::schema::snapshot::{SemioColorspace, SemioImageFrame, SemioImageMetadataEntry, SemioImageSnapshot};
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
/// 🔧️ Unconditional — the `#[cfg(test)] mod tests` block below calls `print_diff`/`parse_diff`/
/// `encode_diff`/`decode_diff` via method syntax on `SemioImageDiff`, which needs `DiffCodec` in
/// scope (the `impl protocol::DiffCodec for SemioImageDiff` block itself compiles fine unqualified,
/// but callers using method syntax do not get the trait for free) (W2b closer fix).
use protocol::{DiffCodec};
use protocol::MutationDiff;

//#region 🔖️FrameDiff
/// 🔺️ Sparse per-field diff for one [`SemioImageFrame`] — a strong entity, per the recipe.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SemioImageFrameDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub delay_ms: Option<u32>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub rgba8: Option<Vec<u8>>,
}

impl SemioImageFrameDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.delay_ms.is_none() && self.rgba8.is_none()
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn apply_row(&self, base: &SemioImageFrame) -> SemioImageFrame {
        let mut next = base.clone();
        if let Some(v) = self.delay_ms {
            next.delay_ms = v;
        }
        if let Some(v) = &self.rgba8 {
            next.rgba8 = v.clone();
        }
        next
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn inverse(&self, base: &SemioImageFrame) -> Self {
        Self { delay_ms: self.delay_ms.map(|_| base.delay_ms), rgba8: self.rgba8.as_ref().map(|_| base.rgba8.clone()) }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn absorb(&mut self, other: Self) {
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
pub type SemioImageMetadataDiff = NamedTripleDiff<String, String, NamedAdded<SemioImageMetadataEntry>>;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

/// 🧮️ Sequential-coalesce absorb per the recipe's normative algorithm (gif/docx precedent).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_indexed<T: Clone, D: Clone>(d1: IndexedTripleDiff<D, T>, d2: IndexedTripleDiff<D, T>, absorb_item: impl Fn(D, D) -> D, apply_item: impl Fn(&T, &D) -> T) -> IndexedTripleDiff<D, T> {
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
        added.push(IndexAdded { index: final_index, item: add.item });
    }
    added.extend(d2.added);
    added.sort_by_key(|a| a.index);

    IndexedTripleDiff { removed, modified, added }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn reproduces_order<K, T>(base: &[T], other: &[T], removed: &[K], added: &[NamedAdded<T>], key_of: &impl Fn(&T) -> K) -> bool
where
    K: PartialEq,
{
    let mut keys: Vec<K> = base.iter().map(key_of).filter(|k| !removed.contains(k)).collect();
    let mut ascending: Vec<&NamedAdded<T>> = added.iter().collect();
    ascending.sort_by_key(|a| a.index);
    for a in ascending {
        let at = a.index.min(keys.len());
        keys.insert(at, key_of(&a.item));
    }
    keys.len() == other.len() && keys.iter().zip(other.iter().map(key_of)).all(|(produced, expected)| *produced == expected)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_named<K, T, D>(items: &mut Vec<T>, diff: &NamedTripleDiff<K, D, NamedAdded<T>>, key_of: impl Fn(&T) -> K, apply_item: impl Fn(&mut T, &D))
where
    K: PartialEq + Clone,
    T: Clone,
{
    items.retain(|i| !diff.removed.contains(&key_of(i)));
    for m in &diff.modified {
        if let Some(item) = items.iter_mut().find(|i| key_of(i) == m.key) {
            apply_item(item, &m.diff);
        }
    }
    let mut ascending: Vec<&NamedAdded<T>> = diff.added.iter().collect();
    ascending.sort_by_key(|a| a.index);
    for a in ascending {
        let at = a.index.min(items.len());
        items.insert(at, a.item.clone());
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_named<K, T, D>(base_items: &[T], diff: &NamedTripleDiff<K, D, NamedAdded<T>>, key_of: impl Fn(&T) -> K, inverse_item: impl Fn(&T, &D) -> D) -> NamedTripleDiff<K, D, NamedAdded<T>>
where
    K: PartialEq + Clone,
    T: Clone,
{
    let removed: Vec<K> = diff.added.iter().map(|a| key_of(&a.item)).collect();
    let mut modified = Vec::new();
    for m in &diff.modified {
        if let Some(original) = base_items.iter().find(|i| key_of(i) == m.key) {
            modified.push(NamedModified { key: m.key.clone(), diff: inverse_item(original, &m.diff) });
        }
    }
    let mut added: Vec<NamedAdded<T>> = diff.removed.iter().filter_map(|k| base_items.iter().position(|i| &key_of(i) == k).map(|index| NamedAdded { index, item: base_items[index].clone() })).collect();
    added.sort_by_key(|a| a.index);
    NamedTripleDiff { removed, modified, added }
}

/// 🧮️ Name-keyed absorb — identity is the KEY, so no index transport is needed.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_named<K, T, D>(d1: NamedTripleDiff<K, D, NamedAdded<T>>, d2: &NamedTripleDiff<K, D, NamedAdded<T>>, key_of: impl Fn(&T) -> K, absorb_item: impl Fn(D, D) -> D, apply_item: impl Fn(&mut T, &D)) -> NamedTripleDiff<K, D, NamedAdded<T>>
where
    K: PartialEq + Clone,
    T: Clone,
    D: Clone,
{
    let d1_added_keys: Vec<K> = d1.added.iter().map(|a| key_of(&a.item)).collect();
    let mut removed = d1.removed.clone();
    let mut annihilated: Vec<K> = Vec::new();
    for k in &d2.removed {
        if d1_added_keys.contains(k) {
            annihilated.push(k.clone());
        } else if !removed.contains(k) {
            removed.push(k.clone());
        }
    }
    let mut working_added: Vec<NamedAdded<T>> = d1.added.into_iter().filter(|a| !annihilated.contains(&key_of(&a.item))).collect();
    let mut modified: Vec<NamedModified<K, D>> = d1.modified.into_iter().filter(|m| !removed.contains(&m.key)).collect();
    for m2 in &d2.modified {
        if let Some(added) = working_added.iter_mut().find(|a| key_of(&a.item) == m2.key) {
            apply_item(&mut added.item, &m2.diff);
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
        let k2 = key_of(&a2.item);
        match working_added.iter_mut().find(|a| key_of(&a.item) == k2) {
            Some(existing) => *existing = a2.clone(),
            None => working_added.push(a2.clone()),
        }
    }
    NamedTripleDiff { removed, modified, added: working_added }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn frames_apply(items: &mut Vec<SemioImageFrame>, diff: &SemioImageFramesDiff) {
    apply_indexed(items, diff, |item, d| d.apply_row(item));
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn frames_inverse(base: &[SemioImageFrame], diff: &SemioImageFramesDiff) -> SemioImageFramesDiff {
    inverse_indexed(base, diff, |item, d| d.inverse(item))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn frames_absorb(d1: SemioImageFramesDiff, d2: SemioImageFramesDiff) -> SemioImageFramesDiff {
    absorb_indexed(
        d1,
        d2,
        |mut a, b| {
            a.absorb(b);
            a
        },
        |item, d| d.apply_row(item),
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn metadata_key(e: &SemioImageMetadataEntry) -> String {
    e.key.clone()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn metadata_apply(items: &mut Vec<SemioImageMetadataEntry>, diff: &SemioImageMetadataDiff) {
    apply_named(items, diff, metadata_key, |item, d| item.value = d.clone());
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn metadata_inverse(base: &[SemioImageMetadataEntry], diff: &SemioImageMetadataDiff) -> SemioImageMetadataDiff {
    inverse_named(base, diff, metadata_key, |item, _d| item.value.clone())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn metadata_absorb(d1: SemioImageMetadataDiff, d2: SemioImageMetadataDiff) -> SemioImageMetadataDiff {
    absorb_named(d1, &d2, metadata_key, |_old, new| new, |item, d| item.value = d.clone())
}
//#endregion 🔖️CollectionWrappers

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.image.diff")]
pub struct SemioImageDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub colorspace: Option<SemioColorspace>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub bit_depth: Option<u8>,
    /// 🎨️ Tri-state: `None` = unchanged, `Some(None)` = ICC removed, `Some(Some(bytes))` = set.
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub icc: Option<Option<Vec<u8>>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub frames: Option<SemioImageFramesDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<SemioImageMetadataDiff>,
}

impl SemioImageDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty_diff(&self) -> bool {
        self.width.is_none() && self.height.is_none() && self.colorspace.is_none() && self.bit_depth.is_none() && self.icc.is_none() && self.frames.is_none() && self.metadata.is_none()
    }
}

impl MutationDiff<SemioImageSnapshot> for SemioImageDiff {
    fn apply(&self, base: &SemioImageSnapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<SemioImageSnapshot> {
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
            crate::standards::v1::subsets::base::schema::triples::validate_indexed_triple(d, next.frames.len(), ["frames"])?;
            frames_apply(&mut next.frames, d);
        }
        if let Some(d) = &self.metadata {
            crate::standards::v1::subsets::base::schema::triples::validate_named_triple(&next.metadata, d, |item| item.key.clone(), |added| added.item.key.clone(), ["metadata"])?;
            metadata_apply(&mut next.metadata, d);
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

    fn is_empty(&self) -> bool {
        self.is_empty_diff()
    }
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











//#endregion 🔖️Primitives

//#region 🔖️ValueCodecs








//#endregion 🔖️ValueCodecs

//#region 🔖️CollectionCodecs




//#endregion 🔖️CollectionCodecs

//#region 🔖️TopLevel




//#endregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec

//#region 🔖️Demo
/// 🌱 Representative `SemioImageDiff` cases built declaratively (empty/no-op, every scalar with the `icc` tri-state and a frame row triple, a
/// bare icc set) — single source of truth for `diff_grammar_conformance_law`/`protocol_walk_law` in `🎹️composer/🦀️.rs`.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<SemioImageDiff> {
    let frames = SemioImageFramesDiff {
        removed: vec![1],
        modified: vec![IndexModified { index: 0, diff: SemioImageFrameDiff { delay_ms: Some(500), ..Default::default() } }],
        added: vec![IndexAdded { index: 1, item: SemioImageFrame { delay_ms: 100, rgba8: vec![6; 9] } }],
    };
    vec![
        SemioImageDiff::default(),
        SemioImageDiff { width: Some(20), height: Some(16), colorspace: Some(SemioColorspace::GrayscaleAlpha), bit_depth: Some(16), icc: Some(None), frames: Some(frames), metadata: None },
        SemioImageDiff { icc: Some(Some(vec![1, 2, 3])), ..Default::default() },
    ]
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests

#[cfg(test)]
use protocol::{DiffBinary,DiffText};
