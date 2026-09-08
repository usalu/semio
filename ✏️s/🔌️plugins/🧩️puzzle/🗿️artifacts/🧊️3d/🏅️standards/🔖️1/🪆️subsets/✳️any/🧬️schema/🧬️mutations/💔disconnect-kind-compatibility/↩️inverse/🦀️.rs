//! ↩️ Inverse for `DisconnectKindCompatibility` — reconstructs a `connect-kind-compatibility` of
//! the captured BASE row. Missing target ⇒ `Vec::new()`.
use crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation;
use crate::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::DisconnectKindCompatibility, base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
    let Some(row) = base.meta.kind_compatibility.iter().find(|row| row.source == payload.source && row.target == payload.target) else {
        return Vec::new();
    };
    vec![crate::standards::v1::subsets::any::schema::mutations::connect_kind_compatibility::mutation::connect_kind_compatibility(row.source.clone(), row.target.clone(), row.bidirectional, row.important, row.specificity)]
}
//#endregion 🔖️Inverse
