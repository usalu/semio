//! 🧪️ `create-material` fixture — `adds-a-second-material-at-the-end`.
//!
//! Transcribed from `../../🔺️diff/🦀️component.rs` and `diff_add_material`: a duplicate material id
//! is FATAL `mutation.duplicate-id`; otherwise the diff is a `materials` triple whose `added` entry
//! is a `NamedAdded { index: base.materials.len(), item }`. Materials live in their OWN top-level
//! collection, parallel to meshes — creating one binds it to nothing.

use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::SemioMeshDiff;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::SemioMeshMutation;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> SemioMeshSnapshot {
    serde_json::from_str(BEFORE).expect("create-material before snapshot decodes")
}
fn expected_after() -> SemioMeshSnapshot {
    serde_json::from_str(AFTER).expect("create-material after snapshot decodes")
}
fn mutation() -> SemioMeshMutation {
    serde_json::from_str(MUTATION).expect("create-material mutation decodes")
}

/// ▶️ A second material appears; no primitive is rebound to it.
#[semio_framework_async_macros::async_test]
async fn adds_the_second_material_without_binding_it_to_anything() {
    let base = before();
    let produced = mutation().diff(&base).diff().apply(&base).expect("create-material applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "create-material/adds-a-second-material-at-the-end: applied state differs from the committed after-snapshot");
    assert_eq!(produced.materials.len(), base.materials.len() + 1, "create-material adds exactly one material");
    assert_eq!(produced.materials[1].id, "mat-b", "the new material occupies the index the NamedAdded entry recorded");
    assert_eq!(produced.materials[1].metallic, 1.0, "the payload's own PBR factors land verbatim");
    assert_eq!(produced.meshes, base.meshes, "creating a material must not rebind any primitive to it");
}

/// ↩️ `create-material`'s undo is a single `delete-material` for the same id.
#[semio_framework_async_macros::async_test]
async fn the_undo_delete_material_removes_the_second_material_again() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "create-material undoes as exactly one delete-material");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward create-material applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo delete-material applies");
    }
    assert_eq!(current, base, "create-material/adds-a-second-material-at-the-end: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the payload are canonical — `SemioRgba` channels are `f32` and every value here is dyadic, so decode→encode is exact.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioMeshSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-material/adds-a-second-material-at-the-end: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("create-material mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("create-material mutation reparses");
    assert_eq!(reencoded, original, "create-material/adds-a-second-material-at-the-end: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: no material with id mat-b exists, so the FATAL mutation.duplicate-id branch must not fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "create-material/adds-a-second-material-at-the-end: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "creating a material with a fresh id must raise no diagnostics");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. Only the `materials` triple, and only its `added` arm.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioMeshMutation as Mutation<SemioMeshSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "create-material/adds-a-second-material-at-the-end: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and only the collection this mutation is
/// allowed to touch appears in it at all.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioMeshDiff = serde_json::from_str(DIFF).expect("committed create-material diff decodes");
    let materials = decoded.materials.as_ref().expect("create-material must write the materials triple");
    assert_eq!(materials.added.len(), 1, "exactly one material is added");
    assert_eq!(materials.added[0].index, 1, "the add carries its target POSITION");
    assert!(materials.removed.is_empty() && materials.modified.is_empty(), "a create neither removes nor modifies");
    assert!(decoded.meshes.is_none() && decoded.textures.is_none(), "no mesh or texture slot may appear in the diff");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "create-material/adds-a-second-material-at-the-end: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioMeshDiff = serde_json::from_str(DIFF).expect("committed create-material diff decodes");
    let produced = decoded.apply(&before()).expect("committed create-material diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "create-material/adds-a-second-material-at-the-end: committed diff did not carry before to after");
}
