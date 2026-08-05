//! ✏️ S Studio app — bulk fixture-driven graph edit command (used by the Flow node-graph engine's own
//! edit-op batch: setFixture/move/connect/deleteSelection).

use crate::apps::space::config::{SpaceConfig, SpaceConfigOperation};
use semio_framework_os::{apply_flow_fixture_to_os_workflow, OsWorkflowCamera, WorkflowDocument, WorkflowOperation};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde_json::Value;

//#region 🔖️NodeGraphEdit
pub mod node_graph_edit {
    use super::*;
    use serde::{Deserialize, Serialize};

    /// 🚧️ TEMP(Wave 3): `operations_json` stays an opaque JSON-array string, mirroring
    /// `apply_flow_fixture_to_os_workflow`'s still-JSON `fixture_json` bridge — typed once the flow
    /// bridge itself is typed.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "node-graph-edit")]
    pub struct NodeGraphEdit {
        pub operations_json: String,
    }

    pub fn handle(payload: &NodeGraphEdit, doc: &DocumentView<'_, WorkflowDocument>, cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowOperation, SpaceConfigOperation>, Fault> {
        let projection = doc.projection;
        let config = cfg.projection;
        let edit_operations = serde_json::from_str::<Value>(&payload.operations_json).ok().and_then(|value| value.get("operations").and_then(Value::as_array).cloned()).unwrap_or_default();
        let mut document_operations = Vec::new();
        let mut config_operations = Vec::new();
        let mut effects = Vec::new();
        for edit in &edit_operations {
            match edit.get("operation").and_then(Value::as_str).unwrap_or("") {
                "setFixture" => {
                    if let Some(fixture_json) = edit.get("fixtureJson").and_then(Value::as_str) {
                        if let Some(camera) = serde_json::from_str::<Value>(fixture_json).ok().and_then(|fixture| fixture.get("camera").cloned()).and_then(|camera| serde_json::from_value::<OsWorkflowCamera>(camera).ok()) {
                            config_operations.push(SpaceConfigOperation::SetCamera { window_id: crate::apps::space::modes::main::windows::workflow::S_PLAY_WINDOW_WORKFLOW.into(), camera: camera.into() });
                        }
                        document_operations.extend(apply_flow_fixture_to_os_workflow(&projection.graph, fixture_json));
                    }
                }
                "move" => {
                    if let (Some(node_id), Some(x), Some(y)) = (edit.get("nodeId").and_then(Value::as_str), edit.get("x").and_then(Value::as_f64), edit.get("y").and_then(Value::as_f64)) {
                        document_operations.push(WorkflowOperation::MoveNode { node_id: node_id.into(), x, y });
                    }
                }
                "connect" => {
                    if let (Some(source_node_id), Some(source_port_id), Some(target_node_id), Some(target_port_id)) =
                        (edit.get("sourceNodeId").and_then(Value::as_str), edit.get("sourcePortId").and_then(Value::as_str), edit.get("targetNodeId").and_then(Value::as_str), edit.get("targetPortId").and_then(Value::as_str))
                    {
                        match crate::apps::space::negotiate_connect_or_notify(projection, source_node_id, source_port_id, target_node_id, target_port_id) {
                            Ok(contract) => document_operations.push(crate::apps::space::connect_edge_operation(source_node_id, source_port_id, target_node_id, target_port_id, contract)),
                            Err(effect) => effects.push(effect),
                        }
                    }
                }
                "deleteSelection" => {
                    for node_id in &config.selected_node_ids {
                        document_operations.push(WorkflowOperation::RemoveNode { node_id: node_id.clone() });
                    }
                }
                _ => {}
            }
        }
        Ok(Emit { document_operations, config_operations, effects, ..Default::default() })
    }
}
//#endregion 🔖️NodeGraphEdit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn space_command_op_text_round_trips_every_variant() {
        store::test_support::assert_op_line_round_trip(&node_graph_edit::NodeGraphEdit { operations_json: "[]".into() });
    }

    #[test]
    fn node_graph_edit_set_fixture_moves_node_and_persists_camera() {
        use crate::apps::space::testkit::{apply_operations, studio_emit};
        use crate::apps::space::SpaceCommand;
        use crate::core::demo_space_projection;
        use semio_framework_os::os_workflow_to_flow_fixture;
        use serde_json::json;
        let projection = demo_space_projection();
        let config = SpaceConfig::default();
        let node = projection.graph.nodes.first().expect("node").clone();
        let camera = OsWorkflowCamera { x: 40.0, y: -20.0, zoom: 2.0 };
        let mut fixture = os_workflow_to_flow_fixture(&projection.graph, &camera);
        fixture["layout"][&node.id] = json!({ "x": 500.0 + node.width / 2.0, "y": 300.0 + node.height / 2.0 });
        let operations_json = json!({ "operations": [{ "operation": "setFixture", "fixtureJson": fixture.to_string() }] }).to_string();
        let emit = studio_emit(&projection, &config, SpaceCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json })).expect("handle");
        let moved = apply_operations(&projection, &emit.document_operations).graph.nodes.into_iter().find(|row| row.id == node.id).expect("node");
        assert!((moved.x - 500.0).abs() < 0.01);
        assert!((moved.y - 300.0).abs() < 0.01);
        assert_eq!(emit.config_operations, vec![SpaceConfigOperation::SetCamera { window_id: crate::apps::space::modes::main::windows::workflow::S_PLAY_WINDOW_WORKFLOW.into(), camera: camera.into() }]);
    }
}
//#endregion 🧪️Tests
