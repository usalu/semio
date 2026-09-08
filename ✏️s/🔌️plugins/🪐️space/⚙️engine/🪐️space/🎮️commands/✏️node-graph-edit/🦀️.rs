//! ✏️ ✏️ S Studio app command — `node-graph-edit`.

use crate::engine::space::config::{SpaceConfig, SpaceConfigMutation, SpaceWindowCamera};
use semio_framework_os::workflow::{MoveNode, RemoveNode};
use semio_framework_os::{apply_flow_fixture_to_os_workflow, WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{app::InteractionView, ArtifactView, ConfigView, Emit, Fault};

/// 🚧️ TEMP(Wave 3): `operations_json` stays an opaque JSON-array string, mirroring
/// `apply_flow_fixture_to_os_workflow`'s still-JSON `fixture_json` bridge — typed once the flow
/// bridge itself is typed.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "node-graph-edit")]
pub struct NodeGraphEdit {
    pub operations_json: String,
}

async fn edit_with_selection(payload: &NodeGraphEdit, projection: &WorkflowSnapshot, selected: &[String]) -> Emit<WorkflowMutation, SpaceConfigMutation> {
    let edit_operations = pack::parse_json(&payload.operations_json).ok().and_then(|value| value.get("operations").and_then(pack::JsonValue::as_array).cloned()).unwrap_or_default();
    let mut artifact_mutations = Vec::new();
    let mut config_mutations = Vec::new();
    let mut effects = Vec::new();
    for edit in &edit_operations {
        match edit.get("operation").and_then(pack::JsonValue::as_str).unwrap_or("") {
            "setFixture" => {
                if let Some(fixture_json) = edit.get("fixtureJson").and_then(pack::JsonValue::as_str) {
                    if let Some(camera) = pack::parse_json(fixture_json).ok().and_then(|fixture| fixture.get("camera").cloned()).and_then(|camera| dsl::from_dsl_value::<SpaceWindowCamera>(pack::json_to_dsl_value(&camera)).ok()) {
                        config_mutations.push(SpaceConfigMutation::SetCamera { window_id: crate::engine::space::modes::main::windows::workflow::S_PLAY_WINDOW_WORKFLOW.into(), camera });
                    }
                    artifact_mutations.extend(apply_flow_fixture_to_os_workflow(&projection.graph, fixture_json));
                }
            }
            "move" => {
                if let (Some(node_id), Some(x), Some(y)) = (edit.get("nodeId").and_then(pack::JsonValue::as_str), edit.get("x").and_then(pack::JsonValue::as_f64), edit.get("y").and_then(pack::JsonValue::as_f64)) {
                    artifact_mutations.push(WorkflowMutation::MoveNode(MoveNode { node_id: node_id.into(), x, y }));
                }
            }
            "connect" => {
                if let (Some(source_node_id), Some(source_port_id), Some(target_node_id), Some(target_port_id)) = (
                    edit.get("sourceNodeId").and_then(pack::JsonValue::as_str),
                    edit.get("sourcePortId").and_then(pack::JsonValue::as_str),
                    edit.get("targetNodeId").and_then(pack::JsonValue::as_str),
                    edit.get("targetPortId").and_then(pack::JsonValue::as_str),
                ) {
                    match crate::engine::space::negotiate_connect_or_notify(projection, source_node_id, source_port_id, target_node_id, target_port_id).await {
                        Ok(contract) => artifact_mutations.push(crate::engine::space::connect_edge_operation(source_node_id, source_port_id, target_node_id, target_port_id, contract).await),
                        Err(effect) => effects.push(effect),
                    }
                }
            }
            "deleteSelection" => {
                for node_id in selected {
                    artifact_mutations.push(WorkflowMutation::RemoveNode(RemoveNode { node_id: node_id.clone() }));
                }
            }
            _ => {}
        }
    }
    Emit { artifact_mutations, config_mutations, effects, ..Default::default() }
}

/// 🕹️ `app_commands!`'s generated `dispatch(doc, cfg)` is framework-fixed at this exact 3-arg shape
/// (no `interaction` slot — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — reachable
/// only through that macro-generated path (`SpaceApp::handle` always routes this command through
/// `apply` below instead), so its `"deleteSelection"` sub-operation degrades to treating the selection
/// as empty; every other sub-operation (`setFixture`/`move`/`connect`) is unaffected.
pub fn handle(payload: &NodeGraphEdit, doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    Ok(crate::engine::space::engine::resolve_future(edit_with_selection(payload, doc.snapshot, &[])))
}

pub async fn apply(payload: &NodeGraphEdit, doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>, interaction: &InteractionView<'_>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    Ok(edit_with_selection(payload, doc.snapshot, &interaction.selection("graph").ids).await)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
