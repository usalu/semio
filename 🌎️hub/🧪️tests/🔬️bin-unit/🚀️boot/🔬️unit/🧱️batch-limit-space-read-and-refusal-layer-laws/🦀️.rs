/// 🧱️ Batch limit, space read and refusal layer laws of the hub boot: the permanent batch refusal, the member space list and event pages, the typed panic refusal layer and the liveness of a not ready process.

/// 📏️ C12 P1 live (a 600-envelope outbox answered `hub.unavailable` forever): a `Commands` batch the document backbone
/// declares legal passes the socket's declaration check, one envelope more is refused PERMANENTLY with the one
/// `HubBatchLimitRefusalMessageV1` (`hub.batch-limit`, level error), and an engine `LimitExceeded` maps to the same permanent
/// message — no size refusal ever carries the transient `hub.unavailable` a client would resend without end.
#[test]
fn a_batch_beyond_its_declaration_is_refused_permanently_and_never_as_transient() {
    run_socket_test(|| async {
        let document = WireArtifactId("p1-declared-batch".to_string());
        let template = sample_envelope("declared", &document).await;
        let mut declared = Vec::new();
        let mut bytes = 0usize;
        let next = loop {
            let next = MutationEnvelope { mutation_id: protocol::MutationId(format!("declared-{:x}", declared.len())), ..template.clone() };
            let mut encoded = Vec::new();
            protocol::encode_envelope(&next, &mut encoded);
            let count_bytes = if declared.len() + 1 < 128 { 1 } else if declared.len() + 1 < 16_384 { 2 } else { 3 };
            if declared.len() == protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES || count_bytes + bytes + encoded.len() > protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_BYTES {
                break next;
            }
            bytes += encoded.len();
            declared.push(next);
        };
        let mut over = declared.clone();
        over.push(next);
        assert!(declared.len() > 256, "the declared batch ({} envelopes) exceeds the engine's former 256-envelope credit", declared.len());
        assert!(undeclared_batch_refusal(&declared).is_none(), "a declared batch passes the socket's declaration check");
        let Some(ApplyOutcome::Rejected { reason, messages }) = undeclared_batch_refusal(&over) else { panic!("one envelope beyond the declaration is refused") };
        let messages: serde_json::Value = serde_json::from_slice(&messages).expect("messages are JSON");
        assert_eq!(messages[0]["code"], semio_hub_refusal::HUB_BATCH_LIMIT_REFUSAL_CODE, "{reason}");
        assert_eq!(messages[0]["level"], "error");
        assert_eq!(messages.as_array().unwrap().len(), 1);
        let engine: serde_json::Value = serde_json::from_slice(&messages_for_error(&db::DbError::LimitExceeded("artifact submit batch item credit"))).expect("messages are JSON");
        assert_eq!(engine[0]["code"], semio_hub_refusal::HUB_BATCH_LIMIT_REFUSAL_CODE, "an engine size refusal is permanent");
        assert_ne!(engine[0]["code"], semio_hub_refusal::HUB_TRANSIENT_APPLY_REFUSAL_CODE);
    });
}

/// 🏘️ A member of many spaces reads its space list and its event pages in bounded time, exactly: the list is one
/// directory query and each event page one membership read (WG8 on 7800: 25–58 s for 82 spaces, the list folded the
/// whole event log and mounted every document). The oracle is the event-log fold (`os_directory::fold_all`): the same
/// visible spaces, discriminators, roles, member and document counts; each answer within the declared page budget.
#[test]
fn a_member_of_many_spaces_reads_its_space_list_and_event_pages_within_the_page_budget() {
    run_socket_test(|| async {
        const SPACES: usize = 120;
        const PAGE_BUDGET: std::time::Duration = std::time::Duration::from_millis(200);
        let state = test_state().await;
        let member = issue_test_session(&state, "many-spaces@example.com").await;
        let stranger = issue_test_session(&state, "many-spaces-stranger@example.com").await;
        for index in 0..SPACES {
            let owner = if index % 3 == 0 { &stranger } else { &member };
            let kind = if index % 2 == 0 { os_directory::DirectorySpaceKind::Studio } else { os_directory::DirectorySpaceKind::Atelier };
            let visibility = if index % 4 == 0 { DirectorySpaceVisibility::Public } else { DirectorySpaceVisibility::Private };
            let space_id = create_space_for_test(&state, &owner.user_id, &format!("Space {index:03}"), kind, visibility).await;
            if index % 6 == 0 {
                upsert_member_for_test(&state, &space_id, "many-spaces@example.com", DirectorySpaceRole::Spectator).await;
            }
            if index % 5 == 0 {
                announce_document_for_test(&state, &space_id, &format!("many-spaces-doc-{index}")).await;
            }
        }
        let events = load_all_directory_events(state.directory.as_ref(), 0).await.expect("directory events");
        let model = os_directory::fold_all(os_directory::DirectoryReadModel::default(), &events).await;
        let mut expected = BTreeMap::new();
        for space in model.spaces.values() {
            let role = space.members.iter().find(|row| row.user_id == member.user_id).map(|row| row.role);
            let access = match directory_space_access_decision(space.view.visibility == DirectorySpaceVisibility::Public, role) {
                DirectorySpaceAccessDecisionV1::Hidden => continue,
                DirectorySpaceAccessDecisionV1::Public => "public",
                DirectorySpaceAccessDecisionV1::Member => "member",
                DirectorySpaceAccessDecisionV1::Author => "author",
            };
            expected.insert(space.view.id.clone(), (access, space.view.name.clone(), u64::from(space.view.member_count), space.documents.len() as u64));
        }
        assert!(expected.len() > 80, "the member sees most of the {SPACES} spaces: {}", expected.len());
        let addr = spawn_server(state.clone()).await;
        let bearer = format!("Bearer {}", member.token);
        let mut fastest = std::time::Duration::MAX;
        let mut listed = serde_json::Value::Null;
        for _ in 0..3 {
            let started = std::time::Instant::now();
            let response = raw_http_get(addr, "/directory/spaces", &[("authorization", bearer.as_str())]).await;
            fastest = fastest.min(started.elapsed());
            assert_eq!(response.status, 200);
            listed = json_body(&response);
        }
        let mut observed = BTreeMap::new();
        for entry in listed.as_array().expect("space list") {
            let space = &entry["space"];
            observed.insert(space["id"].as_str().expect("id").to_string(), (entry["access"].as_str().expect("access"), space["name"].as_str().expect("name").to_string(), space["memberCount"].as_u64().expect("members"), space["documentCount"].as_u64().expect("documents")));
        }
        assert_eq!(observed, expected, "the space list equals the event-log fold");
        assert!(fastest < PAGE_BUDGET, "the list of {} spaces answered in {fastest:?}", expected.len());
        let mut after = 0u64;
        let mut pages = 0usize;
        loop {
            let started = std::time::Instant::now();
            let response = raw_http_get(addr, &format!("/directory/event-page/v1?after={after}"), &[("authorization", bearer.as_str())]).await;
            let elapsed = started.elapsed();
            assert_eq!(response.status, 200, "{}", String::from_utf8_lossy(&response.body));
            let page = json_body(&response);
            assert!(elapsed < PAGE_BUDGET * 2, "event page {pages} answered in {elapsed:?}");
            pages += 1;
            let through = page["throughSeqInclusive"].as_u64().expect("through");
            if !page["hasMore"].as_bool().expect("has more") || through == after {
                break;
            }
            after = through;
        }
        assert!(pages >= 2, "the event log spans several pages: {pages}");
    });
}

/// 🧯️ The refusal layer answers a panicking handler with a typed `500` instead of a dropped
/// connection, and it never detaches a handler from its request: a client that disconnects drops
/// the handler it was waiting on, exactly as without the layer.
#[tokio::test]
async fn the_refusal_layer_types_a_panic_and_keeps_every_handler_request_owned() {
    struct DropSignal(Option<tokio::sync::oneshot::Sender<()>>);
    impl Drop for DropSignal {
        fn drop(&mut self) {
            if let Some(signal) = self.0.take() {
                let _ = signal.send(());
            }
        }
    }
    async fn panicking() -> StatusCode {
        std::panic::panic_any("hostile handler")
    }
    let (started_tx, started_rx) = tokio::sync::oneshot::channel::<()>();
    let (dropped_tx, dropped_rx) = tokio::sync::oneshot::channel::<()>();
    let pending = Arc::new(Mutex::new(Some((started_tx, dropped_tx))));
    let router = axum::Router::new()
        .route("/panic", axum::routing::get(panicking))
        .route(
            "/pending",
            axum::routing::get(move || {
                let signals = pending.lock().unwrap().take();
                async move {
                    let Some((started, dropped)) = signals else { return StatusCode::CONFLICT };
                    let _dropped = DropSignal(Some(dropped));
                    let _ = started.send(());
                    std::future::pending::<StatusCode>().await
                }
            }),
        )
        .layer(axum::middleware::from_fn_with_state(Tracer::disabled(), refusal_middleware));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server = tokio::spawn(async move { axum::serve(listener, router).await });
    let panicked = raw_http_get(addr, "/panic", &[]).await;
    assert_eq!(panicked.status, 500);
    assert!(panicked.headers.to_ascii_lowercase().contains(&format!("{}: {}", semio_hub_refusal::HUB_REFUSAL_HEADER, semio_hub_refusal::refusal_code(500).unwrap())), "{}", panicked.headers);
    let mut client = tokio::net::TcpStream::connect(addr).await.unwrap();
    tokio::io::AsyncWriteExt::write_all(&mut client, format!("GET /pending HTTP/1.1\r\nHost: {addr}\r\n\r\n").as_bytes()).await.unwrap();
    tokio::time::timeout(std::time::Duration::from_secs(5), started_rx).await.expect("handler start deadline").expect("handler started");
    drop(client);
    tokio::time::timeout(std::time::Duration::from_secs(5), dropped_rx).await.expect("a disconnected request drops its handler").expect("handler dropped");
    assert_eq!(raw_http_get(addr, "/panic", &[]).await.status, 500, "a panic never takes the server down");
    server.abort();
}

/// 🫀️ Liveness and readiness are two different questions, and a hub that has not finished booting
/// must answer them differently: `/readyz` refuses with `503` while a required subsystem is down,
/// and `/healthz` still answers `200 live` on the very same process so an orchestrator restarts a
/// wedged hub without ever restarting a warming one.
#[tokio::test]
async fn healthz_reports_liveness_on_a_process_whose_readyz_still_refuses() {
    let state = test_state().await;
    let run_id = state.readiness.run_id.clone();
    let addr = spawn_server(state).await;

    let readiness = raw_http_get(addr, "/readyz", &[]).await;
    assert_eq!(readiness.status, 503);

    let liveness = raw_http_get(addr, "/healthz", &[]).await;
    assert_eq!(liveness.status, 200, "{}", String::from_utf8_lossy(&liveness.body));
    let body: serde_json::Value = serde_json::from_slice(&liveness.body).expect("liveness JSON");
    assert_eq!(body["schema"], "semio.hub.liveness/v1");
    assert_eq!(body["status"], "live");
    assert_eq!(body["runId"], run_id);
    assert!(body["uptimeMs"].is_u64());
    assert!(body.get("directory").is_none(), "liveness never republishes a readiness subsystem");
    assert!(body.get("artifactAuthority").is_none(), "liveness never republishes a readiness subsystem");

    let again = raw_http_get(addr, "/healthz", &[]).await;
    let repeated: serde_json::Value = serde_json::from_slice(&again.body).expect("second liveness JSON");
    assert!(repeated["uptimeMs"].as_u64().expect("uptime") >= body["uptimeMs"].as_u64().expect("uptime"));
}
