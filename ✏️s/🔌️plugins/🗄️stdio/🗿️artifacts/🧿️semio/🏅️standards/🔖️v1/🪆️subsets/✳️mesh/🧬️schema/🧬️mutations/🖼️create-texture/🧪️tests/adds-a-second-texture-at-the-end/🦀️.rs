//! 🧪️ `create-texture` fixture — `adds-a-second-texture-at-the-end`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs` and `diff_add_texture`: a duplicate texture id
//! is FATAL `mutation.duplicate-id`; otherwise the diff is a `textures` triple whose `added` entry
//! is a `NamedAdded { index: base.textures.len(), item }`. Textures are a third top-level
//! collection alongside meshes and materials — creating one binds it to nothing.

use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::SemioMeshDiff;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::SemioMeshMutation;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> SemioMeshSnapshot {
    serde_json::from_str(BEFORE).expect("create-texture before snapshot decodes")
}
fn expected_after() -> SemioMeshSnapshot {
    serde_json::from_str(AFTER).expect("create-texture after snapshot decodes")
}
fn mutation() -> SemioMeshMutation {
    serde_json::from_str(MUTATION).expect("create-texture mutation decodes")
}

/// ▶️ A second texture appears with its own mime and payload bytes.
#[semio_framework_async_macros::async_test]
async fn adds_the_second_texture_with_its_own_mime_and_bytes() {
    let base = before();
    let produced = mutation().diff(&base).diff().apply(&base).expect("create-texture applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "create-texture/adds-a-second-texture-at-the-end: applied state differs from the committed after-snapshot");
    assert_eq!(produced.textures.len(), base.textures.len() + 1, "create-texture adds exactly one texture");
    assert_eq!(produced.textures[1].id, "tex-b", "the new texture occupies the index the NamedAdded entry recorded");
    assert_eq!(produced.textures[1].mime, "image/jpeg", "the payload's own mime type lands verbatim");
    assert_eq!(produced.textures[1].bytes, vec![9u8, 8, 7], "the payload's own bytes land verbatim, never re-encoded");
    assert_eq!((produced.meshes, produced.materials), (base.meshes, base.materials), "creating a texture must not touch a mesh or a material");
}

/// ↩️ `create-texture`'s undo is a single `delete-texture` for the same id.
#[semio_framework_async_macros::async_test]
async fn the_undo_delete_texture_removes_the_second_texture_again() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "create-texture undoes as exactly one delete-texture");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward create-texture applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo delete-texture applies");
    }
    assert_eq!(current, base, "create-texture/adds-a-second-texture-at-the-end: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the payload are canonical — texture bytes travel as a plain JSON number array, never base64.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioMeshSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-texture/adds-a-second-texture-at-the-end: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("create-texture mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("create-texture mutation reparses");
    assert_eq!(reencoded, original, "create-texture/adds-a-second-texture-at-the-end: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: no texture with id tex-b exists, so the FATAL mutation.duplicate-id branch must not fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "create-texture/adds-a-second-texture-at-the-end: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "creating a texture with a fresh id must raise no diagnostics");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. Only the `textures` triple, and only its `added` arm.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioMeshMutation as Mutation<SemioMeshSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "create-texture/adds-a-second-texture-at-the-end: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and only the collection this mutation is
/// allowed to touch appears in it at all.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioMeshDiff = serde_json::from_str(DIFF).expect("committed create-texture diff decodes");
    let textures = decoded.textures.as_ref().expect("create-texture must write the textures triple");
    assert_eq!(textures.added.len(), 1, "exactly one texture is added");
    assert_eq!(textures.added[0].index, 1, "the add carries its target POSITION");
    assert!(textures.removed.is_empty() && textures.modified.is_empty(), "a create neither removes nor modifies");
    assert!(decoded.meshes.is_none() && decoded.materials.is_none(), "no mesh or material slot may appear in the diff");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "create-texture/adds-a-second-texture-at-the-end: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioMeshDiff = serde_json::from_str(DIFF).expect("committed create-texture diff decodes");
    let produced = decoded.apply(&before()).expect("committed create-texture diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "create-texture/adds-a-second-texture-at-the-end: committed diff did not carry before to after");
}
