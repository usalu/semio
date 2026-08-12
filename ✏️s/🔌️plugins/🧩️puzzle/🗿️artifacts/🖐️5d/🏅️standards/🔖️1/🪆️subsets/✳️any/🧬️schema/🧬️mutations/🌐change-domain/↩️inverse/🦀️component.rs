//! ↩️ Inverse for `ChangeDomain` — restores the BASE domain.
use crate::artifacts::puzzle5d::mutations::Puzzle5dMutation;
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::mutation::ChangeDomain, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
    vec![crate::artifacts::puzzle5d::mutations::change_domain::mutation::change_domain(base.domain.clone())]
}
//#endregion 🔖️Inverse
