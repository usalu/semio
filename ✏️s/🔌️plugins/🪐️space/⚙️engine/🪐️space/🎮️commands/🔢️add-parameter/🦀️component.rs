//! 🔢️ 🔢️ S Studio app command — `add-parameter`.

use crate::engine::space::config::{SpaceConfig, SpaceConfigMutation};
use semio_framework_os::{WorkflowMutation, WorkflowParameterType, WorkflowSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};


#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "add-parameter")]
pub struct AddParameter {
    pub name: String,
    pub kind: String,
}

pub async fn handle(payload: &AddParameter, _doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    let parameter_type = match payload.kind.as_str() {
        "categorical" => WorkflowParameterType::Categorical,
        "toggle" => WorkflowParameterType::Toggle,
        "text" => WorkflowParameterType::Text,
        _ => WorkflowParameterType::Numeric,
    };
    Ok(Emit::mutations(vec![crate::engine::space::engine::add_parameter_operation(&parameter_type, &payload.name)]))
}
