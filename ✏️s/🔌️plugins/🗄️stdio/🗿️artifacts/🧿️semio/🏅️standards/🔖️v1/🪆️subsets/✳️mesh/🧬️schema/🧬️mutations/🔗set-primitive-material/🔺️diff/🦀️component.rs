//! 🔗 `set-primitive-material` — Error `mutation.target-missing` when the
//! (`mesh_id`,`primitive_id`) pair is absent, Warning `mutation.no-op` when `material_id` already
//! matches the current value. Referential integrity of `material_id` against the materials
//! collection is the subset's `SemioMeshValidator`'s job (`🚪️io/🦀️component.rs`'s
//! `check_mesh_referential_invariants` / `stdio.semio_mesh.dangling-material-ref`), not this
//! leaf's — its own doc comment names only the address/no-op cases.

use super::mutation::SetPrimitiveMaterial;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::SemioMeshDiff;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &SetPrimitiveMaterial, base: &SemioMeshSnapshot) -> protocol::MutationOutcome<SemioMeshDiff> {
    let Some(primitive) = crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::primitive_at(base, &payload.mesh_id, &payload.primitive_id).await else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Primitive \"{}\" does not exist in mesh \"{}\".", payload.primitive_id, payload.mesh_id), [payload.primitive_id.clone()]).await;
    };
    if primitive.material_id == payload.material_id {
        return protocol::MutationOutcome::empty().await.warn("mutation.no-op", format!("Primitive \"{}\" material is unchanged.", payload.primitive_id)).await;
    }
    protocol::MutationOutcome::new(crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::diff_set_primitive_material(base, &payload.mesh_id, &payload.primitive_id, payload.material_id.clone()))
}
//#endregion 🔖️Diff
