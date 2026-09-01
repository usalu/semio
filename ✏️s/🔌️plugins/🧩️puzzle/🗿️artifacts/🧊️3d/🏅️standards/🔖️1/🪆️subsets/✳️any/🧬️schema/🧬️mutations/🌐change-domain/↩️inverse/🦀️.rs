//! ↩️ Inverse for `ChangeDomain` — restores the BASE domain.
use crate::artifacts::puzzle3d::mutations::Puzzle3dMutation;
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::mutation::ChangeDomain, base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
    vec![crate::artifacts::puzzle3d::mutations::change_domain::mutation::change_domain(base.domain.clone())]
}
//#endregion 🔖️Inverse
