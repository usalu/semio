//! ↩️ Inverse for `SetBlockEntityLayer`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioCadSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::support::find_block_entity;
#[allow(unused_imports)]
use crate::schema::mutations::set_block_entity_layer;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::SetBlockEntityLayer, base: &SemioCadSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::set_block_entity_layer::SetBlockEntityLayer>,
{
    let super::SetBlockEntityLayer { block_name, handle, .. } = payload;
    Ok(match find_block_entity(base, block_name, handle) {
        Some(e) => vec![Op::from(set_block_entity_layer::SetBlockEntityLayer { block_name: block_name.clone(), handle: handle.clone(), layer: e.layer.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
