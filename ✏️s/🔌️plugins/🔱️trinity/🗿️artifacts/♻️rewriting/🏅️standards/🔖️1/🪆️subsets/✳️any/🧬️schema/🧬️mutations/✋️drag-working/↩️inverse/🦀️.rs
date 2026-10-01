//! ↩️ Inverse for `DragWorkingNodes` — ONE `edit-before-fixture` putting the BASE working graph back (never a negated offset,
//! which rounding would not restore exactly); nothing when the drag moves nothing.
use crate::standards::v1::subsets::any::schema::mutations::{edit_before_fixture, RewriteRuleMutation};
use crate::RewritingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::DragWorkingNodes, base: &RewritingSnapshot) -> Vec<RewriteRuleMutation> {
    match super::diff::diff(payload, base).diff().before_fixture_json.is_some() {
        true => vec![edit_before_fixture(base.before_fixture_json.clone())],
        false => Vec::new(),
    }
}
//#endregion 🔖️Inverse
