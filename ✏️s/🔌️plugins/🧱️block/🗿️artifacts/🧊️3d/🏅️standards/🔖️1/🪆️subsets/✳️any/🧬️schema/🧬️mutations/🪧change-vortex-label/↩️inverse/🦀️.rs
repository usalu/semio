//! ↩️ Inverse for `ChangeVortexLabel`.

use semio_s_artifact_block_3d_model::Block3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &crate::change_vortex_label::ChangeVortexLabel, base: &Block3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_vortex_label::ChangeVortexLabel> {
    Ok((|| {
    match base.vortices.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![crate::change_vortex_label::change_vortex_label(payload.id.clone(), existing.label.clone())],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
