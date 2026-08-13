//! 🔢️ 🔢️ S Studio app command — `remove-parameter`.

use crate::apps::space::config::{SpaceConfig, SpaceConfigMutation};
use semio_framework_os::{WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "remove-parameter")]
pub struct RemoveParameter {
    pub parameter_id: String,
}

pub fn handle(payload: &RemoveParameter, _doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![WorkflowMutation::RemoveParameter { parameter_id: payload.parameter_id.clone() }]))
}
