//! ↩️ `create-solid` — undo is `deletesolid` (`delete_solid`) at the same id.

use super::mutation::CreateSolid;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::mutations::{delete_solid, SemioBrepMutation};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &CreateSolid, _base: &SemioBrepSnapshot) -> Vec<SemioBrepMutation> {
    vec![SemioBrepMutation::DeleteSolid(delete_solid::mutation::DeleteSolid { id: payload.id.clone() })]
}
//#endregion 🔖️Inverse
