//! ↩️ Inverse for `RenamePuzzle5d` — restores the BASE label.
use crate::artifacts::puzzle5d::mutations::Puzzle5dMutation;
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::mutation::RenamePuzzle5d, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
    vec![crate::artifacts::puzzle5d::mutations::rename_puzzle5d::mutation::rename_puzzle5d(base.label.clone())]
}
//#endregion 🔖️Inverse
