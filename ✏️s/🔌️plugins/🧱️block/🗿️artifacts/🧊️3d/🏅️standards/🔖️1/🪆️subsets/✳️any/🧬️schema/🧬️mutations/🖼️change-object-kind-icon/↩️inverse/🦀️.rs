//! ↩️ Inverse for `ChangeObjectKindIcon`.

use semio_s_artifact_block_3d_model::Block3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &crate::change_object_kind_icon::ChangeObjectKindIcon, base: &Block3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_object_kind_icon::ChangeObjectKindIcon> {
    Ok((|| {
    vec![crate::change_object_kind_icon::change_object_kind_icon(base.object_kind.icon.clone())]

    })())
}
//#endregion 🔖️Inverse
