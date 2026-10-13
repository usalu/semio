//! ↩️ Inverse for `DeleteSolid`.

use crate::schema::mutations::{create_solid, SemioBrepMutation};
use crate::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::DeleteSolid, base: &SemioBrepSnapshot) -> Result<Vec<SemioBrepMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(index) = base.solids.iter().position(|x| x.id == payload.id) else {
        return Vec::new();
    };
    let x = &base.solids[index];
    vec![SemioBrepMutation::CreateSolid(create_solid::CreateSolid { id: x.id.clone(), shells: x.shells.clone(), at: Some(index) })]

    })())
}
//#endregion 🔖️Inverse
