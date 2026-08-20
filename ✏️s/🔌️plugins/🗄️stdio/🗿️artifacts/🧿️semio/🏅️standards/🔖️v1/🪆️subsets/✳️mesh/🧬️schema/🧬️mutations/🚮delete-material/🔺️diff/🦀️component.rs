//! 🚮 `delete-material` — Error `mutation.target-missing` when material `id` is absent. No
//! `mutation.cascade` message: the sibling `🦠️mutation/🦀️component.rs`'s doc comment records that
//! this mutation deliberately does NOT clear `material_id` references on primitives that pointed
//! at it (no membership-cascade verb exists; matches the pre-existing `RemoveMaterial` behaviour),
//! so there is no dependent set to compute or report here.

use super::mutation::DeleteMaterial;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::SemioMeshDiff;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &DeleteMaterial, base: &SemioMeshSnapshot) -> protocol::MutationOutcome<SemioMeshDiff> {
    if crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::material_at(base, &payload.id).await.is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Material \"{}\" does not exist.", payload.id), [payload.id.clone()]).await;
    }
    protocol::MutationOutcome::new(crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::diff_remove_material(base, &payload.id))
}
//#endregion 🔖️Diff
