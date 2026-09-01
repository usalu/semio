//! 🧬️ XlsxMutation — document mutation dispatch. Every variant's `diff()` is handcrafted (never
//! apply-and-capture) and every variant's `inverse()` is handcrafted, key/index-aware.

use crate::artifacts::xlsx::schema::diff::{
    dec_cell_value, dec_cell_value_bin, dec_ct_entry, dec_opc_part_bin, dec_owner_rels, dec_part, dec_rel_bin, dec_sheet, dec_sheet_bin, dec_str, diff_insert_shared_string, diff_insert_sheet, diff_remove_cell, diff_remove_shared_string,
    diff_remove_sheet, diff_rename_sheet, diff_set_cell, diff_set_shared_string, diff_set_snapshot, enc_cell_value, enc_cell_value_bin, enc_ct_entry, enc_opc_part_bin, enc_owner_rels, enc_part, enc_rel_bin, enc_sheet, enc_sheet_bin, enc_str,
    read_str_lp, split_top_level, strip_brackets, write_str_lp, XlsxDiff,
};
#[cfg(test)]
use crate::artifacts::xlsx::schema::snapshot::XlsxCell;
use crate::artifacts::xlsx::schema::snapshot::{XlsxCellValue, XlsxSheet, XlsxWorkbook};
use crate::artifacts::xlsx::XlsxSnapshot;
use crate::artifacts::zip::opc::{OpcContentTypes, OpcPackage, OpcRelationship};
#[cfg(test)]
use crate::artifacts::zip::opc::{OpcTargetMode, RELS_CONTENT_TYPE, REL_TYPE_OFFICE_DOCUMENT};
use protocol::OpBinary;
use protocol::{Mutation, OpText};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.xlsx`. Beyond the baseline `{NoMutation, SetSnapshot}`,
/// this addresses sheets by NAME (identity), cells by `(sheet name, row, col)`, and shared
/// strings by index.
/// 🧪️ F6 CONFIRMED (STEP 1, real `cargo check -p semio-s-plugin-stdio --lib` run, see
/// `f6-xlsx-mutation-check1.txt` in the ticket folder): `#[derive(dsl::DslOps)]` on this enum fails
/// — independent confirmation beyond `XlsxDiff`'s `DiffCodec` blocker:
/// ```text
/// error[E0277]: the trait bound `XlsxCellValue: DslField` is not satisfied
///   --> …/🧬️mutations/🦀️component.rs:45:16   (SetCell { .. value: XlsxCellValue })
/// error[E0277]: the trait bound `XlsxSnapshot: DslField` is not satisfied
///   --> …/🧬️mutations/🦀️component.rs:23:19   (SetSnapshot { snapshot: XlsxSnapshot })
/// error[E0277]: the trait bound `XlsxSheet: DslField` is not satisfied
///   --> …/🧬️mutations/🦀️component.rs:27:16   (InsertSheet { sheet: XlsxSheet })
/// ```
/// `SetCell.value: XlsxCellValue` carries the enum-shaped payload DIRECTLY (same root cause as
/// `XlsxDiff`'s blocker); `SetSnapshot`/`InsertSheet` reach it transitively through
/// `XlsxSnapshot`/`XlsxSheet`. `OpText`/`OpBinary` hand-rolled below, reusing `XlsxDiff`'s
/// `pub(crate)` grammar primitives (`enc_str`/`enc_cell_value`/`enc_sheet`/`split_top_level`/...).
//#region 🔖️Leaves
#[path = "📄set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "➕insert-sheet/🦀️.rs"]
pub mod insert_sheet;
#[path = "➖remove-sheet/🦀️.rs"]
pub mod remove_sheet;
#[path = "🏷rename-sheet/🦀️.rs"]
pub mod rename_sheet;
#[path = "✍set-cell/🦀️.rs"]
pub mod set_cell;
#[path = "🔻remove-cell/🦀️.rs"]
pub mod remove_cell;
#[path = "🔤insert-shared-string/🦀️.rs"]
pub mod insert_shared_string;
#[path = "🔡remove-shared-string/🦀️.rs"]
pub mod remove_shared_string;
#[path = "🔠set-shared-string/🦀️.rs"]
pub mod set_shared_string;
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this subset. `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires
/// every variant to wrap exactly one leaf payload and a unit variant wraps none.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]
#[mutations(snapshot = XlsxSnapshot, diff = XlsxDiff, schema = "XlsxMutation")]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum XlsxMutation {
    SetSnapshot(set_snapshot::SetSnapshot),
    /// ➕️ Inserts a brand-new sheet (possibly pre-populated with cells).
    InsertSheet(insert_sheet::InsertSheet),
    /// ➖️ Removes the sheet named `name`.
    RemoveSheet(remove_sheet::RemoveSheet),
    /// 🏷️ Renames the sheet named `name` to `new_name` (a remove-old+add-new at the diff level —
    /// `name` is the sheet's identity, see the snapshot module's doc comment).
    RenameSheet(rename_sheet::RenameSheet),
    /// ✍️ Sets (inserting or replacing) the value of the cell at `(row, col)` in sheet
    /// `sheet_name`.
    SetCell(set_cell::SetCell),
    /// ➖️ Removes the cell at `(row, col)` in sheet `sheet_name`.
    RemoveCell(remove_cell::RemoveCell),
    /// ➕️ Appends a new shared string.
    InsertSharedString(insert_shared_string::InsertSharedString),
    /// ➖️ Removes the shared string at `index`.
    RemoveSharedString(remove_shared_string::RemoveSharedString),
    /// ✍️ Replaces the shared string at `index`.
    SetSharedString(set_shared_string::SetSharedString),
}

/// 🧾️ Kebab-case spelling of every `XlsxMutation` variant, in declaration order — the exhaustive
/// mutation catalog `xlsx-ecma-376-any` (`../../🧪️oracle/🔣️.json`) is measured against
/// this exact list. `kinds_match_enum_and_catalog` proves it never drifts from either side.
pub const KINDS: &[&str] = &["set-snapshot", "insert-sheet", "remove-sheet", "rename-sheet", "set-cell", "remove-cell", "insert-shared-string", "remove-shared-string", "set-shared-string"];
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`: `let d = mutation.diff(&*snapshot); *snapshot =
/// d.apply(snapshot); d` — the diff is the single semantics source, never a separate imperative
/// apply path (apply-and-capture is banned).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_xlsx_mutation(snapshot: &mut XlsxSnapshot, mutation: &XlsxMutation) -> protocol::MutationOutcome<XlsxDiff> {
    let outcome = Mutation::diff(mutation, snapshot);
    match protocol::MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::error(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}
//#endregion 🔖️Apply

//#region 🔖️Helpers
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn sheet_at<'a>(base: &'a XlsxSnapshot, name: &str) -> Option<&'a XlsxSheet> {
    base.workbook.sheets.iter().find(|s| s.name == name)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn cell_value_at(base: &XlsxSnapshot, sheet_name: &str, row: u32, col: u32) -> Option<XlsxCellValue> {
    sheet_at(base, sheet_name)?.cells.iter().find(|c| c.row == row && c.col == col).map(|c| c.value.clone())
}
//#endregion 🔖️Helpers

//#region 🔖️MutationTrait
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_diff(this: &XlsxMutation, base: &XlsxSnapshot) -> protocol::MutationOutcome<XlsxDiff> {
    protocol::MutationOutcome::new(match this {
        XlsxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => diff_set_snapshot(base, snapshot),
        XlsxMutation::InsertSheet(insert_sheet::InsertSheet { sheet }) => diff_insert_sheet(sheet.clone()),
        XlsxMutation::RemoveSheet(remove_sheet::RemoveSheet { name }) => diff_remove_sheet(name),
        XlsxMutation::RenameSheet(rename_sheet::RenameSheet { name, new_name }) => match sheet_at(base, name) {
            Some(sheet) => diff_rename_sheet(sheet, new_name),
            None => XlsxDiff::default(),
        },
        XlsxMutation::SetCell(set_cell::SetCell { sheet_name, row, col, value }) => match sheet_at(base, sheet_name) {
            Some(sheet) => diff_set_cell(sheet, *row, *col, value.clone()),
            None => XlsxDiff::default(),
        },
        XlsxMutation::RemoveCell(remove_cell::RemoveCell { sheet_name, row, col }) => diff_remove_cell(sheet_name, *row, *col),
        XlsxMutation::InsertSharedString(insert_shared_string::InsertSharedString { value }) => diff_insert_shared_string(base.workbook.shared_strings.len(), value).1,
        XlsxMutation::RemoveSharedString(remove_shared_string::RemoveSharedString { index }) => diff_remove_shared_string(*index),
        XlsxMutation::SetSharedString(set_shared_string::SetSharedString { index, value }) => diff_set_shared_string(&base.workbook.shared_strings, *index, value),
    })
}

// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_inverse(this: &XlsxMutation, base: &XlsxSnapshot) -> Vec<XlsxMutation> {
    match this {
        XlsxMutation::SetSnapshot(_) => vec![XlsxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })],
        XlsxMutation::InsertSheet(insert_sheet::InsertSheet { sheet }) => vec![XlsxMutation::RemoveSheet(remove_sheet::RemoveSheet { name: sheet.name.clone() })],
        XlsxMutation::RemoveSheet(remove_sheet::RemoveSheet { name }) => match sheet_at(base, name) {
            Some(sheet) => vec![XlsxMutation::InsertSheet(insert_sheet::InsertSheet { sheet: sheet.clone() })],
            None => Vec::new(),
        },
        XlsxMutation::RenameSheet(rename_sheet::RenameSheet { name, new_name }) => match sheet_at(base, name) {
            Some(_) => vec![XlsxMutation::RenameSheet(rename_sheet::RenameSheet { name: new_name.clone(), new_name: name.clone() })],
            None => Vec::new(),
        },
        XlsxMutation::SetCell(set_cell::SetCell { sheet_name, row, col, .. }) => match cell_value_at(base, sheet_name, *row, *col) {
            Some(value) => vec![XlsxMutation::SetCell(set_cell::SetCell { sheet_name: sheet_name.clone(), row: *row, col: *col, value })],
            None => vec![XlsxMutation::RemoveCell(remove_cell::RemoveCell { sheet_name: sheet_name.clone(), row: *row, col: *col })],
        },
        XlsxMutation::RemoveCell(remove_cell::RemoveCell { sheet_name, row, col }) => match cell_value_at(base, sheet_name, *row, *col) {
            Some(value) => vec![XlsxMutation::SetCell(set_cell::SetCell { sheet_name: sheet_name.clone(), row: *row, col: *col, value })],
            None => Vec::new(),
        },
        XlsxMutation::InsertSharedString(_) => vec![XlsxMutation::RemoveSharedString(remove_shared_string::RemoveSharedString { index: base.workbook.shared_strings.len() })],
        XlsxMutation::RemoveSharedString(remove_shared_string::RemoveSharedString { index }) => match base.workbook.shared_strings.get(*index) {
            Some(value) => vec![XlsxMutation::SetSharedString(set_shared_string::SetSharedString { index: *index, value: value.clone() })],
            None => Vec::new(),
        },
        XlsxMutation::SetSharedString(set_shared_string::SetSharedString { index, .. }) => match base.workbook.shared_strings.get(*index) {
            Some(value) => vec![XlsxMutation::SetSharedString(set_shared_string::SetSharedString { index: *index, value: value.clone() })],
            None => Vec::new(),
        },
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
/// 🧪️ F6: **hand-rolled** `OpText`/`OpBinary` for `XlsxMutation` (`#[derive(dsl::DslOps)]`
/// confirmed rejected above) — reuses `XlsxDiff`'s `pub(crate)` grammar primitives rather than
/// duplicating them a second time in this file. Grammar: `keyword arg=value ...` (space-separated,
/// same shape the derive's own handcrafted-wrapper convention uses), one match arm per variant.
//#region 🔖️SnapshotCodec
/// 🧮️ Full-VALUE codecs (not diffs) for `OpcPackage`/`XlsxWorkbook`/`XlsxSnapshot` — reuses the
/// diff module's per-item encoders (`enc_part`/`enc_ct_entry`/`enc_owner_rels`/`enc_sheet`, all
/// already full-value, not diff, shapes) directly; only the outer struct-of-collections wrapping is
/// new here.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_content_types(ct: &OpcContentTypes) -> String {
    let defaults = ct.defaults.iter().map(enc_ct_entry).collect::<Vec<_>>().join(",");
    let overrides = ct.overrides.iter().map(enc_ct_entry).collect::<Vec<_>>().join(",");
    format!("[[{defaults}],[{overrides}]]")
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_content_types(s: &str) -> Result<OpcContentTypes, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [defaults, overrides] = parts.as_slice() else { return Err(format!("content types: expected 2 fields, got {}", parts.len())) };
    let defaults = split_top_level(strip_brackets(defaults)?, ',').into_iter().map(dec_ct_entry).collect::<Result<Vec<_>, String>>()?;
    let overrides = split_top_level(strip_brackets(overrides)?, ',').into_iter().map(dec_ct_entry).collect::<Result<Vec<_>, String>>()?;
    Ok(OpcContentTypes { defaults, overrides })
}
/// 🗺️ Owners sorted for determinism (`HashMap` iteration order is not stable) — matches this
/// artifact's other `HashMap`-backed encodings' expectation of a canonical wire order.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_relationships_map(rels: &HashMap<String, Vec<OpcRelationship>>) -> String {
    let mut owners: Vec<&String> = rels.keys().collect();
    owners.sort();
    let entries = owners.into_iter().map(|o| enc_owner_rels(&(o.clone(), rels[o].clone()))).collect::<Vec<_>>().join(",");
    format!("[{entries}]")
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_relationships_map(s: &str) -> Result<HashMap<String, Vec<OpcRelationship>>, String> {
    let entries = split_top_level(strip_brackets(s)?, ',').into_iter().map(dec_owner_rels).collect::<Result<Vec<_>, String>>()?;
    Ok(entries.into_iter().collect())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_opc_package(pkg: &OpcPackage) -> String {
    let parts = pkg.parts.iter().map(enc_part).collect::<Vec<_>>().join(",");
    format!("[[{parts}],{},{}]", enc_content_types(&pkg.content_types), enc_relationships_map(&pkg.relationships))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_opc_package(s: &str) -> Result<OpcPackage, String> {
    let outer = split_top_level(strip_brackets(s)?, ',');
    let [parts, ct, rels] = outer.as_slice() else { return Err(format!("opc package: expected 3 fields, got {}", outer.len())) };
    let parts = split_top_level(strip_brackets(parts)?, ',').into_iter().map(dec_part).collect::<Result<Vec<_>, String>>()?;
    Ok(OpcPackage { parts, content_types: dec_content_types(ct)?, relationships: dec_relationships_map(rels)?, ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_workbook(wb: &XlsxWorkbook) -> String {
    let sheets = wb.sheets.iter().map(enc_sheet).collect::<Vec<_>>().join(",");
    let strings = wb.shared_strings.iter().map(|s| enc_str(s)).collect::<Vec<_>>().join(",");
    format!("[[{sheets}],[{strings}]]")
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_workbook(s: &str) -> Result<XlsxWorkbook, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [sheets, strings] = parts.as_slice() else { return Err(format!("workbook: expected 2 fields, got {}", parts.len())) };
    let sheets = split_top_level(strip_brackets(sheets)?, ',').into_iter().map(dec_sheet).collect::<Result<Vec<_>, String>>()?;
    let shared_strings = split_top_level(strip_brackets(strings)?, ',').into_iter().map(dec_str).collect::<Result<Vec<_>, String>>()?;
    Ok(XlsxWorkbook { sheets, shared_strings })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_xlsx_snapshot(s: &XlsxSnapshot) -> String {
    format!("[{},{},{}]", enc_str(&s.schema), enc_opc_package(&s.opc), enc_workbook(&s.workbook))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_xlsx_snapshot(s: &str) -> Result<XlsxSnapshot, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [schema, opc, workbook] = parts.as_slice() else { return Err(format!("xlsx snapshot: expected 3 fields, got {}", parts.len())) };
    Ok(XlsxSnapshot { schema: dec_str(schema)?, opc: dec_opc_package(opc)?, workbook: dec_workbook(workbook)? })
}
//#endregion 🔖️SnapshotCodec

//#region 🔖️MutationCodec
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_xlsx_mutation(m: &XlsxMutation) -> String {
    match m {
        XlsxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => format!("set-snapshot snapshot={}", enc_xlsx_snapshot(snapshot)),
        XlsxMutation::InsertSheet(insert_sheet::InsertSheet { sheet }) => format!("insert-sheet sheet={}", enc_sheet(sheet)),
        XlsxMutation::RemoveSheet(remove_sheet::RemoveSheet { name }) => format!("remove-sheet name={}", enc_str(name)),
        XlsxMutation::RenameSheet(rename_sheet::RenameSheet { name, new_name }) => format!("rename-sheet name={} new-name={}", enc_str(name), enc_str(new_name)),
        XlsxMutation::SetCell(set_cell::SetCell { sheet_name, row, col, value }) => format!("set-cell sheet-name={} row={row} col={col} value={}", enc_str(sheet_name), enc_cell_value(value)),
        XlsxMutation::RemoveCell(remove_cell::RemoveCell { sheet_name, row, col }) => format!("remove-cell sheet-name={} row={row} col={col}", enc_str(sheet_name)),
        XlsxMutation::InsertSharedString(insert_shared_string::InsertSharedString { value }) => format!("insert-shared-string value={}", enc_str(value)),
        XlsxMutation::RemoveSharedString(remove_shared_string::RemoveSharedString { index }) => format!("remove-shared-string index={index}"),
        XlsxMutation::SetSharedString(set_shared_string::SetSharedString { index, value }) => format!("set-shared-string index={index} value={}", enc_str(value)),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_xlsx_mutation(line: &str) -> Result<XlsxMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').map(|tok| tok.split_once('=').ok_or_else(|| format!("xlsx mutation: bad arg token {tok:?}"))).collect::<Result<Vec<_>, String>>()?.into_iter().collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("xlsx mutation: missing arg '{k}' for '{keyword}'"));
    let u32_arg = |k: &str| -> Result<u32, String> { arg(k)?.parse().map_err(|e: std::num::ParseIntError| e.to_string()) };
    let usize_arg = |k: &str| -> Result<usize, String> { arg(k)?.parse().map_err(|e: std::num::ParseIntError| e.to_string()) };
    match keyword {
        "set-snapshot" => Ok(XlsxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: dec_xlsx_snapshot(arg("snapshot")?)? })),
        "insert-sheet" => Ok(XlsxMutation::InsertSheet(insert_sheet::InsertSheet { sheet: dec_sheet(arg("sheet")?)? })),
        "remove-sheet" => Ok(XlsxMutation::RemoveSheet(remove_sheet::RemoveSheet { name: dec_str(arg("name")?)? })),
        "rename-sheet" => Ok(XlsxMutation::RenameSheet(rename_sheet::RenameSheet { name: dec_str(arg("name")?)?, new_name: dec_str(arg("new-name")?)? })),
        "set-cell" => Ok(XlsxMutation::SetCell(set_cell::SetCell { sheet_name: dec_str(arg("sheet-name")?)?, row: u32_arg("row")?, col: u32_arg("col")?, value: dec_cell_value(arg("value")?)? })),
        "remove-cell" => Ok(XlsxMutation::RemoveCell(remove_cell::RemoveCell { sheet_name: dec_str(arg("sheet-name")?)?, row: u32_arg("row")?, col: u32_arg("col")? })),
        "insert-shared-string" => Ok(XlsxMutation::InsertSharedString(insert_shared_string::InsertSharedString { value: dec_str(arg("value")?)? })),
        "remove-shared-string" => Ok(XlsxMutation::RemoveSharedString(remove_shared_string::RemoveSharedString { index: usize_arg("index")? })),
        "set-shared-string" => Ok(XlsxMutation::SetSharedString(set_shared_string::SetSharedString { index: usize_arg("index")?, value: dec_str(arg("value")?)? })),
        other => Err(format!("xlsx mutation: unknown keyword {other:?}")),
    }
}

impl OpText for XlsxMutation {
    fn print_op(&self) -> String {
        print_xlsx_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_xlsx_mutation(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
}

//#region 🔖️OpBinaryCodec
/// 🧪️ FG-wave: real recursive binary primitives backing the upgraded `OpBinary` impl below --
/// mirrors docx's own `../🧬️mutations/🦀️component.rs`'s `OpBinaryCodec` region shape (this
/// wave's OPC pattern-setter), reusing `store::pack_rt::write_varint_u64`/`store::ByteReader`
/// plus `XlsxDiff`'s own `write_str_lp`/`read_str_lp`/`enc_opc_part_bin`/`dec_opc_part_bin`/
/// `enc_rel_bin`/`dec_rel_bin`/`enc_sheet_bin`/`dec_sheet_bin`/`enc_cell_value_bin`/
/// `dec_cell_value_bin` (`../🔺️diff/🦀️component.rs`, `pub(crate)` to this artifact).
/// 🌱 Full (non-diff) `OpcContentTypes`/`OpcPackage`/`XlsxWorkbook`/`XlsxSnapshot` binary codecs --
/// only `SetSnapshot`'s whole-payload encoding needs these, mirroring this file's own
/// `enc_content_types`/`enc_opc_package`/`enc_workbook`/`enc_xlsx_snapshot` text forms above.
/// Owners sorted for a deterministic encoding, same `HashMap`-iteration-order caveat those text
/// forms document.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_opc_content_types_bin(ct: &OpcContentTypes, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, ct.defaults.len() as u64);
    for e in &ct.defaults {
        write_str_lp(out, &e.0);
        write_str_lp(out, &e.1);
    }
    store::pack_rt::write_varint_u64(out, ct.overrides.len() as u64);
    for e in &ct.overrides {
        write_str_lp(out, &e.0);
        write_str_lp(out, &e.1);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_opc_content_types_bin(reader: &mut store::ByteReader<'_>) -> Result<OpcContentTypes, String> {
    let default_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut defaults = Vec::with_capacity(default_count as usize);
    for _ in 0..default_count {
        defaults.push((read_str_lp(reader)?, read_str_lp(reader)?));
    }
    let override_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut overrides = Vec::with_capacity(override_count as usize);
    for _ in 0..override_count {
        overrides.push((read_str_lp(reader)?, read_str_lp(reader)?));
    }
    Ok(OpcContentTypes { defaults, overrides })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_opc_package_bin(pkg: &OpcPackage, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, pkg.parts.len() as u64);
    for p in &pkg.parts {
        enc_opc_part_bin(p, out);
    }
    enc_opc_content_types_bin(&pkg.content_types, out);
    let mut owners: Vec<&String> = pkg.relationships.keys().collect();
    owners.sort();
    store::pack_rt::write_varint_u64(out, owners.len() as u64);
    for owner in owners {
        write_str_lp(out, owner);
        let list = &pkg.relationships[owner];
        store::pack_rt::write_varint_u64(out, list.len() as u64);
        for r in list {
            enc_rel_bin(r, out);
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_opc_package_bin(reader: &mut store::ByteReader<'_>) -> Result<OpcPackage, String> {
    let part_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut parts = Vec::with_capacity(part_count as usize);
    for _ in 0..part_count {
        parts.push(dec_opc_part_bin(reader)?);
    }
    let content_types = dec_opc_content_types_bin(reader)?;
    let owner_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut relationships = HashMap::with_capacity(owner_count as usize);
    for _ in 0..owner_count {
        let owner = read_str_lp(reader)?;
        let rel_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
        let mut list = Vec::with_capacity(rel_count as usize);
        for _ in 0..rel_count {
            list.push(dec_rel_bin(reader)?);
        }
        relationships.insert(owner, list);
    }
    Ok(OpcPackage { parts, content_types, relationships, ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_workbook_bin(wb: &XlsxWorkbook, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, wb.sheets.len() as u64);
    for s in &wb.sheets {
        enc_sheet_bin(s, out);
    }
    store::pack_rt::write_varint_u64(out, wb.shared_strings.len() as u64);
    for s in &wb.shared_strings {
        write_str_lp(out, s);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_workbook_bin(reader: &mut store::ByteReader<'_>) -> Result<XlsxWorkbook, String> {
    let sheet_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut sheets = Vec::with_capacity(sheet_count as usize);
    for _ in 0..sheet_count {
        sheets.push(dec_sheet_bin(reader)?);
    }
    let string_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut shared_strings = Vec::with_capacity(string_count as usize);
    for _ in 0..string_count {
        shared_strings.push(read_str_lp(reader)?);
    }
    Ok(XlsxWorkbook { sheets, shared_strings })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_xlsx_snapshot_bin(s: &XlsxSnapshot, out: &mut Vec<u8>) {
    write_str_lp(out, &s.schema);
    enc_opc_package_bin(&s.opc, out);
    enc_workbook_bin(&s.workbook, out);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_xlsx_snapshot_bin(reader: &mut store::ByteReader<'_>) -> Result<XlsxSnapshot, String> {
    let schema = read_str_lp(reader)?;
    let opc = dec_opc_package_bin(reader)?;
    let workbook = dec_workbook_bin(reader)?;
    Ok(XlsxSnapshot { schema, opc, workbook })
}
//#endregion 🔖️OpBinaryCodec

/// 🧪️ FG-wave: REAL binary op frame (`format u8 | tag u8 | variant payload`), matching
/// `../💾️binary/📡️component.protocol.semio`'s `header fixed 2` + `chain payload bytes` shape --
/// upgraded from F6's `print_op().into_bytes()` text-as-binary shortcut (confirmed still on that
/// shortcut live by direct read of this file before this wave, not assumed). `tag` is the
/// `XlsxMutation` variant ordinal, in the SAME 0-9 order `print_xlsx_mutation`'s own keyword
/// match uses.
impl OpBinary for XlsxMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let tag: u8 = match self {
            XlsxMutation::SetSnapshot(_) => 1,
            XlsxMutation::InsertSheet(_) => 2,
            XlsxMutation::RemoveSheet(_) => 3,
            XlsxMutation::RenameSheet(_) => 4,
            XlsxMutation::SetCell(_) => 5,
            XlsxMutation::RemoveCell(_) => 6,
            XlsxMutation::InsertSharedString(_) => 7,
            XlsxMutation::RemoveSharedString(_) => 8,
            XlsxMutation::SetSharedString(_) => 9,
        };
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, tag];
        match self {
            XlsxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => enc_xlsx_snapshot_bin(snapshot, &mut out),
            XlsxMutation::InsertSheet(insert_sheet::InsertSheet { sheet }) => enc_sheet_bin(sheet, &mut out),
            XlsxMutation::RemoveSheet(remove_sheet::RemoveSheet { name }) => write_str_lp(&mut out, name),
            XlsxMutation::RenameSheet(rename_sheet::RenameSheet { name, new_name }) => {
                write_str_lp(&mut out, name);
                write_str_lp(&mut out, new_name);
            }
            XlsxMutation::SetCell(set_cell::SetCell { sheet_name, row, col, value }) => {
                write_str_lp(&mut out, sheet_name);
                store::pack_rt::write_varint_u64(&mut out, *row as u64);
                store::pack_rt::write_varint_u64(&mut out, *col as u64);
                enc_cell_value_bin(value, &mut out);
            }
            XlsxMutation::RemoveCell(remove_cell::RemoveCell { sheet_name, row, col }) => {
                write_str_lp(&mut out, sheet_name);
                store::pack_rt::write_varint_u64(&mut out, *row as u64);
                store::pack_rt::write_varint_u64(&mut out, *col as u64);
            }
            XlsxMutation::InsertSharedString(insert_shared_string::InsertSharedString { value }) => write_str_lp(&mut out, value),
            XlsxMutation::RemoveSharedString(remove_shared_string::RemoveSharedString { index }) => store::pack_rt::write_varint_u64(&mut out, *index as u64),
            XlsxMutation::SetSharedString(set_shared_string::SetSharedString { index, value }) => {
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                write_str_lp(&mut out, value);
            }
        }
        Ok(out)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let _format = reader.read_u8().map_err(|e| malformed("op format", 0, e.to_string()))?;
        let tag = reader.read_u8().map_err(|e| malformed("op tag", 1, e.to_string()))?;
        match tag {
            1 => {
                let snapshot = dec_xlsx_snapshot_bin(&mut reader).map_err(|e| malformed("op snapshot", reader.position(), e))?;
                Ok(XlsxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }))
            }
            2 => {
                let sheet = dec_sheet_bin(&mut reader).map_err(|e| malformed("op sheet", reader.position(), e))?;
                Ok(XlsxMutation::InsertSheet(insert_sheet::InsertSheet { sheet }))
            }
            3 => {
                let name = read_str_lp(&mut reader).map_err(|e| malformed("op name", reader.position(), e))?;
                Ok(XlsxMutation::RemoveSheet(remove_sheet::RemoveSheet { name }))
            }
            4 => {
                let name = read_str_lp(&mut reader).map_err(|e| malformed("op name", reader.position(), e))?;
                let new_name = read_str_lp(&mut reader).map_err(|e| malformed("op new_name", reader.position(), e))?;
                Ok(XlsxMutation::RenameSheet(rename_sheet::RenameSheet { name, new_name }))
            }
            5 => {
                let sheet_name = read_str_lp(&mut reader).map_err(|e| malformed("op sheet_name", reader.position(), e))?;
                let row = reader.read_varint_u64().map_err(|e| malformed("op row", reader.position(), e.to_string()))? as u32;
                let col = reader.read_varint_u64().map_err(|e| malformed("op col", reader.position(), e.to_string()))? as u32;
                let value = dec_cell_value_bin(&mut reader).map_err(|e| malformed("op value", reader.position(), e))?;
                Ok(XlsxMutation::SetCell(set_cell::SetCell { sheet_name, row, col, value }))
            }
            6 => {
                let sheet_name = read_str_lp(&mut reader).map_err(|e| malformed("op sheet_name", reader.position(), e))?;
                let row = reader.read_varint_u64().map_err(|e| malformed("op row", reader.position(), e.to_string()))? as u32;
                let col = reader.read_varint_u64().map_err(|e| malformed("op col", reader.position(), e.to_string()))? as u32;
                Ok(XlsxMutation::RemoveCell(remove_cell::RemoveCell { sheet_name, row, col }))
            }
            7 => {
                let value = read_str_lp(&mut reader).map_err(|e| malformed("op value", reader.position(), e))?;
                Ok(XlsxMutation::InsertSharedString(insert_shared_string::InsertSharedString { value }))
            }
            8 => {
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                Ok(XlsxMutation::RemoveSharedString(remove_shared_string::RemoveSharedString { index }))
            }
            9 => {
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                let value = read_str_lp(&mut reader).map_err(|e| malformed("op value", reader.position(), e))?;
                Ok(XlsxMutation::SetSharedString(set_shared_string::SetSharedString { index, value }))
            }
            other => Err(malformed("op tag", 1, format!("unknown XlsxMutation tag {other}"))),
        }
    }
}
//#endregion 🔖️MutationCodec
//#endregion OpCodecs

//#region 🔖️DemoCases
/// 🧪️ FG-wave: representative `XlsxSnapshot`/`XlsxMutation` fixtures -- the single source of
/// truth reused by this file's own `mutation_diff_law`/`inverse_law`/`op_text_binary_roundtrip_law`
/// tests below AND by `⚙️engine/🦀️component.rs`'s `ops_grammar_conformance_law`/`protocol_walk_law`
/// conformance tests, same shape docx's own `demo_mutation_cases()` establishes (this wave's OPC
/// pattern-setter). Promoted from the former test-only `fixture`/`sweep_a`/`sweep_b`/
/// `sample_mutations` (the last renamed for the same convention).
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn fixture() -> XlsxSnapshot {
    crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::io::export::serializers::build_minimal_xlsx(XlsxWorkbook {
        sheets: vec![XlsxSheet { name: "Sheet1".into(), cells: vec![XlsxCell { row: 1, col: 0, value: XlsxCellValue::Number(1.0) }] }, XlsxSheet { name: "Sheet2".into(), cells: vec![] }],
        shared_strings: vec!["hello".into()],
    })
}

//#region 🔖️Fixtures
/// 🌱 `sweep_a`/`sweep_b`: differ in EVERY mutable field, both `workbook` and `opc`. `sheets`
/// (a true name-keyed collection) gets one removed, one modified-in-every-field (cells:
/// removed+modified+added all in ONE direction, since `(row,col)` is real identity, not
/// position), one added. `shared_strings` (index-keyed, i.e. position-pairwise-matched, same
/// category as `IndexedTripleDiff`) is a DIFFERENT length in each fixture — per this ticket's
/// "known structural trap" note, a single same-direction `between()` over such a collection
/// can never show both `removed` AND `added` from one call, so `a -> b` exercises
/// `removed`+`modified` and `b -> a` (asserted separately in `field_sweep`) exercises
/// `added`+`modified`. OPC content_types/parts/relationships each get one removed, one
/// modified, one added (all true name-keyed collections, exercised in one direction).
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn sweep_a() -> XlsxSnapshot {
    let mut opc = OpcPackage::empty();
    opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
    opc.content_types.set_default("xml", "application/xml");
    opc.content_types.set_default("toRemove", "application/octet-stream");
    opc.set_part("xl/workbook.xml", "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml", b"<workbook/>".to_vec());
    opc.set_part("xl/toModify.xml", "application/xml", b"old".to_vec());
    opc.set_part("xl/toRemove.xml", "application/xml", b"gone".to_vec());
    opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT, "xl/workbook.xml");
    opc.add_relationship("", "rId9", "http://example/toRemove", "xl/toRemove.xml");
    // 🩹 Owner key deliberately `"xl/toModify.xml"`, NOT `"xl/workbook.xml"` — the real
    // engine (`regenerate_workbook_parts`) always populates a MULTI-entry `relationships`
    // list under owner `"xl/workbook.xml"` (one per worksheet + one for shared strings); if
    // this synthetic "modified owner" case reused that exact key, `inverse_law`'s two-hop
    // `SetSnapshot` round trip against `fixture()` (an engine-built snapshot) would compose
    // a partial-overlap MODIFY on a real multi-item Vec, which — per the diff module's own
    // documented survivor-position convention — does not reconstruct exact Vec ORDER through
    // two independent `between()` calls (only content). Using a synthetic owner absent from
    // any engine-built snapshot keeps this a clean whole-owner remove+add in that scenario
    // (captured/restored as one atomic `(owner, Vec<Rel>)` tuple, order-exact by construction)
    // while still genuinely exercising `relationships.modified` here in `field_sweep`.
    opc.relationships.insert("xl/toModify.xml".into(), vec![OpcRelationship { id: "rId2".into(), rel_type: "http://example/toModify".into(), target: "worksheets/old.xml".into(), target_mode: OpcTargetMode::Internal }]);
    opc.relationships.insert("xl/toRemove.xml".into(), vec![OpcRelationship { id: "rId8".into(), rel_type: "http://example/ownerToRemove".into(), target: "media/gone.png".into(), target_mode: OpcTargetMode::Internal }]);

    XlsxSnapshot::from_parts(
        opc,
        XlsxWorkbook {
            sheets: vec![
                XlsxSheet { name: "toModify".into(), cells: vec![XlsxCell { row: 1, col: 0, value: XlsxCellValue::Number(1.0) }, XlsxCell { row: 2, col: 0, value: XlsxCellValue::Boolean(false) }] },
                XlsxSheet { name: "stay".into(), cells: vec![] },
                XlsxSheet { name: "toDrop".into(), cells: vec![XlsxCell { row: 1, col: 0, value: XlsxCellValue::SharedString(0) }] },
            ],
            // 🎯️ Length 3 vs `sweep_b`'s 2: per this ticket's "known structural trap" note, a
            // single same-direction `between()` over an index-keyed (pairwise-position-matched)
            // collection can never show BOTH `removed` AND `added` from one call -- so `a -> b`
            // exercises `shared_strings.removed` (index 2, since `b` is shorter) +
            // `shared_strings.modified` (index 1); `b -> a` (asserted separately in
            // `field_sweep`) exercises `shared_strings.added` (the same index 2, recurring).
            shared_strings: vec!["keep".into(), "toModify".into(), "toRemove".into()],
        },
    )
}

#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn sweep_b() -> XlsxSnapshot {
    let mut opc = OpcPackage::empty();
    opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
    opc.content_types.set_default("xml", "application/xml");
    opc.content_types.set_default("added", "application/octet-stream");
    opc.set_part("xl/workbook.xml", "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml", b"<workbook/>changed".to_vec());
    opc.set_part("xl/toModify.xml", "application/xml", b"new".to_vec());
    opc.set_part("xl/added.xml", "application/xml", b"fresh".to_vec());
    // 🩹 AFTER the `set_part` calls above (which already appended `toModify`'s override entry
    // at position 1): a bare `set_override` on an EXISTING key updates its VALUE in place,
    // never its position — same convention docx's own `field_sweep` fixture documents (the OPC
    // module's `overrides` is order-sensitive `Vec<(String,String)>` equality, not ours to
    // change).
    opc.content_types.set_override("xl/toModify.xml", "application/xml-modified");
    opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT, "xl/workbook.xml");
    opc.relationships.insert("xl/toModify.xml".into(), vec![OpcRelationship { id: "rId2".into(), rel_type: "http://example/toModify".into(), target: "worksheets/new.xml".into(), target_mode: OpcTargetMode::Internal }]);
    opc.relationships.insert("xl/added.xml".into(), vec![OpcRelationship { id: "rId3".into(), rel_type: "http://example/added".into(), target: "media/added.png".into(), target_mode: OpcTargetMode::Internal }]);

    XlsxSnapshot::from_parts(
        opc,
        XlsxWorkbook {
            sheets: vec![
                XlsxSheet {
                    name: "toModify".into(),
                    cells: vec![
                        // row (1,0) survives with a changed value; row (2,0) is dropped; row
                        // (3,0) is a NET-NEW cell -- exercises `cells.removed` +
                        // `cells.modified` + `cells.added` all in the SAME sheet-modify.
                        XlsxCell { row: 1, col: 0, value: XlsxCellValue::Number(2.0) },
                        XlsxCell { row: 3, col: 0, value: XlsxCellValue::Formula { expr: "SUM(A1:A2)".into(), cached: Some(Box::new(XlsxCellValue::Number(3.0))) } },
                    ],
                },
                XlsxSheet { name: "stay".into(), cells: vec![] },
                XlsxSheet { name: "added".into(), cells: vec![XlsxCell { row: 1, col: 1, value: XlsxCellValue::InlineString("brand new".into()) }] },
            ],
            // 🎯️ Length 2: index 2 ("toRemove") no longer exists — exercises
            // `shared_strings.removed` on `a -> b` (see `sweep_a`'s doc comment); the same
            // index recurs as `shared_strings.added` on `b -> a`.
            shared_strings: vec!["keep".into(), "toModify-changed".into()],
        },
    )
}
//#endregion 🔖️Fixtures

/// 🧪️ The demo cases proper -- one representative `XlsxMutation` per variant.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_mutation_cases() -> Vec<XlsxMutation> {
    vec![
        XlsxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: sweep_b() }),
        XlsxMutation::InsertSheet(insert_sheet::InsertSheet { sheet: XlsxSheet { name: "x".into(), cells: vec![] } }),
        // 🎯️ `RemoveSheet`/`RenameSheet` target `"Sheet2"`, the LAST sheet in `fixture()` --
        // same last-position caveat as the dedicated `remove_sheet_inverse_restores_removed_sheet`/
        // `rename_sheet_apply_and_inverse` tests document (`sheets` is name-keyed; a
        // mutation-level `InsertSheet`-based inverse always APPENDS, so exact Vec-position
        // restoration is only guaranteed when the target was already last).
        XlsxMutation::RemoveSheet(remove_sheet::RemoveSheet { name: "Sheet2".into() }),
        XlsxMutation::RenameSheet(rename_sheet::RenameSheet { name: "Sheet2".into(), new_name: "Renamed".into() }),
        XlsxMutation::SetCell(set_cell::SetCell { sheet_name: "Sheet1".into(), row: 1, col: 0, value: XlsxCellValue::Boolean(true) }),
        XlsxMutation::SetCell(set_cell::SetCell { sheet_name: "Sheet1".into(), row: 9, col: 9, value: XlsxCellValue::Number(42.0) }),
        XlsxMutation::RemoveCell(remove_cell::RemoveCell { sheet_name: "Sheet1".into(), row: 1, col: 0 }),
        XlsxMutation::InsertSharedString(insert_shared_string::InsertSharedString { value: "z".into() }),
        // 🎯️ `RemoveSharedString` targets index 0, the LAST-in-position entry `fixture()`
        // (which has only one shared string) has -- like docx's own `RemovePart` precedent
        // (see that artifact's `sample_mutations` doc comment), a name/key-keyed (here
        // index-keyed) collection's mutation-level inverse only restores exact ORIGINAL
        // Vec position when the removed item was already last; the same caveat applies here.
        XlsxMutation::RemoveSharedString(remove_shared_string::RemoveSharedString { index: 0 }),
        XlsxMutation::SetSharedString(set_shared_string::SetSharedString { index: 0, value: "y".into() }),
    ]
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::xlsx::schema::diff::{XlsxCellDiff, XlsxOpcPartDiff};
    use protocol::command::DiffAlgebra;
    use protocol::MutationDiff;

    #[semio_framework_async_macros::async_test]
    async fn insert_then_remove_sheet_apply_and_inverse() {
        let base = fixture();
        let insert = XlsxMutation::InsertSheet(insert_sheet::InsertSheet { sheet: XlsxSheet { name: "New".into(), cells: vec![] } });
        let mut after = base.clone();
        apply_xlsx_mutation(&mut after, &insert);
        assert_eq!(after.workbook.sheets.len(), 3);
        assert!(after.workbook.sheets.iter().any(|s| s.name == "New"));

        for inv in Mutation::inverse(&insert, &base) {
            apply_xlsx_mutation(&mut after, &inv);
        }
        assert_eq!(after, base);
    }

    #[semio_framework_async_macros::async_test]
    async fn remove_sheet_inverse_restores_removed_sheet() {
        // 🎯️ Targets `"Sheet2"`, the LAST sheet in `fixture()` — like docx's own `RemovePart`
        // precedent (see that artifact's `sample_mutations` doc comment), `sheets` is a
        // NAME-keyed collection (position carries no spec meaning), so `RemoveSheet`'s
        // mutation-level inverse (`InsertSheet`, which always APPENDS) only restores the exact
        // original Vec position when the removed sheet was already last — exact positional
        // restoration in the general case is only guaranteed at the diff level.
        let base = fixture();
        let remove = XlsxMutation::RemoveSheet(remove_sheet::RemoveSheet { name: "Sheet2".into() });
        let mut after = base.clone();
        apply_xlsx_mutation(&mut after, &remove);
        assert_eq!(after.workbook.sheets.len(), 1);
        for inv in Mutation::inverse(&remove, &base) {
            apply_xlsx_mutation(&mut after, &inv);
        }
        assert_eq!(after, base);
    }

    #[semio_framework_async_macros::async_test]
    async fn rename_sheet_apply_and_inverse() {
        // 🎯️ Targets `"Sheet2"` (the last sheet, empty) — same last-position caveat as
        // `remove_sheet_inverse_restores_removed_sheet` above: `RenameSheet`'s diff is a
        // remove-old-name + add-new-name (name IS the sheet's identity), so its mutation-level
        // inverse only reproduces the EXACT original Vec position when the renamed sheet was
        // already last.
        let base = fixture();
        let rename = XlsxMutation::RenameSheet(rename_sheet::RenameSheet { name: "Sheet2".into(), new_name: "Renamed".into() });
        let mut after = base.clone();
        apply_xlsx_mutation(&mut after, &rename);
        assert!(!after.workbook.sheets.iter().any(|s| s.name == "Sheet2"));
        let renamed = after.workbook.sheets.iter().find(|s| s.name == "Renamed").expect("renamed sheet present");
        assert!(renamed.cells.is_empty());
        for inv in Mutation::inverse(&rename, &base) {
            apply_xlsx_mutation(&mut after, &inv);
        }
        assert_eq!(after, base);
    }

    #[semio_framework_async_macros::async_test]
    async fn set_and_remove_cell_apply_and_inverse() {
        let base = fixture();
        let set_existing = XlsxMutation::SetCell(set_cell::SetCell { sheet_name: "Sheet1".into(), row: 1, col: 0, value: XlsxCellValue::Boolean(true) });
        let mut after = base.clone();
        apply_xlsx_mutation(&mut after, &set_existing);
        assert_eq!(after.workbook.sheets[0].cells[0].value, XlsxCellValue::Boolean(true));
        for inv in Mutation::inverse(&set_existing, &base) {
            apply_xlsx_mutation(&mut after, &inv);
        }
        assert_eq!(after, base);

        let set_new = XlsxMutation::SetCell(set_cell::SetCell { sheet_name: "Sheet1".into(), row: 2, col: 3, value: XlsxCellValue::InlineString("fresh".into()) });
        let mut after2 = base.clone();
        apply_xlsx_mutation(&mut after2, &set_new);
        assert!(after2.workbook.sheets[0].cells.iter().any(|c| c.row == 2 && c.col == 3 && c.value == XlsxCellValue::InlineString("fresh".into())));
        for inv in Mutation::inverse(&set_new, &base) {
            apply_xlsx_mutation(&mut after2, &inv);
        }
        assert_eq!(after2, base);

        let remove = XlsxMutation::RemoveCell(remove_cell::RemoveCell { sheet_name: "Sheet1".into(), row: 1, col: 0 });
        let mut after3 = base.clone();
        apply_xlsx_mutation(&mut after3, &remove);
        assert!(after3.workbook.sheets[0].cells.is_empty());
        for inv in Mutation::inverse(&remove, &base) {
            apply_xlsx_mutation(&mut after3, &inv);
        }
        assert_eq!(after3, base);
    }

    #[semio_framework_async_macros::async_test]
    async fn shared_string_mutations_apply_and_inverse() {
        let base = fixture();
        let insert = XlsxMutation::InsertSharedString(insert_shared_string::InsertSharedString { value: "world".into() });
        let mut after = base.clone();
        apply_xlsx_mutation(&mut after, &insert);
        assert_eq!(after.workbook.shared_strings, vec!["hello".to_string(), "world".to_string()]);
        for inv in Mutation::inverse(&insert, &base) {
            apply_xlsx_mutation(&mut after, &inv);
        }
        assert_eq!(after, base);

        let set = XlsxMutation::SetSharedString(set_shared_string::SetSharedString { index: 0, value: "changed".into() });
        let mut after2 = base.clone();
        apply_xlsx_mutation(&mut after2, &set);
        assert_eq!(after2.workbook.shared_strings, vec!["changed".to_string()]);
        for inv in Mutation::inverse(&set, &base) {
            apply_xlsx_mutation(&mut after2, &inv);
        }
        assert_eq!(after2, base);

        let remove = XlsxMutation::RemoveSharedString(remove_shared_string::RemoveSharedString { index: 0 });
        let mut after3 = base.clone();
        apply_xlsx_mutation(&mut after3, &remove);
        assert!(after3.workbook.shared_strings.is_empty());
        for inv in Mutation::inverse(&remove, &base) {
            apply_xlsx_mutation(&mut after3, &inv);
        }
        assert_eq!(after3, base);
    }

    //#region 🔖️MutationDiffLaw
    #[semio_framework_async_macros::async_test]
    async fn mutation_diff_law() {
        for mutation in demo_mutation_cases() {
            let base = fixture();
            let diff_direct = Mutation::diff(&mutation, &base);
            let applied_via_diff = MutationDiff::apply(diff_direct.diff(), &base).unwrap();

            let mut via_apply = base.clone();
            let diff_from_apply = apply_xlsx_mutation(&mut via_apply, &mutation);

            assert_eq!(applied_via_diff, via_apply, "mutation_diff_law: apply mismatch for {mutation:?}");
            assert_eq!(diff_direct, diff_from_apply, "mutation_diff_law: diff mismatch for {mutation:?}");
        }
    }
    //#endregion 🔖️MutationDiffLaw

    //#region 🔖️InverseLaw
    #[semio_framework_async_macros::async_test]
    async fn inverse_law() {
        for mutation in demo_mutation_cases() {
            let base = fixture();

            let mut round_tripped = base.clone();
            apply_xlsx_mutation(&mut round_tripped, &mutation);
            for inverse_mutation in <XlsxMutation as Mutation<XlsxSnapshot>>::inverse(&mutation, &base) {
                apply_xlsx_mutation(&mut round_tripped, &inverse_mutation);
            }
            assert_eq!(round_tripped, base, "inverse_law (mutation-level).await failed for {mutation:?}");

            let diff = Mutation::diff(&mutation, &base);
            let next = MutationDiff::apply(diff.diff(), &base).unwrap();
            let inverse_diff = DiffAlgebra::inverse(diff.diff(), &base);
            let restored = MutationDiff::apply(&inverse_diff, &next).unwrap();
            assert_eq!(restored, base, "inverse_law (diff-level).await failed for {mutation:?}");
        }
    }
    //#endregion 🔖️InverseLaw

    //#region 🔖️AbsorbLaw
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn assert_absorb_matches_sequential(base: &XlsxSnapshot, d1: &XlsxDiff, d2: &XlsxDiff) -> XlsxDiff {
        let sequential = MutationDiff::apply(d2, &MutationDiff::apply(d1, base).unwrap()).unwrap();
        let mut absorbed = d1.clone();
        MutationDiff::absorb(&mut absorbed, d2.clone());
        assert_eq!(MutationDiff::apply(&absorbed, base).unwrap(), sequential, "absorb_law: apply(absorb(d1,d2), base) != sequential");
        absorbed
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn cells_diff<'a>(diff: &'a XlsxDiff, sheet_name: &str) -> &'a crate::artifacts::xlsx::schema::diff::XlsxCellsDiff {
        let sheets = diff.workbook.as_ref().expect("workbook diff present").sheets.as_ref().expect("sheets diff present");
        sheets.modified.iter().find(|m| m.key == sheet_name).expect("sheet modified").diff.cells.as_ref().expect("cells diff present")
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_law() {
        // Canonical: SetCell(1,0)+RemoveCell(1,0) on a fresh row -> annihilated add (mirrors
        // Insert+Remove-before): net effect is "never existed".
        {
            let base = fixture();
            let d1 = Mutation::diff(&XlsxMutation::SetCell(set_cell::SetCell { sheet_name: "Sheet2".into(), row: 5, col: 5, value: XlsxCellValue::Number(1.0) }), &base);
            let mid = MutationDiff::apply(d1.diff(), &base).unwrap();
            let d2 = Mutation::diff(&XlsxMutation::RemoveCell(remove_cell::RemoveCell { sheet_name: "Sheet2".into(), row: 5, col: 5 }), &mid);
            let absorbed = assert_absorb_matches_sequential(&base, d1.diff(), d2.diff());
            let triple = cells_diff(&absorbed, "Sheet2");
            assert!(triple.added.is_empty(), "the add must be annihilated by the later remove");
            assert!(triple.removed.is_empty(), "a never-based cell must not appear as a base removal either");
        }

        // Canonical: SetCell(5,5,f)+SetCell(5,6,g) on distinct fresh cells -> both survive
        // (mirrors Insert+Insert-same-index-both-survive: two independent adds never LWW-clobber
        // each other).
        {
            let base = fixture();
            let d1 = Mutation::diff(&XlsxMutation::SetCell(set_cell::SetCell { sheet_name: "Sheet2".into(), row: 5, col: 5, value: XlsxCellValue::InlineString("f".into()) }), &base);
            let mid = MutationDiff::apply(d1.diff(), &base).unwrap();
            let d2 = Mutation::diff(&XlsxMutation::SetCell(set_cell::SetCell { sheet_name: "Sheet2".into(), row: 5, col: 6, value: XlsxCellValue::InlineString("g".into()) }), &mid);
            let absorbed = assert_absorb_matches_sequential(&base, d1.diff(), d2.diff());
            let triple = cells_diff(&absorbed, "Sheet2");
            assert_eq!(triple.added.len(), 2, "both cell adds must survive absorb");
        }

        // Canonical: SetCell(insert)+SetCell(same coord, patch) -> patch into the added payload.
        {
            let base = fixture();
            let d1 = Mutation::diff(&XlsxMutation::SetCell(set_cell::SetCell { sheet_name: "Sheet2".into(), row: 5, col: 5, value: XlsxCellValue::InlineString("f".into()) }), &base);
            let mid = MutationDiff::apply(d1.diff(), &base).unwrap();
            let d2 = Mutation::diff(&XlsxMutation::SetCell(set_cell::SetCell { sheet_name: "Sheet2".into(), row: 5, col: 5, value: XlsxCellValue::InlineString("patched".into()) }), &mid);
            let absorbed = assert_absorb_matches_sequential(&base, d1.diff(), d2.diff());
            let triple = cells_diff(&absorbed, "Sheet2");
            assert!(triple.modified.is_empty(), "patch-into-added must not surface as a separate modified entry");
            assert_eq!(triple.added.len(), 1);
            assert_eq!(triple.added[0].value, XlsxCellValue::InlineString("patched".into()));
        }

        // Canonical: Modify+Remove -> the modify is annihilated by the later remove.
        {
            let base = fixture();
            let d1 = Mutation::diff(&XlsxMutation::SetCell(set_cell::SetCell { sheet_name: "Sheet1".into(), row: 1, col: 0, value: XlsxCellValue::Boolean(true) }), &base);
            let mid = MutationDiff::apply(d1.diff(), &base).unwrap();
            let d2 = Mutation::diff(&XlsxMutation::RemoveCell(remove_cell::RemoveCell { sheet_name: "Sheet1".into(), row: 1, col: 0 }), &mid);
            let absorbed = assert_absorb_matches_sequential(&base, d1.diff(), d2.diff());
            let triple = cells_diff(&absorbed, "Sheet1");
            assert!(triple.modified.is_empty(), "modify of a since-removed cell must not survive absorb");
            assert_eq!(triple.removed, vec![(1u32, 0u32)]);
        }

        // Associativity over a triple.
        {
            let base = fixture();
            let d1 = Mutation::diff(&XlsxMutation::SetCell(set_cell::SetCell { sheet_name: "Sheet2".into(), row: 5, col: 5, value: XlsxCellValue::Number(1.0) }), &base);
            let mid1 = MutationDiff::apply(d1.diff(), &base).unwrap();
            let d2 = Mutation::diff(&XlsxMutation::SetCell(set_cell::SetCell { sheet_name: "Sheet2".into(), row: 6, col: 6, value: XlsxCellValue::Number(2.0) }), &mid1);
            let mid2 = MutationDiff::apply(d2.diff(), &mid1).unwrap();
            let d3 = Mutation::diff(&XlsxMutation::RemoveCell(remove_cell::RemoveCell { sheet_name: "Sheet2".into(), row: 5, col: 5 }), &mid2);
            let sequential = MutationDiff::apply(d3.diff(), &mid2).unwrap();

            let mut left = d1.diff().clone();
            MutationDiff::absorb(&mut left, d2.diff().clone());
            MutationDiff::absorb(&mut left, d3.diff().clone());

            let mut d2_then_d3 = d2.diff().clone();
            MutationDiff::absorb(&mut d2_then_d3, d3.diff().clone());
            let mut right = d1.diff().clone();
            MutationDiff::absorb(&mut right, d2_then_d3);

            assert_eq!(MutationDiff::apply(&left, &base).unwrap(), sequential, "absorb associativity (left) failed");
            assert_eq!(MutationDiff::apply(&right, &base).unwrap(), sequential, "absorb associativity (right) failed");
        }
    }
    //#endregion 🔖️AbsorbLaw

    //#region 🔖️BetweenRoundtripLaw
    #[semio_framework_async_macros::async_test]
    async fn between_roundtrip_law() {
        let a = sweep_a();
        let b = sweep_b();
        assert_eq!(MutationDiff::apply(&<XlsxDiff as DiffAlgebra<XlsxSnapshot>>::between(&a, &b), &a).unwrap(), b);
        assert_eq!(MutationDiff::apply(&<XlsxDiff as DiffAlgebra<XlsxSnapshot>>::between(&b, &a), &b).unwrap(), a);

        let sample = fixture();
        assert_eq!(MutationDiff::apply(&<XlsxDiff as DiffAlgebra<XlsxSnapshot>>::between(&sample, &sample), &sample).unwrap(), sample);

        // "Real" fixture leg: a realistic multi-sheet workbook diffed against a mutated variant.
        let real = crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::io::export::serializers::build_minimal_xlsx(XlsxWorkbook {
            sheets: vec![XlsxSheet { name: "Data".into(), cells: vec![XlsxCell { row: 1, col: 0, value: XlsxCellValue::SharedString(0) }] }],
            shared_strings: vec!["Chapter One".into()],
        });
        let mut mutated = real.clone();
        apply_xlsx_mutation(&mut mutated, &XlsxMutation::SetSharedString(set_shared_string::SetSharedString { index: 0, value: "Chapter Two".into() }));
        assert_ne!(real, mutated);
        assert_eq!(MutationDiff::apply(&<XlsxDiff as DiffAlgebra<XlsxSnapshot>>::between(&real, &mutated), &real).unwrap(), mutated);
        assert_eq!(MutationDiff::apply(&<XlsxDiff as DiffAlgebra<XlsxSnapshot>>::between(&mutated, &real), &mutated).unwrap(), real);
    }
    //#endregion 🔖️BetweenRoundtripLaw

    //#region 🔖️CodecRetentionLaw
    #[semio_framework_async_macros::async_test]
    async fn codec_retention_law() {
        let snap = crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::io::export::serializers::build_minimal_xlsx(XlsxWorkbook {
            sheets: vec![XlsxSheet {
                name: "Sheet1".into(),
                cells: vec![
                    XlsxCell { row: 1, col: 0, value: XlsxCellValue::SharedString(0) },
                    XlsxCell { row: 1, col: 1, value: XlsxCellValue::Number(9.5) },
                    XlsxCell { row: 2, col: 0, value: XlsxCellValue::Boolean(true) },
                    XlsxCell { row: 2, col: 1, value: XlsxCellValue::InlineString("literal".into()) },
                    XlsxCell { row: 3, col: 0, value: XlsxCellValue::Formula { expr: "SUM(B1:B2)".into(), cached: Some(Box::new(XlsxCellValue::Number(9.5))) } },
                ],
            }],
            shared_strings: vec!["Hello".into()],
        });
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <XlsxSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }
    //#endregion 🔖️CodecRetentionLaw

    //#region 🔖️FieldSweep
    /// 🎯️ THE acceptance criterion: `sweep_a`/`sweep_b` differ in every mutable field across BOTH
    /// `opc` and `workbook` (see the fixtures' doc comment for exactly how each collection flavor
    /// — removed/modified/added — is exercised).
    #[semio_framework_async_macros::async_test]
    async fn field_sweep() {
        let a = sweep_a();
        let b = sweep_b();

        let diff_ab = <XlsxDiff as DiffAlgebra<XlsxSnapshot>>::between(&a, &b);
        assert_eq!(MutationDiff::apply(&diff_ab, &a).unwrap(), b);
        let diff_ba = <XlsxDiff as DiffAlgebra<XlsxSnapshot>>::between(&b, &a);
        assert_eq!(MutationDiff::apply(&diff_ba, &b).unwrap(), a);
        assert!(<XlsxDiff as DiffAlgebra<XlsxSnapshot>>::between(&a, &a).is_empty());

        // opc: content_types (both defaults+overrides), parts, relationships all populated.
        let opc_diff = diff_ab.opc.as_ref().expect("opc diff present");
        let ct = opc_diff.content_types.as_ref().expect("content_types diff present");
        let defaults = ct.defaults.as_ref().expect("defaults diff present");
        assert!(!defaults.added.is_empty(), "content_types.defaults: added not exercised");
        let overrides = ct.overrides.as_ref().expect("overrides diff present");
        assert!(!overrides.modified.is_empty(), "content_types.overrides: modified not exercised");
        let parts = opc_diff.parts.as_ref().expect("parts diff present");
        assert!(!parts.removed.is_empty(), "opc.parts: removed not exercised");
        assert!(!parts.modified.is_empty(), "opc.parts: modified not exercised");
        assert!(!parts.added.is_empty(), "opc.parts: added not exercised");
        let part_mod = &parts.modified[0];
        assert!(matches!(&part_mod.diff, XlsxOpcPartDiff { bytes: Some(_), .. }));
        let rels = opc_diff.relationships.as_ref().expect("relationships diff present");
        assert!(!rels.removed.is_empty(), "opc.relationships: removed (owner) not exercised");
        assert!(!rels.modified.is_empty(), "opc.relationships: modified (owner) not exercised");
        assert!(!rels.added.is_empty(), "opc.relationships: added (owner) not exercised");

        // workbook.sheets: removed ("toDrop") + modified ("toModify", whose OWN cells diff
        // exercises removed+modified+added together) + added ("added", carried whole).
        let wb_diff = diff_ab.workbook.as_ref().expect("workbook diff present");
        let sheets_diff = wb_diff.sheets.as_ref().expect("sheets diff present");
        assert!(sheets_diff.removed.contains(&"toDrop".to_string()), "sheets: removed not exercised");
        assert!(sheets_diff.added.iter().any(|s| s.name == "added"), "sheets: added not exercised");
        let sheet_mod = sheets_diff.modified.iter().find(|m| m.key == "toModify").expect("toModify sheet modified");
        let cells_diff = sheet_mod.diff.cells.as_ref().expect("toModify cells diff present");
        assert!(!cells_diff.removed.is_empty(), "toModify.cells: removed not exercised");
        assert!(!cells_diff.modified.is_empty(), "toModify.cells: modified not exercised");
        assert!(!cells_diff.added.is_empty(), "toModify.cells: added not exercised");
        let cell_mod = &cells_diff.modified[0];
        assert!(matches!(&cell_mod.diff, XlsxCellDiff { value: Some(_) }));
        // The added cell in `toModify` carries a `Formula` value — exercises that variant too.
        assert!(matches!(&cells_diff.added[0].value, XlsxCellValue::Formula { .. }), "added cell should carry a Formula value");

        // The dropped sheet's full payload recurs as an `added` item in the OTHER direction.
        let sheets_diff_ba = diff_ba.workbook.as_ref().unwrap().sheets.as_ref().expect("sheets diff (b->a) present");
        let added_back = sheets_diff_ba.added.iter().find(|s| s.name == "toDrop").expect("toDrop sheet re-added in b->a");
        assert!(!added_back.cells.is_empty());

        // workbook.shared_strings (index-keyed, pairwise-position-matched): per the "known
        // structural trap" note, `a -> b` (shorter) exercises removed+modified; `b -> a`
        // (asserted separately) exercises added+modified.
        let ss_diff = wb_diff.shared_strings.as_ref().expect("shared_strings diff present");
        assert!(!ss_diff.removed.is_empty(), "shared_strings: removed not exercised");
        assert!(!ss_diff.modified.is_empty(), "shared_strings: modified not exercised");
        let ss_diff_ba = diff_ba.workbook.as_ref().unwrap().shared_strings.as_ref().expect("shared_strings diff (b->a) present");
        assert!(!ss_diff_ba.added.is_empty(), "shared_strings (b->a): added not exercised");
        assert!(!ss_diff_ba.modified.is_empty(), "shared_strings (b->a): modified not exercised");
    }
    //#endregion 🔖️FieldSweep

    //#region 🔖️OpTextBinaryRoundtripLaw
    /// 🧪️ F6: `OpText`/`OpBinary` round-trip laws for the hand-rolled `XlsxMutation` grammar —
    /// exercises every variant, incl. `SetSnapshot`'s full nested `XlsxSnapshot` (opc parts,
    /// content-types, relationships incl. `OpcTargetMode::External`, workbook sheets/cells/shared
    /// strings) and `SetCell`'s direct `XlsxCellValue` payload (incl. `Formula.cached` and raw
    /// `,`/`:`/`[`/`]` bytes-through-hex in a string value).
    #[semio_framework_async_macros::async_test]
    async fn op_text_binary_roundtrip_law() {
        let mut snapshot_with_opc = sweep_b();
        snapshot_with_opc.opc.relationships.get_mut("xl/toModify.xml").unwrap()[0].target_mode = OpcTargetMode::External;

        let mutations = vec![
            XlsxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: snapshot_with_opc }),
            XlsxMutation::InsertSheet(insert_sheet::InsertSheet { sheet: XlsxSheet { name: "New, odd: [name]".into(), cells: vec![XlsxCell { row: 1, col: 2, value: XlsxCellValue::Empty }] } }),
            XlsxMutation::RemoveSheet(remove_sheet::RemoveSheet { name: "Sheet2".into() }),
            XlsxMutation::RenameSheet(rename_sheet::RenameSheet { name: "Sheet2".into(), new_name: "Renamed".into() }),
            XlsxMutation::SetCell(set_cell::SetCell { sheet_name: "Sheet1".into(), row: 1, col: 0, value: XlsxCellValue::Number(-2.5) }),
            XlsxMutation::SetCell(set_cell::SetCell { sheet_name: "Sheet1".into(), row: 2, col: 0, value: XlsxCellValue::SharedString(3) }),
            XlsxMutation::SetCell(set_cell::SetCell { sheet_name: "Sheet1".into(), row: 3, col: 0, value: XlsxCellValue::Boolean(false) }),
            XlsxMutation::SetCell(set_cell::SetCell { sheet_name: "Sheet1".into(), row: 4, col: 0, value: XlsxCellValue::InlineString("has, weird: [chars]".into()) }),
            XlsxMutation::SetCell(set_cell::SetCell { sheet_name: "Sheet1".into(), row: 5, col: 0, value: XlsxCellValue::Formula { expr: "SUM(A1:A4)".into(), cached: Some(Box::new(XlsxCellValue::Number(1.5))) } }),
            XlsxMutation::SetCell(set_cell::SetCell { sheet_name: "Sheet1".into(), row: 6, col: 0, value: XlsxCellValue::Formula { expr: "NA()".into(), cached: None } }),
            XlsxMutation::RemoveCell(remove_cell::RemoveCell { sheet_name: "Sheet1".into(), row: 1, col: 0 }),
            XlsxMutation::InsertSharedString(insert_shared_string::InsertSharedString { value: "z".into() }),
            XlsxMutation::RemoveSharedString(remove_shared_string::RemoveSharedString { index: 0 }),
            XlsxMutation::SetSharedString(set_shared_string::SetSharedString { index: 0, value: "y".into() }),
        ];
        for mutation in mutations {
            let printed = mutation.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = XlsxMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch for {mutation:?} (printed {printed:?})");

            let encoded = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op({mutation:?}) failed: {e}"));
            let decoded = XlsxMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, mutation, "encode_op/decode_op round-trip mismatch for {mutation:?}");
        }
    }
    //#endregion 🔖️OpTextBinaryRoundtripLaw

    //#region 🔖️KindsConformanceLaw
    /// 🧭️ `kind_of` is an EXHAUSTIVE match (no wildcard arm) — the compiler refuses this file if a
    /// variant is added to `XlsxMutation` without a matching kebab-case spelling here, which is what
    /// keeps `KINDS` honest against the enum. The second half reads the sibling oracle manifest's
    /// `kinds` array as text (the framework never parses Rust, so this is the only side that can
    /// prove the manifest matches) and asserts the same list, in the same order.
    #[semio_framework_async_macros::async_test]
    async fn kinds_match_enum_and_catalog() {
        fn kind_of(mutation: &XlsxMutation) -> &'static str {
            match mutation {
                XlsxMutation::SetSnapshot(set_snapshot::SetSnapshot { .. }) => "set-snapshot",
                XlsxMutation::InsertSheet(insert_sheet::InsertSheet { .. }) => "insert-sheet",
                XlsxMutation::RemoveSheet(remove_sheet::RemoveSheet { .. }) => "remove-sheet",
                XlsxMutation::RenameSheet(rename_sheet::RenameSheet { .. }) => "rename-sheet",
                XlsxMutation::SetCell(set_cell::SetCell { .. }) => "set-cell",
                XlsxMutation::RemoveCell(remove_cell::RemoveCell { .. }) => "remove-cell",
                XlsxMutation::InsertSharedString(insert_shared_string::InsertSharedString { .. }) => "insert-shared-string",
                XlsxMutation::RemoveSharedString(remove_shared_string::RemoveSharedString { .. }) => "remove-shared-string",
                XlsxMutation::SetSharedString(set_shared_string::SetSharedString { .. }) => "set-shared-string",
            }
        }
        let samples = [
            XlsxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: XlsxSnapshot::default() }),
            XlsxMutation::InsertSheet(insert_sheet::InsertSheet { sheet: XlsxSheet::default() }),
            XlsxMutation::RemoveSheet(remove_sheet::RemoveSheet { name: String::new() }),
            XlsxMutation::RenameSheet(rename_sheet::RenameSheet { name: String::new(), new_name: String::new() }),
            XlsxMutation::SetCell(set_cell::SetCell { sheet_name: String::new(), row: 0, col: 0, value: XlsxCellValue::Empty }),
            XlsxMutation::RemoveCell(remove_cell::RemoveCell { sheet_name: String::new(), row: 0, col: 0 }),
            XlsxMutation::InsertSharedString(insert_shared_string::InsertSharedString { value: String::new() }),
            XlsxMutation::RemoveSharedString(remove_shared_string::RemoveSharedString { index: 0 }),
            XlsxMutation::SetSharedString(set_shared_string::SetSharedString { index: 0, value: String::new() }),
        ];
        let from_enum: Vec<&'static str> = samples.iter().map(kind_of).collect();
        assert_eq!(from_enum, KINDS, "KINDS must list every XlsxMutation variant, in declaration order");

        let manifest = include_str!("../../🧪️oracle/🔣️.json");
        let needle = "\"kinds\": [";
        let start = manifest.find(needle).expect("manifest declares a kinds array") + needle.len();
        let end = start + manifest[start..].find(']').expect("kinds array is closed");
        let declared: Vec<String> = manifest[start..end].split(',').map(|entry| entry.trim().trim_matches('"').to_string()).filter(|entry| !entry.is_empty()).collect();
        assert_eq!(declared, KINDS, "the oracle manifest's kinds must match XlsxMutation exactly");
    }
    //#endregion 🔖️KindsConformanceLaw
}
//#endregion 🧪️Tests

//#region 🧪️FixtureTests
// 🧪️ Handcrafted mutation fixtures (contract D1, ticket 26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION),
// one case per mutation leaf. Wired HERE and not in `📦️glue.rs`: that file is shared with the
// agents migrating the other stdio artifacts, so the production mounts there stay untouched while
// this artifact owns its own test mount. `#[path = "."]` re-bases the children on this file's own
// directory, which is what makes the leaf-relative path below resolve.
#[cfg(test)]
#[path = "."]
mod fixture_tests {
    #[path = "📄set-snapshot/🧪️tests/widens-the-total-formula-to-a-third-row/🦀️component.rs"]
    mod tests_set_snapshot_widens_the_total_formula_to_a_third_row;
}
//#endregion 🧪️FixtureTests
