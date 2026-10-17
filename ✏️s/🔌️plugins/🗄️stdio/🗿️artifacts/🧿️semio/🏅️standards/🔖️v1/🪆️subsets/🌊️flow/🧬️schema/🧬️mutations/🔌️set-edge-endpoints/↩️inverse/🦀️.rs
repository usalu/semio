//! ↩️ Inverse for `SetEdgeEndpoints`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioFlowSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::support::edge_at;
#[allow(unused_imports)]
use crate::schema::mutations::set_edge_endpoints;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::SetEdgeEndpoints, base: &SemioFlowSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::set_edge_endpoints::SetEdgeEndpoints>,
{
    let super::SetEdgeEndpoints { id, .. } = payload;
    Ok(match edge_at(base, id) {
        Some(edge) => vec![Op::from(set_edge_endpoints::SetEdgeEndpoints { id: id.clone(), from: edge.from.clone(), to: edge.to.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
