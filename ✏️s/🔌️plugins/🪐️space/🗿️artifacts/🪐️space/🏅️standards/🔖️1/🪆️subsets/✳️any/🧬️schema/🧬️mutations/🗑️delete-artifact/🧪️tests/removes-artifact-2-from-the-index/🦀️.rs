//! 🧪️ `delete-artifact` fixture — `removes-artifact-2-from-the-index`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.

use crate::artifacts::space::standards::v1::subsets::any::schema::diff::SSpaceDiff;
use crate::artifacts::space::standards::v1::subsets::any::schema::mutations::SSpaceMutation;
use crate::artifacts::space::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> SSpaceSnapshot {
    serde_json::from_str(BEFORE).expect("before space index decodes")
}
fn expected_after() -> SSpaceSnapshot {
    serde_json::from_str(AFTER).expect("after space index decodes")
}
fn mutation() -> SSpaceMutation {
    serde_json::from_str(MUTATION).expect("delete-artifact mutation decodes")
}
fn built_outcome() -> protocol::MutationOutcome<SSpaceDiff> {
    <SSpaceMutation as protocol::Mutation<SSpaceSnapshot>>::diff(&mutation(), &before())
}

/// ▶️ Deleting `artifact-2` filters that one row out and keeps `artifact-1` in place.
#[semio_framework_async_macros::async_test]
async fn filters_the_named_row_out_of_the_committed_after() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("delete-artifact applies to its committed before-index");
    assert_eq!(applied, expected_after(), "delete-artifact/removes-artifact-2-from-the-index: the filtered index differs from the committed after-snapshot");
    assert!(!applied.artifacts.iter().any(|row| row.id == "artifact-2"), "delete-artifact/removes-artifact-2-from-the-index: artifact-2 survived its own deletion");
}

/// ↩️ `delete-artifact`'s inverse re-creates the exact row it removed — looked up from BASE, so
/// every one of `artifact-2`'s timestamps and its `updatedBy` come back unchanged.
#[semio_framework_async_macros::async_test]
async fn recreating_the_removed_row_restores_before() {
    let base = before();
    let forward = <SSpaceMutation as protocol::Mutation<SSpaceSnapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("forward delete-artifact applies");
    let inverse = <SSpaceMutation as protocol::Mutation<SSpaceSnapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "delete-artifact/removes-artifact-2-from-the-index: the inverse of one delete is exactly one create");
    for step in &inverse {
        let undo = <SSpaceMutation as protocol::Mutation<SSpaceSnapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the create-artifact inverse step applies");
    }
    assert_eq!(snapshot, base, "delete-artifact/removes-artifact-2-from-the-index: re-creating artifact-2 did not restore the before-index");
}

/// 🔣️ Both committed index snapshots and the `deleteArtifact` payload are canonical.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SSpaceSnapshot = serde_json::from_str(text).expect("space index snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("space index snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("space index snapshot reparses");
        assert_eq!(reencoded, original, "delete-artifact/removes-artifact-2-from-the-index: committed {label} index JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("deleteArtifact payload encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("deleteArtifact payload reparses");
    assert_eq!(reencoded, original, "delete-artifact/removes-artifact-2-from-the-index: committed deleteArtifact JSON is not canonical");
}

/// 🎯️ `artifact-2` is present in the before-index, so the declared `applied` outcome must carry no
/// `mutation.target-missing` fault.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "delete-artifact/removes-artifact-2-from-the-index: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "delete-artifact/removes-artifact-2-from-the-index: deleting a present id must raise no mutation.target-missing fault");
    assert!(produced.messages().is_empty(), "delete-artifact/removes-artifact-2-from-the-index: an accepted delete emits no diagnostics");
}

/// 🔺️ The committed diff is the whole surviving row vector — one entry shorter — and never the
/// `schema` field.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("produced delete-artifact diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "delete-artifact/removes-artifact-2-from-the-index: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to `SSpaceDiff` and re-encodes unchanged.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: SSpaceDiff = serde_json::from_str(DIFF).expect("committed delete-artifact diff decodes");
    assert_eq!(decoded.artifacts.as_ref().map(Vec::len), Some(1), "delete-artifact/removes-artifact-2-from-the-index: the committed diff must carry the one surviving row");
    let reencoded = serde_json::to_value(&decoded).expect("committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "delete-artifact/removes-artifact-2-from-the-index: committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-index to the after-index.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SSpaceDiff = serde_json::from_str(DIFF).expect("committed delete-artifact diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("committed diff applies to the before-index");
    assert_eq!(produced, expected_after(), "delete-artifact/removes-artifact-2-from-the-index: committed diff did not carry before to after");
}
