//! 🧪️ `set-snapshot` fixture — `appends-a-third-line-and-switches-to-crlf`.
//!
//! `TxtDiff::between` is the whole oracle here: `trailing_newline`/`line_ending` are plain
//! `Option` scalars set only when they actually differ, and `lines` is a `TxtLinesDiff`
//! triple whose `between` pairs by position — the two shared lines are byte-identical, so
//! they appear NOWHERE in the delta; only the appended third line shows up, as an `added`
//! entry at its FINAL index 2. `LineEnding::CrLf` serialises `"crLf"` (serde's camelCase
//! lowercases only the leading character of a variant), which is what pins this fixture to
//! `stdio.txt` rather than to tsv's own two-variant `Lf`/`Crlf` enum.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`), every value of which was transcribed from this
//! leaf's own `🔺️diff/🦀️component.rs` oracle. The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::txt::standards::v_utf_8::subsets::any::schema::diff::TxtDiff;
use crate::artifacts::txt::standards::v_utf_8::subsets::any::schema::mutations::{apply_txt_mutation, TxtMutation};
use crate::artifacts::txt::standards::v_utf_8::subsets::any::schema::snapshot::TxtSnapshot;
use crate::artifacts::txt::standards::v_utf_8::subsets::any::schema::snapshot::LineEnding;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> TxtSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> TxtSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> TxtMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ `set-snapshot` carries the committed `before` TxtSnapshot to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    let outcome = apply_txt_mutation(&mut snapshot, &mutation());
    assert!(outcome.messages().is_empty(), "set-snapshot/appends-a-third-line-and-switches-to-crlf: set-snapshot raised diagnostics it should not have");
    assert_eq!(snapshot, expected_after(), "set-snapshot/appends-a-third-line-and-switches-to-crlf: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.lines, vec!["semio".to_string(), "stdio".to_string(), "txt".to_string()], "set-snapshot/appends-a-third-line-and-switches-to-crlf: the third line must land at the end of the line array");
    assert_eq!(snapshot.line_ending, LineEnding::CrLf, "set-snapshot/appends-a-third-line-and-switches-to-crlf: the document must switch to CRLF");
    assert!(snapshot.trailing_newline, "set-snapshot/appends-a-third-line-and-switches-to-crlf: trailingNewline is equal on both sides, so TxtDiff must leave it exactly as the base had it");
}

/// ↩️ `set-snapshot`'s inverse is a single `SetSnapshot` carrying the pre-state TxtSnapshot back, so
/// forward-then-undo restores `before` byte for byte.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <TxtMutation as protocol::Mutation<TxtSnapshot>>::inverse(&mutation, &base);
    assert_eq!(inverse.len(), 1, "set-snapshot/appends-a-third-line-and-switches-to-crlf: undoing a whole-snapshot replacement is exactly one step");
    assert!(matches!(inverse[0], TxtMutation::SetSnapshot { .. }), "set-snapshot/appends-a-third-line-and-switches-to-crlf: the undo step must itself be a SetSnapshot carrying the pre-state");
    let mut snapshot = base.clone();
    apply_txt_mutation(&mut snapshot, &mutation);
    for step in &inverse {
        apply_txt_mutation(&mut snapshot, step);
    }
    assert_eq!(snapshot, base, "set-snapshot/appends-a-third-line-and-switches-to-crlf: inverse did not restore the before-snapshot");
    assert_eq!(snapshot.lines.len(), 2, "set-snapshot/appends-a-third-line-and-switches-to-crlf: the undo must drop the appended line again");
    assert_eq!(snapshot.line_ending, LineEnding::Lf, "set-snapshot/appends-a-third-line-and-switches-to-crlf: the undo must put the LF line ending back");
}

/// 🔣️ Both committed TxtSnapshot snapshots and this leaf's committed mutation payload are already
/// canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: TxtSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "set-snapshot/appends-a-third-line-and-switches-to-crlf: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "set-snapshot/appends-a-third-line-and-switches-to-crlf: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome — status AND every diagnostic this leaf's own diff builder raises for
/// this payload — matches what the mutation actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let declared: Vec<(String, String)> = outcome
        .get("messages")
        .and_then(serde_json::Value::as_array)
        .map(|rows| rows.iter().map(|row| (row["level"].as_str().unwrap_or_default().to_string(), row["code"].as_str().unwrap_or_default().to_string())).collect())
        .unwrap_or_default();
    let raised = <TxtMutation as protocol::Mutation<TxtSnapshot>>::diff(&mutation(), &before());
    let produced: Vec<(String, String)> = raised
        .messages()
        .iter()
        .map(|message| {
            let level = serde_json::to_value(message.level).expect("severity encodes");
            (level.as_str().unwrap_or_default().to_string(), message.code.0.clone())
        })
        .collect();
    assert_eq!(produced, declared, "set-snapshot/appends-a-third-line-and-switches-to-crlf: raised diagnostics differ from the committed 🎯️outcome messages");
    let mut snapshot = before();
    apply_txt_mutation(&mut snapshot, &mutation());
    match status {
        "applied" => assert_ne!(snapshot, before(), "set-snapshot/appends-a-third-line-and-switches-to-crlf: declared applied but the snapshot came back unchanged"),
        "rejected" => assert_eq!(snapshot, before(), "set-snapshot/appends-a-third-line-and-switches-to-crlf: a rejected mutation must leave the snapshot untouched"),
        other => panic!("set-snapshot/appends-a-third-line-and-switches-to-crlf: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta this leaf produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: `set-snapshot` has NO whole-snapshot replacement slot
/// in TxtDiff, so the delta must name only the fields that actually differ.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let raised = <TxtMutation as protocol::Mutation<TxtSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(raised.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "set-snapshot/appends-a-third-line-and-switches-to-crlf: produced diff differs from the committed 🔺️diff/🔣️component.json");
    assert!(raised.diff().trailing_newline.is_none(), "set-snapshot/appends-a-third-line-and-switches-to-crlf: trailingNewline is unchanged and must stay absent from the sparse delta");
    assert_eq!(raised.diff().line_ending, Some(LineEnding::CrLf), "set-snapshot/appends-a-third-line-and-switches-to-crlf: the line-ending scalar is the one document-level field this payload moves");
    let lines = raised.diff().lines.as_ref().expect("set-snapshot/appends-a-third-line-and-switches-to-crlf: the lines triple must be present");
    assert!(lines.modified.is_empty(), "set-snapshot/appends-a-third-line-and-switches-to-crlf: 'semio' and 'stdio' are byte-identical on both sides — a modified entry for either would mean the delta rewrote an untouched line");
    assert!(lines.removed.is_empty(), "set-snapshot/appends-a-third-line-and-switches-to-crlf: nothing is dropped by an append");
    assert_eq!(lines.added.len(), 1, "set-snapshot/appends-a-third-line-and-switches-to-crlf: exactly one line is appended");
    assert_eq!(lines.added[0].index, 2, "set-snapshot/appends-a-third-line-and-switches-to-crlf: TxtLineAdded indices are FINAL-state indices");
}

/// 🔣️ The committed diff is itself canonical and decodes to TxtDiff.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: TxtDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "set-snapshot/appends-a-third-line-and-switches-to-crlf: committed diff JSON is not canonical");
    assert_eq!(decoded.line_ending, Some(LineEnding::CrLf), "set-snapshot/appends-a-third-line-and-switches-to-crlf: 'crLf' must decode back to LineEnding::CrLf — 'crlf' would be tsv's spelling, not txt's");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is
/// a complete description of what this `set-snapshot` changed, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: TxtDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <TxtDiff as protocol::MutationDiff<TxtSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "set-snapshot/appends-a-third-line-and-switches-to-crlf: committed diff did not carry before to after");
}
