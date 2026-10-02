//! 🧪️ `move-nodes` fixture — `🧪️rejects` (a drag of nodes the scene does not hold).
//!
//! Source of truth is the committed JSON quartet beside this file. A rejected case carries `🔺️diff/🚫️.absent` and a
//! `➡️after` byte-identical to `⬅️before`. The committed snapshot holds an UNRESOLVED content handle, so
//! `dag_working_scene` fails soft to an empty scene and every addressed node is missing.

use crate::mutations::{apply_dag_mutation, inverse_dag_mutation, move_nodes, DagMutation};
use crate::{DagDiff, DagSnapshot};

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🚚️move-nodes/🧪️rejects/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🚚️move-nodes/🧪️rejects/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🚚️move-nodes/🧪️rejects/🦠️mutation/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🚚️move-nodes/🧪️rejects/🎯️outcome/🔣️.json");

fn before() -> DagSnapshot {
    dsl::json::from_json_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> DagSnapshot {
    dsl::json::from_json_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> DagMutation {
    dsl::json::from_json_str(MUTATION).expect("mutation decodes")
}

/// ▶️ A rejected `move-nodes` leaves the document byte-identical to the committed `after`, content handle included.
#[semio_framework_async_macros::async_test]
async fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    let mut snapshot = base.clone();
    apply_dag_mutation(&mut snapshot, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "move-nodes/rejects: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.content, base.content, "a rejected drag must not mint a new content handle");
}

/// 🚚️ None of the dragged nodes exists: ONE Error `mutation.target-missing` naming every id in payload order, an empty
/// diff, and the target lookup outranks the finite-offset invariant only for well-formed id lists.
#[semio_framework_async_macros::async_test]
async fn a_drag_of_missing_nodes_is_one_target_missing_error() {
    let produced = <DagMutation as protocol::Mutation<DagSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &DagDiff::default(), "a rejecting move-nodes must carry an empty diff");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.target-missing");
    assert_eq!(messages[0].level, protocol::Severity::Error, "missing drag targets are recoverable (edit or withdraw), never Fatal");
    assert_eq!(messages[0].target, vec!["node-a".to_string(), "node-b".to_string()]);
    let not_finite = <DagMutation as protocol::Mutation<DagSnapshot>>::diff(&move_nodes(vec!["node-a".into()], f64::NAN, 0.0), &before());
    assert_eq!(not_finite.messages()[0].code.0, "mutation.invariant", "a non-finite offset is payload-intrinsic and wins over the lookup");
    let semantics = <DagMutation as protocol::SemanticMutation<DagSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.kind, semantics.record), ("move", "move-nodes", "MovedNodes"));
}

/// ↩️ No addressed node had a base position, so there is nothing to put back.
#[semio_framework_async_macros::async_test]
async fn inverse_has_no_base_position_to_restore() {
    let inverse = inverse_dag_mutation(&before(), &mutation());
    assert!(inverse.is_empty(), "move-nodes/rejects: a rejected drag must have no inverse steps, got {inverse:?}");
}

/// 🔣️ The committed snapshots and payload are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: DagSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&decoded)).expect("snapshot encodes");
        assert_eq!(reencoded, serde_json::from_str::<serde_json::Value>(text).expect("snapshot reparses"), "move-nodes/rejects: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&mutation())).expect("mutation encodes");
    assert_eq!(reencoded, serde_json::from_str::<serde_json::Value>(MUTATION).expect("mutation reparses"), "move-nodes/rejects: committed mutation JSON is not canonical");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("rejected"));
    let produced = <DagMutation as protocol::Mutation<DagSnapshot>>::diff(&mutation(), &before());
    let message = produced.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(serde_json::Value::as_str), Some(message.code.0.as_str()));
    let declared_path: Vec<String> = outcome.get("path").and_then(serde_json::Value::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared_path, message.target);
}
