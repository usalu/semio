//! ↩️ Inverse for `CreateBrep`.

use crate::artifacts::semio::standards::v1::subsets::object::schema::mutations::{SemioObjectMutation, delete_brep};
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(_payload: &super::CreateBrep, base: &SemioObjectSnapshot) -> Vec<SemioObjectMutation> {
    match &base.brep {
        Some(existing) => vec![SemioObjectMutation::CreateBrep(super::CreateBrep { child_id: existing.child_id.clone(), target: existing.target.clone() })],
        None => vec![SemioObjectMutation::DeleteBrep(delete_brep::DeleteBrep {})],
    }
}
//#endregion 🔖️Inverse
