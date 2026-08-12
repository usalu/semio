//! 🔺️ Sparse diff builder for `ReplaceLayerFill`.
use crate::artifacts::draw::diff::{diff_set_fill, DrawDiff};
use crate::artifacts::draw::DrawSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ReplaceLayerFill, _base: &DrawSnapshot) -> DrawDiff {
    diff_set_fill(&payload.layer_id, &payload.fill)
}
//#endregion 🔖️Diff
