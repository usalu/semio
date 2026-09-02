//! ↩️ Inverse for `EditLhs` — the OLD body looked up from BASE.
use crate::artifacts::rewriting::mutations::{edit_lhs, RewriteRuleMutation};
use crate::artifacts::rewriting::RewritingSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::EditLhs, base: &RewritingSnapshot) -> Vec<RewriteRuleMutation> {
    vec![edit_lhs(base.lhs_json.clone())]
}
//#endregion 🔖️Inverse
