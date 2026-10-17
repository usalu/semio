//! ↩️ Inverse for `InsertNode`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioFlowSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::remove_node;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::InsertNode, _base: &SemioFlowSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::remove_node::RemoveNode>,
{
    let super::InsertNode { node, .. } = payload;
    Ok(vec![Op::from(remove_node::RemoveNode { id: node.id.clone() })])
}
//#endregion 🔖️Inverse
