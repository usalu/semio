//! ↩️ Inverse for `ConnectGrips` — always a `disconnect-grips` of the id it created.
use crate::mutations::Puzzle5dMutation;
use crate::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ConnectGrips, _base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
    vec![crate::mutations::disconnect_grips::disconnect_grips(payload.id.clone())]
}
//#endregion 🔖️Inverse
