//! ↩️ Inverse for `ResizeVortex`.

use crate::Block3dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block3dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ResizeVortex, base: &Block3dSnapshot) -> Result<Vec<Block3dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.vortices.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::resize_vortex::resize_vortex(payload.id.clone(), existing.radius)],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
