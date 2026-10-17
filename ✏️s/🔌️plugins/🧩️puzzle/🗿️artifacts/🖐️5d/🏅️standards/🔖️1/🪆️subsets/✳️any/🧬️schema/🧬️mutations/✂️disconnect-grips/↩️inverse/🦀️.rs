//! ↩️ Inverse for `DisconnectGrips` — reconstructs a `connect-grips` of the captured BASE fastener.
//! Missing target ⇒ `Vec::new()`.
use crate::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &super::DisconnectGrips, base: &Puzzle5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::connect_grips::ConnectGrips>,
{
    Ok((|| {
    let Some(at) = base.fasteners.iter().position(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    let fastener = &base.fasteners[at];
    vec![crate::standards::v1::subsets::any::schema::mutations::connect_grips::connect_grips(
        fastener.id.clone(),
        fastener.source.clone(),
        fastener.target.clone(),
        fastener.fastener_kind.clone(),
        fastener.gap,
        fastener.shift,
        fastener.rise,
        fastener.rotation,
        fastener.turn,
        fastener.tilt,
        fastener.x,
        fastener.y, Some(at),
    )]

    })())
}
//#endregion 🔖️Inverse
