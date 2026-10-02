//! 🧪️ `drag-working-nodes` fixture — `✋️moves`: drags node b of the two-node working graph by (20, -10); node a and every other member stay untouched.
//!
//! Source of truth is the committed JSON quintet (`🧫️fixtures/🧬️mutations/✋️drag-working/✋️moves`), authored by
//! `T/🧪️w3-t2-text-trinity-fixtures.py` independently of this implementation (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING).

use crate::standards::v1::subsets::any::schema::diff::RewritingDiff;
use crate::standards::v1::subsets::any::schema::mutations::RewriteRuleMutation;
use crate::RewritingSnapshot;
use crate::{apply_rewrite_rule_mutation, inverse_rewrite_rule_mutation};

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✋️drag-working/✋️moves/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✋️drag-working/✋️moves/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✋️drag-working/✋️moves/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✋️drag-working/✋️moves/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✋️drag-working/✋️moves/🎯️outcome/🔣️.json");

fn before() -> RewritingSnapshot {
    pack::from_json_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> RewritingSnapshot {
    pack::from_json_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> RewriteRuleMutation {
    pack::from_json_str(MUTATION).expect("mutation decodes")
}

/// ▶️ `drag-working-nodes` carries `before` to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_rewrite_rule_mutation(&mut snapshot, &mutation()).expect("drag-working-nodes applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "drag-working-nodes/✋️moves: applied state differs from the committed after-snapshot");
}

/// ↩️ `drag-working-nodes` undoes with exactly ONE row of `EditBeforeFixture`, which restores `before` exactly.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let inverse = inverse_rewrite_rule_mutation(&base, &mutation());
    assert!(matches!(inverse.as_slice(), [RewriteRuleMutation::EditBeforeFixture(_)]), "drag-working-nodes undoes with one EditBeforeFixture row, got {inverse:?}");
    let mut snapshot = base.clone();
    apply_rewrite_rule_mutation(&mut snapshot, &mutation()).expect("forward applies");
    for step in &inverse {
        apply_rewrite_rule_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "drag-working-nodes/✋️moves: the inverse did not restore the before-snapshot");
}

/// 🔣️ The committed snapshots and payload are canonical: decode → encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RewritingSnapshot = pack::from_json_str(text).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&pack::to_json_string(&decoded)).expect("snapshot encodes");
        assert_eq!(reencoded, serde_json::from_str::<serde_json::Value>(text).expect("snapshot reparses"), "drag-working-nodes/✋️moves: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&pack::to_json_string(&mutation())).expect("mutation encodes");
    assert_eq!(reencoded, serde_json::from_str::<serde_json::Value>(MUTATION).expect("mutation reparses"), "drag-working-nodes/✋️moves: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome holds — an applied change with no diagnostics — and the fixture is bound to `drag-working-nodes`'s descriptor.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"));
    let produced = <RewriteRuleMutation as protocol::Mutation<RewritingSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "drag-working-nodes/✋️moves declares no diagnostics, but got {:?}", produced.messages());
    let semantics = <RewriteRuleMutation as protocol::SemanticMutation<RewritingSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("drag", "working-nodes", "drag-working-nodes", "DraggedWorkingNodes"));
}

/// 🔺️ The sparse delta is exactly the committed diff.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <RewriteRuleMutation as protocol::Mutation<RewritingSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::from_str::<serde_json::Value>(&pack::to_json_string(outcome.diff())).expect("produced diff encodes");
    assert_eq!(produced, serde_json::from_str::<serde_json::Value>(DIFF).expect("committed diff decodes"), "drag-working-nodes/✋️moves: the produced diff differs from the committed one");
}

/// 🩹️ Applying the committed diff to `before` yields the committed `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: RewritingDiff = pack::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <RewritingDiff as protocol::MutationDiff<RewritingSnapshot>>::apply(&decoded, &before()).expect("committed diff applies");
    assert_eq!(produced, expected_after(), "drag-working-nodes/✋️moves: the committed diff did not carry before to after");
}
