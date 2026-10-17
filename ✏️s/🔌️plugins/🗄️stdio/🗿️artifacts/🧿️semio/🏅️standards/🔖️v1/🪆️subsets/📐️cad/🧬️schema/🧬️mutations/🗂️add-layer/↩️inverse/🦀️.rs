//! ↩️ Inverse for `AddLayer`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioCadSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::remove_layer;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::AddLayer, _base: &SemioCadSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::remove_layer::RemoveLayer>,
{
    let super::AddLayer { layer, .. } = payload;
    Ok(vec![Op::from(remove_layer::RemoveLayer { name: layer.name.clone() })])
}
//#endregion 🔖️Inverse
