//! 🧪️ `set-slider` fixture — `🧪️rejects` (scrubbing a slider the scene does not hold).
//!
//! Source of truth is the committed JSON quartet beside this file. A rejected case carries `🔺️diff/🚫️.absent` and a
//! `➡️after` byte-identical to `⬅️before`. The committed snapshot holds an UNRESOLVED content handle, so
//! `dag_working_scene` fails soft to an empty scene and the slider is missing.

use crate::mutations::{apply_dag_mutation, inverse_dag_mutation, set_slider, DagMutation, DagSliderField};
use crate::{DagDiff, DagSnapshot};

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🎚️set-slider/🧪️rejects/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🎚️set-slider/🧪️rejects/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🎚️set-slider/🧪️rejects/🦠️mutation/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🎚️set-slider/🧪️rejects/🎯️outcome/🔣️.json");

fn before() -> DagSnapshot {
    dsl::json::from_json_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> DagSnapshot {
    dsl::json::from_json_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> DagMutation {
    dsl::json::from_json_str(MUTATION).expect("mutation decodes")
}

/// ▶️ A rejected `set-slider` leaves the document byte-identical to the committed `after`, content handle included.
#[semio_framework_async_macros::async_test]
async fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    let mut snapshot = base.clone();
    apply_dag_mutation(&mut snapshot, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "set-slider/rejects: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.content, base.content, "a rejected scrub must not mint a new content handle");
}

/// 🎚️ The slider does not exist: ONE Error `mutation.target-missing` naming it and an empty diff; a non-finite number
/// is the payload-intrinsic Fatal `mutation.invariant` and wins over the lookup.
#[semio_framework_async_macros::async_test]
async fn scrubbing_a_missing_slider_is_one_target_missing_error() {
    let produced = <DagMutation as protocol::Mutation<DagSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &DagDiff::default(), "a rejecting set-slider must carry an empty diff");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.target-missing");
    assert_eq!(messages[0].level, protocol::Severity::Error);
    assert_eq!(messages[0].target, vec!["node-a".to_string()]);
    let not_finite = <DagMutation as protocol::Mutation<DagSnapshot>>::diff(&set_slider("node-a".into(), DagSliderField::Value, f64::INFINITY), &before());
    assert_eq!(not_finite.messages()[0].code.0, "mutation.invariant");
    let semantics = <DagMutation as protocol::SemanticMutation<DagSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.kind, semantics.verb), ("set-slider", "set"));
}

/// ↩️ No slider held an old number, so there is nothing to put back.
#[semio_framework_async_macros::async_test]
async fn inverse_has_no_old_number_to_restore() {
    let inverse = inverse_dag_mutation(&before(), &mutation());
    assert!(inverse.is_empty(), "set-slider/rejects: a rejected scrub must have no inverse steps, got {inverse:?}");
}

/// 🔣️ The committed snapshots and payload are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: DagSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&decoded)).expect("snapshot encodes");
        assert_eq!(reencoded, serde_json::from_str::<serde_json::Value>(text).expect("snapshot reparses"), "set-slider/rejects: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&mutation())).expect("mutation encodes");
    assert_eq!(reencoded, serde_json::from_str::<serde_json::Value>(MUTATION).expect("mutation reparses"), "set-slider/rejects: committed mutation JSON is not canonical");
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
