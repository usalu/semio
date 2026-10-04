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

#[cfg(test)]
use crate::schema::snapshot::DocxDocument;
use crate::schema::snapshot::{DocxBlock, DocxParagraph, DocxRun, DocxStyle, DocxTable, DocxTableCell, DocxTableRow, DocxXmlPart, DocxXmlParts};
use crate::DocxSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use semio_s_artifact_stdio_contract::deserialize_double_option;
use semio_s_artifact_stdio_xml::schema::diff::{XmlChildrenDiff, XmlDiff};
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlAttr, XmlNode};
use semio_s_artifact_stdio_xml::{XmlSnapshot, STDIO_XML_DOCUMENT_SCHEMA};
use semio_s_artifact_stdio_zip::opc::{OpcContentTypes, OpcPackage, OpcPart, OpcRelationship, OpcTargetMode};
use semio_s_artifact_stdio_zip::opc::retained::RetainedOpcPackage;
use std::collections::BTreeMap;

//#region 🔖️GenericCollectionTriples
/// 🌳 Index-keyed collection triple, generic over the item type `T` and its per-field diff type
/// `D`. `removed`/`modified` indices refer to BASE state (descending removal order on apply);
/// `added` indices refer to FINAL state (ascending insert, `min(index, len)`).
// 🩹 `#[derive(ToValue, FromValue)]` synthesizes `D: ToValue + FromValue`/`T: ...` automatically
// per own type parameter (see `🌱️value/✨️derive`'s module docs) — no explicit
// `#[value(bound = "...")]` override needed here.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct IndexedTripleDiff<D, T> {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<IndexModified<D>>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<IndexAdded<T>>,
}

impl<D, T> Default for IndexedTripleDiff<D, T> {
    fn default() -> Self {
        Self { removed: Vec::new(), modified: Vec::new(), added: Vec::new() }
    }
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct IndexModified<D> {
    pub index: usize,
    pub diff: D,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct IndexAdded<T> {
    pub index: usize,
    pub item: T,
}

/// 🏷️ Name/key-keyed collection triple, generic over key `K`, item `T`, and per-field diff `D`.
/// `added` carries the full item (which already contains its own key). No explicit
/// `#[value(bound = "...")]` needed — auto-synthesized per type parameter, same as
/// `IndexedTripleDiff` above.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct NamedTripleDiff<K, D, T> {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<K>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<NamedModified<K, D>>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<T>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub order: Vec<K>,
}

impl<K, D, T> Default for NamedTripleDiff<K, D, T> {
    fn default() -> Self {
        Self { removed: Vec::new(), modified: Vec::new(), added: Vec::new(), order: Vec::new() }
    }
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
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
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum DocxBlockDiff {
    Paragraph(DocxParagraphDiff),
    Table(DocxTableDiff),
    Replace { block: DocxBlock },
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocxParagraphDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub runs: Option<DocxRunsDiff>,
    /// 🏳️ Tri-state: `None` = unchanged, `Some(None)` = style cleared, `Some(Some(id))` = set.
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub style: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extra_paragraph_properties: Option<XmlChildrenDiff>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocxRunDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub bold: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub italic: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub underline: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extra_run_properties: Option<XmlChildrenDiff>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocxTableDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub rows: Option<DocxTableRowsDiff>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extra_table_properties: Option<XmlChildrenDiff>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocxTableRowDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub cells: Option<DocxTableCellsDiff>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extra_row_properties: Option<XmlChildrenDiff>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocxTableCellDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub blocks: Option<DocxBlocksDiff>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extra_cell_properties: Option<XmlChildrenDiff>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocxStyleDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 🏳️ Tri-state: `None` = unchanged, `Some(None)` = based_on cleared, `Some(Some(id))` = set.
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub based_on: Option<Option<String>>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocxDocumentDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<DocxBlocksDiff>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub styles: Option<DocxStylesDiff>,
}
//#endregion 🔖️DocumentDiffTypes

//#region 🔖️OpcDiffTypes
pub type DocxOpcCtEntriesDiff = NamedTripleDiff<String, String, (String, String)>;
pub type DocxOpcPartsDiff = NamedTripleDiff<String, DocxOpcPartDiff, OpcPart>;
pub type DocxOpcRelListDiff = NamedTripleDiff<String, DocxOpcRelDiff, OpcRelationship>;
pub type DocxOpcRelationshipsDiff = NamedTripleDiff<String, DocxOpcRelListDiff, (String, Vec<OpcRelationship>)>;

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocxOpcContentTypesDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub defaults: Option<DocxOpcCtEntriesDiff>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub overrides: Option<DocxOpcCtEntriesDiff>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocxOpcPartDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub bytes: Option<Vec<u8>>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocxOpcRelDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub rel_type: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub target_mode: Option<OpcTargetMode>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocxOpcDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub content_types: Option<DocxOpcContentTypesDiff>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub parts: Option<DocxOpcPartsDiff>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub relationships: Option<DocxOpcRelationshipsDiff>,
}
//#endregion 🔖️OpcDiffTypes

//#region 🔖️XmlPartDiffTypes
pub type DocxXmlPartsDiff = NamedTripleDiff<String, DocxXmlPartDiff, DocxXmlPart>;

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocxXmlPartDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub document: Option<XmlDiff>,
}
//#endregion 🔖️XmlPartDiffTypes

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
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.docx.diff")]
pub struct DocxDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub opc: Option<DocxOpcDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub xml_parts: Option<DocxXmlPartsDiff>,
}
//#endregion 🔖️Diff

//#region 🔖️PathAddressing
/// 🧭️ One step down into a nested table cell's block list: `body[block_index]` must be a `Table`;
/// descend to `rows[row].cells[cell].blocks`.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocxPathSegment {
    pub block_index: usize,
    pub row: usize,
    pub cell: usize,
}

/// 🧭️ Addresses one block-list slot: `segments` navigate through nested `Table`s (mirrors svg's
/// `NodePath` chain-of-indices precedent, adapted for docx's Paragraph/Table mixed tree),
/// `index` is the slot within the innermost list.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocxBlockPath {
    #[value(default)]
    pub segments: Vec<DocxPathSegment>,
    pub index: usize,
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
            resolve_blocks(&cell.blocks, rest)
        }
    }
}
//#endregion 🔖️PathAddressing

//#region 🔖️GenericNamedEngine
/// 🧮️ The key sequence `removed`/`added` alone imply — survivors in base order, then the additions
/// in carried order. `NamedTripleDiff::order` is populated exactly when the real target sequence is
/// NOT this one, so an order-insignificant collection never pays for the field.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn default_named_order<K: PartialEq + Clone>(base_keys: &[K], removed: &[K], added_keys: &[K]) -> Vec<K> {
    base_keys.iter().filter(|k| !removed.contains(k)).chain(added_keys.iter()).cloned().collect()
}

/// 🔀️ Rebuilds `items` into `order` when one is carried, and fails loudly when it is not a
/// permutation of what the collection actually holds — a silently dropped or duplicated item is
/// precisely the failure this field exists to prevent.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn reorder_named<K: PartialEq, T>(items: &mut Vec<T>, order: &[K], key_of: impl Fn(&T) -> K) -> MutationApplyResult<()> {
    if order.is_empty() {
        return Ok(());
    }
    if order.len() != items.len() {
        return Err(MutationApplyError::new("mutation.apply.invalid-order", "named ordering does not cover the resulting collection").at(["order"]));
    }
    let mut pool: Vec<Option<T>> = std::mem::take(items).into_iter().map(Some).collect();
    for key in order {
        let slot = pool.iter().position(|held| matches!(held, Some(item) if key_of(item) == *key)).ok_or_else(|| MutationApplyError::new("mutation.apply.invalid-order", "named ordering names an item the collection does not carry").at(["order"]))?;
        items.push(pool[slot].take().expect("the slot was located as occupied one line above"));
    }
    Ok(())
}

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
    let base_keys: Vec<K> = base.iter().map(&key_of).collect();
    let other_keys: Vec<K> = other.iter().map(&key_of).collect();
    let added_keys: Vec<K> = added.iter().map(&key_of).collect();
    let order = if default_named_order(&base_keys, &removed, &added_keys) == other_keys { Vec::new() } else { other_keys };
    if removed.is_empty() && modified.is_empty() && added.is_empty() && order.is_empty() {
        None
    } else {
        Some(NamedTripleDiff { removed, modified, added, order })
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
        let item = items.iter_mut().find(|i| key_of(i) == m.key).ok_or_else(|| MutationApplyError::new("mutation.apply.missing-target", "named modification target does not exist").at(["modified"]))?;
        apply_item(item, &m.diff).map_err(|error| error.under(["modified"]))?;
    }
    for item in &diff.added {
        items.push(item.clone());
    }
    reorder_named(items, &diff.order, &key_of)?;
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
    let base_keys: Vec<K> = base_items.iter().map(&key_of).collect();
    let other_keys = if diff.order.is_empty() { default_named_order(&base_keys, &diff.removed, &removed) } else { diff.order.clone() };
    let added_keys: Vec<K> = added.iter().map(&key_of).collect();
    let order = if default_named_order(&other_keys, &removed, &added_keys) == base_keys { Vec::new() } else { base_keys };
    NamedTripleDiff { removed, modified, added, order }
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
    let d1_order = d1.order.clone();
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
    let order = if !d2.order.is_empty() {
        d2.order
    } else if d1_order.is_empty() {
        Vec::new()
    } else {
        let mut composed: Vec<K> = d1_order.into_iter().filter(|k| !d2.removed.contains(k)).collect();
        for a2 in &d2.added {
            let k2 = key_of(a2);
            if !composed.contains(&k2) {
                composed.push(k2);
            }
        }
        composed
    };
    NamedTripleDiff { removed, modified, added: working_added, order }
}
//#endregion 🔖️GenericNamedEngine

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
fn diff_relationships(old: &semio_s_artifact_stdio_zip::opc::OpcRelationshipOwners, new: &semio_s_artifact_stdio_zip::opc::OpcRelationshipOwners) -> Option<DocxOpcRelationshipsDiff> {
    let mut removed = Vec::new();
    let mut modified = Vec::new();
    for (owner, list) in old.groups() {
        match new.relationships(owner) {
            None => removed.push(owner.clone()),
            Some(nlist) => {
                if let Some(d) = diff_rel_list(list, nlist) {
                    modified.push(NamedModified { key: owner.clone(), diff: d });
                }
            }
        }
    }
    let mut added = Vec::new();
    for (owner, list) in new.groups() {
        if old.relationships(owner).is_none() {
            added.push((owner.clone(), list.clone()));
        }
    }
    if removed.is_empty() && modified.is_empty() && added.is_empty() {
        None
    } else {
        Some(DocxOpcRelationshipsDiff { removed, modified, added, order: Vec::new() })
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_relationships(rels: &mut semio_s_artifact_stdio_zip::opc::OpcRelationshipOwners, diff: &DocxOpcRelationshipsDiff) -> MutationApplyResult<()> {
    let mut added = std::collections::HashSet::new();
    for owner in &diff.removed {
        if rels.relationships(owner).is_none() {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "relationship owner does not exist").at(vec!["removed".to_string(), owner.clone()]));
        }
        if !added.insert(owner) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "relationship owner is repeated").at(vec!["removed".to_string(), owner.clone()]));
        }
    }
    for modified in &diff.modified {
        if rels.relationships(&modified.key).is_none() {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "relationship owner does not exist").at(vec!["modified".to_string(), modified.key.clone()]));
        }
        if diff.removed.contains(&modified.key) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "relationship owner is removed and modified").at(vec!["modified".to_string(), modified.key.clone()]));
        }
    }
    for (owner, _) in &diff.added {
        if rels.relationships(owner).is_some() || !added.insert(owner) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "relationship owner already exists").at(vec!["added".to_string(), owner.clone()]));
        }
    }
    for owner in &diff.removed {
        rels.remove_owner(owner);
    }
    for m in &diff.modified {
        let list = rels.relationships_mut(&m.key).ok_or_else(|| MutationApplyError::new("mutation.apply.missing-target", "relationship owner does not exist").at(vec!["modified".to_string(), m.key.clone()]))?;
        apply_rel_list(list, &m.diff).map_err(|error| error.under(vec!["modified".to_string(), m.key.clone()]))?;
    }
    for (owner, list) in &diff.added {
        rels.replace_owner(owner.clone(), list.clone());
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_relationships(base: &semio_s_artifact_stdio_zip::opc::OpcRelationshipOwners, diff: &DocxOpcRelationshipsDiff) -> DocxOpcRelationshipsDiff {
    let removed: Vec<String> = diff.added.iter().map(|(owner, _)| owner.clone()).collect();
    let mut modified = Vec::new();
    for m in &diff.modified {
        if let Some(list) = base.relationships(&m.key) {
            modified.push(NamedModified { key: m.key.clone(), diff: inverse_rel_list(list, &m.diff) });
        }
    }
    let mut added = Vec::new();
    for owner in &diff.removed {
        if let Some(list) = base.relationships(owner) {
            added.push((owner.clone(), list.clone()));
        }
    }
    DocxOpcRelationshipsDiff { removed, modified, added, order: Vec::new() }
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
    let comment = (base.comment != other.comment).then(|| other.comment.clone());
    if comment.is_none() && content_types.is_none() && parts.is_none() && relationships.is_none() {
        None
    } else {
        Some(DocxOpcDiff { content_types, parts, relationships, comment })
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
    if let Some(comment) = &diff.comment {
        opc.comment.clone_from(comment);
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_opc_diff(base: &OpcPackage, diff: &DocxOpcDiff) -> DocxOpcDiff {
    DocxOpcDiff {
        comment: diff.comment.as_ref().map(|_| base.comment.clone()),
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
        comment: b.comment.or(a.comment),
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

//#region 🔖️XmlPartDiffLogic
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn xml_snapshot(document: &semio_s_artifact_stdio_xml::schema::snapshot::retained::RetainedXmlDocument) -> XmlSnapshot {
    XmlSnapshot {
        schema: STDIO_XML_DOCUMENT_SCHEMA.into(),
        doc: document.materialize_exact().expect("valid retained DOCX XML authority materializes for diff"),
    }
}

fn diff_xml_part(base: &DocxXmlPart, other: &DocxXmlPart) -> Option<DocxXmlPartDiff> {
    let document = XmlDiff::between(&xml_snapshot(&base.document), &xml_snapshot(&other.document));
    let diff = DocxXmlPartDiff { content_type: (base.content_type != other.content_type).then(|| other.content_type.clone()), document: (!document.is_empty()).then_some(document) };
    (diff.content_type.is_some() || diff.document.is_some()).then_some(diff)
}

fn apply_xml_part(part: &mut DocxXmlPart, diff: &DocxXmlPartDiff) -> MutationApplyResult<()> {
    if let Some(content_type) = &diff.content_type {
        part.content_type.clone_from(content_type);
    }
    if let Some(document) = &diff.document {
        let next = document.apply(&xml_snapshot(&part.document))?.doc;
        part.replace_document(next).map_err(|error| MutationApplyError::new("mutation.apply.ownership", error.into_message()).at(["document"]))?;
    }
    Ok(())
}

fn inverse_xml_part(base: &DocxXmlPart, diff: &DocxXmlPartDiff) -> DocxXmlPartDiff {
    DocxXmlPartDiff { content_type: diff.content_type.as_ref().map(|_| base.content_type.clone()), document: diff.document.as_ref().map(|document| document.inverse(&xml_snapshot(&base.document))) }
}

fn absorb_xml_part(mut first: DocxXmlPartDiff, second: DocxXmlPartDiff) -> DocxXmlPartDiff {
    if second.content_type.is_some() {
        first.content_type = second.content_type;
    }
    first.document = match (first.document.take(), second.document) {
        (None, value) => value,
        (value, None) => value,
        (Some(mut left), Some(right)) => {
            left.absorb(right);
            Some(left)
        }
    };
    first
}

fn between_xml_parts(base: &DocxXmlParts, other: &DocxXmlParts) -> Option<DocxXmlPartsDiff> {
    let mut removed = Vec::new();
    let mut modified = Vec::new();
    for item in base {
        match other.iter().find(|candidate| candidate.path == item.path) {
            None => removed.push(item.path.clone()),
            Some(candidate) if candidate != item => {
                if let Some(diff) = diff_xml_part(item, candidate) {
                    modified.push(NamedModified { key: item.path.clone(), diff });
                }
            }
            Some(_) => {}
        }
    }
    let added: Vec<_> = other.iter().filter(|item| !base.iter().any(|candidate| candidate.path == item.path)).cloned().collect();
    let base_keys: Vec<_> = base.iter().map(|item| item.path.clone()).collect();
    let other_keys: Vec<_> = other.iter().map(|item| item.path.clone()).collect();
    let added_keys: Vec<_> = added.iter().map(|item| item.path.clone()).collect();
    let order = if default_named_order(&base_keys, &removed, &added_keys) == other_keys { Vec::new() } else { other_keys };
    if removed.is_empty() && modified.is_empty() && added.is_empty() && order.is_empty() {
        None
    } else {
        Some(NamedTripleDiff { removed, modified, added, order })
    }
}

fn reorder_xml_parts(items: &mut DocxXmlParts, order: &[String]) -> MutationApplyResult<()> {
    if order.is_empty() {
        return Ok(());
    }
    if order.len() != items.len() {
        return Err(MutationApplyError::new("mutation.apply.invalid-order", "named ordering does not cover the resulting collection").at(["order"]));
    }
    for (target, key) in order.iter().enumerate() {
        let position = items
            .iter()
            .enumerate()
            .skip(target)
            .find_map(|(position, item)| (item.path == *key).then_some(position))
            .ok_or_else(|| MutationApplyError::new("mutation.apply.invalid-order", "named ordering names an item the collection does not carry").at(["order"]))?;
        items.swap(target, position);
    }
    Ok(())
}

fn apply_xml_parts(items: &mut DocxXmlParts, diff: &DocxXmlPartsDiff) -> MutationApplyResult<()> {
    for (index, key) in diff.removed.iter().enumerate() {
        if !items.iter().any(|item| item.path == *key) {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "named removal target does not exist").at(["removed"]));
        }
        if diff.removed[..index].contains(key) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "named removal target is repeated").at(["removed"]));
        }
    }
    for (index, modified) in diff.modified.iter().enumerate() {
        if !items.iter().any(|item| item.path == modified.key) {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "named modification target does not exist").at(["modified"]));
        }
        if diff.removed.contains(&modified.key) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "named modification targets a removed item").at(["modified"]));
        }
        if diff.modified[..index].iter().any(|candidate| candidate.key == modified.key) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "named modification target is repeated").at(["modified"]));
        }
    }
    for (index, added) in diff.added.iter().enumerate() {
        if items.iter().any(|item| item.path == added.path) || diff.added[..index].iter().any(|candidate| candidate.path == added.path) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "named addition target already exists").at(["added"]));
        }
    }
    items.retain(|item| !diff.removed.contains(&item.path));
    for modified in &diff.modified {
        let item = items.iter_mut().find(|item| item.path == modified.key).expect("validated XML modification target");
        apply_xml_part(item, &modified.diff).map_err(|error| error.under(["modified"]))?;
    }
    for item in &diff.added {
        items.try_push(item.clone()).map_err(|error| MutationApplyError::new("mutation.apply.ownership", error.to_string()).at(["added"]))?;
    }
    reorder_xml_parts(items, &diff.order)
}

fn inverse_xml_parts(base: &DocxXmlParts, diff: &DocxXmlPartsDiff) -> DocxXmlPartsDiff {
    let removed = diff.added.iter().map(|item| item.path.clone()).collect::<Vec<_>>();
    let modified = diff
        .modified
        .iter()
        .filter_map(|change| base.iter().find(|item| item.path == change.key).map(|item| NamedModified { key: change.key.clone(), diff: inverse_xml_part(item, &change.diff) }))
        .collect();
    let added = diff.removed.iter().filter_map(|key| base.iter().find(|item| item.path == *key).cloned()).collect::<Vec<_>>();
    let base_keys = base.iter().map(|item| item.path.clone()).collect::<Vec<_>>();
    let other_keys = if diff.order.is_empty() { default_named_order(&base_keys, &diff.removed, &removed) } else { diff.order.clone() };
    let added_keys = added.iter().map(|item| item.path.clone()).collect::<Vec<_>>();
    let order = if default_named_order(&other_keys, &removed, &added_keys) == base_keys { Vec::new() } else { base_keys };
    NamedTripleDiff { removed, modified, added, order }
}
//#endregion 🔖️XmlPartDiffLogic

//#region 🔖️Apply
impl MutationDiff<DocxSnapshot> for DocxDiff {
    fn apply(&self, base: &DocxSnapshot) -> MutationApplyResult<DocxSnapshot> {
        let mut next = base.clone();
        if let Some(diff) = &self.opc {
            let mut opc = next.opc.materialize_package_exact().map_err(|error| MutationApplyError::new("mutation.apply.ownership", error.to_string()).under(["opc"]))?;
            apply_opc_diff(&mut opc, diff).map_err(|error| error.under(["opc"]))?;
            next.opc = RetainedOpcPackage::try_from_package(opc).map_err(|error| MutationApplyError::new("mutation.apply.ownership", error.to_string()).under(["opc"]))?;
        }
        if let Some(diff) = &self.xml_parts {
            apply_xml_parts(&mut next.xml_parts, diff).map_err(|error| error.under(["xmlParts"]))?;
        }
        next.validate_authority().and_then(|()| next.project_document().map(|_| ())).map_err(|error| MutationApplyError::new("mutation.apply.invalid-snapshot", error.to_string()))?;
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        self.opc = match (self.opc.take(), other.opc) {
            (None, value) => value,
            (value, None) => value,
            (Some(left), Some(right)) => Some(absorb_opc_diff(left, right)),
        };
        self.xml_parts = match (self.xml_parts.take(), other.xml_parts) {
            (None, value) => value,
            (value, None) => value,
            (Some(left), Some(right)) => Some(absorb_named(
                left,
                right,
                |part| part.path.clone(),
                absorb_xml_part,
                |part, diff| {
                    let _ = apply_xml_part(part, diff);
                },
            )),
        };
    }
}
//#endregion 🔖️Apply

//#region 🔖️DiffAlgebra
impl DiffAlgebra<DocxSnapshot> for DocxDiff {
    fn inverse(&self, base: &DocxSnapshot) -> Self {
        DocxDiff {
            opc: self.opc.as_ref().map(|diff| {
                let opc = base.opc.materialize_package_exact().expect("a valid retained DOCX OPC authority materializes for inverse diff");
                inverse_opc_diff(&opc, diff)
            }),
            xml_parts: self.xml_parts.as_ref().map(|diff| inverse_xml_parts(&base.xml_parts, diff)),
        }
    }

    fn between(base: &DocxSnapshot, other: &DocxSnapshot) -> Self {
        let base_opc = base.opc.materialize_package_exact().expect("a valid retained DOCX OPC authority materializes for diff");
        let other_opc = other.opc.materialize_package_exact().expect("a valid retained DOCX OPC authority materializes for diff");
        DocxDiff { opc: diff_opc(&base_opc, &other_opc), xml_parts: between_xml_parts(&base.xml_parts, &other.xml_parts) }
    }

    fn is_empty(&self) -> bool {
        self.opc.is_none() && self.xml_parts.is_none()
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
    if !s.len().is_multiple_of(2) {
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
    format!("[{}]", items.iter().map(enc).collect::<Vec<_>>().join(","))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_list<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Vec<T>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec).collect()
}
//#endregion 🔖️Primitives

//#region 🔖️XmlValueCodecs
/// 🌳️ Recursive: `E[name,[attrs],[children]]` / `T[text]` / `D[text]` (CData) / `M[text]`
/// (comment) / `P[target,data]` (processing instruction) -- same tag scheme `📰️xml`/`🎨️svg`'s own
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
//#endregion 🔖️ValueCodecs

//#region 🔖️BinaryCodecs
/// 🧪️ FG-wave: real recursive BINARY twins of every text-form codec above, backing the upgraded
/// `DiffCodec::encode_diff`/`decode_diff` below (and, via re-export, `../🧬️mutations/🦀️.rs`'s
/// own upgraded `OpBinary`) — replaces F6's `print_diff().into_bytes()` text-as-binary shortcut.
/// Real LEB128-varint-framed length-prefixed strings/bytes (`store::pack_rt::write_varint_u64` +
/// `store::ByteReader`), 1-byte tri-state presence tags, and 1-byte enum-variant tags — genuinely
/// structured binary, never hex-ASCII text reused as "binary". Same shape
/// `📰️xml/…/🔺️diff/🦀️.rs`'s own `BinaryPrimitives`/`XmlValueBinaryCodecs`/
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
                children.push(dec_xml_node_bin(reader)?);
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
        blocks.push(dec_block_bin(reader)?);
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
        cells.push(dec_cell_bin(reader)?);
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
        rows.push(dec_row_bin(reader)?);
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
        1 => Ok(DocxBlock::Table(dec_table_bin(reader)?)),
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
//#endregion 🔖️ValueBinaryCodecs
//#endregion 🔖️BinaryCodecs

//#region 🔖️TopLevel
impl protocol::DiffCodec for DocxDiff {
    fn print_diff(&self) -> String {
        semio_framework_pack_json::to_json_string(self)
    }

    fn parse_diff(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }

    fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let mut bytes = vec![store::pack_rt::OP_BINARY_FORMAT];
        bytes.extend_from_slice(semio_framework_pack_json::to_json_string(self).as_bytes());
        Ok(bytes)
    }

    fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let payload = bytes.get(1..).ok_or_else(|| protocol::ProtocolError::Malformed { what: "docx diff", offset: 0, detail: "missing format byte".into() })?;
        let text = std::str::from_utf8(payload).map_err(|error| protocol::ProtocolError::Malformed { what: "docx diff", offset: 1, detail: error.to_string() })?;
        semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| protocol::ProtocolError::Malformed { what: "docx diff", offset: 1, detail: error.to_string() })
    }
}
//#endregion 🔖️TopLevel

//#region 🔖️DemoCases
/// 🧪️ FG-wave: representative `DocxDiff` values (both top-level fields, the recursive
/// `Paragraph`/`Table` `DocxBlockDiff` tree incl. a nested table-cell block list, both
/// `style`/`based_on` tri-states, and the OPC layer's content-types/parts/relationships-by-owner
/// triples) — the single source of truth reused by `diff_codec_text_binary_roundtrip_law` below
/// AND by `⚙️engine/🦀️.rs`'s `diff_grammar_conformance_law`/`protocol_walk_law`
/// conformance tests, same shape `📷️png/…/🔺️diff/🦀️.rs`'s own `demo_diff_cases()`
/// establishes.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn xml_node(name: &str) -> XmlNode {
    XmlNode::Element { name: name.to_string(), attrs: vec![XmlAttr { name: "a".into(), value: "1".into() }], children: vec![] }
}

#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn snapshot_a() -> DocxSnapshot {
    let mut snapshot = crate::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_docx(DocxDocument {
        body: vec![DocxBlock::Paragraph(DocxParagraph { runs: vec![DocxRun { text: "old".into(), bold: false, extra_run_properties: vec![xml_node("rPr")], ..Default::default() }], style: None, extra_paragraph_properties: Vec::new() })],
        styles: vec![DocxStyle { id: "keep".into(), name: "Keep".into(), based_on: Some("toRemove".into()) }],
    });
    snapshot.opc.set_part("word/media/to-remove.bin", "application/octet-stream", vec![1, 2]);
    snapshot
}

#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub(crate) fn snapshot_b() -> DocxSnapshot {
    let mut snapshot = crate::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_docx(DocxDocument {
        body: vec![DocxBlock::Paragraph(DocxParagraph {
            runs: vec![DocxRun { text: "new".into(), bold: true, italic: true, ..Default::default() }, DocxRun { text: "second".into(), underline: true, ..Default::default() }],
            style: Some("keep".into()),
            extra_paragraph_properties: vec![xml_node("pPr")],
        })],
        styles: vec![DocxStyle { id: "keep".into(), name: "Keep2".into(), based_on: None }, DocxStyle { id: "added".into(), name: "Added".into(), based_on: None }],
    });
    snapshot.opc.set_part("word/media/added.bin", "application/octet-stream", vec![3, 4]);
    snapshot
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
#[path = "🧪️tests/🔬️handcrafted-diff-codec/🦀️.rs"]
mod handcrafted_diff_codec_tests;
//#endregion 🧪️Tests
//#endregion 🔖️HandcraftedDiffCodec

#[cfg(test)]
#[path = "🧪️tests/🔬️result-apply/🦀️.rs"]
mod result_apply_tests;
