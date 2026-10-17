//! ↩️ Inverse for `ReorderNodes`.

use crate::schema::diff::node_at;
use crate::schema::snapshot::{DrawNode, SemioDrawingSnapshot};

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::ReorderNodes, base: &SemioDrawingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::reorder_nodes::ReorderNodes>,
{
    Ok((|| {
    match node_at(base, &payload.parent) {
        Some(DrawNode::Group { children, .. }) if !children.is_empty() && payload.from < children.len() => {
            let landed_at = payload.to.min(children.len() - 1);
            vec![Op::from(super::ReorderNodes { parent: payload.parent.clone(), from: landed_at, to: payload.from })]
        }
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
