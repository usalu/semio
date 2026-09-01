//! ↩️ Inverse for `MoveNode`.

use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::{NodePath, SemioDrawingDiff, diff_move_node, node_at, node_origin};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::SemioDrawingMutation;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::MoveNode, base: &SemioDrawingSnapshot) -> Vec<SemioDrawingMutation> {
    match node_origin(base, &payload.at) {
        Some(old_origin) => vec![SemioDrawingMutation::MoveNode(super::MoveNode { at: payload.at.clone(), new_origin: old_origin })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
