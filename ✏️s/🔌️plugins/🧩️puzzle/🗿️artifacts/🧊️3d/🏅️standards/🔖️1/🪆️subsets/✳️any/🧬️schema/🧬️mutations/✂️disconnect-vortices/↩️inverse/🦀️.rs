//! ↩️ Inverse for `DisconnectVortices` — reconstructs a `connect-vortices` of the captured BASE
//! attraction. Missing target ⇒ `Vec::new()`.
use crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation;
use crate::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::DisconnectVortices, base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
    let Some(attraction) = base.attractions.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::standards::v1::subsets::any::schema::mutations::connect_vortices::mutation::connect_vortices(
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
        attraction.y,
    )]
}
//#endregion 🔖️Inverse
