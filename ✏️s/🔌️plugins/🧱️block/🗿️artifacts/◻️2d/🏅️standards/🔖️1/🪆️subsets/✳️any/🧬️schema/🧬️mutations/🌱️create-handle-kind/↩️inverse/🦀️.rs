//! ↩️ Inverse for `CreateHandleKind`.

use semio_s_artifact_block_2d_model::Block2dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &crate::create_handle_kind::CreateHandleKind, _base: &Block2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::delete_handle_kind::DeleteHandleKind> {
    Ok((|| {
    vec![crate::delete_handle_kind::delete_handle_kind(payload.handle_kind.id.clone())]

    })())
}
//#endregion 🔖️Inverse
