/// ⚡️ The quick hostile-input laws module: socket grants, admin revocation, check-in policy and route admission.

mod quick {
use semio_hub_route_models::admin::ADMIN_OPERATION_DEADLINE;
use std::sync::Arc;
use semio_hub_runtime::readiness::{CheckpointPolicyLedgerV1, CheckpointPolicyTallyV1, HUB_CHECKPOINT_POLICY, HubCheckpointPolicyV1, HubMode, hub_readiness, with_checkpoint_policy};
use semio_hub_artifact_checkin::DocumentCheckInKey;
use directory::os_directory::{DocumentScope, EditedArtifactFrontierV1};
use semio_hub_native_openable::NativeCodecProviderSetV1;
use semio_hub_route_testkit::{SOCKET_RENDEZVOUS_HANG_GUARD, STUDIO, announce_document_for_test, artifact_document_id_for_test, authorize_test_admin, bearer_headers, client_binary, create_space_for_test, document_socket_request, install_document_open_catalog_for_test, issue_and_exchange_document_open_plan_for_test, issue_test_session, loopback_peer, native_openable_stdio_bundle, next_close_code, next_close_without_authority, next_directory_message, next_server_frame, next_server_frame_at, publish_checkpoint_for_test, raw_http_get, raw_http_request, run_socket_test, sample_envelope, seed_author_token, seed_genesis_document_for_test, seed_genesis_for_document_for_test, socket_hello, socket_request, spawn_restartable_server, spawn_server, stop_recovery_server, tempdir, test_state, test_state_with_directory, upsert_member_for_test};
use semio_hub_directory_sqlite::SqliteDirectory;
use semio_hub_runtime::startup::{StartupCancellationV1, StartupProgressCellV1, configured_artifact_authority};
use semio_framework_trace::record::Tracer;
use protocol::ArtifactId as WireArtifactId;
use semio_hub_document_routes::admin::{admin_intents, cancel_admin_operation};
use semio_hub_document_routes::check_in::{check_in_author, run_policy_check_in};
use tokio_tungstenite::connect_async;
use semio_hub_runtime::presence::db_artifact_id;
use semio_hub_session_routes::session_agent::post_session_sign_out;
#[cfg(test)]
use protocol::{AckStage, ActorId, ApplyOutcome, ClientFrame, Lane, ServerFrame};
#[cfg(test)]
use directory::os_directory::{AdminIntentReceiptV1, AdminIntentStateV1, AdminIntentV1, DirectoryActor, DirectoryActorKind, DirectoryCommand, DirectorySpaceRole, DirectorySpaceVisibility, DirectoryStreamMessage, DocumentCheckInPhaseV1, DocumentCheckInRefusalV1, DocumentCheckInStatusV1, DocumentCheckInV1};
#[cfg(test)]
use axum::body::Bytes;
#[cfg(test)]
use axum::extract::{Path, State};
#[cfg(test)]
use axum::http::StatusCode;
#[cfg(any(test, feature = "testkit"))]
use semio_hub_route_models::state::TestLiveGate;
#[cfg(any(test, feature = "testkit"))]
use semio_hub_runtime::presence::TestPresenceClock;
#[cfg(any(test, feature = "testkit"))]
use semio_hub_session_routes::access_grants::{execute_directory_command_fenced, issue_document_socket_grant_fixture};
#[cfg(test)]
use axum::routing::post;
#[cfg(feature="integration-fixtures")]
use crate::tests::{check_in_cold_pair, check_in_commit, check_in_fixture, check_in_map_edits, check_in_replica_pair, check_in_until_terminal};

    #[test]
    fn socket_grant_revoke_before_command_admission_has_no_storage_effect() {
        run_socket_test(|| async {
            let mut state = test_state().await;
            let live_gate = Arc::new(TestLiveGate::default());
            state.live_gate = Some(live_gate.clone());
            let token = seed_author_token(&state).await;
            announce_document_for_test(&state, STUDIO, "socket-command-revoke").await;
            let receipt = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), "socket-command-revoke".to_string())), bearer_headers(&token), State(state.clone())).await.expect("issue socket grant").0;
            let addr = spawn_server(state.clone()).await;
            let url = format!("ws://{addr}/scopes/{STUDIO}%2Fsocket-command-revoke/document/ws");
            let (mut socket, _) = connect_async(document_socket_request(&url, &token)).await.expect("socket upgrade");
            socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("socket hello");
            tokio::time::timeout(std::time::Duration::from_secs(5), live_gate.socket_before_welcome.acquire()).await.expect("pre-Welcome deadline").expect("pre-Welcome");
            live_gate.socket_welcome_release.add_permits(1);
            assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Welcome { .. }));
            tokio::time::timeout(std::time::Duration::from_secs(5), live_gate.socket_after_welcome.acquire()).await.expect("post-Welcome deadline").expect("post-Welcome");
            live_gate.socket_bootstrap_release.add_permits(1);
            assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Session { .. }));
            tokio::time::timeout(std::time::Duration::from_secs(5), live_gate.document_subscribed.acquire()).await.expect("subscription deadline").expect("subscription");
            live_gate.document_release.add_permits(1);

            let document = db_artifact_id(&DocumentScope::new(STUDIO, "socket-command-revoke"));
            let wire_document = WireArtifactId("socket-command-revoke".into());
            let mut accepted = sample_envelope("accepted-op", &wire_document).await;
            accepted.actor = ActorId(receipt.actor_id.clone());
            socket.send(client_binary(&ClientFrame::Commands { batch_id: 90, envelopes: vec![accepted] }, Lane::Command).await).await.expect("control command received by server");
            tokio::time::timeout(std::time::Duration::from_secs(5), live_gate.socket_command_received.acquire()).await.expect("control command boundary deadline").expect("control command boundary").forget();
            live_gate.socket_command_release.add_permits(1);
            assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Ack { batch_id: 90, .. }));
            assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Commands { envelopes, .. } if envelopes.iter().all(|envelope| envelope.document_id == wire_document)), "the authorized batch's own relay is delivered before any revoke, on the wire document id");
            let accepted_frontier = state.db.document(&document).await.expect("document handle").frontier().await.expect("accepted frontier");
            assert_eq!(accepted_frontier.head_seq, 1, "an actor-matching command persists while authorized");

            let mut revoked = sample_envelope("revoked-op", &wire_document).await;
            revoked.actor = ActorId(receipt.actor_id.clone());
            socket.send(client_binary(&ClientFrame::Commands { batch_id: 91, envelopes: vec![revoked] }, Lane::Command).await).await.expect("revoked command received by server");
            tokio::time::timeout(std::time::Duration::from_secs(5), live_gate.socket_command_received.acquire()).await.expect("command boundary deadline").expect("command boundary").forget();
            assert_eq!(post_session_sign_out(bearer_headers(&token), State(state.clone()), Bytes::new()).await, StatusCode::NO_CONTENT);
            live_gate.socket_command_release.add_permits(1);
            assert_eq!(next_close_without_authority(&mut socket).await, 4401, "no Ack crosses a revoke that wins before command admission");
            let frontier = state.db.document(&document).await.expect("document handle").frontier().await.expect("frontier");
            assert_eq!(frontier.head_seq, 1, "the revoked actor-matching command never reaches durable storage");
        });
    }

    /// 🧾️ A `Commands` batch the engine committed after its socket's frame deadline elapsed is still acknowledged on that
    /// socket, its relay still reaches the socket, and the socket stays open for the next batch: the deadline bounds a
    /// frame's admission only (ticket 26/09/23 session 14c — H13's transient probe saw batch 24 commit at head 6144 while
    /// the deadline closed its socket `1013` and its Ack never came).
    #[test]
    fn a_batch_committed_past_the_frame_deadline_is_still_acknowledged() {
        run_socket_test(|| async {
            let mut state = test_state().await;
            let live_gate = Arc::new(TestLiveGate::default());
            *live_gate.socket_frame_deadline.lock().expect("frame deadline override") = Some(std::time::Duration::from_millis(200));
            live_gate.socket_commit_pause_enabled.store(true, std::sync::atomic::Ordering::Release);
            state.live_gate = Some(live_gate.clone());
            let token = seed_author_token(&state).await;
            announce_document_for_test(&state, STUDIO, "socket-commit-deadline").await;
            let receipt = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), "socket-commit-deadline".to_string())), bearer_headers(&token), State(state.clone())).await.expect("issue socket grant").0;
            let addr = spawn_server(state.clone()).await;
            let url = format!("ws://{addr}/scopes/{STUDIO}%2Fsocket-commit-deadline/document/ws");
            let (mut socket, _) = connect_async(document_socket_request(&url, &token)).await.expect("socket upgrade");
            socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("socket hello");
            tokio::time::timeout(std::time::Duration::from_secs(5), live_gate.socket_before_welcome.acquire()).await.expect("pre-Welcome deadline").expect("pre-Welcome").forget();
            live_gate.socket_welcome_release.add_permits(1);
            assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Welcome { .. }));
            tokio::time::timeout(std::time::Duration::from_secs(5), live_gate.socket_after_welcome.acquire()).await.expect("post-Welcome deadline").expect("post-Welcome").forget();
            live_gate.socket_bootstrap_release.add_permits(1);
            assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Session { .. }));
            tokio::time::timeout(std::time::Duration::from_secs(5), live_gate.document_subscribed.acquire()).await.expect("subscription deadline").expect("subscription").forget();
            live_gate.document_release.add_permits(1);

            let document = db_artifact_id(&DocumentScope::new(STUDIO, "socket-commit-deadline"));
            let wire_document = WireArtifactId("socket-commit-deadline".into());
            for (batch_id, id, head) in [(70u64, "late-commit-op", 1u64), (71, "next-commit-op", 2)] {
                let mut envelope = sample_envelope(id, &wire_document).await;
                envelope.actor = ActorId(receipt.actor_id.clone());
                socket.send(client_binary(&ClientFrame::Commands { batch_id, envelopes: vec![envelope] }, Lane::Command).await).await.expect("command sent");
                tokio::time::timeout(std::time::Duration::from_secs(5), live_gate.socket_command_received.acquire()).await.expect("command boundary deadline").expect("command boundary").forget();
                live_gate.socket_command_release.add_permits(1);
                tokio::time::timeout(std::time::Duration::from_secs(5), live_gate.socket_commit_admitted.acquire()).await.expect("commit boundary deadline").expect("commit boundary").forget();
                let committed = state.db.document(&document).await.expect("document handle").frontier().await.expect("committed frontier");
                assert_eq!(committed.head_seq, head, "the engine committed batch {batch_id} before its answer");
                tokio::time::sleep(std::time::Duration::from_millis(600)).await;
                live_gate.socket_commit_release.add_permits(1);
                match next_server_frame_at(&mut socket, "Ack of a batch committed past the frame deadline").await {
                    ServerFrame::Ack { batch_id: acked, stages, .. } => {
                        assert_eq!(acked, batch_id);
                        assert!(stages.iter().any(|stage| matches!(stage, AckStage::Applied { outcome } if matches!(outcome.as_ref(), ApplyOutcome::Accepted))), "batch {batch_id} is acknowledged as accepted: {stages:?}");
                    }
                    other => panic!("batch {batch_id} answered {other:?} instead of its Ack"),
                }
                assert!(matches!(next_server_frame_at(&mut socket, "relay").await, ServerFrame::Commands { .. }), "the committed batch's relay follows its Ack");
            }
        });
    }

    /// 🔌️ A document socket's authority is its session, membership and sealed plan, never the checkpoint its client seeded
    /// from: a checkpoint published while the socket is open (a member's Check In, or the declared checkpoint policy's) leaves
    /// it open through ten idle minutes of authorization ticks, and its next batch commits and is acknowledged (ticket
    /// 26/09/23 session 14c: on p33 C13's probe socket and both browsers' sockets closed `4401` within one tick of every Check In).
    #[test]
    fn a_plan_socket_outlives_a_check_in_and_ten_idle_minutes_and_its_next_batch_commits() {
        run_socket_test(|| async {
            let mut state = test_state().await;
            state.presence_clock = Some(Arc::new(TestPresenceClock::new()));
            let token = seed_author_token(&state).await;
            let document_id = artifact_document_id_for_test("plan-socket-idle");
            seed_genesis_for_document_for_test(&state, &token, STUDIO, &document_id, "plan-socket-idle").await;
            let scope = DocumentScope::new(STUDIO, &document_id);
            let descriptor = state.directory.get_document_descriptor(&scope).await.expect("descriptor lookup").expect("descriptor");
            install_document_open_catalog_for_test(&mut state, &descriptor);
            let (plan, receipt) = issue_and_exchange_document_open_plan_for_test(&state, &token, &scope, "client:plan-socket-idle").await;
            let addr = spawn_server(state.clone()).await;
            let url = format!("ws://{addr}/scopes/{STUDIO}%2F{document_id}/document/ws?surface={}", plan.surface.surface_id);
            let (mut socket, _) = connect_async(document_socket_request(&url, &token)).await.expect("plan socket upgrade");
            socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("socket hello");
            assert!(matches!(next_server_frame_at(&mut socket, "welcome").await, ServerFrame::Welcome { .. }));
            assert!(matches!(next_server_frame_at(&mut socket, "session").await, ServerFrame::Session { actor, .. } if actor == receipt.actor_id));
            let published = publish_checkpoint_for_test(&state, STUDIO, &document_id).await;
            let active = state.directory.get_active_artifact_checkpoint(&scope).await.expect("active read").expect("active checkpoint");
            assert_eq!(active.baseline_frontier, published.baseline_frontier, "the socket's plan names a superseded checkpoint");
            tokio::time::pause();
            for _ in 0..600 {
                tokio::time::advance(std::time::Duration::from_secs(1)).await;
                tokio::task::yield_now().await;
            }
            tokio::time::resume();
            let document = db_artifact_id(&scope);
            let before = state.db.document(&document).await.expect("document handle").frontier().await.expect("frontier before").head_seq;
            let mut envelope = sample_envelope("after-ten-idle-minutes", &WireArtifactId(document_id.clone())).await;
            envelope.actor = ActorId(receipt.actor_id.clone());
            socket.send(client_binary(&ClientFrame::Commands { batch_id: 600, envelopes: vec![envelope] }, Lane::Command).await).await.expect("batch after ten idle minutes");
            match next_server_frame_at(&mut socket, "Ack after ten idle minutes").await {
                ServerFrame::Ack { batch_id: 600, stages, .. } => assert!(stages.iter().any(|stage| matches!(stage, AckStage::Applied { outcome } if matches!(outcome.as_ref(), ApplyOutcome::Accepted))), "the batch commits: {stages:?}"),
                other => panic!("the batch after ten idle minutes was answered {other:?}"),
            }
            let after = state.db.document(&document).await.expect("document handle").frontier().await.expect("frontier after").head_seq;
            assert_eq!(after, before + 1, "the batch is durable");
        });
    }

    #[tokio::test]
    async fn retained_short_admin_request_drop_duplicate_cancel_and_secret_lifecycle_is_exact() {
        let fixture: serde_json::Value = serde_json::from_str(semio_hub_testing::hub_source("📇️directory/🧪️tests/🧫️fixtures/🏛️retained-short-admin/🔣️.json")).expect("retained short administrator fixture");
        assert_eq!(fixture["cases"].as_array().expect("retained cases").len(), 15);
        let root = tempdir("retained-short-admin");
        std::fs::create_dir_all(&root).expect("retained administrator root");
        let path = root.join("directory.sqlite");
        let directory = SqliteDirectory::connect(path.to_str().expect("retained directory path")).await.expect("retained directory");
        let mut state = test_state_with_directory(root.join("db"), directory, 1024, 256).await;
        let physical = rusqlite::Connection::open(&path).expect("retained administrator physical reader");
        let gate = Arc::new(TestLiveGate::default());
        state.live_gate = Some(gate.clone());
        let email = "retained-short-admin@example.test";
        let _ = authorize_test_admin(&mut state, email).await;
        let admin = issue_test_session(&state, email).await;
        let owner = issue_test_session(&state, "retained-short-owner@example.test").await;
        let space = create_space_for_test(&state, &owner.user_id, "Retained Short", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        let scope = DocumentScope::new(&space, "retained-short-document");
        announce_document_for_test(&state, &scope.space_id, &scope.document_id).await;
        let (addr, shutdown, server) = spawn_restartable_server(state.clone()).await;
        let request_id = "request:retained-short-drop";
        let intent = AdminIntentV1::IssueDocumentShare { request_id: request_id.into(), scope: scope.clone(), ttl_secs: 600 };
        let body = semio_framework_pack_json::to_json_string(&intent);
        *gate.directory_command_pause_user.lock().unwrap() = Some((admin.user_id.clone(), true));
        let dropped = tokio::spawn({
            let authorization = format!("Bearer {}", admin.token);
            let body = body.clone();
            async move { raw_http_request(addr, "POST", "/admin/api/intents", &[("Authorization", &authorization), ("Content-Type", "application/json")], body.as_bytes()).await }
        });
        tokio::time::timeout(std::time::Duration::from_secs(5), gate.directory_command_admitted.acquire()).await.expect("dropped request admission deadline").expect("dropped request admitted").forget();
        dropped.abort();
        let _ = dropped.await;
        *gate.directory_command_pause_user.lock().unwrap() = None;
        gate.directory_command_release.add_permits(1);
        let mut rows = Vec::new();
        for _ in 0..256 {
            rows = state.directory.admin_operation_audit_for_request(request_id).await.expect("dropped request audit");
            if rows.len() == 2 {
                break;
            }
            tokio::task::yield_now().await;
        }
        assert_eq!(rows.len(), 2, "request cancellation cannot cancel its retained operation");
        assert_eq!(rows[1].fact.phase, "succeeded");
        assert_eq!(rows[1].fact.outcome_code, "share-issued");
        assert_eq!(physical.query_row::<i64, _, _>("SELECT count(*) FROM hub_share_grant WHERE space_id = ?1 AND document_id = ?2", rusqlite::params![scope.space_id, scope.document_id], |row| row.get(0)).unwrap(), 1);
        let authorization = format!("Bearer {}", admin.token);
        let retry = raw_http_request(addr, "POST", "/admin/api/intents", &[("Authorization", &authorization), ("Content-Type", "application/json")], body.as_bytes()).await;
        assert_eq!(retry.status, 200);
        let retry_receipt: AdminIntentReceiptV1 = semio_framework_pack_json::from_json_str(std::str::from_utf8(&retry.body).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("retry receipt");
        assert_eq!(retry_receipt.state, AdminIntentStateV1::Succeeded);
        assert!(retry_receipt.result.is_none(), "a lost one-shot share token is never stored or replayed");
        assert_eq!(
            physical.query_row::<i64, _, _>("SELECT count(*) FROM hub_share_grant WHERE space_id = ?1 AND document_id = ?2", rusqlite::params![scope.space_id, scope.document_id], |row| row.get(0)).unwrap(),
            1,
            "retry cannot execute the side effect twice"
        );
        let collision = semio_framework_pack_json::to_json_string(&AdminIntentV1::IssueDocumentShare { request_id: request_id.into(), scope: scope.clone(), ttl_secs: 601 });
        assert_eq!(raw_http_request(addr, "POST", "/admin/api/intents", &[("Authorization", &authorization), ("Content-Type", "application/json")], collision.as_bytes()).await.status, 409);

        let cancelled_scope = DocumentScope::new(&space, "retained-short-cancelled");
        announce_document_for_test(&state, &cancelled_scope.space_id, &cancelled_scope.document_id).await;
        let cancelled_request = "request:retained-short-cancelled";
        let cancelled_body = semio_framework_pack_json::to_json_string(&AdminIntentV1::IssueDocumentShare { request_id: cancelled_request.into(), scope: cancelled_scope.clone(), ttl_secs: 600 });
        *gate.directory_command_pause_user.lock().unwrap() = Some((admin.user_id.clone(), false));
        let cancelled = tokio::spawn({
            let authorization = authorization.clone();
            async move { raw_http_request(addr, "POST", "/admin/api/intents", &[("Authorization", &authorization), ("Content-Type", "application/json")], cancelled_body.as_bytes()).await }
        });
        tokio::time::timeout(std::time::Duration::from_secs(5), gate.directory_command_admitted.acquire()).await.expect("pre-effect cancellation admission deadline").expect("pre-effect cancellation admitted").forget();
        let accepted = state.directory.admin_operation_audit_for_request(cancelled_request).await.expect("cancelled acceptance");
        let operation_id = accepted.first().expect("cancelled accepted row").fact.operation_id.clone();
        cancel_admin_operation(Path(operation_id), bearer_headers(&admin.token), loopback_peer(), State(state.clone())).await.expect("cancel retained operation");
        *gate.directory_command_pause_user.lock().unwrap() = None;
        gate.directory_command_release.add_permits(1);
        let cancelled = cancelled.await.expect("cancelled request task");
        let cancelled_receipt: AdminIntentReceiptV1 = semio_framework_pack_json::from_json_str(std::str::from_utf8(&cancelled.body).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("cancelled receipt");
        assert_eq!(cancelled_receipt.state, AdminIntentStateV1::Cancelled);
        assert_eq!(physical.query_row::<i64, _, _>("SELECT count(*) FROM hub_share_grant WHERE space_id = ?1 AND document_id = ?2", rusqlite::params![cancelled_scope.space_id, cancelled_scope.document_id], |row| row.get(0)).unwrap(), 0);

        let admitted_scope = DocumentScope::new(&space, "retained-short-admitted");
        announce_document_for_test(&state, &admitted_scope.space_id, &admitted_scope.document_id).await;
        let admitted_request = "request:retained-short-admitted";
        let admitted_body = semio_framework_pack_json::to_json_string(&AdminIntentV1::IssueDocumentShare { request_id: admitted_request.into(), scope: admitted_scope.clone(), ttl_secs: 600 });
        gate.admin_effect_pause_enabled.store(true, std::sync::atomic::Ordering::Release);
        let admitted = tokio::spawn({
            let authorization = authorization.clone();
            async move { raw_http_request(addr, "POST", "/admin/api/intents", &[("Authorization", &authorization), ("Content-Type", "application/json")], admitted_body.as_bytes()).await }
        });
        tokio::time::timeout(std::time::Duration::from_secs(5), gate.admin_effect_admitted.acquire()).await.expect("admitted effect deadline").expect("effect admitted").forget();
        let accepted = state.directory.admin_operation_audit_for_request(admitted_request).await.expect("admitted acceptance");
        let operation_id = accepted.first().expect("admitted accepted row").fact.operation_id.clone();
        cancel_admin_operation(Path(operation_id), bearer_headers(&admin.token), loopback_peer(), State(state.clone())).await.expect("late cancellation request");
        gate.admin_effect_pause_enabled.store(false, std::sync::atomic::Ordering::Release);
        gate.admin_effect_release.add_permits(1);
        let admitted = admitted.await.expect("admitted request task");
        let admitted_receipt: AdminIntentReceiptV1 = semio_framework_pack_json::from_json_str(std::str::from_utf8(&admitted.body).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("admitted receipt");
        assert_eq!(admitted_receipt.state, AdminIntentStateV1::Succeeded, "cancellation after effect admission cannot invent rollback");
        assert!(admitted_receipt.result.as_ref().and_then(|result| result.share_token.as_ref()).is_some());
        assert_eq!(physical.query_row::<i64, _, _>("SELECT count(*) FROM hub_share_grant WHERE space_id = ?1 AND document_id = ?2", rusqlite::params![admitted_scope.space_id, admitted_scope.document_id], |row| row.get(0)).unwrap(), 1);

        let queued_scope = DocumentScope::new(&space, "retained-short-admitted-queued");
        announce_document_for_test(&state, &queued_scope.space_id, &queued_scope.document_id).await;
        let first_request = "request:retained-short-admitted-queued-first";
        let second_request = "request:retained-short-admitted-queued-second";
        let first_body = semio_framework_pack_json::to_json_string(&AdminIntentV1::IssueDocumentShare { request_id: first_request.into(), scope: queued_scope.clone(), ttl_secs: 600 });
        let second_body = semio_framework_pack_json::to_json_string(&AdminIntentV1::IssueDocumentShare { request_id: second_request.into(), scope: queued_scope.clone(), ttl_secs: 600 });
        gate.admin_effect_pause_enabled.store(true, std::sync::atomic::Ordering::Release);
        let mut first = tokio::spawn({
            let authorization = authorization.clone();
            async move { raw_http_request(addr, "POST", "/admin/api/intents", &[("Authorization", &authorization), ("Content-Type", "application/json")], first_body.as_bytes()).await }
        });
        tokio::time::timeout(std::time::Duration::from_secs(5), gate.admin_effect_admitted.acquire()).await.expect("first admitted writer deadline").expect("first writer admitted").forget();
        let mut second = tokio::spawn({
            let authorization = authorization.clone();
            async move { raw_http_request(addr, "POST", "/admin/api/intents", &[("Authorization", &authorization), ("Content-Type", "application/json")], second_body.as_bytes()).await }
        });
        for _ in 0..256 {
            if state.directory.admin_operation_audit_for_request(second_request).await.expect("competing acceptance read").len() == 1 {
                break;
            }
            tokio::task::yield_now().await;
        }
        assert!(tokio::time::timeout(std::time::Duration::from_millis(50), &mut second).await.is_err(), "competing same-scope writer cannot pass retained authority");
        assert_eq!(physical.query_row::<i64, _, _>("SELECT count(*) FROM hub_share_grant WHERE space_id = ?1 AND document_id = ?2", rusqlite::params![queued_scope.space_id, queued_scope.document_id], |row| row.get(0)).unwrap(), 0);
        gate.admin_effect_pause_enabled.store(false, std::sync::atomic::Ordering::Release);
        gate.admin_effect_release.add_permits(1);
        let first_response = tokio::time::timeout(std::time::Duration::from_secs(5), &mut first).await.expect("first writer completion deadline").expect("first HTTP task");
        assert_eq!(first_response.status, 200, "a predecessor that releases within the binding-gate budget answers its own waiter");
        let second_response = tokio::time::timeout(std::time::Duration::from_secs(5), &mut second).await.expect("competing writer completion deadline").expect("competing HTTP task");
        assert_eq!(second_response.status, 200);
        let mut first_rows = Vec::new();
        for _ in 0..256 {
            first_rows = state.directory.admin_operation_audit_for_request(first_request).await.expect("first admitted writer audit");
            if first_rows.len() == 2 {
                break;
            }
            tokio::task::yield_now().await;
        }
        let mut second_rows = Vec::new();
        for _ in 0..256 {
            second_rows = state.directory.admin_operation_audit_for_request(second_request).await.expect("second admitted writer audit");
            if second_rows.len() == 2 {
                break;
            }
            tokio::task::yield_now().await;
        }
        assert_eq!(first_rows.len(), 2);
        assert_eq!(second_rows.len(), 2);
        assert_eq!(
            first_rows[1].fact.phase,
            "succeeded",
            "the retained first writer's terminal outcome code names which arm ended it: {} reason {:?}",
            first_rows[1].fact.outcome_code,
            first_rows[1].fact.reason_code
        );
        assert_eq!(second_rows[1].fact.phase, "succeeded", "competing writer terminal outcome {} reason {:?}", second_rows[1].fact.outcome_code, second_rows[1].fact.reason_code);
        assert!(first_rows[1].sequence < second_rows[1].sequence, "the retained first writer terminal precedes its blocked successor");
        assert_eq!(physical.query_row::<i64, _, _>("SELECT count(*) FROM hub_share_grant WHERE space_id = ?1 AND document_id = ?2", rusqlite::params![queued_scope.space_id, queued_scope.document_id], |row| row.get(0)).unwrap(), 2);

        let fenced_scope = DocumentScope::new(&space, "retained-short-admitted-deadline");
        announce_document_for_test(&state, &fenced_scope.space_id, &fenced_scope.document_id).await;
        let fenced_request = "request:retained-short-admitted-deadline";
        let fenced_body = semio_framework_pack_json::to_json_string(&AdminIntentV1::IssueDocumentShare { request_id: fenced_request.into(), scope: fenced_scope.clone(), ttl_secs: 600 });
        gate.admin_effect_pause_enabled.store(true, std::sync::atomic::Ordering::Release);
        let mut fenced = tokio::spawn({
            let authorization = authorization.clone();
            async move { raw_http_request(addr, "POST", "/admin/api/intents", &[("Authorization", &authorization), ("Content-Type", "application/json")], fenced_body.as_bytes()).await }
        });
        tokio::time::timeout(std::time::Duration::from_secs(5), gate.admin_effect_admitted.acquire()).await.expect("fenced admitted writer deadline").expect("fenced writer admitted").forget();
        let fenced_response = tokio::time::timeout(ADMIN_OPERATION_DEADLINE + std::time::Duration::from_secs(2), &mut fenced).await.expect("fenced HTTP deadline response").expect("fenced HTTP task");
        assert_eq!(fenced_response.status, 503, "the HTTP waiter expires without cancelling its admitted writer");
        assert_eq!(physical.query_row::<i64, _, _>("SELECT count(*) FROM hub_share_grant WHERE space_id = ?1 AND document_id = ?2", rusqlite::params![fenced_scope.space_id, fenced_scope.document_id], |row| row.get(0)).unwrap(), 0);
        gate.admin_effect_pause_enabled.store(false, std::sync::atomic::Ordering::Release);
        gate.admin_effect_release.add_permits(1);
        let mut fenced_rows = Vec::new();
        for _ in 0..256 {
            fenced_rows = state.directory.admin_operation_audit_for_request(fenced_request).await.expect("fenced admitted writer audit");
            if fenced_rows.len() == 2 {
                break;
            }
            tokio::task::yield_now().await;
        }
        assert_eq!(fenced_rows.len(), 2);
        assert_eq!(
            fenced_rows[1].fact.phase,
            "succeeded",
            "the expired HTTP waiter never cancels its admitted writer: {} reason {:?}",
            fenced_rows[1].fact.outcome_code,
            fenced_rows[1].fact.reason_code
        );
        assert_eq!(physical.query_row::<i64, _, _>("SELECT count(*) FROM hub_share_grant WHERE space_id = ?1 AND document_id = ?2", rusqlite::params![fenced_scope.space_id, fenced_scope.document_id], |row| row.get(0)).unwrap(), 1);
        assert_eq!(state.admin_operation_tasks.task_count(), 0);
        drop(physical);
        stop_recovery_server(state, shutdown, server).await;
    }

    #[tokio::test]
    async fn native_openable_stdio_provider_is_the_only_atomic_readiness_transition() {
        let unavailable = test_state().await;
        let unavailable_addr = spawn_server(unavailable).await;
        let unavailable_readiness = raw_http_get(unavailable_addr, "/readyz", &[]).await;
        assert_eq!(unavailable_readiness.status, 503);
        let unavailable_json: serde_json::Value = serde_json::from_slice(&unavailable_readiness.body).expect("unavailable readiness JSON");
        assert_eq!(unavailable_json["artifactAuthority"]["ready"], false);
        assert_eq!(unavailable_json["features"]["openPlan"], false);

        let providers = NativeCodecProviderSetV1::linked();
        let root = native_openable_stdio_bundle();
        let configured = configured_artifact_authority(&root, Some(&providers), &Tracer::disabled(), &StartupCancellationV1::default(), &StartupProgressCellV1::default()).await.expect("verified stdio authority").configured().expect("configured stdio authority");
        assert_eq!(configured.catalog.artifact_kind_count(), 29);
        let receipts = semio_hub_stdio_catalog::native_codec_factory_receipts().expect("actual stdio schema-qualified receipts");
        let pairs = receipts.iter().map(|receipt| (receipt.artifact_kind.as_str(), receipt.schema.as_str())).collect::<std::collections::BTreeSet<_>>();
        assert_eq!(pairs.len(), receipts.len(), "every current executable codec has one exact kind/schema identity");
        assert_eq!(pairs.len(), 30, "both original PDF standards retain their executable authority");
        assert_eq!(configured.catalog.codec_count(), pairs.len(), "all current executable kind/schema identities activate atomically");
        assert_eq!(configured.catalog.open_target_count(), 1);
        let mut ready = test_state().await;
        ready.openable_catalog = Some(configured.catalog.clone());
        ready.artifact_authority = Some(configured.authority);
        ready.readiness = Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, true, true, true, true, false, false, "trusted-catalog-never-published-in-this-data-root"));
        let ready_addr = spawn_server(ready).await;
        let readiness = raw_http_get(ready_addr, "/readyz", &[]).await;
        assert_eq!(readiness.status, 200);
        let readiness_json: serde_json::Value = serde_json::from_slice(&readiness.body).expect("ready JSON");
        assert_eq!(readiness_json["artifactAuthority"]["ready"], true);
        assert_eq!(readiness_json["features"]["openPlan"], true);
        assert_eq!(readiness_json["features"]["openPlanExchange"], true);
        let encoded = String::from_utf8(readiness.body).expect("readiness UTF-8");
        assert!(!encoded.contains("receipt"));
        assert!(!encoded.contains("factory"));
        std::fs::remove_dir_all(root).expect("remove stdio bundle fixture");
    }

    /// 📌️ The checkpoint policy is the schema's: absent bounds are the declared default the readiness body carries, each
    /// bound is a decimal within its range, anything else fails boot.
    #[test]
    fn the_checkpoint_policy_is_declared_in_readiness_and_bounded_like_its_schema() {
        assert_eq!(HubCheckpointPolicyV1::configured(None, None), Ok(HUB_CHECKPOINT_POLICY));
        assert_eq!(HubCheckpointPolicyV1::configured(Some(" 1 "), Some("4096")), Ok(HubCheckpointPolicyV1 { edits: 1, payload_bytes: 4_096 }));
        assert_eq!(HubCheckpointPolicyV1::configured(Some("1048576"), Some("1073741824")), Ok(HubCheckpointPolicyV1 { edits: 1_048_576, payload_bytes: 1_073_741_824 }));
        for (edits, bytes) in [(Some("0"), None), (Some("1048577"), None), (Some("ten"), None), (None, Some("4095")), (None, Some("1073741825")), (None, Some("-1"))] {
            assert!(HubCheckpointPolicyV1::configured(edits, bytes).is_err(), "{edits:?} {bytes:?} must fail boot");
        }
        let readiness = with_checkpoint_policy(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, true, true, true, true, false, false, "trusted-catalog-never-published-in-this-data-root"), HubCheckpointPolicyV1 { edits: 7, payload_bytes: 8_192 });
        let body = serde_json::to_value(&readiness).expect("readiness JSON");
        assert_eq!(body["features"]["checkpointPolicy"], serde_json::json!({ "edits": 7, "payloadBytes": 8192 }));
        let declared = serde_json::to_value(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, true, true, true, true, false, false, "trusted-catalog-never-published-in-this-data-root")).expect("readiness JSON");
        assert_eq!(declared["features"]["checkpointPolicy"], serde_json::json!({ "edits": 1024, "payloadBytes": 524288 }));
    }

    /// 🧮️ The policy's tally starts one Check In when a document reaches either bound, keeps tallying — but starts no second
    /// one — while it runs, and tallies documents apart.
    #[test]
    fn the_checkpoint_policy_starts_one_check_in_per_bound_and_tallies_while_it_runs() {
        let ledger = CheckpointPolicyLedgerV1::default();
        let policy = HubCheckpointPolicyV1 { edits: 4, payload_bytes: 1_000 };
        let (first, second) = (DocumentScope::new("space", "first"), DocumentScope::new("space", "second"));
        assert!(!ledger.observe(&first, 3, 10, policy));
        assert!(ledger.observe(&first, 1, 10, policy), "the fourth edit reaches the edit bound");
        assert_eq!(ledger.tally(&first), Some(CheckpointPolicyTallyV1 { edits: 0, payload_bytes: 0, running: true }));
        assert!(!ledger.observe(&first, 9, 5_000, policy), "no second policy Check In while one runs");
        assert!(ledger.observe(&second, 1, 1_000, policy), "the byte bound alone starts one, for another document");
        ledger.finish(&first);
        assert_eq!(ledger.tally(&first), Some(CheckpointPolicyTallyV1 { edits: 9, payload_bytes: 5_000, running: false }), "edits committed meanwhile stay tallied");
        assert!(ledger.observe(&first, 0, 0, policy), "the tally that grew meanwhile starts the next one");
    }

    /// 📌️ Once the edits committed since the active checkpoint reach the policy the hub checks the document in by itself, on
    /// behalf of the author whose batch reached it: the active checkpoint's baseline becomes the committed head and a cold
    /// open reads the replica fold of the ledger, byte for byte; a frontier-less hello from a seeding plan then resumes there.
    #[cfg(feature = "integration-fixtures")]
    #[tokio::test]
    async fn the_checkpoint_policy_checks_a_document_in_at_its_committed_head() {
        let mut fixture = check_in_fixture("policy").await;
        let mut readiness = (*fixture.state.readiness).clone();
        readiness.features.checkpoint_policy = HubCheckpointPolicyV1 { edits: 2, payload_bytes: 1_073_741_824 };
        fixture.state.readiness = Arc::new(readiness);
        let addr = spawn_server(fixture.state.clone()).await;
        let ledger = check_in_map_edits(&fixture, &["61", "62"]).await;
        check_in_commit(&fixture, ledger[0].clone()).await;
        let head = check_in_commit(&fixture, ledger[1].clone()).await;
        let (subject, _) = check_in_author(&fixture.state, &fixture.scope, &bearer_headers(&fixture.author.token)).await.expect("the author checks in");
        assert!(fixture.state.checkpoint_policy.observe(&fixture.scope, 2, 0, fixture.state.readiness.features.checkpoint_policy), "two committed edits reach the policy");
        let status = run_policy_check_in(fixture.state.clone(), subject, fixture.scope.clone(), fixture.handle.clone()).await.expect("the policy Check In starts");
        assert_eq!(status.phase, DocumentCheckInPhaseV1::Ready, "policy check-in ended {status:?}");
        assert_eq!(fixture.state.checkpoint_policy.tally(&fixture.scope).map(|tally| tally.running), Some(false));
        let active = fixture.state.directory.get_active_artifact_checkpoint(&fixture.scope).await.expect("active read").expect("active checkpoint");
        assert_eq!(directory::os_directory::io::binary::artifact_hash::encode_edited_artifact_frontier_v1(&active.baseline_frontier), Some(head), "the active checkpoint is the committed head");
        let cold = check_in_cold_pair(addr, &fixture).await;
        assert_eq!(cold.selection.active_checkpoint_id, active.checkpoint_id);
        assert_eq!((cold.pair().pack.clone(), cold.pair().spr.clone()), check_in_replica_pair(&fixture, &ledger).await, "the policy checkpoint is the replica fold, byte for byte");
    }

    /// 📌️ Check In folds the hub's own ledger onto the active checkpoint and publishes it: after two
    /// edits the active checkpoint's baseline is exactly the named head, its parent is genesis, and a
    /// cold open reads the pair a remote replica holds after the same ledger, byte for byte. A third
    /// edit and a second Check In advance it again from the first checkpoint.
    #[cfg(feature = "integration-fixtures")]
    #[tokio::test]
    async fn check_in_advances_the_active_checkpoint_to_the_named_head_and_cold_opens_from_it() {
        let fixture = check_in_fixture("advances").await;
        let addr = spawn_server(fixture.state.clone()).await;
        let ledger = check_in_map_edits(&fixture, &["43", "44", "45"]).await;
        assert_eq!(ledger.len(), 3, "one envelope per one-operation edit");
        check_in_commit(&fixture, ledger[0].clone()).await;
        let second = check_in_commit(&fixture, ledger[1].clone()).await;

        let (code, status) = check_in_until_terminal(addr, &fixture, &fixture.author.token, "11111111111111111111111111111111", &second).await;
        let status = status.unwrap_or_else(|| panic!("check-in refused with {code}"));
        assert_eq!(code, 202, "a fresh check-in answers accepted");
        assert_eq!(status.phase, DocumentCheckInPhaseV1::Ready, "check-in ended {status:?}");
        let ready = status.ready.clone().expect("ready names its checkpoint");
        assert_eq!(ready.baseline, second);
        assert_eq!(ready.parent_checkpoint_id, directory::os_directory::io::binary::artifact_hash::artifact_hash_hex(&fixture.genesis.checkpoint_id));
        let active = fixture.state.directory.get_active_artifact_checkpoint(&fixture.scope).await.expect("active read").expect("active checkpoint");
        assert_eq!(directory::os_directory::io::binary::artifact_hash::artifact_hash_hex(&active.checkpoint_id), ready.checkpoint_id);
        assert_eq!(directory::os_directory::io::binary::artifact_hash::encode_edited_artifact_frontier_v1(&active.baseline_frontier), Some(second.clone()), "the head advanced to the named head");
        let cold = check_in_cold_pair(addr, &fixture).await;
        assert_eq!(directory::os_directory::io::binary::artifact_hash::artifact_hash_hex(&cold.selection.active_checkpoint_id), ready.checkpoint_id, "a cold open starts from the new checkpoint");
        let expected = check_in_replica_pair(&fixture, &ledger[..2]).await;
        assert_eq!((cold.pair().pack.clone(), cold.pair().spr.clone()), expected, "the checkpoint is the replica fold of the ledger, byte for byte");

        let third = check_in_commit(&fixture, ledger[2].clone()).await;
        let (_, next) = check_in_until_terminal(addr, &fixture, &fixture.author.token, "22222222222222222222222222222222", &third).await;
        let next = next.expect("second check-in status");
        let next_ready = next.ready.clone().unwrap_or_else(|| panic!("second check-in ended {next:?}"));
        assert_eq!(next_ready.parent_checkpoint_id, ready.checkpoint_id, "the second checkpoint extends the first");
        let cold = check_in_cold_pair(addr, &fixture).await;
        assert_eq!(directory::os_directory::io::binary::artifact_hash::artifact_hash_hex(&cold.selection.active_checkpoint_id), next_ready.checkpoint_id);
        assert_eq!((cold.pair().pack.clone(), cold.pair().spr.clone()), check_in_replica_pair(&fixture, &ledger).await);
        assert_eq!(fixture.state.directory.artifact_checkpoint_count(&fixture.scope).await.expect("checkpoint count"), 3, "genesis plus exactly two check-ins");
    }

    /// ⛔️ Only an author checks in, only a committed head of this document is materialized, a head the
    /// active checkpoint already passed is stale, and checking in the active baseline again is the
    /// same checkpoint rather than a second one.
    #[cfg(feature = "integration-fixtures")]
    #[tokio::test]
    async fn check_in_refuses_stale_unknown_and_foreign_heads_and_is_author_owned() {
        let fixture = check_in_fixture("refusals").await;
        let addr = spawn_server(fixture.state.clone()).await;
        let ledger = check_in_map_edits(&fixture, &["43", "44"]).await;
        let first = check_in_commit(&fixture, ledger[0].clone()).await;
        let second = check_in_commit(&fixture, ledger[1].clone()).await;
        let route = format!("/spaces/{}/documents/{}/check-ins", fixture.scope.space_id, fixture.scope.document_id);
        let request = |request_id: &str, head: &EditedArtifactFrontierV1| DocumentCheckInV1 { schema: "semio.hub.document-check-in/v1".into(), request_id: request_id.into(), head: head.clone() }.canonical_json().expect("canonical check-in");
        let body = request("33333333333333333333333333333333", &second);
        assert_eq!(raw_http_request(addr, "POST", &route, &[("Content-Type", "application/json")], body.as_bytes()).await.status, 401, "no credential");
        let spectator = format!("Bearer {}", fixture.spectator.token);
        assert_eq!(raw_http_request(addr, "POST", &route, &[("Authorization", spectator.as_str()), ("Content-Type", "application/json")], body.as_bytes()).await.status, 403, "a spectator never writes a checkpoint");
        let author = format!("Bearer {}", fixture.author.token);
        let mut foreign = second.clone();
        foreign.document_id = "artifact-other".into();
        assert_eq!(raw_http_request(addr, "POST", &route, &[("Authorization", author.as_str()), ("Content-Type", "application/json")], request("33333333333333333333333333333333", &foreign).as_bytes()).await.status, 400, "a head of another document");
        assert_eq!(raw_http_request(addr, "POST", &route, &[("Authorization", author.as_str()), ("Content-Type", "application/json")], format!(" {body}").as_bytes()).await.status, 400, "a noncanonical body");

        let mut forged = second.clone();
        forged.chain_sha256 = format!("{}{}", if forged.chain_sha256.starts_with('a') { "b" } else { "a" }, &forged.chain_sha256[1..]);
        let (_, unknown) = check_in_until_terminal(addr, &fixture, &fixture.author.token, "44444444444444444444444444444444", &forged).await;
        assert_eq!(unknown.map(|status| (status.phase, status.refusal)), Some((DocumentCheckInPhaseV1::Failed, Some(DocumentCheckInRefusalV1::UnknownHead))), "a forged chain is not a ledger point");
        let mut beyond = second.clone();
        beyond.head_edit_ordinal += 1;
        beyond.last_commit_seq += 1;
        let (_, unknown) = check_in_until_terminal(addr, &fixture, &fixture.author.token, "55555555555555555555555555555555", &beyond).await;
        assert_eq!(unknown.map(|status| (status.phase, status.refusal)), Some((DocumentCheckInPhaseV1::Failed, Some(DocumentCheckInRefusalV1::UnknownHead))), "a head the ledger never reached");

        let (_, ready) = check_in_until_terminal(addr, &fixture, &fixture.author.token, "66666666666666666666666666666666", &second).await;
        let ready = ready.and_then(|status| status.ready).expect("the head checks in");
        let (_, stale) = check_in_until_terminal(addr, &fixture, &fixture.author.token, "77777777777777777777777777777777", &first).await;
        assert_eq!(stale.map(|status| (status.phase, status.refusal)), Some((DocumentCheckInPhaseV1::Failed, Some(DocumentCheckInRefusalV1::StaleHead))), "a head the active checkpoint passed is stale");
        let (_, again) = check_in_until_terminal(addr, &fixture, &fixture.author.token, "88888888888888888888888888888888", &second).await;
        assert_eq!(again.and_then(|status| status.ready), Some(ready), "checking in the active baseline answers the same checkpoint");
        assert_eq!(fixture.state.directory.artifact_checkpoint_count(&fixture.scope).await.expect("checkpoint count"), 2, "refusals and the repeat publish nothing");
    }

    /// 🔁️ A retried request answers its own job, the same request id naming another head conflicts,
    /// a Check In the author cancels before publication publishes nothing, and an author removed
    /// from the space mid-flight ends it `authority-changed` with nothing published.
    #[cfg(feature = "integration-fixtures")]
    #[tokio::test]
    async fn check_in_is_idempotent_per_request_cancellable_and_revoked_with_the_author() {
        let mut fixture = check_in_fixture("lifecycle").await;
        let gate = Arc::new(TestLiveGate::default());
        fixture.state.live_gate = Some(gate.clone());
        let addr = spawn_server(fixture.state.clone()).await;
        let ledger = check_in_map_edits(&fixture, &["43", "44"]).await;
        let first = check_in_commit(&fixture, ledger[0].clone()).await;
        let second = check_in_commit(&fixture, ledger[1].clone()).await;
        let route = format!("/spaces/{}/documents/{}/check-ins", fixture.scope.space_id, fixture.scope.document_id);
        let author = format!("Bearer {}", fixture.author.token);
        let post = |request_id: &'static str, head: EditedArtifactFrontierV1| {
            let route = route.clone();
            let author = author.clone();
            async move {
                let body = DocumentCheckInV1 { schema: "semio.hub.document-check-in/v1".into(), request_id: request_id.into(), head }.canonical_json().expect("canonical check-in");
                raw_http_request(addr, "POST", &route, &[("Authorization", author.as_str()), ("Content-Type", "application/json")], body.as_bytes()).await
            }
        };

        gate.check_in_pause_enabled.store(true, std::sync::atomic::Ordering::Release);
        let accepted = post("99999999999999999999999999999999", second.clone()).await;
        assert_eq!(accepted.status, 202);
        tokio::time::timeout(std::time::Duration::from_secs(10), gate.check_in_admitted.acquire()).await.expect("check-in reaches its publication gate").expect("gate admission").forget();
        let joined = post("99999999999999999999999999999999", second.clone()).await;
        assert_eq!(joined.status, 202, "a retry joins the running job");
        let joined = DocumentCheckInStatusV1::parse_canonical_json(std::str::from_utf8(&joined.body).unwrap()).expect("joined status");
        assert_eq!(joined.phase, DocumentCheckInPhaseV1::Materializing);
        assert_eq!(joined.progress.completed_units, 5, "materialized, not yet published");
        assert_eq!(post("99999999999999999999999999999999", first.clone()).await.status, 409, "one request id names one request");
        let cancelled = raw_http_request(addr, "POST", &format!("{route}/99999999999999999999999999999999/cancel"), &[("Authorization", author.as_str())], &[]).await;
        assert!(cancelled.status == 200 || cancelled.status == 202);
        gate.check_in_release.add_permits(1);
        let (_, status) = check_in_until_terminal(addr, &fixture, &fixture.author.token, "99999999999999999999999999999999", &second).await;
        assert_eq!(status.map(|status| (status.phase, status.refusal)), Some((DocumentCheckInPhaseV1::Cancelled, None)));
        assert_eq!(fixture.state.directory.artifact_checkpoint_count(&fixture.scope).await.expect("checkpoint count"), 1, "a cancelled check-in publishes nothing");

        assert_eq!(post("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", second.clone()).await.status, 202);
        tokio::time::timeout(std::time::Duration::from_secs(10), gate.check_in_admitted.acquire()).await.expect("second check-in reaches its gate").expect("gate admission").forget();
        execute_directory_command_fenced(&fixture.state, DirectoryActor { kind: DirectoryActorKind::System, id: "system:check-in-removal".into() }, DirectoryCommand::RemoveMember { space_id: fixture.scope.space_id.clone(), user_id: fixture.author.user_id.clone() }).await.expect("remove the author");
        gate.check_in_release.add_permits(1);
        let key = DocumentCheckInKey { user_id: fixture.author.user_id.clone(), space_id: fixture.scope.space_id.clone(), document_id: fixture.scope.document_id.clone(), request_id: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".into() };
        let job = fixture.state.check_ins.get(&key).expect("the revoked job is retained");
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
        while !job.is_terminal() {
            assert!(std::time::Instant::now() < deadline, "the revoked check-in never ended");
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
        assert_eq!((job.status().phase, job.status().refusal), (DocumentCheckInPhaseV1::Failed, Some(DocumentCheckInRefusalV1::AuthorityChanged)));
        assert_eq!(fixture.state.directory.artifact_checkpoint_count(&fixture.scope).await.expect("checkpoint count"), 1, "a revoked author publishes nothing");
        assert_eq!(raw_http_request(addr, "GET", &format!("{route}/aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"), &[("Authorization", author.as_str())], &[]).await.status, 403, "the removed member lost read access to the job too");
    }

    #[test]
    fn scoped_directory_socket_removal_and_delivery_have_one_total_membership_order() {
        run_socket_test(|| async {
            let mut state = test_state().await;
            let gate = Arc::new(TestLiveGate::default());
            state.live_gate = Some(gate.clone());
            let member = issue_test_session(&state, "scoped-order-target@example.com").await;
            upsert_member_for_test(&state, STUDIO, "scoped-order-target@example.com", DirectorySpaceRole::Spectator).await;
            let (order_document, _) = seed_genesis_document_for_test(&state, "scoped-order-author@example.com", STUDIO, "scoped-order-document").await;
            let addr = spawn_server(state.clone()).await;
            let authorization = format!("Bearer {}", member.token);
            let issue_path = format!("/directory/spaces/{STUDIO}/documents/{order_document}/socket-grants");

            let open = |grant: String, since: u64| {
                let url = format!("ws://{addr}/directory/spaces/{STUDIO}/documents/{order_document}/socket/v1?since={since}");
                async move {
                    let (mut socket, _) = connect_async(socket_request(&url, &grant)).await.expect("ordered scoped socket");
                    socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("ordered scoped hello");
                    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                    socket
                }
            };
            let issue = || {
                let issue_path = issue_path.clone();
                let authorization = authorization.clone();
                async move {
                    let issued = raw_http_request(addr, "POST", &issue_path, &[("Authorization", &authorization)], &[]).await;
                    assert_eq!(issued.status, 200);
                    let receipt: serde_json::Value = serde_json::from_slice(&issued.body).expect("ordered scoped grant JSON");
                    receipt["grant"].as_str().expect("ordered scoped grant").to_string()
                }
            };

            let mut removal_wins = open(issue().await, state.directory.head_seq().await.expect("removal-wins head")).await;
            gate.socket_membership_remove_enabled.store(true, std::sync::atomic::Ordering::Release);
            let mut removal = tokio::spawn({
                let state = state.clone();
                let user_id = member.user_id.clone();
                async move { execute_directory_command_fenced(&state, DirectoryActor { kind: DirectoryActorKind::System, id: "system:scoped-order-removal".into() }, DirectoryCommand::RemoveMember { space_id: STUDIO.into(), user_id }).await }
            });
            tokio::time::timeout(std::time::Duration::from_secs(2), gate.socket_membership_remove_admitted.acquire()).await.expect("removal admission deadline").expect("removal admission").forget();
            gate.socket_scoped_send_mode.store(1, std::sync::atomic::Ordering::Release);
            publish_checkpoint_for_test(&state, STUDIO, &order_document).await;
            tokio::time::timeout(SOCKET_RENDEZVOUS_HANG_GUARD, gate.socket_scoped_send_admitted.acquire()).await.expect("the removal-wins sender gate was never admitted").expect("removal-wins sender").forget();
            gate.socket_scoped_send_release.add_permits(1);
            gate.socket_membership_remove_release.add_permits(1);
            tokio::time::timeout(std::time::Duration::from_secs(2), &mut removal).await.expect("removal-wins completion deadline").expect("removal task").expect("fenced removal");
            assert_eq!(next_close_code(&mut removal_wins, false).await, 4401, "removal winning the membership gate exposes no scoped event");

            upsert_member_for_test(&state, STUDIO, "scoped-order-target@example.com", DirectorySpaceRole::Spectator).await;
            gate.socket_membership_remove_enabled.store(false, std::sync::atomic::Ordering::Release);
            gate.socket_scoped_send_mode.store(0, std::sync::atomic::Ordering::Release);
            let mut delivery_wins = open(issue().await, state.directory.head_seq().await.expect("delivery-wins head")).await;
            gate.socket_scoped_send_mode.store(2, std::sync::atomic::Ordering::Release);
            let delivered_checkpoint = publish_checkpoint_for_test(&state, STUDIO, &order_document).await;
            tokio::time::timeout(SOCKET_RENDEZVOUS_HANG_GUARD, gate.socket_scoped_send_admitted.acquire()).await.expect("the delivery-wins sender gate was never admitted").expect("delivery-wins sender").forget();
            let mut removal = tokio::spawn({
                let state = state.clone();
                let user_id = member.user_id.clone();
                async move { execute_directory_command_fenced(&state, DirectoryActor { kind: DirectoryActorKind::System, id: "system:scoped-order-delivery".into() }, DirectoryCommand::RemoveMember { space_id: STUDIO.into(), user_id }).await }
            });
            let early_removal = tokio::time::timeout(std::time::Duration::from_millis(100), &mut removal).await;
            assert!(
                early_removal.is_err(),
                "removal waits while an admitted scoped send owns the membership gate; it instead returned {:?}",
                early_removal.map(|joined| joined.map(|fenced| fenced.map(|(events, _)| events.len())))
            );
            gate.socket_scoped_send_release.add_permits(1);
            let delivered = next_directory_message(&mut delivery_wins).await;
            assert!(
                matches!(
                    &delivered,
                    DirectoryStreamMessage::Event { event }
                        if matches!(event.body, os_directory::DirectoryEventBody::ArtifactCheckpointPublished { ref checkpoint }
                            if checkpoint.scope == DocumentScope::new(STUDIO, &order_document) && checkpoint.checkpoint_id == delivered_checkpoint.checkpoint_id)
                ),
                "the admitted scoped send is the exact published checkpoint: {delivered:?}"
            );
            tokio::time::timeout(std::time::Duration::from_secs(2), removal).await.expect("delivery-wins removal deadline").expect("removal task").expect("fenced removal");
            assert_eq!(next_close_code(&mut delivery_wins, false).await, 4401, "the one admitted event precedes the terminal membership close");
        });
    }

    #[tokio::test]
    async fn socket_admin_user_gate_rejects_a_late_same_user_grant_after_batch_revoke() {
        let mut state = test_state().await;
        let gate = Arc::new(TestLiveGate::default());
        gate.socket_admin_revoke_pause_enabled.store(true, std::sync::atomic::Ordering::Release);
        state.live_gate = Some(gate.clone());
        let mut admin_headers = authorize_test_admin(&mut state, "socket-admin@example.com").await;
        admin_headers.insert(axum::http::header::CONTENT_TYPE, "application/json".parse().expect("content type"));
        let target = issue_test_session(&state, "socket-target@example.com").await;
        upsert_member_for_test(&state, STUDIO, "socket-target@example.com", DirectorySpaceRole::Author).await;
        announce_document_for_test(&state, STUDIO, "socket-admin-race").await;
        let mut revoke = tokio::spawn({
            let state = state.clone();
            let user_id = target.user_id.clone();
            async move {
                let intent = AdminIntentV1::RevokeUserSessions { request_id: "request:socket-admin-revoke".into(), user_id, reason_code: "test-revoke".into() };
                let body = Bytes::from(semio_framework_pack_json::to_json_string(&intent));
                admin_intents(admin_headers, loopback_peer(), State(state), body).await
            }
        });
        tokio::time::timeout(SOCKET_RENDEZVOUS_HANG_GUARD, gate.socket_admin_revoke_admitted.acquire()).await.expect("the admin revoke gate was never admitted").expect("admin gate").forget();
        let mut issue = tokio::spawn({
            let state = state.clone();
            let token = target.token.clone();
            async move { issue_document_socket_grant_fixture(Path((STUDIO.to_string(), "socket-admin-race".to_string())), bearer_headers(&token), State(state)).await }
        });
        assert!(tokio::time::timeout(std::time::Duration::from_millis(100), &mut issue).await.is_err(), "same-user grant waits behind batch revoke");
        gate.socket_admin_revoke_release.add_permits(1);
        let (status, receipt) = tokio::time::timeout(std::time::Duration::from_secs(2), &mut revoke).await.expect("bounded admin revoke").expect("admin task").expect("admin response");
        assert_eq!(status, StatusCode::OK);
        assert_eq!(receipt.0.state, AdminIntentStateV1::Succeeded);
        let late_issue = tokio::time::timeout(std::time::Duration::from_secs(2), issue).await.expect("bounded late issue").expect("issue task");
        assert!(matches!(late_issue, Err(StatusCode::UNAUTHORIZED)), "revoked session cannot mint");
    }
}
