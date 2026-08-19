//! ↩️ Inverse for `EditLhs` — the OLD body looked up from BASE.
use crate::artifacts::rewrite::mutations::{edit_lhs, RewriteRuleMutation};
use crate::artifacts::rewrite::RewriteSnapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &super::mutation::EditLhs, base: &RewriteSnapshot) -> Vec<RewriteRuleMutation> {
    vec![edit_lhs(base.lhs_json.clone())]
}
//#endregion 🔖️Inverse
