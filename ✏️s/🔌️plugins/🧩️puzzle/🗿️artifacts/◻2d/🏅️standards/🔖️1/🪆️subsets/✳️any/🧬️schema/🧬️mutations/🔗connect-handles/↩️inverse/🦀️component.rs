//! ↩️ Inverse for `ConnectHandles` — always a `disconnect-handles` of the id it created.
use crate::artifacts::puzzle2d::mutations::Puzzle2dMutation;
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::ConnectHandles, _base: &Puzzle2dSnapshot) -> Vec<Puzzle2dMutation> {
    vec![crate::artifacts::puzzle2d::mutations::disconnect_handles::mutation::disconnect_handles(payload.id.clone())]
}
//#endregion 🔖️Inverse
