//! ↩️ Inverse for `ResizeGrip3d`.

use crate::Block5dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block5dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ResizeGrip3d, base: &Block5dSnapshot) -> Result<Vec<Block5dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.grips.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::resize_grip_3d::resize_grip_3d(payload.id.clone(), existing.radius_3d)],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
