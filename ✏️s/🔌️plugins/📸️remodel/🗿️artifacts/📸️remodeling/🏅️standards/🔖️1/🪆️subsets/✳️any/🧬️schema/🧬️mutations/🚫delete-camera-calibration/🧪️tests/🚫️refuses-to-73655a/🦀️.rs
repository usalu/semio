//! 🧪️ `delete-camera-calibration` fixture — `🚫️refuses-to-73655a`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). `delete-camera-calibration-missing` is this vector's scenario id in
//! `../../../../🧪️tests/📸️mutate-remodeling-1/🥒️.feature`, where the same bytes are replayed against
//! this subset's independent Python reference.
//!
//! 🏞️ the missing-target guard runs before the referential one

use crate::artifacts::remodeling::mutations::{apply_remodeling_mutation, inverse_remodeling_mutation, RemodelingMutation};
use crate::artifacts::remodeling::{RemodelingDiff, RemodelingSnapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

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

/// 🚫️ A refused `delete-camera-calibration` leaves the document byte-identical to its committed after-document, which
/// for a refusal IS the before-document.
#[semio_framework_async_macros::async_test]
async fn refusal_leaves_the_document_untouched() {
    let base = before();
    let applied = apply_remodeling_mutation(&base, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(applied, expected_after(), "delete-camera-calibration/refuses-to-73655a: applied state differs from committed after-snapshot");
    assert_eq!(applied, base, "delete-camera-calibration/refuses-to-73655a: a refused mutation must not move the document");
}

/// 🎯️ The declared refusal — status, code, level and diagnostic target — is exactly what this
/// leaf's own guard emits, and the diff it carries is empty rather than half-built.
#[semio_framework_async_macros::async_test]
async fn declared_refusal_holds() {
    let declared = pack::parse_json(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(|status| status.as_str()), Some("rejected"), "delete-camera-calibration/refuses-to-73655a declares a rejected outcome");
    let produced = produced();
    assert_eq!(produced.diff(), &RemodelingDiff::default(), "delete-camera-calibration/refuses-to-73655a: a refusing leaf must carry an empty diff");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "delete-camera-calibration/refuses-to-73655a: exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.target-missing", "delete-camera-calibration/refuses-to-73655a: the declared code must be the emitted one");
    assert_eq!(messages[0].level, protocol::Severity::Error, "delete-camera-calibration/refuses-to-73655a: the declared level must be the emitted one");
    assert_eq!(declared.get("code").and_then(|code| code.as_str()), Some(messages[0].code.0.as_str()), "the committed outcome must name the emitted code");
    let declared_path: Vec<String> = match declared.get("path") {
        Some(pack::JsonValue::Array(entries)) => entries.iter().filter_map(|entry| entry.as_str().map(str::to_string)).collect(),
        _ => Vec::new(),
    };
    assert_eq!(declared_path, messages[0].target, "delete-camera-calibration/refuses-to-73655a: the declared path must be the emitted target");
}

/// 🚫️ A refusal ships no `🔺️diff/🔣️.json` at all — the `🚫️.absent` marker beside it is the
/// repository's own statement that there is no delta to commit, not a forgotten file.
#[semio_framework_async_macros::async_test]
async fn no_diff_is_committed() {
    assert!(include_str!("🔺️diff/🚫️.absent").is_empty(), "delete-camera-calibration/refuses-to-73655a: the absent-diff marker must stay empty");
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
    assert_eq!(snapshot, base, "delete-camera-calibration/refuses-to-73655a: inverse did not restore the before-snapshot");
}

/// 🔣️ The committed snapshots and the committed mutation are already canonical: decode→encode is a
/// fixed point over `pack::json`, which is the codec the crate uses since serde left this type graph.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RemodelingSnapshot = pack::from_json_str(text).expect("snapshot decodes");
        let original = pack::parse_json(text).expect("snapshot reparses");
        assert_eq!(json_of(&decoded), original, "delete-camera-calibration/refuses-to-73655a: committed {label} JSON is not canonical");
    }
    let original = pack::parse_json(MUTATION).expect("mutation reparses");
    assert_eq!(json_of(&mutation()), original, "delete-camera-calibration/refuses-to-73655a: committed mutation JSON is not canonical");
}
