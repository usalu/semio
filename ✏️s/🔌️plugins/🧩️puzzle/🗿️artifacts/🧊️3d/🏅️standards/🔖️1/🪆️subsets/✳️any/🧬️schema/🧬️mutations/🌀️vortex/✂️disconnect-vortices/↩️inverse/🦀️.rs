//! ↩️ Inverse for `DisconnectVortices` — reconstructs a `connect-vortices` of the captured BASE
//! attraction. Missing target ⇒ `Vec::new()`.
use super::mutation::DisconnectVortices;
use crate::standards::v1::subsets::any::schema::mutations::ConnectVortices;
use crate::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<DisconnectVortices> + From<ConnectVortices>>(payload: &super::mutation::DisconnectVortices, base: &Puzzle3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(at) = base.attractions.iter().position(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    let attraction = &base.attractions[at];
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
        attraction.y, Some(at),
    )]

    })())
}
//#endregion 🔖️Inverse
