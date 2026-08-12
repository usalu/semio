//! 🔺️ Sparse diff builder for `UpdateLayerTraceParams`.
use crate::artifacts::draw::diff::{diff_set_trace_params, DrawDiff};
use crate::artifacts::draw::DrawSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::UpdateLayerTraceParams, _base: &DrawSnapshot) -> DrawDiff {
    diff_set_trace_params(&payload.layer_id, &payload.params)
}
//#endregion 🔖️Diff
