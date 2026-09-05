//! 🧪️ `create-curated-item` fixture — `🧲️appends-a-steel-plate-to-the-curation`.
//!
//! `create-curated-item`'s diff oracle guards a single condition — the object is already curated ⇒
//! Fatal `mutation.duplicate-id` — and otherwise emits a genuinely sparse `CurationCuratedDelta`
//! carrying ONE `added` entry. It never rewrites the surviving rows, never re-mints the composed
//! `catalog` kit handle, and never touches the sourcing-owned `stockExtra` overflow.
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
    serde_json::from_str(MUTATION).expect("create-curated-item mutation decodes")
}
fn built_outcome() -> protocol::MutationOutcome<CurationDiff> {
    <SourcingMutation as protocol::Mutation<CurationSnapshot>>::diff(&mutation(), &before())
}

/// ▶️ Curating `plate-steel-8` appends one row at the tail — `apply_curated_delta` extends after
/// filtering, so the two existing picks keep both their order and their counts.
#[semio_framework_async_macros::async_test]
async fn appends_the_new_pick_at_the_tail() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("create-curated-item applies to its committed before-document");
    assert_eq!(applied, expected_after(), "create-curated-item/appends-a-steel-plate-to-the-curation: the extended curation differs from the committed after-snapshot");
    assert_eq!(applied.curated.last().map(|item| item.object_id.as_str()), Some("plate-steel-8"), "create-curated-item/appends-a-steel-plate-to-the-curation: a created pick must land at the tail, not be spliced in");
}

/// ↩️ `create-curated-item`'s inverse is payload-derived: a `delete-curated-item` of the object id
/// it curated, which filters the tail row straight back out.
#[semio_framework_async_macros::async_test]
async fn deleting_the_new_pick_restores_before() {
    let base = before();
    let mut snapshot = protocol::MutationDiff::apply(built_outcome().diff(), &base).expect("forward create-curated-item applies");
    let inverse = <SourcingMutation as protocol::Mutation<CurationSnapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "create-curated-item/appends-a-steel-plate-to-the-curation: the inverse of one create is exactly one delete");
    for step in &inverse {
        let undo = <SourcingMutation as protocol::Mutation<CurationSnapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the delete-curated-item inverse step applies");
    }
    assert_eq!(snapshot, base, "create-curated-item/appends-a-steel-plate-to-the-curation: un-curating the steel plate did not restore the before-document");
}

/// 🔣️ Both committed documents and the `createCuratedItem` payload are canonical.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: CurationSnapshot = serde_json::from_str(text).expect("curation document decodes");
        let reencoded = serde_json::to_value(&decoded).expect("curation document encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("curation document reparses");
        assert_eq!(reencoded, original, "create-curated-item/appends-a-steel-plate-to-the-curation: committed {label} document JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("createCuratedItem payload encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("createCuratedItem payload reparses");
    assert_eq!(reencoded, original, "create-curated-item/appends-a-steel-plate-to-the-curation: committed createCuratedItem JSON is not canonical");
}

/// 🎯️ `plate-steel-8` is not yet curated, so the single `mutation.duplicate-id` guard does not fire
/// and the declared `applied` outcome must be message-free.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "create-curated-item/appends-a-steel-plate-to-the-curation: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "create-curated-item/appends-a-steel-plate-to-the-curation: curating a not-yet-curated object must raise no mutation.duplicate-id fault");
    assert!(produced.messages().is_empty(), "create-curated-item/appends-a-steel-plate-to-the-curation: an accepted curation emits no diagnostics");
}

/// 🔺️ The committed diff pins the sparseness: `curated.added` carries the one new pick, `removed`
/// and `patched` stay empty, `reordered` stays null, and `catalog`/`stockExtra` — the composed kit
/// child and its sourcing-owned overflow — are not in the diff at all.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("produced create-curated-item diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "create-curated-item/appends-a-steel-plate-to-the-curation: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to `CurationDiff` and re-encodes unchanged.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: CurationDiff = serde_json::from_str(DIFF).expect("committed create-curated-item diff decodes");
    let delta = decoded.curated.as_ref().expect("the committed create diff carries a curated delta");
    assert_eq!((delta.added.len(), delta.removed.len(), delta.patched.len()), (1, 0, 0), "create-curated-item/appends-a-steel-plate-to-the-curation: a create is one addition and nothing else");
    assert!(decoded.catalog.is_none(), "create-curated-item/appends-a-steel-plate-to-the-curation: curating must not replace the composed kit catalog handle");
    let reencoded = serde_json::to_value(&decoded).expect("committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "create-curated-item/appends-a-steel-plate-to-the-curation: committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-document to the after-document.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: CurationDiff = serde_json::from_str(DIFF).expect("committed create-curated-item diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("committed diff applies to the before-document");
    assert_eq!(produced, expected_after(), "create-curated-item/appends-a-steel-plate-to-the-curation: committed diff did not carry before to after");
}
