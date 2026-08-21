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
use crate::artifacts::xml::schema::snapshot::{XmlAttr, XmlNode};
use crate::artifacts::zip::opc::{OpcContentTypes, OpcPackage, OpcPart, OpcRelationship, OpcTargetMode};
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
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
    fn default() -> Self {
        Self { removed: Vec::new(), modified: Vec::new(), added: Vec::new() }
    }
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
    fn default() -> Self {
        Self { removed: Vec::new(), modified: Vec::new(), added: Vec::new() }
    }
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
/// 🧪️ F6 VERIFIED: `#[derive(dsl::DslDiff)]` on this struct fails to compile with TWO independent,
/// simultaneous reasons (both captured verbatim via a real `cargo check -p semio-s-plugin-stdio
/// --lib`, per `f6-recon-report.md` §3, then reverted): (1) enum-in-tree —
/// `IndexedTripleDiff<DocxBlockDiff, DocxBlock>: DslField` is not satisfied (`DocxBlockDiff` is a
/// genuine data-carrying enum, `Paragraph`/`Table`/`Replace`, and `DslField` has no impl for it or
/// for the generic collection-triple type wrapping it); (2) tri-state `Option<Option<T>>` —
/// `style: Option<Option<String>>` (`DocxParagraphDiff`) and `based_on: Option<Option<String>>`
/// (`DocxStyleDiff`) both fail with `Option<String>: DslField` is not satisfied, same root cause as
/// `GifDiff`. `DiffCodec` is hand-rolled below, following the svg/gif template exactly (§5 of the
/// recon report).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.docx.diff")]
pub struct DocxDiff {
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub opc: Option<DocxOpcDiff>,
    #[state(artifact)]
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn wrap_body_diff(path: &DocxBlockPath, leaf: DocxBlockLeaf) -> DocxDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn go(segments: &[DocxPathSegment], index: usize, leaf: DocxBlockLeaf) -> DocxBlocksDiff {
        match segments.split_first() {
            None => leaf.into_blocks_diff(index),
            Some((seg, rest)) => {
                let inner = go(rest, index, leaf);
                let cell_diff = DocxTableCellDiff { blocks: Some(Box::pin(inner)) };
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn resolve_blocks<'a>(body: &'a [DocxBlock], segments: &[DocxPathSegment]) -> Option<&'a [DocxBlock]> {
    match segments.split_first() {
        None => Some(body),
        Some((seg, rest)) => {
            let DocxBlock::Table(table) = body.get(seg.block_index)? else { return None };
            let row = table.rows.get(seg.row)?;
            let cell = row.cells.get(seg.cell)?;
            Box::pin(resolve_blocks(&cell.blocks, rest))
        }
    }
}
//#endregion 🔖️PathAddressing

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
fn apply_indexed<T, D>(items: &mut Vec<T>, diff: &IndexedTripleDiff<D, T>, apply_item: impl Fn(&mut T, &D) -> MutationApplyResult<()>) -> MutationApplyResult<()>
where
    T: Clone,
{
    let mut removed = std::collections::HashSet::new();
    for &idx in &diff.removed {
        if idx >= items.len() {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "indexed removal target does not exist").at(vec!["removed".to_string(), idx.to_string()]));
        }
        if !removed.insert(idx) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "indexed removal target is repeated").at(vec!["removed".to_string(), idx.to_string()]));
        }
    }
    let mut modified = std::collections::HashSet::new();
    for m in &diff.modified {
        if m.index >= items.len() {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "indexed modification target does not exist").at(vec!["modified".to_string(), m.index.to_string()]));
        }
        if removed.contains(&m.index) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "indexed modification targets a removed item").at(vec!["modified".to_string(), m.index.to_string()]));
        }
        if !modified.insert(m.index) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "indexed modification target is repeated").at(vec!["modified".to_string(), m.index.to_string()]));
        }
    }
    let final_len = items.len() - removed.len() + diff.added.len();
    let mut added = std::collections::HashSet::new();
    for add in &diff.added {
        if add.index >= final_len {
            return Err(MutationApplyError::new("mutation.apply.invalid-index", "indexed addition is outside the final collection").at(vec!["added".to_string(), add.index.to_string()]));
        }
        if !added.insert(add.index) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "indexed addition occupies a repeated final position").at(vec!["added".to_string(), add.index.to_string()]));
        }
    }
    for m in &diff.modified {
        let item = items.get_mut(m.index).ok_or_else(|| semio_framework_plugin::resolve_ready(MutationApplyError::new("mutation.apply.missing-target", "indexed modification target does not exist")).at(vec!["modified".to_string(), m.index.to_string()]))?;
        apply_item(item, &m.diff).map_err(|error| error.under(vec!["modified".to_string(), m.index.to_string()]))?;
    }
    let mut removed_sorted = diff.removed.clone();
    removed_sorted.sort_unstable_by(|a, b| b.cmp(a));
    for idx in removed_sorted {
        items.remove(idx);
    }
    let mut additions: Vec<&IndexAdded<T>> = diff.added.iter().collect();
    additions.sort_by_key(|a| a.index);
    for add in additions {
        items.insert(add.index, add.item.clone());
    }
    Ok(())
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

/// 🧮️ Sequential-coalesce absorb per the recipe's normative algorithm (svg `absorb_children_diff`,
/// generalized): `absorb_item` recursively absorbs two per-field diffs of the SAME item;
/// `apply_item` patches a `D` onto a `T` (needed when `d2` modifies an item `d1` just added).
#[allow(clippy::too_many_arguments)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
fn apply_named<K, T, D>(items: &mut Vec<T>, diff: &NamedTripleDiff<K, D, T>, key_of: impl Fn(&T) -> K, apply_item: impl Fn(&mut T, &D) -> MutationApplyResult<()>) -> MutationApplyResult<()>
where
    K: PartialEq + Clone,
    T: Clone,
{
    let keys: Vec<K> = items.iter().map(&key_of).collect();
    for key in &diff.removed {
        if !keys.contains(key) {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "named removal target does not exist").at(["removed"]));
        }
    }
    for (index, key) in diff.removed.iter().enumerate() {
        if diff.removed[..index].contains(key) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "named removal target is repeated").at(["removed"]));
        }
    }
    let mut modified_keys = Vec::new();
    for modified in &diff.modified {
        if !keys.contains(&modified.key) {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "named modification target does not exist").at(["modified"]));
        }
        if diff.removed.contains(&modified.key) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "named modification targets a removed item").at(["modified"]));
        }
        if modified_keys.contains(&modified.key) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "named modification target is repeated").at(["modified"]));
        }
        modified_keys.push(modified.key.clone());
    }
    let mut added_keys = Vec::new();
    for item in &diff.added {
        let key = key_of(item);
        if keys.contains(&key) || added_keys.contains(&key) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "named addition target already exists").at(["added"]));
        }
        added_keys.push(key);
    }
    items.retain(|i| !diff.removed.contains(&key_of(i)));
    for m in &diff.modified {
        let item = items.iter_mut().find(|i| key_of(i) == m.key).ok_or_else(|| semio_framework_plugin::resolve_ready(MutationApplyError::new("mutation.apply.missing-target", "named modification target does not exist")).at(["modified"]))?;
        apply_item(item, &m.diff).map_err(|error| error.under(["modified"]))?;
    }
    for item in &diff.added {
        items.push(item.clone());
    }
    Ok(())
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

/// 🧮️ Name-keyed absorb — identity is the KEY (not position), so no index transport is needed:
/// a `d2`-removal of a `d1`-added key annihilates the add; a `d2`-modify of a `d1`-added key
/// patches into the carried payload; everything else composes directly on the shared key space.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
//#endregion 🔖️GenericNamedEngine

//#region 🔖️DocumentDiffLogic
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_paragraph(old: &DocxParagraph, new: &DocxParagraph) -> Option<DocxParagraphDiff> {
    let runs = between_indexed(&old.runs, &new.runs, diff_run);
    let style = if old.style != new.style { Some(new.style.clone()) } else { None };
    if runs.is_none() && style.is_none() {
        None
    } else {
        Some(DocxParagraphDiff { runs, style })
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_table(old: &DocxTable, new: &DocxTable) -> Option<DocxTableDiff> {
    let rows = between_indexed(&old.rows, &new.rows, diff_row);
    if rows.is_none() {
        None
    } else {
        Some(DocxTableDiff { rows })
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_row(old: &DocxTableRow, new: &DocxTableRow) -> Option<DocxTableRowDiff> {
    let cells = between_indexed(&old.cells, &new.cells, diff_cell);
    if cells.is_none() {
        None
    } else {
        Some(DocxTableRowDiff { cells })
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_cell(old: &DocxTableCell, new: &DocxTableCell) -> Option<DocxTableCellDiff> {
    let blocks = between_indexed(&old.blocks, &new.blocks, diff_block);
    if blocks.is_none() {
        None
    } else {
        Some(DocxTableCellDiff { blocks })
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_style(old: &DocxStyle, new: &DocxStyle) -> Option<DocxStyleDiff> {
    if old == new {
        return None;
    }
    Some(DocxStyleDiff { name: (old.name != new.name).then(|| new.name.clone()), based_on: (old.based_on != new.based_on).then(|| new.based_on.clone()) })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_document(base: &DocxDocument, other: &DocxDocument) -> Option<DocxDocumentDiff> {
    let body = between_indexed(&base.body, &other.body, diff_block);
    let styles = between_named(&base.styles, &other.styles, |s| s.id.clone(), diff_style);
    if body.is_none() && styles.is_none() {
        None
    } else {
        Some(DocxDocumentDiff { body, styles })
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_block(block: &mut DocxBlock, diff: &DocxBlockDiff) -> MutationApplyResult<()> {
    match diff {
        DocxBlockDiff::Replace { block: new } => *block = new.clone(),
        DocxBlockDiff::Paragraph(pd) => {
            let DocxBlock::Paragraph(p) = block else {
                return Err(MutationApplyError::new("mutation.apply.kind-mismatch", "paragraph diff targets a non-paragraph block"));
            };
            if let Some(rd) = &pd.runs {
                apply_indexed(&mut p.runs, rd, apply_run).map_err(|error| error.under(["runs"]))?;
            }
            if let Some(s) = &pd.style {
                p.style = s.clone();
            }
        }
        DocxBlockDiff::Table(td) => {
            let DocxBlock::Table(t) = block else {
                return Err(MutationApplyError::new("mutation.apply.kind-mismatch", "table diff targets a non-table block"));
            };
            if let Some(rd) = &td.rows {
                apply_indexed(&mut t.rows, rd, apply_row).map_err(|error| error.under(["rows"]))?;
            }
        }
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_run(run: &mut DocxRun, diff: &DocxRunDiff) -> MutationApplyResult<()> {
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
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_row(row: &mut DocxTableRow, diff: &DocxTableRowDiff) -> MutationApplyResult<()> {
    if let Some(cd) = &diff.cells {
        apply_indexed(&mut row.cells, cd, apply_cell).map_err(|error| error.under(["cells"]))?;
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_cell(cell: &mut DocxTableCell, diff: &DocxTableCellDiff) -> MutationApplyResult<()> {
    if let Some(bd) = &diff.blocks {
        apply_indexed(&mut cell.blocks, bd, apply_block).map_err(|error| error.under(["blocks"]))?;
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_style(style: &mut DocxStyle, diff: &DocxStyleDiff) -> MutationApplyResult<()> {
    if let Some(v) = &diff.name {
        style.name = v.clone();
    }
    if let Some(v) = &diff.based_on {
        style.based_on = v.clone();
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_document_diff(doc: &mut DocxDocument, diff: &DocxDocumentDiff) -> MutationApplyResult<()> {
    if let Some(bd) = &diff.body {
        apply_indexed(&mut doc.body, bd, apply_block).map_err(|error| error.under(["body"]))?;
    }
    if let Some(sd) = &diff.styles {
        apply_named(&mut doc.styles, sd, |s| s.id.clone(), apply_style).map_err(|error| error.under(["styles"]))?;
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn block_with_diff_applied(block: &DocxBlock, diff: &DocxBlockDiff) -> DocxBlock {
    let mut out = block.clone();
    apply_block_for_absorb(&mut out, diff);
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn run_with_diff_applied(run: &DocxRun, diff: &DocxRunDiff) -> DocxRun {
    let mut out = run.clone();
    apply_run_for_absorb(&mut out, diff);
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn row_with_diff_applied(row: &DocxTableRow, diff: &DocxTableRowDiff) -> DocxTableRow {
    let mut out = row.clone();
    apply_row_for_absorb(&mut out, diff);
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn cell_with_diff_applied(cell: &DocxTableCell, diff: &DocxTableCellDiff) -> DocxTableCell {
    let mut out = cell.clone();
    apply_cell_for_absorb(&mut out, diff);
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_indexed_for_absorb<T, D>(items: &mut Vec<T>, diff: &IndexedTripleDiff<D, T>, apply_item: impl Fn(&mut T, &D))
where
    T: Clone,
{
    for modified in &diff.modified {
        apply_item(&mut items[modified.index], &modified.diff);
    }
    let mut removed = diff.removed.clone();
    removed.sort_unstable_by(|a, b| b.cmp(a));
    for index in removed {
        items.remove(index);
    }
    let mut added: Vec<&IndexAdded<T>> = diff.added.iter().collect();
    added.sort_by_key(|item| item.index);
    for item in added {
        items.insert(item.index, item.item.clone());
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_block_for_absorb(block: &mut DocxBlock, diff: &DocxBlockDiff) {
    match diff {
        DocxBlockDiff::Replace { block: new } => *block = new.clone(),
        DocxBlockDiff::Paragraph(pd) => {
            if let DocxBlock::Paragraph(paragraph) = block {
                if let Some(runs) = &pd.runs {
                    apply_indexed_for_absorb(&mut paragraph.runs, runs, apply_run_for_absorb);
                }
                if let Some(style) = &pd.style {
                    paragraph.style = style.clone();
                }
            }
        }
        DocxBlockDiff::Table(td) => {
            if let DocxBlock::Table(table) = block {
                if let Some(rows) = &td.rows {
                    apply_indexed_for_absorb(&mut table.rows, rows, apply_row_for_absorb);
                }
            }
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_run_for_absorb(run: &mut DocxRun, diff: &DocxRunDiff) {
    if let Some(value) = &diff.text {
        run.text = value.clone();
    }
    if let Some(value) = diff.bold {
        run.bold = value;
    }
    if let Some(value) = diff.italic {
        run.italic = value;
    }
    if let Some(value) = diff.underline {
        run.underline = value;
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_row_for_absorb(row: &mut DocxTableRow, diff: &DocxTableRowDiff) {
    if let Some(cells) = &diff.cells {
        apply_indexed_for_absorb(&mut row.cells, cells, apply_cell_for_absorb);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_cell_for_absorb(cell: &mut DocxTableCell, diff: &DocxTableCellDiff) {
    if let Some(blocks) = &diff.blocks {
        apply_indexed_for_absorb(&mut cell.blocks, blocks, apply_block_for_absorb);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_style_for_absorb(style: &mut DocxStyle, diff: &DocxStyleDiff) {
    if let Some(value) = &diff.name {
        style.name = value.clone();
    }
    if let Some(value) = &diff.based_on {
        style.based_on = value.clone();
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_block(base: &DocxBlock, diff: &DocxBlockDiff) -> DocxBlockDiff {
    match diff {
        DocxBlockDiff::Replace { .. } => DocxBlockDiff::Replace { block: base.clone() },
        DocxBlockDiff::Paragraph(pd) => {
            let DocxBlock::Paragraph(p) = base else { return DocxBlockDiff::Replace { block: base.clone() } };
            DocxBlockDiff::Paragraph(DocxParagraphDiff { runs: pd.runs.as_ref().map(|rd| inverse_indexed(&p.runs, rd, inverse_run)), style: pd.style.as_ref().map(|_| p.style.clone()) })
        }
        DocxBlockDiff::Table(td) => {
            let DocxBlock::Table(t) = base else { return DocxBlockDiff::Replace { block: base.clone() } };
            DocxBlockDiff::Table(DocxTableDiff { rows: td.rows.as_ref().map(|rd| inverse_indexed(&t.rows, rd, inverse_row)) })
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_run(base: &DocxRun, diff: &DocxRunDiff) -> DocxRunDiff {
    DocxRunDiff { text: diff.text.as_ref().map(|_| base.text.clone()), bold: diff.bold.map(|_| base.bold), italic: diff.italic.map(|_| base.italic), underline: diff.underline.map(|_| base.underline) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_row(base: &DocxTableRow, diff: &DocxTableRowDiff) -> DocxTableRowDiff {
    DocxTableRowDiff { cells: diff.cells.as_ref().map(|cd| inverse_indexed(&base.cells, cd, inverse_cell)) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_cell(base: &DocxTableCell, diff: &DocxTableCellDiff) -> DocxTableCellDiff {
    DocxTableCellDiff { blocks: diff.blocks.as_ref().map(|bd| inverse_indexed(&base.blocks, bd, inverse_block)) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_style(base: &DocxStyle, diff: &DocxStyleDiff) -> DocxStyleDiff {
    DocxStyleDiff { name: diff.name.as_ref().map(|_| base.name.clone()), based_on: diff.based_on.as_ref().map(|_| base.based_on.clone()) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_document_diff(base: &DocxDocument, diff: &DocxDocumentDiff) -> DocxDocumentDiff {
    DocxDocumentDiff { body: diff.body.as_ref().map(|bd| inverse_indexed(&base.body, bd, inverse_block)), styles: diff.styles.as_ref().map(|sd| inverse_named(&base.styles, sd, |s| s.id.clone(), inverse_style)) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_block_diff(a: DocxBlockDiff, b: DocxBlockDiff) -> DocxBlockDiff {
    match (a, b) {
        (_, DocxBlockDiff::Replace { block }) => DocxBlockDiff::Replace { block },
        (DocxBlockDiff::Replace { block }, b) => DocxBlockDiff::Replace { block: block_with_diff_applied(&block, &b) },
        (DocxBlockDiff::Paragraph(pa), DocxBlockDiff::Paragraph(pb)) => DocxBlockDiff::Paragraph(absorb_paragraph_diff(pa, pb)),
        (DocxBlockDiff::Table(ta), DocxBlockDiff::Table(tb)) => DocxBlockDiff::Table(absorb_table_diff(ta, tb)),
        (_, b) => b,
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_run_diff(a: DocxRunDiff, b: DocxRunDiff) -> DocxRunDiff {
    DocxRunDiff { text: b.text.or(a.text), bold: b.bold.or(a.bold), italic: b.italic.or(a.italic), underline: b.underline.or(a.underline) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_table_diff(mut a: DocxTableDiff, b: DocxTableDiff) -> DocxTableDiff {
    a.rows = match (a.rows.take(), b.rows) {
        (None, x) => x,
        (x, None) => x,
        (Some(ra), Some(rb)) => Some(absorb_indexed(ra, rb, absorb_row_diff, row_with_diff_applied)),
    };
    a
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_row_diff(mut a: DocxTableRowDiff, b: DocxTableRowDiff) -> DocxTableRowDiff {
    a.cells = match (a.cells.take(), b.cells) {
        (None, x) => x,
        (x, None) => x,
        (Some(ca), Some(cb)) => Some(absorb_indexed(ca, cb, absorb_cell_diff, cell_with_diff_applied)),
    };
    a
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_cell_diff(mut a: DocxTableCellDiff, b: DocxTableCellDiff) -> DocxTableCellDiff {
    a.blocks = match (a.blocks.take(), b.blocks) {
        (None, x) => x,
        (x, None) => x,
        (Some(ba), Some(bb)) => Some(absorb_indexed(ba, bb, absorb_block_diff, block_with_diff_applied)),
    };
    a
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_style_diff(mut a: DocxStyleDiff, b: DocxStyleDiff) -> DocxStyleDiff {
    if b.name.is_some() {
        a.name = b.name;
    }
    if b.based_on.is_some() {
        a.based_on = b.based_on;
    }
    a
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
            (Some(sa), Some(sb)) => Some(absorb_named(sa, sb, |s| s.id.clone(), absorb_style_diff, apply_style_for_absorb)),
        },
    }
}
//#endregion 🔖️DocumentDiffLogic

//#region 🔖️OpcDiffLogic
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_ct_entries(old: &[(String, String)], new: &[(String, String)]) -> Option<DocxOpcCtEntriesDiff> {
    between_named(old, new, |(k, _)| k.clone(), |(_, ov), (_, nv)| (ov != nv).then(|| nv.clone()))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_ct_entries(entries: &mut Vec<(String, String)>, diff: &DocxOpcCtEntriesDiff) -> MutationApplyResult<()> {
    apply_named(
        entries,
        diff,
        |(k, _)| k.clone(),
        |(_, v), nv| {
            *v = nv.clone();
            Ok(())
        },
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_ct_entries(base: &[(String, String)], diff: &DocxOpcCtEntriesDiff) -> DocxOpcCtEntriesDiff {
    inverse_named(base, diff, |(k, _)| k.clone(), |(_, v), _| v.clone())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_ct_entries(a: DocxOpcCtEntriesDiff, b: DocxOpcCtEntriesDiff) -> DocxOpcCtEntriesDiff {
    // 🏷️ `D = String` here is already a whole-value replace (LWW) -- absorbing two such diffs on
    // the SAME key is just "the later one wins", i.e. `b`.
    absorb_named(a, b, |(k, _)| k.clone(), |_av, bv| bv, |(_, v), nv| *v = nv.clone())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_content_types(old: &OpcContentTypes, new: &OpcContentTypes) -> Option<DocxOpcContentTypesDiff> {
    let defaults = diff_ct_entries(&old.defaults, &new.defaults);
    let overrides = diff_ct_entries(&old.overrides, &new.overrides);
    if defaults.is_none() && overrides.is_none() {
        None
    } else {
        Some(DocxOpcContentTypesDiff { defaults, overrides })
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_part(old: &OpcPart, new: &OpcPart) -> Option<DocxOpcPartDiff> {
    if old == new {
        return None;
    }
    Some(DocxOpcPartDiff { content_type: (old.content_type != new.content_type).then(|| new.content_type.clone()), bytes: (old.bytes != new.bytes).then(|| new.bytes.clone()) })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_part(part: &mut OpcPart, diff: &DocxOpcPartDiff) {
    if let Some(v) = &diff.content_type {
        part.content_type = v.clone();
    }
    if let Some(v) = &diff.bytes {
        part.bytes = v.clone();
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn part_with_diff_applied(part: &OpcPart, diff: &DocxOpcPartDiff) -> OpcPart {
    let mut out = part.clone();
    apply_part(&mut out, diff);
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_part(base: &OpcPart, diff: &DocxOpcPartDiff) -> DocxOpcPartDiff {
    DocxOpcPartDiff { content_type: diff.content_type.as_ref().map(|_| base.content_type.clone()), bytes: diff.bytes.as_ref().map(|_| base.bytes.clone()) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_part_diff(mut a: DocxOpcPartDiff, b: DocxOpcPartDiff) -> DocxOpcPartDiff {
    if b.content_type.is_some() {
        a.content_type = b.content_type;
    }
    if b.bytes.is_some() {
        a.bytes = b.bytes;
    }
    a
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_parts(old: &[OpcPart], new: &[OpcPart]) -> Option<DocxOpcPartsDiff> {
    between_named(old, new, |p| p.path.clone(), diff_part)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_rel(old: &OpcRelationship, new: &OpcRelationship) -> Option<DocxOpcRelDiff> {
    if old == new {
        return None;
    }
    Some(DocxOpcRelDiff { rel_type: (old.rel_type != new.rel_type).then(|| new.rel_type.clone()), target: (old.target != new.target).then(|| new.target.clone()), target_mode: (old.target_mode != new.target_mode).then_some(new.target_mode) })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_rel(base: &OpcRelationship, diff: &DocxOpcRelDiff) -> DocxOpcRelDiff {
    DocxOpcRelDiff { rel_type: diff.rel_type.as_ref().map(|_| base.rel_type.clone()), target: diff.target.as_ref().map(|_| base.target.clone()), target_mode: diff.target_mode.map(|_| base.target_mode) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_rel_list(old: &[OpcRelationship], new: &[OpcRelationship]) -> Option<DocxOpcRelListDiff> {
    between_named(old, new, |r| r.id.clone(), diff_rel)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_rel_list(list: &mut Vec<OpcRelationship>, diff: &DocxOpcRelListDiff) -> MutationApplyResult<()> {
    apply_named(
        list,
        diff,
        |r| r.id.clone(),
        |relationship, change| {
            apply_rel(relationship, change);
            Ok(())
        },
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn rel_list_with_diff_applied(list: &[OpcRelationship], diff: &DocxOpcRelListDiff) -> Vec<OpcRelationship> {
    let mut out = list.to_vec();
    out.retain(|relationship| !diff.removed.contains(&relationship.id));
    for modified in &diff.modified {
        if let Some(relationship) = out.iter_mut().find(|relationship| relationship.id == modified.key) {
            apply_rel(relationship, &modified.diff);
        }
    }
    out.extend(diff.added.iter().cloned());
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_rel_list(base: &[OpcRelationship], diff: &DocxOpcRelListDiff) -> DocxOpcRelListDiff {
    inverse_named(base, diff, |r| r.id.clone(), inverse_rel)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_rel_list_diff(a: DocxOpcRelListDiff, b: DocxOpcRelListDiff) -> DocxOpcRelListDiff {
    absorb_named(a, b, |r| r.id.clone(), absorb_rel_diff, apply_rel)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
    if removed.is_empty() && modified.is_empty() && added.is_empty() {
        None
    } else {
        Some(DocxOpcRelationshipsDiff { removed, modified, added })
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_relationships(rels: &mut HashMap<String, Vec<OpcRelationship>>, diff: &DocxOpcRelationshipsDiff) -> MutationApplyResult<()> {
    let mut added = std::collections::HashSet::new();
    for owner in &diff.removed {
        if !rels.contains_key(owner) {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "relationship owner does not exist").at(vec!["removed".to_string(), owner.clone()]));
        }
        if !added.insert(owner) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "relationship owner is repeated").at(vec!["removed".to_string(), owner.clone()]));
        }
    }
    for modified in &diff.modified {
        if !rels.contains_key(&modified.key) {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "relationship owner does not exist").at(vec!["modified".to_string(), modified.key.clone()]));
        }
        if diff.removed.contains(&modified.key) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "relationship owner is removed and modified").at(vec!["modified".to_string(), modified.key.clone()]));
        }
    }
    for (owner, _) in &diff.added {
        if rels.contains_key(owner) || !added.insert(owner) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "relationship owner already exists").at(vec!["added".to_string(), owner.clone()]));
        }
    }
    for owner in &diff.removed {
        rels.remove(owner);
    }
    for m in &diff.modified {
        let list = rels.get_mut(&m.key).ok_or_else(|| semio_framework_plugin::resolve_ready(MutationApplyError::new("mutation.apply.missing-target", "relationship owner does not exist")).at(vec!["modified".to_string(), m.key.clone()]))?;
        apply_rel_list(list, &m.diff).map_err(|error| error.under(vec!["modified".to_string(), m.key.clone()]))?;
    }
    for (owner, list) in &diff.added {
        rels.insert(owner.clone(), list.clone());
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_relationships(d1: DocxOpcRelationshipsDiff, d2: DocxOpcRelationshipsDiff) -> DocxOpcRelationshipsDiff {
    absorb_named(d1, d2, |(owner, _)| owner.clone(), absorb_rel_list_diff, |(_, list), diff| *list = rel_list_with_diff_applied(list, diff))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_opc(base: &OpcPackage, other: &OpcPackage) -> Option<DocxOpcDiff> {
    let content_types = diff_content_types(&base.content_types, &other.content_types);
    let parts = diff_parts(&base.parts, &other.parts);
    let relationships = diff_relationships(&base.relationships, &other.relationships);
    if content_types.is_none() && parts.is_none() && relationships.is_none() {
        None
    } else {
        Some(DocxOpcDiff { content_types, parts, relationships })
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_opc_diff(opc: &mut OpcPackage, diff: &DocxOpcDiff) -> MutationApplyResult<()> {
    if let Some(d) = &diff.content_types {
        if let Some(dd) = &d.defaults {
            apply_ct_entries(&mut opc.content_types.defaults, dd).map_err(|error| error.under(["contentTypes", "defaults"]))?;
        }
        if let Some(dd) = &d.overrides {
            apply_ct_entries(&mut opc.content_types.overrides, dd).map_err(|error| error.under(["contentTypes", "overrides"]))?;
        }
    }
    if let Some(d) = &diff.parts {
        apply_named(
            &mut opc.parts,
            d,
            |p| p.path.clone(),
            |part, change| {
                apply_part(part, change);
                Ok(())
            },
        )
        .map_err(|error| error.under(["parts"]))?;
    }
    if let Some(d) = &diff.relationships {
        apply_relationships(&mut opc.relationships, d).map_err(|error| error.under(["relationships"]))?;
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_opc_diff(base: &OpcPackage, diff: &DocxOpcDiff) -> DocxOpcDiff {
    DocxOpcDiff {
        content_types: diff
            .content_types
            .as_ref()
            .map(|d| DocxOpcContentTypesDiff { defaults: d.defaults.as_ref().map(|dd| inverse_ct_entries(&base.content_types.defaults, dd)), overrides: d.overrides.as_ref().map(|dd| inverse_ct_entries(&base.content_types.overrides, dd)) }),
        parts: diff.parts.as_ref().map(|d| inverse_named(&base.parts, d, |p| p.path.clone(), inverse_part)),
        relationships: diff.relationships.as_ref().map(|d| inverse_relationships(&base.relationships, d)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
    async fn apply(&self, base: &DocxSnapshot) -> MutationApplyResult<DocxSnapshot> {
        let mut next = base.clone();
        if let Some(d) = &self.opc {
            apply_opc_diff(&mut next.opc, d).map_err(|error| error.under(["opc"]))?;
        }
        if let Some(d) = &self.document {
            apply_document_diff(&mut next.document, d).map_err(|error| error.under(["document"]))?;
        }
        Ok(next)
    }

    async fn absorb(&mut self, other: Self) {
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
    async fn inverse(&self, base: &DocxSnapshot) -> Self {
        DocxDiff { opc: self.opc.as_ref().map(|d| inverse_opc_diff(&base.opc, d)), document: self.document.as_ref().map(|d| inverse_document_diff(&base.document, d)) }
    }

    async fn between(base: &DocxSnapshot, other: &DocxSnapshot) -> Self {
        DocxDiff { opc: diff_opc(&base.opc, &other.opc), document: diff_document(&base.document, &other.document) }
    }

    async fn is_empty(&self) -> bool {
        self.opc.is_none() && self.document.is_none()
    }
}
//#endregion 🔖️DiffAlgebra

//#region 🔖️SetSnapshot
/// 🧩 Builds the sparse field-by-field diff for a `SetSnapshot` mutation. No `snapshot:
/// Option<DocxSnapshot>` full-replace slot -- this IS `DocxDiff::between`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_snapshot(base: &DocxSnapshot, next: &DocxSnapshot) -> DocxDiff {
    DocxDiff::between(base, next)
}

/// 🧩 Builds the diff for inserting `block` at `path` (`path.index` = insertion index, FINAL
/// state).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_block(path: &DocxBlockPath, block: DocxBlock) -> DocxDiff {
    wrap_body_diff(path, DocxBlockLeaf::Inserted(block))
}

/// 🧩 Builds the diff for removing the block at `path` (`path.index` = BASE-state index).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_block(path: &DocxBlockPath) -> DocxDiff {
    wrap_body_diff(path, DocxBlockLeaf::Removed)
}

/// 🧩 Builds the diff for replacing the block at `path` (BASE-state index) with `new_block`'s full
/// content, via a real structural comparison against `old_block` (never full-replace unless the
/// block KIND actually changed).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_block_content(path: &DocxBlockPath, old_block: &DocxBlock, new_block: &DocxBlock) -> DocxDiff {
    match diff_block(old_block, new_block) {
        None => DocxDiff::default(),
        Some(d) => wrap_body_diff(path, DocxBlockLeaf::Modified(d)),
    }
}

/// 🧩 Builds the diff for editing one run's text within the paragraph at `path`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_run_formatting(document: &DocxDocument, path: &DocxBlockPath, run_index: usize, bold: bool, italic: bool, underline: bool) -> DocxDiff {
    let Some(blocks) = resolve_blocks(&document.body, &path.segments) else { return DocxDiff::default() };
    let Some(DocxBlock::Paragraph(p)) = blocks.get(path.index) else { return DocxDiff::default() };
    let Some(run) = p.runs.get(run_index) else { return DocxDiff::default() };
    let run_diff = DocxRunDiff { text: None, bold: (run.bold != bold).then_some(bold), italic: (run.italic != italic).then_some(italic), underline: (run.underline != underline).then_some(underline) };
    if run_diff.bold.is_none() && run_diff.italic.is_none() && run_diff.underline.is_none() {
        return DocxDiff::default();
    }
    let runs_diff = DocxRunsDiff { modified: vec![IndexModified { index: run_index, diff: run_diff }], ..Default::default() };
    let block_diff = DocxBlockDiff::Paragraph(DocxParagraphDiff { runs: Some(runs_diff), style: None });
    wrap_body_diff(path, DocxBlockLeaf::Modified(block_diff))
}

/// 🧩 Builds the diff for inserting a style.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_style(style: DocxStyle) -> DocxDiff {
    DocxDiff { opc: None, document: Some(DocxDocumentDiff { body: None, styles: Some(DocxStylesDiff { added: vec![style], ..Default::default() }) }) }
}

/// 🧩 Builds the diff for removing a style by id.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_style(id: &str) -> DocxDiff {
    DocxDiff { opc: None, document: Some(DocxDocumentDiff { body: None, styles: Some(DocxStylesDiff { removed: vec![id.to_string()], ..Default::default() }) }) }
}

/// 🧩 Builds the diff for setting a style's name.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_style_name(id: &str, name: &str) -> DocxDiff {
    let sd = DocxStyleDiff { name: Some(name.to_string()), based_on: None };
    DocxDiff { opc: None, document: Some(DocxDocumentDiff { body: None, styles: Some(DocxStylesDiff { modified: vec![NamedModified { key: id.to_string(), diff: sd }], ..Default::default() }) }) }
}

/// 🧩 Builds the diff for setting (or clearing, `based_on: None`) a style's `based_on`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_style_based_on(id: &str, based_on: Option<String>) -> DocxDiff {
    let sd = DocxStyleDiff { name: None, based_on: Some(based_on) };
    DocxDiff { opc: None, document: Some(DocxDocumentDiff { body: None, styles: Some(DocxStylesDiff { modified: vec![NamedModified { key: id.to_string(), diff: sd }], ..Default::default() }) }) }
}

/// 🧩 Builds the diff for setting a raw OPC part (content this typed layer doesn't cover).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_part(opc: &OpcPackage, path: &str, content_type: &str, bytes: Vec<u8>) -> DocxDiff {
    let p = path.trim_start_matches('/').to_string();
    match opc.parts.iter().find(|part| part.path == p) {
        Some(existing) => {
            let new_part = OpcPart { path: p, content_type: content_type.to_string(), bytes };
            match diff_part(existing, &new_part) {
                None => DocxDiff::default(),
                Some(d) => {
                    DocxDiff { opc: Some(DocxOpcDiff { content_types: None, parts: Some(DocxOpcPartsDiff { modified: vec![NamedModified { key: existing.path.clone(), diff: d }], ..Default::default() }), relationships: None }), document: None }
                }
            }
        }
        None => DocxDiff { opc: Some(DocxOpcDiff { content_types: None, parts: Some(DocxOpcPartsDiff { added: vec![OpcPart { path: p, content_type: content_type.to_string(), bytes }], ..Default::default() }), relationships: None }), document: None },
    }
}

/// 🧩 Builds the diff for removing a raw OPC part by path.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_part(path: &str) -> DocxDiff {
    let p = path.trim_start_matches('/').to_string();
    DocxDiff { opc: Some(DocxOpcDiff { content_types: None, parts: Some(DocxOpcPartsDiff { removed: vec![p], ..Default::default() }), relationships: None }), document: None }
}
//#endregion 🔖️SetSnapshot

//#region 🔖️HandcraftedDiffCodec
/// 🧪️ F6: **hand-rolled** `protocol::DiffCodec` for `DocxDiff` (real compile errors captured above
/// on `DocxDiff`'s own doc comment) — same grammar style `GifDiff`/`SvgDiff`'s hand-rolled codecs
/// use (bracket-depth-aware split, hex for strings/bytes, `[0]`/`[1,x]` for `Option<T>`, a single
/// uppercase tag letter for data-carrying enums). This file re-derives its own copies of the small
/// helper functions since each hand-rolled codec is self-contained (no shared "hand-roll helpers"
/// module exists yet — flagged in `f6-recon-report.md` §5 as a good future extraction once ≥3
/// artifacts hand-roll, not worth adding here for one more). The `IndexedTripleDiff<D,T>`/
/// `NamedTripleDiff<K,D,T>` generic collection-triple engine this artifact already introduced
/// (see `GenericCollectionTriples` above) lets the codec side stay generic too — one
/// `enc_indexed_triple`/`enc_named_triple` pair, reused across every `body`/`runs`/table
/// row/cell/`styles`/OPC-parts/OPC-relationships instantiation, instead of five-plus bespoke
/// per-collection encoders.
//#region 🔖️Primitives
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_bool(b: &bool) -> String {
    if *b {
        "1".to_string()
    } else {
        "0".to_string()
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_bool(s: &str) -> Result<bool, String> {
    match s {
        "1" => Ok(true),
        "0" => Ok(false),
        other => Err(format!("bool: bad value {other:?}")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_usize(s: &str) -> Result<usize, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn split_top_level(s: &str, sep: char) -> Vec<&str> {
    if s.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut depth = 0i32;
    let mut start = 0usize;
    for (i, c) in s.char_indices() {
        match c {
            '[' => depth += 1,
            ']' => depth -= 1,
            c if c == sep && depth == 0 => {
                out.push(&s[start..i]);
                start = i + c.len_utf8();
            }
            _ => {}
        }
    }
    out.push(&s[start..]);
    out
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn strip_brackets(s: &str) -> Result<&str, String> {
    s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn encode_option<T>(opt: &Option<T>, enc: impl Fn(&T) -> String) -> String {
    match opt {
        None => "[0]".to_string(),
        Some(v) => format!("[1,{}]", enc(v)),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn decode_option<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Option<T>, String> {
    let inner = strip_brackets(s)?;
    match split_top_level(inner, ',').as_slice() {
        ["0"] => Ok(None),
        [tag, value] if *tag == "1" => Ok(Some(dec(value)?)),
        other => Err(format!("option decode: bad shape {other:?}")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_list<T>(items: &[T], enc: impl Fn(&T) -> String) -> String {
    format!("[{}]", items.iter().map(|i| enc(i)).collect::<Vec<_>>().join(","))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_list<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Vec<T>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec).collect()
}
//#endregion 🔖️Primitives

//#region 🔖️XmlValueCodecs
/// 🌳️ Recursive: `E[name,[attrs],[children]]` / `T[text]` / `D[text]` (CData) / `M[text]`
/// (comment) / `P[target,data]` (processing instruction) -- same tag scheme `📰xml`/`🎨️svg`'s own
/// hand-rolled codecs use (own copy per the no-shared-helpers-module convention). Needed here
/// because every `extra_*_properties: Vec<XmlNode>` raw-retention field (on `DocxRun`/
/// `DocxParagraph`/`DocxTableCell`/`DocxTableRow`/`DocxTable`) carries this type verbatim.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_attr(a: &XmlAttr) -> String {
    format!("[{},{}]", enc_str(&a.name), enc_str(&a.value))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_attr(s: &str) -> Result<XmlAttr, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, value] = parts.as_slice() else { return Err(format!("attr: expected 2 fields, got {}", parts.len())) };
    Ok(XmlAttr { name: dec_str(name)?, value: dec_str(value)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_xml_node(n: &XmlNode) -> String {
    match n {
        XmlNode::Element { name, attrs, children } => {
            let attrs = attrs.iter().map(enc_attr).collect::<Vec<_>>().join(",");
            let children = children.iter().map(enc_xml_node).collect::<Vec<_>>().join(",");
            format!("E[{},[{}],[{}]]", enc_str(name), attrs, children)
        }
        XmlNode::Text { text } => format!("T[{}]", enc_str(text)),
        XmlNode::CData { text } => format!("D[{}]", enc_str(text)),
        XmlNode::Comment { text } => format!("M[{}]", enc_str(text)),
        XmlNode::ProcessingInstruction { target, data } => format!("P[{},{}]", enc_str(target), enc_str(data)),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_xml_node(s: &str) -> Result<XmlNode, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "E" => {
            let parts = split_top_level(inner, ',');
            let [name, attrs, children] = parts.as_slice() else { return Err(format!("element: expected 3 fields, got {}", parts.len())) };
            let attrs = split_top_level(strip_brackets(attrs)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_attr).collect::<Result<Vec<_>, String>>()?;
            let children = split_top_level(strip_brackets(children)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_xml_node).collect::<Result<Vec<_>, String>>()?;
            Ok(XmlNode::Element { name: dec_str(name)?, attrs, children })
        }
        "T" => Ok(XmlNode::Text { text: dec_str(inner)? }),
        "D" => Ok(XmlNode::CData { text: dec_str(inner)? }),
        "M" => Ok(XmlNode::Comment { text: dec_str(inner)? }),
        "P" => {
            let parts = split_top_level(inner, ',');
            let [target, data] = parts.as_slice() else { return Err(format!("PI: expected 2 fields, got {}", parts.len())) };
            Ok(XmlNode::ProcessingInstruction { target: dec_str(target)?, data: dec_str(data)? })
        }
        other => Err(format!("xml node: unknown tag {other:?}")),
    }
}
//#endregion 🔖️XmlValueCodecs

//#region 🔖️ValueCodecs
/// 🌳️ Full-item (non-diff) codecs for every value type this diff's `added`/`Replace` payloads and
/// `SetSnapshot`'s mutation-side codec (see `mutations/component.rs`) carry whole. `pub(crate)` so
/// the mutations file reuses them rather than re-deriving its own copies (same intra-artifact reuse
/// pattern `SvgDiff`/`SvgMutation` established).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_target_mode(m: &OpcTargetMode) -> String {
    match m {
        OpcTargetMode::Internal => "0".to_string(),
        OpcTargetMode::External => "1".to_string(),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_target_mode(s: &str) -> Result<OpcTargetMode, String> {
    match s {
        "0" => Ok(OpcTargetMode::Internal),
        "1" => Ok(OpcTargetMode::External),
        other => Err(format!("target mode: bad value {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_run(r: &DocxRun) -> String {
    format!("[{},{},{},{},{}]", enc_str(&r.text), enc_bool(&r.bold), enc_bool(&r.italic), enc_bool(&r.underline), enc_list(&r.extra_run_properties, enc_xml_node))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_run(s: &str) -> Result<DocxRun, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [text, bold, italic, underline, extra] = parts.as_slice() else { return Err(format!("run: expected 5 fields, got {}", parts.len())) };
    Ok(DocxRun { text: dec_str(text)?, bold: dec_bool(bold)?, italic: dec_bool(italic)?, underline: dec_bool(underline)?, extra_run_properties: dec_list(extra, dec_xml_node)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_paragraph(p: &DocxParagraph) -> String {
    format!("[{},{},{}]", enc_list(&p.runs, enc_run), encode_option(&p.style, |v| enc_str(v)), enc_list(&p.extra_paragraph_properties, enc_xml_node))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_paragraph(s: &str) -> Result<DocxParagraph, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [runs, style, extra] = parts.as_slice() else { return Err(format!("paragraph: expected 3 fields, got {}", parts.len())) };
    Ok(DocxParagraph { runs: dec_list(runs, dec_run)?, style: decode_option(style, dec_str)?, extra_paragraph_properties: dec_list(extra, dec_xml_node)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_cell(c: &DocxTableCell) -> String {
    format!("[{},{}]", enc_list(&c.blocks, enc_block), enc_list(&c.extra_cell_properties, enc_xml_node))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_cell(s: &str) -> Result<DocxTableCell, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [blocks, extra] = parts.as_slice() else { return Err(format!("cell: expected 2 fields, got {}", parts.len())) };
    Ok(DocxTableCell { blocks: dec_list(blocks, dec_block)?, extra_cell_properties: dec_list(extra, dec_xml_node)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_row(r: &DocxTableRow) -> String {
    format!("[{},{}]", enc_list(&r.cells, enc_cell), enc_list(&r.extra_row_properties, enc_xml_node))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_row(s: &str) -> Result<DocxTableRow, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [cells, extra] = parts.as_slice() else { return Err(format!("row: expected 2 fields, got {}", parts.len())) };
    Ok(DocxTableRow { cells: dec_list(cells, dec_cell)?, extra_row_properties: dec_list(extra, dec_xml_node)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_table(t: &DocxTable) -> String {
    format!("[{},{}]", enc_list(&t.rows, enc_row), enc_list(&t.extra_table_properties, enc_xml_node))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_table(s: &str) -> Result<DocxTable, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [rows, extra] = parts.as_slice() else { return Err(format!("table: expected 2 fields, got {}", parts.len())) };
    Ok(DocxTable { rows: dec_list(rows, dec_row)?, extra_table_properties: dec_list(extra, dec_xml_node)? })
}

/// 🌳️ `P[paragraph]` / `T[table]` -- `DocxBlock`'s two variants, tag-prefixed like `enc_xml_node`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_block(b: &DocxBlock) -> String {
    match b {
        DocxBlock::Paragraph(p) => format!("P{}", enc_paragraph(p)),
        DocxBlock::Table(t) => format!("T{}", enc_table(t)),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_block(s: &str) -> Result<DocxBlock, String> {
    let (tag, rest) = s.split_at(1);
    match tag {
        "P" => Ok(DocxBlock::Paragraph(dec_paragraph(rest)?)),
        "T" => Ok(DocxBlock::Table(dec_table(rest)?)),
        other => Err(format!("block: unknown tag {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_style(s: &DocxStyle) -> String {
    format!("[{},{},{}]", enc_str(&s.id), enc_str(&s.name), encode_option(&s.based_on, |v| enc_str(v)))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_style(s: &str) -> Result<DocxStyle, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [id, name, based_on] = parts.as_slice() else { return Err(format!("style: expected 3 fields, got {}", parts.len())) };
    Ok(DocxStyle { id: dec_str(id)?, name: dec_str(name)?, based_on: decode_option(based_on, dec_str)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_opc_part(p: &OpcPart) -> String {
    format!("[{},{},{}]", enc_str(&p.path), enc_str(&p.content_type), hex_encode(&p.bytes))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_opc_part(s: &str) -> Result<OpcPart, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [path, content_type, bytes] = parts.as_slice() else { return Err(format!("opc part: expected 3 fields, got {}", parts.len())) };
    Ok(OpcPart { path: dec_str(path)?, content_type: dec_str(content_type)?, bytes: hex_decode(bytes)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_rel(r: &OpcRelationship) -> String {
    format!("[{},{},{},{}]", enc_str(&r.id), enc_str(&r.rel_type), enc_str(&r.target), enc_target_mode(&r.target_mode))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_rel(s: &str) -> Result<OpcRelationship, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [id, rel_type, target, target_mode] = parts.as_slice() else { return Err(format!("relationship: expected 4 fields, got {}", parts.len())) };
    Ok(OpcRelationship { id: dec_str(id)?, rel_type: dec_str(rel_type)?, target: dec_str(target)?, target_mode: dec_target_mode(target_mode)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_ct_entry(e: &(String, String)) -> String {
    format!("[{},{}]", enc_str(&e.0), enc_str(&e.1))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_ct_entry(s: &str) -> Result<(String, String), String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [k, v] = parts.as_slice() else { return Err(format!("ct entry: expected 2 fields, got {}", parts.len())) };
    Ok((dec_str(k)?, dec_str(v)?))
}

/// 🗺️ One `relationships` map entry (owner path -> that owner's relationship list).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_rel_owner_entry(e: &(String, Vec<OpcRelationship>)) -> String {
    format!("[{},{}]", enc_str(&e.0), enc_list(&e.1, enc_rel))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_rel_owner_entry(s: &str) -> Result<(String, Vec<OpcRelationship>), String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [owner, list] = parts.as_slice() else { return Err(format!("rel owner entry: expected 2 fields, got {}", parts.len())) };
    Ok((dec_str(owner)?, dec_list(list, dec_rel)?))
}
//#endregion 🔖️ValueCodecs

//#region 🔖️GenericTripleCodecs
/// 🌳️ `[removed];[modified];[added]` -- generic over `IndexedTripleDiff<D,T>`'s own `D`/`T`, reused
/// for every index-keyed collection (`body`/`runs`/table rows/cells) instead of one bespoke
/// encoder per collection.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_indexed_triple<D, T>(diff: &IndexedTripleDiff<D, T>, enc_d: impl Fn(&D) -> String, enc_t: impl Fn(&T) -> String) -> String {
    let removed = diff.removed.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let modified = diff.modified.iter().map(|m| format!("{}:{}", m.index, enc_d(&m.diff))).collect::<Vec<_>>().join(",");
    let added = diff.added.iter().map(|a| format!("{}:{}", a.index, enc_t(&a.item))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

/// 🏷️ `[removed];[modified];[added]` -- generic over `NamedTripleDiff<K,D,T>`'s own `K`/`D`/`T`,
/// reused for `styles` and every OPC-layer name-keyed collection (content-type entries, parts,
/// relationship lists, relationships-by-owner).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_named_triple<K, D, T>(diff: &NamedTripleDiff<K, D, T>, enc_k: impl Fn(&K) -> String, enc_d: impl Fn(&D) -> String, enc_t: impl Fn(&T) -> String) -> String {
    let removed = diff.removed.iter().map(|k| enc_k(k)).collect::<Vec<_>>().join(",");
    let modified = diff.modified.iter().map(|m| format!("{}:{}", enc_k(&m.key), enc_d(&m.diff))).collect::<Vec<_>>().join(",");
    let added = diff.added.iter().map(|t| enc_t(t)).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_named_triple<K, D, T>(body: &str, dec_k: impl Fn(&str) -> Result<K, String>, dec_d: impl Fn(&str) -> Result<D, String>, dec_t: impl Fn(&str) -> Result<T, String>) -> Result<NamedTripleDiff<K, D, T>, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("named triple: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|s| dec_k(s)).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (key, rest) = entry.split_once(':').ok_or_else(|| format!("named modified: bad entry {entry:?}"))?;
            Ok(NamedModified { key: dec_k(key)?, diff: dec_d(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|s| dec_t(s)).collect::<Result<Vec<_>, String>>()?;
    Ok(NamedTripleDiff { removed, modified, added })
}
//#endregion 🔖️GenericTripleCodecs

//#region 🔖️DiffValueCodecs
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_runs_diff(d: &DocxRunsDiff) -> String {
    enc_indexed_triple(d, enc_run_diff, enc_run)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_runs_diff(s: &str) -> Result<DocxRunsDiff, String> {
    dec_indexed_triple(s, dec_run_diff, dec_run)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_blocks_diff(d: &DocxBlocksDiff) -> String {
    enc_indexed_triple(d, enc_block_diff, enc_block)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_blocks_diff(s: &str) -> Result<DocxBlocksDiff, String> {
    dec_indexed_triple(s, dec_block_diff, dec_block)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_table_rows_diff(d: &DocxTableRowsDiff) -> String {
    enc_indexed_triple(d, enc_table_row_diff, enc_row)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_table_rows_diff(s: &str) -> Result<DocxTableRowsDiff, String> {
    dec_indexed_triple(s, dec_table_row_diff, dec_row)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_table_cells_diff(d: &DocxTableCellsDiff) -> String {
    enc_indexed_triple(d, enc_table_cell_diff, enc_cell)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_table_cells_diff(s: &str) -> Result<DocxTableCellsDiff, String> {
    dec_indexed_triple(s, dec_table_cell_diff, dec_cell)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_styles_diff(d: &DocxStylesDiff) -> String {
    enc_named_triple(d, |k| enc_str(k), enc_style_diff, enc_style)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_styles_diff(s: &str) -> Result<DocxStylesDiff, String> {
    dec_named_triple(s, dec_str, dec_style_diff, dec_style)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_run_diff(d: &DocxRunDiff) -> String {
    format!("[{},{},{},{}]", encode_option(&d.text, |v| enc_str(v)), encode_option(&d.bold, enc_bool), encode_option(&d.italic, enc_bool), encode_option(&d.underline, enc_bool))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_run_diff(s: &str) -> Result<DocxRunDiff, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [text, bold, italic, underline] = parts.as_slice() else { return Err(format!("run diff: expected 4 fields, got {}", parts.len())) };
    Ok(DocxRunDiff { text: decode_option(text, dec_str)?, bold: decode_option(bold, dec_bool)?, italic: decode_option(italic, dec_bool)?, underline: decode_option(underline, dec_bool)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_paragraph_diff(pd: &DocxParagraphDiff) -> String {
    format!("[{},{}]", encode_option(&pd.runs, enc_runs_diff), encode_option(&pd.style, |inner: &Option<String>| encode_option(inner, |v| enc_str(v))))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_paragraph_diff(s: &str) -> Result<DocxParagraphDiff, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [runs, style] = parts.as_slice() else { return Err(format!("paragraph diff: expected 2 fields, got {}", parts.len())) };
    Ok(DocxParagraphDiff { runs: decode_option(runs, dec_runs_diff)?, style: decode_option(style, |s| decode_option(s, dec_str))? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_table_diff(d: &DocxTableDiff) -> String {
    format!("[{}]", encode_option(&d.rows, enc_table_rows_diff))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_table_diff(s: &str) -> Result<DocxTableDiff, String> {
    let inner = strip_brackets(s)?;
    Ok(DocxTableDiff { rows: decode_option(inner, dec_table_rows_diff)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_table_row_diff(d: &DocxTableRowDiff) -> String {
    format!("[{}]", encode_option(&d.cells, enc_table_cells_diff))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_table_row_diff(s: &str) -> Result<DocxTableRowDiff, String> {
    let inner = strip_brackets(s)?;
    Ok(DocxTableRowDiff { cells: decode_option(inner, dec_table_cells_diff)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_table_cell_diff(d: &DocxTableCellDiff) -> String {
    format!("[{}]", encode_option(&d.blocks, enc_blocks_diff))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_table_cell_diff(s: &str) -> Result<DocxTableCellDiff, String> {
    let inner = strip_brackets(s)?;
    Ok(DocxTableCellDiff { blocks: decode_option(inner, dec_blocks_diff)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_style_diff(d: &DocxStyleDiff) -> String {
    format!("[{},{}]", encode_option(&d.name, |v| enc_str(v)), encode_option(&d.based_on, |inner: &Option<String>| encode_option(inner, |v| enc_str(v))))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_style_diff(s: &str) -> Result<DocxStyleDiff, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [name, based_on] = parts.as_slice() else { return Err(format!("style diff: expected 2 fields, got {}", parts.len())) };
    Ok(DocxStyleDiff { name: decode_option(name, dec_str)?, based_on: decode_option(based_on, |s| decode_option(s, dec_str))? })
}

/// 🌳️ `P[paragraph diff]` / `T[table diff]` / `R[block]` (wholesale replace, node-KIND changed).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_block_diff(d: &DocxBlockDiff) -> String {
    match d {
        DocxBlockDiff::Paragraph(pd) => format!("P{}", enc_paragraph_diff(pd)),
        DocxBlockDiff::Table(td) => format!("T{}", enc_table_diff(td)),
        DocxBlockDiff::Replace { block } => format!("R[{}]", enc_block(block)),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_block_diff(s: &str) -> Result<DocxBlockDiff, String> {
    let (tag, rest) = s.split_at(1);
    match tag {
        "P" => Ok(DocxBlockDiff::Paragraph(dec_paragraph_diff(rest)?)),
        "T" => Ok(DocxBlockDiff::Table(dec_table_diff(rest)?)),
        "R" => Ok(DocxBlockDiff::Replace { block: dec_block(strip_brackets(rest)?)? }),
        other => Err(format!("block diff: unknown tag {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_ct_entries_diff(d: &DocxOpcCtEntriesDiff) -> String {
    enc_named_triple(d, |k| enc_str(k), |v: &String| enc_str(v), enc_ct_entry)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_ct_entries_diff(s: &str) -> Result<DocxOpcCtEntriesDiff, String> {
    dec_named_triple(s, dec_str, dec_str, dec_ct_entry)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_parts_diff(d: &DocxOpcPartsDiff) -> String {
    enc_named_triple(d, |k| enc_str(k), enc_opc_part_diff, enc_opc_part)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_parts_diff(s: &str) -> Result<DocxOpcPartsDiff, String> {
    dec_named_triple(s, dec_str, dec_opc_part_diff, dec_opc_part)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_rel_list_diff(d: &DocxOpcRelListDiff) -> String {
    enc_named_triple(d, |k| enc_str(k), enc_rel_diff, enc_rel)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_rel_list_diff(s: &str) -> Result<DocxOpcRelListDiff, String> {
    dec_named_triple(s, dec_str, dec_rel_diff, dec_rel)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_relationships_diff(d: &DocxOpcRelationshipsDiff) -> String {
    enc_named_triple(d, |k| enc_str(k), enc_rel_list_diff, enc_rel_owner_entry)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_relationships_diff(s: &str) -> Result<DocxOpcRelationshipsDiff, String> {
    dec_named_triple(s, dec_str, dec_rel_list_diff, dec_rel_owner_entry)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_opc_part_diff(d: &DocxOpcPartDiff) -> String {
    format!("[{},{}]", encode_option(&d.content_type, |v| enc_str(v)), encode_option(&d.bytes, |v: &Vec<u8>| hex_encode(v)))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_opc_part_diff(s: &str) -> Result<DocxOpcPartDiff, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [ct, bytes] = parts.as_slice() else { return Err(format!("opc part diff: expected 2 fields, got {}", parts.len())) };
    Ok(DocxOpcPartDiff { content_type: decode_option(ct, dec_str)?, bytes: decode_option(bytes, hex_decode)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_rel_diff(d: &DocxOpcRelDiff) -> String {
    format!("[{},{},{}]", encode_option(&d.rel_type, |v| enc_str(v)), encode_option(&d.target, |v| enc_str(v)), encode_option(&d.target_mode, enc_target_mode))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_rel_diff(s: &str) -> Result<DocxOpcRelDiff, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [rel_type, target, target_mode] = parts.as_slice() else { return Err(format!("rel diff: expected 3 fields, got {}", parts.len())) };
    Ok(DocxOpcRelDiff { rel_type: decode_option(rel_type, dec_str)?, target: decode_option(target, dec_str)?, target_mode: decode_option(target_mode, dec_target_mode)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_content_types_diff(d: &DocxOpcContentTypesDiff) -> String {
    format!("[{},{}]", encode_option(&d.defaults, enc_ct_entries_diff), encode_option(&d.overrides, enc_ct_entries_diff))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_content_types_diff(s: &str) -> Result<DocxOpcContentTypesDiff, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [defaults, overrides] = parts.as_slice() else { return Err(format!("content types diff: expected 2 fields, got {}", parts.len())) };
    Ok(DocxOpcContentTypesDiff { defaults: decode_option(defaults, dec_ct_entries_diff)?, overrides: decode_option(overrides, dec_ct_entries_diff)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_opc_diff(d: &DocxOpcDiff) -> String {
    format!("[{},{},{}]", encode_option(&d.content_types, enc_content_types_diff), encode_option(&d.parts, enc_parts_diff), encode_option(&d.relationships, enc_relationships_diff))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_opc_diff(s: &str) -> Result<DocxOpcDiff, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [ct, p, rel] = parts.as_slice() else { return Err(format!("opc diff: expected 3 fields, got {}", parts.len())) };
    Ok(DocxOpcDiff { content_types: decode_option(ct, dec_content_types_diff)?, parts: decode_option(p, dec_parts_diff)?, relationships: decode_option(rel, dec_relationships_diff)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_document_diff(d: &DocxDocumentDiff) -> String {
    format!("[{},{}]", encode_option(&d.body, enc_blocks_diff), encode_option(&d.styles, enc_styles_diff))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_document_diff(s: &str) -> Result<DocxDocumentDiff, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [body, styles] = parts.as_slice() else { return Err(format!("document diff: expected 2 fields, got {}", parts.len())) };
    Ok(DocxDocumentDiff { body: decode_option(body, dec_blocks_diff)?, styles: decode_option(styles, dec_styles_diff)? })
}
//#endregion 🔖️DiffValueCodecs

//#region 🔖️BinaryCodecs
/// 🧪️ FG-wave: real recursive BINARY twins of every text-form codec above, backing the upgraded
/// `DiffCodec::encode_diff`/`decode_diff` below (and, via re-export, `../🧬️mutations/🦀️component.rs`'s
/// own upgraded `OpBinary`) — replaces F6's `print_diff().into_bytes()` text-as-binary shortcut.
/// Real LEB128-varint-framed length-prefixed strings/bytes (`store::pack_rt::write_varint_u64` +
/// `store::ByteReader`), 1-byte tri-state presence tags, and 1-byte enum-variant tags — genuinely
/// structured binary, never hex-ASCII text reused as "binary". Same shape
/// `📰xml/…/🔺️diff/🦀️component.rs`'s own `BinaryPrimitives`/`XmlValueBinaryCodecs`/
/// `DiffValueBinaryCodecs` regions establish; duplicated here (not imported) per this repo's
/// per-artifact hand-roll convention (no shared "hand-roll helpers" module exists yet, see this
/// file's own `HandcraftedDiffCodec` doc comment).
//#region 🔖️BinaryPrimitives
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}
//#endregion 🔖️BinaryPrimitives

//#region 🔖️XmlValueBinaryCodecs
/// 🌳️ Binary twin of `enc_xml_node`/`dec_xml_node` -- 1-byte kind tag (`0`=Element/`1`=Text/
/// `2`=CData/`3`=Comment/`4`=ProcessingInstruction, matching xml's own binary tag numbering).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_attr_bin(a: &XmlAttr, out: &mut Vec<u8>) {
    write_str_lp(out, &a.name);
    write_str_lp(out, &a.value);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_attr_bin(reader: &mut store::ByteReader<'_>) -> Result<XmlAttr, String> {
    let name = read_str_lp(reader)?;
    let value = read_str_lp(reader)?;
    Ok(XmlAttr { name, value })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_xml_node_bin(node: &XmlNode, out: &mut Vec<u8>) {
    match node {
        XmlNode::Element { name, attrs, children } => {
            out.push(0);
            write_str_lp(out, name);
            store::pack_rt::write_varint_u64(out, attrs.len() as u64);
            for attr in attrs {
                enc_attr_bin(attr, out);
            }
            store::pack_rt::write_varint_u64(out, children.len() as u64);
            for child in children {
                enc_xml_node_bin(child, out);
            }
        }
        XmlNode::Text { text } => {
            out.push(1);
            write_str_lp(out, text);
        }
        XmlNode::CData { text } => {
            out.push(2);
            write_str_lp(out, text);
        }
        XmlNode::Comment { text } => {
            out.push(3);
            write_str_lp(out, text);
        }
        XmlNode::ProcessingInstruction { target, data } => {
            out.push(4);
            write_str_lp(out, target);
            write_str_lp(out, data);
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_xml_node_bin(reader: &mut store::ByteReader<'_>) -> Result<XmlNode, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => {
            let name = read_str_lp(reader)?;
            let attr_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
            let mut attrs = Vec::with_capacity(attr_count as usize);
            for _ in 0..attr_count {
                attrs.push(dec_attr_bin(reader)?);
            }
            let child_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
            let mut children = Vec::with_capacity(child_count as usize);
            for _ in 0..child_count {
                children.push(Box::pin(dec_xml_node_bin(reader))?);
            }
            Ok(XmlNode::Element { name, attrs, children })
        }
        1 => Ok(XmlNode::Text { text: read_str_lp(reader)? }),
        2 => Ok(XmlNode::CData { text: read_str_lp(reader)? }),
        3 => Ok(XmlNode::Comment { text: read_str_lp(reader)? }),
        4 => {
            let target = read_str_lp(reader)?;
            let data = read_str_lp(reader)?;
            Ok(XmlNode::ProcessingInstruction { target, data })
        }
        other => Err(format!("xml node binary: unknown tag {other}")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_xml_node_list_bin(nodes: &[XmlNode], out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, nodes.len() as u64);
    for n in nodes {
        enc_xml_node_bin(n, out);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_xml_node_list_bin(reader: &mut store::ByteReader<'_>) -> Result<Vec<XmlNode>, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut out = Vec::with_capacity(count as usize);
    for _ in 0..count {
        out.push(dec_xml_node_bin(reader)?);
    }
    Ok(out)
}
//#endregion 🔖️XmlValueBinaryCodecs

//#region 🔖️ValueBinaryCodecs
/// 🌳️ Full-item (non-diff) binary codecs, mirrored one-for-one against `../🔖️ValueCodecs`'s text
/// forms above. `pub(crate)` so `../🧬️mutations/🦀️component.rs` reuses these rather than
/// re-deriving its own copies (same intra-artifact reuse pattern the text codecs already use).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_target_mode_bin(m: &OpcTargetMode, out: &mut Vec<u8>) {
    out.push(match m {
        OpcTargetMode::Internal => 0,
        OpcTargetMode::External => 1,
    });
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_target_mode_bin(reader: &mut store::ByteReader<'_>) -> Result<OpcTargetMode, String> {
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => Ok(OpcTargetMode::Internal),
        1 => Ok(OpcTargetMode::External),
        other => Err(format!("target mode binary: bad value {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_run_bin(r: &DocxRun, out: &mut Vec<u8>) {
    write_str_lp(out, &r.text);
    out.push(r.bold as u8);
    out.push(r.italic as u8);
    out.push(r.underline as u8);
    enc_xml_node_list_bin(&r.extra_run_properties, out);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_run_bin(reader: &mut store::ByteReader<'_>) -> Result<DocxRun, String> {
    let text = read_str_lp(reader)?;
    let bold = reader.read_u8().map_err(|e| e.to_string())? != 0;
    let italic = reader.read_u8().map_err(|e| e.to_string())? != 0;
    let underline = reader.read_u8().map_err(|e| e.to_string())? != 0;
    let extra_run_properties = dec_xml_node_list_bin(reader)?;
    Ok(DocxRun { text, bold, italic, underline, extra_run_properties })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_paragraph_bin(p: &DocxParagraph, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, p.runs.len() as u64);
    for r in &p.runs {
        enc_run_bin(r, out);
    }
    out.push(if p.style.is_some() { 1 } else { 0 });
    if let Some(style) = &p.style {
        write_str_lp(out, style);
    }
    enc_xml_node_list_bin(&p.extra_paragraph_properties, out);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_paragraph_bin(reader: &mut store::ByteReader<'_>) -> Result<DocxParagraph, String> {
    let run_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut runs = Vec::with_capacity(run_count as usize);
    for _ in 0..run_count {
        runs.push(dec_run_bin(reader)?);
    }
    let style = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(read_str_lp(reader)?) } else { None };
    let extra_paragraph_properties = dec_xml_node_list_bin(reader)?;
    Ok(DocxParagraph { runs, style, extra_paragraph_properties })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_cell_bin(c: &DocxTableCell, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, c.blocks.len() as u64);
    for b in &c.blocks {
        enc_block_bin(b, out);
    }
    enc_xml_node_list_bin(&c.extra_cell_properties, out);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_cell_bin(reader: &mut store::ByteReader<'_>) -> Result<DocxTableCell, String> {
    let block_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut blocks = Vec::with_capacity(block_count as usize);
    for _ in 0..block_count {
        blocks.push(Box::pin(dec_block_bin(reader))?);
    }
    let extra_cell_properties = dec_xml_node_list_bin(reader)?;
    Ok(DocxTableCell { blocks, extra_cell_properties })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_row_bin(r: &DocxTableRow, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, r.cells.len() as u64);
    for c in &r.cells {
        enc_cell_bin(c, out);
    }
    enc_xml_node_list_bin(&r.extra_row_properties, out);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_row_bin(reader: &mut store::ByteReader<'_>) -> Result<DocxTableRow, String> {
    let cell_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut cells = Vec::with_capacity(cell_count as usize);
    for _ in 0..cell_count {
        cells.push(Box::pin(dec_cell_bin(reader))?);
    }
    let extra_row_properties = dec_xml_node_list_bin(reader)?;
    Ok(DocxTableRow { cells, extra_row_properties })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_table_bin(t: &DocxTable, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, t.rows.len() as u64);
    for r in &t.rows {
        enc_row_bin(r, out);
    }
    enc_xml_node_list_bin(&t.extra_table_properties, out);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_table_bin(reader: &mut store::ByteReader<'_>) -> Result<DocxTable, String> {
    let row_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut rows = Vec::with_capacity(row_count as usize);
    for _ in 0..row_count {
        rows.push(Box::pin(dec_row_bin(reader))?);
    }
    let extra_table_properties = dec_xml_node_list_bin(reader)?;
    Ok(DocxTable { rows, extra_table_properties })
}

/// 🌳️ `0`=Paragraph / `1`=Table -- `DocxBlock`'s two variants, tag-prefixed like `enc_xml_node_bin`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_block_bin(b: &DocxBlock, out: &mut Vec<u8>) {
    match b {
        DocxBlock::Paragraph(p) => {
            out.push(0);
            enc_paragraph_bin(p, out);
        }
        DocxBlock::Table(t) => {
            out.push(1);
            enc_table_bin(t, out);
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_block_bin(reader: &mut store::ByteReader<'_>) -> Result<DocxBlock, String> {
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => Ok(DocxBlock::Paragraph(dec_paragraph_bin(reader)?)),
        1 => Ok(DocxBlock::Table(Box::pin(dec_table_bin(reader))?)),
        other => Err(format!("block binary: unknown tag {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_style_bin(s: &DocxStyle, out: &mut Vec<u8>) {
    write_str_lp(out, &s.id);
    write_str_lp(out, &s.name);
    out.push(if s.based_on.is_some() { 1 } else { 0 });
    if let Some(based_on) = &s.based_on {
        write_str_lp(out, based_on);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_style_bin(reader: &mut store::ByteReader<'_>) -> Result<DocxStyle, String> {
    let id = read_str_lp(reader)?;
    let name = read_str_lp(reader)?;
    let based_on = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(read_str_lp(reader)?) } else { None };
    Ok(DocxStyle { id, name, based_on })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_opc_part_bin(p: &OpcPart, out: &mut Vec<u8>) {
    write_str_lp(out, &p.path);
    write_str_lp(out, &p.content_type);
    write_bytes_lp(out, &p.bytes);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_opc_part_bin(reader: &mut store::ByteReader<'_>) -> Result<OpcPart, String> {
    let path = read_str_lp(reader)?;
    let content_type = read_str_lp(reader)?;
    let bytes = read_bytes_lp(reader)?;
    Ok(OpcPart { path, content_type, bytes })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_rel_bin(r: &OpcRelationship, out: &mut Vec<u8>) {
    write_str_lp(out, &r.id);
    write_str_lp(out, &r.rel_type);
    write_str_lp(out, &r.target);
    enc_target_mode_bin(&r.target_mode, out);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_rel_bin(reader: &mut store::ByteReader<'_>) -> Result<OpcRelationship, String> {
    let id = read_str_lp(reader)?;
    let rel_type = read_str_lp(reader)?;
    let target = read_str_lp(reader)?;
    let target_mode = dec_target_mode_bin(reader)?;
    Ok(OpcRelationship { id, rel_type, target, target_mode })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_ct_entry_bin(e: &(String, String), out: &mut Vec<u8>) {
    write_str_lp(out, &e.0);
    write_str_lp(out, &e.1);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_ct_entry_bin(reader: &mut store::ByteReader<'_>) -> Result<(String, String), String> {
    let k = read_str_lp(reader)?;
    let v = read_str_lp(reader)?;
    Ok((k, v))
}

/// 🗺️ One `relationships` map entry (owner path -> that owner's relationship list).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_rel_owner_entry_bin(e: &(String, Vec<OpcRelationship>), out: &mut Vec<u8>) {
    write_str_lp(out, &e.0);
    store::pack_rt::write_varint_u64(out, e.1.len() as u64);
    for r in &e.1 {
        enc_rel_bin(r, out);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_rel_owner_entry_bin(reader: &mut store::ByteReader<'_>) -> Result<(String, Vec<OpcRelationship>), String> {
    let owner = read_str_lp(reader)?;
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut list = Vec::with_capacity(count as usize);
    for _ in 0..count {
        list.push(dec_rel_bin(reader)?);
    }
    Ok((owner, list))
}
//#endregion 🔖️ValueBinaryCodecs

//#region 🔖️GenericTripleBinaryCodecs
/// 🌳️ Binary twin of `enc_indexed_triple`/`dec_indexed_triple` -- three varint-counted sections
/// (removed indices / modified index+diff pairs / added index+item pairs), generic over `D`/`T`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_indexed_triple_bin<D, T>(diff: &IndexedTripleDiff<D, T>, enc_d: impl Fn(&D, &mut Vec<u8>), enc_t: impl Fn(&T, &mut Vec<u8>), out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, diff.removed.len() as u64);
    for i in &diff.removed {
        store::pack_rt::write_varint_u64(out, *i as u64);
    }
    store::pack_rt::write_varint_u64(out, diff.modified.len() as u64);
    for m in &diff.modified {
        store::pack_rt::write_varint_u64(out, m.index as u64);
        enc_d(&m.diff, out);
    }
    store::pack_rt::write_varint_u64(out, diff.added.len() as u64);
    for a in &diff.added {
        store::pack_rt::write_varint_u64(out, a.index as u64);
        enc_t(&a.item, out);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_indexed_triple_bin<D, T>(reader: &mut store::ByteReader<'_>, dec_d: impl Fn(&mut store::ByteReader<'_>) -> Result<D, String>, dec_t: impl Fn(&mut store::ByteReader<'_>) -> Result<T, String>) -> Result<IndexedTripleDiff<D, T>, String> {
    let removed_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut removed = Vec::with_capacity(removed_count as usize);
    for _ in 0..removed_count {
        removed.push(reader.read_varint_u64().map_err(|e| e.to_string())? as usize);
    }
    let modified_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(modified_count as usize);
    for _ in 0..modified_count {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let diff = dec_d(reader)?;
        modified.push(IndexModified { index, diff });
    }
    let added_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(added_count as usize);
    for _ in 0..added_count {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let item = dec_t(reader)?;
        added.push(IndexAdded { index, item });
    }
    Ok(IndexedTripleDiff { removed, modified, added })
}

/// 🏷️ Binary twin of `enc_named_triple`/`dec_named_triple` -- three varint-counted sections
/// (removed keys / modified key+diff pairs / added whole items), generic over `K`/`D`/`T`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_named_triple_bin<K, D, T>(diff: &NamedTripleDiff<K, D, T>, enc_k: impl Fn(&K, &mut Vec<u8>), enc_d: impl Fn(&D, &mut Vec<u8>), enc_t: impl Fn(&T, &mut Vec<u8>), out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, diff.removed.len() as u64);
    for k in &diff.removed {
        enc_k(k, out);
    }
    store::pack_rt::write_varint_u64(out, diff.modified.len() as u64);
    for m in &diff.modified {
        enc_k(&m.key, out);
        enc_d(&m.diff, out);
    }
    store::pack_rt::write_varint_u64(out, diff.added.len() as u64);
    for t in &diff.added {
        enc_t(t, out);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_named_triple_bin<K, D, T>(
    reader: &mut store::ByteReader<'_>,
    dec_k: impl Fn(&mut store::ByteReader<'_>) -> Result<K, String>,
    dec_d: impl Fn(&mut store::ByteReader<'_>) -> Result<D, String>,
    dec_t: impl Fn(&mut store::ByteReader<'_>) -> Result<T, String>,
) -> Result<NamedTripleDiff<K, D, T>, String> {
    let removed_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut removed = Vec::with_capacity(removed_count as usize);
    for _ in 0..removed_count {
        removed.push(dec_k(reader)?);
    }
    let modified_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(modified_count as usize);
    for _ in 0..modified_count {
        let key = dec_k(reader)?;
        let diff = dec_d(reader)?;
        modified.push(NamedModified { key, diff });
    }
    let added_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(added_count as usize);
    for _ in 0..added_count {
        added.push(dec_t(reader)?);
    }
    Ok(NamedTripleDiff { removed, modified, added })
}
//#endregion 🔖️GenericTripleBinaryCodecs

//#region 🔖️DiffValueBinaryCodecs
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_runs_diff_bin(d: &DocxRunsDiff, out: &mut Vec<u8>) {
    enc_indexed_triple_bin(d, enc_run_diff_bin, enc_run_bin, out)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_runs_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<DocxRunsDiff, String> {
    dec_indexed_triple_bin(reader, dec_run_diff_bin, dec_run_bin)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_blocks_diff_bin(d: &DocxBlocksDiff, out: &mut Vec<u8>) {
    enc_indexed_triple_bin(d, enc_block_diff_bin, enc_block_bin, out)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_blocks_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<DocxBlocksDiff, String> {
    dec_indexed_triple_bin(reader, dec_block_diff_bin, dec_block_bin)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_table_rows_diff_bin(d: &DocxTableRowsDiff, out: &mut Vec<u8>) {
    enc_indexed_triple_bin(d, enc_table_row_diff_bin, enc_row_bin, out)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_table_rows_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<DocxTableRowsDiff, String> {
    dec_indexed_triple_bin(reader, dec_table_row_diff_bin, dec_row_bin)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_table_cells_diff_bin(d: &DocxTableCellsDiff, out: &mut Vec<u8>) {
    enc_indexed_triple_bin(d, enc_table_cell_diff_bin, enc_cell_bin, out)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_table_cells_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<DocxTableCellsDiff, String> {
    dec_indexed_triple_bin(reader, dec_table_cell_diff_bin, dec_cell_bin)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_styles_diff_bin(d: &DocxStylesDiff, out: &mut Vec<u8>) {
    enc_named_triple_bin(d, |k, out| write_str_lp(out, k), enc_style_diff_bin, enc_style_bin, out)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_styles_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<DocxStylesDiff, String> {
    dec_named_triple_bin(reader, |r| read_str_lp(r), dec_style_diff_bin, dec_style_bin)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_run_diff_bin(d: &DocxRunDiff, out: &mut Vec<u8>) {
    out.push(if d.text.is_some() { 1 } else { 0 });
    if let Some(v) = &d.text {
        write_str_lp(out, v);
    }
    out.push(if d.bold.is_some() { 1 } else { 0 });
    if let Some(v) = d.bold {
        out.push(v as u8);
    }
    out.push(if d.italic.is_some() { 1 } else { 0 });
    if let Some(v) = d.italic {
        out.push(v as u8);
    }
    out.push(if d.underline.is_some() { 1 } else { 0 });
    if let Some(v) = d.underline {
        out.push(v as u8);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_run_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<DocxRunDiff, String> {
    let text = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(read_str_lp(reader)?) } else { None };
    let bold = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(reader.read_u8().map_err(|e| e.to_string())? != 0) } else { None };
    let italic = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(reader.read_u8().map_err(|e| e.to_string())? != 0) } else { None };
    let underline = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(reader.read_u8().map_err(|e| e.to_string())? != 0) } else { None };
    Ok(DocxRunDiff { text, bold, italic, underline })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_paragraph_diff_bin(pd: &DocxParagraphDiff, out: &mut Vec<u8>) {
    out.push(if pd.runs.is_some() { 1 } else { 0 });
    if let Some(runs) = &pd.runs {
        enc_runs_diff_bin(runs, out);
    }
    out.push(if pd.style.is_some() { 1 } else { 0 });
    if let Some(style_opt) = &pd.style {
        out.push(if style_opt.is_some() { 1 } else { 0 });
        if let Some(v) = style_opt {
            write_str_lp(out, v);
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_paragraph_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<DocxParagraphDiff, String> {
    let runs = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_runs_diff_bin(reader)?) } else { None };
    let style = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(read_str_lp(reader)?) } else { None }) } else { None };
    Ok(DocxParagraphDiff { runs, style })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_table_diff_bin(d: &DocxTableDiff, out: &mut Vec<u8>) {
    out.push(if d.rows.is_some() { 1 } else { 0 });
    if let Some(rows) = &d.rows {
        enc_table_rows_diff_bin(rows, out);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_table_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<DocxTableDiff, String> {
    let rows = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_table_rows_diff_bin(reader)?) } else { None };
    Ok(DocxTableDiff { rows })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_table_row_diff_bin(d: &DocxTableRowDiff, out: &mut Vec<u8>) {
    out.push(if d.cells.is_some() { 1 } else { 0 });
    if let Some(cells) = &d.cells {
        enc_table_cells_diff_bin(cells, out);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_table_row_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<DocxTableRowDiff, String> {
    let cells = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_table_cells_diff_bin(reader)?) } else { None };
    Ok(DocxTableRowDiff { cells })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_table_cell_diff_bin(d: &DocxTableCellDiff, out: &mut Vec<u8>) {
    out.push(if d.blocks.is_some() { 1 } else { 0 });
    if let Some(blocks) = &d.blocks {
        enc_blocks_diff_bin(blocks, out);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_table_cell_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<DocxTableCellDiff, String> {
    let blocks = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_blocks_diff_bin(reader)?) } else { None };
    Ok(DocxTableCellDiff { blocks })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_style_diff_bin(d: &DocxStyleDiff, out: &mut Vec<u8>) {
    out.push(if d.name.is_some() { 1 } else { 0 });
    if let Some(v) = &d.name {
        write_str_lp(out, v);
    }
    out.push(if d.based_on.is_some() { 1 } else { 0 });
    if let Some(inner) = &d.based_on {
        out.push(if inner.is_some() { 1 } else { 0 });
        if let Some(v) = inner {
            write_str_lp(out, v);
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_style_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<DocxStyleDiff, String> {
    let name = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(read_str_lp(reader)?) } else { None };
    let based_on = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(read_str_lp(reader)?) } else { None }) } else { None };
    Ok(DocxStyleDiff { name, based_on })
}

/// 🌳️ `0`=Paragraph / `1`=Table / `2`=Replace (wholesale replace, block KIND changed) -- binary
/// twin of `enc_block_diff`/`dec_block_diff`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_block_diff_bin(d: &DocxBlockDiff, out: &mut Vec<u8>) {
    match d {
        DocxBlockDiff::Paragraph(pd) => {
            out.push(0);
            enc_paragraph_diff_bin(pd, out);
        }
        DocxBlockDiff::Table(td) => {
            out.push(1);
            enc_table_diff_bin(td, out);
        }
        DocxBlockDiff::Replace { block } => {
            out.push(2);
            enc_block_bin(block, out);
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_block_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<DocxBlockDiff, String> {
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => Ok(DocxBlockDiff::Paragraph(dec_paragraph_diff_bin(reader)?)),
        1 => Ok(DocxBlockDiff::Table(dec_table_diff_bin(reader)?)),
        2 => Ok(DocxBlockDiff::Replace { block: dec_block_bin(reader)? }),
        other => Err(format!("block diff binary: unknown tag {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_ct_entries_diff_bin(d: &DocxOpcCtEntriesDiff, out: &mut Vec<u8>) {
    enc_named_triple_bin(d, |k, out| write_str_lp(out, k), |v: &String, out| write_str_lp(out, v), enc_ct_entry_bin, out)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_ct_entries_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<DocxOpcCtEntriesDiff, String> {
    dec_named_triple_bin(reader, |r| read_str_lp(r), |r| read_str_lp(r), dec_ct_entry_bin)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_opc_part_diff_bin(d: &DocxOpcPartDiff, out: &mut Vec<u8>) {
    out.push(if d.content_type.is_some() { 1 } else { 0 });
    if let Some(v) = &d.content_type {
        write_str_lp(out, v);
    }
    out.push(if d.bytes.is_some() { 1 } else { 0 });
    if let Some(v) = &d.bytes {
        write_bytes_lp(out, v);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_opc_part_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<DocxOpcPartDiff, String> {
    let content_type = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(read_str_lp(reader)?) } else { None };
    let bytes = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(read_bytes_lp(reader)?) } else { None };
    Ok(DocxOpcPartDiff { content_type, bytes })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_parts_diff_bin(d: &DocxOpcPartsDiff, out: &mut Vec<u8>) {
    enc_named_triple_bin(d, |k, out| write_str_lp(out, k), enc_opc_part_diff_bin, enc_opc_part_bin, out)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_parts_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<DocxOpcPartsDiff, String> {
    dec_named_triple_bin(reader, |r| read_str_lp(r), dec_opc_part_diff_bin, dec_opc_part_bin)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_rel_diff_bin(d: &DocxOpcRelDiff, out: &mut Vec<u8>) {
    out.push(if d.rel_type.is_some() { 1 } else { 0 });
    if let Some(v) = &d.rel_type {
        write_str_lp(out, v);
    }
    out.push(if d.target.is_some() { 1 } else { 0 });
    if let Some(v) = &d.target {
        write_str_lp(out, v);
    }
    out.push(if d.target_mode.is_some() { 1 } else { 0 });
    if let Some(v) = &d.target_mode {
        enc_target_mode_bin(v, out);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_rel_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<DocxOpcRelDiff, String> {
    let rel_type = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(read_str_lp(reader)?) } else { None };
    let target = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(read_str_lp(reader)?) } else { None };
    let target_mode = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_target_mode_bin(reader)?) } else { None };
    Ok(DocxOpcRelDiff { rel_type, target, target_mode })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_rel_list_diff_bin(d: &DocxOpcRelListDiff, out: &mut Vec<u8>) {
    enc_named_triple_bin(d, |k, out| write_str_lp(out, k), enc_rel_diff_bin, enc_rel_bin, out)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_rel_list_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<DocxOpcRelListDiff, String> {
    dec_named_triple_bin(reader, |r| read_str_lp(r), dec_rel_diff_bin, dec_rel_bin)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_relationships_diff_bin(d: &DocxOpcRelationshipsDiff, out: &mut Vec<u8>) {
    enc_named_triple_bin(d, |k, out| write_str_lp(out, k), enc_rel_list_diff_bin, enc_rel_owner_entry_bin, out)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_relationships_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<DocxOpcRelationshipsDiff, String> {
    dec_named_triple_bin(reader, |r| read_str_lp(r), dec_rel_list_diff_bin, dec_rel_owner_entry_bin)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_content_types_diff_bin(d: &DocxOpcContentTypesDiff, out: &mut Vec<u8>) {
    out.push(if d.defaults.is_some() { 1 } else { 0 });
    if let Some(v) = &d.defaults {
        enc_ct_entries_diff_bin(v, out);
    }
    out.push(if d.overrides.is_some() { 1 } else { 0 });
    if let Some(v) = &d.overrides {
        enc_ct_entries_diff_bin(v, out);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_content_types_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<DocxOpcContentTypesDiff, String> {
    let defaults = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_ct_entries_diff_bin(reader)?) } else { None };
    let overrides = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_ct_entries_diff_bin(reader)?) } else { None };
    Ok(DocxOpcContentTypesDiff { defaults, overrides })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_opc_diff_bin(d: &DocxOpcDiff, out: &mut Vec<u8>) {
    out.push(if d.content_types.is_some() { 1 } else { 0 });
    if let Some(v) = &d.content_types {
        enc_content_types_diff_bin(v, out);
    }
    out.push(if d.parts.is_some() { 1 } else { 0 });
    if let Some(v) = &d.parts {
        enc_parts_diff_bin(v, out);
    }
    out.push(if d.relationships.is_some() { 1 } else { 0 });
    if let Some(v) = &d.relationships {
        enc_relationships_diff_bin(v, out);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_opc_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<DocxOpcDiff, String> {
    let content_types = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_content_types_diff_bin(reader)?) } else { None };
    let parts = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_parts_diff_bin(reader)?) } else { None };
    let relationships = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_relationships_diff_bin(reader)?) } else { None };
    Ok(DocxOpcDiff { content_types, parts, relationships })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_document_diff_bin(d: &DocxDocumentDiff, out: &mut Vec<u8>) {
    out.push(if d.body.is_some() { 1 } else { 0 });
    if let Some(v) = &d.body {
        enc_blocks_diff_bin(v, out);
    }
    out.push(if d.styles.is_some() { 1 } else { 0 });
    if let Some(v) = &d.styles {
        enc_styles_diff_bin(v, out);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_document_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<DocxDocumentDiff, String> {
    let body = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_blocks_diff_bin(reader)?) } else { None };
    let styles = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_styles_diff_bin(reader)?) } else { None };
    Ok(DocxDocumentDiff { body, styles })
}
//#endregion 🔖️DiffValueBinaryCodecs
//#endregion 🔖️BinaryCodecs

//#region 🔖️TopLevel
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_docx_diff(d: &DocxDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = &d.opc {
        tokens.push(format!("opc={}", enc_opc_diff(v)));
    }
    if let Some(v) = &d.document {
        tokens.push(format!("document={}", enc_document_diff(v)));
    }
    tokens.join(" ")
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_docx_diff(line: &str) -> Result<DocxDiff, String> {
    let mut d = DocxDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("opc=") {
            d.opc = Some(dec_opc_diff(rest)?);
        } else if let Some(rest) = token.strip_prefix("document=") {
            d.document = Some(dec_document_diff(rest)?);
        } else {
            return Err(format!("docx diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl protocol::DiffCodec for DocxDiff {
    async fn print_diff(&self) -> String {
        print_docx_diff(self)
    }
    async fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        parse_docx_diff(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    /// 🧪️ FG-wave: REAL binary frame (`format u8 | flags u8 | [opc][document]`), matching
    /// `../💾️binary/📡️component.protocol.semio`'s `header fixed 2` + `chain payload bytes` shape
    /// — upgraded from F6's `print_diff().into_bytes()` text-as-binary shortcut (per this ticket's
    /// own `📖️grammar-recipe.md` census, 100% of stdio's `DiffCodec` impls were still on that
    /// shortcut before this pilot ladder). `flags` bits 0/1 mark `opc`/`document` presence; each
    /// present field's own recursive binary payload follows in that fixed order (see
    /// `🔖️BinaryCodecs` above).
    async fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let mut flags: u8 = 0;
        if self.opc.is_some() {
            flags |= 0b01;
        }
        if self.document.is_some() {
            flags |= 0b10;
        }
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, flags];
        if let Some(opc) = &self.opc {
            enc_opc_diff_bin(opc, &mut out);
        }
        if let Some(document) = &self.document {
            enc_document_diff_bin(document, &mut out);
        }
        Ok(out)
    }
    async fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes).await;
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let _format = reader.read_u8().await.map_err(|e| malformed("diff format", 0, e.to_string()))?;
        let flags = reader.read_u8().await.map_err(|e| malformed("diff flags", 1, e.to_string()))?;
        let opc = if flags & 0b01 != 0 { Some(dec_opc_diff_bin(&mut reader).map_err(|e| malformed("diff opc", semio_framework_plugin::resolve_ready(reader.position()), e))?) } else { None };
        let document = if flags & 0b10 != 0 { Some(dec_document_diff_bin(&mut reader).map_err(|e| malformed("diff document", semio_framework_plugin::resolve_ready(reader.position()), e))?) } else { None };
        Ok(DocxDiff { opc, document })
    }
}
//#endregion 🔖️TopLevel

//#region 🔖️DemoCases
/// 🧪️ FG-wave: representative `DocxDiff` values (both top-level fields, the recursive
/// `Paragraph`/`Table` `DocxBlockDiff` tree incl. a nested table-cell block list, both
/// `style`/`based_on` tri-states, and the OPC layer's content-types/parts/relationships-by-owner
/// triples) — the single source of truth reused by `diff_codec_text_binary_roundtrip_law` below
/// AND by `⚙️engine/🦀️component.rs`'s `diff_grammar_conformance_law`/`protocol_walk_law`
/// conformance tests, same shape `📷️png/…/🔺️diff/🦀️component.rs`'s own `demo_diff_cases()`
/// establishes.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn xml_node(name: &str) -> XmlNode {
    XmlNode::Element { name: name.to_string(), attrs: vec![XmlAttr { name: "a".into(), value: "1".into() }], children: vec![] }
}

#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn snapshot_a() -> DocxSnapshot {
    let mut opc = OpcPackage::empty();
    opc.content_types.set_default("rels", crate::artifacts::zip::opc::RELS_CONTENT_TYPE);
    opc.content_types.set_default("xml", "application/xml");
    opc.set_part("word/document.xml", "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml", b"<w:document/>".to_vec());
    opc.set_part("word/toRemove.xml", "application/xml", b"gone".to_vec());
    opc.add_relationship("", "rId1", crate::artifacts::zip::opc::REL_TYPE_OFFICE_DOCUMENT, "word/document.xml");
    opc.relationships.insert("word/toRemove.xml".into(), vec![OpcRelationship { id: "rId8".into(), rel_type: "http://example/gone".into(), target: "media/gone.png".into(), target_mode: OpcTargetMode::Internal }]);

    DocxSnapshot::from_parts(
        opc,
        DocxDocument {
            body: vec![
                DocxBlock::Paragraph(DocxParagraph { runs: vec![DocxRun { text: "old".into(), bold: false, extra_run_properties: vec![xml_node("rPr")], ..Default::default() }], style: None, extra_paragraph_properties: Vec::new() }),
                DocxBlock::Table(DocxTable { rows: vec![DocxTableRow { cells: vec![DocxTableCell { blocks: vec![DocxBlock::paragraph("cell")], ..Default::default() }], ..Default::default() }], ..Default::default() }),
            ],
            styles: vec![DocxStyle { id: "keep".into(), name: "Keep".into(), based_on: Some("toRemove".into()) }, DocxStyle { id: "toRemove".into(), name: "Gone".into(), based_on: Some("keep".into()) }],
        },
    )
}

#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn snapshot_b() -> DocxSnapshot {
    let mut opc = OpcPackage::empty();
    opc.content_types.set_default("rels", crate::artifacts::zip::opc::RELS_CONTENT_TYPE);
    opc.content_types.set_default("xml", "application/xml");
    opc.content_types.set_default("added", "application/octet-stream");
    opc.set_part("word/document.xml", "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml", b"<w:document/>changed".to_vec());
    opc.set_part("word/added.xml", "application/xml", b"fresh".to_vec());
    opc.add_relationship("", "rId1", crate::artifacts::zip::opc::REL_TYPE_OFFICE_DOCUMENT, "word/document.xml");
    opc.relationships.insert("word/added.xml".into(), vec![OpcRelationship { id: "rId3".into(), rel_type: "http://example/added".into(), target: "media/added.png".into(), target_mode: OpcTargetMode::External }]);

    DocxSnapshot::from_parts(
        opc,
        DocxDocument {
            body: vec![DocxBlock::Paragraph(DocxParagraph {
                runs: vec![DocxRun { text: "new".into(), bold: true, italic: true, extra_run_properties: Vec::new(), ..Default::default() }, DocxRun { text: "second".into(), underline: true, ..Default::default() }],
                style: Some("keep".into()),
                extra_paragraph_properties: vec![xml_node("pPr")],
            })],
            styles: vec![DocxStyle { id: "keep".into(), name: "Keep2".into(), based_on: None }, DocxStyle { id: "added".into(), name: "Added".into(), based_on: None }],
        },
    )
}

/// 🧪️ The demo cases proper — `default()` (empty diff) plus every real `between()` shape (both
/// directions, and the trivially-empty self-diff).
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<DocxDiff> {
    let a = snapshot_a();
    let b = snapshot_b();
    vec![DocxDiff::default(), DocxDiff::between(&a, &b), DocxDiff::between(&b, &a), DocxDiff::between(&a, &a)]
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
mod handcrafted_diff_codec_tests {
    use super::*;
    use protocol::DiffCodec;

    /// 🧪️ F6: `DiffCodec` round-trip laws over the hand-rolled `DocxDiff` grammar — exercises the
    /// recursive enum tree (`DocxBlockDiff`'s `Paragraph`/`Table` variants, incl. a nested
    /// table-cell block list), both `style`/`based_on` tri-states, the OPC layer's content-types/
    /// parts/relationships-by-owner triples, and every removed/modified/added flavor via a real
    /// `between()` result in both directions.
    #[semio_framework_async_macros::async_test]
    async fn diff_codec_text_binary_roundtrip_law() {
        let a = snapshot_a();
        let b = snapshot_b();
        let cases = vec![DocxDiff::default(), DocxDiff::between(&a, &b), DocxDiff::between(&b, &a), DocxDiff::between(&a, &a)];
        for d in cases {
            let printed = d.print_diff();
            assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
            let parsed = DocxDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
            assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

            let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
            let decoded = DocxDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
            assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
        }

        // Field sweep: confirm every collection flavor and both tri-states actually got exercised
        // above, not just "it round-trips" (an all-`None`/empty diff would round-trip trivially).
        let diff_ab = DocxDiff::between(&a, &b);
        let opc_diff = diff_ab.opc.as_ref().expect("opc diff present");
        assert!(opc_diff.content_types.as_ref().expect("content_types diff present").defaults.as_ref().expect("defaults diff present").added.len() > 0);
        let parts = opc_diff.parts.as_ref().expect("parts diff present");
        assert!(!parts.removed.is_empty() && !parts.modified.is_empty() && !parts.added.is_empty(), "opc.parts: not every flavor exercised");
        let rels = opc_diff.relationships.as_ref().expect("relationships diff present");
        assert!(!rels.removed.is_empty() && !rels.added.is_empty(), "opc.relationships: owner removed/added not exercised");
        let doc_diff = diff_ab.document.as_ref().expect("document diff present");
        let body_diff = doc_diff.body.as_ref().expect("body diff present");
        assert!(!body_diff.removed.is_empty(), "body: removed not exercised");
        assert_eq!(body_diff.modified.len(), 1);
        let DocxBlockDiff::Paragraph(p_diff) = &body_diff.modified[0].diff else { panic!("expected paragraph diff") };
        assert_eq!(p_diff.style, Some(Some("keep".to_string())), "style tri-state Some(Some(_)) not exercised");
        let runs_diff = p_diff.runs.as_ref().expect("runs diff present");
        assert!(!runs_diff.modified.is_empty() && !runs_diff.added.is_empty(), "runs: modified/added not exercised");
        let styles_diff = doc_diff.styles.as_ref().expect("styles diff present");
        assert!(!styles_diff.removed.is_empty() && !styles_diff.added.is_empty(), "styles: removed/added not exercised");
        let style_mod = styles_diff.modified.iter().find(|m| m.key == "keep").expect("keep style modified");
        assert_eq!(style_mod.diff.based_on, Some(None), "based_on tri-state Some(None) not exercised");
    }
}
//#endregion 🧪️Tests
//#endregion 🔖️HandcraftedDiffCodec

#[cfg(test)]
mod result_apply_tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn rejects_missing_style_target_without_mutating_base() {
        let base = DocxSnapshot::default();
        let diff =
            DocxDiff { document: Some(DocxDocumentDiff { styles: Some(DocxStylesDiff { modified: vec![NamedModified { key: "missing".into(), diff: DocxStyleDiff::default() }], ..Default::default() }), ..Default::default() }), ..Default::default() };
        let result = diff.apply(&base);
        assert_eq!(result.unwrap_err().code, "mutation.apply.missing-target");
        assert_eq!(base, DocxSnapshot::default());
    }
}
