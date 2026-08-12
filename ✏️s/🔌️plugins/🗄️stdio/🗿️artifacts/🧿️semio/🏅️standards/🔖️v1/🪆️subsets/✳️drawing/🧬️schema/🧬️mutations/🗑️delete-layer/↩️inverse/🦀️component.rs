//! ↩️ `delete-layer` — undo re-creates the captured layer at its BASE-state index; absent id ⇒
//! `Vec::new()`.

use super::mutation::DeleteLayer;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::{create_layer, SemioDrawingMutation};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteLayer, base: &SemioDrawingSnapshot) -> Vec<SemioDrawingMutation> {
    match base.layers.iter().position(|l| l.id == payload.id) {
        Some(index) => vec![SemioDrawingMutation::CreateLayer(create_layer::mutation::CreateLayer { index, layer: base.layers[index].clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
