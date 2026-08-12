//! 🔺️ Sparse diff builder for `SetLayerLocked`.
use crate::artifacts::draw::diff::{diff_set_layer_locked, DrawDiff};
use crate::artifacts::draw::DrawSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::SetLayerLocked, _base: &DrawSnapshot) -> DrawDiff {
    diff_set_layer_locked(&payload.layer_id, payload.locked)
}
//#endregion 🔖️Diff
