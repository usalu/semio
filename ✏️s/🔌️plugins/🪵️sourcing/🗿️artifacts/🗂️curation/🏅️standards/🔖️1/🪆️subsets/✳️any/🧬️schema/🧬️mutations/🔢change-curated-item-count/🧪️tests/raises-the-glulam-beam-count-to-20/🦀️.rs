//! 🧪️ `change-curated-item-count` fixture — `raises-the-glulam-beam-count-to-20`.
//!
//! `change-curated-item-count`'s diff oracle has TWO guards — Error `mutation.target-missing` when
//! the object is not curated, Warning `mutation.no-op` when the requested count already stands —
//! and otherwise emits a `CurationCuratedDelta` carrying ONE `patched` entry. `count` is set to a
//! FINAL value, never a delta, and the patch entry addresses the row by `objectId` rather than by
//! position, so the untouched sibling never appears in the diff.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`); the derived encodings come from `fixtures generate`.

use crate::artifacts::curation::diff::CurationDiff;
use crate::artifacts::curation::mutations::SourcingMutation;
use crate::artifacts::curation::CurationSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> CurationSnapshot {
    serde_json::from_str(BEFORE).expect("before curation document decodes")
}
fn expected_after() -> CurationSnapshot {
    serde_json::from_str(AFTER).expect("after curation document decodes")
}
fn mutation() -> SourcingMutation {
    serde_json::from_str(MUTATION).expect("change-curated-item-count mutation decodes")
}
fn built_outcome() -> protocol::MutationOutcome<CurationDiff> {
    <SourcingMutation as protocol::Mutation<CurationSnapshot>>::diff(&mutation(), &before())
}

/// ▶️ Raising the glulam beam from 12 to 20 sets a FINAL count — not `+8` — and leaves the CLT
/// panel's own count of 4 and the row order untouched.
#[semio_framework_async_macros::async_test]
async fn sets_the_final_count_on_the_addressed_row() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-curated-item-count applies to its committed before-document");
    assert_eq!(applied, expected_after(), "change-curated-item-count/raises-the-glulam-beam-count-to-20: the recounted curation differs from the committed after-snapshot");
    assert_eq!(applied.curated[0].count, 20, "change-curated-item-count/raises-the-glulam-beam-count-to-20: newCount is an absolute target, never an increment");
    assert_eq!(applied.curated[1].count, 4, "change-curated-item-count/raises-the-glulam-beam-count-to-20: the sibling pick's count must not move");
}

/// ↩️ `change-curated-item-count`'s inverse restores the OLD count read out of BASE — `12` — never
/// a structural inversion of the patch entry.
#[semio_framework_async_macros::async_test]
async fn recounting_to_the_base_value_restores_before() {
    let base = before();
    let mut snapshot = protocol::MutationDiff::apply(built_outcome().diff(), &base).expect("forward change-curated-item-count applies");
    let inverse = <SourcingMutation as protocol::Mutation<CurationSnapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-curated-item-count/raises-the-glulam-beam-count-to-20: the inverse of one recount is exactly one recount back");
    for step in &inverse {
        let undo = <SourcingMutation as protocol::Mutation<CurationSnapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-curated-item-count inverse step applies");
    }
    assert_eq!(snapshot, base, "change-curated-item-count/raises-the-glulam-beam-count-to-20: recounting back to 12 did not restore the before-document");
}

/// 🔣️ Both committed documents and the `changeCuratedItemCount` payload are canonical.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: CurationSnapshot = serde_json::from_str(text).expect("curation document decodes");
        let reencoded = serde_json::to_value(&decoded).expect("curation document encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("curation document reparses");
        assert_eq!(reencoded, original, "change-curated-item-count/raises-the-glulam-beam-count-to-20: committed {label} document JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("changeCuratedItemCount payload encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("changeCuratedItemCount payload reparses");
    assert_eq!(reencoded, original, "change-curated-item-count/raises-the-glulam-beam-count-to-20: committed changeCuratedItemCount JSON is not canonical");
}

/// 🎯️ The beam is curated (so no `mutation.target-missing`) and 20 differs from 12 (so no
/// `mutation.no-op`) — both of this oracle's guards must stay silent.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-curated-item-count/raises-the-glulam-beam-count-to-20: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "change-curated-item-count/raises-the-glulam-beam-count-to-20: a curated object recounted to a new value trips neither guard");
    assert!(produced.messages().is_empty(), "change-curated-item-count/raises-the-glulam-beam-count-to-20: an accepted recount emits no diagnostics");
}

/// 🔺️ The committed diff pins the sparseness: one `patched` entry keyed by `objectId`, `added`/
/// `removed` empty, `reordered` null — a recount must never be expressed as remove-then-add.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("produced change-curated-item-count diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-curated-item-count/raises-the-glulam-beam-count-to-20: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to `CurationDiff` and re-encodes unchanged — the patch entry's
/// `count` is an `Option<u32>` with no skip attribute, so it is emitted as a real number.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: CurationDiff = serde_json::from_str(DIFF).expect("committed change-curated-item-count diff decodes");
    let delta = decoded.curated.as_ref().expect("the committed recount diff carries a curated delta");
    assert_eq!(delta.patched.len(), 1, "change-curated-item-count/raises-the-glulam-beam-count-to-20: exactly one row is patched");
    assert_eq!((delta.patched[0].object_id.as_str(), delta.patched[0].count), ("beam-glulam-240", Some(20)), "change-curated-item-count/raises-the-glulam-beam-count-to-20: the patch must address the beam and carry its final count");
    assert!(delta.added.is_empty() && delta.removed.is_empty(), "change-curated-item-count/raises-the-glulam-beam-count-to-20: a recount must not be expressed as remove-then-add");
    let reencoded = serde_json::to_value(&decoded).expect("committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-curated-item-count/raises-the-glulam-beam-count-to-20: committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-document to the after-document.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: CurationDiff = serde_json::from_str(DIFF).expect("committed change-curated-item-count diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("committed diff applies to the before-document");
    assert_eq!(produced, expected_after(), "change-curated-item-count/raises-the-glulam-beam-count-to-20: committed diff did not carry before to after");
}
