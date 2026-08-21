//! ↩️ Inverse for `DisconnectKindCompatibility` — reconstructs a `connect-kind-compatibility` of
//! the captured BASE row. Missing target ⇒ `Vec::new()`.
use crate::artifacts::puzzle2d::mutations::Puzzle2dMutation;
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::DisconnectKindCompatibility, base: &Puzzle2dSnapshot) -> Vec<Puzzle2dMutation> {
    let Some(row) = base.meta.kind_compatibility.iter().find(|row| row.source == payload.source && row.target == payload.target) else {
        return Vec::new();
    };
    vec![crate::artifacts::puzzle2d::mutations::connect_kind_compatibility::mutation::connect_kind_compatibility(row.source.clone(), row.target.clone(), row.bidirectional, row.important, row.specificity)]
}
//#endregion 🔖️Inverse
