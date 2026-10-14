//! ↩️ Inverse for `UpdatePart2d`.

use semio_s_artifact_block_5d_model::Block5dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &crate::update_part_2d::UpdatePart2d, base: &Block5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::update_part_2d::UpdatePart2d> {
    Ok((|| {
    vec![crate::update_part_2d::update_part_2d(base.part_2d.shape.clone(), base.part_2d.radius, base.part_2d.width, base.part_2d.height, base.part_2d.color.clone(), base.part_2d.icon_kind.clone())]

    })())
}
//#endregion 🔖️Inverse
