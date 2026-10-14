//! ↩️ Inverse for `ChangeRepresentationMeshUrl`.

use semio_s_artifact_block_3d_model::Block3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &crate::change_representation_mesh_url::ChangeRepresentationMeshUrl, base: &Block3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_representation_mesh_url::ChangeRepresentationMeshUrl> {
    Ok((|| {
    match base.representations.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![crate::change_representation_mesh_url::change_representation_mesh_url(payload.id.clone(), existing.mesh_url.clone())],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
