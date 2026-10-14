//! ↩️ Inverse for `ChangeGripKindLabel`.

use semio_s_artifact_block_5d_model::Block5dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &crate::change_grip_kind_label::ChangeGripKindLabel, base: &Block5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_grip_kind_label::ChangeGripKindLabel> {
    Ok((|| {
    match base.grip_kinds.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![crate::change_grip_kind_label::change_grip_kind_label(payload.id.clone(), existing.label.clone())],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
