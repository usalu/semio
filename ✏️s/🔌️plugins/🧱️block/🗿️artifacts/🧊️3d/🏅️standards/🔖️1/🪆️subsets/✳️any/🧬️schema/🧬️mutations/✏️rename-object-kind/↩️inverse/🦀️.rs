//! ↩️ Inverse for `RenameObjectKind`.

use semio_s_artifact_block_3d_model::Block3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &crate::rename_object_kind::RenameObjectKind, base: &Block3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::rename_object_kind::RenameObjectKind> {
    Ok((|| {
    vec![crate::rename_object_kind::rename_object_kind(base.object_kind.name.clone())]

    })())
}
//#endregion 🔖️Inverse
