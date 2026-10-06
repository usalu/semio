//! 🧪️ `drag-rule-nodes` fixture — `🫳️moves`: drags the LHS match (laid out at (10, 0)) and the RHS set clause (default slot (0, 160)) by (20, 40).
//!
//! Source of truth is the committed JSON quintet (`🧫️fixtures/🧬️mutations/🫳️drag-rule/🫳️moves`), authored by
//! `T/🧪️w3-t2-text-trinity-fixtures.py` independently of this implementation (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING).

use crate::standards::v1::subsets::any::schema::diff::RewritingDiff;
use crate::standards::v1::subsets::any::schema::mutations::RewriteRuleMutation;
use crate::RewritingSnapshot;
use crate::{apply_rewrite_rule_mutation, inverse_rewrite_rule_mutation};

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🫳️drag-rule/🫳️moves/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🫳️drag-rule/🫳️moves/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🫳️drag-rule/🫳️moves/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🫳️drag-rule/🫳️moves/🔺️diff/🔣️.json");
const BEFORE_CHILD:&str=include_str!("../../../../../🧫️fixtures/🧬️mutations/🫳️drag-rule/🫳️moves/📸️snapshot/⬅️before/🪆️child/🔣️.json");
const AFTER_CHILD:&str=include_str!("../../../../../🧫️fixtures/🧬️mutations/🫳️drag-rule/🫳️moves/📸️snapshot/➡️after/🪆️child/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🫳️drag-rule/🫳️moves/🎯️outcome/🔣️.json");

fn before() -> RewritingSnapshot {
    {let mut value=crate::standards::v1::subsets::any::io::text::snapshot::decode_rewriting_snapshot_json(BEFORE).expect("before snapshot decodes");let child=semio_s_artifact_stdio_semio::standards::v1::subsets::graph::io::text::snapshot::decode_semio_graph_snapshot_json(BEFORE_CHILD).expect("declared complete Semio child decodes");semio_s_artifact_trinity_jack::materialize_jack_snapshot(&mut value.working_graph.content,child);value}
}
fn expected_after() -> RewritingSnapshot {
    {let mut value=crate::standards::v1::subsets::any::io::text::snapshot::decode_rewriting_snapshot_json(AFTER).expect("after snapshot decodes");let child=semio_s_artifact_stdio_semio::standards::v1::subsets::graph::io::text::snapshot::decode_semio_graph_snapshot_json(AFTER_CHILD).expect("declared complete Semio child decodes");semio_s_artifact_trinity_jack::materialize_jack_snapshot(&mut value.working_graph.content,child);value}
}
fn mutation() -> RewriteRuleMutation {
    crate::standards::v1::subsets::any::io::text::mutations::decode_rewriting_mutation_json(MUTATION).map(|mut value|{if let RewriteRuleMutation::EditWorkingGraph(payload)=&mut value{let child=semio_s_artifact_stdio_semio::standards::v1::subsets::graph::io::text::snapshot::decode_semio_graph_snapshot_json(AFTER_CHILD).expect("declared replacement Semio child decodes");semio_s_artifact_trinity_jack::materialize_jack_snapshot(&mut payload.new_working_graph.content,child);}value}).expect("mutation decodes")
}

/// ▶️ `drag-rule-nodes` carries `before` to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_rewrite_rule_mutation(&mut snapshot, &mutation()).expect("drag-rule-nodes applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "drag-rule-nodes/🫳️moves: applied state differs from the committed after-snapshot");
}

/// ↩️ `drag-rule-nodes` undoes with exactly ONE row of `SetRuleLayoutPoints`, which restores `before` exactly.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let inverse = inverse_rewrite_rule_mutation(&base, &mutation()).expect("valid retained mutation inverse fixture");
    assert!(matches!(inverse.as_slice(), [RewriteRuleMutation::SetRuleLayoutPoints(_)]), "drag-rule-nodes undoes with one SetRuleLayoutPoints row, got {inverse:?}");
    let mut snapshot = base.clone();
    apply_rewrite_rule_mutation(&mut snapshot, &mutation()).expect("forward applies");
    for step in &inverse {
        apply_rewrite_rule_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "drag-rule-nodes/🫳️moves: the inverse did not restore the before-snapshot");
}

/// 🔣️ The committed snapshots and payload are canonical: decode → encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RewritingSnapshot = crate::standards::v1::subsets::any::io::text::snapshot::decode_rewriting_snapshot_json(text).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&crate::standards::v1::subsets::any::io::text::snapshot::encode_rewriting_snapshot_json(&decoded).expect("declared snapshot JSON encodes")).expect("snapshot encodes");
        assert_eq!(reencoded, serde_json::from_str::<serde_json::Value>(text).expect("snapshot reparses"), "drag-rule-nodes/🫳️moves: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&crate::standards::v1::subsets::any::io::text::mutations::encode_rewriting_mutation_json(&mutation()).expect("declared mutation JSON encodes")).expect("mutation encodes");
    assert_eq!(reencoded, serde_json::from_str::<serde_json::Value>(MUTATION).expect("mutation reparses"), "drag-rule-nodes/🫳️moves: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome holds — an applied change with no diagnostics — and the fixture is bound to `drag-rule-nodes`'s descriptor.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"));
    let produced = <RewriteRuleMutation as protocol::Mutation<RewritingSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "drag-rule-nodes/🫳️moves declares no diagnostics, but got {:?}", produced.messages());
    let semantics = <RewriteRuleMutation as protocol::SemanticMutation<RewritingSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("drag", "rule-nodes", "drag-rule-nodes", "DraggedRuleNodes"));
}

/// 🔺️ The sparse delta is exactly the committed diff.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <RewriteRuleMutation as protocol::Mutation<RewritingSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::from_str::<serde_json::Value>(&crate::standards::v1::subsets::any::schema::diff::encode_rewriting_diff_json(outcome.diff()).expect("declared diff JSON encodes")).expect("produced diff encodes");
    assert_eq!(produced, serde_json::from_str::<serde_json::Value>(DIFF).expect("committed diff decodes"), "drag-rule-nodes/🫳️moves: the produced diff differs from the committed one");
}

/// 🩹️ Applying the committed diff to `before` yields the committed `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: RewritingDiff = crate::standards::v1::subsets::any::schema::diff::decode_rewriting_diff_json(DIFF).map(|mut value|{if let Some(parent)=&mut value.working_graph{let child=semio_s_artifact_stdio_semio::standards::v1::subsets::graph::io::text::snapshot::decode_semio_graph_snapshot_json(AFTER_CHILD).expect("declared diff Semio child decodes");semio_s_artifact_trinity_jack::materialize_jack_snapshot(&mut parent.content,child);}value}).expect("committed diff decodes");
    let produced = <RewritingDiff as protocol::MutationDiff<RewritingSnapshot>>::apply(&decoded, &before()).expect("committed diff applies");
    assert_eq!(produced, expected_after(), "drag-rule-nodes/🫳️moves: the committed diff did not carry before to after");
}
