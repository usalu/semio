//! 🧪️ `set-rule-layout-points` fixture — `📍️places`: places the RHS parameter clause at (40, 300) and clears the LHS match back to its default slot.
//!
//! Source of truth is the committed JSON quintet (`🧫️fixtures/🧬️mutations/📍️set-rule-layout/📍️places`), authored by
//! `T/🧪️w3-t2-text-trinity-fixtures.py` independently of this implementation (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING).

use crate::standards::v1::subsets::any::schema::diff::RewritingDiff;
use crate::standards::v1::subsets::any::schema::mutations::RewriteRuleMutation;
use crate::RewritingSnapshot;
use crate::{apply_rewrite_rule_mutation, inverse_rewrite_rule_mutation};

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📍️set-rule-layout/📍️places/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📍️set-rule-layout/📍️places/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📍️set-rule-layout/📍️places/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📍️set-rule-layout/📍️places/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📍️set-rule-layout/📍️places/🎯️outcome/🔣️.json");

fn before() -> RewritingSnapshot {
    pack::from_json_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> RewritingSnapshot {
    pack::from_json_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> RewriteRuleMutation {
    pack::from_json_str(MUTATION).expect("mutation decodes")
}

/// ▶️ `set-rule-layout-points` carries `before` to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_rewrite_rule_mutation(&mut snapshot, &mutation()).expect("set-rule-layout-points applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "set-rule-layout-points/📍️places: applied state differs from the committed after-snapshot");
}

/// ↩️ `set-rule-layout-points` undoes with exactly ONE row of `SetRuleLayoutPoints`, which restores `before` exactly.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let inverse = inverse_rewrite_rule_mutation(&base, &mutation());
    assert!(matches!(inverse.as_slice(), [RewriteRuleMutation::SetRuleLayoutPoints(_)]), "set-rule-layout-points undoes with one SetRuleLayoutPoints row, got {inverse:?}");
    let mut snapshot = base.clone();
    apply_rewrite_rule_mutation(&mut snapshot, &mutation()).expect("forward applies");
    for step in &inverse {
        apply_rewrite_rule_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "set-rule-layout-points/📍️places: the inverse did not restore the before-snapshot");
}

/// 🔣️ The committed snapshots and payload are canonical: decode → encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RewritingSnapshot = pack::from_json_str(text).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&pack::to_json_string(&decoded)).expect("snapshot encodes");
        assert_eq!(reencoded, serde_json::from_str::<serde_json::Value>(text).expect("snapshot reparses"), "set-rule-layout-points/📍️places: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&pack::to_json_string(&mutation())).expect("mutation encodes");
    assert_eq!(reencoded, serde_json::from_str::<serde_json::Value>(MUTATION).expect("mutation reparses"), "set-rule-layout-points/📍️places: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome holds — an applied change with no diagnostics — and the fixture is bound to `set-rule-layout-points`'s descriptor.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"));
    let produced = <RewriteRuleMutation as protocol::Mutation<RewritingSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "set-rule-layout-points/📍️places declares no diagnostics, but got {:?}", produced.messages());
    let semantics = <RewriteRuleMutation as protocol::SemanticMutation<RewritingSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("set", "rule-layout-points", "set-rule-layout-points", "SetRuleLayoutPoints"));
}

/// 🔺️ The sparse delta is exactly the committed diff.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <RewriteRuleMutation as protocol::Mutation<RewritingSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::from_str::<serde_json::Value>(&pack::to_json_string(outcome.diff())).expect("produced diff encodes");
    assert_eq!(produced, serde_json::from_str::<serde_json::Value>(DIFF).expect("committed diff decodes"), "set-rule-layout-points/📍️places: the produced diff differs from the committed one");
}

/// 🩹️ Applying the committed diff to `before` yields the committed `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: RewritingDiff = pack::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <RewritingDiff as protocol::MutationDiff<RewritingSnapshot>>::apply(&decoded, &before()).expect("committed diff applies");
    assert_eq!(produced, expected_after(), "set-rule-layout-points/📍️places: the committed diff did not carry before to after");
}
