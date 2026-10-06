//! 🔺️ SemioPresentationDiff — handcrafted sparse diff over `SemioPresentationSnapshot`
//! (`masters`/`layouts`/`slides`). No `snapshot: Option<SemioPresentationSnapshot>` full-replace
//! slot — even `SetSnapshot`'s diff is the sparse field-by-field `SemioPresentationDiff::between`.
//!
//! Collection key kinds (per the recipe's "Key kinds per collection" rule): `masters`/`layouts`
//! are id-keyed (`NamedTripleDiff`, referenced BY id from `layouts.master_id`/`slides.layout_id`,
//! like docx's name-keyed `styles`); `slides` is INDEX-keyed (`IndexedTripleDiff`, presentation
//! order is significant, like pdf page order), and a slot whose slide changes identity carries the
//! new `id` in `SlideDiff::id` — so a reorder or a whole-deck replacement lands every slide's own
//! identity at its new index instead of leaving the old identifiers behind the new content. `shapes` (owned by masters/layouts/slides alike), `notes`, and `TextBox`/table
//! `blocks` are all index-keyed too.
//!
//! `document::DocBlock` is reused verbatim for text content (`TextBox.blocks`, table cell
//! `blocks`, `Slide.notes`) but is OWNED by the `document` subset, out of this file's write scope
//! — it has no field-level diff type of its own exposed yet, so this file diffs `Vec<DocBlock>`
//! items as WHOLE VALUES (`D = T = DocBlock`; "modified" carries the complete replacement block).
//! This is honest per the recipe's weak/strong-entity split: from this subset's point of view
//! `DocBlock` is a value struct it does not own the internals of, so it is never sub-diffed here.

use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use crate::standards::v1::subsets::base::schema::triples::{IndexAdded, IndexModified, IndexedTripleDiff, NamedModified, NamedTripleDiff};


/// 🧱️ REUSE, don't reinvent — `document::DocBlock`'s own real, already-tested text codec
/// (`ws-codec-document-report.md`), re-exported here so both this file's own leaf encoders AND
/// the sibling `🧬️mutations`/`📸️snapshot` facets can import `{enc_block, dec_block}` from THIS
/// module (matching the pre-existing convention where this file is the one place that owns every
/// value codec presentation's other facets import from).
pub(crate) use crate::document::io::text::diff::{dec_block};
pub(crate) use crate::document::io::text::diff::{enc_block};
use crate::standards::v1::subsets::document::schema::snapshot::DocBlock;
use crate::standards::v1::subsets::presentation::schema::snapshot::SemioPresentationSnapshot;
use crate::standards::v1::subsets::presentation::schema::snapshot::{PlaceholderKind, Slide, SlideFrame, SlideLayout, SlideMaster, SlidePictureImage, SlideShape, SlideTableCell, SlideTableRow};
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;

//#region 🔖️CollectionDiffAliases
pub type SlideShapesDiff = IndexedTripleDiff<SlideShapeDiff, SlideShape>;
/// 🧱️ `document::DocBlock` treated as its own diff (`D = T`) — see module doc comment.
pub type DocBlocksDiff = IndexedTripleDiff<DocBlock, DocBlock>;
pub type SlideTableRowsDiff = IndexedTripleDiff<SlideTableRowDiff, SlideTableRow>;
pub type SlideTableCellsDiff = IndexedTripleDiff<SlideTableCellDiff, SlideTableCell>;
pub type SlideMastersDiff = NamedTripleDiff<String, SlideMasterDiff, SlideMaster>;
pub type SlideLayoutsDiff = NamedTripleDiff<String, SlideLayoutDiff, SlideLayout>;
pub type SlidesDiff = IndexedTripleDiff<SlideDiff, Slide>;
//#endregion 🔖️CollectionDiffAliases

//#region 🔖️DiffTypes
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SlideFrameDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<SemioPoint2>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SlidePictureImageDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub mime: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub bytes: Option<Vec<u8>>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SlideTableCellDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub blocks: Option<DocBlocksDiff>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SlideTableRowDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub cells: Option<SlideTableCellsDiff>,
}

/// 🌳️ Per-shape diff, shaped like `SlideShape` (`Replace` covers a shape-KIND change, e.g.
/// `TextBox` -> `Picture`, same convention as docx's `DocxBlockDiff::Replace`). Tag is
/// `shapeKind` (not `kind`) for the same field/tag-collision reason as `SlideShape` itself (see
/// that type's doc comment) — `Placeholder`'s own `kind` field would otherwise collide.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "shapeKind", rename_all = "camelCase")]
pub enum SlideShapeDiff {
    TextBox {
        #[value(default, skip_serializing_if = "Option::is_none")]
        frame: Option<SlideFrameDiff>,
        #[value(default, skip_serializing_if = "Option::is_none")]
        blocks: Option<DocBlocksDiff>,
    },
    Picture {
        #[value(default, skip_serializing_if = "Option::is_none")]
        frame: Option<SlideFrameDiff>,
        #[value(default, skip_serializing_if = "Option::is_none")]
        image: Option<SlidePictureImageDiff>,
    },
    Table {
        #[value(default, skip_serializing_if = "Option::is_none")]
        frame: Option<SlideFrameDiff>,
        #[value(default, skip_serializing_if = "Option::is_none")]
        rows: Option<SlideTableRowsDiff>,
    },
    Placeholder {
        #[value(default, skip_serializing_if = "Option::is_none")]
        frame: Option<SlideFrameDiff>,
        #[value(default, skip_serializing_if = "Option::is_none")]
        kind: Option<PlaceholderKind>,
    },
    Replace {
        shape: SlideShape,
    },
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SlideMasterDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub shapes: Option<SlideShapesDiff>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SlideLayoutDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub master_id: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub shapes: Option<SlideShapesDiff>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SlideDiff {
    /// 🪪️ The slide's own identity at this index, when it changes: an index-keyed slot whose slide is
    /// replaced by a different one (a reorder, a whole-deck `set-snapshot`) takes the new slide's `id`
    /// with its content, so applying `between(base, next)` to `base` yields exactly `next`.
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 🏳️ Tri-state: `None` = unchanged, `Some(None)` = layout cleared, `Some(Some(id))` = set.
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub layout_id: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub shapes: Option<SlideShapesDiff>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<DocBlocksDiff>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.presentation.diff")]
pub struct SemioPresentationDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub masters: Option<SlideMastersDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub layouts: Option<SlideLayoutsDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub slides: Option<SlidesDiff>,
}
//#endregion 🔖️DiffTypes

//#region 🔖️GenericIndexedEngine
/// 🧮️ Own copy of the generic index-keyed between/apply/inverse/absorb algorithm (docx precedent
/// — every hand-rolled artifact re-derives this small engine against the SHARED `IndexedTripleDiff`
/// type rather than importing a shared algorithm module).
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

#[allow(clippy::too_many_arguments)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_indexed<T, D>(d1: IndexedTripleDiff<D, T>, d2: IndexedTripleDiff<D, T>, absorb_item: impl Fn(D, D) -> D, apply_item: impl Fn(&T, &D) -> T) -> IndexedTripleDiff<D, T>
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
    added.extend(d2.added);
    added.sort_by_key(|a| a.index);

    IndexedTripleDiff { removed, modified, added }
}
//#endregion 🔖️GenericIndexedEngine

//#region 🔖️GenericNamedEngine
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_named<K, T, D>(base: &[T], other: &[T], key_of: impl Fn(&T) -> K, diff_item: impl Fn(&T, &T) -> Option<D>) -> Option<NamedTripleDiff<K, D, T>>
where
    K: PartialEq + Clone,
    T: Clone + PartialEq,
{
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_named<K, T, D>(items: &mut Vec<T>, diff: &NamedTripleDiff<K, D, T>, key_of: impl Fn(&T) -> K, apply_item: impl Fn(&mut T, &D))
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
    for item in &diff.added {
        items.push(item.clone());
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_named<K, T, D>(base_items: &[T], diff: &NamedTripleDiff<K, D, T>, key_of: impl Fn(&T) -> K, inverse_item: impl Fn(&T, &D) -> D) -> NamedTripleDiff<K, D, T>
where
    K: PartialEq + Clone,
    T: Clone,
{
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_named<K, T, D>(d1: NamedTripleDiff<K, D, T>, d2: &NamedTripleDiff<K, D, T>, key_of: impl Fn(&T) -> K, absorb_item: impl Fn(D, D) -> D, apply_item: impl Fn(&mut T, &D)) -> NamedTripleDiff<K, D, T>
where
    K: PartialEq + Clone,
    T: Clone,
    D: Clone,
{
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

/// 🔧 Small `Option<T>` LWW-recursive-absorb helper (`None,x -> x; x,None -> x; Some,Some ->
/// Some(f(a,b))`) — factors out the same three-arm match repeated across every scalar-collection
/// pairing below.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_opt<T>(a: Option<T>, b: Option<T>, f: impl FnOnce(T, T) -> T) -> Option<T> {
    match (a, b) {
        (None, x) => x,
        (x, None) => x,
        (Some(x), Some(y)) => Some(f(x, y)),
    }
}
//#endregion 🔖️GenericNamedEngine

//#region 🔖️ValueDiffLogic
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_frame(old: &SlideFrame, new: &SlideFrame) -> Option<SlideFrameDiff> {
    if old == new {
        return None;
    }
    Some(SlideFrameDiff { origin: (old.origin != new.origin).then_some(new.origin), width: (old.width != new.width).then_some(new.width), height: (old.height != new.height).then_some(new.height) })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_frame(frame: &mut SlideFrame, diff: &SlideFrameDiff) {
    if let Some(v) = diff.origin {
        frame.origin = v;
    }
    if let Some(v) = diff.width {
        frame.width = v;
    }
    if let Some(v) = diff.height {
        frame.height = v;
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_frame(base: &SlideFrame, diff: &SlideFrameDiff) -> SlideFrameDiff {
    SlideFrameDiff { origin: diff.origin.map(|_| base.origin), width: diff.width.map(|_| base.width), height: diff.height.map(|_| base.height) }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_frame(mut a: SlideFrameDiff, b: &SlideFrameDiff) -> SlideFrameDiff {
    if b.origin.is_some() {
        a.origin = b.origin;
    }
    if b.width.is_some() {
        a.width = b.width;
    }
    if b.height.is_some() {
        a.height = b.height;
    }
    a
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_image(old: &SlidePictureImage, new: &SlidePictureImage) -> Option<SlidePictureImageDiff> {
    if old == new {
        return None;
    }
    Some(SlidePictureImageDiff { asset_id: (old.asset_id != new.asset_id).then(|| new.asset_id.clone()), mime: (old.mime != new.mime).then(|| new.mime.clone()), bytes: (old.bytes != new.bytes).then(|| new.bytes.clone()) })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_image(image: &mut SlidePictureImage, diff: &SlidePictureImageDiff) {
    if let Some(v) = &diff.asset_id {
        image.asset_id = v.clone();
    }
    if let Some(v) = &diff.mime {
        image.mime = v.clone();
    }
    if let Some(v) = &diff.bytes {
        image.bytes = v.clone();
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_image(base: &SlidePictureImage, diff: &SlidePictureImageDiff) -> SlidePictureImageDiff {
    SlidePictureImageDiff { asset_id: diff.asset_id.as_ref().map(|_| base.asset_id.clone()), mime: diff.mime.as_ref().map(|_| base.mime.clone()), bytes: diff.bytes.as_ref().map(|_| base.bytes.clone()) }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_image(mut a: SlidePictureImageDiff, b: SlidePictureImageDiff) -> SlidePictureImageDiff {
    if b.asset_id.is_some() {
        a.asset_id = b.asset_id;
    }
    if b.mime.is_some() {
        a.mime = b.mime;
    }
    if b.bytes.is_some() {
        a.bytes = b.bytes;
    }
    a
}

/// 🧱️ Whole-value `DocBlock` "diff" (`D = T`, see module doc comment) — never sub-diffed.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_doc_block(old: &DocBlock, new: &DocBlock) -> Option<DocBlock> {
    (old != new).then(|| new.clone())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_doc_block(block: &mut DocBlock, diff: &DocBlock) {
    *block = diff.clone();
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn doc_block_with_diff_applied(_block: &DocBlock, diff: &DocBlock) -> DocBlock {
    diff.clone()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_doc_block(base: &DocBlock, _diff: &DocBlock) -> DocBlock {
    base.clone()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_doc_block(_a: DocBlock, b: DocBlock) -> DocBlock {
    b
}
//#endregion 🔖️ValueDiffLogic

//#region 🔖️ShapeDiffLogic
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_shape(old: &SlideShape, new: &SlideShape) -> Option<SlideShapeDiff> {
    if old == new {
        return None;
    }
    match (old, new) {
        (SlideShape::TextBox { frame: of, blocks: ob }, SlideShape::TextBox { frame: nf, blocks: nb }) => {
            let frame = diff_frame(of, nf);
            let blocks = between_indexed(ob, nb, diff_doc_block);
            if frame.is_none() && blocks.is_none() {
                None
            } else {
                Some(SlideShapeDiff::TextBox { frame, blocks })
            }
        }
        (SlideShape::Picture { frame: of, image: oi }, SlideShape::Picture { frame: nf, image: ni }) => {
            let frame = diff_frame(of, nf);
            let image = diff_image(oi, ni);
            if frame.is_none() && image.is_none() {
                None
            } else {
                Some(SlideShapeDiff::Picture { frame, image })
            }
        }
        (SlideShape::Table { frame: of, rows: or }, SlideShape::Table { frame: nf, rows: nr }) => {
            let frame = diff_frame(of, nf);
            let rows = between_indexed(or, nr, diff_table_row);
            if frame.is_none() && rows.is_none() {
                None
            } else {
                Some(SlideShapeDiff::Table { frame, rows })
            }
        }
        (SlideShape::Placeholder { frame: of, kind: ok }, SlideShape::Placeholder { frame: nf, kind: nk }) => {
            let frame = diff_frame(of, nf);
            let kind = (ok != nk).then(|| nk.clone());
            if frame.is_none() && kind.is_none() {
                None
            } else {
                Some(SlideShapeDiff::Placeholder { frame, kind })
            }
        }
        _ => Some(SlideShapeDiff::Replace { shape: new.clone() }),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_table_cell(old: &SlideTableCell, new: &SlideTableCell) -> Option<SlideTableCellDiff> {
    let blocks = between_indexed(&old.blocks, &new.blocks, diff_doc_block);
    blocks.map(|blocks| SlideTableCellDiff { blocks: Some(blocks) })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_table_row(old: &SlideTableRow, new: &SlideTableRow) -> Option<SlideTableRowDiff> {
    let cells = between_indexed(&old.cells, &new.cells, diff_table_cell);
    cells.map(|cells| SlideTableRowDiff { cells: Some(cells) })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_shape(shape: &mut SlideShape, diff: &SlideShapeDiff) {
    match diff {
        SlideShapeDiff::Replace { shape: new } => *shape = new.clone(),
        SlideShapeDiff::TextBox { frame, blocks } => {
            if let SlideShape::TextBox { frame: f, blocks: b } = shape {
                if let Some(fd) = frame {
                    apply_frame(f, fd);
                }
                if let Some(bd) = blocks {
                    apply_indexed(b, bd, apply_doc_block);
                }
            }
        }
        SlideShapeDiff::Picture { frame, image } => {
            if let SlideShape::Picture { frame: f, image: i } = shape {
                if let Some(fd) = frame {
                    apply_frame(f, fd);
                }
                if let Some(id) = image {
                    apply_image(i, id);
                }
            }
        }
        SlideShapeDiff::Table { frame, rows } => {
            if let SlideShape::Table { frame: f, rows: r } = shape {
                if let Some(fd) = frame {
                    apply_frame(f, fd);
                }
                if let Some(rd) = rows {
                    apply_indexed(r, rd, apply_table_row);
                }
            }
        }
        SlideShapeDiff::Placeholder { frame, kind } => {
            if let SlideShape::Placeholder { frame: f, kind: k } = shape {
                if let Some(fd) = frame {
                    apply_frame(f, fd);
                }
                if let Some(kd) = kind {
                    *k = kd.clone();
                }
            }
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_table_row(row: &mut SlideTableRow, diff: &SlideTableRowDiff) {
    if let Some(cd) = &diff.cells {
        apply_indexed(&mut row.cells, cd, apply_table_cell);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_table_cell(cell: &mut SlideTableCell, diff: &SlideTableCellDiff) {
    if let Some(bd) = &diff.blocks {
        apply_indexed(&mut cell.blocks, bd, apply_doc_block);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn shape_with_diff_applied(shape: &SlideShape, diff: &SlideShapeDiff) -> SlideShape {
    let mut out = shape.clone();
    apply_shape(&mut out, diff);
    out
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn table_row_with_diff_applied(row: &SlideTableRow, diff: &SlideTableRowDiff) -> SlideTableRow {
    let mut out = row.clone();
    apply_table_row(&mut out, diff);
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_shape(base: &SlideShape, diff: &SlideShapeDiff) -> SlideShapeDiff {
    match diff {
        SlideShapeDiff::Replace { .. } => SlideShapeDiff::Replace { shape: base.clone() },
        SlideShapeDiff::TextBox { frame, blocks } => {
            let SlideShape::TextBox { frame: bf, blocks: bb } = base else { return SlideShapeDiff::Replace { shape: base.clone() } };
            SlideShapeDiff::TextBox { frame: frame.as_ref().map(|fd| inverse_frame(bf, fd)), blocks: blocks.as_ref().map(|bd| inverse_indexed(bb, bd, inverse_doc_block)) }
        }
        SlideShapeDiff::Picture { frame, image } => {
            let SlideShape::Picture { frame: bf, image: bi } = base else { return SlideShapeDiff::Replace { shape: base.clone() } };
            SlideShapeDiff::Picture { frame: frame.as_ref().map(|fd| inverse_frame(bf, fd)), image: image.as_ref().map(|id| inverse_image(bi, id)) }
        }
        SlideShapeDiff::Table { frame, rows } => {
            let SlideShape::Table { frame: bf, rows: br } = base else { return SlideShapeDiff::Replace { shape: base.clone() } };
            SlideShapeDiff::Table { frame: frame.as_ref().map(|fd| inverse_frame(bf, fd)), rows: rows.as_ref().map(|rd| inverse_indexed(br, rd, inverse_table_row)) }
        }
        SlideShapeDiff::Placeholder { frame, kind } => {
            let SlideShape::Placeholder { frame: bf, kind: bk } = base else { return SlideShapeDiff::Replace { shape: base.clone() } };
            SlideShapeDiff::Placeholder { frame: frame.as_ref().map(|fd| inverse_frame(bf, fd)), kind: kind.as_ref().map(|_| bk.clone()) }
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_table_row(base: &SlideTableRow, diff: &SlideTableRowDiff) -> SlideTableRowDiff {
    SlideTableRowDiff { cells: diff.cells.as_ref().map(|cd| inverse_indexed(&base.cells, cd, inverse_table_cell)) }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_table_cell(base: &SlideTableCell, diff: &SlideTableCellDiff) -> SlideTableCellDiff {
    SlideTableCellDiff { blocks: diff.blocks.as_ref().map(|bd| inverse_indexed(&base.blocks, bd, inverse_doc_block)) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_shape(a: SlideShapeDiff, b: SlideShapeDiff) -> SlideShapeDiff {
    match (a, b) {
        (_, SlideShapeDiff::Replace { shape }) => SlideShapeDiff::Replace { shape },
        (SlideShapeDiff::Replace { shape }, b) => SlideShapeDiff::Replace { shape: shape_with_diff_applied(&shape, &b) },
        (SlideShapeDiff::TextBox { frame: fa, blocks: ba }, SlideShapeDiff::TextBox { frame: fb, blocks: bb }) => {
            SlideShapeDiff::TextBox { frame: absorb_opt(fa, fb, |a, b| absorb_frame(a, &b)), blocks: absorb_opt(ba, bb, |x, y| absorb_indexed(x, y, absorb_doc_block, doc_block_with_diff_applied)) }
        }
        (SlideShapeDiff::Picture { frame: fa, image: ia }, SlideShapeDiff::Picture { frame: fb, image: ib }) => SlideShapeDiff::Picture { frame: absorb_opt(fa, fb, |a, b| absorb_frame(a, &b)), image: absorb_opt(ia, ib, absorb_image) },
        (SlideShapeDiff::Table { frame: fa, rows: ra }, SlideShapeDiff::Table { frame: fb, rows: rb }) => {
            SlideShapeDiff::Table { frame: absorb_opt(fa, fb, |a, b| absorb_frame(a, &b)), rows: absorb_opt(ra, rb, |x, y| absorb_indexed(x, y, absorb_table_row_diff, table_row_with_diff_applied)) }
        }
        (SlideShapeDiff::Placeholder { frame: fa, kind: ka }, SlideShapeDiff::Placeholder { frame: fb, kind: kb }) => SlideShapeDiff::Placeholder { frame: absorb_opt(fa, fb, |a, b| absorb_frame(a, &b)), kind: kb.or(ka) },
        (_, b) => b,
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_table_cell_diff(mut a: SlideTableCellDiff, b: SlideTableCellDiff) -> SlideTableCellDiff {
    a.blocks = absorb_opt(a.blocks.take(), b.blocks, |x, y| absorb_indexed(x, y, absorb_doc_block, doc_block_with_diff_applied));
    a
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_table_row_diff(mut a: SlideTableRowDiff, b: SlideTableRowDiff) -> SlideTableRowDiff {
    a.cells = absorb_opt(a.cells.take(), b.cells, |x, y| {
        absorb_indexed(x, y, absorb_table_cell_diff, |c, d| {
            let mut out = c.clone();
            apply_table_cell(&mut out, d);
            out
        })
    });
    a
}
//#endregion 🔖️ShapeDiffLogic

//#region 🔖️StructureDiffLogic
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_master(old: &SlideMaster, new: &SlideMaster) -> Option<SlideMasterDiff> {
    let shapes = between_indexed(&old.shapes, &new.shapes, diff_shape);
    shapes.map(|shapes| SlideMasterDiff { shapes: Some(shapes) })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_layout(old: &SlideLayout, new: &SlideLayout) -> Option<SlideLayoutDiff> {
    let master_id = (old.master_id != new.master_id).then(|| new.master_id.clone());
    let shapes = between_indexed(&old.shapes, &new.shapes, diff_shape);
    if master_id.is_none() && shapes.is_none() {
        None
    } else {
        Some(SlideLayoutDiff { master_id, shapes })
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_slide(old: &Slide, new: &Slide) -> Option<SlideDiff> {
    let id = (old.id != new.id).then(|| new.id.clone());
    let layout_id = (old.layout_id != new.layout_id).then(|| new.layout_id.clone());
    let shapes = between_indexed(&old.shapes, &new.shapes, diff_shape);
    let notes = between_indexed(&old.notes, &new.notes, diff_doc_block);
    if id.is_none() && layout_id.is_none() && shapes.is_none() && notes.is_none() {
        None
    } else {
        Some(SlideDiff { id, layout_id, shapes, notes })
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_master(master: &mut SlideMaster, diff: &SlideMasterDiff) {
    if let Some(sd) = &diff.shapes {
        apply_indexed(&mut master.shapes, sd, apply_shape);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_layout(layout: &mut SlideLayout, diff: &SlideLayoutDiff) {
    if let Some(v) = &diff.master_id {
        layout.master_id = v.clone();
    }
    if let Some(sd) = &diff.shapes {
        apply_indexed(&mut layout.shapes, sd, apply_shape);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_slide(slide: &mut Slide, diff: &SlideDiff) {
    if let Some(id) = &diff.id {
        slide.id = id.clone();
    }
    if let Some(v) = &diff.layout_id {
        slide.layout_id = v.clone();
    }
    if let Some(sd) = &diff.shapes {
        apply_indexed(&mut slide.shapes, sd, apply_shape);
    }
    if let Some(nd) = &diff.notes {
        apply_indexed(&mut slide.notes, nd, apply_doc_block);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn slide_with_diff_applied(slide: &Slide, diff: &SlideDiff) -> Slide {
    let mut out = slide.clone();
    apply_slide(&mut out, diff);
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_master(base: &SlideMaster, diff: &SlideMasterDiff) -> SlideMasterDiff {
    SlideMasterDiff { shapes: diff.shapes.as_ref().map(|sd| inverse_indexed(&base.shapes, sd, inverse_shape)) }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_layout(base: &SlideLayout, diff: &SlideLayoutDiff) -> SlideLayoutDiff {
    SlideLayoutDiff { master_id: diff.master_id.as_ref().map(|_| base.master_id.clone()), shapes: diff.shapes.as_ref().map(|sd| inverse_indexed(&base.shapes, sd, inverse_shape)) }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_slide(base: &Slide, diff: &SlideDiff) -> SlideDiff {
    SlideDiff {
        id: diff.id.as_ref().map(|_| base.id.clone()),
        layout_id: diff.layout_id.as_ref().map(|_| base.layout_id.clone()),
        shapes: diff.shapes.as_ref().map(|sd| inverse_indexed(&base.shapes, sd, inverse_shape)),
        notes: diff.notes.as_ref().map(|nd| inverse_indexed(&base.notes, nd, inverse_doc_block)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_master_diff(mut a: SlideMasterDiff, b: SlideMasterDiff) -> SlideMasterDiff {
    a.shapes = absorb_opt(a.shapes.take(), b.shapes, |x, y| absorb_indexed(x, y, absorb_shape, shape_with_diff_applied));
    a
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_layout_diff(mut a: SlideLayoutDiff, b: SlideLayoutDiff) -> SlideLayoutDiff {
    if b.master_id.is_some() {
        a.master_id = b.master_id;
    }
    a.shapes = absorb_opt(a.shapes.take(), b.shapes, |x, y| absorb_indexed(x, y, absorb_shape, shape_with_diff_applied));
    a
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_slide_diff(mut a: SlideDiff, b: SlideDiff) -> SlideDiff {
    if b.id.is_some() {
        a.id = b.id;
    }
    if b.layout_id.is_some() {
        a.layout_id = b.layout_id;
    }
    a.shapes = absorb_opt(a.shapes.take(), b.shapes, |x, y| absorb_indexed(x, y, absorb_shape, shape_with_diff_applied));
    a.notes = absorb_opt(a.notes.take(), b.notes, |x, y| absorb_indexed(x, y, absorb_doc_block, doc_block_with_diff_applied));
    a
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_snapshot(base: &SemioPresentationSnapshot, other: &SemioPresentationSnapshot) -> SemioPresentationDiff {
    SemioPresentationDiff {
        masters: between_named(&base.masters, &other.masters, |m| m.id.clone(), diff_master),
        layouts: between_named(&base.layouts, &other.layouts, |l| l.id.clone(), diff_layout),
        slides: between_indexed(&base.slides, &other.slides, diff_slide),
    }
}
//#endregion 🔖️StructureDiffLogic

//#region 🔖️Apply
impl MutationDiff<SemioPresentationSnapshot> for SemioPresentationDiff {
    fn apply(&self, base: &SemioPresentationSnapshot) -> protocol::MutationApplyResult<SemioPresentationSnapshot> {
        let mut next = base.clone();
        if let Some(d) = &self.masters {
            crate::standards::v1::subsets::base::schema::triples::validate_named_triple(&next.masters, d, |item| item.id.clone(), |item| item.id.clone(), ["masters"])?;
            apply_named(&mut next.masters, d, |m| m.id.clone(), apply_master);
        }
        if let Some(d) = &self.layouts {
            crate::standards::v1::subsets::base::schema::triples::validate_named_triple(&next.layouts, d, |item| item.id.clone(), |item| item.id.clone(), ["layouts"])?;
            apply_named(&mut next.layouts, d, |l| l.id.clone(), apply_layout);
        }
        if let Some(d) = &self.slides {
            crate::standards::v1::subsets::base::schema::triples::validate_indexed_triple(d, next.slides.len(), ["slides"])?;
            apply_indexed(&mut next.slides, d, apply_slide);
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        self.masters = absorb_opt(self.masters.take(), other.masters, |a, b| absorb_named(a, &b, |m| m.id.clone(), absorb_master_diff, apply_master));
        self.layouts = absorb_opt(self.layouts.take(), other.layouts, |a, b| absorb_named(a, &b, |l| l.id.clone(), absorb_layout_diff, apply_layout));
        self.slides = absorb_opt(self.slides.take(), other.slides, |a, b| absorb_indexed(a, b, absorb_slide_diff, slide_with_diff_applied));
    }
}
//#endregion 🔖️Apply

//#region 🔖️DiffAlgebra
impl DiffAlgebra<SemioPresentationSnapshot> for SemioPresentationDiff {
    fn inverse(&self, base: &SemioPresentationSnapshot) -> Self {
        SemioPresentationDiff {
            masters: self.masters.as_ref().map(|d| inverse_named(&base.masters, d, |m| m.id.clone(), inverse_master)),
            layouts: self.layouts.as_ref().map(|d| inverse_named(&base.layouts, d, |l| l.id.clone(), inverse_layout)),
            slides: self.slides.as_ref().map(|d| inverse_indexed(&base.slides, d, inverse_slide)),
        }
    }

    fn between(base: &SemioPresentationSnapshot, other: &SemioPresentationSnapshot) -> Self {
        diff_snapshot(base, other)
    }

    fn is_empty(&self) -> bool {
        self.masters.is_none() && self.layouts.is_none() && self.slides.is_none()
    }
}
//#endregion 🔖️DiffAlgebra

//#region 🔖️MutationDiffHelpers
/// 🧩 `SetSnapshot`'s diff — no `snapshot: Option<...>` full-replace slot, this IS
/// `SemioPresentationDiff::between`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_snapshot(base: &SemioPresentationSnapshot, next: &SemioPresentationSnapshot) -> SemioPresentationDiff {
    SemioPresentationDiff::between(base, next)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn wrap_slide_diff(index: usize, sd: SlideDiff) -> SemioPresentationDiff {
    SemioPresentationDiff { masters: None, layouts: None, slides: Some(SlidesDiff { modified: vec![IndexModified { index, diff: sd }], ..Default::default() }) }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn wrap_shape_diff(slide_index: usize, shape_index: usize, shape_diff: SlideShapeDiff) -> SemioPresentationDiff {
    let shapes_diff = SlideShapesDiff { modified: vec![IndexModified { index: shape_index, diff: shape_diff }], ..Default::default() };
    wrap_slide_diff(slide_index, SlideDiff { id: None, layout_id: None, shapes: Some(shapes_diff), notes: None })
}

/// 🧩 Diff for inserting `🎞️slide` at `index` (FINAL-state index).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_slide(index: usize, slide: Slide) -> SemioPresentationDiff {
    SemioPresentationDiff { masters: None, layouts: None, slides: Some(SlidesDiff { added: vec![IndexAdded { index, item: slide }], ..Default::default() }) }
}
/// 🧩 Diff for removing the slide at `index` (BASE-state index).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_slide(index: usize) -> SemioPresentationDiff {
    SemioPresentationDiff { masters: None, layouts: None, slides: Some(SlidesDiff { removed: vec![index], ..Default::default() }) }
}
/// 🧩 Diff for setting (or clearing, `layout_id: None`) slide `index`'s `layout_id`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_slide_layout(base: &SemioPresentationSnapshot, index: usize, layout_id: Option<String>) -> SemioPresentationDiff {
    let Some(slide) = base.slides.get(index) else { return SemioPresentationDiff::default() };
    if slide.layout_id == layout_id {
        return SemioPresentationDiff::default();
    }
    wrap_slide_diff(index, SlideDiff { id: None, layout_id: Some(layout_id), shapes: None, notes: None })
}
/// 🧩 Diff for replacing slide `index`'s `notes`, via a real structural comparison.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_slide_notes(base: &SemioPresentationSnapshot, index: usize, notes: &[DocBlock]) -> SemioPresentationDiff {
    let Some(slide) = base.slides.get(index) else { return SemioPresentationDiff::default() };
    let Some(notes_diff) = between_indexed(&slide.notes, notes, diff_doc_block) else { return SemioPresentationDiff::default() };
    wrap_slide_diff(index, SlideDiff { id: None, layout_id: None, shapes: None, notes: Some(notes_diff) })
}
/// 🧩 Diff for inserting `shape` at `shape_index` on slide `slide_index`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_shape(slide_index: usize, shape_index: usize, shape: SlideShape) -> SemioPresentationDiff {
    let shapes_diff = SlideShapesDiff { added: vec![IndexAdded { index: shape_index, item: shape }], ..Default::default() };
    wrap_slide_diff(slide_index, SlideDiff { id: None, layout_id: None, shapes: Some(shapes_diff), notes: None })
}
/// 🧩 Diff for removing the shape at `shape_index` on slide `slide_index`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_shape(slide_index: usize, shape_index: usize) -> SemioPresentationDiff {
    let shapes_diff = SlideShapesDiff { removed: vec![shape_index], ..Default::default() };
    wrap_slide_diff(slide_index, SlideDiff { id: None, layout_id: None, shapes: Some(shapes_diff), notes: None })
}
/// 🧩 Diff for setting shape `shape_index`'s frame on slide `slide_index`, via a real structural
/// comparison against the shape's current frame.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_shape_frame(base: &SemioPresentationSnapshot, slide_index: usize, shape_index: usize, frame: SlideFrame) -> SemioPresentationDiff {
    let Some(shape) = base.slides.get(slide_index).and_then(|s| s.shapes.get(shape_index)) else { return SemioPresentationDiff::default() };
    let Some(frame_diff) = diff_frame(frame_of(shape), &frame) else { return SemioPresentationDiff::default() };
    wrap_shape_diff(slide_index, shape_index, shape_diff_frame_only(shape, frame_diff))
}
/// 🧩 Diff for replacing a `TextBox` shape's `blocks`, via a real structural comparison.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_textbox_blocks(base: &SemioPresentationSnapshot, slide_index: usize, shape_index: usize, blocks: &[DocBlock]) -> SemioPresentationDiff {
    let Some(SlideShape::TextBox { blocks: old, .. }) = base.slides.get(slide_index).and_then(|s| s.shapes.get(shape_index)) else {
        return SemioPresentationDiff::default();
    };
    let Some(blocks_diff) = between_indexed(old, blocks, diff_doc_block) else { return SemioPresentationDiff::default() };
    wrap_shape_diff(slide_index, shape_index, SlideShapeDiff::TextBox { frame: None, blocks: Some(blocks_diff) })
}
/// 🧭️ Read-only accessor: every `SlideShape` variant carries a `frame`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn frame_of(shape: &SlideShape) -> &SlideFrame {
    match shape {
        SlideShape::TextBox { frame, .. } | SlideShape::Picture { frame, .. } | SlideShape::Table { frame, .. } | SlideShape::Placeholder { frame, .. } => frame,
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn shape_diff_frame_only(shape: &SlideShape, frame_diff: SlideFrameDiff) -> SlideShapeDiff {
    match shape {
        SlideShape::TextBox { .. } => SlideShapeDiff::TextBox { frame: Some(frame_diff), blocks: None },
        SlideShape::Picture { .. } => SlideShapeDiff::Picture { frame: Some(frame_diff), image: None },
        SlideShape::Table { .. } => SlideShapeDiff::Table { frame: Some(frame_diff), rows: None },
        SlideShape::Placeholder { .. } => SlideShapeDiff::Placeholder { frame: Some(frame_diff), kind: None },
    }
}

/// 🧩 Diff for inserting a master.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_master(master: SlideMaster) -> SemioPresentationDiff {
    SemioPresentationDiff { masters: Some(SlideMastersDiff { added: vec![master], ..Default::default() }), layouts: None, slides: None }
}
/// 🧩 Diff for removing the master with id `id`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_master(id: &str) -> SemioPresentationDiff {
    SemioPresentationDiff { masters: Some(SlideMastersDiff { removed: vec![id.to_string()], ..Default::default() }), layouts: None, slides: None }
}
/// 🧩 Diff for inserting a layout.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_layout(layout: SlideLayout) -> SemioPresentationDiff {
    SemioPresentationDiff { masters: None, layouts: Some(SlideLayoutsDiff { added: vec![layout], ..Default::default() }), slides: None }
}
/// 🧩 Diff for removing the layout with id `id`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_layout(id: &str) -> SemioPresentationDiff {
    SemioPresentationDiff { masters: None, layouts: Some(SlideLayoutsDiff { removed: vec![id.to_string()], ..Default::default() }), slides: None }
}
/// 🧩 Diff for setting a layout's `master_id`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_layout_master(id: &str, master_id: &str) -> SemioPresentationDiff {
    let ld = SlideLayoutDiff { master_id: Some(master_id.to_string()), shapes: None };
    SemioPresentationDiff { masters: None, layouts: Some(SlideLayoutsDiff { modified: vec![NamedModified { key: id.to_string(), diff: ld }], ..Default::default() }), slides: None }
}
//#endregion 🔖️MutationDiffHelpers

//#region 🔖️HandcraftedDiffCodec
/// 🧪️ Hand-rolled `protocol::DiffCodec` — same grammar style docx/gif/svg's own hand-rolled
/// codecs use (bracket-depth-aware split via the shared `engine::triples` primitives, hex for
/// strings/bytes, `[0]`/`[1,x]` for `Option<T>`, single uppercase tag letters for data-carrying
/// enums). `IndexedTripleDiff`/`NamedTripleDiff`'s own `enc_indexed_triple`/`enc_named_triple`
/// (shared `engine::triples`) drive every collection instantiation below.
//#region 🔖️Primitives











//#endregion 🔖️Primitives

//#region 🔖️ValueCodecs






















//#endregion 🔖️ValueCodecs

//#region 🔖️GenericTripleCodecs




//#endregion 🔖️GenericTripleCodecs

//#region 🔖️DiffValueCodecs






























//#endregion 🔖️DiffValueCodecs

//#region 🔖️TopLevel



//#region 🔖️BinaryPrimitives




//#endregion 🔖️BinaryPrimitives


//#endregion 🔖️TopLevel

//#region 🔖️Demo
/// 🌱 Representative `SemioPresentationDiff` fixtures — promoted to module scope (from the old
/// `mod handcrafted_diff_codec_tests`-local helpers) so `🎹️composer/🦀️.rs`'s conformance
/// laws AND `🧬️mutations/🦀️.rs`'s own test fixtures can reuse them, same promotion model
/// every prior semio wave uses.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn snapshot_a() -> SemioPresentationSnapshot {
    SemioPresentationSnapshot {
        schema: "s.stdio.semio.presentation".into(),
        masters: vec![
            SlideMaster { id: "keep".into(), shapes: vec![SlideShape::Placeholder { frame: SlideFrame { origin: SemioPoint2 { x: 0.0, y: 0.0 }, width: 10.0, height: 10.0 }, kind: PlaceholderKind::Title }] },
            SlideMaster { id: "toRemove".into(), shapes: Vec::new() },
        ],
        layouts: vec![SlideLayout { id: "layout1".into(), master_id: "toRemove".into(), shapes: Vec::new() }],
        slides: vec![
            Slide { id: "s1".into(), layout_id: None, shapes: vec![SlideShape::TextBox { frame: SlideFrame { origin: SemioPoint2 { x: 1.0, y: 1.0 }, width: 5.0, height: 5.0 }, blocks: vec![DocBlock::paragraph("old")] }], notes: Vec::new() },
            Slide { id: "toDrop".into(), layout_id: Some("layout1".into()), shapes: Vec::new(), notes: Vec::new() },
        ],
    }
}

#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn snapshot_b() -> SemioPresentationSnapshot {
    SemioPresentationSnapshot {
        schema: "s.stdio.semio.presentation".into(),
        masters: vec![SlideMaster { id: "keep".into(), shapes: Vec::new() }, SlideMaster { id: "added".into(), shapes: Vec::new() }],
        layouts: vec![SlideLayout { id: "layout1".into(), master_id: "keep".into(), shapes: Vec::new() }],
        slides: vec![Slide {
            id: "s1".into(),
            layout_id: Some("layout1".into()),
            shapes: vec![
                SlideShape::TextBox { frame: SlideFrame { origin: SemioPoint2 { x: 1.0, y: 1.0 }, width: 5.0, height: 5.0 }, blocks: vec![DocBlock::paragraph("new")] },
                SlideShape::Picture { frame: SlideFrame { origin: SemioPoint2::default(), width: 1.0, height: 1.0 }, image: SlidePictureImage { asset_id: "a".into(), mime: "image/png".into(), bytes: vec![9] } },
            ],
            notes: vec![DocBlock::paragraph("noted")],
        }],
    }
}

/// 🌱 Representative `SemioPresentationDiff` cases (empty/no-op, a full masters+layouts+slides
/// sweep both directions, reusing `snapshot_a`/`snapshot_b`, and a slide reorder whose slots carry
/// their new identities) — single source of truth for `grammar_conformance_law`/`protocol_walk_law`
/// in `🎹️composer/🦀️.rs`.
#[cfg(all(test, feature = "conversion-presentation"))]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<SemioPresentationDiff> {
    let a = snapshot_a();
    let b = snapshot_b();
    let mut reordered = a.clone();
    reordered.slides.reverse();
    vec![SemioPresentationDiff::default(), SemioPresentationDiff::between(&a, &b), SemioPresentationDiff::between(&b, &a), SemioPresentationDiff::between(&a, &a), SemioPresentationDiff::between(&a, &reordered)]
}
//#endregion 🔖️Demo

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️handcrafted-diff-codec/🦀️.rs"]
mod handcrafted_diff_codec_tests;
//#endregion 🧪️Tests
//#endregion 🔖️HandcraftedDiffCodec
