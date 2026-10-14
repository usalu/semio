//! 🔺️ Sparse diff builder for `SetLayerBooleanOperation`.
use semio_s_artifact_draw_drawing_diff::{diff_set_boolean_operation, DrawingDiff};
use semio_s_artifact_draw_drawing_model::find_drawing_layer;
use semio_s_artifact_draw_drawing_model::DrawingLayerNode;
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::SetLayerBooleanOperation, base: &DrawingSnapshot) -> protocol::MutationOutcome<DrawingDiff> {
    match find_drawing_layer(base, &payload.layer_id) {
        None => protocol::MutationOutcome::error("mutation.target-missing", format!("Layer \"{}\" does not exist.", payload.layer_id), [payload.layer_id.to_string_owner()]),
        Some(DrawingLayerNode::Boolean(boolean)) if boolean.operation == payload.boolean_operation => {
            protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Layer \"{}\" boolean operation is already \"{}\".", payload.layer_id, payload.boolean_operation))
        }
        Some(_) => protocol::MutationOutcome::new(diff_set_boolean_operation(&payload.layer_id, &payload.boolean_operation)),
    }
}
//#endregion 🔖️Diff
