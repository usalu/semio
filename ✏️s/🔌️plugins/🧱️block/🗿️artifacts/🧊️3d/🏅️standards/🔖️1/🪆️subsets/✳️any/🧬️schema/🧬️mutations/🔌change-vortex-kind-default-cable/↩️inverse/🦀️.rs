//! ↩️ Inverse for `ChangeVortexKindDefaultCableKind`.

use semio_s_artifact_block_3d_model::Block3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &crate::change_vortex_kind_default_cable_kind::ChangeVortexKindDefaultCableKind, base: &Block3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_vortex_kind_default_cable_kind::ChangeVortexKindDefaultCableKind> {
    Ok((|| {
    match semio_s_artifact_block_3d_model::vortex_kinds_of(base).iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![crate::change_vortex_kind_default_cable_kind::change_vortex_kind_default_cable_kind(payload.id.clone(), existing.default_cable_kind.clone())],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
