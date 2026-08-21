//! 🧪️ `edit-rhs` fixture — `rewrites-the-rhs-to-set-a-second-property`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! ♻️ Every `RewriteSnapshot` field is INLINE — `before_fixture_json`/`lhs_json`/`rhs_json` are
//! plain `String`s, `parameter_bindings`/`rule_layout` plain `BTreeMap`s — so this artifact carries no
//! composed child and no content-addressed handle anywhere. Nothing here is unhashable, so this leaf
//! gets a real APPLIED case with a full `🔺️diff/🔣️component.json`.
//!
//! 🎯️ `edit-rhs` is a whole-body replacement of the authored RHS rewrite program, addressed by
//! nothing at all — the payload carries only the new body, so this verb has no target-missing branch;
//! its only non-applying branch is the `mutation.no-op` guard for an already-identical body, which is
//! deliberately NOT what this case pins.

use crate::artifacts::rewrite::diff::RewriteDiff;
use crate::artifacts::rewrite::mutations::{apply_rewrite_rule_mutation, inverse_rewrite_rule_mutation, RewriteRuleMutation};
use crate::artifacts::rewrite::RewriteSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> RewriteSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> RewriteSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> RewriteRuleMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ `edit-rhs` carries `before` to exactly the committed `after` by replacing the whole authored
/// RHS program — here a one-clause `set` body growing a second clause — and touching nothing else.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    let mut snapshot = base.clone();
    apply_rewrite_rule_mutation(&mut snapshot, &mutation()).expect("edit-rhs applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "edit-rhs/rewrites-the-rhs-to-set-a-second-property: applied state differs from committed after-snapshot");
    assert_ne!(snapshot.rhs_json, base.rhs_json, "edit-rhs must actually replace the authored RHS body");
    assert_eq!(snapshot.lhs_json, base.lhs_json, "edit-rhs must not touch the LHS match pattern");
    assert_eq!(snapshot.before_fixture_json, base.before_fixture_json, "edit-rhs must not touch the before-fixture graph");
    assert_eq!(snapshot.parameter_bindings, base.parameter_bindings, "edit-rhs must not reach the parameter-binding map");
    assert_eq!(snapshot.rule_layout, base.rule_layout, "edit-rhs must not reach the rule-layout map");
}

/// ↩️ `edit-rhs` inverts BASE-derived: one `edit-rhs` back to the body BASE was carrying, never a
/// payload-derived undo (the payload only knows the NEW body).
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_rewrite_rule_mutation(&base, &mutation);
    assert_eq!(inverse.len(), 1, "edit-rhs always undoes with exactly one step, got {inverse:?}");
    let RewriteRuleMutation::EditRhs(undo) = &inverse[0] else {
        panic!("edit-rhs's inverse must itself be an edit-rhs, got {:?}", inverse[0]);
    };
    assert_eq!(undo.new_rhs_json, base.rhs_json, "the inverse restores exactly the RHS body BASE carried");
    let mut snapshot = base.clone();
    apply_rewrite_rule_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_rewrite_rule_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "edit-rhs/rewrites-the-rhs-to-set-a-second-property: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `edit-rhs` payload are already canonical:
/// decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RewriteSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "edit-rhs/rewrites-the-rhs-to-set-a-second-property: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "edit-rhs/rewrites-the-rhs-to-set-a-second-property: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome is exactly what `edit-rhs` produces here — an applied change with no
/// diagnostics at all — and this fixture is bound to `edit-rhs`'s own semantic descriptor.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "edit-rhs/rewrites-the-rhs-to-set-a-second-property declares an applied outcome");
    let mut snapshot = before();
    apply_rewrite_rule_mutation(&mut snapshot, &mutation()).expect("edit-rhs/rewrites-the-rhs-to-set-a-second-property: declared applied but the mutation was rejected");
    let produced = <RewriteRuleMutation as protocol::Mutation<RewriteSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "edit-rhs/rewrites-the-rhs-to-set-a-second-property declares no diagnostics, but got {:?}", produced.messages());
    let semantics = <RewriteRuleMutation as protocol::SemanticMutation<RewriteSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("edit", "rhs", "edit-rhs", "EditedRhs"), "the fixture must be bound to edit-rhs's own descriptor");
}

/// 🔺️ The sparse delta `edit-rhs` produces is exactly the committed diff: the single `rhs_json` slot
/// set, and every other slot left `None`.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <RewriteRuleMutation as protocol::Mutation<RewriteSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "edit-rhs/rewrites-the-rhs-to-set-a-second-property: produced diff differs from the committed 🔺️diff/🔣️component.json");
    let typed: RewriteDiff = serde_json::from_str(DIFF).expect("committed diff decodes into RewriteDiff");
    assert!(typed.rhs_json.is_some(), "edit-rhs's delta must set the rhs_json slot");
    assert!(typed.lhs_json.is_none() && typed.before_fixture_json.is_none(), "edit-rhs's delta must leave the other two authored bodies alone");
    assert!(typed.parameter_bindings.is_none() && typed.rule_layout.is_none(), "edit-rhs's delta must never reach either key-addressed map");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own `RewriteDiff`.
/// `RewriteDiff` carries a container-level `#[serde(default)]` and NO per-field
/// `skip_serializing_if`, so all nine sparse slots — including the presence/config-lane ones
/// `edit-rhs` never touches — must be present as `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: RewriteDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "edit-rhs/rewrites-the-rhs-to-set-a-second-property: committed diff JSON is not canonical");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    let slots = committed.as_object().expect("the committed diff is a JSON object");
    assert_eq!(slots.len(), 9, "RewriteDiff emits all nine sparse slots, got {slots:?}");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is a
/// complete description of the `edit-rhs` change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: RewriteDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <RewriteDiff as protocol::MutationDiff<RewriteSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "edit-rhs/rewrites-the-rhs-to-set-a-second-property: committed diff did not carry before to after");
}
