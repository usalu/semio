//! ↩️ Inverse for `UpdatePart3d`.

use semio_s_artifact_block_5d_model::Block5dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &crate::update_part_3d::UpdatePart3d, base: &Block5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::update_part_3d::UpdatePart3d> {
    Ok((|| {
    vec![crate::update_part_3d::update_part_3d(base.part_3d.orientation, base.part_3d.scale)]

    })())
}
//#endregion 🔖️Inverse
