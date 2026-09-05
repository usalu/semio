//! 🔺️ Sparse diff builder for `SetLayerBooleanOperation`.
use crate::artifacts::drawing::diff::{diff_set_boolean_operation, DrawingDiff};
use crate::artifacts::drawing::schema::find_drawing_layer;
use crate::artifacts::drawing::{DrawingLayerNode, DrawingSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::SetLayerBooleanOperation, base: &DrawingSnapshot) -> protocol::MutationOutcome<DrawingDiff> {
    match find_drawing_layer(base, &payload.layer_id) {
        None => protocol::MutationOutcome::error("mutation.target-missing", format!("Layer \"{}\" does not exist.", payload.layer_id), [payload.layer_id.clone()]),
        Some(DrawingLayerNode::Boolean(boolean)) if boolean.operation == payload.boolean_operation => {
            protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Layer \"{}\" boolean operation is already \"{}\".", payload.layer_id, payload.boolean_operation))
        }
        Some(_) => protocol::MutationOutcome::new(diff_set_boolean_operation(&payload.layer_id, &payload.boolean_operation)),
    }
}
//#endregion 🔖️Diff
