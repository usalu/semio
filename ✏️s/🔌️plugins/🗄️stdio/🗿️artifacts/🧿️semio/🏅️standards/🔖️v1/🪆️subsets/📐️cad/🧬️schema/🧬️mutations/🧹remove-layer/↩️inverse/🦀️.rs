//! ↩️ Inverse for `RemoveLayer`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioCadSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::add_layer;
#[allow(unused_imports)]
use crate::schema::mutations::support::find_layer;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::RemoveLayer, base: &SemioCadSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::add_layer::AddLayer>,
{
    let super::RemoveLayer { name } = payload;
    Ok(match find_layer(base, name) {
        Some(l) => vec![Op::from(add_layer::AddLayer { layer: l.clone(), at: base.layers.iter().position(|layer| layer.name == *name) })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
