//! 🕳️ `delete-texture` — Error `mutation.target-missing` when texture `id` is absent. No
//! `mutation.cascade` message: nothing in `SemioMeshSnapshot` references a texture by id
//! (`SemioPrimitive` only carries `material_id`; `SemioMaterial` carries no texture reference), so
//! there are no dependents to compute or report.

use super::mutation::DeleteTexture;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::SemioMeshDiff;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &DeleteTexture, base: &SemioMeshSnapshot) -> protocol::MutationOutcome<SemioMeshDiff> {
    if crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::texture_at(base, &payload.id).await.is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Texture \"{}\" does not exist.", payload.id), [payload.id.clone()]).await;
    }
    protocol::MutationOutcome::new(crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::diff_remove_texture(base, &payload.id))
}
//#endregion 🔖️Diff
