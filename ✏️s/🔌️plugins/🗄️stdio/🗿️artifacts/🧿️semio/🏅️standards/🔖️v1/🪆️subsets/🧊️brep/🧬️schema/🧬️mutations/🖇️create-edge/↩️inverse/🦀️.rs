//! ↩️ Inverse for `CreateEdge`.

use crate::schema::mutations::{delete_edge, SemioBrepMutation};
use crate::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::CreateEdge, _base: &SemioBrepSnapshot) -> Result<Vec<SemioBrepMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![SemioBrepMutation::DeleteEdge(delete_edge::DeleteEdge { id: payload.id.clone() })]

    })())
}
//#endregion 🔖️Inverse
