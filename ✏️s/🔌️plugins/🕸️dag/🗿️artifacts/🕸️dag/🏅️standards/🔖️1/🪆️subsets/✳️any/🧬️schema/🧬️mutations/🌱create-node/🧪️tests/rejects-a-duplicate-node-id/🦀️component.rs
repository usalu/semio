//! 🧪️ `create-node` fixture — `rejects-a-duplicate-node-id`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Per contract D6 a rejected case carries
//! `🔺️diff/🚫️component.absent` and a `➡️after` byte-identical to `⬅️before`.
//!
//! ⚠️ Why every dag case pins a REJECTION branch: `DagSnapshot` persists nodes/edges in an opaque
//! composed `s.stdio.semio.graph` CHILD (`🔖️WorkingScene`), so a committed snapshot JSON carries a
//! handle, never a graph — and every APPLIED dag diff mints a fresh handle whose `child_id` is a
//! `DefaultHasher` digest of the child content. Hand-authoring such an `➡️after` would mean
//! hand-forging a value from `std`'s deliberately unspecified default hasher. The rejection
//! branches reach no hash at all, so those are what this tree pins until child resolution lands.
//!
//! 🌱 `create-node` is the one dag verb with no rejection on an EMPTY scene, so this case seeds the
//! working-scene cache for the committed handle with the very node the committed mutation payload
//! carries — the collision the `mutation.duplicate-id` Fatal guards against.

use crate::artifacts::dag::mutations::{apply_dag_mutation, inverse_dag_mutation, DagMutation};
use crate::artifacts::dag::{cache_dag_content, DagDiff, DagSnapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn mutation() -> DagMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}
fn expected_after() -> DagSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}

/// 🌱 The committed `⬅️before`, with its composed child resolved to a scene holding exactly the
/// node the committed payload tries to create. Nothing here is invented: the seeded node IS the
/// mutation JSON's own `node`.
fn before() -> DagSnapshot {
    let snapshot: DagSnapshot = serde_json::from_str(BEFORE).expect("before snapshot decodes");
    let DagMutation::CreateNode(payload) = mutation() else {
        panic!("rejects-a-duplicate-node-id's committed mutation must be a create-node");
    };
    cache_dag_content(&snapshot.content.child_id, vec![payload.node.clone()], Vec::new());
    snapshot
}

/// ▶️ A rejected `create-node` leaves the document byte-identical to the committed `after`.
#[semio_framework_async_macros::async_test]
async fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    let mut snapshot = base.clone();
    apply_dag_mutation(&mut snapshot, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "create-node/rejects-a-duplicate-node-id: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.content, base.content, "a rejected create must not mint a new content handle");
}

/// 🚨️ A colliding node id is a FATAL `mutation.duplicate-id`, not the Error-level `target-missing`
/// the rest of this vocabulary uses — a duplicate identity is an invariant breach, not a miss.
#[semio_framework_async_macros::async_test]
async fn a_colliding_id_is_a_fatal_duplicate_id() {
    let produced = <DagMutation as protocol::Mutation<DagSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &DagDiff::default(), "a rejecting create-node must carry an empty diff, never a half-built content handle");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.duplicate-id", "an id collision is reported as duplicate-id");
    assert_eq!(messages[0].level, protocol::Severity::Fatal, "duplicate-id is Fatal — no merge policy may absorb it");
    assert_eq!(messages[0].target, vec!["node-a".to_string()], "the diagnostic addresses the colliding node id");
    let semantics = <DagMutation as protocol::SemanticMutation<DagSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("create", "node", "create-node", "CreatedNode"), "the fixture must be bound to create-node's own descriptor");
}

/// ↩️ `create-node`'s inverse is payload-derived, not BASE-derived: it is a `delete-node` of the id
/// it was asked to create even when the create itself was refused.
#[semio_framework_async_macros::async_test]
async fn inverse_is_always_a_delete_of_the_requested_id() {
    let inverse = inverse_dag_mutation(&before(), &mutation());
    assert_eq!(inverse.len(), 1, "create-node always undoes with exactly one step, got {inverse:?}");
    let DagMutation::DeleteNode(undo) = &inverse[0] else {
        panic!("create-node's inverse must be a delete-node, got {:?}", inverse[0]);
    };
    assert_eq!(undo.id, "node-a", "the inverse deletes exactly the id the payload carried");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: DagSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-node/rejects-a-duplicate-node-id: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "create-node/rejects-a-duplicate-node-id: committed mutation JSON is not canonical");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("rejected"), "create-node/rejects-a-duplicate-node-id declares a rejected outcome");
    let produced = <DagMutation as protocol::Mutation<DagSnapshot>>::diff(&mutation(), &before());
    let message = produced.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(serde_json::Value::as_str), Some(message.code.0.as_str()), "the declared code must match the emitted one");
    let declared_path: Vec<String> = outcome.get("path").and_then(serde_json::Value::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared_path, message.target, "the declared path must match the emitted target");
}
