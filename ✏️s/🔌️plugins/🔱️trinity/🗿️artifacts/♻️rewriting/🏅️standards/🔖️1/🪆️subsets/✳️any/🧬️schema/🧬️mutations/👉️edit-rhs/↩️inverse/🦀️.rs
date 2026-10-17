//! ↩️ Inverse for `EditRhs` — the OLD body looked up from BASE.
use crate::standards::v1::subsets::any::schema::mutations::edit_rhs::edit_rhs;

use crate::standards::v1::subsets::any::schema::snapshot::RewritingSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &super::EditRhs, base: &RewritingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::edit_rhs::EditRhs>,
{
    Ok((|| {
    vec![edit_rhs(base.rhs.clone())]

    })())
}
//#endregion 🔖️Inverse
