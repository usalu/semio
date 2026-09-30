
use super::*;
use semio_framework_async::{CancelToken, TraceId};

fn sample_job_id() -> String {
    "ab".repeat(16)
}

/// 🆔️ A well-formed 32-lower-hex job id that is never the fixture's own `sampleJobId` — a hostile
/// "foreign job" candidate that happens to equal the sample asserts nothing at all.
fn foreign_job_id() -> String {
    let candidate = "2".repeat(32);
    if sample_job_id() == candidate {
        "3".repeat(32)
    } else {
        candidate
    }
}

fn sample_proposal_hash() -> String {
    "cd".repeat(32)
}

fn context(cancel: &CancelToken) -> OperationContext {
    OperationContext { actor: 1, generation: 0, trace: TraceId(1), lane: 1, deadline_ms: Some(u64::MAX), cancel: cancel.child_now(), capability: None }
}

fn scope() -> DocumentScope {
    DocumentScope::new("space:alpha".to_string(), "doc:tokyo".to_string())
}

/// 🎭️ A scripted transport: it records every request it is handed and replays one canned reply,
/// so the client's own decoding, bounds and error mapping are exercised with no hub at all.
struct ScriptedTransport {
    replies: std::sync::Mutex<Vec<Result<InferenceHubResponseV1, InferenceHubTransportErrorV1>>>,
    seen: std::sync::Mutex<Vec<InferenceHubRequestV1>>,
}

impl ScriptedTransport {
    fn new(replies: Vec<Result<InferenceHubResponseV1, InferenceHubTransportErrorV1>>) -> Self {
        Self { replies: std::sync::Mutex::new(replies), seen: std::sync::Mutex::new(Vec::new()) }
    }

    fn ok(status: u16, body: serde_json::Value) -> Self {
        Self::new(vec![Ok(InferenceHubResponseV1 { status, body: serde_json::to_vec(&body).expect("scripted body") })])
    }

    fn requests(&self) -> Vec<InferenceHubRequestV1> {
        self.seen.lock().expect("scripted transport lock").clone()
    }
}

impl InferenceHubTransport for ScriptedTransport {
    async fn request(&self, context: &OperationContext, request: &InferenceHubRequestV1) -> Result<InferenceHubResponseV1, InferenceHubTransportErrorV1> {
        if context.cancel.is_cancelled_now() {
            return Err(InferenceHubTransportErrorV1::Cancelled);
        }
        self.seen.lock().expect("scripted transport lock").push(request.clone());
        let mut replies = self.replies.lock().expect("scripted transport lock");
        if replies.is_empty() {
            return Err(InferenceHubTransportErrorV1::Unavailable);
        }
        replies.remove(0)
    }
}

fn block_on<F: std::future::Future>(future: F) -> F::Output {
    tokio::runtime::Builder::new_current_thread().enable_all().build().expect("current-thread runtime").block_on(future)
}

//#region 🧪️Vocabulary


#[test]
fn a_503_inference_unavailable_becomes_a_retryable_plugin_unavailable_that_names_the_missing_binding() {
    let error = InferenceRouteErrorV1::Unavailable.to_gateway_error("inference_submit");
    assert_eq!(error.code, GatewayErrorCode::PluginUnavailable);
    assert!(error.retryable, "a hub without a trusted binding is a tier, not a permanent failure");
    assert!(error.message.contains("inference.unavailable"), "{}", error.message);
    assert!(error.message.contains("features.inference"), "the caller must learn which binding is missing: {}", error.message);
    assert_eq!(error.details["httpStatus"], 503);
    assert_eq!(error.details["inferenceCode"], "inference.unavailable");

    let commit = InferenceRouteErrorV1::CommitUnavailable.to_gateway_error("inference_approve");
    assert_eq!(commit.code, GatewayErrorCode::PluginUnavailable);
    assert!(commit.retryable);
    assert!(commit.message.contains("approval.commit-unavailable") && commit.message.contains("nothing was applied"), "{}", commit.message);

    assert_eq!(InferenceRouteErrorV1::Denied.to_gateway_error("x").code, GatewayErrorCode::PermissionDenied);
    assert!(!InferenceRouteErrorV1::Denied.to_gateway_error("x").retryable);
    assert_eq!(InferenceRouteErrorV1::NotFound.to_gateway_error("x").code, GatewayErrorCode::NotFound);
    assert_eq!(InferenceRouteErrorV1::Invalid.to_gateway_error("x").code, GatewayErrorCode::InputInvalid);
    assert_eq!(InferenceRouteErrorV1::Bounds.to_gateway_error("x").code, GatewayErrorCode::BudgetExceeded);
    assert_eq!(InferenceRouteErrorV1::Conflict.to_gateway_error("x").code, GatewayErrorCode::PreconditionFailed);
    assert_eq!(InferenceRouteErrorV1::Expired.to_gateway_error("x").code, GatewayErrorCode::PreconditionFailed);
    assert_eq!(InferenceRouteErrorV1::Cancelled.to_gateway_error("x").code, GatewayErrorCode::Cancelled);
    assert_eq!(InferenceRouteErrorV1::Storage.to_gateway_error("x").code, GatewayErrorCode::PluginUnavailable);
    assert_eq!(InferenceRouteErrorV1::Capacity.to_gateway_error("x").code, GatewayErrorCode::PluginUnavailable);
}


//#endregion 🧪️Vocabulary

//#region 🧪️WireShapes
#[test]
fn a_submit_intent_encodes_within_the_fixed_bound_and_every_hostile_field_is_refused() {
    let request = HubInferenceSubmitRequestV1::new("test.document.inference", sample_job_id(), INFERENCE_JOB_MAX_LIFETIME_MS);
    let encoded = request.encode().expect("a well-formed intent encodes");
    assert!(encoded.len() <= INFERENCE_REQUEST_MAX_BYTES);
    let decoded: HubInferenceSubmitRequestV1 = serde_json::from_slice(&encoded).expect("closed round trip");
    assert_eq!(decoded, request);

    let hostile: Vec<(&str, Box<dyn Fn(&mut HubInferenceSubmitRequestV1)>)> = vec![
        ("wrong-schema", Box::new(|value: &mut HubInferenceSubmitRequestV1| value.schema = "semio.hub.inference-request/v2".into())),
        ("wrong-version", Box::new(|value: &mut HubInferenceSubmitRequestV1| value.version = 2)),
        ("non-hex-request-id", Box::new(|value: &mut HubInferenceSubmitRequestV1| value.request_id = "Z".repeat(32))),
        ("short-request-id", Box::new(|value: &mut HubInferenceSubmitRequestV1| value.request_id.truncate(31))),
        ("malformed-service", Box::new(|value: &mut HubInferenceSubmitRequestV1| value.service_id = "../test.document".into())),
        ("wrong-policy", Box::new(|value: &mut HubInferenceSubmitRequestV1| value.policy_version = 2)),
        ("zero-lifetime", Box::new(|value: &mut HubInferenceSubmitRequestV1| value.lifetime_ms = 0)),
        ("over-lifetime", Box::new(|value: &mut HubInferenceSubmitRequestV1| value.lifetime_ms = INFERENCE_JOB_MAX_LIFETIME_MS + 1)),
    ];
    for (name, mutate) in &hostile {
        let mut candidate = request.clone();
        mutate(&mut candidate);
        assert_eq!(candidate.validate(), Err(InferenceRouteErrorV1::Invalid), "{name} was admitted");
    }
    assert!(
        serde_json::from_str::<HubInferenceSubmitRequestV1>(&format!(
            "{{\"schema\":\"{HUB_INFERENCE_REQUEST_SCHEMA}\",\"version\":1,\"requestId\":\"{}\",\"serviceId\":\"test.document.inference\",\"policyVersion\":1,\"lifetimeMs\":1,\"mapPack\":\"smuggled\"}}",
            sample_job_id()
        ))
        .is_err(),
        "a client may never smuggle an extra field past the closed intent"
    );
}

#[test]
fn an_approval_intent_carries_only_the_job_and_its_exact_proposal_digest() {
    let request = HubInferenceApprovalRequestV1::new(sample_job_id(), sample_proposal_hash());
    assert!(request.encode().is_ok());
    for (name, mutate) in [("short-hash", 63_usize), ("long-hash", 65)] {
        let mut candidate = request.clone();
        candidate.proposal_hash = "a".repeat(mutate);
        assert_eq!(candidate.validate(), Err(InferenceRouteErrorV1::Invalid), "{name} was admitted");
    }
    let mut foreign_job = request.clone();
    foreign_job.job_id = "not-hex-at-all".into();
    assert_eq!(foreign_job.validate(), Err(InferenceRouteErrorV1::Invalid));
    assert!(
        serde_json::from_str::<HubInferenceApprovalRequestV1>(&format!(
            "{{\"schema\":\"{HUB_INFERENCE_APPROVAL_SCHEMA}\",\"version\":1,\"jobId\":\"{}\",\"proposalHash\":\"{}\",\"actor\":\"user:forged\"}}",
            sample_job_id(),
            sample_proposal_hash()
        ))
        .is_err()
    );
}

#[test]
fn the_four_client_paths_are_exact_percent_encoded_hub_paths() {
    let scope = scope();
    let job = sample_job_id();
    assert_eq!(hub_inference_jobs_path(&scope, "inference/document"), "/spaces/space%3Aalpha/documents/doc%3Atokyo/inference/document/jobs");
    assert_eq!(hub_inference_job_events_path(&scope, "inference/document", &job, 4), format!("/spaces/space%3Aalpha/documents/doc%3Atokyo/inference/document/jobs/{job}/events?after=4"));
    assert_eq!(hub_inference_job_cancel_path(&scope, "inference/document", &job), format!("/spaces/space%3Aalpha/documents/doc%3Atokyo/inference/document/jobs/{job}/cancel"));
    assert_eq!(hub_inference_job_approval_path(&scope, "inference/document", &job), format!("/spaces/space%3Aalpha/documents/doc%3Atokyo/inference/document/jobs/{job}/approval"));
}

#[test]
fn a_reply_decodes_by_its_closed_code_and_never_by_its_ambiguous_status() {
    let commit = InferenceHubResponseV1 { status: 503, body: br#"{"schema":"semio.hub.inference-error/v1","code":"approval.commit-unavailable"}"#.to_vec() };
    assert_eq!(decode_inference_reply::<HubInferenceApprovalReceiptV1>(&commit).unwrap_err(), InferenceRouteErrorV1::CommitUnavailable);
    let storage = InferenceHubResponseV1 { status: 503, body: br#"{"schema":"semio.hub.inference-error/v1","code":"inference.storage"}"#.to_vec() };
    assert_eq!(decode_inference_reply::<HubInferenceJobReceiptV1>(&storage).unwrap_err(), InferenceRouteErrorV1::Storage);
    let cancelled = InferenceHubResponseV1 { status: 409, body: br#"{"schema":"semio.hub.inference-error/v1","code":"inference.cancelled"}"#.to_vec() };
    assert_eq!(decode_inference_reply::<HubInferenceEventPageV1>(&cancelled).unwrap_err(), InferenceRouteErrorV1::Cancelled);
    let opaque = InferenceHubResponseV1 { status: 403, body: b"<html>proxy</html>".to_vec() };
    assert_eq!(decode_inference_reply::<HubInferenceJobReceiptV1>(&opaque).unwrap_err(), InferenceRouteErrorV1::Denied);
    let foreign = InferenceHubResponseV1 { status: 500, body: br#"{"schema":"some.other/v1","code":"inference.denied"}"#.to_vec() };
    assert_eq!(decode_inference_reply::<HubInferenceJobReceiptV1>(&foreign).unwrap_err(), InferenceRouteErrorV1::Unavailable, "an error body that is not the hub's own closed shape is never trusted for its code");
}

#[test]
fn a_two_hundred_reply_must_declare_its_own_exact_schema_and_carry_no_unknown_field() {
    let job = sample_job_id();
    let good = serde_json::json!({ "schema": HUB_INFERENCE_RECEIPT_SCHEMA, "jobId": job, "state": "succeeded", "proposalState": "offered", "proposalHash": sample_proposal_hash(), "cursor": 4, "expiresAtMs": 1_000 });
    let receipt: HubInferenceJobReceiptV1 = decode_inference_reply(&InferenceHubResponseV1 { status: 200, body: serde_json::to_vec(&good).expect("body") }).expect("a well-formed receipt decodes");
    assert_eq!(receipt.state, InferenceJobStateV1::Succeeded);
    assert_eq!(receipt.proposal_state, InferenceProposalStateV1::Offered);

    let mut wrong_schema = good.clone();
    wrong_schema["schema"] = serde_json::json!(HUB_INFERENCE_EVENTS_SCHEMA);
    assert_eq!(decode_inference_reply::<HubInferenceJobReceiptV1>(&InferenceHubResponseV1 { status: 200, body: serde_json::to_vec(&wrong_schema).expect("body") }).unwrap_err(), InferenceRouteErrorV1::Invalid);

    let mut leaked = good.clone();
    leaked["proposal"] = serde_json::json!("private bytes");
    assert_eq!(
        decode_inference_reply::<HubInferenceJobReceiptV1>(&InferenceHubResponseV1 { status: 200, body: serde_json::to_vec(&leaked).expect("body") }).unwrap_err(),
        InferenceRouteErrorV1::Invalid,
        "a private field appearing on the wire must fail loudly, never be dropped"
    );

    assert_eq!(decode_inference_reply::<HubInferenceJobReceiptV1>(&InferenceHubResponseV1 { status: 200, body: vec![b'{'; INFERENCE_RESPONSE_MAX_BYTES + 1] }).unwrap_err(), InferenceRouteErrorV1::Bounds);
}




//#endregion 🧪️WireShapes

//#region 🧪️Calls
#[test]
fn a_submit_call_posts_the_bounded_closed_intent_to_the_exact_job_route() {
    let transport =
        ScriptedTransport::ok(200, serde_json::json!({ "schema": HUB_INFERENCE_RECEIPT_SCHEMA, "jobId": sample_job_id(), "state": "accepted", "proposalState": "none", "proposalHash": serde_json::Value::Null, "cursor": 0, "expiresAtMs": 9 }));
    let cancel = CancelToken::root_now();
    let request = HubInferenceSubmitRequestV1::new("test.document.inference", sample_job_id(), 1_000);
    let receipt = block_on(submit_hub_inference_job(&transport, &context(&cancel), "https://hub.invalid", &scope(), "inference/document", &request)).expect("scripted receipt");
    assert_eq!(receipt.job_id, sample_job_id());
    assert_eq!(receipt.proposal_hash, None);
    let seen = transport.requests();
    assert_eq!(seen.len(), 1);
    assert_eq!(seen[0].method, InferenceHubMethodV1::Post);
    assert_eq!(seen[0].path, hub_inference_jobs_path(&scope(), "inference/document"));
    assert!(seen[0].body.len() <= INFERENCE_REQUEST_MAX_BYTES);
    assert_eq!(seen[0].maximum_response_bytes, INFERENCE_RESPONSE_MAX_BYTES);
}

#[test]
fn an_events_call_refuses_a_foreign_job_id_or_an_out_of_range_cursor_before_any_request() {
    let transport = ScriptedTransport::new(Vec::new());
    let cancel = CancelToken::root_now();
    assert_eq!(block_on(read_hub_inference_job_events(&transport, &context(&cancel), "https://hub.invalid", &scope(), "inference/document", "not-a-job", 0)).unwrap_err(), InferenceRouteErrorV1::Invalid);
    assert_eq!(block_on(read_hub_inference_job_events(&transport, &context(&cancel), "https://hub.invalid", &scope(), "inference/document", &sample_job_id(), INFERENCE_PROGRESS_MAX_CURSOR + 1)).unwrap_err(), InferenceRouteErrorV1::Invalid);
    assert!(transport.requests().is_empty(), "a malformed read never reaches the network");
}

#[test]
fn an_already_cancelled_operation_context_never_reaches_the_hub_and_maps_to_cancelled() {
    let transport = ScriptedTransport::ok(200, serde_json::json!({}));
    let cancel = CancelToken::root_now();
    cancel.cancel_now();
    let error = block_on(cancel_hub_inference_job(&transport, &context(&cancel), "https://hub.invalid", &scope(), "inference/document", &sample_job_id())).unwrap_err();
    assert_eq!(error, InferenceRouteErrorV1::Cancelled);
    assert_eq!(error.to_gateway_error("inference_cancel").code, GatewayErrorCode::Cancelled);
    assert!(transport.requests().is_empty(), "a cancelled call is never sent");
}

#[test]
fn a_transport_failure_maps_onto_the_closed_route_vocabulary_and_never_a_fabricated_success() {
    for (transport_error, expected) in [
        (InferenceHubTransportErrorV1::Unauthorized, InferenceRouteErrorV1::Denied),
        (InferenceHubTransportErrorV1::DeadlineExceeded, InferenceRouteErrorV1::Unavailable),
        (InferenceHubTransportErrorV1::Unavailable, InferenceRouteErrorV1::Unavailable),
        (InferenceHubTransportErrorV1::ResourceLimit, InferenceRouteErrorV1::Bounds),
        (InferenceHubTransportErrorV1::InvalidRequest("authority mismatch"), InferenceRouteErrorV1::Invalid),
        (InferenceHubTransportErrorV1::Cancelled, InferenceRouteErrorV1::Cancelled),
    ] {
        let transport = ScriptedTransport::new(vec![Err(transport_error.clone())]);
        let cancel = CancelToken::root_now();
        let approval = HubInferenceApprovalRequestV1::new(sample_job_id(), sample_proposal_hash());
        assert_eq!(block_on(approve_hub_inference_job(&transport, &context(&cancel), "https://hub.invalid", &scope(), "inference/document", &approval)).unwrap_err(), expected, "{transport_error:?}");
    }
}

#[test]
fn an_approval_receipt_must_bind_the_exact_job_proposal_and_durable_undo_scope() {
    let scope = scope();
    let request = HubInferenceApprovalRequestV1::new(sample_job_id(), sample_proposal_hash());
    let exact = serde_json::json!({
        "schema": HUB_INFERENCE_APPROVAL_RECEIPT_SCHEMA,
        "jobId": request.job_id.clone(),
        "mutationId": "aa".repeat(16),
        "commandHash": "bb".repeat(32),
        "proposalHash": request.proposal_hash.clone(),
        "applied": true,
        "undo": {
            "targetId": "cc".repeat(16),
            "expectedCurrent": {
                "documentId": scope.document_id.clone(),
                "headEditOrdinal": 2,
                "headEditId": "aa".repeat(16),
                "lastCommitSeq": 2,
                "chainSha256": "dd".repeat(32),
            },
        },
    });
    let transport = ScriptedTransport::ok(200, exact.clone());
    let cancel = CancelToken::root_now();
    let receipt = block_on(approve_hub_inference_job(&transport, &context(&cancel), "https://hub.invalid", &scope, "inference/document", &request)).expect("exact approval receipt");
    assert_eq!(receipt.undo.target_id, "cc".repeat(16));

    for (name, candidate) in [
        ("foreign-job", {
            let mut value = exact.clone();
            value["jobId"] = serde_json::json!(foreign_job_id());
            value
        }),
        ("foreign-proposal", {
            let mut value = exact.clone();
            value["proposalHash"] = serde_json::json!("11".repeat(32));
            value
        }),
        ("long-mutation", {
            let mut value = exact.clone();
            value["mutationId"] = serde_json::json!("11".repeat(32));
            value
        }),
        ("malformed-command", {
            let mut value = exact.clone();
            value["commandHash"] = serde_json::json!("g".repeat(64));
            value
        }),
        ("guessed-target", {
            let mut value = exact.clone();
            value["undo"]["targetId"] = serde_json::json!("short");
            value
        }),
        ("foreign-document", {
            let mut value = exact.clone();
            value["undo"]["expectedCurrent"]["documentId"] = serde_json::json!("other-document");
            value
        }),
    ] {
        let transport = ScriptedTransport::ok(200, candidate);
        assert_eq!(block_on(approve_hub_inference_job(&transport, &context(&CancelToken::root_now()), "https://hub.invalid", &scope, "inference/document", &request)), Err(InferenceRouteErrorV1::Conflict), "{name}",);
    }
}



#[test]
fn a_retained_local_wait_is_interrupted_by_its_own_operation_label_and_by_nothing_else() {
    let label = inference_operation_label("space:alpha", "doc:tokyo", Some(&sample_job_id()));
    let other = inference_operation_label("space:alpha", "doc:tokyo", None);
    let cancel = CancelToken::root_now();
    retain_inference_operation(&label, cancel.clone());
    assert!(!interrupt_inference_operation("inference:space:alpha/doc:other/*"), "an unrelated label interrupts nothing");
    assert!(!cancel.is_cancelled_now());
    assert!(!interrupt_inference_operation(&other), "the document-wide label is a different retained wait");
    assert!(interrupt_inference_operation(&label));
    assert!(cancel.is_cancelled_now(), "the retained token is really cancelled, not merely reported");
    release_inference_operation(&label);
    assert!(!interrupt_inference_operation(&label), "a released wait is gone");
}
//#endregion 🧪️Calls

//#region 🧪️Policy
fn principal(scopes: &[&str]) -> AgentPrincipal {
    AgentPrincipal::from_scope_names("agent:test", "test agent", &scopes.iter().map(|scope| (*scope).to_string()).collect::<Vec<_>>(), None)
}

#[test]
fn every_inference_job_tool_is_denied_without_its_scope_and_admitted_by_inference_execute() {
    let engine = PolicyEngine::new(Arc::new(crate::handles::HandleTable::new()), crate::policy::AutoApprovePolicy::Never);
    let unscoped = principal(&[]);
    for capability in inference_job_capabilities() {
        let denied = engine.authorize_scopes(&unscoped, &capability).expect_err("an unscoped principal is denied");
        assert_eq!(denied.code, GatewayErrorCode::PermissionDenied, "{}", capability.id);
    }
    let granted = principal(&["inference.execute"]);
    for capability in inference_job_capabilities() {
        engine.authorize_scopes(&granted, &capability).unwrap_or_else(|error| panic!("{} was refused for a granted principal: {error:?}", capability.id));
    }
    let read_only = principal(&["artifact.read"]);
    engine.authorize_scopes(&read_only, &inference_events_capability()).expect("artifacts.read alone reads the owner-private page");
    assert_eq!(engine.authorize_scopes(&read_only, &inference_submit_capability()).expect_err("a reader cannot submit").code, GatewayErrorCode::PermissionDenied);
    assert_eq!(engine.authorize_scopes(&read_only, &inference_approve_capability()).expect_err("a reader cannot approve").code, GatewayErrorCode::PermissionDenied);
    assert_eq!(engine.authorize_scopes(&read_only, &inference_cancel_capability()).expect_err("a reader cannot cancel").code, GatewayErrorCode::PermissionDenied);
}

#[test]
fn a_job_handle_is_readable_only_by_its_own_session_and_its_own_authenticated_subject() {
    let handles = crate::handles::HandleTable::new();
    let mine = crate::handles::SessionHandle::new("sess_mine");
    let theirs = crate::handles::SessionHandle::new("sess_theirs");
    let subject = HubInferenceSubjectV1 { hub_origin: "https://hub.invalid".into(), space_id: "space:alpha".into(), user_id: "user-a".into(), authority_generation: 7 };
    let payload = InferenceJobHandlePayloadV1 {
        site: InferenceExecutionSiteV1::Hub,
        service_id: "test.document.inference".into(),
        artifact_kind: "test.document".into(),
        plugin_id: "test".into(),
        document_id: "doc:tokyo".into(),
        job_id: sample_job_id(),
        hub: Some(HubInferenceJobBindingV1 { route: "inference/document".into(), space_id: subject.space_id.clone(), subject_user_id: subject.user_id.clone(), authority_generation: subject.authority_generation, request_id: sample_job_id(), base: None }),
    };
    let handle = handles.mint(crate::handles::HandleKind::Job, mine.clone(), crate::handles::Attachment::Artifact { artifact_id: "doc:tokyo".into() }, serde_json::to_value(&payload).expect("payload"), 1_000);
    assert_eq!(resolve_inference_job_handle(&handles, &mine, &handle, 1_001).expect("the minting session reads its own job"), payload);
    check_hub_job_subject(&payload, &subject).expect("the minting subject owns it");
    assert_eq!(resolve_inference_job_handle(&handles, &theirs, &handle, 1_001).expect_err("a second connection cannot read it").code, GatewayErrorCode::PermissionDenied);
    for (name, mutate) in [
        ("other-user", HubInferenceSubjectV1 { user_id: "user-b".into(), ..subject.clone() }),
        ("stale-authorization-generation", HubInferenceSubjectV1 { authority_generation: 8, ..subject.clone() }),
        ("cross-space", HubInferenceSubjectV1 { space_id: "space:beta".into(), ..subject.clone() }),
    ] {
        assert_eq!(check_hub_job_subject(&payload, &mutate).expect_err(name).code, GatewayErrorCode::PermissionDenied, "{name} read another subject's job");
    }
    assert_eq!(resolve_inference_job_handle(&handles, &mine, "job_never_minted", 1_001).expect_err("unknown handle").code, GatewayErrorCode::NotFound);
}

/// 🌅️ A booting hub's readiness (`startup` present) is an unavailable roster, retryable, never an
/// empty one; a ready or merely not-ready hub's roster is read as declared.
#[test]
fn a_booting_hub_roster_is_unavailable_never_empty() {
    let cancel = CancelToken::root_now();
    let starting = ScriptedTransport::ok(503, serde_json::json!({ "status": "not-ready", "features": { "inferenceServices": [] }, "startup": { "stage": "guest-codec-executing", "completedUnits": 1, "totalUnits": 2 } }));
    assert_eq!(block_on(read_hub_inference_services(&starting, &context(&cancel), "http://127.0.0.1:1")).expect_err("booting hub"), InferenceRouteErrorV1::Unavailable);
    let blocked = ScriptedTransport::ok(503, serde_json::json!({ "status": "not-ready", "features": { "inferenceServices": [] } }));
    assert_eq!(block_on(read_hub_inference_services(&blocked, &context(&cancel), "http://127.0.0.1:1")).expect("not-ready hub roster"), Vec::new());
    let ready = ScriptedTransport::ok(200, serde_json::json!({ "status": "ready", "features": { "inferenceServices": [{ "serviceId": "test.document.inference", "route": "inference/document" }] } }));
    assert_eq!(block_on(read_hub_inference_services(&ready, &context(&cancel), "http://127.0.0.1:1")).expect("ready hub roster").len(), 1);
}

#[test]
fn the_four_capabilities_are_direct_object_typed_gateway_tools_with_bilingual_descriptions() {
    let expected = ["inference_submit", "inference_events", "inference_cancel", "inference_approve"];
    let capabilities = inference_job_capabilities();
    assert_eq!(capabilities.len(), expected.len());
    for (capability, name) in capabilities.iter().zip(expected) {
        let ToolExposure::Direct { tool_name } = &capability.exposure else { panic!("{} is not directly exposed", capability.id) };
        assert_eq!(tool_name, name);
        assert!(crate::protocol::is_valid_tool_name(tool_name), "{tool_name} is not a valid MCP tool name");
        assert_eq!(capability.input_schema["type"], "object", "{}", capability.id);
        assert_eq!(capability.input_schema["$schema"], "https://json-schema.org/draft/2020-12/schema");
        assert_eq!(capability.output_schema["type"], "object", "{}", capability.id);
        assert!(capability.effects.external, "{} crosses the network to the hub", capability.id);
        let (english, german) = capability.description.split_once(" — ").unwrap_or_else(|| panic!("{} is not bilingual", capability.id));
        assert!(english.len() > 40 && german.len() > 40, "{} has an empty half", capability.id);
        assert_ne!(english, german);
    }
    assert!(crate::GATEWAY_TOOL_NAMES.iter().filter(|name| expected.contains(name)).count() == expected.len(), "the census must list every inference job tool");
}
//#endregion 🧪️Policy

#[test]
fn neutral_job_pages_refuse_foreign_jobs_and_private_payloads() {
    let source = include_str!("../../🧫️fixtures/🗳️job-client/🔣️.json");
    let vectors: serde_json::Value = serde_json::from_str(source).unwrap();
    let owned = semio_framework_os_kernel::os_pack::json::parse(source).unwrap();
    assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_os_kernel::os_pack::json::to_json_string(&owned)).unwrap(), vectors);
    for case in vectors["operationLabels"].as_array().unwrap() {
        assert_eq!(inference_operation_label(case["spaceId"].as_str().unwrap(), case["documentId"].as_str().unwrap(), case["jobId"].as_str()), case["expected"].as_str().unwrap());
    }
    for case in vectors["cases"].as_array().unwrap() {
        let cancel = CancelToken::root_now();
        let transport = ScriptedTransport::ok(200, case["response"].clone());
        let result = block_on(read_hub_inference_job_events(&transport, &context(&cancel), "https://hub.invalid", &scope(), "inference/document", case["requestedJobId"].as_str().unwrap(), 0));
        match case["expectedError"].as_str() {
            Some(code) => assert_eq!(result.unwrap_err().code(), code, "{}", case["name"]),
            None => assert_eq!(serde_json::to_value(result.unwrap()).unwrap()["state"], case["response"]["state"]),
        }
    }
}
