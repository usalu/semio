//! ↩️ Inverse for `ScaleNode`.

use crate::schema::diff::node_at;
use crate::schema::snapshot::{DrawNode, SemioDrawingSnapshot};

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::ScaleNode, base: &SemioDrawingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::scale_node::ScaleNode>,
{
    Ok((|| {
    match node_at(base, &payload.at) {
        Some(DrawNode::Group { transform, .. }) => vec![Op::from(super::ScaleNode { at: payload.at.clone(), new_scale: transform.scale })],
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
