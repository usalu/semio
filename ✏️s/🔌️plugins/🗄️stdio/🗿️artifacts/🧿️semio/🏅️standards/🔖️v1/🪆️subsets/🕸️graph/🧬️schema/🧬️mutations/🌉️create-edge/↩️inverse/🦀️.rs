//! ↩️ Inverse for `CreateEdge`.

use crate::schema::mutations::delete_edge;
use crate::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::CreateEdge, base: &SemioGraphSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::delete_edge::DeleteEdge>,
{
    Ok((|| {
    let endpoints_exist = [&payload.source, &payload.target].iter().all(|id| base.nodes.iter().any(|n| n.id == **id));
    if !endpoints_exist || base.edges.iter().any(|e| e.id == payload.id) {
        return Vec::new();
    }
    vec![Op::from(delete_edge::DeleteEdge { id: payload.id.clone() })]

    })())
}
//#endregion 🔖️Inverse
