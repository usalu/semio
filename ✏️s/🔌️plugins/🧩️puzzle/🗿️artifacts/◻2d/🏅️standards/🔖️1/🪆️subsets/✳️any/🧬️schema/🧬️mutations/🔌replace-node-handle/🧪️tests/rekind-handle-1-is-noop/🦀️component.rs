//! 🧪️ `replace-node-handle` fixture — `rekind-handle-1-is-noop`.
//!
//! The builder clones the owner node and compares the clone against the original BEFORE writing the
//! new handle, so its `next == *node` guard always fires: every `replace-node-handle` is a warned
//! no-op with an empty diff. This fixture pins that actual behaviour, not the intent.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::puzzle2d::mutations::Puzzle2dMutation;
use crate::artifacts::puzzle2d::mutations::{apply_puzzle2d_mutation, inverse_puzzle2d_mutation};
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> Puzzle2dSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> Puzzle2dSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> Puzzle2dMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ The committed `replace-node-handle` payload carries `before` to exactly the committed `after`, and
/// lands the change this case is named for.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_puzzle2d_mutation(&mut snapshot, &mutation()).expect("replace-node-handle applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "replace-node-handle/rekind-handle-1-is-noop: applied state differs from committed after-snapshot");
    assert_eq!(snapshot, before(), "replace-node-handle/rekind-handle-1-is-noop: the builder's clone-then-compare guard fires first, so nothing may change");
    let node = snapshot.nodes.iter().find(|node| node.id == "node-a").expect("node-a is untouched");
    assert_eq!(node.handles[0].handle_kind.as_deref(), Some("handle-kind-a"), "replace-node-handle/rekind-handle-1-is-noop: handle-1 must keep its base kind");
}

/// ↩️ Applying `replace-node-handle` then the inverse it derives from `before` restores `before` exactly.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_puzzle2d_mutation(&base, &mutation);
    let mut snapshot = base.clone();
    apply_puzzle2d_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_puzzle2d_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "replace-node-handle/rekind-handle-1-is-noop: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `replace-node-handle` payload are already canonical:
/// decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Puzzle2dSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "replace-node-handle/rekind-handle-1-is-noop: committed {label} JSON is not canonical");
    }
    let decoded_mutation = mutation();
    let reencoded = serde_json::to_value(&decoded_mutation).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "replace-node-handle/rekind-handle-1-is-noop: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `replace-node-handle` actually produces on this base.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let mut snapshot = before();
    let applied = apply_puzzle2d_mutation(&mut snapshot, &mutation()).is_ok();
    match status {
        "applied" => assert!(applied, "replace-node-handle/rekind-handle-1-is-noop: declared applied but the mutation was rejected"),
        "rejected" => {
            assert!(!applied, "replace-node-handle/rekind-handle-1-is-noop: declared rejected but the mutation applied");
            assert_eq!(snapshot, before(), "replace-node-handle/rekind-handle-1-is-noop: rejected mutation must leave the snapshot untouched");
        }
        other => panic!("replace-node-handle/rekind-handle-1-is-noop: unknown outcome status {other:?}"),
    }
    let messages = outcome.get("messages").and_then(serde_json::Value::as_array).expect("replace-node-handle/rekind-handle-1-is-noop: this case declares a warn no-op and must list it");
    assert_eq!(messages[0]["code"].as_str(), Some("mutation.no-op"), "replace-node-handle/rekind-handle-1-is-noop: the declared message must be the no-op warning the builder raises");
}

/// 🔺️ The sparse delta `replace-node-handle` produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: it pins WHICH collections and fields this mutation is
/// allowed to touch, not merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <Puzzle2dMutation as protocol::Mutation<Puzzle2dSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "replace-node-handle/rekind-handle-1-is-noop: produced diff differs from the committed 🔺️diff/🔣️component.json");
    assert!(committed["nodes"].is_null(), "replace-node-handle/rekind-handle-1-is-noop: the no-op guard must leave the nodes delta unset");
    assert!(committed.as_object().expect("the committed diff is a JSON object").values().all(serde_json::Value::is_null), "replace-node-handle/rekind-handle-1-is-noop: a no-op diff must carry no populated field at all");
}

/// 🔣️ The committed `replace-node-handle` diff is itself canonical and decodes to `Puzzle2dDiff`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::puzzle2d::diff::Puzzle2dDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "replace-node-handle/rekind-handle-1-is-noop: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed `replace-node-handle` diff directly to `before` yields the committed `after` —
/// the diff is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::puzzle2d::diff::Puzzle2dDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::puzzle2d::diff::Puzzle2dDiff as protocol::MutationDiff<Puzzle2dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "replace-node-handle/rekind-handle-1-is-noop: committed diff did not carry before to after");
}
