//! ↩️ Inverse for `ChangeObjectKindLabel`.

use semio_s_artifact_block_3d_model::Block3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &crate::change_object_kind_label::ChangeObjectKindLabel, base: &Block3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_object_kind_label::ChangeObjectKindLabel> {
    Ok((|| {
    vec![crate::change_object_kind_label::change_object_kind_label(base.object_kind.label.clone())]

    })())
}
//#endregion 🔖️Inverse
