//! 🧪️ `create-artifact` fixture — `appends-artifact-3-to-the-index`.
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
    serde_json::from_str(MUTATION).expect("create-artifact mutation decodes")
}
fn built_outcome() -> protocol::MutationOutcome<SSpaceDiff> {
    <SSpaceMutation as protocol::Mutation<SSpaceSnapshot>>::diff(&mutation(), &before())
}

/// ▶️ Creating `artifact-3` appends exactly one row and leaves `artifact-1`/`artifact-2` untouched.
#[semio_framework_async_macros::async_test]
async fn appends_the_new_row_to_the_committed_after() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("create-artifact applies to its committed before-index");
    assert_eq!(applied, expected_after(), "create-artifact/appends-artifact-3-to-the-index: the appended index differs from the committed after-snapshot");
}

/// ↩️ `create-artifact`'s inverse is `delete-artifact` on the very id it minted — replaying it
/// restores the two-row index byte for byte.
#[semio_framework_async_macros::async_test]
async fn deleting_the_created_row_restores_before() {
    let base = before();
    let forward = <SSpaceMutation as protocol::Mutation<SSpaceSnapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("forward create-artifact applies");
    let inverse = <SSpaceMutation as protocol::Mutation<SSpaceSnapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "create-artifact/appends-artifact-3-to-the-index: the inverse of one create is exactly one delete");
    for step in &inverse {
        let undo = <SSpaceMutation as protocol::Mutation<SSpaceSnapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the delete-artifact inverse step applies");
    }
    assert_eq!(snapshot, base, "create-artifact/appends-artifact-3-to-the-index: deleting artifact-3 back out did not restore the before-index");
}

/// 🔣️ Both committed index snapshots and the `createArtifact` payload are canonical.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SSpaceSnapshot = serde_json::from_str(text).expect("space index snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("space index snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("space index snapshot reparses");
        assert_eq!(reencoded, original, "create-artifact/appends-artifact-3-to-the-index: committed {label} index JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("createArtifact payload encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("createArtifact payload reparses");
    assert_eq!(reencoded, original, "create-artifact/appends-artifact-3-to-the-index: committed createArtifact JSON is not canonical");
}

/// 🎯️ `artifact-3` is a free id in the before-index, so the declared `applied` outcome must carry
/// no `mutation.duplicate-id` fault at all.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "create-artifact/appends-artifact-3-to-the-index: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "create-artifact/appends-artifact-3-to-the-index: creating a fresh id must raise no mutation.duplicate-id fault");
    assert!(produced.messages().is_empty(), "create-artifact/appends-artifact-3-to-the-index: an accepted create emits no diagnostics");
}

/// 🔺️ `SSpaceDiff` replaces the whole `artifacts` field and never touches `schema` — the committed
/// diff pins that the append rewrites the row vector and nothing else.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("produced create-artifact diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "create-artifact/appends-artifact-3-to-the-index: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to `SSpaceDiff` and re-encodes unchanged.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: SSpaceDiff = serde_json::from_str(DIFF).expect("committed create-artifact diff decodes");
    assert!(decoded.schema.is_none(), "create-artifact/appends-artifact-3-to-the-index: creating a row must leave the index schema field alone");
    let reencoded = serde_json::to_value(&decoded).expect("committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "create-artifact/appends-artifact-3-to-the-index: committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-index to the after-index.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SSpaceDiff = serde_json::from_str(DIFF).expect("committed create-artifact diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("committed diff applies to the before-index");
    assert_eq!(produced, expected_after(), "create-artifact/appends-artifact-3-to-the-index: committed diff did not carry before to after");
}
