//! ↩️ Inverse for `CreateVortexKind`.

use semio_s_artifact_block_3d_model::Block3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &crate::create_vortex_kind::CreateVortexKind, _base: &Block3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::delete_vortex_kind::DeleteVortexKind> {
    Ok((|| {
    vec![crate::delete_vortex_kind::delete_vortex_kind(payload.vortex_kind.id.clone())]

    })())
}
//#endregion 🔖️Inverse
