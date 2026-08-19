//! ↩️ Inverse for `ConnectKindCompatibility` — always a `disconnect-kind-compatibility` of the
//! pair it created.
use crate::artifacts::puzzle3d::mutations::Puzzle3dMutation;
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::ConnectKindCompatibility, _base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
    vec![crate::artifacts::puzzle3d::mutations::disconnect_kind_compatibility::mutation::disconnect_kind_compatibility(payload.source.clone(), payload.target.clone())]
}
//#endregion 🔖️Inverse
