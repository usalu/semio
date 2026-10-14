//! ↩️ Inverse for `ChangeHandleKindColor`.

use semio_s_artifact_block_2d_model::Block2dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &crate::change_handle_kind_color::ChangeHandleKindColor, base: &Block2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_handle_kind_color::ChangeHandleKindColor> {
    Ok((|| {
    match base.handle_kinds.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![crate::change_handle_kind_color::change_handle_kind_color(payload.id.clone(), existing.color.clone())],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
