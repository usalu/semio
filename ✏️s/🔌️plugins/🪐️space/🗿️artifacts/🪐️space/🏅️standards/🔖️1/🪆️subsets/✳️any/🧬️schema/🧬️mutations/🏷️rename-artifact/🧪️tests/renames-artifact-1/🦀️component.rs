//! 🧪️ `rename-artifact` fixture — `renames-artifact-1`.
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
    serde_json::from_str(MUTATION).expect("rename-artifact mutation decodes")
}
fn built_outcome() -> protocol::MutationOutcome<SSpaceDiff> {
    <SSpaceMutation as protocol::Mutation<SSpaceSnapshot>>::diff(&mutation(), &before())
}

/// ▶️ Renaming `artifact-1` rewrites only that row's `name`; its ids, kind, dialect and both
/// timestamps stay exactly as committed (a rename is not a touch).
#[semio_framework_async_macros::async_test]
async fn rewrites_only_the_name_of_the_committed_after() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("rename-artifact applies to its committed before-index");
    assert_eq!(applied, expected_after(), "rename-artifact/renames-artifact-1: the renamed index differs from the committed after-snapshot");
    let renamed = applied.artifacts.iter().find(|row| row.id == "artifact-1").expect("artifact-1 survives its own rename");
    assert_eq!(renamed.updated_at_ms, 1000, "rename-artifact/renames-artifact-1: renaming must not restamp updatedAtMs");
}

/// ↩️ `rename-artifact`'s inverse carries the OLD name read out of BASE, so replaying it puts
/// "Site Plan" back on `artifact-1`.
#[semio_framework_async_macros::async_test]
async fn renaming_back_restores_before() {
    let base = before();
    let forward = <SSpaceMutation as protocol::Mutation<SSpaceSnapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("forward rename-artifact applies");
    let inverse = <SSpaceMutation as protocol::Mutation<SSpaceSnapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "rename-artifact/renames-artifact-1: the inverse of one rename is exactly one rename back");
    for step in &inverse {
        let undo = <SSpaceMutation as protocol::Mutation<SSpaceSnapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the rename-artifact inverse step applies");
    }
    assert_eq!(snapshot, base, "rename-artifact/renames-artifact-1: renaming back to \"Site Plan\" did not restore the before-index");
}

/// 🔣️ Both committed index snapshots and the `renameArtifact` payload are canonical.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SSpaceSnapshot = serde_json::from_str(text).expect("space index snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("space index snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("space index snapshot reparses");
        assert_eq!(reencoded, original, "rename-artifact/renames-artifact-1: committed {label} index JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("renameArtifact payload encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("renameArtifact payload reparses");
    assert_eq!(reencoded, original, "rename-artifact/renames-artifact-1: committed renameArtifact JSON is not canonical");
}

/// 🎯️ "Site Plan Rev B" is neither `artifact-1`'s current name (which would be a `mutation.no-op`
/// warning with an empty diff) nor any sibling's name (which would be a fatal collision), so the
/// declared `applied` outcome must be message-free.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "rename-artifact/renames-artifact-1: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "rename-artifact/renames-artifact-1: a genuine, uncontested new name raises neither mutation.no-op nor mutation.duplicate-id");
    assert!(produced.messages().is_empty(), "rename-artifact/renames-artifact-1: an accepted rename emits no diagnostics");
}

/// 🔺️ The committed diff is the whole row vector with one `name` swapped — `artifact-2` is carried
/// through byte-identical and `schema` stays untouched.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("produced rename-artifact diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "rename-artifact/renames-artifact-1: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff decodes to `SSpaceDiff` and re-encodes unchanged.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: SSpaceDiff = serde_json::from_str(DIFF).expect("committed rename-artifact diff decodes");
    let rows = decoded.artifacts.as_ref().expect("the committed rename diff carries the row vector");
    assert_eq!(rows[1].name, "Massing Study", "rename-artifact/renames-artifact-1: the untargeted sibling row must ride through the diff unchanged");
    let reencoded = serde_json::to_value(&decoded).expect("committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "rename-artifact/renames-artifact-1: committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-index to the after-index.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SSpaceDiff = serde_json::from_str(DIFF).expect("committed rename-artifact diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("committed diff applies to the before-index");
    assert_eq!(produced, expected_after(), "rename-artifact/renames-artifact-1: committed diff did not carry before to after");
}
