//! ↩️ Inverse for `DisconnectKindCompatibility` — reconstructs a `connect-kind-compatibility` of
//! the captured BASE row. Missing target ⇒ `Vec::new()`.
use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
use crate::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::DisconnectKindCompatibility, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
    let Some(row) = base.kind_compatibility.iter().find(|row| row.source == payload.source && row.target == payload.target) else {
        return Vec::new();
    };
    vec![crate::standards::v1::subsets::any::schema::mutations::connect_kind_compatibility::connect_kind_compatibility(row.source.clone(), row.target.clone(), row.bidirectional, row.important, row.specificity)]
}
//#endregion 🔖️Inverse
