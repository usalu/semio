//! 🔺️ XlsxDiff — handcrafted sparse diff over `XlsxSnapshot` (`opc: OpcPackage` +
//! `workbook: XlsxWorkbook`). No `snapshot: Option<XlsxSnapshot>` full-replace slot — even
//! `SetSnapshot`'s diff is the sparse field-by-field `XlsxDiff::between(base, next)`.
//!
//! `workbook.sheets` is name-keyed (a sheet's `name` is its identity — rename is documented
//! remove+add, see the snapshot module); each sheet's `cells` is keyed by the `(row, col)`
//! coordinate tuple (a sparse spreadsheet wants coordinate addressing, not index-positional —
//! this ticket's own brief calls this out as the per-artifact judgment call); `shared_strings` is
//! index-keyed. All three use the same generic `NamedTripleDiff<K, D, T>` engine (`K = String` /
//! `(u32, u32)` / `usize` respectively) docx's `🔺️diff` established — copied here (not hoisted
//! into a shared module) per this wave's ownership boundary, same rationale as `zip::opc` diff
//! placement below.
//!
//! **OPC diff placement**: `zip::opc::OpcPackage` (reused directly, not reimplemented — see that
//! module) has no diff type of its own yet, same gap docx's wave found. Defined HERE for the same
//! reason docx defined its own copy (this wave's ownership boundary is xlsx-mounted files only;
//! `zip/📦️opc` is out of bounds) — flagged again in `glue_followup` for hoisting once a third
//! consumer (pptx/bcf) needs the identical shape.

use crate::artifacts::xlsx::schema::snapshot::{XlsxCell, XlsxCellValue, XlsxSheet, XlsxWorkbook};
use crate::artifacts::xlsx::XlsxSnapshot;
use crate::artifacts::zip::opc::{OpcContentTypes, OpcPackage, OpcPart, OpcRelationship, OpcTargetMode};
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

//#region 🔖️GenericCollectionTriples
/// 🏷️ Name/key-keyed collection triple, generic over key `K`, item `T`, and per-field diff `D`.
/// `removed`/`modified` keys refer to BASE state; `added` carries the full item (already
/// containing its own key). Identity is the KEY, not position — no index transport is needed on
/// absorb.
// 🩹 `bound(...)` overrides serde's default per-field-`default` bound inference (a known
// serde_derive limitation: a `#[serde(default)]` `Vec<_>` field conservatively demands
// `D: Default`/`T: Default` even though `Vec<_>: Default` never actually needs its item type to
// be `Default` — the real requirement is only `Serialize`/`Deserialize`), same fix docx's diff
// module documents.
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
    async fn default() -> Self {
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

//#region 🔖️WorkbookDiffTypes
/// 🧮️ A cell's per-field diff — `value` is the ONLY diffable field (`row`/`col` are the cell's
/// identity, the `(u32,u32)` key `XlsxCellsDiff` diffs by). `XlsxCellValue` is a value/weak
/// entity per the recipe (a value union, not a keyed collection) — whole-value replaced, never
/// sub-diffed field-by-field (its own `Formula.cached` nests another `XlsxCellValue`, so
/// sub-diffing would need a second recursive diff type for no real gain over LWW-replace).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XlsxCellDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<XlsxCellValue>,
}

pub type XlsxCellsDiff = NamedTripleDiff<(u32, u32), XlsxCellDiff, XlsxCell>;
pub type XlsxSheetsDiff = NamedTripleDiff<String, XlsxSheetDiff, XlsxSheet>;
pub type XlsxSharedStringsDiff = NamedTripleDiff<usize, String, (usize, String)>;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XlsxSheetDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cells: Option<XlsxCellsDiff>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XlsxWorkbookDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sheets: Option<XlsxSheetsDiff>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shared_strings: Option<XlsxSharedStringsDiff>,
}
//#endregion 🔖️WorkbookDiffTypes

//#region 🔖️OpcDiffTypes
pub type XlsxOpcCtEntriesDiff = NamedTripleDiff<String, String, (String, String)>;
pub type XlsxOpcPartsDiff = NamedTripleDiff<String, XlsxOpcPartDiff, OpcPart>;
pub type XlsxOpcRelListDiff = NamedTripleDiff<String, XlsxOpcRelDiff, OpcRelationship>;
pub type XlsxOpcRelationshipsDiff = NamedTripleDiff<String, XlsxOpcRelListDiff, (String, Vec<OpcRelationship>)>;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XlsxOpcContentTypesDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub defaults: Option<XlsxOpcCtEntriesDiff>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overrides: Option<XlsxOpcCtEntriesDiff>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XlsxOpcPartDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bytes: Option<Vec<u8>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XlsxOpcRelDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rel_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_mode: Option<OpcTargetMode>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XlsxOpcDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_types: Option<XlsxOpcContentTypesDiff>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parts: Option<XlsxOpcPartsDiff>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relationships: Option<XlsxOpcRelationshipsDiff>,
}
//#endregion 🔖️OpcDiffTypes

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.xlsx`.
/// 🧪️ F6 CONFIRMED (STEP 1, real `cargo check -p semio-s-plugin-stdio --lib` run, see
/// `f6-xlsx-diff-check2.txt` in the ticket folder): `#[derive(dsl::DslDiff)]` on this struct fails
/// to compile — root cause `XlsxCellValue: DslField is not satisfied` (§3a's "enum-in-tree" rule):
/// ```text
/// error[E0277]: the trait bound `XlsxCellValue: DslField` is not satisfied
///   --> …/🔺️diff/🦀️component.rs:72:23   (pub value: Option<XlsxCellValue>)
/// help: the trait `DslField` is not implemented for `…snapshot::component::XlsxCellValue`
///   --> …/📸️snapshot/🦀️component.rs:26:1   (pub enum XlsxCellValue)
/// ```
/// `XlsxCellValue` (`Number`/`SharedString`/`InlineString`/`Boolean`/`Formula{expr,cached}`/
/// `Empty`) is a genuine data-carrying enum reachable from `XlsxCellDiff.value` — no `DslField`
/// impl exists or can be added (no `impl<T: DslVariants> DslField for T` bridge in this codebase,
/// per `f6-recon-report.md` §3a). Independently, the top-level `opc`/`workbook` fields also fail
/// (`XlsxOpcDiff`/`XlsxWorkbookDiff: DslField` not satisfied) until every nested struct in the tree
/// gets `#[derive(dsl::DslRecord)]`, AND the shared `NamedTripleDiff<K,D,T>` collection-triple type
/// this file's collections use has no `DslField` impl (no blanket impl for arbitrary generic
/// structs, only `Vec`/`BTreeMap`/arrays) — a second, independent structural blocker beyond the
/// enum. `DiffCodec` is hand-rolled below (§5's template, `f6-recon-report.md`).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.xlsx.diff")]
pub struct XlsxDiff {
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub opc: Option<XlsxOpcDiff>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workbook: Option<XlsxWorkbookDiff>,
}
//#endregion 🔖️Diff

//#region 🔖️GenericNamedEngine
async fn between_named<K, T, D>(base: &[T], other: &[T], key_of: impl Fn(&T) -> K, diff_item: impl Fn(&T, &T) -> Option<D>) -> Option<NamedTripleDiff<K, D, T>>
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

async fn apply_named<K, T, D>(items: &mut Vec<T>, diff: &NamedTripleDiff<K, D, T>, key_of: impl Fn(&T) -> K, apply_item: impl Fn(&mut T, &D) -> MutationApplyResult<()>) -> MutationApplyResult<()>
where
    K: PartialEq + Clone,
    T: Clone,
{
    let keys: Vec<K> = items.iter().map(&key_of).collect();
    for (position, key) in diff.removed.iter().enumerate() {
        if !keys.contains(key) {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "named removal target does not exist"));
        }
        if diff.removed[..position].contains(key) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "named removal target is repeated"));
        }
    }
    for (position, modified) in diff.modified.iter().enumerate() {
        if !keys.contains(&modified.key) {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "named modification target does not exist"));
        }
        if diff.removed.contains(&modified.key) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "named modification targets a removed item"));
        }
        if diff.modified[..position].iter().any(|candidate| candidate.key == modified.key) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "named modification target is repeated"));
        }
    }
    for item in &diff.added {
        let key = key_of(item);
        if keys.contains(&key) || diff.added.iter().filter(|candidate| key_of(candidate) == key).count() != 1 {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "named addition target already exists"));
        }
    }
    items.retain(|i| !diff.removed.contains(&key_of(i)));
    for m in &diff.modified {
        let item = items.iter_mut().find(|i| key_of(i) == m.key).ok_or_else(|| MutationApplyError::new("mutation.apply.missing-target", "named modification target does not exist"))?;
        apply_item(item, &m.diff).map_err(|error| error.under(["modified"]))?;
    }
    for item in &diff.added {
        items.push(item.clone());
    }
    Ok(())
}

async fn inverse_named<K, T, D>(base_items: &[T], diff: &NamedTripleDiff<K, D, T>, key_of: impl Fn(&T) -> K, inverse_item: impl Fn(&T, &D) -> D) -> NamedTripleDiff<K, D, T>
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

/// 🧮️ Name-keyed absorb — identity is the KEY (not position): a `d2`-removal of a `d1`-added key
/// annihilates the add; a `d2`-modify of a `d1`-added key patches into the carried payload;
/// everything else composes directly on the shared key space.
async fn absorb_named<K, T, D>(d1: NamedTripleDiff<K, D, T>, d2: NamedTripleDiff<K, D, T>, key_of: impl Fn(&T) -> K, absorb_item: impl Fn(D, D) -> D, apply_item: impl Fn(&mut T, &D)) -> NamedTripleDiff<K, D, T>
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

//#region 🔖️WorkbookDiffLogic
async fn cell_key(cell: &XlsxCell) -> (u32, u32) {
    (cell.row, cell.col)
}

async fn diff_cell(old: &XlsxCell, new: &XlsxCell) -> Option<XlsxCellDiff> {
    if old.value == new.value {
        return None;
    }
    Some(XlsxCellDiff { value: Some(new.value.clone()) })
}

async fn apply_cell(cell: &mut XlsxCell, diff: &XlsxCellDiff) -> MutationApplyResult<()> {
    if let Some(v) = &diff.value {
        cell.value = v.clone();
    }
    Ok(())
}

async fn apply_cell_for_absorb(cell: &mut XlsxCell, diff: &XlsxCellDiff) {
    if let Some(value) = &diff.value {
        cell.value = value.clone();
    }
}

async fn inverse_cell(base: &XlsxCell, diff: &XlsxCellDiff) -> XlsxCellDiff {
    XlsxCellDiff { value: diff.value.as_ref().map(|_| base.value.clone()) }
}

async fn absorb_cell_diff(mut a: XlsxCellDiff, b: XlsxCellDiff) -> XlsxCellDiff {
    if b.value.is_some() {
        a.value = b.value;
    }
    a
}

async fn diff_sheet(old: &XlsxSheet, new: &XlsxSheet) -> Option<XlsxSheetDiff> {
    let cells = between_named(&old.cells, &new.cells, cell_key, diff_cell);
    if cells.is_none() {
        None
    } else {
        Some(XlsxSheetDiff { cells })
    }
}

async fn apply_sheet(sheet: &mut XlsxSheet, diff: &XlsxSheetDiff) -> MutationApplyResult<()> {
    if let Some(cd) = &diff.cells {
        apply_named(&mut sheet.cells, cd, cell_key, apply_cell).map_err(|error| error.under(["cells"]))?;
    }
    Ok(())
}

async fn apply_sheet_for_absorb(sheet: &mut XlsxSheet, diff: &XlsxSheetDiff) {
    if let Some(cells) = &diff.cells {
        apply_named_for_absorb(&mut sheet.cells, cells, cell_key, apply_cell_for_absorb);
    }
}

async fn apply_named_for_absorb<K, T, D>(items: &mut Vec<T>, diff: &NamedTripleDiff<K, D, T>, key_of: impl Fn(&T) -> K, apply_item: impl Fn(&mut T, &D))
where
    K: PartialEq + Clone,
    T: Clone,
{
    items.retain(|item| !diff.removed.contains(&key_of(item)));
    for modified in &diff.modified {
        if let Some(item) = items.iter_mut().find(|item| key_of(item) == modified.key) {
            apply_item(item, &modified.diff);
        }
    }
    for added in &diff.added {
        items.push(added.clone());
    }
}

async fn inverse_sheet(base: &XlsxSheet, diff: &XlsxSheetDiff) -> XlsxSheetDiff {
    XlsxSheetDiff { cells: diff.cells.as_ref().map(|cd| inverse_named(&base.cells, cd, cell_key, inverse_cell)) }
}

async fn absorb_sheet_diff(mut a: XlsxSheetDiff, b: XlsxSheetDiff) -> XlsxSheetDiff {
    a.cells = match (a.cells.take(), b.cells) {
        (None, x) => x,
        (x, None) => x,
        (Some(ca), Some(cb)) => Some(absorb_named(ca, cb, cell_key, absorb_cell_diff, apply_cell_for_absorb)),
    };
    a
}

async fn diff_shared_string(old: &(usize, String), new: &(usize, String)) -> Option<String> {
    (old.1 != new.1).then(|| new.1.clone())
}

async fn shared_strings_pairs(strings: &[String]) -> Vec<(usize, String)> {
    strings.iter().cloned().enumerate().collect()
}

async fn diff_shared_strings(old: &[String], new: &[String]) -> Option<XlsxSharedStringsDiff> {
    between_named(&shared_strings_pairs(old), &shared_strings_pairs(new), |(i, _)| *i, diff_shared_string)
}

async fn apply_shared_strings(strings: &mut Vec<String>, diff: &XlsxSharedStringsDiff) -> MutationApplyResult<()> {
    let mut pairs = shared_strings_pairs(strings);
    apply_named(
        &mut pairs,
        diff,
        |(i, _)| *i,
        |(_, value), next| {
            *value = next.clone();
            Ok(())
        },
    )?;
    pairs.sort_by_key(|(i, _)| *i);
    *strings = pairs.into_iter().map(|(_, v)| v).collect();
    Ok(())
}

async fn inverse_shared_strings(base: &[String], diff: &XlsxSharedStringsDiff) -> XlsxSharedStringsDiff {
    inverse_named(&shared_strings_pairs(base), diff, |(i, _)| *i, |(_, v), _| v.clone())
}

async fn absorb_shared_strings_diff(a: XlsxSharedStringsDiff, b: XlsxSharedStringsDiff) -> XlsxSharedStringsDiff {
    // 🏷️ `D = String` here is already a whole-value replace (LWW) — absorbing two such diffs on
    // the SAME index is just "the later one wins", i.e. `b` (same pattern as docx's content-types
    // entries absorb).
    absorb_named(a, b, |(i, _)| *i, |_av, bv| bv, |(_, v), nv| *v = nv.clone())
}

async fn diff_workbook(base: &XlsxWorkbook, other: &XlsxWorkbook) -> Option<XlsxWorkbookDiff> {
    let sheets = between_named(&base.sheets, &other.sheets, |s| s.name.clone(), diff_sheet);
    let shared_strings = diff_shared_strings(&base.shared_strings, &other.shared_strings);
    if sheets.is_none() && shared_strings.is_none() {
        None
    } else {
        Some(XlsxWorkbookDiff { sheets, shared_strings })
    }
}

async fn apply_workbook_diff(workbook: &mut XlsxWorkbook, diff: &XlsxWorkbookDiff) -> MutationApplyResult<()> {
    if let Some(sd) = &diff.sheets {
        apply_named(&mut workbook.sheets, sd, |s| s.name.clone(), apply_sheet).map_err(|error| error.under(["sheets"]))?;
    }
    if let Some(ssd) = &diff.shared_strings {
        apply_shared_strings(&mut workbook.shared_strings, ssd).map_err(|error| error.under(["sharedStrings"]))?;
    }
    Ok(())
}

async fn inverse_workbook_diff(base: &XlsxWorkbook, diff: &XlsxWorkbookDiff) -> XlsxWorkbookDiff {
    XlsxWorkbookDiff { sheets: diff.sheets.as_ref().map(|sd| inverse_named(&base.sheets, sd, |s| s.name.clone(), inverse_sheet)), shared_strings: diff.shared_strings.as_ref().map(|ssd| inverse_shared_strings(&base.shared_strings, ssd)) }
}

async fn absorb_workbook_diff(a: XlsxWorkbookDiff, b: XlsxWorkbookDiff) -> XlsxWorkbookDiff {
    XlsxWorkbookDiff {
        sheets: match (a.sheets, b.sheets) {
            (None, x) => x,
            (x, None) => x,
            (Some(sa), Some(sb)) => Some(absorb_named(sa, sb, |s| s.name.clone(), absorb_sheet_diff, apply_sheet_for_absorb)),
        },
        shared_strings: match (a.shared_strings, b.shared_strings) {
            (None, x) => x,
            (x, None) => x,
            (Some(a), Some(b)) => Some(absorb_shared_strings_diff(a, b)),
        },
    }
}
//#endregion 🔖️WorkbookDiffLogic

//#region 🔖️OpcDiffLogic
async fn diff_ct_entries(old: &[(String, String)], new: &[(String, String)]) -> Option<XlsxOpcCtEntriesDiff> {
    between_named(old, new, |(k, _)| k.clone(), |(_, ov), (_, nv)| (ov != nv).then(|| nv.clone()))
}

async fn apply_ct_entries(entries: &mut Vec<(String, String)>, diff: &XlsxOpcCtEntriesDiff) -> MutationApplyResult<()> {
    apply_named(
        entries,
        diff,
        |(k, _)| k.clone(),
        |(_, value), next| {
            *value = next.clone();
            Ok(())
        },
    )
}

async fn inverse_ct_entries(base: &[(String, String)], diff: &XlsxOpcCtEntriesDiff) -> XlsxOpcCtEntriesDiff {
    inverse_named(base, diff, |(k, _)| k.clone(), |(_, v), _| v.clone())
}

async fn absorb_ct_entries(a: XlsxOpcCtEntriesDiff, b: XlsxOpcCtEntriesDiff) -> XlsxOpcCtEntriesDiff {
    absorb_named(a, b, |(k, _)| k.clone(), |_av, bv| bv, |(_, v), nv| *v = nv.clone())
}

async fn diff_content_types(old: &OpcContentTypes, new: &OpcContentTypes) -> Option<XlsxOpcContentTypesDiff> {
    let defaults = diff_ct_entries(&old.defaults, &new.defaults);
    let overrides = diff_ct_entries(&old.overrides, &new.overrides);
    if defaults.is_none() && overrides.is_none() {
        None
    } else {
        Some(XlsxOpcContentTypesDiff { defaults, overrides })
    }
}

async fn diff_part(old: &OpcPart, new: &OpcPart) -> Option<XlsxOpcPartDiff> {
    if old == new {
        return None;
    }
    Some(XlsxOpcPartDiff { content_type: (old.content_type != new.content_type).then(|| new.content_type.clone()), bytes: (old.bytes != new.bytes).then(|| new.bytes.clone()) })
}

async fn apply_part(part: &mut OpcPart, diff: &XlsxOpcPartDiff) {
    if let Some(v) = &diff.content_type {
        part.content_type = v.clone();
    }
    if let Some(v) = &diff.bytes {
        part.bytes = v.clone();
    }
}

async fn part_with_diff_applied(part: &OpcPart, diff: &XlsxOpcPartDiff) -> OpcPart {
    let mut out = part.clone();
    apply_part(&mut out, diff);
    out
}

async fn inverse_part(base: &OpcPart, diff: &XlsxOpcPartDiff) -> XlsxOpcPartDiff {
    XlsxOpcPartDiff { content_type: diff.content_type.as_ref().map(|_| base.content_type.clone()), bytes: diff.bytes.as_ref().map(|_| base.bytes.clone()) }
}

async fn absorb_part_diff(mut a: XlsxOpcPartDiff, b: XlsxOpcPartDiff) -> XlsxOpcPartDiff {
    if b.content_type.is_some() {
        a.content_type = b.content_type;
    }
    if b.bytes.is_some() {
        a.bytes = b.bytes;
    }
    a
}

async fn diff_parts(old: &[OpcPart], new: &[OpcPart]) -> Option<XlsxOpcPartsDiff> {
    between_named(old, new, |p| p.path.clone(), diff_part)
}

async fn diff_rel(old: &OpcRelationship, new: &OpcRelationship) -> Option<XlsxOpcRelDiff> {
    if old == new {
        return None;
    }
    Some(XlsxOpcRelDiff { rel_type: (old.rel_type != new.rel_type).then(|| new.rel_type.clone()), target: (old.target != new.target).then(|| new.target.clone()), target_mode: (old.target_mode != new.target_mode).then_some(new.target_mode) })
}

async fn apply_rel(rel: &mut OpcRelationship, diff: &XlsxOpcRelDiff) {
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

async fn inverse_rel(base: &OpcRelationship, diff: &XlsxOpcRelDiff) -> XlsxOpcRelDiff {
    XlsxOpcRelDiff { rel_type: diff.rel_type.as_ref().map(|_| base.rel_type.clone()), target: diff.target.as_ref().map(|_| base.target.clone()), target_mode: diff.target_mode.map(|_| base.target_mode) }
}

async fn absorb_rel_diff(mut a: XlsxOpcRelDiff, b: XlsxOpcRelDiff) -> XlsxOpcRelDiff {
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

async fn diff_rel_list(old: &[OpcRelationship], new: &[OpcRelationship]) -> Option<XlsxOpcRelListDiff> {
    between_named(old, new, |r| r.id.clone(), diff_rel)
}

async fn apply_rel_list(list: &mut Vec<OpcRelationship>, diff: &XlsxOpcRelListDiff) -> MutationApplyResult<()> {
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

async fn rel_list_with_diff_applied(list: &[OpcRelationship], diff: &XlsxOpcRelListDiff) -> Vec<OpcRelationship> {
    let mut out = list.to_vec();
    apply_named_for_absorb(&mut out, diff, |relationship| relationship.id.clone(), |relationship, change| apply_rel(relationship, change));
    out
}

async fn inverse_rel_list(base: &[OpcRelationship], diff: &XlsxOpcRelListDiff) -> XlsxOpcRelListDiff {
    inverse_named(base, diff, |r| r.id.clone(), inverse_rel)
}

async fn absorb_rel_list_diff(a: XlsxOpcRelListDiff, b: XlsxOpcRelListDiff) -> XlsxOpcRelListDiff {
    absorb_named(a, b, |r| r.id.clone(), absorb_rel_diff, apply_rel)
}

async fn diff_relationships(old: &HashMap<String, Vec<OpcRelationship>>, new: &HashMap<String, Vec<OpcRelationship>>) -> Option<XlsxOpcRelationshipsDiff> {
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
        Some(XlsxOpcRelationshipsDiff { removed, modified, added })
    }
}

async fn apply_relationships(rels: &mut HashMap<String, Vec<OpcRelationship>>, diff: &XlsxOpcRelationshipsDiff) -> MutationApplyResult<()> {
    for (position, owner) in diff.removed.iter().enumerate() {
        if !rels.contains_key(owner) {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "relationship owner does not exist"));
        }
        if diff.removed[..position].contains(owner) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "relationship owner removal is repeated"));
        }
    }
    for (position, m) in diff.modified.iter().enumerate() {
        if !rels.contains_key(&m.key) {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "relationship owner does not exist"));
        }
        if diff.removed.contains(&m.key) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "relationship owner is both removed and modified"));
        }
        if diff.modified[..position].iter().any(|candidate| candidate.key == m.key) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "relationship owner modification is repeated"));
        }
    }
    for (position, (owner, _)) in diff.added.iter().enumerate() {
        if rels.contains_key(owner) || diff.added[..position].iter().any(|(candidate, _)| candidate == owner) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "relationship owner already exists"));
        }
        if diff.removed.contains(owner) || diff.modified.iter().any(|candidate| candidate.key == *owner) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "relationship owner is both changed and added"));
        }
    }
    for owner in &diff.removed {
        rels.remove(owner);
    }
    for m in &diff.modified {
        let list = rels.get_mut(&m.key).ok_or_else(|| MutationApplyError::new("mutation.apply.missing-target", "relationship owner does not exist"))?;
        apply_rel_list(list, &m.diff).map_err(|error| error.under(["modified"]))?;
    }
    for (owner, list) in &diff.added {
        rels.insert(owner.clone(), list.clone());
    }
    Ok(())
}

async fn inverse_relationships(base: &HashMap<String, Vec<OpcRelationship>>, diff: &XlsxOpcRelationshipsDiff) -> XlsxOpcRelationshipsDiff {
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
    XlsxOpcRelationshipsDiff { removed, modified, added }
}

async fn absorb_relationships(d1: XlsxOpcRelationshipsDiff, d2: XlsxOpcRelationshipsDiff) -> XlsxOpcRelationshipsDiff {
    absorb_named(d1, d2, |(owner, _)| owner.clone(), absorb_rel_list_diff, |(_, list), diff| *list = rel_list_with_diff_applied(list, diff))
}

async fn diff_opc(base: &OpcPackage, other: &OpcPackage) -> Option<XlsxOpcDiff> {
    let content_types = diff_content_types(&base.content_types, &other.content_types);
    let parts = diff_parts(&base.parts, &other.parts);
    let relationships = diff_relationships(&base.relationships, &other.relationships);
    if content_types.is_none() && parts.is_none() && relationships.is_none() {
        None
    } else {
        Some(XlsxOpcDiff { content_types, parts, relationships })
    }
}

async fn apply_opc_diff(opc: &mut OpcPackage, diff: &XlsxOpcDiff) -> MutationApplyResult<()> {
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

async fn inverse_opc_diff(base: &OpcPackage, diff: &XlsxOpcDiff) -> XlsxOpcDiff {
    XlsxOpcDiff {
        content_types: diff
            .content_types
            .as_ref()
            .map(|d| XlsxOpcContentTypesDiff { defaults: d.defaults.as_ref().map(|dd| inverse_ct_entries(&base.content_types.defaults, dd)), overrides: d.overrides.as_ref().map(|dd| inverse_ct_entries(&base.content_types.overrides, dd)) }),
        parts: diff.parts.as_ref().map(|d| inverse_named(&base.parts, d, |p| p.path.clone(), inverse_part)),
        relationships: diff.relationships.as_ref().map(|d| inverse_relationships(&base.relationships, d)),
    }
}

async fn absorb_opc_diff(a: XlsxOpcDiff, b: XlsxOpcDiff) -> XlsxOpcDiff {
    XlsxOpcDiff {
        content_types: match (a.content_types, b.content_types) {
            (None, x) => x,
            (x, None) => x,
            (Some(ca), Some(cb)) => Some(XlsxOpcContentTypesDiff {
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
impl MutationDiff<XlsxSnapshot> for XlsxDiff {
    async fn apply(&self, base: &XlsxSnapshot) -> MutationApplyResult<XlsxSnapshot> {
        let mut next = base.clone();
        if let Some(d) = &self.opc {
            apply_opc_diff(&mut next.opc, d).map_err(|error| error.under(["opc"]))?;
        }
        if let Some(d) = &self.workbook {
            apply_workbook_diff(&mut next.workbook, d).map_err(|error| error.under(["workbook"]))?;
        }
        Ok(next)
    }

    async fn absorb(&mut self, other: Self) {
        self.opc = match (self.opc.take(), other.opc) {
            (None, x) => x,
            (x, None) => x,
            (Some(a), Some(b)) => Some(absorb_opc_diff(a, b)),
        };
        self.workbook = match (self.workbook.take(), other.workbook) {
            (None, x) => x,
            (x, None) => x,
            (Some(a), Some(b)) => Some(absorb_workbook_diff(a, b)),
        };
    }
}
//#endregion 🔖️Apply

//#region 🔖️DiffAlgebra
impl DiffAlgebra<XlsxSnapshot> for XlsxDiff {
    async fn inverse(&self, base: &XlsxSnapshot) -> Self {
        XlsxDiff { opc: self.opc.as_ref().map(|d| inverse_opc_diff(&base.opc, d)), workbook: self.workbook.as_ref().map(|d| inverse_workbook_diff(&base.workbook, d)) }
    }

    async fn between(base: &XlsxSnapshot, other: &XlsxSnapshot) -> Self {
        XlsxDiff { opc: diff_opc(&base.opc, &other.opc), workbook: diff_workbook(&base.workbook, &other.workbook) }
    }

    async fn is_empty(&self) -> bool {
        self.opc.is_none() && self.workbook.is_none()
    }
}
//#endregion 🔖️DiffAlgebra

//#region 🔖️MutationConstructors
/// 🧩 Builds the sparse field-by-field diff for a `SetSnapshot` mutation. No `snapshot:
/// Option<XlsxSnapshot>` full-replace slot — this IS `XlsxDiff::between`.
pub async fn diff_set_snapshot(base: &XlsxSnapshot, next: &XlsxSnapshot) -> XlsxDiff {
    XlsxDiff::between(base, next)
}

/// 🧩 Builds the diff for inserting a brand-new (possibly non-empty) sheet.
pub async fn diff_insert_sheet(sheet: XlsxSheet) -> XlsxDiff {
    XlsxDiff { opc: None, workbook: Some(XlsxWorkbookDiff { sheets: Some(XlsxSheetsDiff { added: vec![sheet], ..Default::default() }), shared_strings: None }) }
}

/// 🧩 Builds the diff for removing the sheet named `name`.
pub async fn diff_remove_sheet(name: &str) -> XlsxDiff {
    XlsxDiff { opc: None, workbook: Some(XlsxWorkbookDiff { sheets: Some(XlsxSheetsDiff { removed: vec![name.to_string()], ..Default::default() }), shared_strings: None }) }
}

/// 🧩 Builds the diff for renaming a sheet — `name` is the sheet's KEY (identity), so a rename is
/// a remove-old-name + add-new-name-with-full-content at the diff level (documented in the
/// snapshot module's doc comment, same category as docx's OPC-part-rename gotcha).
pub async fn diff_rename_sheet(old_sheet: &XlsxSheet, new_name: &str) -> XlsxDiff {
    if old_sheet.name == new_name {
        return XlsxDiff::default();
    }
    let renamed = XlsxSheet { name: new_name.to_string(), cells: old_sheet.cells.clone() };
    XlsxDiff { opc: None, workbook: Some(XlsxWorkbookDiff { sheets: Some(XlsxSheetsDiff { removed: vec![old_sheet.name.clone()], added: vec![renamed], ..Default::default() }), shared_strings: None }) }
}

/// 🧩 Builds the diff for setting (inserting or replacing) one cell's value in sheet `sheet_name`.
pub async fn diff_set_cell(sheet: &XlsxSheet, row: u32, col: u32, value: XlsxCellValue) -> XlsxDiff {
    let sheet_diff = match sheet.cells.iter().find(|c| c.row == row && c.col == col) {
        Some(existing) if existing.value == value => return XlsxDiff::default(),
        Some(_) => XlsxSheetDiff { cells: Some(XlsxCellsDiff { modified: vec![NamedModified { key: (row, col), diff: XlsxCellDiff { value: Some(value) } }], ..Default::default() }) },
        None => XlsxSheetDiff { cells: Some(XlsxCellsDiff { added: vec![XlsxCell { row, col, value }], ..Default::default() }) },
    };
    XlsxDiff { opc: None, workbook: Some(XlsxWorkbookDiff { sheets: Some(XlsxSheetsDiff { modified: vec![NamedModified { key: sheet.name.clone(), diff: sheet_diff }], ..Default::default() }), shared_strings: None }) }
}

/// 🧩 Builds the diff for removing the cell at `(row, col)` in sheet `sheet_name`.
pub async fn diff_remove_cell(sheet_name: &str, row: u32, col: u32) -> XlsxDiff {
    let sheet_diff = XlsxSheetDiff { cells: Some(XlsxCellsDiff { removed: vec![(row, col)], ..Default::default() }) };
    XlsxDiff { opc: None, workbook: Some(XlsxWorkbookDiff { sheets: Some(XlsxSheetsDiff { modified: vec![NamedModified { key: sheet_name.to_string(), diff: sheet_diff }], ..Default::default() }), shared_strings: None }) }
}

/// 🧩 Builds the diff for appending a new shared string, returning its assigned index alongside
/// the diff (callers building `SharedString(idx)` cell values need the index up front).
pub async fn diff_insert_shared_string(existing_len: usize, value: &str) -> (usize, XlsxDiff) {
    let idx = existing_len;
    let diff = XlsxDiff { opc: None, workbook: Some(XlsxWorkbookDiff { sheets: None, shared_strings: Some(XlsxSharedStringsDiff { added: vec![(idx, value.to_string())], ..Default::default() }) }) };
    (idx, diff)
}

/// 🧩 Builds the diff for removing the shared string at `index` (any cell still referencing it
/// by index is the caller's responsibility — mirrors how zip/OPC name-keyed removal never
/// cascades into referrers).
pub async fn diff_remove_shared_string(index: usize) -> XlsxDiff {
    XlsxDiff { opc: None, workbook: Some(XlsxWorkbookDiff { sheets: None, shared_strings: Some(XlsxSharedStringsDiff { removed: vec![index], ..Default::default() }) }) }
}

/// 🧩 Builds the diff for replacing the shared string at `index`. `index == strings.len()` is
/// treated as an APPEND (an `added` entry, not a no-op `modified` patch onto a nonexistent key) —
/// this is what makes `RemoveSharedString`'s mutation-level inverse (`SetSharedString` at the
/// removed index) actually restore the shared string when it was the LAST one (same documented
/// last-position caveat as docx's `RemovePart`/svg's `SetAttribute{value:None}` precedent — exact
/// positional restoration for a non-last removal is only guaranteed at the diff level, not via a
/// reconstructed mutation). `index > strings.len()` (a genuine gap) is a graceful no-op per the
/// recipe's out-of-range-key convention.
pub async fn diff_set_shared_string(strings: &[String], index: usize, value: &str) -> XlsxDiff {
    let shared_strings_diff = match strings.get(index) {
        Some(existing) if existing == value => None,
        Some(_) => Some(XlsxSharedStringsDiff { modified: vec![NamedModified { key: index, diff: value.to_string() }], ..Default::default() }),
        None if index == strings.len() => Some(XlsxSharedStringsDiff { added: vec![(index, value.to_string())], ..Default::default() }),
        None => None,
    };
    match shared_strings_diff {
        None => XlsxDiff::default(),
        Some(ssd) => XlsxDiff { opc: None, workbook: Some(XlsxWorkbookDiff { sheets: None, shared_strings: Some(ssd) }) },
    }
}
//#endregion 🔖️MutationConstructors

//#region 🔖️HandcraftedDiffCodec
/// 🧪️ F6: **hand-rolled** `protocol::DiffCodec` for `XlsxDiff` — required per the doc comment on
/// `XlsxDiff` itself (real `cargo check` failure: `XlsxCellValue: DslField` not satisfied, plus the
/// generic `NamedTripleDiff<K,D,T>` collection type has no `DslField` impl either). Same grammar
/// style `GifDiff`/`SvgDiff`'s hand-rolled codecs use (bracket-depth-aware split, hex for
/// strings/bytes, `[0]`/`[1,x]` for `Option<T>`, `[removed];[modified];[added]` for collection
/// triples) — see `f6-recon-report.md` §5 for the primitive rationale; this file re-derives its own
/// copies of the small helper functions since each hand-rolled codec is self-contained (no shared
/// "hand-roll helpers" module exists yet). One addition beyond the gif/svg precedent: a GENERIC
/// `enc_triple`/`dec_triple` pair, since `NamedTripleDiff<K,D,T>` is reused across SIX distinct
/// `(K,D,T)` instantiations in this file (cells/sheets/shared_strings/ct-entries/parts/rel-lists,
/// plus relationships nesting a rel-list triple as its OWN `D`) — writing six near-identical
/// bespoke encoders would violate this ticket's "concise code" rule for no benefit.
//#region 🔖️Primitives
pub(crate) async fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
pub(crate) async fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
pub(crate) async fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
pub(crate) async fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
pub(crate) async fn parse_u32(s: &str) -> Result<u32, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}
pub(crate) async fn parse_usize(s: &str) -> Result<usize, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}
/// 🔢️ `f64::to_string`/`str::parse::<f64>()` round-trip exactly (std's shortest-round-trip float
/// formatting) — no manual bit-pattern encoding needed. None of `.`/`-`/`e`/`inf`/`NaN` clash with
/// this grammar's `,`/`;`/`:`/`[`/`]` separators.
pub(crate) async fn enc_f64(n: f64) -> String {
    n.to_string()
}
pub(crate) async fn dec_f64(s: &str) -> Result<f64, String> {
    s.parse().map_err(|e: std::num::ParseFloatError| e.to_string())
}
pub(crate) async fn split_top_level(s: &str, sep: char) -> Vec<&str> {
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
pub(crate) async fn strip_brackets(s: &str) -> Result<&str, String> {
    s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))
}
pub(crate) async fn encode_option<T>(opt: &Option<T>, enc: impl Fn(&T) -> String) -> String {
    match opt {
        None => "[0]".to_string(),
        Some(v) => format!("[1,{}]", enc(v)),
    }
}
pub(crate) async fn decode_option<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Option<T>, String> {
    let inner = strip_brackets(s)?;
    match split_top_level(inner, ',').as_slice() {
        ["0"] => Ok(None),
        [tag, value] if *tag == "1" => Ok(Some(dec(value)?)),
        other => Err(format!("option decode: bad shape {other:?}")),
    }
}
//#endregion 🔖️Primitives

//#region 🔖️GenericTripleCodec
/// 🧮️ `[removed];[modified];[added]` — `removed` is comma-joined encoded keys, `modified` is
/// comma-joined `key:diff` entries, `added` is comma-joined encoded items. Keys are always
/// hex/decimal (never contain a literal `:`), so `entry.split_once(':')` unambiguously separates a
/// `modified` entry's key from its (possibly itself bracket-nested, comma-and-semicolon-bearing)
/// diff body — same reasoning `f6-recon-report.md` §5 documents for collection-triple entries.
async fn enc_triple<K, D, T>(triple: &NamedTripleDiff<K, D, T>, enc_key: impl Fn(&K) -> String, enc_diff: impl Fn(&D) -> String, enc_item: impl Fn(&T) -> String) -> String {
    let removed = triple.removed.iter().map(|k| enc_key(k)).collect::<Vec<_>>().join(",");
    let modified = triple.modified.iter().map(|m| format!("{}:{}", enc_key(&m.key), enc_diff(&m.diff))).collect::<Vec<_>>().join(",");
    let added = triple.added.iter().map(|t| enc_item(t)).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}
async fn dec_triple<K, D, T>(body: &str, dec_key: impl Fn(&str) -> Result<K, String>, dec_diff: impl Fn(&str) -> Result<D, String>, dec_item: impl Fn(&str) -> Result<T, String>) -> Result<NamedTripleDiff<K, D, T>, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("triple: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().map(|s| dec_key(s)).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .map(|entry| {
            let (k, d) = entry.split_once(':').ok_or_else(|| format!("triple modified: missing ':' in {entry:?}"))?;
            Ok(NamedModified { key: dec_key(k)?, diff: dec_diff(d)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',').into_iter().map(|s| dec_item(s)).collect::<Result<Vec<_>, String>>()?;
    Ok(NamedTripleDiff { removed, modified, added })
}
//#endregion 🔖️GenericTripleCodec

//#region 🔖️CellValueCodec
/// 🔢️ `N[f64]`/`S[usize]`/`I[hex]`/`B[0|1]`/`F[expr_hex,cached_option]`/`E[]` — single-uppercase-
/// letter tag prefix immediately followed by the bracketed positional payload, same convention
/// `f6-recon-report.md` §5 and `SvgDiff`'s `enc_xml_node` use for data-carrying enums.
pub(crate) async fn enc_cell_value(v: &XlsxCellValue) -> String {
    match v {
        XlsxCellValue::Number(n) => format!("N[{}]", enc_f64(*n)),
        XlsxCellValue::SharedString(i) => format!("S[{i}]"),
        XlsxCellValue::InlineString(s) => format!("I[{}]", enc_str(s)),
        XlsxCellValue::Boolean(b) => format!("B[{}]", if *b { "1" } else { "0" }),
        XlsxCellValue::Formula { expr, cached } => format!("F[{},{}]", enc_str(expr), encode_option(cached, |c| enc_cell_value(c))),
        XlsxCellValue::Empty => "E[]".to_string(),
    }
}
pub(crate) async fn dec_cell_value(s: &str) -> Result<XlsxCellValue, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "N" => Ok(XlsxCellValue::Number(dec_f64(inner)?)),
        "S" => Ok(XlsxCellValue::SharedString(parse_usize(inner)?)),
        "I" => Ok(XlsxCellValue::InlineString(dec_str(inner)?)),
        "B" => Ok(XlsxCellValue::Boolean(inner == "1")),
        "F" => {
            let parts = split_top_level(inner, ',');
            let [expr, cached] = parts.as_slice() else { return Err(format!("formula: expected 2 fields, got {}", parts.len())) };
            Ok(XlsxCellValue::Formula { expr: dec_str(expr)?, cached: decode_option(cached, |c| dec_cell_value(c).map(Box::new))? })
        }
        "E" => Ok(XlsxCellValue::Empty),
        other => Err(format!("cell value: unknown tag {other:?}")),
    }
}
pub(crate) async fn enc_cell_key(k: &(u32, u32)) -> String {
    format!("[{},{}]", k.0, k.1)
}
pub(crate) async fn dec_cell_key(s: &str) -> Result<(u32, u32), String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [row, col] = parts.as_slice() else { return Err(format!("cell key: expected 2 fields, got {}", parts.len())) };
    Ok((parse_u32(row)?, parse_u32(col)?))
}
pub(crate) async fn enc_cell(c: &XlsxCell) -> String {
    format!("[{},{},{}]", c.row, c.col, enc_cell_value(&c.value))
}
pub(crate) async fn dec_cell(s: &str) -> Result<XlsxCell, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [row, col, value] = parts.as_slice() else { return Err(format!("cell: expected 3 fields, got {}", parts.len())) };
    Ok(XlsxCell { row: parse_u32(row)?, col: parse_u32(col)?, value: dec_cell_value(value)? })
}
async fn enc_cell_diff(d: &XlsxCellDiff) -> String {
    encode_option(&d.value, |v| enc_cell_value(v))
}
async fn dec_cell_diff(s: &str) -> Result<XlsxCellDiff, String> {
    Ok(XlsxCellDiff { value: decode_option(s, dec_cell_value)? })
}
async fn enc_cells_diff(t: &XlsxCellsDiff) -> String {
    enc_triple(t, enc_cell_key, enc_cell_diff, enc_cell)
}
async fn dec_cells_diff(s: &str) -> Result<XlsxCellsDiff, String> {
    dec_triple(s, dec_cell_key, dec_cell_diff, dec_cell)
}
//#endregion 🔖️CellValueCodec

//#region 🔖️WorkbookCodec
pub(crate) async fn enc_sheet(s: &XlsxSheet) -> String {
    let cells = s.cells.iter().map(enc_cell).collect::<Vec<_>>().join(",");
    format!("[{},[{}]]", enc_str(&s.name), cells)
}
pub(crate) async fn dec_sheet(s: &str) -> Result<XlsxSheet, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, cells] = parts.as_slice() else { return Err(format!("sheet: expected 2 fields, got {}", parts.len())) };
    let cells = split_top_level(strip_brackets(cells)?, ',').into_iter().map(dec_cell).collect::<Result<Vec<_>, String>>()?;
    Ok(XlsxSheet { name: dec_str(name)?, cells })
}
async fn enc_sheet_diff(d: &XlsxSheetDiff) -> String {
    encode_option(&d.cells, |c| enc_cells_diff(c))
}
async fn dec_sheet_diff(s: &str) -> Result<XlsxSheetDiff, String> {
    Ok(XlsxSheetDiff { cells: decode_option(s, dec_cells_diff)? })
}
async fn enc_sheets_diff(t: &XlsxSheetsDiff) -> String {
    enc_triple(t, |k| enc_str(k), enc_sheet_diff, enc_sheet)
}
async fn dec_sheets_diff(s: &str) -> Result<XlsxSheetsDiff, String> {
    dec_triple(s, dec_str, dec_sheet_diff, dec_sheet)
}
async fn enc_shared_string_item(item: &(usize, String)) -> String {
    format!("[{},{}]", item.0, enc_str(&item.1))
}
async fn dec_shared_string_item(s: &str) -> Result<(usize, String), String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [idx, value] = parts.as_slice() else { return Err(format!("shared string item: expected 2 fields, got {}", parts.len())) };
    Ok((parse_usize(idx)?, dec_str(value)?))
}
async fn enc_shared_strings_diff(t: &XlsxSharedStringsDiff) -> String {
    enc_triple(t, |k| k.to_string(), |d| enc_str(d), enc_shared_string_item)
}
async fn dec_shared_strings_diff(s: &str) -> Result<XlsxSharedStringsDiff, String> {
    dec_triple(s, parse_usize, dec_str, dec_shared_string_item)
}
async fn enc_workbook_diff(d: &XlsxWorkbookDiff) -> String {
    format!("[{},{}]", encode_option(&d.sheets, |t| enc_sheets_diff(t)), encode_option(&d.shared_strings, |t| enc_shared_strings_diff(t)))
}
async fn dec_workbook_diff(s: &str) -> Result<XlsxWorkbookDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [sheets, shared_strings] = parts.as_slice() else { return Err(format!("workbook diff: expected 2 fields, got {}", parts.len())) };
    Ok(XlsxWorkbookDiff { sheets: decode_option(sheets, dec_sheets_diff)?, shared_strings: decode_option(shared_strings, dec_shared_strings_diff)? })
}
//#endregion 🔖️WorkbookCodec

//#region 🔖️OpcCodec
pub(crate) async fn enc_ct_entry(item: &(String, String)) -> String {
    format!("[{},{}]", enc_str(&item.0), enc_str(&item.1))
}
pub(crate) async fn dec_ct_entry(s: &str) -> Result<(String, String), String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [k, v] = parts.as_slice() else { return Err(format!("ct entry: expected 2 fields, got {}", parts.len())) };
    Ok((dec_str(k)?, dec_str(v)?))
}
async fn enc_ct_entries_diff(t: &XlsxOpcCtEntriesDiff) -> String {
    enc_triple(t, |k| enc_str(k), |d| enc_str(d), enc_ct_entry)
}
async fn dec_ct_entries_diff(s: &str) -> Result<XlsxOpcCtEntriesDiff, String> {
    dec_triple(s, dec_str, dec_str, dec_ct_entry)
}
async fn enc_content_types_diff(d: &XlsxOpcContentTypesDiff) -> String {
    format!("[{},{}]", encode_option(&d.defaults, |t| enc_ct_entries_diff(t)), encode_option(&d.overrides, |t| enc_ct_entries_diff(t)))
}
async fn dec_content_types_diff(s: &str) -> Result<XlsxOpcContentTypesDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [defaults, overrides] = parts.as_slice() else { return Err(format!("content types diff: expected 2 fields, got {}", parts.len())) };
    Ok(XlsxOpcContentTypesDiff { defaults: decode_option(defaults, dec_ct_entries_diff)?, overrides: decode_option(overrides, dec_ct_entries_diff)? })
}
pub(crate) async fn enc_part(p: &OpcPart) -> String {
    format!("[{},{},{}]", enc_str(&p.path), enc_str(&p.content_type), hex_encode(&p.bytes))
}
pub(crate) async fn dec_part(s: &str) -> Result<OpcPart, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [path, ct, bytes] = parts.as_slice() else { return Err(format!("opc part: expected 3 fields, got {}", parts.len())) };
    Ok(OpcPart { path: dec_str(path)?, content_type: dec_str(ct)?, bytes: hex_decode(bytes)? })
}
async fn enc_part_diff(d: &XlsxOpcPartDiff) -> String {
    format!("[{},{}]", encode_option(&d.content_type, |v| enc_str(v)), encode_option(&d.bytes, |v| hex_encode(v)))
}
async fn dec_part_diff(s: &str) -> Result<XlsxOpcPartDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [ct, bytes] = parts.as_slice() else { return Err(format!("opc part diff: expected 2 fields, got {}", parts.len())) };
    Ok(XlsxOpcPartDiff { content_type: decode_option(ct, dec_str)?, bytes: decode_option(bytes, hex_decode)? })
}
async fn enc_parts_diff(t: &XlsxOpcPartsDiff) -> String {
    enc_triple(t, |k| enc_str(k), enc_part_diff, enc_part)
}
async fn dec_parts_diff(s: &str) -> Result<XlsxOpcPartsDiff, String> {
    dec_triple(s, dec_str, dec_part_diff, dec_part)
}
pub(crate) async fn enc_target_mode(m: &OpcTargetMode) -> String {
    match m {
        OpcTargetMode::Internal => "0".to_string(),
        OpcTargetMode::External => "1".to_string(),
    }
}
pub(crate) async fn dec_target_mode(s: &str) -> Result<OpcTargetMode, String> {
    match s {
        "0" => Ok(OpcTargetMode::Internal),
        "1" => Ok(OpcTargetMode::External),
        other => Err(format!("target mode: unknown {other:?}")),
    }
}
pub(crate) async fn enc_rel(r: &OpcRelationship) -> String {
    format!("[{},{},{},{}]", enc_str(&r.id), enc_str(&r.rel_type), enc_str(&r.target), enc_target_mode(&r.target_mode))
}
pub(crate) async fn dec_rel(s: &str) -> Result<OpcRelationship, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, rel_type, target, mode] = parts.as_slice() else { return Err(format!("opc rel: expected 4 fields, got {}", parts.len())) };
    Ok(OpcRelationship { id: dec_str(id)?, rel_type: dec_str(rel_type)?, target: dec_str(target)?, target_mode: dec_target_mode(mode)? })
}
async fn enc_rel_diff(d: &XlsxOpcRelDiff) -> String {
    format!("[{},{},{}]", encode_option(&d.rel_type, |v| enc_str(v)), encode_option(&d.target, |v| enc_str(v)), encode_option(&d.target_mode, |v| enc_target_mode(v)),)
}
async fn dec_rel_diff(s: &str) -> Result<XlsxOpcRelDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [rel_type, target, mode] = parts.as_slice() else { return Err(format!("opc rel diff: expected 3 fields, got {}", parts.len())) };
    Ok(XlsxOpcRelDiff { rel_type: decode_option(rel_type, dec_str)?, target: decode_option(target, dec_str)?, target_mode: decode_option(mode, dec_target_mode)? })
}
async fn enc_rel_list_diff(t: &XlsxOpcRelListDiff) -> String {
    enc_triple(t, |k| enc_str(k), enc_rel_diff, enc_rel)
}
async fn dec_rel_list_diff(s: &str) -> Result<XlsxOpcRelListDiff, String> {
    dec_triple(s, dec_str, dec_rel_diff, dec_rel)
}
pub(crate) async fn enc_owner_rels(item: &(String, Vec<OpcRelationship>)) -> String {
    let rels = item.1.iter().map(enc_rel).collect::<Vec<_>>().join(",");
    format!("[{},[{}]]", enc_str(&item.0), rels)
}
pub(crate) async fn dec_owner_rels(s: &str) -> Result<(String, Vec<OpcRelationship>), String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [owner, rels] = parts.as_slice() else { return Err(format!("owner rels: expected 2 fields, got {}", parts.len())) };
    let rels = split_top_level(strip_brackets(rels)?, ',').into_iter().map(dec_rel).collect::<Result<Vec<_>, String>>()?;
    Ok((dec_str(owner)?, rels))
}
async fn enc_relationships_diff(t: &XlsxOpcRelationshipsDiff) -> String {
    enc_triple(t, |k| enc_str(k), enc_rel_list_diff, enc_owner_rels)
}
async fn dec_relationships_diff(s: &str) -> Result<XlsxOpcRelationshipsDiff, String> {
    dec_triple(s, dec_str, dec_rel_list_diff, dec_owner_rels)
}
async fn enc_opc_diff(d: &XlsxOpcDiff) -> String {
    format!("[{},{},{}]", encode_option(&d.content_types, |v| enc_content_types_diff(v)), encode_option(&d.parts, |v| enc_parts_diff(v)), encode_option(&d.relationships, |v| enc_relationships_diff(v)),)
}
async fn dec_opc_diff(s: &str) -> Result<XlsxOpcDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [ct, p, r] = parts.as_slice() else { return Err(format!("opc diff: expected 3 fields, got {}", parts.len())) };
    Ok(XlsxOpcDiff { content_types: decode_option(ct, dec_content_types_diff)?, parts: decode_option(p, dec_parts_diff)?, relationships: decode_option(r, dec_relationships_diff)? })
}
//#endregion 🔖️OpcCodec

//#region 🔖️BinaryCodecs
/// 🧪️ FG-wave: real recursive BINARY twins of every text-form codec above, backing the upgraded
/// `DiffCodec::encode_diff`/`decode_diff` below (and, via re-export, `../🧬️mutations/🦀️component.rs`'s
/// own upgraded `OpBinary`) — replaces F6's `print_diff().into_bytes()` text-as-binary shortcut.
/// Real LEB128-varint-framed length-prefixed strings/bytes (`store::pack_rt::write_varint_u64` +
/// `store::ByteReader`), 1-byte tri-state presence tags, and 1-byte enum-variant tags — genuinely
/// structured binary, never hex-ASCII text reused as "binary". Same shape docx's own
/// `🔺️diff/🦀️component.rs` `BinaryPrimitives`/`ValueBinaryCodecs`/`GenericTripleBinaryCodecs`/
/// `DiffValueBinaryCodecs` regions establish (this wave's OPC pattern-setter); duplicated here
/// (not imported) per this repo's per-artifact hand-roll convention (no shared "hand-roll
/// helpers" module exists yet, see this file's own `HandcraftedDiffCodec` doc comment).
//#region 🔖️BinaryPrimitives
pub(crate) async fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}
pub(crate) async fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}
pub(crate) async fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}
pub(crate) async fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}
//#endregion 🔖️BinaryPrimitives

//#region 🔖️ValueBinaryCodecs
/// 🌳️ Full-item (non-diff) binary codecs, mirrored one-for-one against `../🔖️CellValueCodec`/
/// `../🔖️WorkbookCodec`/`../🔖️OpcCodec`'s text forms above. `pub(crate)` so
/// `../🧬️mutations/🦀️component.rs` reuses these rather than re-deriving its own copies (same
/// intra-artifact reuse pattern the text codecs already use).
pub(crate) async fn enc_target_mode_bin(m: &OpcTargetMode, out: &mut Vec<u8>) {
    out.push(match m {
        OpcTargetMode::Internal => 0,
        OpcTargetMode::External => 1,
    });
}
pub(crate) async fn dec_target_mode_bin(reader: &mut store::ByteReader<'_>) -> Result<OpcTargetMode, String> {
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => Ok(OpcTargetMode::Internal),
        1 => Ok(OpcTargetMode::External),
        other => Err(format!("target mode binary: bad value {other}")),
    }
}

pub(crate) async fn enc_opc_part_bin(p: &OpcPart, out: &mut Vec<u8>) {
    write_str_lp(out, &p.path);
    write_str_lp(out, &p.content_type);
    write_bytes_lp(out, &p.bytes);
}
pub(crate) async fn dec_opc_part_bin(reader: &mut store::ByteReader<'_>) -> Result<OpcPart, String> {
    let path = read_str_lp(reader)?;
    let content_type = read_str_lp(reader)?;
    let bytes = read_bytes_lp(reader)?;
    Ok(OpcPart { path, content_type, bytes })
}

pub(crate) async fn enc_rel_bin(r: &OpcRelationship, out: &mut Vec<u8>) {
    write_str_lp(out, &r.id);
    write_str_lp(out, &r.rel_type);
    write_str_lp(out, &r.target);
    enc_target_mode_bin(&r.target_mode, out);
}
pub(crate) async fn dec_rel_bin(reader: &mut store::ByteReader<'_>) -> Result<OpcRelationship, String> {
    let id = read_str_lp(reader)?;
    let rel_type = read_str_lp(reader)?;
    let target = read_str_lp(reader)?;
    let target_mode = dec_target_mode_bin(reader)?;
    Ok(OpcRelationship { id, rel_type, target, target_mode })
}

pub(crate) async fn enc_ct_entry_bin(e: &(String, String), out: &mut Vec<u8>) {
    write_str_lp(out, &e.0);
    write_str_lp(out, &e.1);
}
pub(crate) async fn dec_ct_entry_bin(reader: &mut store::ByteReader<'_>) -> Result<(String, String), String> {
    let k = read_str_lp(reader)?;
    let v = read_str_lp(reader)?;
    Ok((k, v))
}

/// 🗺️ One `relationships` map entry (owner path -> that owner's relationship list).
pub(crate) async fn enc_owner_rels_bin(e: &(String, Vec<OpcRelationship>), out: &mut Vec<u8>) {
    write_str_lp(out, &e.0);
    store::pack_rt::write_varint_u64(out, e.1.len() as u64);
    for r in &e.1 {
        enc_rel_bin(r, out);
    }
}
pub(crate) async fn dec_owner_rels_bin(reader: &mut store::ByteReader<'_>) -> Result<(String, Vec<OpcRelationship>), String> {
    let owner = read_str_lp(reader)?;
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut list = Vec::with_capacity(count as usize);
    for _ in 0..count {
        list.push(dec_rel_bin(reader)?);
    }
    Ok((owner, list))
}

/// 🔢️ `XlsxCellValue` (data-carrying enum) -- 1-byte kind tag (`0`=Number/`1`=SharedString/
/// `2`=InlineString/`3`=Boolean/`4`=Formula/`5`=Empty), matching `enc_cell_value`'s own
/// `N`/`S`/`I`/`B`/`F`/`E` text-tag numbering.
pub(crate) async fn enc_cell_value_bin(v: &XlsxCellValue, out: &mut Vec<u8>) {
    match v {
        XlsxCellValue::Number(n) => {
            out.push(0);
            out.extend_from_slice(&n.to_le_bytes());
        }
        XlsxCellValue::SharedString(i) => {
            out.push(1);
            store::pack_rt::write_varint_u64(out, *i as u64);
        }
        XlsxCellValue::InlineString(s) => {
            out.push(2);
            write_str_lp(out, s);
        }
        XlsxCellValue::Boolean(b) => {
            out.push(3);
            out.push(*b as u8);
        }
        XlsxCellValue::Formula { expr, cached } => {
            out.push(4);
            write_str_lp(out, expr);
            out.push(if cached.is_some() { 1 } else { 0 });
            if let Some(c) = cached {
                enc_cell_value_bin(c, out);
            }
        }
        XlsxCellValue::Empty => out.push(5),
    }
}
pub(crate) async fn dec_cell_value_bin(reader: &mut store::ByteReader<'_>) -> Result<XlsxCellValue, String> {
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => {
            let bytes = reader.read_bytes(8).map_err(|e| e.to_string())?;
            let arr: [u8; 8] = bytes.try_into().map_err(|_| "cell value binary: short f64".to_string())?;
            Ok(XlsxCellValue::Number(f64::from_le_bytes(arr)))
        }
        1 => Ok(XlsxCellValue::SharedString(reader.read_varint_u64().map_err(|e| e.to_string())? as usize)),
        2 => Ok(XlsxCellValue::InlineString(read_str_lp(reader)?)),
        3 => Ok(XlsxCellValue::Boolean(reader.read_u8().map_err(|e| e.to_string())? != 0)),
        4 => {
            let expr = read_str_lp(reader)?;
            let cached = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(Box::new(dec_cell_value_bin(reader)?)) } else { None };
            Ok(XlsxCellValue::Formula { expr, cached })
        }
        5 => Ok(XlsxCellValue::Empty),
        other => Err(format!("cell value binary: unknown tag {other}")),
    }
}

pub(crate) async fn enc_cell_bin(c: &XlsxCell, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, c.row as u64);
    store::pack_rt::write_varint_u64(out, c.col as u64);
    enc_cell_value_bin(&c.value, out);
}
pub(crate) async fn dec_cell_bin(reader: &mut store::ByteReader<'_>) -> Result<XlsxCell, String> {
    let row = reader.read_varint_u64().map_err(|e| e.to_string())? as u32;
    let col = reader.read_varint_u64().map_err(|e| e.to_string())? as u32;
    let value = dec_cell_value_bin(reader)?;
    Ok(XlsxCell { row, col, value })
}

pub(crate) async fn enc_sheet_bin(s: &XlsxSheet, out: &mut Vec<u8>) {
    write_str_lp(out, &s.name);
    store::pack_rt::write_varint_u64(out, s.cells.len() as u64);
    for c in &s.cells {
        enc_cell_bin(c, out);
    }
}
pub(crate) async fn dec_sheet_bin(reader: &mut store::ByteReader<'_>) -> Result<XlsxSheet, String> {
    let name = read_str_lp(reader)?;
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut cells = Vec::with_capacity(count as usize);
    for _ in 0..count {
        cells.push(dec_cell_bin(reader)?);
    }
    Ok(XlsxSheet { name, cells })
}
//#endregion 🔖️ValueBinaryCodecs

//#region 🔖️GenericTripleBinaryCodecs
/// 🏷️ Binary twin of `enc_triple`/`dec_triple` -- three varint-counted sections (removed keys /
/// modified key+diff pairs / added whole items), generic over `K`/`D`/`T`. Xlsx has no
/// INDEX-positional collection (unlike docx's `body`/`runs`/`rows`/`cells`) -- `shared_strings` is
/// modeled as a NAME-keyed (here `usize`-keyed) `NamedTripleDiff` too, so only this one generic
/// binary twin is needed (docx's sibling `IndexedTripleDiff` binary twin has no xlsx counterpart).
async fn enc_named_triple_bin<K, D, T>(diff: &NamedTripleDiff<K, D, T>, enc_k: impl Fn(&K, &mut Vec<u8>), enc_d: impl Fn(&D, &mut Vec<u8>), enc_t: impl Fn(&T, &mut Vec<u8>), out: &mut Vec<u8>) {
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
async fn dec_named_triple_bin<K, D, T>(
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
async fn enc_cell_key_bin(k: &(u32, u32), out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, k.0 as u64);
    store::pack_rt::write_varint_u64(out, k.1 as u64);
}
async fn dec_cell_key_bin(reader: &mut store::ByteReader<'_>) -> Result<(u32, u32), String> {
    let row = reader.read_varint_u64().map_err(|e| e.to_string())? as u32;
    let col = reader.read_varint_u64().map_err(|e| e.to_string())? as u32;
    Ok((row, col))
}

async fn enc_cell_diff_bin(d: &XlsxCellDiff, out: &mut Vec<u8>) {
    out.push(if d.value.is_some() { 1 } else { 0 });
    if let Some(v) = &d.value {
        enc_cell_value_bin(v, out);
    }
}
async fn dec_cell_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<XlsxCellDiff, String> {
    let value = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_cell_value_bin(reader)?) } else { None };
    Ok(XlsxCellDiff { value })
}

async fn enc_cells_diff_bin(d: &XlsxCellsDiff, out: &mut Vec<u8>) {
    enc_named_triple_bin(d, enc_cell_key_bin, enc_cell_diff_bin, enc_cell_bin, out)
}
async fn dec_cells_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<XlsxCellsDiff, String> {
    dec_named_triple_bin(reader, dec_cell_key_bin, dec_cell_diff_bin, dec_cell_bin)
}

async fn enc_sheet_diff_bin(d: &XlsxSheetDiff, out: &mut Vec<u8>) {
    out.push(if d.cells.is_some() { 1 } else { 0 });
    if let Some(c) = &d.cells {
        enc_cells_diff_bin(c, out);
    }
}
async fn dec_sheet_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<XlsxSheetDiff, String> {
    let cells = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_cells_diff_bin(reader)?) } else { None };
    Ok(XlsxSheetDiff { cells })
}

async fn enc_sheets_diff_bin(d: &XlsxSheetsDiff, out: &mut Vec<u8>) {
    enc_named_triple_bin(d, |k, out| write_str_lp(out, k), enc_sheet_diff_bin, enc_sheet_bin, out)
}
async fn dec_sheets_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<XlsxSheetsDiff, String> {
    dec_named_triple_bin(reader, |r| read_str_lp(r), dec_sheet_diff_bin, dec_sheet_bin)
}

async fn enc_shared_string_item_bin(item: &(usize, String), out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, item.0 as u64);
    write_str_lp(out, &item.1);
}
async fn dec_shared_string_item_bin(reader: &mut store::ByteReader<'_>) -> Result<(usize, String), String> {
    let idx = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    let value = read_str_lp(reader)?;
    Ok((idx, value))
}

async fn enc_shared_strings_diff_bin(d: &XlsxSharedStringsDiff, out: &mut Vec<u8>) {
    enc_named_triple_bin(d, |k, out| store::pack_rt::write_varint_u64(out, *k as u64), |v: &String, out| write_str_lp(out, v), enc_shared_string_item_bin, out)
}
async fn dec_shared_strings_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<XlsxSharedStringsDiff, String> {
    dec_named_triple_bin(reader, |r| Ok(r.read_varint_u64().map_err(|e| e.to_string())? as usize), |r| read_str_lp(r), dec_shared_string_item_bin)
}

pub(crate) async fn enc_workbook_diff_bin(d: &XlsxWorkbookDiff, out: &mut Vec<u8>) {
    out.push(if d.sheets.is_some() { 1 } else { 0 });
    if let Some(v) = &d.sheets {
        enc_sheets_diff_bin(v, out);
    }
    out.push(if d.shared_strings.is_some() { 1 } else { 0 });
    if let Some(v) = &d.shared_strings {
        enc_shared_strings_diff_bin(v, out);
    }
}
pub(crate) async fn dec_workbook_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<XlsxWorkbookDiff, String> {
    let sheets = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_sheets_diff_bin(reader)?) } else { None };
    let shared_strings = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_shared_strings_diff_bin(reader)?) } else { None };
    Ok(XlsxWorkbookDiff { sheets, shared_strings })
}

async fn enc_ct_entries_diff_bin(d: &XlsxOpcCtEntriesDiff, out: &mut Vec<u8>) {
    enc_named_triple_bin(d, |k, out| write_str_lp(out, k), |v: &String, out| write_str_lp(out, v), enc_ct_entry_bin, out)
}
async fn dec_ct_entries_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<XlsxOpcCtEntriesDiff, String> {
    dec_named_triple_bin(reader, |r| read_str_lp(r), |r| read_str_lp(r), dec_ct_entry_bin)
}

async fn enc_content_types_diff_bin(d: &XlsxOpcContentTypesDiff, out: &mut Vec<u8>) {
    out.push(if d.defaults.is_some() { 1 } else { 0 });
    if let Some(v) = &d.defaults {
        enc_ct_entries_diff_bin(v, out);
    }
    out.push(if d.overrides.is_some() { 1 } else { 0 });
    if let Some(v) = &d.overrides {
        enc_ct_entries_diff_bin(v, out);
    }
}
async fn dec_content_types_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<XlsxOpcContentTypesDiff, String> {
    let defaults = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_ct_entries_diff_bin(reader)?) } else { None };
    let overrides = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_ct_entries_diff_bin(reader)?) } else { None };
    Ok(XlsxOpcContentTypesDiff { defaults, overrides })
}

async fn enc_opc_part_diff_bin(d: &XlsxOpcPartDiff, out: &mut Vec<u8>) {
    out.push(if d.content_type.is_some() { 1 } else { 0 });
    if let Some(v) = &d.content_type {
        write_str_lp(out, v);
    }
    out.push(if d.bytes.is_some() { 1 } else { 0 });
    if let Some(v) = &d.bytes {
        write_bytes_lp(out, v);
    }
}
async fn dec_opc_part_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<XlsxOpcPartDiff, String> {
    let content_type = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(read_str_lp(reader)?) } else { None };
    let bytes = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(read_bytes_lp(reader)?) } else { None };
    Ok(XlsxOpcPartDiff { content_type, bytes })
}

async fn enc_parts_diff_bin(d: &XlsxOpcPartsDiff, out: &mut Vec<u8>) {
    enc_named_triple_bin(d, |k, out| write_str_lp(out, k), enc_opc_part_diff_bin, enc_opc_part_bin, out)
}
async fn dec_parts_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<XlsxOpcPartsDiff, String> {
    dec_named_triple_bin(reader, |r| read_str_lp(r), dec_opc_part_diff_bin, dec_opc_part_bin)
}

async fn enc_rel_diff_bin(d: &XlsxOpcRelDiff, out: &mut Vec<u8>) {
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
async fn dec_rel_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<XlsxOpcRelDiff, String> {
    let rel_type = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(read_str_lp(reader)?) } else { None };
    let target = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(read_str_lp(reader)?) } else { None };
    let target_mode = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_target_mode_bin(reader)?) } else { None };
    Ok(XlsxOpcRelDiff { rel_type, target, target_mode })
}

async fn enc_rel_list_diff_bin(d: &XlsxOpcRelListDiff, out: &mut Vec<u8>) {
    enc_named_triple_bin(d, |k, out| write_str_lp(out, k), enc_rel_diff_bin, enc_rel_bin, out)
}
async fn dec_rel_list_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<XlsxOpcRelListDiff, String> {
    dec_named_triple_bin(reader, |r| read_str_lp(r), dec_rel_diff_bin, dec_rel_bin)
}

async fn enc_relationships_diff_bin(d: &XlsxOpcRelationshipsDiff, out: &mut Vec<u8>) {
    enc_named_triple_bin(d, |k, out| write_str_lp(out, k), enc_rel_list_diff_bin, enc_owner_rels_bin, out)
}
async fn dec_relationships_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<XlsxOpcRelationshipsDiff, String> {
    dec_named_triple_bin(reader, |r| read_str_lp(r), dec_rel_list_diff_bin, dec_owner_rels_bin)
}

pub(crate) async fn enc_opc_diff_bin(d: &XlsxOpcDiff, out: &mut Vec<u8>) {
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
pub(crate) async fn dec_opc_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<XlsxOpcDiff, String> {
    let content_types = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_content_types_diff_bin(reader)?) } else { None };
    let parts = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_parts_diff_bin(reader)?) } else { None };
    let relationships = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_relationships_diff_bin(reader)?) } else { None };
    Ok(XlsxOpcDiff { content_types, parts, relationships })
}
//#endregion 🔖️DiffValueBinaryCodecs
//#endregion 🔖️BinaryCodecs

//#region 🔖️TopLevel
/// 🏷️ Space-separated `name=value` tokens, one per non-`None` top field — absent token = unchanged.
/// No token/separator value ever contains a literal space (hex/decimal/`,`/`;`/`:`/`[`/`]` only),
/// so top-level tokenizing is a trivial `line.split(' ')`, same as gif/svg's hand-rolled codecs.
async fn print_xlsx_diff(d: &XlsxDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = &d.opc {
        tokens.push(format!("opc={}", enc_opc_diff(v)));
    }
    if let Some(v) = &d.workbook {
        tokens.push(format!("workbook={}", enc_workbook_diff(v)));
    }
    tokens.join(" ")
}
async fn parse_xlsx_diff(line: &str) -> Result<XlsxDiff, String> {
    let mut d = XlsxDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("opc=") {
            d.opc = Some(dec_opc_diff(rest)?);
        } else if let Some(rest) = token.strip_prefix("workbook=") {
            d.workbook = Some(dec_workbook_diff(rest)?);
        } else {
            return Err(format!("xlsx diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl protocol::DiffCodec for XlsxDiff {
    async fn print_diff(&self) -> String {
        print_xlsx_diff(self)
    }
    async fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        parse_xlsx_diff(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    /// 🧪️ FG-wave: REAL binary frame (`format u8 | flags u8 | [opc][workbook]`), matching
    /// `../💾️binary/📡️component.protocol.semio`'s `header fixed 2` + `chain payload bytes` shape
    /// — upgraded from F6's `print_diff().into_bytes()` text-as-binary shortcut (per this ticket's
    /// own `📖️grammar-recipe.md` census, 100% of stdio's `DiffCodec` impls were still on that
    /// shortcut before this pilot ladder; confirmed live by direct read of this file before this
    /// wave, not assumed). `flags` bits 0/1 mark `opc`/`workbook` presence; each present field's
    /// own recursive binary payload follows in that fixed order (see `🔖️BinaryCodecs` above).
    async fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let mut flags: u8 = 0;
        if self.opc.is_some() {
            flags |= 0b01;
        }
        if self.workbook.is_some() {
            flags |= 0b10;
        }
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, flags];
        if let Some(opc) = &self.opc {
            enc_opc_diff_bin(opc, &mut out);
        }
        if let Some(workbook) = &self.workbook {
            enc_workbook_diff_bin(workbook, &mut out);
        }
        Ok(out)
    }
    async fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let _format = reader.read_u8().map_err(|e| malformed("diff format", 0, e.to_string()))?;
        let flags = reader.read_u8().map_err(|e| malformed("diff flags", 1, e.to_string()))?;
        let opc = if flags & 0b01 != 0 { Some(dec_opc_diff_bin(&mut reader).map_err(|e| malformed("diff opc", reader.position(), e))?) } else { None };
        let workbook = if flags & 0b10 != 0 { Some(dec_workbook_diff_bin(&mut reader).map_err(|e| malformed("diff workbook", reader.position(), e))?) } else { None };
        Ok(XlsxDiff { opc, workbook })
    }
}
//#endregion 🔖️TopLevel

//#region 🔖️DemoCases
/// 🧪️ FG-wave: representative `XlsxSnapshot`/`XlsxDiff` values (both top-level fields, every
/// `XlsxCellValue` variant incl. `Formula.cached`, the OPC layer's content-types/parts/
/// relationships-by-owner triples incl. `OpcTargetMode::External`) — the single source of truth
/// reused by `diff_codec_text_binary_roundtrip_law` below AND by `⚙️engine/🦀️component.rs`'s
/// `diff_grammar_conformance_law`/`protocol_walk_law` conformance tests, same shape docx's own
/// `snapshot_a()`/`snapshot_b()`/`demo_diff_cases()` establish (this wave's OPC pattern-setter).
/// Promoted from the former test-only `sample_a`/`sample_b` (renamed for the same convention).
#[cfg(test)]
pub(crate) async fn snapshot_a() -> XlsxSnapshot {
    crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::io::export::serializers::build_minimal_xlsx(XlsxWorkbook {
        sheets: vec![
            XlsxSheet { name: "Sheet1".into(), cells: vec![XlsxCell { row: 1, col: 0, value: XlsxCellValue::Number(1.0) }] },
            XlsxSheet { name: "ToDrop".into(), cells: vec![XlsxCell { row: 1, col: 0, value: XlsxCellValue::SharedString(0) }] },
        ],
        shared_strings: vec!["hello".into()],
    })
}

#[cfg(test)]
pub(crate) async fn snapshot_b() -> XlsxSnapshot {
    let mut snap = crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::io::export::serializers::build_minimal_xlsx(XlsxWorkbook {
        sheets: vec![
            XlsxSheet {
                name: "Sheet1".into(),
                cells: vec![
                    XlsxCell { row: 1, col: 0, value: XlsxCellValue::Boolean(true) },
                    XlsxCell { row: 2, col: 2, value: XlsxCellValue::Formula { expr: "SUM(A1:A2)".into(), cached: Some(Box::new(XlsxCellValue::Number(-3.5))) } },
                    XlsxCell { row: 3, col: 0, value: XlsxCellValue::InlineString("brand new, with: odd [chars]".into()) },
                    XlsxCell { row: 4, col: 0, value: XlsxCellValue::Empty },
                ],
            },
            XlsxSheet { name: "Added".into(), cells: vec![] },
        ],
        shared_strings: vec!["hello".into(), "world".into()],
    });
    snap.opc.content_types.set_default("added", "application/octet-stream");
    snap.opc.set_part("xl/added.xml", "application/xml", b"fresh".to_vec());
    snap.opc.add_relationship("xl/added.xml", "rId9", "http://example/added", "media/added.png");
    snap.opc.relationships.get_mut("xl/added.xml").unwrap()[0].target_mode = OpcTargetMode::External;
    snap
}

/// 🧪️ The demo cases proper — `default()` (empty diff) plus every real `between()` shape (both
/// directions, and the trivially-empty self-diff).
#[cfg(test)]
pub(crate) async fn demo_diff_cases() -> Vec<XlsxDiff> {
    let a = snapshot_a();
    let b = snapshot_b();
    vec![XlsxDiff::default(), XlsxDiff::between(&a, &b), XlsxDiff::between(&b, &a), XlsxDiff::between(&a, &a)]
}
//#endregion 🔖️DemoCases
//#endregion 🔖️HandcraftedDiffCodec

//#region 🧪️Tests
#[cfg(test)]
mod handcrafted_diff_codec_tests {
    use super::*;
    use protocol::DiffCodec;

    /// 🧪️ `DiffCodec` round-trip laws over the hand-rolled `XlsxDiff` grammar — exercises every
    /// `XlsxCellValue` variant (incl. `Formula.cached` and a value containing raw `,`/`:`/`[`/`]`
    /// bytes-through-hex), the OPC content-types/parts/relationships triples (incl.
    /// `OpcTargetMode::External`), and both `opc`/`workbook` top-level tokens together and alone.
    #[test]
    async fn diff_codec_text_binary_roundtrip_law() {
        let a = snapshot_a();
        let b = snapshot_b();
        let empty = XlsxSnapshot::default();

        let cases = vec![
            XlsxDiff::default(),
            <XlsxDiff as DiffAlgebra<XlsxSnapshot>>::between(&a, &b),
            <XlsxDiff as DiffAlgebra<XlsxSnapshot>>::between(&b, &a),
            <XlsxDiff as DiffAlgebra<XlsxSnapshot>>::between(&a, &empty),
            <XlsxDiff as DiffAlgebra<XlsxSnapshot>>::between(&empty, &a),
        ];
        for d in cases {
            let printed = d.print_diff();
            assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
            let parsed = XlsxDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
            assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

            let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
            let decoded = XlsxDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
            assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
        }
    }
}
//#endregion 🧪️Tests

#[cfg(test)]
mod result_apply_tests {
    use super::*;

    #[test]
    async fn rejects_missing_sheet_target_without_mutating_base() {
        let base = XlsxSnapshot::default();
        let diff =
            XlsxDiff { workbook: Some(XlsxWorkbookDiff { sheets: Some(XlsxSheetsDiff { modified: vec![NamedModified { key: "missing".into(), diff: XlsxSheetDiff::default() }], ..Default::default() }), ..Default::default() }), ..Default::default() };
        let result = diff.apply(&base);
        assert_eq!(result.unwrap_err().code, "mutation.apply.missing-target");
        assert_eq!(base, XlsxSnapshot::default());
    }
}
