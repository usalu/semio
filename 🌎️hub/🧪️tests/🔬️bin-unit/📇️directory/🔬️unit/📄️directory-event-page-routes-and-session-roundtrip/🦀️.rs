/// 📄️ Directory event page routes and session roundtrip: the raw hole scan, the session generation revalidation, the canonical byte prefix, the transactional append admission, the noncanonical query rejection and the auth session roundtrip.

#[tokio::test]
async fn directory_event_page_v1_route_scans_raw_holes_bounds_canonical_receipt_and_visibility() {
    let state = test_state().await;
    let caller = issue_test_session(&state, "event-page-member@example.com").await;
    upsert_member_for_test(&state, STUDIO, "event-page-member@example.com", DirectorySpaceRole::Spectator).await;
    let outsider = issue_test_session(&state, "event-page-owner@example.com").await;
    let hidden_space = create_space_for_test(&state, &outsider.user_id, "Hidden", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    let after = state.directory.head_seq().await.expect("event-page start");
    let appended = append_directory_page_test_events(&state, &[(hidden_space.clone(), "hidden-11".into()), (STUDIO.into(), "visible-12".into()), (hidden_space.clone(), "hidden-13".into()), (STUDIO.into(), "visible-14".into())]).await;
    let addr = spawn_server(state.clone()).await;
    let authorization = event_page_authorization(&caller.token);
    let response = raw_http_get(addr, &format!("/directory/event-page/v1?after={after}"), &[("Authorization", &authorization)]).await;
    assert_eq!(response.status, 200);
    assert!(response.headers.to_ascii_lowercase().contains("content-type: application/json"));
    let canonical = std::str::from_utf8(&response.body).expect("event-page UTF-8");
    let page = DirectoryEventPageV1::parse_canonical_json(canonical).expect("canonical event page");
    assert_eq!(page.after_seq_exclusive, after);
    assert_eq!(page.through_seq_inclusive, appended[3].seq);
    assert_eq!(page.events.iter().map(|event| event.seq).collect::<Vec<_>>(), vec![appended[1].seq, appended[3].seq]);
    assert!(!canonical.contains(&hidden_space));
    assert!(!canonical.contains("hidden-11"));
    assert!(!canonical.contains("hidden-13"));

    let hidden_after = state.directory.head_seq().await.expect("hidden scan start");
    let hidden = (0..DIRECTORY_EVENT_PAGE_MAX_RAW_ROWS).map(|index| (hidden_space.clone(), format!("hidden-saturated-{index}"))).collect::<Vec<_>>();
    let hidden_events = append_directory_page_test_events(&state, &hidden).await;
    let response = raw_http_get(addr, &format!("/directory/event-page/v1?after={hidden_after}"), &[("Authorization", &authorization)]).await;
    let page = DirectoryEventPageV1::parse_canonical_json(std::str::from_utf8(&response.body).expect("hidden page UTF-8")).expect("hidden page");
    assert!(page.events.is_empty());
    assert_eq!(page.through_seq_inclusive, hidden_events.last().expect("hidden tail").seq);
    assert!(page.has_more, "a saturated raw scan advertises the bounded follow-up even when every row is hidden");
}

#[tokio::test]
async fn directory_event_page_v1_route_revalidates_session_generation_after_read_before_response() {
    let mut state = test_state().await;
    let session = issue_test_session(&state, "event-page-revoked@example.com").await;
    upsert_member_for_test(&state, STUDIO, "event-page-revoked@example.com", DirectorySpaceRole::Spectator).await;
    append_directory_page_test_events(&state, &[(STUDIO.into(), "before-revoke".into())]).await;
    let capability = SessionCapability::parse(&session.token).expect("event-page capability");
    let record = state.directory.authenticate_session(&capability).await.expect("session lookup").expect("active session");
    let gate = Arc::new(TestLiveGate::default());
    gate.directory_event_page_fence_enabled.store(true, std::sync::atomic::Ordering::Release);
    state.live_gate = Some(gate.clone());
    let addr = spawn_server(state.clone()).await;
    let authorization = event_page_authorization(&session.token);
    let request = tokio::spawn(async move { raw_http_get(addr, "/directory/event-page/v1?after=0", &[("Authorization", &authorization)]).await });
    let admitted = tokio::time::timeout(std::time::Duration::from_secs(2), gate.directory_event_page_read_admitted.acquire()).await.expect("event-page read fence deadline").expect("event-page read fence");
    admitted.forget();
    state.directory.revoke_auth_session(&record.id, "event-page-test", None, "event-page-test").await.expect("revoke session").expect("revoked row");
    gate.directory_event_page_read_release.add_permits(1);
    let response = tokio::time::timeout(std::time::Duration::from_secs(2), request).await.expect("revalidation response deadline").expect("revalidation request");
    assert_eq!(response.status, 401);
    assert!(response.body.is_empty());

    let session = issue_test_session(&state, "event-page-cancel@example.com").await;
    let authorization = event_page_authorization(&session.token);
    let cancelled = tokio::spawn(async move { raw_http_get(addr, "/directory/event-page/v1?after=0", &[("Authorization", &authorization)]).await });
    let admitted = tokio::time::timeout(std::time::Duration::from_secs(2), gate.directory_event_page_read_admitted.acquire()).await.expect("cancel read fence deadline").expect("cancel read fence");
    admitted.forget();
    let control = gate.directory_event_page_control.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone().expect("request-owned cancellation control");
    cancelled.abort();
    assert!(cancelled.await.expect_err("request task cancelled").is_cancelled());
    tokio::time::timeout(std::time::Duration::from_secs(2), async {
        while control.active.load(std::sync::atomic::Ordering::Acquire) {
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("server request cancellation deadline");
    assert!(control.cancelled.load(std::sync::atomic::Ordering::Acquire));
    gate.directory_event_page_read_release.add_permits(1);
}

#[tokio::test]
async fn directory_event_page_v1_route_stops_at_canonical_byte_prefix_without_skipping_visible_seq() {
    let state = test_state().await;
    let token = seed_author_token(&state).await;
    let after = state.directory.head_seq().await.expect("byte-prefix start");
    let appended = append_directory_page_test_events(&state, &[(STUDIO.into(), "a".repeat(32 * 1024)), (STUDIO.into(), "b".repeat(32 * 1024))]).await;
    let addr = spawn_server(state).await;
    let authorization = event_page_authorization(&token);
    let first = raw_http_get(addr, &format!("/directory/event-page/v1?after={after}"), &[("Authorization", &authorization)]).await;
    assert_eq!(first.status, 200);
    assert!(first.body.len() <= DIRECTORY_EVENT_PAGE_MAX_BYTES);
    let first = DirectoryEventPageV1::parse_canonical_json(std::str::from_utf8(&first.body).expect("first page UTF-8")).expect("first page");
    assert_eq!(first.events.iter().map(|event| event.seq).collect::<Vec<_>>(), vec![appended[0].seq]);
    assert_eq!(first.through_seq_inclusive, appended[0].seq);
    assert!(first.has_more);
    let second = raw_http_get(addr, &format!("/directory/event-page/v1?after={}", first.through_seq_inclusive), &[("Authorization", &authorization)]).await;
    let second = DirectoryEventPageV1::parse_canonical_json(std::str::from_utf8(&second.body).expect("second page UTF-8")).expect("second page");
    assert_eq!(second.events.iter().map(|event| event.seq).collect::<Vec<_>>(), vec![appended[1].seq]);
    assert_eq!(second.through_seq_inclusive, appended[1].seq);
}

#[tokio::test]
async fn directory_event_page_v1_append_admission_is_transactional_for_sqlite_postgres_and_neo4j() {
    let state = test_state().await;
    let mut exact = DirectoryEvent {
        seq: 1,
        id: "event-boundary".into(),
        hlc: os_directory::Hlc { physical_ms: 1, logical: 0 },
        actor: DirectoryActor { kind: DirectoryActorKind::System, id: "system:event-page-boundary".into() },
        space_id: Some(STUDIO.into()),
        user_id: None,
        body: os_directory::DirectoryEventBody::SpaceRenamed { space_id: STUDIO.into(), name: String::new() },
        recorded_at_ms: 1,
    };
    let base = semio_framework_pack_json::to_json_string(&exact).len();
    let os_directory::DirectoryEventBody::SpaceRenamed { name, .. } = &mut exact.body else { unreachable!() };
    *name = "x".repeat(os_directory::DIRECTORY_EVENT_PAGE_MAX_EVENT_BYTES - base);
    assert_eq!(semio_framework_pack_json::to_json_string(&exact).len(), os_directory::DIRECTORY_EVENT_PAGE_MAX_EVENT_BYTES);
    assert_eq!(validate_directory_event_page_event(&exact), Ok(()));
    if let os_directory::DirectoryEventBody::SpaceRenamed { name, .. } = &mut exact.body {
        name.push('x');
    }
    assert_eq!(validate_directory_event_page_event(&exact), Err(DirectoryEventPageErrorV1::Invalid));

    let head = state.directory.head_seq().await.expect("head before rejected append");
    let rejected = semio_hub_directory_contract::NewDirectoryEvent {
        hlc: os_directory::Hlc { physical_ms: 1, logical: 0 },
        actor: DirectoryActor { kind: DirectoryActorKind::System, id: "system:event-page-reject".into() },
        space_id: Some(STUDIO.into()),
        user_id: None,
        body: os_directory::DirectoryEventBody::SpaceRenamed { space_id: STUDIO.into(), name: "x".repeat(os_directory::DIRECTORY_EVENT_PAGE_MAX_EVENT_BYTES) },
    };
    assert!(matches!(state.directory.append_events(&[rejected]).await, Err(DirectoryError::Conflict(_))));
    assert_eq!(state.directory.head_seq().await.expect("head after rejected append"), head, "SQLite rolls back both row and dense sequence");
    let space = state.directory.get_space(STUDIO).await.expect("space projection").expect("seed space");
    assert_ne!(space.name.len(), os_directory::DIRECTORY_EVENT_PAGE_MAX_EVENT_BYTES, "rejected event never reaches projection");

    let postgres = semio_hub_testing::hub_module_tree("📇️directory/🐘️postgres/🦀️.rs");
    let neo4j = semio_hub_testing::hub_module_tree("📇️directory/🌐️neo4j/🦀️.rs");
    let postgres_writes = postgres.matches("INSERT INTO hub_directory_event(seq").count() + postgres.matches("INSERT INTO hub_directory_event (seq").count();
    let neo4j_writes = neo4j.matches("CREATE (e:DirectoryEvent {seq:").count();
    assert!(postgres_writes >= 3 && neo4j_writes >= 3, "both remote backends still persist directory events on every append seam");
    assert_eq!(postgres.matches("validate_directory_event_page_event(&").count(), postgres_writes, "all PostgreSQL full-event append seams admit before persistence");
    assert_eq!(neo4j.matches("validate_directory_event_page_event(&").count(), neo4j_writes, "all Neo4j full-event append seams admit before persistence");
}

#[tokio::test]
async fn directory_event_page_v1_route_rejects_noncanonical_query_and_stale_bearer_without_body() {
    let mut state = test_state().await;
    let token = seed_author_token(&state).await;
    let stale = issue_test_session(&state, "event-page-stale@example.com").await;
    let stale_capability = SessionCapability::parse(&stale.token).expect("stale capability");
    let stale_record = state.directory.authenticate_session(&stale_capability).await.expect("stale lookup").expect("stale session");
    state.directory.revoke_auth_session(&stale_record.id, "stale", None, "stale").await.expect("stale revoke").expect("stale row");
    let gate = Arc::new(TestLiveGate::default());
    gate.directory_event_page_fence_enabled.store(true, std::sync::atomic::Ordering::Release);
    state.live_gate = Some(gate.clone());
    let addr = spawn_server(state).await;
    let authorization = event_page_authorization(&token);
    for path in [
        "/directory/event-page/v1",
        "/directory/event-page/v1?",
        "/directory/event-page/v1?after=",
        "/directory/event-page/v1?after=00",
        "/directory/event-page/v1?after=1&after=2",
        "/directory/event-page/v1?since=0",
        "/directory/event-page/v1?after=%30",
        "/directory/event-page/v1?after=9007199254740992",
    ] {
        let response = raw_http_get(addr, path, &[("Authorization", &authorization)]).await;
        assert_eq!(response.status, 400, "query {path}");
        assert!(response.body.is_empty(), "query rejection is body-free");
    }
    let missing = raw_http_get(addr, "/directory/event-page/v1?after=0", &[]).await;
    assert_eq!(missing.status, 401);
    assert!(missing.body.is_empty());
    let stale_authorization = event_page_authorization(&stale.token);
    let stale = raw_http_get(addr, "/directory/event-page/v1?after=0", &[("Authorization", &stale_authorization)]).await;
    assert_eq!(stale.status, 401);
    assert!(stale.body.is_empty());
    assert_eq!(gate.directory_event_page_read_admitted.available_permits(), 0, "bad query and pre-read authentication failures perform no directory event scan");
}

// 🔬️ `GET /auth/sessions/me` + `POST /auth/sessions/me/sign-out`: a live session resolves the
// caller's identity; signing out makes the SAME token unauthorized on a subsequent call.
#[tokio::test]
async fn auth_sessions_me_roundtrip() {
    let state = test_state().await;
    let session = issue_test_session(&state, "me@example.com").await;
    let mut headers = HeaderMap::new();
    headers.insert(axum::http::header::AUTHORIZATION, format!("Bearer {}", session.token).parse().unwrap());

    let me = get_session_me(headers.clone(), State(state.clone())).await.expect("session me");
    assert_eq!(me.0.user_id, session.user_id);
    assert_eq!(me.0.email, "me@example.com");
    assert_eq!(me.0.schema, "semio.directory.session-authority.v1");
    assert_eq!(me.0.authorization_generation, 1);
    assert!(me.0.session_binding_sha256.len() == 64 && me.0.session_binding_sha256.bytes().any(|byte| byte != b'0'));
    assert_eq!(me.0.canonical_json().as_deref().and_then(os_directory::schema::session_authority::DirectorySessionAuthorityV1::parse_canonical_json), Some(me.0.clone()));

    assert_eq!(post_session_sign_out(headers.clone(), State(state.clone()), Bytes::new()).await, StatusCode::NO_CONTENT);
    assert_eq!(get_session_me(headers, State(state)).await.err(), Some(StatusCode::UNAUTHORIZED));
}
