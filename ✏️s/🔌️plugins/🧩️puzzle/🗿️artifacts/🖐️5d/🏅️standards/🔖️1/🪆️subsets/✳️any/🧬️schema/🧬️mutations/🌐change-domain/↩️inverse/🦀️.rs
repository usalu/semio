//! ↩️ Inverse for `ChangeDomain` — restores the BASE domain.
use crate::mutations::Puzzle5dMutation;
use crate::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ChangeDomain, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
    vec![crate::mutations::change_domain::change_domain(base.domain.clone())]
}
//#endregion 🔖️Inverse
