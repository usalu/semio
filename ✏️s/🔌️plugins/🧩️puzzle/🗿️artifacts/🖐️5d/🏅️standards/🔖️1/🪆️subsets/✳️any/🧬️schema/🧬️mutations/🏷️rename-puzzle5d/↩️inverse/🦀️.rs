//! ↩️ Inverse for `RenamePuzzle5d` — restores the BASE label.
use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
use crate::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::RenamePuzzle5d, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
    vec![crate::standards::v1::subsets::any::schema::mutations::rename_puzzle5d::rename_puzzle5d(base.label.clone())]
}
//#endregion 🔖️Inverse
