//! ↩️ Inverse for `CreateLayer`.

use crate::standards::v1::subsets::drawing::schema::mutations::{delete_layer, SemioDrawingMutation};
use crate::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::CreateLayer, _base: &SemioDrawingSnapshot) -> Vec<SemioDrawingMutation> {
    vec![SemioDrawingMutation::DeleteLayer(delete_layer::DeleteLayer { id: payload.layer.id.clone() })]
}
//#endregion 🔖️Inverse
