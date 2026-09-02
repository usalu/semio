//! 🧪️ `edit-before-fixture` fixture — `swaps-in-a-two-node-before-graph`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! ♻️ Every `RewriteSnapshot` field is INLINE — `before_fixture_json`/`lhs_json`/`rhs_json` are
//! plain `String`s, `parameter_bindings`/`rule_layout` plain `BTreeMap`s — so this artifact carries no
//! composed child and no content-addressed handle anywhere. Nothing here is unhashable, so this leaf
//! gets a real APPLIED case with a full `🔺️diff/🔣️.json`.
//!
//! 🖼️ `edit-before-fixture` is the one rewrite verb whose body is a whole FOREIGN document — a
//! `trinity.graph` fixture — carried as an opaque JSON string. That string is still an ordinary inline
//! `String` field on `RewriteSnapshot`, not a composed child, so it is fully hand-authorable here.

use crate::artifacts::rewrite::diff::RewriteDiff;
use crate::artifacts::rewrite::mutations::RewriteRuleMutation;
use crate::artifacts::rewrite::RewriteSnapshot;
use crate::artifacts::rewrite::{apply_rewrite_rule_mutation, inverse_rewrite_rule_mutation};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> RewriteSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> RewriteSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> RewriteRuleMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ `edit-before-fixture` carries `before` to exactly the committed `after` by replacing the whole
/// "before" working-graph body — here a bare header growing a two-node `nodes` array — and nothing else.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    let mut snapshot = base.clone();
    apply_rewrite_rule_mutation(&mut snapshot, &mutation()).expect("edit-before-fixture applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "edit-before-fixture/swaps-in-a-two-node-before-graph: applied state differs from committed after-snapshot");
    assert_ne!(snapshot.before_fixture_json, base.before_fixture_json, "edit-before-fixture must actually replace the before-graph body");
    assert!(snapshot.before_fixture_json.contains("\"nodes\""), "the committed after must carry the two-node graph the payload supplied");
    assert_eq!(snapshot.lhs_json, base.lhs_json, "edit-before-fixture must not touch the LHS match pattern");
    assert_eq!(snapshot.rhs_json, base.rhs_json, "edit-before-fixture must not touch the RHS rewrite program");
    assert_eq!(snapshot.parameter_bindings, base.parameter_bindings, "edit-before-fixture must not reach the parameter-binding map");
    assert_eq!(snapshot.rule_layout, base.rule_layout, "edit-before-fixture must not reach the rule-layout map");
}

/// ↩️ `edit-before-fixture` inverts BASE-derived: one `edit-before-fixture` back to the graph body BASE
/// was carrying — the whole foreign document, verbatim.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_rewrite_rule_mutation(&base, &mutation);
    assert_eq!(inverse.len(), 1, "edit-before-fixture always undoes with exactly one step, got {inverse:?}");
    let RewriteRuleMutation::EditBeforeFixture(undo) = &inverse[0] else {
        panic!("edit-before-fixture's inverse must itself be an edit-before-fixture, got {:?}", inverse[0]);
    };
    assert_eq!(undo.new_before_fixture_json, base.before_fixture_json, "the inverse restores exactly the before-graph body BASE carried");
    let mut snapshot = base.clone();
    apply_rewrite_rule_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_rewrite_rule_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "edit-before-fixture/swaps-in-a-two-node-before-graph: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `edit-before-fixture` payload are already canonical:
/// decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RewriteSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "edit-before-fixture/swaps-in-a-two-node-before-graph: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "edit-before-fixture/swaps-in-a-two-node-before-graph: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome is exactly what `edit-before-fixture` produces here — an applied change with no
/// diagnostics at all — and this fixture is bound to `edit-before-fixture`'s own semantic descriptor.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "edit-before-fixture/swaps-in-a-two-node-before-graph declares an applied outcome");
    let mut snapshot = before();
    apply_rewrite_rule_mutation(&mut snapshot, &mutation()).expect("edit-before-fixture/swaps-in-a-two-node-before-graph: declared applied but the mutation was rejected");
    let produced = <RewriteRuleMutation as protocol::Mutation<RewriteSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "edit-before-fixture/swaps-in-a-two-node-before-graph declares no diagnostics, but got {:?}", produced.messages());
    let semantics = <RewriteRuleMutation as protocol::SemanticMutation<RewriteSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("edit", "before-fixture", "edit-before-fixture", "EditedBeforeFixture"), "the fixture must be bound to edit-before-fixture's own descriptor");
}

/// 🔺️ The sparse delta `edit-before-fixture` produces is exactly the committed diff: the single
/// `before_fixture_json` slot set, with neither authored rule half disturbed.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <RewriteRuleMutation as protocol::Mutation<RewriteSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "edit-before-fixture/swaps-in-a-two-node-before-graph: produced diff differs from the committed 🔺️diff/🔣️.json");
    let typed: RewriteDiff = serde_json::from_str(DIFF).expect("committed diff decodes into RewriteDiff");
    assert!(typed.before_fixture_json.is_some(), "edit-before-fixture's delta must set the before_fixture_json slot");
    assert!(typed.lhs_json.is_none() && typed.rhs_json.is_none(), "edit-before-fixture must never disturb either authored rule half");
    assert!(typed.parameter_bindings.is_none() && typed.rule_layout.is_none(), "edit-before-fixture's delta must never reach either key-addressed map");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own `RewriteDiff`.
/// `RewriteDiff` carries a container-level `#[serde(default)]` and NO per-field
/// `skip_serializing_if`, so all nine sparse slots — including the presence/config-lane ones
/// `edit-before-fixture` never touches — must be present as `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: RewriteDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "edit-before-fixture/swaps-in-a-two-node-before-graph: committed diff JSON is not canonical");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    let slots = committed.as_object().expect("the committed diff is a JSON object");
    assert_eq!(slots.len(), 9, "RewriteDiff emits all nine sparse slots, got {slots:?}");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is a
/// complete description of the `edit-before-fixture` change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: RewriteDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <RewriteDiff as protocol::MutationDiff<RewriteSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "edit-before-fixture/swaps-in-a-two-node-before-graph: committed diff did not carry before to after");
}
