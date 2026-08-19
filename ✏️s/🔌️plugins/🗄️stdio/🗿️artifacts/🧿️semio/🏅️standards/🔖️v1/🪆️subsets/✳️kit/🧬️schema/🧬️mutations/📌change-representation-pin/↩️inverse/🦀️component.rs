//! ↩️ `change-representation-pin` — self-inverse: another re-pin restoring the BASE-state pin;
//! empty when the index was out of range.

use super::mutation::ChangeRepresentationPin;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::mutations::SemioKitMutation;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &ChangeRepresentationPin, base: &SemioKitSnapshot) -> Vec<SemioKitMutation> {
    match base.representations.get(payload.index) {
        Some(link) => vec![SemioKitMutation::ChangeRepresentationPin(ChangeRepresentationPin { index: payload.index, pin: link.pin.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
