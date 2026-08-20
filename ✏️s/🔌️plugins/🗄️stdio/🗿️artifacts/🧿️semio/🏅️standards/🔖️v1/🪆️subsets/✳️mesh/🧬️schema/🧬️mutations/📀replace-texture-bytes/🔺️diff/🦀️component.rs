//! 📀 `replace-texture-bytes` — Error `mutation.target-missing` when texture `id` is absent,
//! Warning `mutation.no-op` when `new_bytes` already equals the current bytes.

use super::mutation::ReplaceTextureBytes;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::SemioMeshDiff;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ReplaceTextureBytes, base: &SemioMeshSnapshot) -> protocol::MutationOutcome<SemioMeshDiff> {
    let Some(texture) = crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::texture_at(base, &payload.id).await else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Texture \"{}\" does not exist.", payload.id), [payload.id.clone()]).await;
    };
    if texture.bytes == payload.new_bytes {
        return protocol::MutationOutcome::empty().await.warn("mutation.no-op", format!("Texture \"{}\" bytes are unchanged.", payload.id)).await;
    }
    protocol::MutationOutcome::new(crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::diff_replace_texture_bytes(base, &payload.id, payload.new_bytes.clone()))
}
//#endregion 🔖️Diff
