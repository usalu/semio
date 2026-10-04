//! 🔺️ Sparse fill-rule change with missing-target and no-op outcomes.
use crate::{DrawingSnapshot,diff::{DrawingDiff,diff_set_layer_fill_rule},schema::{find_drawing_layer,layer_base}};
pub fn diff(payload:&super::mutation::SetLayerFillRule,base:&DrawingSnapshot)->protocol::MutationOutcome<DrawingDiff> {
    let Some(layer)=find_drawing_layer(base,&payload.layer_id) else {return protocol::MutationOutcome::error("mutation.target-missing","Layer does not exist",[payload.layer_id.clone()]);};
    if layer_base(layer).attributes.fill_rule==payload.fill_rule {return protocol::MutationOutcome::empty().warning("mutation.no-op","Layer already uses this fill rule");}
    protocol::MutationOutcome::new(diff_set_layer_fill_rule(&payload.layer_id,payload.fill_rule))
}
