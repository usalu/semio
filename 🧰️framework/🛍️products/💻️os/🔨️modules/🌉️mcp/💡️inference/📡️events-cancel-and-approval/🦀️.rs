//! 📡️ Events, cancel and approval of inference jobs: the events and cancel handlers, the approve handler and the guest and hub approval paths. See [`super`].

use super::*;

//#region 💡️InferenceJobTools
pub(super) fn inference_events_handler(context: &InferenceToolContext<'_>, arguments: serde_json::Value) -> CallToolResult {
    let read = (|| {
        authorize_inference(context.policy(), context.principal, &inference_events_capability())?;
        let handle = job_handle_argument(&arguments)?;
        let workspace = context.workspace("inference_events")?;
        let payload = context.job(workspace, handle)?;
        let after = arguments.get("after").and_then(serde_json::Value::as_u64).unwrap_or(0);
        let page = match payload.site {
            InferenceExecutionSiteV1::Guest => guest_job_page(&payload.job_id, after)?,
            InferenceExecutionSiteV1::Hub => {
                let hub = hub_scope_route(&payload)?;
                let page = workspace.read_hub_inference_job_events(&payload.document_id, &hub.route, &payload.job_id, after)?;
                hub_events_page(&handle_service(&payload), &payload.document_id, &page)
            }
        };
        Ok((page_text(&page), serde_json::json!({ "jobHandle": handle, "job": page_value(&page), "baseBinding": payload.hub.as_ref().and_then(|hub| hub.base.clone()) })))
    })();
    match read {
        Ok((text, structured)) => job_reply(text, structured),
        Err(error) => CallToolResult::tool_error(&error),
    }
}

pub(super) fn inference_cancel_handler(context: &InferenceToolContext<'_>, arguments: serde_json::Value) -> CallToolResult {
    let cancelled = (|| {
        authorize_inference(context.policy(), context.principal, &inference_cancel_capability())?;
        let handle = job_handle_argument(&arguments)?;
        let workspace = context.workspace("inference_cancel")?;
        let payload = context.job(workspace, handle)?;
        let (page, interrupted) = match payload.site {
            InferenceExecutionSiteV1::Guest => {
                job_registry().request_cancel(&payload.job_id)?;
                (guest_job_page(&payload.job_id, 0)?, true)
            }
            InferenceExecutionSiteV1::Hub => {
                let hub = hub_scope_route(&payload)?;
                let interrupted = interrupt_inference_operation(&inference_operation_label(&hub.space_id, &payload.document_id, Some(&payload.job_id))) | interrupt_inference_operation(&inference_operation_label(&hub.space_id, &payload.document_id, None));
                let page = workspace.cancel_hub_inference_job(&payload.document_id, &hub.route, &payload.job_id)?;
                (hub_events_page(&handle_service(&payload), &payload.document_id, &page), interrupted)
            }
        };
        Ok((format!("cancel requested — {}", page_text(&page)), serde_json::json!({ "jobHandle": handle, "job": page_value(&page), "localWaitInterrupted": interrupted })))
    })();
    match cancelled {
        Ok((text, structured)) => job_reply(text, structured),
        Err(error) => CallToolResult::tool_error(&error),
    }
}

pub(super) fn inference_approve_handler(context: &InferenceToolContext<'_>, arguments: serde_json::Value) -> CallToolResult {
    let approved = (|| {
        authorize_inference(context.policy(), context.principal, &inference_approve_capability())?;
        let handle = job_handle_argument(&arguments)?;
        let proposal_hash = arguments.get("proposalHash").and_then(serde_json::Value::as_str).ok_or_else(|| inference_input_invalid("proposalHash is required"))?;
        let workspace = context.workspace("inference_approve")?;
        let payload = context.job(workspace, handle)?;
        match payload.site {
            InferenceExecutionSiteV1::Guest => approve_guest_job(context, workspace, handle, &payload, proposal_hash),
            InferenceExecutionSiteV1::Hub => approve_hub_job(context, workspace, handle, &payload, proposal_hash),
        }
    })();
    match approved {
        Ok((text, structured)) => job_reply(text, structured),
        Err(error) => CallToolResult::tool_error(&error),
    }
}

/// ✅️ Commits one guest proposal: the approval is consumed exactly once, the document is bound as
/// this session's document for the plugin, and the declared commit action is prepared and invoked
/// on it through `ActionAdapter` — the normal edit path, whose relay carries the edit to the hub's
/// document ledger for a hub-bound document.
fn approve_guest_job(context: &InferenceToolContext<'_>, workspace: &Arc<HeadlessWorkspace>, handle: &str, payload: &InferenceJobHandlePayloadV1, proposal_hash: &str) -> Result<(String, serde_json::Value), GatewayError> {
    let job = guest_job(&payload.job_id).ok_or_else(|| GatewayError::new(GatewayErrorCode::NotFound, format!("no inference job {} in this process", payload.job_id)))?;
    let proposal = job.proposal.clone().ok_or_else(|| GatewayError::new(GatewayErrorCode::PreconditionFailed, format!("job {} has offered no proposal", payload.job_id)))?;
    if proposal.hash != proposal_hash {
        return Err(GatewayError::new(GatewayErrorCode::PreconditionFailed, "proposalHash does not name this job's offered proposal").with_details(serde_json::json!({ "field": "proposalHash" })));
    }
    let jobs = job_registry();
    if !jobs.resume_approved(&payload.job_id) {
        return Err(GatewayError::new(GatewayErrorCode::PreconditionFailed, format!("job {} has no proposal awaiting approval — it was already approved, withdrawn or cancelled", payload.job_id)));
    }
    match commit_guest_proposal(context, workspace, &payload.document_id, &proposal) {
        Ok(report) => {
            let committed = serde_json::to_value(&report).unwrap_or(serde_json::Value::Null);
            update_guest_job(&payload.job_id, |job| job.committed = Some(committed.clone()));
            jobs.record_event(&payload.job_id, "committed", Some(report.invocation_id.clone()));
            jobs.succeed(&payload.job_id, serde_json::json!({ "proposal": proposal, "commit": committed }));
        }
        Err(error) => {
            update_guest_job(&payload.job_id, |job| job.commit_refused = true);
            jobs.fail(&payload.job_id, error.clone());
            return Err(error);
        }
    }
    let page = guest_job_page(&payload.job_id, 0)?;
    let undo_token = page.commit.as_ref().and_then(|commit| commit.get("undoToken")).cloned();
    Ok((format!("approved — {}", page_text(&page)), serde_json::json!({ "jobHandle": handle, "job": page_value(&page), "undoToken": undo_token })))
}

fn commit_guest_proposal(context: &InferenceToolContext<'_>, workspace: &Arc<HeadlessWorkspace>, document_id: &str, proposal: &InferenceProposalV1) -> Result<InvocationReport, GatewayError> {
    let (Some(capability_id), Some(input)) = (proposal.capability_id.as_deref(), proposal.input.clone()) else {
        return Err(GatewayError::new(GatewayErrorCode::Internal, "an offered guest proposal carries no commit capability"));
    };
    if let Some((pack, spr)) = workspace.read_artifact_bytes(document_id)? {
        workspace.bind_hub_session_document(document_id, &pack, &spr)?;
    }
    let instance = commit_instance_slot(context.catalog, capability_id);
    let prepared = context.actions.prepare(context.catalog, context.principal, &context.session, capability_id, input, instance, inference_wall_now_ms())?;
    if prepared.expected_revision.as_ref().is_some_and(|revision| revision.artifact_id != document_id) {
        let _ = context.actions.cancel(&context.session, &prepared.prepared_handle, inference_wall_now_ms());
        return Err(GatewayError::new(GatewayErrorCode::PreconditionFailed, format!("this session addresses another document of `{capability_id}`'s plugin, so the commit cannot land on `{document_id}` unambiguously")));
    }
    let request = InvokeRequest { prepared_handle: Some(prepared.prepared_handle), capability_id: None, input: None, expected_revision: prepared.expected_revision, idempotency_key: Some(format!("inference-{}", proposal.hash)), approval_handle: None };
    let report = context.actions.invoke(context.catalog, context.principal, &context.session, request, instance, inference_wall_now_ms())?;
    if report.status != InvocationStatus::Succeeded {
        return Err(GatewayError::new(GatewayErrorCode::SideEffectRejected, format!("the commit action `{capability_id}` answered {:?}", report.status)));
    }
    Ok(report)
}

#[cfg(not(target_arch = "wasm32"))]
fn commit_instance_slot(catalog: &Catalog, capability_id: &str) -> u32 {
    capability_instance_slot(catalog, capability_id).unwrap_or(0)
}

#[cfg(target_arch = "wasm32")]
fn commit_instance_slot(_catalog: &Catalog, _capability_id: &str) -> u32 {
    0
}

/// ✅️ Relays one approval to the hub that executed the job; the hub rebuilds and commits the typed
/// effect server-side and hands back a durable undo target, retained as this session's undo token.
/// Every precondition is checked BEFORE the relay; once the hub's receipt exists the edit is a fact,
/// so a follow-up that cannot complete (undo retention, the events page) is a named `warnings` row
/// on a successful answer, never an error that would tell the agent to retry an approval that landed.
fn approve_hub_job(context: &InferenceToolContext<'_>, workspace: &Arc<HeadlessWorkspace>, handle: &str, payload: &InferenceJobHandlePayloadV1, proposal_hash: &str) -> Result<(String, serde_json::Value), GatewayError> {
    let hub = hub_scope_route(payload)?;
    let request = HubInferenceApprovalRequestV1::new(payload.job_id.clone(), proposal_hash.to_string());
    request.validate().map_err(|error| error.to_gateway_error("inference_approve"))?;
    let base = hub.base.as_ref().ok_or_else(|| GatewayError::new(GatewayErrorCode::PreconditionFailed, "approved inference job has no retained canonical base binding"))?;
    let receipt = workspace.approve_hub_inference_job(&payload.document_id, &hub.route, &request)?;
    let scope = DocumentScope::new(hub.space_id.clone(), payload.document_id.clone());
    let mut warnings = Vec::new();
    let undo_token = context
        .actions
        .retain_hub_inference_approval_undo(&context.session, &base.hub_origin, &scope, &hub.route, &receipt.undo, inference_wall_now_ms())
        .map_err(|error| warnings.push(format!("undo-unavailable: {}", error.message)))
        .ok();
    let job = workspace
        .read_hub_inference_job_events(&payload.document_id, &hub.route, &payload.job_id, 0)
        .map(|events| page_value(&hub_events_page(&handle_service(payload), &payload.document_id, &events)))
        .map_err(|error| warnings.push(format!("events-unavailable: {}", error.message)))
        .ok();
    Ok((
        format!("approval of job {} produced mutation {} (applied: {})", receipt.job_id, receipt.mutation_id, receipt.applied),
        serde_json::json!({ "jobHandle": handle, "job": job, "undoToken": undo_token, "commit": serde_json::to_value(&receipt).unwrap_or(serde_json::Value::Null), "baseBinding": hub.base, "warnings": warnings }),
    ))
}
//#endregion 💡️InferenceJobTools
