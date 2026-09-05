//! 🧪️ `change-manifest-id` fixture — `📦️repoints-manifest`.
//!
//! `meta.manifestId` is a document-root singleton, so the builder has no missing-target branch: it
//! clones the whole `meta` block, rewrites the one field, and publishes `meta` wholesale.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::puzzle2d::mutations::Puzzle2dMutation;
use crate::artifacts::puzzle2d::mutations::{apply_puzzle2d_mutation, inverse_puzzle2d_mutation};
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> Puzzle2dSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> Puzzle2dSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> Puzzle2dMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ The committed `change-manifest-id` payload carries `before` to exactly the committed `after`, and
/// lands the change this case is named for.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_puzzle2d_mutation(&mut snapshot, &mutation()).expect("change-manifest-id applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "change-manifest-id/repoints-manifest: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.meta.manifest_id.as_deref(), Some("manifest-beta"), "change-manifest-id/repoints-manifest: the manifest reference did not move");
    assert_eq!(snapshot.meta.kind_compatibility, before().meta.kind_compatibility, "change-manifest-id/repoints-manifest: republishing meta must carry the compatibility table through unchanged");
}

/// ↩️ Applying `change-manifest-id` then the inverse it derives from `before` restores `before` exactly.
#[test]
fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_puzzle2d_mutation(&base, &mutation);
    let mut snapshot = base.clone();
    apply_puzzle2d_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_puzzle2d_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "change-manifest-id/repoints-manifest: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-manifest-id` payload are already canonical:
/// decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Puzzle2dSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-manifest-id/repoints-manifest: committed {label} JSON is not canonical");
    }
    let decoded_mutation = mutation();
    let reencoded = serde_json::to_value(&decoded_mutation).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-manifest-id/repoints-manifest: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `change-manifest-id` actually produces on this base.
#[test]
fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let mut snapshot = before();
    let applied = apply_puzzle2d_mutation(&mut snapshot, &mutation()).is_ok();
    match status {
        "applied" => assert!(applied, "change-manifest-id/repoints-manifest: declared applied but the mutation was rejected"),
        "rejected" => {
            assert!(!applied, "change-manifest-id/repoints-manifest: declared rejected but the mutation applied");
            assert_eq!(snapshot, before(), "change-manifest-id/repoints-manifest: rejected mutation must leave the snapshot untouched");
        }
        other => panic!("change-manifest-id/repoints-manifest: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta `change-manifest-id` produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: it pins WHICH collections and fields this mutation is
/// allowed to touch, not merely that the end state matches.
#[test]
fn produces_committed_diff() {
    let base = before();
    let outcome = <Puzzle2dMutation as protocol::Mutation<Puzzle2dSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-manifest-id/repoints-manifest: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert_eq!(committed["meta"]["manifestId"].as_str(), Some("manifest-beta"), "change-manifest-id/repoints-manifest: the diff must publish the new manifest id on meta");
    assert!(committed["nodes"].is_null() && committed["edges"].is_null(), "change-manifest-id/repoints-manifest: a meta edit must touch no collection delta");
}

/// 🔣️ The committed `change-manifest-id` diff is itself canonical and decodes to `Puzzle2dDiff`.
#[test]
fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::puzzle2d::diff::Puzzle2dDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-manifest-id/repoints-manifest: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed `change-manifest-id` diff directly to `before` yields the committed `after` —
/// the diff is a complete description of the change, not a summary of it.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::puzzle2d::diff::Puzzle2dDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::puzzle2d::diff::Puzzle2dDiff as protocol::MutationDiff<Puzzle2dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-manifest-id/repoints-manifest: committed diff did not carry before to after");
}
