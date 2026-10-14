//! ↩️ Restore the authored group isolation setting.
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
use semio_s_artifact_draw_drawing_model::{DrawingLayerNode, find_drawing_layer};
pub fn inverse<Op: From<crate::set_group_isolation::mutation::SetGroupIsolation>>(payload:&super::mutation::SetGroupIsolation,base:&DrawingSnapshot)-> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok(match find_drawing_layer(base,&payload.layer_id) {Some(DrawingLayerNode::Group(group))=>vec![super::mutation::set_group_isolation(payload.layer_id.clone().into(),group.isolation)],_=>Vec::new()})
}
