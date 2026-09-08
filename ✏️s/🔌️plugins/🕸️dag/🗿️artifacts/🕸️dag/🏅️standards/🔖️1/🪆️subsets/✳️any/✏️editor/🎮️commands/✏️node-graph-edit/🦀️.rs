//! 🕸️ 🕸️ DAG play app commands command — `node-graph-edit`.

use crate::mutations::{connect_nodes, dag_snapshot_mutations};
use crate::op::DagMutation;
use crate::DagSnapshot;
use crate::editor::dag::commands::delete_selection::delete_selection_result;
use crate::editor::dag::config::{DagConfig, DagConfigMutation};
use semio_framework_artifact_infinite_dag::{dag_document_from_fixture, DagFixture};
use semio_framework_plugin::{app::InteractionView, ArtifactView, ConfigView, Emit, Fault};
#[cfg(test)]
use serde::{Deserialize, Serialize};

/// 🎯️ One batched edit inside a `NodeGraphEdit` — mirrors the pre-migration `nodeGraphEdit` action's
/// `operations` JSON array (`"setFixture"`/`"deleteSelection"`/`"connect"` sub-kinds), now closed and
/// typed instead of stringly-tagged JSON.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslEnum)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
pub enum DagNodeGraphEditOp {
    #[dsl(key = "set-fixture")]
    SetFixture { fixture_json: String },
    #[dsl(key = "delete-selection")]
    DeleteSelection,
    #[dsl(key = "connect")]
    Connect { source_node_id: String, source_port_id: String, target_node_id: String, target_port_id: String },
}

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[dsl(keyword = "node-graph-edit")]
pub struct NodeGraphEdit {
    #[dsl(statements)]
    pub operations: Vec<DagNodeGraphEditOp>,
}

/// 🕹️ `app_commands!`'s generated `dispatch(doc, cfg)` is framework-fixed at this exact 3-arg shape (no
/// `interaction` slot — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — reachable only
/// through that macro-generated path (`DagPlayApp::handle` always routes this command through `apply`
/// below instead), so its `DeleteSelection` sub-op degrades to treating the selection as empty.
pub fn handle(payload: &NodeGraphEdit, doc: &ArtifactView<'_, DagSnapshot>, cfg: &ConfigView<'_, DagConfig>) -> Result<Emit<DagMutation, DagConfigMutation>, Fault> {
    Ok(apply_to(payload, doc, cfg, &[]))
}

pub fn apply(payload: &NodeGraphEdit, doc: &ArtifactView<'_, DagSnapshot>, cfg: &ConfigView<'_, DagConfig>, interaction: &InteractionView<'_>) -> Result<Emit<DagMutation, DagConfigMutation>, Fault> {
    Ok(apply_to(payload, doc, cfg, &interaction.selection("graph").ids))
}

fn apply_to(payload: &NodeGraphEdit, doc: &ArtifactView<'_, DagSnapshot>, _cfg: &ConfigView<'_, DagConfig>, selected: &[String]) -> Emit<DagMutation, DagConfigMutation> {
    let document = doc.snapshot;
    let mut artifact_mutations: Vec<DagMutation> = Vec::new();
    let mut config_mutations: Vec<DagConfigMutation> = Vec::new();
    for sub_operation in &payload.operations {
        match sub_operation {
            DagNodeGraphEditOp::SetFixture { fixture_json } => {
                if let Ok(fixture) = dsl::json::from_json_str::<DagFixture>(fixture_json) {
                    config_mutations.push(DagConfigMutation::ChangeCamera(crate::editor::dag::config::ChangeCamera { x: fixture.camera.x, y: fixture.camera.y, zoom: fixture.camera.zoom }));
                    artifact_mutations.extend(dag_snapshot_mutations(document, &dag_document_from_fixture(&fixture).into()));
                }
            }
            DagNodeGraphEditOp::DeleteSelection => {
                if let Some(removes) = delete_selection_result(document, selected) {
                    artifact_mutations.extend(removes);
                }
            }
            DagNodeGraphEditOp::Connect { source_node_id, source_port_id, target_node_id, target_port_id } => {
                if let Ok(edge) = crate::schema::connect_edge(document, source_node_id, source_port_id, target_node_id, target_port_id) {
                    artifact_mutations.push(connect_nodes(edge.id, edge.source, edge.target, edge.route_style, edge.properties));
                }
            }
        }
    }
    Emit { artifact_mutations, config_mutations, ..Default::default() }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
