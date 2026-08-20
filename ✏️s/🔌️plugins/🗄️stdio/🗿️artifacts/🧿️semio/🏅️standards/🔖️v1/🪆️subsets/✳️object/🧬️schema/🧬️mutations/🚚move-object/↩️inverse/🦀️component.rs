//! ↩️ `move-object` — self-inverse: another `move-object` restoring the BASE-state translation.

use super::mutation::MoveObject;
use crate::artifacts::semio::standards::v1::subsets::object::schema::mutations::SemioObjectMutation;
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(_payload: &MoveObject, base: &SemioObjectSnapshot) -> Vec<SemioObjectMutation> {
    vec![SemioObjectMutation::MoveObject(MoveObject { translation: base.transform.translation })]
}
//#endregion 🔖️Inverse
