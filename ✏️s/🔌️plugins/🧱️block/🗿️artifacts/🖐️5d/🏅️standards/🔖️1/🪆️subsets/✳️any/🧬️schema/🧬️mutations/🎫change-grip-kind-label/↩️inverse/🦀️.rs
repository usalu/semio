//! ↩️ Inverse for `ChangeGripKindLabel`.

use crate::Block5dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block5dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ChangeGripKindLabel, base: &Block5dSnapshot) -> Result<Vec<Block5dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.grip_kinds.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::change_grip_kind_label::change_grip_kind_label(payload.id.clone(), existing.label.clone())],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
