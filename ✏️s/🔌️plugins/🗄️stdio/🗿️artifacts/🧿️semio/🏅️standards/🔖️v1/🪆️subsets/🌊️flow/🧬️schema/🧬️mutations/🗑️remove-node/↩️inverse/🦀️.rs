//! ↩️ Inverse for `RemoveNode`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioFlowSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::insert_node;
#[allow(unused_imports)]
use crate::schema::mutations::support::node_at;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::RemoveNode, base: &SemioFlowSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::insert_node::InsertNode>,
{
    let super::RemoveNode { id } = payload;
    Ok(match node_at(base, id) {
        Some(node) => vec![Op::from(insert_node::InsertNode { node: node.clone(), at: base.nodes.iter().position(|n| n.id == *id) })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
