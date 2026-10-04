//! ↩️ Inverse for `ChangeHandleHandleKind`.

use crate::Block2dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block2dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ChangeHandleHandleKind, base: &Block2dSnapshot) -> Result<Vec<Block2dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.handles.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::change_handle_handle_kind::change_handle_handle_kind(payload.id.clone(), existing.handle_kind.clone())],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
