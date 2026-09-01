//! ↩️ Inverse for `CreateSolid`.

use crate::artifacts::semio::standards::v1::subsets::brep::schema::mutations::{SemioBrepMutation, delete_solid};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::{BrepSolid, BrepSolidShell, SemioBrepSnapshot};

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::CreateSolid, _base: &SemioBrepSnapshot) -> Vec<SemioBrepMutation> {
    vec![SemioBrepMutation::DeleteSolid(delete_solid::DeleteSolid { id: payload.id.clone() })]
}
//#endregion 🔖️Inverse
