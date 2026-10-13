//! ↩️ Inverse for `CreateFace`.

use crate::schema::mutations::{delete_face, SemioBrepMutation};
use crate::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::CreateFace, _base: &SemioBrepSnapshot) -> Result<Vec<SemioBrepMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![SemioBrepMutation::DeleteFace(delete_face::DeleteFace { id: payload.id.clone() })]

    })())
}
//#endregion 🔖️Inverse
