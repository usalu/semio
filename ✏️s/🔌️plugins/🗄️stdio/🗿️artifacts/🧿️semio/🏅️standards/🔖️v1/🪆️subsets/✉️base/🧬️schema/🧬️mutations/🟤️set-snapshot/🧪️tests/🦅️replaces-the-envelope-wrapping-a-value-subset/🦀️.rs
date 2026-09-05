//! 🧪️ `🟤️set-snapshot` fixture — `🦅️replaces-the-envelope-wrapping-a-value-subset`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`.
//!
//! 🧿️ `✉️base` is the ENVELOPE union, and `SetSnapshot` is the only mutation it owns that is not a
//! pass-through to one of the eighteen wrapped subsets. Its diff is therefore deliberately NOT a
//! sparse delta: `SemioMutation::diff` answers `SemioDiff::Replace(base)` unconditionally, because
//! set-snapshot is the only way the SUBSET KIND itself can change and no sparse representation for
//! "this artifact used to be a value, now it is a flow" exists. This fixture pins exactly that —
//! the committed diff is a whole `replace`, and asserting a per-field delta here would be wrong.

use crate::artifacts::semio::standards::v1::subsets::base::schema::diff::SemioDiff;
use crate::artifacts::semio::standards::v1::subsets::base::schema::mutations::{apply_semio_mutation, SemioMutation};
use crate::artifacts::semio::standards::v1::subsets::base::schema::snapshot::{SemioSnapshot, SemioSubsetSnapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> SemioSnapshot {
    serde_json::from_str(BEFORE).expect("before envelope snapshot decodes")
}
fn expected_after() -> SemioSnapshot {
    serde_json::from_str(AFTER).expect("after envelope snapshot decodes")
}
fn mutation() -> SemioMutation {
    serde_json::from_str(MUTATION).expect("set-snapshot mutation decodes")
}

/// ▶️ `set-snapshot` carries the envelope to exactly the committed `after`: the wrapped value graph
/// counts 42, and the envelope still declares the `value` subset.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    let outcome = apply_semio_mutation(&mut snapshot, &mutation());
    assert!(outcome.messages().is_empty(), "semio-any/set-snapshot: a genuinely changed envelope must not raise any message");
    assert!(matches!(snapshot.subset, SemioSubsetSnapshot::Value(_)), "semio-any/set-snapshot: this fixture keeps the wrapped subset kind stable");
    assert_eq!(snapshot, expected_after(), "semio-any/set-snapshot: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse of `set-snapshot` is `set-snapshot(base)` — the envelope's only undo, since a
/// wrapped subset's own inverses cannot express a kind change.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <SemioMutation as protocol::Mutation<SemioSnapshot>>::inverse(&mutation, &base);
    assert!(matches!(inverse.as_slice(), [SemioMutation::SetSnapshot(_)]), "semio-any/set-snapshot: the envelope's inverse is a single set-snapshot back to the base");
    let mut snapshot = base.clone();
    apply_semio_mutation(&mut snapshot, &mutation);
    for step in &inverse {
        apply_semio_mutation(&mut snapshot, step);
    }
    assert_eq!(snapshot, base, "semio-any/set-snapshot: inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed envelopes and the mutation are already canonical. Two envelope-specific wire
/// shapes are pinned: `SemioSubsetSnapshot` is INTERNALLY tagged on `subset`, so the wrapped
/// snapshot's own `schema` sits beside the `"subset": "value"` discriminator; and `SemioMutation`
/// is ADJACENTLY tagged (`mutation` + `payload`) precisely so a wrapped subset mutation's own
/// `"mutation"` key cannot collide with the envelope's.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioSnapshot = serde_json::from_str(text).expect("envelope snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("envelope snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("envelope snapshot reparses");
        assert_eq!(reencoded, original, "semio-any/set-snapshot: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("set-snapshot mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("set-snapshot mutation reparses");
    assert_eq!(reencoded, original, "semio-any/set-snapshot: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome is `applied` — the envelope really moves, so neither the
/// `mutation.no-op` warning nor the `mutation.target-missing` subset-mismatch error appears.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    assert_eq!(status, "applied", "semio-any/set-snapshot: this fixture declares an applied outcome");
    let mut snapshot = before();
    let produced = apply_semio_mutation(&mut snapshot, &mutation());
    assert!(produced.messages().is_empty(), "semio-any/set-snapshot: declared applied, so no diagnostic may be raised");
    assert_ne!(snapshot, before(), "semio-any/set-snapshot: an applied set-snapshot must actually move the envelope");
}

/// 🔺️ The diff this mutation produces is exactly the committed diff — the load-bearing assertion
/// for the envelope: it must be `SemioDiff::Replace` carrying the whole successor snapshot, NOT a
/// `SemioDiff::Value` delegating into the wrapped subset. Producing the narrower per-subset delta
/// would reach the same end state and would be wrong, because it cannot express a kind change.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioMutation as protocol::Mutation<SemioSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced envelope diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed envelope diff decodes");
    assert_eq!(produced, committed, "semio-any/set-snapshot: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to `SemioDiff::Replace` — internally
/// tagged on `kind`, so the boxed successor snapshot's own fields sit beside `"kind": "replace"`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: SemioDiff = serde_json::from_str(DIFF).expect("committed envelope diff decodes");
    let SemioDiff::Replace(replacement) = &decoded else {
        panic!("semio-any/set-snapshot: the envelope's set-snapshot diff must be a whole Replace, never a wrapped-subset delta");
    };
    assert_eq!(**replacement, expected_after(), "semio-any/set-snapshot: the Replace payload must be the committed after-snapshot itself");
    let reencoded = serde_json::to_value(&decoded).expect("envelope diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed envelope diff reparses");
    assert_eq!(reencoded, original, "semio-any/set-snapshot: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields the committed `after` — `Replace`
/// short-circuits the per-subset dispatch entirely and hands back its own payload.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioDiff = serde_json::from_str(DIFF).expect("committed envelope diff decodes");
    let produced = <SemioDiff as protocol::MutationDiff<SemioSnapshot>>::apply(&decoded, &before()).expect("committed envelope diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "semio-any/set-snapshot: committed diff did not carry before to after");
}
