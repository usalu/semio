//! 🎨 `create-material` — Fatal `mutation.duplicate-id` when material `id` already exists.

use super::mutation::CreateMaterial;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::SemioMeshDiff;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &CreateMaterial, base: &SemioMeshSnapshot) -> protocol::MutationOutcome<SemioMeshDiff> {
    if crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::material_at(base, &payload.material.id).await.is_some() {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Material \"{}\" already exists.", payload.material.id), [payload.material.id.clone()]).await;
    }
    protocol::MutationOutcome::new(crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::diff_add_material(base, payload.material.clone()))
}
//#endregion 🔖️Diff
