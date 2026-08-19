//! ↩️ Inverse for `ConnectKindCompatibility` — always a `disconnect-kind-compatibility` of the
//! pair it created.
use crate::artifacts::puzzle5d::mutations::Puzzle5dMutation;
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::ConnectKindCompatibility, _base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
    vec![crate::artifacts::puzzle5d::mutations::disconnect_kind_compatibility::mutation::disconnect_kind_compatibility(payload.source.clone(), payload.target.clone())]
}
//#endregion 🔖️Inverse
