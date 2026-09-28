//! ↩️ Restore the fill rule from the mutation's base snapshot.
use crate::{DrawingSnapshot,mutations::DrawingMutation,schema::{find_drawing_layer,layer_base}};
pub fn inverse(payload:&super::mutation::SetLayerFillRule,base:&DrawingSnapshot)->Vec<DrawingMutation> {
    find_drawing_layer(base,&payload.layer_id).map(|layer|vec![super::mutation::set_layer_fill_rule(payload.layer_id.clone(),layer_base(layer).attributes.fill_rule)]).unwrap_or_default()
}
