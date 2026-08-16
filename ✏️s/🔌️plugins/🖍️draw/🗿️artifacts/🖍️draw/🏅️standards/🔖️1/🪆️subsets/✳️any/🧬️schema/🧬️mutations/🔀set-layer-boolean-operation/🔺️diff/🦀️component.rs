//! 🔺️ Sparse diff builder for `SetLayerBooleanOperation`.
use crate::artifacts::draw::diff::{diff_set_boolean_operation, DrawDiff};
use crate::artifacts::draw::schema::find_draw_layer;
use crate::artifacts::draw::{DrawLayerNode, DrawSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::SetLayerBooleanOperation, base: &DrawSnapshot) -> protocol::MutationOutcome<DrawDiff> {
    match find_draw_layer(base, &payload.layer_id) {
        None => protocol::MutationOutcome::error("mutation.target-missing", format!("Layer \"{}\" does not exist.", payload.layer_id), [payload.layer_id.clone()]),
        Some(DrawLayerNode::Boolean(boolean)) if boolean.operation == payload.boolean_operation => {
            protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Layer \"{}\" boolean operation is already \"{}\".", payload.layer_id, payload.boolean_operation))
        }
        Some(_) => protocol::MutationOutcome::new(diff_set_boolean_operation(&payload.layer_id, &payload.boolean_operation)),
    }
}
//#endregion 🔖️Diff
