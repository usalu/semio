//! ↩️ Inverse for `CreateLayer`.

use crate::schema::mutations::delete_layer;
use crate::schema::snapshot::SemioDrawingSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::CreateLayer, _base: &SemioDrawingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::delete_layer::DeleteLayer>,
{
    Ok((|| {
    vec![Op::from(delete_layer::DeleteLayer { id: payload.layer.id.clone() })]

    })())
}
//#endregion 🔖️Inverse
