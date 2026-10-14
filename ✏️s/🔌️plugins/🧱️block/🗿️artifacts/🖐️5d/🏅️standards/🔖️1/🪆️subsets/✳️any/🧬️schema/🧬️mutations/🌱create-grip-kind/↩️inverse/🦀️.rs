//! ↩️ Inverse for `CreateGripKind`.

use semio_s_artifact_block_5d_model::Block5dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &crate::create_grip_kind::CreateGripKind, _base: &Block5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::delete_grip_kind::DeleteGripKind> {
    Ok((|| {
    vec![crate::delete_grip_kind::delete_grip_kind(payload.grip_kind.id.clone())]

    })())
}
//#endregion 🔖️Inverse
