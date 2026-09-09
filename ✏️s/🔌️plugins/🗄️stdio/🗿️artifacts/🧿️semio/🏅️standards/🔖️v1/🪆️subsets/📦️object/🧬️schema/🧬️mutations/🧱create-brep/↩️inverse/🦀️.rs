//! ↩️ Inverse for `CreateBrep`.

use crate::standards::v1::subsets::object::schema::mutations::{delete_brep, SemioObjectMutation};
use crate::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::CreateBrep, base: &SemioObjectSnapshot) -> Vec<SemioObjectMutation> {
    if base.brep.is_some() || crate::standards::v1::subsets::base::schema::child::validate_semio_child_identity(&payload.child_id, &payload.target, "brep").is_err() {
        return Vec::new();
    }
    vec![SemioObjectMutation::DeleteBrep(delete_brep::DeleteBrep {})]
}
//#endregion 🔖️Inverse
