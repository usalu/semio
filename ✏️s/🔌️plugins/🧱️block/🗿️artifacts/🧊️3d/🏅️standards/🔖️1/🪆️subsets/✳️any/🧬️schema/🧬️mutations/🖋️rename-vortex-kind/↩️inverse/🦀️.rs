//! ↩️ Inverse for `RenameVortexKind`.

use semio_s_artifact_block_3d_model::Block3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &crate::rename_vortex_kind::RenameVortexKind, base: &Block3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::rename_vortex_kind::RenameVortexKind> {
    Ok((|| {
    match semio_s_artifact_block_3d_model::vortex_kinds_of(base).iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![crate::rename_vortex_kind::rename_vortex_kind(payload.id.clone(), existing.name.clone())],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
