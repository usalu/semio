//! 🗂️ S Studio app — node/instance selection commands.

use crate::apps::space::config::{SpaceConfig, SpaceConfigOperation};
use semio_framework_os::{WorkflowDocument, WorkflowOperation};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};

//#region 🔖️SelectInstance
pub mod select_instance {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "select-instance")]
    pub struct SelectInstance {
        pub node_id: Option<String>,
    }

    pub fn handle(payload: &SelectInstance, _doc: &DocumentView<'_, WorkflowDocument>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowOperation, SpaceConfigOperation>, Fault> {
        let node_ids = payload.node_id.iter().cloned().collect();
        Ok(Emit::config(vec![SpaceConfigOperation::SetActiveNode { node_id: payload.node_id.clone() }, SpaceConfigOperation::SetSelection { node_ids }]))
    }
}
//#endregion 🔖️SelectInstance

//#region 🔖️GraphSelection
/// 🔁️ Shared body for `node_graph_select` and `set_media_node_selection` — both replace the node
/// selection wholesale (optionally "select all"), publish presence, and set a single active node when
/// exactly one is selected.
fn select_nodes(node_ids: Vec<String>, select_all: bool, projection: &WorkflowDocument, _config: &SpaceConfig) -> Emit<WorkflowOperation, SpaceConfigOperation> {
    let node_ids = if select_all { projection.graph.nodes.iter().map(|node| node.id.clone()).collect() } else { node_ids };
    let mut config_operations = vec![SpaceConfigOperation::SetSelection { node_ids: node_ids.clone() }];
    if node_ids.len() == 1 {
        config_operations.push(SpaceConfigOperation::SetActiveNode { node_id: node_ids.first().cloned() });
    }
    Emit::config(config_operations)
}

pub mod node_graph_select {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "node-graph-select")]
    pub struct NodeGraphSelect {
        pub node_ids: Vec<String>,
        pub select_all: bool,
    }

    pub fn handle(payload: &NodeGraphSelect, doc: &DocumentView<'_, WorkflowDocument>, cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowOperation, SpaceConfigOperation>, Fault> {
        Ok(select_nodes(payload.node_ids.clone(), payload.select_all, doc.projection, cfg.projection))
    }
}

pub mod set_media_node_selection {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-media-node-selection")]
    pub struct SetMediaNodeSelection {
        pub node_ids: Vec<String>,
        pub select_all: bool,
    }

    pub fn handle(payload: &SetMediaNodeSelection, doc: &DocumentView<'_, WorkflowDocument>, cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowOperation, SpaceConfigOperation>, Fault> {
        Ok(select_nodes(payload.node_ids.clone(), payload.select_all, doc.projection, cfg.projection))
    }
}
//#endregion 🔖️GraphSelection

//#region 🔖️SetAppInstanceSelection
pub mod set_app_instance_selection {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-app-instance-selection")]
    pub struct SetAppInstanceSelection {
        pub node_ids: Vec<String>,
    }

    pub fn handle(payload: &SetAppInstanceSelection, _doc: &DocumentView<'_, WorkflowDocument>, cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowOperation, SpaceConfigOperation>, Fault> {
        let mut config_operations = vec![SpaceConfigOperation::SetSelection { node_ids: payload.node_ids.clone() }];
        if payload.node_ids.len() == 1 {
            config_operations.push(SpaceConfigOperation::SetActiveNode { node_id: payload.node_ids.first().cloned() });
        }
        Ok(Emit::config(config_operations))
    }
}
//#endregion 🔖️SetAppInstanceSelection

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn space_command_op_text_round_trips_every_variant() {
        use crate::apps::space::SpaceCommand;
        store::test_support::assert_op_line_round_trip(&SpaceCommand::SelectInstance(select_instance::SelectInstance { node_id: Some("n1".into()) }));
        store::test_support::assert_op_line_round_trip(&SpaceCommand::NodeGraphSelect(node_graph_select::NodeGraphSelect { node_ids: vec!["n1".into()], select_all: false }));
        store::test_support::assert_op_line_round_trip(&SpaceCommand::SetMediaNodeSelection(set_media_node_selection::SetMediaNodeSelection { node_ids: vec![], select_all: true }));
        store::test_support::assert_op_line_round_trip(&SpaceCommand::SetAppInstanceSelection(set_app_instance_selection::SetAppInstanceSelection { node_ids: vec!["n1".into()] }));
    }

    #[test]
    fn presence_heartbeat_publishes_peer_for_other_clients() {
        use crate::apps::space::testkit::{apply_config, studio_emit, studio_presence_peers_json};
        use crate::apps::space::SpaceCommand;
        use crate::core::demo_space_projection;
        let projection = demo_space_projection();
        let config = SpaceConfig::default();
        let first_node_id = projection.graph.nodes[0].id.clone();
        let select_emit = studio_emit(&projection, &config, &SpaceCommand::NodeGraphSelect(node_graph_select::NodeGraphSelect { node_ids: vec![first_node_id], select_all: false })).expect("handle");
        let config_after_select = apply_config(&config, &select_emit.config_operations);
        let _ = studio_emit(
            &projection,
            &config_after_select,
            &SpaceCommand::PresenceHeartbeat(crate::apps::space::commands::presence::presence_heartbeat::PresenceHeartbeat { client_id: "client-test-a".into(), name: "Ada".into() }),
        )
        .expect("handle");
        let other_config = SpaceConfig { client_id: Some("client-test-b".into()), space_id: config_after_select.space_id.clone(), ..SpaceConfig::default() };
        let peers = studio_presence_peers_json(&other_config);
        assert!(peers.contains("client-test-a"));
        assert!(peers.contains("Ada"));
        assert!(peers.contains(r#""selectionCount":1"#));
        let self_config = SpaceConfig { client_id: Some("client-test-a".into()), ..config_after_select };
        let self_view = studio_presence_peers_json(&self_config);
        assert!(!self_view.contains("client-test-a"));
    }
}
//#endregion 🧪️Tests
