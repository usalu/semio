//! ↩️ `scale-object` — self-inverse: another `scale-object` restoring the BASE-state scale.

use super::mutation::ScaleObject;
use crate::artifacts::semio::standards::v1::subsets::object::schema::mutations::SemioObjectMutation;
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(_payload: &ScaleObject, base: &SemioObjectSnapshot) -> Vec<SemioObjectMutation> {
    vec![SemioObjectMutation::ScaleObject(ScaleObject { scale: base.transform.scale })]
}
//#endregion 🔖️Inverse
