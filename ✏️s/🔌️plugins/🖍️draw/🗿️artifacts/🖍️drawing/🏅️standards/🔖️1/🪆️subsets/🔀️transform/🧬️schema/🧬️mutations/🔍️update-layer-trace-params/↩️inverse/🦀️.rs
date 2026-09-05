//! ↩️ Inverse for `UpdateLayerTraceParams` — the OLD params captured from BASE. Missing target or a
//! non-trace layer ⇒ `Vec::new()`.
use crate::artifacts::drawing::mutations::DrawingMutation;
use crate::artifacts::drawing::schema::find_drawing_layer;
use crate::artifacts::drawing::{DrawingLayerNode, DrawingSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::UpdateLayerTraceParams, base: &DrawingSnapshot) -> Vec<DrawingMutation> {
    match find_drawing_layer(base, &payload.layer_id) {
        Some(DrawingLayerNode::Trace(trace)) => vec![super::mutation::update_layer_trace_params(payload.layer_id.clone(), trace.params.clone())],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
