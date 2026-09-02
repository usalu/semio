//! 🧪️ `connect-nodes` fixture — `rejects-a-missing-source-node`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Per contract D6 a rejected case carries
//! `🔺️diff/🚫️.absent` and a `➡️after` byte-identical to `⬅️before`.
//!
//! ⚠️ `DagSnapshot` persists nodes/edges in an opaque composed `s.stdio.semio.graph` CHILD, so this
//! committed snapshot decodes to an UNRESOLVED handle and `dag_working_scene` fails soft to an
//! empty scene (`🔖️WorkingScene`) — the state this case pins.

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

/// ▶️ A rejected `connect-nodes` leaves the document byte-identical to the committed `after`.
#[semio_framework_async_macros::async_test]
async fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    let mut snapshot = base.clone();
    apply_dag_mutation(&mut snapshot, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "connect-nodes/rejects-a-missing-source-node: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.content, base.content, "a rejected connect must not mint a new content handle");
}

/// 🔗 Endpoints are `"<nodeId>@<portId>"` strings. The reported target is the SPLIT node id
/// (`node-a`), never the raw endpoint (`node-a@out`) and never the edge id — this verb is the only
/// one in the vocabulary whose diagnostic address is derived from a payload field rather than
/// carried by it.
#[semio_framework_async_macros::async_test]
async fn the_reported_target_is_the_split_source_node_not_the_endpoint() {
    let produced = <DagMutation as protocol::Mutation<DagSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &DagDiff::default(), "a rejecting connect-nodes must carry an empty diff");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected — the source is checked before the target node, so only one miss is reported, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.target-missing", "a missing endpoint node is reported as target-missing");
    assert_eq!(messages[0].level, protocol::Severity::Error, "a missing endpoint is an Error; the self-loop, cycle and duplicate-edge-id branches are the Fatal ones");
    assert_eq!(messages[0].target, vec!["node-a".to_string()], "the diagnostic names the SOURCE node id split out of \"node-a@out\"");
    assert_ne!(messages[0].target, vec!["edge-1".to_string()], "the edge id is what this verb CREATES, so it is never the missing target");
    let semantics = <DagMutation as protocol::SemanticMutation<DagSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("connect", "nodes", "connect-nodes", "ConnectedNodes"), "the fixture must be bound to connect-nodes' own descriptor");
}

/// ↩️ `connect-nodes` inverts from its own payload, not from BASE: the undo is a `disconnect-nodes`
/// of the edge id it was asked to create, even though the connect was refused.
#[semio_framework_async_macros::async_test]
async fn inverse_is_always_a_disconnect_of_the_requested_edge() {
    let inverse = inverse_dag_mutation(&before(), &mutation());
    assert_eq!(inverse.len(), 1, "connect-nodes always undoes with exactly one step, got {inverse:?}");
    let DagMutation::DisconnectNodes(undo) = &inverse[0] else {
        panic!("connect-nodes' inverse must be a disconnect-nodes, got {:?}", inverse[0]);
    };
    assert_eq!(undo.id, "edge-1", "the inverse disconnects exactly the edge id the payload carried");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point — including
/// the `routeStyle` scalar's camelCase variant name.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: DagSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "connect-nodes/rejects-a-missing-source-node: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "connect-nodes/rejects-a-missing-source-node: committed mutation JSON is not canonical");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("rejected"), "connect-nodes/rejects-a-missing-source-node declares a rejected outcome");
    let produced = <DagMutation as protocol::Mutation<DagSnapshot>>::diff(&mutation(), &before());
    let message = produced.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(serde_json::Value::as_str), Some(message.code.0.as_str()), "the declared code must match the emitted one");
    let declared_path: Vec<String> = outcome.get("path").and_then(serde_json::Value::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared_path, message.target, "the declared path must match the emitted target");
}
