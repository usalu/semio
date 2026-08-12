//! ↩️ `unbind-representation` — undo is `bind-representation` with the escrowed link from BASE;
//! empty when the index was already out of range.

use super::mutation::UnbindRepresentation;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::mutations::{bind_representation, SemioKitMutation};
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &UnbindRepresentation, base: &SemioKitSnapshot) -> Vec<SemioKitMutation> {
    match base.representations.get(payload.index) {
        Some(link) => vec![SemioKitMutation::BindRepresentation(bind_representation::mutation::BindRepresentation { target: link.target.clone(), pin: link.pin.clone(), role: link.role.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
