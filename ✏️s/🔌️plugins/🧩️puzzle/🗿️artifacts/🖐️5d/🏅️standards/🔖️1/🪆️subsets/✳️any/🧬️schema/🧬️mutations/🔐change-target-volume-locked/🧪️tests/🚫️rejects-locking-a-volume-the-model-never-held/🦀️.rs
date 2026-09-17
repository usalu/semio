//! 🧪️ `change-target-volume-locked` fixture — `🚫️rejects-locking-a-volume-the-model-never-held`.
//!
//! Addressing a target volume the model never held is an Error-level `mutation.target-missing`.
//!
//! Contract D6: a rejected vector commits no `🔺️diff/🔣️.json` at all — the empty
//! `🔺️diff/🚫️.absent` sentinel beside this file stands in its place, so nothing here invents an
//! empty patch. Snapshot canonicality is measured through `dsl::json`, the encoder this artifact's
//! documents are written with.

use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
use crate::standards::v1::subsets::any::schema::mutations::{apply_puzzle5d_mutation, inverse_puzzle5d_mutation};
use crate::Puzzle5dSnapshot;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔐change-target-volume-locked/🚫️rejects-locking-a-volume-the-model-never-held/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔐change-target-volume-locked/🚫️rejects-locking-a-volume-the-model-never-held/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔐change-target-volume-locked/🚫️rejects-locking-a-volume-the-model-never-held/🦠️mutation/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔐change-target-volume-locked/🚫️rejects-locking-a-volume-the-model-never-held/🎯️outcome/🔣️.json");
const DIFF_ABSENT: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔐change-target-volume-locked/🚫️rejects-locking-a-volume-the-model-never-held/🔺️diff/🚫️.absent");

fn before() -> Puzzle5dSnapshot {
    dsl::json::from_json_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> Puzzle5dSnapshot {
    dsl::json::from_json_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> Puzzle5dMutation {
    dsl::json::from_json_str(MUTATION).expect("mutation decodes")
}

/// ▶️ A refused `change-target-volume-locked` still applies cleanly — its diff is the default one — and leaves the model at
/// the committed `after`, which is the committed `before`.
#[test]
fn rejection_leaves_the_document_at_the_committed_after() {
    let mut snapshot = before();
    apply_puzzle5d_mutation(&mut snapshot, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "change-target-volume-locked/rejects-locking-a-volume-the-model-never-held: applied state differs from committed after-snapshot");
    assert_eq!(expected_after(), before(), "change-target-volume-locked/rejects-locking-a-volume-the-model-never-held: a rejected vector's two committed snapshots must be identical");
}

/// 🚨️ The refusal is exactly the declared one: default diff, one message, its code, its level and its
/// target address.
#[test]
fn the_refusal_is_the_declared_one() {
    assert!(DIFF_ABSENT.is_empty(), "change-target-volume-locked/rejects-locking-a-volume-the-model-never-held: the D6 sentinel 🔺️diff/🚫️.absent must stay empty");
    let produced = <Puzzle5dMutation as protocol::Mutation<Puzzle5dSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff::default(), "change-target-volume-locked/rejects-locking-a-volume-the-model-never-held: a refusing diff builder answers the default diff, never a half-built delta");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "change-target-volume-locked/rejects-locking-a-volume-the-model-never-held: exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.target-missing", "change-target-volume-locked/rejects-locking-a-volume-the-model-never-held: the refusal code is fixed by this vector");
    assert_eq!(messages[0].level, protocol::Severity::Error, "change-target-volume-locked/rejects-locking-a-volume-the-model-never-held: the refusal level is fixed by this vector");
    assert_eq!(messages[0].target, vec!["volume-the-model-never-held".to_string()], "change-target-volume-locked/rejects-locking-a-volume-the-model-never-held: the diagnostic addresses exactly what the payload named");
}

/// ↩️ `change-target-volume-locked`'s inverse is BASE-derived: with no entry to read there is no prior flag to restore.
#[test]
fn inverse_of_a_refusal() {
    let inverse = inverse_puzzle5d_mutation(&before(), &mutation());
    assert_eq!(inverse.len(), 0, "change-target-volume-locked/rejects-locking-a-volume-the-model-never-held: got {inverse:?}");
}

/// 🔣️ Both committed snapshots and the committed `change-target-volume-locked` payload are already canonical.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Puzzle5dSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-target-volume-locked/rejects-locking-a-volume-the-model-never-held: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-target-volume-locked/rejects-locking-a-volume-the-model-never-held: committed mutation JSON is not canonical");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[test]
fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("rejected"), "change-target-volume-locked/rejects-locking-a-volume-the-model-never-held declares a rejected outcome");
    let produced = <Puzzle5dMutation as protocol::Mutation<Puzzle5dSnapshot>>::diff(&mutation(), &before());
    let message = produced.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(serde_json::Value::as_str), Some(message.code.0.as_str()), "change-target-volume-locked/rejects-locking-a-volume-the-model-never-held: the declared code must match the emitted one");
    let declared_path: Vec<String> = outcome.get("path").and_then(serde_json::Value::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared_path, message.target, "change-target-volume-locked/rejects-locking-a-volume-the-model-never-held: the declared path must match the emitted target");
}
