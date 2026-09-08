//! ↩️ Inverse for `ReplacePath`.

use crate::standards::v1::subsets::drawing::schema::diff::node_at;
use crate::standards::v1::subsets::drawing::schema::mutations::SemioDrawingMutation;
use crate::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, SemioDrawingSnapshot};

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::ReplacePath, base: &SemioDrawingSnapshot) -> Vec<SemioDrawingMutation> {
    match node_at(base, &payload.at) {
        Some(DrawNode::Path { segments, .. }) => vec![SemioDrawingMutation::ReplacePath(super::ReplacePath { at: payload.at.clone(), new_segments: segments.clone() })],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
