//! 🧪️ `change-part-kind-description` fixture — `rewrites-part-kind-description`: the part kind's own `description` is rewritten
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::mutations::{apply_block5d_mutation, inverse_block5d_mutation};
use crate::artifacts::block5d::Block5dSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> Block5dSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> Block5dSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> Block5dMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ The mutation carries `before` to exactly the committed `after`: the part kind's own `description` is rewritten
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_block5d_mutation(&mut snapshot, &mutation()).expect("change-part-kind-description applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "change-part-kind-description/rewrites-part-kind-description: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.part_kind.description, "One habitation capsule with a plug rim.", "change-part-kind-description must rewrite the kind description, never the session meta description");
}

/// ↩️ Applying the mutation then its inverse restores `before` exactly.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_block5d_mutation(&base, &mutation);
    let mut snapshot = base.clone();
    apply_block5d_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_block5d_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "change-part-kind-description/rewrites-part-kind-description: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Block5dSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-part-kind-description/rewrites-part-kind-description: committed {label} JSON is not canonical");
    }
    let decoded_mutation = mutation();
    let reencoded = serde_json::to_value(&decoded_mutation).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-part-kind-description/rewrites-part-kind-description: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what the mutation actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let mut snapshot = before();
    let applied = apply_block5d_mutation(&mut snapshot, &mutation()).is_ok();
    match status {
        "applied" => assert!(applied, "change-part-kind-description/rewrites-part-kind-description: declared applied but the mutation was rejected"),
        "rejected" => {
            assert!(!applied, "change-part-kind-description/rewrites-part-kind-description: declared rejected but the mutation applied");
            assert_eq!(snapshot, before(), "change-part-kind-description/rewrites-part-kind-description: rejected mutation must leave the snapshot untouched");
        }
        other => panic!("change-part-kind-description/rewrites-part-kind-description: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: it pins WHICH collections and fields `change-part-kind-description` is
/// allowed to touch, not merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <Block5dMutation as protocol::Mutation<Block5dSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-part-kind-description/rewrites-part-kind-description: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::block5d::diff::Block5dDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-part-kind-description/rewrites-part-kind-description: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is a
/// complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::block5d::diff::Block5dDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::block5d::diff::Block5dDiff as protocol::MutationDiff<Block5dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-part-kind-description/rewrites-part-kind-description: committed diff did not carry before to after");
}
