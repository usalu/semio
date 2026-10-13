//! ↩️ Inverse for `UnbindRepresentation`.

use crate::schema::mutations::{bind_representation, SemioKitMutation};
use crate::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::UnbindRepresentation, base: &SemioKitSnapshot) -> Result<Vec<SemioKitMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    if payload.index >= base.representations.len() {
        return Vec::new();
    }
    let link = &base.representations[payload.index];
    vec![SemioKitMutation::BindRepresentation(bind_representation::BindRepresentation { target: link.target.clone(), pin: link.pin.clone(), role: link.role.clone(), at: Some(payload.index) })]

    })())
}
//#endregion 🔖️Inverse
