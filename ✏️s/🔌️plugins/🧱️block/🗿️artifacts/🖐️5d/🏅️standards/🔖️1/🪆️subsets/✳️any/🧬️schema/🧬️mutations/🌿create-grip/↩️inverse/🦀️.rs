//! ↩️ Inverse for `CreateGrip`.

use semio_s_artifact_block_5d_model::Block5dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &crate::create_grip::CreateGrip, _base: &Block5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::delete_grip::DeleteGrip> {
    Ok((|| {
    vec![crate::delete_grip::delete_grip(payload.grip.id.clone())]

    })())
}
//#endregion 🔖️Inverse
