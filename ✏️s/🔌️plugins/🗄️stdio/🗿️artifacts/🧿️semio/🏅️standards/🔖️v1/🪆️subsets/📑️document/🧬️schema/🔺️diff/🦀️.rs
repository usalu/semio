//! 🔺️ SemioDocumentDiff — handcrafted sparse diff over `SemioDocumentSnapshot`
//! (`styles`/`images`/`blocks`). No `snapshot: Option<SemioDocumentSnapshot>` full-replace slot —
//! even `SetSnapshot`'s diff is the sparse field-by-field `SemioDocumentDiff::between(base, next)`.
//!
//! `blocks` is a recursive tree (`List`/`Table`/`Quote` all nest `DocBlock` further down — list
//! items, table cells, blockquote bodies), diffed with the same index-keyed recursive-triple
//! pattern xml/svg/md/docx use, generalized here via the STANDARD-LEVEL shared
//! `engine::triples::{IndexedTripleDiff, NamedTripleDiff, NamedAdded}` (per this ticket's brief: reuse the
//! real, tested generic collection-diff engine instead of re-deriving docx's own local copy).
//! `styles`/`images` are name-keyed (by `DocStyle::id`/`DocImage::id`) via the same module's
//! `NamedTripleDiff`. The generic BETWEEN/APPLY/INVERSE/ABSORB engine functions themselves
//! (`between_indexed`/`apply_indexed`/`inverse_indexed`/`absorb_indexed` and their named-triple
//! twins) are not part of `engine::triples` (that module only owns the wire TYPES + text codec) —
//! per-artifact local copies are the established convention (docx keeps its own copy too).
//!
//! 🧪️ Per f6-final-summary.md §4.4/§4.3: `#[derive(dsl::DslDiff)]` would fail here for the exact
//! same two reasons docx hit — `DocBlockDiff` is a genuine data-carrying enum reachable through
//! `IndexedTripleDiff<DocBlockDiff, DocBlock>` (`DslField` has no impl for either the enum or the
//! generic collection-triple wrapping it), and every tri-state field below
//! (`style_id`/`based_on`/`width`/`height`/`size`/`font`/`color`/`link`: all `Option<Option<T>>`)
//! fails with `Option<T>: DslField` is not satisfied. `DiffBinary,DiffCodec,DiffText` is hand-rolled below, following
//! the svg/gif/docx template exactly.

use crate::standards::v1::subsets::base::schema::triples::{IndexAdded, IndexModified, IndexedTripleDiff, NamedModified, NamedTripleDiff};



use crate::standards::v1::subsets::document::schema::snapshot::{DocBlock, DocImage, DocListItem, DocRun, DocStyle, DocTableCell, DocTableRow, RunStyle, SemioDocumentSnapshot};
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;

//#region 🔖️DocumentDiffTypes
pub type StylesDiff = NamedTripleDiff<String, DocStyleDiff, NamedAdded<DocStyle>>;
pub type ImagesDiff = NamedTripleDiff<String, DocImageDiff, NamedAdded<DocImage>>;
pub type BlocksDiff = IndexedTripleDiff<DocBlockDiff, DocBlock>;
pub type RunsDiff = IndexedTripleDiff<DocRunDiff, DocRun>;
pub type ListItemsDiff = IndexedTripleDiff<DocListItemDiff, DocListItem>;
pub type TableRowsDiff = IndexedTripleDiff<DocTableRowDiff, DocTableRow>;
pub type TableCellsDiff = IndexedTripleDiff<DocTableCellDiff, DocTableCell>;

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocStyleDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 🏳️ Tri-state: `None` = unchanged, `Some(None)` = cleared, `Some(Some(id))` = set.
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub based_on: Option<Option<String>>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocImageDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub mime: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub bytes: Option<Vec<u8>>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct RunStyleDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub bold: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub italic: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub underline: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<Option<f64>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub font: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<Option<String>>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocRunDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<RunStyleDiff>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocListItemDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub blocks: Option<BlocksDiff>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocTableRowDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub cells: Option<TableCellsDiff>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocTableCellDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub blocks: Option<BlocksDiff>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocParagraphDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub style_id: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub runs: Option<RunsDiff>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocHeadingDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<u8>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub style_id: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub runs: Option<RunsDiff>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocListDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub ordered: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<ListItemsDiff>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocTableDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub rows: Option<TableRowsDiff>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocCodeDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocQuoteDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub blocks: Option<BlocksDiff>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocImageBlockDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub alt: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<Option<f64>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<Option<f64>>,
}

/// 🌳 Per-block diff, shaped like `DocBlock`. `Replace` covers a block-KIND change (e.g.
/// Paragraph -> Table).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum DocBlockDiff {
    Paragraph(DocParagraphDiff),
    Heading(DocHeadingDiff),
    List(DocListDiff),
    Table(DocTableDiff),
    Code(DocCodeDiff),
    Quote(DocQuoteDiff),
    Image(DocImageBlockDiff),
    Replace { block: DocBlock },
}

/// 🩹 Manual `Default` (no fieldless variant exists to `#[derive(Default)]` from) for the same
/// shared `engine::triples::IndexedTripleDiff<D,T>` bound reason `DocBlock`/`DocStyle` document —
/// `DocBlockDiff` is used as `D` in `BlocksDiff`. Never meaningfully constructed via this path
/// (only satisfies the trait bound); `Replace{block: DocBlock::default()}` is the least-surprising
/// technically-valid value (a no-op replace onto the default `PageBreak`).
impl Default for DocBlockDiff {
    fn default() -> Self {
        DocBlockDiff::Replace { block: DocBlock::default() }
    }
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.document.diff")]
pub struct SemioDocumentDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub styles: Option<StylesDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub images: Option<ImagesDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub blocks: Option<BlocksDiff>,
}
//#endregion 🔖️DocumentDiffTypes

//#region 🔖️GenericIndexedEngine
/// 🧮️ Between (positional, per the recipe's index-keyed matching rule): pairwise-compares
/// `0..min(base,other)` as `modified`, base tail as `removed`, other tail as `added`.
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

/// 🧮️ Maps a base-side index through a diff's OWN removed/added to the position it ends up at
/// once that diff has been applied (svg `SvgDiff`'s `transform_index`, generalized).
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

/// 🧮️ Sequential-coalesce absorb per the recipe's normative algorithm (svg/docx precedent):
/// `absorb_item` recursively absorbs two per-field diffs of the SAME item; `apply_item` patches a
/// `D` onto a `T` (needed when `d2` modifies an item `d1` just added).
#[allow(clippy::too_many_arguments)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_indexed<T, D>(d1: IndexedTripleDiff<D, T>, d2: &IndexedTripleDiff<D, T>, absorb_item: impl Fn(D, D) -> D, apply_item: impl Fn(&T, &D) -> T) -> IndexedTripleDiff<D, T>
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
    for a2 in &d2.added {
        added.push(a2.clone());
    }
    added.sort_by_key(|a| a.index);

    IndexedTripleDiff { removed, modified, added }
}
//#endregion 🔖️GenericIndexedEngine

//#region 🔖️GenericNamedEngine
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_named<K, T, D>(base: &[T], other: &[T], key_of: impl Fn(&T) -> K, diff_item: impl Fn(&T, &T) -> Option<D>) -> Option<NamedTripleDiff<K, D, NamedAdded<T>>>
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
    for (index, o) in other.iter().enumerate() {
        let ok = key_of(o);
        if !base.iter().any(|b| key_of(b) == ok) {
            added.push(NamedAdded { index, item: o.clone() });
        }
    }
    if removed.is_empty() && modified.is_empty() && added.is_empty() {
        return None;
    }
    if !reproduces_order(base, other, &removed, &added, &key_of) {
        return Some(NamedTripleDiff { removed: base.iter().map(&key_of).collect(), modified: Vec::new(), added: other.iter().cloned().enumerate().map(|(index, item)| NamedAdded { index, item }).collect() });
    }
    Some(NamedTripleDiff { removed, modified, added })
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

/// 🧮️ Name-keyed absorb — identity is the KEY (not position), so no index transport is needed.
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
//#endregion 🔖️GenericNamedEngine

//#region 🔖️DocumentDiffLogic
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_run(old: &DocRun, new: &DocRun) -> Option<DocRunDiff> {
    if old == new {
        return None;
    }
    Some(DocRunDiff { text: (old.text != new.text).then(|| new.text.clone()), style: diff_run_style(&old.style, &new.style) })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_run_style(old: &RunStyle, new: &RunStyle) -> Option<RunStyleDiff> {
    if old == new {
        return None;
    }
    Some(RunStyleDiff {
        bold: (old.bold != new.bold).then_some(new.bold),
        italic: (old.italic != new.italic).then_some(new.italic),
        underline: (old.underline != new.underline).then_some(new.underline),
        size: (old.size != new.size).then_some(new.size),
        font: (old.font != new.font).then(|| new.font.clone()),
        color: (old.color != new.color).then(|| new.color.clone()),
        link: (old.link != new.link).then(|| new.link.clone()),
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_list_item(old: &DocListItem, new: &DocListItem) -> Option<DocListItemDiff> {
    let blocks = between_indexed(&old.blocks, &new.blocks, diff_block);
    blocks.map(|blocks| DocListItemDiff { blocks: Some(blocks) })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_cell(old: &DocTableCell, new: &DocTableCell) -> Option<DocTableCellDiff> {
    let blocks = between_indexed(&old.blocks, &new.blocks, diff_block);
    blocks.map(|blocks| DocTableCellDiff { blocks: Some(blocks) })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_row(old: &DocTableRow, new: &DocTableRow) -> Option<DocTableRowDiff> {
    let cells = between_indexed(&old.cells, &new.cells, diff_cell);
    cells.map(|cells| DocTableRowDiff { cells: Some(cells) })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_style(old: &DocStyle, new: &DocStyle) -> Option<DocStyleDiff> {
    if old == new {
        return None;
    }
    Some(DocStyleDiff { name: (old.name != new.name).then(|| new.name.clone()), based_on: (old.based_on != new.based_on).then(|| new.based_on.clone()) })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_image(old: &DocImage, new: &DocImage) -> Option<DocImageDiff> {
    if old == new {
        return None;
    }
    Some(DocImageDiff { mime: (old.mime != new.mime).then(|| new.mime.clone()), bytes: (old.bytes != new.bytes).then(|| new.bytes.clone()) })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn diff_block(old: &DocBlock, new: &DocBlock) -> Option<DocBlockDiff> {
    if old == new {
        return None;
    }
    match (old, new) {
        (DocBlock::Paragraph { style_id: os, runs: or }, DocBlock::Paragraph { style_id: ns, runs: nr }) => {
            let style_id = (os != ns).then(|| ns.clone());
            let runs = between_indexed(or, nr, diff_run);
            if style_id.is_none() && runs.is_none() {
                None
            } else {
                Some(DocBlockDiff::Paragraph(DocParagraphDiff { style_id, runs }))
            }
        }
        (DocBlock::Heading { level: ol, style_id: os, runs: or }, DocBlock::Heading { level: nl, style_id: ns, runs: nr }) => {
            let level = (ol != nl).then_some(*nl);
            let style_id = (os != ns).then(|| ns.clone());
            let runs = between_indexed(or, nr, diff_run);
            if level.is_none() && style_id.is_none() && runs.is_none() {
                None
            } else {
                Some(DocBlockDiff::Heading(DocHeadingDiff { level, style_id, runs }))
            }
        }
        (DocBlock::List { ordered: oo, items: oi }, DocBlock::List { ordered: no, items: ni }) => {
            let ordered = (oo != no).then_some(*no);
            let items = between_indexed(oi, ni, diff_list_item);
            if ordered.is_none() && items.is_none() {
                None
            } else {
                Some(DocBlockDiff::List(DocListDiff { ordered, items }))
            }
        }
        (DocBlock::Table { rows: or }, DocBlock::Table { rows: nr }) => {
            let rows = between_indexed(or, nr, diff_row);
            rows.map(|rows| DocBlockDiff::Table(DocTableDiff { rows: Some(rows) }))
        }
        (DocBlock::Code { language: ol, text: ot }, DocBlock::Code { language: nl, text: nt }) => {
            let language = (ol != nl).then(|| nl.clone());
            let text = (ot != nt).then(|| nt.clone());
            if language.is_none() && text.is_none() {
                None
            } else {
                Some(DocBlockDiff::Code(DocCodeDiff { language, text }))
            }
        }
        (DocBlock::Quote { blocks: ob }, DocBlock::Quote { blocks: nb }) => {
            let blocks = between_indexed(ob, nb, diff_block);
            blocks.map(|blocks| DocBlockDiff::Quote(DocQuoteDiff { blocks: Some(blocks) }))
        }
        (DocBlock::Image { image_id: oid, alt: oa, width: ow, height: oh }, DocBlock::Image { image_id: nid, alt: na, width: nw, height: nh }) => {
            let image_id = (oid != nid).then(|| nid.clone());
            let alt = (oa != na).then(|| na.clone());
            let width = (ow != nw).then_some(*nw);
            let height = (oh != nh).then_some(*nh);
            if image_id.is_none() && alt.is_none() && width.is_none() && height.is_none() {
                None
            } else {
                Some(DocBlockDiff::Image(DocImageBlockDiff { image_id, alt, width, height }))
            }
        }
        (DocBlock::PageBreak, DocBlock::PageBreak) => None,
        _ => Some(DocBlockDiff::Replace { block: new.clone() }),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_run(run: &mut DocRun, diff: &DocRunDiff) {
    if let Some(v) = &diff.text {
        run.text = v.clone();
    }
    if let Some(sd) = &diff.style {
        apply_run_style(&mut run.style, sd);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_run_style(style: &mut RunStyle, diff: &RunStyleDiff) {
    if let Some(v) = diff.bold {
        style.bold = v;
    }
    if let Some(v) = diff.italic {
        style.italic = v;
    }
    if let Some(v) = diff.underline {
        style.underline = v;
    }
    if let Some(v) = &diff.size {
        style.size = *v;
    }
    if let Some(v) = &diff.font {
        style.font = v.clone();
    }
    if let Some(v) = &diff.color {
        style.color = v.clone();
    }
    if let Some(v) = &diff.link {
        style.link = v.clone();
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_list_item(item: &mut DocListItem, diff: &DocListItemDiff) {
    if let Some(bd) = &diff.blocks {
        apply_indexed(&mut item.blocks, bd, apply_block);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_cell(cell: &mut DocTableCell, diff: &DocTableCellDiff) {
    if let Some(bd) = &diff.blocks {
        apply_indexed(&mut cell.blocks, bd, apply_block);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_row(row: &mut DocTableRow, diff: &DocTableRowDiff) {
    if let Some(cd) = &diff.cells {
        apply_indexed(&mut row.cells, cd, apply_cell);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_style(style: &mut DocStyle, diff: &DocStyleDiff) {
    if let Some(v) = &diff.name {
        style.name = v.clone();
    }
    if let Some(v) = &diff.based_on {
        style.based_on = v.clone();
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_image(image: &mut DocImage, diff: &DocImageDiff) {
    if let Some(v) = &diff.mime {
        image.mime = v.clone();
    }
    if let Some(v) = &diff.bytes {
        image.bytes = v.clone();
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_block(block: &mut DocBlock, diff: &DocBlockDiff) {
    match diff {
        DocBlockDiff::Replace { block: new } => *block = new.clone(),
        DocBlockDiff::Paragraph(pd) => {
            if let DocBlock::Paragraph { style_id, runs } = block {
                if let Some(s) = &pd.style_id {
                    *style_id = s.clone();
                }
                if let Some(rd) = &pd.runs {
                    apply_indexed(runs, rd, apply_run);
                }
            }
        }
        DocBlockDiff::Heading(hd) => {
            if let DocBlock::Heading { level, style_id, runs } = block {
                if let Some(l) = hd.level {
                    *level = l;
                }
                if let Some(s) = &hd.style_id {
                    *style_id = s.clone();
                }
                if let Some(rd) = &hd.runs {
                    apply_indexed(runs, rd, apply_run);
                }
            }
        }
        DocBlockDiff::List(ld) => {
            if let DocBlock::List { ordered, items } = block {
                if let Some(o) = ld.ordered {
                    *ordered = o;
                }
                if let Some(id) = &ld.items {
                    apply_indexed(items, id, apply_list_item);
                }
            }
        }
        DocBlockDiff::Table(td) => {
            if let DocBlock::Table { rows } = block {
                if let Some(rd) = &td.rows {
                    apply_indexed(rows, rd, apply_row);
                }
            }
        }
        DocBlockDiff::Code(cd) => {
            if let DocBlock::Code { language, text } = block {
                if let Some(l) = &cd.language {
                    *language = l.clone();
                }
                if let Some(t) = &cd.text {
                    *text = t.clone();
                }
            }
        }
        DocBlockDiff::Quote(qd) => {
            if let DocBlock::Quote { blocks } = block {
                if let Some(bd) = &qd.blocks {
                    apply_indexed(blocks, bd, apply_block);
                }
            }
        }
        DocBlockDiff::Image(id) => {
            if let DocBlock::Image { image_id, alt, width, height } = block {
                if let Some(v) = &id.image_id {
                    *image_id = v.clone();
                }
                if let Some(v) = &id.alt {
                    *alt = v.clone();
                }
                if let Some(v) = &id.width {
                    *width = *v;
                }
                if let Some(v) = &id.height {
                    *height = *v;
                }
            }
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn block_with_diff_applied(block: &DocBlock, diff: &DocBlockDiff) -> DocBlock {
    let mut out = block.clone();
    apply_block(&mut out, diff);
    out
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn run_with_diff_applied(run: &DocRun, diff: &DocRunDiff) -> DocRun {
    let mut out = run.clone();
    apply_run(&mut out, diff);
    out
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn list_item_with_diff_applied(item: &DocListItem, diff: &DocListItemDiff) -> DocListItem {
    let mut out = item.clone();
    apply_list_item(&mut out, diff);
    out
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn row_with_diff_applied(row: &DocTableRow, diff: &DocTableRowDiff) -> DocTableRow {
    let mut out = row.clone();
    apply_row(&mut out, diff);
    out
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn cell_with_diff_applied(cell: &DocTableCell, diff: &DocTableCellDiff) -> DocTableCell {
    let mut out = cell.clone();
    apply_cell(&mut out, diff);
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_run(base: &DocRun, diff: &DocRunDiff) -> DocRunDiff {
    DocRunDiff {
        text: diff.text.as_ref().map(|_| base.text.clone()),
        style: diff.style.as_ref().map(|_| RunStyleDiff {
            bold: Some(base.style.bold),
            italic: Some(base.style.italic),
            underline: Some(base.style.underline),
            size: Some(base.style.size),
            font: Some(base.style.font.clone()),
            color: Some(base.style.color.clone()),
            link: Some(base.style.link.clone()),
        }),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_list_item(base: &DocListItem, diff: &DocListItemDiff) -> DocListItemDiff {
    DocListItemDiff { blocks: diff.blocks.as_ref().map(|bd| inverse_indexed(&base.blocks, bd, inverse_block)) }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_cell(base: &DocTableCell, diff: &DocTableCellDiff) -> DocTableCellDiff {
    DocTableCellDiff { blocks: diff.blocks.as_ref().map(|bd| inverse_indexed(&base.blocks, bd, inverse_block)) }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_row(base: &DocTableRow, diff: &DocTableRowDiff) -> DocTableRowDiff {
    DocTableRowDiff { cells: diff.cells.as_ref().map(|cd| inverse_indexed(&base.cells, cd, inverse_cell)) }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_style(base: &DocStyle, diff: &DocStyleDiff) -> DocStyleDiff {
    DocStyleDiff { name: diff.name.as_ref().map(|_| base.name.clone()), based_on: diff.based_on.as_ref().map(|_| base.based_on.clone()) }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_image(base: &DocImage, diff: &DocImageDiff) -> DocImageDiff {
    DocImageDiff { mime: diff.mime.as_ref().map(|_| base.mime.clone()), bytes: diff.bytes.as_ref().map(|_| base.bytes.clone()) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_block(base: &DocBlock, diff: &DocBlockDiff) -> DocBlockDiff {
    match diff {
        DocBlockDiff::Replace { .. } => DocBlockDiff::Replace { block: base.clone() },
        DocBlockDiff::Paragraph(pd) => {
            let DocBlock::Paragraph { style_id, runs } = base else { return DocBlockDiff::Replace { block: base.clone() } };
            DocBlockDiff::Paragraph(DocParagraphDiff { style_id: pd.style_id.as_ref().map(|_| style_id.clone()), runs: pd.runs.as_ref().map(|rd| inverse_indexed(runs, rd, inverse_run)) })
        }
        DocBlockDiff::Heading(hd) => {
            let DocBlock::Heading { level, style_id, runs } = base else { return DocBlockDiff::Replace { block: base.clone() } };
            DocBlockDiff::Heading(DocHeadingDiff { level: hd.level.map(|_| *level), style_id: hd.style_id.as_ref().map(|_| style_id.clone()), runs: hd.runs.as_ref().map(|rd| inverse_indexed(runs, rd, inverse_run)) })
        }
        DocBlockDiff::List(ld) => {
            let DocBlock::List { ordered, items } = base else { return DocBlockDiff::Replace { block: base.clone() } };
            DocBlockDiff::List(DocListDiff { ordered: ld.ordered.map(|_| *ordered), items: ld.items.as_ref().map(|id| inverse_indexed(items, id, inverse_list_item)) })
        }
        DocBlockDiff::Table(td) => {
            let DocBlock::Table { rows } = base else { return DocBlockDiff::Replace { block: base.clone() } };
            DocBlockDiff::Table(DocTableDiff { rows: td.rows.as_ref().map(|rd| inverse_indexed(rows, rd, inverse_row)) })
        }
        DocBlockDiff::Code(cd) => {
            let DocBlock::Code { language, text } = base else { return DocBlockDiff::Replace { block: base.clone() } };
            DocBlockDiff::Code(DocCodeDiff { language: cd.language.as_ref().map(|_| language.clone()), text: cd.text.as_ref().map(|_| text.clone()) })
        }
        DocBlockDiff::Quote(qd) => {
            let DocBlock::Quote { blocks } = base else { return DocBlockDiff::Replace { block: base.clone() } };
            DocBlockDiff::Quote(DocQuoteDiff { blocks: qd.blocks.as_ref().map(|bd| inverse_indexed(blocks, bd, inverse_block)) })
        }
        DocBlockDiff::Image(id) => {
            let DocBlock::Image { image_id, alt, width, height } = base else { return DocBlockDiff::Replace { block: base.clone() } };
            DocBlockDiff::Image(DocImageBlockDiff { image_id: id.image_id.as_ref().map(|_| image_id.clone()), alt: id.alt.as_ref().map(|_| alt.clone()), width: id.width.as_ref().map(|_| *width), height: id.height.as_ref().map(|_| *height) })
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_run_style_diff(a: RunStyleDiff, b: RunStyleDiff) -> RunStyleDiff {
    RunStyleDiff { bold: b.bold.or(a.bold), italic: b.italic.or(a.italic), underline: b.underline.or(a.underline), size: b.size.or(a.size), font: b.font.or(a.font), color: b.color.or(a.color), link: b.link.or(a.link) }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_run_diff(a: DocRunDiff, b: DocRunDiff) -> DocRunDiff {
    DocRunDiff {
        text: b.text.or(a.text),
        style: match (a.style, b.style) {
            (None, x) => x,
            (x, None) => x,
            (Some(sa), Some(sb)) => Some(absorb_run_style_diff(sa, sb)),
        },
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_list_item_diff(mut a: DocListItemDiff, b: DocListItemDiff) -> DocListItemDiff {
    a.blocks = match (a.blocks.take(), b.blocks) {
        (None, x) => x,
        (x, None) => x,
        (Some(ba), Some(bb)) => Some(absorb_indexed(ba, &bb, absorb_block_diff, block_with_diff_applied)),
    };
    a
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_cell_diff(mut a: DocTableCellDiff, b: DocTableCellDiff) -> DocTableCellDiff {
    a.blocks = match (a.blocks.take(), b.blocks) {
        (None, x) => x,
        (x, None) => x,
        (Some(ba), Some(bb)) => Some(absorb_indexed(ba, &bb, absorb_block_diff, block_with_diff_applied)),
    };
    a
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_row_diff(mut a: DocTableRowDiff, b: DocTableRowDiff) -> DocTableRowDiff {
    a.cells = match (a.cells.take(), b.cells) {
        (None, x) => x,
        (x, None) => x,
        (Some(ca), Some(cb)) => Some(absorb_indexed(ca, &cb, absorb_cell_diff, cell_with_diff_applied)),
    };
    a
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_style_diff(mut a: DocStyleDiff, b: DocStyleDiff) -> DocStyleDiff {
    if b.name.is_some() {
        a.name = b.name;
    }
    if b.based_on.is_some() {
        a.based_on = b.based_on;
    }
    a
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_image_diff(mut a: DocImageDiff, b: DocImageDiff) -> DocImageDiff {
    if b.mime.is_some() {
        a.mime = b.mime;
    }
    if b.bytes.is_some() {
        a.bytes = b.bytes;
    }
    a
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_block_diff(a: DocBlockDiff, b: DocBlockDiff) -> DocBlockDiff {
    match (a, b) {
        (_, DocBlockDiff::Replace { block }) => DocBlockDiff::Replace { block },
        (DocBlockDiff::Replace { block }, b) => DocBlockDiff::Replace { block: block_with_diff_applied(&block, &b) },
        (DocBlockDiff::Paragraph(mut pa), DocBlockDiff::Paragraph(pb)) => {
            if pb.style_id.is_some() {
                pa.style_id = pb.style_id;
            }
            pa.runs = match (pa.runs.take(), pb.runs) {
                (None, x) => x,
                (x, None) => x,
                (Some(ra), Some(rb)) => Some(absorb_indexed(ra, &rb, absorb_run_diff, run_with_diff_applied)),
            };
            DocBlockDiff::Paragraph(pa)
        }
        (DocBlockDiff::Heading(mut ha), DocBlockDiff::Heading(hb)) => {
            if hb.level.is_some() {
                ha.level = hb.level;
            }
            if hb.style_id.is_some() {
                ha.style_id = hb.style_id;
            }
            ha.runs = match (ha.runs.take(), hb.runs) {
                (None, x) => x,
                (x, None) => x,
                (Some(ra), Some(rb)) => Some(absorb_indexed(ra, &rb, absorb_run_diff, run_with_diff_applied)),
            };
            DocBlockDiff::Heading(ha)
        }
        (DocBlockDiff::List(mut la), DocBlockDiff::List(lb)) => {
            if lb.ordered.is_some() {
                la.ordered = lb.ordered;
            }
            la.items = match (la.items.take(), lb.items) {
                (None, x) => x,
                (x, None) => x,
                (Some(ia), Some(ib)) => Some(absorb_indexed(ia, &ib, absorb_list_item_diff, list_item_with_diff_applied)),
            };
            DocBlockDiff::List(la)
        }
        (DocBlockDiff::Table(mut ta), DocBlockDiff::Table(tb)) => {
            ta.rows = match (ta.rows.take(), tb.rows) {
                (None, x) => x,
                (x, None) => x,
                (Some(ra), Some(rb)) => Some(absorb_indexed(ra, &rb, absorb_row_diff, row_with_diff_applied)),
            };
            DocBlockDiff::Table(ta)
        }
        (DocBlockDiff::Code(mut ca), DocBlockDiff::Code(cb)) => {
            if cb.language.is_some() {
                ca.language = cb.language;
            }
            if cb.text.is_some() {
                ca.text = cb.text;
            }
            DocBlockDiff::Code(ca)
        }
        (DocBlockDiff::Quote(mut qa), DocBlockDiff::Quote(qb)) => {
            qa.blocks = match (qa.blocks.take(), qb.blocks) {
                (None, x) => x,
                (x, None) => x,
                (Some(ba), Some(bb)) => Some(absorb_indexed(ba, &bb, absorb_block_diff, block_with_diff_applied)),
            };
            DocBlockDiff::Quote(qa)
        }
        (DocBlockDiff::Image(mut ia), DocBlockDiff::Image(ib)) => {
            if ib.image_id.is_some() {
                ia.image_id = ib.image_id;
            }
            if ib.alt.is_some() {
                ia.alt = ib.alt;
            }
            if ib.width.is_some() {
                ia.width = ib.width;
            }
            if ib.height.is_some() {
                ia.height = ib.height;
            }
            DocBlockDiff::Image(ia)
        }
        // 🩹 Mismatched non-Replace variant pair only arises from a malformed hand-built diff
        // pair (never from real between()/mutation output, which always keys a modify against
        // the item's own live kind) — b wins, matching docx's own fallback.
        (_, b) => b,
    }
}
//#endregion 🔖️DocumentDiffLogic

//#region 🔖️TopLevel
/// 🧭️ Whole-document `between`: `styles`/`images` name-keyed, `blocks` index-keyed recursive.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_document(base: &SemioDocumentSnapshot, other: &SemioDocumentSnapshot) -> SemioDocumentDiff {
    SemioDocumentDiff {
        styles: between_named(&base.styles, &other.styles, |s| s.id.clone(), diff_style),
        images: between_named(&base.images, &other.images, |i| i.id.clone(), diff_image),
        blocks: between_indexed(&base.blocks, &other.blocks, diff_block),
    }
}

impl MutationDiff<SemioDocumentSnapshot> for SemioDocumentDiff {
    fn apply(&self, base: &SemioDocumentSnapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<SemioDocumentSnapshot> {
        let mut out = base.clone();
        if let Some(sd) = &self.styles {
            crate::standards::v1::subsets::base::schema::triples::validate_named_triple(&out.styles, sd, |item| item.id.clone(), |added| added.item.id.clone(), ["styles"])?;
            apply_named(&mut out.styles, sd, |s| s.id.clone(), apply_style);
        }
        if let Some(id) = &self.images {
            crate::standards::v1::subsets::base::schema::triples::validate_named_triple(&out.images, id, |item| item.id.clone(), |added| added.item.id.clone(), ["images"])?;
            apply_named(&mut out.images, id, |i| i.id.clone(), apply_image);
        }
        if let Some(bd) = &self.blocks {
            crate::standards::v1::subsets::base::schema::triples::validate_indexed_triple(bd, out.blocks.len(), ["blocks"])?;
            apply_indexed(&mut out.blocks, bd, apply_block);
        }
        Ok(out)
    }

    fn absorb(&mut self, other: Self) {
        let styles = std::mem::take(&mut self.styles);
        self.styles = match (styles, other.styles) {
            (None, x) => x,
            (x, None) => x,
            (Some(sa), Some(sb)) => Some(absorb_named(sa, &sb, |s| s.id.clone(), absorb_style_diff, apply_style)),
        };
        let images = std::mem::take(&mut self.images);
        self.images = match (images, other.images) {
            (None, x) => x,
            (x, None) => x,
            (Some(ia), Some(ib)) => Some(absorb_named(ia, &ib, |i| i.id.clone(), absorb_image_diff, apply_image)),
        };
        let blocks = std::mem::take(&mut self.blocks);
        self.blocks = match (blocks, other.blocks) {
            (None, x) => x,
            (x, None) => x,
            (Some(ba), Some(bb)) => Some(absorb_indexed(ba, &bb, absorb_block_diff, block_with_diff_applied)),
        };
    }
}

impl DiffAlgebra<SemioDocumentSnapshot> for SemioDocumentDiff {
    fn between(base: &SemioDocumentSnapshot, other: &SemioDocumentSnapshot) -> Self {
        diff_document(base, other)
    }
    fn inverse(&self, base: &SemioDocumentSnapshot) -> Self {
        SemioDocumentDiff {
            styles: self.styles.as_ref().map(|sd| inverse_named(&base.styles, sd, |s| s.id.clone(), inverse_style)),
            images: self.images.as_ref().map(|id| inverse_named(&base.images, id, |i| i.id.clone(), inverse_image)),
            blocks: self.blocks.as_ref().map(|bd| inverse_indexed(&base.blocks, bd, inverse_block)),
        }
    }
    fn is_empty(&self) -> bool {
        self.styles.is_none() && self.images.is_none() && self.blocks.is_none()
    }
}


//#endregion 🔖️TopLevel

//#region 🔖️HandcraftedDiffCodec
//#region 🔖️Primitives














//#endregion 🔖️Primitives

//#region 🔖️BinaryPrimitives




//#endregion 🔖️BinaryPrimitives

//#region 🔖️ValueCodecs


















//#endregion 🔖️ValueCodecs

//#region 🔖️DiffValueCodecs





















































//#endregion 🔖️HandcraftedDiffCodec

//#region 🔖️Demo
/// 🌱 Representative `SemioDocumentDiff` cases (empty/no-op, a full styles+images+blocks sweep both
/// directions, reusing `tests::snapshot_a`/`tests::snapshot_b`) — single source of truth for
/// `grammar_conformance_law`/`protocol_walk_law` in `🎹️composer/🦀️.rs`.
#[cfg(all(test, feature = "conversion-document"))]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<SemioDocumentDiff> {
    let a = snapshot_a();
    let b = snapshot_b();
    vec![
        SemioDocumentDiff::default(),
        <SemioDocumentDiff as DiffAlgebra<SemioDocumentSnapshot>>::between(&a, &b),
        <SemioDocumentDiff as DiffAlgebra<SemioDocumentSnapshot>>::between(&b, &a),
        <SemioDocumentDiff as DiffAlgebra<SemioDocumentSnapshot>>::between(&a, &a),
    ]
}

/// 🌱 Base fixture for `demo_diff_cases`/`diff_codec_text_binary_roundtrip_law` — module-scope
/// (not `mod tests`-local) so both this facet's own tests and `demo_diff_cases` share one source of
/// truth (model/flow's own `sweep_a`/`sweep_b` promotion precedent).
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn snapshot_a() -> SemioDocumentSnapshot {
    SemioDocumentSnapshot {
        schema: "s.stdio.semio.document".into(),
        styles: vec![DocStyle { id: "keep".into(), name: "Keep".into(), based_on: Some("toRemove".into()) }, DocStyle { id: "toRemove".into(), name: "Gone".into(), based_on: None }],
        images: vec![DocImage { id: "toRemove".into(), mime: "image/png".into(), bytes: vec![9, 9] }],
        blocks: vec![
            DocBlock::Paragraph { style_id: None, runs: vec![DocRun { text: "old".into(), style: RunStyle { bold: false, ..Default::default() } }] },
            DocBlock::Table { rows: vec![DocTableRow { cells: vec![DocTableCell { blocks: vec![DocBlock::paragraph("cell")] }] }] },
        ],
    }
}

#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn snapshot_b() -> SemioDocumentSnapshot {
    SemioDocumentSnapshot {
        schema: "s.stdio.semio.document".into(),
        styles: vec![DocStyle { id: "keep".into(), name: "Keep2".into(), based_on: None }, DocStyle { id: "added".into(), name: "Added".into(), based_on: None }],
        images: vec![DocImage { id: "added".into(), mime: "image/jpeg".into(), bytes: vec![1] }],
        blocks: vec![DocBlock::Paragraph { style_id: Some("keep".into()), runs: vec![DocRun { text: "new".into(), style: RunStyle { bold: true, italic: true, ..Default::default() } }, DocRun::plain("second")] }],
    }
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
