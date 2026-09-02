//! 🧪️ `remove-parameter-binding` fixture — `drops-the-repeat-binding`.
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
//! 🧹️ `remove-parameter-binding` encodes a CLEAR as an explicit `None` under the key — never as an
//! omitted key, which the sparse-map apply would read as "no change". An already-absent key is a
//! `mutation.no-op` warning here, never `mutation.target-missing`: this map family has no rejection
//! branch at all. This case pins the applied removal of a key that really is present.

use crate::artifacts::jack::PropertyValue;
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

/// ▶️ `remove-parameter-binding` carries `before` to exactly the committed `after` by taking ONE key out
/// of the `parameter_bindings` map — shrinking it by exactly one entry.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    let mut snapshot = base.clone();
    apply_rewrite_rule_mutation(&mut snapshot, &mutation()).expect("remove-parameter-binding applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "remove-parameter-binding/drops-the-repeat-binding: applied state differs from committed after-snapshot");
    assert!(!snapshot.parameter_bindings.contains_key("repeat"), "remove-parameter-binding must actually take the addressed key out of the map");
    assert_eq!(snapshot.parameter_bindings.get("caption"), Some(&PropertyValue::String("Capsule".into())), "the sibling binding must survive a single-key removal untouched");
    assert_eq!(snapshot.parameter_bindings.len(), base.parameter_bindings.len() - 1, "a removal shrinks the map by exactly one entry");
    assert_eq!(snapshot.rule_layout, base.rule_layout, "remove-parameter-binding must never reach the OTHER key-addressed map");
}

/// ↩️ `remove-parameter-binding` inverts BASE-derived: a `change-parameter-binding` putting the removed
/// value back. The removed VALUE lives only on BASE — the payload carries just a key — so a
/// payload-derived inverse could not exist for this verb.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_rewrite_rule_mutation(&base, &mutation);
    assert_eq!(inverse.len(), 1, "removing a present binding always undoes with exactly one step, got {inverse:?}");
    let RewriteRuleMutation::ChangeParameterBinding(undo) = &inverse[0] else {
        panic!("remove-parameter-binding's inverse must be a change-parameter-binding, got {:?}", inverse[0]);
    };
    assert_eq!(undo.key, "repeat", "the inverse re-binds exactly the key the payload removed");
    assert_eq!(undo.new_value, PropertyValue::Number(4.0), "the inverse restores the numeric value only BASE knew");
    let mut snapshot = base.clone();
    apply_rewrite_rule_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_rewrite_rule_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "remove-parameter-binding/drops-the-repeat-binding: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `remove-parameter-binding` payload are already canonical:
/// decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RewriteSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "remove-parameter-binding/drops-the-repeat-binding: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "remove-parameter-binding/drops-the-repeat-binding: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome is exactly what `remove-parameter-binding` produces here — an applied change with no
/// diagnostics at all — and this fixture is bound to `remove-parameter-binding`'s own semantic descriptor.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "remove-parameter-binding/drops-the-repeat-binding declares an applied outcome");
    let mut snapshot = before();
    apply_rewrite_rule_mutation(&mut snapshot, &mutation()).expect("remove-parameter-binding/drops-the-repeat-binding: declared applied but the mutation was rejected");
    let produced = <RewriteRuleMutation as protocol::Mutation<RewriteSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "remove-parameter-binding/drops-the-repeat-binding declares no diagnostics, but got {:?}", produced.messages());
    let semantics = <RewriteRuleMutation as protocol::SemanticMutation<RewriteSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("remove", "parameter-binding", "remove-parameter-binding", "RemovedParameterBinding"), "the fixture must be bound to remove-parameter-binding's own descriptor");
}

/// 🔺️ The sparse delta is exactly the committed diff: a `parameter_bindings` map holding the removed
/// key mapped to `None` — the clear sentinel this artifact's `apply_map_delta` reads.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <RewriteRuleMutation as protocol::Mutation<RewriteSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "remove-parameter-binding/drops-the-repeat-binding: produced diff differs from the committed 🔺️diff/🔣️.json");
    let typed: RewriteDiff = serde_json::from_str(DIFF).expect("committed diff decodes into RewriteDiff");
    let bindings = typed.parameter_bindings.as_ref().expect("remove-parameter-binding's delta carries a parameter_bindings map");
    assert_eq!(bindings.len(), 1, "a single-key removal must appear in the delta as exactly one entry, got {bindings:?}");
    assert_eq!(bindings.get("repeat"), Some(&None), "a removal is encoded as an explicit None under the key, never as an omitted key");
    assert!(!bindings.contains_key("caption"), "the surviving binding must not appear in the delta at all");
    assert!(typed.rule_layout.is_none(), "remove-parameter-binding must never reach the rule_layout slot");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own `RewriteDiff`.
/// `RewriteDiff` carries a container-level `#[serde(default)]` and NO per-field
/// `skip_serializing_if`, so all nine sparse slots — including the presence/config-lane ones
/// `remove-parameter-binding` never touches — must be present as `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: RewriteDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "remove-parameter-binding/drops-the-repeat-binding: committed diff JSON is not canonical");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    let slots = committed.as_object().expect("the committed diff is a JSON object");
    assert_eq!(slots.len(), 9, "RewriteDiff emits all nine sparse slots, got {slots:?}");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is a
/// complete description of the `remove-parameter-binding` change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: RewriteDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <RewriteDiff as protocol::MutationDiff<RewriteSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "remove-parameter-binding/drops-the-repeat-binding: committed diff did not carry before to after");
}
