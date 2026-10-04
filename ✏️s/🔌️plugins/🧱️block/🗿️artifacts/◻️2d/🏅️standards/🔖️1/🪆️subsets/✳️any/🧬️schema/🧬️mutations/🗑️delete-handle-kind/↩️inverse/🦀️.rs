//! ↩️ Inverse for `DeleteHandleKind`.

use crate::Block2dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block2dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::DeleteHandleKind, base: &Block2dSnapshot) -> Result<Vec<Block2dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.handle_kinds.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::create_handle_kind::create_handle_kind(existing.clone())],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
