//! ↩️ Inverse for `RenamePartKind`.

use semio_s_artifact_block_5d_model::Block5dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &crate::rename_part_kind::RenamePartKind, base: &Block5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::rename_part_kind::RenamePartKind> {
    Ok((|| {
    vec![crate::rename_part_kind::rename_part_kind(base.part_kind.name.clone())]

    })())
}
//#endregion 🔖️Inverse
