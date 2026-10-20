//! 🔺️ Sparse diff builder for `UpdateLayerTraceParams`.
use semio_s_artifact_draw_drawing_diff::{diff_set_trace_params, DrawingDiff};
use semio_s_artifact_draw_drawing_model::find_drawing_layer;
use semio_s_artifact_draw_drawing_model::DrawingLayerNode;
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::UpdateLayerTraceParams, base: &DrawingSnapshot) -> protocol::MutationOutcome<DrawingDiff> {
    match find_drawing_layer(base, &payload.layer_id) {
        None => protocol::MutationOutcome::error("mutation.target-missing", format!("Layer \"{}\" does not exist.", payload.layer_id), [payload.layer_id.to_string_owner()]),
        Some(DrawingLayerNode::Trace(trace)) if trace.params == payload.params => protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Layer \"{}\" trace params are unchanged.", payload.layer_id)),
        Some(_) => protocol::MutationOutcome::new(diff_set_trace_params(&payload.layer_id, &payload.params)),
    }
}
//#endregion 🔖️Diff
