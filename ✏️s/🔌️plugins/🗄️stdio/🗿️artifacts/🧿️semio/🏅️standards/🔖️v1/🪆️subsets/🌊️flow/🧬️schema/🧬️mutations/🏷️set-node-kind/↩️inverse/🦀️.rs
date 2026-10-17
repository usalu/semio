//! ↩️ Inverse for `SetNodeKind`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioFlowSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::support::node_at;
#[allow(unused_imports)]
use crate::schema::mutations::set_node_kind;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::SetNodeKind, base: &SemioFlowSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::set_node_kind::SetNodeKind>,
{
    let super::SetNodeKind { id, .. } = payload;
    Ok(match node_at(base, id) {
        Some(node) => vec![Op::from(set_node_kind::SetNodeKind { id: id.clone(), kind: node.kind.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
