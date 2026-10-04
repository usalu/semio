//! 🧪️ `scale-selection3d` fixture — `🧱️zero-factor`.
//!
//! A zero factor would flatten the part; the schema's `exclusiveMinimum: 0` forbids it: a Fatal `mutation.invariant`.
//!
//! Source of truth is the committed JSON quintet under `🧫️fixtures/🧬️mutations/🔍️scale-selection3d/🧱️zero-factor/`
//! (contract D1); the scene is the synthetic selection scene shared by every selection-transform vector.

use crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
use crate::standards::v1::subsets::any::schema::mutations::apply_puzzle5d_mutation;
use crate::Puzzle5dSnapshot;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔍️scale-selection3d/🧱️zero-factor/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔍️scale-selection3d/🧱️zero-factor/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔍️scale-selection3d/🧱️zero-factor/🦠️mutation/🔣️.json");
const DIFF_ABSENT: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔍️scale-selection3d/🧱️zero-factor/🔺️diff/🚫️.absent");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔍️scale-selection3d/🧱️zero-factor/🎯️outcome/🔣️.json");

fn before() -> Puzzle5dSnapshot {
    semio_framework_pack_json::from_json_str(BEFORE, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before snapshot decodes")
}
fn expected_after() -> Puzzle5dSnapshot {
    semio_framework_pack_json::from_json_str(AFTER, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("after snapshot decodes")
}
fn mutation() -> Puzzle5dMutation {
    semio_framework_pack_json::from_json_str(MUTATION, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation decodes")
}
fn outcome() -> serde_json::Value {
    serde_json::from_str(OUTCOME).expect("outcome decodes")
}

/// 🗣️ `(level, code, target)` of every message `scale-selection3d` raises on the committed base.
fn produced_messages() -> Vec<(semio_framework_diagnostic::Severity, String, Vec<String>)> {
    let produced = <Puzzle5dMutation as protocol::Mutation<Puzzle5dSnapshot>>::diff(&mutation(), &before());
    produced.messages().iter().map(|message| (message.level, message.code.0.clone(), message.target.clone())).collect()
}

/// 🔣️ Both committed snapshots and the committed `scale-selection3d` payload are already canonical.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Puzzle5dSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "scale-selection3d/zero-factor: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&mutation())).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "scale-selection3d/zero-factor: committed mutation JSON is not canonical");
}

/// ▶️ A refused `scale-selection3d` still applies cleanly — its diff is the default one — and leaves the scene at
/// the committed `after`, which is the committed `before`.
#[test]
fn refusal_leaves_the_document_at_the_committed_after() {
    let mut snapshot = before();
    apply_puzzle5d_mutation(&mut snapshot, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "scale-selection3d/zero-factor: applied state differs from committed after-snapshot");
    assert_eq!(expected_after(), before(), "scale-selection3d/zero-factor: a rejected vector's two committed snapshots must be identical");
}

/// 🧱️ The payload breaks a hard bound of its own schema, so the refusal is exactly one Fatal
/// `mutation.invariant` addressing the declared path, with the default diff.
#[test]
fn the_invariant_is_the_declared_refusal() {
    assert!(DIFF_ABSENT.is_empty(), "scale-selection3d/zero-factor: the D6 sentinel 🔺️diff/🚫️.absent must stay empty");
    let produced = <Puzzle5dMutation as protocol::Mutation<Puzzle5dSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &Puzzle5dDiff::default(), "scale-selection3d/zero-factor: a Fatal outcome carries the default diff");
    let outcome = outcome();
    assert_eq!(outcome["status"].as_str(), Some("rejected"), "scale-selection3d/zero-factor declares a rejected outcome");
    assert_eq!(outcome["code"].as_str(), Some("mutation.invariant"), "scale-selection3d/zero-factor declares the invariant refusal");
    let path: Vec<String> = outcome["path"].as_array().expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(produced_messages(), vec![(semio_framework_diagnostic::Severity::Fatal, "mutation.invariant".to_string(), path)], "scale-selection3d/zero-factor: the refusal differs from the declared one");
}

/// 🌐️ The refusal does not depend on the scene: the empty scene refuses the same payload the same way.
#[test]
fn the_invariant_is_independent_of_the_base() {
    let produced = <Puzzle5dMutation as protocol::Mutation<Puzzle5dSnapshot>>::diff(&mutation(), &Puzzle5dSnapshot::default());
    assert_eq!(produced.messages().iter().map(|message| (message.level, message.code.0.as_str())).collect::<Vec<_>>(), vec![(semio_framework_diagnostic::Severity::Fatal, "mutation.invariant")], "scale-selection3d/zero-factor: an invariant is a property of the payload alone");
}
