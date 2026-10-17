//! ↩️ Inverse for `ChangeRepresentationPin`.

use crate::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::ChangeRepresentationPin, base: &SemioKitSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::change_representation_pin::ChangeRepresentationPin>,
{
    Ok((|| {
    match base.representations.get(payload.index) {
        Some(link) => vec![Op::from(super::ChangeRepresentationPin { index: payload.index, pin: link.pin.clone() })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
