//! ↩️ Inverse for `DeleteLayer`.

use crate::schema::mutations::{create_layer, SemioDrawingMutation};
use crate::schema::snapshot::SemioDrawingSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::DeleteLayer, base: &SemioDrawingSnapshot) -> Result<Vec<SemioDrawingMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.layers.iter().position(|l| l.id == payload.id) {
        Some(index) => vec![SemioDrawingMutation::CreateLayer(create_layer::CreateLayer { index, layer: base.layers[index].clone() })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
