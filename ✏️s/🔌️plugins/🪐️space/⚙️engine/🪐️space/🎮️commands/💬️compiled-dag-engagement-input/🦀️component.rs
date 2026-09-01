//! 💬️ 💬️ S Studio app command — `compiled-dag-engagement-input`.

use crate::engine::space::config::{SpaceConfig, SpaceConfigMutation};
use semio_framework_os::{WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};


#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "compiled-dag-engagement-input")]
pub struct CompiledDagEngagementInput {
    pub value: String,
}

pub async fn handle(payload: &CompiledDagEngagementInput, _doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    Ok(Emit::config(vec![SpaceConfigMutation::SetCompiledDagEngagementInput { value: payload.value.clone() }]))
}
