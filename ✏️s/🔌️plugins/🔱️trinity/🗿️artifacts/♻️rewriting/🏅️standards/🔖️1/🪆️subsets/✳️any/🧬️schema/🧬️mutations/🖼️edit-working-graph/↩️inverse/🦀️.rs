//! ↩️ Inverse for `EditWorkingGraph` — the OLD body looked up from BASE.
use crate::standards::v1::subsets::any::schema::mutations::{edit_working_graph, RewriteRuleMutation};
use crate::RewritingSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::EditWorkingGraph, base: &RewritingSnapshot) -> Result<Vec<RewriteRuleMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![edit_working_graph(base.working_graph.clone())]

    })())
}
//#endregion 🔖️Inverse
