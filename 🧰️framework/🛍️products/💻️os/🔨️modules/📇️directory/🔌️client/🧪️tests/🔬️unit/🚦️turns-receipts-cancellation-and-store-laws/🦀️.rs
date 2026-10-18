/// 🚦️ Turns, receipts, cancellation and store laws: the bounded stream turn, the canonical receipt parse, the cancellation region, the execution target module resolution, the component store eviction, the canonical pair and the delegation vectors.

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
    let canonical = semio_framework_pack_json::to_json_string(&receipt);

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
    forged.push_response(Ok(HttpResponse { status: 200, body: semio_framework_pack_json::to_json_string(&substituted).into_bytes() })).await;
    assert_eq!(authenticated_client(forged, &capability).command(&root_ctx(), &request).await.expect_err("digest substitution"), DirectoryCommandErrorCodeV1::Invalid);

    let redacted = FakeTransport::default();
    let leaking =
        DirectoryCommandReceiptV1::seal(request.request_id.clone(), directory_command_sha256(&command), DirectoryCommandOutcomeV1::SecretUndeliverable, Vec::new(), DirectoryCommandResultV1::Invite { invite_token: "invite.v1.replayed".into() });
    redacted.push_response(Ok(HttpResponse { status: 200, body: semio_framework_pack_json::to_json_string(&leaking).into_bytes() })).await;
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

/// 🧩️ Resolving the component a hub document runs, by the serving catalog generation, against the neutral
/// fixture `🧫️fixtures/📇️directory/🧩️execution-target-module-resolution-v1.json` over the hub's own lease corpus
/// (`document-execution-target-lease-v1`, whose digests the hub minted independently): the local copy only on an
/// equal content hash, a store entry only when its bytes verify, otherwise the hub's bytes — verified before they
/// are stored — and never anything off the lease; each case names the requests and steps it takes.
#[semio_framework_async_macros::async_test]
async fn execution_target_module_resolution_follows_the_serving_generation() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../🧪️tests/🧫️fixtures/📇️directory/🧩️execution-target-module-resolution-v1.json")).expect("resolution fixture");
    let corpus: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../🧪️tests/🧫️fixtures/🔏️document-execution-target-lease-v1/🔣️.json")).expect("execution target lease corpus");
    let hex_bytes = |text: &str| -> Vec<u8> { (0..text.len() / 2).map(|index| u8::from_str_radix(&text[index * 2..index * 2 + 2], 16).expect("hex")).collect() };
    let component = hex_bytes(corpus["componentHex"].as_str().expect("component hex"));
    let descriptor = hex_bytes(corpus["descriptorHex"].as_str().expect("descriptor hex"));
    let intent: DocumentOpenIntentV1 = semio_framework_pack_json::from_json_str(&serde_json::to_string(&corpus["intent"]).expect("intent json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("corpus intent");
    let lease: DocumentExecutionTargetLeaseFieldsV1 = semio_framework_pack_json::from_json_str(&serde_json::to_string(&corpus["manifest"]).expect("manifest json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("corpus manifest");
    let body = |bytes: &[u8]| Ok(HttpResponse { status: 200, body: bytes.to_vec() });
    let flipped = |bytes: &[u8]| {
        let mut bytes = bytes.to_vec();
        bytes[0] ^= 0xff;
        bytes
    };
    for case in fixture["cases"].as_array().expect("cases") {
        let id = case["id"].as_str().expect("case id");
        let root = std::env::temp_dir().join(format!("semio-execution-target-law-{}-{id}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let store = ExecutionTargetModuleStore::new(&root);
        let (component_path, descriptor_path) = (store.component_path(&lease.component.sha256), store.descriptor_path(&lease.descriptor.sha256));
        let seed = |path: &std::path::Path, bytes: &[u8]| {
            std::fs::create_dir_all(path.parent().expect("store parent")).expect("store dir");
            std::fs::write(path, bytes).expect("seed store entry");
        };
        match case["store"].as_str().expect("store") {
            "empty" => {}
            "verified" => {
                seed(&component_path, &component);
                seed(&descriptor_path, &descriptor);
            }
            "tamperedComponent" => {
                seed(&component_path, &flipped(&component));
                seed(&descriptor_path, &descriptor);
            }
            "tamperedDescriptor" => {
                seed(&component_path, &component);
                seed(&descriptor_path, &flipped(&descriptor));
            }
            other => panic!("unknown store state {other}"),
        }
        let transport = FakeTransport::default();
        let hub = case["hub"].as_str().expect("hub");
        let mut asked = std::collections::HashMap::new();
        for request in case["expected"]["requests"].as_array().expect("expected requests") {
            let kind = request.as_str().expect("request");
            let count = asked.entry(kind).or_insert(0usize);
            *count += 1;
            if hub == "unavailable" || (hub == "unavailableOnce" && *count == 1) {
                transport.push_response(Ok(HttpResponse { status: 503, body: br#"{"schema":"semio.hub.document-open-plan-error/v1","code":"deadline-exceeded"}"#.to_vec() })).await;
                continue;
            }
            let response = match kind {
                "manifest" => FakeTransport::json_response(200, &corpus["manifest"]).await,
                "component" => body(&if hub == "wrongComponent" { flipped(&component) } else { component.clone() }),
                _ => body(&if hub == "wrongDescriptor" { flipped(&descriptor) } else { descriptor.clone() }),
            };
            transport.push_response(response).await;
        }
        let local = match case["local"].as_str().expect("local") {
            "leaseComponent" => Some(lease.component.sha256.clone()),
            "otherComponent" => Some("0".repeat(64)),
            _ => None,
        };
        let client = DirectoryClient::new(transport.clone(), "http://hub.local");
        let ctx = root_ctx();
        let mut steps = Vec::new();
        let outcome = client
            .resolve_execution_target_module(&ctx, &intent, local.as_deref(), &store, |step| {
                if hub == "cancelledAtComponent" && step == ExecutionTargetModuleStep::Component {
                    ctx.cancel.cancel_now();
                }
                steps.push(match step {
                    ExecutionTargetModuleStep::Lease => "lease",
                    ExecutionTargetModuleStep::Component => "component",
                    ExecutionTargetModuleStep::Descriptor => "descriptor",
                    ExecutionTargetModuleStep::Verified => "verified",
                });
            })
            .await;
        let requests: Vec<&str> = transport
            .requests
            .lock()
            .unwrap()
            .iter()
            .map(|request| if request.url.ends_with("/execution-target/manifest") { "manifest" } else if request.url.ends_with("/execution-target/component") { "component" } else { "descriptor" })
            .collect();
        let named = match &outcome {
            Ok(resolved) => match resolved.source {
                ExecutionTargetModuleSource::Local => "local",
                ExecutionTargetModuleSource::Store => "store",
                ExecutionTargetModuleSource::Hub => "hub",
            },
            Err(DirectoryClientError::Cancelled) | Err(DirectoryClientError::Transport(TransportError::Cancelled)) => "cancelled",
            Err(_) => "refused",
        };
        assert_eq!(named, case["expected"]["outcome"], "{id}: {outcome:?}");
        assert_eq!(serde_json::json!(requests), case["expected"]["requests"], "{id} requests");
        assert_eq!(serde_json::json!(steps), case["expected"]["steps"], "{id} steps");
        match &outcome {
            Ok(resolved) if resolved.source == ExecutionTargetModuleSource::Local => assert!(resolved.files.is_none(), "{id}: the local component needs no files"),
            Ok(resolved) => {
                let files = resolved.files.as_ref().expect("resolved files");
                assert_eq!(std::fs::read(&files.component).expect("stored component"), component, "{id}: the mounted component is the lease's bytes");
                assert_eq!(std::fs::read(&files.descriptor).expect("stored descriptor"), descriptor, "{id}: the mounted descriptor is the lease's bytes");
            }
            Err(_) => {
                for (path, bytes) in [(&component_path, &component), (&descriptor_path, &descriptor)] {
                    assert!(std::fs::read(path).map_or(true, |stored| stored == *bytes || case["store"] != "empty"), "{id}: nothing off the lease is stored");
                }
                if case["store"] == "empty" && hub != "wrongDescriptor" {
                    assert!(!component_path.exists(), "{id}: a refused or cancelled resolution leaves no component behind");
                }
            }
        }
        let _ = std::fs::remove_dir_all(&root);
    }
}

/// 🗄️ The execution-target store's own bound over the fixture's `eviction` corpus: components beyond the capacity are
/// evicted least recently used first (admitting or reading uses an entry, the one just admitted stays), a read serves only
/// bytes that still verify and removes an entry that does not, and a second store over the same root — a restarted process —
/// serves what the first one kept. The steps pause so every use lands on a distinct file time even on a coarse clock.
#[test]
fn the_execution_target_store_evicts_the_least_recently_used_component_and_serves_only_verified_bytes() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../🧪️tests/🧫️fixtures/📇️directory/🧩️execution-target-module-resolution-v1.json")).expect("resolution fixture");
    let eviction = &fixture["eviction"];
    let capacity = eviction["componentCapacityBytes"].as_u64().expect("capacity");
    let components: std::collections::BTreeMap<String, (Vec<u8>, DocumentExecutionTargetComponentV1)> = eviction["components"]
        .as_object()
        .expect("components")
        .iter()
        .map(|(name, component)| {
            let bytes = vec![component["fill"].as_u64().expect("fill") as u8; component["byteLength"].as_u64().expect("length") as usize];
            let expected = DocumentExecutionTargetComponentV1 { sha256: component["sha256"].as_str().expect("sha256").to_string(), blake3: String::new(), byte_length: bytes.len() as u64 };
            assert_eq!(semio_framework_hash::sha256_hex(&bytes), expected.sha256, "{name}: the fixture's content address is the bytes' own");
            (name.clone(), (bytes, expected))
        })
        .collect();
    let root = std::env::temp_dir().join(format!("semio-execution-target-eviction-law-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    let mut store = ExecutionTargetModuleStore::with_component_capacity(&root, capacity);
    for (index, step) in eviction["steps"].as_array().expect("steps").iter().enumerate() {
        std::thread::sleep(std::time::Duration::from_millis(20));
        let named = step.get("component").and_then(serde_json::Value::as_str).map(|name| &components[name]);
        match step["op"].as_str().expect("op") {
            "admit" => store.admit_component(&named.expect("admitted").1.sha256, &named.expect("admitted").0).expect("admit"),
            "read" => {
                let (bytes, expected) = named.expect("read");
                let served = store.component(expected);
                assert_eq!(served.is_some(), step["hit"].as_bool().expect("hit"), "step {index}: read");
                assert!(served.is_none_or(|served| served == *bytes), "step {index}: a hit serves exactly the admitted bytes");
            }
            "tamper" => {
                let path = store.component_path(&named.expect("tampered").1.sha256);
                let mut bytes = std::fs::read(&path).expect("tampered entry");
                bytes[0] ^= 0xff;
                std::fs::write(&path, bytes).expect("tamper");
            }
            "restart" => store = ExecutionTargetModuleStore::with_component_capacity(&root, capacity),
            other => panic!("unknown eviction op {other}"),
        }
        let resident: Vec<&str> = components.iter().filter(|(_, (_, expected))| store.component_path(&expected.sha256).exists()).map(|(name, _)| name.as_str()).collect();
        assert_eq!(serde_json::json!(resident), step["resident"], "step {index}: resident components");
    }
    let _ = std::fs::remove_dir_all(&root);
}

/// 🪢️ The native pair fetch over the shared fixture: exact route and `Accept`, a transient refusal asked again, a final one
/// named, and a pair admitted only as the checkpoint the open was authorized for.
#[semio_framework_async_macros::async_test]
async fn the_canonical_pair_is_fetched_verified_and_admitted_as_the_authorized_checkpoint() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../🧪️tests/🧫️fixtures/📇️directory/🪢️canonical-checkpoint-pair-v1.json")).expect("canonical pair fixture");
    let capability = format!("session.v1.{}.{}", "a".repeat(32), "b".repeat(64));
    let hex = |value: &serde_json::Value| -> Vec<u8> {
        let text = value.as_str().unwrap();
        (0..text.len()).step_by(2).map(|index| u8::from_str_radix(&text[index..index + 2], 16).unwrap()).collect()
    };
    for case in fixture["admissions"].as_array().unwrap() {
        let pair = fixture["pairs"].as_array().unwrap().iter().find(|pair| pair["id"] == case["pair"]).unwrap();
        let expected: DocumentOpenCheckpointV1 = semio_framework_pack_json::from_json_str(&case["expected"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("fixture checkpoint");
        let scope = DocumentScope::new(case["scope"]["spaceId"].as_str().unwrap(), case["scope"]["documentId"].as_str().unwrap());
        let transport = FakeTransport::default();
        transport.push_response(Ok(HttpResponse { status: 503, body: Vec::new() })).await;
        transport.push_response(Ok(HttpResponse { status: 200, body: hex(&pair["bodyHex"]) })).await;
        let client = authenticated_client(transport.clone(), &capability);
        let answer = client.document_canonical_checkpoint_pair(&root_ctx(), &scope, &expected).await;
        let id = &case["id"];
        match case["refusal"].as_str() {
            None => assert_eq!(answer.as_ref().map(|decoded| decoded.pack_bytes.len() as u64).ok(), pair["pack"]["length"].as_u64(), "{id}"),
            Some(code) => assert!(matches!(&answer, Err(DirectoryClientError::Decode(detail)) if detail == code), "{id}: {answer:?}"),
        }
        let requests = transport.requests.lock().unwrap();
        assert_eq!(requests.len(), 2, "{id}: the 503 was asked again once");
        assert!(requests.iter().all(|request| request.method == HttpMethod::Get && request.url == format!("http://hub.local{}", canonical_checkpoint_pair_path(&scope)) && request.bearer.as_deref() == Some(capability.as_str())), "{id}");
        assert_eq!(*transport.accepts.lock().unwrap(), vec![CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE_V1.to_string(); 2], "{id}");
    }
    let genesis = &fixture["pairs"][0];
    let scope = DocumentScope::new(genesis["selection"]["spaceId"].as_str().unwrap(), genesis["selection"]["documentId"].as_str().unwrap());
    let expected: DocumentOpenCheckpointV1 = semio_framework_pack_json::from_json_str(&fixture["admissions"][0]["expected"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    for (status, refusal) in [(401, "unauthorized"), (404, "http 404"), (503, "http 503")] {
        let transport = FakeTransport::default();
        for _ in 0..CANONICAL_CHECKPOINT_PAIR_TRANSIENT_ATTEMPTS {
            transport.push_response(Ok(HttpResponse { status, body: Vec::new() })).await;
        }
        let answer = authenticated_client(transport.clone(), &capability).document_canonical_checkpoint_pair(&root_ctx(), &scope, &expected).await;
        let named = match &answer {
            Err(DirectoryClientError::Unauthorized) => "unauthorized".to_string(),
            Err(DirectoryClientError::Http { status, .. }) => format!("http {status}"),
            other => format!("{other:?}"),
        };
        assert_eq!(named, refusal);
        assert_eq!(transport.requests.lock().unwrap().len(), if status == 503 { CANONICAL_CHECKPOINT_PAIR_TRANSIENT_ATTEMPTS as usize } else { 1 }, "{status}");
    }
}

/// 🛟️ A rebootstrap fetch reads the same route and admits the pair only as the control's checkpoint.
#[semio_framework_async_macros::async_test]
async fn a_rebootstrap_pair_is_admitted_only_as_the_controls_checkpoint() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../🧪️tests/🧫️fixtures/📇️directory/🪢️canonical-checkpoint-pair-v1.json")).expect("canonical pair fixture");
    let capability = format!("session.v1.{}.{}", "a".repeat(32), "b".repeat(64));
    for case in fixture["rebootstrapAdmissions"].as_array().unwrap() {
        let pair = fixture["pairs"].as_array().unwrap().iter().find(|pair| pair["id"] == case["pair"]).unwrap();
        let text = pair["bodyHex"].as_str().unwrap();
        let body: Vec<u8> = (0..text.len()).step_by(2).map(|index| u8::from_str_radix(&text[index..index + 2], 16).unwrap()).collect();
        let raw = &case["control"];
        let hash = |value: &serde_json::Value| crate::io::binary::artifact_hash::parse_artifact_hash_hex(value.as_str().unwrap()).unwrap();
        let control = crate::RebootstrapRequired {
            scope: DocumentScope::new(raw["scope"]["spaceId"].as_str().unwrap(), raw["scope"]["documentId"].as_str().unwrap()),
            checkpoint_id: hash(&raw["checkpointId"]),
            descriptor_digest_v1: hash(&raw["descriptorDigestV1"]),
            baseline_frontier: semio_framework_pack_json::from_json_str(&raw["baselineFrontier"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("fixture baseline"),
        };
        let transport = FakeTransport::default();
        transport.push_response(Ok(HttpResponse { status: 200, body })).await;
        let answer = authenticated_client(transport.clone(), &capability).rebootstrap_canonical_checkpoint_pair(&root_ctx(), &control).await;
        match case["refusal"].as_str() {
            None => assert!(answer.is_ok(), "{}: {answer:?}", case["id"]),
            Some(code) => assert!(matches!(&answer, Err(DirectoryClientError::Decode(detail)) if detail == code), "{}: {answer:?}", case["id"]),
        }
        assert_eq!(transport.requests.lock().unwrap()[0].url, format!("http://hub.local{}", canonical_checkpoint_pair_path(&control.scope)));
    }
}

/// 🤖️ The Rust twin answers the shared delegation-list vectors (`🤖️delegations/🧫️fixtures/📋️agent-delegation-list.json`) the
/// TypeScript parser + ShellHost filter answer: a malformed row is dropped, never the listing; revoked or expired rows name no agent.
#[test]
fn live_agent_principals_answer_the_shared_delegation_list_vectors() {
    let vectors = semio_framework_pack_json::parse(fixture_text!("../../../../../🤖️delegations/🧪️tests/🧫️fixtures/📋️agent-delegation-list.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the delegation vectors parse");
    let cases = vectors.get("cases").and_then(semio_framework_pack_json::Value::as_array).expect("cases");
    assert!(!cases.is_empty());
    for case in cases {
        let name = case.get("name").and_then(semio_framework_pack_json::Value::as_str).expect("name");
        let body = match case.get("body") {
            Some(body) => semio_framework_pack_json::to_string(body),
            None => case.get("bodyText").and_then(semio_framework_pack_json::Value::as_str).expect("bodyText").to_string(),
        };
        let now_ms = case.get("nowMs").and_then(semio_framework_pack_json::Value::as_i64).expect("nowMs");
        let expected: Vec<String> = case.get("principals").and_then(semio_framework_pack_json::Value::as_array).expect("principals").iter().map(|principal| principal.as_str().expect("principal").to_string()).collect();
        assert_eq!(agent_delegations::live_agent_principals(&body, now_ms), expected, "{name}");
    }
}
