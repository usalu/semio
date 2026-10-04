//! ↩️ Inverse for `MoveVortex`.

use crate::Block3dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block3dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::MoveVortex, base: &Block3dSnapshot) -> Result<Vec<Block3dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.vortices.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::move_vortex::move_vortex(payload.id.clone(), existing.position, existing.direction)],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
