//! 🧪️ `delete-reference` scene_snapshot — `🚫️removes-reference-1`.
//!
//! Unpins `reference-1`. Like the target-volume delete, nothing references a reference plane, so
//! the builder emits a single removal with no cascade.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use semio_s_artifact_puzzle_3d_mutations::Puzzle3dMutation;
use semio_s_artifact_puzzle_3d_mutations::apply_puzzle3d_mutation;
use semio_s_artifact_puzzle_3d_mutations::inverse_puzzle3d_mutation;

use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;

static BEFORE: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧪️tests/🧫️fixtures/🧬️mutations/🖼️reference/🚮delete-reference/🚫️removes-reference-1/📸️snapshot/⬅️before/🔣️.json")).expect("committed test data reads"));
static AFTER: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧪️tests/🧫️fixtures/🧬️mutations/🖼️reference/🚮delete-reference/🚫️removes-reference-1/📸️snapshot/➡️after/🔣️.json")).expect("committed test data reads"));
static MUTATION: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧪️tests/🧫️fixtures/🧬️mutations/🖼️reference/🚮delete-reference/🚫️removes-reference-1/🦠️mutation/🔣️.json")).expect("committed test data reads"));
static DIFF: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧪️tests/🧫️fixtures/🧬️mutations/🖼️reference/🚮delete-reference/🚫️removes-reference-1/🔺️diff/🔣️.json")).expect("committed test data reads"));
static OUTCOME: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧪️tests/🧫️fixtures/🧬️mutations/🖼️reference/🚮delete-reference/🚫️removes-reference-1/🎯️outcome/🔣️.json")).expect("committed test data reads"));

fn before() -> Puzzle3dSnapshot {
    semio_framework_pack_json::from_json_str(BEFORE.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before snapshot decodes")
}
fn expected_after() -> Puzzle3dSnapshot {
    semio_framework_pack_json::from_json_str(AFTER.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("after snapshot decodes")
}
fn mutation() -> Puzzle3dMutation {
    semio_framework_pack_json::from_json_str(MUTATION.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation decodes")
}

/// ▶️ The committed `delete-reference` payload carries `before` to exactly the committed `after`, and
/// lands the change this case is named for.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_puzzle3d_mutation(&mut snapshot, &mutation()).expect("delete-reference applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "delete-reference/removes-reference-1: applied state differs from committed after-snapshot");
    assert!(snapshot.references.is_empty(), "delete-reference/removes-reference-1: reference-1 survived the delete");
    assert_eq!(snapshot.objects, before().objects, "delete-reference/removes-reference-1: unpinning a reference plane must not touch the placed objects");
}

/// ↩️ Applying `delete-reference` then the inverse it derives from `before` restores `before` exactly.
#[test]
fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_puzzle3d_mutation(&base, &mutation).expect("valid retained mutation inverse scene_snapshot");
    let mut snapshot = base.clone();
    apply_puzzle3d_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in inverse.iter().rev() {
        apply_puzzle3d_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "delete-reference/removes-reference-1: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `delete-reference` payload are already canonical:
/// decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE.as_str()), ("after", AFTER.as_str())] {
        let decoded: Puzzle3dSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-reference/removes-reference-1: committed {label} JSON is not canonical");
    }
    let decoded_mutation = mutation();
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&decoded_mutation)).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION.as_str()).expect("mutation reparses");
    assert_eq!(reencoded, original, "delete-reference/removes-reference-1: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `delete-reference` actually produces on this base.
#[test]
fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME.as_str()).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let mut snapshot = before();
    let applied = apply_puzzle3d_mutation(&mut snapshot, &mutation()).is_ok();
    match status {
        "applied" => assert!(applied, "delete-reference/removes-reference-1: declared applied but the mutation was rejected"),
        "rejected" => {
            assert!(!applied, "delete-reference/removes-reference-1: declared rejected but the mutation applied");
            assert_eq!(snapshot, before(), "delete-reference/removes-reference-1: rejected mutation must leave the snapshot untouched");
        }
        other => panic!("delete-reference/removes-reference-1: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta `delete-reference` produces is exactly the committed diff — the single most
/// load-bearing assertion in the scene_snapshot: it pins WHICH collections and fields this mutation is
/// allowed to touch, not merely that the end state matches.
#[test]
fn produces_committed_diff() {
    let base = before();
    let outcome = <Puzzle3dMutation as protocol::ColdMutation<Puzzle3dSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(outcome.diff())).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF.as_str()).expect("committed diff decodes");
    assert_eq!(produced, committed, "delete-reference/removes-reference-1: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert_eq!(committed["references"]["removed"][0]["id"].as_str(), Some("reference-1"), "delete-reference/removes-reference-1: the diff must remove reference-1 by id");
    assert!(committed["targetVolumes"].is_null(), "delete-reference/removes-reference-1: a reference delete has no cascade");
}

/// 🔣️ The committed `delete-reference` diff is itself canonical and decodes to `Puzzle3dDiff`.
#[test]
fn committed_diff_is_canonical() {
    let decoded: semio_s_artifact_puzzle_3d_diff::Puzzle3dDiff = semio_framework_pack_json::from_json_str(DIFF.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&decoded)).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF.as_str()).expect("committed diff reparses");
    assert_eq!(reencoded, original, "delete-reference/removes-reference-1: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed `delete-reference` diff directly to `before` yields the committed `after` —
/// the diff is a complete description of the change, not a summary of it.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: semio_s_artifact_puzzle_3d_diff::Puzzle3dDiff = semio_framework_pack_json::from_json_str(DIFF.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let produced = protocol::apply_diff_cold(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-reference/removes-reference-1: committed diff did not carry before to after");
}

/// ➕️ The concrete inverse rows' diffs sum to exactly the negative of the forward diff (law L3): replaying them restores `before`,
/// the absorbed sum carries the applied state back, and it equals `diff.inverse(before)`.
#[test]
fn inverse_sums_to_the_negative_diff() {
    ::semio_framework_async::poll::resolve_ready(protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation(), &before()));
}
