//! ↩️ Inverse for `DeleteWorkingNodes` — ONE `edit-before-fixture` putting the BASE working graph back, every removed node and
//! edge with it; nothing when the delete removes nothing.
use crate::standards::v1::subsets::any::schema::mutations::{edit_before_fixture, RewriteRuleMutation};
use crate::RewritingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::DeleteWorkingNodes, base: &RewritingSnapshot) -> Vec<RewriteRuleMutation> {
    match super::diff::diff(payload, base).diff().before_fixture_json.is_some() {
        true => vec![edit_before_fixture(base.before_fixture_json.clone())],
        false => Vec::new(),
    }
}
//#endregion 🔖️Inverse
