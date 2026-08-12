//! 🔺️ Sparse diff builder for `SetLayerOpacity`.
use crate::artifacts::draw::diff::{diff_set_layer_opacity, DrawDiff};
use crate::artifacts::draw::DrawSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::SetLayerOpacity, _base: &DrawSnapshot) -> DrawDiff {
    diff_set_layer_opacity(&payload.layer_id, payload.opacity)
}
//#endregion 🔖️Diff
