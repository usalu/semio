//! ↩️ Inverse for `CreateVertex`.

use crate::standards::v1::subsets::brep::schema::mutations::{delete_vertex, SemioBrepMutation};
use crate::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::CreateVertex, _base: &SemioBrepSnapshot) -> Result<Vec<SemioBrepMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![SemioBrepMutation::DeleteVertex(delete_vertex::DeleteVertex { id: payload.id.clone() })]

    })())
}
//#endregion 🔖️Inverse
