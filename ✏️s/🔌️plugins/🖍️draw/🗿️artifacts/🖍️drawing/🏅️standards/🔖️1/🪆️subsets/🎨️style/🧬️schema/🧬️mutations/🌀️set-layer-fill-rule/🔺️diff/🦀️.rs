//! 🔺️ Sparse fill-rule change with missing-target and no-op outcomes.
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
use semio_s_artifact_draw_drawing_diff::{DrawingDiff, diff_set_layer_fill_rule};
use semio_s_artifact_draw_drawing_model::{find_drawing_layer, layer_base};
pub fn diff(payload:&super::mutation::SetLayerFillRule,base:&DrawingSnapshot)->protocol::MutationOutcome<DrawingDiff> {
    let Some(layer)=find_drawing_layer(base,&payload.layer_id) else {return protocol::MutationOutcome::error("mutation.target-missing","Layer does not exist",[payload.layer_id.to_string_owner()]);};
    if layer_base(layer).attributes.fill_rule==payload.fill_rule {return protocol::MutationOutcome::empty().warning("mutation.no-op","Layer already uses this fill rule");}
    protocol::MutationOutcome::new(diff_set_layer_fill_rule(&payload.layer_id,payload.fill_rule))
}
