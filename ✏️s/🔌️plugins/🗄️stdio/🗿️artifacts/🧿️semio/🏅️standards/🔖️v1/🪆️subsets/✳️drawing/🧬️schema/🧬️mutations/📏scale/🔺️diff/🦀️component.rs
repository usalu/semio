//! 🔺️ `scale` — delegates to the shared `diff::diff_scale_node` helper.

use super::mutation::Scale;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::{diff_scale_node, SemioDrawingDiff};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &Scale, base: &SemioDrawingSnapshot) -> SemioDrawingDiff {
    diff_scale_node(base, &payload.at, payload.new_scale)
}
//#endregion 🔖️Diff
