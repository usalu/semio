//! 🧪️ `scale-selection` scene_snapshot — `⛓️stays`.
//!
//! Scaling moves no pose: only `object-a`'s scale doubles, nothing follows and no attraction is re-derived.
//!
//! Source of truth is the committed JSON quintet under `🧫️fixtures/🧬️mutations/🔍️scale-selection/⛓️stays/`
//! (contract D1); the scene is the synthetic selection scene shared by every selection-transform vector.

use semio_s_artifact_puzzle_3d_diff::Puzzle3dDiff;
use semio_s_artifact_puzzle_3d_mutations::apply_puzzle3d_mutation;
use semio_s_artifact_puzzle_3d_mutations::Puzzle3dMutation;
use semio_s_artifact_puzzle_3d_mutations::inverse_puzzle3d_mutation;

use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;

static BEFORE: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧪️tests/🧫️fixtures/🧬️mutations/🔍️scale-selection/⛓️stays/📸️snapshot/⬅️before/🔣️.json")).expect("committed test data reads"));
static AFTER: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧪️tests/🧫️fixtures/🧬️mutations/🔍️scale-selection/⛓️stays/📸️snapshot/➡️after/🔣️.json")).expect("committed test data reads"));
static MUTATION: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧪️tests/🧫️fixtures/🧬️mutations/🔍️scale-selection/⛓️stays/🦠️mutation/🔣️.json")).expect("committed test data reads"));
static DIFF: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧪️tests/🧫️fixtures/🧬️mutations/🔍️scale-selection/⛓️stays/🔺️diff/🔣️.json")).expect("committed test data reads"));
static OUTCOME: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧪️tests/🧫️fixtures/🧬️mutations/🔍️scale-selection/⛓️stays/🎯️outcome/🔣️.json")).expect("committed test data reads"));

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

/// 🗣️ `(level, code, target)` of every message `scale-selection` raises on the committed base.
fn produced_messages() -> Vec<(semio_framework_diagnostic::Severity, String, Vec<String>)> {
    let produced = <Puzzle3dMutation as protocol::Mutation<Puzzle3dSnapshot>>::diff(&mutation(), &before());
    produced.messages().iter().map(|message| (message.level, message.code.0.clone(), message.target.clone())).collect()
}

/// 🔣️ Both committed snapshots and the committed `scale-selection` payload are already canonical.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE.as_str()), ("after", AFTER.as_str())] {
        let decoded: Puzzle3dSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "scale-selection/stays: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&mutation())).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION.as_str()).expect("mutation reparses");
    assert_eq!(reencoded, original, "scale-selection/stays: committed mutation JSON is not canonical");
}

/// 📜️ `(level, code, target)` of every message the committed outcome declares.
fn declared_messages() -> Vec<(semio_framework_diagnostic::Severity, String, Vec<String>)> {
    let level = |text: &str| match text {
        "info" => semio_framework_diagnostic::Severity::Info,
        "warning" => semio_framework_diagnostic::Severity::Warning,
        "error" => semio_framework_diagnostic::Severity::Error,
        "fatal" => semio_framework_diagnostic::Severity::Fatal,
        other => panic!("scale-selection/stays: unknown message level {other:?}"),
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
    let outcome = <Puzzle3dMutation as protocol::Mutation<Puzzle3dSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(outcome.diff())).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF.as_str()).expect("committed diff decodes");
    assert_eq!(produced, committed, "scale-selection/stays: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert!(committed["meta"].is_null() && committed["references"].is_null(), "scale-selection/stays: a selection transform touches no document meta and no reference");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after`.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: Puzzle3dDiff = semio_framework_pack_json::from_json_str(DIFF.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let produced = protocol::apply_diff(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "scale-selection/stays: committed diff did not carry before to after");
}

/// ▶️ The committed payload carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_puzzle3d_mutation(&mut snapshot, &mutation()).expect("scale-selection applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "scale-selection/stays: applied state differs from committed after-snapshot");
    assert_ne!(snapshot, before(), "scale-selection/stays: an applied vector must move the scene");
}

/// ↩️ Applying the payload then the inverse it derives from `before` restores `before` EXACTLY — the
/// inverse is absolute setters read off the base, never a negated parameter.
#[test]
fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_puzzle3d_mutation(&base, &mutation).expect("valid retained mutation inverse scene_snapshot");
    assert!(!inverse.is_empty(), "scale-selection/stays: a moving vector must have something to undo");
    let mut snapshot = base.clone();
    apply_puzzle3d_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in inverse.iter().rev() {
        apply_puzzle3d_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "scale-selection/stays: inverse did not restore the before-snapshot");
}

/// 🎯️ The declared outcome — `applied`, with exactly the declared warnings — is what `scale-selection` emits.
#[test]
fn declared_outcome_holds() {
    assert_eq!(outcome()["status"].as_str(), Some("applied"), "scale-selection/stays declares an applied outcome");
    assert_eq!(produced_messages(), declared_messages(), "scale-selection/stays: the produced messages differ from the declared ones");
}

/// 🧾️ The committed diff is itself canonical and decodes to `Puzzle3dDiff`.
#[test]
fn committed_diff_is_canonical() {
    let decoded: Puzzle3dDiff = semio_framework_pack_json::from_json_str(DIFF.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&decoded)).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF.as_str()).expect("committed diff reparses");
    assert_eq!(reencoded, original, "scale-selection/stays: committed diff JSON is not canonical");
}

/// ➕️ The concrete inverse rows' diffs sum to exactly the negative of the forward diff (law L3): replaying them restores `before`,
/// the absorbed sum carries the applied state back, and it equals `diff.inverse(before)`.
#[test]
fn inverse_sums_to_the_negative_diff() {
    ::semio_framework_async::poll::resolve_ready(protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation(), &before()));
}
