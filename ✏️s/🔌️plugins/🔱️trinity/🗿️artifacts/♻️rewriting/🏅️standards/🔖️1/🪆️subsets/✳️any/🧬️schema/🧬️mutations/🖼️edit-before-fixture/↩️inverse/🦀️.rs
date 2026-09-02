//! ↩️ Inverse for `EditBeforeFixture` — the OLD body looked up from BASE.
use crate::artifacts::rewriting::mutations::{edit_before_fixture, RewriteRuleMutation};
use crate::artifacts::rewriting::RewritingSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::EditBeforeFixture, base: &RewritingSnapshot) -> Vec<RewriteRuleMutation> {
    vec![edit_before_fixture(base.before_fixture_json.clone())]
}
//#endregion 🔖️Inverse
