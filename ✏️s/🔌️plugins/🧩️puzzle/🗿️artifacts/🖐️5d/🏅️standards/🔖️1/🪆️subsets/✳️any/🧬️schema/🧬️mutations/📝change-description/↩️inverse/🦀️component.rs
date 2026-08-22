//! ↩️ Inverse for `ChangeDescription` — restores the BASE description.
use crate::artifacts::puzzle5d::mutations::Puzzle5dMutation;
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::mutation::ChangeDescription, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
    vec![crate::artifacts::puzzle5d::mutations::change_description::mutation::change_description(base.meta.description.clone())]
}
//#endregion 🔖️Inverse
