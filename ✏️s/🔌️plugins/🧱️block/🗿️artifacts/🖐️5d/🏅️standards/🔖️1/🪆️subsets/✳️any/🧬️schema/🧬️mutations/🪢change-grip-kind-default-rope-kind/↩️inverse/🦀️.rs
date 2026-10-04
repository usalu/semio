//! ↩️ Inverse for `ChangeGripKindDefaultRopeKind`.

use crate::Block5dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block5dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ChangeGripKindDefaultRopeKind, base: &Block5dSnapshot) -> Result<Vec<Block5dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.grip_kinds.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::change_grip_kind_default_rope_kind::change_grip_kind_default_rope_kind(payload.id.clone(), existing.default_rope_kind.clone())],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
