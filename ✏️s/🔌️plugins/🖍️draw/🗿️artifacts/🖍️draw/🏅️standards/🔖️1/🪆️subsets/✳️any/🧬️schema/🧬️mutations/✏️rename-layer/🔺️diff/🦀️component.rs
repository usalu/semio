//! 🔺️ Sparse diff builder for `RenameLayer`.
use crate::artifacts::draw::diff::{diff_set_layer_name, DrawDiff};
use crate::artifacts::draw::DrawSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::RenameLayer, _base: &DrawSnapshot) -> DrawDiff {
    diff_set_layer_name(&payload.layer_id, &payload.new_name)
}
//#endregion 🔖️Diff
