//! ↩️ Inverse for `DeleteLayer`.

use crate::schema::mutations::create_layer;
use crate::schema::snapshot::SemioDrawingSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::DeleteLayer, base: &SemioDrawingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::create_layer::CreateLayer>,
{
    Ok((|| {
    match base.layers.iter().position(|l| l.id == payload.id) {
        Some(index) => vec![Op::from(create_layer::CreateLayer { index, layer: base.layers[index].clone() })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
