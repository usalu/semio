//! ✏️ ✏️ S Studio app command — `node-graph-edit`.

use crate::engine::space::config::{SpaceConfig, SpaceConfigMutation};
use semio_framework_os::{apply_flow_fixture_to_os_workflow, OsWorkflowCamera, WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{app::InteractionView, ArtifactView, ConfigView, Emit, Fault};
use serde_json::Value;

use serde::{Deserialize, Serialize};

/// 🚧️ TEMP(Wave 3): `operations_json` stays an opaque JSON-array string, mirroring
/// `apply_flow_fixture_to_os_workflow`'s still-JSON `fixture_json` bridge — typed once the flow
/// bridge itself is typed.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "node-graph-edit")]
pub struct NodeGraphEdit {
    pub operations_json: String,
}

async fn edit_with_selection(payload: &NodeGraphEdit, projection: &WorkflowSnapshot, selected: &[String]) -> Emit<WorkflowMutation, SpaceConfigMutation> {
    let edit_operations = serde_json::from_str::<Value>(&payload.operations_json).ok().and_then(|value| value.get("operations").and_then(Value::as_array).cloned()).unwrap_or_default();
    let mut artifact_mutations = Vec::new();
    let mut config_mutations = Vec::new();
    let mut effects = Vec::new();
    for edit in &edit_operations {
        match edit.get("operation").and_then(Value::as_str).unwrap_or("") {
            "setFixture" => {
                if let Some(fixture_json) = edit.get("fixtureJson").and_then(Value::as_str) {
                    if let Some(camera) = serde_json::from_str::<Value>(fixture_json).ok().and_then(|fixture| fixture.get("camera").cloned()).and_then(|camera| serde_json::from_value::<OsWorkflowCamera>(camera).ok()) {
                        config_mutations.push(SpaceConfigMutation::SetCamera { window_id: crate::engine::space::modes::main::windows::workflow::S_PLAY_WINDOW_WORKFLOW.into(), camera: camera.into() });
                    }
                    artifact_mutations.extend(apply_flow_fixture_to_os_workflow(&projection.graph, fixture_json));
                }
            }
            "move" => {
                if let (Some(node_id), Some(x), Some(y)) = (edit.get("nodeId").and_then(Value::as_str), edit.get("x").and_then(Value::as_f64), edit.get("y").and_then(Value::as_f64)) {
                    artifact_mutations.push(WorkflowMutation::MoveNode { node_id: node_id.into(), x, y });
                }
            }
            "connect" => {
                if let (Some(source_node_id), Some(source_port_id), Some(target_node_id), Some(target_port_id)) =
                    (edit.get("sourceNodeId").and_then(Value::as_str), edit.get("sourcePortId").and_then(Value::as_str), edit.get("targetNodeId").and_then(Value::as_str), edit.get("targetPortId").and_then(Value::as_str))
                {
                    match crate::engine::space::negotiate_connect_or_notify(projection, source_node_id, source_port_id, target_node_id, target_port_id) {
                        Ok(contract) => artifact_mutations.push(crate::engine::space::connect_edge_operation(source_node_id, source_port_id, target_node_id, target_port_id, contract)),
                        Err(effect) => effects.push(effect),
                    }
                }
            }
            "deleteSelection" => {
                for node_id in selected {
                    artifact_mutations.push(WorkflowMutation::RemoveNode { node_id: node_id.clone() });
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
pub async fn handle(payload: &NodeGraphEdit, doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    Ok(edit_with_selection(payload, doc.snapshot, &[]))
}

pub async fn apply(payload: &NodeGraphEdit, doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>, interaction: &InteractionView<'_>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    Ok(edit_with_selection(payload, doc.snapshot, &interaction.selection("graph").ids))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn space_command_op_text_round_trips_every_variant() {
        use crate::engine::space::SpaceCommand;
        store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::NodeGraphEdit(NodeGraphEdit { operations_json: "[]".into() }));
    }

    #[semio_framework_async_macros::async_test]
    async fn node_graph_edit_set_fixture_moves_node_and_persists_camera() {
        use crate::engine::space::testkit::{apply_mutations, studio_emit};
        use crate::engine::space::SpaceCommand;
        use crate::demo_space_projection;
        use semio_framework_os::os_workflow_to_flow_fixture;
        use serde_json::json;
        let projection = demo_space_projection();
        let config = SpaceConfig::default();
        let node = projection.graph.nodes.first().expect("node").clone();
        let camera = OsWorkflowCamera { x: 40.0, y: -20.0, zoom: 2.0 };
        let mut fixture = os_workflow_to_flow_fixture(&projection.graph, &camera);
        fixture["layout"][&node.id] = json!({ "x": 500.0 + node.width / 2.0, "y": 300.0 + node.height / 2.0 });
        let operations_json = json!({ "operations": [{ "operation": "setFixture", "fixtureJson": fixture.to_string() }] }).to_string();
        let emit = studio_emit(&projection, &config, &SpaceCommand::NodeGraphEdit(NodeGraphEdit { operations_json })).expect("handle");
        let moved = apply_mutations(&projection, &emit.artifact_mutations).graph.nodes.into_iter().find(|row| row.id == node.id).expect("node");
        assert!((moved.x - 500.0).abs() < 0.01);
        assert!((moved.y - 300.0).abs() < 0.01);
        assert_eq!(emit.config_mutations, vec![SpaceConfigMutation::SetCamera { window_id: crate::engine::space::modes::main::windows::workflow::S_PLAY_WINDOW_WORKFLOW.into(), camera: camera.into() }]);
    }
}
//#endregion 🧪️Tests
