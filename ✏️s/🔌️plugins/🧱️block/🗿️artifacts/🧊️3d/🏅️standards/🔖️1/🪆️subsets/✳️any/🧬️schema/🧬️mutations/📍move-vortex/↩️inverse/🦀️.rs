//! ↩️ Inverse for `MoveVortex`.

use semio_s_artifact_block_3d_model::Block3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &crate::move_vortex::MoveVortex, base: &Block3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::move_vortex::MoveVortex> {
    Ok((|| {
    match base.vortices.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![crate::move_vortex::move_vortex(payload.id.clone(), existing.position, existing.direction)],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
