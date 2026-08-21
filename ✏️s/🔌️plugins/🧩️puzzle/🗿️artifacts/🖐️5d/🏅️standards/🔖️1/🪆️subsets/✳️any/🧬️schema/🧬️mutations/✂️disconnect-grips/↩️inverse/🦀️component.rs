//! ↩️ Inverse for `DisconnectGrips` — reconstructs a `connect-grips` of the captured BASE fastener.
//! Missing target ⇒ `Vec::new()`.
use crate::artifacts::puzzle5d::mutations::Puzzle5dMutation;
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::DisconnectGrips, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
    let Some(fastener) = base.fasteners.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::artifacts::puzzle5d::mutations::connect_grips::mutation::connect_grips(
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
        fastener.y,
    )]
}
//#endregion 🔖️Inverse
