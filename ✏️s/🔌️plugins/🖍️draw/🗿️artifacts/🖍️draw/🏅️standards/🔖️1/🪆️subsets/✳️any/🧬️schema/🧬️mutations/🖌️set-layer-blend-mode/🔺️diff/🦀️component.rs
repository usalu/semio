//! 🔺️ Sparse diff builder for `SetLayerBlendMode`.
use crate::artifacts::draw::diff::{diff_set_layer_blend_mode, DrawDiff};
use crate::artifacts::draw::DrawSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::SetLayerBlendMode, _base: &DrawSnapshot) -> DrawDiff {
    diff_set_layer_blend_mode(&payload.layer_id, &payload.blend_mode)
}
//#endregion 🔖️Diff
