//! 🧪️ `delete-texture` fixture — `removes-the-leading-texture-and-keeps-the-trailing-one`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: an unknown texture id is Error
//! `mutation.target-missing`; otherwise the diff is a bare `textures.removed[id]` carrying no
//! bytes. The inverse uses the same strip-tail/re-create/rebuild-tail dance as `delete-mesh` and
//! `delete-material`, so the fixture removes the LEADING texture of two to make the order
//! restoration observable — and the removed bytes must come back from `base`, since the diff never
//! carried them.

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
    serde_json::from_str(BEFORE).expect("delete-texture before snapshot decodes")
}
fn expected_after() -> SemioMeshSnapshot {
    serde_json::from_str(AFTER).expect("delete-texture after snapshot decodes")
}
fn mutation() -> SemioMeshMutation {
    serde_json::from_str(MUTATION).expect("delete-texture mutation decodes")
}

/// ▶️ The leading texture goes; the trailing one slides down to index 0.
#[semio_framework_async_macros::async_test]
async fn removes_the_leading_texture() {
    let base = before();
    assert_eq!(base.textures.len(), 2, "the fixture needs a trailing texture for the order-restoring inverse to matter");
    let produced = mutation().diff(&base).diff().apply(&base).expect("delete-texture applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "delete-texture/removes-the-leading-texture-and-keeps-the-trailing-one: applied state differs from the committed after-snapshot");
    assert!(!produced.textures.iter().any(|texture| texture.id == "tex-a"), "the named texture must be gone");
    assert_eq!(produced.textures, vec![base.textures[1].clone()], "the trailing texture slides down into index 0");
    assert_eq!(produced.materials, base.materials, "deleting a texture must not touch a material");
}

/// ↩️ The undo strips the tail, re-creates the texture WITH its captured bytes, rebuilds the tail.
#[semio_framework_async_macros::async_test]
async fn the_undo_strips_the_tail_recreates_the_texture_then_rebuilds_the_tail() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 3, "one delete per trailing texture, then the re-create, then one create per trailing texture");
    let SemioMeshMutation::CreateTexture(recreate) = &undo[1] else { panic!("the middle undo step must re-create the removed texture") };
    assert_eq!(recreate.texture.bytes, base.textures[0].bytes, "the undo must recapture the removed texture's own bytes — the diff never carried them");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward delete-texture applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("each undo step applies to the running state");
    }
    assert_eq!(current, base, "delete-texture/removes-the-leading-texture-and-keeps-the-trailing-one: the undo did not restore the before-snapshot, order included");
}

/// 🔣️ Snapshots and the `{"DeleteTexture":{"id":"tex-a"}}` payload are canonical fixed points.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioMeshSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-texture/removes-the-leading-texture-and-keeps-the-trailing-one: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("delete-texture mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("delete-texture mutation reparses");
    assert_eq!(reencoded, original, "delete-texture/removes-the-leading-texture-and-keeps-the-trailing-one: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the texture exists, so mutation.target-missing must not fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "delete-texture/removes-the-leading-texture-and-keeps-the-trailing-one: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "deleting an existing texture must raise no diagnostics");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. Only `textures.removed`, carrying the ID and no bytes.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioMeshMutation as Mutation<SemioMeshSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "delete-texture/removes-the-leading-texture-and-keeps-the-trailing-one: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and only the collection this mutation is
/// allowed to touch appears in it at all.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioMeshDiff = serde_json::from_str(DIFF).expect("committed delete-texture diff decodes");
    let textures = decoded.textures.as_ref().expect("delete-texture must write the textures triple");
    assert_eq!(textures.removed, vec!["tex-a".to_string()], "the removal is addressed by texture id");
    assert!(textures.modified.is_empty() && textures.added.is_empty(), "a removal neither modifies nor adds");
    assert!(decoded.meshes.is_none() && decoded.materials.is_none(), "no mesh or material slot may appear in the diff");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "delete-texture/removes-the-leading-texture-and-keeps-the-trailing-one: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioMeshDiff = serde_json::from_str(DIFF).expect("committed delete-texture diff decodes");
    let produced = decoded.apply(&before()).expect("committed delete-texture diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-texture/removes-the-leading-texture-and-keeps-the-trailing-one: committed diff did not carry before to after");
}
