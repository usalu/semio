//! ↩️ Restores the deleted texture at its original position without disturbing referenced siblings.

use crate::standards::v1::subsets::mesh::schema::mutations::{patch_snapshot, SemioMeshMutation};
use crate::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use semio_framework_value::ToValue;

pub fn inverse(payload: &super::DeleteTexture, base: &SemioMeshSnapshot) -> Result<Vec<SemioMeshMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(position) = base.textures.iter().position(|texture| texture.id == payload.id) else { return Vec::new(); };
    vec![SemioMeshMutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch: semio_s_artifact_stdio_contract::editing::SnapshotPatch::Insert {
        path: format!("/textures/{position}"), value: base.textures[position].to_value(), index: None,
    } })]

    })())
}
