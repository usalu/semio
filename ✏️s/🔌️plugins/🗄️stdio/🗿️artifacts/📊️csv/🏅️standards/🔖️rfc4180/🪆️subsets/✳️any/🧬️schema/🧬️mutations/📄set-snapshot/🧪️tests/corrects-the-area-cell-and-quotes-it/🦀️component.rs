//! 🧪️ `set-snapshot` fixture — `corrects-the-area-cell-and-quotes-it`.
//!
//! `CsvDiff::between` only reaches for a remove+add pair when a record's FIELD COUNT
//! changes; both records keep two fields here, so the edited data row becomes a positional
//! `CsvRecordDiff` whose `fields` vector carries `null` for the untouched `city` cell and a
//! `CsvFieldDiff` for the `area` cell. RFC 4180 treats quoting as optional and this schema
//! retains it losslessly, so flipping `quoted` is a real second sub-field of that one cell's
//! patch — not a rendering detail.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`), every value of which was transcribed from this
//! leaf's own `🔺️diff/🦀️component.rs` oracle. The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::csv::standards::v_rfc4180::subsets::any::schema::diff::CsvDiff;
use crate::artifacts::csv::standards::v_rfc4180::subsets::any::schema::mutations::{apply_csv_mutation, CsvMutation};
use crate::artifacts::csv::standards::v_rfc4180::subsets::any::schema::snapshot::CsvSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> CsvSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> CsvSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> CsvMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ `set-snapshot` carries the committed `before` CsvSnapshot to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    let outcome = apply_csv_mutation(&mut snapshot, &mutation());
    assert!(outcome.messages().is_empty(), "set-snapshot/corrects-the-area-cell-and-quotes-it: set-snapshot raised diagnostics it should not have");
    assert_eq!(snapshot, expected_after(), "set-snapshot/corrects-the-area-cell-and-quotes-it: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.records[1].fields[1].value, "204.1", "set-snapshot/corrects-the-area-cell-and-quotes-it: the area cell must land on 204.1");
    assert!(snapshot.records[1].fields[1].quoted, "set-snapshot/corrects-the-area-cell-and-quotes-it: the area cell must come back marked as quoted");
    assert_eq!(snapshot.records[0], before().records[0], "set-snapshot/corrects-the-area-cell-and-quotes-it: the header record is identical on both sides and must survive untouched");
    assert!(snapshot.has_header, "set-snapshot/corrects-the-area-cell-and-quotes-it: hasHeader is unchanged by this payload");
}

/// ↩️ `set-snapshot`'s inverse is a single `SetSnapshot` carrying the pre-state CsvSnapshot back, so
/// forward-then-undo restores `before` byte for byte.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <CsvMutation as protocol::Mutation<CsvSnapshot>>::inverse(&mutation, &base);
    assert_eq!(inverse.len(), 1, "set-snapshot/corrects-the-area-cell-and-quotes-it: undoing a whole-snapshot replacement is exactly one step");
    assert!(matches!(inverse[0], CsvMutation::SetSnapshot { .. }), "set-snapshot/corrects-the-area-cell-and-quotes-it: the undo step must itself be a SetSnapshot carrying the pre-state");
    let mut snapshot = base.clone();
    apply_csv_mutation(&mut snapshot, &mutation);
    for step in &inverse {
        apply_csv_mutation(&mut snapshot, step);
    }
    assert_eq!(snapshot, base, "set-snapshot/corrects-the-area-cell-and-quotes-it: inverse did not restore the before-snapshot");
    assert_eq!(snapshot.records[1].fields[1].value, "204", "set-snapshot/corrects-the-area-cell-and-quotes-it: the undo must restore the unrounded area value");
    assert!(!snapshot.records[1].fields[1].quoted, "set-snapshot/corrects-the-area-cell-and-quotes-it: the undo must also clear the quoting flag again");
}

/// 🔣️ Both committed CsvSnapshot snapshots and this leaf's committed mutation payload are already
/// canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: CsvSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "set-snapshot/corrects-the-area-cell-and-quotes-it: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "set-snapshot/corrects-the-area-cell-and-quotes-it: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome — status AND every diagnostic this leaf's own diff builder raises for
/// this payload — matches what the mutation actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let declared: Vec<(String, String)> =
        outcome.get("messages").and_then(serde_json::Value::as_array).map(|rows| rows.iter().map(|row| (row["level"].as_str().unwrap_or_default().to_string(), row["code"].as_str().unwrap_or_default().to_string())).collect()).unwrap_or_default();
    let raised = <CsvMutation as protocol::Mutation<CsvSnapshot>>::diff(&mutation(), &before());
    let produced: Vec<(String, String)> = raised
        .messages()
        .iter()
        .map(|message| {
            let level = serde_json::to_value(message.level).expect("severity encodes");
            (level.as_str().unwrap_or_default().to_string(), message.code.0.clone())
        })
        .collect();
    assert_eq!(produced, declared, "set-snapshot/corrects-the-area-cell-and-quotes-it: raised diagnostics differ from the committed 🎯️outcome messages");
    let mut snapshot = before();
    apply_csv_mutation(&mut snapshot, &mutation());
    match status {
        "applied" => assert_ne!(snapshot, before(), "set-snapshot/corrects-the-area-cell-and-quotes-it: declared applied but the snapshot came back unchanged"),
        "rejected" => assert_eq!(snapshot, before(), "set-snapshot/corrects-the-area-cell-and-quotes-it: a rejected mutation must leave the snapshot untouched"),
        other => panic!("set-snapshot/corrects-the-area-cell-and-quotes-it: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta this leaf produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: `set-snapshot` has NO whole-snapshot replacement slot
/// in CsvDiff, so the delta must name only the fields that actually differ.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let raised = <CsvMutation as protocol::Mutation<CsvSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(raised.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "set-snapshot/corrects-the-area-cell-and-quotes-it: produced diff differs from the committed 🔺️diff/🔣️component.json");
    assert!(raised.diff().has_header.is_none(), "set-snapshot/corrects-the-area-cell-and-quotes-it: hasHeader is equal on both sides and must stay absent from the sparse delta");
    let records = raised.diff().records.as_ref().expect("set-snapshot/corrects-the-area-cell-and-quotes-it: the records triple must be present");
    assert!(records.removed.is_empty() && records.added.is_empty(), "set-snapshot/corrects-the-area-cell-and-quotes-it: a same-arity cell edit is a positional patch, never a remove+add record pair");
    assert_eq!(records.modified.len(), 1, "set-snapshot/corrects-the-area-cell-and-quotes-it: only the data record is patched");
    assert_eq!(records.modified[0].index, 1, "set-snapshot/corrects-the-area-cell-and-quotes-it: CsvRecordModified indices are BASE-state indices");
}

/// 🔣️ The committed diff is itself canonical and decodes to CsvDiff.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: CsvDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "set-snapshot/corrects-the-area-cell-and-quotes-it: committed diff JSON is not canonical");
    let fields = decoded.records.as_ref().expect("records triple").modified[0].diff.fields.as_ref().expect("positional field patch");
    assert!(fields[0].is_none(), "set-snapshot/corrects-the-area-cell-and-quotes-it: the untouched city cell must round-trip as a positional null, never as an empty CsvFieldDiff object");
    assert_eq!(fields[1].as_ref().expect("area patch").quoted, Some(true), "set-snapshot/corrects-the-area-cell-and-quotes-it: the quoting flag is part of the cell patch, not a codec artefact");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is
/// a complete description of what this `set-snapshot` changed, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: CsvDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <CsvDiff as protocol::MutationDiff<CsvSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "set-snapshot/corrects-the-area-cell-and-quotes-it: committed diff did not carry before to after");
}
