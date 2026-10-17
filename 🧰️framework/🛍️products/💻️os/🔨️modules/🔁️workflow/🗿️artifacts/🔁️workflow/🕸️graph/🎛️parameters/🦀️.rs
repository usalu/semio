//! 🎛️ Parameters of the workflow graph: parameter ids, types, bindings and patches, port synchronisation and config binding validation. See [`super`].

use super::*;

//#region 🔖️WorkflowParameters
// 🎛️ Ported down from `framework/product/os/core`'s `instance::🔖️Parameters` region (W3 "the
// inversion" — see `## The inversion` in the plan) verbatim except for the `Os` -> `Workflow` rename:
// none of it actually needed the os-core plugin/artifact registry (`validate_workflow_parameter_config_binding`
// takes an already-resolved `ConfigSpec` as a plain argument, it never looks one up itself), so the
// whole region is pure and belongs at this layer. os-core keeps only the registry LOOKUP
// (`os_app_registration(...).config`) that feeds this function's `config_spec` argument.
pub const WORKFLOW_PARAMETER_PORT_PREFIX: &str = "param.";

static WORKFLOW_PARAMETER_ID: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// 🆔️ Fallback id minter for a parameter created without a caller-supplied id — every real
/// caller (`os-core`'s `OsWorkflowStore::add_parameter`) supplies one via its own id minter instead, so
/// this counter is scoped independently to this crate (not the same sequence as os-core's `create_os_id`).
// 🚫️async: E1 transitive — consumed by std Iterator/Option combinators (external traits) in
// sync closures; pure, no I/O (R9).
fn create_workflow_parameter_id() -> String {
    let n = WORKFLOW_PARAMETER_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
    format!("param-{n}")
}

#[derive(Clone, Debug, PartialEq, Eq, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue)]
#[value(rename_all = "lowercase")]
pub enum WorkflowParameterType {
    Numeric,
    Categorical,
    Toggle,
    Text,
}

/// 🎯️ `field_path` names a field `id` in the target node's app's declared `ConfigSpec` —
/// see `validate_workflow_parameter_config_binding` (type-checks against the field's `ArgSchema`).
#[derive(Clone, Debug, PartialEq, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct WorkflowParameterBinding {
    pub parameter_id: String,
    pub node_id: String,
    pub field_path: String,
}

#[derive(Clone, Debug, PartialEq, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "type", rename_all = "lowercase")]
pub enum WorkflowParameter {
    Numeric { id: String, name: String, value: f64, min: Option<f64>, max: Option<f64>, step: Option<f64> },
    Categorical { id: String, name: String, value: String, options: Vec<String> },
    Toggle { id: String, name: String, value: bool },
    Text { id: String, name: String, value: String },
}

pub type WorkflowParameterPatch = semio_framework_value::DslValue;

// 🚫️async: E1 transitive — consumed by std Iterator/Option combinators (external traits) in
// sync closures; pure, no I/O (R9).
pub fn workflow_parameter_id(parameter: &WorkflowParameter) -> &str {
    match parameter {
        WorkflowParameter::Numeric { id, .. } | WorkflowParameter::Categorical { id, .. } | WorkflowParameter::Toggle { id, .. } | WorkflowParameter::Text { id, .. } => id,
    }
}

// 🚫️async: E1 transitive — consumed by std Iterator/Option combinators (external traits) in
// sync closures; pure, no I/O (R9).
pub fn workflow_parameter_name(parameter: &WorkflowParameter) -> String {
    match parameter {
        WorkflowParameter::Numeric { name, .. } | WorkflowParameter::Categorical { name, .. } | WorkflowParameter::Toggle { name, .. } | WorkflowParameter::Text { name, .. } => name.clone(),
    }
}

// 🚫️async: E1 transitive — consumed by std Iterator/Option combinators (external traits) in
// sync closures; pure, no I/O (R9).
pub fn workflow_parameter_value(parameter: &WorkflowParameter) -> semio_framework_value::DslValue {
    match parameter {
        WorkflowParameter::Numeric { value, .. } => semio_framework_value::ToValue::to_value(value),
        WorkflowParameter::Categorical { value, .. } => semio_framework_value::ToValue::to_value(value),
        WorkflowParameter::Toggle { value, .. } => semio_framework_value::ToValue::to_value(value),
        WorkflowParameter::Text { value, .. } => semio_framework_value::ToValue::to_value(value),
    }
}

pub async fn workflow_parameter_types_compatible(left: &WorkflowParameterType, right: &WorkflowParameterType) -> bool {
    left == right
}

pub async fn create_default_workflow_parameter(parameter_type: &WorkflowParameterType, name: &str, id: Option<&str>) -> WorkflowParameter {
    let parameter_id = id.map_or_else(create_workflow_parameter_id, str::to_string);
    match parameter_type {
        WorkflowParameterType::Numeric => WorkflowParameter::Numeric { id: parameter_id, name: name.into(), value: 0.0, min: Some(0.0), max: Some(100.0), step: Some(1.0) },
        WorkflowParameterType::Categorical => WorkflowParameter::Categorical { id: parameter_id, name: name.into(), value: "Option A".into(), options: vec!["Option A".into(), "Option B".into()] },
        WorkflowParameterType::Toggle => WorkflowParameter::Toggle { id: parameter_id, name: name.into(), value: false },
        WorkflowParameterType::Text => WorkflowParameter::Text { id: parameter_id, name: name.into(), value: String::new() },
    }
}

async fn clamp_workflow_numeric_value(value: f64, min: Option<f64>, max: Option<f64>, step: Option<f64>) -> f64 {
    let mut next = value;
    if let Some(min) = min.filter(|v| v.is_finite()) {
        next = next.max(min);
    }
    if let Some(max) = max.filter(|v| v.is_finite()) {
        next = next.min(max);
    }
    if let Some(step) = step.filter(|v| v.is_finite() && *v > 0.0) {
        let anchor = min.filter(|v| v.is_finite()).unwrap_or(0.0);
        next = anchor + ((next - anchor) / step).round() * step;
        if let Some(min) = min.filter(|v| v.is_finite()) {
            next = next.max(min);
        }
        if let Some(max) = max.filter(|v| v.is_finite()) {
            next = next.min(max);
        }
    }
    next
}

/// 🎛️ Applies a partial patch to a workflow parameter, enforcing type constraints. Ported
/// verbatim from os-core's `patch_os_parameter`.
pub async fn patch_workflow_parameter(parameter: &WorkflowParameter, patch: &semio_framework_value::DslValue) -> WorkflowParameter {
    let name = patch.get("name").and_then(|v| v.as_str()).map_or_else(|| workflow_parameter_name(parameter), str::to_string);
    let patch_type = patch.get("type").and_then(|v| v.as_str());
    let use_numeric = patch_type == Some("numeric") || (patch_type.is_none() && matches!(parameter, WorkflowParameter::Numeric { .. }));
    if use_numeric {
        let current = match parameter {
            WorkflowParameter::Numeric { .. } => parameter.clone(),
            _ => create_default_workflow_parameter(&WorkflowParameterType::Numeric, &name, Some(workflow_parameter_id(parameter))).await,
        };
        if let WorkflowParameter::Numeric { id, min: current_min, max: current_max, step: current_step, value: current_value, .. } = current {
            let min = patch.get("min").and_then(|v| v.as_f64()).or(current_min);
            let max = patch.get("max").and_then(|v| v.as_f64()).or(current_max);
            let step = patch.get("step").and_then(|v| v.as_f64()).or(current_step);
            let raw_value = patch.get("value").and_then(|v| v.as_f64()).unwrap_or(current_value);
            return WorkflowParameter::Numeric { id, name, min, max, step, value: clamp_workflow_numeric_value(raw_value, min, max, step).await };
        }
    }
    let use_categorical = patch_type == Some("categorical") || (patch_type.is_none() && matches!(parameter, WorkflowParameter::Categorical { .. }));
    if use_categorical {
        let current = match parameter {
            WorkflowParameter::Categorical { .. } => parameter.clone(),
            _ => create_default_workflow_parameter(&WorkflowParameterType::Categorical, &name, Some(workflow_parameter_id(parameter))).await,
        };
        if let WorkflowParameter::Categorical { id, value: current_value, options: current_options, .. } = current {
            let options = patch.get("options").and_then(|v| v.as_array()).map_or(current_options, |entries| entries.iter().filter_map(|entry| entry.as_str().map(str::to_string)).collect::<Vec<_>>());
            let unique_options = if options.is_empty() { vec!["Option A".into()] } else { options };
            let value = patch
                .get("value")
                .and_then(|v| v.as_str())
                .filter(|v| unique_options.iter().any(|option| option == *v))
                .map(str::to_string)
                .or_else(|| unique_options.iter().find(|option| **option == current_value).cloned())
                .unwrap_or_else(|| unique_options[0].clone());
            return WorkflowParameter::Categorical { id, name, options: unique_options, value };
        }
    }
    if patch_type == Some("toggle") || (patch_type.is_none() && matches!(parameter, WorkflowParameter::Toggle { .. })) {
        let current = match parameter {
            WorkflowParameter::Toggle { .. } => parameter.clone(),
            _ => create_default_workflow_parameter(&WorkflowParameterType::Toggle, &name, Some(workflow_parameter_id(parameter))).await,
        };
        if let WorkflowParameter::Toggle { id, value: current_value, .. } = current {
            let value = patch.get("value").and_then(|v| v.as_bool()).unwrap_or(current_value);
            return WorkflowParameter::Toggle { id, name, value };
        }
    }
    let current = match parameter {
        WorkflowParameter::Text { .. } => parameter.clone(),
        _ => create_default_workflow_parameter(&WorkflowParameterType::Text, &name, Some(workflow_parameter_id(parameter))).await,
    };
    if let WorkflowParameter::Text { id, value: current_value, .. } = current {
        let value = patch.get("value").and_then(|v| v.as_str()).map_or(current_value, str::to_string);
        return WorkflowParameter::Text { id, name, value };
    }
    parameter.clone()
}

/// ✅️ Type-checks one binding's `field_path` against the target app's declared `ConfigSpec` —
/// `config_spec` is caller-resolved (os-core looks it up via `os_app_registration`), so this function
/// itself needs no registry. Ported verbatim from os-core's `validate_parameter_config_binding`.
/// Returns a `protocol::MutationMessage` rather than the deleted free-form `{kind, uri, message}`
/// diagnostic struct (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` §C10) —
/// `Warning`, not
/// `Error`/`Fatal`, because this isn't a rejected `Mutation::diff` (the two Laws binding those levels
/// to an empty/unchanged diff don't apply here): it's `reconcile_workflow_snapshot`'s post-merge
/// integrity pass, called only to decide whether a stale binding gets corrective-dropped, and the
/// drop always happens regardless of this message's level. The domain code
/// (`workflow/parameter-binding-invalid`) is intentionally outside the frozen nine `mutation.*` codes —
/// those govern `diff`-leaf outcomes only, not this reconcile-pass diagnostic (see
/// `🏪️store/🔄️sync/🦀️.rs`'s `ArtifactEvent::Conflict(MutationMessage)` for the identical
/// precedent).
pub async fn validate_workflow_parameter_config_binding(binding: &WorkflowParameterBinding, parameter_type: &WorkflowParameterType, config_spec: &semio_framework::ConfigSpec) -> Result<(), protocol::MutationMessage> {
    let uri = format!("{}#{}", binding.node_id, binding.field_path);
    let Some(field) = config_spec.fields.iter().find(|field| field.id == binding.field_path) else {
        return Err(protocol::MutationMessage {
            level: semio_framework_diagnostic::Severity::Warning,
            code: semio_framework_diagnostic::FaultCode::new("workflow/parameter-binding-invalid"),
            message: format!("binding targets config field '{}', which the app's ConfigSpec does not declare", binding.field_path),
            target: vec![uri],
            op_index: None,
        });
    };
    let compatible = match (parameter_type, &field.schema) {
        (WorkflowParameterType::Numeric, semio_framework::ArgSchema::Number { .. }) | (WorkflowParameterType::Toggle, semio_framework::ArgSchema::Boolean) => true,
        (WorkflowParameterType::Categorical, semio_framework::ArgSchema::String { options, .. }) => !options.is_empty(),
        (WorkflowParameterType::Text, semio_framework::ArgSchema::String { options, .. }) => options.is_empty(),
        _ => false,
    };
    if compatible {
        Ok(())
    } else {
        Err(protocol::MutationMessage {
            level: semio_framework_diagnostic::Severity::Warning,
            code: semio_framework_diagnostic::FaultCode::new("workflow/parameter-binding-invalid"),
            message: format!("parameter type {parameter_type:?} cannot drive config field '{}' ({:?})", binding.field_path, field.schema),
            target: vec![uri],
            op_index: None,
        })
    }
}

// 🚫️async: E1 transitive — consumed by std Iterator/Option combinators (external traits) in
// sync closures; pure, no I/O (R9).
pub fn workflow_parameter_port_id(node_id: &str, parameter_id: &str) -> String {
    media_port_id_for_spec(node_id, &format!("{WORKFLOW_PARAMETER_PORT_PREFIX}{parameter_id}"), "in")
}

// 🚫️async: E1 transitive — consumed by std Iterator/Option combinators (external traits) in
// sync closures; pure, no I/O (R9).
pub fn is_workflow_parameter_port_id(port_id: &str) -> bool {
    media_port_spec_id(port_id).is_some_and(|spec_id| spec_id.starts_with(WORKFLOW_PARAMETER_PORT_PREFIX))
}

// 🚫️async: E1 transitive — consumed by std Iterator/Option combinators (external traits) in
// sync closures; pure, no I/O (R9).
pub fn workflow_parameter_id_from_port_id(port_id: &str) -> Option<String> {
    let spec_id = media_port_spec_id(port_id)?;
    spec_id.strip_prefix(WORKFLOW_PARAMETER_PORT_PREFIX).map(str::to_string)
}

// 🚫️async: E1 transitive — consumed by std Iterator/Option combinators (external traits) in
// sync closures; pure, no I/O (R9).
pub fn media_port_id_for_spec(instance_id: &str, spec_id: &str, direction: &str) -> String {
    format!("{instance_id}:{spec_id}:{direction}")
}

// 🚫️async: E1 transitive — consumed by std Iterator/Option combinators (external traits) in
// sync closures; pure, no I/O (R9).
pub fn media_port_spec_id(port_id: &str) -> Option<String> {
    let parts: Vec<_> = port_id.split(':').collect();
    if parts.len() < 3 {
        return None;
    }
    Some(parts[1..parts.len() - 1].join(":"))
}

/// 🎛️ Resyncs one node's parameter-bound input ports from `bindings` — every non-parameter port is
/// kept as-is, every parameter port is rebuilt from scratch (idempotent: bind/unbind/patch all funnel
/// through this). Ported verbatim from os-core's `sync_workflow_node_parameter_ports`.
// 🚫️async: E1 transitive — consumed by std Iterator::map (external trait) in a sync closure;
// pure node reshape, no I/O (R9).
pub fn sync_workflow_node_parameter_ports(node: &WorkflowNode, bindings: &[WorkflowParameterBinding]) -> WorkflowNode {
    let node_bindings: Vec<_> = bindings.iter().filter(|binding| binding.node_id == node.id).collect();
    let base_inputs: Vec<_> = node.inputs.iter().filter(|port| !is_workflow_parameter_port_id(&port.id)).cloned().collect();
    let parameter_inputs: Vec<_> = node_bindings
        .iter()
        .map(|binding| WorkflowMediaPort {
            id: workflow_parameter_port_id(&node.id, &binding.parameter_id),
            spec: MediaPortSpec {
                id: format!("{WORKFLOW_PARAMETER_PORT_PREFIX}{}", binding.parameter_id),
                label: "Parameter".into(),
                direction: MediaPortDirection::In,
                media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
                kind_id: Some("parameter.value".into()),
                required: false,
                multiplicity: PortMultiplicity::One,
            },
        })
        .collect();
    let inputs: Vec<_> = base_inputs.into_iter().chain(parameter_inputs).collect();
    let port_count = inputs.len().max(node.outputs.len()).max(1);
    WorkflowNode { inputs, height: 56.0 + port_count as f64 * 18.0, ..node.clone() }
}

pub fn sync_workflow_parameter_ports(graph: &Workflow, bindings: &[WorkflowParameterBinding]) -> Workflow {
    Workflow { schema: WORKFLOW_SCHEMA.into(), nodes: graph.nodes.iter().map(|node| sync_workflow_node_parameter_ports(node, bindings)).collect(), edges: graph.edges.clone() }
}
//#endregion 🔖️WorkflowParameters
