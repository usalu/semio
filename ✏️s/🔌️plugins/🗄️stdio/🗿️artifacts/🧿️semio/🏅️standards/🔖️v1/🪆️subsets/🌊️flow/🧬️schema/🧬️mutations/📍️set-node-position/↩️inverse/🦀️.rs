//! ↩️ Inverse for `SetNodePosition`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioFlowSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::support::node_at;
#[allow(unused_imports)]
use crate::schema::mutations::set_node_position;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::SetNodePosition, base: &SemioFlowSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::set_node_position::SetNodePosition>,
{
    let super::SetNodePosition { id, .. } = payload;
    Ok(match node_at(base, id) {
        Some(node) => vec![Op::from(set_node_position::SetNodePosition { id: id.clone(), position: node.position })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
