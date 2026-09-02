//! 🧪️ `change-node-operator-kind` fixture — `rejects-rebinding-the-operator-of-a-missing-node`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Per contract D6 a rejected case carries
//! `🔺️diff/🚫️.absent` and a `➡️after` byte-identical to `⬅️before`.
//!
//! ⚠️ `DagSnapshot` persists nodes/edges in an opaque composed `s.stdio.semio.graph` CHILD, so this
//! committed snapshot decodes to an UNRESOLVED handle and `dag_working_scene` fails soft to an
//! empty scene (`🔖️WorkingScene`) — the state this case pins.

use crate::artifacts::dag::mutations::{apply_dag_mutation, change_node_operator_kind, inverse_dag_mutation, DagMutation};
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

/// ▶️ A rejected `change-node-operator-kind` leaves the document byte-identical to the committed
/// `after`.
#[semio_framework_async_macros::async_test]
async fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    let mut snapshot = base.clone();
    apply_dag_mutation(&mut snapshot, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "change-node-operator-kind/rejects-rebinding-the-operator-of-a-missing-node: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.content, base.content, "a rejected operator rebind must not mint a new content handle");
}

/// 🧮 `operator_kind` is the vocabulary's only OPTIONAL node scalar — `None` means "no compute
/// binding" and is a legal target value. Unbinding an absent node is therefore still a miss, not the
/// no-op an already-unbound node would produce.
#[semio_framework_async_macros::async_test]
async fn unbinding_an_absent_node_is_still_a_miss() {
    let produced = <DagMutation as protocol::Mutation<DagSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &DagDiff::default(), "a rejecting change-node-operator-kind must carry an empty diff");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.target-missing", "a missing node is reported as target-missing");
    assert_eq!(messages[0].level, protocol::Severity::Error, "this verb has no Fatal branch at all");
    assert_eq!(messages[0].target, vec!["node-a".to_string()], "the diagnostic addresses the payload's node id");
    let unbind = <DagMutation as protocol::Mutation<DagSnapshot>>::diff(&change_node_operator_kind("node-a".into(), None), &before());
    assert_eq!(unbind.messages()[0].code.0, "mutation.target-missing", "a None payload against an absent node reports the miss, never the no-op");
    let semantics = <DagMutation as protocol::SemanticMutation<DagSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.kind, semantics.record), ("change", "change-node-operator-kind", "ChangedNodeOperatorKind"), "the fixture must be bound to change-node-operator-kind's own descriptor");
}

/// ↩️ The inverse is a counter-write of the OLD `Option<String>` read from BASE; with no node found
/// the inverse is empty rather than an unbind to `None`.
#[semio_framework_async_macros::async_test]
async fn inverse_has_no_old_binding_to_restore() {
    let inverse = inverse_dag_mutation(&before(), &mutation());
    assert!(inverse.is_empty(), "change-node-operator-kind/rejects-rebinding-the-operator-of-a-missing-node: a rejected change must have no inverse steps, got {inverse:?}");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point. The payload's
/// `newOperatorKind` carries no `skip_serializing_if`, so it stays present on the wire.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: DagSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-node-operator-kind/rejects-rebinding-the-operator-of-a-missing-node: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-node-operator-kind/rejects-rebinding-the-operator-of-a-missing-node: committed mutation JSON is not canonical");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("rejected"), "change-node-operator-kind/rejects-rebinding-the-operator-of-a-missing-node declares a rejected outcome");
    let produced = <DagMutation as protocol::Mutation<DagSnapshot>>::diff(&mutation(), &before());
    let message = produced.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(serde_json::Value::as_str), Some(message.code.0.as_str()), "the declared code must match the emitted one");
    let declared_path: Vec<String> = outcome.get("path").and_then(serde_json::Value::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared_path, message.target, "the declared path must match the emitted target");
}
