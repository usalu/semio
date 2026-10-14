//! ↩️ Inverse for `ChangeHandleHandleKind`.

use semio_s_artifact_block_2d_model::Block2dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &crate::change_handle_handle_kind::ChangeHandleHandleKind, base: &Block2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_handle_handle_kind::ChangeHandleHandleKind> {
    Ok((|| {
    match base.handles.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![crate::change_handle_handle_kind::change_handle_handle_kind(payload.id.clone(), existing.handle_kind.clone())],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
