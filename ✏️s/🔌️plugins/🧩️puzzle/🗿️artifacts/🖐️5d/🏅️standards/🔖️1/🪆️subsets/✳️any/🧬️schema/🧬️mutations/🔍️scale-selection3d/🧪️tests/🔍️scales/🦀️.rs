//! 🧪️ `scale-selection3d` fixture — `🔍️scales`.
//!
//! Factors (2, 1, 0.5): `part-a`'s uniform scale becomes a per-axis triple, `part-b`, which carries no scale, reads as one, and `volume-1` scales its triple.
//!
//! Source of truth is the committed JSON quintet under `🧫️fixtures/🧬️mutations/🔍️scale-selection3d/🔍️scales/`
//! (contract D1); the scene is the synthetic selection scene shared by every selection-transform vector.

use crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
use crate::standards::v1::subsets::any::schema::mutations::{apply_puzzle5d_mutation,inverse_puzzle5d_mutation};

use crate::Puzzle5dSnapshot;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔍️scale-selection3d/🔍️scales/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔍️scale-selection3d/🔍️scales/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔍️scale-selection3d/🔍️scales/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔍️scale-selection3d/🔍️scales/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔍️scale-selection3d/🔍️scales/🎯️outcome/🔣️.json");

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
        assert_eq!(reencoded, original, "scale-selection3d/scales: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&mutation())).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "scale-selection3d/scales: committed mutation JSON is not canonical");
}

/// 📜️ `(level, code, target)` of every message the committed outcome declares.
fn declared_messages() -> Vec<(semio_framework_diagnostic::Severity, String, Vec<String>)> {
    let level = |text: &str| match text {
        "info" => semio_framework_diagnostic::Severity::Info,
        "warning" => semio_framework_diagnostic::Severity::Warning,
        "error" => semio_framework_diagnostic::Severity::Error,
        "fatal" => semio_framework_diagnostic::Severity::Fatal,
        other => panic!("scale-selection3d/scales: unknown message level {other:?}"),
    };
    let strings = |value: &serde_json::Value| value.as_array().expect("an array of strings").iter().map(|entry| entry.as_str().expect("a string").to_string()).collect::<Vec<_>>();
    outcome().get("messages").and_then(serde_json::Value::as_array).map_or_else(Vec::new, |messages| {
        messages.iter().map(|message| (level(message["level"].as_str().expect("a level")), message["code"].as_str().expect("a code").to_string(), strings(&message["target"]))).collect()
    })
}

/// 🔺️ The sparse delta `scale-selection3d` produces is exactly the committed diff: WHICH records it patches, and
/// every patched record whole.
#[test]
fn produces_committed_diff() {
    let outcome = <Puzzle5dMutation as protocol::Mutation<Puzzle5dSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(outcome.diff())).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "scale-selection3d/scales: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert!(committed["fasteners"].is_null() && committed["meta"].is_null(), "scale-selection3d/scales: a selection transform touches no relation and no document meta");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after`.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: Puzzle5dDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let produced = protocol::apply_diff(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "scale-selection3d/scales: committed diff did not carry before to after");
}

/// ▶️ The committed payload carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_puzzle5d_mutation(&mut snapshot, &mutation()).expect("scale-selection3d applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "scale-selection3d/scales: applied state differs from committed after-snapshot");
    assert_ne!(snapshot, before(), "scale-selection3d/scales: an applied vector must move the scene");
}

/// ↩️ Applying the payload then the inverse it derives from `before` restores `before` EXACTLY — the
/// inverse is absolute setters read off the base, never a negated parameter.
#[test]
fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_puzzle5d_mutation(&base, &mutation).expect("valid retained mutation inverse fixture");
    assert!(!inverse.is_empty(), "scale-selection3d/scales: a moving vector must have something to undo");
    let mut snapshot = base.clone();
    apply_puzzle5d_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in inverse.iter().rev() {
        apply_puzzle5d_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "scale-selection3d/scales: inverse did not restore the before-snapshot");
}

/// 🎯️ The declared outcome — `applied`, with exactly the declared warnings — is what `scale-selection3d` emits.
#[test]
fn declared_outcome_holds() {
    assert_eq!(outcome()["status"].as_str(), Some("applied"), "scale-selection3d/scales declares an applied outcome");
    assert_eq!(produced_messages(), declared_messages(), "scale-selection3d/scales: the produced messages differ from the declared ones");
}

/// 🧾️ The committed diff is itself canonical and decodes to `Puzzle5dDiff`.
#[test]
fn committed_diff_is_canonical() {
    let decoded: Puzzle5dDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&decoded)).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "scale-selection3d/scales: committed diff JSON is not canonical");
}

/// ➕️ The concrete inverse rows' diffs sum to exactly the negative of the forward diff (law L3): replaying them restores `before`,
/// the absorbed sum carries the applied state back, and it equals `diff.inverse(before)`.
#[test]
fn inverse_sums_to_the_negative_diff() {
    ::semio_framework_async::poll::resolve_ready(protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation(), &before()));
}
