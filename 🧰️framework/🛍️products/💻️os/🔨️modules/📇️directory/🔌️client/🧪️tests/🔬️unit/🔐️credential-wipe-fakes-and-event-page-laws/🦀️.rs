/// 🔐️ Credential wipe, fakes and event page laws: the in place credential wipe, the recording fake transport and WS connection, the authenticated client helper, the canonical event page, the consumer credential and inherited frame laws and the grant fixtures.

#[test]
fn intrinsic_bytes_in_decoded_credentials_are_wiped_in_place() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../../../🔨️modules/🌱️value/🧬️bytes/🧪️tests/🧬️base64/🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let bytes: Vec<u8> = serde_json::from_value(row["octets"].clone()).unwrap();
        let mut value = DslValue::Bytes(bytes.clone());
        assert_eq!(wipe_dsl_value(&mut value), bytes.len());
        assert_eq!(value, DslValue::Bytes(vec![0; bytes.len()]));
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RecordedRequest {
    pub method: HttpMethod,
    pub url: String,
    pub bearer: Option<String>,
    pub body: Vec<u8>,
}

#[derive(Clone, Default)]
pub struct FakeTransport {
    pub responses: Arc<Mutex<VecDeque<Result<HttpResponse, TransportError>>>>,
    pub requests: Arc<Mutex<Vec<RecordedRequest>>>,
    pub ws_outcomes: Arc<Mutex<VecDeque<Result<VecDeque<Result<Option<String>, TransportError>>, TransportError>>>>,
    pub ws_urls: Arc<Mutex<Vec<String>>>,
    pub ws_closes: Arc<AtomicUsize>,
    pub ws_sends: Arc<AtomicUsize>,
    pub cancel_after_grant: Arc<AtomicBool>,
    pub cancel_after_grant_number: Arc<AtomicUsize>,
    pub cancel_after_open: Arc<AtomicBool>,
    /// 🧪️ Cooperative yield points `http()` passes through (checking `ctx.cancel` at each one)
    /// BEFORE touching `responses`/`requests` — 0 (the default) keeps every existing test's
    /// synchronous-looking behavior unchanged; a cancellation test sets this > 0 so an
    /// interleaved caller has a real window to flip the token between yields (see
    /// `an_in_flight_request_is_cancelled_when_its_context_is_cancelled` below).
    pub yields_before_response: Arc<AtomicU32>,
    /// 🪢️ The `Accept` media type of every `get_accepting` read, in order.
    pub accepts: Arc<Mutex<Vec<String>>>,
}

impl FakeTransport {
    pub async fn push_response(&self, response: Result<HttpResponse, TransportError>) {
        self.responses.lock().unwrap().push_back(response);
    }

    pub async fn push_ws(&self, outcome: Result<VecDeque<Result<Option<String>, TransportError>>, TransportError>) {
        self.ws_outcomes.lock().unwrap().push_back(outcome);
    }

    pub async fn json_response(status: u16, body: &serde_json::Value) -> Result<HttpResponse, TransportError> {
        Ok(HttpResponse { status, body: serde_json::to_vec(body).unwrap() })
    }
}

// 🔀️ `pub` (was private): now named in `impl DirectoryTransport for FakeTransport`'s public
// `type Ws = FakeWs;` associated type.
pub struct FakeWs {
    frames: VecDeque<Result<Option<String>, TransportError>>,
    close_frame: Option<Option<u16>>,
    closes: Option<Arc<AtomicUsize>>,
    sends: Option<Arc<AtomicUsize>>,
}

impl FakeWs {
    /// 🧪️ Creates a late-dial socket whose close is observable by the cancellation law.
    pub fn with_close_observer(closes: Arc<AtomicUsize>) -> Self {
        Self { frames: VecDeque::new(), close_frame: None, closes: Some(closes), sends: None }
    }

    /// 🛑️ Creates a socket whose next receive preserves one close code.
    pub fn with_close_code(code: u16) -> Self {
        Self { frames: VecDeque::new(), close_frame: Some(Some(code)), closes: None, sends: None }
    }
}

impl DirectoryWsConnection for FakeWs {
    fn send_text(&mut self, _text: String) -> Result<(), TransportError> {
        Ok(())
    }

    fn send_binary(&mut self, _bytes: Vec<u8>) -> Result<(), TransportError> {
        if let Some(sends) = &self.sends {
            sends.fetch_add(1, Ordering::SeqCst);
        }
        Ok(())
    }

    fn try_recv_text(&mut self) -> Result<DirectoryWsPoll, TransportError> {
        if let Some(code) = self.close_frame.take() {
            return Ok(DirectoryWsPoll::Closed(code));
        }
        match self.frames.pop_front() {
            Some(Ok(Some(text))) => Ok(DirectoryWsPoll::Text(text)),
            Some(Ok(None)) => Ok(DirectoryWsPoll::Closed(None)),
            Some(Err(error)) => Err(error),
            None => Ok(DirectoryWsPoll::Pending),
        }
    }

    fn close(&mut self) {
        if let Some(closes) = &self.closes {
            closes.fetch_add(1, Ordering::SeqCst);
        }
    }
}

impl DirectoryTransport for FakeTransport {
    type Ws = FakeWs;
    async fn http(&self, ctx: &OperationContext, method: HttpMethod, url: &str, bearer: Option<&str>, body: Option<Vec<u8>>) -> Result<HttpResponse, TransportError> {
        for _ in 0..self.yields_before_response.load(Ordering::SeqCst) {
            if ctx.cancel.is_cancelled().await {
                return Err(TransportError::Cancelled);
            }
            semio_framework_async::yield_once().await;
        }
        if ctx.cancel.is_cancelled().await {
            return Err(TransportError::Cancelled);
        }
        self.requests.lock().unwrap().push(RecordedRequest { method, url: url.to_string(), bearer: bearer.map(str::to_string), body: body.unwrap_or_default() });
        self.responses.lock().unwrap().pop_front().unwrap_or_else(|| Err(TransportError::Io("no scripted response".to_string())))
    }

    async fn get_accepting(&self, ctx: &OperationContext, url: &str, bearer: Option<&str>, accept: &str) -> Result<HttpResponse, TransportError> {
        self.accepts.lock().unwrap().push(accept.to_string());
        self.http(ctx, HttpMethod::Get, url, bearer, None).await
    }

    fn issue_socket_grant(&self, ctx: &OperationContext, url: &str, bearer: &str, body: &[u8], _timeout_ms: u64) -> Result<HttpResponse, TransportError> {
        if ctx.cancel.is_cancelled_now() {
            return Err(TransportError::Cancelled);
        }
        let request_number = {
            let mut requests = self.requests.lock().unwrap();
            requests.push(RecordedRequest { method: HttpMethod::Post, url: url.to_string(), bearer: Some(bearer.to_string()), body: body.to_vec() });
            requests.len()
        };
        let response = self.responses.lock().unwrap().pop_front().unwrap_or_else(|| Err(TransportError::Io("no scripted response".to_string())));
        if self.cancel_after_grant.load(Ordering::SeqCst) || self.cancel_after_grant_number.load(Ordering::SeqCst) == request_number {
            ctx.cancel.cancel_now();
        }
        response
    }

    fn open_ws(&self, ctx: &OperationContext, url: &str, _protocols: &[String], _timeout_ms: u64) -> Result<Self::Ws, TransportError> {
        if ctx.cancel.is_cancelled_now() {
            return Err(TransportError::Cancelled);
        }
        self.ws_urls.lock().unwrap().push(url.to_string());
        let frames = self.ws_outcomes.lock().unwrap().pop_front().unwrap_or_else(|| Err(TransportError::Io("no scripted ws".to_string())))?;
        if self.cancel_after_open.load(Ordering::SeqCst) {
            ctx.cancel.cancel_now();
        }
        Ok(FakeWs { frames, close_frame: None, closes: Some(self.ws_closes.clone()), sends: Some(self.ws_sends.clone()) })
    }
}

fn root_ctx() -> OperationContext {
    OperationContext { actor: 0, generation: 0, trace: TraceId(0), lane: 0, deadline_ms: None, cancel: CancelToken::root_now(), capability: None }
}

fn authenticated_client(transport: FakeTransport, capability: &str) -> Arc<DirectoryClient<FakeTransport>> {
    Arc::new(DirectoryClient::authenticated(transport, Arc::new(LocalHubCredential::test("http://hub.local", capability))))
}

#[semio_framework_async_macros::async_test]
async fn directory_event_page_preserves_canonical_bytes_bounds_and_cancels_before_io() {
    let capability = format!("session.v1.{}.{}", "a".repeat(32), "b".repeat(64));
    let mut page = DirectoryEventPageV1 {
        schema: "semio.directory.event-page.v1".into(),
        session_binding_sha256: "c".repeat(64),
        authorization_generation: 9,
        after_seq_exclusive: 3,
        through_seq_inclusive: 5,
        has_more: true,
        events: Vec::new(),
        receipt_sha256: String::new(),
    };
    page.receipt_sha256 = semio_framework_hash::sha256_hex(page.canonical_unsigned_json().as_bytes());
    let canonical = semio_framework_pack_json::to_json_string(&page);
    let transport = FakeTransport::default();
    transport.push_response(Ok(HttpResponse { status: 200, body: canonical.as_bytes().to_vec() })).await;
    let client = authenticated_client(transport.clone(), &capability);
    let retained = client.event_page(&root_ctx(), 3).await.expect("canonical page");
    assert_eq!(retained.canonical_json(), canonical);
    assert_eq!(retained.session_binding_sha256(), page.session_binding_sha256);
    assert_eq!(retained.authorization_generation(), 9);
    assert_eq!(retained.after_seq_exclusive(), 3);
    assert_eq!(retained.through_seq_inclusive(), 5);
    assert!(retained.has_more());
    assert_eq!(retained.receipt_sha256(), page.receipt_sha256);
    let mut bootstrap = DirectoryEventPageBootstrapV1::new(7, 3).expect("bootstrap owner");
    let first_ack = bootstrap.present(retained.clone()).expect("first pending page");
    let mut forged_ack = first_ack.clone();
    forged_ack.receipt_sha256 = "d".repeat(64);
    assert!(bootstrap.acknowledge(&forged_ack).is_err());
    assert_eq!(bootstrap.after(), 3);
    assert_eq!(bootstrap.acknowledge(&first_ack).expect("first Home ACK"), DirectoryBootstrapTransition::Fetch { after: 5 });
    let second = CanonicalDirectoryEventPageV1::from_header_parts("{}".into(), page.session_binding_sha256.clone(), page.authorization_generation, 5, 8, false, "e".repeat(64));
    let second_ack = bootstrap.present(second).expect("second pending page");
    assert_eq!(bootstrap.acknowledge(&second_ack).expect("final Home ACK"), DirectoryBootstrapTransition::Live { since: 8 });
    assert_eq!(bootstrap.wake(false), Some(8));
    assert_eq!(bootstrap.wake(false), None, "a dirty burst cannot duplicate the page fetch");

    let mut rejected = DirectoryEventPageBootstrapV1::new(7, 3).expect("retry owner");
    let rejected_ack = rejected.present(retained.clone()).expect("pending retry page");
    assert_eq!(rejected.reject(7, &rejected_ack.receipt_sha256).expect("exact rejection"), 3);
    rejected.close();
    assert!(rejected.present(retained.clone()).is_err());
    let requests = transport.requests.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, HttpMethod::Get);
    assert_eq!(requests[0].url, "http://hub.local/directory/event-page/v1?after=3");
    assert_eq!(requests[0].bearer.as_deref(), Some(capability.as_str()));
    drop(requests);

    let cancelled_transport = FakeTransport::default();
    cancelled_transport.push_response(Ok(HttpResponse { status: 200, body: canonical.as_bytes().to_vec() })).await;
    let cancelled_client = authenticated_client(cancelled_transport.clone(), &capability);
    let cancelled_ctx = root_ctx();
    cancelled_ctx.cancel.cancel_now();
    assert!(matches!(cancelled_client.event_page(&cancelled_ctx, 3).await, Err(DirectoryClientError::Cancelled)));
    assert!(cancelled_transport.requests.lock().unwrap().is_empty());

    let oversized_transport = FakeTransport::default();
    oversized_transport.push_response(Ok(HttpResponse { status: 200, body: vec![b'x'; DIRECTORY_EVENT_PAGE_MAX_BYTES + 1] })).await;
    assert!(matches!(authenticated_client(oversized_transport, &capability).event_page(&root_ctx(), 0).await, Err(DirectoryClientError::Decode(_))));

    let noncanonical_transport = FakeTransport::default();
    noncanonical_transport.push_response(Ok(HttpResponse { status: 200, body: format!("{canonical} ").into_bytes() })).await;
    assert!(matches!(authenticated_client(noncanonical_transport, &capability).event_page(&root_ctx(), 3).await, Err(DirectoryClientError::Decode(_))));

    let unsafe_transport = FakeTransport::default();
    assert!(matches!(authenticated_client(unsafe_transport.clone(), &capability).event_page(&root_ctx(), DOCUMENT_OPEN_MAX_SAFE_INTEGER + 1).await, Err(DirectoryClientError::Decode(_))));
    assert!(unsafe_transport.requests.lock().unwrap().is_empty());

    let wake: DirectoryStreamMessage = semio_framework_pack_json::from_json_str(
        &serde_json::json!({ "kind": "event", "event": { "seq": 99, "id": "wake", "hlc": { "physicalMs": 1, "logical": 0 }, "actor": { "kind": "system", "id": "sys" }, "body": { "kind": "space.archived", "spaceId": "sp-1" }, "recordedAtMs": 1 } })
            .to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject,
    )
    .expect("wakeup event");
    let mut acknowledged = client.stream_acknowledged(3).expect("acknowledged stream");
    acknowledged.track(&wake);
    acknowledged.track(&DirectoryStreamMessage::Heartbeat { head_seq: 101 });
    assert_eq!(acknowledged.since(), 3, "observed wakeups never advance the committed cursor");
    acknowledged.acknowledge(5).expect("exact Home ACK");
    assert_eq!(acknowledged.since(), 5);
    assert!(acknowledged.acknowledge(4).is_err());
    assert!(acknowledged.acknowledge(DOCUMENT_OPEN_MAX_SAFE_INTEGER + 1).is_err());

    let mut observed = client.stream(3);
    observed.track(&wake);
    assert_eq!(observed.since(), 99, "legacy observed-frontier stream remains explicit");
}

#[test]
fn decoded_consumer_credential_moves_success_secret_and_wipes_every_invalid_field() {
    let capability = format!("session.v1.{}.{}", "a".repeat(32), "b".repeat(64));
    let schema = "semio.local.consumer-credential/v1";
    let client_class = "native";
    let hub_origin = "http://127.0.0.1:8787";
    let decoded_keys_len = "schema".len() + "clientClass".len() + "hubOrigin".len() + "capability".len() + "expiresAtMs".len();
    let success_wipe_len = decoded_keys_len + schema.len() + client_class.len() + hub_origin.len();
    let envelope = serde_json::to_vec(&serde_json::json!({
        "schema": schema,
        "clientClass": client_class,
        "hubOrigin": hub_origin,
        "capability": capability,
        "expiresAtMs": wall_now_ms() + 30_000
    }))
    .expect("external JSON credential oracle");
    let success_wipes = AtomicUsize::new(0);
    let credential = decode_local_hub_credential(&envelope, "native", Some(&success_wipes)).expect("valid credential");
    assert_eq!(credential.capability().expect("credential capability"), capability);
    assert_eq!(success_wipes.load(Ordering::SeqCst), success_wipe_len, "success wipes every decoded key and non-secret string while moving the sole capability allocation");

    let invalid_wipes = AtomicUsize::new(0);
    assert!(matches!(decode_local_hub_credential(&envelope, "mcp", Some(&invalid_wipes)), Err(DirectoryClientError::Unauthorized)));
    assert_eq!(invalid_wipes.load(Ordering::SeqCst), success_wipe_len + capability.len(), "invalid-after-decode wipes every decoded string byte including the capability");
}

#[test]
fn non_ascii_capability_crossing_a_fixed_boundary_is_denied_without_panicking_and_fully_wiped() {
    use std::io::Cursor;

    let capability = format!("session.v1.{}é.{}", "a".repeat(31), "b".repeat(63));
    assert_eq!(capability.len(), 108);
    let schema = "semio.local.consumer-credential/v1";
    let client_class = "native";
    let hub_origin = "http://127.0.0.1:8787";
    let decoded_keys_len = "schema".len() + "clientClass".len() + "hubOrigin".len() + "capability".len() + "expiresAtMs".len();
    let expected_wipe_len = decoded_keys_len + schema.len() + client_class.len() + hub_origin.len() + capability.len();
    let envelope = serde_json::to_vec(&serde_json::json!({
        "schema": schema,
        "clientClass": client_class,
        "hubOrigin": hub_origin,
        "capability": capability,
        "expiresAtMs": wall_now_ms() + 30_000
    }))
    .expect("external JSON credential oracle");
    let mut framed = Vec::with_capacity(4 + envelope.len());
    framed.extend_from_slice(&u32::try_from(envelope.len()).expect("bounded envelope").to_be_bytes());
    framed.extend_from_slice(&envelope);
    let raw_wipes = AtomicUsize::new(0);
    let mut cursor = Cursor::new(framed);
    let raw = read_local_hub_credential_frame(&mut cursor, Some(&raw_wipes)).expect("complete frame");
    let decoded_wipes = AtomicUsize::new(0);
    assert!(matches!(decode_local_hub_credential(&raw.bytes, client_class, Some(&decoded_wipes)), Err(DirectoryClientError::Unauthorized)));
    drop(raw);
    assert_eq!(decoded_wipes.load(Ordering::SeqCst), expected_wipe_len);
    assert_eq!(raw_wipes.load(Ordering::SeqCst), envelope.len() + 1);
}

#[test]
fn inherited_frame_reader_wipes_exactly_every_allocated_body_and_trailing_probe_byte() {
    use std::io::Cursor;

    let partial_wipes = AtomicUsize::new(0);
    let mut partial = Cursor::new([8u32.to_be_bytes().as_slice(), b"abc"].concat());
    assert!(matches!(read_local_hub_credential_frame(&mut partial, Some(&partial_wipes)), Err(DirectoryClientError::Unauthorized)));
    assert_eq!(partial_wipes.load(Ordering::SeqCst), 8);

    let complete_wipes = AtomicUsize::new(0);
    let mut complete = Cursor::new([4u32.to_be_bytes().as_slice(), b"body"].concat());
    drop(read_local_hub_credential_frame(&mut complete, Some(&complete_wipes)).expect("complete frame"));
    assert_eq!(complete_wipes.load(Ordering::SeqCst), 5);

    let trailing_wipes = AtomicUsize::new(0);
    let mut trailing = Cursor::new([4u32.to_be_bytes().as_slice(), b"body", b"x"].concat());
    assert!(matches!(read_local_hub_credential_frame(&mut trailing, Some(&trailing_wipes)), Err(DirectoryClientError::Unauthorized)));
    assert_eq!(trailing_wipes.load(Ordering::SeqCst), 5);
}

async fn push_document_grant(transport: &FakeTransport) {
    transport
        .push_response(
            FakeTransport::json_response(
                200,
                &serde_json::json!({
                    "schema": "semio.hub.document-socket-grant/v1",
                    "protocol": "semio.session.v1",
                    "actorId": format!("hub.v1.{}", "3".repeat(64)),
                    "expiresAtMs": wall_now_ms() + 30_000
                }),
            )
            .await,
        )
        .await;
}

async fn push_grant(transport: &FakeTransport) {
    transport
        .push_response(
            FakeTransport::json_response(
                200,
                &serde_json::json!({
                    "schema": "semio.hub.socket-grant/v1",
                    "protocol": "semio.socket.v1",
                    "grant": format!("socket.v1.{}.{}", "1".repeat(32), "2".repeat(64)),
                    "actorId": format!("hub.v1.{}", "3".repeat(64)),
                    "expiresAtMs": wall_now_ms() + 30_000
                }),
            )
            .await,
        )
        .await;
}

async fn push_document_plan(transport: &FakeTransport, space_id: &str, document_id: &str, schema: &str, surface_id: &str) -> String {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../🧪️tests/🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json")).expect("neutral plan fixture");
    let mut plan = fixture["validPlan"].clone();
    plan["expiresAtUnixMs"] = serde_json::json!(u64::try_from(wall_now_ms()).expect("wall clock") + 20_000);
    plan["scope"] = serde_json::json!({ "spaceId": space_id, "documentId": document_id });
    plan["artifact"]["schema"] = serde_json::json!(schema);
    plan["surface"]["surfaceId"] = serde_json::json!(surface_id);
    plan["checkpoint"]["baselineFrontier"]["documentId"] = serde_json::json!(document_id);
    let receipt = plan["receipt"].as_str().expect("fixture receipt").to_string();
    transport.push_response(FakeTransport::json_response(200, &plan).await).await;
    receipt
}

/// 🪪️ The exact receipt-free lease projection of the same neutral plan {@link push_document_plan}
/// serves, at the component length its receipt names and a pinned descriptor length. It is built through the one shared relation, so a test can only
/// make admission fail by changing a real field.
fn document_lease_fields(space_id: &str, document_id: &str, schema: &str, surface_id: &str) -> DocumentExecutionTargetLeaseFieldsV1 {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../🧪️tests/🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json")).expect("neutral plan fixture");
    let mut plan = fixture["validPlan"].clone();
    plan["expiresAtUnixMs"] = serde_json::json!(u64::try_from(wall_now_ms()).expect("wall clock") + 20_000);
    plan["scope"] = serde_json::json!({ "spaceId": space_id, "documentId": document_id });
    plan["artifact"]["schema"] = serde_json::json!(schema);
    plan["surface"]["surfaceId"] = serde_json::json!(surface_id);
    plan["checkpoint"]["baselineFrontier"]["documentId"] = serde_json::json!(document_id);
    let decoded: DocumentOpenPlanV1 = semio_framework_pack_json::from_json_str(&serde_json::to_string(&plan).expect("plan json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("neutral plan");
    lease_fields_from_plan_v1(&decoded, decoded.package.component_receipt.wasm_bytes, 512, None).expect("non-actor fixture projection")
}

fn document_expectation(schema: &str, surface_id: Option<&str>) -> DocumentSocketExpectationV1 {
    DocumentSocketExpectationV1 { artifact_schema: schema.to_string(), pack_schema_hash: [0x11; 32], requested_surface_id: surface_id.map(str::to_string), lease: None }
}

/// 🌐️ The execution-target status vocabulary is ONE vocabulary: every code of the language-neutral corpus
/// (`document-execution-target-lease-v1` `expected.status` / `statusRoles`) decodes to the Rust twin with the corpus's exact
/// English and German text and live-region role, and the twin has no code the corpus lacks.
#[test]
fn execution_target_status_vocabulary_matches_the_corpus() {
    use crate::schema::{DocumentExecutionTargetLocaleV1, DocumentExecutionTargetStatusCodeV1};
    let corpus: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../🧪️tests/🧫️fixtures/🔏️document-execution-target-lease-v1/🔣️.json")).expect("execution target lease corpus");
    let status = corpus["expected"]["status"].as_object().expect("corpus status vocabulary");
    for (code, text) in status {
        let decoded: DocumentExecutionTargetStatusCodeV1 = semio_framework_pack_json::from_json_str(&format!("\"{code}\""), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_else(|_| panic!("the Rust twin lacks status {code}"));
        assert_eq!(decoded.text(DocumentExecutionTargetLocaleV1::En), text["en"].as_str().expect("en"), "{code} en");
        assert_eq!(decoded.text(DocumentExecutionTargetLocaleV1::De), text["de"].as_str().expect("de"), "{code} de");
        assert_eq!(decoded.aria_role(), corpus["expected"]["statusRoles"][code.as_str()].as_str().expect("role"), "{code} role");
    }
    assert_eq!(status.len(), 9, "the corpus names every status the twins speak");
}
