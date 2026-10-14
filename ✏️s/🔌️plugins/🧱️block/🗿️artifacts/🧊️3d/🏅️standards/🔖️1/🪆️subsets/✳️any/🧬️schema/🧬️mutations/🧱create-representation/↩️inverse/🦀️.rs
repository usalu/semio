//! ↩️ Inverse for `CreateRepresentation`.

use semio_s_artifact_block_3d_model::Block3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &crate::create_representation::CreateRepresentation, _base: &Block3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::delete_representation::DeleteRepresentation> {
    Ok((|| {
    vec![crate::delete_representation::delete_representation(payload.representation.id.clone())]

    })())
}
//#endregion 🔖️Inverse
