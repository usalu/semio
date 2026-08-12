//! ↩️ Inverse for `DeleteObject` — reconstructs a `create-object` of the captured BASE object,
//! then re-`connect-vortices`es every attraction BASE shows touching one of its vortices (severed
//! cascade). Missing target ⇒ `Vec::new()`.
use crate::artifacts::puzzle3d::mutations::Puzzle3dMutation;
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::DeleteObject, base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
    let Some(object) = base.objects.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    let index = base.objects.iter().position(|entry| entry.id == payload.id);
    let vortex_ids: Vec<String> = object.vortices.iter().map(|vortex| format!("{}:{}", object.id, vortex.id)).collect();
    let mut mutations = vec![crate::artifacts::puzzle3d::mutations::create_object::mutation::create_object(object.clone(), index)];
    for attraction in base.attractions.iter().filter(|attraction| vortex_ids.contains(&attraction.attracting) || vortex_ids.contains(&attraction.attracted)) {
        mutations.push(crate::artifacts::puzzle3d::mutations::connect_vortices::mutation::connect_vortices(
            attraction.id.clone(), attraction.attracting.clone(), attraction.attracted.clone(),
            attraction.gap, attraction.shift, attraction.rise, attraction.rotation, attraction.turn, attraction.tilt, attraction.x, attraction.y,
        ));
    }
    mutations
}
//#endregion 🔖️Inverse
