//! ↩️ Inverse for `DeleteLayer`.

use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::{SemioDrawingMutation, create_layer};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::DeleteLayer, base: &SemioDrawingSnapshot) -> Vec<SemioDrawingMutation> {
    match base.layers.iter().position(|l| l.id == payload.id) {
        Some(index) => vec![SemioDrawingMutation::CreateLayer(create_layer::CreateLayer { index, layer: base.layers[index].clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
