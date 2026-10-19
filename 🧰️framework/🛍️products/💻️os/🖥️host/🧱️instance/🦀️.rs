//! 📦️ App instance schemas, parameters, and studio bindings.
// #region instance

use semio_framework_artifact_workflow_workflow::WorkflowNode;
use semio_framework::ConfigSpec;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicU64, Ordering};

pub const OS_PARAMETER_PORT_PREFIX: &str = "param.";

//#region 🔖️Schemas
/// 🔗️ Handle to an app's own `framework/sync`-hosted vcs document — the os document never
/// embeds app content, only this reference (mirrors `framework/sync`'s `ArtifactActorConfig`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, semio_framework_dsl_record_derive::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct OsArtifactRef {
    pub document_id: String,
    pub schema: String,
}

/// 🆔️ Mints a fresh app document id — uuid-v7 (time-ordered), matching the id shape semio_hub already
/// uses for its own entities (`framework/product/os/semio_hub/rs/bin.rs`'s `Uuid::now_v7()`).
pub fn create_os_artifact_id() -> String {
    semio_framework_os_kernel::os_identity::time_ordered_id()
}

// 🧷️ `OsAppInstance` is deleted — `workflow::WorkflowNode` (kernel crate) absorbs it entirely;
// `WorkflowNode.id` IS the app-instance identity now (see the kernel crate's `🔖️InstanceIdentity`
// region doc). `OsArtifactRef` stays (still used generically by `ArtifactHost`'s document-open
// sequence), just no longer nested inside a per-instance record here.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsInstanceState {
    pub id: u32,
    pub app_id: String,
    pub controller_id: String,
    pub document_json: String,
    pub view_state: semio_framework::ViewModel,
    pub generation: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OsParameterType {
    Numeric,
    Categorical,
    Toggle,
    Text,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsParameterFieldSpec {
    pub field_path: String,
    pub label: String,
    #[serde(rename = "type")]
    pub parameter_type: OsParameterType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_dsl_record_derive::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct OsParameterFieldBinding {
    pub parameter_id: String,
    pub node_id: String,
    /// 🎯️ Names a field `id` in the target `node`'s app's declared `ConfigSpec`
    /// (resolved via `registry::os_app_registration(node.plugin_id, node.app_id).config`) — see
    /// `build_configure_config` (overlays the bound parameter's value onto that config field for
    /// an `AppCommand::Configure` payload; the analogous type-check against the field's `ArgSchema`
    /// this field never had its own live caller for lives on the real workflow-graph binding type,
    /// `workflow::validate_workflow_parameter_config_binding`). Historically a JSON pointer into
    /// the node's live document (`apply_parameter_values_to_snapshot`'s still-live overlay,
    /// used only by the media-export path today) — that document-snapshot sense is now
    /// superseded by the config-field sense for anything driving a running app instance.
    pub field_path: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_dsl_record_derive::DslEnum)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum OsParameter {
    Numeric {
        id: String,
        name: String,
        value: f64,
        #[serde(skip_serializing_if = "Option::is_none")]
        min: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        max: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        step: Option<f64>,
    },
    Categorical {
        id: String,
        name: String,
        value: String,
        options: Vec<String>,
    },
    Toggle {
        id: String,
        name: String,
        value: bool,
    },
    Text {
        id: String,
        name: String,
        value: String,
    },
}
//#endregion 🔖️Schemas

//#region 🔖️Parameters
static OS_ID: AtomicU64 = AtomicU64::new(0);

/// 🆔️ Allocates stable ids for OS studio entities.
pub fn create_os_id(prefix: &str) -> String {
    let n = OS_ID.fetch_add(1, Ordering::Relaxed) + 1;
    format!("{prefix}-{n}")
}

/// 🎛️ Reads the runtime value from a space parameter definition.
pub fn os_parameter_value(parameter: &OsParameter) -> Value {
    match parameter {
        OsParameter::Numeric { value, .. } => Value::from(*value),
        OsParameter::Categorical { value, .. } => Value::from(value.clone()),
        OsParameter::Toggle { value, .. } => Value::from(*value),
        OsParameter::Text { value, .. } => Value::from(value.clone()),
    }
}

/// 🎛️ Returns whether a parameter type can drive a bindable field type.
pub fn os_parameter_types_compatible(left: &OsParameterType, right: &OsParameterType) -> bool {
    left == right
}

/// 🎛️ Creates a default space parameter of the given type.
pub fn create_default_os_parameter(parameter_type: &OsParameterType, name: &str, id: Option<&str>) -> OsParameter {
    let parameter_id = id.map_or_else(|| create_os_id("param"), str::to_string);
    match parameter_type {
        OsParameterType::Numeric => OsParameter::Numeric { id: parameter_id, name: name.into(), value: 0.0, min: Some(0.0), max: Some(100.0), step: Some(1.0) },
        OsParameterType::Categorical => OsParameter::Categorical { id: parameter_id, name: name.into(), value: "Option A".into(), options: vec!["Option A".into(), "Option B".into()] },
        OsParameterType::Toggle => OsParameter::Toggle { id: parameter_id, name: name.into(), value: false },
        OsParameterType::Text => OsParameter::Text { id: parameter_id, name: name.into(), value: String::new() },
    }
}

fn clamp_numeric_value(value: f64, min: Option<f64>, max: Option<f64>, step: Option<f64>) -> f64 {
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

/// 🎛️ Applies a partial patch to a space parameter, enforcing type constraints.
pub fn patch_os_parameter(parameter: &OsParameter, patch: &Value) -> OsParameter {
    let name = patch.get("name").and_then(|v| v.as_str()).map_or_else(|| parameter_name(parameter), str::to_string);
    let patch_type = patch.get("type").and_then(|v| v.as_str());
    let use_numeric = patch_type == Some("numeric") || (patch_type.is_none() && matches!(parameter, OsParameter::Numeric { .. }));
    if use_numeric {
        let current = match parameter {
            OsParameter::Numeric { .. } => parameter.clone(),
            _ => create_default_os_parameter(&OsParameterType::Numeric, &name, Some(parameter_id(parameter))),
        };
        if let OsParameter::Numeric { id, min: current_min, max: current_max, step: current_step, value: current_value, .. } = current {
            let min = patch.get("min").and_then(|v| v.as_f64()).or(current_min);
            let max = patch.get("max").and_then(|v| v.as_f64()).or(current_max);
            let step = patch.get("step").and_then(|v| v.as_f64()).or(current_step);
            let raw_value = patch.get("value").and_then(|v| v.as_f64()).unwrap_or(current_value);
            return OsParameter::Numeric { id, name, min, max, step, value: clamp_numeric_value(raw_value, min, max, step) };
        }
    }
    let use_categorical = patch_type == Some("categorical") || (patch_type.is_none() && matches!(parameter, OsParameter::Categorical { .. }));
    if use_categorical {
        let current = match parameter {
            OsParameter::Categorical { .. } => parameter.clone(),
            _ => create_default_os_parameter(&OsParameterType::Categorical, &name, Some(parameter_id(parameter))),
        };
        if let OsParameter::Categorical { id, value: current_value, options: current_options, .. } = current {
            let options = patch.get("options").and_then(|v| v.as_array()).map_or(current_options, |entries| entries.iter().filter_map(|entry| entry.as_str().map(str::to_string)).collect::<Vec<_>>());
            let unique_options = if options.is_empty() { vec!["Option A".into()] } else { options };
            let value = patch
                .get("value")
                .and_then(|v| v.as_str())
                .filter(|v| unique_options.iter().any(|option| option == *v))
                .map(str::to_string)
                .or_else(|| unique_options.iter().find(|option| **option == current_value).cloned())
                .unwrap_or_else(|| unique_options[0].clone());
            return OsParameter::Categorical { id, name, options: unique_options, value };
        }
    }
    if patch_type == Some("toggle") || (patch_type.is_none() && matches!(parameter, OsParameter::Toggle { .. })) {
        let current = match parameter {
            OsParameter::Toggle { .. } => parameter.clone(),
            _ => create_default_os_parameter(&OsParameterType::Toggle, &name, Some(parameter_id(parameter))),
        };
        if let OsParameter::Toggle { id, value: current_value, .. } = current {
            let value = patch.get("value").and_then(|v| v.as_bool()).unwrap_or(current_value);
            return OsParameter::Toggle { id, name, value };
        }
    }
    let current = match parameter {
        OsParameter::Text { .. } => parameter.clone(),
        _ => create_default_os_parameter(&OsParameterType::Text, &name, Some(parameter_id(parameter))),
    };
    if let OsParameter::Text { id, value: current_value, .. } = current {
        let value = patch.get("value").and_then(|v| v.as_str()).map_or(current_value, str::to_string);
        return OsParameter::Text { id, name, value };
    }
    parameter.clone()
}

fn parameter_id(parameter: &OsParameter) -> &str {
    match parameter {
        OsParameter::Numeric { id, .. } | OsParameter::Categorical { id, .. } | OsParameter::Toggle { id, .. } | OsParameter::Text { id, .. } => id,
    }
}

fn parameter_name(parameter: &OsParameter) -> String {
    match parameter {
        OsParameter::Numeric { name, .. } | OsParameter::Categorical { name, .. } | OsParameter::Toggle { name, .. } | OsParameter::Text { name, .. } => name.clone(),
    }
}

fn json_pointer_segments(pointer: &str) -> Vec<String> {
    if let Some(rest) = pointer.strip_prefix('/') {
        rest.split('/').filter(|segment| !segment.is_empty()).map(str::to_string).collect()
    } else {
        pointer.split('.').filter(|segment| !segment.is_empty()).map(str::to_string).collect()
    }
}

/// 🎛️ Deep-sets a JSON-pointer path on a plain object snapshot.
pub fn set_json_pointer_value(root: &mut Value, pointer: &str, value: Value) {
    let segments = json_pointer_segments(pointer);
    if segments.is_empty() {
        return;
    }
    let mut current = root;
    for segment in &segments[..segments.len() - 1] {
        if !current.is_object() {
            *current = Value::Object(Default::default());
        }
        let object = current.as_object_mut().expect("current is always an object here");
        let entry = object.entry(segment.clone()).or_insert_with(|| Value::Object(Default::default()));
        if !entry.is_object() {
            *entry = Value::Object(Default::default());
        }
        current = entry;
    }
    if let Some(object) = current.as_object_mut() {
        object.insert(segments.last().cloned().unwrap_or_default(), value);
    }
}

/// 🎛️ Applies bound space parameter values onto an app snapshot via JSON pointers. 🩹️
/// Pre-`ConfigSpec` document-snapshot overlay, kept for its one remaining live caller
/// (`app_instance_document_patches_for_binding`, the media-export path's synthetic-document seed)
/// — `field_path` here is still read as a JSON pointer into that bare document, distinct from the
/// config field `id` sense `build_configure_config` gives it for driving a running app
/// instance's config (see `OsParameterFieldBinding::field_path`'s doc).
pub fn apply_parameter_values_to_snapshot(snapshot: Value, bindings: &[OsParameterFieldBinding], parameters: &[OsParameter], node_id: &str) -> Value {
    let node_bindings: Vec<_> = bindings.iter().filter(|binding| binding.node_id == node_id).collect();
    if node_bindings.is_empty() {
        return snapshot;
    }
    let mut clone = snapshot;
    for binding in node_bindings {
        let Some(parameter) = parameters.iter().find(|entry| entry.id() == binding.parameter_id) else {
            continue;
        };
        set_json_pointer_value(&mut clone, &binding.field_path, os_parameter_value(parameter));
    }
    clone
}

// 🪦️ `validate_parameter_config_binding` (this `instance` module's own copy, over
// `OsParameterFieldBinding`/`OsParameterType`) DELETED
// (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` M1): its only callers
// repo-wide were its own 3 unit tests below (also deleted) — dead code, superseded by the
// structurally identical, actually-live `workflow::validate_workflow_parameter_config_binding`
// (called from `reconcile_workflow_snapshot`'s `ParameterBindingValidation` region against the
// real `WorkflowParameterBinding`/`WorkflowParameterType`/`workflow`-graph types, not this
// module's parallel `Os*` vocabulary).

trait OsParameterId {
    fn id(&self) -> &str;
}

impl OsParameterId for OsParameter {
    fn id(&self) -> &str {
        parameter_id(self)
    }
}

/// 🎛️ Resolves bound parameter values for a workflow node as a field-path map.
pub fn resolve_parameter_values_for_instance(bindings: &[OsParameterFieldBinding], parameters: &[OsParameter], node_id: &str) -> HashMap<String, Value> {
    let mut values = HashMap::new();
    for binding in bindings.iter().filter(|entry| entry.node_id == node_id) {
        let Some(parameter) = parameters.iter().find(|entry| entry.id() == binding.parameter_id) else {
            continue;
        };
        values.insert(binding.field_path.clone(), os_parameter_value(parameter));
    }
    values
}

/// 🎛️ Builds the workflow input port id for a bound space parameter.
pub fn parameter_port_id(node_id: &str, parameter_id: &str) -> String {
    media_port_id_for_spec(node_id, &format!("{OS_PARAMETER_PORT_PREFIX}{parameter_id}"), "in")
}

/// 🎛️ Returns whether a media port id denotes a space parameter input channel.
pub fn is_parameter_port_id(port_id: &str) -> bool {
    media_port_spec_id(port_id).is_some_and(|spec_id| spec_id.starts_with(OS_PARAMETER_PORT_PREFIX))
}

/// 🎛️ Extracts the space parameter id from a parameter input port id.
pub fn parameter_id_from_port_id(port_id: &str) -> Option<String> {
    let spec_id = media_port_spec_id(port_id)?;
    spec_id.strip_prefix(OS_PARAMETER_PORT_PREFIX).map(str::to_string)
}

pub fn media_port_id_for_spec(instance_id: &str, spec_id: &str, direction: &str) -> String {
    format!("{instance_id}:{spec_id}:{direction}")
}

pub fn media_port_spec_id(port_id: &str) -> Option<String> {
    let parts: Vec<_> = port_id.split(':').collect();
    if parts.len() < 3 {
        return None;
    }
    Some(parts[1..parts.len() - 1].join(":"))
}
//#endregion 🔖️Parameters

//#region 🔖️Materialize
/// 🎚️ Default config value seeded from `config_spec.fields[].default` — what a freshly
/// spawned instance's config resolves to before any explicit `instance.config`/binding overlay.
fn config_spec_default_value(config_spec: &ConfigSpec) -> Value {
    let mut defaults = serde_json::Map::new();
    for field in &config_spec.fields {
        if let Some(default) = &field.default {
            let json_default = Value::from(default.clone());
            defaults.insert(field.id.clone(), json_default);
        }
    }
    Value::Object(defaults)
}

/// 🧩️ Builds the dynamic config value for an `AppCommand::Configure` payload: starts from the
/// app's own `ConfigSpec` defaults, then overlays every parameter bound to one of `config_spec`'s
/// fields with that parameter's current value — the config-driving counterpart to
/// `apply_parameter_values_to_snapshot`'s document-JSON-pointer overlay (see
/// `OsParameterFieldBinding::field_path`'s doc for how the two diverge). Callers (the
/// renderer/headless-runner drivers dispatching `AppCommand::Configure`, both out of this crate's
/// scope) `store::pack_rt::encode_wire_value` the result themselves — this function only builds the
/// value, it never sends anything over a channel.
///
/// 🚧️ WP-1.B ripple: this used to start from `OsAppInstance::config` (an inline `DslValue` on the
/// deleted per-instance record); config now lives on the node's own config artifact
/// (`WorkflowNode::config_ref`), which this work package does not wire up (out of scope — see the
/// master plan's "Config on the node" wave). Always starts from `config_spec`'s own defaults until
/// that lands.
pub fn build_configure_config(node_id: &str, parameters: &[OsParameter], bindings: &[OsParameterFieldBinding], config_spec: &ConfigSpec) -> semio_framework_value::DslValue {
    let mut config = semio_framework_value::DslValue::from(&config_spec_default_value(config_spec));
    let entries = match &mut config {
        semio_framework_value::DslValue::Object(entries) => entries,
        _ => {
            config = semio_framework_value::DslValue::Object(vec![]);
            match &mut config {
                semio_framework_value::DslValue::Object(entries) => entries,
                _ => unreachable!("config object branch"),
            }
        }
    };
    for binding in bindings.iter().filter(|binding| binding.node_id == node_id) {
        let Some(field) = config_spec.fields.iter().find(|field| field.id == binding.field_path) else {
            continue;
        };
        let Some(parameter) = parameters.iter().find(|entry| entry.id() == binding.parameter_id) else {
            continue;
        };
        let value = semio_framework_value::DslValue::from(&os_parameter_value(parameter));
        if let Some((_, slot)) = entries.iter_mut().find(|(key, _)| key == &field.id) {
            *slot = value;
        } else {
            entries.push((field.id.clone(), value));
        }
    }
    config
}

/// 🧩️ Overlays bound parameter values onto an app instance's current document snapshot.
/// Content itself lives in the app's own `framework/sync`-hosted document (referenced by
/// {@link OsArtifactRef}, read host-side and passed in as `current_document_json`) — this function
/// no longer resolves embedded/upstream source documents; that concept was deleted with
/// `OsSourceDocument`. Cross-instance ("upstream") dataflow through workflow edges is deferred
/// (see `ArtifactHost` doc-comment) to a follow-up that reads the upstream app's live document.
pub fn materialize_os_app_instance_document_json(current_document_json: &str, node_id: &str, bindings: &[OsParameterFieldBinding], parameters: &[OsParameter]) -> String {
    let snapshot: Value = serde_json::from_str(current_document_json).unwrap_or_else(|_| json!({}));
    let with_params = apply_parameter_values_to_snapshot(snapshot, bindings, parameters, node_id);
    serde_json::to_string(&with_params).unwrap_or_else(|_| "{}".into())
}

/// 🔀️ Host-side hook for the common case: when a bound parameter's value changes, computes the
/// patched document JSON for every app instance with a field bound to it, keyed by document id — the
/// host dispatches each as a snapshot replace into that app's own document store (e.g. via the program
/// WIT boundary's `load-app-document`, or `framework/sync`'s document actor once the app is wired onto
/// `ArtifactHost`). This covers the "common/simple case" per the JSON-pointer overlay convention
/// {@link apply_parameter_values_to_snapshot} already established — a true typed operation into the bound
/// app's own `Mutation` vocabulary requires that app's real (non-opaque) Mutation type and is left to each app's
/// own `ArtifactApp` migration (WS-F); until then this snapshot-replace path is the host's only lever.
pub fn app_instance_document_patches_for_binding(
    parameter_id: &str,
    nodes: &[WorkflowNode],
    bindings: &[OsParameterFieldBinding],
    parameters: &[OsParameter],
    current_document_json: impl Fn(&str) -> Option<String>,
) -> Vec<(String, String)> {
    let bound_node_ids: HashSet<String> = bindings.iter().filter(|binding| binding.parameter_id == parameter_id).map(|binding| binding.node_id.clone()).collect();
    nodes
        .iter()
        .filter(|node| bound_node_ids.contains(&node.id))
        .filter_map(|node| {
            let current_json = current_document_json(&node.artifact_ref)?;
            let patched = materialize_os_app_instance_document_json(&current_json, &node.id, bindings, parameters);
            Some((node.artifact_ref.clone(), patched))
        })
        .collect()
}
//#endregion 🔖️Materialize

//#region 🧪️Tests
#[cfg(test)]
include!("../🧪️tests/🔬️instance-unit/🦀️.rs");
//#endregion 🧪️Tests
// #endregion instance
