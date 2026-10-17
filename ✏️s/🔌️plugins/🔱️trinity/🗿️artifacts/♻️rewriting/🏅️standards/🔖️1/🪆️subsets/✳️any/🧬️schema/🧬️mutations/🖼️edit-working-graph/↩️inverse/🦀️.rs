//! ↩️ Inverse for `EditWorkingGraph` — the OLD body looked up from BASE.
use crate::standards::v1::subsets::any::schema::mutations::edit_working_graph::edit_working_graph;

use crate::standards::v1::subsets::any::schema::snapshot::RewritingSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &super::EditWorkingGraph, base: &RewritingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::edit_working_graph::EditWorkingGraph>,
{
    Ok((|| {
    vec![edit_working_graph(base.working_graph.clone())]

    })())
}
//#endregion 🔖️Inverse
