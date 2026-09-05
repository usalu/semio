//! ↩️ Inverse for `CreateLayer`.

use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::{SemioDrawingMutation, delete_layer};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawLayer, SemioDrawingSnapshot};

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::CreateLayer, _base: &SemioDrawingSnapshot) -> Vec<SemioDrawingMutation> {
    vec![SemioDrawingMutation::DeleteLayer(delete_layer::DeleteLayer { id: payload.layer.id.clone() })]
}
//#endregion 🔖️Inverse
