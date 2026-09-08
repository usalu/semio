//! ↩️ Inverse for `ChangeDomain` — restores the BASE domain.
use crate::mutations::Puzzle3dMutation;
use crate::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::mutation::ChangeDomain, base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
    vec![crate::mutations::change_domain::mutation::change_domain(base.domain.clone())]
}
//#endregion 🔖️Inverse
