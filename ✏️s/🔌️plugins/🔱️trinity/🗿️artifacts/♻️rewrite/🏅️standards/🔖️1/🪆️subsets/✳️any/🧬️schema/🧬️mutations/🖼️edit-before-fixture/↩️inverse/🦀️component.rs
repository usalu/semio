//! ↩️ Inverse for `EditBeforeFixture` — the OLD body looked up from BASE.
use crate::artifacts::rewrite::mutations::{edit_before_fixture, RewriteRuleMutation};
use crate::artifacts::rewrite::RewriteSnapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &super::mutation::EditBeforeFixture, base: &RewriteSnapshot) -> Vec<RewriteRuleMutation> {
    vec![edit_before_fixture(base.before_fixture_json.clone())]
}
//#endregion 🔖️Inverse
