//! ↩️ Inverse for `CreateEdge`.

use crate::standards::v1::subsets::graph::schema::mutations::{delete_edge, SemioGraphMutation};
use crate::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::CreateEdge, base: &SemioGraphSnapshot) -> Result<Vec<SemioGraphMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let endpoints_exist = [&payload.source, &payload.target].iter().all(|id| base.nodes.iter().any(|n| n.id == **id));
    if !endpoints_exist || base.edges.iter().any(|e| e.id == payload.id) {
        return Vec::new();
    }
    vec![SemioGraphMutation::DeleteEdge(delete_edge::DeleteEdge { id: payload.id.clone() })]

    })())
}
//#endregion 🔖️Inverse
