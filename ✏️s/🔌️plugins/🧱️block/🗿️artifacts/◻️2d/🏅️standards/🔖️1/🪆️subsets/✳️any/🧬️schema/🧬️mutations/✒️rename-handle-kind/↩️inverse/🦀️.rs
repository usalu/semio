//! ↩️ Inverse for `RenameHandleKind`.

use semio_s_artifact_block_2d_model::Block2dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &crate::rename_handle_kind::RenameHandleKind, base: &Block2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::rename_handle_kind::RenameHandleKind> {
    Ok((|| {
    match base.handle_kinds.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![crate::rename_handle_kind::rename_handle_kind(payload.id.clone(), existing.name.clone())],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
