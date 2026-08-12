//! 🔺️ `rotate` — delegates to the shared `diff::diff_rotate_node` helper.

use super::mutation::Rotate;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::{diff_rotate_node, SemioDrawingDiff};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &Rotate, base: &SemioDrawingSnapshot) -> SemioDrawingDiff {
    diff_rotate_node(base, &payload.at, payload.new_rotation)
}
//#endregion 🔖️Diff
