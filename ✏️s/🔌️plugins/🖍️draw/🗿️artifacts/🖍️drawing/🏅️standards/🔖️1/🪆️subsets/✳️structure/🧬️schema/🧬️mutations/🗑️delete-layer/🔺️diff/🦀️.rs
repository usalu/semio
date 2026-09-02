//! 🔺️ Sparse diff builder for `DeleteLayer`.
use crate::artifacts::drawing::diff::{diff_remove_layer, DrawingDiff};
use crate::artifacts::drawing::schema::find_drawing_layer;
use crate::artifacts::drawing::DrawingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DeleteLayer, base: &DrawingSnapshot) -> protocol::MutationOutcome<DrawingDiff> {
    if find_drawing_layer(base, &payload.layer_id).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Layer \"{}\" does not exist.", payload.layer_id), [payload.layer_id.clone()]);
    }
    protocol::MutationOutcome::new(diff_remove_layer(&payload.layer_id))
}
//#endregion 🔖️Diff
