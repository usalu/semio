//! 🔺️ Sparse diff builder for `UpdateLayerTraceParams`.
use crate::artifacts::draw::diff::{diff_set_trace_params, DrawDiff};
use crate::artifacts::draw::schema::find_draw_layer;
use crate::artifacts::draw::{DrawLayerNode, DrawSnapshot};

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::UpdateLayerTraceParams, base: &DrawSnapshot) -> protocol::MutationOutcome<DrawDiff> {
    match find_draw_layer(base, &payload.layer_id) {
        None => protocol::MutationOutcome::error("mutation.target-missing", format!("Layer \"{}\" does not exist.", payload.layer_id), [payload.layer_id.clone()]),
        Some(DrawLayerNode::Trace(trace)) if trace.params == payload.params => protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Layer \"{}\" trace params are unchanged.", payload.layer_id)),
        Some(_) => protocol::MutationOutcome::new(diff_set_trace_params(&payload.layer_id, &payload.params)),
    }
}
//#endregion 🔖️Diff
