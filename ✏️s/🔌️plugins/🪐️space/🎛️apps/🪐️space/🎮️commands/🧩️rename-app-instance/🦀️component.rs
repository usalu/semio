//! 🧩️ 🧩️ S Studio app command — `rename-app-instance`.

use crate::apps::space::config::{SpaceConfig, SpaceConfigMutation};
use semio_framework_os::{WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "rename-app-instance")]
pub struct RenameAppInstance {
    pub label: Option<String>,
}

pub fn handle(payload: &RenameAppInstance, doc: &ArtifactView<'_, WorkflowSnapshot>, cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    match crate::apps::space::primary_selected_node_id(cfg.snapshot) {
        Some(node_id) => {
            let next_label = payload.label.clone().or_else(|| doc.snapshot.graph.nodes.iter().find(|row| row.id == node_id).map(|node| format!("{} (renamed)", node.label)));
            match next_label {
                Some(next_label) => Ok(Emit::mutations(vec![WorkflowMutation::PatchNode { node_id, label: next_label }])),
                None => Ok(Emit::default()),
            }
        }
        None => Ok(Emit::default()),
    }
}
