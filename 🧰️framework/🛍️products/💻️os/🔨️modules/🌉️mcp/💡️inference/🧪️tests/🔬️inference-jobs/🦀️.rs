
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

