//! ↩️ Inverse for `UpdateLayerTraceParams` — the OLD params captured from BASE. Missing target or a
//! non-trace layer ⇒ `Vec::new()`.
use crate::artifacts::draw::schema::find_draw_layer;
use crate::artifacts::draw::mutations::DrawMutation;
use crate::artifacts::draw::{DrawLayerNode, DrawSnapshot};

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::UpdateLayerTraceParams, base: &DrawSnapshot) -> Vec<DrawMutation> {
    match find_draw_layer(base, &payload.layer_id) {
        Some(DrawLayerNode::Trace(trace)) => vec![super::mutation::update_layer_trace_params(payload.layer_id.clone(), trace.params.clone())],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
