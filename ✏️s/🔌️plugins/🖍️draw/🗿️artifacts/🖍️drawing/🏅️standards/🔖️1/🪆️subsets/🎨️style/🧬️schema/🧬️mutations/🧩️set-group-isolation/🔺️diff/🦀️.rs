//! 🔺️ Sparse isolation change with group-only target validation.
use crate::{DrawingSnapshot,DrawingLayerNode,diff::{DrawingDiff,diff_set_group_isolation},schema::find_drawing_layer};
pub fn diff(payload:&super::mutation::SetGroupIsolation,base:&DrawingSnapshot)->protocol::MutationOutcome<DrawingDiff> {
    let group=match find_drawing_layer(base,&payload.layer_id) {
        Some(DrawingLayerNode::Group(group))=>group,
        Some(_)=>return protocol::MutationOutcome::error("mutation.target-mismatch","Isolation applies to groups only",[payload.layer_id.clone()]),
        None=>return protocol::MutationOutcome::error("mutation.target-missing","The target group does not exist",[payload.layer_id.clone()]),
    };
    if group.isolation==payload.isolation {return protocol::MutationOutcome::empty().warn("mutation.no-op","Group already uses this isolation setting");}
    protocol::MutationOutcome::new(diff_set_group_isolation(&payload.layer_id,payload.isolation))
}
