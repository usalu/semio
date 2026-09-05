//! 🔺️ Sparse diff builder for `RenameLayer`.
use crate::artifacts::drawing::diff::{diff_set_layer_name, DrawingDiff};
use crate::artifacts::drawing::schema::{find_drawing_layer, layer_base};
use crate::artifacts::drawing::DrawingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::RenameLayer, base: &DrawingSnapshot) -> protocol::MutationOutcome<DrawingDiff> {
    let Some(layer) = find_drawing_layer(base, &payload.layer_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Layer \"{}\" does not exist.", payload.layer_id), [payload.layer_id.clone()]);
    };
    if layer_base(layer).name == payload.new_name {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Layer \"{}\" is already named \"{}\".", payload.layer_id, payload.new_name));
    }
    protocol::MutationOutcome::new(diff_set_layer_name(&payload.layer_id, &payload.new_name))
}
//#endregion 🔖️Diff
