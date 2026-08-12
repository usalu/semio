//! 🔺️ Sparse diff builder for `ReplaceLayerStroke`.
use crate::artifacts::draw::diff::{diff_set_stroke, DrawDiff};
use crate::artifacts::draw::DrawSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ReplaceLayerStroke, _base: &DrawSnapshot) -> DrawDiff {
    diff_set_stroke(&payload.layer_id, &payload.stroke)
}
//#endregion 🔖️Diff
