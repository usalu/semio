//! ↩️ Inverse for `ChangeHandleKindLabel`.

use semio_s_artifact_block_2d_model::Block2dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &crate::change_handle_kind_label::ChangeHandleKindLabel, base: &Block2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_handle_kind_label::ChangeHandleKindLabel> {
    Ok((|| {
    match base.handle_kinds.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![crate::change_handle_kind_label::change_handle_kind_label(payload.id.clone(), existing.label.clone())],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
