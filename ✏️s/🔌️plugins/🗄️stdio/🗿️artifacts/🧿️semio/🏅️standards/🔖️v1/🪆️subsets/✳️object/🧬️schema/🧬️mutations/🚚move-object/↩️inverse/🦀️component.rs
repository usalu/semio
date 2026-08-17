//! ↩️ `move-object` — self-inverse: another `move-object` restoring the BASE-state translation.

use super::mutation::MoveObject;
use crate::artifacts::semio::standards::v1::subsets::object::schema::mutations::SemioObjectMutation;
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &MoveObject, base: &SemioObjectSnapshot) -> Vec<SemioObjectMutation> {
    vec![SemioObjectMutation::MoveObject(MoveObject { translation: base.transform.translation })]
}
//#endregion 🔖️Inverse
