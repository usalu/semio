//! 🧪️ `change-rule-layout-point` fixture — `nudges-the-capsule-var-off-the-shaft`.
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
//! 📐️ `change-rule-layout-point` upserts one key of `rule_layout`, whose value is the named record
//! `LayoutPoint` rather than a bare `(f64, f64)` tuple — the DSL engine has no `DslField` binding for
//! raw Rust tuples, so the point is a two-field record on the wire (`{"x": .., "y": ..}`). This case
//! pins the applied MOVE of a var that already has a point.

use crate::artifacts::rewrite::diff::RewriteDiff;
use crate::artifacts::rewrite::mutations::RewriteRuleMutation;
use crate::artifacts::rewrite::LayoutPoint;
use crate::artifacts::rewrite::RewriteSnapshot;
use crate::artifacts::rewrite::{apply_rewrite_rule_mutation, inverse_rewrite_rule_mutation};

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

/// ▶️ `change-rule-layout-point` carries `before` to exactly the committed `after` by upserting ONE key
/// of the `rule_layout` map, leaving the other var's point exactly where it sat.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    let mut snapshot = base.clone();
    apply_rewrite_rule_mutation(&mut snapshot, &mutation()).expect("change-rule-layout-point applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "change-rule-layout-point/nudges-the-capsule-var-off-the-shaft: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.rule_layout.get("c"), Some(&LayoutPoint { x: 24.0, y: 16.5 }), "change-rule-layout-point must upsert the addressed var to the payload's point");
    assert_eq!(snapshot.rule_layout.get("shaft"), Some(&LayoutPoint { x: 0.0, y: 0.0 }), "the other var's point must survive a single-key upsert untouched");
    assert_eq!(snapshot.rule_layout.len(), base.rule_layout.len(), "moving an existing var must not change the map's cardinality");
    assert_eq!(snapshot.parameter_bindings, base.parameter_bindings, "change-rule-layout-point must never reach the OTHER key-addressed map");
}

/// ↩️ `change-rule-layout-point` inverts BASE-derived and BRANCHES: an existing key inverts to a
/// `change` back to the old point, an absent one to a `remove`. This case moves an EXISTING var, so
/// the inverse must be the `change` arm carrying BASE's own point.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_rewrite_rule_mutation(&base, &mutation);
    assert_eq!(inverse.len(), 1, "change-rule-layout-point always undoes with exactly one step, got {inverse:?}");
    let RewriteRuleMutation::ChangeRuleLayoutPoint(undo) = &inverse[0] else {
        panic!("moving an EXISTING var must invert to another change-rule-layout-point, not a remove, got {:?}", inverse[0]);
    };
    assert_eq!(undo.key, "c", "the inverse addresses exactly the var the payload addressed");
    assert_eq!(undo.new_point, LayoutPoint { x: 12.0, y: -8.0 }, "the inverse restores exactly the point BASE held for that var");
    let mut snapshot = base.clone();
    apply_rewrite_rule_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_rewrite_rule_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "change-rule-layout-point/nudges-the-capsule-var-off-the-shaft: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-rule-layout-point` payload are already canonical:
/// decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RewriteSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-rule-layout-point/nudges-the-capsule-var-off-the-shaft: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-rule-layout-point/nudges-the-capsule-var-off-the-shaft: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome is exactly what `change-rule-layout-point` produces here — an applied change with no
/// diagnostics at all — and this fixture is bound to `change-rule-layout-point`'s own semantic descriptor.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-rule-layout-point/nudges-the-capsule-var-off-the-shaft declares an applied outcome");
    let mut snapshot = before();
    apply_rewrite_rule_mutation(&mut snapshot, &mutation()).expect("change-rule-layout-point/nudges-the-capsule-var-off-the-shaft: declared applied but the mutation was rejected");
    let produced = <RewriteRuleMutation as protocol::Mutation<RewriteSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "change-rule-layout-point/nudges-the-capsule-var-off-the-shaft declares no diagnostics, but got {:?}", produced.messages());
    let semantics = <RewriteRuleMutation as protocol::SemanticMutation<RewriteSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("change", "rule-layout-point", "change-rule-layout-point", "ChangedRuleLayoutPoint"), "the fixture must be bound to change-rule-layout-point's own descriptor");
}

/// 🔺️ The sparse delta is exactly the committed diff: a `rule_layout` map holding ONE key mapped to
/// `Some(LayoutPoint)`, and the `parameter_bindings` slot left entirely alone.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <RewriteRuleMutation as protocol::Mutation<RewriteSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-rule-layout-point/nudges-the-capsule-var-off-the-shaft: produced diff differs from the committed 🔺️diff/🔣️component.json");
    let typed: RewriteDiff = serde_json::from_str(DIFF).expect("committed diff decodes into RewriteDiff");
    let layout = typed.rule_layout.as_ref().expect("change-rule-layout-point's delta carries a rule_layout map");
    assert_eq!(layout.len(), 1, "a single-var move must appear in the delta as exactly one entry, got {layout:?}");
    assert_eq!(layout.get("c"), Some(&Some(LayoutPoint { x: 24.0, y: 16.5 })), "the delta maps the addressed var to Some(new point)");
    assert!(typed.parameter_bindings.is_none(), "change-rule-layout-point must never reach the parameter_bindings slot");
    assert!(typed.lhs_json.is_none() && typed.rhs_json.is_none() && typed.before_fixture_json.is_none(), "a layout move must not disturb any authored body");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own `RewriteDiff`.
/// `RewriteDiff` carries a container-level `#[serde(default)]` and NO per-field
/// `skip_serializing_if`, so all nine sparse slots — including the presence/config-lane ones
/// `change-rule-layout-point` never touches — must be present as `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: RewriteDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-rule-layout-point/nudges-the-capsule-var-off-the-shaft: committed diff JSON is not canonical");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    let slots = committed.as_object().expect("the committed diff is a JSON object");
    assert_eq!(slots.len(), 9, "RewriteDiff emits all nine sparse slots, got {slots:?}");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is a
/// complete description of the `change-rule-layout-point` change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: RewriteDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <RewriteDiff as protocol::MutationDiff<RewriteSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-rule-layout-point/nudges-the-capsule-var-off-the-shaft: committed diff did not carry before to after");
}
