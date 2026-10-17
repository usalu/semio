//! 🔭️ Job tool registration and hub read of inference: the job tool registration, the deterministic request id and the hub inference read. See [`super`].

use super::*;

//#region 💡️InferenceJobTools
/// 💡️ Registers the four inference job tools plus `inference_run`. Like every other tool in this
/// crate they are ALWAYS present in `tools/list`; only a call's result varies by whether a
/// workspace (and, for a hub-executed service, a hub binding) exists, and every call is first gated
/// by the connection's own granted MCP scopes.
pub fn register_inference_job_tools(registry: &mut InMemoryToolRegistry, catalog: Arc<Catalog>, workspace: Option<Arc<HeadlessWorkspace>>, actions: Arc<ActionAdapter>, principal: AgentPrincipal, session: SessionHandle) {
    let definitions: [(CapabilityDefinition, &str, fn(&InferenceToolContext<'_>, serde_json::Value) -> CallToolResult); 5] = [
        (inference_submit_capability(), "inference_submit", inference_submit_handler),
        (inference_events_capability(), "inference_events", inference_events_handler),
        (inference_cancel_capability(), "inference_cancel", inference_cancel_handler),
        (inference_approve_capability(), "inference_approve", inference_approve_handler),
        (inference_run_capability(), "inference_run", inference_run_handler),
    ];
    for (capability, tool_name, handler) in definitions {
        let tool = tool_from_capability(&capability, tool_name);
        let (tool_workspace, tool_actions, tool_catalog, tool_principal, tool_session) = (workspace.clone(), actions.clone(), catalog.clone(), principal.clone(), session.clone());
        registry
            .register(tool, move |arguments| {
                let context = InferenceToolContext { workspace: tool_workspace.as_ref(), actions: &tool_actions, catalog: &tool_catalog, principal: &tool_principal, session: tool_session.clone() };
                handler(&context, arguments)
            })
            .expect("inference job tool names are valid");
    }
}
//#endregion 💡️InferenceJobTools

//#region 💡️InferenceHubRead
/// 🆔️ The deterministic client idempotency key one hub-backed inference READ uses, derived from
/// nothing but the authenticated subject and the document. The hub's ledger scopes idempotency on
/// `(user, authorization generation, space, document, request_id)`, so re-reading the same field
/// reconciles to exactly ONE hub job instead of starting a fresh one each poll.
pub fn deterministic_inference_request_id(subject: &HubInferenceSubjectV1, document_id: &str) -> String {
    let seed = format!("semio.mcp.inference-read/v1\0{}\0{}\0{}\0{}\0{}", subject.hub_origin, subject.space_id, document_id, subject.user_id, subject.authority_generation);
    framework_hash::hash_bytes(seed.as_bytes())[..INFERENCE_REQUEST_ID_HEX_LENGTH].to_string()
}

/// 💡️ A READ of a hub-executed service's field: submits (or reconciles to) the one job this subject
/// owns for the document under a deterministic request id and returns its owner-private receipt. A
/// guest service answers `None` — a read never spends guest time — so the caller falls through to
/// the discovery path, which tells the reader to run it with `inference_run` or `inference_submit`.
pub fn hub_inference_read(workspace: &Arc<HeadlessWorkspace>, artifact_id: &str, inference_schema: &str) -> Option<Result<serde_json::Value, GatewayError>> {
    let service = match resolve_document_inference_service(workspace, artifact_id, Some(inference_schema), None) {
        Ok(service) if service.site == InferenceExecutionSiteV1::Hub => service,
        Ok(_) | Err(_) => return None,
    };
    let route = service.route.clone()?;
    let subject = match workspace.hub_inference_subject() {
        Ok(subject) => subject,
        Err(error) => return Some(Err(error)),
    };
    let request = HubInferenceSubmitRequestV1::new(service.service_id(), deterministic_inference_request_id(&subject, artifact_id), INFERENCE_JOB_MAX_LIFETIME_MS);
    Some(workspace.submit_hub_inference_job(artifact_id, &route, &request).map(|receipt| {
        serde_json::json!({
            "artifactId": artifact_id,
            "inferenceSchema": inference_schema,
            "subjectUserId": subject.user_id,
            "authorityGeneration": subject.authority_generation,
            "job": page_value(&hub_receipt_page(&service, artifact_id, &receipt)),
        })
    }))
}
//#endregion 💡️InferenceHubRead
