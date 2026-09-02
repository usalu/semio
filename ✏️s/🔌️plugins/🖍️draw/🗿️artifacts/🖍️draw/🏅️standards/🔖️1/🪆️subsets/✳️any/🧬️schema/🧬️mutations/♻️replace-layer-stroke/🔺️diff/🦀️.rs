//! 🔺️ Sparse diff builder for `ReplaceLayerStroke`.
use crate::artifacts::draw::diff::{diff_set_stroke, DrawDiff};
use crate::artifacts::draw::schema::{find_draw_layer, layer_base};
use crate::artifacts::draw::DrawSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ReplaceLayerStroke, base: &DrawSnapshot) -> protocol::MutationOutcome<DrawDiff> {
    let Some(layer) = find_draw_layer(base, &payload.layer_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Layer \"{}\" does not exist.", payload.layer_id), [payload.layer_id.clone()]);
    };
    if layer_base(layer).attributes.stroke == payload.stroke {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Layer \"{}\" stroke is unchanged.", payload.layer_id));
    }
    protocol::MutationOutcome::new(diff_set_stroke(&payload.layer_id, &payload.stroke))
}
//#endregion 🔖️Diff
