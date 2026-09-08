//! ↩️ Inverse for `EditRhs` — the OLD body looked up from BASE.
use crate::standards::v1::subsets::any::schema::mutations::{edit_rhs, RewriteRuleMutation};
use crate::RewritingSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::EditRhs, base: &RewritingSnapshot) -> Vec<RewriteRuleMutation> {
    vec![edit_rhs(base.rhs_json.clone())]
}
//#endregion 🔖️Inverse
