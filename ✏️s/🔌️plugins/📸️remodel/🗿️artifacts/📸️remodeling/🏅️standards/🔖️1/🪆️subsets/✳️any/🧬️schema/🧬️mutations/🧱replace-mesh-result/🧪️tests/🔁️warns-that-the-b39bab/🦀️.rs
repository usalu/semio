//! 🧪️ `replace-mesh-result` fixture — `🔁️warns-that-the-b39bab`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). `replace-mesh-result-noop` is this vector's scenario id in
//! `../../../../🧪️tests/📸️mutate-remodeling-1/🥒️.feature`, where the same bytes are replayed against
//! this subset's independent Python reference.
//!
//! 🏞️ the staging guard is checked BEFORE the no-op, so an identical staged handle is still refused

use crate::artifacts::remodeling::mutations::{apply_remodeling_mutation, inverse_remodeling_mutation, RemodelingMutation};
use crate::artifacts::remodeling::{RemodelingDiff, RemodelingSnapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");

fn before() -> RemodelingSnapshot {
    pack::from_json_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> RemodelingSnapshot {
    pack::from_json_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> RemodelingMutation {
    pack::from_json_str(MUTATION).expect("mutation decodes")
}
fn produced() -> protocol::MutationOutcome<RemodelingDiff> {
    <RemodelingMutation as protocol::Mutation<RemodelingSnapshot>>::diff(&mutation(), &before())
}
fn json_of<T: dsl::ToValue>(value: &T) -> pack::JsonValue {
    pack::json_from_dsl_value(&dsl::ToValue::to_value(value))
}

/// 🔁️ A warned no-op leaves the document byte-identical to its committed after-document, which for
/// a no-op IS the before-document — and, unlike a refusal, it is still an APPLIED outcome.
#[semio_framework_async_macros::async_test]
async fn no_op_leaves_the_document_untouched() {
    let base = before();
    let applied = apply_remodeling_mutation(&base, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(applied, expected_after(), "replace-mesh-result/warns-that-the-b39bab: applied state differs from committed after-snapshot");
    assert_eq!(applied, base, "replace-mesh-result/warns-that-the-b39bab: a no-op must not move the document");
}

/// 🎯️ The declared Warning — not an Error — is what this leaf emits, over an empty diff that is
/// still committed as a real all-null delta rather than as an absent one.
#[semio_framework_async_macros::async_test]
async fn declared_warning_holds() {
    let declared = pack::parse_json(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(|status| status.as_str()), Some("applied"), "replace-mesh-result/warns-that-the-b39bab declares an applied outcome");
    let produced = produced();
    assert_eq!(produced.diff(), &RemodelingDiff::default(), "replace-mesh-result/warns-that-the-b39bab: a no-op leaf must carry an empty diff");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "replace-mesh-result/warns-that-the-b39bab: exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.no-op", "replace-mesh-result/warns-that-the-b39bab: an identical resubmission is a no-op");
    assert_eq!(messages[0].level, protocol::Severity::Warning, "replace-mesh-result/warns-that-the-b39bab: a no-op is a Warning, never an Error");
    let committed = pack::parse_json(DIFF).expect("committed diff decodes");
    assert_eq!(json_of(produced.diff()), committed, "replace-mesh-result/warns-that-the-b39bab: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// ↩️ Applying the verb and then EVERY step of its own computed inverse restores the committed
/// before-document — member positions included, which a delete undone by re-appending would fail.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_before_document() {
    let base = before();
    let mut snapshot = apply_remodeling_mutation(&base, &mutation()).expect("forward applies");
    for step in &inverse_remodeling_mutation(&base, &mutation()) {
        snapshot = apply_remodeling_mutation(&snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "replace-mesh-result/warns-that-the-b39bab: inverse did not restore the before-snapshot");
}

/// 🔣️ The committed snapshots and the committed mutation are already canonical: decode→encode is a
/// fixed point over `pack::json`, which is the codec the crate uses since serde left this type graph.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RemodelingSnapshot = pack::from_json_str(text).expect("snapshot decodes");
        let original = pack::parse_json(text).expect("snapshot reparses");
        assert_eq!(json_of(&decoded), original, "replace-mesh-result/warns-that-the-b39bab: committed {label} JSON is not canonical");
    }
    let original = pack::parse_json(MUTATION).expect("mutation reparses");
    assert_eq!(json_of(&mutation()), original, "replace-mesh-result/warns-that-the-b39bab: committed mutation JSON is not canonical");
}
