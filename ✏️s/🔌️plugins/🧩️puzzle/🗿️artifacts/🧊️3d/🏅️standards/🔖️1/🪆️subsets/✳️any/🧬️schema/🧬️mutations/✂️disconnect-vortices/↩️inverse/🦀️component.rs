//! ↩️ Inverse for `DisconnectVortices` — reconstructs a `connect-vortices` of the captured BASE
//! attraction. Missing target ⇒ `Vec::new()`.
use crate::artifacts::puzzle3d::mutations::Puzzle3dMutation;
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::DisconnectVortices, base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
    let Some(attraction) = base.attractions.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::artifacts::puzzle3d::mutations::connect_vortices::mutation::connect_vortices(
        attraction.id.clone(), attraction.attracting.clone(), attraction.attracted.clone(),
        attraction.gap, attraction.shift, attraction.rise, attraction.rotation, attraction.turn, attraction.tilt, attraction.x, attraction.y,
    )]
}
//#endregion 🔖️Inverse
