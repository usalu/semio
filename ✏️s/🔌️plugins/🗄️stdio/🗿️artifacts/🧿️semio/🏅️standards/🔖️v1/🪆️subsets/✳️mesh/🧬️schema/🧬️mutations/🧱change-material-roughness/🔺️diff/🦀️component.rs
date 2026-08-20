//! 🧱 `change-material-roughness` — Error `mutation.target-missing` when material `id` is absent,
//! Warning `mutation.no-op` when `new_roughness` already equals the current value, Fatal
//! `mutation.invariant` when `new_roughness` is not finite.

use super::mutation::ChangeMaterialRoughness;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::SemioMeshDiff;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeMaterialRoughness, base: &SemioMeshSnapshot) -> protocol::MutationOutcome<SemioMeshDiff> {
    let Some(material) = crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::material_at(base, &payload.id).await else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Material \"{}\" does not exist.", payload.id), [payload.id.clone()]).await;
    };
    if material.roughness == payload.new_roughness {
        return protocol::MutationOutcome::empty().await.warn("mutation.no-op", format!("Material \"{}\" roughness factor is already {}.", payload.id, payload.new_roughness)).await;
    }
    if !payload.new_roughness.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Material \"{}\" roughness factor {} is not finite.", payload.id, payload.new_roughness), [payload.id.clone()]).await;
    }
    protocol::MutationOutcome::new(crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::diff_change_material_roughness(base, &payload.id, payload.new_roughness))
}
//#endregion 🔖️Diff
