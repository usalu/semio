//! ↩️ Inverse for `UpdateLayerTraceParams` — the OLD params captured from BASE. Missing target or a
//! non-trace layer ⇒ `Vec::new()`.

use semio_s_artifact_draw_drawing_model::find_drawing_layer;
use semio_s_artifact_draw_drawing_model::DrawingLayerNode;
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::update_layer_trace_params::mutation::UpdateLayerTraceParams>>(payload: &super::mutation::UpdateLayerTraceParams, base: &DrawingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok(match find_drawing_layer(base, &payload.layer_id) {
        Some(DrawingLayerNode::Trace(trace)) => vec![super::mutation::update_layer_trace_params(payload.layer_id.clone(), trace.params.clone())],
        _ => Vec::new(),
    })
}
//#endregion 🔖️Inverse
