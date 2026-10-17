/// 📨️ Laws of invite redemption and archive projections, uncertain commits, the exhaustive module and the document index.

/// 🎟️ Two independent service writers still produce one durable invitation claim across restart and rebuild.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn invite_redemption_sqlite_claim_is_exactly_once_across_concurrency_restart_and_rebuild() {
    let root = std::env::temp_dir().join(format!("semio-invite-claim-{}", time_ordered_id()));
    std::fs::create_dir(&root).expect("create invite test directory");
    let path = root.join("directory.sqlite3");
    let path_text = path.to_string_lossy().into_owned();
    let primary_backend = SqliteDirectory::connect(&path_text).await.expect("connect primary");
    primary_backend.seed().await.expect("seed");
    let primary = Arc::new(HubDirectories::from(primary_backend));
    let invited = primary.create_user("invite-race@example.com", "Invite Race", None, None, None).await.expect("create invited user");
    let issued = primary.issue_invite("default", SpaceRole::Spectator, 3600, "invite-race").await.expect("issue invite");
    let secondary = Arc::new(HubDirectories::from(SqliteDirectory::connect(&path_text).await.expect("connect secondary")));
    let services = [Arc::new(DirectoryService::new(primary.clone(), 16)), Arc::new(DirectoryService::new(secondary.clone(), 16))];
    let barrier = Arc::new(tokio::sync::Barrier::new(3));
    let mut claims = Vec::new();
    for service in services {
        let barrier = barrier.clone();
        let capability = issued.capability.clone();
        let user_id = invited.id.clone();
        claims.push(tokio::spawn(async move {
            barrier.wait().await;
            service.redeem_invite(user_actor(&user_id), &capability, &user_id).await
        }));
    }
    barrier.wait().await;
    let results = [claims.remove(0).await.expect("first claim task"), claims.remove(0).await.expect("second claim task")];
    assert_eq!(results.iter().filter(|result| result.is_ok()).count(), 2);
    let returned_ids: BTreeSet<_> = results.iter().map(|result| result.as_ref().expect("same-user claim result").event().id.as_str()).collect();
    assert_eq!(results.iter().filter(|result| matches!(result, Ok(InviteRedemptionCommit::NewlyCommitted { .. }))).count(), 1);
    assert_eq!(results.iter().filter(|result| matches!(result, Ok(InviteRedemptionCommit::AlreadyCommitted { .. }))).count(), 1);
    assert_eq!(returned_ids.len(), 1, "the second service returns the original immutable event");
    let events = primary.events_since(0, DIRECTORY_EVENT_READ_MAX).await.expect("durable events");
    let redeemed: Vec<_> = events.iter().filter(|event| matches!(event.body, DirectoryEventBody::InviteRedeemed { .. })).collect();
    assert_eq!(redeemed.len(), 1);
    assert_eq!(primary.list_invites("default").await.expect("claimed invite")[0].accepted_at, Some(redeemed[0].recorded_at_ms));
    assert_eq!(primary.list_members("default").await.expect("projected membership").iter().filter(|(user, _)| user.id == invited.id).count(), 1);

    let other = primary.create_user("invite-race-other@example.com", "Invite Race Other", None, None, None).await.expect("create competing user");
    let contested = primary.issue_invite("default", SpaceRole::Spectator, 3600, "invite-race-different-users").await.expect("issue contested invite");
    let barrier = Arc::new(tokio::sync::Barrier::new(3));
    let contenders = [(primary.clone(), invited.id.clone()), (secondary.clone(), other.id.clone())];
    let mut claims = Vec::new();
    for (directory, user_id) in contenders {
        let barrier = barrier.clone();
        let capability = contested.capability.clone();
        claims.push(tokio::spawn(async move {
            barrier.wait().await;
            DirectoryService::new(directory, 16).redeem_invite(user_actor(&user_id), &capability, &user_id).await
        }));
    }
    barrier.wait().await;
    let contested_results = [claims.remove(0).await.expect("first contested claim"), claims.remove(0).await.expect("second contested claim")];
    assert_eq!(contested_results.iter().filter(|result| result.is_ok()).count(), 1);
    assert_eq!(contested_results.iter().filter(|result| matches!(result, Err(DirectoryError::Conflict(message)) if message == "invite already accepted")).count(), 1);
    let contested_event = primary
        .events_since(0, DIRECTORY_EVENT_READ_MAX)
        .await
        .expect("contested durable events")
        .into_iter()
        .find(|event| matches!(&event.body, DirectoryEventBody::InviteRedeemed { invite_id, .. } if invite_id == &contested.record.id))
        .expect("one contested event");
    let contested_user = contested_event.user_id.clone().expect("contested event user");
    let contested_retry = DirectoryService::new(primary.clone(), 16).redeem_invite(user_actor(&contested_user), &contested.capability, &contested_user).await.expect("winning user idempotent retry");
    assert_eq!(contested_retry.event().id, contested_event.id);
    drop(primary);
    drop(secondary);

    let reopened_backend = SqliteDirectory::connect(&path_text).await.expect("reopen");
    let reopened = Arc::new(HubDirectories::from(reopened_backend));
    let reopened_service = DirectoryService::new(reopened.clone(), 16);
    let reopened_retry = reopened_service.redeem_invite(user_actor(&invited.id), &issued.capability, &invited.id).await.expect("restart same-user retry");
    assert_eq!(reopened_retry.event().id, redeemed[0].id);
    let reopened_contested = reopened_service.redeem_invite(user_actor(&contested_user), &contested.capability, &contested_user).await.expect("restart contested winner retry");
    assert_eq!(reopened_contested.event().id, contested_event.id);
    let before = reopened.events_since(0, DIRECTORY_EVENT_READ_MAX).await.expect("events before rebuild");
    reopened.rebuild_projections().await.expect("rebuild projections");
    let after = reopened.events_since(0, DIRECTORY_EVENT_READ_MAX).await.expect("events after rebuild");
    assert_eq!(before, after);
    assert_eq!(after.iter().filter(|event| matches!(event.body, DirectoryEventBody::InviteRedeemed { .. })).count(), 2);
    assert_eq!(reopened.list_members("default").await.expect("rebuilt membership").iter().filter(|(user, _)| user.id == invited.id).count(), 1);
    drop(reopened_service);
    drop(reopened);
    for target in [&path, &root.join("directory.sqlite3-wal"), &root.join("directory.sqlite3-shm")] {
        if target.exists() {
            std::fs::remove_file(target).expect("remove exact invite test file");
        }
    }
    std::fs::remove_dir(&root).expect("remove invite test directory");
}

struct DirectoryRebuildWriterProbe {
    connection: std::sync::Mutex<rusqlite::Connection>,
    excluded: AtomicBool,
}

impl ProjectionRebuildControl for DirectoryRebuildWriterProbe {
    fn is_cancelled(&self) -> bool {
        false
    }

    fn report(&self, progress: ProjectionRebuildProgress) {
        if progress.completed_events != 0 {
            return;
        }
        let connection = self.connection.lock().unwrap();
        let result = connection.execute_batch("BEGIN IMMEDIATE");
        self.excluded.store(matches!(&result, Err(rusqlite::Error::SqliteFailure(error, _)) if matches!(error.code, rusqlite::ErrorCode::DatabaseBusy | rusqlite::ErrorCode::DatabaseLocked)), Ordering::SeqCst);
        if result.is_ok() {
            connection.execute_batch("ROLLBACK").unwrap();
        }
    }
}

/// 🏛️ Independent service writers cannot retain or restore an Author after the archive event.
#[tokio::test]
async fn invite_archive_projection_serializes_independent_service_decisions() {
    let fixture: serde_json::Value = serde_json::from_str(semio_hub_testing::hub_source("📇️directory/🧪️tests/🧫️fixtures/🎟️invite-redemption-transaction-v1/🔣️.json")).unwrap();
    for row in fixture["backendOrders"].as_array().unwrap() {
        let root = semio_hub_testing::test_artifact_root().join(format!("archive-writers-{}", time_ordered_id()));
        std::fs::create_dir_all(&root).unwrap();
        let path = root.join("directory.sqlite3");
        let primary = SqliteDirectory::connect(path.to_str().unwrap()).await.unwrap();
        primary.seed().await.unwrap();
        let directory = Arc::new(HubDirectories::from(primary));
        let secondary = Arc::new(HubDirectories::from(SqliteDirectory::connect(path.to_str().unwrap()).await.unwrap()));
        let first = Arc::new(DirectoryService::new(directory.clone(), 32));
        let second = DirectoryService::new(secondary.clone(), 32);
        let owner = user_actor("seed");
        let space = create_space(&first, &owner, DirectorySpaceKind::Studio).await;
        let member = directory.create_user("archive-race@example.test", "Archive Race", None, None, None).await.unwrap();
        let issued = directory.issue_invite(&space, SpaceRole::Author, 600, "archive-backend-order").await.unwrap();
        if row["first"] == "member" {
            second.execute(owner.clone(), DirectoryCommand::UpsertMember { space_id: space.clone(), email: member.email.clone(), role: DirectorySpaceRole::Spectator }).await.unwrap();
        }
        let command = if row["first"] == "archive" { DirectoryCommand::ArchiveSpace { space_id: space.clone() } } else { DirectoryCommand::UpsertMember { space_id: space.clone(), email: member.email.clone(), role: DirectorySpaceRole::Author } };
        let claim = NewDirectoryCommandReceipt {
            actor_user_id: "seed".into(),
            request_id: "a00102030405060708090a0b0c0d0e0f".into(),
            command_sha256: directory::os_directory::io::text::directory_command_sha256(&command),
            result_kind: directory_command_result_kind(&command),
            claimed_at: now_ms(),
        };
        let fence = first.arm_decision_test_fence();
        let pending = {
            let first = first.clone();
            let actor = owner.clone();
            let command = command.clone();
            let claim = claim.clone();
            tokio::spawn(async move { first.execute_idempotent(actor, claim, command).await })
        };
        tokio::time::timeout(std::time::Duration::from_secs(5), fence.reached.notified()).await.unwrap();
        if row["between"] == "invite" {
            assert!(matches!(second.redeem_invite(user_actor(&member.id), &issued.capability, &member.id).await.unwrap(), InviteRedemptionCommit::NewlyCommitted { .. }));
        } else {
            second.execute(owner, DirectoryCommand::ArchiveSpace { space_id: space.clone() }).await.unwrap();
        }
        let head = directory.head_seq().await.unwrap();
        fence.release.notify_one();
        let result = tokio::time::timeout(std::time::Duration::from_secs(5), pending).await.unwrap().unwrap();
        assert_eq!(result.is_ok(), row["accepted"].as_bool().unwrap(), "{}", row["name"]);
        assert_eq!(directory.get_space(&space).await.unwrap().unwrap().kind, "archive");
        assert_eq!(directory.get_role(&space, &member.id).await.unwrap(), Some(SpaceRole::Spectator), "{}", row["name"]);
        assert_eq!(directory.head_seq().await.unwrap() - head, row["appended"].as_u64().unwrap(), "{}", row["name"]);
        if !row["accepted"].as_bool().unwrap() {
            assert!(matches!(result, Err(DirectoryError::Conflict(_))));
            let retry = first.execute_idempotent(user_actor("seed"), claim.clone(), command).await;
            assert!(matches!(retry, Err(DirectoryError::Conflict(_))), "same command retries its decision, not a poisoned pending receipt");
            assert!(matches!(directory.claim_or_read_directory_command_receipt(&claim).await.unwrap(), DirectoryCommandClaimV1::Claimed(_)));
            assert!(matches!(directory.release_directory_command_receipt(&claim.actor_user_id, &claim.request_id, &"f".repeat(64)).await, Err(DirectoryError::Conflict(_))));
            assert!(matches!(directory.claim_or_read_directory_command_receipt(&claim).await.unwrap(), DirectoryCommandClaimV1::Existing(record) if record.disposition == DirectoryCommandDispositionV1::Pending));
            directory.release_directory_command_receipt(&claim.actor_user_id, &claim.request_id, &claim.command_sha256).await.unwrap();
        } else {
            assert!(matches!(directory.release_directory_command_receipt(&claim.actor_user_id, &claim.request_id, &claim.command_sha256).await, Err(DirectoryError::Conflict(_))));
            assert!(matches!(directory.claim_or_read_directory_command_receipt(&claim).await.unwrap(), DirectoryCommandClaimV1::Existing(record) if record.disposition == DirectoryCommandDispositionV1::Completed));
        }
        let events = directory.events_since(0, DIRECTORY_EVENT_READ_MAX).await.unwrap();
        let folded = events.iter().fold(directory::os_directory::DirectoryReadModel::default(), directory::os_directory::fold);
        assert!(folded.spaces[&space].members.iter().all(|member| member.role == DirectorySpaceRole::Spectator));
        let probe_connection = rusqlite::Connection::open(&path).unwrap();
        probe_connection.busy_timeout(std::time::Duration::ZERO).unwrap();
        let probe = DirectoryRebuildWriterProbe { connection: std::sync::Mutex::new(probe_connection), excluded: AtomicBool::new(false) };
        directory.rebuild_projections_controlled(&probe).await.unwrap();
        assert_eq!(probe.excluded.load(Ordering::SeqCst), row["rebuildWriterExcluded"].as_bool().unwrap(), "rebuild owns backend writer before counting events");
        drop(probe);
        assert_eq!(directory.get_role(&space, &member.id).await.unwrap(), Some(SpaceRole::Spectator));
        drop(first);
        drop(second);
        drop(directory);
        drop(secondary);
        for filename in ["directory.sqlite3", "directory.sqlite3-wal", "directory.sqlite3-shm"] {
            let file = root.join(filename);
            if file.exists() {
                std::fs::remove_file(file).unwrap();
            }
        }
        std::fs::remove_dir(root).unwrap();
    }
}

/// 🌫️ A lost commit acknowledgement retains the exact claim and never repeats its durable mutation.
#[tokio::test]
async fn directory_command_uncertain_commit_retains_claim_and_never_reexecutes() {
    let fixture: serde_json::Value = serde_json::from_str(semio_hub_testing::hub_source("📇️directory/🧪️tests/🧫️fixtures/🎟️invite-redemption-transaction-v1/🔣️.json")).unwrap();
    let row = fixture["receiptRelease"].as_array().unwrap().iter().find(|row| row["outcome"] == "uncertain").unwrap();
    assert_eq!(row["release"], false);
    let backend = SqliteDirectory::connect(":memory:").await.unwrap();
    backend.seed().await.unwrap();
    let directory = Arc::new(HubDirectories::from(backend));
    let service = DirectoryService::new(directory.clone(), 16);
    let mut messages = service.subscribe();
    let command = DirectoryCommand::RenameSpace { space_id: "default".into(), name: "Committed without acknowledgement".into() };
    let claim = NewDirectoryCommandReceipt {
        actor_user_id: "seed".into(),
        request_id: "b00102030405060708090a0b0c0d0e0f".into(),
        command_sha256: directory::os_directory::io::text::directory_command_sha256(&command),
        result_kind: directory_command_result_kind(&command),
        claimed_at: now_ms(),
    };
    let head = directory.head_seq().await.unwrap();
    if let HubDirectories::Sqlite(backend) = directory.as_ref() {
        backend.fail_next_append_commit_ack();
    }
    assert!(matches!(service.execute_idempotent(user_actor("seed"), claim.clone(), command.clone()).await, Err(DirectoryError::Backend(_))));
    assert_eq!(directory.head_seq().await.unwrap(), head + 1);
    assert_eq!(directory.get_space("default").await.unwrap().unwrap().name, "Committed without acknowledgement");
    assert!(matches!(directory.claim_or_read_directory_command_receipt(&claim).await.unwrap(), DirectoryCommandClaimV1::Existing(record) if record.disposition == DirectoryCommandDispositionV1::Pending));
    let retry = service.execute_idempotent(user_actor("seed"), claim.clone(), command).await.unwrap();
    assert!(matches!(retry, DirectoryCommandExecutionV1::Receipt(receipt) if receipt.events.is_empty()));
    assert_eq!(directory.head_seq().await.unwrap(), head + 1);
    assert!(matches!(messages.try_recv(), Err(tokio::sync::broadcast::error::TryRecvError::Empty)));
}

/// 🎟️ A projection failure rolls back the accepted marker and event so the exact capability remains retryable.
#[tokio::test]
async fn invite_redemption_projection_failure_rolls_back_claim_event_and_membership() {
    let backend = SqliteDirectory::connect(":memory:").await.expect("connect");
    backend.seed().await.expect("seed");
    let mut clock = HubClock::new();
    backend
        .append_events(&[new_event(
            &mut clock,
            &DirectoryActor { kind: DirectoryActorKind::System, id: "system:invite-failure".into() },
            None,
            Some("u-invite-failure".into()),
            DirectoryEventBody::UserCreated { user_id: "u-invite-failure".into(), email: "invite-failure@example.com".into(), display_name: "Invite Failure".into() },
        )])
        .await
        .expect("seed failure user");
    let issued = backend.issue_invite("default", SpaceRole::Spectator, 3600, "invite-failure").await.expect("issue invite");
    let head_before_forgery = backend.head_seq().await.expect("head before forged redemption");
    let forged = new_event(
        &mut clock,
        &user_actor("u-invite-failure"),
        Some("default".into()),
        Some("u-invite-failure".into()),
        DirectoryEventBody::InviteRedeemed { space_id: "default".into(), user_id: "u-invite-failure".into(), invite_id: issued.record.id.clone(), role: DirectorySpaceRole::Spectator },
    );
    assert!(matches!(backend.append_events(&[forged]).await, Err(DirectoryError::Conflict(_))));
    assert_eq!(backend.head_seq().await.expect("head after forged redemption"), head_before_forgery);
    backend.install_invite_projection_failure().expect("install projection failure");
    let directory = Arc::new(HubDirectories::from(backend));
    let service = DirectoryService::new(directory.clone(), 16);
    assert!(matches!(service.redeem_invite(user_actor("u-invite-failure"), &issued.capability, "u-invite-failure").await, Err(DirectoryError::Backend(_))));
    assert_eq!(directory.list_invites("default").await.expect("invite after rollback")[0].accepted_at, None);
    assert_eq!(directory.events_since(0, DIRECTORY_EVENT_READ_MAX).await.expect("events after rollback").iter().filter(|event| matches!(event.body, DirectoryEventBody::InviteRedeemed { .. })).count(), 0);
    assert_eq!(directory.get_role("default", "u-invite-failure").await.expect("membership after rollback"), None);
    let HubDirectories::Sqlite(backend) = directory.as_ref() else { panic!("invite rollback law requires SQLite") };
    backend.clear_invite_projection_failure().expect("clear projection failure");
    let redeemed = service.redeem_invite(user_actor("u-invite-failure"), &issued.capability, "u-invite-failure").await.expect("retry exact invite");
    assert!(matches!(redeemed, InviteRedemptionCommit::NewlyCommitted { .. }));
    assert_eq!(directory.get_role("default", "u-invite-failure").await.expect("membership after retry"), Some(SpaceRole::Spectator));
    backend.install_invite_acceptance_marker_for_test(&issued.record.id, "missing-event").expect("install corrupt acceptance marker");
    assert!(matches!(service.redeem_invite(user_actor("u-invite-failure"), &issued.capability, "u-invite-failure").await, Err(DirectoryError::Backend(_))));
}

/// 📣️ A committed redemption is broadcast before a later command can enter the shared writer.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn invite_redemption_commit_and_publication_precede_the_next_directory_command() {
    let directory = fresh_dir().await;
    let service = Arc::new(DirectoryService::new(directory.clone(), 16));
    let owner = user_actor("u-owner");
    let space_id = create_space(&service, &owner, DirectorySpaceKind::Studio).await;
    let invited = directory.create_user("invite-order@example.com", "Invite Order", None, None, None).await.expect("create invited user");
    let issued = directory.issue_invite(&space_id, SpaceRole::Spectator, 3600, "invite-order").await.expect("issue invite");
    let mut receiver = service.subscribe();
    let fence = service.arm_publication_test_fence();
    let redeem_service = service.clone();
    let redeem_user = invited.id.clone();
    let capability = issued.capability.clone();
    let redeem = tokio::spawn(async move { redeem_service.redeem_invite(user_actor(&redeem_user), &capability, &redeem_user).await });
    fence.reached.notified().await;
    let rename_service = service.clone();
    let rename_actor = owner.clone();
    let rename_space = space_id.clone();
    let rename = tokio::spawn(async move { rename_service.execute(rename_actor, DirectoryCommand::RenameSpace { space_id: rename_space, name: "After Invite".into() }).await });
    tokio::task::yield_now().await;
    assert!(!rename.is_finished(), "later command remains excluded until redemption publication");
    fence.release.notify_one();
    let redeemed = redeem.await.expect("redeem task").expect("redeem");
    let renamed = rename.await.expect("rename task").expect("rename").0;
    assert!(redeemed.event().seq < renamed[0].seq);
    let first = receiver.recv().await.expect("redeem publication");
    let second = receiver.recv().await.expect("rename publication");
    assert!(matches!(first, DirectoryStreamMessage::Event { event } if event.seq == redeemed.event().seq));
    assert!(matches!(second, DirectoryStreamMessage::Event { event } if event.seq == renamed[0].seq));
    let retried = service.redeem_invite(user_actor(&invited.id), &issued.capability, &invited.id).await.expect("same-user retry");
    assert!(matches!(retried, InviteRedemptionCommit::AlreadyCommitted { .. }));
    assert_eq!(retried.event(), redeemed.event());
    assert!(tokio::time::timeout(std::time::Duration::from_millis(25), receiver.recv()).await.is_err(), "idempotent retry never republishes the original event");
}

mod exhaustive {
    use super::*;
use semio_hub_directory_service::{ARTIFACT_CAS_SWEEP_CONTINUATION_BYTES, ARTIFACT_CAS_SWEEP_CONTINUATION_PAYLOAD_BYTES, ArtifactCasSweepRequest, DirectoryService};
use directory::os_directory::{DirectoryActor, DirectoryActorKind};

    #[tokio::test]
    async fn artifact_chunk_cas_opaque_continuation_converges_after_page_overflow_cancel_and_resume() {
        let fixture: serde_json::Value = serde_json::from_str(semio_hub_testing::hub_source("🗿️artifact-authority/🧪️tests/🧫️fixtures/🧱️artifact-chunk-cas/🔣️.json")).expect("artifact CAS fixture");
        let law = &fixture["sweepContinuation"];
        let object_counts = law["planObjectCounts"].as_array().expect("plan object counts");
        let total_objects = law["totalObjects"].as_u64().expect("total objects");
        let maximum = usize::try_from(law["requestMaximumObjects"].as_u64().expect("request maximum")).expect("bounded request maximum");
        assert_eq!(law["tokenPayloadBytes"].as_u64(), Some(ARTIFACT_CAS_SWEEP_CONTINUATION_PAYLOAD_BYTES as u64));
        assert_eq!(law["tokenAuthenticationBytes"].as_u64(), Some((ARTIFACT_CAS_SWEEP_CONTINUATION_BYTES - ARTIFACT_CAS_SWEEP_CONTINUATION_PAYLOAD_BYTES) as u64));
        assert_eq!(law["cursorExposesObjectIdentity"].as_bool(), Some(false));
        assert_eq!(law["invalidAfterGenerationChange"].as_bool(), Some(true));
        assert_eq!(law["invalidAfterRestart"].as_bool(), Some(true));
        let dir = fresh_dir().await;
        let service = DirectoryService::new(dir.clone(), 64);
        let system = DirectoryActor { kind: DirectoryActorKind::System, id: "system:artifact-authority".into() };
        for (index, count) in object_counts.iter().enumerate() {
            let count = usize::try_from(count.as_u64().expect("plan object count")).expect("bounded plan object count");
            service.reserve_artifact_cas(system.clone(), sweep_convergence_plan(index, count), 200, 100).await.expect("reserve convergence plan");
        }
        assert_eq!(dir.artifact_cas_ledger_generation().await.expect("sweep generation"), law["ledgerGeneration"].as_u64().expect("fixture generation"));
        let storage = MemoryArtifactChunkCasStorage::default();
        let probe = ArtifactCasProbe::new(201, None);
        let context = OperationContext::new(10_000, AuthorityLimits::maximum(), &probe);
        assert!(matches!(
            service.sweep_artifact_cas(&storage, ArtifactCasSweepRequest { execute: true, max_objects: maximum + 1, continuation: None }, &context).await,
            Err(semio_hub_artifact_authority::AuthorityError::ResourceLimit("artifact CAS sweep object"))
        ));
        let first = service.sweep_artifact_cas(&storage, ArtifactCasSweepRequest { execute: true, max_objects: maximum, continuation: None }, &context).await.expect("first bounded sweep");
        let continuation = first.continuation.expect("first sweep continuation");
        assert_eq!(first.examined_objects, law["expectedExaminedPerRequest"][0].as_u64().expect("first examined"));
        assert_eq!(format!("{continuation:?}"), "ArtifactCasSweepContinuation(<opaque>)");
        let first_position = service.artifact_cas_sweep_position(continuation, true).expect("decode owned continuation");
        assert_eq!(first_position.observed_generation, law["expectedFirstCursor"]["observedGeneration"].as_u64().expect("cursor generation"));
        assert_eq!(first_position.after_generation, law["expectedFirstCursor"]["afterGeneration"].as_u64().expect("cursor page"));
        assert_eq!(first_position.object_offset as u64, law["expectedFirstCursor"]["objectOffset"].as_u64().expect("cursor offset"));
        assert!(matches!(service.sweep_artifact_cas(&storage, ArtifactCasSweepRequest { execute: false, max_objects: maximum, continuation: Some(continuation) }, &context).await, Err(semio_hub_artifact_authority::AuthorityError::Store(_))));

        let restarted = DirectoryService::new(dir.clone(), 64);
        assert!(matches!(restarted.sweep_artifact_cas(&storage, ArtifactCasSweepRequest { execute: true, max_objects: maximum, continuation: Some(continuation) }, &context).await, Err(semio_hub_artifact_authority::AuthorityError::Store(_))));
        let cancel_probe = ArtifactCasProbe::new(201, Some(1));
        let cancel_context = OperationContext::new(10_000, AuthorityLimits::maximum(), &cancel_probe);
        assert!(matches!(service.sweep_artifact_cas(&storage, ArtifactCasSweepRequest { execute: true, max_objects: maximum, continuation: Some(continuation) }, &cancel_context).await, Err(semio_hub_artifact_authority::AuthorityError::Cancelled)));
        assert_eq!(cancel_probe.progress.lock().expect("cancel progress").iter().filter(|progress| progress.stage == AuthorityProgressStage::CasSweep).count(), 1);
        let second = service.sweep_artifact_cas(&storage, ArtifactCasSweepRequest { execute: true, max_objects: maximum, continuation: Some(continuation) }, &context).await.expect("resume bounded sweep");
        assert_eq!(second.examined_objects, law["expectedExaminedPerRequest"][1].as_u64().expect("second examined"));
        assert!(second.continuation.is_none());
        assert_eq!(first.examined_objects + second.examined_objects, total_objects);

        let changed = service.sweep_artifact_cas(&storage, ArtifactCasSweepRequest { execute: true, max_objects: 1, continuation: None }, &context).await.expect("generation-bound continuation").continuation.expect("generation-bound token");
        service.reserve_artifact_cas(system, sweep_convergence_plan(object_counts.len(), 2), 500, 300).await.expect("advance sweep generation");
        assert!(matches!(service.sweep_artifact_cas(&storage, ArtifactCasSweepRequest { execute: true, max_objects: maximum, continuation: Some(changed) }, &context).await, Err(semio_hub_artifact_authority::AuthorityError::Store(_))));
    }
}

#[test]
fn directory_document_index_is_ordered_idempotent_and_backend_descriptor_bound() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧪️tests/🧫️fixtures/📇️document-index-v1/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let events: Vec<directory::os_directory::DirectoryEvent> = semio_framework_pack_json::from_json_str(&row["events"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let client = events.iter().fold(directory::os_directory::DirectoryReadModel::default(), directory::os_directory::fold);
        assert_eq!(client.spaces["space-fixture"].indexed_documents.len(), row["clientRows"].as_u64().unwrap() as usize, "{}", row["id"]);
        let mut backend = semio_hub_directory::MemoryArtifactProjection::default();
        let result = backend.fold_atomically(&events);
        assert_eq!(result.is_ok(), row["backendAccepted"].as_bool().unwrap(), "{}: {result:?}", row["id"]);
        if result.is_err() {
            assert_eq!(backend, semio_hub_directory::MemoryArtifactProjection::default());
        }
    }
}
