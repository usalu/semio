//! 🔺️ Sparse diff builder for `ReplaceLayerFill`.
use semio_s_artifact_draw_drawing_diff::{diff_set_fill, DrawingDiff};
use semio_s_artifact_draw_drawing_model::{find_drawing_layer, layer_base};
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ReplaceLayerFill, base: &DrawingSnapshot) -> protocol::MutationOutcome<DrawingDiff> {
    let Some(layer) = find_drawing_layer(base, &payload.layer_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Layer \"{}\" does not exist.", payload.layer_id), [payload.layer_id.to_string_owner()]);
    };
    if layer_base(layer).attributes.fill == payload.fill {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Layer \"{}\" fill is unchanged.", payload.layer_id));
    }
    protocol::MutationOutcome::new(diff_set_fill(&payload.layer_id, &payload.fill))
}
//#endregion 🔖️Diff
