//! `Vec::new()`.
//! ↩️ Inverse for `ReplaceObjectVortex` — restores the BASE vortex payload. Missing target ⇒
use super::mutation::ReplaceObjectVortex;
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<ReplaceObjectVortex>>(payload: &super::mutation::ReplaceObjectVortex, base: &Puzzle3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(object) = base.objects.iter().find(|entry| entry.id == payload.object_id) else {
        return Vec::new();
    };
    let Some(vortex) = object.vortices.iter().find(|vortex| vortex.id == payload.vortex_id) else {
        return Vec::new();
    };
    vec![crate::replace_object_vortex::mutation::replace_object_vortex(payload.object_id.clone(), payload.vortex_id.clone(), vortex.clone())]

    })())
}
//#endregion 🔖️Inverse
