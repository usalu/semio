//! ↩️ Inverse for `ChangePartKindLabel`.

use semio_s_artifact_block_5d_model::Block5dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &crate::change_part_kind_label::ChangePartKindLabel, base: &Block5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_part_kind_label::ChangePartKindLabel> {
    Ok((|| {
    vec![crate::change_part_kind_label::change_part_kind_label(base.part_kind.label.clone())]

    })())
}
//#endregion 🔖️Inverse
