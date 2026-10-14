//! ↩️ Inverse for `DeleteHandleKind`.

use semio_s_artifact_block_2d_model::Block2dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &crate::delete_handle_kind::DeleteHandleKind, base: &Block2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::create_handle_kind::CreateHandleKind> {
    Ok((|| {
    match base.handle_kinds.iter().enumerate().find(|(_, item)| item.id == payload.id) {
        Some((position, existing)) => vec![crate::create_handle_kind::create_handle_kind_at(existing.clone(), position as u32)],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
