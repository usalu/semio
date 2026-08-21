//! 🧪️ `touch-artifact` fixture — `stamps-artifact-1-with-a-new-editor`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.

use crate::artifacts::space::standards::v1::subsets::any::schema::diff::SSpaceDiff;
use crate::artifacts::space::standards::v1::subsets::any::schema::mutations::SSpaceMutation;
use crate::artifacts::space::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> SSpaceSnapshot {
    serde_json::from_str(BEFORE).expect("before space index decodes")
}
fn expected_after() -> SSpaceSnapshot {
    serde_json::from_str(AFTER).expect("after space index decodes")
}
fn mutation() -> SSpaceMutation {
    serde_json::from_str(MUTATION).expect("touch-artifact mutation decodes")
}
fn built_outcome() -> protocol::MutationOutcome<SSpaceDiff> {
    <SSpaceMutation as protocol::Mutation<SSpaceSnapshot>>::diff(&mutation(), &before())
}

/// ▶️ Touching `artifact-1` stamps `updatedAtMs`/`updatedBy` only — `createdAtMs`/`createdBy` and
/// the row's name are checkpoint-invariant.
#[semio_framework_async_macros::async_test]
async fn stamps_only_the_updated_pair_of_the_committed_after() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("touch-artifact applies to its committed before-index");
    assert_eq!(applied, expected_after(), "touch-artifact/stamps-artifact-1-with-a-new-editor: the stamped index differs from the committed after-snapshot");
    let touched = applied.artifacts.iter().find(|row| row.id == "artifact-1").expect("artifact-1 survives its own touch");
    assert_eq!((touched.created_at_ms, touched.created_by.as_str(), touched.name.as_str()), (1000, "user:ada", "Site Plan"), "touch-artifact/stamps-artifact-1-with-a-new-editor: a touch must leave creation metadata and the name alone");
}

/// ↩️ `touch-artifact`'s inverse is a touch back to the OLD `updatedAtMs`/`updatedBy` pair read out
/// of BASE — `1000` / `user:ada`.
#[semio_framework_async_macros::async_test]
async fn restamping_the_old_pair_restores_before() {
    let base = before();
    let forward = <SSpaceMutation as protocol::Mutation<SSpaceSnapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("forward touch-artifact applies");
    let inverse = <SSpaceMutation as protocol::Mutation<SSpaceSnapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "touch-artifact/stamps-artifact-1-with-a-new-editor: the inverse of one touch is exactly one touch back");
    for step in &inverse {
        let undo = <SSpaceMutation as protocol::Mutation<SSpaceSnapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the touch-artifact inverse step applies");
    }
    assert_eq!(snapshot, base, "touch-artifact/stamps-artifact-1-with-a-new-editor: restamping user:ada@1000 did not restore the before-index");
}

/// 🔣️ Both committed index snapshots and the `touchArtifact` payload are canonical.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SSpaceSnapshot = serde_json::from_str(text).expect("space index snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("space index snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("space index snapshot reparses");
        assert_eq!(reencoded, original, "touch-artifact/stamps-artifact-1-with-a-new-editor: committed {label} index JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("touchArtifact payload encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("touchArtifact payload reparses");
    assert_eq!(reencoded, original, "touch-artifact/stamps-artifact-1-with-a-new-editor: committed touchArtifact JSON is not canonical");
}

/// 🎯️ `touch-artifact` has no no-op guard at all — repeated stamps are legal — so the only fault it
/// could raise here is `mutation.target-missing`, and `artifact-1` exists.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "touch-artifact/stamps-artifact-1-with-a-new-editor: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "touch-artifact/stamps-artifact-1-with-a-new-editor: touching a present id must raise no mutation.target-missing fault");
    assert!(produced.messages().is_empty(), "touch-artifact/stamps-artifact-1-with-a-new-editor: an accepted touch emits no diagnostics");
}

/// 🔺️ The committed diff is the whole row vector with one `updatedAtMs`/`updatedBy` pair swapped;
/// `artifact-2`'s own stamps must not be dragged along.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("produced touch-artifact diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "touch-artifact/stamps-artifact-1-with-a-new-editor: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff decodes to `SSpaceDiff` and re-encodes unchanged.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: SSpaceDiff = serde_json::from_str(DIFF).expect("committed touch-artifact diff decodes");
    let rows = decoded.artifacts.as_ref().expect("the committed touch diff carries the row vector");
    assert_eq!((rows[1].updated_at_ms, rows[1].updated_by.as_str()), (2500, "user:grace"), "touch-artifact/stamps-artifact-1-with-a-new-editor: the untouched sibling keeps its own stamp inside the diff");
    let reencoded = serde_json::to_value(&decoded).expect("committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "touch-artifact/stamps-artifact-1-with-a-new-editor: committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-index to the after-index.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SSpaceDiff = serde_json::from_str(DIFF).expect("committed touch-artifact diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("committed diff applies to the before-index");
    assert_eq!(produced, expected_after(), "touch-artifact/stamps-artifact-1-with-a-new-editor: committed diff did not carry before to after");
}
