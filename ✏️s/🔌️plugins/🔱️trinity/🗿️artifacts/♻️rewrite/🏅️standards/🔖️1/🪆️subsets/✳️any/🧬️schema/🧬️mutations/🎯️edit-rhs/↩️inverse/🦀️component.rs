//! ↩️ Inverse for `EditRhs` — the OLD body looked up from BASE.
use crate::artifacts::rewrite::mutations::{edit_rhs, RewriteRuleMutation};
use crate::artifacts::rewrite::RewriteSnapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &super::mutation::EditRhs, base: &RewriteSnapshot) -> Vec<RewriteRuleMutation> {
    vec![edit_rhs(base.rhs_json.clone())]
}
//#endregion 🔖️Inverse
