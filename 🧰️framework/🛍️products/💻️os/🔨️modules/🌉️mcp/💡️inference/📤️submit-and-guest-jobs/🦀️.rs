//! 📤️ Submit and guest jobs of inference: the submit handler, guest job submission and spawn, the guest run, hub job submission and the service handler. See [`super`].

use super::*;

//#region 💡️InferenceJobTools
pub(super) fn inference_submit_handler(context: &InferenceToolContext<'_>, arguments: serde_json::Value) -> CallToolResult {
    let submitted = (|| {
        authorize_inference(context.policy(), context.principal, &inference_submit_capability())?;
        let document_id = arguments.get("documentId").and_then(serde_json::Value::as_str).ok_or_else(|| inference_input_invalid("documentId is required"))?;
        let workspace = context.workspace("inference_submit")?;
        let service = resolve_document_inference_service(workspace, document_id, arguments.get("inferenceSchema").and_then(serde_json::Value::as_str), arguments.get("pluginId").and_then(serde_json::Value::as_str))?;
        match service.site {
            InferenceExecutionSiteV1::Guest => submit_guest_job(context, workspace, service, document_id, &arguments),
            InferenceExecutionSiteV1::Hub => submit_hub_job(context, workspace, service, document_id, &arguments),
        }
    })();
    match submitted {
        Ok((text, structured)) => job_reply(text, structured),
        Err(error) => CallToolResult::tool_error(&error),
    }
}

/// 🧵️ Starts one guest job: the contract is checked on the caller, the job is minted and handed a
/// session-owned handle, and the plugin's guest runs on a job thread of its own so the call answers
/// at once. Every later fact — progress, result, proposal, cancel — lands in the job's journal.
fn submit_guest_job(context: &InferenceToolContext<'_>, workspace: &Arc<HeadlessWorkspace>, service: InferenceService, document_id: &str, arguments: &serde_json::Value) -> Result<(String, serde_json::Value), GatewayError> {
    inference_original_turn(arguments)?;
    validate_inference_request(&service.declared, arguments.get("payload").filter(|value| !value.is_null()), Some(document_id))?;
    let jobs = job_registry();
    let job_id = jobs.begin("inference.submit");
    let cancel = InferenceCancel::default();
    let hook = cancel.0.clone();
    jobs.bind_cancel(&job_id, move || hook.cancel_now());
    retain_guest_job(&job_id, GuestInferenceJob { service: service.clone(), document_id: document_id.to_string(), proposal: None, committed: None, commit_refused: false });
    let payload = InferenceJobHandlePayloadV1 { site: InferenceExecutionSiteV1::Guest, service_id: service.service_id().to_string(), artifact_kind: service.declared.artifact_kind.clone(), plugin_id: service.plugin_id().to_string(), document_id: document_id.to_string(), job_id: job_id.clone(), hub: None };
    let handle = context.handles().mint(HandleKind::Job, context.session.clone(), Attachment::Artifact { artifact_id: document_id.to_string() }, serde_json::to_value(&payload).unwrap_or(serde_json::Value::Null), inference_wall_now_ms());
    spawn_guest_job(Arc::clone(workspace), Arc::clone(context.actions), Arc::clone(context.catalog), service, document_id.to_string(), arguments.clone(), job_id.clone(), cancel)?;
    let page = guest_job_page(&job_id, 0)?;
    Ok((page_text(&page), serde_json::json!({ "jobHandle": handle, "sessionId": context.session.0, "job": page_value(&page) })))
}

#[cfg(not(target_arch = "wasm32"))]
fn spawn_guest_job(workspace: Arc<HeadlessWorkspace>, actions: Arc<ActionAdapter>, catalog: Arc<Catalog>, service: InferenceService, document_id: String, arguments: serde_json::Value, job_id: String, cancel: InferenceCancel) -> Result<(), GatewayError> {
    let thread_job = job_id.clone();
    std::thread::Builder::new()
        .name(format!("semio-inference-{job_id}"))
        .spawn(move || run_guest_job(&workspace, &actions, &catalog, &service, &document_id, &arguments, &thread_job, cancel))
        .map(|_| ())
        .map_err(|error| {
            let error = GatewayError::new(GatewayErrorCode::Internal, format!("could not start the inference job thread: {error}"));
            job_registry().fail(&job_id, error.clone());
            error
        })
}

#[cfg(target_arch = "wasm32")]
fn spawn_guest_job(_workspace: Arc<HeadlessWorkspace>, _actions: Arc<ActionAdapter>, _catalog: Arc<Catalog>, _service: InferenceService, _document_id: String, _arguments: serde_json::Value, job_id: String, _cancel: InferenceCancel) -> Result<(), GatewayError> {
    let error = GatewayError::new(GatewayErrorCode::PluginUnavailable, "a guest inference job needs a native gateway with job threads");
    job_registry().fail(&job_id, error.clone());
    Err(error)
}

/// 🏃️ The job thread: run the guest, then either finish with its result, or — for a service that
/// declares a commit action — offer the proposal that action would commit and park for approval.
fn run_guest_job(workspace: &Arc<HeadlessWorkspace>, actions: &ActionAdapter, catalog: &Catalog, service: &InferenceService, document_id: &str, arguments: &serde_json::Value, job_id: &str, cancel: InferenceCancel) {
    let jobs = job_registry();
    let run = match run_guest_inference(workspace, actions, &service.declared, Some(document_id), arguments, job_id, cancel) {
        Ok(Some(run)) if !jobs.is_cancel_requested(job_id) => run,
        Ok(_) => {
            jobs.mark_cancelled(job_id);
            return;
        }
        Err(_) if jobs.is_cancel_requested(job_id) => {
            jobs.mark_cancelled(job_id);
            return;
        }
        Err(error) => {
            jobs.fail(job_id, error);
            return;
        }
    };
    let result = run.payload.as_deref().and_then(inference_run_result_value).unwrap_or(serde_json::Value::Null);
    let Some(action) = service.commit_action.as_deref() else {
        jobs.succeed(job_id, result);
        return;
    };
    let Some(capability) = commit_capability(catalog, &service.declared.artifact_kind, service.plugin_id(), action) else {
        jobs.fail(job_id, GatewayError::new(GatewayErrorCode::PluginUnavailable, format!("`{}` declares the commit action `{action}`, but `{}` publishes no such action on `{}` in this catalog", service.service_id(), service.plugin_id(), service.declared.artifact_kind)));
        return;
    };
    match guest_proposal(service.service_id(), document_id, action, capability.id.as_str(), &capability_argument_ids(capability), &result) {
        Ok(proposal) => {
            update_guest_job(job_id, |job| job.proposal = Some(proposal.clone()));
            jobs.await_approval(job_id, serde_json::to_value(&proposal).unwrap_or(serde_json::Value::Null));
        }
        Err(error) => {
            jobs.fail(job_id, error);
        }
    }
}

/// 🌎️ Starts one hub-executed job through the route family the hub published for the service.
pub(super) fn submit_hub_job(context: &InferenceToolContext<'_>, workspace: &Arc<HeadlessWorkspace>, service: InferenceService, document_id: &str, arguments: &serde_json::Value) -> Result<(String, serde_json::Value), GatewayError> {
    let route = service.route.clone().ok_or_else(|| GatewayError::new(GatewayErrorCode::Internal, "a hub-executed service resolved without its route"))?;
    let lifetime_ms = arguments.get("lifetimeMs").and_then(serde_json::Value::as_u64).unwrap_or(INFERENCE_JOB_MAX_LIFETIME_MS);
    let request_id = arguments.get("requestId").and_then(serde_json::Value::as_str).map_or_else(mint_inference_request_id, str::to_string);
    let subject = workspace.hub_inference_subject()?;
    let request = HubInferenceSubmitRequestV1::new(service.service_id(), request_id.clone(), lifetime_ms);
    request.validate().map_err(|error| error.to_gateway_error("inference_submit"))?;
    let (base, base_diagnostic) = match workspace.hub_inference_base(document_id) {
        Ok(base) => (Some(base), None),
        Err(error) => (None, Some(error.message.clone())),
    };
    let receipt = workspace.submit_hub_inference_job(document_id, &route, &request)?;
    let payload = InferenceJobHandlePayloadV1 {
        site: InferenceExecutionSiteV1::Hub,
        service_id: service.service_id().to_string(),
        artifact_kind: service.declared.artifact_kind.clone(),
        plugin_id: service.plugin_id().to_string(),
        document_id: document_id.to_string(),
        job_id: receipt.job_id.clone(),
        hub: Some(HubInferenceJobBindingV1 { route, space_id: subject.space_id.clone(), subject_user_id: subject.user_id.clone(), authority_generation: subject.authority_generation, request_id, base: base.clone() }),
    };
    let handle = context.handles().mint(HandleKind::Job, context.session.clone(), Attachment::Artifact { artifact_id: document_id.to_string() }, serde_json::to_value(&payload).unwrap_or(serde_json::Value::Null), inference_wall_now_ms());
    let page = hub_receipt_page(&service, document_id, &receipt);
    Ok((page_text(&page), serde_json::json!({ "jobHandle": handle, "sessionId": context.session.0, "job": page_value(&page), "baseBinding": base, "baseBindingUnavailable": base_diagnostic })))
}

/// 🌎️ The minimal service identity a hub job's handle carries, enough to project its pages.
pub(super) fn handle_service(payload: &InferenceJobHandlePayloadV1) -> InferenceService {
    InferenceService {
        declared: DeclaredInference {
            owner: payload.plugin_id.clone(),
            artifact_kind: payload.artifact_kind.clone(),
            artifact_schema: payload.artifact_kind.clone(),
            artifact_schema_version: 1,
            inference_schema: payload.service_id.clone(),
            inference_schema_version: 1,
            algorithm_version: 1,
            policy_version: 1,
            contributor: payload.plugin_id.clone(),
            depends_on: Vec::new(),
            payload: None,
        },
        site: payload.site,
        route: payload.hub.as_ref().map(|hub| hub.route.clone()),
        commit_action: None,
    }
}
//#endregion 💡️InferenceJobTools
