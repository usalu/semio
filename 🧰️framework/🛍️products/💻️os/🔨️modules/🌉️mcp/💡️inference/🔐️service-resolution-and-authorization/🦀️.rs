//! 🔐️ Service resolution and authorization of inference jobs: operation interrupts, request ids, document service resolution, the tool context, job handles and the guest job page. See [`super`].

use super::*;

//#region 💡️InferenceJobTools
/// 🛑️ Interrupts the retained local wait for one operation label; `false` when nothing was waiting.
pub fn interrupt_inference_operation(label: &str) -> bool {
    let registry = inflight_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let mut interrupted = false;
    for (existing, token) in registry.iter() {
        if existing.as_str() == label {
            token.cancel_now();
            interrupted = true;
        }
    }
    interrupted
}

pub(super) fn inference_wall_now_ms() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis() as u64
}

/// 🎲️ A fresh 32-lower-hex client idempotency key. The hub mints the `jobId`; this is only the
/// client's own scoped `(user, authorization generation, space, document, request_id)` key.
pub fn mint_inference_request_id() -> String {
    let entropy = format!("semio.mcp.inference-request/v1\0{}\0{}", inference_wall_now_ms(), mint_id(HandleKind::Job, inference_wall_now_ms()));
    framework_hash::hash_bytes(entropy.as_bytes())[..INFERENCE_REQUEST_ID_HEX_LENGTH].to_string()
}

//#region 🔖️InferenceServiceResolution
/// 🗂️ The artifact kind one readable document IS — the owning app's dialect coordinate when the
/// workspace knows it, else the schema a folder document shares with its kind.
pub(super) fn document_artifact_kind(workspace: &HeadlessWorkspace, document_id: &str) -> Result<String, GatewayError> {
    let contents = workspace.read_resource(&format!("semio://artifact/{document_id}/schema"))?;
    let body: serde_json::Value = contents.first().and_then(|content| content.text.as_deref()).and_then(|text| serde_json::from_str(text).ok()).ok_or_else(|| GatewayError::new(GatewayErrorCode::Internal, format!("`{document_id}` schema resource returned no decodable body")))?;
    body.get("artifactKind").or_else(|| body.get("schema")).and_then(serde_json::Value::as_str).map(str::to_string).ok_or_else(|| GatewayError::new(GatewayErrorCode::NotFound, format!("document `{document_id}` names no artifact kind")))
}

/// 🧭️ The one service a request names on an open document, resolved against the workspace's
/// declared roster and the services its hub (if any) executes itself.
pub fn resolve_document_inference_service(workspace: &HeadlessWorkspace, document_id: &str, inference_schema: Option<&str>, plugin_id: Option<&str>) -> Result<InferenceService, GatewayError> {
    let artifact_kind = document_artifact_kind(workspace, document_id)?;
    select_inference_service(&declared_inferences_for_workspace(workspace)?, &workspace.hub_inference_services()?, &artifact_kind, inference_schema, plugin_id)
}
//#endregion 🔖️InferenceServiceResolution

pub(super) fn inference_input_invalid(detail: &str) -> GatewayError {
    GatewayError::new(GatewayErrorCode::InputInvalid, detail.to_string())
}

/// 🔐️ The one local admission gate: the connection's granted MCP scopes must cover the
/// capability's declared scopes. A hub-executed job is re-authorized by the hub on every route; a
/// guest job's commit is authorized again by the commit action's own policy.
pub(super) fn authorize_inference(policy: &PolicyEngine, principal: &AgentPrincipal, capability: &CapabilityDefinition) -> Result<(), GatewayError> {
    policy.authorize_scopes(principal, capability)
}

pub(super) struct InferenceToolContext<'a> {
    pub workspace: Option<&'a Arc<HeadlessWorkspace>>,
    pub actions: &'a Arc<ActionAdapter>,
    pub catalog: &'a Arc<Catalog>,
    pub principal: &'a AgentPrincipal,
    pub(super) session: SessionHandle,
}

impl InferenceToolContext<'_> {
    pub fn policy(&self) -> &PolicyEngine {
        self.actions.policy()
    }

    pub fn handles(&self) -> &HandleTable {
        self.actions.handles().as_ref()
    }

    pub fn workspace(&self, what: &str) -> Result<&Arc<HeadlessWorkspace>, GatewayError> {
        self.workspace.ok_or_else(|| workspace_binding_required(what))
    }

    /// 🎫️ Resolves one `job_` handle against this connection's own session; a hub job's live subject
    /// must still be the exact subject and authority generation that minted it.
    pub(crate) fn job(&self, workspace: &HeadlessWorkspace, handle: &str) -> Result<InferenceJobHandlePayloadV1, GatewayError> {
        let payload = resolve_inference_job_handle(self.handles(), &self.session, handle, inference_wall_now_ms())?;
        if payload.hub.is_some() {
            check_hub_job_subject(&payload, &workspace.hub_inference_subject()?)?;
        }
        Ok(payload)
    }
}

/// 🎫️ One `job_` handle of THIS session, decoded to the inference job it names.
pub fn resolve_inference_job_handle(handles: &HandleTable, session: &SessionHandle, handle: &str, now_ms: u64) -> Result<InferenceJobHandlePayloadV1, GatewayError> {
    let record = handles.resolve(handle, session, now_ms)?;
    if record.kind != HandleKind::Job {
        return Err(inference_input_invalid("handle is not an inference job handle"));
    }
    serde_json::from_value(record.payload).map_err(|_| GatewayError::new(GatewayErrorCode::InputInvalid, "this job handle does not name an inference job"))
}

/// 👤️ A hub job is owner-private locally as well as on the hub: the live subject must be the one
/// that minted it, under the same authority generation, in the same space.
pub fn check_hub_job_subject(payload: &InferenceJobHandlePayloadV1, subject: &HubInferenceSubjectV1) -> Result<(), GatewayError> {
    let Some(hub) = payload.hub.as_ref() else { return Ok(()) };
    if hub.subject_user_id != subject.user_id || hub.authority_generation != subject.authority_generation || hub.space_id != subject.space_id {
        return Err(GatewayError::new(GatewayErrorCode::PermissionDenied, "this inference job belongs to a different authenticated subject or authority generation"));
    }
    Ok(())
}

pub(super) fn job_handle_argument(arguments: &serde_json::Value) -> Result<&str, GatewayError> {
    arguments.get("jobHandle").and_then(serde_json::Value::as_str).ok_or_else(|| inference_input_invalid("jobHandle is required"))
}

pub(super) fn job_reply(text: String, structured: serde_json::Value) -> CallToolResult {
    CallToolResult::ok(vec![ContentBlock::Text { text }], Some(structured))
}

pub(super) fn page_value(page: &InferenceJobPageV1) -> serde_json::Value {
    serde_json::to_value(page).unwrap_or(serde_json::Value::Null)
}

pub(super) fn page_text(page: &InferenceJobPageV1) -> String {
    format!("{} job {} ({}) is {:?} / {:?}, {} event(s), {} progress row(s), next cursor {}", page.service_id, page.job_id, if page.site == InferenceExecutionSiteV1::Guest { "guest" } else { "hub" }, page.state, page.proposal_state, page.events.len(), page.progress.len(), page.next_cursor)
}

/// 📃️ The current page of one guest job, after `after`.
pub(super) fn guest_job_page(job_id: &str, after: u64) -> Result<InferenceJobPageV1, GatewayError> {
    let registry = job_registry();
    let (Some(snapshot), Some(journal), Some(job)) = (registry.snapshot(job_id), registry.events(job_id, after, INFERENCE_JOB_EVENT_PAGE_MAX_ITEMS), guest_job(job_id)) else {
        return Err(GatewayError::new(GatewayErrorCode::NotFound, format!("no inference job {job_id} in this process")));
    };
    Ok(guest_page(job_id, &snapshot, &journal.events, journal.next_cursor, &job))
}

pub(super) fn hub_scope_route(payload: &InferenceJobHandlePayloadV1) -> Result<&HubInferenceJobBindingV1, GatewayError> {
    payload.hub.as_ref().ok_or_else(|| GatewayError::new(GatewayErrorCode::Internal, "a hub-executed job handle carries no hub binding"))
}
//#endregion 💡️InferenceJobTools
