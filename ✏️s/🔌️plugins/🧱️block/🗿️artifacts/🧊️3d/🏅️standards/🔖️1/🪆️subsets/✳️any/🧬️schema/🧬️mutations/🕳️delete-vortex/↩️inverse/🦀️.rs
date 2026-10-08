//! ↩️ Inverse for `DeleteVortex`.

use crate::Block3dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block3dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::DeleteVortex, base: &Block3dSnapshot) -> Result<Vec<Block3dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.vortices.iter().enumerate().find(|(_, item)| item.id == payload.id) {
        Some((position, existing)) => vec![super::super::create_vortex::create_vortex_at(existing.clone(), position as u32)],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
