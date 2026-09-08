//! ↩️ Inverse for `ConnectKindCompatibility` — always a `disconnect-kind-compatibility` of the
//! pair it created.
use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
use crate::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ConnectKindCompatibility, _base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
    vec![crate::standards::v1::subsets::any::schema::mutations::disconnect_kind_compatibility::disconnect_kind_compatibility(payload.source.clone(), payload.target.clone())]
}
//#endregion 🔖️Inverse
