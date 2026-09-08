
use super::*;
use semio_framework_async::{CancelToken, TraceId};
use semio_framework_os_kernel::os_directory::client::{DirectoryWsConnection, DirectoryWsPoll, HttpMethod, HttpResponse, TransportError};
use std::collections::VecDeque;
use std::sync::Mutex;
use std::sync::atomic::AtomicUsize;

#[derive(Clone)]
struct RecordingTransport {
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
        self.responses.lock().unwrap().pop_front().ok_or_else(|| TransportError::Io("fixture response exhausted".to_string()))
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
    let transport = RecordingTransport { responses: Arc::new(Mutex::new(responses)), requests: requests.clone() };
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

    let repo_root = crate::workspace::find_repo_root().expect("repo root");
    let corpus: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(repo_root.join("🌎️hub/🧪️fixtures/📇️directory/🔏️document-execution-target-lease-v1/🔣️.json")).expect("execution-target corpus")).expect("execution-target corpus json");
    let mut manifest: DocumentExecutionTargetLeaseFieldsV1 = semio_framework_os_kernel::os_pack::json::from_json_str(&serde_json::to_string(&corpus["manifest"]).unwrap()).expect("manifest");
    let descriptor = crate::workspace::load_package_descriptor(&repo_root.join("✏️s/🔌️plugins/🌍️gis")).expect("installed GIS descriptor test input");
    let descriptor_bytes = semio_framework_os_kernel::os_store::pack_rt::encode_wire_value(&descriptor.to_value());
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
        responses: Arc::new(Mutex::new(VecDeque::from([HttpResponse { status: 200, body: semio_framework_os_kernel::os_pack::json::to_json_string(&manifest).into_bytes() }, HttpResponse { status: 200, body: descriptor_bytes }]))),
        requests: requests.clone(),
    };
    let client = DirectoryClient::new(transport, "http://hub.invalid");
    let selected = binding.refresh_catalog(&client, &Arc::new(snapshot), &context(Some(20_000))).await.unwrap();
    assert_eq!(selected.selections.len(), 1);
    assert_eq!(selected.selections[0].lease.package, manifest.package);
    assert_eq!(selected.selections[0].descriptor.manifest.plugin_id, "gis");
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
    let message = semio_framework_os_kernel::os_pack::json::from_json_str::<DirectoryStreamMessage>(&contract["cases"]["memberRevoked"]["streamMessage"].to_string()).unwrap();
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
