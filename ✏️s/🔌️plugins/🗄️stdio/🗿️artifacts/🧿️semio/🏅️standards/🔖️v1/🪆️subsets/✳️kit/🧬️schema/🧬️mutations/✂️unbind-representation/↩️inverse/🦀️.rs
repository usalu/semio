//! ↩️ Inverse for `UnbindRepresentation`.

use crate::artifacts::semio::standards::v1::subsets::kit::schema::mutations::{SemioKitMutation, bind_representation};
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::UnbindRepresentation, base: &SemioKitSnapshot) -> Vec<SemioKitMutation> {
    if payload.index >= base.representations.len() {
        return Vec::new();
    }
    let tail = &base.representations[payload.index..];
    let mut undo: Vec<SemioKitMutation> = (1..tail.len()).map(|_| SemioKitMutation::UnbindRepresentation(super::UnbindRepresentation { index: payload.index })).collect();
    undo.extend(tail.iter().map(|link| SemioKitMutation::BindRepresentation(bind_representation::BindRepresentation { target: link.target.clone(), pin: link.pin.clone(), role: link.role.clone() })));
    undo
}
//#endregion 🔖️Inverse
