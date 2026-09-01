//! 🧪️ `set-snapshot` fixture — `renames-the-alpha-row-and-switches-to-crlf`.
//!
//! IANA TSV has no quoting mechanism at all, so a row is a bare `Vec<String>` and
//! `TsvRowDiff::between` yields a positional `Option<String>` per column — a changed cell is
//! the replacement string itself, not a sub-record. `TsvDiff` also owns two whole-file
//! retention scalars; this payload moves `line_ending` and deliberately leaves
//! `trailing_newline` equal so the delta has to prove it can skip it.
//! `LineEnding::Crlf` here serialises `"crlf"` — tsv declares the variant as one word,
//! unlike txt's `CrLf`/`"crLf"`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`), every value of which was transcribed from this
//! leaf's own `🔺️diff/🦀️component.rs` oracle. The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::tsv::standards::iana::subsets::any::schema::diff::TsvDiff;
use crate::artifacts::tsv::standards::iana::subsets::any::schema::mutations::{apply_tsv_mutation, TsvMutation};
use crate::artifacts::tsv::standards::iana::subsets::any::schema::snapshot::LineEnding;
use crate::artifacts::tsv::standards::iana::subsets::any::schema::snapshot::TsvSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> TsvSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> TsvSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> TsvMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ `set-snapshot` carries the committed `before` TsvSnapshot to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    let outcome = apply_tsv_mutation(&mut snapshot, &mutation());
    assert!(outcome.messages().is_empty(), "set-snapshot/renames-the-alpha-row-and-switches-to-crlf: set-snapshot raised diagnostics it should not have");
    assert_eq!(snapshot, expected_after(), "set-snapshot/renames-the-alpha-row-and-switches-to-crlf: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.records[1], vec!["1".to_string(), "Beta".to_string()], "set-snapshot/renames-the-alpha-row-and-switches-to-crlf: only the name column of the data row moves");
    assert_eq!(snapshot.records[0], before().records[0], "set-snapshot/renames-the-alpha-row-and-switches-to-crlf: the id/name header row is identical on both sides and must survive untouched");
    assert_eq!(snapshot.line_ending, LineEnding::Crlf, "set-snapshot/renames-the-alpha-row-and-switches-to-crlf: the file must switch to CRLF");
    assert!(snapshot.trailing_newline, "set-snapshot/renames-the-alpha-row-and-switches-to-crlf: trailingNewline is equal on both sides, so TsvDiff must leave it alone");
}

/// ↩️ `set-snapshot`'s inverse is a single `SetSnapshot` carrying the pre-state TsvSnapshot back, so
/// forward-then-undo restores `before` byte for byte.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <TsvMutation as protocol::Mutation<TsvSnapshot>>::inverse(&mutation, &base);
    assert_eq!(inverse.len(), 1, "set-snapshot/renames-the-alpha-row-and-switches-to-crlf: undoing a whole-snapshot replacement is exactly one step");
    assert!(matches!(inverse[0], TsvMutation::SetSnapshot(_)), "set-snapshot/renames-the-alpha-row-and-switches-to-crlf: the undo step must itself be a SetSnapshot carrying the pre-state");
    let mut snapshot = base.clone();
    apply_tsv_mutation(&mut snapshot, &mutation);
    for step in &inverse {
        apply_tsv_mutation(&mut snapshot, step);
    }
    assert_eq!(snapshot, base, "set-snapshot/renames-the-alpha-row-and-switches-to-crlf: inverse did not restore the before-snapshot");
    assert_eq!(snapshot.records[1][1], "Alpha", "set-snapshot/renames-the-alpha-row-and-switches-to-crlf: the undo must put Alpha back in the name column");
    assert_eq!(snapshot.line_ending, LineEnding::Lf, "set-snapshot/renames-the-alpha-row-and-switches-to-crlf: the undo must restore the LF line ending");
}

/// 🔣️ Both committed TsvSnapshot snapshots and this leaf's committed mutation payload are already
/// canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: TsvSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "set-snapshot/renames-the-alpha-row-and-switches-to-crlf: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "set-snapshot/renames-the-alpha-row-and-switches-to-crlf: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome — status AND every diagnostic this leaf's own diff builder raises for
/// this payload — matches what the mutation actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let declared: Vec<(String, String)> =
        outcome.get("messages").and_then(serde_json::Value::as_array).map(|rows| rows.iter().map(|row| (row["level"].as_str().unwrap_or_default().to_string(), row["code"].as_str().unwrap_or_default().to_string())).collect()).unwrap_or_default();
    let raised = <TsvMutation as protocol::Mutation<TsvSnapshot>>::diff(&mutation(), &before());
    let produced: Vec<(String, String)> = raised
        .messages()
        .iter()
        .map(|message| {
            let level = serde_json::to_value(message.level).expect("severity encodes");
            (level.as_str().unwrap_or_default().to_string(), message.code.0.clone())
        })
        .collect();
    assert_eq!(produced, declared, "set-snapshot/renames-the-alpha-row-and-switches-to-crlf: raised diagnostics differ from the committed 🎯️outcome messages");
    let mut snapshot = before();
    apply_tsv_mutation(&mut snapshot, &mutation());
    match status {
        "applied" => assert_ne!(snapshot, before(), "set-snapshot/renames-the-alpha-row-and-switches-to-crlf: declared applied but the snapshot came back unchanged"),
        "rejected" => assert_eq!(snapshot, before(), "set-snapshot/renames-the-alpha-row-and-switches-to-crlf: a rejected mutation must leave the snapshot untouched"),
        other => panic!("set-snapshot/renames-the-alpha-row-and-switches-to-crlf: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta this leaf produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: `set-snapshot` has NO whole-snapshot replacement slot
/// in TsvDiff, so the delta must name only the fields that actually differ.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let raised = <TsvMutation as protocol::Mutation<TsvSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(raised.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "set-snapshot/renames-the-alpha-row-and-switches-to-crlf: produced diff differs from the committed 🔺️diff/🔣️component.json");
    assert!(raised.diff().trailing_newline.is_none(), "set-snapshot/renames-the-alpha-row-and-switches-to-crlf: trailingNewline is unchanged and must stay absent from the sparse delta");
    assert_eq!(raised.diff().line_ending, Some(LineEnding::Crlf), "set-snapshot/renames-the-alpha-row-and-switches-to-crlf: the line-ending scalar is the one whole-file field this payload moves");
    let records = raised.diff().records.as_ref().expect("set-snapshot/renames-the-alpha-row-and-switches-to-crlf: the rows triple must be present");
    assert!(records.removed.is_empty() && records.added.is_empty(), "set-snapshot/renames-the-alpha-row-and-switches-to-crlf: both rows keep two columns, so this is a positional patch and never a remove+add row pair");
    assert_eq!(records.modified.len(), 1, "set-snapshot/renames-the-alpha-row-and-switches-to-crlf: only the data row is patched");
    let fields = records.modified[0].diff.fields.as_ref().expect("set-snapshot/renames-the-alpha-row-and-switches-to-crlf: the positional column patch must be present");
    assert!(fields[0].is_none(), "set-snapshot/renames-the-alpha-row-and-switches-to-crlf: the id column is unchanged and must be a positional null");
}

/// 🔣️ The committed diff is itself canonical and decodes to TsvDiff.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: TsvDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "set-snapshot/renames-the-alpha-row-and-switches-to-crlf: committed diff JSON is not canonical");
    assert_eq!(decoded.line_ending, Some(LineEnding::Crlf), "set-snapshot/renames-the-alpha-row-and-switches-to-crlf: 'crlf' must decode back to tsv's own one-word Crlf variant");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is
/// a complete description of what this `set-snapshot` changed, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: TsvDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <TsvDiff as protocol::MutationDiff<TsvSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "set-snapshot/renames-the-alpha-row-and-switches-to-crlf: committed diff did not carry before to after");
}
