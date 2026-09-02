//! 🧪️ `replace-node-kind` fixture — `rejects-rekinding-a-missing-node`.
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

/// ▶️ A rejected `replace-node-kind` leaves the document byte-identical to the committed `after`.
#[semio_framework_async_macros::async_test]
async fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    let mut snapshot = base.clone();
    apply_dag_mutation(&mut snapshot, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "replace-node-kind/rejects-rekinding-a-missing-node: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.content, base.content, "a rejected kind swap must not mint a new content handle");
}

/// 🔁 `new_kind` is the whole tagged `DagNodeKind` union — every kind-specific field (a Slider's
/// bounds, a Note's text, a Computation's ports) rides inside it. A miss must reject that entire
/// payload rather than land a partial port list on the graph.
#[semio_framework_async_macros::async_test]
async fn the_whole_tagged_kind_payload_is_refused_at_once() {
    let produced = <DagMutation as protocol::Mutation<DagSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &DagDiff::default(), "a rejecting replace-node-kind must carry an empty diff — never a half-applied kind");
    assert_eq!(produced.diff().content, None, "the composed content slot must stay untouched");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.target-missing", "a missing node is reported as target-missing");
    assert_eq!(messages[0].level, protocol::Severity::Error, "this verb has no Fatal branch at all");
    assert_eq!(messages[0].target, vec!["node-a".to_string()], "the diagnostic addresses the payload's node id");
    let semantics = <DagMutation as protocol::SemanticMutation<DagSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.kind, semantics.record), ("replace", "replace-node-kind", "ReplacedNodeKind"), "the fixture must be bound to replace-node-kind's own descriptor");
}

/// ↩️ The inverse is a counter-replace carrying the OLD kind read from BASE; with no node found
/// there is no old kind, so the inverse is empty.
#[semio_framework_async_macros::async_test]
async fn inverse_has_no_old_kind_to_restore() {
    let inverse = inverse_dag_mutation(&before(), &mutation());
    assert!(inverse.is_empty(), "replace-node-kind/rejects-rekinding-a-missing-node: a rejected replace must have no inverse steps, got {inverse:?}");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point — including
/// the internally-tagged `kind` discriminant inside `newKind`.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: DagSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "replace-node-kind/rejects-rekinding-a-missing-node: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "replace-node-kind/rejects-rekinding-a-missing-node: committed mutation JSON is not canonical");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("rejected"), "replace-node-kind/rejects-rekinding-a-missing-node declares a rejected outcome");
    let produced = <DagMutation as protocol::Mutation<DagSnapshot>>::diff(&mutation(), &before());
    let message = produced.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(serde_json::Value::as_str), Some(message.code.0.as_str()), "the declared code must match the emitted one");
    let declared_path: Vec<String> = outcome.get("path").and_then(serde_json::Value::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared_path, message.target, "the declared path must match the emitted target");
}
