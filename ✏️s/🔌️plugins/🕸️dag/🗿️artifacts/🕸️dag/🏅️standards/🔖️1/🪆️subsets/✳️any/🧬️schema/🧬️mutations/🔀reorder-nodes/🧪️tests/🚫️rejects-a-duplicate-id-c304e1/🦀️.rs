//! 🧪️ `reorder-nodes` fixture — `🚫️rejects-a-duplicate-id-in-the-order`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Per contract D6 a rejected case carries
//! `🔺️diff/🚫️.absent` and a `➡️after` byte-identical to `⬅️before`.
//!
//! ⚠️ `DagSnapshot` persists nodes/edges in an opaque composed `s.stdio.semio.graph` CHILD, so this
//! committed snapshot decodes to an UNRESOLVED handle (`🔖️WorkingScene`).
//!
//! 🔀 `reorder-nodes` is the one dag verb with NO target-missing branch — unknown ids are tolerated
//! and merely reported alongside a real reorder. Its only hard rejection is the duplicate-id
//! invariant, which is evaluated on the payload alone, before the scene is consulted at all. That
//! makes it the branch this fixture pins.

use crate::artifacts::dag::mutations::{apply_dag_mutation, inverse_dag_mutation, DagMutation};
use crate::artifacts::dag::{DagDiff, DagSnapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> DagSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> DagSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> DagMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ A rejected `reorder-nodes` leaves the document byte-identical to the committed `after`.
#[semio_framework_async_macros::async_test]
async fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    let mut snapshot = base.clone();
    apply_dag_mutation(&mut snapshot, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "reorder-nodes/rejects-a-duplicate-id-in-the-order: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.content, base.content, "a rejected reorder must not mint a new content handle");
}

/// 🚨️ A repeated id in a FINAL-state order list is a `mutation.invariant` FATAL, and the reported
/// target is the offending id list itself — not a node address, and not the empty order the
/// unresolved scene would otherwise imply.
#[semio_framework_async_macros::async_test]
async fn a_repeated_id_is_a_fatal_invariant_naming_the_duplicates() {
    let produced = <DagMutation as protocol::Mutation<DagSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &DagDiff::default(), "a rejecting reorder-nodes must carry an empty diff");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.invariant", "a duplicated id in the order is an invariant breach, not a missing target");
    assert_eq!(messages[0].level, protocol::Severity::Fatal, "the duplicate-order invariant is Fatal — no merge policy may absorb it");
    assert_eq!(messages[0].target, vec!["node-a".to_string()], "the diagnostic names the duplicated id(s), one entry per repeat beyond the first");
    let semantics = <DagMutation as protocol::SemanticMutation<DagSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("reorder", "nodes", "reorder-nodes", "ReorderedNodes"), "reorder-nodes is the vocabulary's only COLLECTION-scoped verb — its entity is the plural nodes");
    assert!(<DagMutation as protocol::SemanticMutation<DagSnapshot>>::target(&mutation()).is_empty(), "a collection-scoped verb addresses no single entity");
}

/// ↩️ `reorder-nodes` inverts unconditionally to BASE's own id order — so against an unresolved
/// scene it is a reorder to the EMPTY order, not an absent step.
#[semio_framework_async_macros::async_test]
async fn inverse_is_a_reorder_back_to_the_bases_own_order() {
    let inverse = inverse_dag_mutation(&before(), &mutation());
    assert_eq!(inverse.len(), 1, "reorder-nodes always undoes with exactly one counter-reorder, got {inverse:?}");
    let DagMutation::ReorderNodes(undo) = &inverse[0] else {
        panic!("reorder-nodes' inverse must be a reorder-nodes, got {:?}", inverse[0]);
    };
    assert!(undo.order.is_empty(), "BASE's unresolved scene has no nodes, so the counter-order is empty");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: DagSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "reorder-nodes/rejects-a-duplicate-id-in-the-order: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "reorder-nodes/rejects-a-duplicate-id-in-the-order: committed mutation JSON is not canonical");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("rejected"), "reorder-nodes/rejects-a-duplicate-id-in-the-order declares a rejected outcome");
    let produced = <DagMutation as protocol::Mutation<DagSnapshot>>::diff(&mutation(), &before());
    let message = produced.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(serde_json::Value::as_str), Some(message.code.0.as_str()), "the declared code must match the emitted one");
    let declared_path: Vec<String> = outcome.get("path").and_then(serde_json::Value::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared_path, message.target, "the declared path must match the emitted target");
}
