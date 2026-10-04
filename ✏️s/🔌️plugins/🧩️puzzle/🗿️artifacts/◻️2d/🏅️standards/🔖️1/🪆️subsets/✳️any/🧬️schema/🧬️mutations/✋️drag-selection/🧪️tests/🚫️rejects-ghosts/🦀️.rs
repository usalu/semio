//! 🧪️ `drag-selection` fixture — `🚫️rejects-ghosts`.
//!
//! Every target is absent: Error-level `mutation.target-missing`, nothing moves.
//!
//! Source of truth is the committed JSON quintet under `🧫️fixtures/🧬️mutations/✋️drag-selection/🚫️rejects-ghosts/`
//! (contract D1); the board is the synthetic selection board shared by every selection-transform vector.

use crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
use crate::standards::v1::subsets::any::schema::mutations::{apply_puzzle2d_mutation, inverse_puzzle2d_mutation};
use crate::Puzzle2dSnapshot;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✋️drag-selection/🚫️rejects-ghosts/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✋️drag-selection/🚫️rejects-ghosts/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✋️drag-selection/🚫️rejects-ghosts/🦠️mutation/🔣️.json");
const DIFF_ABSENT: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✋️drag-selection/🚫️rejects-ghosts/🔺️diff/🚫️.absent");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✋️drag-selection/🚫️rejects-ghosts/🎯️outcome/🔣️.json");

fn before() -> Puzzle2dSnapshot {
    semio_framework_pack_json::from_json_str(BEFORE, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before snapshot decodes")
}
fn expected_after() -> Puzzle2dSnapshot {
    semio_framework_pack_json::from_json_str(AFTER, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("after snapshot decodes")
}
fn mutation() -> Puzzle2dMutation {
    semio_framework_pack_json::from_json_str(MUTATION, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation decodes")
}
fn outcome() -> serde_json::Value {
    serde_json::from_str(OUTCOME).expect("outcome decodes")
}

/// 🗣️ `(level, code, target)` of every message `drag-selection` raises on the committed base.
fn produced_messages() -> Vec<(semio_framework_diagnostic::Severity, String, Vec<String>)> {
    let produced = <Puzzle2dMutation as protocol::Mutation<Puzzle2dSnapshot>>::diff(&mutation(), &before());
    produced.messages().iter().map(|message| (message.level, message.code.0.clone(), message.target.clone())).collect()
}

/// 🔣️ Both committed snapshots and the committed `drag-selection` payload are already canonical.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Puzzle2dSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "drag-selection/rejects-ghosts: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&mutation())).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "drag-selection/rejects-ghosts: committed mutation JSON is not canonical");
}

/// ▶️ A refused `drag-selection` still applies cleanly — its diff is the default one — and leaves the board at
/// the committed `after`, which is the committed `before`.
#[test]
fn rejection_leaves_the_document_at_the_committed_after() {
    let mut snapshot = before();
    apply_puzzle2d_mutation(&mut snapshot, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "drag-selection/rejects-ghosts: applied state differs from committed after-snapshot");
    assert_eq!(expected_after(), before(), "drag-selection/rejects-ghosts: a rejected vector's two committed snapshots must be identical");
}

/// 🚨️ The refusal is exactly the declared one: default diff, one Error-level message, its code and
/// every target the payload named.
#[test]
fn the_refusal_is_the_declared_one() {
    assert!(DIFF_ABSENT.is_empty(), "drag-selection/rejects-ghosts: the D6 sentinel 🔺️diff/🚫️.absent must stay empty");
    let produced = <Puzzle2dMutation as protocol::Mutation<Puzzle2dSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &Puzzle2dDiff::default(), "drag-selection/rejects-ghosts: a refusing diff builder answers the default diff");
    let outcome = outcome();
    assert_eq!(outcome["status"].as_str(), Some("rejected"), "drag-selection/rejects-ghosts declares a rejected outcome");
    let path: Vec<String> = outcome["path"].as_array().expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(produced_messages(), vec![(semio_framework_diagnostic::Severity::Error, outcome["code"].as_str().expect("a code").to_string(), path)], "drag-selection/rejects-ghosts: the refusal differs from the declared one");
}

/// ↩️ Nothing moved, so nothing is undone.
#[test]
fn inverse_of_a_refusal_is_empty() {
    assert!(inverse_puzzle2d_mutation(&before(), &mutation()).expect("valid retained mutation inverse fixture").is_empty(), "drag-selection/rejects-ghosts: a refusal must yield no inverse step");
}
