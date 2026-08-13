//! 🧩️ 🧩️ S Studio app command — `delete-selection`.

use crate::apps::space::config::{SpaceConfig, SpaceConfigMutation};
use semio_framework_os::{WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "delete-selection")]
pub struct DeleteSelection {}

pub fn handle(_payload: &DeleteSelection, _doc: &ArtifactView<'_, WorkflowSnapshot>, cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    let config = cfg.snapshot;
    let artifact_mutations = config.selected_node_ids.iter().cloned().map(|node_id| WorkflowMutation::RemoveNode { node_id }).collect();
    Ok(Emit { artifact_mutations, config_mutations: vec![SpaceConfigMutation::SetSelection { node_ids: Vec::new() }, SpaceConfigMutation::SetActiveNode { node_id: None }, SpaceConfigMutation::SetFocusedNode { node_id: None }], ..Default::default() })
}
