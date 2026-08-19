//! 🔺️ Sparse diff builder for `DeleteLayer`.
use crate::artifacts::draw::diff::{diff_remove_layer, DrawDiff};
use crate::artifacts::draw::schema::find_draw_layer;
use crate::artifacts::draw::DrawSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::DeleteLayer, base: &DrawSnapshot) -> protocol::MutationOutcome<DrawDiff> {
    if find_draw_layer(base, &payload.layer_id).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Layer \"{}\" does not exist.", payload.layer_id), [payload.layer_id.clone()]);
    }
    protocol::MutationOutcome::new(diff_remove_layer(&payload.layer_id))
}
//#endregion 🔖️Diff
