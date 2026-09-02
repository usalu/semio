//! 🧪️ `duplicate-layer` fixture — `rejects-a-missing-source-layer`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Per contract D6 a rejected case carries
//! `🔺️diff/🚫️.absent` instead of a diff encoding, and `➡️after` is byte-identical to
//! `⬅️before`.
//!
//! ⚠️ Why this verb's covering case is the REJECTION branch and not a successful duplicate: the
//! duplicate's id is content-addressed through `create_draw_id`/`DefaultHasher`
//! (`🧬️schema/🦀️component.rs`, `draw_id_hex`). A hand-authored `➡️after` would have to embed that
//! hash, i.e. hand-forge a value produced by `std`'s deliberately unspecified default hasher — the
//! same class of forbidden hand-reimplementation the recipe bans for the binary codecs. The
//! `target-missing` branch reaches no hash at all, so it is the branch this fixture pins.

use crate::artifacts::draw::mutations::{apply_draw_mutation, inverse_draw_mutation, DrawMutation};
use crate::artifacts::draw::schema::find_draw_layer;
use crate::artifacts::draw::{DrawDiff, DrawSnapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> DrawSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> DrawSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> DrawMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ A rejected `duplicate-layer` leaves the document byte-identical: the committed `after` is the
/// committed `before`.
#[semio_framework_async_macros::async_test]
async fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    assert!(find_draw_layer(&base, "shape-missing").is_none(), "rejects-a-missing-source-layer's before-snapshot must NOT carry the addressed source");
    let mut snapshot = base.clone();
    apply_draw_mutation(&mut snapshot, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "duplicate-layer/rejects-a-missing-source-layer: applied state differs from committed after-snapshot");
    assert_eq!(snapshot, base, "a rejected duplicate must not touch the document");
}

/// 🧬️ `duplicate-layer` addresses only its SOURCE — it carries no id for the copy. With that source
/// absent, the diff builder reports `mutation.target-missing` against the source id and produces an
/// empty diff, never a partially-built clone.
#[semio_framework_async_macros::async_test]
async fn missing_source_is_reported_as_target_missing() {
    let produced = <DrawMutation as protocol::Mutation<DrawSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &DrawDiff::default(), "a rejecting duplicate-layer must carry an empty diff");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.target-missing", "duplicate-layer reports a missing source as target-missing");
    assert_eq!(messages[0].level, protocol::Severity::Error, "a missing source is an Error, not a Fatal — the duplicate-id guard is the Fatal one");
    assert_eq!(messages[0].target, vec!["shape-missing".to_string()], "the diagnostic addresses the payload's own layer_id");
}

/// ↩️ With no source to copy there is nothing to undo, so the inverse is empty — not a `delete-layer`
/// of an id that was never minted.
#[semio_framework_async_macros::async_test]
async fn inverse_has_nothing_to_undo() {
    let inverse = inverse_draw_mutation(&before(), &mutation());
    assert!(inverse.is_empty(), "duplicate-layer/rejects-a-missing-source-layer: a rejected duplicate must have no inverse steps, got {inverse:?}");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: DrawSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "duplicate-layer/rejects-a-missing-source-layer: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "duplicate-layer/rejects-a-missing-source-layer: committed mutation JSON is not canonical");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("rejected"), "duplicate-layer/rejects-a-missing-source-layer declares a rejected outcome");
    let produced = <DrawMutation as protocol::Mutation<DrawSnapshot>>::diff(&mutation(), &before());
    let message = produced.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(serde_json::Value::as_str), Some(message.code.0.as_str()), "the declared code must match the emitted one");
    let declared_path: Vec<String> = outcome.get("path").and_then(serde_json::Value::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared_path, message.target, "the declared path must match the emitted target");
}
