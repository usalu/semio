//! ↩️ Inverse for `CreateFace`.

use crate::artifacts::semio::standards::v1::subsets::brep::schema::mutations::{SemioBrepMutation, delete_face};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::{BrepFace, BrepSurface, SemioBrepSnapshot};

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::CreateFace, _base: &SemioBrepSnapshot) -> Vec<SemioBrepMutation> {
    vec![SemioBrepMutation::DeleteFace(delete_face::DeleteFace { id: payload.id.clone() })]
}
//#endregion 🔖️Inverse
