//! ↩️ Inverse for `DisconnectKindCompatibility` — reconstructs a `connect-kind-compatibility` of
//! the captured BASE row. Missing target ⇒ `Vec::new()`.
use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
use crate::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::DisconnectKindCompatibility, base: &Puzzle5dSnapshot) -> Result<Vec<Puzzle5dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(at) = base.kind_compatibility.iter().position(|row| row.source == payload.source && row.target == payload.target) else {
        return Vec::new();
    };
    let row = &base.kind_compatibility[at];
    vec![crate::standards::v1::subsets::any::schema::mutations::connect_kind_compatibility::connect_kind_compatibility(row.source.clone(), row.target.clone(), row.bidirectional, row.important, row.specificity, Some(at))]

    })())
}
//#endregion 🔖️Inverse
