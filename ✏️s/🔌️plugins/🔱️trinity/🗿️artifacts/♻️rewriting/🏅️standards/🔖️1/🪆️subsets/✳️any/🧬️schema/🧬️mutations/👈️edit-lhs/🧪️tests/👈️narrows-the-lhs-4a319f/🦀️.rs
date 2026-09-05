//! 🧪️ `edit-lhs` fixture — `👈️narrows-the-lhs-pattern-to-a-shaft-neighbour`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! ♻️ Every `RewritingSnapshot` field is INLINE — `before_fixture_json`/`lhs_json`/`rhs_json` are
//! plain `String`s, `parameter_bindings`/`rule_layout` plain `BTreeMap`s — so this artifact carries no
//! composed child and no content-addressed handle anywhere. Nothing here is unhashable, so this leaf
//! gets a real APPLIED case with a full `🔺️diff/🔣️.json`.
//!
//! 🔍️ `edit-lhs` replaces the authored LHS MATCH PATTERN — the half of the rule that selects which
//! subgraph the rewriting fires on. Unlike its `edit-rhs` sibling it carries no `#[dsl(lang = "json")]`
//! annotation on its snapshot field, but on the mutation lane the two behave identically: a
//! whole-body string replacement with a single `mutation.no-op` guard and no addressable target.

use crate::artifacts::rewriting::diff::RewritingDiff;
use crate::artifacts::rewriting::mutations::RewriteRuleMutation;
use crate::artifacts::rewriting::RewritingSnapshot;
use crate::artifacts::rewriting::{apply_rewrite_rule_mutation, inverse_rewrite_rule_mutation};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> RewritingSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> RewritingSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> RewriteRuleMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ `edit-lhs` carries `before` to exactly the committed `after` by replacing the whole authored LHS
/// pattern — here narrowing a bare `Piece` match to one adjacent to `shaft` — and touching nothing else.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    let mut snapshot = base.clone();
    apply_rewrite_rule_mutation(&mut snapshot, &mutation()).expect("edit-lhs applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "edit-lhs/narrows-the-lhs-pattern-to-a-shaft-neighbour: applied state differs from committed after-snapshot");
    assert_ne!(snapshot.lhs_json, base.lhs_json, "edit-lhs must actually replace the authored LHS pattern");
    assert_eq!(snapshot.rhs_json, base.rhs_json, "edit-lhs must not touch the RHS rewriting program");
    assert_eq!(snapshot.before_fixture_json, base.before_fixture_json, "edit-lhs must not touch the before-fixture graph");
    assert_eq!(snapshot.parameter_bindings, base.parameter_bindings, "edit-lhs must not reach the parameter-binding map");
    assert_eq!(snapshot.rule_layout, base.rule_layout, "edit-lhs must not reach the rule-layout map");
}

/// ↩️ `edit-lhs` inverts BASE-derived: one `edit-lhs` back to the pattern BASE was carrying.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_rewrite_rule_mutation(&base, &mutation);
    assert_eq!(inverse.len(), 1, "edit-lhs always undoes with exactly one step, got {inverse:?}");
    let RewriteRuleMutation::EditLhs(undo) = &inverse[0] else {
        panic!("edit-lhs's inverse must itself be an edit-lhs, got {:?}", inverse[0]);
    };
    assert_eq!(undo.new_lhs_json, base.lhs_json, "the inverse restores exactly the LHS pattern BASE carried");
    let mut snapshot = base.clone();
    apply_rewrite_rule_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_rewrite_rule_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "edit-lhs/narrows-the-lhs-pattern-to-a-shaft-neighbour: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `edit-lhs` payload are already canonical:
/// decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RewritingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "edit-lhs/narrows-the-lhs-pattern-to-a-shaft-neighbour: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "edit-lhs/narrows-the-lhs-pattern-to-a-shaft-neighbour: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome is exactly what `edit-lhs` produces here — an applied change with no
/// diagnostics at all — and this fixture is bound to `edit-lhs`'s own semantic descriptor.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "edit-lhs/narrows-the-lhs-pattern-to-a-shaft-neighbour declares an applied outcome");
    let mut snapshot = before();
    apply_rewrite_rule_mutation(&mut snapshot, &mutation()).expect("edit-lhs/narrows-the-lhs-pattern-to-a-shaft-neighbour: declared applied but the mutation was rejected");
    let produced = <RewriteRuleMutation as protocol::Mutation<RewritingSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "edit-lhs/narrows-the-lhs-pattern-to-a-shaft-neighbour declares no diagnostics, but got {:?}", produced.messages());
    let semantics = <RewriteRuleMutation as protocol::SemanticMutation<RewritingSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("edit", "lhs", "edit-lhs", "EditedLhs"), "the fixture must be bound to edit-lhs's own descriptor");
}

/// 🔺️ The sparse delta `edit-lhs` produces is exactly the committed diff: the single `lhs_json` slot
/// set, and in particular the sibling `rhs_json` slot untouched.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <RewriteRuleMutation as protocol::Mutation<RewritingSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "edit-lhs/narrows-the-lhs-pattern-to-a-shaft-neighbour: produced diff differs from the committed 🔺️diff/🔣️.json");
    let typed: RewritingDiff = serde_json::from_str(DIFF).expect("committed diff decodes into RewritingDiff");
    assert!(typed.lhs_json.is_some(), "edit-lhs's delta must set the lhs_json slot");
    assert!(typed.rhs_json.is_none(), "edit-lhs must never bleed into its rhs_json sibling slot");
    assert!(typed.before_fixture_json.is_none(), "edit-lhs's delta must leave the before-fixture graph alone");
    assert!(typed.parameter_bindings.is_none() && typed.rule_layout.is_none(), "edit-lhs's delta must never reach either key-addressed map");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own `RewritingDiff`.
/// `RewritingDiff` carries a container-level `#[serde(default)]` and NO per-field
/// `skip_serializing_if`, so all nine sparse slots — including the presence/config-lane ones
/// `edit-lhs` never touches — must be present as `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: RewritingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "edit-lhs/narrows-the-lhs-pattern-to-a-shaft-neighbour: committed diff JSON is not canonical");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    let slots = committed.as_object().expect("the committed diff is a JSON object");
    assert_eq!(slots.len(), 9, "RewritingDiff emits all nine sparse slots, got {slots:?}");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is a
/// complete description of the `edit-lhs` change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: RewritingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <RewritingDiff as protocol::MutationDiff<RewritingSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "edit-lhs/narrows-the-lhs-pattern-to-a-shaft-neighbour: committed diff did not carry before to after");
}
