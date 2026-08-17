//! 🔺️ SemioPresentationDiff — handcrafted sparse diff over `SemioPresentationSnapshot`
//! (`masters`/`layouts`/`slides`). No `snapshot: Option<SemioPresentationSnapshot>` full-replace
//! slot — even `SetSnapshot`'s diff is the sparse field-by-field `SemioPresentationDiff::between`.
//!
//! Collection key kinds (per the recipe's "Key kinds per collection" rule): `masters`/`layouts`
//! are id-keyed (`NamedTripleDiff`, referenced BY id from `layouts.master_id`/`slides.layout_id`,
//! like docx's name-keyed `styles`); `slides` is INDEX-keyed (`IndexedTripleDiff`, presentation
//! order is significant, like pdf page order) even though each `Slide` also carries its own `id`
//! identity field. `shapes` (owned by masters/layouts/slides alike), `notes`, and `TextBox`/table
//! `blocks` are all index-keyed too.
//!
//! `document::DocBlock` is reused verbatim for text content (`TextBox.blocks`, table cell
//! `blocks`, `Slide.notes`) but is OWNED by the `document` subset, out of this file's write scope
//! — it has no field-level diff type of its own exposed yet, so this file diffs `Vec<DocBlock>`
//! items as WHOLE VALUES (`D = T = DocBlock`; "modified" carries the complete replacement block).
//! This is honest per the recipe's weak/strong-entity split: from this subset's point of view
//! `DocBlock` is a value struct it does not own the internals of, so it is never sub-diffed here.

use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint2;
use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::{split_top_level, strip_brackets, IndexAdded, IndexModified, IndexedTripleDiff, NamedModified, NamedTripleDiff};
/// 🧱️ REUSE, don't reinvent — `document::DocBlock`'s own real, already-tested text codec
/// (`ws-codec-document-report.md`), re-exported here so both this file's own leaf encoders AND
/// the sibling `🧬️mutations`/`📸️snapshot` facets can import `{enc_block, dec_block}` from THIS
/// module (matching the pre-existing convention where this file is the one place that owns every
/// value codec presentation's other facets import from).
pub(crate) use crate::artifacts::semio::standards::v1::subsets::document::schema::diff::{dec_block, enc_block};
use crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::DocBlock;
use crate::artifacts::semio::standards::v1::subsets::presentation::schema::snapshot::SemioPresentationSnapshot;
use crate::artifacts::semio::standards::v1::subsets::presentation::schema::snapshot::{PlaceholderKind, Slide, SlideFrame, SlideLayout, SlideMaster, SlidePictureImage, SlideShape, SlideTableCell, SlideTableRow};
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SlideFrameDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<SemioPoint2>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SlidePictureImageDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mime: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bytes: Option<Vec<u8>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SlideTableCellDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocks: Option<DocBlocksDiff>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SlideTableRowDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cells: Option<SlideTableCellsDiff>,
}

/// 🌳️ Per-shape diff, shaped like `SlideShape` (`Replace` covers a shape-KIND change, e.g.
/// `TextBox` -> `Picture`, same convention as docx's `DocxBlockDiff::Replace`). Tag is
/// `shapeKind` (not `kind`) for the same field/tag-collision reason as `SlideShape` itself (see
/// that type's doc comment) — `Placeholder`'s own `kind` field would otherwise collide.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "shapeKind", rename_all = "camelCase")]
pub enum SlideShapeDiff {
    TextBox {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        frame: Option<SlideFrameDiff>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        blocks: Option<DocBlocksDiff>,
    },
    Picture {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        frame: Option<SlideFrameDiff>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        image: Option<SlidePictureImageDiff>,
    },
    Table {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        frame: Option<SlideFrameDiff>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        rows: Option<SlideTableRowsDiff>,
    },
    Placeholder {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        frame: Option<SlideFrameDiff>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        kind: Option<PlaceholderKind>,
    },
    Replace {
        shape: SlideShape,
    },
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SlideMasterDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shapes: Option<SlideShapesDiff>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SlideLayoutDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub master_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shapes: Option<SlideShapesDiff>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SlideDiff {
    /// 🏳️ Tri-state: `None` = unchanged, `Some(None)` = layout cleared, `Some(Some(id))` = set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layout_id: Option<Option<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shapes: Option<SlideShapesDiff>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<DocBlocksDiff>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.presentation.diff")]
pub struct SemioPresentationDiff {
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub masters: Option<SlideMastersDiff>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layouts: Option<SlideLayoutsDiff>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slides: Option<SlidesDiff>,
}
//#endregion 🔖️DiffTypes

//#region 🔖️GenericIndexedEngine
/// 🧮️ Own copy of the generic index-keyed between/apply/inverse/absorb algorithm (docx precedent
/// — every hand-rolled artifact re-derives this small engine against the SHARED `IndexedTripleDiff`
/// type rather than importing a shared algorithm module).
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

//#region 🔖️GenericNamedEngine
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

fn absorb_named<K, T, D>(d1: NamedTripleDiff<K, D, T>, d2: NamedTripleDiff<K, D, T>, key_of: impl Fn(&T) -> K, absorb_item: impl Fn(D, D) -> D, apply_item: impl Fn(&mut T, &D)) -> NamedTripleDiff<K, D, T>
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
fn absorb_opt<T>(a: Option<T>, b: Option<T>, f: impl FnOnce(T, T) -> T) -> Option<T> {
    match (a, b) {
        (None, x) => x,
        (x, None) => x,
        (Some(x), Some(y)) => Some(f(x, y)),
    }
}
//#endregion 🔖️GenericNamedEngine

//#region 🔖️ValueDiffLogic
fn diff_frame(old: &SlideFrame, new: &SlideFrame) -> Option<SlideFrameDiff> {
    if old == new {
        return None;
    }
    Some(SlideFrameDiff { origin: (old.origin != new.origin).then_some(new.origin), width: (old.width != new.width).then_some(new.width), height: (old.height != new.height).then_some(new.height) })
}
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
fn frame_with_diff_applied(frame: &SlideFrame, diff: &SlideFrameDiff) -> SlideFrame {
    let mut out = *frame;
    apply_frame(&mut out, diff);
    out
}
fn inverse_frame(base: &SlideFrame, diff: &SlideFrameDiff) -> SlideFrameDiff {
    SlideFrameDiff { origin: diff.origin.map(|_| base.origin), width: diff.width.map(|_| base.width), height: diff.height.map(|_| base.height) }
}
fn absorb_frame(mut a: SlideFrameDiff, b: SlideFrameDiff) -> SlideFrameDiff {
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

fn diff_image(old: &SlidePictureImage, new: &SlidePictureImage) -> Option<SlidePictureImageDiff> {
    if old == new {
        return None;
    }
    Some(SlidePictureImageDiff { asset_id: (old.asset_id != new.asset_id).then(|| new.asset_id.clone()), mime: (old.mime != new.mime).then(|| new.mime.clone()), bytes: (old.bytes != new.bytes).then(|| new.bytes.clone()) })
}
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
fn image_with_diff_applied(image: &SlidePictureImage, diff: &SlidePictureImageDiff) -> SlidePictureImage {
    let mut out = image.clone();
    apply_image(&mut out, diff);
    out
}
fn inverse_image(base: &SlidePictureImage, diff: &SlidePictureImageDiff) -> SlidePictureImageDiff {
    SlidePictureImageDiff { asset_id: diff.asset_id.as_ref().map(|_| base.asset_id.clone()), mime: diff.mime.as_ref().map(|_| base.mime.clone()), bytes: diff.bytes.as_ref().map(|_| base.bytes.clone()) }
}
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
fn diff_doc_block(old: &DocBlock, new: &DocBlock) -> Option<DocBlock> {
    (old != new).then(|| new.clone())
}
fn apply_doc_block(block: &mut DocBlock, diff: &DocBlock) {
    *block = diff.clone();
}
fn doc_block_with_diff_applied(_block: &DocBlock, diff: &DocBlock) -> DocBlock {
    diff.clone()
}
fn inverse_doc_block(base: &DocBlock, _diff: &DocBlock) -> DocBlock {
    base.clone()
}
fn absorb_doc_block(_a: DocBlock, b: DocBlock) -> DocBlock {
    b
}
//#endregion 🔖️ValueDiffLogic

//#region 🔖️ShapeDiffLogic
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

fn diff_table_cell(old: &SlideTableCell, new: &SlideTableCell) -> Option<SlideTableCellDiff> {
    let blocks = between_indexed(&old.blocks, &new.blocks, diff_doc_block);
    blocks.map(|blocks| SlideTableCellDiff { blocks: Some(blocks) })
}
fn diff_table_row(old: &SlideTableRow, new: &SlideTableRow) -> Option<SlideTableRowDiff> {
    let cells = between_indexed(&old.cells, &new.cells, diff_table_cell);
    cells.map(|cells| SlideTableRowDiff { cells: Some(cells) })
}

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
fn apply_table_row(row: &mut SlideTableRow, diff: &SlideTableRowDiff) {
    if let Some(cd) = &diff.cells {
        apply_indexed(&mut row.cells, cd, apply_table_cell);
    }
}
fn apply_table_cell(cell: &mut SlideTableCell, diff: &SlideTableCellDiff) {
    if let Some(bd) = &diff.blocks {
        apply_indexed(&mut cell.blocks, bd, apply_doc_block);
    }
}
fn shape_with_diff_applied(shape: &SlideShape, diff: &SlideShapeDiff) -> SlideShape {
    let mut out = shape.clone();
    apply_shape(&mut out, diff);
    out
}
fn table_row_with_diff_applied(row: &SlideTableRow, diff: &SlideTableRowDiff) -> SlideTableRow {
    let mut out = row.clone();
    apply_table_row(&mut out, diff);
    out
}

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
fn inverse_table_row(base: &SlideTableRow, diff: &SlideTableRowDiff) -> SlideTableRowDiff {
    SlideTableRowDiff { cells: diff.cells.as_ref().map(|cd| inverse_indexed(&base.cells, cd, inverse_table_cell)) }
}
fn inverse_table_cell(base: &SlideTableCell, diff: &SlideTableCellDiff) -> SlideTableCellDiff {
    SlideTableCellDiff { blocks: diff.blocks.as_ref().map(|bd| inverse_indexed(&base.blocks, bd, inverse_doc_block)) }
}

fn absorb_shape(a: SlideShapeDiff, b: SlideShapeDiff) -> SlideShapeDiff {
    match (a, b) {
        (_, SlideShapeDiff::Replace { shape }) => SlideShapeDiff::Replace { shape },
        (SlideShapeDiff::Replace { shape }, b) => SlideShapeDiff::Replace { shape: shape_with_diff_applied(&shape, &b) },
        (SlideShapeDiff::TextBox { frame: fa, blocks: ba }, SlideShapeDiff::TextBox { frame: fb, blocks: bb }) => {
            SlideShapeDiff::TextBox { frame: absorb_opt(fa, fb, absorb_frame), blocks: absorb_opt(ba, bb, |x, y| absorb_indexed(x, y, absorb_doc_block, doc_block_with_diff_applied)) }
        }
        (SlideShapeDiff::Picture { frame: fa, image: ia }, SlideShapeDiff::Picture { frame: fb, image: ib }) => SlideShapeDiff::Picture { frame: absorb_opt(fa, fb, absorb_frame), image: absorb_opt(ia, ib, absorb_image) },
        (SlideShapeDiff::Table { frame: fa, rows: ra }, SlideShapeDiff::Table { frame: fb, rows: rb }) => {
            SlideShapeDiff::Table { frame: absorb_opt(fa, fb, absorb_frame), rows: absorb_opt(ra, rb, |x, y| absorb_indexed(x, y, absorb_table_row_diff, table_row_with_diff_applied)) }
        }
        (SlideShapeDiff::Placeholder { frame: fa, kind: ka }, SlideShapeDiff::Placeholder { frame: fb, kind: kb }) => SlideShapeDiff::Placeholder { frame: absorb_opt(fa, fb, absorb_frame), kind: kb.or(ka) },
        (_, b) => b,
    }
}
fn absorb_table_cell_diff(mut a: SlideTableCellDiff, b: SlideTableCellDiff) -> SlideTableCellDiff {
    a.blocks = absorb_opt(a.blocks.take(), b.blocks, |x, y| absorb_indexed(x, y, absorb_doc_block, doc_block_with_diff_applied));
    a
}
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
fn diff_master(old: &SlideMaster, new: &SlideMaster) -> Option<SlideMasterDiff> {
    let shapes = between_indexed(&old.shapes, &new.shapes, diff_shape);
    shapes.map(|shapes| SlideMasterDiff { shapes: Some(shapes) })
}
fn diff_layout(old: &SlideLayout, new: &SlideLayout) -> Option<SlideLayoutDiff> {
    let master_id = (old.master_id != new.master_id).then(|| new.master_id.clone());
    let shapes = between_indexed(&old.shapes, &new.shapes, diff_shape);
    if master_id.is_none() && shapes.is_none() {
        None
    } else {
        Some(SlideLayoutDiff { master_id, shapes })
    }
}
fn diff_slide(old: &Slide, new: &Slide) -> Option<SlideDiff> {
    let layout_id = (old.layout_id != new.layout_id).then(|| new.layout_id.clone());
    let shapes = between_indexed(&old.shapes, &new.shapes, diff_shape);
    let notes = between_indexed(&old.notes, &new.notes, diff_doc_block);
    if layout_id.is_none() && shapes.is_none() && notes.is_none() {
        None
    } else {
        Some(SlideDiff { layout_id, shapes, notes })
    }
}

fn apply_master(master: &mut SlideMaster, diff: &SlideMasterDiff) {
    if let Some(sd) = &diff.shapes {
        apply_indexed(&mut master.shapes, sd, apply_shape);
    }
}
fn apply_layout(layout: &mut SlideLayout, diff: &SlideLayoutDiff) {
    if let Some(v) = &diff.master_id {
        layout.master_id = v.clone();
    }
    if let Some(sd) = &diff.shapes {
        apply_indexed(&mut layout.shapes, sd, apply_shape);
    }
}
fn apply_slide(slide: &mut Slide, diff: &SlideDiff) {
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
fn slide_with_diff_applied(slide: &Slide, diff: &SlideDiff) -> Slide {
    let mut out = slide.clone();
    apply_slide(&mut out, diff);
    out
}

fn inverse_master(base: &SlideMaster, diff: &SlideMasterDiff) -> SlideMasterDiff {
    SlideMasterDiff { shapes: diff.shapes.as_ref().map(|sd| inverse_indexed(&base.shapes, sd, inverse_shape)) }
}
fn inverse_layout(base: &SlideLayout, diff: &SlideLayoutDiff) -> SlideLayoutDiff {
    SlideLayoutDiff { master_id: diff.master_id.as_ref().map(|_| base.master_id.clone()), shapes: diff.shapes.as_ref().map(|sd| inverse_indexed(&base.shapes, sd, inverse_shape)) }
}
fn inverse_slide(base: &Slide, diff: &SlideDiff) -> SlideDiff {
    SlideDiff {
        layout_id: diff.layout_id.as_ref().map(|_| base.layout_id.clone()),
        shapes: diff.shapes.as_ref().map(|sd| inverse_indexed(&base.shapes, sd, inverse_shape)),
        notes: diff.notes.as_ref().map(|nd| inverse_indexed(&base.notes, nd, inverse_doc_block)),
    }
}

fn absorb_master_diff(mut a: SlideMasterDiff, b: SlideMasterDiff) -> SlideMasterDiff {
    a.shapes = absorb_opt(a.shapes.take(), b.shapes, |x, y| absorb_indexed(x, y, absorb_shape, shape_with_diff_applied));
    a
}
fn absorb_layout_diff(mut a: SlideLayoutDiff, b: SlideLayoutDiff) -> SlideLayoutDiff {
    if b.master_id.is_some() {
        a.master_id = b.master_id;
    }
    a.shapes = absorb_opt(a.shapes.take(), b.shapes, |x, y| absorb_indexed(x, y, absorb_shape, shape_with_diff_applied));
    a
}
fn absorb_slide_diff(mut a: SlideDiff, b: SlideDiff) -> SlideDiff {
    if b.layout_id.is_some() {
        a.layout_id = b.layout_id;
    }
    a.shapes = absorb_opt(a.shapes.take(), b.shapes, |x, y| absorb_indexed(x, y, absorb_shape, shape_with_diff_applied));
    a.notes = absorb_opt(a.notes.take(), b.notes, |x, y| absorb_indexed(x, y, absorb_doc_block, doc_block_with_diff_applied));
    a
}

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
            crate::artifacts::semio::standards::v1::subsets::any::schema::triples::validate_named_triple(&next.masters, d, |item| item.id.clone(), |item| item.id.clone(), ["masters"])?;
            apply_named(&mut next.masters, d, |m| m.id.clone(), apply_master);
        }
        if let Some(d) = &self.layouts {
            crate::artifacts::semio::standards::v1::subsets::any::schema::triples::validate_named_triple(&next.layouts, d, |item| item.id.clone(), |item| item.id.clone(), ["layouts"])?;
            apply_named(&mut next.layouts, d, |l| l.id.clone(), apply_layout);
        }
        if let Some(d) = &self.slides {
            crate::artifacts::semio::standards::v1::subsets::any::schema::triples::validate_indexed_triple(d, next.slides.len(), ["slides"])?;
            apply_indexed(&mut next.slides, d, apply_slide);
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        self.masters = absorb_opt(self.masters.take(), other.masters, |a, b| absorb_named(a, b, |m| m.id.clone(), absorb_master_diff, apply_master));
        self.layouts = absorb_opt(self.layouts.take(), other.layouts, |a, b| absorb_named(a, b, |l| l.id.clone(), absorb_layout_diff, apply_layout));
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
pub fn diff_set_snapshot(base: &SemioPresentationSnapshot, next: &SemioPresentationSnapshot) -> SemioPresentationDiff {
    SemioPresentationDiff::between(base, next)
}

fn wrap_slide_diff(index: usize, sd: SlideDiff) -> SemioPresentationDiff {
    SemioPresentationDiff { masters: None, layouts: None, slides: Some(SlidesDiff { modified: vec![IndexModified { index, diff: sd }], ..Default::default() }) }
}
fn wrap_shape_diff(slide_index: usize, shape_index: usize, shape_diff: SlideShapeDiff) -> SemioPresentationDiff {
    let shapes_diff = SlideShapesDiff { modified: vec![IndexModified { index: shape_index, diff: shape_diff }], ..Default::default() };
    wrap_slide_diff(slide_index, SlideDiff { layout_id: None, shapes: Some(shapes_diff), notes: None })
}

/// 🧩 Diff for inserting `slide` at `index` (FINAL-state index).
pub fn diff_insert_slide(index: usize, slide: Slide) -> SemioPresentationDiff {
    SemioPresentationDiff { masters: None, layouts: None, slides: Some(SlidesDiff { added: vec![IndexAdded { index, item: slide }], ..Default::default() }) }
}
/// 🧩 Diff for removing the slide at `index` (BASE-state index).
pub fn diff_remove_slide(index: usize) -> SemioPresentationDiff {
    SemioPresentationDiff { masters: None, layouts: None, slides: Some(SlidesDiff { removed: vec![index], ..Default::default() }) }
}
/// 🧩 Diff for setting (or clearing, `layout_id: None`) slide `index`'s `layout_id`.
pub fn diff_set_slide_layout(base: &SemioPresentationSnapshot, index: usize, layout_id: Option<String>) -> SemioPresentationDiff {
    let Some(slide) = base.slides.get(index) else { return SemioPresentationDiff::default() };
    if slide.layout_id == layout_id {
        return SemioPresentationDiff::default();
    }
    wrap_slide_diff(index, SlideDiff { layout_id: Some(layout_id), shapes: None, notes: None })
}
/// 🧩 Diff for replacing slide `index`'s `notes`, via a real structural comparison.
pub fn diff_set_slide_notes(base: &SemioPresentationSnapshot, index: usize, notes: Vec<DocBlock>) -> SemioPresentationDiff {
    let Some(slide) = base.slides.get(index) else { return SemioPresentationDiff::default() };
    let Some(notes_diff) = between_indexed(&slide.notes, &notes, diff_doc_block) else { return SemioPresentationDiff::default() };
    wrap_slide_diff(index, SlideDiff { layout_id: None, shapes: None, notes: Some(notes_diff) })
}
/// 🧩 Diff for inserting `shape` at `shape_index` on slide `slide_index`.
pub fn diff_insert_shape(slide_index: usize, shape_index: usize, shape: SlideShape) -> SemioPresentationDiff {
    let shapes_diff = SlideShapesDiff { added: vec![IndexAdded { index: shape_index, item: shape }], ..Default::default() };
    wrap_slide_diff(slide_index, SlideDiff { layout_id: None, shapes: Some(shapes_diff), notes: None })
}
/// 🧩 Diff for removing the shape at `shape_index` on slide `slide_index`.
pub fn diff_remove_shape(slide_index: usize, shape_index: usize) -> SemioPresentationDiff {
    let shapes_diff = SlideShapesDiff { removed: vec![shape_index], ..Default::default() };
    wrap_slide_diff(slide_index, SlideDiff { layout_id: None, shapes: Some(shapes_diff), notes: None })
}
/// 🧩 Diff for setting shape `shape_index`'s frame on slide `slide_index`, via a real structural
/// comparison against the shape's current frame.
pub fn diff_set_shape_frame(base: &SemioPresentationSnapshot, slide_index: usize, shape_index: usize, frame: SlideFrame) -> SemioPresentationDiff {
    let Some(shape) = base.slides.get(slide_index).and_then(|s| s.shapes.get(shape_index)) else { return SemioPresentationDiff::default() };
    let Some(frame_diff) = diff_frame(frame_of(shape), &frame) else { return SemioPresentationDiff::default() };
    wrap_shape_diff(slide_index, shape_index, shape_diff_frame_only(shape, frame_diff))
}
/// 🧩 Diff for replacing a `TextBox` shape's `blocks`, via a real structural comparison.
pub fn diff_set_textbox_blocks(base: &SemioPresentationSnapshot, slide_index: usize, shape_index: usize, blocks: Vec<DocBlock>) -> SemioPresentationDiff {
    let Some(SlideShape::TextBox { blocks: old, .. }) = base.slides.get(slide_index).and_then(|s| s.shapes.get(shape_index)) else {
        return SemioPresentationDiff::default();
    };
    let Some(blocks_diff) = between_indexed(old, &blocks, diff_doc_block) else { return SemioPresentationDiff::default() };
    wrap_shape_diff(slide_index, shape_index, SlideShapeDiff::TextBox { frame: None, blocks: Some(blocks_diff) })
}
/// 🧭️ Read-only accessor: every `SlideShape` variant carries a `frame`.
pub fn frame_of(shape: &SlideShape) -> &SlideFrame {
    match shape {
        SlideShape::TextBox { frame, .. } | SlideShape::Picture { frame, .. } | SlideShape::Table { frame, .. } | SlideShape::Placeholder { frame, .. } => frame,
    }
}
fn shape_diff_frame_only(shape: &SlideShape, frame_diff: SlideFrameDiff) -> SlideShapeDiff {
    match shape {
        SlideShape::TextBox { .. } => SlideShapeDiff::TextBox { frame: Some(frame_diff), blocks: None },
        SlideShape::Picture { .. } => SlideShapeDiff::Picture { frame: Some(frame_diff), image: None },
        SlideShape::Table { .. } => SlideShapeDiff::Table { frame: Some(frame_diff), rows: None },
        SlideShape::Placeholder { .. } => SlideShapeDiff::Placeholder { frame: Some(frame_diff), kind: None },
    }
}

/// 🧩 Diff for inserting a master.
pub fn diff_insert_master(master: SlideMaster) -> SemioPresentationDiff {
    SemioPresentationDiff { masters: Some(SlideMastersDiff { added: vec![master], ..Default::default() }), layouts: None, slides: None }
}
/// 🧩 Diff for removing the master with id `id`.
pub fn diff_remove_master(id: &str) -> SemioPresentationDiff {
    SemioPresentationDiff { masters: Some(SlideMastersDiff { removed: vec![id.to_string()], ..Default::default() }), layouts: None, slides: None }
}
/// 🧩 Diff for inserting a layout.
pub fn diff_insert_layout(layout: SlideLayout) -> SemioPresentationDiff {
    SemioPresentationDiff { masters: None, layouts: Some(SlideLayoutsDiff { added: vec![layout], ..Default::default() }), slides: None }
}
/// 🧩 Diff for removing the layout with id `id`.
pub fn diff_remove_layout(id: &str) -> SemioPresentationDiff {
    SemioPresentationDiff { masters: None, layouts: Some(SlideLayoutsDiff { removed: vec![id.to_string()], ..Default::default() }), slides: None }
}
/// 🧩 Diff for setting a layout's `master_id`.
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
pub(crate) fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
pub(crate) fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
pub(crate) fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
pub(crate) fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
pub(crate) fn enc_f64(v: f64) -> String {
    v.to_bits().to_string()
}
pub(crate) fn enc_bool(b: bool) -> String {
    if b {
        "1".to_string()
    } else {
        "0".to_string()
    }
}
pub(crate) fn dec_bool(s: &str) -> Result<bool, String> {
    match s {
        "1" => Ok(true),
        "0" => Ok(false),
        other => Err(format!("bool: bad value {other:?}")),
    }
}
pub(crate) fn dec_f64(s: &str) -> Result<f64, String> {
    s.parse::<u64>().map(f64::from_bits).map_err(|e: std::num::ParseIntError| e.to_string())
}
pub(crate) fn parse_usize(s: &str) -> Result<usize, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}
pub(crate) fn encode_option<T>(opt: &Option<T>, enc: impl Fn(&T) -> String) -> String {
    match opt {
        None => "[0]".to_string(),
        Some(v) => format!("[1,{}]", enc(v)),
    }
}
pub(crate) fn decode_option<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Option<T>, String> {
    let inner = strip_brackets(s)?;
    match split_top_level(inner, ',').as_slice() {
        ["0"] => Ok(None),
        [tag, value] if *tag == "1" => Ok(Some(dec(value)?)),
        other => Err(format!("option decode: bad shape {other:?}")),
    }
}
pub(crate) fn enc_list<T>(items: &[T], enc: impl Fn(&T) -> String) -> String {
    format!("[{}]", items.iter().map(|i| enc(i)).collect::<Vec<_>>().join(","))
}
pub(crate) fn dec_list<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Vec<T>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec).collect()
}
//#endregion 🔖️Primitives

//#region 🔖️ValueCodecs
pub(crate) fn enc_semio_point2(p: &SemioPoint2) -> String {
    format!("[{},{}]", enc_f64(p.x), enc_f64(p.y))
}
pub(crate) fn dec_semio_point2(s: &str) -> Result<SemioPoint2, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [x, y] = parts.as_slice() else { return Err(format!("point2: expected 2 fields, got {}", parts.len())) };
    Ok(SemioPoint2 { x: dec_f64(x)?, y: dec_f64(y)? })
}
pub(crate) fn enc_frame(f: &SlideFrame) -> String {
    format!("[{},{},{}]", enc_semio_point2(&f.origin), enc_f64(f.width), enc_f64(f.height))
}
pub(crate) fn dec_frame(s: &str) -> Result<SlideFrame, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [origin, width, height] = parts.as_slice() else { return Err(format!("frame: expected 3 fields, got {}", parts.len())) };
    Ok(SlideFrame { origin: dec_semio_point2(origin)?, width: dec_f64(width)?, height: dec_f64(height)? })
}
pub(crate) fn enc_image(i: &SlidePictureImage) -> String {
    format!("[{},{},{}]", enc_str(&i.asset_id), enc_str(&i.mime), hex_encode(&i.bytes))
}
pub(crate) fn dec_image(s: &str) -> Result<SlidePictureImage, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [asset_id, mime, bytes] = parts.as_slice() else { return Err(format!("image: expected 3 fields, got {}", parts.len())) };
    Ok(SlidePictureImage { asset_id: dec_str(asset_id)?, mime: dec_str(mime)?, bytes: hex_decode(bytes)? })
}
pub(crate) fn enc_placeholder_kind(k: &PlaceholderKind) -> String {
    match k {
        PlaceholderKind::Title => "T".to_string(),
        PlaceholderKind::Subtitle => "S".to_string(),
        PlaceholderKind::Body => "B".to_string(),
        PlaceholderKind::Footer => "F".to_string(),
        PlaceholderKind::SlideNumber => "N".to_string(),
        PlaceholderKind::DateTime => "D".to_string(),
        PlaceholderKind::Other { value } => format!("O[{}]", enc_str(value)),
    }
}
pub(crate) fn dec_placeholder_kind(s: &str) -> Result<PlaceholderKind, String> {
    match s {
        "T" => Ok(PlaceholderKind::Title),
        "S" => Ok(PlaceholderKind::Subtitle),
        "B" => Ok(PlaceholderKind::Body),
        "F" => Ok(PlaceholderKind::Footer),
        "N" => Ok(PlaceholderKind::SlideNumber),
        "D" => Ok(PlaceholderKind::DateTime),
        other if other.starts_with('O') => Ok(PlaceholderKind::Other { value: dec_str(strip_brackets(&other[1..])?)? }),
        other => Err(format!("placeholder kind: unknown tag {other:?}")),
    }
}
/// 🧱️ `DocRun`/`RunStyle`/`DocListItem`/`DocTableCell`/`DocTableRow`/`DocBlock` are all OWNED by
/// `document` — no local codec for any of them lives here anymore. `enc_block`/`dec_block`
/// (re-exported above from `document::schema::diff`, the same real, already-tested codec
/// `ws-codec-document-report.md` landed) already handles every one of these leaf types internally
/// (Paragraph/Heading's `runs: Vec<DocRun>`, List's `items: Vec<DocListItem>`, Table's
/// `rows: Vec<DocTableRow>` -> `cells: Vec<DocTableCell>`, Quote's recursive `Vec<DocBlock>`) — a
/// prior draft of this file duplicated all of these, a real policy violation this wave fixes.
pub(crate) fn enc_table_cell(c: &SlideTableCell) -> String {
    enc_list(&c.blocks, enc_block)
}
pub(crate) fn dec_table_cell(s: &str) -> Result<SlideTableCell, String> {
    Ok(SlideTableCell { blocks: dec_list(s, dec_block)? })
}
pub(crate) fn enc_table_row(r: &SlideTableRow) -> String {
    enc_list(&r.cells, enc_table_cell)
}
pub(crate) fn dec_table_row(s: &str) -> Result<SlideTableRow, String> {
    Ok(SlideTableRow { cells: dec_list(s, dec_table_cell)? })
}
/// 🌳️ `X[frame,blocks]` TextBox / `P[frame,image]` Picture / `T[frame,rows]` Table /
/// `H[frame,kind]` placeHolder.
pub(crate) fn enc_shape(shape: &SlideShape) -> String {
    match shape {
        SlideShape::TextBox { frame, blocks } => format!("X[{},{}]", enc_frame(frame), enc_list(blocks, enc_block)),
        SlideShape::Picture { frame, image } => format!("P[{},{}]", enc_frame(frame), enc_image(image)),
        SlideShape::Table { frame, rows } => format!("T[{},{}]", enc_frame(frame), enc_list(rows, enc_table_row)),
        SlideShape::Placeholder { frame, kind } => format!("H[{},{}]", enc_frame(frame), enc_placeholder_kind(kind)),
    }
}
pub(crate) fn dec_shape(s: &str) -> Result<SlideShape, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    let parts = split_top_level(inner, ',');
    match tag {
        "X" => {
            let [frame, blocks] = parts.as_slice() else { return Err(format!("textbox: expected 2 fields, got {}", parts.len())) };
            Ok(SlideShape::TextBox { frame: dec_frame(frame)?, blocks: dec_list(blocks, dec_block)? })
        }
        "P" => {
            let [frame, image] = parts.as_slice() else { return Err(format!("picture: expected 2 fields, got {}", parts.len())) };
            Ok(SlideShape::Picture { frame: dec_frame(frame)?, image: dec_image(image)? })
        }
        "T" => {
            let [frame, rows] = parts.as_slice() else { return Err(format!("table: expected 2 fields, got {}", parts.len())) };
            Ok(SlideShape::Table { frame: dec_frame(frame)?, rows: dec_list(rows, dec_table_row)? })
        }
        "H" => {
            let [frame, kind] = parts.as_slice() else { return Err(format!("placeholder: expected 2 fields, got {}", parts.len())) };
            Ok(SlideShape::Placeholder { frame: dec_frame(frame)?, kind: dec_placeholder_kind(kind)? })
        }
        other => Err(format!("shape: unknown tag {other:?}")),
    }
}
pub(crate) fn enc_master(m: &SlideMaster) -> String {
    format!("[{},{}]", enc_str(&m.id), enc_list(&m.shapes, enc_shape))
}
pub(crate) fn dec_master(s: &str) -> Result<SlideMaster, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [id, shapes] = parts.as_slice() else { return Err(format!("master: expected 2 fields, got {}", parts.len())) };
    Ok(SlideMaster { id: dec_str(id)?, shapes: dec_list(shapes, dec_shape)? })
}
pub(crate) fn enc_layout(l: &SlideLayout) -> String {
    format!("[{},{},{}]", enc_str(&l.id), enc_str(&l.master_id), enc_list(&l.shapes, enc_shape))
}
pub(crate) fn dec_layout(s: &str) -> Result<SlideLayout, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [id, master_id, shapes] = parts.as_slice() else { return Err(format!("layout: expected 3 fields, got {}", parts.len())) };
    Ok(SlideLayout { id: dec_str(id)?, master_id: dec_str(master_id)?, shapes: dec_list(shapes, dec_shape)? })
}
pub(crate) fn enc_slide(sl: &Slide) -> String {
    format!("[{},{},{},{}]", enc_str(&sl.id), encode_option(&sl.layout_id, |v| enc_str(v)), enc_list(&sl.shapes, enc_shape), enc_list(&sl.notes, enc_block))
}
pub(crate) fn dec_slide(s: &str) -> Result<Slide, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [id, layout_id, shapes, notes] = parts.as_slice() else { return Err(format!("slide: expected 4 fields, got {}", parts.len())) };
    Ok(Slide { id: dec_str(id)?, layout_id: decode_option(layout_id, dec_str)?, shapes: dec_list(shapes, dec_shape)?, notes: dec_list(notes, dec_block)? })
}
/// 🌱 Full (non-diff) snapshot codec — only `SetSnapshot`'s whole-payload op encoding needs this.
pub(crate) fn enc_presentation_snapshot(s: &SemioPresentationSnapshot) -> String {
    format!("[{},{},{},{}]", enc_str(&s.schema), enc_list(&s.masters, enc_master), enc_list(&s.layouts, enc_layout), enc_list(&s.slides, enc_slide))
}
pub(crate) fn dec_presentation_snapshot(s: &str) -> Result<SemioPresentationSnapshot, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [schema, masters, layouts, slides] = parts.as_slice() else { return Err(format!("snapshot: expected 4 fields, got {}", parts.len())) };
    Ok(SemioPresentationSnapshot { schema: dec_str(schema)?, masters: dec_list(masters, dec_master)?, layouts: dec_list(layouts, dec_layout)?, slides: dec_list(slides, dec_slide)? })
}
//#endregion 🔖️ValueCodecs

//#region 🔖️GenericTripleCodecs
/// 🌳️ `[removed];[modified];[added]` — generic over `IndexedTripleDiff<D,T>`'s own `D`/`T`, local
/// copy (see the file's `GenericCollectionTriples` doc comment for why not the shared one).
fn enc_indexed_triple<D, T>(diff: &IndexedTripleDiff<D, T>, enc_d: impl Fn(&D) -> String, enc_t: impl Fn(&T) -> String) -> String {
    let removed = diff.removed.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let modified = diff.modified.iter().map(|m| format!("{}:{}", m.index, enc_d(&m.diff))).collect::<Vec<_>>().join(",");
    let added = diff.added.iter().map(|a| format!("{}:{}", a.index, enc_t(&a.item))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}
fn dec_indexed_triple<D, T>(body: &str, dec_d: impl Fn(&str) -> Result<D, String>, dec_t: impl Fn(&str) -> Result<T, String>) -> Result<IndexedTripleDiff<D, T>, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("indexed triple: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_usize).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("indexed modified: bad entry {entry:?}"))?;
            Ok(IndexModified { index: parse_usize(idx)?, diff: dec_d(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("indexed added: bad entry {entry:?}"))?;
            Ok(IndexAdded { index: parse_usize(idx)?, item: dec_t(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(IndexedTripleDiff { removed, modified, added })
}
/// 🏷️ `[removed];[modified];[added]` — generic over `NamedTripleDiff<K,D,T>`'s own `K`/`D`/`T`.
fn enc_named_triple<K, D, T>(diff: &NamedTripleDiff<K, D, T>, enc_k: impl Fn(&K) -> String, enc_d: impl Fn(&D) -> String, enc_t: impl Fn(&T) -> String) -> String {
    let removed = diff.removed.iter().map(|k| enc_k(k)).collect::<Vec<_>>().join(",");
    let modified = diff.modified.iter().map(|m| format!("{}:{}", enc_k(&m.key), enc_d(&m.diff))).collect::<Vec<_>>().join(",");
    let added = diff.added.iter().map(|t| enc_t(t)).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}
fn dec_named_triple<K, D, T>(s: &str, dec_k: impl Fn(&str) -> Result<K, String>, dec_d: impl Fn(&str) -> Result<D, String>, dec_t: impl Fn(&str) -> Result<T, String>) -> Result<NamedTripleDiff<K, D, T>, String> {
    let three = split_top_level(s, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("named triple: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|e| dec_k(e)).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (k, rest) = entry.split_once(':').ok_or_else(|| format!("named triple modified: bad entry {entry:?}"))?;
            Ok(NamedModified { key: dec_k(k)?, diff: dec_d(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|e| dec_t(e)).collect::<Result<Vec<_>, String>>()?;
    Ok(NamedTripleDiff { removed, modified, added })
}
//#endregion 🔖️GenericTripleCodecs

//#region 🔖️DiffValueCodecs
fn enc_frame_diff(d: &SlideFrameDiff) -> String {
    format!("[{},{},{}]", encode_option(&d.origin, |v| enc_semio_point2(v)), encode_option(&d.width, |v| enc_f64(*v)), encode_option(&d.height, |v| enc_f64(*v)))
}
fn dec_frame_diff(s: &str) -> Result<SlideFrameDiff, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [origin, width, height] = parts.as_slice() else { return Err(format!("frame diff: expected 3 fields, got {}", parts.len())) };
    Ok(SlideFrameDiff { origin: decode_option(origin, dec_semio_point2)?, width: decode_option(width, dec_f64)?, height: decode_option(height, dec_f64)? })
}
fn enc_image_diff(d: &SlidePictureImageDiff) -> String {
    format!("[{},{},{}]", encode_option(&d.asset_id, |v| enc_str(v)), encode_option(&d.mime, |v| enc_str(v)), encode_option(&d.bytes, |v: &Vec<u8>| hex_encode(v)))
}
fn dec_image_diff(s: &str) -> Result<SlidePictureImageDiff, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [asset_id, mime, bytes] = parts.as_slice() else { return Err(format!("image diff: expected 3 fields, got {}", parts.len())) };
    Ok(SlidePictureImageDiff { asset_id: decode_option(asset_id, dec_str)?, mime: decode_option(mime, dec_str)?, bytes: decode_option(bytes, hex_decode)? })
}
fn enc_doc_blocks_diff(d: &DocBlocksDiff) -> String {
    enc_indexed_triple(d, enc_block, enc_block)
}
fn dec_doc_blocks_diff(s: &str) -> Result<DocBlocksDiff, String> {
    dec_indexed_triple(s, dec_block, dec_block)
}
fn enc_table_cell_diff(d: &SlideTableCellDiff) -> String {
    format!("[{}]", encode_option(&d.blocks, enc_doc_blocks_diff))
}
fn dec_table_cell_diff(s: &str) -> Result<SlideTableCellDiff, String> {
    Ok(SlideTableCellDiff { blocks: decode_option(strip_brackets(s)?, dec_doc_blocks_diff)? })
}
fn enc_table_cells_diff(d: &SlideTableCellsDiff) -> String {
    enc_indexed_triple(d, enc_table_cell_diff, enc_table_cell)
}
fn dec_table_cells_diff(s: &str) -> Result<SlideTableCellsDiff, String> {
    dec_indexed_triple(s, dec_table_cell_diff, dec_table_cell)
}
fn enc_table_row_diff(d: &SlideTableRowDiff) -> String {
    format!("[{}]", encode_option(&d.cells, enc_table_cells_diff))
}
fn dec_table_row_diff(s: &str) -> Result<SlideTableRowDiff, String> {
    Ok(SlideTableRowDiff { cells: decode_option(strip_brackets(s)?, dec_table_cells_diff)? })
}
fn enc_table_rows_diff(d: &SlideTableRowsDiff) -> String {
    enc_indexed_triple(d, enc_table_row_diff, enc_table_row)
}
fn dec_table_rows_diff(s: &str) -> Result<SlideTableRowsDiff, String> {
    dec_indexed_triple(s, dec_table_row_diff, dec_table_row)
}
fn enc_shapes_diff(d: &SlideShapesDiff) -> String {
    enc_indexed_triple(d, enc_shape_diff, enc_shape)
}
fn dec_shapes_diff(s: &str) -> Result<SlideShapesDiff, String> {
    dec_indexed_triple(s, dec_shape_diff, dec_shape)
}
/// 🌳️ `X[frame,blocks]`/`P[frame,image]`/`T[frame,rows]`/`H[frame,kind]`/`R[shape]` (wholesale
/// replace, shape KIND changed).
fn enc_shape_diff(d: &SlideShapeDiff) -> String {
    match d {
        SlideShapeDiff::TextBox { frame, blocks } => format!("X[{},{}]", encode_option(frame, enc_frame_diff), encode_option(blocks, enc_doc_blocks_diff)),
        SlideShapeDiff::Picture { frame, image } => format!("P[{},{}]", encode_option(frame, enc_frame_diff), encode_option(image, enc_image_diff)),
        SlideShapeDiff::Table { frame, rows } => format!("T[{},{}]", encode_option(frame, enc_frame_diff), encode_option(rows, enc_table_rows_diff)),
        SlideShapeDiff::Placeholder { frame, kind } => format!("H[{},{}]", encode_option(frame, enc_frame_diff), encode_option(kind, |v| enc_placeholder_kind(v))),
        SlideShapeDiff::Replace { shape } => format!("R[{}]", enc_shape(shape)),
    }
}
fn dec_shape_diff(s: &str) -> Result<SlideShapeDiff, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "X" => {
            let parts = split_top_level(inner, ',');
            let [frame, blocks] = parts.as_slice() else { return Err(format!("textbox diff: expected 2 fields, got {}", parts.len())) };
            Ok(SlideShapeDiff::TextBox { frame: decode_option(frame, dec_frame_diff)?, blocks: decode_option(blocks, dec_doc_blocks_diff)? })
        }
        "P" => {
            let parts = split_top_level(inner, ',');
            let [frame, image] = parts.as_slice() else { return Err(format!("picture diff: expected 2 fields, got {}", parts.len())) };
            Ok(SlideShapeDiff::Picture { frame: decode_option(frame, dec_frame_diff)?, image: decode_option(image, dec_image_diff)? })
        }
        "T" => {
            let parts = split_top_level(inner, ',');
            let [frame, rows] = parts.as_slice() else { return Err(format!("table diff: expected 2 fields, got {}", parts.len())) };
            Ok(SlideShapeDiff::Table { frame: decode_option(frame, dec_frame_diff)?, rows: decode_option(rows, dec_table_rows_diff)? })
        }
        "H" => {
            let parts = split_top_level(inner, ',');
            let [frame, kind] = parts.as_slice() else { return Err(format!("placeholder diff: expected 2 fields, got {}", parts.len())) };
            Ok(SlideShapeDiff::Placeholder { frame: decode_option(frame, dec_frame_diff)?, kind: decode_option(kind, dec_placeholder_kind)? })
        }
        "R" => Ok(SlideShapeDiff::Replace { shape: dec_shape(inner)? }),
        other => Err(format!("shape diff: unknown tag {other:?}")),
    }
}
fn enc_master_diff(d: &SlideMasterDiff) -> String {
    format!("[{}]", encode_option(&d.shapes, enc_shapes_diff))
}
fn dec_master_diff(s: &str) -> Result<SlideMasterDiff, String> {
    Ok(SlideMasterDiff { shapes: decode_option(strip_brackets(s)?, dec_shapes_diff)? })
}
fn enc_layout_diff(d: &SlideLayoutDiff) -> String {
    format!("[{},{}]", encode_option(&d.master_id, |v| enc_str(v)), encode_option(&d.shapes, enc_shapes_diff))
}
fn dec_layout_diff(s: &str) -> Result<SlideLayoutDiff, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [master_id, shapes] = parts.as_slice() else { return Err(format!("layout diff: expected 2 fields, got {}", parts.len())) };
    Ok(SlideLayoutDiff { master_id: decode_option(master_id, dec_str)?, shapes: decode_option(shapes, dec_shapes_diff)? })
}
fn enc_slide_diff(d: &SlideDiff) -> String {
    format!("[{},{},{}]", encode_option(&d.layout_id, |inner: &Option<String>| encode_option(inner, |v| enc_str(v))), encode_option(&d.shapes, enc_shapes_diff), encode_option(&d.notes, enc_doc_blocks_diff))
}
fn dec_slide_diff(s: &str) -> Result<SlideDiff, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [layout_id, shapes, notes] = parts.as_slice() else { return Err(format!("slide diff: expected 3 fields, got {}", parts.len())) };
    Ok(SlideDiff { layout_id: decode_option(layout_id, |s| decode_option(s, dec_str))?, shapes: decode_option(shapes, dec_shapes_diff)?, notes: decode_option(notes, dec_doc_blocks_diff)? })
}
fn enc_masters_diff(d: &SlideMastersDiff) -> String {
    enc_named_triple(d, |k| enc_str(k), enc_master_diff, enc_master)
}
fn dec_masters_diff(s: &str) -> Result<SlideMastersDiff, String> {
    dec_named_triple(s, dec_str, dec_master_diff, dec_master)
}
fn enc_layouts_diff(d: &SlideLayoutsDiff) -> String {
    enc_named_triple(d, |k| enc_str(k), enc_layout_diff, enc_layout)
}
fn dec_layouts_diff(s: &str) -> Result<SlideLayoutsDiff, String> {
    dec_named_triple(s, dec_str, dec_layout_diff, dec_layout)
}
fn enc_slides_diff(d: &SlidesDiff) -> String {
    enc_indexed_triple(d, enc_slide_diff, enc_slide)
}
fn dec_slides_diff(s: &str) -> Result<SlidesDiff, String> {
    dec_indexed_triple(s, dec_slide_diff, dec_slide)
}
//#endregion 🔖️DiffValueCodecs

//#region 🔖️TopLevel
fn print_presentation_diff(d: &SemioPresentationDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = &d.masters {
        tokens.push(format!("masters={}", enc_masters_diff(v)));
    }
    if let Some(v) = &d.layouts {
        tokens.push(format!("layouts={}", enc_layouts_diff(v)));
    }
    if let Some(v) = &d.slides {
        tokens.push(format!("slides={}", enc_slides_diff(v)));
    }
    tokens.join(" ")
}
fn parse_presentation_diff(line: &str) -> Result<SemioPresentationDiff, String> {
    let mut d = SemioPresentationDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("masters=") {
            d.masters = Some(dec_masters_diff(rest)?);
        } else if let Some(rest) = token.strip_prefix("layouts=") {
            d.layouts = Some(dec_layouts_diff(rest)?);
        } else if let Some(rest) = token.strip_prefix("slides=") {
            d.slides = Some(dec_slides_diff(rest)?);
        } else {
            return Err(format!("presentation diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

//#region 🔖️BinaryPrimitives
/// 🧪️ Real LEB128-varint-length-prefixed binary primitives (`store::pack_rt::write_varint_u64` /
/// `store::ByteReader`, same helpers every other semio wave's `DiffCodec` upgrade reuses) backing
/// the real `DiffCodec::encode_diff`/`decode_diff` below.
pub(crate) fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}
pub(crate) fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}
pub(crate) fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}
pub(crate) fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}
//#endregion 🔖️BinaryPrimitives

impl protocol::DiffCodec for SemioPresentationDiff {
    fn print_diff(&self) -> String {
        print_presentation_diff(self)
    }
    fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        parse_presentation_diff(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    /// ⚡️ ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION presentation wave: real
    /// binary diff frame, replacing the old `print_diff().into_bytes()` text-as-binary shortcut.
    /// `format u8` + `presence u8` (bit0=`masters`, bit1=`layouts`, bit2=`slides`) are two REAL
    /// fixed fields; each present collection then follows as its own varint-length-prefixed opaque
    /// blob (the same `enc_masters_diff`/`enc_layouts_diff`/`enc_slides_diff` bracket/hex text
    /// `print_diff` already produces) — one opaque blob per present collection rather than a
    /// per-segment `Cond` because a SECOND `if`-guard on a field that's itself only conditionally
    /// decoded hard-errors `eval_cond` (`protocol-cond-cannot-chain`, per the grammar recipe's own
    /// gap table; every prior semio wave's own diff binary upgrade hit the identical shape).
    fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const DIFF_BINARY_FORMAT: u8 = 1;
        let mut presence = 0u8;
        if self.masters.is_some() {
            presence |= 0b001;
        }
        if self.layouts.is_some() {
            presence |= 0b010;
        }
        if self.slides.is_some() {
            presence |= 0b100;
        }
        let mut out = vec![DIFF_BINARY_FORMAT, presence];
        if let Some(v) = &self.masters {
            write_str_lp(&mut out, &enc_masters_diff(v));
        }
        if let Some(v) = &self.layouts {
            write_str_lp(&mut out, &enc_layouts_diff(v));
        }
        if let Some(v) = &self.slides {
            write_str_lp(&mut out, &enc_slides_diff(v));
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
        let masters = if presence & 0b001 != 0 {
            let text = read_str_lp(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff masters blob", offset: 2, detail: e })?;
            Some(dec_masters_diff(&text).map_err(|e| protocol::ProtocolError::Malformed { what: "diff masters text", offset: 2, detail: e })?)
        } else {
            None
        };
        let layouts = if presence & 0b010 != 0 {
            let text = read_str_lp(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff layouts blob", offset: 2, detail: e })?;
            Some(dec_layouts_diff(&text).map_err(|e| protocol::ProtocolError::Malformed { what: "diff layouts text", offset: 2, detail: e })?)
        } else {
            None
        };
        let slides = if presence & 0b100 != 0 {
            let text = read_str_lp(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff slides blob", offset: 2, detail: e })?;
            Some(dec_slides_diff(&text).map_err(|e| protocol::ProtocolError::Malformed { what: "diff slides text", offset: 2, detail: e })?)
        } else {
            None
        };
        Ok(SemioPresentationDiff { masters, layouts, slides })
    }
}
//#endregion 🔖️TopLevel

//#region 🔖️Demo
/// 🌱 Representative `SemioPresentationDiff` fixtures — promoted to module scope (from the old
/// `mod handcrafted_diff_codec_tests`-local helpers) so `🎹️composer/🦀️component.rs`'s conformance
/// laws AND `🧬️mutations/🦀️component.rs`'s own test fixtures can reuse them, same promotion model
/// every prior semio wave uses.
#[cfg(test)]
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
/// sweep both directions, reusing `snapshot_a`/`snapshot_b`) — single source of truth for
/// `grammar_conformance_law`/`protocol_walk_law` in `🎹️composer/🦀️component.rs`.
#[cfg(test)]
pub(crate) fn demo_diff_cases() -> Vec<SemioPresentationDiff> {
    let a = snapshot_a();
    let b = snapshot_b();
    vec![SemioPresentationDiff::default(), SemioPresentationDiff::between(&a, &b), SemioPresentationDiff::between(&b, &a), SemioPresentationDiff::between(&a, &a)]
}
//#endregion 🔖️Demo

//#region 🧪️Tests
#[cfg(test)]
mod handcrafted_diff_codec_tests {
    use super::*;
    use protocol::DiffCodec;

    /// 🧪️ `SlideShapeDiff::Replace` coverage: a shape-KIND change at the same slide/shape index
    /// (never reachable through any single mutation variant — only through a real structural
    /// `between()` on two full snapshots, e.g. `SetSnapshot`) must fall back to whole-shape
    /// replacement, round-trip through the hand-rolled `DiffCodec`, and apply/inverse correctly.
    #[test]
    fn shape_kind_change_produces_replace_and_round_trips() {
        let mut a = snapshot_a();
        a.slides[0].shapes = vec![SlideShape::TextBox { frame: SlideFrame { origin: SemioPoint2 { x: 0.0, y: 0.0 }, width: 1.0, height: 1.0 }, blocks: vec![DocBlock::paragraph("was text")] }];
        let mut b = a.clone();
        b.slides[0].shapes = vec![SlideShape::Placeholder { frame: SlideFrame { origin: SemioPoint2 { x: 0.0, y: 0.0 }, width: 1.0, height: 1.0 }, kind: PlaceholderKind::Footer }];

        let diff = SemioPresentationDiff::between(&a, &b);
        let shapes_diff = diff.slides.as_ref().unwrap().modified[0].diff.shapes.as_ref().expect("shapes diff present");
        assert_eq!(shapes_diff.modified.len(), 1);
        assert!(matches!(&shapes_diff.modified[0].diff, SlideShapeDiff::Replace { .. }), "expected Replace for a shape-kind change, got {:?}", shapes_diff.modified[0].diff);

        assert_eq!(MutationDiff::apply(&diff, &a).expect("apply must succeed for a well-formed fixture"), b);
        let inv = DiffAlgebra::inverse(&diff, &a);
        assert_eq!(MutationDiff::apply(&inv, &b).expect("apply must succeed for a well-formed fixture"), a);

        let printed = diff.print_diff();
        let parsed = SemioPresentationDiff::parse_diff(&printed).expect("parse_diff");
        assert_eq!(parsed, diff, "Replace round-trip through the hand-rolled grammar failed (printed {printed:?})");
    }

    /// 🧪️ `DiffCodec` round-trip law over the hand-rolled `SemioPresentationDiff` grammar —
    /// exercises masters/layouts (named-keyed removed/modified/added), slides (index-keyed
    /// removed/modified/added incl. nested shape + `DocBlock`-reuse changes), and the `layout_id`
    /// tri-state, in both directions plus the empty/self cases.
    #[test]
    fn diff_codec_text_binary_roundtrip_law() {
        let a = snapshot_a();
        let b = snapshot_b();
        let cases = vec![SemioPresentationDiff::default(), SemioPresentationDiff::between(&a, &b), SemioPresentationDiff::between(&b, &a), SemioPresentationDiff::between(&a, &a)];
        for d in cases {
            let printed = d.print_diff();
            assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
            let parsed = SemioPresentationDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
            assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

            let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
            let decoded = SemioPresentationDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
            assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
        }

        // Field sweep confirmation: every collection flavor + the tri-state actually exercised.
        let diff_ab = SemioPresentationDiff::between(&a, &b);
        let masters = diff_ab.masters.as_ref().expect("masters diff present");
        assert!(!masters.removed.is_empty() && !masters.modified.is_empty() && !masters.added.is_empty(), "masters: not every flavor exercised");
        let layouts = diff_ab.layouts.as_ref().expect("layouts diff present");
        assert!(!layouts.modified.is_empty(), "layouts: master_id modify not exercised");
        let slides = diff_ab.slides.as_ref().expect("slides diff present");
        assert!(!slides.removed.is_empty(), "slides: removed not exercised");
        assert_eq!(slides.modified.len(), 1);
        let slide_diff = &slides.modified[0].diff;
        assert_eq!(slide_diff.layout_id, Some(Some("layout1".to_string())), "layout_id tri-state Some(Some(_)) not exercised");
        let shapes_diff = slide_diff.shapes.as_ref().expect("shapes diff present");
        assert!(!shapes_diff.modified.is_empty() && !shapes_diff.added.is_empty(), "shapes: modified/added not exercised");
        assert!(slide_diff.notes.as_ref().expect("notes diff present").added.len() > 0, "notes: added not exercised");
    }
}
//#endregion 🧪️Tests
//#endregion 🔖️HandcraftedDiffCodec
