//! 🧪️ `drag-selection` scene_snapshot — `🧱️no-targets`.
//!
//! An empty target set is what the schema's `minItems: 1` forbids: a Fatal `mutation.invariant`, nothing moves.
//!
//! Source of truth is the committed JSON quintet under `🧫️fixtures/🧬️mutations/✋️drag-selection/🧱️no-targets/`
//! (contract D1); the scene is the synthetic selection scene shared by every selection-transform vector.

use semio_s_artifact_puzzle_3d_diff::Puzzle3dDiff;
use semio_s_artifact_puzzle_3d_mutations::apply_puzzle3d_mutation;
use semio_s_artifact_puzzle_3d_mutations::Puzzle3dMutation;
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;

static BEFORE: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧪️tests/🧫️fixtures/🧬️mutations/✋️drag-selection/🧱️no-targets/📸️snapshot/⬅️before/🔣️.json")).expect("committed test data reads"));
static AFTER: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧪️tests/🧫️fixtures/🧬️mutations/✋️drag-selection/🧱️no-targets/📸️snapshot/➡️after/🔣️.json")).expect("committed test data reads"));
static MUTATION: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧪️tests/🧫️fixtures/🧬️mutations/✋️drag-selection/🧱️no-targets/🦠️mutation/🔣️.json")).expect("committed test data reads"));
static DIFF_ABSENT: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧪️tests/🧫️fixtures/🧬️mutations/✋️drag-selection/🧱️no-targets/🔺️diff/🚫️.absent")).expect("committed test data reads"));
static OUTCOME: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧪️tests/🧫️fixtures/🧬️mutations/✋️drag-selection/🧱️no-targets/🎯️outcome/🔣️.json")).expect("committed test data reads"));

fn before() -> Puzzle3dSnapshot {
    semio_framework_pack_json::from_json_str(BEFORE.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before snapshot decodes")
}
fn expected_after() -> Puzzle3dSnapshot {
    semio_framework_pack_json::from_json_str(AFTER.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("after snapshot decodes")
}
fn mutation() -> Puzzle3dMutation {
    semio_framework_pack_json::from_json_str(MUTATION.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation decodes")
}
fn outcome() -> serde_json::Value {
    serde_json::from_str(OUTCOME.as_str()).expect("outcome decodes")
}

/// 🗣️ `(level, code, target)` of every message `drag-selection` raises on the committed base.
fn produced_messages() -> Vec<(semio_framework_diagnostic::Severity, String, Vec<String>)> {
    let produced = <Puzzle3dMutation as protocol::Mutation<Puzzle3dSnapshot>>::diff(&mutation(), &before());
    produced.messages().iter().map(|message| (message.level, message.code.0.clone(), message.target.clone())).collect()
}

/// 🔣️ Both committed snapshots and the committed `drag-selection` payload are already canonical.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE.as_str()), ("after", AFTER.as_str())] {
        let decoded: Puzzle3dSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "drag-selection/no-targets: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&mutation())).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION.as_str()).expect("mutation reparses");
    assert_eq!(reencoded, original, "drag-selection/no-targets: committed mutation JSON is not canonical");
}

/// ▶️ A refused `drag-selection` still applies cleanly — its diff is the default one — and leaves the scene at
/// the committed `after`, which is the committed `before`.
#[test]
fn refusal_leaves_the_document_at_the_committed_after() {
    let mut snapshot = before();
    apply_puzzle3d_mutation(&mut snapshot, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "drag-selection/no-targets: applied state differs from committed after-snapshot");
    assert_eq!(expected_after(), before(), "drag-selection/no-targets: a rejected vector's two committed snapshots must be identical");
}

/// 🧱️ The payload breaks a hard bound of its own schema, so the refusal is exactly one Fatal
/// `mutation.invariant` addressing the declared path, with the default diff.
#[test]
fn the_invariant_is_the_declared_refusal() {
    assert!(DIFF_ABSENT.as_str().is_empty(), "drag-selection/no-targets: the D6 sentinel 🔺️diff/🚫️.absent must stay empty");
    let produced = <Puzzle3dMutation as protocol::Mutation<Puzzle3dSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &Puzzle3dDiff::default(), "drag-selection/no-targets: a Fatal outcome carries the default diff");
    let outcome = outcome();
    assert_eq!(outcome["status"].as_str(), Some("rejected"), "drag-selection/no-targets declares a rejected outcome");
    assert_eq!(outcome["code"].as_str(), Some("mutation.invariant"), "drag-selection/no-targets declares the invariant refusal");
    let path: Vec<String> = outcome["path"].as_array().expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(produced_messages(), vec![(semio_framework_diagnostic::Severity::Fatal, "mutation.invariant".to_string(), path)], "drag-selection/no-targets: the refusal differs from the declared one");
}

/// 🌐️ The refusal does not depend on the scene: the empty scene refuses the same payload the same way.
#[test]
fn the_invariant_is_independent_of_the_base() {
    let produced = <Puzzle3dMutation as protocol::Mutation<Puzzle3dSnapshot>>::diff(&mutation(), &Puzzle3dSnapshot::default());
    assert_eq!(produced.messages().iter().map(|message| (message.level, message.code.0.as_str())).collect::<Vec<_>>(), vec![(semio_framework_diagnostic::Severity::Fatal, "mutation.invariant")], "drag-selection/no-targets: an invariant is a property of the payload alone");
}
