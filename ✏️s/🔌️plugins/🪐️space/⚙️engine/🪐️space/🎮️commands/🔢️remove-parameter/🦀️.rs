//! 🔢️ 🔢️ S Studio app command — `remove-parameter`.

use crate::engine::space::config::{SpaceConfig, SpaceConfigMutation};
use semio_framework_os::workflow::RemoveParameter as RemoveParameterMutation;
use semio_framework_os::{WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};


#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "remove-parameter")]
pub struct RemoveParameter {
    pub parameter_id: String,
}

pub fn handle(payload: &RemoveParameter, _doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![WorkflowMutation::RemoveParameter(RemoveParameterMutation { parameter_id: payload.parameter_id.clone() })]))
}
