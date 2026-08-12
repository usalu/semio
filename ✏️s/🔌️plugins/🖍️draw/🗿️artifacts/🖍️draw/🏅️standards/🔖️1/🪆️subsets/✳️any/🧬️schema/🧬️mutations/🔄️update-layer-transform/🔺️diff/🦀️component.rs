//! 🔺️ Sparse diff builder for `UpdateLayerTransform`.
use crate::artifacts::draw::diff::{diff_set_layer_transform, DrawDiff};
use crate::artifacts::draw::DrawSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::UpdateLayerTransform, _base: &DrawSnapshot) -> DrawDiff {
    diff_set_layer_transform(&payload.layer_id, &payload.transform)
}
//#endregion 🔖️Diff
