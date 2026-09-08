//! 🧪️ `create-asset` fixture — `🖼️stores-a-new-d56283`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). `create-asset` is this vector's scenario id in
//! `../../../../🧪️tests/📸️mutate-remodeling-1/🥒️.feature`, where the same bytes are replayed against
//! this subset's independent Python reference.
//!
//! 🏞️ the pre-existing two-stream unit vector, regenerated onto the corrected toy base

use crate::mutations::{apply_remodeling_mutation, inverse_remodeling_mutation, RemodelingMutation};
use crate::{RemodelingDiff, RemodelingSnapshot};

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

/// ▶️ The verb reaches its committed after-document, and moved it.
#[semio_framework_async_macros::async_test]
async fn reaches_the_committed_after_document() {
    let applied = apply_remodeling_mutation(&before(), &mutation()).expect("create-asset applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "create-asset/stores-a-new-d56283: applied state differs from committed after-snapshot");
    assert_ne!(applied, before(), "create-asset/stores-a-new-d56283: an applied vector must move the document");
}

/// 🔺️ The sparse delta this leaf produces is EXACTLY the committed diff — the load-bearing
/// assertion, because it pins WHICH lanes the verb is allowed to touch, not merely the end state.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = produced();
    let committed = pack::parse_json(DIFF).expect("committed diff decodes");
    assert_eq!(json_of(outcome.diff()), committed, "create-asset/stores-a-new-d56283: produced diff differs from the committed 🔺️diff/🔣️.json");
    let decoded: RemodelingDiff = pack::from_json_str(DIFF).expect("committed diff decodes into the diff type");
    assert_eq!(json_of(&decoded), committed, "create-asset/stores-a-new-d56283: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the delta is a COMPLETE
/// description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_carries_before_to_after() {
    let decoded: RemodelingDiff = pack::from_json_str(DIFF).expect("committed diff decodes");
    let applied = <RemodelingDiff as protocol::MutationDiff<RemodelingSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(applied, expected_after(), "create-asset/stores-a-new-d56283: committed diff did not carry before to after");
}

/// 🎯️ The declared outcome — its status and every diagnostic it names — is what this leaf emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared = pack::parse_json(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(|status| status.as_str()), Some("applied"), "create-asset/stores-a-new-d56283 declares an applied outcome");
    let produced = produced();
    let declared_codes: Vec<String> = match declared.get("messages") {
        Some(pack::JsonValue::Array(entries)) => entries.iter().filter_map(|entry| entry.get("code").and_then(|code| code.as_str()).map(str::to_string)).collect(),
        _ => Vec::new(),
    };
    let emitted: Vec<String> = produced.messages().iter().map(|message| message.code.0.clone()).collect();
    assert_eq!(emitted, declared_codes, "create-asset/stores-a-new-d56283: emitted diagnostics differ from the declared ones");
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
    assert_eq!(snapshot, base, "create-asset/stores-a-new-d56283: inverse did not restore the before-snapshot");
}

/// 🔣️ The committed snapshots and the committed mutation are already canonical: decode→encode is a
/// fixed point over `pack::json`, which is the codec the crate uses since serde left this type graph.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RemodelingSnapshot = pack::from_json_str(text).expect("snapshot decodes");
        let original = pack::parse_json(text).expect("snapshot reparses");
        assert_eq!(json_of(&decoded), original, "create-asset/stores-a-new-d56283: committed {label} JSON is not canonical");
    }
    let original = pack::parse_json(MUTATION).expect("mutation reparses");
    assert_eq!(json_of(&mutation()), original, "create-asset/stores-a-new-d56283: committed mutation JSON is not canonical");
}
