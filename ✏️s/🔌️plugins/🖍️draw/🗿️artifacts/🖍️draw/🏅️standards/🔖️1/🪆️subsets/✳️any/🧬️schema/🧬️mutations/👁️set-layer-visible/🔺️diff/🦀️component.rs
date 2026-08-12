//! 🔺️ Sparse diff builder for `SetLayerVisible`.
use crate::artifacts::draw::diff::{diff_set_layer_visible, DrawDiff};
use crate::artifacts::draw::DrawSnapshot;

//#region 🔖️Diff
/// 🔺️ One `visible` field patch — a real sparse `DrawDiff`, never apply-then-capture.
pub fn diff(payload: &super::mutation::SetLayerVisible, _base: &DrawSnapshot) -> DrawDiff {
    diff_set_layer_visible(&payload.layer_id, payload.visible)
}
//#endregion 🔖️Diff
