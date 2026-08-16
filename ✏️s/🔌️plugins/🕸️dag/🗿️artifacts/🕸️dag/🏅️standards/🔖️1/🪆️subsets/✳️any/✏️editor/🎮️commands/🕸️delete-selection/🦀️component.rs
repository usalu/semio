//! 🕸️ 🕸️ DAG play app commands command — `delete-selection`.

use crate::editor::dag::config::{DagConfig, DagConfigMutation};
use crate::artifacts::dag::op::DagMutation;
use crate::artifacts::dag::DagSnapshot;
use semio_framework_plugin::{app::InteractionView, ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️Shared
/// 🗑️ Builds the removal `DagMutation`s for the given node ids, or `None` when none of them exist —
/// shared by `delete_selection::DeleteSelection` and `node_graph_edit::DagNodeGraphEditOp::DeleteSelection`
/// (both were the same `handle_action` "deleteSelection" logic, reachable from two different action ids
/// pre-migration). No config mutation clears the selection any more: the framework auto-prunes the
/// deleted ids out of `graph`'s selection via `DagPlayApp::interaction_topology`
/// (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM). `remove_node::RemoveNode` deliberately
/// does NOT use this helper: it only ever removes the one node it names.
pub(crate) fn delete_selection_result(document: &DagSnapshot, node_ids: &[String]) -> Option<Vec<DagMutation>> {
    let removes = crate::artifacts::dag::schema::remove_nodes_operations(document, node_ids);
    if removes.is_empty() {
        None
    } else {
        Some(removes)
    }
}
//#endregion 🔖️Shared

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "delete-selection")]
pub struct DeleteSelection {}

/// 🕹️ `app_commands!`'s generated `dispatch(doc, cfg)` is framework-fixed at this exact 3-arg shape (no
/// `interaction` slot — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — reachable only
/// through that macro-generated path (`DagPlayApp::handle` always routes this command through `apply`
/// below instead), so it degrades to treating the selection as empty, matching `space`'s identical
/// `delete_selection` split.
pub fn handle(payload: &DeleteSelection, doc: &ArtifactView<'_, DagSnapshot>, cfg: &ConfigView<'_, DagConfig>) -> Result<Emit<DagMutation, DagConfigMutation>, Fault> {
    let _ = cfg;
    apply_to(payload, doc, &[])
}

pub fn apply(payload: &DeleteSelection, doc: &ArtifactView<'_, DagSnapshot>, _cfg: &ConfigView<'_, DagConfig>, interaction: &InteractionView<'_>) -> Result<Emit<DagMutation, DagConfigMutation>, Fault> {
    apply_to(payload, doc, &interaction.selection("graph").ids)
}

fn apply_to(_payload: &DeleteSelection, doc: &ArtifactView<'_, DagSnapshot>, selected: &[String]) -> Result<Emit<DagMutation, DagConfigMutation>, Fault> {
    match delete_selection_result(doc.snapshot, selected) {
        Some(removes) => Ok(Emit::mutations(removes)),
        None => Ok(Emit::default()),
    }
}
