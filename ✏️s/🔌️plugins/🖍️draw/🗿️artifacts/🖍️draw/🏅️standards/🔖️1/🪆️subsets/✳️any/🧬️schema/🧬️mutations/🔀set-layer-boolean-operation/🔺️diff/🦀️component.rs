//! 🔺️ Sparse diff builder for `SetLayerBooleanOperation`.
use crate::artifacts::draw::diff::{diff_set_boolean_operation, DrawDiff};
use crate::artifacts::draw::DrawSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::SetLayerBooleanOperation, _base: &DrawSnapshot) -> DrawDiff {
    diff_set_boolean_operation(&payload.layer_id, &payload.boolean_operation)
}
//#endregion 🔖️Diff
