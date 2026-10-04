//! ↩️ Inverse for `DeleteHandle`.

use crate::Block2dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block2dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::DeleteHandle, base: &Block2dSnapshot) -> Result<Vec<Block2dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.handles.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::create_handle::create_handle(existing.clone())],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
