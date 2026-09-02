//! 🧪️ `set-snapshot` fixture — `widens-the-total-formula-to-a-third-row`.
//!
//! Worksheets are NAME-keyed and cells are keyed by their `(row, col)` identity pair, so the
//! committed delta nests a `NamedTripleDiff` keyed by sheet name around a second one keyed by
//! a two-element `[row, col]` array. `XlsxCellValue` is a weak value union, so a formula edit
//! replaces the cell's whole value rather than sub-diffing `expr` — and the untouched empty
//! cell beside it must not appear at all.
//! Encoding note this fixture is deliberately built around: `XlsxCellValue` is internally
//! tagged, so only its struct-variant `Formula` and unit-variant `Empty` arms are
//! serde-serializable at all — `Number(f64)`/`SharedString(usize)`/`InlineString(String)`/
//! `Boolean(bool)` are internally tagged NEWTYPE variants over non-map payloads, which serde
//! refuses to serialize. The fixture therefore exercises the two arms that genuinely encode
//! rather than committing JSON the type cannot produce.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`), every value of which was transcribed from this
//! leaf's own `🔺️diff/🦀️.rs` oracle. The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::schema::diff::XlsxDiff;
use crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::schema::mutations::{apply_xlsx_mutation, XlsxMutation};
use crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::schema::snapshot::XlsxSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> XlsxSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> XlsxSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> XlsxMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ `set-snapshot` carries the committed `before` XlsxSnapshot to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    let outcome = apply_xlsx_mutation(&mut snapshot, &mutation());
    assert!(outcome.messages().is_empty(), "set-snapshot/widens-the-total-formula-to-a-third-row: set-snapshot raised diagnostics it should not have");
    assert_eq!(snapshot, expected_after(), "set-snapshot/widens-the-total-formula-to-a-third-row: applied state differs from committed after-snapshot");
    let cells = &snapshot.workbook.sheets[0].cells;
    assert!(
        matches!(&cells[0].value, crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::schema::snapshot::XlsxCellValue::Formula { expr, cached } if expr == "SUM(B1:B3)" && cached.is_none()),
        "set-snapshot/widens-the-total-formula-to-a-third-row: the total cell must carry the widened formula and still have no cached value"
    );
    assert_eq!((cells[0].row, cells[0].col), (4, 1), "set-snapshot/widens-the-total-formula-to-a-third-row: a cell's (row, col) pair is its identity and is never rewritten by a value edit");
    assert_eq!(cells[1], before().workbook.sheets[0].cells[1], "set-snapshot/widens-the-total-formula-to-a-third-row: the empty cell below is identical on both sides and must survive untouched");
    assert!(snapshot.workbook.shared_strings.is_empty(), "set-snapshot/widens-the-total-formula-to-a-third-row: the shared-string table is untouched — this workbook has none");
    assert_eq!(snapshot.opc, before().opc, "set-snapshot/widens-the-total-formula-to-a-third-row: the OPC package is identical on both sides");
}

/// ↩️ `set-snapshot`'s inverse is a single `SetSnapshot` carrying the pre-state XlsxSnapshot back, so
/// forward-then-undo restores `before` byte for byte.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <XlsxMutation as protocol::Mutation<XlsxSnapshot>>::inverse(&mutation, &base);
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
        let decoded: XlsxSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "set-snapshot/widens-the-total-formula-to-a-third-row: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
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
            let level = serde_json::to_value(message.level).expect("severity encodes");
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
/// in XlsxDiff, so the delta must name only the fields that actually differ.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let raised = <XlsxMutation as protocol::Mutation<XlsxSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(raised.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "set-snapshot/widens-the-total-formula-to-a-third-row: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert!(raised.diff().opc.is_none(), "set-snapshot/widens-the-total-formula-to-a-third-row: a workbook-level edit must never reach into the lossless OPC lane");
    let workbook = raised.diff().workbook.as_ref().expect("set-snapshot/widens-the-total-formula-to-a-third-row: the workbook diff must be present");
    assert!(workbook.shared_strings.is_none(), "set-snapshot/widens-the-total-formula-to-a-third-row: the SST is equal on both sides and must stay absent");
    let sheets = workbook.sheets.as_ref().expect("set-snapshot/widens-the-total-formula-to-a-third-row: the sheets triple must be present");
    assert!(sheets.removed.is_empty() && sheets.added.is_empty(), "set-snapshot/widens-the-total-formula-to-a-third-row: the sheet is patched in place — a rename would be a remove+add pair, which this payload is not");
    assert_eq!(sheets.modified[0].key, "Sheet1", "set-snapshot/widens-the-total-formula-to-a-third-row: XlsxSheetsDiff is keyed by sheet NAME");
    let cells = sheets.modified[0].diff.cells.as_ref().expect("set-snapshot/widens-the-total-formula-to-a-third-row: the cells triple must be present");
    assert_eq!(cells.modified.len(), 1, "set-snapshot/widens-the-total-formula-to-a-third-row: only the total cell is patched");
    assert_eq!(cells.modified[0].key, (4, 1), "set-snapshot/widens-the-total-formula-to-a-third-row: XlsxCellsDiff is keyed by the (row, col) identity pair, not by a position in the sparse list");
}

/// 🔣️ The committed diff is itself canonical and decodes to XlsxDiff.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: XlsxDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "set-snapshot/widens-the-total-formula-to-a-third-row: committed diff JSON is not canonical");
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(DIFF).expect("diff reparses").pointer("/workbook/sheets/modified/0/diff/cells/modified/0/key"),
        Some(&serde_json::json!([4, 1])),
        "set-snapshot/widens-the-total-formula-to-a-third-row: the (u32, u32) cell key encodes as a two-element JSON array — anything else would mean the committed diff was keyed by list position instead of by cell identity"
    );
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is
/// a complete description of what this `set-snapshot` changed, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: XlsxDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <XlsxDiff as protocol::MutationDiff<XlsxSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "set-snapshot/widens-the-total-formula-to-a-third-row: committed diff did not carry before to after");
}
