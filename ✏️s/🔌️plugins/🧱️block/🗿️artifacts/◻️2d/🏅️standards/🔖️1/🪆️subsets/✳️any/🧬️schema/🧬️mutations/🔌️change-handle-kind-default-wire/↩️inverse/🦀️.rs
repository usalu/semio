//! ↩️ Inverse for `ChangeHandleKindDefaultWireKind`.

use semio_s_artifact_block_2d_model::Block2dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &crate::change_handle_kind_default_wire_kind::ChangeHandleKindDefaultWireKind, base: &Block2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_handle_kind_default_wire_kind::ChangeHandleKindDefaultWireKind> {
    Ok((|| {
    match base.handle_kinds.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![crate::change_handle_kind_default_wire_kind::change_handle_kind_default_wire_kind(payload.id.clone(), existing.default_wire_kind.clone())],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
