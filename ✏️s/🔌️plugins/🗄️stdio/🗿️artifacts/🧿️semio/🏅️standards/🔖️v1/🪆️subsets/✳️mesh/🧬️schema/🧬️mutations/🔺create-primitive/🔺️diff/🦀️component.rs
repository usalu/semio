//! 🔺 `create-primitive` — Error `mutation.target-missing` when `mesh_id` is absent, Fatal
//! `mutation.duplicate-id` when `primitive.id` already exists in that mesh. Referential integrity
//! of `primitive.material_id` against the materials collection is the subset's
//! `SemioMeshValidator`'s job (`🚪️io/🦀️component.rs`'s `check_mesh_referential_invariants` /
//! `stdio.semio_mesh.dangling-material-ref`), not this leaf's.

use super::mutation::CreatePrimitive;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::SemioMeshDiff;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &CreatePrimitive, base: &SemioMeshSnapshot) -> protocol::MutationOutcome<SemioMeshDiff> {
    let Some(mesh) = crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::mesh_at(base, &payload.mesh_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Mesh \"{}\" does not exist.", payload.mesh_id), [payload.mesh_id.clone()]);
    };
    if mesh.primitives.iter().any(|p| p.id == payload.primitive.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Primitive \"{}\" already exists in mesh \"{}\".", payload.primitive.id, payload.mesh_id), [payload.primitive.id.clone()]);
    }
    protocol::MutationOutcome::new(crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::diff_add_primitive(base, &payload.mesh_id, payload.primitive.clone()))
}
//#endregion 🔖️Diff
