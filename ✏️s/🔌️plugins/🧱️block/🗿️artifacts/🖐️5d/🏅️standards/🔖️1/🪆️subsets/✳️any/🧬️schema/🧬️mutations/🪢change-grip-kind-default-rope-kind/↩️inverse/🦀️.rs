//! ↩️ Inverse for `ChangeGripKindDefaultRopeKind`.

use semio_s_artifact_block_5d_model::Block5dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &crate::change_grip_kind_default_rope_kind::ChangeGripKindDefaultRopeKind, base: &Block5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_grip_kind_default_rope_kind::ChangeGripKindDefaultRopeKind> {
    Ok((|| {
    match base.grip_kinds.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![crate::change_grip_kind_default_rope_kind::change_grip_kind_default_rope_kind(payload.id.clone(), existing.default_rope_kind.clone())],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
