//! ↩️ Inverse for `ChangeRepresentationPin`.

use crate::schema::mutations::SemioKitMutation;
use crate::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::ChangeRepresentationPin, base: &SemioKitSnapshot) -> Result<Vec<SemioKitMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.representations.get(payload.index) {
        Some(link) => vec![SemioKitMutation::ChangeRepresentationPin(super::ChangeRepresentationPin { index: payload.index, pin: link.pin.clone() })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
