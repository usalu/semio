//! 🧪️ `edit-working-graph` fixture — `🕸️swaps`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! 🪆️ Typed LHS/RHS and keyed maps accompany the actual content-addressed Semio child.
//!
//! 🖼️ `edit-working-graph` is the one rewriting verb whose body is a whole FOREIGN document — a
//! `trinity.graph` fixture — carried as an opaque JSON string. That string is still an ordinary inline
//! `String` field on `RewritingSnapshot`, not a composed child, so it is fully hand-authorable here.

use crate::standards::v1::subsets::any::schema::diff::RewritingDiff;
use crate::standards::v1::subsets::any::schema::mutations::RewriteRuleMutation;
use crate::RewritingSnapshot;
use crate::{apply_rewrite_rule_mutation, inverse_rewrite_rule_mutation};

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🖼️edit-working-graph/🕸️swaps/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🖼️edit-working-graph/🕸️swaps/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🖼️edit-working-graph/🕸️swaps/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🖼️edit-working-graph/🕸️swaps/🔺️diff/🔣️.json");
const BEFORE_CHILD:&str=include_str!("../../../../../🧫️fixtures/🧬️mutations/🖼️edit-working-graph/🕸️swaps/📸️snapshot/⬅️before/🪆️child/🔣️.json");
const AFTER_CHILD:&str=include_str!("../../../../../🧫️fixtures/🧬️mutations/🖼️edit-working-graph/🕸️swaps/📸️snapshot/➡️after/🪆️child/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🖼️edit-working-graph/🕸️swaps/🎯️outcome/🔣️.json");

fn before() -> RewritingSnapshot {
    {let mut value=crate::standards::v1::subsets::any::io::text::snapshot::decode_rewriting_snapshot_json(BEFORE).expect("before snapshot decodes");let child=semio_s_artifact_stdio_semio::standards::v1::subsets::graph::io::text::snapshot::decode_semio_graph_snapshot_json(BEFORE_CHILD).expect("declared complete Semio child decodes");semio_s_artifact_trinity_jack::materialize_jack_snapshot(&mut value.working_graph.content,child);value}
}
fn expected_after() -> RewritingSnapshot {
    {let mut value=crate::standards::v1::subsets::any::io::text::snapshot::decode_rewriting_snapshot_json(AFTER).expect("after snapshot decodes");let child=semio_s_artifact_stdio_semio::standards::v1::subsets::graph::io::text::snapshot::decode_semio_graph_snapshot_json(AFTER_CHILD).expect("declared complete Semio child decodes");semio_s_artifact_trinity_jack::materialize_jack_snapshot(&mut value.working_graph.content,child);value}
}
fn mutation() -> RewriteRuleMutation {
    crate::standards::v1::subsets::any::io::text::mutations::decode_rewriting_mutation_json(MUTATION).map(|mut value|{if let RewriteRuleMutation::EditWorkingGraph(payload)=&mut value{let child=semio_s_artifact_stdio_semio::standards::v1::subsets::graph::io::text::snapshot::decode_semio_graph_snapshot_json(AFTER_CHILD).expect("declared replacement Semio child decodes");semio_s_artifact_trinity_jack::materialize_jack_snapshot(&mut payload.new_working_graph.content,child);}value}).expect("mutation decodes")
}

/// ▶️ `edit-working-graph` carries `before` to exactly the committed `after` by replacing the whole
/// "before" working-graph body — here a bare header growing a two-node `nodes` array — and nothing else.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    let mut snapshot = base.clone();
    apply_rewrite_rule_mutation(&mut snapshot, &mutation()).expect("edit-working-graph applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "edit-working-graph/swaps-in-a-two-node-before-graph: applied state differs from committed after-snapshot");
    assert_ne!(snapshot.working_graph, base.working_graph, "edit-working-graph must actually replace the before-graph body");
    assert_eq!(snapshot.working_graph.content, expected_after().working_graph.content, "the committed after retains the exact authored child identity");
    let child: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧫️fixtures/🧬️mutations/🖼️edit-working-graph/🕸️swaps/📸️snapshot/➡️after/🪆️child/🔣️.json")).expect("typed committed child");
    assert_eq!(child["nodes"].as_array().expect("typed child nodes").len(), 2, "the committed after retains the two-node graph");
    assert_eq!(snapshot.lhs, base.lhs, "edit-working-graph must not touch the LHS match pattern");
    assert_eq!(snapshot.rhs, base.rhs, "edit-working-graph must not touch the RHS rewriting program");
    assert_eq!(snapshot.parameter_bindings, base.parameter_bindings, "edit-working-graph must not reach the parameter-binding map");
    assert_eq!(snapshot.rule_layout, base.rule_layout, "edit-working-graph must not reach the rule-layout map");
}

/// ↩️ `edit-working-graph` inverts BASE-derived: one `edit-working-graph` back to the graph body BASE
/// was carrying — the whole foreign document, verbatim.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_rewrite_rule_mutation(&base, &mutation).expect("valid retained mutation inverse fixture");
    assert_eq!(inverse.len(), 1, "edit-working-graph always undoes with exactly one step, got {inverse:?}");
    let RewriteRuleMutation::EditWorkingGraph(undo) = &inverse[0] else {
        panic!("edit-working-graph's inverse must itself be an edit-working-graph, got {:?}", inverse[0]);
    };
    assert_eq!(undo.new_working_graph, base.working_graph, "the inverse restores exactly the before-graph body BASE carried");
    let mut snapshot = base.clone();
    apply_rewrite_rule_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_rewrite_rule_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "edit-working-graph/swaps-in-a-two-node-before-graph: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `edit-working-graph` payload are already canonical:
/// decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RewritingSnapshot = crate::standards::v1::subsets::any::io::text::snapshot::decode_rewriting_snapshot_json(text).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&crate::standards::v1::subsets::any::io::text::snapshot::encode_rewriting_snapshot_json(&decoded).expect("declared snapshot JSON encodes")).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "edit-working-graph/swaps-in-a-two-node-before-graph: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&crate::standards::v1::subsets::any::io::text::mutations::encode_rewriting_mutation_json(&mutation()).expect("declared mutation JSON encodes")).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "edit-working-graph/swaps-in-a-two-node-before-graph: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome is exactly what `edit-working-graph` produces here — an applied change with no
/// diagnostics at all — and this fixture is bound to `edit-working-graph`'s own semantic descriptor.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "edit-working-graph/swaps-in-a-two-node-before-graph declares an applied outcome");
    let mut snapshot = before();
    apply_rewrite_rule_mutation(&mut snapshot, &mutation()).expect("edit-working-graph/swaps-in-a-two-node-before-graph: declared applied but the mutation was rejected");
    let produced = <RewriteRuleMutation as protocol::Mutation<RewritingSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "edit-working-graph/swaps-in-a-two-node-before-graph declares no diagnostics, but got {:?}", produced.messages());
    let semantics = <RewriteRuleMutation as protocol::SemanticMutation<RewritingSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("edit", "working-graph", "edit-working-graph", "EditedWorkingGraph"), "the fixture must be bound to edit-working-graph's own descriptor");
}

/// 🔺️ The sparse delta `edit-working-graph` produces is exactly the committed diff: the single
/// `working_graph` slot set, with neither authored rule half disturbed.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <RewriteRuleMutation as protocol::Mutation<RewritingSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::from_str::<serde_json::Value>(&crate::standards::v1::subsets::any::schema::diff::encode_rewriting_diff_json(outcome.diff()).expect("declared diff JSON encodes")).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "edit-working-graph/swaps-in-a-two-node-before-graph: produced diff differs from the committed 🔺️diff/🔣️.json");
    let typed: RewritingDiff = crate::standards::v1::subsets::any::schema::diff::decode_rewriting_diff_json(DIFF).map(|mut value|{if let Some(parent)=&mut value.working_graph{let child=semio_s_artifact_stdio_semio::standards::v1::subsets::graph::io::text::snapshot::decode_semio_graph_snapshot_json(AFTER_CHILD).expect("declared diff Semio child decodes");semio_s_artifact_trinity_jack::materialize_jack_snapshot(&mut parent.content,child);}value}).expect("committed diff decodes into RewritingDiff");
    assert!(typed.working_graph.is_some(), "edit-working-graph's delta must set the working_graph slot");
    assert!(typed.lhs.is_none() && typed.rhs.is_none(), "edit-working-graph must never disturb either authored rule half");
    assert!(typed.parameter_bindings.is_none() && typed.rule_layout.is_none(), "edit-working-graph's delta must never reach either key-addressed map");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own `RewritingDiff`.
/// `RewritingDiff` carries a container-level `#[serde(default)]` and NO per-field
/// `skip_serializing_if`, so all five document slots, including those
/// `edit-working-graph` never touches — must be present as `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: RewritingDiff = crate::standards::v1::subsets::any::schema::diff::decode_rewriting_diff_json(DIFF).map(|mut value|{if let Some(parent)=&mut value.working_graph{let child=semio_s_artifact_stdio_semio::standards::v1::subsets::graph::io::text::snapshot::decode_semio_graph_snapshot_json(AFTER_CHILD).expect("declared diff Semio child decodes");semio_s_artifact_trinity_jack::materialize_jack_snapshot(&mut parent.content,child);}value}).expect("committed diff decodes");
    let reencoded = serde_json::from_str::<serde_json::Value>(&crate::standards::v1::subsets::any::schema::diff::encode_rewriting_diff_json(&decoded).expect("declared diff JSON encodes")).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "edit-working-graph/swaps-in-a-two-node-before-graph: committed diff JSON is not canonical");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    let slots = committed.as_object().expect("the committed diff is a JSON object");
    assert_eq!(slots.len(), 5, "RewritingDiff emits all five document slots, got {slots:?}");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is a
/// complete description of the `edit-working-graph` change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: RewritingDiff = crate::standards::v1::subsets::any::schema::diff::decode_rewriting_diff_json(DIFF).map(|mut value|{if let Some(parent)=&mut value.working_graph{let child=semio_s_artifact_stdio_semio::standards::v1::subsets::graph::io::text::snapshot::decode_semio_graph_snapshot_json(AFTER_CHILD).expect("declared diff Semio child decodes");semio_s_artifact_trinity_jack::materialize_jack_snapshot(&mut parent.content,child);}value}).expect("committed diff decodes");
    let produced = <RewritingDiff as protocol::MutationDiff<RewritingSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "edit-working-graph/swaps-in-a-two-node-before-graph: committed diff did not carry before to after");
}
