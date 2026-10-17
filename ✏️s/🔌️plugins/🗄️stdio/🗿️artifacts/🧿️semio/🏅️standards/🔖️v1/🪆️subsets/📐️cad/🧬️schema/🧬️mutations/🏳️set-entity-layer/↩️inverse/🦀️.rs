//! ↩️ Inverse for `SetEntityLayer`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioCadSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::support::find_entity;
#[allow(unused_imports)]
use crate::schema::mutations::set_entity_layer;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::SetEntityLayer, base: &SemioCadSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::set_entity_layer::SetEntityLayer>,
{
    let super::SetEntityLayer { handle, .. } = payload;
    Ok(match find_entity(base, handle) {
        Some(e) => vec![Op::from(set_entity_layer::SetEntityLayer { handle: handle.clone(), layer: e.layer.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
