//! ↩️ Inverse for `RotateNode`.

use crate::schema::diff::node_at;
use crate::schema::mutations::SemioDrawingMutation;
use crate::schema::snapshot::{DrawNode, SemioDrawingSnapshot};

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::RotateNode, base: &SemioDrawingSnapshot) -> Result<Vec<SemioDrawingMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match node_at(base, &payload.at) {
        Some(DrawNode::Group { transform, .. }) => vec![SemioDrawingMutation::RotateNode(super::RotateNode { at: payload.at.clone(), new_rotation: transform.rotation })],
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
