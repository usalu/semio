//! ↩️ Inverse for `CreateVortex`.

use semio_s_artifact_block_3d_model::Block3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &crate::create_vortex::CreateVortex, _base: &Block3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::delete_vortex::DeleteVortex> {
    Ok((|| {
    vec![crate::delete_vortex::delete_vortex(payload.vortex.id.clone())]

    })())
}
//#endregion 🔖️Inverse
