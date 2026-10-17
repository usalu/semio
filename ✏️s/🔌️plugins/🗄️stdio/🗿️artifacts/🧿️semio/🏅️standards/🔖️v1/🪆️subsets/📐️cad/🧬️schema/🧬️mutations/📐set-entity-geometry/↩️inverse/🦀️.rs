//! ↩️ Inverse for `SetEntityGeometry`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioCadSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::support::find_entity;
#[allow(unused_imports)]
use crate::schema::mutations::set_entity_geometry;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::SetEntityGeometry, base: &SemioCadSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::set_entity_geometry::SetEntityGeometry>,
{
    let super::SetEntityGeometry { handle, .. } = payload;
    Ok(match find_entity(base, handle) {
        Some(e) => vec![Op::from(set_entity_geometry::SetEntityGeometry { handle: handle.clone(), entity: e.entity.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
