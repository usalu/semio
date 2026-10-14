//! ↩️ Inverse for `ChangePartKindUnit`.

use semio_s_artifact_block_5d_model::Block5dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &crate::change_part_kind_unit::ChangePartKindUnit, base: &Block5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_part_kind_unit::ChangePartKindUnit> {
    Ok((|| {
    vec![crate::change_part_kind_unit::change_part_kind_unit(base.part_kind.unit.clone())]

    })())
}
//#endregion 🔖️Inverse
