//! ↩️ Inverse for `ChangeHandleKindColor`.

use crate::Block2dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block2dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ChangeHandleKindColor, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
    match base.handle_kinds.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::change_handle_kind_color::change_handle_kind_color(payload.id.clone(), existing.color.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
