//! ↩️ Restore the fill rule from the mutation's base snapshot.
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
use semio_s_artifact_draw_drawing_model::{find_drawing_layer, layer_base};
pub fn inverse<Op: From<crate::set_layer_fill_rule::mutation::SetLayerFillRule>>(payload:&super::mutation::SetLayerFillRule,base:&DrawingSnapshot)-> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok(find_drawing_layer(base,&payload.layer_id).map(|layer|vec![super::mutation::set_layer_fill_rule(payload.layer_id.clone().into(),layer_base(layer).attributes.fill_rule)]).unwrap_or_default())
}
