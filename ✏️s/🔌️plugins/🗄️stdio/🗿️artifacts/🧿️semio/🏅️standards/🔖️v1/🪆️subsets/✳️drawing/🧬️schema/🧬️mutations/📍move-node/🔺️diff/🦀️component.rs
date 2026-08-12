//! 🔺️ `move-node` — delegates to the shared `diff::diff_move_node` helper (reused by `drag-nodes`).

use super::mutation::MoveNode;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::{diff_move_node, SemioDrawingDiff};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &MoveNode, base: &SemioDrawingSnapshot) -> SemioDrawingDiff {
    diff_move_node(base, &payload.at, payload.new_origin)
}
//#endregion 🔖️Diff
