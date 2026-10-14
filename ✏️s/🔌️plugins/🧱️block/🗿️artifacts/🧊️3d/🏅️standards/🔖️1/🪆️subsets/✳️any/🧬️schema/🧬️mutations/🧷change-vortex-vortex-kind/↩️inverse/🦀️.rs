//! ↩️ Inverse for `ChangeVortexVortexKind`.

use semio_s_artifact_block_3d_model::Block3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &crate::change_vortex_vortex_kind::ChangeVortexVortexKind, base: &Block3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_vortex_vortex_kind::ChangeVortexVortexKind> {
    Ok((|| {
    match base.vortices.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![crate::change_vortex_vortex_kind::change_vortex_vortex_kind(payload.id.clone(), existing.vortex_kind.clone())],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
