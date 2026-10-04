
use super::*;
use semio_framework_async::{CancelToken, TraceId};
use semio_framework_os_kernel::os_directory::client::{DirectoryWsConnection, DirectoryWsPoll, HttpMethod, HttpResponse, TransportError};
use std::collections::VecDeque;
use std::sync::Mutex;
use std::sync::atomic::AtomicUsize;

#[derive(Clone)]
struct RecordingTransport {
    faults: Arc<Mutex<VecDeque<TransportError>>>,
    responses: Arc<Mutex<VecDeque<HttpResponse>>>,
    requests: Arc<Mutex<Vec<(HttpMethod, String, bool)>>>,
}

struct NoopWs;

#[derive(Clone)]
struct ObservedTransport {
    closes: Arc<AtomicUsize>,
}

struct ObservedWs {
    closes: Arc<AtomicUsize>,
}

impl DirectoryWsConnection for NoopWs {
    fn send_text(&mut self, _text: String) -> Result<(), TransportError> {
        Ok(())
    }
    fn send_binary(&mut self, _bytes: Vec<u8>) -> Result<(), TransportError> {
        Ok(())
    }
    fn try_recv_text(&mut self) -> Result<DirectoryWsPoll, TransportError> {
        Ok(DirectoryWsPoll::Pending)
    }
    fn close(&mut self) {}
}

impl DirectoryWsConnection for ObservedWs {
    fn send_text(&mut self, _text: String) -> Result<(), TransportError> {
        Ok(())
    }
    fn send_binary(&mut self, _bytes: Vec<u8>) -> Result<(), TransportError> {
        Ok(())
    }
    fn try_recv_text(&mut self) -> Result<DirectoryWsPoll, TransportError> {
        Ok(DirectoryWsPoll::Pending)
    }
    fn close(&mut self) {
        self.closes.fetch_add(1, Ordering::SeqCst);
    }
}

impl DirectoryTransport for RecordingTransport {
    type Ws = NoopWs;

    async fn http(&self, _ctx: &OperationContext, method: HttpMethod, url: &str, bearer: Option<&str>, _body: Option<Vec<u8>>) -> Result<HttpResponse, TransportError> {
        self.requests.lock().unwrap().push((method, url.to_string(), bearer.is_some()));
        if let Some(fault) = self.faults.lock().unwrap().pop_front() {
            return Err(fault);
        }
        self.responses.lock().unwrap().pop_front().ok_or_else(|| TransportError::Io("fixture response exhausted".to_string()))
    }

    async fn get_accepting(&self, _ctx: &OperationContext, _url: &str, _bearer: Option<&str>, _accept: &str) -> Result<HttpResponse, TransportError> {
        Err(TransportError::Io("fixture binary reads are not exercised".into()))
    }

    fn issue_socket_grant(&self, _ctx: &OperationContext, _url: &str, _bearer: &str, _body: &[u8], _timeout_ms: u64) -> Result<HttpResponse, TransportError> {
        Err(TransportError::Io("fixture socket grants are not exercised".into()))
    }

    fn open_ws(&self, _ctx: &OperationContext, _url: &str, _protocols: &[String], _timeout_ms: u64) -> Result<Self::Ws, TransportError> {
        Ok(NoopWs)
    }
}

impl DirectoryTransport for ObservedTransport {
    type Ws = ObservedWs;

    async fn http(&self, _ctx: &OperationContext, _method: HttpMethod, _url: &str, _bearer: Option<&str>, _body: Option<Vec<u8>>) -> Result<HttpResponse, TransportError> {
        Err(TransportError::Io("observed transport has no HTTP fixture".to_string()))
    }

    async fn get_accepting(&self, _ctx: &OperationContext, _url: &str, _bearer: Option<&str>, _accept: &str) -> Result<HttpResponse, TransportError> {
        Err(TransportError::Io("observed transport has no HTTP fixture".to_string()))
    }

    fn issue_socket_grant(&self, _ctx: &OperationContext, _url: &str, _bearer: &str, _body: &[u8], _timeout_ms: u64) -> Result<HttpResponse, TransportError> {
        Err(TransportError::Io("observed transport has no socket grant fixture".to_string()))
    }

    fn open_ws(&self, _ctx: &OperationContext, _url: &str, _protocols: &[String], _timeout_ms: u64) -> Result<Self::Ws, TransportError> {
        Ok(ObservedWs { closes: self.closes.clone() })
    }
}

fn context(deadline_ms: Option<u64>) -> OperationContext {
    OperationContext { actor: 7, generation: 0, trace: TraceId(9), lane: 1, deadline_ms, cancel: CancelToken::root_now(), capability: None }
}

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🔣️authenticated-hub-descriptor-index.json")).unwrap()
}

fn client_for(case: &serde_json::Value) -> (DirectoryClient<RecordingTransport>, Arc<Mutex<Vec<(HttpMethod, String, bool)>>>) {
    faulted_client_for(case, Vec::new())
}

/// 💥️ [`client_for`] whose first requests fail with the injected transport `faults`, in order, before any response.
fn faulted_client_for(case: &serde_json::Value, faults: Vec<TransportError>) -> (DirectoryClient<RecordingTransport>, Arc<Mutex<Vec<(HttpMethod, String, bool)>>>) {
    let responses = case["responses"]
        .as_array()
        .unwrap()
        .iter()
        .map(|response| HttpResponse {
            status: response["status"].as_u64().unwrap() as u16,
            body: match response.get("canonicalBody").and_then(serde_json::Value::as_str) {
                Some(canonical) => canonical.as_bytes().to_vec(),
                None => serde_json::to_vec(&response["body"]).unwrap(),
            },
        })
        .collect();
    let requests = Arc::new(Mutex::new(Vec::new()));
    let transport = RecordingTransport { faults: Arc::new(Mutex::new(faults.into())), responses: Arc::new(Mutex::new(responses)), requests: requests.clone() };
    let client = DirectoryClient::new(transport, "http://hub.invalid");
    (client, requests)
}

#[tokio::test]
async fn authenticated_hub_workspace_fixture_reaches_ready_without_retaining_bearer() {
    let contract = fixture();
    let case = &contract["cases"]["memberReady"];
    let (client, requests) = client_for(case);
    let binding = HubRemoteBinding::new("http://hub.invalid", "space-a").unwrap();
    let snapshot = binding.refresh(&client, &context(Some(20_000)), 1_000, 10_000).await.unwrap();
    assert_eq!(snapshot.authenticated_user_id, "user-a");
    assert_eq!(snapshot.documents.len(), 1);
    assert!(snapshot.documents.contains_key(&DocumentScope::new("space-a", "shared-doc")));
    assert_eq!(binding.progress(), HubBindingProgress { phase: HubBindingPhase::Ready, completed: 1, total: 1 });
    assert_eq!(requests.lock().unwrap().as_slice(), &[(HttpMethod::Get, "http://hub.invalid/auth/sessions/me".to_string(), false), (HttpMethod::Get, "http://hub.invalid/directory/spaces/space-a".to_string(), false),]);
    let rendered = format!("{:?} {:?} {:?}", binding.state(), binding.progress(), binding.diagnostic());
    assert!(!rendered.contains("session.v1."));
}

#[tokio::test]
async fn authenticated_hub_catalog_hydrates_exact_selected_descriptor_and_revocation_removes_it() {
    let contract = fixture();
    let (directory_client, _) = client_for(&contract["cases"]["memberReady"]);
    let seed_binding = HubRemoteBinding::new("http://hub.invalid", "space-a").unwrap();
    let seed = seed_binding.refresh(&directory_client, &context(Some(20_000)), 1_000, 10_000).await.unwrap();

    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../../📇️directory/🧫️fixtures/🔏️document-execution-target-lease-v1/🔣️.json")).expect("execution-target corpus json");
    let mut manifest: DocumentExecutionTargetLeaseFieldsV1 = semio_framework_pack_json::from_json_str(&serde_json::to_string(&corpus["manifest"]).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("manifest");
    let (descriptor, descriptor_bytes) = crate::source_builders::catalog_contract_descriptor();
    let descriptor_sha256 = framework_hash::sha256_hex(&descriptor_bytes);
    manifest.package.plugin_id = descriptor.manifest.plugin_id.clone();
    manifest.package.package_id = descriptor.package_id.clone();
    manifest.package.version = descriptor.manifest.version.clone();
    manifest.package.component_sha256 = descriptor.hashes.wasm_sha256.clone();
    manifest.package.descriptor_byte_sha256 = descriptor_sha256.clone();
    manifest.component.sha256 = descriptor.hashes.wasm_sha256.clone();
    manifest.descriptor.sha256 = descriptor_sha256;
    manifest.descriptor.byte_length = u64::try_from(descriptor_bytes.len()).expect("descriptor length");
    if let semio_framework_os_kernel::os_directory::schema::DocumentExecutionTargetBrowserActorV1::ClosedBrowserActor { source_component_sha256, source_descriptor_byte_sha256, .. } = &mut manifest.browser_actor {
        source_component_sha256.clone_from(&manifest.package.component_sha256);
        source_descriptor_byte_sha256.clone_from(&manifest.package.descriptor_byte_sha256);
    }

    let mut snapshot = seed.as_ref().clone();
    snapshot.space.id = manifest.scope.space_id.clone();
    let mut document = snapshot.documents.values().next().expect("seed document").clone();
    document.scope = manifest.scope.clone();
    document.descriptor_digest_v1 = manifest.descriptor_digest_v1.clone();
    document.view.descriptor.space_id = manifest.scope.space_id.clone();
    document.view.descriptor.document_id = manifest.scope.document_id.clone();
    document.view.descriptor.artifact_kind = manifest.artifact.kind.clone();
    document.view.descriptor.artifact_schema = manifest.artifact.schema.clone();
    document.view.descriptor.pack_schema_hash = manifest.artifact.pack_schema_hash.clone();
    document.view.descriptor.owner.plugin_id = manifest.package.plugin_id.clone();
    document.view.descriptor.owner.package_id = manifest.package.package_id.clone();
    document.view.descriptor.owner.version = manifest.package.version.clone();
    document.view.descriptor.owner.package_hash = manifest.package.component_sha256.clone();
    snapshot.documents = HashMap::from([(manifest.scope.clone(), document)]);

    let binding = HubRemoteBinding::new("http://hub.invalid", manifest.scope.space_id.clone()).unwrap();
    binding.install_snapshot_for_test(snapshot.clone());
    let requests = Arc::new(Mutex::new(Vec::new()));
    let transport = RecordingTransport {
        faults: Arc::default(),
        responses: Arc::new(Mutex::new(VecDeque::from([HttpResponse { status: 200, body: semio_framework_pack_json::to_json_string(&manifest).into_bytes() }, HttpResponse { status: 200, body: descriptor_bytes }]))),
        requests: requests.clone(),
    };
    let client = DirectoryClient::new(transport, "http://hub.invalid");
    let selected = binding.refresh_catalog(&client, &Arc::new(snapshot), &context(Some(20_000))).await.unwrap();
    assert_eq!(selected.selections.len(), 1);
    assert_eq!(selected.selections[0].lease.package, manifest.package);
    assert_eq!(selected.selections[0].descriptor.manifest.plugin_id, "contract-test");
    let paths: Vec<_> = requests.lock().unwrap().iter().map(|(_, url, _)| url.clone()).collect();
    assert_eq!(paths, ["http://hub.invalid/spaces/raum%3A%C3%A4/documents/karte%3A%E6%9D%B1%E4%BA%AC/execution-target/manifest", "http://hub.invalid/spaces/raum%3A%C3%A4/documents/karte%3A%E6%9D%B1%E4%BA%AC/execution-target/descriptor",],);
    binding.invalidate_stream();
    assert_eq!(binding.ready_catalog_snapshot(i64::MIN).unwrap_err().code, GatewayErrorCode::PluginUnavailable);
}

#[tokio::test]
async fn authenticated_hub_workspace_rejects_public_nonmember_and_cross_space_atomically() {
    let contract = fixture();
    for (case_name, expected) in [("publicWithoutMembership", HubBindingError::MembershipRequired), ("sameDocumentOtherSpace", HubBindingError::InvalidResponse("document escaped the selected space"))] {
        let (client, _) = client_for(&contract["cases"][case_name]);
        let binding = HubRemoteBinding::new("http://hub.invalid", "space-a").unwrap();
        let error = binding.refresh(&client, &context(Some(20_000)), 1_000, 10_000).await.unwrap_err();
        assert_eq!(error, expected);
        assert!(!matches!(binding.state(), HubRemoteBindingState::Ready(_)));
    }
}

/// 📚️ LAW: a space past one bounded administration window binds EVERY document — the documents window is followed to its
/// last page, then the members window until the principal's own row (a cursor advances only its own window) — and pages
/// that cross an authorization change are refused, never stitched (fixture cases `pagedWindows`,
/// `pagesCrossAnAuthorizationChange`; live: a 66-document space on hub 7800/p33 refused every open).
#[tokio::test]
async fn a_space_past_one_window_binds_every_document_and_never_stitches_across_an_authorization_change() {
    let contract = fixture();
    let requested = |case: &serde_json::Value| case["expected"]["requests"].as_array().unwrap().iter().map(|path| (HttpMethod::Get, format!("http://hub.invalid{}", path.as_str().unwrap()), false)).collect::<Vec<_>>();
    let paged = &contract["cases"]["pagedWindows"];
    let (client, requests) = client_for(paged);
    let binding = HubRemoteBinding::new("http://hub.invalid", "space-a").unwrap();
    let snapshot = binding.refresh(&client, &context(Some(20_000)), 1_000, 10_000).await.unwrap();
    let mut bound = snapshot.documents.keys().map(|scope| descriptor_resource_uri(scope)).collect::<Vec<_>>();
    bound.sort();
    let expected = paged["expected"]["resourceUris"].as_array().unwrap().iter().map(|uri| uri.as_str().unwrap().to_string()).filter(|uri| uri.ends_with("/descriptor")).collect::<Vec<_>>();
    assert_eq!(bound, expected);
    assert_eq!(snapshot.membership.user_id, "user-a");
    assert_eq!(binding.progress(), HubBindingProgress { phase: HubBindingPhase::Ready, completed: 3, total: 3 });
    assert_eq!(requests.lock().unwrap().as_slice(), requested(paged).as_slice());
    let crossed = &contract["cases"]["pagesCrossAnAuthorizationChange"];
    let (client, requests) = client_for(crossed);
    let binding = HubRemoteBinding::new("http://hub.invalid", "space-a").unwrap();
    assert_eq!(binding.refresh(&client, &context(Some(20_000)), 1_000, 10_000).await.unwrap_err(), HubBindingError::InvalidResponse("space administration pages crossed an authorization change"));
    assert!(matches!(binding.state(), HubRemoteBindingState::Refreshing));
    assert_eq!(requests.lock().unwrap().as_slice(), requested(crossed).as_slice());
}

#[tokio::test]
async fn an_agent_binds_at_its_capped_role_below_its_account_but_no_principal_differs_upward_or_as_a_human() {
    let contract = fixture();
    let (client, _) = client_for(&contract["cases"]["agentBelowMembership"]);
    let binding = HubRemoteBinding::new("http://hub.invalid", "space-a").unwrap();
    let snapshot = binding.refresh(&client, &context(Some(20_000)), 1_000, 10_000).await.unwrap();
    assert_eq!((snapshot.space.role, snapshot.membership.role), (DirectorySpaceRole::Spectator, DirectorySpaceRole::Author));
    assert!(snapshot.documents.contains_key(&DocumentScope::new("space-a", "shared-doc")));
    for case_name in ["humanBelowMembership", "principalAboveMembership"] {
        let (client, _) = client_for(&contract["cases"][case_name]);
        let binding = HubRemoteBinding::new("http://hub.invalid", "space-a").unwrap();
        assert_eq!(binding.refresh(&client, &context(Some(20_000)), 1_000, 10_000).await.unwrap_err(), HubBindingError::MembershipRequired, "{case_name}");
        assert!(matches!(binding.state(), HubRemoteBindingState::Revoked), "{case_name}");
    }
}

#[tokio::test]
async fn authenticated_hub_workspace_unauthorized_cancelled_and_deadline_never_publish() {
    let contract = fixture();
    let (client, _) = client_for(&contract["cases"]["expiredToken"]);
    let binding = HubRemoteBinding::new("http://hub.invalid", "space-a").unwrap();
    assert_eq!(binding.refresh(&client, &context(Some(20_000)), 1_000, 10_000).await.unwrap_err(), HubBindingError::Unauthorized);
    assert!(matches!(binding.state(), HubRemoteBindingState::Revoked));

    let (client, requests) = client_for(&contract["cases"]["memberReady"]);
    let cancelled = context(Some(20_000));
    cancelled.cancel.cancel_now();
    let binding = HubRemoteBinding::new("http://hub.invalid", "space-a").unwrap();
    assert_eq!(binding.refresh(&client, &cancelled, 1_000, 10_000).await.unwrap_err(), HubBindingError::Cancelled);
    assert!(requests.lock().unwrap().is_empty());

    let binding = HubRemoteBinding::new("http://hub.invalid", "space-a").unwrap();
    assert_eq!(binding.refresh(&client, &context(Some(10_000)), 1_000, 10_000).await.unwrap_err(), HubBindingError::DeadlineExceeded);
    assert!(requests.lock().unwrap().is_empty());
}

#[tokio::test]
async fn authenticated_hub_workspace_revocation_and_stream_loss_invalidate_ready_snapshot() {
    let contract = fixture();
    let (client, _) = client_for(&contract["cases"]["memberReady"]);
    let binding = HubRemoteBinding::new("http://hub.invalid", "space-a").unwrap();
    binding.refresh(&client, &context(Some(20_000)), 1_000, 10_000).await.unwrap();
    let message = semio_framework_pack_json::from_json_str::<DirectoryStreamMessage>(&contract["cases"]["memberRevoked"]["streamMessage"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(binding.observe_stream_message(&message), HubStreamObservation::Revoked);
    assert!(matches!(binding.state(), HubRemoteBindingState::Revoked));
    assert!(binding.ready_snapshot(1_000).unwrap_err().retryable);

    let (client, _) = client_for(&contract["cases"]["memberReady"]);
    let binding = HubRemoteBinding::new("http://hub.invalid", "space-a").unwrap();
    binding.refresh(&client, &context(Some(20_000)), 1_000, 10_000).await.unwrap();
    let refreshing_generation = binding.begin_refresh(HubBindingPhase::Authenticating, 0);
    binding.invalidate_stream();
    assert_ne!(binding.generation.load(Ordering::SeqCst), refreshing_generation);
    assert!(matches!(binding.state(), HubRemoteBindingState::Refreshing));
    assert!(binding.ready_snapshot(1_000).unwrap_err().retryable);
}

/// ⏳️ A directory event invalidates the authority; a hub-bound call issued in that window waits for
/// the refresh in flight (announced by the binding itself) instead of answering a retryable refusal,
/// still fails closed once its bounded wait passes, and never waits on a revoked binding.
#[tokio::test]
async fn a_call_during_an_authority_refresh_waits_for_it_and_fails_closed_only_after_the_wait() {
    let contract = fixture();
    let (client, _) = client_for(&contract["cases"]["memberReady"]);
    let binding = Arc::new(HubRemoteBinding::new("http://hub.invalid", "space-a").unwrap());
    let snapshot = binding.refresh(&client, &context(Some(20_000)), 1_000, 10_000).await.unwrap();
    binding.install_catalog_for_test(Vec::new());
    assert!(binding.ready_catalog_snapshot(1_000).is_ok());

    binding.invalidate("hub directory event requires an authenticated descriptor refresh");
    assert!(binding.ready_catalog_snapshot(1_000).unwrap_err().retryable, "the gate itself stays instantaneous and closed");
    let refresher = {
        let binding = Arc::clone(&binding);
        let snapshot = snapshot.as_ref().clone();
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(150));
            binding.install_snapshot_for_test(snapshot);
            std::thread::sleep(std::time::Duration::from_millis(150));
            binding.install_catalog_for_test(Vec::new());
        })
    };
    let started = std::time::Instant::now();
    binding.await_settled(HUB_AUTHORITY_SETTLE_WAIT_MS);
    let waited = started.elapsed();
    refresher.join().unwrap();
    assert!(binding.ready_catalog_snapshot(1_000).is_ok(), "the call proceeds on the refreshed authority");
    assert!(waited >= std::time::Duration::from_millis(300), "a bound snapshot without its catalog is still settling: waited {waited:?}");
    assert!(waited < std::time::Duration::from_millis(HUB_AUTHORITY_SETTLE_WAIT_MS), "woken by the announcement, not by the deadline: waited {waited:?}");

    binding.invalidate("hub directory stream continuity was lost");
    let started = std::time::Instant::now();
    binding.await_settled(200);
    assert!(started.elapsed() >= std::time::Duration::from_millis(200));
    assert!(binding.ready_snapshot(1_000).unwrap_err().retryable, "a refresh that never lands still fails closed");

    binding.revoke(HubBindingError::MembershipRequired);
    let started = std::time::Instant::now();
    binding.await_settled(HUB_AUTHORITY_SETTLE_WAIT_MS);
    assert!(started.elapsed() < std::time::Duration::from_millis(1_000), "a revoked binding is settled: nothing will refresh it");
}

/// 🔁️ A failing authority refresh backs off instead of re-asking a busy hub back-to-back: the pause
/// doubles from the base and saturates at the maximum.
#[test]
fn failed_authority_refreshes_back_off_and_saturate() {
    let pauses: Vec<u64> = (1..=8).map(hub_refresh_retry_ms).collect();
    assert_eq!(pauses, vec![100, 200, 400, 800, 1_600, 3_200, 5_000, 5_000]);
    assert_eq!(hub_refresh_retry_ms(u32::MAX), HUB_REFRESH_RETRY_MAX_MS);
    assert_eq!(hub_refresh_retry_ms(0), HUB_REFRESH_RETRY_BASE_MS);
}

#[tokio::test]
async fn native_driver_closes_post_open_cancelled_and_stale_authority_dials_once_without_refresh() {
    let contract = fixture();
    let (client, requests) = client_for(&contract["cases"]["memberReady"]);
    let binding = HubRemoteBinding::new("http://hub.invalid", "space-a").unwrap();
    binding.refresh(&client, &context(Some(20_000)), 1_000, 10_000).await.unwrap();
    let initial_requests = requests.lock().unwrap().len();
    let initial_generation = binding.generation.load(Ordering::SeqCst);
    let authority_generation = binding.authority_generation.load(Ordering::SeqCst);

    let cancelled_closes = Arc::new(AtomicUsize::new(0));
    let cancelled_client = Arc::new(DirectoryClient::new(ObservedTransport { closes: cancelled_closes.clone() }, "http://hub.invalid"));
    let mut cancelled_stream = cancelled_client.stream(0);
    let cancelled = context(Some(20_000));
    assert!(matches!(cancelled_stream.turn(&cancelled, 10_001), semio_framework_os_kernel::os_directory::client::DirectoryStreamTurn::Dial { .. }));
    cancelled.cancel.cancel_now();
    let cancelled_turn = complete_authorized_directory_dial(&mut cancelled_stream, &binding, &cancelled, authority_generation, 10_002, ObservedWs { closes: cancelled_closes.clone() });
    assert!(matches!(cancelled_turn, semio_framework_os_kernel::os_directory::client::DirectoryStreamTurn::Closed));
    assert_eq!(cancelled_closes.load(Ordering::SeqCst), 1);
    assert_eq!(binding.generation.load(Ordering::SeqCst), initial_generation);
    assert_eq!(binding.authority_generation.load(Ordering::SeqCst), authority_generation);
    assert!(matches!(binding.state(), HubRemoteBindingState::Ready(_)));
    assert_eq!(requests.lock().unwrap().len(), initial_requests);

    let stale_closes = Arc::new(AtomicUsize::new(0));
    let stale_client = Arc::new(DirectoryClient::new(ObservedTransport { closes: stale_closes.clone() }, "http://hub.invalid"));
    let mut stale_stream = stale_client.stream(0);
    let active = context(Some(20_000));
    assert!(matches!(stale_stream.turn(&active, 10_003), semio_framework_os_kernel::os_directory::client::DirectoryStreamTurn::Dial { .. }));
    binding.invalidate_stream();
    let invalidated_generation = binding.generation.load(Ordering::SeqCst);
    let stale_turn = complete_authorized_directory_dial(&mut stale_stream, &binding, &active, authority_generation, 10_004, ObservedWs { closes: stale_closes.clone() });
    assert!(matches!(stale_turn, semio_framework_os_kernel::os_directory::client::DirectoryStreamTurn::Closed));
    assert_eq!(stale_closes.load(Ordering::SeqCst), 1);
    assert_eq!(binding.generation.load(Ordering::SeqCst), invalidated_generation);
    assert_eq!(binding.authority_generation.load(Ordering::SeqCst), 0);
    assert!(matches!(binding.state(), HubRemoteBindingState::Refreshing));
    assert_eq!(requests.lock().unwrap().len(), initial_requests);
}

#[test]
fn authenticated_hub_workspace_fixed_caps_and_scoped_uri_laws() {
    assert!(validate_hub_origin("https://hub.invalid", "space-a").is_ok());
    assert!(validate_hub_origin("file:///tmp/hub", "space-a").is_err());
    assert!(validate_hub_origin(&format!("http://{}", "h".repeat(2_048)), "space-a").is_err());
    let scope = DocumentScope::new("space/a", "dokument/ä");
    let uri = descriptor_resource_uri(&scope);
    assert_eq!(parse_descriptor_resource_uri(&uri), Some(scope));
    assert_eq!(parse_descriptor_resource_uri("semio://workspace/scopes/space-a/shared-doc/schema"), None);
    assert!(bounded_diagnostic(&"é".repeat(HUB_BINDING_DIAGNOSTIC_MAX_BYTES)).len() <= HUB_BINDING_DIAGNOSTIC_MAX_BYTES);
    assert_eq!(validate_document_count(HUB_DESCRIPTOR_INDEX_MAX_DOCUMENTS, HUB_DESCRIPTOR_INDEX_MAX_DOCUMENTS as u32), Ok(()));
    assert_eq!(validate_document_count(HUB_DESCRIPTOR_INDEX_MAX_DOCUMENTS + 1, (HUB_DESCRIPTOR_INDEX_MAX_DOCUMENTS + 1) as u32), Err(HubBindingError::CapacityExceeded));
}

/// 🔌️ A hub-unavailable refusal keeps its cause typed and bilingual (shared fixture `🔣️hub-unavailable-refusal.json`):
/// a transport fault names its own detail (a refused connection) instead of the bare word "transport" the semio MCP
/// answered while its gateway was wedged (ticket 26/09/23, session 14b), and a spent network byte budget is its own
/// `network-budget` cause — the hub is fine and the call continues once the budget refills (session 14c).
#[test]
fn a_hub_unavailable_refusal_names_its_typed_cause_in_english_and_german() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️hub-unavailable-refusal.json")).unwrap();
    assert_eq!(corpus["detailMaxChars"], HUB_UNAVAILABLE_DETAIL_MAX_CHARS);
    for case in corpus["cases"].as_array().unwrap() {
        let fault = &case["fault"];
        let error = match fault["kind"].as_str().unwrap() {
            "transport" => DirectoryClientError::Transport(TransportError::Io(fault["io"].as_str().unwrap().to_string())),
            "http" => DirectoryClientError::Http { status: u16::try_from(fault["status"].as_u64().unwrap()).unwrap(), body: fault["body"].as_str().unwrap().to_string() },
            "decode" => DirectoryClientError::Decode(fault["decode"].as_str().unwrap().to_string()),
            "budget" => DirectoryClientError::Transport(TransportError::BudgetExhausted),
            other => panic!("unknown fault kind {other}"),
        };
        assert_eq!(serde_json::to_value(binding_error_to_gateway(map_client_error(error))).unwrap(), case["expected"], "{}", case["id"]);
    }
}

/// 🔁️ An injected transport fault leaves the binding refreshing (never revoked) with its own cause as the last fault,
/// and the next refresh over a healthy transport binds it again: a transient transport failure is recoverable, never
/// a wedge (ticket 26/09/23, session 14b).
#[tokio::test]
async fn a_transport_fault_keeps_its_cause_and_the_next_refresh_recovers_the_binding() {
    let contract = fixture();
    let exhausted = "Connection Failed: Connect error: Connection refused (os error 61)";
    let (client, requests) = faulted_client_for(&contract["cases"]["memberReady"], vec![TransportError::Io(exhausted.to_string())]);
    let binding = HubRemoteBinding::new("http://hub.invalid", "space-a").unwrap();
    let fault = binding.refresh(&client, &context(Some(20_000)), 1_000, 10_000).await.unwrap_err();
    assert_eq!(fault, HubBindingError::Unavailable(HubUnavailableCause::Transport { detail: exhausted.to_string() }));
    assert!(matches!(binding.state(), HubRemoteBindingState::Refreshing), "a transport fault never revokes the binding");
    let refusal = binding.ready_snapshot(1_000).unwrap_err();
    assert!(refusal.retryable);
    assert!(refusal.details["lastFault"].as_str().is_some_and(|last| last.contains(exhausted)), "the refusal names the fault: {:?}", refusal.details);
    let snapshot = binding.refresh(&client, &context(Some(20_000)), 1_000, 10_000).await.expect("the next refresh over a healthy transport binds again");
    assert_eq!(snapshot.authenticated_user_id, "user-a");
    assert!(binding.ready_snapshot(1_000).is_ok());
    assert_eq!(binding.diagnostic(), None);
    assert_eq!(requests.lock().unwrap().len(), 3, "one faulted request, then the session and the space page");
}

#[test]
fn a_directory_dial_refusal_names_its_cause_and_stays_retryable() {
    let refusal = directory_dial_refusal(&"http 413: Failed to buffer the request body: length limit exceeded");
    assert_eq!(refusal.code, GatewayErrorCode::PluginUnavailable);
    assert!(refusal.retryable, "a failed dial is retried by the stream's reconnect ladder");
    assert!(refusal.message.contains("http 413: Failed to buffer the request body"), "the dial's own cause is named: {}", refusal.message);
    assert!(refusal.message.contains("authenticated snapshot was not activated"));
}

/// 🧾️ A GIS selection whose authenticated lease names `document_id` in the fixture's space: the manifest, the document view
/// the snapshot holds for it, and the canonical descriptor bytes both name.
fn catalog_selection(seed: &AuthorizedDocumentView, document_id: &str) -> (DocumentExecutionTargetLeaseFieldsV1, AuthorizedDocumentView, Vec<u8>) {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../../📇️directory/🧫️fixtures/🔏️document-execution-target-lease-v1/🔣️.json")).expect("execution-target corpus json");
    let mut manifest: DocumentExecutionTargetLeaseFieldsV1 = semio_framework_pack_json::from_json_str(&serde_json::to_string(&corpus["manifest"]).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("manifest");
    let (descriptor, descriptor_bytes) = crate::source_builders::catalog_contract_descriptor();
    let descriptor_sha256 = framework_hash::sha256_hex(&descriptor_bytes);
    manifest.scope.document_id = document_id.to_string();
    manifest.checkpoint.baseline_frontier.document_id = document_id.to_string();
    manifest.package.plugin_id = descriptor.manifest.plugin_id.clone();
    manifest.package.package_id = descriptor.package_id.clone();
    manifest.package.version = descriptor.manifest.version.clone();
    manifest.package.component_sha256 = descriptor.hashes.wasm_sha256.clone();
    manifest.package.descriptor_byte_sha256 = descriptor_sha256.clone();
    manifest.component.sha256 = descriptor.hashes.wasm_sha256.clone();
    manifest.descriptor.sha256 = descriptor_sha256;
    manifest.descriptor.byte_length = u64::try_from(descriptor_bytes.len()).expect("descriptor length");
    if let semio_framework_os_kernel::os_directory::schema::DocumentExecutionTargetBrowserActorV1::ClosedBrowserActor { source_component_sha256, source_descriptor_byte_sha256, .. } = &mut manifest.browser_actor {
        source_component_sha256.clone_from(&manifest.package.component_sha256);
        source_descriptor_byte_sha256.clone_from(&manifest.package.descriptor_byte_sha256);
    }
    let mut document = seed.clone();
    document.scope = manifest.scope.clone();
    document.descriptor_digest_v1 = manifest.descriptor_digest_v1.clone();
    document.view.descriptor.space_id = manifest.scope.space_id.clone();
    document.view.descriptor.document_id = manifest.scope.document_id.clone();
    document.view.descriptor.artifact_kind = manifest.artifact.kind.clone();
    document.view.descriptor.artifact_schema = manifest.artifact.schema.clone();
    document.view.descriptor.pack_schema_hash = manifest.artifact.pack_schema_hash.clone();
    document.view.descriptor.owner.plugin_id = manifest.package.plugin_id.clone();
    document.view.descriptor.owner.package_id = manifest.package.package_id.clone();
    document.view.descriptor.owner.version = manifest.package.version.clone();
    document.view.descriptor.owner.package_hash = manifest.package.component_sha256.clone();
    (manifest, document, descriptor_bytes)
}

/// 🗃️ A catalog refresh fetches a package's descriptor body once, however many documents select it, and the next refresh —
/// every directory event starts one — fetches manifests only: on hub 7800 a space of 63 documents over 33 packages moved
/// 16.7 MB of descriptors per refresh and spent the gateway's byte budget within a minute (ticket 26/09/23, session 14c).
#[tokio::test]
async fn a_catalog_refresh_fetches_each_descriptor_once_and_the_next_refresh_none() {
    let mut snapshot = {
        let contract = fixture();
        let (directory_client, _) = client_for(&contract["cases"]["memberReady"]);
        let seed_binding = HubRemoteBinding::new("http://hub.invalid", "space-a").unwrap();
        seed_binding.refresh(&directory_client, &context(Some(20_000)), 1_000, 10_000).await.unwrap().as_ref().clone()
    };
    let seed = snapshot.documents.values().next().expect("seed document").clone();
    let (first, first_document, descriptor_bytes) = catalog_selection(&seed, "karte:berlin");
    let (second, second_document, _) = catalog_selection(&seed, "karte:rom");
    snapshot.space.id = first.scope.space_id.clone();
    snapshot.documents = HashMap::from([(first.scope.clone(), first_document), (second.scope.clone(), second_document)]);
    let binding = HubRemoteBinding::new("http://hub.invalid", first.scope.space_id.clone()).unwrap();
    binding.install_snapshot_for_test(snapshot.clone());
    let manifest = |lease: &DocumentExecutionTargetLeaseFieldsV1| HttpResponse { status: 200, body: semio_framework_pack_json::to_json_string(lease).into_bytes() };
    let requests = Arc::new(Mutex::new(Vec::new()));
    let transport = RecordingTransport {
        faults: Arc::default(),
        responses: Arc::new(Mutex::new(VecDeque::from([manifest(&first), HttpResponse { status: 200, body: descriptor_bytes }, manifest(&second), manifest(&first), manifest(&second)]))),
        requests: requests.clone(),
    };
    let client = DirectoryClient::new(transport, "http://hub.invalid");
    let snapshot = Arc::new(snapshot);
    let selected = binding.refresh_catalog(&client, &snapshot, &context(Some(20_000))).await.unwrap();
    assert_eq!(selected.selections.len(), 1, "two documents of one package select it once");
    assert_eq!(selected.dialect_kinds.len(), 2);
    let again = binding.refresh_catalog(&client, &snapshot, &context(Some(20_000))).await.unwrap();
    assert_eq!(again.selections, selected.selections, "the next refresh selects the identical verified descriptor");
    let routes: Vec<String> = requests.lock().unwrap().iter().map(|(_, url, _)| url.rsplit('/').next().unwrap().to_string()).collect();
    assert_eq!(routes, ["manifest", "descriptor", "manifest", "manifest", "manifest"], "one descriptor body across both refreshes");
}

/// 💰️ A spent network byte budget is a wait for its refill, never a refusal: the wait reports what it has and needs,
/// ends as soon as the budget admits the transfer (a whole allowance for a transfer larger than it), and only
/// cancellation ends it early.
#[test]
fn a_spent_network_budget_is_a_wait_for_the_refill_never_a_refusal() {
    let reads = AtomicUsize::new(0);
    let refilling = || {
        let read = reads.fetch_add(1, Ordering::SeqCst);
        semio_framework_os_services::HttpPackageBudget { remaining_bytes: if read < 3 { 10 } else { 1_000 }, capacity_bytes: 1_000 }
    };
    let mut waits = Vec::new();
    assert_eq!(await_network_budget(refilling, 400, &CancelToken::root_now(), |remaining, needed| waits.push((remaining, needed))), Ok(()));
    assert_eq!(waits, [(10, 400), (10, 400), (10, 400)], "each wait names what the budget has and what the transfer needs");
    let full = || semio_framework_os_services::HttpPackageBudget { remaining_bytes: 1_000, capacity_bytes: 1_000 };
    assert_eq!(await_network_budget(full, 5_000, &CancelToken::root_now(), |_, _| panic!("a full budget admits a transfer larger than the allowance")), Ok(()));
    let cancel = CancelToken::root_now();
    let spent = || semio_framework_os_services::HttpPackageBudget { remaining_bytes: 0, capacity_bytes: 1_000 };
    assert_eq!(await_network_budget(spent, 1, &cancel, |_, _| cancel.cancel_now()), Err(HubBindingError::Cancelled));
    assert_eq!(map_client_error(DirectoryClientError::Transport(TransportError::BudgetExhausted)), HubBindingError::Unavailable(HubUnavailableCause::NetworkBudget));
    assert_eq!(map_catalog_client_error(DirectoryClientError::Transport(TransportError::BudgetExhausted)), HubBindingError::Unavailable(HubUnavailableCause::NetworkBudget));
}

/// ⏳️ A call parked on an authority refresh that waits for the network byte budget spans the refill instead of refusing
/// at its ordinary settle deadline, and settles the moment the refresh lands.
#[tokio::test]
async fn a_call_parked_on_a_budget_wait_spans_the_refill_and_settles_when_the_refresh_lands() {
    let contract = fixture();
    let (client, _) = client_for(&contract["cases"]["memberReady"]);
    let binding = Arc::new(HubRemoteBinding::new("http://hub.invalid", "space-a").unwrap());
    let snapshot = binding.refresh(&client, &context(Some(20_000)), 1_000, 10_000).await.unwrap();
    binding.invalidate("hub directory event requires an authenticated descriptor refresh");
    binding.note_network_budget_wait(3 * 1024 * 1024, HUB_REFRESH_NETWORK_BUDGET_BYTES);
    assert_eq!(binding.progress(), HubBindingProgress { phase: HubBindingPhase::AwaitingNetworkBudget, completed: 3 * 1024 * 1024, total: HUB_REFRESH_NETWORK_BUDGET_BYTES as usize });
    let refusal = binding.ready_snapshot(1_000).unwrap_err();
    assert_eq!(refusal.details["phase"], "awaiting-network-budget", "a refusal names the budget wait: {:?}", refusal.details);
    let refresher = {
        let binding = Arc::clone(&binding);
        let snapshot = snapshot.as_ref().clone();
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(400));
            binding.install_snapshot_for_test(snapshot);
            binding.install_catalog_for_test(Vec::new());
        })
    };
    let started = std::time::Instant::now();
    binding.await_settled(100);
    let waited = started.elapsed();
    refresher.join().unwrap();
    assert!(waited >= std::time::Duration::from_millis(400), "the budget wait outlasts the ordinary settle deadline: waited {waited:?}");
    assert!(waited < std::time::Duration::from_millis(HUB_NETWORK_BUDGET_SETTLE_WAIT_MS), "woken by the refresh, not by the deadline: waited {waited:?}");
    assert!(binding.ready_catalog_snapshot(1_000).is_ok());
}

/// 🔁️ A canonical pair attempt that raced a directory event waits for the refresh it started and then mounts; one that
/// fails with the authority settled answers at once; a spent budget waits for the refill and tries again.
#[tokio::test]
async fn a_pair_attempt_racing_an_authority_refresh_waits_for_it_and_then_mounts() {
    let contract = fixture();
    let (client, _) = client_for(&contract["cases"]["memberReady"]);
    let binding = Arc::new(HubRemoteBinding::new("http://hub.invalid", "space-a").unwrap());
    let snapshot = binding.refresh(&client, &context(Some(20_000)), 1_000, 10_000).await.unwrap();
    binding.install_catalog_for_test(Vec::new());
    binding.invalidate("hub directory event requires an authenticated descriptor refresh");
    let refresher = {
        let binding = Arc::clone(&binding);
        let snapshot = snapshot.as_ref().clone();
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(150));
            binding.install_snapshot_for_test(snapshot);
            binding.install_catalog_for_test(Vec::new());
        })
    };
    let attempts = AtomicUsize::new(0);
    let mounted = settled_pair_attempt(&binding, HUB_AUTHORITY_SETTLE_WAIT_MS, || Ok(()), || {
        attempts.fetch_add(1, Ordering::SeqCst);
        if binding.settling() { Err(CanonicalPairMountError::DescriptorUnavailable) } else { Ok("mounted") }
    });
    refresher.join().unwrap();
    assert_eq!(mounted, Ok("mounted"));
    assert!(attempts.load(Ordering::SeqCst) >= 2, "the first attempt raced the refresh, a later one mounted");

    let started = std::time::Instant::now();
    let unknown: Result<(), _> = settled_pair_attempt(&binding, HUB_AUTHORITY_SETTLE_WAIT_MS, || Ok(()), || Err(CanonicalPairMountError::DescriptorUnavailable));
    assert_eq!(unknown, Err(CanonicalPairMountError::DescriptorUnavailable));
    assert!(started.elapsed() < std::time::Duration::from_millis(1_000), "a settled authority answers at once");

    let budget_waits = AtomicUsize::new(0);
    let transfers = AtomicUsize::new(0);
    let refilled = settled_pair_attempt(
        &binding,
        HUB_AUTHORITY_SETTLE_WAIT_MS,
        || {
            budget_waits.fetch_add(1, Ordering::SeqCst);
            Ok(())
        },
        || if transfers.fetch_add(1, Ordering::SeqCst) == 0 { Err(CanonicalPairMountError::BudgetExhausted) } else { Ok("mounted after the refill") },
    );
    assert_eq!(refilled, Ok("mounted after the refill"));
    assert_eq!(budget_waits.load(Ordering::SeqCst), 1);
}

/// 🧩️ A loopback hub answering `execution-target/component` streams from a queue of named answers, counting every request.
struct AssetHub {
    origin: String,
    requests: Arc<AtomicUsize>,
    answers: Arc<Mutex<VecDeque<String>>>,
}

fn asset_hub(component: Vec<u8>, busy_retry_after_ms: u64) -> AssetHub {
    use std::io::{Read, Write};
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("asset hub listener");
    let origin = format!("http://{}", listener.local_addr().expect("asset hub address"));
    let requests = Arc::new(AtomicUsize::new(0));
    let answers: Arc<Mutex<VecDeque<String>>> = Arc::new(Mutex::new(VecDeque::new()));
    let (served, queue) = (requests.clone(), answers.clone());
    std::thread::spawn(move || {
        for mut stream in listener.incoming().filter_map(Result::ok) {
            let mut head = Vec::new();
            let mut byte = [0u8; 1];
            while !head.ends_with(b"\r\n\r\n") && stream.read(&mut byte).is_ok_and(|read| read == 1) {
                head.push(byte[0]);
            }
            let head = String::from_utf8_lossy(&head).to_ascii_lowercase();
            let length = head.lines().find_map(|line| line.strip_prefix("content-length:")).and_then(|value| value.trim().parse::<usize>().ok()).unwrap_or(0);
            let mut body = vec![0u8; length];
            let _ = stream.read_exact(&mut body);
            assert!(head.starts_with("post /spaces/space-a/documents/doc-1/execution-target/component http/1.1"), "{head}");
            assert!(head.contains("authorization: bearer session.v1."), "the asset stream carries the session credential");
            assert_eq!(head.matches("content-type:").count(), 1, "the hub admits exactly one content type");
            served.fetch_add(1, Ordering::SeqCst);
            let answer = queue.lock().unwrap().pop_front().expect("an answer for every component request");
            let respond = |status: &str, kind: &str, body: &[u8], declared: usize| -> Vec<u8> { format!("HTTP/1.1 {status}\r\ncontent-type: {kind}\r\ncontent-length: {declared}\r\nconnection: close\r\n\r\n").into_bytes().into_iter().chain(body.iter().copied()).collect() };
            let json = |status: &str, body: String| respond(status, "application/json", body.as_bytes(), body.len());
            let bytes = match answer.as_str() {
                "exact" => respond("200 OK", "application/octet-stream", &component, component.len()),
                "wrong" => {
                    let mut wrong = component.clone();
                    wrong[0] ^= 0xff;
                    respond("200 OK", "application/octet-stream", &wrong, component.len())
                }
                "short" => respond("200 OK", "application/octet-stream", &component[..component.len() / 2], component.len()),
                "busy" => json("429 Too Many Requests", format!(r#"{{"schema":"semio.hub.rate-limit-refusal/v1","code":"rate-limited","class":"execution-target-asset","retryAfterMs":{busy_retry_after_ms},"message":{{"en":"Too many requests in a short time. Wait a moment, then try again.","de":"Zu viele Anfragen in kurzer Zeit. Bitte einen Moment warten und es dann erneut versuchen."}}}}"#)),
                "unavailable" => json("503 Service Unavailable", r#"{"schema":"semio.hub.document-open-plan-error/v1","code":"deadline-exceeded"}"#.to_string()),
                "unauthorized" => json("401 Unauthorized", r#"{"schema":"semio.hub.document-open-plan-error/v1","code":"denied"}"#.to_string()),
                other => panic!("unknown asset hub answer {other}"),
            };
            let _ = stream.write_all(&bytes);
            let _ = stream.flush();
        }
    });
    AssetHub { origin, requests, answers }
}

/// 🧩️ Components are static assets, over the neutral corpus `🧫️fixtures/🔣️hub-component-asset.json`: a store copy that
/// verifies costs no request — a restarted gateway opens with zero component bytes — while a missing one streams from the
/// hub with its bytes reported as they arrive, a busy hub is a wait, an interrupted stream or a hub shortage is asked again
/// a bounded number of times, and bytes off the lease are refused and never kept. The transfer never touches the per-minute
/// API budget: it runs on a pool of its own.
#[test]
fn a_component_is_a_static_asset_kept_once_and_streamed_only_when_the_store_lacks_it() {
    use semio_framework_os_kernel::os_directory::client::ExecutionTargetModuleStore;
    use semio_framework_os_kernel::os_directory::DocumentExecutionTargetComponentV1;
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️hub-component-asset.json")).unwrap();
    assert_eq!(corpus["streamAttempts"].as_u64(), Some(u64::from(HUB_COMPONENT_STREAM_ATTEMPTS)));
    let length = corpus["component"]["byteLength"].as_u64().unwrap() as usize;
    let component: Vec<u8> = (0..length).map(|index| ((index * 31 + 7) % 251) as u8).collect();
    let expected = DocumentExecutionTargetComponentV1 { sha256: corpus["component"]["sha256"].as_str().unwrap().to_string(), blake3: String::new(), byte_length: length as u64 };
    assert_eq!(framework_hash::sha256_hex(&component), expected.sha256, "the corpus names the component's own content address");
    let runtime = Arc::new(semio_framework_os_services::TokioHostRuntime::new());
    let hub = asset_hub(component.clone(), corpus["busyRetryAfterMs"].as_u64().unwrap());
    let credential = Arc::new(LocalHubCredential::from_minted_session(&hub.origin, &format!("session.v1.{}.{}", "a".repeat(32), "b".repeat(64))).unwrap());
    let intent = DocumentOpenIntentV1 { schema: "semio.hub.document-open-intent/v1".into(), version: 1, scope: DocumentScope::new("space-a", "doc-1"), requested_surface_id: None, client_instance_id: "mcp-component-law".into() };
    for case in corpus["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let root = std::env::temp_dir().join(format!("semio-mcp-component-asset-law-{}-{id}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let seeded = ExecutionTargetModuleStore::new(&root);
        match case["store"].as_str().unwrap() {
            "empty" => {}
            "verified" => seeded.admit_component(&expected.sha256, &component).unwrap(),
            "tampered" => {
                let mut tampered = component.clone();
                tampered[0] ^= 0xff;
                seeded.admit_component(&expected.sha256, &tampered).unwrap();
            }
            other => panic!("unknown store state {other}"),
        }
        for (run, entry) in case["runs"].as_array().unwrap().iter().enumerate() {
            hub.answers.lock().unwrap().extend(entry["hub"].as_array().unwrap().iter().map(|answer| answer.as_str().unwrap().to_string()));
            let before = hub.requests.load(Ordering::SeqCst);
            let assets = HubComponentAssets::new(runtime.clone(), credential.clone(), ExecutionTargetModuleStore::new(&root));
            let mut phases: Vec<&'static str> = Vec::new();
            let mut received: Vec<u64> = Vec::new();
            let outcome = assets.component(&intent, &expected, &CancelToken::root_now(), |transfer| {
                if phases.last() != Some(&transfer.phase.label()) {
                    phases.push(transfer.phase.label());
                }
                if transfer.phase == HubComponentTransferPhase::Receiving {
                    received.push(transfer.received_bytes);
                }
                assert!(transfer.message("law").starts_with(transfer.phase.label()), "every progress message names its phase first");
            });
            let requests = hub.requests.load(Ordering::SeqCst) - before;
            let named = match &outcome {
                Ok(bytes) => {
                    assert!(*bytes == component, "{id} run {run}: only the lease's bytes are ever handed out");
                    if requests == 0 { "store" } else { "hub" }
                }
                Err(error) => match error.code {
                    GatewayErrorCode::PreconditionFailed => "refused",
                    GatewayErrorCode::PluginUnavailable if error.retryable => "unavailable",
                    GatewayErrorCode::PermissionDenied => "denied",
                    _ => panic!("{id} run {run}: unexpected refusal {error:?}"),
                },
            };
            assert_eq!(named, entry["expected"]["outcome"], "{id} run {run}: {outcome:?}", outcome = outcome.as_ref().map(Vec::len));
            assert_eq!(requests as u64, entry["expected"]["componentRequests"].as_u64().unwrap(), "{id} run {run}: component requests");
            assert_eq!(serde_json::json!(phases), entry["expected"]["phases"], "{id} run {run}: progress phases");
            if named == "hub" {
                assert_eq!(received.last(), Some(&(length as u64)), "{id} run {run}: the progress reaches the whole component");
                assert!(received.len() > 2, "{id} run {run}: the bytes are reported as they arrive");
            }
            assert!(hub.answers.lock().unwrap().is_empty(), "{id} run {run}: every declared answer was asked for");
            assert_eq!(ExecutionTargetModuleStore::new(&root).component(&expected).is_some(), outcome.is_ok(), "{id} run {run}: the store keeps exactly the verified component");
        }
        let _ = std::fs::remove_dir_all(&root);
    }
}
