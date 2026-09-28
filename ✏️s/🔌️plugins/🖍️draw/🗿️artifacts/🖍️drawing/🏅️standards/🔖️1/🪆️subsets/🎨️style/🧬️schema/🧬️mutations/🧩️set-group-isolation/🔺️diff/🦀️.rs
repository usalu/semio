//! 🔺️ Sparse isolation change with group-only target validation.
use crate::{DrawingSnapshot,DrawingLayerNode,diff::{DrawingDiff,diff_set_group_isolation},schema::find_drawing_layer};
pub fn diff(payload:&super::mutation::SetGroupIsolation,base:&DrawingSnapshot)->protocol::MutationOutcome<DrawingDiff> {
    let Some(DrawingLayerNode::Group(group))=find_drawing_layer(base,&payload.layer_id) else {return protocol::MutationOutcome::error("mutation.target-invalid","Isolation needs an existing group",[payload.layer_id.clone()]);};
    if group.isolation==payload.isolation {return protocol::MutationOutcome::empty().warn("mutation.no-op","Group already uses this isolation setting");}
    protocol::MutationOutcome::new(diff_set_group_isolation(&payload.layer_id,payload.isolation))
}
