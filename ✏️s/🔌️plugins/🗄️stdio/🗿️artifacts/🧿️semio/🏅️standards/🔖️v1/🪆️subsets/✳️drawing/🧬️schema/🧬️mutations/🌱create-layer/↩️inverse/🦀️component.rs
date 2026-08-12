//! ↩️ `create-layer` — undo is `delete-layer` addressed by the created layer's own id.

use super::mutation::CreateLayer;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::{delete_layer, SemioDrawingMutation};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &CreateLayer, _base: &SemioDrawingSnapshot) -> Vec<SemioDrawingMutation> {
    vec![SemioDrawingMutation::DeleteLayer(delete_layer::mutation::DeleteLayer { id: payload.layer.id.clone() })]
}
//#endregion 🔖️Inverse
