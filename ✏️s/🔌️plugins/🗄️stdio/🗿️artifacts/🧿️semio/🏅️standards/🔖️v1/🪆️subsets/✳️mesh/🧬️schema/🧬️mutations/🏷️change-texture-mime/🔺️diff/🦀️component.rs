//! 🏷️ `change-texture-mime` — Error `mutation.target-missing` when texture `id` is absent,
//! Warning `mutation.no-op` when `new_mime` already equals the current value.

use super::mutation::ChangeTextureMime;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::SemioMeshDiff;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeTextureMime, base: &SemioMeshSnapshot) -> protocol::MutationOutcome<SemioMeshDiff> {
    let Some(texture) = crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::texture_at(base, &payload.id).await else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Texture \"{}\" does not exist.", payload.id), [payload.id.clone()]).await;
    };
    if texture.mime == payload.new_mime {
        return protocol::MutationOutcome::empty().await.warn("mutation.no-op", format!("Texture \"{}\" mime type is already \"{}\".", payload.id, payload.new_mime)).await;
    }
    protocol::MutationOutcome::new(crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::diff_change_texture_mime(base, &payload.id, payload.new_mime.clone()))
}
//#endregion 🔖️Diff
