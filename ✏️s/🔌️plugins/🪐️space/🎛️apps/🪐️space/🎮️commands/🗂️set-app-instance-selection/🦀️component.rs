//! 🗂️ 🗂️ S Studio app command — `set-app-instance-selection`.

use crate::apps::space::config::{SpaceConfig, SpaceConfigMutation};
use semio_framework_os::{WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "set-app-instance-selection")]
pub struct SetAppInstanceSelection {
    pub node_ids: Vec<String>,
}

pub fn handle(payload: &SetAppInstanceSelection, _doc: &ArtifactView<'_, WorkflowSnapshot>, cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    let mut config_mutations = vec![SpaceConfigMutation::SetSelection { node_ids: payload.node_ids.clone() }];
    if payload.node_ids.len() == 1 {
        config_mutations.push(SpaceConfigMutation::SetActiveNode { node_id: payload.node_ids.first().cloned() });
    }
    Ok(Emit::config(config_mutations))
}
