//! 🗂️ 🗂️ S Studio app command — `select-instance`.

use crate::apps::space::config::{SpaceConfig, SpaceConfigMutation};
use semio_framework_os::{WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "select-instance")]
pub struct SelectInstance {
    pub node_id: Option<String>,
}

pub fn handle(payload: &SelectInstance, _doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    let node_ids = payload.node_id.iter().cloned().collect();
    Ok(Emit::config(vec![SpaceConfigMutation::SetActiveNode { node_id: payload.node_id.clone() }, SpaceConfigMutation::SetSelection { node_ids }]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn space_command_op_text_round_trips_every_variant() {
        use crate::apps::space::SpaceCommand;
        store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::SelectInstance(SelectInstance { node_id: Some("n1".into()) }));
        store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::NodeGraphSelect(crate::apps::space::commands::node_graph_select::NodeGraphSelect { node_ids: vec!["n1".into()], select_all: false }));
        store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::SetMediaNodeSelection(crate::apps::space::commands::set_media_node_selection::SetMediaNodeSelection { node_ids: vec![], select_all: true }));
        store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::SetAppInstanceSelection(crate::apps::space::commands::set_app_instance_selection::SetAppInstanceSelection { node_ids: vec!["n1".into()] }));
    }

    #[test]
    fn presence_heartbeat_publishes_peer_for_other_clients() {
        use crate::apps::space::testkit::{apply_config, studio_emit, studio_presence_peers_json};
        use crate::apps::space::SpaceCommand;
        use crate::demo_space_projection;
        let projection = demo_space_projection();
        let config = SpaceConfig::default();
        let first_node_id = projection.graph.nodes[0].id.clone();
        let select_emit = studio_emit(&projection, &config, &SpaceCommand::NodeGraphSelect(crate::apps::space::commands::node_graph_select::NodeGraphSelect { node_ids: vec![first_node_id], select_all: false })).expect("handle");
        let config_after_select = apply_config(&config, &select_emit.config_mutations);
        let _ = studio_emit(&projection, &config_after_select, &SpaceCommand::PresenceHeartbeat(crate::apps::space::commands::presence_heartbeat::PresenceHeartbeat { client_id: "client-test-a".into(), name: "Ada".into() })).expect("handle");
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
