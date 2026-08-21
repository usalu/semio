//! 🧪️ `set-snapshot` fixture — `rewrites-the-two-middle-bytes`.
//!
//! `BinaryDiff` has no `bytes` slot at all: `diff_set_snapshot` forwards straight to
//! `BinaryDiff::between`, whose common-prefix/common-suffix scan reduces a whole-buffer
//! replacement to ONE `ByteSplice{offset, remove_len, insert}` covering just the differing
//! middle. This fixture keeps the payload four bytes long so the committed splice can be
//! checked by hand: prefix `00` and suffix `03` survive, `01 02` becomes `ff fe`.
//! `bytes`/`insert` are `Vec<u8>` and serde encodes them as JSON arrays of byte NUMBERS —
//! the `#[dsl(base64)]` attribute governs only the DSL/op codec, never this JSON.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`), every value of which was transcribed from this
//! leaf's own `🔺️diff/🦀️component.rs` oracle. The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::binary::standards::v_raw::subsets::any::schema::diff::BinaryDiff;
use crate::artifacts::binary::standards::v_raw::subsets::any::schema::mutations::{apply_binary_mutation, BinaryMutation};
use crate::artifacts::binary::standards::v_raw::subsets::any::schema::snapshot::BinarySnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> BinarySnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> BinarySnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> BinaryMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ `set-snapshot` carries the committed `before` BinarySnapshot to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    let outcome = apply_binary_mutation(&mut snapshot, &mutation());
    assert!(outcome.messages().is_empty(), "set-snapshot/rewrites-the-two-middle-bytes: set-snapshot raised diagnostics it should not have");
    assert_eq!(snapshot, expected_after(), "set-snapshot/rewrites-the-two-middle-bytes: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.bytes, vec![0u8, 255, 254, 3], "set-snapshot/rewrites-the-two-middle-bytes: the buffer must land on 00 ff fe 03");
    assert_eq!(snapshot.schema, before().schema, "set-snapshot/rewrites-the-two-middle-bytes: BinaryDiff carries no schema slot, so the envelope id must survive a whole-buffer replacement");
}

/// ↩️ `set-snapshot`'s inverse is a single `SetSnapshot` carrying the pre-state BinarySnapshot back, so
/// forward-then-undo restores `before` byte for byte.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <BinaryMutation as protocol::Mutation<BinarySnapshot>>::inverse(&mutation, &base);
    assert_eq!(inverse.len(), 1, "set-snapshot/rewrites-the-two-middle-bytes: undoing a whole-snapshot replacement is exactly one step");
    assert!(matches!(inverse[0], BinaryMutation::SetSnapshot { .. }), "set-snapshot/rewrites-the-two-middle-bytes: the undo step must itself be a SetSnapshot carrying the pre-state");
    let mut snapshot = base.clone();
    apply_binary_mutation(&mut snapshot, &mutation);
    for step in &inverse {
        apply_binary_mutation(&mut snapshot, step);
    }
    assert_eq!(snapshot, base, "set-snapshot/rewrites-the-two-middle-bytes: inverse did not restore the before-snapshot");
    assert_eq!(snapshot.bytes, vec![0u8, 1, 2, 3], "set-snapshot/rewrites-the-two-middle-bytes: the undo must put 01 02 back between the surviving prefix and suffix");
}

/// 🔣️ Both committed BinarySnapshot snapshots and this leaf's committed mutation payload are already
/// canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: BinarySnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "set-snapshot/rewrites-the-two-middle-bytes: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "set-snapshot/rewrites-the-two-middle-bytes: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome — status AND every diagnostic this leaf's own diff builder raises for
/// this payload — matches what the mutation actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let declared: Vec<(String, String)> =
        outcome.get("messages").and_then(serde_json::Value::as_array).map(|rows| rows.iter().map(|row| (row["level"].as_str().unwrap_or_default().to_string(), row["code"].as_str().unwrap_or_default().to_string())).collect()).unwrap_or_default();
    let raised = <BinaryMutation as protocol::Mutation<BinarySnapshot>>::diff(&mutation(), &before());
    let produced: Vec<(String, String)> = raised
        .messages()
        .iter()
        .map(|message| {
            let level = serde_json::to_value(message.level).expect("severity encodes");
            (level.as_str().unwrap_or_default().to_string(), message.code.0.clone())
        })
        .collect();
    assert_eq!(produced, declared, "set-snapshot/rewrites-the-two-middle-bytes: raised diagnostics differ from the committed 🎯️outcome messages");
    let mut snapshot = before();
    apply_binary_mutation(&mut snapshot, &mutation());
    match status {
        "applied" => assert_ne!(snapshot, before(), "set-snapshot/rewrites-the-two-middle-bytes: declared applied but the snapshot came back unchanged"),
        "rejected" => assert_eq!(snapshot, before(), "set-snapshot/rewrites-the-two-middle-bytes: a rejected mutation must leave the snapshot untouched"),
        other => panic!("set-snapshot/rewrites-the-two-middle-bytes: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta this leaf produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: `set-snapshot` has NO whole-snapshot replacement slot
/// in BinaryDiff, so the delta must name only the fields that actually differ.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let raised = <BinaryMutation as protocol::Mutation<BinarySnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(raised.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "set-snapshot/rewrites-the-two-middle-bytes: produced diff differs from the committed 🔺️diff/🔣️component.json");
    assert_eq!(raised.diff().splices.len(), 1, "set-snapshot/rewrites-the-two-middle-bytes: BinaryDiff::between emits exactly one splice for one contiguous differing region");
    assert_eq!(raised.diff().splices[0].offset, 1, "set-snapshot/rewrites-the-two-middle-bytes: the shared leading 00 byte must stay outside the splice");
    assert_eq!(raised.diff().splices[0].remove_len, 2, "set-snapshot/rewrites-the-two-middle-bytes: the shared trailing 03 byte must stay outside the splice");
}

/// 🔣️ The committed diff is itself canonical and decodes to BinaryDiff.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: BinaryDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "set-snapshot/rewrites-the-two-middle-bytes: committed diff JSON is not canonical");
    assert_eq!(decoded.splices[0].insert, vec![255u8, 254], "set-snapshot/rewrites-the-two-middle-bytes: insert bytes decode from a JSON number array — a base64 string here would mean the committed diff was written against the wrong codec");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is
/// a complete description of what this `set-snapshot` changed, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: BinaryDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <BinaryDiff as protocol::MutationDiff<BinarySnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "set-snapshot/rewrites-the-two-middle-bytes: committed diff did not carry before to after");
}
