//! ↩️ Inverse for `DisconnectKindCompatibility` — reconstructs a `connect-kind-compatibility` of
//! the captured BASE row. Missing target ⇒ `Vec::new()`.
use crate::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &super::DisconnectKindCompatibility, base: &Puzzle5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::connect_kind_compatibility::ConnectKindCompatibility>,
{
    Ok((|| {
    let Some(at) = base.kind_compatibility.iter().position(|row| row.source == payload.source && row.target == payload.target) else {
        return Vec::new();
    };
    let row = &base.kind_compatibility[at];
    vec![crate::standards::v1::subsets::any::schema::mutations::connect_kind_compatibility::connect_kind_compatibility(row.source.clone(), row.target.clone(), row.bidirectional, row.important, row.specificity, Some(at))]

    })())
}
//#endregion 🔖️Inverse
