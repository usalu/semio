//! ↩️ Inverse for `DeleteHandle`.

use semio_s_artifact_block_2d_model::Block2dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &crate::delete_handle::DeleteHandle, base: &Block2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::create_handle::CreateHandle> {
    Ok((|| {
    match base.handles.iter().enumerate().find(|(_, item)| item.id == payload.id) {
        Some((position, existing)) => vec![crate::create_handle::create_handle_at(existing.clone(), position as u32)],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
