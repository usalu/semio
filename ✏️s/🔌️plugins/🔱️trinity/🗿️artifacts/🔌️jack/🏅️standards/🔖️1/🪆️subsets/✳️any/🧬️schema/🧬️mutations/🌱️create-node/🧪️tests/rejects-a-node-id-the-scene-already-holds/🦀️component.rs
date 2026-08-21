//! 🧪️ `create-node` fixture — `rejects-a-node-id-the-scene-already-holds`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`).
//!
//! ⚠️ Why no jack case can pin a state-CHANGING branch: `JackSnapshot` keeps `schema`/`name`/
//! `manifest`/`camera`/`root_node_id` inline but persists its nodes and edges in an opaque composed
//! `s.stdio.semio.graph` CHILD (`🔖️ContentBridge`/`🔖️WorkingScene`), and every one of this
//! vocabulary's eight diff builders funnels its changed scene through `diff_replace_content`, which
//! mints a fresh handle whose `child_id` is a `std::collections::hash_map::DefaultHasher` digest of
//! the child content. Hand-authoring such an `➡️after` would mean hand-forging a value from `std`'s
//! deliberately unspecified default hasher. `create-node` is the one verb here with no `mutation.no-op` guard at all, so its
//! `mutation.duplicate-id` Fatal is the only branch that reaches no hash — that is what this case pins.
//!
//! 🌱️ Following `dag`'s precedent, this case SEEDS the working-scene cache for the committed handle
//! with the very node the committed mutation payload carries, so the id collision the Fatal guards
//! against is reachable. Nothing here is invented: the seeded node IS the mutation JSON's own `node`.

use crate::artifacts::jack::mutations::{apply_trinity_graph_mutation, inverse_trinity_graph_mutation, TrinityGraphMutation};
use crate::artifacts::jack::{jack_working_scene, JackDiff, JackSnapshot, cache_jack_content};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn expected_after() -> JackSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> TrinityGraphMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// 🌱️ The committed `⬅️before`, with its composed child resolved to a scene holding exactly the node
/// the committed payload tries to create.
fn before() -> JackSnapshot {
    let snapshot: JackSnapshot = serde_json::from_str(BEFORE).expect("before snapshot decodes");
    let TrinityGraphMutation::CreateNode(payload) = mutation() else {
        panic!("rejects-a-node-id-the-scene-already-holds's committed mutation must be a create-node");
    };
    cache_jack_content(&snapshot.content.child_id, vec![payload.node.clone()], Vec::new());
    snapshot
}

/// ▶️ A rejected `create-node` leaves the document byte-identical to the committed `after` — in
/// particular the composed content handle is the one the committed `⬅️before` names, not a re-mint.
#[semio_framework_async_macros::async_test]
async fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    let mut snapshot = base.clone();
    apply_trinity_graph_mutation(&mut snapshot, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "create-node/rejects-a-node-id-the-scene-already-holds: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.content.child_id, base.content.child_id, "a rejected create-node must not mint a new content handle");
}

/// 🚨️ A colliding node id is a FATAL `mutation.duplicate-id` — an invariant breach no merge policy
/// may absorb — not the Error-level `mutation.target-missing` this vocabulary's delete/rename/move
/// verbs raise for a miss.
#[semio_framework_async_macros::async_test]
async fn a_colliding_node_id_is_a_fatal_duplicate_id() {
    let base = before();
    assert_eq!(jack_working_scene(&base).nodes.len(), 1, "the seeded scene must already hold exactly the node the payload wants to create");
    let produced = <TrinityGraphMutation as protocol::Mutation<JackSnapshot>>::diff(&mutation(), &base);
    assert_eq!(produced.diff(), &JackDiff::default(), "a rejecting create-node must carry an empty diff, never a half-built content handle");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.duplicate-id", "an id collision is reported as duplicate-id, never as target-missing");
    assert_eq!(messages[0].level, protocol::Severity::Fatal, "duplicate-id is Fatal — a duplicate identity is an invariant breach, not a miss");
    assert_eq!(messages[0].target, vec!["shaft".to_string()], "the diagnostic addresses the colliding NODE id the payload carried");
    let semantics = <TrinityGraphMutation as protocol::SemanticMutation<JackSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("create", "node", "create-node", "CreatedNode"), "the fixture must be bound to create-node's own descriptor");
}

/// ↩️ `create-node`'s inverse is PAYLOAD-derived, not BASE-derived: it is a `delete-node` of the id it
/// was asked to create, emitted even when the create itself was refused.
#[semio_framework_async_macros::async_test]
async fn inverse_is_always_a_delete_of_the_requested_id() {
    let inverse = inverse_trinity_graph_mutation(&before(), &mutation());
    assert_eq!(inverse.len(), 1, "create-node always undoes with exactly one step, got {inverse:?}");
    let TrinityGraphMutation::DeleteNode(undo) = &inverse[0] else {
        panic!("create-node's inverse must be a delete-node, got {:?}", inverse[0]);
    };
    assert_eq!(undo.id, "shaft", "the inverse deletes exactly the id the payload carried, not anything read off BASE");
}

/// 🔣️ Both committed snapshots and the committed `create-node` payload are already canonical:
/// decode→encode is a fixed point. `JackSnapshot` skips `manifest_id`/`root_node_id` when they are
/// `None`, so their absence here is canonical, not an omission.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: JackSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-node/rejects-a-node-id-the-scene-already-holds: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "create-node/rejects-a-node-id-the-scene-already-holds: committed mutation JSON is not canonical");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what `create-node`'s own diff builder
/// emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("rejected"), "create-node/rejects-a-node-id-the-scene-already-holds declares a rejected outcome");
    let produced = <TrinityGraphMutation as protocol::Mutation<JackSnapshot>>::diff(&mutation(), &before());
    let message = produced.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(serde_json::Value::as_str), Some(message.code.0.as_str()), "the declared code must match the emitted one");
    let declared_path: Vec<String> = outcome.get("path").and_then(serde_json::Value::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared_path, message.target, "the declared path must match the emitted target");
}
