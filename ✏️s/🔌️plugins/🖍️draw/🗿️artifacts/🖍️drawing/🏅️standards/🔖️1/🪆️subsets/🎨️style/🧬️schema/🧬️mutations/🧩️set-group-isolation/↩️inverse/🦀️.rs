//! ↩️ Restore the authored group isolation setting.
use crate::{DrawingSnapshot,DrawingLayerNode,mutations::DrawingMutation,schema::find_drawing_layer};
pub fn inverse(payload:&super::mutation::SetGroupIsolation,base:&DrawingSnapshot)-> Result<Vec<DrawingMutation>, semio_framework_value::ValueError> {
    Ok(match find_drawing_layer(base,&payload.layer_id) {Some(DrawingLayerNode::Group(group))=>vec![super::mutation::set_group_isolation(payload.layer_id.clone().into(),group.isolation)],_=>Vec::new()})
}
