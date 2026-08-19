//! ↩️ `rotate-object` — self-inverse: another `rotate-object` restoring the BASE-state rotation.

use super::mutation::RotateObject;
use crate::artifacts::semio::standards::v1::subsets::object::schema::mutations::SemioObjectMutation;
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &RotateObject, base: &SemioObjectSnapshot) -> Vec<SemioObjectMutation> {
    vec![SemioObjectMutation::RotateObject(RotateObject { rotation: base.transform.rotation })]
}
//#endregion 🔖️Inverse
