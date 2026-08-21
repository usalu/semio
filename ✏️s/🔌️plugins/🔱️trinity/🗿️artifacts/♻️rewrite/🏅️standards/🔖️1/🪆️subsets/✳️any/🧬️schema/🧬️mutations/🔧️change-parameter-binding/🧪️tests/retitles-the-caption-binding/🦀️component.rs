//! 🧪️ `change-parameter-binding` fixture — `retitles-the-caption-binding`.
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
//! 🔧️ `change-parameter-binding` is a KEY-ADDRESSED upsert on the `parameter_bindings` map, so unlike
//! the three whole-body edits it carries a real `target()` (the key). A key it cannot find is not an
//! error here — an upsert simply inserts — so its only non-applying branch is `mutation.no-op` for a
//! value that already matches. This case pins the applied UPDATE of an existing key.

use crate::artifacts::rewrite::diff::RewriteDiff;
use crate::artifacts::rewrite::mutations::{apply_rewrite_rule_mutation, inverse_rewrite_rule_mutation, RewriteRuleMutation};
use crate::artifacts::rewrite::RewriteSnapshot;
use crate::artifacts::jack::PropertyValue;

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

/// ▶️ `change-parameter-binding` carries `before` to exactly the committed `after` by upserting ONE key
/// of the `parameter_bindings` map, leaving its sibling key and both other maps' contents alone.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    let mut snapshot = base.clone();
    apply_rewrite_rule_mutation(&mut snapshot, &mutation()).expect("change-parameter-binding applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "change-parameter-binding/retitles-the-caption-binding: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.parameter_bindings.get("caption"), Some(&PropertyValue::String("Capsule Tower".into())), "change-parameter-binding must upsert the addressed key to the payload's value");
    assert_eq!(snapshot.parameter_bindings.get("repeat"), base.parameter_bindings.get("repeat"), "the sibling binding must survive a single-key upsert untouched");
    assert_eq!(snapshot.parameter_bindings.len(), base.parameter_bindings.len(), "updating an existing key must not change the map's cardinality");
    assert_eq!(snapshot.rule_layout, base.rule_layout, "change-parameter-binding must never reach the OTHER key-addressed map");
}

/// ↩️ `change-parameter-binding` inverts BASE-derived and BRANCHES on whether the key existed: an
/// existing key inverts to a `change` back to the old value, an absent one to a `remove`. This case
/// updates an EXISTING key, so the inverse must be the `change` arm.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_rewrite_rule_mutation(&base, &mutation);
    assert_eq!(inverse.len(), 1, "change-parameter-binding always undoes with exactly one step, got {inverse:?}");
    let RewriteRuleMutation::ChangeParameterBinding(undo) = &inverse[0] else {
        panic!("updating an EXISTING binding must invert to another change-parameter-binding, not a remove, got {:?}", inverse[0]);
    };
    assert_eq!(undo.key, "caption", "the inverse addresses exactly the key the payload addressed");
    assert_eq!(Some(&undo.new_value), base.parameter_bindings.get("caption"), "the inverse restores exactly the value BASE held under that key");
    let mut snapshot = base.clone();
    apply_rewrite_rule_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_rewrite_rule_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "change-parameter-binding/retitles-the-caption-binding: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-parameter-binding` payload are already canonical:
/// decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RewriteSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-parameter-binding/retitles-the-caption-binding: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-parameter-binding/retitles-the-caption-binding: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome is exactly what `change-parameter-binding` produces here — an applied change with no
/// diagnostics at all — and this fixture is bound to `change-parameter-binding`'s own semantic descriptor.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-parameter-binding/retitles-the-caption-binding declares an applied outcome");
    let mut snapshot = before();
    apply_rewrite_rule_mutation(&mut snapshot, &mutation()).expect("change-parameter-binding/retitles-the-caption-binding: declared applied but the mutation was rejected");
    let produced = <RewriteRuleMutation as protocol::Mutation<RewriteSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "change-parameter-binding/retitles-the-caption-binding declares no diagnostics, but got {:?}", produced.messages());
    let semantics = <RewriteRuleMutation as protocol::SemanticMutation<RewriteSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("change", "parameter-binding", "change-parameter-binding", "ChangedParameterBinding"), "the fixture must be bound to change-parameter-binding's own descriptor");
}

/// 🔺️ The sparse delta is exactly the committed diff: a `parameter_bindings` map holding ONE key,
/// mapped to `Some(value)` — an upsert, never a whole-map replacement.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <RewriteRuleMutation as protocol::Mutation<RewriteSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-parameter-binding/retitles-the-caption-binding: produced diff differs from the committed 🔺️diff/🔣️component.json");
    let typed: RewriteDiff = serde_json::from_str(DIFF).expect("committed diff decodes into RewriteDiff");
    let bindings = typed.parameter_bindings.as_ref().expect("change-parameter-binding's delta carries a parameter_bindings map");
    assert_eq!(bindings.len(), 1, "a single-key upsert must appear in the delta as exactly one entry, got {bindings:?}");
    assert_eq!(bindings.get("caption"), Some(&Some(PropertyValue::String("Capsule Tower".into()))), "the delta maps the addressed key to Some(new value)");
    assert!(typed.rule_layout.is_none(), "change-parameter-binding must never reach the rule_layout slot");
    assert!(typed.lhs_json.is_none() && typed.rhs_json.is_none() && typed.before_fixture_json.is_none(), "a map upsert must not disturb any authored body");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own `RewriteDiff`.
/// `RewriteDiff` carries a container-level `#[serde(default)]` and NO per-field
/// `skip_serializing_if`, so all nine sparse slots — including the presence/config-lane ones
/// `change-parameter-binding` never touches — must be present as `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: RewriteDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-parameter-binding/retitles-the-caption-binding: committed diff JSON is not canonical");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    let slots = committed.as_object().expect("the committed diff is a JSON object");
    assert_eq!(slots.len(), 9, "RewriteDiff emits all nine sparse slots, got {slots:?}");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is a
/// complete description of the `change-parameter-binding` change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: RewriteDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <RewriteDiff as protocol::MutationDiff<RewriteSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-parameter-binding/retitles-the-caption-binding: committed diff did not carry before to after");
}
