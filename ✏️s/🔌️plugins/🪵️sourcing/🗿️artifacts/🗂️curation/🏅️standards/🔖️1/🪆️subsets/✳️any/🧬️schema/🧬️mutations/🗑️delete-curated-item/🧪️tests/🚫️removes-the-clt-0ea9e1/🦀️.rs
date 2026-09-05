//! 🧪️ `delete-curated-item` fixture — `🚫️removes-the-clt-panel-from-the-curation`.
//!
//! `delete-curated-item`'s diff oracle guards a single condition — the object is not curated ⇒
//! Error `mutation.target-missing` — and otherwise emits a `CurationCuratedDelta` carrying ONE
//! `removed` id. The payload deliberately does NOT carry the removed count; that value is recovered
//! from BASE at inverse time, which is what makes the round trip lossless.
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
    serde_json::from_str(MUTATION).expect("delete-curated-item mutation decodes")
}
fn built_outcome() -> protocol::MutationOutcome<CurationDiff> {
    <SourcingMutation as protocol::Mutation<CurationSnapshot>>::diff(&mutation(), &before())
}

/// ▶️ Un-curating `panel-clt-3000` filters that one row out; the glulam beam keeps its count of 12.
#[semio_framework_async_macros::async_test]
async fn filters_the_named_pick_out_of_the_curation() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("delete-curated-item applies to its committed before-document");
    assert_eq!(applied, expected_after(), "delete-curated-item/removes-the-clt-panel-from-the-curation: the shortened curation differs from the committed after-snapshot");
    assert!(!applied.curated.iter().any(|item| item.object_id == "panel-clt-3000"), "delete-curated-item/removes-the-clt-panel-from-the-curation: the CLT panel survived its own removal");
}

/// ↩️ `delete-curated-item`'s inverse re-creates the FULL row read out of BASE — count `4` included,
/// even though the payload never carried it.
#[semio_framework_async_macros::async_test]
async fn recreating_the_removed_pick_restores_its_count() {
    let base = before();
    let mut snapshot = protocol::MutationDiff::apply(built_outcome().diff(), &base).expect("forward delete-curated-item applies");
    let inverse = <SourcingMutation as protocol::Mutation<CurationSnapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "delete-curated-item/removes-the-clt-panel-from-the-curation: the inverse of one delete is exactly one create");
    for step in &inverse {
        let undo = <SourcingMutation as protocol::Mutation<CurationSnapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the create-curated-item inverse step applies");
    }
    assert_eq!(snapshot, base, "delete-curated-item/removes-the-clt-panel-from-the-curation: re-curating the CLT panel did not restore the before-document, count and all");
}

/// 🔣️ Both committed documents and the `deleteCuratedItem` payload are canonical — the payload is
/// the bare `objectId`, with no count riding along.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: CurationSnapshot = serde_json::from_str(text).expect("curation document decodes");
        let reencoded = serde_json::to_value(&decoded).expect("curation document encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("curation document reparses");
        assert_eq!(reencoded, original, "delete-curated-item/removes-the-clt-panel-from-the-curation: committed {label} document JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("deleteCuratedItem payload encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("deleteCuratedItem payload reparses");
    assert_eq!(reencoded, original, "delete-curated-item/removes-the-clt-panel-from-the-curation: committed deleteCuratedItem JSON is not canonical");
}

/// 🎯️ `panel-clt-3000` is curated in the before-document, so the single `mutation.target-missing`
/// guard does not fire and the declared `applied` outcome must be message-free.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "delete-curated-item/removes-the-clt-panel-from-the-curation: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "delete-curated-item/removes-the-clt-panel-from-the-curation: un-curating a curated object must raise no mutation.target-missing fault");
    assert!(produced.messages().is_empty(), "delete-curated-item/removes-the-clt-panel-from-the-curation: an accepted removal emits no diagnostics");
}

/// 🔺️ The committed diff pins the sparseness: `curated.removed` carries the one id, `added` and
/// `patched` stay empty, and the surviving glulam row is NOT restated anywhere in the diff.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("produced delete-curated-item diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "delete-curated-item/removes-the-clt-panel-from-the-curation: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to `CurationDiff` and re-encodes unchanged.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: CurationDiff = serde_json::from_str(DIFF).expect("committed delete-curated-item diff decodes");
    let delta = decoded.curated.as_ref().expect("the committed delete diff carries a curated delta");
    assert_eq!(delta.removed, vec!["panel-clt-3000".to_string()], "delete-curated-item/removes-the-clt-panel-from-the-curation: the committed diff must remove exactly the addressed id");
    assert!(delta.added.is_empty() && delta.patched.is_empty(), "delete-curated-item/removes-the-clt-panel-from-the-curation: a removal must not smuggle additions or patches alongside");
    let reencoded = serde_json::to_value(&decoded).expect("committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "delete-curated-item/removes-the-clt-panel-from-the-curation: committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-document to the after-document.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: CurationDiff = serde_json::from_str(DIFF).expect("committed delete-curated-item diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("committed diff applies to the before-document");
    assert_eq!(produced, expected_after(), "delete-curated-item/removes-the-clt-panel-from-the-curation: committed diff did not carry before to after");
}
