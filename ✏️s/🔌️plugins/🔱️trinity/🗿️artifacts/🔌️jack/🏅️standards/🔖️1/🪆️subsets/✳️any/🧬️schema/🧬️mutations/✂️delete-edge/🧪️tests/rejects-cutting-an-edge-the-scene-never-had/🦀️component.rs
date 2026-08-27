//! 🧪️ `delete-edge` fixture — `rejects-cutting-an-edge-the-scene-never-had`.
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
//! deliberately unspecified default hasher. `delete-edge` has no `mutation.no-op` guard, so its `mutation.target-missing` Error
//! is the only branch that reaches no hash — that is what this case pins.
//!
//! ✂️ `delete-edge` is the only verb in this vocabulary addressed by EDGE id: it searches
//! `scene.edges` and never `scene.nodes`, so the reported target is the edge id verbatim and no node
//! is ever consulted. Unlike its `delete-node` sibling it has no cascade at all.

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

/// 🕳️ The committed `⬅️before`, whose composed child is never resolved — an edgeless scene.
fn before() -> JackSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}

/// ▶️ A rejected `delete-edge` leaves the document byte-identical to the committed `after` — in
/// particular the composed content handle is the one the committed `⬅️before` names, not a re-mint.
#[semio_framework_async_macros::async_test]
async fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    let mut snapshot = base.clone();
    apply_trinity_graph_mutation(&mut snapshot, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "delete-edge/rejects-cutting-an-edge-the-scene-never-had: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.content.child_id, base.content.child_id, "a rejected delete-edge must not mint a new content handle");
}

/// 🚫️ An edge that is not in the scene is an Error-level `mutation.target-missing` naming the EDGE id.
/// `delete-edge` never looks at `scene.nodes`, so no endpoint id can ever appear in its diagnostic.
#[semio_framework_async_macros::async_test]
async fn a_missing_edge_is_reported_by_its_edge_id() {
    let base = before();
    assert!(jack_working_scene(&base).edges.is_empty(), "rejects-cutting-an-edge-the-scene-never-had's before-snapshot must decode to an unresolved, edgeless scene");
    let produced = <TrinityGraphMutation as protocol::Mutation<JackSnapshot>>::diff(&mutation(), &base);
    assert_eq!(produced.diff(), &JackDiff::default(), "a rejecting delete-edge must carry an empty diff, never a half-built content handle");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.target-missing", "an edge the scene does not hold is reported as target-missing");
    assert_eq!(messages[0].level, protocol::Severity::Error, "a missing cut target is an Error — this verb has no Fatal branch at all");
    assert_eq!(messages[0].target, vec!["shaft-to-capsule-a".to_string()], "the diagnostic names the EDGE id, not either endpoint node");
    let semantics = <TrinityGraphMutation as protocol::SemanticMutation<JackSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("delete", "edge", "delete-edge", "DeletedEdge"), "the fixture must be bound to delete-edge's own descriptor");
}

/// ↩️ `delete-edge` inverts BASE-derived: it reconstructs the exact edge BASE held — id, kind, both
/// port-qualified endpoints and properties. With no such edge captured, the inverse is empty.
#[semio_framework_async_macros::async_test]
async fn inverse_has_no_edge_to_reconnect() {
    let inverse = inverse_trinity_graph_mutation(&before(), &mutation());
    assert!(inverse.is_empty(), "delete-edge/rejects-cutting-an-edge-the-scene-never-had: a rejected cut must have no inverse steps, got {inverse:?}");
}

/// 🔣️ Both committed snapshots and the committed `delete-edge` payload are already canonical:
/// decode→encode is a fixed point. `JackSnapshot` skips `manifest_id`/`root_node_id` when they are
/// `None`, so their absence here is canonical, not an omission.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: JackSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-edge/rejects-cutting-an-edge-the-scene-never-had: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "delete-edge/rejects-cutting-an-edge-the-scene-never-had: committed mutation JSON is not canonical");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what `delete-edge`'s own diff builder
/// emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("rejected"), "delete-edge/rejects-cutting-an-edge-the-scene-never-had declares a rejected outcome");
    let produced = <TrinityGraphMutation as protocol::Mutation<JackSnapshot>>::diff(&mutation(), &before());
    let message = produced.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(serde_json::Value::as_str), Some(message.code.0.as_str()), "the declared code must match the emitted one");
    let declared_path: Vec<String> = outcome.get("path").and_then(serde_json::Value::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared_path, message.target, "the declared path must match the emitted target");
}
