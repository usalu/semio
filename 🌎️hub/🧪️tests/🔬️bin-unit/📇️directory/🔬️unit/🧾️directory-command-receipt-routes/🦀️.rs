/// 🧾️ Directory command receipt routes: the idempotent receipt route, the cross-user denials, the byte bounds and the lost reply store.

#[tokio::test]
async fn directory_command_receipt_v1_route_is_request_idempotent_for_concurrent_identical_ids() {
    let state = test_state().await;
    let author = issue_test_session(&state, "command-receipt-author@example.com").await;
    let space_id = create_space_for_test(&state, &author.user_id, "Receipt Space", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    let addr = spawn_server(state.clone()).await;
    let mut live = state.directory_service.subscribe();
    let command = DirectoryCommand::CreateInvite { space_id: space_id.clone(), role: DirectorySpaceRole::Spectator, ttl_secs: 3_600 };
    let request_id = "1f2e3d4c5b6a7988a1b2c3d4e5f60718";
    let (first, second) = tokio::join!(post_directory_command_for_test(addr, &author.token, request_id, command.clone()), post_directory_command_for_test(addr, &author.token, request_id, command.clone()),);
    assert_eq!((first.status, second.status), (202, 202), "an idempotent duplicate is answered, never failed");
    let receipts = [parse_directory_command_receipt_for_test(&first, request_id, &command), parse_directory_command_receipt_for_test(&second, request_id, &command)];
    let tokens: Vec<String> = receipts
        .iter()
        .filter_map(|receipt| match &receipt.result {
            os_directory::DirectoryCommandResultV1::Invite { invite_token } => Some(invite_token.clone()),
            os_directory::DirectoryCommandResultV1::None => None,
        })
        .collect();
    assert_eq!(tokens.len(), 1, "exactly one response carries the one-shot capability");
    assert_eq!(receipts.iter().filter(|receipt| receipt.outcome == DirectoryCommandOutcomeV1::Accepted).count(), 1);
    assert!(receipts.iter().any(|receipt| receipt.outcome == DirectoryCommandOutcomeV1::SecretUndeliverable && receipt.result == os_directory::DirectoryCommandResultV1::None));
    let token = tokens.into_iter().next().expect("issued capability");

    let invites = state.directory.list_invites(&space_id).await.expect("invite rows");
    assert_eq!(invites.len(), 1, "two concurrent identical request ids mint exactly one invitation");
    assert!(!format!("{invites:?}").contains(&token), "no invite row retains the capability plaintext");

    let retry = post_directory_command_for_test(addr, &author.token, request_id, command.clone()).await;
    let replayed = parse_directory_command_receipt_for_test(&retry, request_id, &command);
    assert_eq!(replayed.outcome, DirectoryCommandOutcomeV1::SecretUndeliverable);
    assert!(!std::str::from_utf8(&retry.body).expect("retry UTF-8").contains(&token), "a later resolution of the same id is redacted");
    assert_eq!(state.directory.list_invites(&space_id).await.expect("invite rows after retry").len(), 1);

    let events = state.directory.events_since(0, 1_000).await.expect("durable log");
    assert!(!format!("{events:?}").contains(&token), "the capability never enters the durable event log");
    let mut broadcast = Vec::new();
    while let Ok(message) = live.try_recv() {
        broadcast.push(format!("{message:?}"));
    }
    assert!(!broadcast.join("\n").contains(&token), "the capability never enters the live broadcast");
}

#[tokio::test]
async fn directory_command_receipt_v1_route_denies_cross_user_spectator_and_digest_substitution() {
    let state = test_state().await;
    let author = issue_test_session(&state, "command-receipt-owner@example.com").await;
    let spectator = issue_test_session(&state, "command-receipt-spectator@example.com").await;
    let space_id = create_space_for_test(&state, &author.user_id, "Denial Space", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    upsert_member_for_test(&state, &space_id, "command-receipt-spectator@example.com", DirectorySpaceRole::Spectator).await;
    let addr = spawn_server(state.clone()).await;
    let invite = DirectoryCommand::CreateInvite { space_id: space_id.clone(), role: DirectorySpaceRole::Spectator, ttl_secs: 3_600 };
    let request_id = "0a1b2c3d4e5f60718293a4b5c6d7e8f9";

    let accepted = post_directory_command_for_test(addr, &author.token, request_id, invite.clone()).await;
    assert_eq!(accepted.status, 202);
    let token = match parse_directory_command_receipt_for_test(&accepted, request_id, &invite).result {
        DirectoryCommandResultV1::Invite { invite_token } => invite_token,
        DirectoryCommandResultV1::None => panic!("the first accepted create-invite carries its capability"),
    };

    let denied = post_directory_command_for_test(addr, &spectator.token, request_id, invite.clone()).await;
    assert_eq!(denied.status, 403, "a spectator is denied before any stored completion is consulted");
    assert!(denied.body.is_empty(), "a denial carries no body, so no receipt or capability leaks");

    let cross_user =
        post_directory_command_for_test(addr, &spectator.token, request_id, DirectoryCommand::CreateSpace { name: "Spectator Space".into(), space_kind: os_directory::DirectorySpaceKind::Studio, visibility: DirectorySpaceVisibility::Private }).await;
    assert_eq!(cross_user.status, 202, "the idempotency key is scoped to the authenticated user");
    assert!(!std::str::from_utf8(&cross_user.body).expect("cross-user UTF-8").contains(&token), "another user's equal request id never discovers a stored capability");

    let substituted = post_directory_command_for_test(addr, &author.token, request_id, DirectoryCommand::RenameSpace { space_id: space_id.clone(), name: "Substituted".into() }).await;
    assert_eq!(substituted.status, 409, "an equal key with an unequal command digest is a generic conflict");
    assert!(substituted.body.is_empty());
    assert_eq!(state.directory.list_invites(&space_id).await.expect("invite rows").len(), 1, "no denial or conflict executed a second time");

    let capability = SessionCapability::parse(&author.token).expect("author capability");
    let record = state.directory.authenticate_session(&capability).await.expect("session lookup").expect("active session");
    state.directory.revoke_auth_session(&record.id, "command-receipt-test", None, "command-receipt-test").await.expect("revoke session").expect("revoked row");
    let revoked = post_directory_command_for_test(addr, &author.token, request_id, invite.clone()).await;
    assert_eq!(revoked.status, 401, "authentication re-runs before any stored completion is returned");
    assert!(revoked.body.is_empty());
}

#[tokio::test]
async fn directory_command_receipt_v1_route_bounds_request_and_receipt_bytes() {
    let state = test_state().await;
    let author = issue_test_session(&state, "command-receipt-bounds@example.com").await;
    let space_id = create_space_for_test(&state, &author.user_id, "Bounds Space", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    let addr = spawn_server(state.clone()).await;
    let authorization = format!("Bearer {}", author.token);

    let fitting = DirectoryCommand::RenameSpace { space_id: space_id.clone(), name: "n".repeat(64) };
    let request = DirectoryCommandRequestV1::new("9f8e7d6c5b4a39281706f5e4d3c2b1a0", fitting.clone());
    assert!(request.canonical_json().len() <= DIRECTORY_COMMAND_REQUEST_MAX_BYTES);
    let accepted = raw_http_request(addr, "POST", "/directory/commands", &[("Authorization", &authorization), ("Content-Type", "application/json")], request.canonical_json().as_bytes()).await;
    assert_eq!(accepted.status, 202);
    assert!(accepted.body.len() <= os_directory::DIRECTORY_COMMAND_RECEIPT_MAX_BYTES, "an admitted request can never produce an over-ceiling receipt");

    let padding = DIRECTORY_COMMAND_REQUEST_MAX_BYTES + 1 - DirectoryCommandRequestV1::new("9f8e7d6c5b4a39281706f5e4d3c2b1a1", DirectoryCommand::RenameSpace { space_id: space_id.clone(), name: String::new() }).canonical_json().len();
    let oversize = DirectoryCommandRequestV1::new("9f8e7d6c5b4a39281706f5e4d3c2b1a1", DirectoryCommand::RenameSpace { space_id: space_id.clone(), name: "x".repeat(padding) });
    assert_eq!(oversize.canonical_json().len(), DIRECTORY_COMMAND_REQUEST_MAX_BYTES + 1);
    let rejected = raw_http_request(addr, "POST", "/directory/commands", &[("Authorization", &authorization), ("Content-Type", "application/json")], oversize.canonical_json().as_bytes()).await;
    assert!(matches!(rejected.status, 400 | 413), "one byte past the request ceiling is refused, got {}", rejected.status);

    for hostile in [
        "{\"requestId\":\"9f8e7d6c5b4a39281706f5e4d3c2b1a2\",\"schema\":\"semio.directory.command-request.v1\",\"command\":{\"kind\":\"archive-space\",\"spaceId\":\"x\"}}",
        "{\"schema\":\"semio.directory.command-request.v1\",\"requestId\":\"9F8E7D6C5B4A39281706F5E4D3C2B1A2\",\"command\":{\"kind\":\"archive-space\",\"spaceId\":\"x\"}}",
        "{\"schema\":\"semio.directory.command-request.v1\",\"requestId\":\"00000000000000000000000000000000\",\"command\":{\"kind\":\"archive-space\",\"spaceId\":\"x\"}}",
        "{\"schema\":\"semio.directory.command-request.v1\",\"requestId\":\"9f8e7d6c5b4a39281706f5e4d3c2b1a2\",\"command\":{\"kind\":\"archive-space\",\"spaceId\":\"x\"},\"extra\":1}",
        "{\"kind\":\"archive-space\",\"spaceId\":\"x\"}",
    ] {
        let response = raw_http_request(addr, "POST", "/directory/commands", &[("Authorization", &authorization), ("Content-Type", "application/json")], hostile.as_bytes()).await;
        assert_eq!(response.status, 400, "a noncanonical, mis-cased, zero-id, unknown-field, or bare-command body is refused: {hostile}");
        assert!(response.body.is_empty());
    }
}

#[tokio::test]
async fn directory_command_receipt_v1_store_resolves_a_lost_reply_and_survives_restart() {
    let path = tempdir("command-receipt-store");
    std::fs::create_dir_all(&path).expect("receipt store dir");
    let database = path.join("directory.sqlite");
    let database_path = database.to_str().expect("receipt store path").to_string();
    let claim = |request_id: &str, digest: &str, kind: DirectoryCommandResultKindV1, actor: &str| NewDirectoryCommandReceipt {
        actor_user_id: actor.to_string(),
        request_id: request_id.to_string(),
        command_sha256: digest.to_string(),
        result_kind: kind,
        claimed_at: 1_700_000_000_000,
    };
    let invite_digest = directory_command_sha256(&DirectoryCommand::CreateInvite { space_id: "space".into(), role: DirectorySpaceRole::Spectator, ttl_secs: 3_600 });
    let rename_digest = directory_command_sha256(&DirectoryCommand::RenameSpace { space_id: "space".into(), name: "Renamed".into() });
    let request_id = "1f2e3d4c5b6a7988a1b2c3d4e5f60718";

    let directory = SqliteDirectory::connect(&database_path).await.expect("connect receipt store");
    directory.seed().await.expect("seed receipt store");
    let pending_claim = claim(request_id, &invite_digest, DirectoryCommandResultKindV1::Invite, "user-a");
    assert!(matches!(directory.claim_or_read_directory_command_receipt(&pending_claim).await.expect("first claim"), DirectoryCommandClaimV1::Claimed(_)));
    let lost = match directory.claim_or_read_directory_command_receipt(&pending_claim).await.expect("lost-reply resolution") {
        DirectoryCommandClaimV1::Existing(record) => record,
        other => panic!("a claimed key never re-executes: {other:?}"),
    };
    assert_eq!(lost.disposition, DirectoryCommandDispositionV1::Pending);
    assert_eq!(replay_directory_command_receipt(&lost).outcome, DirectoryCommandOutcomeV1::SecretUndeliverable, "a reply lost between durable command and response is honestly undeliverable");
    assert!(matches!(directory.claim_or_read_directory_command_receipt(&claim(request_id, &rename_digest, DirectoryCommandResultKindV1::None, "user-a")).await.expect("digest substitution"), DirectoryCommandClaimV1::Conflict));
    assert!(
        matches!(directory.claim_or_read_directory_command_receipt(&claim(request_id, &invite_digest, DirectoryCommandResultKindV1::Invite, "user-b")).await.expect("cross-user claim"), DirectoryCommandClaimV1::Claimed(_)),
        "the key is scoped to the authenticated user"
    );

    let replay_digest = replay_directory_command_receipt(&DirectoryCommandReceiptRecord { disposition: DirectoryCommandDispositionV1::Completed, ..lost.clone() }).receipt_sha256;
    let completed = directory
        .complete_directory_command_receipt(&DirectoryCommandReceiptCompletion {
            actor_user_id: "user-a".into(),
            request_id: request_id.into(),
            event_seq_first: None,
            event_seq_last: None,
            receipt_sha256: replay_digest.clone(),
            completed_at: 1_700_000_000_001,
        })
        .await
        .expect("durable completion");
    assert_eq!(completed.disposition, DirectoryCommandDispositionV1::Completed);
    assert!(
        directory
            .complete_directory_command_receipt(&DirectoryCommandReceiptCompletion {
                actor_user_id: "user-a".into(),
                request_id: request_id.into(),
                event_seq_first: None,
                event_seq_last: None,
                receipt_sha256: replay_digest.clone(),
                completed_at: 1_700_000_000_002
            })
            .await
            .is_err(),
        "one claim completes exactly once"
    );
    drop(directory);

    let restarted = SqliteDirectory::connect(&database_path).await.expect("reconnect receipt store");
    restarted.seed().await.expect("reseed receipt store");
    let resolved = match restarted.claim_or_read_directory_command_receipt(&pending_claim).await.expect("restart resolution") {
        DirectoryCommandClaimV1::Existing(record) => record,
        other => panic!("a completed key survives restart: {other:?}"),
    };
    assert_eq!(resolved.disposition, DirectoryCommandDispositionV1::Completed);
    assert_eq!(resolved.receipt_sha256.as_deref(), Some(replay_digest.as_str()), "the durable row carries the canonical redacted receipt digest");
    let replayed = replay_directory_command_receipt(&resolved);
    assert_eq!(replayed.outcome, DirectoryCommandOutcomeV1::SecretUndeliverable);
    assert_eq!(replayed.receipt_sha256, replay_digest);
    assert!(replayed.validate().is_ok() && replayed.events.is_empty() && replayed.result == DirectoryCommandResultV1::None);

    let plain = NewDirectoryCommandReceipt { result_kind: DirectoryCommandResultKindV1::None, command_sha256: rename_digest.clone(), request_id: "9f8e7d6c5b4a39281706f5e4d3c2b1a0".into(), ..pending_claim.clone() };
    assert!(matches!(restarted.claim_or_read_directory_command_receipt(&plain).await.expect("plain claim"), DirectoryCommandClaimV1::Claimed(_)));
    restarted
        .complete_directory_command_receipt(&DirectoryCommandReceiptCompletion {
            actor_user_id: plain.actor_user_id.clone(),
            request_id: plain.request_id.clone(),
            event_seq_first: Some(7),
            event_seq_last: Some(8),
            receipt_sha256: "0".repeat(64),
            completed_at: 1_700_000_000_003,
        })
        .await
        .expect("plain completion");
    let plain_record = match restarted.claim_or_read_directory_command_receipt(&plain).await.expect("plain resolution") {
        DirectoryCommandClaimV1::Existing(record) => record,
        other => panic!("a completed plain key replays: {other:?}"),
    };
    assert_eq!((plain_record.event_seq_first, plain_record.event_seq_last), (Some(7), Some(8)));
    assert_eq!(replay_directory_command_receipt(&plain_record).outcome, DirectoryCommandOutcomeV1::PreviouslyAccepted, "a completed secret-free command resolves as previously accepted");
    drop(restarted);
    let _ = std::fs::remove_dir_all(&path);
}
