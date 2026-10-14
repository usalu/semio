//! ↩️ Inverse for `ChangeVortexKindColor`.

use semio_s_artifact_block_3d_model::Block3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &crate::change_vortex_kind_color::ChangeVortexKindColor, base: &Block3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_vortex_kind_color::ChangeVortexKindColor> {
    Ok((|| {
    match semio_s_artifact_block_3d_model::vortex_kinds_of(base).iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![crate::change_vortex_kind_color::change_vortex_kind_color(payload.id.clone(), existing.color.clone())],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
