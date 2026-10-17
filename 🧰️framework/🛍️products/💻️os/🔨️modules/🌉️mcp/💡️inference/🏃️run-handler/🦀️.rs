//! 🏃️ Run handler of inference: the guest run record, the original retained turn and authority, the guest and hub inference runs and the merged run fields. See [`super`].

use super::*;

//#region 💡️InferenceJobTools
/// 🏁 One finished guest run: the guest's own result bytes and whether it reported completion.
struct GuestInferenceRun {
    pub inference_schema: String,
    pub complete: bool,
    pub payload: Option<Vec<u8>>,
}

/// 🎟️ All retained axes come from the original turn's explicit grant.
pub(crate) fn inference_original_retained(grant:&serde_json::Value)->Result<semio_framework_value::RetainedCloneGrant,GatewayError>{
    let invalid=||inference_field_invalid("turn.grant","all five explicit retained axes are required within the cross-platform address range");
    let fields=grant.as_object().ok_or_else(invalid)?;
    if fields.len()!=5{return Err(invalid())}
    let axis=|name|fields.get(name).and_then(serde_json::Value::as_u64).and_then(|number|u32::try_from(number).ok()).map(|number|number as usize).ok_or_else(invalid);
    Ok(semio_framework_value::RetainedCloneGrant{maximum_items:axis("maximumItems")?,maximum_copy_bytes:axis("maximumCopyBytes")?,maximum_capacity_bytes:axis("maximumCapacityBytes")?,maximum_release_bytes:axis("maximumReleaseBytes")?,maximum_depth:axis("maximumDepth")?})
}

/// 🎟️ The command borrows original cancellation and carries its one caller turn and work policy.
struct InferenceCallerAuthority<'a>{pub turn:semio_framework_actor::RetainedTurnInput,pub retained_policy:semio_framework_value::RetainedCloneGrant,pub revision:u64,pub cancellation_id:&'a str,pub work_units:u64,pub maximum_elapsed_milliseconds:u64}

/// 🎟️ Exact JSON identities are mandatory before any guest command or job is born.
pub(crate) fn inference_original_turn(arguments: &serde_json::Value) -> Result<InferenceCallerAuthority<'_>, GatewayError> {
    let integer = |value: &serde_json::Value, name: &str, minimum: u64| value.get(name)
        .and_then(serde_json::Value::as_u64)
        .filter(|number| *number >= minimum && *number <= 9007199254740991)
        .ok_or_else(|| inference_field_invalid(name, "an explicit exact unsigned JSON identity or work bound is required"));
    let source = arguments.get("turn").and_then(serde_json::Value::as_object)
        .filter(|source| source.len() == 5)
        .ok_or_else(|| inference_field_invalid("turn", "the original operation, generation, epoch, work and total grant are required"))?;
    let turn_value = &arguments["turn"];
    let work = source.get("work").filter(|value| value.as_object().is_some_and(|fields| fields.len() == 3))
        .ok_or_else(|| inference_field_invalid("turn.work", "the original clock binding, absolute deadline and total fuel are required"))?;
    let turn = semio_framework_actor::RetainedTurnInput {
        operation: integer(turn_value, "operation", 1)?,
        generation: integer(turn_value, "generation", 1)?,
        epoch: integer(turn_value, "epoch", 1)?,
        work: semio_framework_actor::OriginalActorTurnWork {
            clock_binding: integer(work, "clockBinding", 1)?,
            absolute_deadline_us: integer(work, "absoluteDeadlineUs", 1)?,
            fuel: integer(work, "fuel", 0)?,
        },
        grant: inference_original_retained(source.get("grant")
            .ok_or_else(|| inference_field_invalid("turn.grant", "the original total grant is required"))?)?,
    };
    let policy = arguments.get("retainedPolicy").ok_or_else(|| inference_field_invalid("retainedPolicy", "an explicit original call policy is required"))?;
    let retained_policy = inference_original_retained(policy)
        .map_err(|_| inference_field_invalid("retainedPolicy", "all five explicit call limits are required"))?;
    let total = turn.grant;
    if retained_policy.maximum_items > total.maximum_items
        || retained_policy.maximum_copy_bytes > total.maximum_copy_bytes
        || retained_policy.maximum_capacity_bytes > total.maximum_capacity_bytes
        || retained_policy.maximum_release_bytes > total.maximum_release_bytes
        || retained_policy.maximum_depth > total.maximum_depth {
        return Err(inference_field_invalid("retainedPolicy", "the call policy exceeds its original total grant"));
    }
    let work_units = integer(arguments, "workUnits", 1)?;
    if work_units > turn.work.fuel {
        return Err(inference_field_invalid("workUnits", "the call work limit exceeds its original remaining fuel"));
    }
    let cancellation_id = arguments.get("cancellationId").and_then(serde_json::Value::as_str)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| inference_field_invalid("cancellationId", "the original nonempty cancellation identity is required"))?;
    Ok(InferenceCallerAuthority {
        turn, retained_policy, revision: integer(arguments, "revision", 0)?, cancellation_id,
        work_units, maximum_elapsed_milliseconds: integer(arguments, "maximumElapsedMilliseconds", 1)?,
    })
}

/// 🏃️ The one guest engine both `inference_run` and a guest job drive: bind the artifact's document
/// into the request, route it to the service's plugin guest, run it under `cancel`, reporting each
/// step on `job_id`. `Ok(None)` is a run the caller's cancel stopped.
pub(super) fn run_guest_inference(workspace: &Arc<HeadlessWorkspace>, actions: &ActionAdapter, item: &DeclaredInference, artifact_id: Option<&str>, arguments: &serde_json::Value, job_id: &str, cancel: InferenceCancel) -> Result<Option<GuestInferenceRun>, GatewayError> {
    let authority=inference_original_turn(arguments)?;
    let jobs = job_registry();
    jobs.report_progress(job_id, INFERENCE_BINDING_PROGRESS_START, Some(match artifact_id {
        Some(artifact_id) => format!("binding artifact `{artifact_id}` into the request body"),
        None => "no artifact binding declared — the caller's own body travels verbatim".to_string(),
    }));
    let artifact_document = match resolve_inference_artifact_document(workspace, item, artifact_id, job_id, &cancel.0) {
        Ok(document) => document,
        Err(_) if jobs.is_cancel_requested(job_id) => return Ok(None),
        Err(error) => return Err(error.with_details(serde_json::json!({ "field": "artifactId" }))),
    };
    let command = InferCommand {
        plugin_id: item.route_plugin_id().to_string(),
        artifact_kind: item.artifact_kind.clone(),
        inference_schema: item.inference_schema.clone(),
        revision: authority.revision,
        turn: authority.turn,
        retained_policy: authority.retained_policy,
        cancellation_id: authority.cancellation_id.to_string(),
        work_units: authority.work_units,
        maximum_elapsed_milliseconds: authority.maximum_elapsed_milliseconds,
        canonical_payload: inference_run_payload_bytes(arguments),
        artifact_id: artifact_id.unwrap_or_default().to_string(),
        artifact_document,
        cancel,
    };
    jobs.report_progress(job_id, 0.25, Some(format!("routing `{}/{}` to `{}`", command.artifact_kind, command.inference_schema, command.plugin_id)));
    if jobs.is_cancel_requested(job_id) {
        return Ok(None);
    }
    jobs.report_progress(job_id, 0.35, Some("running in the plugin's guest".to_string()));
    match actions.run_inference(INFERENCE_ROUTED_INSTANCE, command) {
        Err(_) if jobs.is_cancel_requested(job_id) => Ok(None),
        Err(error) => Err(error),
        Ok(outcome) => Ok(Some(GuestInferenceRun { inference_schema: outcome.inference_schema, complete: outcome.complete, payload: outcome.payload })),
    }
}

/// 💡️ Runs one declared inference to completion in a single call — the synchronous sibling of
/// `inference_submit`, resolved through the same service model. A guest service runs here, in its
/// plugin's guest; a hub-executed service is submitted to the hub and its events are followed to a
/// terminal page. Nothing is committed: an offered proposal is returned, never applied.
///
/// 🎫️ The job is minted BEFORE the contract is checked, and that ordering is load-bearing:
/// `JobRegistry::begin` binds the id to this call's `_meta.progressToken`, so every step from
/// here on — including a REFUSAL — is pushed to the client as `notifications/progress` and is
/// readable afterwards with `job_get`/`job_cancel`.
pub(super) fn inference_run_handler(context: &InferenceToolContext<'_>, arguments: serde_json::Value) -> CallToolResult {
    if let Err(error) = authorize_inference(context.policy(), context.principal, &inference_run_capability()) {
        return CallToolResult::tool_error(&error);
    }
    let Some(artifact_kind) = arguments.get("artifactKind").and_then(serde_json::Value::as_str) else {
        return CallToolResult::tool_error(&inference_input_invalid("artifactKind is required"));
    };
    let Some(inference_schema) = arguments.get("inferenceSchema").and_then(serde_json::Value::as_str) else {
        return CallToolResult::tool_error(&inference_input_invalid("inferenceSchema is required"));
    };
    let workspace = match context.workspace("inference_run") {
        Ok(workspace) => workspace,
        Err(error) => return CallToolResult::tool_error(&error),
    };
    let service = match declared_inferences_for_workspace(workspace).and_then(|declared| Ok((declared, workspace.hub_inference_services()?))).and_then(|(declared, hub)| select_inference_service(&declared, &hub, artifact_kind, Some(inference_schema), arguments.get("pluginId").and_then(serde_json::Value::as_str))) {
        Ok(service) => service,
        Err(error) => return CallToolResult::tool_error(&error),
    };
    let requested_artifact = arguments.get("artifactId").and_then(serde_json::Value::as_str);
    if service.site == InferenceExecutionSiteV1::Hub {
        return match requested_artifact {
            Some(document_id) => run_hub_inference(context, workspace, service, document_id, &arguments),
            None => CallToolResult::tool_error(&inference_field_invalid("artifactId", format!("`{}` is executed by the bound hub on a document: name it with `artifactId`", service.service_id()))),
        };
    }
    let item = service.declared.clone();
    let caller_payload = arguments.get("payload").filter(|value| !value.is_null());
    let authority=match inference_original_turn(&arguments){Ok(authority)=>authority,Err(error)=>return CallToolResult::tool_error(&error)};
    let cancellation_id=authority.cancellation_id;
    let cancel = InferenceCancel::default();

    let jobs = job_registry();
    let job_id = jobs.begin("inference.run");
    let base = serde_json::json!({
        "jobId": job_id,
        "artifactKind": item.artifact_kind,
        "inferenceSchema": item.inference_schema,
        "pluginId": item.route_plugin_id(),
        "cancellationId": cancellation_id,
        "artifactId": requested_artifact.unwrap_or_default(),
        "site": InferenceExecutionSiteV1::Guest,
    });
    let hook = cancel.0.clone();
    jobs.bind_cancel(&job_id, move || hook.cancel_now());
    jobs.report_progress(&job_id, 0.05, Some(format!("checking `{}` against its published payload contract", item.inference_schema)));
    if let Err(error) = validate_inference_request(&item, caller_payload, requested_artifact) {
        let field = inference_error_field(&error);
        let error = error.with_details(merge_inference_run_fields(base.clone(), serde_json::json!({ "field": field })));
        jobs.fail(&job_id, error.clone());
        return CallToolResult::tool_error(&error);
    }
    let cancelled = |job_id: &str, base: serde_json::Value| {
        job_registry().mark_cancelled(job_id);
        CallToolResult::ok(vec![ContentBlock::Text { text: format!("inference job {job_id} was cancelled") }], Some(merge_inference_run_fields(base, serde_json::json!({ "status": "CANCELLED", "complete": false }))))
    };
    match run_guest_inference(workspace, context.actions, &item, requested_artifact, &arguments, &job_id, cancel) {
        Ok(Some(_)) | Err(_) if jobs.is_cancel_requested(&job_id) => cancelled(&job_id, base),
        Ok(None) => cancelled(&job_id, base),
        Ok(Some(run)) => {
            let structured = merge_inference_run_fields(base, serde_json::json!({ "status": if run.complete{"SUCCEEDED"}else{"RUNNING"}, "complete": run.complete, "payload": run.payload.as_deref().and_then(inference_run_result_value), "payloadBytes": run.payload.as_ref().map_or(0,Vec::len) }));
            jobs.succeed(&job_id, structured.clone());
            CallToolResult::ok(vec![ContentBlock::Text { text: format!("`{}` produced {} byte(s) (complete: {})", run.inference_schema, run.payload.as_ref().map_or(0,Vec::len), run.complete) }], Some(structured))
        }
        Err(error) => {
            let field = inference_error_field(&error);
            let error = error.with_details(merge_inference_run_fields(base, serde_json::json!({ "field": field })));
            jobs.fail(&job_id, error.clone());
            CallToolResult::tool_error(&error)
        }
    }
}

/// ⏳️ How often `inference_run` re-reads a hub-executed job's events while it follows it.
const HUB_INFERENCE_RUN_POLL_MS: u64 = 200;

/// 🌎️ `inference_run` on a hub-executed service: submit, then follow the job's events until the hub
/// reports it terminal or offering a proposal, within the job's own lifetime.
fn run_hub_inference(context: &InferenceToolContext<'_>, workspace: &Arc<HeadlessWorkspace>, service: InferenceService, document_id: &str, arguments: &serde_json::Value) -> CallToolResult {
    let followed = (|| {
        let (_, submitted) = submit_hub_job(context, workspace, service.clone(), document_id, arguments)?;
        let handle = submitted.get("jobHandle").and_then(serde_json::Value::as_str).unwrap_or_default().to_string();
        let payload = context.job(workspace, &handle)?;
        let hub = hub_scope_route(&payload)?;
        let deadline = std::time::Instant::now() + std::time::Duration::from_millis(arguments.get("lifetimeMs").and_then(serde_json::Value::as_u64).unwrap_or(INFERENCE_JOB_MAX_LIFETIME_MS));
        loop {
            let page = hub_events_page(&service, document_id, &workspace.read_hub_inference_job_events(document_id, &hub.route, &payload.job_id, 0)?);
            let settled = !matches!(page.state, InferenceJobStateV1::Accepted | InferenceJobStateV1::Running);
            if settled || std::time::Instant::now() >= deadline {
                let status = match page.state {
                    InferenceJobStateV1::Succeeded => "SUCCEEDED",
                    InferenceJobStateV1::Cancelled => "CANCELLED",
                    InferenceJobStateV1::Failed => "FAILED",
                    InferenceJobStateV1::Accepted | InferenceJobStateV1::Running => "RUNNING",
                };
                return Ok((page_text(&page), serde_json::json!({ "jobId": page.job_id, "status": status, "artifactKind": service.declared.artifact_kind, "inferenceSchema": service.service_id(), "pluginId": service.plugin_id(), "artifactId": document_id, "site": InferenceExecutionSiteV1::Hub, "jobHandle": handle, "job": page_value(&page) })));
            }
            std::thread::sleep(std::time::Duration::from_millis(HUB_INFERENCE_RUN_POLL_MS));
        }
    })();
    match followed {
        Ok((text, structured)) => job_reply(text, structured),
        Err(error) => CallToolResult::tool_error(&error),
    }
}

/// 🧩️ Folds the terminal fields onto the invariant ones — one object, built once, so the tool reply
/// and the job registry's own retained result are literally the same value.
fn merge_inference_run_fields(mut base: serde_json::Value, extra: serde_json::Value) -> serde_json::Value {
    if let (Some(base_map), Some(extra_map)) = (base.as_object_mut(), extra.as_object()) {
        for (key, value) in extra_map {
            base_map.insert(key.clone(), value.clone());
        }
    }
    base
}

/// 📈️ The span of `inference_run`'s job progress its artifact binding reports into, between the
/// binding row and routing.
pub(super) const INFERENCE_BINDING_PROGRESS_START: f64 = 0.15;

pub(super) const INFERENCE_BINDING_PROGRESS_SPAN: f64 = 0.1;

/// 🔢️ `RoutingArtifactChannel` resolves an inference's plugin from the command's own `pluginId`, so
/// the `instance` slot the mutation protocol encodes carries no meaning here — a fixed, documented
/// zero rather than a fabricated per-capability slot.
const INFERENCE_ROUTED_INSTANCE: u32 = 0;
//#endregion 💡️InferenceJobTools
