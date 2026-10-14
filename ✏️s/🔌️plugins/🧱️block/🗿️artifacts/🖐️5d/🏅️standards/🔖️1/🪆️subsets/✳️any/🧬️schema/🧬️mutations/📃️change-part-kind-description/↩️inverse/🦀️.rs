//! ↩️ Inverse for `ChangePartKindDescription`.

use semio_s_artifact_block_5d_model::Block5dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &crate::change_part_kind_description::ChangePartKindDescription, base: &Block5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_part_kind_description::ChangePartKindDescription> {
    Ok((|| {
    vec![crate::change_part_kind_description::change_part_kind_description(base.part_kind.description.clone())]

    })())
}
//#endregion 🔖️Inverse
