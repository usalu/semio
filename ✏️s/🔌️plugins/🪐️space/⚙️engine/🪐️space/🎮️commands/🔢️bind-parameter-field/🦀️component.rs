//! 🔢️ 🔢️ S Studio app command — `bind-parameter-field`.

use crate::engine::space::config::{SpaceConfig, SpaceConfigMutation};
use semio_framework_os::{WorkflowMutation, WorkflowParameterBinding, WorkflowSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "bind-parameter-field")]
pub struct BindParameterField {
    pub node_id: String,
    pub field_path: String,
    pub parameter_id: String,
}

pub fn handle(payload: &BindParameterField, _doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    if payload.parameter_id.is_empty() || payload.parameter_id == "__direct__" {
        Ok(Emit::mutations(vec![WorkflowMutation::UnbindParameterField { node_id: payload.node_id.clone(), field_path: payload.field_path.clone() }]))
    } else {
        Ok(Emit::mutations(vec![WorkflowMutation::BindParameterField { binding: WorkflowParameterBinding { parameter_id: payload.parameter_id.clone(), node_id: payload.node_id.clone(), field_path: payload.field_path.clone() } }]))
    }
}
