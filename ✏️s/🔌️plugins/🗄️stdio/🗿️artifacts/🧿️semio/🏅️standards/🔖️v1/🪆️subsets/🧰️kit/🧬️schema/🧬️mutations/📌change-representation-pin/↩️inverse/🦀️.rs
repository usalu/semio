//! ↩️ Inverse for `ChangeRepresentationPin`.

use crate::artifacts::semio::standards::v1::subsets::kit::schema::mutations::SemioKitMutation;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::ChangeRepresentationPin, base: &SemioKitSnapshot) -> Vec<SemioKitMutation> {
    match base.representations.get(payload.index) {
        Some(link) => vec![SemioKitMutation::ChangeRepresentationPin(super::ChangeRepresentationPin { index: payload.index, pin: link.pin.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
