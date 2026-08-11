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
use protocol::MutationDiff;
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
    fn default() -> Self { Self { removed: Vec::new(), modified: Vec::new(), added: Vec::new() } }
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
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub opc: Option<XlsxOpcDiff>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workbook: Option<XlsxWorkbookDiff>,
}
//#endregion 🔖️Diff

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

/// 🧮️ Name-keyed absorb — identity is the KEY (not position): a `d2`-removal of a `d1`-added key
/// annihilates the add; a `d2`-modify of a `d1`-added key patches into the carried payload;
/// everything else composes directly on the shared key space.
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

//#region 🔖️WorkbookDiffLogic
fn cell_key(cell: &XlsxCell) -> (u32, u32) { (cell.row, cell.col) }

fn diff_cell(old: &XlsxCell, new: &XlsxCell) -> Option<XlsxCellDiff> {
    if old.value == new.value {
        return None;
    }
    Some(XlsxCellDiff { value: Some(new.value.clone()) })
}

fn apply_cell(cell: &mut XlsxCell, diff: &XlsxCellDiff) {
    if let Some(v) = &diff.value {
        cell.value = v.clone();
    }
}

fn cell_with_diff_applied(cell: &XlsxCell, diff: &XlsxCellDiff) -> XlsxCell {
    let mut out = cell.clone();
    apply_cell(&mut out, diff);
    out
}

fn inverse_cell(base: &XlsxCell, diff: &XlsxCellDiff) -> XlsxCellDiff {
    XlsxCellDiff { value: diff.value.as_ref().map(|_| base.value.clone()) }
}

fn absorb_cell_diff(mut a: XlsxCellDiff, b: XlsxCellDiff) -> XlsxCellDiff {
    if b.value.is_some() {
        a.value = b.value;
    }
    a
}

fn diff_sheet(old: &XlsxSheet, new: &XlsxSheet) -> Option<XlsxSheetDiff> {
    let cells = between_named(&old.cells, &new.cells, cell_key, diff_cell);
    if cells.is_none() { None } else { Some(XlsxSheetDiff { cells }) }
}

fn apply_sheet(sheet: &mut XlsxSheet, diff: &XlsxSheetDiff) {
    if let Some(cd) = &diff.cells {
        apply_named(&mut sheet.cells, cd, cell_key, apply_cell);
    }
}

fn sheet_with_diff_applied(sheet: &XlsxSheet, diff: &XlsxSheetDiff) -> XlsxSheet {
    let mut out = sheet.clone();
    apply_sheet(&mut out, diff);
    out
}

fn inverse_sheet(base: &XlsxSheet, diff: &XlsxSheetDiff) -> XlsxSheetDiff {
    XlsxSheetDiff { cells: diff.cells.as_ref().map(|cd| inverse_named(&base.cells, cd, cell_key, inverse_cell)) }
}

fn absorb_sheet_diff(mut a: XlsxSheetDiff, b: XlsxSheetDiff) -> XlsxSheetDiff {
    a.cells = match (a.cells.take(), b.cells) {
        (None, x) => x,
        (x, None) => x,
        (Some(ca), Some(cb)) => Some(absorb_named(ca, cb, cell_key, absorb_cell_diff, apply_cell)),
    };
    a
}

fn diff_shared_string(old: &(usize, String), new: &(usize, String)) -> Option<String> {
    (old.1 != new.1).then(|| new.1.clone())
}

fn shared_strings_pairs(strings: &[String]) -> Vec<(usize, String)> {
    strings.iter().cloned().enumerate().collect()
}

fn diff_shared_strings(old: &[String], new: &[String]) -> Option<XlsxSharedStringsDiff> {
    between_named(&shared_strings_pairs(old), &shared_strings_pairs(new), |(i, _)| *i, diff_shared_string)
}

fn apply_shared_strings(strings: &mut Vec<String>, diff: &XlsxSharedStringsDiff) {
    let mut pairs = shared_strings_pairs(strings);
    apply_named(&mut pairs, diff, |(i, _)| *i, |(_, v), nv| *v = nv.clone());
    pairs.sort_by_key(|(i, _)| *i);
    *strings = pairs.into_iter().map(|(_, v)| v).collect();
}

fn inverse_shared_strings(base: &[String], diff: &XlsxSharedStringsDiff) -> XlsxSharedStringsDiff {
    inverse_named(&shared_strings_pairs(base), diff, |(i, _)| *i, |(_, v), _| v.clone())
}

fn absorb_shared_strings_diff(a: XlsxSharedStringsDiff, b: XlsxSharedStringsDiff) -> XlsxSharedStringsDiff {
    // 🏷️ `D = String` here is already a whole-value replace (LWW) — absorbing two such diffs on
    // the SAME index is just "the later one wins", i.e. `b` (same pattern as docx's content-types
    // entries absorb).
    absorb_named(a, b, |(i, _)| *i, |_av, bv| bv, |(_, v), nv| *v = nv.clone())
}

fn diff_workbook(base: &XlsxWorkbook, other: &XlsxWorkbook) -> Option<XlsxWorkbookDiff> {
    let sheets = between_named(&base.sheets, &other.sheets, |s| s.name.clone(), diff_sheet);
    let shared_strings = diff_shared_strings(&base.shared_strings, &other.shared_strings);
    if sheets.is_none() && shared_strings.is_none() { None } else { Some(XlsxWorkbookDiff { sheets, shared_strings }) }
}

fn apply_workbook_diff(workbook: &mut XlsxWorkbook, diff: &XlsxWorkbookDiff) {
    if let Some(sd) = &diff.sheets {
        apply_named(&mut workbook.sheets, sd, |s| s.name.clone(), apply_sheet);
    }
    if let Some(ssd) = &diff.shared_strings {
        apply_shared_strings(&mut workbook.shared_strings, ssd);
    }
}

fn inverse_workbook_diff(base: &XlsxWorkbook, diff: &XlsxWorkbookDiff) -> XlsxWorkbookDiff {
    XlsxWorkbookDiff {
        sheets: diff.sheets.as_ref().map(|sd| inverse_named(&base.sheets, sd, |s| s.name.clone(), inverse_sheet)),
        shared_strings: diff.shared_strings.as_ref().map(|ssd| inverse_shared_strings(&base.shared_strings, ssd)),
    }
}

fn absorb_workbook_diff(a: XlsxWorkbookDiff, b: XlsxWorkbookDiff) -> XlsxWorkbookDiff {
    XlsxWorkbookDiff {
        sheets: match (a.sheets, b.sheets) {
            (None, x) => x,
            (x, None) => x,
            (Some(sa), Some(sb)) => Some(absorb_named(sa, sb, |s| s.name.clone(), absorb_sheet_diff, apply_sheet)),
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
fn diff_ct_entries(old: &[(String, String)], new: &[(String, String)]) -> Option<XlsxOpcCtEntriesDiff> {
    between_named(old, new, |(k, _)| k.clone(), |(_, ov), (_, nv)| (ov != nv).then(|| nv.clone()))
}

fn apply_ct_entries(entries: &mut Vec<(String, String)>, diff: &XlsxOpcCtEntriesDiff) {
    apply_named(entries, diff, |(k, _)| k.clone(), |(_, v), nv| *v = nv.clone());
}

fn inverse_ct_entries(base: &[(String, String)], diff: &XlsxOpcCtEntriesDiff) -> XlsxOpcCtEntriesDiff {
    inverse_named(base, diff, |(k, _)| k.clone(), |(_, v), _| v.clone())
}

fn absorb_ct_entries(a: XlsxOpcCtEntriesDiff, b: XlsxOpcCtEntriesDiff) -> XlsxOpcCtEntriesDiff {
    absorb_named(a, b, |(k, _)| k.clone(), |_av, bv| bv, |(_, v), nv| *v = nv.clone())
}

fn diff_content_types(old: &OpcContentTypes, new: &OpcContentTypes) -> Option<XlsxOpcContentTypesDiff> {
    let defaults = diff_ct_entries(&old.defaults, &new.defaults);
    let overrides = diff_ct_entries(&old.overrides, &new.overrides);
    if defaults.is_none() && overrides.is_none() { None } else { Some(XlsxOpcContentTypesDiff { defaults, overrides }) }
}

fn diff_part(old: &OpcPart, new: &OpcPart) -> Option<XlsxOpcPartDiff> {
    if old == new {
        return None;
    }
    Some(XlsxOpcPartDiff {
        content_type: (old.content_type != new.content_type).then(|| new.content_type.clone()),
        bytes: (old.bytes != new.bytes).then(|| new.bytes.clone()),
    })
}

fn apply_part(part: &mut OpcPart, diff: &XlsxOpcPartDiff) {
    if let Some(v) = &diff.content_type {
        part.content_type = v.clone();
    }
    if let Some(v) = &diff.bytes {
        part.bytes = v.clone();
    }
}

fn part_with_diff_applied(part: &OpcPart, diff: &XlsxOpcPartDiff) -> OpcPart {
    let mut out = part.clone();
    apply_part(&mut out, diff);
    out
}

fn inverse_part(base: &OpcPart, diff: &XlsxOpcPartDiff) -> XlsxOpcPartDiff {
    XlsxOpcPartDiff {
        content_type: diff.content_type.as_ref().map(|_| base.content_type.clone()),
        bytes: diff.bytes.as_ref().map(|_| base.bytes.clone()),
    }
}

fn absorb_part_diff(mut a: XlsxOpcPartDiff, b: XlsxOpcPartDiff) -> XlsxOpcPartDiff {
    if b.content_type.is_some() {
        a.content_type = b.content_type;
    }
    if b.bytes.is_some() {
        a.bytes = b.bytes;
    }
    a
}

fn diff_parts(old: &[OpcPart], new: &[OpcPart]) -> Option<XlsxOpcPartsDiff> {
    between_named(old, new, |p| p.path.clone(), diff_part)
}

fn diff_rel(old: &OpcRelationship, new: &OpcRelationship) -> Option<XlsxOpcRelDiff> {
    if old == new {
        return None;
    }
    Some(XlsxOpcRelDiff {
        rel_type: (old.rel_type != new.rel_type).then(|| new.rel_type.clone()),
        target: (old.target != new.target).then(|| new.target.clone()),
        target_mode: (old.target_mode != new.target_mode).then_some(new.target_mode),
    })
}

fn apply_rel(rel: &mut OpcRelationship, diff: &XlsxOpcRelDiff) {
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

fn inverse_rel(base: &OpcRelationship, diff: &XlsxOpcRelDiff) -> XlsxOpcRelDiff {
    XlsxOpcRelDiff {
        rel_type: diff.rel_type.as_ref().map(|_| base.rel_type.clone()),
        target: diff.target.as_ref().map(|_| base.target.clone()),
        target_mode: diff.target_mode.map(|_| base.target_mode),
    }
}

fn absorb_rel_diff(mut a: XlsxOpcRelDiff, b: XlsxOpcRelDiff) -> XlsxOpcRelDiff {
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

fn diff_rel_list(old: &[OpcRelationship], new: &[OpcRelationship]) -> Option<XlsxOpcRelListDiff> {
    between_named(old, new, |r| r.id.clone(), diff_rel)
}

fn apply_rel_list(list: &mut Vec<OpcRelationship>, diff: &XlsxOpcRelListDiff) {
    apply_named(list, diff, |r| r.id.clone(), apply_rel);
}

fn rel_list_with_diff_applied(list: &[OpcRelationship], diff: &XlsxOpcRelListDiff) -> Vec<OpcRelationship> {
    let mut out = list.to_vec();
    apply_rel_list(&mut out, diff);
    out
}

fn inverse_rel_list(base: &[OpcRelationship], diff: &XlsxOpcRelListDiff) -> XlsxOpcRelListDiff {
    inverse_named(base, diff, |r| r.id.clone(), inverse_rel)
}

fn absorb_rel_list_diff(a: XlsxOpcRelListDiff, b: XlsxOpcRelListDiff) -> XlsxOpcRelListDiff {
    absorb_named(a, b, |r| r.id.clone(), absorb_rel_diff, apply_rel)
}

fn diff_relationships(old: &HashMap<String, Vec<OpcRelationship>>, new: &HashMap<String, Vec<OpcRelationship>>) -> Option<XlsxOpcRelationshipsDiff> {
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
    if removed.is_empty() && modified.is_empty() && added.is_empty() { None } else { Some(XlsxOpcRelationshipsDiff { removed, modified, added }) }
}

fn apply_relationships(rels: &mut HashMap<String, Vec<OpcRelationship>>, diff: &XlsxOpcRelationshipsDiff) {
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

fn inverse_relationships(base: &HashMap<String, Vec<OpcRelationship>>, diff: &XlsxOpcRelationshipsDiff) -> XlsxOpcRelationshipsDiff {
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

fn absorb_relationships(d1: XlsxOpcRelationshipsDiff, d2: XlsxOpcRelationshipsDiff) -> XlsxOpcRelationshipsDiff {
    absorb_named(
        d1,
        d2,
        |(owner, _)| owner.clone(),
        absorb_rel_list_diff,
        |(_, list), diff| *list = rel_list_with_diff_applied(list, diff),
    )
}

fn diff_opc(base: &OpcPackage, other: &OpcPackage) -> Option<XlsxOpcDiff> {
    let content_types = diff_content_types(&base.content_types, &other.content_types);
    let parts = diff_parts(&base.parts, &other.parts);
    let relationships = diff_relationships(&base.relationships, &other.relationships);
    if content_types.is_none() && parts.is_none() && relationships.is_none() { None } else { Some(XlsxOpcDiff { content_types, parts, relationships }) }
}

fn apply_opc_diff(opc: &mut OpcPackage, diff: &XlsxOpcDiff) {
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

fn inverse_opc_diff(base: &OpcPackage, diff: &XlsxOpcDiff) -> XlsxOpcDiff {
    XlsxOpcDiff {
        content_types: diff.content_types.as_ref().map(|d| XlsxOpcContentTypesDiff {
            defaults: d.defaults.as_ref().map(|dd| inverse_ct_entries(&base.content_types.defaults, dd)),
            overrides: d.overrides.as_ref().map(|dd| inverse_ct_entries(&base.content_types.overrides, dd)),
        }),
        parts: diff.parts.as_ref().map(|d| inverse_named(&base.parts, d, |p| p.path.clone(), inverse_part)),
        relationships: diff.relationships.as_ref().map(|d| inverse_relationships(&base.relationships, d)),
    }
}

fn absorb_opc_diff(a: XlsxOpcDiff, b: XlsxOpcDiff) -> XlsxOpcDiff {
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
    fn apply(&self, base: &XlsxSnapshot) -> XlsxSnapshot {
        let mut next = base.clone();
        if let Some(d) = &self.opc {
            apply_opc_diff(&mut next.opc, d);
        }
        if let Some(d) = &self.workbook {
            apply_workbook_diff(&mut next.workbook, d);
        }
        next
    }

    fn absorb(&mut self, other: Self) {
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
    fn inverse(&self, base: &XlsxSnapshot) -> Self {
        XlsxDiff {
            opc: self.opc.as_ref().map(|d| inverse_opc_diff(&base.opc, d)),
            workbook: self.workbook.as_ref().map(|d| inverse_workbook_diff(&base.workbook, d)),
        }
    }

    fn between(base: &XlsxSnapshot, other: &XlsxSnapshot) -> Self {
        XlsxDiff { opc: diff_opc(&base.opc, &other.opc), workbook: diff_workbook(&base.workbook, &other.workbook) }
    }

    fn is_empty(&self) -> bool {
        self.opc.is_none() && self.workbook.is_none()
    }
}
//#endregion 🔖️DiffAlgebra

//#region 🔖️MutationConstructors
/// 🧩 Builds the sparse field-by-field diff for a `SetSnapshot` mutation. No `snapshot:
/// Option<XlsxSnapshot>` full-replace slot — this IS `XlsxDiff::between`.
pub fn diff_set_snapshot(base: &XlsxSnapshot, next: &XlsxSnapshot) -> XlsxDiff {
    XlsxDiff::between(base, next)
}

/// 🧩 Builds the diff for inserting a brand-new (possibly non-empty) sheet.
pub fn diff_insert_sheet(sheet: XlsxSheet) -> XlsxDiff {
    XlsxDiff { opc: None, workbook: Some(XlsxWorkbookDiff { sheets: Some(XlsxSheetsDiff { added: vec![sheet], ..Default::default() }), shared_strings: None }) }
}

/// 🧩 Builds the diff for removing the sheet named `name`.
pub fn diff_remove_sheet(name: &str) -> XlsxDiff {
    XlsxDiff { opc: None, workbook: Some(XlsxWorkbookDiff { sheets: Some(XlsxSheetsDiff { removed: vec![name.to_string()], ..Default::default() }), shared_strings: None }) }
}

/// 🧩 Builds the diff for renaming a sheet — `name` is the sheet's KEY (identity), so a rename is
/// a remove-old-name + add-new-name-with-full-content at the diff level (documented in the
/// snapshot module's doc comment, same category as docx's OPC-part-rename gotcha).
pub fn diff_rename_sheet(old_sheet: &XlsxSheet, new_name: &str) -> XlsxDiff {
    if old_sheet.name == new_name {
        return XlsxDiff::default();
    }
    let renamed = XlsxSheet { name: new_name.to_string(), cells: old_sheet.cells.clone() };
    XlsxDiff {
        opc: None,
        workbook: Some(XlsxWorkbookDiff {
            sheets: Some(XlsxSheetsDiff { removed: vec![old_sheet.name.clone()], added: vec![renamed], ..Default::default() }),
            shared_strings: None,
        }),
    }
}

/// 🧩 Builds the diff for setting (inserting or replacing) one cell's value in sheet `sheet_name`.
pub fn diff_set_cell(sheet: &XlsxSheet, row: u32, col: u32, value: XlsxCellValue) -> XlsxDiff {
    let sheet_diff = match sheet.cells.iter().find(|c| c.row == row && c.col == col) {
        Some(existing) if existing.value == value => return XlsxDiff::default(),
        Some(_) => XlsxSheetDiff { cells: Some(XlsxCellsDiff { modified: vec![NamedModified { key: (row, col), diff: XlsxCellDiff { value: Some(value) } }], ..Default::default() }) },
        None => XlsxSheetDiff { cells: Some(XlsxCellsDiff { added: vec![XlsxCell { row, col, value }], ..Default::default() }) },
    };
    XlsxDiff {
        opc: None,
        workbook: Some(XlsxWorkbookDiff { sheets: Some(XlsxSheetsDiff { modified: vec![NamedModified { key: sheet.name.clone(), diff: sheet_diff }], ..Default::default() }), shared_strings: None }),
    }
}

/// 🧩 Builds the diff for removing the cell at `(row, col)` in sheet `sheet_name`.
pub fn diff_remove_cell(sheet_name: &str, row: u32, col: u32) -> XlsxDiff {
    let sheet_diff = XlsxSheetDiff { cells: Some(XlsxCellsDiff { removed: vec![(row, col)], ..Default::default() }) };
    XlsxDiff {
        opc: None,
        workbook: Some(XlsxWorkbookDiff { sheets: Some(XlsxSheetsDiff { modified: vec![NamedModified { key: sheet_name.to_string(), diff: sheet_diff }], ..Default::default() }), shared_strings: None }),
    }
}

/// 🧩 Builds the diff for appending a new shared string, returning its assigned index alongside
/// the diff (callers building `SharedString(idx)` cell values need the index up front).
pub fn diff_insert_shared_string(existing_len: usize, value: &str) -> (usize, XlsxDiff) {
    let idx = existing_len;
    let diff = XlsxDiff {
        opc: None,
        workbook: Some(XlsxWorkbookDiff { sheets: None, shared_strings: Some(XlsxSharedStringsDiff { added: vec![(idx, value.to_string())], ..Default::default() }) }),
    };
    (idx, diff)
}

/// 🧩 Builds the diff for removing the shared string at `index` (any cell still referencing it
/// by index is the caller's responsibility — mirrors how zip/OPC name-keyed removal never
/// cascades into referrers).
pub fn diff_remove_shared_string(index: usize) -> XlsxDiff {
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
pub fn diff_set_shared_string(strings: &[String], index: usize, value: &str) -> XlsxDiff {
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
pub(crate) fn parse_u32(s: &str) -> Result<u32, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}
pub(crate) fn parse_usize(s: &str) -> Result<usize, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}
/// 🔢️ `f64::to_string`/`str::parse::<f64>()` round-trip exactly (std's shortest-round-trip float
/// formatting) — no manual bit-pattern encoding needed. None of `.`/`-`/`e`/`inf`/`NaN` clash with
/// this grammar's `,`/`;`/`:`/`[`/`]` separators.
pub(crate) fn enc_f64(n: f64) -> String {
    n.to_string()
}
pub(crate) fn dec_f64(s: &str) -> Result<f64, String> {
    s.parse().map_err(|e: std::num::ParseFloatError| e.to_string())
}
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
pub(crate) fn strip_brackets(s: &str) -> Result<&str, String> {
    s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))
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
//#endregion 🔖️Primitives

//#region 🔖️GenericTripleCodec
/// 🧮️ `[removed];[modified];[added]` — `removed` is comma-joined encoded keys, `modified` is
/// comma-joined `key:diff` entries, `added` is comma-joined encoded items. Keys are always
/// hex/decimal (never contain a literal `:`), so `entry.split_once(':')` unambiguously separates a
/// `modified` entry's key from its (possibly itself bracket-nested, comma-and-semicolon-bearing)
/// diff body — same reasoning `f6-recon-report.md` §5 documents for collection-triple entries.
fn enc_triple<K, D, T>(triple: &NamedTripleDiff<K, D, T>, enc_key: impl Fn(&K) -> String, enc_diff: impl Fn(&D) -> String, enc_item: impl Fn(&T) -> String) -> String {
    let removed = triple.removed.iter().map(|k| enc_key(k)).collect::<Vec<_>>().join(",");
    let modified = triple.modified.iter().map(|m| format!("{}:{}", enc_key(&m.key), enc_diff(&m.diff))).collect::<Vec<_>>().join(",");
    let added = triple.added.iter().map(|t| enc_item(t)).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}
fn dec_triple<K, D, T>(
    body: &str,
    dec_key: impl Fn(&str) -> Result<K, String>,
    dec_diff: impl Fn(&str) -> Result<D, String>,
    dec_item: impl Fn(&str) -> Result<T, String>,
) -> Result<NamedTripleDiff<K, D, T>, String> {
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
pub(crate) fn enc_cell_value(v: &XlsxCellValue) -> String {
    match v {
        XlsxCellValue::Number(n) => format!("N[{}]", enc_f64(*n)),
        XlsxCellValue::SharedString(i) => format!("S[{i}]"),
        XlsxCellValue::InlineString(s) => format!("I[{}]", enc_str(s)),
        XlsxCellValue::Boolean(b) => format!("B[{}]", if *b { "1" } else { "0" }),
        XlsxCellValue::Formula { expr, cached } => format!("F[{},{}]", enc_str(expr), encode_option(cached, |c| enc_cell_value(c))),
        XlsxCellValue::Empty => "E[]".to_string(),
    }
}
pub(crate) fn dec_cell_value(s: &str) -> Result<XlsxCellValue, String> {
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
pub(crate) fn enc_cell_key(k: &(u32, u32)) -> String {
    format!("[{},{}]", k.0, k.1)
}
pub(crate) fn dec_cell_key(s: &str) -> Result<(u32, u32), String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [row, col] = parts.as_slice() else { return Err(format!("cell key: expected 2 fields, got {}", parts.len())) };
    Ok((parse_u32(row)?, parse_u32(col)?))
}
pub(crate) fn enc_cell(c: &XlsxCell) -> String {
    format!("[{},{},{}]", c.row, c.col, enc_cell_value(&c.value))
}
pub(crate) fn dec_cell(s: &str) -> Result<XlsxCell, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [row, col, value] = parts.as_slice() else { return Err(format!("cell: expected 3 fields, got {}", parts.len())) };
    Ok(XlsxCell { row: parse_u32(row)?, col: parse_u32(col)?, value: dec_cell_value(value)? })
}
fn enc_cell_diff(d: &XlsxCellDiff) -> String {
    encode_option(&d.value, |v| enc_cell_value(v))
}
fn dec_cell_diff(s: &str) -> Result<XlsxCellDiff, String> {
    Ok(XlsxCellDiff { value: decode_option(s, dec_cell_value)? })
}
fn enc_cells_diff(t: &XlsxCellsDiff) -> String {
    enc_triple(t, enc_cell_key, enc_cell_diff, enc_cell)
}
fn dec_cells_diff(s: &str) -> Result<XlsxCellsDiff, String> {
    dec_triple(s, dec_cell_key, dec_cell_diff, dec_cell)
}
//#endregion 🔖️CellValueCodec

//#region 🔖️WorkbookCodec
pub(crate) fn enc_sheet(s: &XlsxSheet) -> String {
    let cells = s.cells.iter().map(enc_cell).collect::<Vec<_>>().join(",");
    format!("[{},[{}]]", enc_str(&s.name), cells)
}
pub(crate) fn dec_sheet(s: &str) -> Result<XlsxSheet, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, cells] = parts.as_slice() else { return Err(format!("sheet: expected 2 fields, got {}", parts.len())) };
    let cells = split_top_level(strip_brackets(cells)?, ',').into_iter().map(dec_cell).collect::<Result<Vec<_>, String>>()?;
    Ok(XlsxSheet { name: dec_str(name)?, cells })
}
fn enc_sheet_diff(d: &XlsxSheetDiff) -> String {
    encode_option(&d.cells, |c| enc_cells_diff(c))
}
fn dec_sheet_diff(s: &str) -> Result<XlsxSheetDiff, String> {
    Ok(XlsxSheetDiff { cells: decode_option(s, dec_cells_diff)? })
}
fn enc_sheets_diff(t: &XlsxSheetsDiff) -> String {
    enc_triple(t, |k| enc_str(k), enc_sheet_diff, enc_sheet)
}
fn dec_sheets_diff(s: &str) -> Result<XlsxSheetsDiff, String> {
    dec_triple(s, dec_str, dec_sheet_diff, dec_sheet)
}
fn enc_shared_string_item(item: &(usize, String)) -> String {
    format!("[{},{}]", item.0, enc_str(&item.1))
}
fn dec_shared_string_item(s: &str) -> Result<(usize, String), String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [idx, value] = parts.as_slice() else { return Err(format!("shared string item: expected 2 fields, got {}", parts.len())) };
    Ok((parse_usize(idx)?, dec_str(value)?))
}
fn enc_shared_strings_diff(t: &XlsxSharedStringsDiff) -> String {
    enc_triple(t, |k| k.to_string(), |d| enc_str(d), enc_shared_string_item)
}
fn dec_shared_strings_diff(s: &str) -> Result<XlsxSharedStringsDiff, String> {
    dec_triple(s, parse_usize, dec_str, dec_shared_string_item)
}
fn enc_workbook_diff(d: &XlsxWorkbookDiff) -> String {
    format!("[{},{}]", encode_option(&d.sheets, |t| enc_sheets_diff(t)), encode_option(&d.shared_strings, |t| enc_shared_strings_diff(t)))
}
fn dec_workbook_diff(s: &str) -> Result<XlsxWorkbookDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [sheets, shared_strings] = parts.as_slice() else { return Err(format!("workbook diff: expected 2 fields, got {}", parts.len())) };
    Ok(XlsxWorkbookDiff { sheets: decode_option(sheets, dec_sheets_diff)?, shared_strings: decode_option(shared_strings, dec_shared_strings_diff)? })
}
//#endregion 🔖️WorkbookCodec

//#region 🔖️OpcCodec
pub(crate) fn enc_ct_entry(item: &(String, String)) -> String {
    format!("[{},{}]", enc_str(&item.0), enc_str(&item.1))
}
pub(crate) fn dec_ct_entry(s: &str) -> Result<(String, String), String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [k, v] = parts.as_slice() else { return Err(format!("ct entry: expected 2 fields, got {}", parts.len())) };
    Ok((dec_str(k)?, dec_str(v)?))
}
fn enc_ct_entries_diff(t: &XlsxOpcCtEntriesDiff) -> String {
    enc_triple(t, |k| enc_str(k), |d| enc_str(d), enc_ct_entry)
}
fn dec_ct_entries_diff(s: &str) -> Result<XlsxOpcCtEntriesDiff, String> {
    dec_triple(s, dec_str, dec_str, dec_ct_entry)
}
fn enc_content_types_diff(d: &XlsxOpcContentTypesDiff) -> String {
    format!("[{},{}]", encode_option(&d.defaults, |t| enc_ct_entries_diff(t)), encode_option(&d.overrides, |t| enc_ct_entries_diff(t)))
}
fn dec_content_types_diff(s: &str) -> Result<XlsxOpcContentTypesDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [defaults, overrides] = parts.as_slice() else { return Err(format!("content types diff: expected 2 fields, got {}", parts.len())) };
    Ok(XlsxOpcContentTypesDiff { defaults: decode_option(defaults, dec_ct_entries_diff)?, overrides: decode_option(overrides, dec_ct_entries_diff)? })
}
pub(crate) fn enc_part(p: &OpcPart) -> String {
    format!("[{},{},{}]", enc_str(&p.path), enc_str(&p.content_type), hex_encode(&p.bytes))
}
pub(crate) fn dec_part(s: &str) -> Result<OpcPart, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [path, ct, bytes] = parts.as_slice() else { return Err(format!("opc part: expected 3 fields, got {}", parts.len())) };
    Ok(OpcPart { path: dec_str(path)?, content_type: dec_str(ct)?, bytes: hex_decode(bytes)? })
}
fn enc_part_diff(d: &XlsxOpcPartDiff) -> String {
    format!("[{},{}]", encode_option(&d.content_type, |v| enc_str(v)), encode_option(&d.bytes, |v| hex_encode(v)))
}
fn dec_part_diff(s: &str) -> Result<XlsxOpcPartDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [ct, bytes] = parts.as_slice() else { return Err(format!("opc part diff: expected 2 fields, got {}", parts.len())) };
    Ok(XlsxOpcPartDiff { content_type: decode_option(ct, dec_str)?, bytes: decode_option(bytes, hex_decode)? })
}
fn enc_parts_diff(t: &XlsxOpcPartsDiff) -> String {
    enc_triple(t, |k| enc_str(k), enc_part_diff, enc_part)
}
fn dec_parts_diff(s: &str) -> Result<XlsxOpcPartsDiff, String> {
    dec_triple(s, dec_str, dec_part_diff, dec_part)
}
pub(crate) fn enc_target_mode(m: &OpcTargetMode) -> String {
    match m {
        OpcTargetMode::Internal => "0".to_string(),
        OpcTargetMode::External => "1".to_string(),
    }
}
pub(crate) fn dec_target_mode(s: &str) -> Result<OpcTargetMode, String> {
    match s {
        "0" => Ok(OpcTargetMode::Internal),
        "1" => Ok(OpcTargetMode::External),
        other => Err(format!("target mode: unknown {other:?}")),
    }
}
pub(crate) fn enc_rel(r: &OpcRelationship) -> String {
    format!("[{},{},{},{}]", enc_str(&r.id), enc_str(&r.rel_type), enc_str(&r.target), enc_target_mode(&r.target_mode))
}
pub(crate) fn dec_rel(s: &str) -> Result<OpcRelationship, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, rel_type, target, mode] = parts.as_slice() else { return Err(format!("opc rel: expected 4 fields, got {}", parts.len())) };
    Ok(OpcRelationship { id: dec_str(id)?, rel_type: dec_str(rel_type)?, target: dec_str(target)?, target_mode: dec_target_mode(mode)? })
}
fn enc_rel_diff(d: &XlsxOpcRelDiff) -> String {
    format!(
        "[{},{},{}]",
        encode_option(&d.rel_type, |v| enc_str(v)),
        encode_option(&d.target, |v| enc_str(v)),
        encode_option(&d.target_mode, |v| enc_target_mode(v)),
    )
}
fn dec_rel_diff(s: &str) -> Result<XlsxOpcRelDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [rel_type, target, mode] = parts.as_slice() else { return Err(format!("opc rel diff: expected 3 fields, got {}", parts.len())) };
    Ok(XlsxOpcRelDiff { rel_type: decode_option(rel_type, dec_str)?, target: decode_option(target, dec_str)?, target_mode: decode_option(mode, dec_target_mode)? })
}
fn enc_rel_list_diff(t: &XlsxOpcRelListDiff) -> String {
    enc_triple(t, |k| enc_str(k), enc_rel_diff, enc_rel)
}
fn dec_rel_list_diff(s: &str) -> Result<XlsxOpcRelListDiff, String> {
    dec_triple(s, dec_str, dec_rel_diff, dec_rel)
}
pub(crate) fn enc_owner_rels(item: &(String, Vec<OpcRelationship>)) -> String {
    let rels = item.1.iter().map(enc_rel).collect::<Vec<_>>().join(",");
    format!("[{},[{}]]", enc_str(&item.0), rels)
}
pub(crate) fn dec_owner_rels(s: &str) -> Result<(String, Vec<OpcRelationship>), String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [owner, rels] = parts.as_slice() else { return Err(format!("owner rels: expected 2 fields, got {}", parts.len())) };
    let rels = split_top_level(strip_brackets(rels)?, ',').into_iter().map(dec_rel).collect::<Result<Vec<_>, String>>()?;
    Ok((dec_str(owner)?, rels))
}
fn enc_relationships_diff(t: &XlsxOpcRelationshipsDiff) -> String {
    enc_triple(t, |k| enc_str(k), enc_rel_list_diff, enc_owner_rels)
}
fn dec_relationships_diff(s: &str) -> Result<XlsxOpcRelationshipsDiff, String> {
    dec_triple(s, dec_str, dec_rel_list_diff, dec_owner_rels)
}
fn enc_opc_diff(d: &XlsxOpcDiff) -> String {
    format!(
        "[{},{},{}]",
        encode_option(&d.content_types, |v| enc_content_types_diff(v)),
        encode_option(&d.parts, |v| enc_parts_diff(v)),
        encode_option(&d.relationships, |v| enc_relationships_diff(v)),
    )
}
fn dec_opc_diff(s: &str) -> Result<XlsxOpcDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [ct, p, r] = parts.as_slice() else { return Err(format!("opc diff: expected 3 fields, got {}", parts.len())) };
    Ok(XlsxOpcDiff { content_types: decode_option(ct, dec_content_types_diff)?, parts: decode_option(p, dec_parts_diff)?, relationships: decode_option(r, dec_relationships_diff)? })
}
//#endregion 🔖️OpcCodec

//#region 🔖️TopLevel
/// 🏷️ Space-separated `name=value` tokens, one per non-`None` top field — absent token = unchanged.
/// No token/separator value ever contains a literal space (hex/decimal/`,`/`;`/`:`/`[`/`]` only),
/// so top-level tokenizing is a trivial `line.split(' ')`, same as gif/svg's hand-rolled codecs.
fn print_xlsx_diff(d: &XlsxDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = &d.opc {
        tokens.push(format!("opc={}", enc_opc_diff(v)));
    }
    if let Some(v) = &d.workbook {
        tokens.push(format!("workbook={}", enc_workbook_diff(v)));
    }
    tokens.join(" ")
}
fn parse_xlsx_diff(line: &str) -> Result<XlsxDiff, String> {
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
    fn print_diff(&self) -> String {
        print_xlsx_diff(self)
    }
    fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        parse_xlsx_diff(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    /// ⚡️ Binary = the text bytes verbatim, same simplification `GifDiff`/`SvgDiff`'s hand-rolled
    /// codecs use (and the repo's only other hand-rolled `DiffCodec`, `WriterDiff`) — satisfies
    /// every `DiffCodec` law without inventing a second wire format.
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

//#region 🧪️Tests
#[cfg(test)]
mod handcrafted_diff_codec_tests {
    use super::*;
    use protocol::DiffCodec;

    fn sample_a() -> XlsxSnapshot {
        crate::artifacts::xlsx::engine::build_minimal_xlsx(XlsxWorkbook {
            sheets: vec![
                XlsxSheet { name: "Sheet1".into(), cells: vec![XlsxCell { row: 1, col: 0, value: XlsxCellValue::Number(1.0) }] },
                XlsxSheet { name: "ToDrop".into(), cells: vec![XlsxCell { row: 1, col: 0, value: XlsxCellValue::SharedString(0) }] },
            ],
            shared_strings: vec!["hello".into()],
        })
    }

    fn sample_b() -> XlsxSnapshot {
        let mut snap = crate::artifacts::xlsx::engine::build_minimal_xlsx(XlsxWorkbook {
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

    /// 🧪️ `DiffCodec` round-trip laws over the hand-rolled `XlsxDiff` grammar — exercises every
    /// `XlsxCellValue` variant (incl. `Formula.cached` and a value containing raw `,`/`:`/`[`/`]`
    /// bytes-through-hex), the OPC content-types/parts/relationships triples (incl.
    /// `OpcTargetMode::External`), and both `opc`/`workbook` top-level tokens together and alone.
    #[test]
    fn diff_codec_text_binary_roundtrip_law() {
        let a = sample_a();
        let b = sample_b();
        let empty = XlsxSnapshot::default();

        let cases = vec![
            XlsxDiff::default(),
            <XlsxDiff as protocol::command::DiffAlgebra<XlsxSnapshot>>::between(&a, &b),
            <XlsxDiff as protocol::command::DiffAlgebra<XlsxSnapshot>>::between(&b, &a),
            <XlsxDiff as protocol::command::DiffAlgebra<XlsxSnapshot>>::between(&a, &empty),
            <XlsxDiff as protocol::command::DiffAlgebra<XlsxSnapshot>>::between(&empty, &a),
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
