//! 🧪️ `create-target-volume` fixture — `🚫️rejects-a-volume-id-the-model-already-holds`.
//!
//! Re-painting a volume id the model already holds is a FATAL `mutation.duplicate-id`, not the Error-level miss the addressing verbs raise.
//!
//! Contract D6: a rejected vector commits no `🔺️diff/🔣️.json` at all — the empty
//! `🔺️diff/🚫️.absent` sentinel beside this file stands in its place, so nothing here invents an
//! empty patch. Snapshot canonicality is measured through `dsl::json`, the encoder this artifact's
//! documents are written with.

use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
use crate::standards::v1::subsets::any::schema::mutations::{apply_puzzle5d_mutation, inverse_puzzle5d_mutation};
use crate::Puzzle5dSnapshot;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🌍create-target-volume/🚫️rejects-a-volume-id-the-model-already-holds/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🌍create-target-volume/🚫️rejects-a-volume-id-the-model-already-holds/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🌍create-target-volume/🚫️rejects-a-volume-id-the-model-already-holds/🦠️mutation/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🌍create-target-volume/🚫️rejects-a-volume-id-the-model-already-holds/🎯️outcome/🔣️.json");
const DIFF_ABSENT: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🌍create-target-volume/🚫️rejects-a-volume-id-the-model-already-holds/🔺️diff/🚫️.absent");

fn before() -> Puzzle5dSnapshot {
    dsl::json::from_json_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> Puzzle5dSnapshot {
    dsl::json::from_json_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> Puzzle5dMutation {
    dsl::json::from_json_str(MUTATION).expect("mutation decodes")
}

/// ▶️ A refused `create-target-volume` still applies cleanly — its diff is the default one — and leaves the model at
/// the committed `after`, which is the committed `before`.
#[test]
fn rejection_leaves_the_document_at_the_committed_after() {
    let mut snapshot = before();
    apply_puzzle5d_mutation(&mut snapshot, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "create-target-volume/rejects-a-volume-id-the-model-already-holds: applied state differs from committed after-snapshot");
    assert_eq!(expected_after(), before(), "create-target-volume/rejects-a-volume-id-the-model-already-holds: a rejected vector's two committed snapshots must be identical");
}

/// 🚨️ The refusal is exactly the declared one: default diff, one message, its code, its level and its
/// target address.
#[test]
fn the_refusal_is_the_declared_one() {
    assert!(DIFF_ABSENT.is_empty(), "create-target-volume/rejects-a-volume-id-the-model-already-holds: the D6 sentinel 🔺️diff/🚫️.absent must stay empty");
    let produced = <Puzzle5dMutation as protocol::Mutation<Puzzle5dSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff::default(), "create-target-volume/rejects-a-volume-id-the-model-already-holds: a refusing diff builder answers the default diff, never a half-built delta");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "create-target-volume/rejects-a-volume-id-the-model-already-holds: exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.duplicate-id", "create-target-volume/rejects-a-volume-id-the-model-already-holds: the refusal code is fixed by this vector");
    assert_eq!(messages[0].level, protocol::Severity::Fatal, "create-target-volume/rejects-a-volume-id-the-model-already-holds: the refusal level is fixed by this vector");
    assert_eq!(messages[0].target, vec!["volume-1".to_string()], "create-target-volume/rejects-a-volume-id-the-model-already-holds: the diagnostic addresses exactly what the payload named");
}

/// ↩️ `create-target-volume`'s inverse is PAYLOAD-derived, so a refused create still answers a `delete-target-volume` of the id it was asked to create.
#[test]
fn inverse_of_a_refusal() {
    let inverse = inverse_puzzle5d_mutation(&before(), &mutation());
    assert_eq!(inverse.len(), 1, "create-target-volume/rejects-a-volume-id-the-model-already-holds: got {inverse:?}");
}

/// 🔣️ Both committed snapshots and the committed `create-target-volume` payload are already canonical.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Puzzle5dSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-target-volume/rejects-a-volume-id-the-model-already-holds: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "create-target-volume/rejects-a-volume-id-the-model-already-holds: committed mutation JSON is not canonical");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[test]
fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("rejected"), "create-target-volume/rejects-a-volume-id-the-model-already-holds declares a rejected outcome");
    let produced = <Puzzle5dMutation as protocol::Mutation<Puzzle5dSnapshot>>::diff(&mutation(), &before());
    let message = produced.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(serde_json::Value::as_str), Some(message.code.0.as_str()), "create-target-volume/rejects-a-volume-id-the-model-already-holds: the declared code must match the emitted one");
    let declared_path: Vec<String> = outcome.get("path").and_then(serde_json::Value::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared_path, message.target, "create-target-volume/rejects-a-volume-id-the-model-already-holds: the declared path must match the emitted target");
}
