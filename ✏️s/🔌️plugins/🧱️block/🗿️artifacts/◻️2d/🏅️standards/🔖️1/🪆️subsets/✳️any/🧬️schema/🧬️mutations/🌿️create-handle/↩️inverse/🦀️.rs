//! ↩️ Inverse for `CreateHandle`.

use semio_s_artifact_block_2d_model::Block2dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &crate::create_handle::CreateHandle, _base: &Block2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::delete_handle::DeleteHandle> {
    Ok((|| {
    vec![crate::delete_handle::delete_handle(payload.handle.id.clone())]

    })())
}
//#endregion 🔖️Inverse
