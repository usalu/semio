//! ↩️ Inverse for `ConnectKindCompatibility` — always a `disconnect-kind-compatibility` of the
//! pair it created.
use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
use crate::Puzzle2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ConnectKindCompatibility, _base: &Puzzle2dSnapshot) -> Vec<Puzzle2dMutation> {
    vec![crate::standards::v1::subsets::any::schema::mutations::disconnect_kind_compatibility::disconnect_kind_compatibility(payload.source.clone(), payload.target.clone())]
}
//#endregion 🔖️Inverse
