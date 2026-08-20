//! 🖼️ `create-texture` — Fatal `mutation.duplicate-id` when texture `id` already exists.

use super::mutation::CreateTexture;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::SemioMeshDiff;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &CreateTexture, base: &SemioMeshSnapshot) -> protocol::MutationOutcome<SemioMeshDiff> {
    if crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::texture_at(base, &payload.texture.id).await.is_some() {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Texture \"{}\" already exists.", payload.texture.id), [payload.texture.id.clone()]).await;
    }
    protocol::MutationOutcome::new(crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::diff_add_texture(base, payload.texture.clone()))
}
//#endregion 🔖️Diff
