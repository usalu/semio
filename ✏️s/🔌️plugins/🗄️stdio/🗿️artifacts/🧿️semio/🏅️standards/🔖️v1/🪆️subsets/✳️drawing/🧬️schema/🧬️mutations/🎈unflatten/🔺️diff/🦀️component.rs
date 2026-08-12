//! 🔺️ `unflatten` — delegates straight to `diff_at_path`/`DrawNodeDiff::Replace`; naturally a
//! no-op when `at` doesn't resolve (the wrapping `Group.children` triple diff skips a missing
//! index at every level of `diff_at_path`'s nesting).

use super::mutation::UnflattenNode;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::{diff_at_path, DrawNodeDiff, SemioDrawingDiff};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &UnflattenNode, _base: &SemioDrawingSnapshot) -> SemioDrawingDiff {
    diff_at_path(&payload.at, DrawNodeDiff::Replace { node: payload.original.clone() })
}
//#endregion 🔖️Diff
