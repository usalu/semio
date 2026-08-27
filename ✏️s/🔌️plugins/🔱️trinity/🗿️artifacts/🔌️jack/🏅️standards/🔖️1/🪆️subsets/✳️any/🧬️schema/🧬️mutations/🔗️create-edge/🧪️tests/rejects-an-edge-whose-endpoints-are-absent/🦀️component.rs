//! 🧪️ `create-edge` fixture — `rejects-an-edge-whose-endpoints-are-absent`.
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
//! deliberately unspecified default hasher. `create-edge` has no `mutation.no-op` guard, and of its two rejection branches this case
//! pins the SECOND — `mutation.invariant` for absent endpoints, the one its `create-node` sibling has
//! no analogue of.
//!
//! 🔗️ `create-edge` is the only verb in this vocabulary that validates REFERENTIAL integrity inside
//! its own diff builder: it checks the duplicate-id branch first, then resolves both port-qualified
//! endpoints (`nodeId@portId`) against the scene's nodes. On an unresolved, nodeless scene the
//! duplicate check passes vacuously and the endpoint check is what fires.

use crate::artifacts::jack::mutations::TrinityGraphMutation;
use crate::artifacts::jack::{apply_trinity_graph_mutation, inverse_trinity_graph_mutation};
use crate::artifacts::jack::{jack_working_scene, JackDiff, JackSnapshot};

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

/// 🕳️ The committed `⬅️before`, whose composed child is never resolved — an empty scene, so the
/// duplicate-id branch passes vacuously and the endpoint check is the one that rejects.
fn before() -> JackSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}

/// ▶️ A rejected `create-edge` leaves the document byte-identical to the committed `after` — in
/// particular the composed content handle is the one the committed `⬅️before` names, not a re-mint.
#[semio_framework_async_macros::async_test]
async fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    let mut snapshot = base.clone();
    apply_trinity_graph_mutation(&mut snapshot, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "create-edge/rejects-an-edge-whose-endpoints-are-absent: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.content.child_id, base.content.child_id, "a rejected create-edge must not mint a new content handle");
}

/// 🚨️ Endpoints that resolve to no node are a FATAL `mutation.invariant` addressed by the EDGE id —
/// not `mutation.target-missing`, and not addressed by either endpoint's node id.
#[semio_framework_async_macros::async_test]
async fn absent_endpoints_are_a_fatal_invariant_not_a_missing_target() {
    let base = before();
    let scene = jack_working_scene(&base);
    assert!(scene.nodes.is_empty() && scene.edges.is_empty(), "rejects-an-edge-whose-endpoints-are-absent's before-snapshot must decode to an unresolved, empty scene");
    let produced = <TrinityGraphMutation as protocol::Mutation<JackSnapshot>>::diff(&mutation(), &base);
    assert_eq!(produced.diff(), &JackDiff::default(), "a rejecting create-edge must carry an empty diff, never a half-built content handle");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.invariant", "a dangling endpoint is an invariant breach, not a missing target");
    assert_eq!(messages[0].level, protocol::Severity::Fatal, "mutation.invariant is Fatal here — an edge to nowhere would corrupt the graph, so it can never be absorbed");
    assert_eq!(messages[0].target, vec!["shaft-to-capsule-a".to_string()], "the diagnostic names the EDGE id, never `shaft` or `capsule-a` — the absent endpoints are the reason, not the address");
    let semantics = <TrinityGraphMutation as protocol::SemanticMutation<JackSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("create", "edge", "create-edge", "CreatedEdge"), "the fixture must be bound to create-edge's own descriptor");
}

/// ↩️ `create-edge`'s inverse is PAYLOAD-derived like its `create-node` sibling's: a `delete-edge` of the
/// id it was asked to create, emitted even though the create was refused — unlike this vocabulary's
/// delete verbs, whose BASE-derived inverses collapse to `Vec::new()` on a rejection.
#[semio_framework_async_macros::async_test]
async fn inverse_is_always_a_delete_of_the_requested_edge() {
    let inverse = inverse_trinity_graph_mutation(&before(), &mutation());
    assert_eq!(inverse.len(), 1, "create-edge always undoes with exactly one step even when refused, got {inverse:?}");
    let TrinityGraphMutation::DeleteEdge(undo) = &inverse[0] else {
        panic!("create-edge's inverse must be a delete-edge, got {:?}", inverse[0]);
    };
    assert_eq!(undo.id, "shaft-to-capsule-a", "the inverse deletes exactly the edge id the payload carried, not anything read off BASE");
}

/// 🔣️ Both committed snapshots and the committed `create-edge` payload are already canonical:
/// decode→encode is a fixed point. `JackSnapshot` skips `manifest_id`/`root_node_id` when they are
/// `None`, so their absence here is canonical, not an omission.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: JackSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-edge/rejects-an-edge-whose-endpoints-are-absent: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "create-edge/rejects-an-edge-whose-endpoints-are-absent: committed mutation JSON is not canonical");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what `create-edge`'s own diff builder
/// emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("rejected"), "create-edge/rejects-an-edge-whose-endpoints-are-absent declares a rejected outcome");
    let produced = <TrinityGraphMutation as protocol::Mutation<JackSnapshot>>::diff(&mutation(), &before());
    let message = produced.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(serde_json::Value::as_str), Some(message.code.0.as_str()), "the declared code must match the emitted one");
    let declared_path: Vec<String> = outcome.get("path").and_then(serde_json::Value::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared_path, message.target, "the declared path must match the emitted target");
}
