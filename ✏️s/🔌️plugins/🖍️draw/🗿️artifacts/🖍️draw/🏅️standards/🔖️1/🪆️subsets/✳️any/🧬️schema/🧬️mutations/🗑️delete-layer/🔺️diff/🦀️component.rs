//! 🔺️ Sparse diff builder for `DeleteLayer`.
use crate::artifacts::draw::diff::{diff_remove_layer, DrawDiff};
use crate::artifacts::draw::DrawSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DeleteLayer, _base: &DrawSnapshot) -> DrawDiff {
    diff_remove_layer(&payload.layer_id)
}
//#endregion 🔖️Diff
