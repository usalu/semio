//! 🧪️ `create-node` fixture — `🧱️zero-width-node`.
//!
//! A rectangle of zero width is what the node record's `exclusiveMinimum: 0` forbids: a Fatal `mutation.invariant`, no node is added.
//!
//! Source of truth is the committed JSON quintet under `🧫️fixtures/🧬️mutations/🌱create-node/🧱️zero-width-node/`
//! (contract D1); the board is the synthetic selection board shared by every selection-transform vector.

use crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
use crate::standards::v1::subsets::any::schema::mutations::apply_puzzle2d_mutation;
use crate::Puzzle2dSnapshot;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🌱create-node/🧱️zero-width-node/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🌱create-node/🧱️zero-width-node/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🌱create-node/🧱️zero-width-node/🦠️mutation/🔣️.json");
const DIFF_ABSENT: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🌱create-node/🧱️zero-width-node/🔺️diff/🚫️.absent");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🌱create-node/🧱️zero-width-node/🎯️outcome/🔣️.json");

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

/// 🗣️ `(level, code, target)` of every message `create-node` raises on the committed base.
fn produced_messages() -> Vec<(semio_framework_diagnostic::Severity, String, Vec<String>)> {
    let produced = <Puzzle2dMutation as protocol::Mutation<Puzzle2dSnapshot>>::diff(&mutation(), &before());
    produced.messages().iter().map(|message| (message.level, message.code.0.clone(), message.target.clone())).collect()
}

/// 🔣️ Both committed snapshots and the committed `create-node` payload are already canonical.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Puzzle2dSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-node/zero-width-node: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&mutation())).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "create-node/zero-width-node: committed mutation JSON is not canonical");
}

/// ▶️ A refused `create-node` still applies cleanly — its diff is the default one — and leaves the board at
/// the committed `after`, which is the committed `before`.
#[test]
fn refusal_leaves_the_document_at_the_committed_after() {
    let mut snapshot = before();
    apply_puzzle2d_mutation(&mut snapshot, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "create-node/zero-width-node: applied state differs from committed after-snapshot");
    assert_eq!(expected_after(), before(), "create-node/zero-width-node: a rejected vector's two committed snapshots must be identical");
}

/// 🧱️ The payload breaks a hard bound of its own schema, so the refusal is exactly one Fatal
/// `mutation.invariant` addressing the declared path, with the default diff.
#[test]
fn the_invariant_is_the_declared_refusal() {
    assert!(DIFF_ABSENT.is_empty(), "create-node/zero-width-node: the D6 sentinel 🔺️diff/🚫️.absent must stay empty");
    let produced = <Puzzle2dMutation as protocol::Mutation<Puzzle2dSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &Puzzle2dDiff::default(), "create-node/zero-width-node: a Fatal outcome carries the default diff");
    let outcome = outcome();
    assert_eq!(outcome["status"].as_str(), Some("rejected"), "create-node/zero-width-node declares a rejected outcome");
    assert_eq!(outcome["code"].as_str(), Some("mutation.invariant"), "create-node/zero-width-node declares the invariant refusal");
    let path: Vec<String> = outcome["path"].as_array().expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(produced_messages(), vec![(semio_framework_diagnostic::Severity::Fatal, "mutation.invariant".to_string(), path)], "create-node/zero-width-node: the refusal differs from the declared one");
}

/// 🌐️ The refusal does not depend on the board: the empty board refuses the same payload the same way.
#[test]
fn the_invariant_is_independent_of_the_base() {
    let produced = <Puzzle2dMutation as protocol::Mutation<Puzzle2dSnapshot>>::diff(&mutation(), &Puzzle2dSnapshot::default());
    assert_eq!(produced.messages().iter().map(|message| (message.level, message.code.0.as_str())).collect::<Vec<_>>(), vec![(semio_framework_diagnostic::Severity::Fatal, "mutation.invariant")], "create-node/zero-width-node: an invariant is a property of the payload alone");
}
