//! ↩️ Inverse for `ChangeObjectKindDescription`.

use semio_s_artifact_block_3d_model::Block3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &crate::change_object_kind_description::ChangeObjectKindDescription, base: &Block3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_object_kind_description::ChangeObjectKindDescription> {
    Ok((|| {
    vec![crate::change_object_kind_description::change_object_kind_description(base.object_kind.description.clone())]

    })())
}
//#endregion 🔖️Inverse
