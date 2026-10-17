//! ↩️ Inverse for `RotateNode`.

use crate::schema::diff::node_at;
use crate::schema::snapshot::{DrawNode, SemioDrawingSnapshot};

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::RotateNode, base: &SemioDrawingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::rotate_node::RotateNode>,
{
    Ok((|| {
    match node_at(base, &payload.at) {
        Some(DrawNode::Group { transform, .. }) => vec![Op::from(super::RotateNode { at: payload.at.clone(), new_rotation: transform.rotation })],
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
