//! 🔺️ Sparse diff builder for `DeleteLayer`.
use semio_s_artifact_draw_drawing_diff::{diff_remove_layer, DrawingDiff};
use semio_s_artifact_draw_drawing_model::find_drawing_layer;
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DeleteLayer, base: &DrawingSnapshot) -> protocol::MutationOutcome<DrawingDiff> {
    if find_drawing_layer(base, &payload.layer_id).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Layer \"{}\" does not exist.", payload.layer_id), [payload.layer_id.to_string_owner()]);
    }
    protocol::MutationOutcome::new(diff_remove_layer(&base.layers, &payload.layer_id))
}
//#endregion 🔖️Diff
