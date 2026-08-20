//! ↩️ `rotate-object` — self-inverse: another `rotate-object` restoring the BASE-state rotation.

use super::mutation::RotateObject;
use crate::artifacts::semio::standards::v1::subsets::object::schema::mutations::SemioObjectMutation;
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(_payload: &RotateObject, base: &SemioObjectSnapshot) -> Vec<SemioObjectMutation> {
    vec![SemioObjectMutation::RotateObject(RotateObject { rotation: base.transform.rotation })]
}
//#endregion 🔖️Inverse
