//! ↩️ Inverse for `DisconnectKindCompatibility` — reconstructs a `connect-kind-compatibility` of
//! the captured BASE row. Missing target ⇒ `Vec::new()`.
use crate::artifacts::puzzle3d::mutations::Puzzle3dMutation;
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::DisconnectKindCompatibility, base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
    let Some(row) = base.meta.kind_compatibility.iter().find(|row| row.source == payload.source && row.target == payload.target) else {
        return Vec::new();
    };
    vec![crate::artifacts::puzzle3d::mutations::connect_kind_compatibility::mutation::connect_kind_compatibility(
        row.source.clone(), row.target.clone(), row.bidirectional, row.important, row.specificity,
    )]
}
//#endregion 🔖️Inverse
