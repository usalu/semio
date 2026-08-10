//! 🔍️ S Studio app — open/close a node's own plugin instance window.

use crate::apps::space::config::{SpaceConfig, SpaceConfigMutation};
use semio_framework_os::{WorkflowSnapshot, WorkflowMutation};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault, HostEffect};

//#region 🔖️OpenInstance
pub mod open_instance {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "open-instance")]
    pub struct OpenInstance {
        pub node_id: Option<String>,
    }

    pub fn handle(payload: &OpenInstance, doc: &ArtifactView<'_, WorkflowSnapshot>, cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
        match payload.node_id.clone().or_else(|| crate::apps::space::primary_selected_node_id(cfg.snapshot)) {
            Some(node_id) => match doc.snapshot.graph.nodes.iter().find(|row| row.id == node_id) {
                Some(node) => Ok(Emit {
                    config_mutations: vec![
                        SpaceConfigMutation::SetFocusedNode { node_id: Some(node_id.clone()) },
                        SpaceConfigMutation::SetActiveNode { node_id: Some(node_id.clone()) },
                        SpaceConfigMutation::SetSelection { node_ids: vec![node_id.clone()] },
                    ],
                    effects: vec![HostEffect::OpenPluginInstance { plugin_id: node.plugin_id.clone(), app_id: node.app_id.clone(), os_instance_id: Some(node.id.clone()) }],
                    ..Default::default()
                }),
                None => Ok(Emit::default()),
            },
            None => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️OpenInstance

//#region 🔖️CloseFocusedInstance
pub mod close_focused_instance {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "close-focused-instance")]
    pub struct CloseFocusedInstance {}

    pub fn handle(_payload: &CloseFocusedInstance, _doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
        Ok(Emit::config(vec![SpaceConfigMutation::SetFocusedNode { node_id: None }]))
    }
}
//#endregion 🔖️CloseFocusedInstance

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::space::testkit::{apply_config, seed_draw_plugin, studio_emit};
    use crate::apps::space::SpaceCommand;
    use crate::demo_space_projection;

    #[test]
    fn space_command_op_text_round_trips_every_variant() {
        store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::OpenInstance(open_instance::OpenInstance { node_id: Some("n1".into()) }));
        store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::CloseFocusedInstance(close_focused_instance::CloseFocusedInstance {}));
    }

    #[test]
    fn open_instance_emits_open_plugin_instance_effect_matching_instance() {
        seed_draw_plugin();
        let projection = demo_space_projection();
        let node = projection.graph.nodes.iter().find(|node| node.plugin_id == "draw").expect("draw node").clone();
        let config = SpaceConfig::default();
        let emit = studio_emit(&projection, &config, &SpaceCommand::OpenInstance(open_instance::OpenInstance { node_id: Some(node.id.clone()) })).expect("handle");
        assert!(emit.artifact_mutations.is_empty(), "opening an instance is a host effect, not a document operation");
        let opened = emit
            .effects
            .iter()
            .find_map(|effect| match effect {
                HostEffect::OpenPluginInstance { plugin_id, app_id, os_instance_id } => Some((plugin_id.clone(), app_id.clone(), os_instance_id.clone())),
                _ => None,
            })
            .expect("OpenPluginInstance effect");
        assert_eq!(opened.0, "draw");
        assert_eq!(opened.1, "draw");
        assert_eq!(opened.2.as_deref(), Some(node.id.as_str()));
    }

    #[test]
    fn open_and_close_focused_instance() {
        let projection = demo_space_projection();
        let config = SpaceConfig::default();
        let node_id = projection.graph.nodes.first().expect("node").id.clone();
        let open_emit = studio_emit(&projection, &config, &SpaceCommand::OpenInstance(open_instance::OpenInstance { node_id: Some(node_id.clone()) })).expect("handle");
        assert!(open_emit.config_mutations.contains(&SpaceConfigMutation::SetFocusedNode { node_id: Some(node_id.clone()) }));
        let config_after_open = apply_config(&config, &open_emit.config_mutations);
        assert_eq!(config_after_open.focused_node_id.as_deref(), Some(node_id.as_str()));
        let close_emit = studio_emit(&projection, &config_after_open, &SpaceCommand::CloseFocusedInstance(close_focused_instance::CloseFocusedInstance {})).expect("handle");
        assert_eq!(close_emit.config_mutations, vec![SpaceConfigMutation::SetFocusedNode { node_id: None }]);
    }
}
//#endregion 🧪️Tests
