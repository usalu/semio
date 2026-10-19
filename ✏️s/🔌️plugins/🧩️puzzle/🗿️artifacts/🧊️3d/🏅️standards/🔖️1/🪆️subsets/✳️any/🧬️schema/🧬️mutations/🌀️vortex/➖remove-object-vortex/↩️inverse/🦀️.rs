//! vortex, then re-`connect-vortices`es every attraction BASE shows touching it (severed cascade).
//! ↩️ Inverse for `RemoveObjectVortex` — reconstructs an `add-object-vortex` of the captured BASE
//! Missing target ⇒ `Vec::new()`.
use super::mutation::RemoveObjectVortex;
use {crate::add_object_vortex::mutation::AddObjectVortex, crate::connect_vortices::mutation::ConnectVortices};
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<RemoveObjectVortex> + From<AddObjectVortex> + From<ConnectVortices>>(payload: &super::mutation::RemoveObjectVortex, base: &Puzzle3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(object) = base.objects.iter().find(|entry| entry.id == payload.object_id) else {
        return Vec::new();
    };
    let Some(vortex) = object.vortices.iter().find(|vortex| vortex.id == payload.vortex_id) else {
        return Vec::new();
    };
    let index = object.vortices.iter().position(|v| v.id == payload.vortex_id);
    let full_id = format!("{}:{}", payload.object_id, payload.vortex_id);
    let mut mutations = vec![crate::add_object_vortex::mutation::add_object_vortex(payload.object_id.clone(), vortex.clone(), index)];
    for (at, attraction) in base.attractions.iter().enumerate().filter(|(_, attraction)| attraction.attracting == full_id || attraction.attracted == full_id) {
        mutations.push(crate::connect_vortices::mutation::connect_vortices(
            attraction.id.clone(),
            attraction.attracting.clone(),
            attraction.attracted.clone(),
            attraction.gap,
            attraction.shift,
            attraction.rise,
            attraction.rotation,
            attraction.turn,
            attraction.tilt,
            attraction.x,
            attraction.y, Some(at),
        ));
    }
    mutations.reverse();
    mutations

    })())
}
//#endregion 🔖️Inverse
