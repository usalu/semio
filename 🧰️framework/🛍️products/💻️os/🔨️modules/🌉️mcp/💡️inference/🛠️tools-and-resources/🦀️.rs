//! 🛠️ Tools and resources of inference: the list and get handlers, tool registration and the inference resources. See [`super`].

use super::*;

//#region 🔖️Tools
pub(super) fn workspace_binding_required(what: &str) -> GatewayError {
    GatewayError::new(GatewayErrorCode::PluginUnavailable, format!("`{what}` needs a live workspace — bind one with `--folder <path>` or `--hub <url> --space <id>`")).with_details(serde_json::json!({ "bindWith": ["--folder", "--hub"] })).retryable()
}

fn inference_list_handler(workspace: Option<&Arc<HeadlessWorkspace>>, arguments: serde_json::Value) -> CallToolResult {
    let Some(workspace) = workspace else {
        return CallToolResult::tool_error(&workspace_binding_required("inference_list"));
    };
    match arguments.get("artifactId").and_then(serde_json::Value::as_str) {
        Some(artifact_id) => match declared_inferences_for_artifact(workspace, artifact_id) {
            Ok((schema, declared)) => {
                let count = declared.len();
                let structured = serde_json::json!({ "artifactId": artifact_id, "artifactKind": schema, "declared": declared });
                CallToolResult::ok(vec![ContentBlock::Text { text: format!("{count} declared inference(s) for {artifact_id}") }], Some(structured))
            }
            Err(error) => CallToolResult::tool_error(&error),
        },
        None => match declared_inferences_for_workspace(workspace) {
            Ok(declared) => {
                let count = declared.len();
                let structured = serde_json::json!({ "declared": declared });
                CallToolResult::ok(vec![ContentBlock::Text { text: format!("{count} declared inference(s) in this workspace") }], Some(structured))
            }
            Err(error) => CallToolResult::tool_error(&error),
        },
    }
}

fn inference_get_handler(workspace: Option<&Arc<HeadlessWorkspace>>, arguments: serde_json::Value) -> CallToolResult {
    let Some(workspace) = workspace else {
        return CallToolResult::tool_error(&workspace_binding_required("inference_get"));
    };
    let Some(artifact_id) = arguments.get("artifactId").and_then(serde_json::Value::as_str) else {
        return CallToolResult::tool_error(&GatewayError::new(GatewayErrorCode::InputInvalid, "artifactId is required"));
    };
    let Some(inference_schema) = arguments.get("inferenceSchema").and_then(serde_json::Value::as_str) else {
        return CallToolResult::tool_error(&GatewayError::new(GatewayErrorCode::InputInvalid, "inferenceSchema is required"));
    };
    if let Some(outcome) = hub_inference_read(workspace, artifact_id, inference_schema) {
        return match outcome {
            Ok(body) => CallToolResult::ok(vec![ContentBlock::Text { text: format!("hub inference job for `{inference_schema}` on {artifact_id}") }], Some(body)),
            Err(error) => CallToolResult::tool_error(&error),
        };
    }
    match declared_inferences_for_artifact(workspace, artifact_id) {
        Err(error) => CallToolResult::tool_error(&error),
        Ok((schema, declared)) => match lookup_inference(&declared, inference_schema) {
            InferenceLookup::NoSuchService => CallToolResult::tool_error(&no_such_service_error(&schema, inference_schema)),
            InferenceLookup::Execute(item) => CallToolResult::tool_error(&execution_not_wired_error(&item)),
        },
    }
}

/// 💡️ Registers the inference tool(s) — always present in `tools/list` regardless of tier; only
/// their RESULT varies by whether `workspace` is bound (`🦀️.rs`'s own
/// `DECLARED_STUB_TOOL_NAMES` convention: presence never depends on tier).
pub fn register_inference_tools(registry: &mut InMemoryToolRegistry, workspace: Option<Arc<HeadlessWorkspace>>) {
    let list_capability = inference_list_capability();
    let list_tool = tool_from_capability(&list_capability, "inference_list");
    let list_workspace = workspace.clone();
    registry.register(list_tool, move |arguments| inference_list_handler(list_workspace.as_ref(), arguments)).expect("inference_list is a valid tool name");

    let get_capability = inference_get_capability();
    let get_tool = tool_from_capability(&get_capability, "inference_get");
    let get_workspace = workspace.clone();
    registry.register(get_tool, move |arguments| inference_get_handler(get_workspace.as_ref(), arguments)).expect("inference_get is a valid tool name");
}
//#endregion 🔖️Tools

//#region 🔖️Resources
/// 💡️ The `semio://artifact/{id}/inference…` resources this facet answers; `None` when `uri` is not
/// one of ours (so the composing registry falls through to the next facet), `Some(Err(NOT_FOUND))`
/// for a URI that IS clearly ours but malformed (empty artifact id, empty/nested field segment).
pub fn read_inference_resource(uri: &str, workspace: Option<&Arc<HeadlessWorkspace>>) -> Option<Result<Vec<ResourceContent>, GatewayError>> {
    let rest = uri.strip_prefix("semio://artifact/")?;
    let (artifact_id, sub) = rest.split_once('/')?;
    if sub != "inference" && !sub.starts_with("inference/") {
        return None;
    }
    if artifact_id.is_empty() {
        return Some(Err(GatewayError::new(GatewayErrorCode::NotFound, format!("malformed inference resource uri: {uri}"))));
    }
    let field = match sub.strip_prefix("inference/") {
        None => None,
        Some(field) if !field.is_empty() && !field.contains('/') => Some(field),
        Some(_) => return Some(Err(GatewayError::new(GatewayErrorCode::NotFound, format!("malformed inference resource uri: {uri}")))),
    };
    let Some(workspace) = workspace else {
        return Some(Err(workspace_binding_required(uri)));
    };
    if let Some(outcome) = field.and_then(|field| hub_inference_read(workspace, artifact_id, field)) {
        return Some(outcome.map(|body| vec![ResourceContent { uri: uri.to_string(), mime_type: Some("application/json".to_string()), text: Some(body.to_string()), blob: None }]));
    }
    Some(match declared_inferences_for_artifact(workspace, artifact_id) {
        Err(error) => Err(error),
        Ok((schema, declared)) => match field {
            None => {
                let body = serde_json::json!({ "artifactId": artifact_id, "artifactKind": schema, "declared": declared });
                Ok(vec![ResourceContent { uri: uri.to_string(), mime_type: Some("application/json".to_string()), text: Some(body.to_string()), blob: None }])
            }
            Some(field) => match lookup_inference(&declared, field) {
                InferenceLookup::NoSuchService => Err(no_such_service_error(&schema, field)),
                InferenceLookup::Execute(item) => Err(execution_not_wired_error(&item)),
            },
        },
    })
}

/// 💡️ The inference resource entries to advertise in `resources/list` — one `.../inference` index
/// per artifact id this workspace currently knows about (bare tier: empty, matching
/// `WorkspaceResourceRegistry`'s own convention of only listing real, known ids).
pub fn inference_resources(workspace: Option<&Arc<HeadlessWorkspace>>) -> Vec<Resource> {
    let Some(workspace) = workspace else {
        return Vec::new();
    };
    let Ok(ids) = workspace.workspace_artifact_ids() else {
        return Vec::new();
    };
    ids.into_iter()
        .map(|artifact_id| Resource {
            uri: format!("semio://artifact/{artifact_id}/inference"),
            name: format!("{artifact_id}-inference"),
            title: Some(format!("Declared inferences for {artifact_id}")),
            description: Some("Every inference service declared for this artifact's kind, and whether it can be read yet".to_string()),
            mime_type: Some("application/json".to_string()),
            size: None,
        })
        .collect()
}
//#endregion 🔖️Resources
