//! ↩️ Inverse for `RemoveObjectVortex` — reconstructs an `add-object-vortex` of the captured BASE
//! vortex, then re-`connect-vortices`es every attraction BASE shows touching it (severed cascade).
//! Missing target ⇒ `Vec::new()`.
use crate::artifacts::puzzle3d::mutations::Puzzle3dMutation;
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::RemoveObjectVortex, base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
    let Some(object) = base.objects.iter().find(|entry| entry.id == payload.object_id) else {
        return Vec::new();
    };
    let Some(vortex) = object.vortices.iter().find(|vortex| vortex.id == payload.vortex_id) else {
        return Vec::new();
    };
    let index = object.vortices.iter().position(|v| v.id == payload.vortex_id);
    let full_id = format!("{}:{}", payload.object_id, payload.vortex_id);
    let mut mutations = vec![crate::artifacts::puzzle3d::mutations::add_object_vortex::mutation::add_object_vortex(payload.object_id.clone(), vortex.clone(), index)];
    for attraction in base.attractions.iter().filter(|attraction| attraction.attracting == full_id || attraction.attracted == full_id) {
        mutations.push(crate::artifacts::puzzle3d::mutations::connect_vortices::mutation::connect_vortices(
            attraction.id.clone(), attraction.attracting.clone(), attraction.attracted.clone(),
            attraction.gap, attraction.shift, attraction.rise, attraction.rotation, attraction.turn, attraction.tilt, attraction.x, attraction.y,
        ));
    }
    mutations
}
//#endregion 🔖️Inverse
