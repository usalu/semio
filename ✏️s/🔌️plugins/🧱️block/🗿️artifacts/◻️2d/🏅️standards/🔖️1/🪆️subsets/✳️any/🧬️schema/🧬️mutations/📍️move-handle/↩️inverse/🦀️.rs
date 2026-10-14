//! ↩️ Inverse for `MoveHandle`.

use semio_s_artifact_block_2d_model::Block2dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &crate::move_handle::MoveHandle, base: &Block2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::move_handle::MoveHandle> {
    Ok((|| {
    match base.handles.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![crate::move_handle::move_handle(payload.id.clone(), existing.angle, existing.radius)],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
