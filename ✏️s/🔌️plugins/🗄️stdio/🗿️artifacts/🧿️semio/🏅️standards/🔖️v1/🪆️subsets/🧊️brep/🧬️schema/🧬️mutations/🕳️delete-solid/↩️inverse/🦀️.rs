//! ↩️ Inverse for `DeleteSolid`.

use crate::artifacts::semio::standards::v1::subsets::brep::schema::mutations::{SemioBrepMutation, create_solid, delete_solid};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::DeleteSolid, base: &SemioBrepSnapshot) -> Vec<SemioBrepMutation> {
    let Some(index) = base.solids.iter().position(|x| x.id == payload.id) else {
        return Vec::new();
    };
    let tail = &base.solids[index..];
    let mut undo: Vec<SemioBrepMutation> = tail
        .iter()
        .skip(1)
        .map(|x| SemioBrepMutation::DeleteSolid(delete_solid::DeleteSolid { id: x.id.clone() }))
        .collect();
    undo.extend(tail.iter().map(|x| SemioBrepMutation::CreateSolid(create_solid::CreateSolid { id: x.id.clone(), shells: x.shells.clone() })));
    undo
}
//#endregion 🔖️Inverse
