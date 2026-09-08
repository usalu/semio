//! ↩️ Inverse for `ChangeDomain` — restores the BASE domain.
use crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation;
use crate::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::mutation::ChangeDomain, base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
    vec![crate::standards::v1::subsets::any::schema::mutations::change_domain::mutation::change_domain(base.domain.clone())]
}
//#endregion 🔖️Inverse
