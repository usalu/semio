//! ↩️ Inverse for `MoveNode`.

use crate::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::MoveNode, base: &SemioGraphSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::move_node::MoveNode>,
{
    Ok((|| {
    match base.nodes.iter().find(|n| n.id == payload.id) {
        Some(node) => vec![Op::from(super::MoveNode { id: payload.id.clone(), new_position: node.position })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
