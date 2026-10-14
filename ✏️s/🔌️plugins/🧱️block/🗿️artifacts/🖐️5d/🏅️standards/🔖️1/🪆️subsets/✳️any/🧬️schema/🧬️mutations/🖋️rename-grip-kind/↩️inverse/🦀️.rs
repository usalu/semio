//! ↩️ Inverse for `RenameGripKind`.

use semio_s_artifact_block_5d_model::Block5dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &crate::rename_grip_kind::RenameGripKind, base: &Block5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::rename_grip_kind::RenameGripKind> {
    Ok((|| {
    match base.grip_kinds.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![crate::rename_grip_kind::rename_grip_kind(payload.id.clone(), existing.name.clone())],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
