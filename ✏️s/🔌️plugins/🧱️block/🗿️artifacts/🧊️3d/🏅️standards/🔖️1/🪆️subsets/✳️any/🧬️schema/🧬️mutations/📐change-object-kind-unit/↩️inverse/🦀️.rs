//! ↩️ Inverse for `ChangeObjectKindUnit`.

use semio_s_artifact_block_3d_model::Block3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &crate::change_object_kind_unit::ChangeObjectKindUnit, base: &Block3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_object_kind_unit::ChangeObjectKindUnit> {
    Ok((|| {
    vec![crate::change_object_kind_unit::change_object_kind_unit(base.object_kind.unit.clone())]

    })())
}
//#endregion 🔖️Inverse
