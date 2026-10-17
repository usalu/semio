//! ↩️ Inverse for `DeleteNode`.

use crate::schema::diff::node_at;
use crate::schema::mutations::create_node;
use crate::schema::snapshot::{DrawNode, SemioDrawingSnapshot};

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::DeleteNode, base: &SemioDrawingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::create_node::CreateNode>,
{
    Ok((|| {
    let Some((parent, index)) = super::parent_and_index(&payload.at) else { return Vec::new() };
    match node_at(base, &parent) {
        Some(DrawNode::Group { children, .. }) => match children.get(index) {
            Some(node) => vec![Op::from(create_node::CreateNode { parent, index, node: node.clone() })],
            None => Vec::new(),
        },
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
