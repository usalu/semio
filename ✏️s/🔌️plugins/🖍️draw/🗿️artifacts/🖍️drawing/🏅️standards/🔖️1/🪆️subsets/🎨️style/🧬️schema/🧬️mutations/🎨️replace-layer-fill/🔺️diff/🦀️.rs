//! 🔺️ Sparse diff builder for `ReplaceLayerFill`.
use crate::artifacts::drawing::diff::{diff_set_fill, DrawingDiff};
use crate::artifacts::drawing::schema::{find_drawing_layer, layer_base};
use crate::artifacts::drawing::DrawingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ReplaceLayerFill, base: &DrawingSnapshot) -> protocol::MutationOutcome<DrawingDiff> {
    let Some(layer) = find_drawing_layer(base, &payload.layer_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Layer \"{}\" does not exist.", payload.layer_id), [payload.layer_id.clone()]);
    };
    if layer_base(layer).attributes.fill == payload.fill {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Layer \"{}\" fill is unchanged.", payload.layer_id));
    }
    protocol::MutationOutcome::new(diff_set_fill(&payload.layer_id, &payload.fill))
}
//#endregion 🔖️Diff
