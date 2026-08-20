//! ↩️ `delete-solid` — reconstructs the removed solid from BASE via `CreateSolid`.
//! Missing target ⇒ `Vec::new()`.

use super::mutation::DeleteSolid;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::mutations::{create_solid, SemioBrepMutation};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &DeleteSolid, base: &SemioBrepSnapshot) -> Vec<SemioBrepMutation> {
    match base.solids.iter().find(|x| x.id == payload.id) {
        Some(x) => vec![SemioBrepMutation::CreateSolid(create_solid::mutation::CreateSolid { id: x.id.clone(), shells: x.shells.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
