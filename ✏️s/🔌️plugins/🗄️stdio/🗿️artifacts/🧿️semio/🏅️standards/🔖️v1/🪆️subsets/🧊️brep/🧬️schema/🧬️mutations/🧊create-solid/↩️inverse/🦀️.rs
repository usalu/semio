//! ↩️ Inverse for `CreateSolid`.

use crate::standards::v1::subsets::brep::schema::mutations::{delete_solid, SemioBrepMutation};
use crate::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::CreateSolid, _base: &SemioBrepSnapshot) -> Result<Vec<SemioBrepMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![SemioBrepMutation::DeleteSolid(delete_solid::DeleteSolid { id: payload.id.clone() })]

    })())
}
//#endregion 🔖️Inverse
