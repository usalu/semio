//! ↩️ Inverse for `SetElement`.

use crate::schema::snapshot::SemioModelSnapshot;
use crate::schema::mutations::set_element;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::SetElement, base: &SemioModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::set_element::SetElement>,
{
    let super::SetElement { id, class, placement, geometry, spatial_id, psets } = payload;
    Ok(match base.elements.iter().find(|e| &e.id == id) {
        Some(original) => vec![Op::from(set_element::SetElement {
            id: id.clone(),
            class: class.as_ref().map(|_| original.class.clone()),
            placement: placement.as_ref().map(|_| original.placement),
            geometry: geometry.as_ref().map(|_| original.geometry.clone()),
            spatial_id: spatial_id.as_ref().map(|_| original.spatial_id.clone()),
            psets: psets.as_ref().map(|_| original.psets.clone()),
        })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
