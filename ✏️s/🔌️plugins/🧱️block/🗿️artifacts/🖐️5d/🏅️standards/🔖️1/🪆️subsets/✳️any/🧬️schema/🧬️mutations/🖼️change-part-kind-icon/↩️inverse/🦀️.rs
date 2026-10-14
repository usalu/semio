//! ↩️ Inverse for `ChangePartKindIcon`.

use semio_s_artifact_block_5d_model::Block5dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &crate::change_part_kind_icon::ChangePartKindIcon, base: &Block5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_part_kind_icon::ChangePartKindIcon> {
    Ok((|| {
    vec![crate::change_part_kind_icon::change_part_kind_icon(base.part_kind.icon.clone())]

    })())
}
//#endregion 🔖️Inverse
