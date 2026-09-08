//! ↩️ Inverse for `ChangeDescription` — restores the BASE description.
use crate::mutations::Puzzle5dMutation;
use crate::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ChangeDescription, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
    vec![crate::mutations::change_description::change_description(base.meta.description.clone())]
}
//#endregion 🔖️Inverse
