//! ↩️ Inverse for `EditLhs` — the OLD body looked up from BASE.
use crate::standards::v1::subsets::any::schema::mutations::edit_lhs::edit_lhs;

use crate::standards::v1::subsets::any::schema::snapshot::RewritingSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &super::EditLhs, base: &RewritingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::edit_lhs::EditLhs>,
{
    Ok((|| {
    vec![edit_lhs(base.lhs.clone())]

    })())
}
//#endregion 🔖️Inverse
