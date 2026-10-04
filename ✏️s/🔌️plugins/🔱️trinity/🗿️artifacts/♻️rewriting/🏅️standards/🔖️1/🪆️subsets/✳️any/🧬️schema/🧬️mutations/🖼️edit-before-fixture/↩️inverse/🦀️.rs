//! ↩️ Inverse for `EditBeforeFixture` — the OLD body looked up from BASE.
use crate::standards::v1::subsets::any::schema::mutations::{edit_before_fixture, RewriteRuleMutation};
use crate::RewritingSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::EditBeforeFixture, base: &RewritingSnapshot) -> Result<Vec<RewriteRuleMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![edit_before_fixture(base.working_graph.clone())]

    })())
}
//#endregion 🔖️Inverse
