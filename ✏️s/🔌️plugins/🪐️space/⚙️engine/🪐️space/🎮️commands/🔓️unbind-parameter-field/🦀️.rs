//! 🔢️ 🔢️ S Studio app command — `unbind-parameter-field`.

use crate::engine::space::config::{SpaceConfig, SpaceConfigMutation};
use semio_framework_os::workflow::UnbindParameterField as UnbindParameterFieldMutation;
use semio_framework_os::{WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};


#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "unbind-parameter-field")]
pub struct UnbindParameterField {
    pub node_id: String,
    pub field_path: String,
}

pub fn handle(payload: &UnbindParameterField, _doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![WorkflowMutation::UnbindParameterField(UnbindParameterFieldMutation { node_id: payload.node_id.clone(), field_path: payload.field_path.clone() })]))
}
