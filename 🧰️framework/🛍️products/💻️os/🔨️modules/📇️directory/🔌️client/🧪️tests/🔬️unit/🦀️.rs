use super::*;
use crate::os_directory::{DirectoryCommandOutcomeV1, DirectoryCommandResultV1, directory_command_sha256};
use semio_framework_async::{CancelToken, TraceId};
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

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
    let canonical = crate::os_pack::json::to_json_string(&page);
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
    let second = CanonicalDirectoryEventPageV1 {
        canonical_json: "{}".into(),
        session_binding_sha256: page.session_binding_sha256.clone(),
        authorization_generation: page.authorization_generation,
        after_seq_exclusive: 5,
        through_seq_inclusive: 8,
        has_more: false,
        receipt_sha256: "e".repeat(64),
    };
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

    let wake: DirectoryStreamMessage = crate::os_pack::json::from_json_str(
        &serde_json::json!({ "kind": "event", "event": { "seq": 99, "id": "wake", "hlc": { "physicalMs": 1, "logical": 0 }, "actor": { "kind": "system", "id": "sys" }, "body": { "kind": "space.archived", "spaceId": "sp-1" }, "recordedAtMs": 1 } })
            .to_string(),
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
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json")).expect("neutral plan fixture");
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
/// serves, at pinned byte lengths. It is built through the one shared relation, so a test can only
/// make admission fail by changing a real field.
fn document_lease_fields(space_id: &str, document_id: &str, schema: &str, surface_id: &str) -> DocumentExecutionTargetLeaseFieldsV1 {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json")).expect("neutral plan fixture");
    let mut plan = fixture["validPlan"].clone();
    plan["expiresAtUnixMs"] = serde_json::json!(u64::try_from(wall_now_ms()).expect("wall clock") + 20_000);
    plan["scope"] = serde_json::json!({ "spaceId": space_id, "documentId": document_id });
    plan["artifact"]["schema"] = serde_json::json!(schema);
    plan["surface"]["surfaceId"] = serde_json::json!(surface_id);
    plan["checkpoint"]["baselineFrontier"]["documentId"] = serde_json::json!(document_id);
    let decoded: DocumentOpenPlanV1 = crate::os_pack::json::from_json_str(&serde_json::to_string(&plan).expect("plan json")).expect("neutral plan");
    lease_fields_from_plan_v1(&decoded, 1_024, 512, None).expect("non-actor fixture projection")
}

fn document_expectation(schema: &str, surface_id: Option<&str>) -> DocumentSocketExpectationV1 {
    DocumentSocketExpectationV1 { artifact_schema: schema.to_string(), pack_schema_hash: [0x11; 32], requested_surface_id: surface_id.map(str::to_string), lease: None }
}

/// 🪪️ The one shared full-field lease relation, driven by the language-neutral
/// `document-execution-target-lease-v1` corpus: the positive GIS Map viewer vector's plan
/// projection equals its manifest, its exact component and descriptor bytes hash to the declared
/// SHA-256/BLAKE3 digests, and every single-field substitution in the corpus is denied by the
/// same relation the retained `DocumentSocketAuthorityV1` uses.
#[test]
fn execution_target_lease_compares_every_plan_and_verified_byte_field() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../🌎️hub/📇️directory/🧫️fixtures/🔏️document-execution-target-lease-v1/🔣️.json")).expect("execution target lease corpus");
    let decode_fields =
        |value: &serde_json::Value| -> Result<DocumentExecutionTargetLeaseFieldsV1, ()> { crate::os_pack::json::from_json_str::<DocumentExecutionTargetLeaseFieldsV1>(&serde_json::to_string(value).expect("fields json")).map_err(|_| ()) };
    let manifest = decode_fields(&corpus["manifest"]).expect("corpus manifest");
    manifest.validate().expect("corpus manifest is a valid lease projection");
    let hex_bytes = |text: &str| -> Vec<u8> { (0..text.len() / 2).map(|index| u8::from_str_radix(&text[index * 2..index * 2 + 2], 16).expect("hex")).collect() };
    let component = hex_bytes(corpus["componentHex"].as_str().expect("component hex"));
    let descriptor_bytes = hex_bytes(corpus["descriptorHex"].as_str().expect("descriptor hex"));
    assert_eq!(semio_framework_hash::sha256_hex(&component), manifest.component.sha256);
    assert_eq!(semio_framework_hash::hash_bytes(&component), manifest.component.blake3);
    assert_eq!(semio_framework_hash::sha256_hex(&descriptor_bytes), manifest.descriptor.sha256);
    assert_eq!(component.len() as u64, manifest.component.byte_length);
    assert_eq!(descriptor_bytes.len() as u64, manifest.descriptor.byte_length);

    let mut plan_json = corpus["plan"].clone();
    plan_json["expiresAtUnixMs"] = serde_json::json!(u64::try_from(wall_now_ms()).expect("wall clock") + 20_000);
    let plan: DocumentOpenPlanV1 = crate::os_pack::json::from_json_str(&serde_json::to_string(&plan_json).expect("plan json")).expect("corpus plan");
    plan.validate(u64::try_from(wall_now_ms()).expect("wall clock")).expect("corpus plan validates");
    let projected = lease_fields_from_plan_v1(&plan, manifest.component.byte_length, manifest.descriptor.byte_length, manifest.browser_actor.byte_length()).expect("closed actor projection");
    assert!(same_lease_fields_v1(&projected, &manifest));

    let expectation = DocumentSocketExpectationV1 {
        artifact_schema: plan.artifact.schema.clone(),
        pack_schema_hash: decode_lower_hex_32(&plan.artifact.pack_schema_hash).expect("pack schema hash"),
        requested_surface_id: Some(plan.surface.surface_id.clone()),
        lease: Some(manifest.clone()),
    };
    let authority = DocumentSocketAuthorityV1::from_plan(corpus["hubOrigin"].as_str().expect("hub origin").to_string(), &plan, &expectation, u64::try_from(wall_now_ms()).expect("wall clock")).expect("validated retained authority");
    assert!(authority.matches_lease_fields(&manifest));
    let mut unbound = authority.clone();
    unbound.admitted_lease = None;
    assert!(!unbound.matches_lease_fields(&manifest));

    let mut substitutions = 0usize;
    for vector in corpus["hostile"].as_array().expect("hostile rows") {
        if vector["kind"] != "manifest-field" {
            continue;
        }
        assert_eq!(vector["expected"], "unpublished");
        let mut candidate = corpus["manifest"].clone();
        let mut cursor = &mut candidate;
        let path: Vec<&str> = vector["path"].as_str().expect("hostile path").split('.').collect();
        for segment in &path[..path.len() - 1] {
            cursor = cursor.get_mut(*segment).expect("hostile path segment");
        }
        cursor[path[path.len() - 1]] = vector["value"].clone();
        let denied = match decode_fields(&candidate) {
            Err(()) => true,
            Ok(mutated) => mutated.validate().is_err() || (!same_lease_fields_v1(&projected, &mutated) && !authority.matches_lease_fields(&mutated)),
        };
        assert!(denied, "single-field substitution {} was admitted", vector["name"]);
        substitutions += 1;
    }
    assert!(substitutions >= 30, "corpus lost single-field substitutions: {substitutions}");
}

#[test]
fn url_components_percent_encode_utf8_punctuation_without_aliases() {
    assert_eq!(encode_url_component("space /東京?#"), "space%20%2F%E6%9D%B1%E4%BA%AC%3F%23");
    assert_eq!(encode_url_component("a-z_A.9~"), "a-z_A.9~");
}

#[semio_framework_async_macros::async_test]
async fn native_document_admission_issues_validates_and_exchanges_exactly_once() {
    let transport = FakeTransport::default();
    let space_id = "space /東京?";
    let document_id = "document#ä";
    let surface_id = "surface /editor?#";
    let receipt = push_document_plan(&transport, space_id, document_id, "demo/v1", surface_id).await;
    push_grant(&transport).await;
    let client = authenticated_client(transport.clone(), "protected-session");
    let mut expectation = document_expectation("demo/v1", Some(surface_id));
    expectation.lease = Some(document_lease_fields(space_id, document_id, "demo/v1", surface_id));

    let admission = client.admit_document_socket(&root_ctx(), space_id, document_id, &expectation, "native-instance", 30_000).expect("document admission");
    assert_eq!(admission.authority.scope, DocumentScope::new(space_id, document_id));
    assert_eq!(admission.authority.artifact.schema, "demo/v1");
    assert_eq!(admission.authority.surface.surface_id, surface_id);
    assert_eq!(admission.authority.pack_schema_hash, [0x11; 32]);
    assert_eq!(admission.authority.package.plugin_id, "s.gis:地図");
    assert_eq!(admission.authority.package.package_id, "s.gis.gismap:codec");
    assert_eq!(admission.authority.package.version, "1.0.0:β");
    assert_eq!(admission.authority.parent_dialect, DocumentOpenParentDialectV1 { artifact_kind: "s.gis:gismap".into(), standard: "1".into(), subset: "*".into() });
    assert_eq!(admission.authority.surface.app_id, "app.gis");
    assert_eq!(admission.authority.surface.window_kind_id, "window.document");
    assert_eq!(admission.authority.surface.renderer_target, DocumentOpenRendererTargetV1::React);

    let requests = transport.requests.lock().unwrap();
    assert_eq!(requests.len(), 2);
    assert_eq!(requests[0].url, "http://hub.local/spaces/space%20%2F%E6%9D%B1%E4%BA%AC%3F/documents/document%23%C3%A4/open-plan");
    assert_eq!(requests[1].url, "http://hub.local/spaces/space%20%2F%E6%9D%B1%E4%BA%AC%3F/documents/document%23%C3%A4/socket-grants");
    let intent: serde_json::Value = serde_json::from_slice(&requests[0].body).expect("independent intent decode");
    assert_eq!(
        intent,
        serde_json::json!({
            "schema": "semio.hub.document-open-intent/v1",
            "version": 1,
            "scope": { "spaceId": space_id, "documentId": document_id },
            "requestedSurfaceId": surface_id,
            "clientInstanceId": "native-instance"
        })
    );
    let exchange: serde_json::Value = serde_json::from_slice(&requests[1].body).expect("independent exchange decode");
    assert_eq!(
        exchange,
        serde_json::json!({
            "schema": "semio.hub.document-plan-socket-grant-intent/v1",
            "version": 1,
            "planReceipt": receipt
        })
    );
    assert!(!requests[0].body.windows(receipt.len()).any(|window| window == receipt.as_bytes()));
}

#[semio_framework_async_macros::async_test]
async fn hostile_or_cancelled_plan_never_reaches_receipt_exchange() {
    let transport = FakeTransport::default();
    let receipt = push_document_plan(&transport, "space", "document", "foreign/v1", "surface").await;
    let client = authenticated_client(transport.clone(), "protected-session");
    let expectation = document_expectation("expected/v1", Some("surface"));
    let error = match client.admit_document_socket(&root_ctx(), "space", "document", &expectation, "native-instance", 5_000) {
        Err(error) => error,
        Ok(_) => panic!("schema substitution accepted"),
    };
    assert!(!error.to_string().contains(&receipt));
    assert_eq!(transport.requests.lock().unwrap().len(), 1, "invalid plan is never exchanged");

    let cancelled_transport = FakeTransport::default();
    push_document_plan(&cancelled_transport, "space", "document", "expected/v1", "surface").await;
    cancelled_transport.cancel_after_grant.store(true, Ordering::SeqCst);
    let cancelled = authenticated_client(cancelled_transport.clone(), "protected-session");
    assert!(matches!(cancelled.admit_document_socket(&root_ctx(), "space", "document", &expectation, "native-instance", 5_000), Err(DirectoryClientError::Cancelled)));
    assert_eq!(cancelled_transport.requests.lock().unwrap().len(), 1, "cancel after plan prevents exchange");
}

#[semio_framework_async_macros::async_test]
async fn cancellation_after_receipt_exchange_never_reaches_a_document_socket() {
    let transport = FakeTransport::default();
    push_document_plan(&transport, "space", "document", "expected/v1", "surface").await;
    push_grant(&transport).await;
    transport.cancel_after_grant_number.store(2, Ordering::SeqCst);
    let client = authenticated_client(transport.clone(), "protected-session");
    let expectation = document_expectation("expected/v1", Some("surface"));

    assert!(matches!(client.admit_document_socket(&root_ctx(), "space", "document", &expectation, "native-instance", 5_000), Err(DirectoryClientError::Cancelled)));
    assert_eq!(transport.requests.lock().unwrap().len(), 2, "plan and receipt exchange completed once");
    assert!(transport.ws_urls.lock().unwrap().is_empty(), "cancelled exchanged grant never reaches a socket URL or protocol header");
}

#[semio_framework_async_macros::async_test]
async fn protected_document_admission_is_bounded_and_redacts_hostile_responses() {
    let oversized = FakeTransport::default();
    oversized.push_response(Ok(HttpResponse { status: 200, body: vec![b'x'; DOCUMENT_ADMISSION_RESPONSE_MAX_BYTES + 1] })).await;
    let client = authenticated_client(oversized.clone(), "protected-session");
    let expectation = document_expectation("expected/v1", None);
    let error = match client.admit_document_socket(&root_ctx(), "space", "document", &expectation, "native-instance", u64::MAX) {
        Err(error) => error,
        Ok(_) => panic!("oversized plan accepted"),
    };
    assert_eq!(error.to_string(), "decode: protected document admission response exceeded 64 KiB");
    assert_eq!(oversized.requests.lock().unwrap().len(), 1);

    let hostile = FakeTransport::default();
    let secret = format!("plan.v1.{}.{}", "a".repeat(32), "b".repeat(43));
    hostile.push_response(Ok(HttpResponse { status: 500, body: secret.as_bytes().to_vec() })).await;
    let client = authenticated_client(hostile.clone(), "protected-session");
    let error = match client.admit_document_socket(&root_ctx(), "space", "document", &expectation, "native-instance", 5_000) {
        Err(error) => error,
        Ok(_) => panic!("hostile response accepted"),
    };
    assert!(!error.to_string().contains(&secret));
    assert_eq!(error.to_string(), "http 500: protected document admission rejected");
}

#[semio_framework_async_macros::async_test]
async fn mismatched_local_plugin_selection_never_exchanges_a_plan_receipt() {
    let transport = FakeTransport::default();
    let receipt = push_document_plan(&transport, "space", "document", "expected/v1", "surface").await;
    let client = authenticated_client(transport.clone(), "protected-session");
    let mut expectation = document_expectation("expected/v1", Some("surface"));
    let mut lease = document_lease_fields("space", "document", "expected/v1", "surface");
    lease.package.package_id = "foreign.package".into();
    expectation.lease = Some(lease);
    let error = match client.admit_document_socket(&root_ctx(), "space", "document", &expectation, "native-instance", 5_000) {
        Err(error) => error,
        Ok(_) => panic!("foreign plugin selection accepted"),
    };
    assert!(!error.to_string().contains(&receipt));
    assert_eq!(transport.requests.lock().unwrap().len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn ws_url_switches_scheme_and_encodes_query() {
    assert_eq!(directory_ws_url("http://127.0.0.1:8787", 0), "ws://127.0.0.1:8787/directory/socket/v1?since=0");
    assert_eq!(directory_ws_url("https://hub.example", 42), "wss://hub.example/directory/socket/v1?since=42");
    assert_eq!(directory_scoped_ws_url("https://hub.example", &DocumentScope::new("space /a", "document#b"), 7), "wss://hub.example/directory/spaces/space%20%2Fa/documents/document%23b/socket/v1?since=7");
}

#[semio_framework_async_macros::async_test]
async fn scoped_stream_close_4401_is_terminal_and_never_redials() {
    let client = Arc::new(authenticated_client(FakeTransport::default(), "tok"));
    let scope = DocumentScope::new("space-a", "document-a");
    let mut stream = client.stream_scoped(scope.clone(), 9);
    let DirectoryStreamTurn::DialScoped { scope: dial_scope, since, .. } = stream.turn(&root_ctx(), 0) else { panic!("scoped stream must dial exact scope") };
    assert_eq!(dial_scope, scope);
    assert_eq!(since, 9);
    assert!(matches!(stream.complete_dial(0, Ok(FakeWs::with_close_code(4401))), DirectoryStreamTurn::Idle));
    assert!(matches!(stream.turn(&root_ctx(), 1), DirectoryStreamTurn::Revoked(revoked) if revoked == scope));
    assert!(matches!(stream.turn(&root_ctx(), u64::MAX), DirectoryStreamTurn::Closed));
}

#[semio_framework_async_macros::async_test]
async fn scoped_stream_issues_and_dials_the_same_encoded_scope() {
    let transport = FakeTransport::default();
    push_grant(&transport).await;
    transport.push_ws(Ok(std::collections::VecDeque::new())).await;
    let client = authenticated_client(transport.clone(), "tok");
    let scope = DocumentScope::new("space /a", "document#b");
    let _connection = client.open_scoped_stream_ws(&root_ctx(), &scope, 7, 100).expect("scoped socket opens");
    let requests = transport.requests.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].url, "http://hub.local/directory/spaces/space%20%2Fa/documents/document%23b/socket-grants");
    assert!(requests[0].body.is_empty());
    assert_eq!(transport.ws_urls.lock().unwrap().as_slice(), ["ws://hub.local/directory/spaces/space%20%2Fa/documents/document%23b/socket/v1?since=7"]);
}

#[semio_framework_async_macros::async_test]
async fn backoff_doubles_and_caps() {
    assert_eq!(next_backoff_ms(500), 1000);
    assert_eq!(next_backoff_ms(20_000), 30_000);
    assert_eq!(next_backoff_ms(30_000), 30_000);
    assert_eq!(next_backoff_ms(0), HUB_RECONNECT_MIN_MS);
}

#[semio_framework_async_macros::async_test]
async fn spaces_decodes_and_sends_bearer() {
    let transport = FakeTransport::default();
    transport.push_response(FakeTransport::json_response(200, &serde_json::json!([])).await).await;
    let client = authenticated_client(transport.clone(), "tok");

    let spaces = client.spaces(&root_ctx()).await.expect("decodes");
    assert!(spaces.is_empty());
    let requests = transport.requests.lock().unwrap();
    assert_eq!(requests[0].url, "http://hub.local/directory/spaces");
    assert_eq!(requests[0].bearer.as_deref(), Some("tok"));
}

#[semio_framework_async_macros::async_test]
async fn space_exposes_the_durable_document_descriptor() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧫️fixtures/📇️directory/🪪️document-descriptor.json")).expect("descriptor fixture");
    let descriptor = fixture.get("valid").expect("valid descriptor").clone();
    let document: DocumentView = crate::os_pack::json::from_json_str(&serde_json::json!({ "descriptor": descriptor, "headSeq": 7, "commitSeq": 6, "epoch": 2 }).to_string()).expect("document view fixture");
    let space = MemberSpaceViewV1 {
        id: "space-a".into(),
        name: "Fixture".into(),
        kind: crate::os_directory::DirectorySpaceKind::Studio,
        visibility: crate::os_directory::DirectorySpaceVisibility::Private,
        owner_user_id: "user-owner".into(),
        role: DirectorySpaceRole::Author,
        member_count: 1,
        document_count: 1,
        active_connections: 0,
        created_at_ms: 1,
        updated_at_ms: 2,
    };
    let mut page = DirectorySpaceAdministrationPageV1::Author {
        schema: crate::os_directory::DIRECTORY_SPACE_ADMINISTRATION_PAGE_SCHEMA.into(),
        session_binding_sha256: "a".repeat(64),
        authorization_generation: 7,
        space_id: "space-a".into(),
        space,
        members: DirectorySpaceAdministrationMemberWindowV1 { rows: Vec::new(), next_cursor: None },
        documents: DirectorySpaceAdministrationDocumentWindowV1 { rows: vec![document], next_cursor: None },
        invites: DirectorySpaceAdministrationInviteWindowV1 { rows: Vec::new(), next_cursor: None },
        capabilities: DirectorySpaceAdministrationCapabilitiesV1 { rename_space: true, set_visibility: true, delete_space: true, upsert_member: true, remove_member: true, create_invite: true, revoke_invite: true },
        receipt_sha256: String::new(),
    };
    let receipt = semio_framework_hash::sha256_hex(page.canonical_unsigned_json().as_bytes());
    if let DirectorySpaceAdministrationPageV1::Author { receipt_sha256, .. } = &mut page {
        *receipt_sha256 = receipt;
    }
    let canonical = crate::os_pack::json::to_json_string(&page);
    assert!(!canonical.contains("selector") && !canonical.contains("secretDigest") && !canonical.contains("passwordHash"));
    let transport = FakeTransport::default();
    transport.push_response(Ok(HttpResponse { status: 200, body: canonical.clone().into_bytes() })).await;
    let client = authenticated_client(transport.clone(), "member-token");

    let fetched = client.space_administration_page(&root_ctx(), "space-a", None).await.expect("administration page decodes");
    assert_eq!(fetched.canonical_json(), canonical);
    let DirectorySpaceAdministrationPageV1::Author { documents, .. } = fetched.page().clone() else { panic!("author projection") };
    assert_eq!(documents.rows[0].descriptor.document_id, "shared-document");
    assert_eq!(documents.rows[0].descriptor.owner.plugin_id, "s.gis");
    assert_eq!(documents.rows[0].descriptor.bootstrap_frontier.head_seq, 7);
    let requests = transport.requests.lock().unwrap();
    assert_eq!(requests[0].bearer.as_deref(), Some("member-token"));
}

#[semio_framework_async_macros::async_test]
async fn unauthorized_status_maps_to_unauthorized_error() {
    let transport = FakeTransport::default();
    transport.push_response(Ok(HttpResponse { status: 401, body: Vec::new() })).await;
    let client = DirectoryClient::new(transport, "http://hub.local");

    let error = client.me(&root_ctx()).await.expect_err("401 is unauthorized");
    assert!(matches!(error, DirectoryClientError::Unauthorized));
}

#[semio_framework_async_macros::async_test]
async fn session_authority_client_preserves_canonical_binding_and_rejects_reordered_body() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧬️schema/🪪️session-authority-v1/🔣️.json")).expect("session authority fixture");
    let canonical = fixture["raw"][0]["source"].as_str().expect("canonical");
    let reordered = fixture["raw"][1]["source"].as_str().expect("reordered");
    let transport = FakeTransport::default();
    transport.push_response(Ok(HttpResponse { status: 200, body: canonical.as_bytes().to_vec() })).await;
    transport.push_response(Ok(HttpResponse { status: 200, body: reordered.as_bytes().to_vec() })).await;
    let client = authenticated_client(transport, "session-authority-token");
    let authority = client.me(&root_ctx()).await.expect("canonical authority");
    assert_eq!(authority.session_binding_sha256, fixture["bindingGoldens"][0]["sessionBindingSha256"]);
    assert_eq!(authority.authorization_generation, 7);
    assert!(matches!(client.me(&root_ctx()).await, Err(DirectoryClientError::Decode(_))));
}

#[semio_framework_async_macros::async_test]
async fn inference_client_refuses_substituted_hub_receipt_and_page_coordinates() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧫️fixtures/💡️gis-map-inference-port-v1/🔣️.json")).expect("inference fixture");
    let mut substituted_receipt = fixture["wire"]["receipt"].clone();
    substituted_receipt["schema"] = serde_json::json!("semio.hub.inference-job-events/v1");
    let mut substituted_page = fixture["wire"]["page"].clone();
    substituted_page["jobId"] = serde_json::json!("2".repeat(32));
    let transport = FakeTransport::default();
    transport.push_response(Ok(HttpResponse { status: 200, body: substituted_receipt.to_string().into_bytes() })).await;
    transport.push_response(Ok(HttpResponse { status: 200, body: substituted_page.to_string().into_bytes() })).await;
    let client = authenticated_client(transport, "inference-token");
    let scope = DocumentScope { space_id: "space-inference".to_string(), document_id: "document-inference".to_string() };
    let request = GisMapInferenceJobRequestV1 { schema: "semio.hub.inference-job/v1".to_string(), version: 1, request_id: "a".repeat(32), service_id: "s.gis.gismap.inference".to_string(), policy_version: 1, lifetime_ms: 60_000 };
    assert_eq!(client.submit_gis_map_inference_job(&root_ctx(), &scope, &request).await, Err(GisMapInferencePortCodeV1::Invalid));
    assert_eq!(client.read_gis_map_inference_events(&root_ctx(), &scope, "1".repeat(32).as_str(), 0).await, Err(GisMapInferencePortCodeV1::Invalid));
}

#[semio_framework_async_macros::async_test]
async fn stream_reconnects_and_resumes_from_last_seq() {
    let transport = FakeTransport::default();
    transport.push_ws(Err(TransportError::Io("refused".to_string()))).await;
    let event =
        serde_json::json!({ "kind": "event", "event": { "seq": 7, "id": "e1", "hlc": { "physicalMs": 1, "logical": 0 }, "actor": { "kind": "system", "id": "sys" }, "body": { "kind": "space.archived", "spaceId": "sp-1" }, "recordedAtMs": 1 } })
            .to_string();
    transport.push_ws(Ok(std::collections::VecDeque::from([Ok(Some(event)), Ok(None)]))).await;
    transport.push_ws(Err(TransportError::Io("refused again".to_string()))).await;
    push_grant(&transport).await;
    push_grant(&transport).await;
    push_grant(&transport).await;

    let client = authenticated_client(transport.clone(), "tok");
    let mut stream = client.stream(0);
    let ctx = root_ctx();

    let DirectoryStreamTurn::Dial { client: dial, since } = stream.turn(&ctx, 0) else { panic!("first turn must dial") };
    assert!(matches!(stream.complete_dial(0, dial.open_stream_ws(&ctx, since, 100).map_err(|error| TransportError::Io(error.to_string()))), DirectoryStreamTurn::ReconnectAt(HUB_RECONNECT_MIN_MS)));
    assert!(matches!(stream.turn(&ctx, HUB_RECONNECT_MIN_MS), DirectoryStreamTurn::Dial { .. }));
    let result = client.open_stream_ws(&ctx, 0, 100).map_err(|error| TransportError::Io(error.to_string()));
    assert!(matches!(stream.complete_dial(HUB_RECONNECT_MIN_MS, result), DirectoryStreamTurn::Idle));
    match stream.turn(&ctx, HUB_RECONNECT_MIN_MS) {
        DirectoryStreamTurn::Message(DirectoryStreamMessage::Event { event }) => assert_eq!(event.seq, 7),
        _ => panic!("second connection must deliver the event"),
    }
    assert_eq!(stream.since(), 7);
    assert!(matches!(stream.turn(&ctx, HUB_RECONNECT_MIN_MS), DirectoryStreamTurn::ReconnectAt(1_000)));
    let DirectoryStreamTurn::Dial { client: dial, since } = stream.turn(&ctx, 1_000) else { panic!("reconnect deadline must dial") };
    assert!(matches!(stream.complete_dial(1_000, dial.open_stream_ws(&ctx, since, 100).map_err(|error| TransportError::Io(error.to_string()))), DirectoryStreamTurn::ReconnectAt(2_000)));

    let ws_urls = transport.ws_urls.lock().unwrap();
    assert_eq!(ws_urls.len(), 3);
    assert!(ws_urls[0].ends_with("since=0"));
    assert!(ws_urls[2].ends_with("since=7"), "the resumed dial carries the last-seen seq, not the original since");
}

#[semio_framework_async_macros::async_test]
async fn stream_turn_is_bounded_and_preserves_event_order() {
    let transport = FakeTransport::default();
    let frames = (1..=2)
            .map(|seq| {
                Ok(Some(
                    serde_json::json!({ "kind": "event", "event": { "seq": seq, "id": format!("e{seq}"), "hlc": { "physicalMs": seq, "logical": 0 }, "actor": { "kind": "system", "id": "sys" }, "body": { "kind": "space.archived", "spaceId": format!("sp-{seq}") }, "recordedAtMs": seq } })
                        .to_string(),
                ))
            })
            .collect();
    transport.push_ws(Ok(frames)).await;
    push_grant(&transport).await;
    let client = authenticated_client(transport, "tok");
    let ctx = root_ctx();
    let mut stream = client.stream(0);
    let DirectoryStreamTurn::Dial { client, since } = stream.turn(&ctx, 0) else { panic!("first turn must dial") };
    assert!(matches!(stream.complete_dial(0, client.open_stream_ws(&ctx, since, 100).map_err(|error| TransportError::Io(error.to_string()))), DirectoryStreamTurn::Idle));

    let started = std::time::Instant::now();
    let mut seqs = Vec::new();
    for _ in 0..2 {
        match stream.turn(&ctx, 0) {
            DirectoryStreamTurn::Message(DirectoryStreamMessage::Event { event }) => seqs.push(event.seq),
            _ => panic!("scripted event must be delivered"),
        }
    }
    assert_eq!(seqs, vec![1, 2]);
    assert!(started.elapsed() < std::time::Duration::from_millis(8));
}

/// 🧪️ The closed command boundary: a canonical sealed request in, a raw-byte-capped canonical
/// receipt out, closed codes for every failure, and no raw server text in any UI-facing error.
#[semio_framework_async_macros::async_test]
async fn directory_command_parses_only_a_bounded_canonical_receipt_and_never_echoes_server_text() {
    let capability = format!("session.v1.{}.{}", "a".repeat(32), "b".repeat(64));
    let command = DirectoryCommand::CreateInvite { space_id: "space-a".into(), role: DirectorySpaceRole::Spectator, ttl_secs: 3_600 };
    let request = DirectoryCommandRequestV1::new("1f2e3d4c5b6a7988a1b2c3d4e5f60718", command.clone());
    let receipt = DirectoryCommandReceiptV1::seal(request.request_id.clone(), directory_command_sha256(&command), DirectoryCommandOutcomeV1::Accepted, Vec::new(), DirectoryCommandResultV1::Invite { invite_token: "invite.v1.one-shot".into() });
    let canonical = crate::os_pack::json::to_json_string(&receipt);

    let transport = FakeTransport::default();
    transport.push_response(Ok(HttpResponse { status: 200, body: canonical.as_bytes().to_vec() })).await;
    let client = authenticated_client(transport.clone(), &capability);
    let delivered = client.command(&root_ctx(), &request).await.expect("canonical receipt");
    assert_eq!(delivered.canonical_json, canonical);
    assert_eq!(delivered.receipt, receipt);
    assert_eq!(transport.requests.lock().unwrap().first().map(|entry| entry.body.clone()).expect("sealed request bytes"), request.canonical_json().into_bytes());

    for (status, code) in [
        (401u16, DirectoryCommandErrorCodeV1::Unauthorized),
        (403, DirectoryCommandErrorCodeV1::Forbidden),
        (409, DirectoryCommandErrorCodeV1::RequestConflict),
        (413, DirectoryCommandErrorCodeV1::TooLarge),
        (503, DirectoryCommandErrorCodeV1::Overloaded),
        (500, DirectoryCommandErrorCodeV1::Invalid),
    ] {
        let transport = FakeTransport::default();
        transport.push_response(Ok(HttpResponse { status, body: b"hub text a UI-facing error must never carry".to_vec() })).await;
        let client = authenticated_client(transport, &capability);
        let error = client.command(&root_ctx(), &request).await.expect_err("closed denial");
        assert_eq!(error, code, "status {status}");
        assert!(!format!("{error:?}").contains("hub text"));
    }

    let oversized = FakeTransport::default();
    oversized.push_response(Ok(HttpResponse { status: 200, body: vec![b'x'; DIRECTORY_COMMAND_RECEIPT_MAX_BYTES + 1] })).await;
    assert_eq!(authenticated_client(oversized, &capability).command(&root_ctx(), &request).await.expect_err("byte ceiling"), DirectoryCommandErrorCodeV1::TooLarge);

    let forged = FakeTransport::default();
    let mut substituted = receipt.clone();
    substituted.command_sha256 = directory_command_sha256(&DirectoryCommand::RenameSpace { space_id: "space-a".into(), name: "Substituted".into() });
    forged.push_response(Ok(HttpResponse { status: 200, body: crate::os_pack::json::to_json_string(&substituted).into_bytes() })).await;
    assert_eq!(authenticated_client(forged, &capability).command(&root_ctx(), &request).await.expect_err("digest substitution"), DirectoryCommandErrorCodeV1::Invalid);

    let redacted = FakeTransport::default();
    let leaking =
        DirectoryCommandReceiptV1::seal(request.request_id.clone(), directory_command_sha256(&command), DirectoryCommandOutcomeV1::SecretUndeliverable, Vec::new(), DirectoryCommandResultV1::Invite { invite_token: "invite.v1.replayed".into() });
    redacted.push_response(Ok(HttpResponse { status: 200, body: crate::os_pack::json::to_json_string(&leaking).into_bytes() })).await;
    assert_eq!(authenticated_client(redacted, &capability).command(&root_ctx(), &request).await.expect_err("redaction violation"), DirectoryCommandErrorCodeV1::Invalid);

    let cancelled = FakeTransport::default();
    cancelled.push_response(Ok(HttpResponse { status: 200, body: canonical.as_bytes().to_vec() })).await;
    let client = authenticated_client(cancelled.clone(), &capability);
    let ctx = root_ctx();
    ctx.cancel.cancel_now();
    assert_eq!(client.command(&ctx, &request).await.expect_err("pre-cancelled"), DirectoryCommandErrorCodeV1::Cancelled);
    assert!(cancelled.requests.lock().unwrap().is_empty(), "an already-cancelled operation never builds a command request");

    let mut malformed = request.clone();
    malformed.request_id = "not-hex".into();
    assert_eq!(authenticated_client(FakeTransport::default(), &capability).command(&root_ctx(), &malformed).await.expect_err("malformed correlation"), DirectoryCommandErrorCodeV1::Invalid);
}

//#region 🔖️CancellationTests
/// 🧪️ A `ctx` that is ALREADY cancelled before the call starts must never reach the transport
/// at all — `request_json`'s own up-front check (see `DirectoryClientError::Cancelled`'s doc).
#[semio_framework_async_macros::async_test]
async fn a_request_with_an_already_cancelled_context_never_reaches_the_transport() {
    let transport = FakeTransport::default();
    transport.push_response(FakeTransport::json_response(200, &serde_json::json!([])).await).await;
    let client = DirectoryClient::new(transport.clone(), "http://hub.local");
    let ctx = root_ctx();
    ctx.cancel.cancel_now();

    let result = client.spaces(&ctx).await;
    assert!(matches!(result, Err(DirectoryClientError::Cancelled)), "got {result:?}");
    assert!(transport.requests.lock().unwrap().is_empty(), "an already-cancelled context must never even build a request");
}

/// 🧪️ The property this ticket asks for: an IN-FLIGHT request — already past
/// `request_json`'s up-front check, genuinely inside the transport call — is cancelled once its
/// `OperationContext` is cancelled. `FakeTransport::yields_before_response` gives `http()` two
/// cooperative yield points (checking `ctx.cancel` at each); `semio_framework_async::join2` drives
/// the request future and a "canceller" future that cancels after ONE yield in lockstep on the
/// SAME thread — no real time, no real thread, fully deterministic.
#[semio_framework_async_macros::async_test]
async fn an_in_flight_request_is_cancelled_when_its_context_is_cancelled() {
    let transport = FakeTransport::default();
    transport.yields_before_response.store(2, Ordering::SeqCst);
    transport.push_response(FakeTransport::json_response(200, &serde_json::json!([])).await).await;
    let client = DirectoryClient::new(transport.clone(), "http://hub.local");
    let ctx = root_ctx();

    let request_fut = client.spaces(&ctx);
    let canceller_fut = async {
        semio_framework_async::yield_once().await;
        ctx.cancel.cancel().await;
    };
    let (result, ()) = semio_framework_async::join2(request_fut, canceller_fut).await;
    assert!(matches!(result, Err(DirectoryClientError::Transport(TransportError::Cancelled))), "an in-flight request must observe cancellation, got {result:?}");
    assert!(transport.requests.lock().unwrap().is_empty(), "the cancelled call must never reach the scripted response — the response stays queued, unconsumed");
}

/// 🧪️ A cancelled context closes the finite stream state machine rather than reconnecting.
#[semio_framework_async_macros::async_test]
async fn cancelling_the_context_closes_an_open_stream() {
    let transport = FakeTransport::default();
    let client = Arc::new(DirectoryClient::new(transport, "http://hub.local"));
    let mut stream = client.stream(0);
    let ctx = root_ctx();
    ctx.cancel.cancel_now();

    assert!(matches!(stream.turn(&ctx, 0), DirectoryStreamTurn::Closed));
    assert!(matches!(stream.turn(&root_ctx(), 0), DirectoryStreamTurn::Closed));
}

/// 🧪️ Cancellation while a grant/dial is outstanding closes a late socket exactly once.
#[semio_framework_async_macros::async_test]
async fn cancelling_a_pending_grant_dial_closes_its_late_socket() {
    let client = authenticated_client(FakeTransport::default(), "tok");
    let mut stream = client.stream(0);
    let ctx = root_ctx();
    assert!(matches!(stream.turn(&ctx, 0), DirectoryStreamTurn::Dial { .. }));

    ctx.cancel.cancel_now();
    assert!(matches!(stream.turn(&ctx, 1), DirectoryStreamTurn::Closed));
    let closes = Arc::new(AtomicUsize::new(0));
    assert!(matches!(stream.complete_dial(2, Ok(FakeWs::with_close_observer(closes.clone()))), DirectoryStreamTurn::Closed));
    assert_eq!(closes.load(Ordering::SeqCst), 1);
    assert!(matches!(stream.turn(&root_ctx(), 3), DirectoryStreamTurn::Closed));
}

/// 🧪️ A cancellation that races the fresh grant prevents any socket dial from starting.
#[semio_framework_async_macros::async_test]
async fn cancellation_after_grant_refresh_never_opens_or_greets_a_socket() {
    let transport = FakeTransport::default();
    push_grant(&transport).await;
    transport.cancel_after_grant.store(true, Ordering::SeqCst);
    let client = authenticated_client(transport.clone(), "tok");

    assert!(matches!(client.open_stream_ws(&root_ctx(), 0, 100), Err(DirectoryClientError::Cancelled)));
    assert!(transport.ws_urls.lock().unwrap().is_empty());
    assert_eq!(transport.ws_sends.load(Ordering::SeqCst), 0);
    assert_eq!(transport.ws_closes.load(Ordering::SeqCst), 0);
}

/// 🧪️ A cancellation after transport open but before tag-7 greeting closes the socket exactly
/// once and sends no frame.
#[semio_framework_async_macros::async_test]
async fn cancellation_after_socket_open_closes_before_socket_hello() {
    let transport = FakeTransport::default();
    push_grant(&transport).await;
    transport.push_ws(Ok(std::collections::VecDeque::new())).await;
    transport.cancel_after_open.store(true, Ordering::SeqCst);
    let client = authenticated_client(transport.clone(), "tok");

    assert!(matches!(client.open_stream_ws(&root_ctx(), 0, 100), Err(DirectoryClientError::Cancelled)));
    assert_eq!(transport.ws_urls.lock().unwrap().len(), 1);
    assert_eq!(transport.ws_sends.load(Ordering::SeqCst), 0);
    assert_eq!(transport.ws_closes.load(Ordering::SeqCst), 1);
}
//#endregion 🔖️CancellationTests
