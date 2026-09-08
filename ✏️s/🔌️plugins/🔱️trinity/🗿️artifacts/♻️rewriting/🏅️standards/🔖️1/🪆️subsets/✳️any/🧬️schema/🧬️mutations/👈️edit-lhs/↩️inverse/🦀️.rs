//! ↩️ Inverse for `EditLhs` — the OLD body looked up from BASE.
use crate::standards::v1::subsets::any::schema::mutations::{edit_lhs, RewriteRuleMutation};
use crate::RewritingSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::EditLhs, base: &RewritingSnapshot) -> Vec<RewriteRuleMutation> {
    vec![edit_lhs(base.lhs_json.clone())]
}
//#endregion 🔖️Inverse
