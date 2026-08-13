//! 🧩️ 🧩️ S Studio app command — `remove-app-instance`.

use crate::apps::space::config::{SpaceConfig, SpaceConfigMutation};
use semio_framework_os::{WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "remove-app-instance")]
pub struct RemoveAppInstance {
    pub node_id: Option<String>,
}

pub fn handle(payload: &RemoveAppInstance, _doc: &ArtifactView<'_, WorkflowSnapshot>, cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    let config = cfg.snapshot;
    match payload.node_id.clone().or_else(|| crate::apps::space::primary_selected_node_id(config)) {
        Some(node_id) => {
            let mut config_mutations = Vec::new();
            if config.active_node_id.as_deref() == Some(node_id.as_str()) {
                config_mutations.push(SpaceConfigMutation::SetActiveNode { node_id: None });
            }
            if config.focused_node_id.as_deref() == Some(node_id.as_str()) {
                config_mutations.push(SpaceConfigMutation::SetFocusedNode { node_id: None });
            }
            Ok(Emit { artifact_mutations: vec![WorkflowMutation::RemoveNode { node_id }], config_mutations, ..Default::default() })
        }
        None => Ok(Emit::default()),
    }
}
