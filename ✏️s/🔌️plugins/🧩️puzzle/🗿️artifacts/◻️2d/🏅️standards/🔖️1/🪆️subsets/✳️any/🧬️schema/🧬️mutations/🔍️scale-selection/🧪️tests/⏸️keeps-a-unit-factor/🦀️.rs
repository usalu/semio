//! 🧪️ `scale-selection` snapshot — `⏸️keeps-a-unit-factor`.
//!
//! A unit factor is a Warning-level `mutation.no-op`: the default diff, nothing to undo.
//!
//! Source of truth is the committed JSON quintet under `🧫️fixtures/🧬️mutations/🔍️scale-selection/⏸️keeps-a-unit-factor/`
//! (contract D1); the board is the synthetic selection board shared by every selection-transform vector.

use crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
use crate::standards::v1::subsets::any::schema::mutations::{apply_puzzle2d_mutation,inverse_puzzle2d_mutation};

use crate::Puzzle2dSnapshot;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔍️scale-selection/⏸️keeps-a-unit-factor/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔍️scale-selection/⏸️keeps-a-unit-factor/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔍️scale-selection/⏸️keeps-a-unit-factor/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔍️scale-selection/⏸️keeps-a-unit-factor/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔍️scale-selection/⏸️keeps-a-unit-factor/🎯️outcome/🔣️.json");

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

/// 🗣️ `(level, code, target)` of every message `scale-selection` raises on the committed base.
fn produced_messages() -> Vec<(semio_framework_diagnostic::Severity, String, Vec<String>)> {
    let produced = <Puzzle2dMutation as protocol::Mutation<Puzzle2dSnapshot>>::diff(&mutation(), &before());
    produced.messages().iter().map(|message| (message.level, message.code.0.clone(), message.target.clone())).collect()
}

/// 🔣️ Both committed snapshots and the committed `scale-selection` payload are already canonical.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Puzzle2dSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "scale-selection/keeps-a-unit-factor: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&mutation())).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "scale-selection/keeps-a-unit-factor: committed mutation JSON is not canonical");
}

/// 📜️ `(level, code, target)` of every message the committed outcome declares.
fn declared_messages() -> Vec<(semio_framework_diagnostic::Severity, String, Vec<String>)> {
    let level = |text: &str| match text {
        "info" => semio_framework_diagnostic::Severity::Info,
        "warning" => semio_framework_diagnostic::Severity::Warning,
        "error" => semio_framework_diagnostic::Severity::Error,
        "fatal" => semio_framework_diagnostic::Severity::Fatal,
        other => panic!("scale-selection/keeps-a-unit-factor: unknown message level {other:?}"),
    };
    let strings = |value: &serde_json::Value| value.as_array().expect("an array of strings").iter().map(|entry| entry.as_str().expect("a string").to_string()).collect::<Vec<_>>();
    outcome().get("messages").and_then(serde_json::Value::as_array).map_or_else(Vec::new, |messages| {
        messages.iter().map(|message| (level(message["level"].as_str().expect("a level")), message["code"].as_str().expect("a code").to_string(), strings(&message["target"]))).collect()
    })
}

/// 🔺️ The sparse delta `scale-selection` produces is exactly the committed diff: WHICH records it patches, and
/// every patched record whole.
#[test]
fn produces_committed_diff() {
    let outcome = <Puzzle2dMutation as protocol::Mutation<Puzzle2dSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(outcome.diff())).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "scale-selection/keeps-a-unit-factor: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert!(committed["edges"].is_null() && committed["meta"].is_null() && committed["camera"].is_null(), "scale-selection/keeps-a-unit-factor: a selection transform touches neither edges, meta nor camera");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after`.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: Puzzle2dDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let produced = protocol::apply_diff(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "scale-selection/keeps-a-unit-factor: committed diff did not carry before to after");
}

/// ⏸️ A no-op still applies cleanly and leaves the board byte-identical.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_puzzle2d_mutation(&mut snapshot, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "scale-selection/keeps-a-unit-factor: applied state differs from committed after-snapshot");
    assert_eq!(expected_after(), before(), "scale-selection/keeps-a-unit-factor: a no-op vector's two committed snapshots must be identical");
}

/// 🎯️ The declared no-op is exactly what `scale-selection` emits: a Warning-level `mutation.no-op` and the default diff.
#[test]
fn declared_outcome_holds() {
    assert_eq!(outcome()["status"].as_str(), Some("no-op"), "scale-selection/keeps-a-unit-factor declares a no-op outcome");
    assert_eq!(produced_messages(), declared_messages(), "scale-selection/keeps-a-unit-factor: the produced messages differ from the declared ones");
    let produced = <Puzzle2dMutation as protocol::Mutation<Puzzle2dSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &Puzzle2dDiff::default(), "scale-selection/keeps-a-unit-factor: a no-op answers the default diff");
}

/// ↩️ Nothing moved, so nothing is undone.
#[test]
fn inverse_is_empty() {
    assert!(inverse_puzzle2d_mutation(&before(), &mutation()).expect("valid retained mutation inverse snapshot").is_empty(), "scale-selection/keeps-a-unit-factor: a no-op must yield no inverse step");
}
