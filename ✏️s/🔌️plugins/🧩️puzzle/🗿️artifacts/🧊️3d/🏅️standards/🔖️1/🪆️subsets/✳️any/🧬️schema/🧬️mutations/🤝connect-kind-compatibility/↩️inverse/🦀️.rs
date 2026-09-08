//! ↩️ Inverse for `ConnectKindCompatibility` — always a `disconnect-kind-compatibility` of the
//! pair it created.
use crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation;
use crate::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::ConnectKindCompatibility, _base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
    vec![crate::standards::v1::subsets::any::schema::mutations::disconnect_kind_compatibility::mutation::disconnect_kind_compatibility(payload.source.clone(), payload.target.clone())]
}
//#endregion 🔖️Inverse
