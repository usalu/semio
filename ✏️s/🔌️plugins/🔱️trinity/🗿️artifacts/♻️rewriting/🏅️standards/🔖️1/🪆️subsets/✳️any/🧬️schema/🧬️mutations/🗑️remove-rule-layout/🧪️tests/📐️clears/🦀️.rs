//! 🧪️ `remove-rule-layout-point` fixture — `📐️clears`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! 🪆️ Typed LHS/RHS and keyed maps accompany the actual content-addressed Semio child.
//!

use crate::standards::v1::subsets::any::schema::diff::RewritingDiff;
use crate::standards::v1::subsets::any::schema::mutations::RewriteRuleMutation;
use crate::LayoutPoint;
use crate::RewritingSnapshot;
use crate::{apply_rewrite_rule_mutation, inverse_rewrite_rule_mutation};

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🗑️remove-rule-layout/📐️clears/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🗑️remove-rule-layout/📐️clears/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🗑️remove-rule-layout/📐️clears/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🗑️remove-rule-layout/📐️clears/🔺️diff/🔣️.json");
const BEFORE_CHILD:&str=include_str!("../../../../../🧫️fixtures/🧬️mutations/🗑️remove-rule-layout/📐️clears/📸️snapshot/⬅️before/🪆️child/🔣️.json");
const AFTER_CHILD:&str=include_str!("../../../../../🧫️fixtures/🧬️mutations/🗑️remove-rule-layout/📐️clears/📸️snapshot/➡️after/🪆️child/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🗑️remove-rule-layout/📐️clears/🎯️outcome/🔣️.json");

fn before() -> RewritingSnapshot {
    {let mut value=crate::standards::v1::subsets::any::io::text::snapshot::decode_rewriting_snapshot_json(BEFORE).expect("before snapshot decodes");let child=semio_s_artifact_stdio_semio::standards::v1::subsets::graph::io::text::snapshot::decode_semio_graph_snapshot_json(BEFORE_CHILD).expect("declared complete Semio child decodes");semio_s_artifact_trinity_jack::materialize_jack_snapshot(&mut value.working_graph.content,child);value}
}
fn expected_after() -> RewritingSnapshot {
    {let mut value=crate::standards::v1::subsets::any::io::text::snapshot::decode_rewriting_snapshot_json(AFTER).expect("after snapshot decodes");let child=semio_s_artifact_stdio_semio::standards::v1::subsets::graph::io::text::snapshot::decode_semio_graph_snapshot_json(AFTER_CHILD).expect("declared complete Semio child decodes");semio_s_artifact_trinity_jack::materialize_jack_snapshot(&mut value.working_graph.content,child);value}
}
fn mutation() -> RewriteRuleMutation {
    crate::standards::v1::subsets::any::io::text::mutations::decode_rewriting_mutation_json(MUTATION).map(|mut value|{if let RewriteRuleMutation::EditWorkingGraph(payload)=&mut value{let child=semio_s_artifact_stdio_semio::standards::v1::subsets::graph::io::text::snapshot::decode_semio_graph_snapshot_json(AFTER_CHILD).expect("declared replacement Semio child decodes");semio_s_artifact_trinity_jack::materialize_jack_snapshot(&mut payload.new_working_graph.content,child);}value}).expect("mutation decodes")
}

/// ▶️ `remove-rule-layout-point` carries `before` to exactly the committed `after` by taking ONE var out
/// of the `rule_layout` map — shrinking it by exactly one entry.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    let mut snapshot = base.clone();
    apply_rewrite_rule_mutation(&mut snapshot, &mutation()).expect("remove-rule-layout-point applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "remove-rule-layout-point/clears-the-shaft-layout-point: applied state differs from committed after-snapshot");
    assert!(!snapshot.rule_layout.contains_key("shaft"), "remove-rule-layout-point must actually un-pin the addressed var");
    assert_eq!(snapshot.rule_layout.get("c"), Some(&LayoutPoint { x: 12.0, y: -8.0 }), "the other var's point must survive a single-key removal untouched");
    assert_eq!(snapshot.rule_layout.len(), base.rule_layout.len() - 1, "a removal shrinks the map by exactly one entry");
    assert_eq!(snapshot.parameter_bindings, base.parameter_bindings, "remove-rule-layout-point must never reach the OTHER key-addressed map");
}

/// ↩️ `remove-rule-layout-point` inverts BASE-derived: a `change-rule-layout-point` re-pinning the var at
/// the point BASE held. The point exists only on BASE — the payload carries just a key.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_rewrite_rule_mutation(&base, &mutation).expect("valid retained mutation inverse fixture");
    assert_eq!(inverse.len(), 1, "removing a pinned var always undoes with exactly one step, got {inverse:?}");
    let RewriteRuleMutation::ChangeRuleLayoutPoint(undo) = &inverse[0] else {
        panic!("remove-rule-layout-point's inverse must be a change-rule-layout-point, got {:?}", inverse[0]);
    };
    assert_eq!(undo.key, "shaft", "the inverse re-pins exactly the var the payload removed");
    assert_eq!(undo.new_point, LayoutPoint { x: 0.0, y: 0.0 }, "the inverse restores the point only BASE knew");
    let mut snapshot = base.clone();
    apply_rewrite_rule_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_rewrite_rule_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "remove-rule-layout-point/clears-the-shaft-layout-point: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `remove-rule-layout-point` payload are already canonical:
/// decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RewritingSnapshot = crate::standards::v1::subsets::any::io::text::snapshot::decode_rewriting_snapshot_json(text).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&crate::standards::v1::subsets::any::io::text::snapshot::encode_rewriting_snapshot_json(&decoded).expect("declared snapshot JSON encodes")).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "remove-rule-layout-point/clears-the-shaft-layout-point: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&crate::standards::v1::subsets::any::io::text::mutations::encode_rewriting_mutation_json(&mutation()).expect("declared mutation JSON encodes")).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "remove-rule-layout-point/clears-the-shaft-layout-point: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome is exactly what `remove-rule-layout-point` produces here — an applied change with no
/// diagnostics at all — and this fixture is bound to `remove-rule-layout-point`'s own semantic descriptor.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "remove-rule-layout-point/clears-the-shaft-layout-point declares an applied outcome");
    let mut snapshot = before();
    apply_rewrite_rule_mutation(&mut snapshot, &mutation()).expect("remove-rule-layout-point/clears-the-shaft-layout-point: declared applied but the mutation was rejected");
    let produced = <RewriteRuleMutation as protocol::Mutation<RewritingSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "remove-rule-layout-point/clears-the-shaft-layout-point declares no diagnostics, but got {:?}", produced.messages());
    let semantics = <RewriteRuleMutation as protocol::SemanticMutation<RewritingSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("remove", "rule-layout-point", "remove-rule-layout-point", "RemovedRuleLayoutPoint"), "the fixture must be bound to remove-rule-layout-point's own descriptor");
}

/// 🔺️ The sparse delta is exactly the committed diff: a `rule_layout` map holding the removed var mapped
/// to `None`, and no entry at all for the var that stays pinned.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <RewriteRuleMutation as protocol::Mutation<RewritingSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::from_str::<serde_json::Value>(&crate::standards::v1::subsets::any::schema::diff::encode_rewriting_diff_json(outcome.diff()).expect("declared diff JSON encodes")).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "remove-rule-layout-point/clears-the-shaft-layout-point: produced diff differs from the committed 🔺️diff/🔣️.json");
    let typed: RewritingDiff = crate::standards::v1::subsets::any::schema::diff::decode_rewriting_diff_json(DIFF).map(|mut value|{if let Some(parent)=&mut value.working_graph{let child=semio_s_artifact_stdio_semio::standards::v1::subsets::graph::io::text::snapshot::decode_semio_graph_snapshot_json(AFTER_CHILD).expect("declared diff Semio child decodes");semio_s_artifact_trinity_jack::materialize_jack_snapshot(&mut parent.content,child);}value}).expect("committed diff decodes into RewritingDiff");
    let layout = typed.rule_layout.as_ref().expect("remove-rule-layout-point's delta carries a rule_layout map");
    assert_eq!(layout.entries().len(), 1, "a single-var removal must appear in the delta as exactly one entry, got {layout:?}");
    assert_eq!(layout.entries().get("shaft").map(|entry| entry.operation()), Some(&replication::MapEntryOperation::Remove), "a removal uses an explicit tagged operation under the addressed key");
    assert!(!layout.entries().contains_key("c"), "the var that stays pinned must not appear in the delta at all");
    assert!(typed.parameter_bindings.is_none(), "remove-rule-layout-point must never reach the parameter_bindings slot");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own `RewritingDiff`.
/// `RewritingDiff` carries a container-level `#[serde(default)]` and NO per-field
/// `skip_serializing_if`, so all five document slots, including those
/// `remove-rule-layout-point` never touches — must be present as `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: RewritingDiff = crate::standards::v1::subsets::any::schema::diff::decode_rewriting_diff_json(DIFF).map(|mut value|{if let Some(parent)=&mut value.working_graph{let child=semio_s_artifact_stdio_semio::standards::v1::subsets::graph::io::text::snapshot::decode_semio_graph_snapshot_json(AFTER_CHILD).expect("declared diff Semio child decodes");semio_s_artifact_trinity_jack::materialize_jack_snapshot(&mut parent.content,child);}value}).expect("committed diff decodes");
    let reencoded = serde_json::from_str::<serde_json::Value>(&crate::standards::v1::subsets::any::schema::diff::encode_rewriting_diff_json(&decoded).expect("declared diff JSON encodes")).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "remove-rule-layout-point/clears-the-shaft-layout-point: committed diff JSON is not canonical");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    let slots = committed.as_object().expect("the committed diff is a JSON object");
    assert_eq!(slots.len(), 5, "RewritingDiff emits all five document slots, got {slots:?}");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is a
/// complete description of the `remove-rule-layout-point` change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: RewritingDiff = crate::standards::v1::subsets::any::schema::diff::decode_rewriting_diff_json(DIFF).map(|mut value|{if let Some(parent)=&mut value.working_graph{let child=semio_s_artifact_stdio_semio::standards::v1::subsets::graph::io::text::snapshot::decode_semio_graph_snapshot_json(AFTER_CHILD).expect("declared diff Semio child decodes");semio_s_artifact_trinity_jack::materialize_jack_snapshot(&mut parent.content,child);}value}).expect("committed diff decodes");
    let produced = <RewritingDiff as protocol::MutationDiff<RewritingSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "remove-rule-layout-point/clears-the-shaft-layout-point: committed diff did not carry before to after");
}
