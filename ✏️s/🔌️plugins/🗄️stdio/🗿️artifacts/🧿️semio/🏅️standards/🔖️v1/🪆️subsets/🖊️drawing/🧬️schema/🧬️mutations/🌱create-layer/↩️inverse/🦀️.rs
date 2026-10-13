//! ↩️ Inverse for `CreateLayer`.

use crate::schema::mutations::{delete_layer, SemioDrawingMutation};
use crate::schema::snapshot::SemioDrawingSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::CreateLayer, _base: &SemioDrawingSnapshot) -> Result<Vec<SemioDrawingMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![SemioDrawingMutation::DeleteLayer(delete_layer::DeleteLayer { id: payload.layer.id.clone() })]

    })())
}
//#endregion 🔖️Inverse
