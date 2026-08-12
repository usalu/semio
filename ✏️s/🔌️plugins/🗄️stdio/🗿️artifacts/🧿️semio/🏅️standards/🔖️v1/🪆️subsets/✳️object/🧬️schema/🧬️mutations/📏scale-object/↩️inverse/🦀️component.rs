//! ↩️ `scale-object` — self-inverse: another `scale-object` restoring the BASE-state scale.

use super::mutation::ScaleObject;
use crate::artifacts::semio::standards::v1::subsets::object::schema::mutations::SemioObjectMutation;
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ScaleObject, base: &SemioObjectSnapshot) -> Vec<SemioObjectMutation> {
    vec![SemioObjectMutation::ScaleObject(super::mutation::ScaleObject { scale: base.transform.scale })]
}
//#endregion 🔖️Inverse
