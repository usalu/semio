//! 🧪️ `drag-selection2d` fixture — `⏸️keeps-a-zero-offset`.
//!
//! A zero offset is a Warning-level `mutation.no-op`: the default diff, nothing to undo.
//!
//! Source of truth is the committed JSON quintet under `🧫️fixtures/🧬️mutations/✋️drag-selection2d/⏸️keeps-a-zero-offset/`
//! (contract D1); the scene is the synthetic selection scene shared by every selection-transform vector.

use crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
use crate::standards::v1::subsets::any::schema::mutations::{apply_puzzle5d_mutation,inverse_puzzle5d_mutation};

use crate::Puzzle5dSnapshot;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✋️drag-selection2d/⏸️keeps-a-zero-offset/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✋️drag-selection2d/⏸️keeps-a-zero-offset/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✋️drag-selection2d/⏸️keeps-a-zero-offset/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✋️drag-selection2d/⏸️keeps-a-zero-offset/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✋️drag-selection2d/⏸️keeps-a-zero-offset/🎯️outcome/🔣️.json");

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

/// 🗣️ `(level, code, target)` of every message `drag-selection2d` raises on the committed base.
fn produced_messages() -> Vec<(semio_framework_diagnostic::Severity, String, Vec<String>)> {
    let produced = <Puzzle5dMutation as protocol::Mutation<Puzzle5dSnapshot>>::diff(&mutation(), &before());
    produced.messages().iter().map(|message| (message.level, message.code.0.clone(), message.target.clone())).collect()
}

/// 🔣️ Both committed snapshots and the committed `drag-selection2d` payload are already canonical.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Puzzle5dSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "drag-selection2d/keeps-a-zero-offset: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&mutation())).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "drag-selection2d/keeps-a-zero-offset: committed mutation JSON is not canonical");
}

/// 📜️ `(level, code, target)` of every message the committed outcome declares.
fn declared_messages() -> Vec<(semio_framework_diagnostic::Severity, String, Vec<String>)> {
    let level = |text: &str| match text {
        "info" => semio_framework_diagnostic::Severity::Info,
        "warning" => semio_framework_diagnostic::Severity::Warning,
        "error" => semio_framework_diagnostic::Severity::Error,
        "fatal" => semio_framework_diagnostic::Severity::Fatal,
        other => panic!("drag-selection2d/keeps-a-zero-offset: unknown message level {other:?}"),
    };
    let strings = |value: &serde_json::Value| value.as_array().expect("an array of strings").iter().map(|entry| entry.as_str().expect("a string").to_string()).collect::<Vec<_>>();
    outcome().get("messages").and_then(serde_json::Value::as_array).map_or_else(Vec::new, |messages| {
        messages.iter().map(|message| (level(message["level"].as_str().expect("a level")), message["code"].as_str().expect("a code").to_string(), strings(&message["target"]))).collect()
    })
}

/// 🔺️ The sparse delta `drag-selection2d` produces is exactly the committed diff: WHICH records it patches, and
/// every patched record whole.
#[test]
fn produces_committed_diff() {
    let outcome = <Puzzle5dMutation as protocol::Mutation<Puzzle5dSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(outcome.diff())).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "drag-selection2d/keeps-a-zero-offset: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert!(committed["fasteners"].is_null() && committed["meta"].is_null(), "drag-selection2d/keeps-a-zero-offset: a selection transform touches no relation and no document meta");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after`.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: Puzzle5dDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let produced = protocol::apply_diff(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "drag-selection2d/keeps-a-zero-offset: committed diff did not carry before to after");
}

/// ⏸️ A no-op still applies cleanly and leaves the scene byte-identical.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_puzzle5d_mutation(&mut snapshot, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "drag-selection2d/keeps-a-zero-offset: applied state differs from committed after-snapshot");
    assert_eq!(expected_after(), before(), "drag-selection2d/keeps-a-zero-offset: a no-op vector's two committed snapshots must be identical");
}

/// 🎯️ The declared no-op is exactly what `drag-selection2d` emits: a Warning-level `mutation.no-op` and the default diff.
#[test]
fn declared_outcome_holds() {
    assert_eq!(outcome()["status"].as_str(), Some("no-op"), "drag-selection2d/keeps-a-zero-offset declares a no-op outcome");
    assert_eq!(produced_messages(), declared_messages(), "drag-selection2d/keeps-a-zero-offset: the produced messages differ from the declared ones");
    let produced = <Puzzle5dMutation as protocol::Mutation<Puzzle5dSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &Puzzle5dDiff::default(), "drag-selection2d/keeps-a-zero-offset: a no-op answers the default diff");
}

/// ↩️ Nothing moved, so nothing is undone.
#[test]
fn inverse_is_empty() {
    assert!(inverse_puzzle5d_mutation(&before(), &mutation()).expect("valid retained mutation inverse fixture").is_empty(), "drag-selection2d/keeps-a-zero-offset: a no-op must yield no inverse step");
}
