//! 🔺️ DocxDiff — handcrafted sparse diff over `DocxSnapshot` (`opc: OpcPackage` +
//! `document: DocxDocument`). No `snapshot: Option<DocxSnapshot>` full-replace slot — even
//! `SetSnapshot`'s diff is the sparse field-by-field `DocxDiff::between(base, next)`.
//!
//! `document.body` is a recursive tree (`DocxBlock::Table` nests `rows -> cells -> blocks`, same
//! shape as WordprocessingML itself), diffed with the same index-keyed recursive-triple pattern
//! xml/svg/md use — generalized here via `IndexedTripleDiff<D, T>` (shared engine, per-collection
//! `pub type` aliases keep the facet mirrors and the recipe's per-collection naming). `styles` and
//! the OPC layer's `parts`/`content_types` entries/`relationships`-by-owner are name-keyed, via the
//! analogous `NamedTripleDiff<K, D, T>`.
//!
//! **OPC diff placement**: `zip::opc::OpcPackage` (reused directly, not reimplemented — see that
//! module) has no diff type of its own yet. Per this ticket's OPC-pattern-setter brief, one is
//! defined HERE (this wave's docx agent owns only files already mounted for docx; `zip/📦️opc` is
//! outside that boundary) — see `glue_followup` in this wave's report for hoisting it to
//! `zip::opc` so xlsx/pptx/bcf can reuse it verbatim instead of re-deriving their own copy.

use crate::artifacts::docx::schema::snapshot::{DocxBlock, DocxDocument, DocxParagraph, DocxRun, DocxStyle, DocxTable, DocxTableCell, DocxTableRow};
use crate::artifacts::docx::DocxSnapshot;
use crate::artifacts::zip::opc::{OpcContentTypes, OpcPackage, OpcPart, OpcRelationship, OpcTargetMode};
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

//#region 🔖️GenericCollectionTriples
/// 🌳 Index-keyed collection triple, generic over the item type `T` and its per-field diff type
/// `D`. `removed`/`modified` indices refer to BASE state (descending removal order on apply);
/// `added` indices refer to FINAL state (ascending insert, `min(index, len)`).
// 🩹 `bound(...)` overrides serde's default per-field-`default` bound inference, which
// (a known serde_derive limitation) conservatively requires `D: Default`/`T: Default` for a
// `#[serde(default)]` field even though `Vec<_>: Default` never actually needs its item type to
// be `Default` -- the real requirement is only `Deserialize`/`Serialize` on the item types.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", bound(serialize = "D: Serialize, T: Serialize", deserialize = "D: Deserialize<'de>, T: Deserialize<'de>"))]
pub struct IndexedTripleDiff<D, T> {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<IndexModified<D>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<IndexAdded<T>>,
}

impl<D, T> Default for IndexedTripleDiff<D, T> {
    fn default() -> Self { Self { removed: Vec::new(), modified: Vec::new(), added: Vec::new() } }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexModified<D> {
    pub index: usize,
    pub diff: D,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexAdded<T> {
    pub index: usize,
    pub item: T,
}

/// 🏷️ Name/key-keyed collection triple, generic over key `K`, item `T`, and per-field diff `D`.
/// `added` carries the full item (which already contains its own key).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", bound(serialize = "K: Serialize, D: Serialize, T: Serialize", deserialize = "K: Deserialize<'de>, D: Deserialize<'de>, T: Deserialize<'de>"))]
pub struct NamedTripleDiff<K, D, T> {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<K>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<NamedModified<K, D>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<T>,
}

impl<K, D, T> Default for NamedTripleDiff<K, D, T> {
    fn default() -> Self { Self { removed: Vec::new(), modified: Vec::new(), added: Vec::new() } }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedModified<K, D> {
    pub key: K,
    pub diff: D,
}
//#endregion 🔖️GenericCollectionTriples

//#region 🔖️DocumentDiffTypes
pub type DocxBlocksDiff = IndexedTripleDiff<DocxBlockDiff, DocxBlock>;
pub type DocxRunsDiff = IndexedTripleDiff<DocxRunDiff, DocxRun>;
pub type DocxTableRowsDiff = IndexedTripleDiff<DocxTableRowDiff, DocxTableRow>;
pub type DocxTableCellsDiff = IndexedTripleDiff<DocxTableCellDiff, DocxTableCell>;
pub type DocxStylesDiff = NamedTripleDiff<String, DocxStyleDiff, DocxStyle>;

/// 🌳 Per-block diff, shaped like `DocxBlock` (`Paragraph` <-> `Table`; `Replace` covers a
/// paragraph<->table kind change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum DocxBlockDiff {
    Paragraph(DocxParagraphDiff),
    Table(DocxTableDiff),
    Replace { block: DocxBlock },
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocxParagraphDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runs: Option<DocxRunsDiff>,
    /// 🏳️ Tri-state: `None` = unchanged, `Some(None)` = style cleared, `Some(Some(id))` = set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<Option<String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocxRunDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bold: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub italic: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub underline: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocxTableDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rows: Option<DocxTableRowsDiff>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocxTableRowDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cells: Option<DocxTableCellsDiff>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocxTableCellDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocks: Option<DocxBlocksDiff>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocxStyleDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 🏳️ Tri-state: `None` = unchanged, `Some(None)` = based_on cleared, `Some(Some(id))` = set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub based_on: Option<Option<String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocxDocumentDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<DocxBlocksDiff>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub styles: Option<DocxStylesDiff>,
}
//#endregion 🔖️DocumentDiffTypes

//#region 🔖️OpcDiffTypes
pub type DocxOpcCtEntriesDiff = NamedTripleDiff<String, String, (String, String)>;
pub type DocxOpcPartsDiff = NamedTripleDiff<String, DocxOpcPartDiff, OpcPart>;
pub type DocxOpcRelListDiff = NamedTripleDiff<String, DocxOpcRelDiff, OpcRelationship>;
pub type DocxOpcRelationshipsDiff = NamedTripleDiff<String, DocxOpcRelListDiff, (String, Vec<OpcRelationship>)>;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocxOpcContentTypesDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub defaults: Option<DocxOpcCtEntriesDiff>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overrides: Option<DocxOpcCtEntriesDiff>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocxOpcPartDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bytes: Option<Vec<u8>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocxOpcRelDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rel_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_mode: Option<OpcTargetMode>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocxOpcDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_types: Option<DocxOpcContentTypesDiff>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parts: Option<DocxOpcPartsDiff>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relationships: Option<DocxOpcRelationshipsDiff>,
}
//#endregion 🔖️OpcDiffTypes

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.docx`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.docx.diff")]
pub struct DocxDiff {
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub opc: Option<DocxOpcDiff>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document: Option<DocxDocumentDiff>,
}
//#endregion 🔖️Diff

//#region 🔖️PathAddressing
/// 🧭️ One step down into a nested table cell's block list: `body[block_index]` must be a `Table`;
/// descend to `rows[row].cells[cell].blocks`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocxPathSegment {
    pub block_index: usize,
    pub row: usize,
    pub cell: usize,
}

/// 🧭️ Addresses one block-list slot: `segments` navigate through nested `Table`s (mirrors svg's
/// `NodePath` chain-of-indices precedent, adapted for docx's Paragraph/Table mixed tree),
/// `index` is the slot within the innermost list.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocxBlockPath {
    #[serde(default)]
    pub segments: Vec<DocxPathSegment>,
    pub index: usize,
}

enum DocxBlockLeaf {
    Modified(DocxBlockDiff),
    Inserted(DocxBlock),
    Removed,
}

impl DocxBlockLeaf {
    fn into_blocks_diff(self, index: usize) -> DocxBlocksDiff {
        match self {
            Self::Modified(diff) => DocxBlocksDiff { modified: vec![IndexModified { index, diff }], ..Default::default() },
            Self::Inserted(block) => DocxBlocksDiff { added: vec![IndexAdded { index, item: block }], ..Default::default() },
            Self::Removed => DocxBlocksDiff { removed: vec![index], ..Default::default() },
        }
    }
}

/// 🧭️ Lowers a `leaf` diff targeting the block addressed by `path` into a full `DocxDiff` by
/// nesting it through `Table -> rows -> cells -> blocks` from the document root down to that
/// depth. `path.segments == []` addresses `document.body` directly.
fn wrap_body_diff(path: &DocxBlockPath, leaf: DocxBlockLeaf) -> DocxDiff {
    fn go(segments: &[DocxPathSegment], index: usize, leaf: DocxBlockLeaf) -> DocxBlocksDiff {
        match segments.split_first() {
            None => leaf.into_blocks_diff(index),
            Some((seg, rest)) => {
                let inner = go(rest, index, leaf);
                let cell_diff = DocxTableCellDiff { blocks: Some(inner) };
                let cells_diff = DocxTableCellsDiff { modified: vec![IndexModified { index: seg.cell, diff: cell_diff }], ..Default::default() };
                let row_diff = DocxTableRowDiff { cells: Some(cells_diff) };
                let rows_diff = DocxTableRowsDiff { modified: vec![IndexModified { index: seg.row, diff: row_diff }], ..Default::default() };
                let table_diff = DocxBlockDiff::Table(DocxTableDiff { rows: Some(rows_diff) });
                DocxBlocksDiff { modified: vec![IndexModified { index: seg.block_index, diff: table_diff }], ..Default::default() }
            }
        }
    }
    let body = go(&path.segments, path.index, leaf);
    DocxDiff { opc: None, document: Some(DocxDocumentDiff { body: Some(body), styles: None }) }
}

/// 🧭️ Resolves the block list a path's segments navigate to (the parent list `path.index` slots
/// into), immutable form. `pub` so the mutations module can look up prior state for its own
/// handcrafted `diff()`/`inverse()` bodies without duplicating this traversal.
pub fn resolve_blocks<'a>(body: &'a [DocxBlock], segments: &[DocxPathSegment]) -> Option<&'a [DocxBlock]> {
    match segments.split_first() {
        None => Some(body),
        Some((seg, rest)) => {
            let DocxBlock::Table(table) = body.get(seg.block_index)? else { return None };
            let row = table.rows.get(seg.row)?;
            let cell = row.cells.get(seg.cell)?;
            resolve_blocks(&cell.blocks, rest)
        }
    }
}
//#endregion 🔖️PathAddressing

//#region 🔖️GenericIndexedEngine
/// 🧮️ Between (positional, per the recipe's index-keyed matching rule): pairwise-compares
/// `0..min(base,other)` as `modified`, base tail as `removed`, other tail as `added`.
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
    if modified.is_empty() && removed.is_empty() && added.is_empty() { None } else { Some(IndexedTripleDiff { removed, modified, added }) }
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

/// 🧮️ Maps a base-side index through a diff's OWN removed/added to the position it ends up at
/// once that diff has been applied (svg `SvgDiff`'s `transform_index`, generalized).
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

/// 🧮️ Sequential-coalesce absorb per the recipe's normative algorithm (svg `absorb_children_diff`,
/// generalized): `absorb_item` recursively absorbs two per-field diffs of the SAME item;
/// `apply_item` patches a `D` onto a `T` (needed when `d2` modifies an item `d1` just added).
#[allow(clippy::too_many_arguments)]
fn absorb_indexed<T, D>(
    d1: IndexedTripleDiff<D, T>,
    d2: IndexedTripleDiff<D, T>,
    absorb_item: impl Fn(D, D) -> D,
    apply_item: impl Fn(&T, &D) -> T,
) -> IndexedTripleDiff<D, T>
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
    if removed.is_empty() && modified.is_empty() && added.is_empty() { None } else { Some(NamedTripleDiff { removed, modified, added }) }
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

/// 🧮️ Name-keyed absorb — identity is the KEY (not position), so no index transport is needed:
/// a `d2`-removal of a `d1`-added key annihilates the add; a `d2`-modify of a `d1`-added key
/// patches into the carried payload; everything else composes directly on the shared key space.
fn absorb_named<K, T, D>(
    d1: NamedTripleDiff<K, D, T>,
    d2: NamedTripleDiff<K, D, T>,
    key_of: impl Fn(&T) -> K,
    absorb_item: impl Fn(D, D) -> D,
    apply_item: impl Fn(&mut T, &D),
) -> NamedTripleDiff<K, D, T>
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
//#endregion 🔖️GenericNamedEngine

//#region 🔖️DocumentDiffLogic
fn diff_block(old: &DocxBlock, new: &DocxBlock) -> Option<DocxBlockDiff> {
    if old == new {
        return None;
    }
    match (old, new) {
        (DocxBlock::Paragraph(op), DocxBlock::Paragraph(np)) => diff_paragraph(op, np).map(DocxBlockDiff::Paragraph),
        (DocxBlock::Table(ot), DocxBlock::Table(nt)) => diff_table(ot, nt).map(DocxBlockDiff::Table),
        _ => Some(DocxBlockDiff::Replace { block: new.clone() }),
    }
}

fn diff_paragraph(old: &DocxParagraph, new: &DocxParagraph) -> Option<DocxParagraphDiff> {
    let runs = between_indexed(&old.runs, &new.runs, diff_run);
    let style = if old.style != new.style { Some(new.style.clone()) } else { None };
    if runs.is_none() && style.is_none() { None } else { Some(DocxParagraphDiff { runs, style }) }
}

fn diff_run(old: &DocxRun, new: &DocxRun) -> Option<DocxRunDiff> {
    if old == new {
        return None;
    }
    Some(DocxRunDiff {
        text: (old.text != new.text).then(|| new.text.clone()),
        bold: (old.bold != new.bold).then_some(new.bold),
        italic: (old.italic != new.italic).then_some(new.italic),
        underline: (old.underline != new.underline).then_some(new.underline),
    })
}

fn diff_table(old: &DocxTable, new: &DocxTable) -> Option<DocxTableDiff> {
    let rows = between_indexed(&old.rows, &new.rows, diff_row);
    if rows.is_none() { None } else { Some(DocxTableDiff { rows }) }
}

fn diff_row(old: &DocxTableRow, new: &DocxTableRow) -> Option<DocxTableRowDiff> {
    let cells = between_indexed(&old.cells, &new.cells, diff_cell);
    if cells.is_none() { None } else { Some(DocxTableRowDiff { cells }) }
}

fn diff_cell(old: &DocxTableCell, new: &DocxTableCell) -> Option<DocxTableCellDiff> {
    let blocks = between_indexed(&old.blocks, &new.blocks, diff_block);
    if blocks.is_none() { None } else { Some(DocxTableCellDiff { blocks }) }
}

fn diff_style(old: &DocxStyle, new: &DocxStyle) -> Option<DocxStyleDiff> {
    if old == new {
        return None;
    }
    Some(DocxStyleDiff {
        name: (old.name != new.name).then(|| new.name.clone()),
        based_on: (old.based_on != new.based_on).then(|| new.based_on.clone()),
    })
}

fn diff_document(base: &DocxDocument, other: &DocxDocument) -> Option<DocxDocumentDiff> {
    let body = between_indexed(&base.body, &other.body, diff_block);
    let styles = between_named(&base.styles, &other.styles, |s| s.id.clone(), diff_style);
    if body.is_none() && styles.is_none() { None } else { Some(DocxDocumentDiff { body, styles }) }
}

fn apply_block(block: &mut DocxBlock, diff: &DocxBlockDiff) {
    match diff {
        DocxBlockDiff::Replace { block: new } => *block = new.clone(),
        DocxBlockDiff::Paragraph(pd) => {
            if let DocxBlock::Paragraph(p) = block {
                if let Some(rd) = &pd.runs {
                    apply_indexed(&mut p.runs, rd, apply_run);
                }
                if let Some(s) = &pd.style {
                    p.style = s.clone();
                }
            }
        }
        DocxBlockDiff::Table(td) => {
            if let DocxBlock::Table(t) = block {
                if let Some(rd) = &td.rows {
                    apply_indexed(&mut t.rows, rd, apply_row);
                }
            }
        }
    }
}

fn apply_run(run: &mut DocxRun, diff: &DocxRunDiff) {
    if let Some(v) = &diff.text {
        run.text = v.clone();
    }
    if let Some(v) = diff.bold {
        run.bold = v;
    }
    if let Some(v) = diff.italic {
        run.italic = v;
    }
    if let Some(v) = diff.underline {
        run.underline = v;
    }
}

fn apply_row(row: &mut DocxTableRow, diff: &DocxTableRowDiff) {
    if let Some(cd) = &diff.cells {
        apply_indexed(&mut row.cells, cd, apply_cell);
    }
}

fn apply_cell(cell: &mut DocxTableCell, diff: &DocxTableCellDiff) {
    if let Some(bd) = &diff.blocks {
        apply_indexed(&mut cell.blocks, bd, apply_block);
    }
}

fn apply_style(style: &mut DocxStyle, diff: &DocxStyleDiff) {
    if let Some(v) = &diff.name {
        style.name = v.clone();
    }
    if let Some(v) = &diff.based_on {
        style.based_on = v.clone();
    }
}

fn apply_document_diff(doc: &mut DocxDocument, diff: &DocxDocumentDiff) {
    if let Some(bd) = &diff.body {
        apply_indexed(&mut doc.body, bd, apply_block);
    }
    if let Some(sd) = &diff.styles {
        apply_named(&mut doc.styles, sd, |s| s.id.clone(), apply_style);
    }
}

fn block_with_diff_applied(block: &DocxBlock, diff: &DocxBlockDiff) -> DocxBlock {
    let mut out = block.clone();
    apply_block(&mut out, diff);
    out
}

fn run_with_diff_applied(run: &DocxRun, diff: &DocxRunDiff) -> DocxRun {
    let mut out = run.clone();
    apply_run(&mut out, diff);
    out
}

fn row_with_diff_applied(row: &DocxTableRow, diff: &DocxTableRowDiff) -> DocxTableRow {
    let mut out = row.clone();
    apply_row(&mut out, diff);
    out
}

fn cell_with_diff_applied(cell: &DocxTableCell, diff: &DocxTableCellDiff) -> DocxTableCell {
    let mut out = cell.clone();
    apply_cell(&mut out, diff);
    out
}

fn inverse_block(base: &DocxBlock, diff: &DocxBlockDiff) -> DocxBlockDiff {
    match diff {
        DocxBlockDiff::Replace { .. } => DocxBlockDiff::Replace { block: base.clone() },
        DocxBlockDiff::Paragraph(pd) => {
            let DocxBlock::Paragraph(p) = base else { return DocxBlockDiff::Replace { block: base.clone() } };
            DocxBlockDiff::Paragraph(DocxParagraphDiff {
                runs: pd.runs.as_ref().map(|rd| inverse_indexed(&p.runs, rd, inverse_run)),
                style: pd.style.as_ref().map(|_| p.style.clone()),
            })
        }
        DocxBlockDiff::Table(td) => {
            let DocxBlock::Table(t) = base else { return DocxBlockDiff::Replace { block: base.clone() } };
            DocxBlockDiff::Table(DocxTableDiff { rows: td.rows.as_ref().map(|rd| inverse_indexed(&t.rows, rd, inverse_row)) })
        }
    }
}

fn inverse_run(base: &DocxRun, diff: &DocxRunDiff) -> DocxRunDiff {
    DocxRunDiff {
        text: diff.text.as_ref().map(|_| base.text.clone()),
        bold: diff.bold.map(|_| base.bold),
        italic: diff.italic.map(|_| base.italic),
        underline: diff.underline.map(|_| base.underline),
    }
}

fn inverse_row(base: &DocxTableRow, diff: &DocxTableRowDiff) -> DocxTableRowDiff {
    DocxTableRowDiff { cells: diff.cells.as_ref().map(|cd| inverse_indexed(&base.cells, cd, inverse_cell)) }
}

fn inverse_cell(base: &DocxTableCell, diff: &DocxTableCellDiff) -> DocxTableCellDiff {
    DocxTableCellDiff { blocks: diff.blocks.as_ref().map(|bd| inverse_indexed(&base.blocks, bd, inverse_block)) }
}

fn inverse_style(base: &DocxStyle, diff: &DocxStyleDiff) -> DocxStyleDiff {
    DocxStyleDiff {
        name: diff.name.as_ref().map(|_| base.name.clone()),
        based_on: diff.based_on.as_ref().map(|_| base.based_on.clone()),
    }
}

fn inverse_document_diff(base: &DocxDocument, diff: &DocxDocumentDiff) -> DocxDocumentDiff {
    DocxDocumentDiff {
        body: diff.body.as_ref().map(|bd| inverse_indexed(&base.body, bd, inverse_block)),
        styles: diff.styles.as_ref().map(|sd| inverse_named(&base.styles, sd, |s| s.id.clone(), inverse_style)),
    }
}

fn absorb_block_diff(a: DocxBlockDiff, b: DocxBlockDiff) -> DocxBlockDiff {
    match (a, b) {
        (_, DocxBlockDiff::Replace { block }) => DocxBlockDiff::Replace { block },
        (DocxBlockDiff::Replace { block }, b) => DocxBlockDiff::Replace { block: block_with_diff_applied(&block, &b) },
        (DocxBlockDiff::Paragraph(pa), DocxBlockDiff::Paragraph(pb)) => DocxBlockDiff::Paragraph(absorb_paragraph_diff(pa, pb)),
        (DocxBlockDiff::Table(ta), DocxBlockDiff::Table(tb)) => DocxBlockDiff::Table(absorb_table_diff(ta, tb)),
        (_, b) => b,
    }
}

fn absorb_paragraph_diff(mut a: DocxParagraphDiff, b: DocxParagraphDiff) -> DocxParagraphDiff {
    if b.style.is_some() {
        a.style = b.style;
    }
    a.runs = match (a.runs.take(), b.runs) {
        (None, x) => x,
        (x, None) => x,
        (Some(ra), Some(rb)) => Some(absorb_indexed(ra, rb, absorb_run_diff, run_with_diff_applied)),
    };
    a
}

fn absorb_run_diff(a: DocxRunDiff, b: DocxRunDiff) -> DocxRunDiff {
    DocxRunDiff {
        text: b.text.or(a.text),
        bold: b.bold.or(a.bold),
        italic: b.italic.or(a.italic),
        underline: b.underline.or(a.underline),
    }
}

fn absorb_table_diff(mut a: DocxTableDiff, b: DocxTableDiff) -> DocxTableDiff {
    a.rows = match (a.rows.take(), b.rows) {
        (None, x) => x,
        (x, None) => x,
        (Some(ra), Some(rb)) => Some(absorb_indexed(ra, rb, absorb_row_diff, row_with_diff_applied)),
    };
    a
}

fn absorb_row_diff(mut a: DocxTableRowDiff, b: DocxTableRowDiff) -> DocxTableRowDiff {
    a.cells = match (a.cells.take(), b.cells) {
        (None, x) => x,
        (x, None) => x,
        (Some(ca), Some(cb)) => Some(absorb_indexed(ca, cb, absorb_cell_diff, cell_with_diff_applied)),
    };
    a
}

fn absorb_cell_diff(mut a: DocxTableCellDiff, b: DocxTableCellDiff) -> DocxTableCellDiff {
    a.blocks = match (a.blocks.take(), b.blocks) {
        (None, x) => x,
        (x, None) => x,
        (Some(ba), Some(bb)) => Some(absorb_indexed(ba, bb, absorb_block_diff, block_with_diff_applied)),
    };
    a
}

fn absorb_style_diff(mut a: DocxStyleDiff, b: DocxStyleDiff) -> DocxStyleDiff {
    if b.name.is_some() {
        a.name = b.name;
    }
    if b.based_on.is_some() {
        a.based_on = b.based_on;
    }
    a
}

fn absorb_document_diff(a: DocxDocumentDiff, b: DocxDocumentDiff) -> DocxDocumentDiff {
    DocxDocumentDiff {
        body: match (a.body, b.body) {
            (None, x) => x,
            (x, None) => x,
            (Some(ba), Some(bb)) => Some(absorb_indexed(ba, bb, absorb_block_diff, block_with_diff_applied)),
        },
        styles: match (a.styles, b.styles) {
            (None, x) => x,
            (x, None) => x,
            (Some(sa), Some(sb)) => Some(absorb_named(sa, sb, |s| s.id.clone(), absorb_style_diff, apply_style)),
        },
    }
}
//#endregion 🔖️DocumentDiffLogic

//#region 🔖️OpcDiffLogic
fn diff_ct_entries(old: &[(String, String)], new: &[(String, String)]) -> Option<DocxOpcCtEntriesDiff> {
    between_named(old, new, |(k, _)| k.clone(), |(_, ov), (_, nv)| (ov != nv).then(|| nv.clone()))
}

fn apply_ct_entries(entries: &mut Vec<(String, String)>, diff: &DocxOpcCtEntriesDiff) {
    apply_named(entries, diff, |(k, _)| k.clone(), |(_, v), nv| *v = nv.clone());
}

fn inverse_ct_entries(base: &[(String, String)], diff: &DocxOpcCtEntriesDiff) -> DocxOpcCtEntriesDiff {
    inverse_named(base, diff, |(k, _)| k.clone(), |(_, v), _| v.clone())
}

fn absorb_ct_entries(a: DocxOpcCtEntriesDiff, b: DocxOpcCtEntriesDiff) -> DocxOpcCtEntriesDiff {
    // 🏷️ `D = String` here is already a whole-value replace (LWW) -- absorbing two such diffs on
    // the SAME key is just "the later one wins", i.e. `b`.
    absorb_named(a, b, |(k, _)| k.clone(), |_av, bv| bv, |(_, v), nv| *v = nv.clone())
}

fn diff_content_types(old: &OpcContentTypes, new: &OpcContentTypes) -> Option<DocxOpcContentTypesDiff> {
    let defaults = diff_ct_entries(&old.defaults, &new.defaults);
    let overrides = diff_ct_entries(&old.overrides, &new.overrides);
    if defaults.is_none() && overrides.is_none() { None } else { Some(DocxOpcContentTypesDiff { defaults, overrides }) }
}

fn diff_part(old: &OpcPart, new: &OpcPart) -> Option<DocxOpcPartDiff> {
    if old == new {
        return None;
    }
    Some(DocxOpcPartDiff {
        content_type: (old.content_type != new.content_type).then(|| new.content_type.clone()),
        bytes: (old.bytes != new.bytes).then(|| new.bytes.clone()),
    })
}

fn apply_part(part: &mut OpcPart, diff: &DocxOpcPartDiff) {
    if let Some(v) = &diff.content_type {
        part.content_type = v.clone();
    }
    if let Some(v) = &diff.bytes {
        part.bytes = v.clone();
    }
}

fn part_with_diff_applied(part: &OpcPart, diff: &DocxOpcPartDiff) -> OpcPart {
    let mut out = part.clone();
    apply_part(&mut out, diff);
    out
}

fn inverse_part(base: &OpcPart, diff: &DocxOpcPartDiff) -> DocxOpcPartDiff {
    DocxOpcPartDiff {
        content_type: diff.content_type.as_ref().map(|_| base.content_type.clone()),
        bytes: diff.bytes.as_ref().map(|_| base.bytes.clone()),
    }
}

fn absorb_part_diff(mut a: DocxOpcPartDiff, b: DocxOpcPartDiff) -> DocxOpcPartDiff {
    if b.content_type.is_some() {
        a.content_type = b.content_type;
    }
    if b.bytes.is_some() {
        a.bytes = b.bytes;
    }
    a
}

fn diff_parts(old: &[OpcPart], new: &[OpcPart]) -> Option<DocxOpcPartsDiff> {
    between_named(old, new, |p| p.path.clone(), diff_part)
}

fn diff_rel(old: &OpcRelationship, new: &OpcRelationship) -> Option<DocxOpcRelDiff> {
    if old == new {
        return None;
    }
    Some(DocxOpcRelDiff {
        rel_type: (old.rel_type != new.rel_type).then(|| new.rel_type.clone()),
        target: (old.target != new.target).then(|| new.target.clone()),
        target_mode: (old.target_mode != new.target_mode).then_some(new.target_mode),
    })
}

fn apply_rel(rel: &mut OpcRelationship, diff: &DocxOpcRelDiff) {
    if let Some(v) = &diff.rel_type {
        rel.rel_type = v.clone();
    }
    if let Some(v) = &diff.target {
        rel.target = v.clone();
    }
    if let Some(v) = diff.target_mode {
        rel.target_mode = v;
    }
}

fn inverse_rel(base: &OpcRelationship, diff: &DocxOpcRelDiff) -> DocxOpcRelDiff {
    DocxOpcRelDiff {
        rel_type: diff.rel_type.as_ref().map(|_| base.rel_type.clone()),
        target: diff.target.as_ref().map(|_| base.target.clone()),
        target_mode: diff.target_mode.map(|_| base.target_mode),
    }
}

fn absorb_rel_diff(mut a: DocxOpcRelDiff, b: DocxOpcRelDiff) -> DocxOpcRelDiff {
    if b.rel_type.is_some() {
        a.rel_type = b.rel_type;
    }
    if b.target.is_some() {
        a.target = b.target;
    }
    if b.target_mode.is_some() {
        a.target_mode = b.target_mode;
    }
    a
}

fn diff_rel_list(old: &[OpcRelationship], new: &[OpcRelationship]) -> Option<DocxOpcRelListDiff> {
    between_named(old, new, |r| r.id.clone(), diff_rel)
}

fn apply_rel_list(list: &mut Vec<OpcRelationship>, diff: &DocxOpcRelListDiff) {
    apply_named(list, diff, |r| r.id.clone(), apply_rel);
}

fn rel_list_with_diff_applied(list: &[OpcRelationship], diff: &DocxOpcRelListDiff) -> Vec<OpcRelationship> {
    let mut out = list.to_vec();
    apply_rel_list(&mut out, diff);
    out
}

fn inverse_rel_list(base: &[OpcRelationship], diff: &DocxOpcRelListDiff) -> DocxOpcRelListDiff {
    inverse_named(base, diff, |r| r.id.clone(), inverse_rel)
}

fn absorb_rel_list_diff(a: DocxOpcRelListDiff, b: DocxOpcRelListDiff) -> DocxOpcRelListDiff {
    absorb_named(a, b, |r| r.id.clone(), absorb_rel_diff, apply_rel)
}

fn diff_relationships(old: &HashMap<String, Vec<OpcRelationship>>, new: &HashMap<String, Vec<OpcRelationship>>) -> Option<DocxOpcRelationshipsDiff> {
    let mut removed = Vec::new();
    let mut modified = Vec::new();
    for (owner, list) in old {
        match new.get(owner) {
            None => removed.push(owner.clone()),
            Some(nlist) => {
                if let Some(d) = diff_rel_list(list, nlist) {
                    modified.push(NamedModified { key: owner.clone(), diff: d });
                }
            }
        }
    }
    let mut added = Vec::new();
    for (owner, list) in new {
        if !old.contains_key(owner) {
            added.push((owner.clone(), list.clone()));
        }
    }
    if removed.is_empty() && modified.is_empty() && added.is_empty() { None } else { Some(DocxOpcRelationshipsDiff { removed, modified, added }) }
}

fn apply_relationships(rels: &mut HashMap<String, Vec<OpcRelationship>>, diff: &DocxOpcRelationshipsDiff) {
    for owner in &diff.removed {
        rels.remove(owner);
    }
    for m in &diff.modified {
        if let Some(list) = rels.get_mut(&m.key) {
            apply_rel_list(list, &m.diff);
        }
    }
    for (owner, list) in &diff.added {
        rels.insert(owner.clone(), list.clone());
    }
}

fn inverse_relationships(base: &HashMap<String, Vec<OpcRelationship>>, diff: &DocxOpcRelationshipsDiff) -> DocxOpcRelationshipsDiff {
    let removed: Vec<String> = diff.added.iter().map(|(owner, _)| owner.clone()).collect();
    let mut modified = Vec::new();
    for m in &diff.modified {
        if let Some(list) = base.get(&m.key) {
            modified.push(NamedModified { key: m.key.clone(), diff: inverse_rel_list(list, &m.diff) });
        }
    }
    let mut added = Vec::new();
    for owner in &diff.removed {
        if let Some(list) = base.get(owner) {
            added.push((owner.clone(), list.clone()));
        }
    }
    DocxOpcRelationshipsDiff { removed, modified, added }
}

fn absorb_relationships(d1: DocxOpcRelationshipsDiff, d2: DocxOpcRelationshipsDiff) -> DocxOpcRelationshipsDiff {
    absorb_named(
        d1,
        d2,
        |(owner, _)| owner.clone(),
        absorb_rel_list_diff,
        |(_, list), diff| *list = rel_list_with_diff_applied(list, diff),
    )
}

fn diff_opc(base: &OpcPackage, other: &OpcPackage) -> Option<DocxOpcDiff> {
    let content_types = diff_content_types(&base.content_types, &other.content_types);
    let parts = diff_parts(&base.parts, &other.parts);
    let relationships = diff_relationships(&base.relationships, &other.relationships);
    if content_types.is_none() && parts.is_none() && relationships.is_none() { None } else { Some(DocxOpcDiff { content_types, parts, relationships }) }
}

fn apply_opc_diff(opc: &mut OpcPackage, diff: &DocxOpcDiff) {
    if let Some(d) = &diff.content_types {
        if let Some(dd) = &d.defaults {
            apply_ct_entries(&mut opc.content_types.defaults, dd);
        }
        if let Some(dd) = &d.overrides {
            apply_ct_entries(&mut opc.content_types.overrides, dd);
        }
    }
    if let Some(d) = &diff.parts {
        apply_named(&mut opc.parts, d, |p| p.path.clone(), apply_part);
    }
    if let Some(d) = &diff.relationships {
        apply_relationships(&mut opc.relationships, d);
    }
}

fn inverse_opc_diff(base: &OpcPackage, diff: &DocxOpcDiff) -> DocxOpcDiff {
    DocxOpcDiff {
        content_types: diff.content_types.as_ref().map(|d| DocxOpcContentTypesDiff {
            defaults: d.defaults.as_ref().map(|dd| inverse_ct_entries(&base.content_types.defaults, dd)),
            overrides: d.overrides.as_ref().map(|dd| inverse_ct_entries(&base.content_types.overrides, dd)),
        }),
        parts: diff.parts.as_ref().map(|d| inverse_named(&base.parts, d, |p| p.path.clone(), inverse_part)),
        relationships: diff.relationships.as_ref().map(|d| inverse_relationships(&base.relationships, d)),
    }
}

fn absorb_opc_diff(a: DocxOpcDiff, b: DocxOpcDiff) -> DocxOpcDiff {
    DocxOpcDiff {
        content_types: match (a.content_types, b.content_types) {
            (None, x) => x,
            (x, None) => x,
            (Some(ca), Some(cb)) => Some(DocxOpcContentTypesDiff {
                defaults: match (ca.defaults, cb.defaults) {
                    (None, x) => x,
                    (x, None) => x,
                    (Some(da), Some(db)) => Some(absorb_ct_entries(da, db)),
                },
                overrides: match (ca.overrides, cb.overrides) {
                    (None, x) => x,
                    (x, None) => x,
                    (Some(da), Some(db)) => Some(absorb_ct_entries(da, db)),
                },
            }),
        },
        parts: match (a.parts, b.parts) {
            (None, x) => x,
            (x, None) => x,
            (Some(pa), Some(pb)) => Some(absorb_named(pa, pb, |p| p.path.clone(), absorb_part_diff, |part, diff| *part = part_with_diff_applied(part, diff))),
        },
        relationships: match (a.relationships, b.relationships) {
            (None, x) => x,
            (x, None) => x,
            (Some(ra), Some(rb)) => Some(absorb_relationships(ra, rb)),
        },
    }
}
//#endregion 🔖️OpcDiffLogic

//#region 🔖️Apply
impl MutationDiff<DocxSnapshot> for DocxDiff {
    fn apply(&self, base: &DocxSnapshot) -> DocxSnapshot {
        let mut next = base.clone();
        if let Some(d) = &self.opc {
            apply_opc_diff(&mut next.opc, d);
        }
        if let Some(d) = &self.document {
            apply_document_diff(&mut next.document, d);
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        self.opc = match (self.opc.take(), other.opc) {
            (None, x) => x,
            (x, None) => x,
            (Some(a), Some(b)) => Some(absorb_opc_diff(a, b)),
        };
        self.document = match (self.document.take(), other.document) {
            (None, x) => x,
            (x, None) => x,
            (Some(a), Some(b)) => Some(absorb_document_diff(a, b)),
        };
    }
}
//#endregion 🔖️Apply

//#region 🔖️DiffAlgebra
impl DiffAlgebra<DocxSnapshot> for DocxDiff {
    fn inverse(&self, base: &DocxSnapshot) -> Self {
        DocxDiff {
            opc: self.opc.as_ref().map(|d| inverse_opc_diff(&base.opc, d)),
            document: self.document.as_ref().map(|d| inverse_document_diff(&base.document, d)),
        }
    }

    fn between(base: &DocxSnapshot, other: &DocxSnapshot) -> Self {
        DocxDiff { opc: diff_opc(&base.opc, &other.opc), document: diff_document(&base.document, &other.document) }
    }

    fn is_empty(&self) -> bool {
        self.opc.is_none() && self.document.is_none()
    }
}
//#endregion 🔖️DiffAlgebra

//#region 🔖️SetSnapshot
/// 🧩 Builds the sparse field-by-field diff for a `SetSnapshot` mutation. No `snapshot:
/// Option<DocxSnapshot>` full-replace slot -- this IS `DocxDiff::between`.
pub fn diff_set_snapshot(base: &DocxSnapshot, next: &DocxSnapshot) -> DocxDiff {
    DocxDiff::between(base, next)
}

/// 🧩 Builds the diff for inserting `block` at `path` (`path.index` = insertion index, FINAL
/// state).
pub fn diff_insert_block(path: &DocxBlockPath, block: DocxBlock) -> DocxDiff {
    wrap_body_diff(path, DocxBlockLeaf::Inserted(block))
}

/// 🧩 Builds the diff for removing the block at `path` (`path.index` = BASE-state index).
pub fn diff_remove_block(path: &DocxBlockPath) -> DocxDiff {
    wrap_body_diff(path, DocxBlockLeaf::Removed)
}

/// 🧩 Builds the diff for replacing the block at `path` (BASE-state index) with `new_block`'s full
/// content, via a real structural comparison against `old_block` (never full-replace unless the
/// block KIND actually changed).
pub fn diff_set_block_content(path: &DocxBlockPath, old_block: &DocxBlock, new_block: &DocxBlock) -> DocxDiff {
    match diff_block(old_block, new_block) {
        None => DocxDiff::default(),
        Some(d) => wrap_body_diff(path, DocxBlockLeaf::Modified(d)),
    }
}

/// 🧩 Builds the diff for editing one run's text within the paragraph at `path`.
pub fn diff_set_run_text(document: &DocxDocument, path: &DocxBlockPath, run_index: usize, text: &str) -> DocxDiff {
    let Some(blocks) = resolve_blocks(&document.body, &path.segments) else { return DocxDiff::default() };
    let Some(DocxBlock::Paragraph(p)) = blocks.get(path.index) else { return DocxDiff::default() };
    let Some(run) = p.runs.get(run_index) else { return DocxDiff::default() };
    if run.text == text {
        return DocxDiff::default();
    }
    let run_diff = DocxRunDiff { text: Some(text.to_string()), bold: None, italic: None, underline: None };
    let runs_diff = DocxRunsDiff { modified: vec![IndexModified { index: run_index, diff: run_diff }], ..Default::default() };
    let block_diff = DocxBlockDiff::Paragraph(DocxParagraphDiff { runs: Some(runs_diff), style: None });
    wrap_body_diff(path, DocxBlockLeaf::Modified(block_diff))
}

/// 🧩 Builds the diff for setting one run's bold/italic/underline flags within the paragraph at
/// `path`.
pub fn diff_set_run_formatting(document: &DocxDocument, path: &DocxBlockPath, run_index: usize, bold: bool, italic: bool, underline: bool) -> DocxDiff {
    let Some(blocks) = resolve_blocks(&document.body, &path.segments) else { return DocxDiff::default() };
    let Some(DocxBlock::Paragraph(p)) = blocks.get(path.index) else { return DocxDiff::default() };
    let Some(run) = p.runs.get(run_index) else { return DocxDiff::default() };
    let run_diff = DocxRunDiff {
        text: None,
        bold: (run.bold != bold).then_some(bold),
        italic: (run.italic != italic).then_some(italic),
        underline: (run.underline != underline).then_some(underline),
    };
    if run_diff.bold.is_none() && run_diff.italic.is_none() && run_diff.underline.is_none() {
        return DocxDiff::default();
    }
    let runs_diff = DocxRunsDiff { modified: vec![IndexModified { index: run_index, diff: run_diff }], ..Default::default() };
    let block_diff = DocxBlockDiff::Paragraph(DocxParagraphDiff { runs: Some(runs_diff), style: None });
    wrap_body_diff(path, DocxBlockLeaf::Modified(block_diff))
}

/// 🧩 Builds the diff for inserting a style.
pub fn diff_insert_style(style: DocxStyle) -> DocxDiff {
    DocxDiff { opc: None, document: Some(DocxDocumentDiff { body: None, styles: Some(DocxStylesDiff { added: vec![style], ..Default::default() }) }) }
}

/// 🧩 Builds the diff for removing a style by id.
pub fn diff_remove_style(id: &str) -> DocxDiff {
    DocxDiff { opc: None, document: Some(DocxDocumentDiff { body: None, styles: Some(DocxStylesDiff { removed: vec![id.to_string()], ..Default::default() }) }) }
}

/// 🧩 Builds the diff for setting a style's name.
pub fn diff_set_style_name(id: &str, name: &str) -> DocxDiff {
    let sd = DocxStyleDiff { name: Some(name.to_string()), based_on: None };
    DocxDiff { opc: None, document: Some(DocxDocumentDiff { body: None, styles: Some(DocxStylesDiff { modified: vec![NamedModified { key: id.to_string(), diff: sd }], ..Default::default() }) }) }
}

/// 🧩 Builds the diff for setting (or clearing, `based_on: None`) a style's `based_on`.
pub fn diff_set_style_based_on(id: &str, based_on: Option<String>) -> DocxDiff {
    let sd = DocxStyleDiff { name: None, based_on: Some(based_on) };
    DocxDiff { opc: None, document: Some(DocxDocumentDiff { body: None, styles: Some(DocxStylesDiff { modified: vec![NamedModified { key: id.to_string(), diff: sd }], ..Default::default() }) }) }
}

/// 🧩 Builds the diff for setting a raw OPC part (content this typed layer doesn't cover).
pub fn diff_set_part(opc: &OpcPackage, path: &str, content_type: &str, bytes: Vec<u8>) -> DocxDiff {
    let p = path.trim_start_matches('/').to_string();
    match opc.parts.iter().find(|part| part.path == p) {
        Some(existing) => {
            let new_part = OpcPart { path: p, content_type: content_type.to_string(), bytes };
            match diff_part(existing, &new_part) {
                None => DocxDiff::default(),
                Some(d) => DocxDiff {
                    opc: Some(DocxOpcDiff { content_types: None, parts: Some(DocxOpcPartsDiff { modified: vec![NamedModified { key: existing.path.clone(), diff: d }], ..Default::default() }), relationships: None }),
                    document: None,
                },
            }
        }
        None => DocxDiff {
            opc: Some(DocxOpcDiff { content_types: None, parts: Some(DocxOpcPartsDiff { added: vec![OpcPart { path: p, content_type: content_type.to_string(), bytes }], ..Default::default() }), relationships: None }),
            document: None,
        },
    }
}

/// 🧩 Builds the diff for removing a raw OPC part by path.
pub fn diff_remove_part(path: &str) -> DocxDiff {
    let p = path.trim_start_matches('/').to_string();
    DocxDiff { opc: Some(DocxOpcDiff { content_types: None, parts: Some(DocxOpcPartsDiff { removed: vec![p], ..Default::default() }), relationships: None }), document: None }
}
//#endregion 🔖️SetSnapshot
