//! ↩️ Inverse for `UnflattenNode`: the node the restore overwrites is captured whole, so undoing it puts exactly that
//! node back — for every payload the grammar admits, not only for one whose current node is `flatten(original)`.

use crate::schema::diff::node_at;
use crate::schema::mutations::unflatten_node;
use crate::schema::snapshot::SemioDrawingSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::UnflattenNode, base: &SemioDrawingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::unflatten_node::UnflattenNode>,
{
    Ok((|| {
    match node_at(base, &payload.at) {
        Some(node) if *node != payload.original => vec![Op::from(unflatten_node::UnflattenNode { at: payload.at.clone(), original: node.clone() })],
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
