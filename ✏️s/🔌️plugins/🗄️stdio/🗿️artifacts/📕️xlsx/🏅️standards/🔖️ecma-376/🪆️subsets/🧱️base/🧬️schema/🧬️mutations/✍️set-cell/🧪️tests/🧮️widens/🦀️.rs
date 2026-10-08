//! 🧪️ `set-cell` fixture — `🧮️widens`.
//!
//! The snapshot's authority is one logical XML document per XML-bearing OPC part, so widening one formula is a sparse edit of exactly ONE
//! part — the worksheet — while the workbook part, the shared strings, the relationships and the lossless OPC lane stay absent from the
//! delta. The projected workbook shows the widened formula at its unchanged `(row, col)` identity, and the empty cell below it survives
//! untouched.
//!
//! The committed before/after snapshots are never hand edited: `committed_snapshots_are_the_declared_workbooks` holds them to
//! `build_minimal_xlsx` of the two declared workbooks.

use crate::standards::v_ecma_376::subsets::base::schema::construction::build_minimal_xlsx;
use crate::standards::v_ecma_376::subsets::base::schema::mutations::{apply_xlsx_mutation, cell_address::xlsx_cell_address, set_cell::SetCell, XlsxMutation};
use crate::standards::v_ecma_376::subsets::base::schema::snapshot::{XlsxCell, XlsxCellValue, XlsxSheet, XlsxSnapshot, XlsxWorkbook};

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✍️set-cell/🧮️widens/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✍️set-cell/🧮️widens/📸️snapshot/➡️after/🔣️.json");

/// 🧮️ The declared workbook: a total formula at `B4` over `expr`, and an empty cell at `B5` below it.
fn declared_workbook(expr: &str) -> XlsxWorkbook {
    XlsxWorkbook {
        sheets: vec![XlsxSheet { name: "Sheet1".into(), cells: vec![XlsxCell { row: 4, col: 1, value: XlsxCellValue::Formula { expr: expr.into(), cached: None } }, XlsxCell { row: 5, col: 1, value: XlsxCellValue::Empty }] }],
        shared_strings: vec![],
    }
}

fn snapshot_of(text: &str) -> XlsxSnapshot {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes")
}

fn mutation(base: &XlsxSnapshot) -> XlsxMutation {
    XlsxMutation::SetCell(SetCell { address: xlsx_cell_address(base, "Sheet1", 4, 1).expect("the total cell has an address"), value: XlsxCellValue::Formula { expr: "SUM(B1:B3)".into(), cached: None } })
}

/// 🧾️ The committed snapshots are exactly `build_minimal_xlsx` of the declared workbooks.
#[semio_framework_async_macros::async_test]
async fn committed_snapshots_are_the_declared_workbooks() {
    assert_eq!(snapshot_of(BEFORE), build_minimal_xlsx(declared_workbook("SUM(B1:B2)")), "set-cell/widens: committed before-snapshot drifted from its declared workbook");
    assert_eq!(snapshot_of(AFTER), build_minimal_xlsx(declared_workbook("SUM(B1:B3)")), "set-cell/widens: committed after-snapshot drifted from its declared workbook");
}

/// ▶️ `set-cell` carries the committed `before` snapshot to exactly the committed `after`, leaving every other cell and part alone.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = snapshot_of(BEFORE);
    let mut snapshot = base.clone();
    let outcome = apply_xlsx_mutation(&mut snapshot, &mutation(&base));
    assert!(outcome.messages().is_empty(), "set-cell/widens: raised diagnostics it should not have");
    assert_eq!(snapshot, snapshot_of(AFTER), "set-cell/widens: applied state differs from committed after-snapshot");
    let projected = snapshot.project_workbook().expect("the canonical XML parts project a workbook");
    let cells = &projected.sheets[0].cells;
    assert!(matches!(&cells[0].value, XlsxCellValue::Formula { expr, cached } if expr == "SUM(B1:B3)" && cached.is_none()), "set-cell/widens: the total cell must carry the widened formula and still have no cached value");
    assert_eq!((cells[0].row, cells[0].col), (4, 1), "set-cell/widens: a cell's (row, col) pair is its identity and is never rewritten by a value edit");
    assert_eq!(cells[1], base.project_workbook().expect("projects").sheets[0].cells[1], "set-cell/widens: the empty cell below is identical on both sides and must survive untouched");
    assert_eq!(snapshot.opc, base.opc, "set-cell/widens: the OPC package is identical on both sides");
}

/// 🔺️ The sparse delta names only the worksheet part that holds the total cell, never the OPC lane and never a position in the part list.
#[semio_framework_async_macros::async_test]
async fn produces_a_diff_of_the_worksheet_part_only() {
    let base = snapshot_of(BEFORE);
    let raised = <XlsxMutation as protocol::Mutation<XlsxSnapshot>>::diff(&mutation(&base), &base);
    assert!(raised.diff().opc.is_none(), "set-cell/widens: a formula edit must never reach into the lossless OPC lane");
    let parts = raised.diff().xml_parts.as_ref().expect("set-cell/widens: the XML parts diff must be present");
    assert!(parts.removed.is_empty() && parts.added.is_empty(), "set-cell/widens: the worksheet is patched in place — no part is added or removed");
    assert_eq!(parts.modified.iter().map(|part| part.key.clone()).collect::<Vec<_>>(), vec![xlsx_cell_address(&base, "Sheet1", 4, 1).expect("the total cell has an address").part_path], "set-cell/widens: only the worksheet part is patched, keyed by its OPC path");
}

/// ↩️ The concrete inverse is one `set-cell` restoring the widened-from formula, and the inverse sum law holds for the edit.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before_and_satisfies_the_sum_law() {
    let base = snapshot_of(BEFORE);
    let mutation = mutation(&base);
    let inverse = <XlsxMutation as protocol::Mutation<XlsxSnapshot>>::inverse(&mutation, &base).expect("valid retained mutation inverse fixture");
    assert!(matches!(inverse.as_slice(), [XlsxMutation::SetCell(_)]), "set-cell/widens: undoing a cell edit is exactly one set-cell");
    let mut snapshot = base.clone();
    apply_xlsx_mutation(&mut snapshot, &mutation);
    for step in &inverse {
        apply_xlsx_mutation(&mut snapshot, step);
    }
    assert_eq!(snapshot, base, "set-cell/widens: inverse did not restore the before-snapshot");
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
}
