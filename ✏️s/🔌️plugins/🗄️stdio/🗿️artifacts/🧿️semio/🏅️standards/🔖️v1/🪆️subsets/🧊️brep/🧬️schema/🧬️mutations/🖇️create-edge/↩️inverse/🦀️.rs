//! ↩️ Inverse for `CreateEdge`.

use crate::schema::mutations::delete_edge;
use crate::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::CreateEdge, _base: &SemioBrepSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::delete_edge::DeleteEdge>,
{
    Ok((|| {
    vec![Op::from(delete_edge::DeleteEdge { id: payload.id.clone() })]

    })())
}
//#endregion 🔖️Inverse
