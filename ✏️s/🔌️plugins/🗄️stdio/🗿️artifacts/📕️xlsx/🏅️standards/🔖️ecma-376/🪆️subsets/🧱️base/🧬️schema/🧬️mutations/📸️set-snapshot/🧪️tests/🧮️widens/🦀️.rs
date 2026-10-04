//! 🧪️ `set-snapshot` fixture — `🧮️widens`.
//!
//! The snapshot's authority is one logical XML document per XML-bearing OPC part, so widening one
//! formula is a sparse edit of exactly ONE part — the worksheet — while the workbook part, the shared
//! strings, the relationships and the lossless OPC lane stay absent from the delta. The projected
//! workbook shows the widened formula at its unchanged `(row, col)` identity, and the empty cell below
//! it survives untouched.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). It is never hand edited: `zzz_write_committed_quintet`
//! below writes it from `build_minimal_xlsx` of the two declared workbooks and this leaf's own
//! canonical diff (`cargo test -p semio-s-artifact-stdio-xlsx --lib -- --ignored zzz_write_committed_quintet`),
//! and `committed_snapshots_are_the_declared_workbooks` holds it to that. The `.op.semio`/`.spr.semio`/
//! `.dsl.semio`/`.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and
//! are asserted by the shared codec-matrix harness, not here.

use crate::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_xlsx;
use crate::standards::v_ecma_376::subsets::base::schema::diff::XlsxDiff;
use crate::standards::v_ecma_376::subsets::base::schema::mutations::{apply_xlsx_mutation, cell_address::xlsx_cell_address, set_snapshot::SetSnapshot, XlsxMutation};
use crate::standards::v_ecma_376::subsets::base::schema::snapshot::{XlsxCell, XlsxCellValue, XlsxSheet, XlsxSnapshot, XlsxWorkbook};

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📸️set-snapshot/🧮️widens/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📸️set-snapshot/🧮️widens/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📸️set-snapshot/🧮️widens/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📸️set-snapshot/🧮️widens/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📸️set-snapshot/🧮️widens/🎯️outcome/🔣️.json");

/// 🧮️ The declared workbook: a total formula at `B4` over `expr`, and an empty cell at `B5` below it.
fn declared_workbook(expr: &str) -> XlsxWorkbook {
    XlsxWorkbook {
        sheets: vec![XlsxSheet { name: "Sheet1".into(), cells: vec![XlsxCell { row: 4, col: 1, value: XlsxCellValue::Formula { expr: expr.into(), cached: None } }, XlsxCell { row: 5, col: 1, value: XlsxCellValue::Empty }] }],
        shared_strings: vec![],
    }
}

fn before() -> XlsxSnapshot {
    semio_framework_pack_json::from_json_str(BEFORE, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before snapshot decodes")
}
fn expected_after() -> XlsxSnapshot {
    semio_framework_pack_json::from_json_str(AFTER, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("after snapshot decodes")
}
fn mutation() -> XlsxMutation {
    semio_framework_pack_json::from_json_str(MUTATION, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation decodes")
}
fn workbook(snapshot: &XlsxSnapshot) -> XlsxWorkbook {
    snapshot.project_workbook().expect("the canonical XML parts project a workbook")
}
fn worksheet_part(snapshot: &XlsxSnapshot) -> String {
    xlsx_cell_address(snapshot, "Sheet1", 4, 1).expect("the total cell has an address").part_path
}

/// 🧾️ The committed snapshots are exactly `build_minimal_xlsx` of the declared workbooks — the quintet is the
/// generator's output, not a hand-edited approximation.
#[semio_framework_async_macros::async_test]
async fn committed_snapshots_are_the_declared_workbooks() {
    assert_eq!(before(), build_minimal_xlsx(declared_workbook("SUM(B1:B2)")), "set-snapshot/widens-the-total-formula-to-a-third-row: committed before-snapshot drifted from its declared workbook");
    assert_eq!(expected_after(), build_minimal_xlsx(declared_workbook("SUM(B1:B3)")), "set-snapshot/widens-the-total-formula-to-a-third-row: committed after-snapshot drifted from its declared workbook");
    assert_eq!(mutation(), XlsxMutation::SetSnapshot(SetSnapshot { snapshot: expected_after() }), "set-snapshot/widens-the-total-formula-to-a-third-row: the committed mutation carries the committed after-snapshot");
}

/// ▶️ `set-snapshot` carries the committed `before` XlsxSnapshot to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    let outcome = apply_xlsx_mutation(&mut snapshot, &mutation());
    assert!(outcome.messages().is_empty(), "set-snapshot/widens-the-total-formula-to-a-third-row: set-snapshot raised diagnostics it should not have");
    assert_eq!(snapshot, expected_after(), "set-snapshot/widens-the-total-formula-to-a-third-row: applied state differs from committed after-snapshot");
    let projected = workbook(&snapshot);
    let cells = &projected.sheets[0].cells;
    assert!(
        matches!(&cells[0].value, XlsxCellValue::Formula { expr, cached } if expr == "SUM(B1:B3)" && cached.is_none()),
        "set-snapshot/widens-the-total-formula-to-a-third-row: the total cell must carry the widened formula and still have no cached value"
    );
    assert_eq!((cells[0].row, cells[0].col), (4, 1), "set-snapshot/widens-the-total-formula-to-a-third-row: a cell's (row, col) pair is its identity and is never rewritten by a value edit");
    assert_eq!(cells[1], workbook(&before()).sheets[0].cells[1], "set-snapshot/widens-the-total-formula-to-a-third-row: the empty cell below is identical on both sides and must survive untouched");
    assert!(projected.shared_strings.is_empty(), "set-snapshot/widens-the-total-formula-to-a-third-row: the shared-string table is untouched — this workbook has none");
    assert_eq!(snapshot.opc, before().opc, "set-snapshot/widens-the-total-formula-to-a-third-row: the OPC package is identical on both sides");
}

/// ↩️ `set-snapshot`'s inverse is a single `SetSnapshot` carrying the pre-state XlsxSnapshot back, so
/// forward-then-undo restores `before` byte for byte.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <XlsxMutation as protocol::Mutation<XlsxSnapshot>>::inverse(&mutation, &base).expect("valid retained mutation inverse fixture");
    assert_eq!(inverse.len(), 1, "set-snapshot/widens-the-total-formula-to-a-third-row: undoing a whole-snapshot replacement is exactly one step");
    assert!(matches!(inverse[0], XlsxMutation::SetSnapshot(_)), "set-snapshot/widens-the-total-formula-to-a-third-row: the undo step must itself be a SetSnapshot carrying the pre-state");
    let mut snapshot = base.clone();
    apply_xlsx_mutation(&mut snapshot, &mutation);
    for step in &inverse {
        apply_xlsx_mutation(&mut snapshot, step);
    }
    assert_eq!(snapshot, base, "set-snapshot/widens-the-total-formula-to-a-third-row: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed XlsxSnapshot snapshots and this leaf's committed mutation payload are already
/// canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: XlsxSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "set-snapshot/widens-the-total-formula-to-a-third-row: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&mutation())).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "set-snapshot/widens-the-total-formula-to-a-third-row: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome — status AND every diagnostic this leaf's own diff builder raises for
/// this payload — matches what the mutation actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let declared: Vec<(String, String)> =
        outcome.get("messages").and_then(serde_json::Value::as_array).map(|rows| rows.iter().map(|row| (row["level"].as_str().unwrap_or_default().to_string(), row["code"].as_str().unwrap_or_default().to_string())).collect()).unwrap_or_default();
    let raised = <XlsxMutation as protocol::Mutation<XlsxSnapshot>>::diff(&mutation(), &before());
    let produced: Vec<(String, String)> = raised
        .messages()
        .iter()
        .map(|message| {
            let level = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&message.level)).expect("severity encodes");
            (level.as_str().unwrap_or_default().to_string(), message.code.0.clone())
        })
        .collect();
    assert_eq!(produced, declared, "set-snapshot/widens-the-total-formula-to-a-third-row: raised diagnostics differ from the committed 🎯️outcome messages");
    let mut snapshot = before();
    apply_xlsx_mutation(&mut snapshot, &mutation());
    match status {
        "applied" => assert_ne!(snapshot, before(), "set-snapshot/widens-the-total-formula-to-a-third-row: declared applied but the snapshot came back unchanged"),
        "rejected" => assert_eq!(snapshot, before(), "set-snapshot/widens-the-total-formula-to-a-third-row: a rejected mutation must leave the snapshot untouched"),
        other => panic!("set-snapshot/widens-the-total-formula-to-a-third-row: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta this leaf produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: `set-snapshot` has NO whole-snapshot replacement slot
/// in XlsxDiff, so the delta must name only the part that actually differs.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let raised = <XlsxMutation as protocol::Mutation<XlsxSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(raised.diff())).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "set-snapshot/widens-the-total-formula-to-a-third-row: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert!(raised.diff().opc.is_none(), "set-snapshot/widens-the-total-formula-to-a-third-row: a formula edit must never reach into the lossless OPC lane");
    let parts = raised.diff().xml_parts.as_ref().expect("set-snapshot/widens-the-total-formula-to-a-third-row: the XML parts diff must be present");
    assert!(parts.removed.is_empty() && parts.added.is_empty(), "set-snapshot/widens-the-total-formula-to-a-third-row: the worksheet is patched in place — no part is added or removed");
    assert_eq!(
        parts.modified.iter().map(|part| part.key.clone()).collect::<Vec<_>>(),
        vec![worksheet_part(&base)],
        "set-snapshot/widens-the-total-formula-to-a-third-row: only the worksheet part holding the total cell is patched, keyed by its OPC path"
    );
}

/// 🔣️ The committed diff is itself canonical and decodes to XlsxDiff.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: XlsxDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&decoded)).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "set-snapshot/widens-the-total-formula-to-a-third-row: committed diff JSON is not canonical");
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(DIFF).expect("diff reparses").pointer("/xmlParts/modified/0/key"),
        Some(&serde_json::Value::String(worksheet_part(&before()))),
        "set-snapshot/widens-the-total-formula-to-a-third-row: the committed diff is keyed by the worksheet's OPC part path, never by a position in the part list"
    );
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is
/// a complete description of what this `set-snapshot` changed, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: XlsxDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let produced = <XlsxDiff as protocol::MutationDiff<XlsxSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "set-snapshot/widens-the-total-formula-to-a-third-row: committed diff did not carry before to after");
}

/// 🖊️ The ONLY way the committed quintet is ever refreshed: `build_minimal_xlsx` of the declared workbooks, the
/// `SetSnapshot` carrying the after-snapshot, and this leaf's own canonical diff — never a hand edit. Run it
/// deliberately after a codec or snapshot-shape change, then re-run the laws above.
#[semio_framework_async_macros::async_test]
#[ignore]
async fn zzz_write_committed_quintet() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧫️fixtures/🧬️mutations/📸️set-snapshot/🧮️widens");
    let pretty = |compact: String| serde_json::to_string_pretty(&serde_json::from_str::<serde_json::Value>(&compact).expect("canonical JSON")).expect("pretty JSON") + "\n";
    let before = build_minimal_xlsx(declared_workbook("SUM(B1:B2)"));
    let after = build_minimal_xlsx(declared_workbook("SUM(B1:B3)"));
    let mutation = XlsxMutation::SetSnapshot(SetSnapshot { snapshot: after.clone() });
    let raised = <XlsxMutation as protocol::Mutation<XlsxSnapshot>>::diff(&mutation, &before);
    for (path, text) in [
        ("📸️snapshot/⬅️before/🔣️.json", pretty(semio_framework_pack_json::to_json_string(&before))),
        ("📸️snapshot/➡️after/🔣️.json", pretty(semio_framework_pack_json::to_json_string(&after))),
        ("🦠️mutation/🔣️.json", pretty(semio_framework_pack_json::to_json_string(&mutation))),
        ("🔺️diff/🔣️.json", pretty(semio_framework_pack_json::to_json_string(raised.diff()))),
        ("🎯️outcome/🔣️.json", pretty(r#"{"status":"applied"}"#.to_string())),
    ] {
        std::fs::write(root.join(path), text).unwrap_or_else(|error| panic!("write {path}: {error}"));
    }
}
