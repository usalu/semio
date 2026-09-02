//! 🔺️ Sparse diff builder for `UpdateLayerTraceParams`.
use crate::artifacts::drawing::diff::{diff_set_trace_params, DrawingDiff};
use crate::artifacts::drawing::schema::find_drawing_layer;
use crate::artifacts::drawing::{DrawingLayerNode, DrawingSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::UpdateLayerTraceParams, base: &DrawingSnapshot) -> protocol::MutationOutcome<DrawingDiff> {
    match find_drawing_layer(base, &payload.layer_id) {
        None => protocol::MutationOutcome::error("mutation.target-missing", format!("Layer \"{}\" does not exist.", payload.layer_id), [payload.layer_id.clone()]),
        Some(DrawingLayerNode::Trace(trace)) if trace.params == payload.params => protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Layer \"{}\" trace params are unchanged.", payload.layer_id)),
        Some(_) => protocol::MutationOutcome::new(diff_set_trace_params(&payload.layer_id, &payload.params)),
    }
}
//#endregion 🔖️Diff
