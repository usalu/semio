//! ↩️ Inverse for `ChangeGripGripKind`.

use semio_s_artifact_block_5d_model::Block5dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &crate::change_grip_grip_kind::ChangeGripGripKind, base: &Block5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_grip_grip_kind::ChangeGripGripKind> {
    Ok((|| {
    match base.grips.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![crate::change_grip_grip_kind::change_grip_grip_kind(payload.id.clone(), existing.grip_kind.clone())],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
