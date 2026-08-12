//! ↩️ `create-texture` — undo is `delete-texture` at the same id, unconditional.

use super::mutation::CreateTexture;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::{delete_texture, SemioMeshMutation};
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &CreateTexture, _base: &SemioMeshSnapshot) -> Vec<SemioMeshMutation> {
    vec![SemioMeshMutation::DeleteTexture(delete_texture::mutation::DeleteTexture { id: payload.texture.id.clone() })]
}
//#endregion 🔖️Inverse
