//! ↩️ Inverse for `ScaleNode`.

use crate::standards::v1::subsets::drawing::schema::diff::node_at;
use crate::standards::v1::subsets::drawing::schema::mutations::SemioDrawingMutation;
use crate::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, SemioDrawingSnapshot};

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::ScaleNode, base: &SemioDrawingSnapshot) -> Vec<SemioDrawingMutation> {
    match node_at(base, &payload.at) {
        Some(DrawNode::Group { transform, .. }) => vec![SemioDrawingMutation::ScaleNode(super::ScaleNode { at: payload.at.clone(), new_scale: transform.scale })],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
