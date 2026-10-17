//! ↩️ Inverse for `SetEdgeKind`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioFlowSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::support::edge_at;
#[allow(unused_imports)]
use crate::schema::mutations::set_edge_kind;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::SetEdgeKind, base: &SemioFlowSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::set_edge_kind::SetEdgeKind>,
{
    let super::SetEdgeKind { id, .. } = payload;
    Ok(match edge_at(base, id) {
        Some(edge) => vec![Op::from(set_edge_kind::SetEdgeKind { id: id.clone(), kind: edge.kind.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
