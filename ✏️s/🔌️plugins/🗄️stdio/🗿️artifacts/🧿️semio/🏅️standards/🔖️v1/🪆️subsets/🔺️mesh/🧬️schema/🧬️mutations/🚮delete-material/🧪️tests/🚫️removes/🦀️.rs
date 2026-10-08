//! 🧪️ `delete-material` fixture — `🚫️removes`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: an unknown material id is Error
//! `mutation.target-missing`; otherwise the diff is a bare `materials.removed[id]`. There is NO
//! cascade into `meshes` — a primitive whose `materialId` names the deleted material keeps naming
//! it, dangling — and the single-collection diff is what records that choice. 
//! The inverse is one create carrying the removed index (`at`), so order is restored exactly.

use crate::standards::v1::subsets::mesh::schema::diff::SemioMeshDiff;
use crate::standards::v1::subsets::mesh::schema::mutations::SemioMeshMutation;
use crate::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🚮delete-material/🚫️removes/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🚮delete-material/🚫️removes/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🚮delete-material/🚫️removes/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🚮delete-material/🚫️removes/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🚮delete-material/🚫️removes/🎯️outcome/🔣️.json");

fn before() -> SemioMeshSnapshot {
    semio_framework_pack_json::from_json_str(BEFORE, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("delete-material before snapshot decodes")
}
fn expected_after() -> SemioMeshSnapshot {
    semio_framework_pack_json::from_json_str(AFTER, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("delete-material after snapshot decodes")
}
fn mutation() -> SemioMeshMutation {
    semio_framework_pack_json::from_json_str(MUTATION, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("delete-material mutation decodes")
}

/// ▶️ The leading material goes; the trailing one slides down and no primitive is rewritten.
#[semio_framework_async_macros::async_test]
async fn removes_the_leading_material_without_cascading_into_primitives() {
    let base = before();
    let produced = protocol::apply_diff(mutation().diff(&base).diff(), &base).expect("delete-material applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "delete-material/removes-the-leading-material-and-keeps-the-trailing-one: applied state differs from the committed after-snapshot");
    assert!(!produced.materials.iter().any(|material| material.id == "mat-a"), "the named material must be gone");
    assert_eq!(produced.materials, vec![base.materials[1].clone()], "the trailing material slides down into index 0");
    assert_eq!(produced.meshes, base.meshes, "delete-material must NOT cascade into the primitives that reference a material");
}

/// ↩️ The undo is ONE create carrying the removed index, so the original position is restored exactly.
#[semio_framework_async_macros::async_test]
async fn the_undo_recreates_the_material_at_its_original_index() {
    let base = before();
    let mutation = mutation();
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
    let undo = mutation.inverse(&base).expect("valid retained mutation inverse fixture");
    assert_eq!(undo.len(), 1, "the removed material is restored by exactly one create");
    assert!(matches!(&undo[0], SemioMeshMutation::CreateMaterial(create) if create.at == Some(0)), "the create must name the removed index");
    let mut current = protocol::apply_diff(mutation.diff(&base).diff(), &base).expect("forward delete-material applies");
    for step in undo.iter().rev() {
        current = protocol::apply_diff(step.diff(&current).diff(), &current).expect("each undo step applies to the running state");
    }
    assert_eq!(current, base, "delete-material/removes-the-leading-material-and-keeps-the-trailing-one: the undo did not restore the before-snapshot, order included");
}

/// 🔣️ Snapshots and the `{"DeleteMaterial":{"id":"mat-a"}}` payload are canonical fixed points.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioMeshSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-material/removes-the-leading-material-and-keeps-the-trailing-one: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&(mutation()))).expect("delete-material mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("delete-material mutation reparses");
    assert_eq!(reencoded, original, "delete-material/removes-the-leading-material-and-keeps-the-trailing-one: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the material exists, so mutation.target-missing must not fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "delete-material/removes-the-leading-material-and-keeps-the-trailing-one: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "deleting an existing material must raise no diagnostics — this leaf has no cascade to report");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. Only `materials.removed`.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioMeshMutation as Mutation<SemioMeshSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(outcome.diff())).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "delete-material/removes-the-leading-material-and-keeps-the-trailing-one: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and only the collection this mutation is
/// allowed to touch appears in it at all.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioMeshDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed delete-material diff decodes");
    let materials = decoded.materials.as_ref().expect("delete-material must write the materials triple");
    assert_eq!(materials.removed, vec!["mat-a".to_string()], "the removal is addressed by material id");
    assert!(materials.modified.is_empty() && materials.added.is_empty(), "a removal neither modifies nor adds");
    assert!(decoded.meshes.is_none() && decoded.textures.is_none(), "no mesh or texture slot may appear in the diff");
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&decoded)).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "delete-material/removes-the-leading-material-and-keeps-the-trailing-one: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioMeshDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed delete-material diff decodes");
    let produced = protocol::apply_diff(&decoded, &before()).expect("committed delete-material diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-material/removes-the-leading-material-and-keeps-the-trailing-one: committed diff did not carry before to after");
}
