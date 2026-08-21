//! 🧪️ `📄set-snapshot` fixture — `bumps-the-version-lexeme-and-appends-a-tag`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`.
//!
//! 🔣️ The case rewrites the `version` member's NUMBER LEXEME (`"1"` → `"2"`, never an `f64` — the
//! snapshot keeps arbitrary precision on purpose) and appends a third element to the `tags` array,
//! leaving the `name` member alone — so `JsonDiff::between` must nest a name-keyed object triple
//! around an index-keyed array triple and must NOT fall back to `JsonValueDiff::Replace`.

use crate::artifacts::json::schema::diff::{JsonDiff, JsonValueDiff};
use crate::artifacts::json::schema::mutations::{apply_json_mutation, JsonMutation};
use crate::artifacts::json::JsonSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> JsonSnapshot {
    serde_json::from_str(BEFORE).expect("before JSON document snapshot decodes")
}
fn expected_after() -> JsonSnapshot {
    serde_json::from_str(AFTER).expect("after JSON document snapshot decodes")
}
fn mutation() -> JsonMutation {
    serde_json::from_str(MUTATION).expect("set-snapshot mutation decodes")
}

/// ▶️ `set-snapshot` carries the document to exactly the committed `after`: version lexeme `"2"`
/// and a three-element `tags` array.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    let outcome = apply_json_mutation(&mut snapshot, &mutation());
    assert!(outcome.messages().is_empty(), "json/set-snapshot: a genuinely changed document must not raise any message");
    assert_eq!(snapshot, expected_after(), "json/set-snapshot: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse of `set-snapshot` is `set-snapshot(base)` — it must restore the `"1"` lexeme and
/// drop the appended tag, member insertion order included.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <JsonMutation as protocol::Mutation<JsonSnapshot>>::inverse(&mutation, &base);
    let mut snapshot = base.clone();
    apply_json_mutation(&mut snapshot, &mutation);
    for step in &inverse {
        apply_json_mutation(&mut snapshot, step);
    }
    assert_eq!(snapshot, base, "json/set-snapshot: inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed snapshots and the mutation are already canonical: `JsonValue` is internally
/// tagged on `kind`, every variant is a STRUCT variant (a tuple variant would fail serde's
/// internally-tagged flattening at runtime), and numbers keep their source lexeme as a string.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: JsonSnapshot = serde_json::from_str(text).expect("JSON document snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("JSON document snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("JSON document snapshot reparses");
        assert_eq!(reencoded, original, "json/set-snapshot: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("set-snapshot mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("set-snapshot mutation reparses");
    assert_eq!(reencoded, original, "json/set-snapshot: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome is `applied` — the document really moves, so the `mutation.no-op`
/// warning an identical set-snapshot would raise never appears.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    assert_eq!(status, "applied", "json/set-snapshot: this fixture declares an applied outcome");
    let mut snapshot = before();
    let produced = apply_json_mutation(&mut snapshot, &mutation());
    assert!(produced.messages().is_empty(), "json/set-snapshot: declared applied, so no diagnostic may be raised");
    assert_ne!(snapshot, before(), "json/set-snapshot: an applied set-snapshot must actually move the document");
}

/// 🔺️ The sparse `JsonDiff` this mutation produces is exactly the committed diff — the
/// load-bearing assertion: the unchanged `name` member must never appear in `object.modified`, and
/// the two changed members must be reported in BASE member order (`version` before `tags`).
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <JsonMutation as protocol::Mutation<JsonSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced JSON diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed JSON diff decodes");
    assert_eq!(produced, committed, "json/set-snapshot: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to `JsonDiff` as a structural `Object`
/// delta — a `Replace` at the root would also reach the right end state and is exactly what this
/// assertion forbids.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: JsonDiff = serde_json::from_str(DIFF).expect("committed JSON diff decodes");
    let JsonValueDiff::Object { diff: object } = decoded.value.as_ref().expect("the committed diff patches the root value") else {
        panic!("json/set-snapshot: the root delta must be a structural object diff, never a wholesale replace");
    };
    assert!(object.removed.is_empty() && object.added.is_empty() && object.modified.len() == 2, "json/set-snapshot: exactly the two changed members may appear, and no member may be removed or re-added");
    let reencoded = serde_json::to_value(&decoded).expect("JSON diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed JSON diff reparses");
    assert_eq!(reencoded, original, "json/set-snapshot: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields the committed `after` — the nested
/// member + element delta is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: JsonDiff = serde_json::from_str(DIFF).expect("committed JSON diff decodes");
    let produced = <JsonDiff as protocol::MutationDiff<JsonSnapshot>>::apply(&decoded, &before()).expect("committed JSON diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "json/set-snapshot: committed diff did not carry before to after");
}
