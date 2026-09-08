//! 🔢️ 🔢️ S Studio app command — `patch-parameter`.

use crate::engine::space::config::{SpaceConfig, SpaceConfigMutation};
use pack::json::{self, Value};
use semio_framework_os::{WorkflowMutation, WorkflowParameter, WorkflowSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

use crate::engine::space::engine::parameter_entity_id;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "patch-parameter")]
pub struct PatchParameter {
    pub parameter_id: String,
    pub field: String,
    pub value: String,
}

pub fn handle(payload: &PatchParameter, doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    let projection = doc.snapshot;
    let value_json: Value = json::parse(&payload.value).unwrap_or_else(|_| Value::String(payload.value.clone()));
    let mut current = None;
    for entry in &projection.parameters {
        if crate::engine::space::engine::resolve_future(parameter_entity_id(entry)) == payload.parameter_id {
            current = Some(entry);
            break;
        }
    }
    let patch = if payload.field == "addOption" {
        value_json.as_str().map(str::to_string).and_then(|option| {
            current.and_then(|entry| match entry {
                WorkflowParameter::Categorical { options, .. } => {
                    let mut next_options = options.clone();
                    if !next_options.iter().any(|row| row == &option) {
                        next_options.push(option.clone());
                    }
                    Some(json::object([("options".to_string(), json::array(next_options.into_iter().map(Value::String))), ("value".to_string(), Value::String(option))]))
                }
                _ => None,
            })
        })
    } else if payload.field == "removeOption" {
        value_json.as_str().map(str::to_string).and_then(|option| {
            current.and_then(|entry| match entry {
                WorkflowParameter::Categorical { options, value, .. } => {
                    let next_options: Vec<_> = options.iter().filter(|row| row.as_str() != option).cloned().collect();
                    let next_value = if next_options.iter().any(|row| row.as_str() == value.as_str()) { value.clone() } else { next_options.first().cloned().unwrap_or_default() };
                    Some(json::object([("options".to_string(), json::array(next_options.into_iter().map(Value::String))), ("value".to_string(), Value::String(next_value))]))
                }
                _ => None,
            })
        })
    } else {
        Some(json::object([(payload.field.clone(), value_json)]))
    };
    match patch {
        Some(patch) => match crate::engine::space::engine::resolve_future(crate::engine::space::engine::patch_parameter_operation(projection, &payload.parameter_id, &patch)) {
            Some(operation) => Ok(Emit::mutations(vec![operation])),
            None => Ok(Emit::default()),
        },
        None => Ok(Emit::default()),
    }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
