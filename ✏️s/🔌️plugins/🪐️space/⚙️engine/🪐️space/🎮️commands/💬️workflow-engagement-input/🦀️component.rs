//! 💬️ 💬️ S Studio app command — `workflow-engagement-input`.

use crate::engine::space::config::{SpaceConfig, SpaceConfigMutation};
use semio_framework_os::{WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "workflow-engagement-input")]
pub struct WorkflowEngagementInput {
    pub value: String,
}

pub async fn handle(payload: &WorkflowEngagementInput, _doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    Ok(Emit::config(vec![SpaceConfigMutation::SetWorkflowEngagementInput { value: payload.value.clone() }]))
}
