//! 🧬️ XlsxMutation — document mutation dispatch. Every variant's `diff()` is handcrafted (never
//! apply-and-capture) and every variant's `inverse()` is handcrafted, key/index-aware.

use crate::artifacts::xlsx::schema::diff::{
    dec_cell_value, dec_ct_entry, dec_owner_rels, dec_part, dec_sheet, dec_str, diff_insert_sheet, diff_insert_shared_string, diff_remove_cell,
    diff_remove_shared_string, diff_remove_sheet, diff_rename_sheet, diff_set_cell, diff_set_shared_string, diff_set_snapshot, enc_cell_value,
    enc_ct_entry, enc_owner_rels, enc_part, enc_sheet, enc_str, split_top_level, strip_brackets, XlsxDiff,
};
use crate::artifacts::xlsx::schema::snapshot::{XlsxCellValue, XlsxSheet, XlsxWorkbook};
use crate::artifacts::xlsx::XlsxSnapshot;
use crate::artifacts::zip::opc::{OpcContentTypes, OpcPackage, OpcRelationship};
use protocol::{Mutation, OpText};
#[cfg(test)]
use protocol::OpBinary;
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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum XlsxMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: XlsxSnapshot,
    },
    /// ➕️ Inserts a brand-new sheet (possibly pre-populated with cells).
    InsertSheet {
        sheet: XlsxSheet,
    },
    /// ➖️ Removes the sheet named `name`.
    RemoveSheet {
        name: String,
    },
    /// 🏷️ Renames the sheet named `name` to `new_name` (a remove-old+add-new at the diff level —
    /// `name` is the sheet's identity, see the snapshot module's doc comment).
    RenameSheet {
        name: String,
        new_name: String,
    },
    /// ✍️ Sets (inserting or replacing) the value of the cell at `(row, col)` in sheet
    /// `sheet_name`.
    SetCell {
        sheet_name: String,
        row: u32,
        col: u32,
        value: XlsxCellValue,
    },
    /// ➖️ Removes the cell at `(row, col)` in sheet `sheet_name`.
    RemoveCell {
        sheet_name: String,
        row: u32,
        col: u32,
    },
    /// ➕️ Appends a new shared string.
    InsertSharedString {
        value: String,
    },
    /// ➖️ Removes the shared string at `index`.
    RemoveSharedString {
        index: usize,
    },
    /// ✍️ Replaces the shared string at `index`.
    SetSharedString {
        index: usize,
        value: String,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`: `let d = mutation.diff(&*snapshot); *snapshot =
/// d.apply(snapshot); d` — the diff is the single semantics source, never a separate imperative
/// apply path (apply-and-capture is banned).
pub fn apply_xlsx_mutation(snapshot: &mut XlsxSnapshot, mutation: &XlsxMutation) -> XlsxDiff {
    let diff = Mutation::diff(mutation, snapshot);
    *snapshot = protocol::MutationDiff::apply(&diff, snapshot);
    diff
}
//#endregion 🔖️Apply

//#region 🔖️Helpers
fn sheet_at<'a>(base: &'a XlsxSnapshot, name: &str) -> Option<&'a XlsxSheet> {
    base.workbook.sheets.iter().find(|s| s.name == name)
}

fn cell_value_at(base: &XlsxSnapshot, sheet_name: &str, row: u32, col: u32) -> Option<XlsxCellValue> {
    sheet_at(base, sheet_name)?.cells.iter().find(|c| c.row == row && c.col == col).map(|c| c.value.clone())
}
//#endregion 🔖️Helpers

//#region 🔖️MutationTrait
impl Mutation<XlsxSnapshot> for XlsxMutation {
    type Diff = XlsxDiff;

    fn diff(&self, base: &XlsxSnapshot) -> Self::Diff {
        match self {
            XlsxMutation::NoMutation => XlsxDiff::default(),
            XlsxMutation::SetSnapshot { snapshot } => diff_set_snapshot(base, snapshot),
            XlsxMutation::InsertSheet { sheet } => diff_insert_sheet(sheet.clone()),
            XlsxMutation::RemoveSheet { name } => diff_remove_sheet(name),
            XlsxMutation::RenameSheet { name, new_name } => match sheet_at(base, name) {
                Some(sheet) => diff_rename_sheet(sheet, new_name),
                None => XlsxDiff::default(),
            },
            XlsxMutation::SetCell { sheet_name, row, col, value } => match sheet_at(base, sheet_name) {
                Some(sheet) => diff_set_cell(sheet, *row, *col, value.clone()),
                None => XlsxDiff::default(),
            },
            XlsxMutation::RemoveCell { sheet_name, row, col } => diff_remove_cell(sheet_name, *row, *col),
            XlsxMutation::InsertSharedString { value } => diff_insert_shared_string(base.workbook.shared_strings.len(), value).1,
            XlsxMutation::RemoveSharedString { index } => diff_remove_shared_string(*index),
            XlsxMutation::SetSharedString { index, value } => diff_set_shared_string(&base.workbook.shared_strings, *index, value),
        }
    }

    fn inverse(&self, base: &XlsxSnapshot) -> Vec<Self> {
        match self {
            XlsxMutation::NoMutation => vec![XlsxMutation::NoMutation],
            XlsxMutation::SetSnapshot { .. } => vec![XlsxMutation::SetSnapshot { snapshot: base.clone() }],
            XlsxMutation::InsertSheet { sheet } => vec![XlsxMutation::RemoveSheet { name: sheet.name.clone() }],
            XlsxMutation::RemoveSheet { name } => match sheet_at(base, name) {
                Some(sheet) => vec![XlsxMutation::InsertSheet { sheet: sheet.clone() }],
                None => vec![XlsxMutation::NoMutation],
            },
            XlsxMutation::RenameSheet { name, new_name } => match sheet_at(base, name) {
                Some(_) => vec![XlsxMutation::RenameSheet { name: new_name.clone(), new_name: name.clone() }],
                None => vec![XlsxMutation::NoMutation],
            },
            XlsxMutation::SetCell { sheet_name, row, col, .. } => match cell_value_at(base, sheet_name, *row, *col) {
                Some(value) => vec![XlsxMutation::SetCell { sheet_name: sheet_name.clone(), row: *row, col: *col, value }],
                None => vec![XlsxMutation::RemoveCell { sheet_name: sheet_name.clone(), row: *row, col: *col }],
            },
            XlsxMutation::RemoveCell { sheet_name, row, col } => match cell_value_at(base, sheet_name, *row, *col) {
                Some(value) => vec![XlsxMutation::SetCell { sheet_name: sheet_name.clone(), row: *row, col: *col, value }],
                None => vec![XlsxMutation::NoMutation],
            },
            XlsxMutation::InsertSharedString { .. } => vec![XlsxMutation::RemoveSharedString { index: base.workbook.shared_strings.len() }],
            XlsxMutation::RemoveSharedString { index } => match base.workbook.shared_strings.get(*index) {
                Some(value) => vec![XlsxMutation::SetSharedString { index: *index, value: value.clone() }],
                None => vec![XlsxMutation::NoMutation],
            },
            XlsxMutation::SetSharedString { index, .. } => match base.workbook.shared_strings.get(*index) {
                Some(value) => vec![XlsxMutation::SetSharedString { index: *index, value: value.clone() }],
                None => vec![XlsxMutation::NoMutation],
            },
        }
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
fn enc_content_types(ct: &OpcContentTypes) -> String {
    let defaults = ct.defaults.iter().map(enc_ct_entry).collect::<Vec<_>>().join(",");
    let overrides = ct.overrides.iter().map(enc_ct_entry).collect::<Vec<_>>().join(",");
    format!("[[{defaults}],[{overrides}]]")
}
fn dec_content_types(s: &str) -> Result<OpcContentTypes, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [defaults, overrides] = parts.as_slice() else { return Err(format!("content types: expected 2 fields, got {}", parts.len())) };
    let defaults = split_top_level(strip_brackets(defaults)?, ',').into_iter().map(dec_ct_entry).collect::<Result<Vec<_>, String>>()?;
    let overrides = split_top_level(strip_brackets(overrides)?, ',').into_iter().map(dec_ct_entry).collect::<Result<Vec<_>, String>>()?;
    Ok(OpcContentTypes { defaults, overrides })
}
/// 🗺️ Owners sorted for determinism (`HashMap` iteration order is not stable) — matches this
/// artifact's other `HashMap`-backed encodings' expectation of a canonical wire order.
fn enc_relationships_map(rels: &HashMap<String, Vec<OpcRelationship>>) -> String {
    let mut owners: Vec<&String> = rels.keys().collect();
    owners.sort();
    let entries = owners.into_iter().map(|o| enc_owner_rels(&(o.clone(), rels[o].clone()))).collect::<Vec<_>>().join(",");
    format!("[{entries}]")
}
fn dec_relationships_map(s: &str) -> Result<HashMap<String, Vec<OpcRelationship>>, String> {
    let entries = split_top_level(strip_brackets(s)?, ',').into_iter().map(dec_owner_rels).collect::<Result<Vec<_>, String>>()?;
    Ok(entries.into_iter().collect())
}
fn enc_opc_package(pkg: &OpcPackage) -> String {
    let parts = pkg.parts.iter().map(enc_part).collect::<Vec<_>>().join(",");
    format!("[[{parts}],{},{}]", enc_content_types(&pkg.content_types), enc_relationships_map(&pkg.relationships))
}
fn dec_opc_package(s: &str) -> Result<OpcPackage, String> {
    let outer = split_top_level(strip_brackets(s)?, ',');
    let [parts, ct, rels] = outer.as_slice() else { return Err(format!("opc package: expected 3 fields, got {}", outer.len())) };
    let parts = split_top_level(strip_brackets(parts)?, ',').into_iter().map(dec_part).collect::<Result<Vec<_>, String>>()?;
    Ok(OpcPackage { parts, content_types: dec_content_types(ct)?, relationships: dec_relationships_map(rels)? })
}
fn enc_workbook(wb: &XlsxWorkbook) -> String {
    let sheets = wb.sheets.iter().map(enc_sheet).collect::<Vec<_>>().join(",");
    let strings = wb.shared_strings.iter().map(|s| enc_str(s)).collect::<Vec<_>>().join(",");
    format!("[[{sheets}],[{strings}]]")
}
fn dec_workbook(s: &str) -> Result<XlsxWorkbook, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [sheets, strings] = parts.as_slice() else { return Err(format!("workbook: expected 2 fields, got {}", parts.len())) };
    let sheets = split_top_level(strip_brackets(sheets)?, ',').into_iter().map(dec_sheet).collect::<Result<Vec<_>, String>>()?;
    let shared_strings = split_top_level(strip_brackets(strings)?, ',').into_iter().map(dec_str).collect::<Result<Vec<_>, String>>()?;
    Ok(XlsxWorkbook { sheets, shared_strings })
}
fn enc_xlsx_snapshot(s: &XlsxSnapshot) -> String {
    format!("[{},{},{}]", enc_str(&s.schema), enc_opc_package(&s.opc), enc_workbook(&s.workbook))
}
fn dec_xlsx_snapshot(s: &str) -> Result<XlsxSnapshot, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [schema, opc, workbook] = parts.as_slice() else { return Err(format!("xlsx snapshot: expected 3 fields, got {}", parts.len())) };
    Ok(XlsxSnapshot { schema: dec_str(schema)?, opc: dec_opc_package(opc)?, workbook: dec_workbook(workbook)? })
}
//#endregion 🔖️SnapshotCodec

//#region 🔖️MutationCodec
fn print_xlsx_mutation(m: &XlsxMutation) -> String {
    match m {
        XlsxMutation::NoMutation => "no-mutation".to_string(),
        XlsxMutation::SetSnapshot { snapshot } => format!("set-snapshot snapshot={}", enc_xlsx_snapshot(snapshot)),
        XlsxMutation::InsertSheet { sheet } => format!("insert-sheet sheet={}", enc_sheet(sheet)),
        XlsxMutation::RemoveSheet { name } => format!("remove-sheet name={}", enc_str(name)),
        XlsxMutation::RenameSheet { name, new_name } => format!("rename-sheet name={} new-name={}", enc_str(name), enc_str(new_name)),
        XlsxMutation::SetCell { sheet_name, row, col, value } => format!("set-cell sheet-name={} row={row} col={col} value={}", enc_str(sheet_name), enc_cell_value(value)),
        XlsxMutation::RemoveCell { sheet_name, row, col } => format!("remove-cell sheet-name={} row={row} col={col}", enc_str(sheet_name)),
        XlsxMutation::InsertSharedString { value } => format!("insert-shared-string value={}", enc_str(value)),
        XlsxMutation::RemoveSharedString { index } => format!("remove-shared-string index={index}"),
        XlsxMutation::SetSharedString { index, value } => format!("set-shared-string index={index} value={}", enc_str(value)),
    }
}
fn parse_xlsx_mutation(line: &str) -> Result<XlsxMutation, String> {
    if line == "no-mutation" {
        return Ok(XlsxMutation::NoMutation);
    }
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> = rest
        .split(' ')
        .map(|tok| tok.split_once('=').ok_or_else(|| format!("xlsx mutation: bad arg token {tok:?}")))
        .collect::<Result<Vec<_>, String>>()?
        .into_iter()
        .collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("xlsx mutation: missing arg '{k}' for '{keyword}'"));
    let u32_arg = |k: &str| -> Result<u32, String> { arg(k)?.parse().map_err(|e: std::num::ParseIntError| e.to_string()) };
    let usize_arg = |k: &str| -> Result<usize, String> { arg(k)?.parse().map_err(|e: std::num::ParseIntError| e.to_string()) };
    match keyword {
        "set-snapshot" => Ok(XlsxMutation::SetSnapshot { snapshot: dec_xlsx_snapshot(arg("snapshot")?)? }),
        "insert-sheet" => Ok(XlsxMutation::InsertSheet { sheet: dec_sheet(arg("sheet")?)? }),
        "remove-sheet" => Ok(XlsxMutation::RemoveSheet { name: dec_str(arg("name")?)? }),
        "rename-sheet" => Ok(XlsxMutation::RenameSheet { name: dec_str(arg("name")?)?, new_name: dec_str(arg("new-name")?)? }),
        "set-cell" => Ok(XlsxMutation::SetCell { sheet_name: dec_str(arg("sheet-name")?)?, row: u32_arg("row")?, col: u32_arg("col")?, value: dec_cell_value(arg("value")?)? }),
        "remove-cell" => Ok(XlsxMutation::RemoveCell { sheet_name: dec_str(arg("sheet-name")?)?, row: u32_arg("row")?, col: u32_arg("col")? }),
        "insert-shared-string" => Ok(XlsxMutation::InsertSharedString { value: dec_str(arg("value")?)? }),
        "remove-shared-string" => Ok(XlsxMutation::RemoveSharedString { index: usize_arg("index")? }),
        "set-shared-string" => Ok(XlsxMutation::SetSharedString { index: usize_arg("index")?, value: dec_str(arg("value")?)? }),
        other => Err(format!("xlsx mutation: unknown keyword {other:?}")),
    }
}

impl protocol::OpText for XlsxMutation {
    fn print_op(&self) -> String {
        print_xlsx_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_xlsx_mutation(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
}

/// ⚡️ Binary = the text bytes verbatim, same simplification as `XlsxDiff`'s hand-rolled codec.
impl protocol::OpBinary for XlsxMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(self.print_op().into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let line = std::str::from_utf8(bytes).map_err(|e| protocol::ProtocolError::Malformed { what: "op utf8", offset: 0, detail: e.to_string() })?;
        Self::parse_op(line).map_err(|e| protocol::ProtocolError::Malformed { what: "op text", offset: 0, detail: e.to_string() })
    }
}
//#endregion 🔖️MutationCodec
//#endregion OpCodecs

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::xlsx::schema::diff::{XlsxCellDiff, XlsxOpcPartDiff};
    use crate::artifacts::xlsx::schema::snapshot::{XlsxCell, XlsxWorkbook};
    use crate::artifacts::zip::opc::{OpcPackage, OpcRelationship, OpcTargetMode, REL_TYPE_OFFICE_DOCUMENT, RELS_CONTENT_TYPE};
    use protocol::command::DiffAlgebra;
    use protocol::MutationDiff;

    fn fixture() -> XlsxSnapshot {
        crate::artifacts::xlsx::engine::build_minimal_xlsx(XlsxWorkbook {
            sheets: vec![
                XlsxSheet { name: "Sheet1".into(), cells: vec![XlsxCell { row: 1, col: 0, value: XlsxCellValue::Number(1.0) }] },
                XlsxSheet { name: "Sheet2".into(), cells: vec![] },
            ],
            shared_strings: vec!["hello".into()],
        })
    }

    #[test]
    fn insert_then_remove_sheet_apply_and_inverse() {
        let base = fixture();
        let insert = XlsxMutation::InsertSheet { sheet: XlsxSheet { name: "New".into(), cells: vec![] } };
        let mut after = base.clone();
        apply_xlsx_mutation(&mut after, &insert);
        assert_eq!(after.workbook.sheets.len(), 3);
        assert!(after.workbook.sheets.iter().any(|s| s.name == "New"));

        for inv in Mutation::inverse(&insert, &base) {
            apply_xlsx_mutation(&mut after, &inv);
        }
        assert_eq!(after, base);
    }

    #[test]
    fn remove_sheet_inverse_restores_removed_sheet() {
        // 🎯️ Targets `"Sheet2"`, the LAST sheet in `fixture()` — like docx's own `RemovePart`
        // precedent (see that artifact's `sample_mutations` doc comment), `sheets` is a
        // NAME-keyed collection (position carries no spec meaning), so `RemoveSheet`'s
        // mutation-level inverse (`InsertSheet`, which always APPENDS) only restores the exact
        // original Vec position when the removed sheet was already last — exact positional
        // restoration in the general case is only guaranteed at the diff level.
        let base = fixture();
        let remove = XlsxMutation::RemoveSheet { name: "Sheet2".into() };
        let mut after = base.clone();
        apply_xlsx_mutation(&mut after, &remove);
        assert_eq!(after.workbook.sheets.len(), 1);
        for inv in Mutation::inverse(&remove, &base) {
            apply_xlsx_mutation(&mut after, &inv);
        }
        assert_eq!(after, base);
    }

    #[test]
    fn rename_sheet_apply_and_inverse() {
        // 🎯️ Targets `"Sheet2"` (the last sheet, empty) — same last-position caveat as
        // `remove_sheet_inverse_restores_removed_sheet` above: `RenameSheet`'s diff is a
        // remove-old-name + add-new-name (name IS the sheet's identity), so its mutation-level
        // inverse only reproduces the EXACT original Vec position when the renamed sheet was
        // already last.
        let base = fixture();
        let rename = XlsxMutation::RenameSheet { name: "Sheet2".into(), new_name: "Renamed".into() };
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

    #[test]
    fn set_and_remove_cell_apply_and_inverse() {
        let base = fixture();
        let set_existing = XlsxMutation::SetCell { sheet_name: "Sheet1".into(), row: 1, col: 0, value: XlsxCellValue::Boolean(true) };
        let mut after = base.clone();
        apply_xlsx_mutation(&mut after, &set_existing);
        assert_eq!(after.workbook.sheets[0].cells[0].value, XlsxCellValue::Boolean(true));
        for inv in Mutation::inverse(&set_existing, &base) {
            apply_xlsx_mutation(&mut after, &inv);
        }
        assert_eq!(after, base);

        let set_new = XlsxMutation::SetCell { sheet_name: "Sheet1".into(), row: 2, col: 3, value: XlsxCellValue::InlineString("fresh".into()) };
        let mut after2 = base.clone();
        apply_xlsx_mutation(&mut after2, &set_new);
        assert!(after2.workbook.sheets[0].cells.iter().any(|c| c.row == 2 && c.col == 3 && c.value == XlsxCellValue::InlineString("fresh".into())));
        for inv in Mutation::inverse(&set_new, &base) {
            apply_xlsx_mutation(&mut after2, &inv);
        }
        assert_eq!(after2, base);

        let remove = XlsxMutation::RemoveCell { sheet_name: "Sheet1".into(), row: 1, col: 0 };
        let mut after3 = base.clone();
        apply_xlsx_mutation(&mut after3, &remove);
        assert!(after3.workbook.sheets[0].cells.is_empty());
        for inv in Mutation::inverse(&remove, &base) {
            apply_xlsx_mutation(&mut after3, &inv);
        }
        assert_eq!(after3, base);
    }

    #[test]
    fn shared_string_mutations_apply_and_inverse() {
        let base = fixture();
        let insert = XlsxMutation::InsertSharedString { value: "world".into() };
        let mut after = base.clone();
        apply_xlsx_mutation(&mut after, &insert);
        assert_eq!(after.workbook.shared_strings, vec!["hello".to_string(), "world".to_string()]);
        for inv in Mutation::inverse(&insert, &base) {
            apply_xlsx_mutation(&mut after, &inv);
        }
        assert_eq!(after, base);

        let set = XlsxMutation::SetSharedString { index: 0, value: "changed".into() };
        let mut after2 = base.clone();
        apply_xlsx_mutation(&mut after2, &set);
        assert_eq!(after2.workbook.shared_strings, vec!["changed".to_string()]);
        for inv in Mutation::inverse(&set, &base) {
            apply_xlsx_mutation(&mut after2, &inv);
        }
        assert_eq!(after2, base);

        let remove = XlsxMutation::RemoveSharedString { index: 0 };
        let mut after3 = base.clone();
        apply_xlsx_mutation(&mut after3, &remove);
        assert!(after3.workbook.shared_strings.is_empty());
        for inv in Mutation::inverse(&remove, &base) {
            apply_xlsx_mutation(&mut after3, &inv);
        }
        assert_eq!(after3, base);
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
    fn sweep_a() -> XlsxSnapshot {
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
        opc.relationships.insert(
            "xl/toModify.xml".into(),
            vec![OpcRelationship { id: "rId2".into(), rel_type: "http://example/toModify".into(), target: "worksheets/old.xml".into(), target_mode: OpcTargetMode::Internal }],
        );
        opc.relationships.insert(
            "xl/toRemove.xml".into(),
            vec![OpcRelationship { id: "rId8".into(), rel_type: "http://example/ownerToRemove".into(), target: "media/gone.png".into(), target_mode: OpcTargetMode::Internal }],
        );

        XlsxSnapshot::from_parts(
            opc,
            XlsxWorkbook {
                sheets: vec![
                    XlsxSheet {
                        name: "toModify".into(),
                        cells: vec![
                            XlsxCell { row: 1, col: 0, value: XlsxCellValue::Number(1.0) },
                            XlsxCell { row: 2, col: 0, value: XlsxCellValue::Boolean(false) },
                        ],
                    },
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

    fn sweep_b() -> XlsxSnapshot {
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
        opc.relationships.insert(
            "xl/toModify.xml".into(),
            vec![OpcRelationship { id: "rId2".into(), rel_type: "http://example/toModify".into(), target: "worksheets/new.xml".into(), target_mode: OpcTargetMode::Internal }],
        );
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

    //#region 🔖️MutationDiffLaw
    fn sample_mutations() -> Vec<XlsxMutation> {
        vec![
            XlsxMutation::NoMutation,
            XlsxMutation::SetSnapshot { snapshot: sweep_b() },
            XlsxMutation::InsertSheet { sheet: XlsxSheet { name: "x".into(), cells: vec![] } },
            // 🎯️ `RemoveSheet`/`RenameSheet` target `"Sheet2"`, the LAST sheet in `fixture()` --
            // same last-position caveat as the dedicated `remove_sheet_inverse_restores_removed_sheet`/
            // `rename_sheet_apply_and_inverse` tests document (`sheets` is name-keyed; a
            // mutation-level `InsertSheet`-based inverse always APPENDS, so exact Vec-position
            // restoration is only guaranteed when the target was already last).
            XlsxMutation::RemoveSheet { name: "Sheet2".into() },
            XlsxMutation::RenameSheet { name: "Sheet2".into(), new_name: "Renamed".into() },
            XlsxMutation::SetCell { sheet_name: "Sheet1".into(), row: 1, col: 0, value: XlsxCellValue::Boolean(true) },
            XlsxMutation::SetCell { sheet_name: "Sheet1".into(), row: 9, col: 9, value: XlsxCellValue::Number(42.0) },
            XlsxMutation::RemoveCell { sheet_name: "Sheet1".into(), row: 1, col: 0 },
            XlsxMutation::InsertSharedString { value: "z".into() },
            // 🎯️ `RemoveSharedString` targets index 0, the LAST-in-position entry `fixture()`
            // (which has only one shared string) has -- like docx's own `RemovePart` precedent
            // (see that artifact's `sample_mutations` doc comment), a name/key-keyed (here
            // index-keyed) collection's mutation-level inverse only restores exact ORIGINAL
            // Vec position when the removed item was already last; the same caveat applies here.
            XlsxMutation::RemoveSharedString { index: 0 },
            XlsxMutation::SetSharedString { index: 0, value: "y".into() },
        ]
    }

    #[test]
    fn mutation_diff_law() {
        for mutation in sample_mutations() {
            let base = fixture();
            let diff_direct = Mutation::diff(&mutation, &base);
            let applied_via_diff = MutationDiff::apply(&diff_direct, &base);

            let mut via_apply = base.clone();
            let diff_from_apply = apply_xlsx_mutation(&mut via_apply, &mutation);

            assert_eq!(applied_via_diff, via_apply, "mutation_diff_law: apply mismatch for {mutation:?}");
            assert_eq!(diff_direct, diff_from_apply, "mutation_diff_law: diff mismatch for {mutation:?}");
        }
    }
    //#endregion 🔖️MutationDiffLaw

    //#region 🔖️InverseLaw
    #[test]
    fn inverse_law() {
        for mutation in sample_mutations() {
            let base = fixture();

            let mut round_tripped = base.clone();
            apply_xlsx_mutation(&mut round_tripped, &mutation);
            for inverse_mutation in <XlsxMutation as Mutation<XlsxSnapshot>>::inverse(&mutation, &base) {
                apply_xlsx_mutation(&mut round_tripped, &inverse_mutation);
            }
            assert_eq!(round_tripped, base, "inverse_law (mutation-level) failed for {mutation:?}");

            let diff = Mutation::diff(&mutation, &base);
            let next = MutationDiff::apply(&diff, &base);
            let inverse_diff = DiffAlgebra::inverse(&diff, &base);
            let restored = MutationDiff::apply(&inverse_diff, &next);
            assert_eq!(restored, base, "inverse_law (diff-level) failed for {mutation:?}");
        }
    }
    //#endregion 🔖️InverseLaw

    //#region 🔖️AbsorbLaw
    fn assert_absorb_matches_sequential(base: &XlsxSnapshot, d1: &XlsxDiff, d2: &XlsxDiff) -> XlsxDiff {
        let sequential = MutationDiff::apply(d2, &MutationDiff::apply(d1, base));
        let mut absorbed = d1.clone();
        MutationDiff::absorb(&mut absorbed, d2.clone());
        assert_eq!(MutationDiff::apply(&absorbed, base), sequential, "absorb_law: apply(absorb(d1,d2), base) != sequential");
        absorbed
    }

    fn cells_diff<'a>(diff: &'a XlsxDiff, sheet_name: &str) -> &'a crate::artifacts::xlsx::schema::diff::XlsxCellsDiff {
        let sheets = diff.workbook.as_ref().expect("workbook diff present").sheets.as_ref().expect("sheets diff present");
        sheets.modified.iter().find(|m| m.key == sheet_name).expect("sheet modified").diff.cells.as_ref().expect("cells diff present")
    }

    #[test]
    fn absorb_law() {
        // Canonical: SetCell(1,0)+RemoveCell(1,0) on a fresh row -> annihilated add (mirrors
        // Insert+Remove-before): net effect is "never existed".
        {
            let base = fixture();
            let d1 = Mutation::diff(&XlsxMutation::SetCell { sheet_name: "Sheet2".into(), row: 5, col: 5, value: XlsxCellValue::Number(1.0) }, &base);
            let mid = MutationDiff::apply(&d1, &base);
            let d2 = Mutation::diff(&XlsxMutation::RemoveCell { sheet_name: "Sheet2".into(), row: 5, col: 5 }, &mid);
            let absorbed = assert_absorb_matches_sequential(&base, &d1, &d2);
            let triple = cells_diff(&absorbed, "Sheet2");
            assert!(triple.added.is_empty(), "the add must be annihilated by the later remove");
            assert!(triple.removed.is_empty(), "a never-based cell must not appear as a base removal either");
        }

        // Canonical: SetCell(5,5,f)+SetCell(5,6,g) on distinct fresh cells -> both survive
        // (mirrors Insert+Insert-same-index-both-survive: two independent adds never LWW-clobber
        // each other).
        {
            let base = fixture();
            let d1 = Mutation::diff(&XlsxMutation::SetCell { sheet_name: "Sheet2".into(), row: 5, col: 5, value: XlsxCellValue::InlineString("f".into()) }, &base);
            let mid = MutationDiff::apply(&d1, &base);
            let d2 = Mutation::diff(&XlsxMutation::SetCell { sheet_name: "Sheet2".into(), row: 5, col: 6, value: XlsxCellValue::InlineString("g".into()) }, &mid);
            let absorbed = assert_absorb_matches_sequential(&base, &d1, &d2);
            let triple = cells_diff(&absorbed, "Sheet2");
            assert_eq!(triple.added.len(), 2, "both cell adds must survive absorb");
        }

        // Canonical: SetCell(insert)+SetCell(same coord, patch) -> patch into the added payload.
        {
            let base = fixture();
            let d1 = Mutation::diff(&XlsxMutation::SetCell { sheet_name: "Sheet2".into(), row: 5, col: 5, value: XlsxCellValue::InlineString("f".into()) }, &base);
            let mid = MutationDiff::apply(&d1, &base);
            let d2 = Mutation::diff(&XlsxMutation::SetCell { sheet_name: "Sheet2".into(), row: 5, col: 5, value: XlsxCellValue::InlineString("patched".into()) }, &mid);
            let absorbed = assert_absorb_matches_sequential(&base, &d1, &d2);
            let triple = cells_diff(&absorbed, "Sheet2");
            assert!(triple.modified.is_empty(), "patch-into-added must not surface as a separate modified entry");
            assert_eq!(triple.added.len(), 1);
            assert_eq!(triple.added[0].value, XlsxCellValue::InlineString("patched".into()));
        }

        // Canonical: Modify+Remove -> the modify is annihilated by the later remove.
        {
            let base = fixture();
            let d1 = Mutation::diff(&XlsxMutation::SetCell { sheet_name: "Sheet1".into(), row: 1, col: 0, value: XlsxCellValue::Boolean(true) }, &base);
            let mid = MutationDiff::apply(&d1, &base);
            let d2 = Mutation::diff(&XlsxMutation::RemoveCell { sheet_name: "Sheet1".into(), row: 1, col: 0 }, &mid);
            let absorbed = assert_absorb_matches_sequential(&base, &d1, &d2);
            let triple = cells_diff(&absorbed, "Sheet1");
            assert!(triple.modified.is_empty(), "modify of a since-removed cell must not survive absorb");
            assert_eq!(triple.removed, vec![(1u32, 0u32)]);
        }

        // Associativity over a triple.
        {
            let base = fixture();
            let d1 = Mutation::diff(&XlsxMutation::SetCell { sheet_name: "Sheet2".into(), row: 5, col: 5, value: XlsxCellValue::Number(1.0) }, &base);
            let mid1 = MutationDiff::apply(&d1, &base);
            let d2 = Mutation::diff(&XlsxMutation::SetCell { sheet_name: "Sheet2".into(), row: 6, col: 6, value: XlsxCellValue::Number(2.0) }, &mid1);
            let mid2 = MutationDiff::apply(&d2, &mid1);
            let d3 = Mutation::diff(&XlsxMutation::RemoveCell { sheet_name: "Sheet2".into(), row: 5, col: 5 }, &mid2);
            let sequential = MutationDiff::apply(&d3, &mid2);

            let mut left = d1.clone();
            MutationDiff::absorb(&mut left, d2.clone());
            MutationDiff::absorb(&mut left, d3.clone());

            let mut d2_then_d3 = d2.clone();
            MutationDiff::absorb(&mut d2_then_d3, d3.clone());
            let mut right = d1.clone();
            MutationDiff::absorb(&mut right, d2_then_d3);

            assert_eq!(MutationDiff::apply(&left, &base), sequential, "absorb associativity (left) failed");
            assert_eq!(MutationDiff::apply(&right, &base), sequential, "absorb associativity (right) failed");
        }
    }
    //#endregion 🔖️AbsorbLaw

    //#region 🔖️BetweenRoundtripLaw
    #[test]
    fn between_roundtrip_law() {
        let a = sweep_a();
        let b = sweep_b();
        assert_eq!(MutationDiff::apply(&<XlsxDiff as DiffAlgebra<XlsxSnapshot>>::between(&a, &b), &a), b);
        assert_eq!(MutationDiff::apply(&<XlsxDiff as DiffAlgebra<XlsxSnapshot>>::between(&b, &a), &b), a);

        let sample = fixture();
        assert_eq!(MutationDiff::apply(&<XlsxDiff as DiffAlgebra<XlsxSnapshot>>::between(&sample, &sample), &sample), sample);

        // "Real" fixture leg: a realistic multi-sheet workbook diffed against a mutated variant.
        let real = crate::artifacts::xlsx::engine::build_minimal_xlsx(XlsxWorkbook {
            sheets: vec![XlsxSheet { name: "Data".into(), cells: vec![XlsxCell { row: 1, col: 0, value: XlsxCellValue::SharedString(0) }] }],
            shared_strings: vec!["Chapter One".into()],
        });
        let mut mutated = real.clone();
        apply_xlsx_mutation(&mut mutated, &XlsxMutation::SetSharedString { index: 0, value: "Chapter Two".into() });
        assert_ne!(real, mutated);
        assert_eq!(MutationDiff::apply(&<XlsxDiff as DiffAlgebra<XlsxSnapshot>>::between(&real, &mutated), &real), mutated);
        assert_eq!(MutationDiff::apply(&<XlsxDiff as DiffAlgebra<XlsxSnapshot>>::between(&mutated, &real), &mutated), real);
    }
    //#endregion 🔖️BetweenRoundtripLaw

    //#region 🔖️CodecRetentionLaw
    #[test]
    fn codec_retention_law() {
        let snap = crate::artifacts::xlsx::engine::build_minimal_xlsx(XlsxWorkbook {
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
    #[test]
    fn field_sweep() {
        let a = sweep_a();
        let b = sweep_b();

        let diff_ab = <XlsxDiff as DiffAlgebra<XlsxSnapshot>>::between(&a, &b);
        assert_eq!(MutationDiff::apply(&diff_ab, &a), b);
        let diff_ba = <XlsxDiff as DiffAlgebra<XlsxSnapshot>>::between(&b, &a);
        assert_eq!(MutationDiff::apply(&diff_ba, &b), a);
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
    #[test]
    fn op_text_binary_roundtrip_law() {
        let mut snapshot_with_opc = sweep_b();
        snapshot_with_opc.opc.relationships.get_mut("xl/toModify.xml").unwrap()[0].target_mode = OpcTargetMode::External;

        let mutations = vec![
            XlsxMutation::NoMutation,
            XlsxMutation::SetSnapshot { snapshot: snapshot_with_opc },
            XlsxMutation::InsertSheet { sheet: XlsxSheet { name: "New, odd: [name]".into(), cells: vec![XlsxCell { row: 1, col: 2, value: XlsxCellValue::Empty }] } },
            XlsxMutation::RemoveSheet { name: "Sheet2".into() },
            XlsxMutation::RenameSheet { name: "Sheet2".into(), new_name: "Renamed".into() },
            XlsxMutation::SetCell { sheet_name: "Sheet1".into(), row: 1, col: 0, value: XlsxCellValue::Number(-2.5) },
            XlsxMutation::SetCell { sheet_name: "Sheet1".into(), row: 2, col: 0, value: XlsxCellValue::SharedString(3) },
            XlsxMutation::SetCell { sheet_name: "Sheet1".into(), row: 3, col: 0, value: XlsxCellValue::Boolean(false) },
            XlsxMutation::SetCell { sheet_name: "Sheet1".into(), row: 4, col: 0, value: XlsxCellValue::InlineString("has, weird: [chars]".into()) },
            XlsxMutation::SetCell {
                sheet_name: "Sheet1".into(),
                row: 5,
                col: 0,
                value: XlsxCellValue::Formula { expr: "SUM(A1:A4)".into(), cached: Some(Box::new(XlsxCellValue::Number(1.5))) },
            },
            XlsxMutation::SetCell { sheet_name: "Sheet1".into(), row: 6, col: 0, value: XlsxCellValue::Formula { expr: "NA()".into(), cached: None } },
            XlsxMutation::RemoveCell { sheet_name: "Sheet1".into(), row: 1, col: 0 },
            XlsxMutation::InsertSharedString { value: "z".into() },
            XlsxMutation::RemoveSharedString { index: 0 },
            XlsxMutation::SetSharedString { index: 0, value: "y".into() },
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
}
//#endregion 🧪️Tests
