
#[cfg(test)]
use protocol::{AckStage, ActorId, ApplyOutcome, ClientFrame, Lane, MutationEnvelope, ServerFrame};
#[cfg(test)]
use directory::os_directory::{AdminIntentStateV1, AdminIntentV1, DOCUMENT_OPEN_PLAN_MAX_TTL_MS, DirectoryActor, DirectoryActorKind, DirectoryCommand, DirectoryEvent, DirectorySpaceRole, DirectorySpaceVisibility, DirectoryStreamMessage, DocumentDescriptor, DocumentExecutionTargetLeaseFieldsV1, DocumentOpenArtifactV1, DocumentOpenGrantV1, DocumentOpenIntentV1, DocumentOpenParentDialectV1, DocumentOpenPlanErrorCodeV1, DocumentOpenPlanV1, DocumentPlanSocketGrantIntentV1};
#[cfg(test)]
use axum::body::Bytes;
#[cfg(any(test, feature = "testkit"))]
use semio_hub_ledgers::open_plan::{DOCUMENT_OPEN_PLAN_DECODE_WIPE_OBSERVATIONS, DOCUMENT_OPEN_PLAN_DECODE_WIPE_OBSERVING, DocumentOpenPlanDecodeWipeObservationV1};
#[cfg(test)]
use axum::extract::{OriginalUri, Path, State};
#[cfg(test)]
use axum::http::StatusCode;
#[cfg(any(test, feature = "testkit"))]
use semio_hub_route_models::state::{TestDocumentOpenCatalog, TestDocumentOpenPlanIssueGate, TestLiveGate};
#[cfg(any(test, feature = "testkit"))]
use semio_hub_session_routes::access_grants::issue_document_socket_grant_fixture;
#[cfg(test)]
use directory::os_directory;
use semio_hub_directory_contract::model::{AdminEffectCommitV1, AuthSessionIssue, AuthSessionKind, SpaceRole};
use std::sync::Arc;
use semio_hub_artifact_creation::ArtifactCreationActorV1;
use semio_hub_ledgers::open_plan::{DOCUMENT_OPEN_PLAN_BINDING_CAPACITY, DOCUMENT_OPEN_PLAN_LEDGER_CAPACITY, DocumentOpenPlanAuthorityV1, DocumentOpenPlanCapabilityV1, DocumentOpenPlanLedgerV1, DocumentOpenPlanStateV1};
use semio_hub_ledgers::access_grants::{DOCUMENT_OPEN_PLAN_EXCHANGE_REQUEST_MAX_BYTES, DOCUMENT_OPEN_PLAN_REQUEST_MAX_BYTES, document_open_checkpoint, socket_actor_id};
use semio_hub_runtime::support::{DirectoryJson, now_ms};
use semio_hub_route_testkit::{DocumentOpenPlanLedgerFixture, LawHangWatchdogV1, STUDIO, TEST_ARTIFACT_PARENT_DIALECT_KIND, TEST_EXECUTION_TARGET_COMPONENT_BYTES, TEST_EXECUTION_TARGET_DESCRIPTOR_BYTES, announce_document_for_test, artifact_document_id_for_test, authorize_test_admin, bearer_headers, client_binary, create_space_for_test, document_descriptor_for_test, document_open_catalog_for_descriptor, document_open_catalog_for_descriptor_with_generation, document_open_plan_test_authority, document_socket_request, install_document_open_catalog_for_test, issue_and_exchange_document_open_plan_for_test, issue_test_session, loopback_peer, next_close_code, next_close_without_authority, next_directory_message, next_server_frame, next_server_frame_at, post_directory_command_for_test, publish_checkpoint_for_test, publish_genesis_checkpoint_for_test, raw_http_get, raw_http_request, raw_http_request_transport, run_socket_test, sample_envelope, seed_author_token, seed_genesis_for_document_for_test, socket_hello, socket_request, spawn_server, test_state, test_state_with_capacity, upsert_member_for_test};
use semio_hub_route_models::document_socket::{DocumentPlanBootstrapV1, HUB_CATCH_UP_ORIGIN};
use directory::os_directory::{DocumentScope, same_lease_fields_v1};
use semio_hub_runtime::readiness::{HubMode, hub_readiness};
use semio_hub_state::HubState;
use semio_hub_auth::rate_limit::{HubStreamLimiterV1, RateLimitRefusalV1, RateLimitSubjectV1, StreamLimitClassV1};
use semio_hub_ledgers::socket_grants::{SESSION_PROTOCOL_V1, SOCKET_GRANT_BINDING_PENDING_CAPACITY, SocketAudienceV1, SocketBindingKeyV1, SocketBindingValidityV1, SocketGrantKeyV1, SocketGrantLedgerErrorV1, SocketGrantLedgerV1, SocketSubjectV1};
use semio_hub_directory_contract::{SessionCapability, SocketGrantCapability, identity_subject_digest};
use semio_framework_hash::Sha256;
use futures::StreamExt;
use protocol::ArtifactId as WireArtifactId;
use tokio_tungstenite::tungstenite::Message as WsMessage;
use semio_hub_route_models::admin::{admin_effect_receipt_claim, admin_intent_digest};
use semio_hub_document_routes::admin::{admin_intents, execute_admin_intent};
use semio_hub_session_routes::guard::authenticate_admin_principal;
use semio_hub_session_routes::access_grants::{authenticate_document_credential, document_execution_target_selection, issue_directory_socket_grant, issue_document_open_plan, issue_document_open_plan_inner};
use tokio_tungstenite::connect_async;
use semio_hub_document_routes::document_socket::{consume_document_socket_grant, document_plan_bootstrap_current, document_plan_socket_validity};
use semio_hub_runtime::presence::{db_artifact_id, document_scope_key_v1};
use semio_hub_route_models::directory_events::directory_message_matches_scope;
use semio_hub_session_routes::session_agent::post_session_sign_out;
async fn publish_openable_document_for_test(state: &HubState, token: &str, space_id: &str, document_id: &str) -> (DocumentDescriptor, os_directory::ArtifactCheckpoint) {
    let pack = b"document-open-genesis-pack";
    let spr = b"document-open-genesis-spr";
    let mut descriptor = document_descriptor_for_test(space_id, document_id);
    descriptor.bootstrap_snapshot_hash = os_directory::io::binary::artifact_hash::hex_lower(&Sha256::digest(pack));
    let session = state.directory.authenticate_session(&SessionCapability::parse(token).expect("document-open author capability")).await.expect("document-open author session read").expect("document-open author session");
    let actor = ArtifactCreationActorV1 { user_id: session.user_id, session_id: session.id, authorization_generation: session.authorization_generation };
    let parent_dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: TEST_ARTIFACT_PARENT_DIALECT_KIND.into(), standard: "1".into(), subset: "*".into() };
    let checkpoint = publish_genesis_checkpoint_for_test(state, actor, "66".repeat(32), parent_dialect, descriptor.clone(), pack, spr).await;
    (descriptor, checkpoint)
}

fn document_open_plan_secret(index: u32) -> [u8; 32] {
    let mut secret = [0u8; 32];
    secret[28..].copy_from_slice(&index.to_be_bytes());
    secret
}

fn document_open_plan_authority_for_scope(base: &DocumentOpenPlanAuthorityV1, binding: u32, document: u32) -> DocumentOpenPlanAuthorityV1 {
    let mut authority = base.clone();
    authority.scope.document_id = format!("document-{document}");
    authority.descriptor.document_id = authority.scope.document_id.clone();
    authority.descriptor_digest_v1 = os_directory::io::binary::artifact_hash::hex_lower(&os_directory::io::binary::descriptor_digest::descriptor_digest_v1(&authority.descriptor).expect("scoped descriptor digest").0);
    authority.checkpoint.descriptor_digest_v1 = authority.descriptor_digest_v1.clone();
    authority.checkpoint.baseline_frontier.document_id = authority.scope.document_id.clone();
    authority.subject = SocketSubjectV1::Session {
        session_id: format!("open-plan-session-{binding}"),
        user_id: format!("open-plan-user-{binding}"),
        authorization_generation: authority.revalidation.session_generation.expect("session generation"),
        role: Some(SpaceRole::Author),
        expires_at_ms: i64::MAX,
        session_kind: AuthSessionKind::External,
        device_instance_id: format!("open-plan-device-{binding}"),
    };
    authority
}

async fn document_open_plan_authority_for_session(state: &HubState, fixture: &DocumentOpenPlanLedgerFixture, token: &str, scope: DocumentScope) -> DocumentOpenPlanAuthorityV1 {
    let capability = SessionCapability::parse(token).expect("session capability");
    let session = state.directory.authenticate_session(&capability).await.expect("session lookup").expect("active session");
    let role = state.directory.get_role(&scope.space_id, &session.user_id).await.expect("role lookup").expect("space member");
    let descriptor = state.directory.get_document_descriptor(&scope).await.expect("descriptor lookup").expect("announced descriptor");
    let mut authority = document_open_plan_test_authority(fixture);
    authority.scope = scope;
    authority.descriptor = descriptor;
    authority.descriptor_digest_v1 = os_directory::io::binary::artifact_hash::hex_lower(&os_directory::io::binary::descriptor_digest::descriptor_digest_v1(&authority.descriptor).expect("descriptor digest").0);
    authority.package.plugin_id = authority.descriptor.owner.plugin_id.clone();
    authority.package.package_id = authority.descriptor.owner.package_id.clone();
    authority.package.version = authority.descriptor.owner.version.clone();
    authority.package.component_sha256 = authority.descriptor.owner.package_hash.clone();
    authority.artifact.kind = authority.descriptor.artifact_kind.clone();
    authority.artifact.schema = authority.descriptor.artifact_schema.clone();
    authority.artifact.pack_schema_hash = authority.descriptor.pack_schema_hash.clone();
    if let Some(catalog) = &state.openable_catalog {
        let selected = catalog.resolve_document_open(&authority.descriptor, None, matches!(role, SpaceRole::Author)).expect("test catalog selection");
        authority.catalog.generation_id = catalog.generation_id().into();
        authority.package = selected.package;
        authority.artifact = selected.artifact;
        authority.parent_dialect = selected.parent_dialect;
        authority.surface = selected.surface;
        authority.browser_actor = selected.browser_actor;
        authority.grant = selected.grant;
    }
    authority.checkpoint = document_open_checkpoint(state.directory.get_active_artifact_checkpoint(&authority.scope).await.expect("checkpoint lookup").expect("committed document checkpoint"));
    authority.grant.write = matches!(role, SpaceRole::Author);
    let directory_revision = state.directory.head_seq().await.expect("directory revision");
    authority.revalidation.directory_revision = directory_revision;
    authority.revalidation.membership_generation = directory_revision;
    authority.revalidation.session_generation = Some(session.authorization_generation);
    authority.revalidation.share_generation = None;
    authority.subject = SocketSubjectV1::Session { session_id: session.id, user_id: session.user_id, authorization_generation: session.authorization_generation, role: Some(role), expires_at_ms: session.expires_at, session_kind: session.session_kind, device_instance_id: session.device_instance_id };
    authority.server_actor_id = socket_actor_id(&session.secret_digest, true);
    authority.validate().expect("authenticated route authority");
    authority
}

#[test]
fn document_open_plan_ledger_is_digest_only_bounded_single_use_revalidated_and_restart_scoped() {
    let fixture: DocumentOpenPlanLedgerFixture = semio_framework_pack_json::from_json_str(semio_hub_testing::hub_source("../🧰️framework/🛍️products/💻️os/🧪️tests/🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("document open plan fixture");
    let authority = document_open_plan_test_authority(&fixture);
    for (field, value) in [("artifactKind", String::new()), ("artifactKind", " s.foreign.document".to_owned()), ("standard", String::new()), ("subset", String::new()), ("standard", "\u{85}".to_owned()), ("subset", " * ".to_owned()), ("standard", "🌊".repeat(65))] {
        let mut hostile = authority.clone();
        match field {
            "artifactKind" => hostile.parent_dialect.artifact_kind = value,
            "standard" => hostile.parent_dialect.standard = value,
            "subset" => hostile.parent_dialect.subset = value,
            _ => unreachable!(),
        }
        assert_eq!(hostile.validate(), Err(DocumentOpenPlanErrorCodeV1::Stale), "invalid private parent {field}");
    }
    let ledger = Arc::new(DocumentOpenPlanLedgerV1::default());
    let secret = std::array::from_fn(|index| u8::try_from(index + 1).expect("fixture secret byte"));
    let public = ledger.issue_with_capability(authority.clone(), fixture.now_ms, fixture.now_ms + DOCUMENT_OPEN_PLAN_MAX_TTL_MS, DocumentOpenPlanCapabilityV1::from_secret(secret)).expect("issue fixture plan");
    assert_eq!(public.receipt, "open.v1.AQIDBAUGBwgJCgsMDQ4PEBESExQVFhcYGRobHB0eHyA");
    assert_eq!(public.parent_dialect, fixture.valid_plan.parent_dialect);
    let mut public_parent_kind = public.clone();
    public_parent_kind.parent_dialect.artifact_kind.insert(0, ' ');
    assert_eq!(public_parent_kind.validate(fixture.now_ms), Err(DocumentOpenPlanErrorCodeV1::Denied));
    let mut public_parent_two_space = public.clone();
    public_parent_two_space.parent_dialect.artifact_kind = "s.note.note".into();
    public_parent_two_space.artifact.kind = "2d.note".into();
    assert_eq!(public_parent_two_space.validate(fixture.now_ms), Ok(()));
    let mut public_parent_control = public.clone();
    public_parent_control.parent_dialect.standard.push('\u{85}');
    assert_eq!(public_parent_control.validate(fixture.now_ms), Err(DocumentOpenPlanErrorCodeV1::Denied));
    let expected_digest = [0x5d, 0x05, 0xc0, 0xd4, 0x09, 0x43, 0xe9, 0xab, 0xd9, 0x66, 0x33, 0xca, 0x62, 0xfe, 0x36, 0x2b, 0x58, 0xc6, 0x6e, 0xa8, 0x5a, 0xe0, 0xa0, 0xba, 0x1f, 0x02, 0x48, 0x8e, 0x38, 0xc0, 0xd3, 0x24];
    {
        let inner = ledger.inner.lock().expect("open plan ledger");
        let record = inner.records.get(&expected_digest).expect("digest-only record");
        assert_eq!(record.receipt_digest, expected_digest);
        assert_eq!(record.issued_at_ms, fixture.now_ms);
        assert_eq!(record.socket_grant_selector, None);
    }

    let barrier = Arc::new(std::sync::Barrier::new(9));
    let exchange_at = fixture.now_ms + 1;
    let attempts = (0..8)
        .map(|_| {
            let ledger = ledger.clone();
            let barrier = barrier.clone();
            let receipt = public.receipt.clone();
            let authority = authority.clone();
            std::thread::spawn(move || {
                barrier.wait();
                ledger.exchange(&receipt, &authority, exchange_at, "socket-selector").is_ok()
            })
        })
        .collect::<Vec<_>>();
    barrier.wait();
    assert_eq!(attempts.into_iter().map(|attempt| attempt.join().expect("exchange race")).filter(|won| *won).count(), 1);
    assert_eq!(ledger.exchange(&public.receipt, &authority, fixture.now_ms + 2, "socket-selector-2"), Err(DocumentOpenPlanErrorCodeV1::AlreadyConsumed));
    assert_eq!(DocumentOpenPlanLedgerV1::default().exchange(&public.receipt, &authority, fixture.now_ms + 2, "socket-selector"), Err(DocumentOpenPlanErrorCodeV1::Denied));

    let mismatch_ledger = DocumentOpenPlanLedgerV1::default();
    let mismatch = mismatch_ledger.issue_with_capability(authority.clone(), fixture.now_ms, fixture.now_ms + 100, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(10))).expect("issue mismatch plan");
    let mut foreign = authority.clone();
    foreign.catalog.generation_id = "77".repeat(32);
    assert_eq!(mismatch_ledger.exchange(&mismatch.receipt, &foreign, fixture.now_ms + 1, "socket-selector"), Err(DocumentOpenPlanErrorCodeV1::Stale));
    let mut foreign_parent = authority.clone();
    foreign_parent.parent_dialect.standard = "2".into();
    assert_eq!(mismatch_ledger.exchange(&mismatch.receipt, &foreign_parent, fixture.now_ms + 2, "socket-selector"), Err(DocumentOpenPlanErrorCodeV1::Stale));
    assert_eq!(mismatch_ledger.exchange(&mismatch.receipt, &authority, fixture.now_ms + 100, "socket-selector"), Err(DocumentOpenPlanErrorCodeV1::Expired));

    let replacement_ledger = DocumentOpenPlanLedgerV1::default();
    let first = replacement_ledger.issue_with_capability(authority.clone(), fixture.now_ms, fixture.now_ms + 100, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(11))).expect("first outstanding plan");
    let second = replacement_ledger.issue_with_capability(authority.clone(), fixture.now_ms + 1, fixture.now_ms + 101, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(12))).expect("replacement outstanding plan");
    assert_eq!(replacement_ledger.exchange(&first.receipt, &authority, fixture.now_ms + 2, "socket-selector"), Err(DocumentOpenPlanErrorCodeV1::Stale));
    replacement_ledger.invalidate_receipt(&second.receipt).expect("cancel after publication");
    assert_eq!(replacement_ledger.exchange(&second.receipt, &authority, fixture.now_ms + 2, "socket-selector"), Err(DocumentOpenPlanErrorCodeV1::Stale));

    let replacement_expiry_ledger = DocumentOpenPlanLedgerV1::default();
    let replaced_a = replacement_expiry_ledger.issue_with_capability(authority.clone(), fixture.now_ms, fixture.now_ms + 100, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(14))).expect("replacement A");
    let replaced_b = replacement_expiry_ledger.issue_with_capability(authority.clone(), fixture.now_ms + 1, fixture.now_ms + 200, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(15))).expect("replacement B");
    let replaced_c =
        replacement_expiry_ledger.issue_with_capability(authority.clone(), fixture.now_ms + 101, fixture.now_ms + 201, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(16))).expect("replacement C after A expiry sweep");
    assert_eq!(replacement_expiry_ledger.exchange(&replaced_a.receipt, &authority, fixture.now_ms + 102, "socket-a"), Err(DocumentOpenPlanErrorCodeV1::Denied));
    assert_eq!(replacement_expiry_ledger.exchange(&replaced_b.receipt, &authority, fixture.now_ms + 102, "socket-b"), Err(DocumentOpenPlanErrorCodeV1::Stale));
    assert!(replacement_expiry_ledger.exchange(&replaced_c.receipt, &authority, fixture.now_ms + 102, "socket-c").is_ok());

    let revoke_ledger = DocumentOpenPlanLedgerV1::default();
    let revoked = revoke_ledger.issue_with_capability(authority.clone(), fixture.now_ms, fixture.now_ms + 100, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(13))).expect("revocable plan");
    assert_eq!(revoke_ledger.invalidate_binding(&authority.subject.binding()), 1);
    assert_eq!(revoke_ledger.exchange(&revoked.receipt, &authority, fixture.now_ms + 1, "socket-selector"), Err(DocumentOpenPlanErrorCodeV1::Stale));

    let share_scope = authority.scope.clone();
    let mut share = authority.clone();
    share.subject = SocketSubjectV1::Share { share_id: "share-plan".into(), selector: "share-selector".into(), scope: share_scope, expires_at_ms: i64::MAX };
    share.revalidation.session_generation = None;
    share.revalidation.share_generation = Some(1);
    share.surface.role = directory::os_directory::DocumentOpenSurfaceRoleV1::Viewer;
    share.grant.write = false;
    assert!(DocumentOpenPlanLedgerV1::default().issue(share.clone(), fixture.now_ms, fixture.now_ms + 100).is_ok());
    share.grant.write = true;
    assert_eq!(DocumentOpenPlanLedgerV1::default().issue(share, fixture.now_ms, fixture.now_ms + 100), Err(DocumentOpenPlanErrorCodeV1::Stale));

    let mut beyond_binding = authority.clone();
    if let SocketSubjectV1::Session { expires_at_ms, .. } = &mut beyond_binding.subject {
        *expires_at_ms = i64::try_from(fixture.now_ms + 50).expect("binding expiry");
    }
    assert_eq!(DocumentOpenPlanLedgerV1::default().issue_with_capability(beyond_binding, fixture.now_ms, fixture.now_ms + 100, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(22)),), Err(DocumentOpenPlanErrorCodeV1::Denied));

    let binding_bounded = DocumentOpenPlanLedgerV1::default();
    for document in 0..DOCUMENT_OPEN_PLAN_BINDING_CAPACITY {
        let scoped = document_open_plan_authority_for_scope(&authority, 1, document as u32);
        binding_bounded.issue_with_capability(scoped, fixture.now_ms, fixture.now_ms + 100, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(100 + document as u32))).expect("within binding capacity");
    }
    let overflow_scope = document_open_plan_authority_for_scope(&authority, 1, DOCUMENT_OPEN_PLAN_BINDING_CAPACITY as u32);
    assert_eq!(binding_bounded.issue_with_capability(overflow_scope, fixture.now_ms, fixture.now_ms + 100, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(500))), Err(DocumentOpenPlanErrorCodeV1::DeadlineExceeded));

    let globally_bounded = DocumentOpenPlanLedgerV1::default();
    for index in 0..DOCUMENT_OPEN_PLAN_LEDGER_CAPACITY {
        let scoped = document_open_plan_authority_for_scope(&authority, (index / DOCUMENT_OPEN_PLAN_BINDING_CAPACITY) as u32, index as u32);
        globally_bounded.issue_with_capability(scoped, fixture.now_ms, fixture.now_ms + 100, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(1_000 + index as u32))).expect("within global capacity");
    }
    let overflow = document_open_plan_authority_for_scope(&authority, 99, DOCUMENT_OPEN_PLAN_LEDGER_CAPACITY as u32);
    assert_eq!(globally_bounded.issue_with_capability(overflow, fixture.now_ms, fixture.now_ms + 100, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(10_000))), Err(DocumentOpenPlanErrorCodeV1::DeadlineExceeded));
    assert!(matches!(DocumentOpenPlanCapabilityV1::parse("open.v1.AQIDBAUGBwgJCgsMDQ4PEBESExQVFhcYGRobHB0eHyB"), Err(DocumentOpenPlanErrorCodeV1::Denied)));
}

#[test]
fn document_open_plan_receipt_exchange_admits_one_exact_bounded_secret_free_socket_grant() {
    let fixture: DocumentOpenPlanLedgerFixture = semio_framework_pack_json::from_json_str(semio_hub_testing::hub_source("../🧰️framework/🛍️products/💻️os/🧪️tests/🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("document open plan fixture");
    let authority = document_open_plan_test_authority(&fixture);
    let plans = Arc::new(DocumentOpenPlanLedgerV1::default());
    let sockets = Arc::new(SocketGrantLedgerV1::default());
    let public = plans.issue_with_capability(authority.clone(), fixture.now_ms, fixture.now_ms + 100, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(30))).expect("route-inaccessible plan fixture");

    let barrier = Arc::new(std::sync::Barrier::new(9));
    let attempts = (0..8)
        .map(|_| {
            let plans = plans.clone();
            let sockets = sockets.clone();
            let barrier = barrier.clone();
            let authority = authority.clone();
            let receipt = public.receipt.clone();
            std::thread::spawn(move || {
                barrier.wait();
                plans.exchange_to_socket_grant(&receipt, &authority, fixture.now_ms + 1, sockets.as_ref())
            })
        })
        .collect::<Vec<_>>();
    barrier.wait();
    let outcomes = attempts.into_iter().map(|attempt| attempt.join().expect("plan exchange race")).collect::<Vec<_>>();
    assert_eq!(outcomes.iter().filter(|outcome| outcome.is_ok()).count(), 1);
    assert_eq!(outcomes.iter().filter(|outcome| matches!(outcome, Err(DocumentOpenPlanErrorCodeV1::AlreadyConsumed))).count(), 7);
    let response = outcomes.into_iter().find_map(Result::ok).expect("one socket grant response");
    assert_eq!(response.schema, "semio.hub.document-socket-grant/v1");
    assert_eq!(response.protocol, SESSION_PROTOCOL_V1);
    assert_eq!(response.actor_id, authority.server_actor_id);
    assert_eq!(response.expires_at_ms, i64::try_from(fixture.now_ms + 100).expect("fixture expiry"));
    let encoded_response = serde_json::to_string(&response).expect("socket grant response encodes");
    assert_eq!(serde_json::from_str::<serde_json::Value>(&encoded_response).expect("receipt json").as_object().expect("receipt object").keys().cloned().collect::<Vec<_>>(), ["actorId", "expiresAtMs", "protocol", "schema"]);
    assert!(!encoded_response.contains(&public.receipt));
    assert!(!encoded_response.contains(&authority.descriptor_digest_v1));
    assert!(!encoded_response.contains(&authority.package.component_sha256));
    assert!(!encoded_response.contains(&authority.scope.document_id));
    assert!(!encoded_response.contains("receipt"));
    assert!(!encoded_response.contains("socket.v1."), "a document socket grant carries no capability");
    let audience = SocketAudienceV1::Document(authority.scope.clone());
    let pending = sockets.pending_document_binding(&audience, &authority.subject.binding(), i64::try_from(fixture.now_ms + 2).expect("fixture time")).expect("exact pending document grant");
    assert_eq!(pending.key, SocketGrantKeyV1::CredentialBinding);
    assert_eq!(pending.actor_id, authority.server_actor_id);
    assert_eq!(pending.subject, authority.subject);
    assert_eq!(pending.document_plan.as_deref(), Some(&authority));
    assert_eq!(pending.expires_at_ms, response.expires_at_ms);
    assert_eq!(sockets.inner.lock().expect("socket ledger").records.len(), 1);
    let plan_digest = DocumentOpenPlanCapabilityV1::parse(&public.receipt).expect("plan receipt parses").digest();
    let plan_record = plans.inner.lock().expect("plan ledger").records.get(&plan_digest).expect("plan record").clone();
    assert_eq!(plan_record.state, DocumentOpenPlanStateV1::Consumed);
    assert_eq!(plan_record.socket_grant_selector.as_deref(), Some(pending.selector.as_str()));

    let capacity_plans = DocumentOpenPlanLedgerV1::default();
    let capacity_sockets = SocketGrantLedgerV1::default();
    let capacity_plan = capacity_plans.issue_with_capability(authority.clone(), fixture.now_ms, fixture.now_ms + 100, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(31))).expect("capacity plan");
    for _ in 0..SOCKET_GRANT_BINDING_PENDING_CAPACITY {
        capacity_sockets
            .admit_document(
                SocketAudienceV1::Document(authority.scope.clone()),
                authority.server_actor_id.clone(),
                authority.subject.clone(),
                i64::try_from(fixture.now_ms).expect("fixture time"),
                i64::try_from(fixture.now_ms + 1_000).expect("fixture expiry"),
                Arc::new(authority.clone()),
            )
            .expect("fill per-binding socket grant capacity");
    }
    let refused_capability = SocketGrantCapability::mint().expect("refused document capability");
    assert_eq!(
        SocketGrantLedgerV1::default().issue(&refused_capability, SocketAudienceV1::Document(authority.scope.clone()), authority.server_actor_id.clone(), authority.subject.clone(), 1, 1_000),
        Err(SocketGrantLedgerErrorV1::Rejected),
        "a document audience never answers to a capability"
    );
    assert!(matches!(capacity_plans.exchange_to_socket_grant(&capacity_plan.receipt, &authority, fixture.now_ms + 1, &capacity_sockets), Err(DocumentOpenPlanErrorCodeV1::DeadlineExceeded)));
    let capacity_digest = DocumentOpenPlanCapabilityV1::parse(&capacity_plan.receipt).expect("capacity receipt parses").digest();
    let capacity_record = capacity_plans.inner.lock().expect("capacity plan ledger").records.get(&capacity_digest).expect("capacity plan remains").clone();
    assert_eq!(capacity_record.state, DocumentOpenPlanStateV1::Issued);
    assert_eq!(capacity_record.socket_grant_selector, None);
    capacity_sockets.invalidate_binding(authority.subject.binding());
    assert!(capacity_plans.exchange_to_socket_grant(&capacity_plan.receipt, &authority, fixture.now_ms + 2, &capacity_sockets).is_ok());
}

/// 🪪️ Every execution-target asset route re-authenticates the exact scope and role, reloads the
/// durable descriptor, and resolves the current trusted selection before any body: it accepts
/// only the bounded open intent, never a package/digest/generation/path/receipt selector, serves
/// exactly the selected bytes, and denies after a catalog rotation.
///
/// 🚫 Unauthenticated, foreign-scope, foreign-surface, query-smuggled, oversized and
/// non-JSON requests never reach a byte.
///
/// 🔁 A rotation between reads denies every subsequent body rather than mixing generations.
///
/// 🚧 An unconfigured catalog advertises nothing and serves nothing.
#[tokio::test]
async fn execution_target_asset_routes_revalidate_scope_role_descriptor_and_catalog_before_each_body() {
    let mut state = test_state().await;
    let document_id = artifact_document_id_for_test("execution-target-assets");
    let token = seed_author_token(&state).await;
    let (descriptor, _) = publish_openable_document_for_test(&state, &token, STUDIO, &document_id).await;
    let scope = DocumentScope::new(STUDIO, &document_id);
    state.openable_catalog = Some(document_open_catalog_for_descriptor(&descriptor));
    state.readiness = Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, true, true, true, true, false, false, "trusted-catalog-never-published-in-this-data-root"));
    let authorization = format!("Bearer {token}");
    let headers = [("Authorization", authorization.as_str()), ("Content-Type", "application/json")];
    let root = format!("/spaces/{STUDIO}/documents/{document_id}/execution-target");
    let intent_body = |surface: &str| {
        semio_framework_pack_json::to_json_string(&DocumentOpenIntentV1 {
            schema: "semio.hub.document-open-intent/v1".into(),
            version: 1,
            scope: scope.clone(),
            requested_surface_id: Some(surface.into()),
            client_instance_id: "client:execution-target".into(),
        })
    };
    let addr = spawn_server(state.clone()).await;

    let manifest = raw_http_request(addr, "POST", &format!("{root}/manifest"), &headers, intent_body("surface.test.editor").as_bytes()).await;
    assert_eq!(manifest.status, 200, "{}", String::from_utf8_lossy(&manifest.body));
    let manifest_text = String::from_utf8(manifest.body).expect("manifest UTF-8");
    assert!(!manifest_text.contains(&token) && !manifest_text.contains("client:execution-target") && !manifest_text.contains("sessionId") && !manifest_text.contains("receipt"));
    let fields: DocumentExecutionTargetLeaseFieldsV1 = semio_framework_pack_json::from_json_str(&manifest_text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("manifest JSON");
    fields.validate().expect("route manifest is a valid lease projection");
    assert_eq!(fields.scope, scope);
    assert_eq!(fields.catalog.generation_id, state.openable_catalog.as_ref().expect("catalog").generation_id());
    assert_eq!(fields.component.byte_length, TEST_EXECUTION_TARGET_COMPONENT_BYTES.len() as u64);
    assert_eq!(fields.descriptor.byte_length, TEST_EXECUTION_TARGET_DESCRIPTOR_BYTES.len() as u64);

    let component = raw_http_request(addr, "POST", &format!("{root}/component"), &headers, intent_body("surface.test.editor").as_bytes()).await;
    assert_eq!(component.status, 200);
    assert_eq!(component.body, TEST_EXECUTION_TARGET_COMPONENT_BYTES);

    let asset_class = StreamLimitClassV1::ExecutionTargetAsset;
    let mut ruled = state.clone();
    ruled.asset_streams = Arc::new(HubStreamLimiterV1::with_policies([(asset_class, semio_hub_auth::rate_limit::StreamLimitPolicyV1 { hub_streams: 1, principal_streams: 1, admission_wait_ms: 50, retry_after_ms: 1_000 })]));
    let ruled_addr = spawn_server(ruled.clone()).await;
    let held = ruled.asset_streams.admit(asset_class, RateLimitSubjectV1::principal("another-principal")).await.expect("the one hub-wide asset stream");
    let busy = raw_http_request(ruled_addr, "POST", &format!("{root}/component"), &headers, intent_body("surface.test.editor").as_bytes()).await;
    assert_eq!(busy.status, 429, "a component whose stream rule stays taken is the typed refusal");
    assert_eq!(serde_json::from_slice::<serde_json::Value>(&busy.body).expect("refusal JSON"), serde_json::to_value(RateLimitRefusalV1::streams(asset_class, 1_000)).unwrap());
    assert!(busy.headers.to_ascii_lowercase().contains("retry-after: 1"), "{}", busy.headers);
    assert_eq!(raw_http_request(ruled_addr, "POST", &format!("{root}/manifest"), &headers, intent_body("surface.test.editor").as_bytes()).await.status, 200, "the lease is no asset stream");
    drop(held);
    let admitted = raw_http_request(ruled_addr, "POST", &format!("{root}/component"), &headers, intent_body("surface.test.editor").as_bytes()).await;
    assert_eq!((admitted.status, admitted.body.as_slice()), (200, TEST_EXECUTION_TARGET_COMPONENT_BYTES), "a freed stream slot admits the component");
    tokio::time::timeout(std::time::Duration::from_secs(2), async {
        while ruled.asset_streams.live_streams(asset_class) != 0 {
            tokio::time::sleep(std::time::Duration::from_millis(5)).await;
        }
    })
    .await
    .expect("a finished component body frees its stream slot");
    let descriptor_body = raw_http_request(addr, "POST", &format!("{root}/descriptor"), &headers, intent_body("surface.test.editor").as_bytes()).await;
    assert_eq!(descriptor_body.status, 200);
    assert_eq!(descriptor_body.body, TEST_EXECUTION_TARGET_DESCRIPTOR_BYTES);

    let corpus: serde_json::Value = serde_json::from_str(semio_hub_testing::hub_source("../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧪️tests/🧫️fixtures/🔏️document-execution-target-lease-v1/🔣️.json")).expect("closed actor neutral corpus");
    let mut closed_selection = state.openable_catalog.as_ref().unwrap().resolve_document_open(&descriptor, Some("surface.test.editor"), true).unwrap();
    closed_selection.surface.renderer_target = os_directory::DocumentOpenRendererTargetV1::Wasm;
    let mut actor_json = corpus["plan"]["browserActor"].clone();
    actor_json["sha256"] = serde_json::json!(os_directory::io::binary::artifact_hash::hex_lower(&Sha256::digest(b"abc")));
    actor_json["sourceComponentSha256"] = serde_json::json!(closed_selection.package.component_sha256);
    actor_json["sourceDescriptorByteSha256"] = serde_json::json!(closed_selection.package.descriptor_byte_sha256);
    closed_selection.browser_actor = semio_framework_pack_json::from_json_str(&actor_json.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("closed actor fixture");
    let expected_actor = closed_selection
        .browser_actor
        .to_lease(os_directory::DocumentBrowserActorSourceV1 { component_sha256: &closed_selection.package.component_sha256, descriptor_byte_sha256: &closed_selection.package.descriptor_byte_sha256 }, "wasm", Some(3))
        .expect("closed actor lease");
    let mut closed_state = state.clone();
    closed_state.openable_catalog = Some(Arc::new(TestDocumentOpenCatalog {
        generation_id: "88".repeat(32),
        open_targets: vec![closed_selection.clone()].into_boxed_slice(),
        component: TEST_EXECUTION_TARGET_COMPONENT_BYTES.into(),
        descriptor: TEST_EXECUTION_TARGET_DESCRIPTOR_BYTES.into(),
        browser_actor: Some(Arc::from(&b"abc"[..])),
    }));
    let closed_addr = spawn_server(closed_state.clone()).await;
    let closed_manifest = raw_http_request(closed_addr, "POST", &format!("{root}/manifest"), &headers, intent_body("surface.test.editor").as_bytes()).await;
    assert_eq!(closed_manifest.status, 200);
    let closed_text = String::from_utf8(closed_manifest.body).expect("closed manifest UTF-8");
    let closed_fields: DocumentExecutionTargetLeaseFieldsV1 = semio_framework_pack_json::from_json_str(&closed_text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("closed manifest");
    closed_fields.validate().expect("closed manifest bound");
    assert_eq!(closed_fields.browser_actor, expected_actor);
    for private in ["path", "moduleUrl", "actorBytes", "receipt", "sessionId", "client:execution-target"] {
        assert!(!closed_text.contains(&format!("\"{private}\"")));
    }
    assert!(!closed_text.contains(&token));
    let closed_plan = raw_http_request(closed_addr, "POST", &format!("/spaces/{STUDIO}/documents/{document_id}/open-plan"), &headers, intent_body("surface.test.editor").as_bytes()).await;
    assert_eq!(closed_plan.status, 200);
    let closed_plan: DocumentOpenPlanV1 = semio_framework_pack_json::from_json_str(std::str::from_utf8(&closed_plan.body).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("closed plan");
    assert_eq!(closed_plan.browser_actor, closed_selection.browser_actor);
    let actor_body = raw_http_request(closed_addr, "POST", &format!("{root}/browser-actor"), &headers, intent_body("surface.test.editor").as_bytes()).await;
    assert_eq!(actor_body.status, 200);
    assert_eq!(actor_body.body, b"abc");
    let absent_actor = raw_http_request(addr, "POST", &format!("{root}/browser-actor"), &headers, intent_body("surface.test.editor").as_bytes()).await;
    assert_eq!(absent_actor.status, 503);
    assert_ne!(absent_actor.body, b"abc");
    for (selection, browser_actor) in [(closed_selection, None), (state.openable_catalog.as_ref().unwrap().resolve_document_open(&descriptor, Some("surface.test.editor"), true).unwrap(), Some(Arc::from(&b"abc"[..])))] {
        let mut invalid_state = state.clone();
        invalid_state.openable_catalog = Some(Arc::new(TestDocumentOpenCatalog {
            generation_id: "99".repeat(32),
            open_targets: vec![selection].into_boxed_slice(),
            component: TEST_EXECUTION_TARGET_COMPONENT_BYTES.into(),
            descriptor: TEST_EXECUTION_TARGET_DESCRIPTOR_BYTES.into(),
            browser_actor,
        }));
        let invalid_addr = spawn_server(invalid_state).await;
        let invalid = raw_http_request(invalid_addr, "POST", &format!("{root}/manifest"), &headers, intent_body("surface.test.editor").as_bytes()).await;
        assert_eq!(invalid.status, 503, "actor identity/body presence mismatch");
    }

    for asset in ["manifest", "component", "descriptor", "browser-actor"] {
        let route = format!("{root}/{asset}");
        let anonymous = raw_http_request(addr, "POST", &route, &[("Content-Type", "application/json")], intent_body("surface.test.editor").as_bytes()).await;
        assert_eq!(anonymous.status, 401, "{asset} served an unauthenticated caller");
        let foreign_scope = raw_http_request(addr, "POST", &format!("/spaces/{STUDIO}/documents/{document_id}-foreign/execution-target/{asset}"), &headers, intent_body("surface.test.editor").as_bytes()).await;
        assert!(foreign_scope.status == 400 || foreign_scope.status == 404, "{asset} accepted a foreign scope with status {}", foreign_scope.status);
        let foreign_surface = raw_http_request(addr, "POST", &route, &headers, intent_body("surface.foreign").as_bytes()).await;
        assert_eq!(foreign_surface.status, 503, "{asset} accepted a foreign surface");
        assert_eq!(serde_json::from_slice::<serde_json::Value>(&foreign_surface.body).expect("foreign error")["code"], "component-unavailable");
        let smuggled = raw_http_request(addr, "POST", &format!("{route}?package=semio:fixture"), &headers, intent_body("surface.test.editor").as_bytes()).await;
        assert_eq!(smuggled.status, 400, "{asset} accepted a query selector");
        let unknown_field =
            raw_http_request(addr, "POST", &route, &headers, br#"{"schema":"semio.hub.document-open-intent/v1","version":1,"scope":{"spaceId":"studio","documentId":"execution-target-assets"},"clientInstanceId":"c","componentSha256":"aa"}"#).await;
        assert_eq!(unknown_field.status, 400, "{asset} accepted a client-supplied digest selector");
        let oversized = raw_http_request(addr, "POST", &route, &headers, &vec![b'x'; 9 * 1024]).await;
        assert!(oversized.status == 400 || oversized.status == 413, "{asset} accepted an oversized body");
    }

    let mut rotated = state.clone();
    rotated.openable_catalog = Some(document_open_catalog_for_descriptor_with_generation(&descriptor, "77".repeat(32)));
    let rotated_addr = spawn_server(rotated.clone()).await;
    let rotated_manifest = raw_http_request(rotated_addr, "POST", &format!("{root}/manifest"), &headers, intent_body("surface.test.editor").as_bytes()).await;
    assert_eq!(rotated_manifest.status, 200);
    let rotated_fields: DocumentExecutionTargetLeaseFieldsV1 = semio_framework_pack_json::from_json_str(&String::from_utf8(rotated_manifest.body).expect("rotated UTF-8"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("rotated manifest");
    assert_ne!(rotated_fields.catalog.generation_id, fields.catalog.generation_id);
    assert!(!same_lease_fields_v1(&rotated_fields, &fields));

    let mut unavailable = state.clone();
    unavailable.openable_catalog = None;
    let unavailable_addr = spawn_server(unavailable).await;
    let denied = raw_http_request(unavailable_addr, "POST", &format!("{root}/component"), &headers, intent_body("surface.test.editor").as_bytes()).await;
    assert_eq!(denied.status, 503);
    assert_eq!(serde_json::from_slice::<serde_json::Value>(&denied.body).expect("catalog error")["code"], "catalog-unavailable");
}

#[tokio::test]
async fn execution_target_selection_final_fence_matches_neutral_races() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🪪️execution-target-relay-v1/🔣️.json")).expect("execution-target relay fixture");
    for vector in fixture["fences"].as_array().expect("neutral fences") {
        let mut state = test_state().await;
        let token = seed_author_token(&state).await;
        let document_id = artifact_document_id_for_test("execution-target-fence");
        let (descriptor, _) = publish_openable_document_for_test(&state, &token, STUDIO, &document_id).await;
        let scope = DocumentScope::new(STUDIO, document_id);
        state.openable_catalog = Some(document_open_catalog_for_descriptor(&descriptor));
        state.readiness = Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, true, true, true, true, false, false, "trusted-catalog-never-published-in-this-data-root"));
        let mut headers = bearer_headers(&token);
        headers.insert(axum::http::header::CONTENT_TYPE, "application/json".parse().expect("content type"));
        let gate = Arc::new(TestDocumentOpenPlanIssueGate::default());
        state.document_open_plan_issue_gate = Some(gate.clone());
        let body = Bytes::from(semio_framework_pack_json::to_json_string(&DocumentOpenIntentV1 {
            schema: "semio.hub.document-open-intent/v1".into(),
            version: 1,
            scope: scope.clone(),
            requested_surface_id: Some("surface.test.editor".into()),
            client_instance_id: "client:fence".into(),
        }));
        let task = tokio::spawn(document_execution_target_selection(scope.space_id.clone(), scope.document_id.clone(), headers, state.clone(), body));
        tokio::time::timeout(std::time::Duration::from_secs(2), gate.admitted.acquire()).await.expect("target final fence deadline").expect("target reached final fence").forget();
        match vector["mutation"].as_str().expect("mutation") {
            "unchanged" => {}
            "directory-advanced" => announce_document_for_test(&state, STUDIO, "execution-target-racing-document").await,
            "session-revoked" => {
                let capability = SessionCapability::parse(&token).expect("session capability");
                let session = state.directory.authenticate_session(&capability).await.expect("session lookup").expect("session");
                state.directory.revoke_auth_session(&session.id, "target-fence", None, "target-fence").await.expect("revoke session").expect("revoked row");
            }
            "cancelled" => task.abort(),
            _ => panic!("unknown target fence mutation"),
        }
        gate.release.add_permits(1);
        let outcome = match task.await {
            Ok(Ok((fields, assets, _))) => {
                assert_eq!(fields.scope, scope);
                let mut stream = assets.component.stream().expect("selected component asset streams");
                let mut read = Vec::new();
                while let Some(chunk) = stream.next_chunk().await {
                    read.extend_from_slice(&chunk.expect("selected component chunk verifies"));
                }
                assert_eq!(read.as_slice(), TEST_EXECUTION_TARGET_COMPONENT_BYTES);
                "selected"
            }
            Ok(Err((_, error))) if error.0.code == DocumentOpenPlanErrorCodeV1::Stale => "stale",
            Ok(Err((_, error))) if error.0.code == DocumentOpenPlanErrorCodeV1::Denied => "denied",
            Err(error) if error.is_cancelled() => "cancelled",
            _ => panic!("unexpected execution-target fence outcome"),
        };
        assert_eq!(outcome, vector["expected"].as_str().expect("expected fence outcome"));
        assert!(state.document_open_plans.inner.lock().expect("plan ledger").records.is_empty(), "asset reads never mint or consume plan receipts");
    }
}

#[tokio::test]
async fn document_open_and_execution_target_refuse_descriptor_or_index_without_genesis() {
    for indexed in [false, true] {
        let mut state = test_state().await;
        let token = seed_author_token(&state).await;
        let document_id = artifact_document_id_for_test(if indexed { "indexed-without-genesis" } else { "descriptor-without-genesis" });
        let scope = DocumentScope::new(STUDIO, &document_id);
        announce_document_for_test(&state, STUDIO, &document_id).await;
        let descriptor = state.directory.get_document_descriptor(&scope).await.expect("descriptor lookup").expect("descriptor-only document");
        if indexed {
            let session = state.directory.authenticate_session(&SessionCapability::parse(&token).expect("indexed author capability")).await.expect("indexed author session read").expect("indexed author session");
            let event = semio_hub_directory_contract::NewDirectoryEvent {
                hlc: semio_hub_directory_contract::HubClock::new().tick(),
                actor: DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#indexed-without-genesis", session.user_id) },
                space_id: Some(scope.space_id.clone()),
                user_id: Some(session.user_id),
                body: os_directory::DirectoryEventBody::DocumentIndexed {
                    scope: scope.clone(),
                    descriptor_digest_v1: os_directory::io::binary::descriptor_digest::descriptor_digest_v1(&descriptor).expect("descriptor digest"),
                    entry: os_directory::DocumentIndexEntryV1 { name: "Indexed without genesis".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: TEST_ARTIFACT_PARENT_DIALECT_KIND.into(), standard: "1".into(), subset: "*".into() } },
                },
            };
            state.directory.append_decided_events(&[event]).await.expect("corrupt indexed-only fixture");
        }
        install_document_open_catalog_for_test(&mut state, &descriptor);
        let mut headers = bearer_headers(&token);
        headers.insert(axum::http::header::CONTENT_TYPE, "application/json".parse().expect("content type"));
        let body = Bytes::from(semio_framework_pack_json::to_json_string(&DocumentOpenIntentV1 {
            schema: "semio.hub.document-open-intent/v1".into(),
            version: 1,
            scope: scope.clone(),
            requested_surface_id: Some("surface.test.editor".into()),
            client_instance_id: "client:no-genesis".into(),
        }));
        let authorization = format!("Bearer {token}");
        let raw_headers = [("Authorization", authorization.as_str()), ("Content-Type", "application/json")];
        let addr = spawn_server(state.clone()).await;
        for path in [format!("/spaces/{}/documents/{}/open-plan", scope.space_id, scope.document_id), format!("/spaces/{}/documents/{}/execution-target/manifest", scope.space_id, scope.document_id)] {
            let denied = raw_http_request(addr, "POST", &path, &raw_headers, &body).await;
            assert_eq!(denied.status, 404, "{path} must refuse a document without committed genesis");
            let error: serde_json::Value = serde_json::from_slice(&denied.body).expect("closed checkpoint error");
            assert_eq!(error["code"], "not-found");
            assert!(!denied.body.windows(TEST_EXECUTION_TARGET_COMPONENT_BYTES.len()).any(|window| window == TEST_EXECUTION_TARGET_COMPONENT_BYTES));
        }
        assert!(state.document_open_plans.inner.lock().expect("plan ledger").records.is_empty(), "pre-genesis route refusal cannot mint a plan");
        if !indexed {
            let mut no_catalog = state.clone();
            no_catalog.openable_catalog = None;
            let no_catalog_addr = spawn_server(no_catalog).await;
            let denied = raw_http_request(no_catalog_addr, "POST", &format!("/spaces/{}/documents/{}/execution-target/manifest", scope.space_id, scope.document_id), &raw_headers, &body).await;
            assert_eq!(denied.status, 404, "checkpoint absence must be decided before catalog availability");
            assert_eq!(serde_json::from_slice::<serde_json::Value>(&denied.body).expect("closed no-catalog checkpoint error")["code"], "not-found");
        }
        let Err((plan_status, DirectoryJson(plan_error))) = issue_document_open_plan_inner(scope.space_id.clone(), scope.document_id.clone(), headers.clone(), state.clone(), body.clone()).await else {
            panic!("descriptor/index without genesis minted an open plan");
        };
        assert_eq!((plan_status, plan_error.code), (StatusCode::NOT_FOUND, DocumentOpenPlanErrorCodeV1::NotFound));
        let Err((target_status, DirectoryJson(target_error))) = document_execution_target_selection(scope.space_id.clone(), scope.document_id.clone(), headers, state, body).await else {
            panic!("descriptor/index without genesis minted an execution target");
        };
        assert_eq!((target_status, target_error.code), (StatusCode::NOT_FOUND, DocumentOpenPlanErrorCodeV1::NotFound));
    }
}

#[tokio::test]
async fn document_open_plan_issue_route_is_catalog_bound_authenticated_bounded_cancel_safe_and_exchangeable() {
    let _watchdog = LawHangWatchdogV1::arm("document_open_plan_issue_route_is_catalog_bound_authenticated_bounded_cancel_safe_and_exchangeable");
    let mut state = test_state().await;
    let document_id = artifact_document_id_for_test("open-plan-issue");
    let token = seed_author_token(&state).await;
    let (descriptor, _) = publish_openable_document_for_test(&state, &token, STUDIO, &document_id).await;
    let scope = DocumentScope::new(STUDIO, &document_id);
    state.openable_catalog = Some(document_open_catalog_for_descriptor(&descriptor));
    state.readiness = Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, true, true, true, true, false, false, "trusted-catalog-never-published-in-this-data-root"));
    let authorization = format!("Bearer {token}");
    let headers = [("Authorization", authorization.as_str()), ("Content-Type", "application/json")];
    let plan_route = format!("/spaces/{STUDIO}/documents/{document_id}/open-plan");
    let grant_route = format!("/spaces/{STUDIO}/documents/{document_id}/socket-grants");
    let intent_body = |surface: &str, client: &str| {
        semio_framework_pack_json::to_json_string(&DocumentOpenIntentV1 { schema: "semio.hub.document-open-intent/v1".into(), version: 1, scope: scope.clone(), requested_surface_id: Some(surface.into()), client_instance_id: client.into() })
    };
    let grant_body = |receipt: &str| semio_framework_pack_json::to_json_string(&DocumentPlanSocketGrantIntentV1 { schema: "semio.hub.document-plan-socket-grant-intent/v1".into(), version: 1, plan_receipt: receipt.into() });
    let addr = spawn_server(state.clone()).await;
    let readiness = raw_http_get(addr, "/readyz", &[]).await;
    let readiness_json: serde_json::Value = serde_json::from_slice(&readiness.body).expect("readiness JSON");
    assert_eq!(readiness_json["features"]["openPlan"], true);
    assert_eq!(readiness_json["features"]["openPlanExchange"], true);

    let success = raw_http_request(addr, "POST", &plan_route, &headers, intent_body("surface.test.editor", "client:private").as_bytes()).await;
    assert_eq!(success.status, 200, "{}", String::from_utf8_lossy(&success.body));
    let success_text = String::from_utf8(success.body).expect("plan UTF-8");
    assert!(!success_text.contains(&token));
    assert!(!success_text.contains("client:private"));
    assert!(!success_text.contains("sessionId"));
    assert!(!success_text.contains("descriptor\""));
    let plan: DocumentOpenPlanV1 = semio_framework_pack_json::from_json_str(&success_text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("plan JSON");
    assert_eq!(plan.scope, scope);
    assert_eq!(plan.catalog.generation_id, state.openable_catalog.as_ref().expect("catalog").generation_id());
    assert_eq!(plan.parent_dialect, DocumentOpenParentDialectV1 { artifact_kind: TEST_ARTIFACT_PARENT_DIALECT_KIND.into(), standard: "1".into(), subset: "*".into() });
    assert_eq!(plan.surface.surface_id, "surface.test.editor");
    assert!(plan.grant.write);
    assert!(plan.expires_at_unix_ms.saturating_sub(u64::try_from(now_ms()).expect("time")) <= DOCUMENT_OPEN_PLAN_MAX_TTL_MS);
    let exchange = raw_http_request(addr, "POST", &grant_route, &headers, grant_body(&plan.receipt).as_bytes()).await;
    assert_eq!(exchange.status, 200);
    let exchange_json: serde_json::Value = serde_json::from_slice(&exchange.body).expect("exchange JSON");
    assert_eq!(exchange_json["schema"], "semio.hub.document-socket-grant/v1");
    assert!(exchange_json.get("grant").is_none(), "a document socket grant carries no capability");
    assert!(!String::from_utf8(exchange.body).expect("exchange UTF-8").contains(&plan.receipt));

    let foreign_surface = raw_http_request(addr, "POST", &plan_route, &headers, intent_body("surface.foreign", "client:foreign").as_bytes()).await;
    assert_eq!(foreign_surface.status, 503);
    assert_eq!(serde_json::from_slice::<serde_json::Value>(&foreign_surface.body).expect("foreign error")["code"], "component-unavailable");
    let hostile = format!(r#"{{"schema":"semio.hub.document-open-intent/v1","version":1,"scope":{{"spaceId":"{STUDIO}","documentId":"{document_id}"}},"clientInstanceId":"client","actor":"caller"}}"#);
    assert_eq!(raw_http_request(addr, "POST", &plan_route, &headers, hostile.as_bytes()).await.status, 400);
    assert_eq!(raw_http_request(addr, "POST", &format!("{plan_route}?surface=surface.test.editor"), &headers, intent_body("surface.test.editor", "client:query").as_bytes()).await.status, 400);
    let wrong_scope = semio_framework_pack_json::to_json_string(&DocumentOpenIntentV1 {
        schema: "semio.hub.document-open-intent/v1".into(),
        version: 1,
        scope: DocumentScope::new(STUDIO, "other"),
        requested_surface_id: Some("surface.test.editor".into()),
        client_instance_id: "client:scope".into(),
    });
    assert_eq!(raw_http_request(addr, "POST", &plan_route, &headers, wrong_scope.as_bytes()).await.status, 400);
    let mut oversized = intent_body("surface.test.editor", "client:oversized").into_bytes();
    oversized.resize(DOCUMENT_OPEN_PLAN_REQUEST_MAX_BYTES + 1, b' ');
    let oversized_transport = raw_http_request_transport(addr, "POST", &plan_route, &headers, &oversized).await;
    assert!(oversized_transport.is_none_or(|response| response.status == 413));
    let oversized_request =
        axum::http::Request::builder().uri(&plan_route).header(axum::http::header::AUTHORIZATION, &authorization).header(axum::http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(oversized)).expect("oversized request");
    assert!(matches!(issue_document_open_plan(OriginalUri(plan_route.parse().expect("plan URI")), Path((STUDIO.into(), document_id.into())), State(state.clone()), oversized_request).await, Err((StatusCode::PAYLOAD_TOO_LARGE, _))));

    let issued_share = state.directory.issue_share_token(&scope, 60, "open-plan-issue-share").await.expect("share issue");
    let share_authorization = format!("Bearer {}", issued_share.capability.expose_once());
    let share_headers = [("Authorization", share_authorization.as_str()), ("Content-Type", "application/json")];
    let share_response = raw_http_request(addr, "POST", &plan_route, &share_headers, intent_body("surface.test.viewer", "client:share").as_bytes()).await;
    assert_eq!(share_response.status, 200);
    let share_plan: DocumentOpenPlanV1 = semio_framework_pack_json::from_json_str(&String::from_utf8(share_response.body).expect("share plan UTF-8"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("share plan");
    assert_eq!(share_plan.parent_dialect, plan.parent_dialect);
    assert!(!share_plan.grant.write);
    assert_eq!(share_plan.surface.role, os_directory::DocumentOpenSurfaceRoleV1::Viewer);
    assert!(share_plan.revalidation.session_generation.is_none());
    assert_eq!(share_plan.revalidation.share_generation, Some(1));
    assert_eq!(raw_http_request(addr, "POST", &grant_route, &share_headers, grant_body(&share_plan.receipt).as_bytes()).await.status, 200);

    let mut unavailable = state.clone();
    unavailable.openable_catalog = None;
    unavailable.readiness = Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, false, true, true, true, false, false, "trusted-catalog-never-published-in-this-data-root"));
    let unavailable_addr = spawn_server(unavailable).await;
    let unavailable_readiness = raw_http_get(unavailable_addr, "/readyz", &[]).await;
    let unavailable_readiness: serde_json::Value = serde_json::from_slice(&unavailable_readiness.body).expect("unavailable readiness JSON");
    assert_eq!(unavailable_readiness["features"]["openPlan"], false);
    assert_eq!(unavailable_readiness["features"]["openPlanExchange"], false);
    let unavailable_response = raw_http_request(unavailable_addr, "POST", &plan_route, &headers, intent_body("surface.test.editor", "client:unavailable").as_bytes()).await;
    assert_eq!(unavailable_response.status, 503);
    let unavailable_text = String::from_utf8(unavailable_response.body).expect("unavailable UTF-8");
    assert_eq!(serde_json::from_str::<serde_json::Value>(&unavailable_text).expect("unavailable JSON")["code"], "catalog-unavailable");
    assert!(!unavailable_text.contains("fixture"));

    let fixture: DocumentOpenPlanLedgerFixture = semio_framework_pack_json::from_json_str(semio_hub_testing::hub_source("../🧰️framework/🛍️products/💻️os/🧪️tests/🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("document open fixture");
    let mut capacity_authority = document_open_plan_authority_for_session(&state, &fixture, &token, scope.clone()).await;
    capacity_authority.catalog.generation_id = state.openable_catalog.as_ref().expect("catalog").generation_id().into();
    capacity_authority.package = state.openable_catalog.as_ref().expect("catalog").resolve_document_open(&descriptor, Some("surface.test.editor"), true).expect("selection").package;
    capacity_authority.artifact = DocumentOpenArtifactV1 { kind: descriptor.artifact_kind.clone(), schema: descriptor.artifact_schema.clone(), pack_schema_hash: descriptor.pack_schema_hash.clone() };
    capacity_authority.surface = state.openable_catalog.as_ref().expect("catalog").resolve_document_open(&descriptor, Some("surface.test.editor"), true).expect("selection").surface;
    capacity_authority.grant = DocumentOpenGrantV1 { read: true, write: true, observe: true };
    let capacity_now = u64::try_from(now_ms()).expect("capacity time");
    for index in 0..DOCUMENT_OPEN_PLAN_BINDING_CAPACITY {
        let mut authority = capacity_authority.clone();
        authority.scope.document_id = format!("capacity-{index}");
        authority.descriptor.document_id = authority.scope.document_id.clone();
        authority.descriptor_digest_v1 = os_directory::io::binary::artifact_hash::hex_lower(&os_directory::io::binary::descriptor_digest::descriptor_digest_v1(&authority.descriptor).expect("capacity digest").0);
        authority.checkpoint.descriptor_digest_v1 = authority.descriptor_digest_v1.clone();
        authority.checkpoint.baseline_frontier.document_id = authority.scope.document_id.clone();
        state.document_open_plans.issue_with_capability(authority, capacity_now, capacity_now + 10_000, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(1_000 + index as u32))).expect("fill plan capacity");
    }
    let capacity_response = raw_http_request(addr, "POST", &plan_route, &headers, intent_body("surface.test.editor", "client:capacity").as_bytes()).await;
    assert_eq!(capacity_response.status, 503);
    assert_eq!(serde_json::from_slice::<serde_json::Value>(&capacity_response.body).expect("capacity JSON")["code"], "deadline-exceeded");

    let mut cancelled = test_state().await;
    let cancelled_token = seed_author_token(&cancelled).await;
    let cancelled_document_id = artifact_document_id_for_test("open-plan-cancelled");
    let (cancelled_descriptor, _) = publish_openable_document_for_test(&cancelled, &cancelled_token, STUDIO, &cancelled_document_id).await;
    let cancelled_scope = DocumentScope::new(STUDIO, cancelled_document_id);
    cancelled.openable_catalog = Some(document_open_catalog_for_descriptor(&cancelled_descriptor));
    cancelled.readiness = Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, true, true, true, true, false, false, "trusted-catalog-never-published-in-this-data-root"));
    let mut cancelled_headers = bearer_headers(&cancelled_token);
    cancelled_headers.insert(axum::http::header::CONTENT_TYPE, "application/json".parse().expect("content type"));
    let gate = Arc::new(TestDocumentOpenPlanIssueGate::default());
    cancelled.document_open_plan_issue_gate = Some(gate.clone());
    let cancelled_body = Bytes::from(semio_framework_pack_json::to_json_string(&DocumentOpenIntentV1 {
        schema: "semio.hub.document-open-intent/v1".into(),
        version: 1,
        scope: cancelled_scope.clone(),
        requested_surface_id: Some("surface.test.editor".into()),
        client_instance_id: "client:cancelled".into(),
    }));
    let cancelled_state = cancelled.clone();
    let task = tokio::spawn(issue_document_open_plan_inner(STUDIO.into(), cancelled_scope.document_id, cancelled_headers, cancelled_state, cancelled_body));
    tokio::time::timeout(std::time::Duration::from_secs(2), gate.admitted.acquire()).await.expect("issuer publication fence deadline").expect("issuer reached publication fence").forget();
    task.abort();
    assert!(matches!(task.await, Err(error) if error.is_cancelled()));
    assert!(cancelled.document_open_plans.inner.lock().expect("cancelled ledger").records.is_empty());
}

#[tokio::test]
async fn document_open_plan_socket_consume_revalidates_surface_descriptor_catalog_revision_and_checkpoint() {
    let _watchdog = LawHangWatchdogV1::arm("document_open_plan_socket_consume_revalidates_surface_descriptor_catalog_revision_and_checkpoint");
    let mut state = test_state().await;
    let document_id = artifact_document_id_for_test("open-plan-consume");
    let token = seed_author_token(&state).await;
    let (descriptor, _) = publish_openable_document_for_test(&state, &token, STUDIO, &document_id).await;
    let scope = DocumentScope::new(STUDIO, &document_id);
    state.openable_catalog = Some(document_open_catalog_for_descriptor(&descriptor));
    state.readiness = Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, true, true, true, true, false, false, "trusted-catalog-never-published-in-this-data-root"));

    let (subject, _) = authenticate_document_credential(&state, &scope, &token).await.expect("document credential");
    let audience = SocketAudienceV1::Document(scope.clone());
    issue_and_exchange_document_open_plan_for_test(&state, &token, &scope, "client:surface").await;
    assert!(matches!(consume_document_socket_grant(&state, &subject, audience.clone(), Some("surface.test.viewer")).await, Err((StatusCode::UNAUTHORIZED, _))));
    assert!(state.socket_grants.pending_document_binding(&audience, &subject.binding(), now_ms()).is_err(), "surface substitution terminally rejects the pending grant");

    issue_and_exchange_document_open_plan_for_test(&state, &token, &scope, "client:checkpoint").await;
    let checked_in = state.socket_grants.pending_document_binding(&audience, &subject.binding(), now_ms()).expect("pending grant before the Check In");
    publish_checkpoint_for_test(&state, STUDIO, &document_id).await;
    assert_eq!(document_plan_socket_validity(&state, &checked_in, Some("surface.test.editor")).await, SocketBindingValidityV1::Active, "a Check In changes nobody's authority");
    assert_eq!(document_plan_bootstrap_current(&state, &checked_in).await, DocumentPlanBootstrapV1::Stale);
    assert!(matches!(consume_document_socket_grant(&state, &subject, audience.clone(), Some("surface.test.editor")).await, Err((StatusCode::CONFLICT, "socket-grant-stale"))));
    assert!(state.socket_grants.pending_document_binding(&audience, &subject.binding(), now_ms()).is_err(), "a checkpoint published since the exchange rejects the pending grant as stale");

    issue_and_exchange_document_open_plan_for_test(&state, &token, &scope, "client:catalog").await;
    let rotated = state.socket_grants.pending_document_binding(&audience, &subject.binding(), now_ms()).expect("pending grant before the catalog publication");
    state.openable_catalog = Some(document_open_catalog_for_descriptor_with_generation(&descriptor, "77".repeat(32)));
    assert_eq!(document_plan_socket_validity(&state, &rotated, Some("surface.test.editor")).await, SocketBindingValidityV1::Active, "a catalog generation with the same selection changes nobody's authority");
    assert_eq!(document_plan_bootstrap_current(&state, &rotated).await, DocumentPlanBootstrapV1::Stale);
    assert!(matches!(consume_document_socket_grant(&state, &subject, audience.clone(), Some("surface.test.editor")).await, Err((StatusCode::CONFLICT, "socket-grant-stale"))));
    assert!(state.socket_grants.pending_document_binding(&audience, &subject.binding(), now_ms()).is_err(), "a catalog generation published since the exchange rejects the pending grant as stale");

    state.openable_catalog = Some(document_open_catalog_for_descriptor(&descriptor));
    issue_and_exchange_document_open_plan_for_test(&state, &token, &scope, "client:exact").await;
    let pending = state.socket_grants.pending_document_binding(&audience, &subject.binding(), now_ms()).expect("exact pending grant");
    let authority = pending.document_plan.as_ref().expect("retained plan authority");
    let mut hostile_descriptor = pending.clone();
    Arc::make_mut(hostile_descriptor.document_plan.as_mut().expect("descriptor authority")).descriptor_digest_v1 = "00".repeat(32);
    assert_eq!(document_plan_socket_validity(&state, &hostile_descriptor, Some("surface.test.editor")).await, SocketBindingValidityV1::Unauthorized);
    let mut hostile_revision = pending.clone();
    Arc::make_mut(hostile_revision.document_plan.as_mut().expect("revision authority")).revalidation.directory_revision += 1;
    assert_eq!(document_plan_socket_validity(&state, &hostile_revision, Some("surface.test.editor")).await, SocketBindingValidityV1::Unauthorized);
    let mut hostile_checkpoint = pending.clone();
    Arc::make_mut(hostile_checkpoint.document_plan.as_mut().expect("checkpoint authority")).checkpoint.descriptor_digest_v1 = "00".repeat(32);
    assert_eq!(document_plan_socket_validity(&state, &hostile_checkpoint, Some("surface.test.editor")).await, SocketBindingValidityV1::Unauthorized);
    for field in ["artifactKind", "standard", "subset"] {
        let mut hostile = pending.clone();
        let dialect = &mut Arc::make_mut(hostile.document_plan.as_mut().expect("dialect authority")).parent_dialect;
        match field {
            "artifactKind" => dialect.artifact_kind.push_str(".foreign"),
            "standard" => dialect.standard.push_str("-foreign"),
            "subset" => dialect.subset.push_str("-foreign"),
            _ => unreachable!(),
        }
        assert_eq!(document_plan_socket_validity(&state, &hostile, Some("surface.test.editor")).await, SocketBindingValidityV1::Unauthorized, "foreign parent {field}");
    }
    assert_eq!(authority.scope, scope);
    let admission = consume_document_socket_grant(&state, &subject, audience, Some("surface.test.editor")).await.expect("exact current authority consumes");
    assert_eq!(admission.record.document_plan.as_deref(), Some(authority.as_ref()));
}

#[tokio::test]
async fn document_open_plan_exchange_route_is_authenticated_exact_hostile_and_single_use() {
    let _watchdog = LawHangWatchdogV1::arm("document_open_plan_exchange_route_is_authenticated_exact_hostile_and_single_use");
    let mut state = test_state().await;
    let fixture: DocumentOpenPlanLedgerFixture = semio_framework_pack_json::from_json_str(semio_hub_testing::hub_source("../🧰️framework/🛍️products/💻️os/🧪️tests/🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("document open plan fixture");
    let token = seed_author_token(&state).await;
    let document_id = artifact_document_id_for_test("open-plan-route");
    let document_id = document_id.as_str();
    let foreign_document_id = artifact_document_id_for_test("open-plan-route-foreign");
    let foreign_document_id = foreign_document_id.as_str();
    seed_genesis_for_document_for_test(&state, &token, STUDIO, document_id, "open-plan-route").await;
    seed_genesis_for_document_for_test(&state, &token, STUDIO, foreign_document_id, "open-plan-route-foreign").await;
    let route_descriptor = state.directory.get_document_descriptor(&DocumentScope::new(STUDIO, document_id)).await.expect("route descriptor").expect("route document");
    state.openable_catalog = Some(document_open_catalog_for_descriptor(&route_descriptor));
    state.readiness = Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, true, true, true, true, false, false, "trusted-catalog-never-published-in-this-data-root"));
    let scope = DocumentScope::new(STUDIO, document_id);
    let mut authority = document_open_plan_authority_for_session(&state, &fixture, &token, scope.clone()).await;
    let addr = spawn_server(state.clone()).await;
    let route = format!("/spaces/{STUDIO}/documents/{document_id}/socket-grants");
    let authorization = format!("Bearer {token}");
    let request_headers = [("Authorization", authorization.as_str()), ("Content-Type", "application/json")];
    let request_body = |receipt: &str| semio_framework_pack_json::to_json_string(&DocumentPlanSocketGrantIntentV1 { schema: "semio.hub.document-plan-socket-grant-intent/v1".into(), version: 1, plan_receipt: receipt.into() });
    let readiness = raw_http_get(addr, "/readyz", &[]).await;
    let readiness_json: serde_json::Value = serde_json::from_slice(&readiness.body).expect("readiness JSON");
    assert_eq!(readiness_json["features"]["openPlan"], true, "verified catalog-backed plan issuance is advertised with exchange");
    assert_eq!(readiness_json["features"]["openPlanExchange"], true);

    let now = u64::try_from(now_ms()).expect("nonnegative route time");
    let public = state.document_open_plans.issue_with_capability(authority.clone(), now, now + 10_000, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(40))).expect("private issuer fixture");
    let success = raw_http_request(addr, "POST", &route, &request_headers, request_body(&public.receipt).as_bytes()).await;
    assert_eq!(success.status, 200);
    let encoded_success = String::from_utf8(success.body.clone()).expect("success UTF-8");
    assert!(!encoded_success.contains(&public.receipt));
    assert!(!encoded_success.contains(&authority.descriptor_digest_v1));
    if let SocketSubjectV1::Session { session_id, user_id, .. } = &authority.subject {
        assert!(!encoded_success.contains(session_id));
        assert!(!encoded_success.contains(user_id));
    }
    let success_json: serde_json::Value = serde_json::from_slice(&success.body).expect("socket grant JSON");
    assert_eq!(success_json["schema"], "semio.hub.document-socket-grant/v1");
    assert_eq!(success_json["protocol"], SESSION_PROTOCOL_V1);
    assert_eq!(success_json["actorId"], authority.server_actor_id);
    assert!(success_json.get("grant").is_none(), "a document socket grant carries no capability");
    let pending = state.socket_grants.pending_document_binding(&SocketAudienceV1::Document(scope.clone()), &authority.subject.binding(), now_ms()).expect("route-bound pending grant");
    assert_eq!(pending.subject, authority.subject);
    assert_eq!(pending.document_plan.as_deref(), Some(&authority));

    let replay = raw_http_request(addr, "POST", &route, &request_headers, request_body(&public.receipt).as_bytes()).await;
    assert_eq!(replay.status, 409);
    let replay_json: serde_json::Value = serde_json::from_slice(&replay.body).expect("replay error JSON");
    assert_eq!(replay_json, serde_json::json!({ "schema": "semio.hub.document-open-plan-error/v1", "code": "already-consumed" }));
    assert!(!String::from_utf8(replay.body).expect("replay UTF-8").contains(&public.receipt));

    let foreign = issue_test_session(&state, "open-plan-route-foreign@example.com").await;
    upsert_member_for_test(&state, STUDIO, "open-plan-route-foreign@example.com", DirectorySpaceRole::Author).await;
    let current_revision = state.directory.head_seq().await.expect("current directory revision");
    authority.revalidation.directory_revision = current_revision;
    authority.revalidation.membership_generation = current_revision;
    let foreign_authorization = format!("Bearer {}", foreign.token);
    let foreign_headers = [("Authorization", foreign_authorization.as_str()), ("Content-Type", "application/json")];
    let bound = state.document_open_plans.issue_with_capability(authority.clone(), now + 1, now + 10_001, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(41))).expect("binding fixture plan");
    let wrong_binding = raw_http_request(addr, "POST", &route, &foreign_headers, request_body(&bound.receipt).as_bytes()).await;
    assert_eq!(wrong_binding.status, 401);
    let wrong_binding_json: serde_json::Value = serde_json::from_slice(&wrong_binding.body).expect("binding error JSON");
    assert_eq!(wrong_binding_json["code"], "denied");
    let correct_after_foreign = raw_http_request(addr, "POST", &route, &request_headers, request_body(&bound.receipt).as_bytes()).await;
    assert_eq!(correct_after_foreign.status, 200, "foreign authentication cannot consume the exact receipt");

    let scoped = state.document_open_plans.issue_with_capability(authority.clone(), now + 2, now + 10_002, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(42))).expect("scope fixture plan");
    let foreign_route = format!("/spaces/{STUDIO}/documents/{foreign_document_id}/socket-grants");
    let wrong_scope = raw_http_request(addr, "POST", &foreign_route, &request_headers, request_body(&scoped.receipt).as_bytes()).await;
    assert_eq!(wrong_scope.status, 401);
    assert_eq!(raw_http_request(addr, "POST", &route, &request_headers, request_body(&scoped.receipt).as_bytes()).await.status, 200, "foreign path cannot consume the exact receipt");

    let strict = state.document_open_plans.issue_with_capability(authority.clone(), now + 3, now + 10_003, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(43))).expect("strict request fixture plan");
    let hostile = format!(r#"{{"schema":"semio.hub.document-plan-socket-grant-intent/v1","version":1,"planReceipt":"{}","actor":"caller-selected"}}"#, strict.receipt);
    let unknown_field = raw_http_request(addr, "POST", &route, &request_headers, hostile.as_bytes()).await;
    assert_eq!(unknown_field.status, 400);
    assert_eq!(raw_http_request(addr, "POST", &route, &request_headers, request_body(&strict.receipt).as_bytes()).await.status, 200, "rejected unknown authority field cannot consume the receipt");

    let query_plan = state.document_open_plans.issue_with_capability(authority.clone(), now + 4, now + 10_004, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(44))).expect("query fixture plan");
    assert_eq!(raw_http_request(addr, "POST", &format!("{route}?receipt=forbidden"), &request_headers, request_body(&query_plan.receipt).as_bytes()).await.status, 400);
    assert_eq!(raw_http_request(addr, "POST", &route, &request_headers, request_body(&query_plan.receipt).as_bytes()).await.status, 200, "query rejection cannot consume the body receipt");

    let bounded = state.document_open_plans.issue_with_capability(authority.clone(), now + 5, now + 10_005, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(45))).expect("bounded request fixture plan");
    let mut oversized = request_body(&bounded.receipt).into_bytes();
    oversized.resize(DOCUMENT_OPEN_PLAN_EXCHANGE_REQUEST_MAX_BYTES + 1, b' ');
    let oversized_response = raw_http_request(addr, "POST", &route, &request_headers, &oversized).await;
    assert_eq!(oversized_response.status, 413);
    assert_eq!(serde_json::from_slice::<serde_json::Value>(&oversized_response.body).expect("bounded error JSON")["code"], "denied");
    assert_eq!(raw_http_request(addr, "POST", &route, &request_headers, request_body(&bounded.receipt).as_bytes()).await.status, 200, "oversized body cannot consume the receipt");

    let issued_share = state.directory.issue_share_token(&scope, 60, "open-plan-route-share").await.expect("share issue");
    let share_token = issued_share.capability.expose_once();
    let mut share_authority = authority.clone();
    share_authority.subject = SocketSubjectV1::Share { share_id: issued_share.record.id, selector: issued_share.record.selector, scope: scope.clone(), expires_at_ms: issued_share.record.expires_at };
    share_authority.revalidation.session_generation = None;
    share_authority.revalidation.share_generation = Some(1);
    let share_selection = state.openable_catalog.as_ref().expect("catalog").resolve_document_open(&share_authority.descriptor, Some("surface.test.viewer"), false).expect("share selection");
    share_authority.package = share_selection.package;
    share_authority.artifact = share_selection.artifact;
    share_authority.parent_dialect = share_selection.parent_dialect;
    share_authority.surface = share_selection.surface;
    share_authority.grant = share_selection.grant;
    share_authority.server_actor_id = socket_actor_id(&[0x44; 32], false);
    share_authority.validate().expect("share route authority");
    let share_plan = state.document_open_plans.issue_with_capability(share_authority.clone(), now + 6, now + 10_006, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(46))).expect("share fixture plan");
    let share_authorization = format!("Bearer {share_token}");
    let share_headers = [("Authorization", share_authorization.as_str()), ("Content-Type", "application/json")];
    let share_response = raw_http_request(addr, "POST", &route, &share_headers, request_body(&share_plan.receipt).as_bytes()).await;
    assert_eq!(share_response.status, 200);
    let share_json: serde_json::Value = serde_json::from_slice(&share_response.body).expect("share socket grant JSON");
    assert_eq!(share_json["actorId"], share_authority.server_actor_id);
    assert!(share_json.get("grant").is_none());
    let share_pending = state.socket_grants.pending_document_binding(&SocketAudienceV1::Document(scope), &share_authority.subject.binding(), now_ms()).expect("share pending grant");
    assert_eq!(share_pending.document_plan.as_deref(), Some(&share_authority));
    assert!(!share_pending.document_plan.as_ref().expect("share plan").grant.write);

    let invalid_receipt = request_body("open.v1.AQIDBAUGBwgJCgsMDQ4PEBESExQVFhcYGRobHB0eHyB");
    let invalid = raw_http_request(addr, "POST", &route, &request_headers, invalid_receipt.as_bytes()).await;
    assert_eq!(invalid.status, 400);
    assert_eq!(serde_json::from_slice::<serde_json::Value>(&invalid.body).expect("invalid error JSON")["code"], "denied");
}

#[test]
fn document_open_plan_late_invalid_receipt_wipes_exact_candidate_bytes() {
    DOCUMENT_OPEN_PLAN_DECODE_WIPE_OBSERVATIONS.lock().expect("wipe observations").clear();
    let mut hostile = "open.v1.AQIDBAUGBwgJCgsMDQ4PEBESExQVFhcYGRobHB0eHyA".to_string();
    hostile.pop();
    hostile.push('!');
    DOCUMENT_OPEN_PLAN_DECODE_WIPE_OBSERVING.with(|observing| observing.set(true));
    let result = DocumentOpenPlanCapabilityV1::parse(&hostile);
    DOCUMENT_OPEN_PLAN_DECODE_WIPE_OBSERVING.with(|observing| observing.set(false));
    assert!(matches!(result, Err(DocumentOpenPlanErrorCodeV1::Denied)));
    let observations = std::mem::take(&mut *DOCUMENT_OPEN_PLAN_DECODE_WIPE_OBSERVATIONS.lock().expect("wipe observations"));
    assert_eq!(observations, vec![DocumentOpenPlanDecodeWipeObservationV1 { nonzero_before: 31, after: [0; 32] }]);
}

#[tokio::test]
async fn document_open_plan_admin_revocation_invalidates_session_and_share_bindings() {
    let watchdog = LawHangWatchdogV1::arm("document_open_plan_admin_revocation_invalidates_session_and_share_bindings");
    watchdog.at("hub state open");
    let mut state = test_state().await;
    let fixture: DocumentOpenPlanLedgerFixture = semio_framework_pack_json::from_json_str(semio_hub_testing::hub_source("../🧰️framework/🛍️products/💻️os/🧪️tests/🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("document open plan fixture");
    let admin_headers = authorize_test_admin(&mut state, "open-plan-admin@example.com").await;
    let mut principal = authenticate_admin_principal(&state, &admin_headers, Some(loopback_peer().0)).await.expect("verified open-plan administrator principal");
    principal.correlation_id = "open-plan-admin-revocation".into();

    let session = issue_test_session(&state, "open-plan-revoked-session@example.com").await;
    let session_capability = SessionCapability::parse(&session.token).expect("session capability");
    let session_record = state.directory.authenticate_session(&session_capability).await.expect("session lookup").expect("active session");
    let session_now = u64::try_from(now_ms()).expect("nonnegative session time");
    let mut session_authority = document_open_plan_test_authority(&fixture);
    session_authority.revalidation.session_generation = Some(session_record.authorization_generation);
    session_authority.subject = SocketSubjectV1::Session {
        session_id: session_record.id.clone(),
        user_id: session_record.user_id.clone(),
        authorization_generation: session_record.authorization_generation,
        role: Some(SpaceRole::Author),
        expires_at_ms: session_record.expires_at,
        session_kind: session_record.session_kind,
        device_instance_id: session_record.device_instance_id.clone(),
    };
    let session_plan = state.document_open_plans.issue_with_capability(session_authority.clone(), session_now, session_now + 100, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(20))).expect("session plan");
    let session_intent = AdminIntentV1::RevokeUserSessions { request_id: "request:open-plan-session-revoke".into(), user_id: session_record.user_id, reason_code: "test-revoke".into() };
    let session_digest = admin_intent_digest(&session_intent);
    let mut session_revoke_authority = None;
    watchdog.at("session revocation");
    let session_revoke = execute_admin_intent(&state, &principal, "operation:open-plan-session-revoke", &session_digest, session_intent, None, &mut session_revoke_authority).await;
    drop(session_revoke_authority);
    assert_eq!(session_revoke.phase, "succeeded");
    assert_eq!(state.document_open_plans.exchange(&session_plan.receipt, &session_authority, session_now + 1, "socket-after-session-revoke"), Err(DocumentOpenPlanErrorCodeV1::Stale));

    watchdog.at("share space and document announce");
    let space_effect = admin_effect_receipt_claim("operation:open-plan-share-space", &"0".repeat(64), "directory-events-appended");
    assert!(
        matches!(
            state
                .directory_service
                .execute_create_space_with_id_and_admin_effect(
                    principal.event_actor(),
                    fixture.valid_plan.scope.space_id.clone(),
                    "Open plan share".into(),
                    os_directory::DirectorySpaceKind::Studio,
                    DirectorySpaceVisibility::Private,
                    &space_effect
                )
                .await,
            AdminEffectCommitV1::Applied(_)
        ),
        "the fixture's own space carries the plan a share is revoked on"
    );
    state
        .directory_service
        .execute(DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#open-plan-test", principal.user_id) }, DirectoryCommand::AnnounceDocument { descriptor: Box::new(fixture.descriptor.clone()) })
        .await
        .expect("announce the fixture's plan document");
    let issued_share = state.directory.issue_share_token(&fixture.valid_plan.scope, 60, "open-plan-share").await.expect("share issue");
    let mut share_authority = document_open_plan_test_authority(&fixture);
    share_authority.grant.write = false;
    share_authority.surface.role = os_directory::DocumentOpenSurfaceRoleV1::Viewer;
    share_authority.revalidation.session_generation = None;
    share_authority.revalidation.share_generation = Some(1);
    share_authority.subject = SocketSubjectV1::Share { share_id: issued_share.record.id.clone(), selector: issued_share.record.selector.clone(), scope: issued_share.record.scope.clone(), expires_at_ms: issued_share.record.expires_at };
    let share_now = u64::try_from(now_ms()).expect("nonnegative share time");
    let share_plan = state.document_open_plans.issue_with_capability(share_authority.clone(), share_now, share_now + 100, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(21))).expect("share plan");
    let share_intent = AdminIntentV1::RevokeDocumentShare { request_id: "request:open-plan-share-revoke".into(), scope: issued_share.record.scope, share_id: issued_share.record.id, reason_code: "test-revoke".into() };
    let share_digest = admin_intent_digest(&share_intent);
    let mut share_revoke_authority = None;
    watchdog.at("share revocation");
    let share_revoke = execute_admin_intent(&state, &principal, "operation:open-plan-share-revoke", &share_digest, share_intent, None, &mut share_revoke_authority).await;
    drop(share_revoke_authority);
    assert_eq!(share_revoke.phase, "succeeded");
    assert_eq!(state.document_open_plans.exchange(&share_plan.receipt, &share_authority, share_now + 1, "socket-after-share-revoke"), Err(DocumentOpenPlanErrorCodeV1::Stale));
    watchdog.at("hub state teardown");
    drop(state);
}

/// 👥️ The next server frame that is not a presence roster — rosters fan out independently of a law's commands.
async fn next_command_frame<S>(ws: &mut S, phase: &str) -> ServerFrame
where
    S: StreamExt<Item = Result<WsMessage, tokio_tungstenite::tungstenite::Error>> + Unpin,
{
    loop {
        let frame = next_server_frame_at(ws, phase).await;
        if !matches!(frame, ServerFrame::Presence { .. }) {
            return frame;
        }
    }
}

/// 🪪️ LAW (C13, hub 7800 p33): an undo belongs to its author at the hub's socket too — a `Revert` or `Reinstate` naming an
/// operation another actor committed is answered with the typed `history.foreign-transition` refusal, commits nothing and is
/// relayed to nobody (on p33 the hub persisted B's crafted revert of A's note edit and only the replicas' fold refused it);
/// the author's own undo is admitted and relayed. The replicas' fold refusal stays the second line.
#[test]
fn a_foreign_history_transition_is_refused_at_the_socket_and_never_relayed() {
    run_socket_test(|| async {
        let state = test_state().await;
        let token_a = seed_author_token(&state).await;
        let second = AuthSessionIssue {
            user_id: "seed".into(),
            identity_provider: "test-verifier".into(),
            identity_subject_digest: identity_subject_digest("test-verifier", "seed").expect("seed subject digest"),
            ttl_secs: 3_600,
            device_instance_id: "seed-device-b".into(),
            session_kind: AuthSessionKind::DevelopmentLocal,
            correlation_id: directory::os_identity::time_ordered_id(),
            peer_class: "test".into(),
        };
        let token_b = state.directory.issue_auth_session(&second).await.expect("second author session").capability.expose_once();
        announce_document_for_test(&state, STUDIO, "undo-a").await;
        let document = WireArtifactId("undo-a".into());
        let url = {
            let addr = spawn_server(state.clone()).await;
            format!("ws://{addr}/scopes/{STUDIO}%2Fundo-a/document/ws")
        };
        let receipt_a = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), "undo-a".to_string())), bearer_headers(&token_a), State(state.clone())).await.expect("grant a").0;
        let (mut a, _) = connect_async(document_socket_request(&url, &token_a)).await.expect("socket a");
        a.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("hello a");
        assert!(matches!(next_command_frame(&mut a, "a welcome").await, ServerFrame::Welcome { .. }));
        assert!(matches!(next_command_frame(&mut a, "a session").await, ServerFrame::Session { .. }));
        let mut edit = sample_envelope("a-edit", &document).await;
        edit.actor = ActorId(receipt_a.actor_id.clone());
        a.send(client_binary(&ClientFrame::Commands { batch_id: 1, envelopes: vec![edit.clone()] }, Lane::Command).await).await.expect("a edits");
        let accepted = |frame: &ServerFrame, batch: u64| matches!(frame, ServerFrame::Ack { batch_id, stages, .. } if *batch_id == batch && matches!(stages.last(), Some(AckStage::Applied { outcome }) if matches!(outcome.as_ref(), ApplyOutcome::Accepted)));
        let ack = next_command_frame(&mut a, "a edit ack").await;
        assert!(accepted(&ack, 1), "{ack:?}");
        assert!(matches!(next_command_frame(&mut a, "a edit relay").await, ServerFrame::Commands { .. }));
        let receipt_b = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), "undo-a".to_string())), bearer_headers(&token_b), State(state.clone())).await.expect("grant b").0;
        assert_ne!(receipt_b.actor_id, receipt_a.actor_id, "two sessions are two actors");
        let (mut b, _) = connect_async(document_socket_request(&url, &token_b)).await.expect("socket b");
        b.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("hello b");
        assert!(matches!(next_command_frame(&mut b, "b welcome").await, ServerFrame::Welcome { .. }));
        assert!(matches!(next_command_frame(&mut b, "b catch-up").await, ServerFrame::Commands { .. }));
        assert!(matches!(next_command_frame(&mut b, "b session").await, ServerFrame::Session { .. }));
        let frontier = || async { state.db.document(&db_artifact_id(&DocumentScope::new(STUDIO, "undo-a"))).await.expect("document handle").frontier().await.expect("frontier") };
        let before = frontier().await;
        for (batch, transition) in [(2, protocol::HistoryTransition::Revert { mutation_ids: vec![edit.mutation_id.clone()] }), (3, protocol::HistoryTransition::Reinstate { mutation_ids: vec![edit.mutation_id.clone()] })] {
            let crafted = protocol::history_transition_envelope(&transition, &document, &ActorId(receipt_b.actor_id.clone()), vec![edit.mutation_id.clone()], protocol::HybridLogicalTimestamp::new(batch, 0));
            b.send(client_binary(&ClientFrame::Commands { batch_id: batch, envelopes: vec![crafted] }, Lane::Command).await).await.expect("b crafts");
            match next_command_frame(&mut b, "crafted ack").await {
                ServerFrame::Ack { batch_id, stages, .. } if batch_id == batch => match stages.last() {
                    Some(AckStage::Applied { outcome }) => match outcome.as_ref() {
                        ApplyOutcome::Rejected { messages, .. } => {
                            let messages: serde_json::Value = serde_json::from_slice(messages).expect("rejection messages are one JSON MutationMessage array");
                            assert!(messages.as_array().expect("message array").iter().any(|message| message["code"] == "history.foreign-transition"), "{messages}");
                        }
                        other => panic!("a foreign {transition:?} must be refused: {other:?}"),
                    },
                    other => panic!("expected an applied stage: {other:?}"),
                },
                other => panic!("expected the crafted batch's ack: {other:?}"),
            }
        }
        assert_eq!(frontier().await.commit_seq, before.commit_seq, "a refused transition commits nothing");
        let own = protocol::history_transition_envelope(&protocol::HistoryTransition::Revert { mutation_ids: vec![edit.mutation_id.clone()] }, &document, &ActorId(receipt_a.actor_id.clone()), vec![edit.mutation_id.clone()], protocol::HybridLogicalTimestamp::new(9, 0));
        a.send(client_binary(&ClientFrame::Commands { batch_id: 4, envelopes: vec![own.clone()] }, Lane::Command).await).await.expect("a undoes");
        let ack = next_command_frame(&mut a, "own undo ack").await;
        assert!(accepted(&ack, 4), "the author's own undo is admitted: {ack:?}");
        assert!(matches!(next_command_frame(&mut b, "own undo relay").await, ServerFrame::Commands { envelopes, .. } if envelopes[0].mutation_id == own.mutation_id), "b's first relay after the refusals is a's own undo — the refused transitions were never relayed");
        a.close(None).await.expect("close a");
        b.close(None).await.expect("close b");
    });
}

/// ✏️ LAW (ticket 26/09/30 NON-DESTRUCTIVE-HISTORY-EDITING, `📋️design.md` §9.1): a `Supersede` has no ownership rule at the
/// socket — B's supersession of A's operation commits and is relayed to A — and a supersession the hub refuses (it names
/// an operation the log does not hold) is the typed `history.unknown-target`, commits nothing, is relayed to nobody and
/// leaves its author's socket live: no rebuild follows, its author retracts the refused step (`BackboneMessage::Retract`,
/// follow-up 4) and its next supersession commits on the same socket.
#[test]
fn a_supersession_is_admitted_whatever_its_author_and_its_refusal_leaves_the_socket_live() {
    run_socket_test(|| async {
        let state = test_state().await;
        let author_a = issue_test_session(&state, "supersede-a@example.com").await;
        let author_b = issue_test_session(&state, "supersede-b@example.com").await;
        upsert_member_for_test(&state, STUDIO, "supersede-a@example.com", DirectorySpaceRole::Author).await;
        upsert_member_for_test(&state, STUDIO, "supersede-b@example.com", DirectorySpaceRole::Author).await;
        let document_id = artifact_document_id_for_test("supersede-document");
        seed_genesis_for_document_for_test(&state, &author_a.token, STUDIO, &document_id, "supersede-document").await;
        let document = WireArtifactId(document_id.clone());
        let url = {
            let addr = spawn_server(state.clone()).await;
            format!("ws://{addr}/scopes/{STUDIO}%2F{document_id}/document/ws")
        };
        let open = |token: String| {
            let (url, state, document_id) = (url.clone(), state.clone(), document_id.clone());
            async move {
                let receipt = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), document_id)), bearer_headers(&token), State(state)).await.expect("document socket grant").0;
                let (mut socket, _) = connect_async(document_socket_request(&url, &token)).await.expect("document socket");
                socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("hello");
                while !matches!(next_command_frame(&mut socket, "until session").await, ServerFrame::Session { .. }) {}
                (receipt.actor_id, socket)
            }
        };
        let applied = |frame: &ServerFrame, batch: u64| match frame {
            ServerFrame::Ack { batch_id, stages, .. } if *batch_id == batch => match stages.last() {
                Some(AckStage::Applied { outcome }) => Some(outcome.as_ref().clone()),
                _ => None,
            },
            _ => None,
        };
        let (actor_a, mut a) = open(author_a.token.clone()).await;
        let mut edit = sample_envelope("supersede-a-edit", &document).await;
        edit.actor = ActorId(actor_a.clone());
        a.send(client_binary(&ClientFrame::Commands { batch_id: 1, envelopes: vec![edit.clone()] }, Lane::Command).await).await.expect("a edits");
        assert!(matches!(applied(&next_command_frame(&mut a, "a edit ack").await, 1), Some(ApplyOutcome::Accepted)));
        assert!(matches!(next_command_frame(&mut a, "a edit relay").await, ServerFrame::Commands { .. }));
        let (actor_b, mut b) = open(author_b.token.clone()).await;
        assert_ne!(actor_a, actor_b, "two sessions are two actors");
        let supersede = |targets: &[&protocol::MutationId], at: u64| {
            let transition = protocol::TransitionSupersede { scope: None, inputs: targets.iter().map(|target| protocol::SupersededInput { target: (*target).clone(), replacement: protocol::InputReplacement::Withdrawn }).collect() };
            let dependencies = transition.targets();
            let mut envelope = protocol::history_transition_envelope(&protocol::HistoryTransition::Supersede(transition), &document, &ActorId(actor_b.clone()), dependencies, protocol::HybridLogicalTimestamp::new(at, 0));
            envelope.observed = Some(edit.mutation_id.clone());
            envelope.target = vec!["value".into()];
            envelope
        };
        let frontier = || async { state.db.document(&db_artifact_id(&DocumentScope::new(STUDIO, document_id.as_str()))).await.expect("document handle").frontier().await.expect("frontier") };

        let foreign = supersede(&[&edit.mutation_id], 2);
        b.send(client_binary(&ClientFrame::Commands { batch_id: 2, envelopes: vec![foreign.clone()] }, Lane::Command).await).await.expect("b supersedes a's edit");
        assert!(matches!(applied(&next_command_frame(&mut b, "foreign supersede ack").await, 2), Some(ApplyOutcome::Accepted)), "a supersession of another author's operation is admitted");
        assert!(matches!(next_command_frame(&mut b, "foreign supersede own relay").await, ServerFrame::Commands { .. }));
        assert!(matches!(next_command_frame(&mut a, "foreign supersede relay").await, ServerFrame::Commands { envelopes, .. } if envelopes[0].mutation_id == foreign.mutation_id), "and relayed to the superseded operation's author");

        let before = frontier().await;
        let ghost = protocol::MutationId("supersede-ghost".into());
        b.send(client_binary(&ClientFrame::Commands { batch_id: 3, envelopes: vec![supersede(&[&ghost], 3)] }, Lane::Command).await).await.expect("b supersedes a ghost");
        match applied(&next_command_frame(&mut b, "ghost supersede ack").await, 3) {
            Some(ApplyOutcome::Rejected { messages, .. }) => {
                let messages: serde_json::Value = serde_json::from_slice(&messages).expect("rejection messages are one JSON MutationMessage array");
                assert!(messages.as_array().expect("message array").iter().any(|message| message["code"] == "history.unknown-target"), "{messages}");
            }
            other => panic!("a supersession of an operation the log does not hold is refused: {other:?}"),
        }
        assert_eq!(frontier().await.commit_seq, before.commit_seq, "a refused supersession commits nothing");
        let next = supersede(&[&edit.mutation_id], 4);
        b.send(client_binary(&ClientFrame::Commands { batch_id: 4, envelopes: vec![next.clone()] }, Lane::Command).await).await.expect("b supersedes again on the same socket");
        match next_command_frame(&mut b, "the frame after the refusal").await {
            frame @ ServerFrame::Ack { .. } => assert!(matches!(applied(&frame, 4), Some(ApplyOutcome::Accepted)), "the live socket commits the next supersession: {frame:?}"),
            other => panic!("a refusal is followed by no rebuild, only the next batch's answer: {other:?}"),
        }
        assert!(matches!(next_command_frame(&mut a, "next supersede relay").await, ServerFrame::Commands { envelopes, .. } if envelopes[0].mutation_id == next.mutation_id), "a's next relay is b's committed supersession — the refused one was never relayed");
        a.close(None).await.expect("close a");
        b.close(None).await.expect("close b");
    });
}

//#region ✏️ConcurrentSupersede
type SupersedeReplica = directory::os_store::ArtifactStore<semio_s_artifact_gis_gismap::GisMapSnapshot, semio_s_artifact_gis_gismap::GisMapMutation>;

/// 🧑‍🤝‍🧑️ One author of the concurrent-supersede law: a GIS Map replica authoring as the hub actor its document socket
/// speaks for — the actor a sync actor stamps — the socket, the events it knows and every batch the hub relayed to it.
struct SupersedeAuthor {
    actor: String,
    socket: tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
    replica: SupersedeReplica,
    known: std::collections::HashSet<String>,
    relayed: Vec<MutationEnvelope>,
    batch: u64,
}

impl SupersedeAuthor {
    /// 🔓️ Opens a document socket as `token`'s session and a replica of `document` on the GIS Map genesis pair.
    async fn open(state: &HubState, url: &str, document: &str, token: &str) -> Self {
        use semio_s_artifact_gis_gismap::{GisMapMutation, GisMapSnapshot, GIS_MAP_SCHEMA};
        let actor = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), document.to_string())), bearer_headers(token), State(state.clone())).await.expect("document socket grant").0.actor_id;
        let (mut socket, _) = connect_async(document_socket_request(url, token)).await.expect("document socket");
        socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("hello");
        while !matches!(next_command_frame(&mut socket, "until session").await, ServerFrame::Session { .. }) {}
        let pack = <GisMapSnapshot as pack::snapshot::ArtifactPack>::encode_pack(&GisMapSnapshot::default());
        let spr = directory::os_store::text_format::empty_document_spr(document, GIS_MAP_SCHEMA).await;
        let parsed = directory::os_store::text_format::parse_document_pack::<GisMapSnapshot, GisMapMutation>(&pack, &spr).await.expect("genesis pair parses");
        let mut replica = SupersedeReplica::new(parsed.into_envelope(), directory::ActorId(actor.clone())).await.expect("replica");
        replica.install_document_store_owners_exact(directory::os_store::document_store_owners::bounded_artifact_store_owners());
        assert_eq!(replica.local_actor_id(), &directory::ActorId(actor.clone()), "the replica authors as its socket's actor");
        Self { actor, socket, replica, known: std::collections::HashSet::new(), relayed: Vec::new(), batch: 0 }
    }

    /// 🖋️ Dispatches `command` on the replica and answers the events it authored, as its socket sends them.
    async fn author(&mut self, command: directory::os_store::store_schema::ArtifactCommand<semio_s_artifact_gis_gismap::GisMapMutation>) -> Vec<MutationEnvelope> {
        self.replica.dispatch(command).await.expect("the replica authors");
        let fresh: Vec<MutationEnvelope> = self.replica.event_log().expect("event log").into_iter().filter(|event| self.known.insert(event.mutation_id.0.clone())).collect();
        fresh.into_iter().map(|envelope| MutationEnvelope { actor: ActorId(self.actor.clone()), ..envelope }).collect()
    }

    /// 🛫️ Sends `envelopes` as one batch and answers its terminal outcome; relays read meanwhile are kept.
    async fn submit(&mut self, envelopes: &[MutationEnvelope]) -> ApplyOutcome {
        self.batch += 1;
        let batch = self.batch;
        self.socket.send(client_binary(&ClientFrame::Commands { batch_id: batch, envelopes: envelopes.to_vec() }, Lane::Command).await).await.expect("batch");
        loop {
            match next_command_frame(&mut self.socket, "concurrent supersede ack").await {
                ServerFrame::Ack { batch_id, stages, .. } if batch_id == batch => {
                    if let Some(AckStage::Applied { outcome }) = stages.last() {
                        return outcome.as_ref().clone();
                    }
                }
                ServerFrame::Commands { envelopes, .. } => self.relayed.extend(envelopes),
                _ => {}
            }
        }
    }

    /// 🛬️ Reads until the hub relayed every one of `events`.
    async fn await_relay(&mut self, events: &[MutationEnvelope]) {
        while !events.iter().all(|event| self.relayed.iter().any(|relayed| relayed.mutation_id == event.mutation_id)) {
            if let ServerFrame::Commands { envelopes, .. } = next_command_frame(&mut self.socket, "concurrent supersede relay").await {
                self.relayed.extend(envelopes);
            }
        }
    }

    /// 🔄️ Ingests every relayed event this replica does not hold yet; its own relays are the events it authored.
    async fn ingest_relays(&mut self) {
        for envelope in std::mem::take(&mut self.relayed) {
            if self.known.insert(envelope.mutation_id.0.clone()) {
                self.replica.ingest_remote(envelope).await.expect("the replica folds a relayed event");
            }
        }
    }

    /// 🧽️ Closes the replica through its bounded retirement.
    fn close(mut self) {
        loop {
            match self.replica.close_owned_step(1, directory::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES).expect("replica closes") {
                directory::os_store::SnapshotRetirementStep::Complete => break,
                directory::os_store::SnapshotRetirementStep::Pending { .. } => {}
                directory::os_store::SnapshotRetirementStep::Blocked => panic!("replica close blocked"),
            }
        }
    }
}

/// 📍️ A GIS Map position `id` at `index`, longitude `lon`.
fn supersede_position(index: usize, id: &str, lon: f64) -> semio_s_artifact_gis_gismap::GisMapMutation {
    use semio_s_artifact_gis_gismap::mutations::create_position::CreatePosition;
    let data = directory::DslValue::object([("lon".into(), directory::DslValue::float(lon)), ("lat".into(), directory::DslValue::float(47.0))]);
    semio_s_artifact_gis_gismap::GisMapMutation::CreatePosition(CreatePosition { index, item: semio_s_artifact_gis_gismap::MapFeature { id: id.into(), data } })
}

/// 🤝️ Both replicas hold the same fold: snapshot, effective supersessions, content revision, per-mutation outcomes,
/// alternatives and the line they stand on.
fn assert_supersede_convergence(a: &SupersedeReplica, b: &SupersedeReplica, round: &str) {
    let alternatives = |replica: &SupersedeReplica| replica.envelope().vcs.alternatives.iter().map(|alternative| (alternative.id.clone(), alternative.name.clone(), alternative.checkpoint_ids.clone())).collect::<Vec<_>>();
    assert_eq!(a.snapshot().expect("snapshot"), b.snapshot().expect("snapshot"), "{round}: the same document");
    assert_eq!(a.supersessions(), b.supersessions(), "{round}: the same effective supersessions");
    assert_eq!(a.content_revision(), b.content_revision(), "{round}: the same content revision");
    assert_eq!(a.mutation_outcomes().expect("outcomes"), b.mutation_outcomes().expect("outcomes"), "{round}: the same outcomes");
    assert_eq!((a.active_line_id(), alternatives(a)), (b.active_line_id(), alternatives(b)), "{round}: the same alternatives and line");
}

/// 🏁️ The transition of `events` the fold keeps for a contested operation: the last by `(hlc, id)`.
fn last_supersession(events: &[&MutationEnvelope]) -> String {
    events.iter().max_by(|left, right| (left.timestamp.cmp_key(), &left.mutation_id.0).cmp(&(right.timestamp.cmp_key(), &right.mutation_id.0))).expect("contested events").mutation_id.0.clone()
}

/// 🌗️ LAW (ticket 26/09/30 NON-DESTRUCTIVE-HISTORY-EDITING follow-up 3): two authors supersede concurrently through the
/// hub — each authored without observing the other — and both replicas converge on one fold: the same document, effective
/// supersessions, content revision, outcomes and alternatives. Under `Normal` the hub takes both: the same operation
/// replaced twice, different operations, a withdrawal against a replacement of one operation, and a trunk-scoped
/// supersession against a new alternative's scoped one; a contested operation folds the last supersession by `(hlc,
/// id)`, and the trunk-scoped one takes effect exactly when its author checks the trunk out again. Under `Vigilant` the
/// later concurrent supersession of the same operation is refused `mutation.clamped`, its author retracts it
/// (`BackboneMessage::Retract`, audit F-m8) and converges with the other.
#[test]
fn concurrent_supersessions_through_the_hub_converge_both_replicas() {
    run_socket_test(|| async {
        use directory::os_store::{store_schema::ArtifactCommand, store_schema::SupersedeInput};
        let supersede = |target: &MutationEnvelope, replacement: Option<semio_s_artifact_gis_gismap::GisMapMutation>| SupersedeInput { target: target.mutation_id.clone(), replacement };
        for policy in [protocol::MergePolicy::Normal, protocol::MergePolicy::Vigilant] {
            let mut state = test_state().await;
            state.merge_policy = policy;
            let document = format!("concurrent-supersede-{}", format!("{policy:?}").to_lowercase());
            announce_document_for_test(&state, STUDIO, &document).await;
            let (seed_token, alice_token, bob_token) = (seed_author_token(&state).await, seed_author_token(&state).await, seed_author_token(&state).await);
            let url = format!("ws://{}/scopes/{STUDIO}%2F{document}/document/ws", spawn_server(state.clone()).await);
            let mut seed = SupersedeAuthor::open(&state, &url, &document, &seed_token).await;
            let mut alice = SupersedeAuthor::open(&state, &url, &document, &alice_token).await;
            let mut bob = SupersedeAuthor::open(&state, &url, &document, &bob_token).await;
            let mut edits = Vec::new();
            for (index, id) in ["p0", "p1", "p2", "p3"].into_iter().enumerate() {
                edits.extend(seed.author(ArtifactCommand::Apply { mutations: vec![supersede_position(index, id, 7.0 + index as f64)], transaction: None }).await);
            }
            assert!(matches!(seed.submit(&edits).await, ApplyOutcome::Accepted), "{policy:?}: the seed edits commit");
            for author in [&mut alice, &mut bob] {
                author.await_relay(&edits).await;
                author.ingest_relays().await;
            }
            assert_supersede_convergence(&alice.replica, &bob.replica, "seed");
            let ops: Vec<MutationEnvelope> = edits.clone();
            if policy == protocol::MergePolicy::Vigilant {
                let first = alice.author(ArtifactCommand::Supersede { scope: None, inputs: vec![supersede(&ops[0], Some(supersede_position(0, "p0", 10.0)))] }).await;
                let second = bob.author(ArtifactCommand::Supersede { scope: None, inputs: vec![supersede(&ops[0], Some(supersede_position(0, "p0", 20.0)))] }).await;
                assert!(matches!(alice.submit(&first).await, ApplyOutcome::Accepted), "vigilant: the first supersession commits");
                bob.await_relay(&first).await;
                match bob.submit(&second).await {
                    ApplyOutcome::Rejected { messages, .. } => {
                        let messages: serde_json::Value = serde_json::from_slice(&messages).expect("rejection messages");
                        assert!(messages.as_array().expect("messages").iter().any(|message| message["code"] == "mutation.clamped"), "vigilant: {messages}");
                    }
                    other => panic!("vigilant refuses the concurrent supersession of the same operation: {other:?}"),
                }
                let (channel, remote) = directory::os_store::backbone::ChannelBackbone::pair("concurrent-supersede-retract").await;
                bob.replica.attach_backbone(directory::os_store::backbone::Backbones::Channel(channel)).await.expect("attach");
                remote.push(directory::os_store::backbone::BackboneMessage::Retract { mutation_ids: second.iter().map(|event| event.mutation_id.0.clone()).collect() }).await.expect("retract");
                bob.replica.tick().await.expect("the refused supersession retracts");
                alice.await_relay(&first).await;
                for author in [&mut alice, &mut bob] {
                    author.ingest_relays().await;
                }
                assert_supersede_convergence(&alice.replica, &bob.replica, "vigilant");
                assert_eq!(bob.replica.supersessions().get(&ops[0].mutation_id).map(|supersession| supersession.transition_id.clone()), Some(first[0].mutation_id.0.clone()), "vigilant: the committed supersession is the one in effect");
            } else {
                let rounds: [(&str, ArtifactCommand<semio_s_artifact_gis_gismap::GisMapMutation>, ArtifactCommand<semio_s_artifact_gis_gismap::GisMapMutation>); 4] = [
                    ("same operation", ArtifactCommand::Supersede { scope: None, inputs: vec![supersede(&ops[0], Some(supersede_position(0, "p0", 10.0)))] }, ArtifactCommand::Supersede { scope: None, inputs: vec![supersede(&ops[0], Some(supersede_position(0, "p0", 20.0)))] }),
                    ("different operations", ArtifactCommand::Supersede { scope: None, inputs: vec![supersede(&ops[1], None)] }, ArtifactCommand::Supersede { scope: None, inputs: vec![supersede(&ops[2], Some(supersede_position(2, "p2", 22.0)))] }),
                    ("withdraw against replace", ArtifactCommand::Supersede { scope: None, inputs: vec![supersede(&ops[3], None)] }, ArtifactCommand::Supersede { scope: None, inputs: vec![supersede(&ops[3], Some(supersede_position(3, "p3", 33.0)))] }),
                    ("trunk against alternative", ArtifactCommand::Supersede { scope: Some(alice.replica.trunk_alternative_id()), inputs: vec![supersede(&ops[0], Some(supersede_position(0, "p0", 40.0)))] }, ArtifactCommand::CreateAlternativeWithSupersede { name: "b".into(), inputs: vec![supersede(&ops[1], Some(supersede_position(1, "p1", 41.0)))] }),
                ];
                let mut trunk_scoped = Vec::new();
                for (round, a_command, b_command) in rounds {
                    let a_events = alice.author(a_command).await;
                    let b_events = bob.author(b_command).await;
                    assert!(matches!(alice.submit(&a_events).await, ApplyOutcome::Accepted), "{round}: normal takes the first");
                    assert!(matches!(bob.submit(&b_events).await, ApplyOutcome::Accepted), "{round}: normal takes the concurrent second and reports it");
                    let all: Vec<MutationEnvelope> = a_events.iter().chain(b_events.iter()).cloned().collect();
                    for author in [&mut alice, &mut bob] {
                        author.await_relay(&all).await;
                        author.ingest_relays().await;
                    }
                    assert_supersede_convergence(&alice.replica, &bob.replica, round);
                    let effective = |op: &MutationEnvelope| alice.replica.supersessions().get(&op.mutation_id).map(|supersession| supersession.transition_id.clone());
                    match round {
                        "same operation" | "withdraw against replace" => {
                            let contested = if round == "same operation" { &ops[0] } else { &ops[3] };
                            assert_eq!(effective(contested), Some(last_supersession(&[&a_events[0], &b_events[0]])), "{round}: the last supersession by (hlc, id) is in effect");
                        }
                        "different operations" => assert_eq!((effective(&ops[1]), effective(&ops[2])), (Some(a_events[0].mutation_id.0.clone()), Some(b_events[0].mutation_id.0.clone())), "{round}: both are in effect"),
                        _ => {
                            let branch = b_events.iter().rev().find(|event| matches!(protocol::history_transition_from_envelope(event), Ok(Some(protocol::HistoryTransition::Supersede(_))))).expect("the scoped supersession");
                            assert_eq!(effective(&ops[1]), Some(branch.mutation_id.0.clone()), "{round}: both stand on the new alternative, its scoped supersession in effect");
                            assert_ne!(effective(&ops[0]), Some(a_events[0].mutation_id.0.clone()), "{round}: the trunk-scoped supersession waits on the trunk");
                            trunk_scoped = a_events;
                        }
                    }
                }
                let trunk = alice.replica.trunk_alternative_id();
                let checkout = alice.author(ArtifactCommand::SwitchAlternative { alternative_id: trunk }).await;
                assert!(matches!(alice.submit(&checkout).await, ApplyOutcome::Accepted), "the trunk checkout commits");
                for author in [&mut alice, &mut bob] {
                    author.await_relay(&checkout).await;
                    author.ingest_relays().await;
                }
                assert_supersede_convergence(&alice.replica, &bob.replica, "back on the trunk");
                assert_eq!(bob.replica.supersessions().get(&ops[0].mutation_id).map(|supersession| supersession.transition_id.clone()), Some(trunk_scoped[0].mutation_id.0.clone()), "on the trunk its scoped supersession is in effect");
            }
            for author in [seed, alice, bob] {
                author.close();
            }
        }
    });
}
//#endregion ✏️ConcurrentSupersede

#[test]
fn socket_grant_document_route_is_exact_replay_safe_actor_bound_and_revoke_live() {
    run_socket_test(|| async {
        let state = test_state().await;
        let token = seed_author_token(&state).await;
        announce_document_for_test(&state, STUDIO, "socket-a").await;
        announce_document_for_test(&state, STUDIO, "socket-b").await;
        let nonmember = issue_test_session(&state, "socket-nonmember@example.com").await;
        let unauthorized_existing = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), "socket-a".to_string())), bearer_headers(&nonmember.token), State(state.clone())).await.err();
        let unauthorized_missing = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), "socket-missing".to_string())), bearer_headers(&nonmember.token), State(state.clone())).await.err();
        assert_eq!(unauthorized_existing, Some(StatusCode::UNAUTHORIZED));
        assert_eq!(unauthorized_missing, unauthorized_existing, "unauthorized callers cannot enumerate descriptor existence");
        let receipt = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), "socket-a".to_string())), bearer_headers(&token), State(state.clone())).await.expect("issue document socket grant").0;
        assert_eq!(receipt.schema, "semio.hub.document-socket-grant/v1");
        assert_eq!(receipt.protocol, SESSION_PROTOCOL_V1);
        assert!(receipt.actor_id.starts_with("hub.v1."));

        let addr = spawn_server(state.clone()).await;
        let rejected = connect_async(document_socket_request(&format!("ws://{addr}/scopes/{STUDIO}%2Fsocket-b/document/ws"), &token)).await.expect_err("cross-document grant rejected");
        assert!(matches!(rejected, tokio_tungstenite::tungstenite::Error::Http(response) if response.status().as_u16() == 401));

        let url = format!("ws://{addr}/scopes/{STUDIO}%2Fsocket-a/document/ws");
        let (mut socket, response) = connect_async(document_socket_request(&url, &token)).await.expect("upgrade socket grant");
        assert_eq!(response.headers().get(tokio_tungstenite::tungstenite::http::header::SEC_WEBSOCKET_PROTOCOL).and_then(|value| value.to_str().ok()), Some(SESSION_PROTOCOL_V1));
        socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("socket hello");
        assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Welcome { .. }));
        assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Session { actor, .. } if actor == receipt.actor_id));

        let mut forged = sample_envelope("forged-actor", &WireArtifactId("socket-a".into())).await;
        forged.actor = ActorId("client-selected-forgery".into());
        socket.send(client_binary(&ClientFrame::Commands { batch_id: 77, envelopes: vec![forged] }, Lane::Command).await).await.expect("forged command");
        match next_server_frame(&mut socket).await {
            ServerFrame::Ack { batch_id: 77, stages, .. } => match &stages[0] {
                AckStage::Applied { outcome } => match outcome.as_ref() {
                    ApplyOutcome::Rejected { reason, .. } => {
                        assert_eq!(reason, "socket subject actor mismatch");
                    }
                    other => panic!("forged actor was not rejected: {other:?}"),
                },
                other => panic!("unexpected forged actor stage: {other:?}"),
            },
            other => panic!("expected forged actor ack, got {other:?}"),
        }

        let mut committed = sample_envelope("committed-once", &WireArtifactId("socket-a".into())).await;
        committed.actor = ActorId(receipt.actor_id.clone());
        let accepted = |frame: &ServerFrame, batch: u64| matches!(frame, ServerFrame::Ack { batch_id, stages, .. } if *batch_id == batch && matches!(stages.last(), Some(AckStage::Applied { outcome }) if matches!(outcome.as_ref(), ApplyOutcome::Accepted)));
        socket.send(client_binary(&ClientFrame::Commands { batch_id: 78, envelopes: vec![committed.clone()] }, Lane::Command).await).await.expect("first commit");
        let ack = next_server_frame(&mut socket).await;
        assert!(accepted(&ack, 78), "{ack:?}");
        assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Commands { envelopes, .. } if envelopes[0].mutation_id == committed.mutation_id), "a first commit is relayed");
        let resend_receipt = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), "socket-a".to_string())), bearer_headers(&token), State(state.clone())).await.expect("resend grant").0;
        assert_eq!(resend_receipt.actor_id, receipt.actor_id);
        let (mut resend, _) = connect_async(document_socket_request(&url, &token)).await.expect("resend socket");
        resend.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("resend hello");
        assert!(matches!(next_server_frame(&mut resend).await, ServerFrame::Welcome { .. }));
        assert!(matches!(next_server_frame(&mut resend).await, ServerFrame::Commands { envelopes, origin, .. } if envelopes[0].mutation_id == committed.mutation_id && origin.0 == HUB_CATCH_UP_ORIGIN), "the reconnect is caught up with the committed edit, under the hub's declared catch-up origin");
        assert!(matches!(next_server_frame(&mut resend).await, ServerFrame::Session { .. }));
        resend.send(client_binary(&ClientFrame::Commands { batch_id: 79, envelopes: vec![committed.clone()] }, Lane::Command).await).await.expect("resend after reconnect");
        let ack = next_server_frame(&mut resend).await;
        assert!(accepted(&ack, 79), "an idempotent resend after reconnect is acknowledged: {ack:?}");
        resend.send(client_binary(&ClientFrame::PreviewPublish { key: "relay-fence".into(), seq: 79, payload: vec![] }, Lane::Preview).await).await.expect("relay fence");
        assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Preview { key, seq: 79, .. } if key == "relay-fence"), "an idempotent resend is never relayed to peers as a new commit");
        let committed_frontier = state.db.document(&db_artifact_id(&DocumentScope::new(STUDIO, "socket-a"))).await.expect("document handle").frontier().await.expect("frontier");
        assert_eq!(committed_frontier.commit_seq, 1, "the resend did not commit twice");
        resend.close(None).await.expect("close resend socket");

        let legacy_receipt = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), "socket-a".to_string())), bearer_headers(&token), State(state.clone())).await.expect("issue legacy-carrier rejection grant").0;
        let (mut legacy, _) = connect_async(document_socket_request(&url, &token)).await.expect("legacy rejection socket");
        legacy.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("initial socket hello");
        assert!(matches!(next_server_frame(&mut legacy).await, ServerFrame::Welcome { .. }));
        assert!(matches!(next_server_frame(&mut legacy).await, ServerFrame::Commands { origin, .. } if origin.0 == HUB_CATCH_UP_ORIGIN && origin.0 != legacy_receipt.actor_id), "a joiner behind the head is caught up before Session, never under its own actor");
        let echo_fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧫️fixtures/document-echo-suppression-v1/🔣️.json")).expect("echo suppression fixture");
        assert_eq!(echo_fixture["hubCatchUpOrigin"].as_str(), Some(HUB_CATCH_UP_ORIGIN), "the hub's catch-up origin is the fixture's declared one");
        assert_eq!(semio_hub_testing::hub_source("🏗️bootstrap/🔌️document-socket/🦀️.rs").matches("ActorId(HUB_CATCH_UP_ORIGIN.into())").count(), 2, "the hello tail and the FrontierAdvertise catch-up both carry the declared catch-up origin");
        assert!(matches!(next_server_frame(&mut legacy).await, ServerFrame::Session { .. }));
        legacy.send(WsMessage::Binary(vec![0, 0].into())).await.expect("legacy tag-zero frame");
        assert_eq!(next_close_code(&mut legacy, false).await, 4401, "v1 rejects the legacy actor/token carrier after upgrade");

        let replay = connect_async(document_socket_request(&url, &token)).await.expect_err("consumed grant replay rejected");
        assert!(matches!(replay, tokio_tungstenite::tungstenite::Error::Http(response) if response.status().as_u16() == 401));
        let pending = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), "socket-a".to_string())), bearer_headers(&token), State(state.clone())).await.expect("issue pending grant").0;
        assert_eq!(pending.actor_id, receipt.actor_id, "session-derived actor is stable across grants");

        assert_eq!(post_session_sign_out(bearer_headers(&token), State(state.clone()), Bytes::new()).await, StatusCode::NO_CONTENT);
        assert_eq!(next_close_code(&mut socket, false).await, 4401, "successful durable revoke immediately invalidates a live socket");
        let revoked_pending = connect_async(document_socket_request(&url, &token)).await.expect_err("pending grant invalidated by revoke");
        assert!(matches!(revoked_pending, tokio_tungstenite::tungstenite::Error::Http(response) if response.status().as_u16() == 401));
    });
}

#[test]
fn socket_grant_revoke_and_welcome_have_a_bounded_binding_linearization() {
    run_socket_test(|| async {
        let mut state = test_state().await;
        let gate = Arc::new(TestLiveGate::default());
        state.live_gate = Some(gate.clone());
        let token = seed_author_token(&state).await;
        announce_document_for_test(&state, STUDIO, "socket-linearized").await;
        let receipt = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), "socket-linearized".to_string())), bearer_headers(&token), State(state.clone())).await.expect("issue socket grant").0;
        let addr = spawn_server(state.clone()).await;
        let url = format!("ws://{addr}/scopes/{STUDIO}%2Fsocket-linearized/document/ws");
        let (mut socket, _) = connect_async(document_socket_request(&url, &token)).await.expect("socket upgrade");
        socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("socket hello");
        tokio::time::timeout(std::time::Duration::from_secs(2), gate.socket_before_welcome.acquire()).await.expect("pre-welcome gate deadline").expect("pre-welcome gate");
        let mut revoke = tokio::spawn({
            let state = state.clone();
            let token = token.clone();
            async move { post_session_sign_out(bearer_headers(&token), State(state), Bytes::new()).await }
        });
        assert!(tokio::time::timeout(std::time::Duration::from_millis(100), &mut revoke).await.is_err(), "revoke waits while Welcome owns the binding linearization");
        gate.socket_welcome_release.add_permits(1);
        assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Welcome { .. }), "Welcome linearizes before the waiting revoke");
        tokio::time::timeout(std::time::Duration::from_secs(2), gate.socket_after_welcome.acquire()).await.expect("post-Welcome boundary deadline").expect("post-Welcome boundary");
        assert_eq!(tokio::time::timeout(std::time::Duration::from_secs(2), revoke).await.expect("bounded revoke completion").expect("revoke task"), StatusCode::NO_CONTENT);
        gate.socket_bootstrap_release.add_permits(1);
        assert_eq!(next_close_without_authority(&mut socket).await, 4401, "a revoke winning after Welcome suppresses bootstrap and Session authority");
    });
}

#[test]
fn socket_grant_revoke_before_lag_authorization_reads_no_private_control() {
    run_socket_test(|| async {
        let mut state = test_state_with_capacity(1024, 1).await;
        let live_gate = Arc::new(TestLiveGate::default());
        state.live_gate = Some(live_gate.clone());
        let token = seed_author_token(&state).await;
        announce_document_for_test(&state, STUDIO, "socket-lag-revoke").await;
        let receipt = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), "socket-lag-revoke".to_string())), bearer_headers(&token), State(state.clone())).await.expect("issue socket grant").0;
        let addr = spawn_server(state.clone()).await;
        let url = format!("ws://{addr}/scopes/{STUDIO}%2Fsocket-lag-revoke/document/ws");
        let (mut socket, _) = connect_async(document_socket_request(&url, &token)).await.expect("socket upgrade");
        socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("socket hello");
        tokio::time::timeout(std::time::Duration::from_secs(5), live_gate.socket_before_welcome.acquire()).await.expect("pre-Welcome deadline").expect("pre-Welcome");
        live_gate.socket_welcome_release.add_permits(1);
        assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Welcome { .. }));
        tokio::time::timeout(std::time::Duration::from_secs(5), live_gate.socket_after_welcome.acquire()).await.expect("post-Welcome deadline").expect("post-Welcome");
        live_gate.socket_bootstrap_release.add_permits(1);
        assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Session { .. }));
        tokio::time::timeout(std::time::Duration::from_secs(5), live_gate.document_subscribed.acquire()).await.expect("subscription deadline").expect("subscription");
        let fanout = state.fanout_for(&document_scope_key_v1(&DocumentScope::new(STUDIO, "socket-lag-revoke")));
        fanout.send(ServerFrame::Presence { peers: vec![b"first".to_vec()] }).expect("first fanout");
        fanout.send(ServerFrame::Presence { peers: vec![b"second".to_vec()] }).expect("second fanout");
        live_gate.document_release.add_permits(1);
        tokio::time::timeout(std::time::Duration::from_secs(5), live_gate.socket_lag_received.acquire()).await.expect("lag boundary deadline").expect("lag boundary");
        assert_eq!(post_session_sign_out(bearer_headers(&token), State(state), Bytes::new()).await, StatusCode::NO_CONTENT);
        live_gate.socket_lag_release.add_permits(1);
        assert_eq!(next_close_without_authority(&mut socket).await, 4401, "revoked lag path discloses no control frame");
        assert_eq!(live_gate.socket_rebootstrap_read.available_permits(), 0, "revoked lag path never enters the private checkpoint/control read");
    });
}

#[test]
fn socket_grant_revoke_before_broadcast_authorization_suppresses_frame() {
    run_socket_test(|| async {
        let mut state = test_state().await;
        let live_gate = Arc::new(TestLiveGate::default());
        state.live_gate = Some(live_gate.clone());
        live_gate.socket_broadcast_pause_enabled.store(true, std::sync::atomic::Ordering::Release);
        let token = seed_author_token(&state).await;
        announce_document_for_test(&state, STUDIO, "socket-broadcast-revoke").await;
        let receipt = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), "socket-broadcast-revoke".to_string())), bearer_headers(&token), State(state.clone())).await.expect("issue socket grant").0;
        let addr = spawn_server(state.clone()).await;
        let url = format!("ws://{addr}/scopes/{STUDIO}%2Fsocket-broadcast-revoke/document/ws");
        let (mut socket, _) = connect_async(document_socket_request(&url, &token)).await.expect("socket upgrade");
        socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("socket hello");
        tokio::time::timeout(std::time::Duration::from_secs(2), live_gate.socket_before_welcome.acquire()).await.expect("pre-Welcome deadline").expect("pre-Welcome");
        live_gate.socket_welcome_release.add_permits(1);
        assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Welcome { .. }));
        tokio::time::timeout(std::time::Duration::from_secs(2), live_gate.socket_after_welcome.acquire()).await.expect("post-Welcome deadline").expect("post-Welcome");
        live_gate.socket_bootstrap_release.add_permits(1);
        assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Session { .. }));
        tokio::time::timeout(std::time::Duration::from_secs(2), live_gate.document_subscribed.acquire()).await.expect("subscription deadline").expect("subscription");
        let fanout = state.fanout_for(&document_scope_key_v1(&DocumentScope::new(STUDIO, "socket-broadcast-revoke")));
        fanout.send(ServerFrame::Presence { peers: vec![b"private-presence".to_vec()] }).expect("fanout");
        live_gate.document_release.add_permits(1);
        tokio::time::timeout(std::time::Duration::from_secs(2), live_gate.socket_broadcast_received.acquire()).await.expect("broadcast boundary deadline").expect("broadcast boundary");
        assert_eq!(post_session_sign_out(bearer_headers(&token), State(state), Bytes::new()).await, StatusCode::NO_CONTENT);
        live_gate.socket_broadcast_release.add_permits(1);
        assert_eq!(next_close_without_authority(&mut socket).await, 4401, "a broadcast received before a winning revoke is never disclosed afterward");
    });
}

#[test]
fn socket_grant_directory_route_uses_credential_free_hello_and_revokes_live() {
    run_socket_test(|| async {
        let state = test_state().await;
        let token = seed_author_token(&state).await;
        let receipt = issue_directory_socket_grant(bearer_headers(&token), State(state.clone()), Bytes::new()).await.expect("issue directory socket grant").0;
        let since = state.directory.head_seq().await.expect("directory head");
        let addr = spawn_server(state.clone()).await;
        let url = format!("ws://{addr}/directory/socket/v1?since={since}");
        let (mut socket, _) = connect_async(socket_request(&url, &receipt.grant)).await.expect("directory socket");
        socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("credential-free hello");
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        state
            .directory_service
            .execute(DirectoryActor { kind: DirectoryActorKind::User, id: "user:seed#directory-socket-law".into() }, DirectoryCommand::RenameSpace { space_id: STUDIO.into(), name: "Socket Grant Studio".into() })
            .await
            .expect("member-visible directory event");
        assert!(matches!(next_directory_message(&mut socket).await, DirectoryStreamMessage::Event { event } if event.space_id.as_deref() == Some(STUDIO)));
        assert_eq!(post_session_sign_out(bearer_headers(&token), State(state), Bytes::new()).await, StatusCode::NO_CONTENT);
        assert_eq!(next_close_code(&mut socket, false).await, 4401);
    });
}

/// 🚦️ Using an authority never waits on another use of it: while a member's document command is in
/// flight — holding its principal, membership and space bindings, waiting for the document's writer —
/// the space owner still adds a newcomer at once and the member's own directory socket still delivers
/// that event at once. Every use held its bindings exclusively, so one slow write in a space stalled
/// every directory command and every directory delivery of that space behind it (C10: a member's Home
/// never listed a space's new documents for 60–120 s).
#[test]
fn a_command_in_flight_never_stalls_the_spaces_directory_commands_or_deliveries() {
    run_socket_test(|| async {
        let state = test_state().await;
        let owner = issue_test_session(&state, "fence-owner@example.com").await;
        let member = issue_test_session(&state, "fence-member@example.com").await;
        issue_test_session(&state, "fence-newcomer@example.com").await;
        let space_id = create_space_for_test(&state, &owner.user_id, "Fence space", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        upsert_member_for_test(&state, &space_id, "fence-member@example.com", DirectorySpaceRole::Author).await;
        announce_document_for_test(&state, &space_id, "fence-doc").await;
        let addr = spawn_server(state.clone()).await;
        let grant = issue_document_socket_grant_fixture(Path((space_id.clone(), "fence-doc".to_string())), bearer_headers(&member.token), State(state.clone())).await.expect("member document grant").0;
        let (mut document, _) = connect_async(document_socket_request(&format!("ws://{addr}/scopes/{space_id}%2Ffence-doc/document/ws"), &member.token)).await.expect("member document socket");
        document.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("document hello");
        assert!(matches!(next_server_frame(&mut document).await, ServerFrame::Welcome { .. }));
        assert!(matches!(next_server_frame(&mut document).await, ServerFrame::Session { .. }));
        let directory_grant = issue_directory_socket_grant(bearer_headers(&member.token), State(state.clone()), Bytes::new()).await.expect("member directory grant").0;
        let since = state.directory.head_seq().await.expect("directory head");
        let (mut directory, _) = connect_async(socket_request(&format!("ws://{addr}/directory/socket/v1?since={since}"), &directory_grant.grant)).await.expect("member directory socket");
        directory.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("directory hello");

        let scope = DocumentScope::new(space_id.clone(), "fence-doc");
        let writer = state.socket_binding_gates.document_write(&scope).lock_owned().await;
        let mut command = sample_envelope("fence-command", &WireArtifactId("fence-doc".into())).await;
        command.actor = ActorId(grant.actor_id.clone());
        document.send(client_binary(&ClientFrame::Commands { batch_id: 31, envelopes: vec![command] }, Lane::Command).await).await.expect("in-flight command");
        let membership = SocketBindingKeyV1::Membership { user_id: member.user_id.clone(), space_id: space_id.clone() };
        let admitted = tokio::time::timeout(std::time::Duration::from_secs(5), async {
            while state.socket_binding_gates.gate(membership.clone()).try_write_owned().is_ok() {
                tokio::time::sleep(std::time::Duration::from_millis(5)).await;
            }
        })
        .await;
        assert!(admitted.is_ok(), "the member's command is in flight, holding its membership");

        let upserted_at = std::time::Instant::now();
        let upsert = DirectoryCommand::UpsertMember { space_id: space_id.clone(), email: "fence-newcomer@example.com".into(), role: DirectorySpaceRole::Spectator };
        let response = post_directory_command_for_test(addr, &owner.token, "0123456789abcdef0123456789abcdef", upsert).await;
        assert_eq!(response.status, 202, "the owner's command is not fenced by the member's write: {}", String::from_utf8_lossy(&response.body));
        assert!(matches!(next_directory_message(&mut directory).await, DirectoryStreamMessage::Event { event } if matches!(&event.body, os_directory::DirectoryEventBody::MemberUpserted { space_id: upserted, .. } if *upserted == space_id)), "the member's directory socket delivers the space's event while its own command is in flight");
        assert!(upserted_at.elapsed() < std::time::Duration::from_secs(2), "neither waited on the in-flight write: {:?}", upserted_at.elapsed());

        drop(writer);
        match next_server_frame(&mut document).await {
            ServerFrame::Ack { batch_id: 31, stages, .. } => assert!(matches!(stages.last(), Some(AckStage::Applied { outcome }) if matches!(outcome.as_ref(), ApplyOutcome::Accepted)), "{stages:?}"),
            other => panic!("expected the in-flight command's Ack, got {other:?}"),
        }
    });
}

#[tokio::test]
async fn scoped_directory_socket_ledger_indexes_and_invalidates_exact_membership() {
    let ledger = SocketGrantLedgerV1::default();
    let scope = DocumentScope::new("space-a", "document-a");
    let audience = SocketAudienceV1::DirectoryScoped(scope.clone());
    let subject = SocketSubjectV1::Session { session_id: "session-a".into(), user_id: "user-a".into(), authorization_generation: 7, role: Some(SpaceRole::Spectator), expires_at_ms: 10_000, session_kind: AuthSessionKind::External, device_instance_id: "device-a".into() };
    let capability = SocketGrantCapability::mint().expect("scoped capability");
    ledger.issue(&capability, audience.clone(), "hub.v1.scoped".into(), subject.clone(), 1, 9_000).expect("scoped issue");
    let pending = ledger.pending(&capability, &audience, 2).expect("pending scoped grant");
    assert_eq!(
        pending.bindings(),
        vec![
            SocketBindingKeyV1::User("user-a".into()),
            SocketBindingKeyV1::Session("session-a".into()),
            SocketBindingKeyV1::DirectorySpaceAuthority { space_id: "space-a".into() },
            SocketBindingKeyV1::Membership { user_id: "user-a".into(), space_id: "space-a".into() },
        ]
    );
    let consumed = ledger.consume(&pending, 3).expect("consume scoped grant");
    let (live_id, notify) = ledger.register_live(&consumed).expect("register scoped live lease");
    assert!(ledger.is_live(&consumed, &live_id));
    ledger.invalidate_binding(SocketBindingKeyV1::Membership { user_id: "user-a".into(), space_id: "space-a".into() });
    assert!(!ledger.is_live(&consumed, &live_id));
    tokio::time::timeout(std::time::Duration::from_millis(50), notify.notified()).await.expect("membership invalidation notifies once");

    let later = SocketGrantCapability::mint().expect("later scoped capability");
    ledger.issue(&later, audience.clone(), "hub.v1.later".into(), subject, 4, 9_000).expect("later issue");
    ledger.invalidate_binding(SocketBindingKeyV1::Membership { user_id: "user-a".into(), space_id: "space-a".into() });
    assert!(ledger.pending(&later, &audience, 5).is_err(), "membership invalidation also removes pending grants");
}

#[test]
fn scoped_directory_socket_message_matching_is_body_exact_and_removal_private() {
    let scope = DocumentScope::new("space-a", "document-a");
    let event = |body: os_directory::DirectoryEventBody| DirectoryStreamMessage::Event {
        event: Box::new(DirectoryEvent {
            seq: 1,
            id: "event-a".into(),
            hlc: os_directory::Hlc { physical_ms: 1, logical: 0 },
            actor: DirectoryActor { kind: DirectoryActorKind::System, id: "system:test".into() },
            space_id: Some("space-a".into()),
            user_id: None,
            body,
            recorded_at_ms: 1,
        }),
    };
    assert!(directory_message_matches_scope(&scope, &event(os_directory::DirectoryEventBody::DocumentAnnounced { descriptor: document_descriptor_for_test("space-a", "document-a") })));
    assert!(!directory_message_matches_scope(&scope, &event(os_directory::DirectoryEventBody::DocumentAnnounced { descriptor: document_descriptor_for_test("space-a", "document-b") })));
    assert!(!directory_message_matches_scope(&scope, &event(os_directory::DirectoryEventBody::MemberRemoved { space_id: "space-a".into(), user_id: "user-a".into() })));
    assert!(!directory_message_matches_scope(&scope, &DirectoryStreamMessage::Heartbeat { head_seq: 99 }));
    assert!(directory_message_matches_scope(&scope, &DirectoryStreamMessage::Presence { space_id: "space-a".into(), document_id: "document-a".into(), actors: Vec::new() }));
    assert!(!directory_message_matches_scope(&scope, &DirectoryStreamMessage::Presence { space_id: "space-a".into(), document_id: "document-b".into(), actors: Vec::new() }));
}

#[test]
fn scoped_directory_socket_route_rejects_scope_substitution_and_rest_removal_closes_without_event() {
    run_socket_test(|| async {
        let state = test_state().await;
        let owner_token = seed_author_token(&state).await;
        let member = issue_test_session(&state, "scoped-member@example.com").await;
        upsert_member_for_test(&state, STUDIO, "scoped-member@example.com", DirectorySpaceRole::Spectator).await;
        announce_document_for_test(&state, STUDIO, "scoped-document-a").await;
        announce_document_for_test(&state, STUDIO, "scoped-document-b").await;
        let unaffected = issue_test_session(&state, "scoped-unaffected@example.com").await;
        let unaffected_space = create_space_for_test(&state, &unaffected.user_id, "Scoped Unaffected", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        let unaffected_document = artifact_document_id_for_test("scoped-unaffected-document");
        seed_genesis_for_document_for_test(&state, &unaffected.token, &unaffected_space, &unaffected_document, "scoped-unaffected-document").await;
        let addr = spawn_server(state.clone()).await;
        let authorization = format!("Bearer {}", member.token);
        let issue_path = format!("/directory/spaces/{STUDIO}/documents/scoped-document-a/socket-grants");
        let issued = raw_http_request(addr, "POST", &issue_path, &[("Authorization", &authorization)], &[]).await;
        assert_eq!(issued.status, 200);
        let receipt: serde_json::Value = serde_json::from_slice(&issued.body).expect("scoped grant JSON");
        let grant = receipt["grant"].as_str().expect("scoped grant");
        let substituted = format!("ws://{addr}/directory/spaces/{STUDIO}/documents/scoped-document-b/socket/v1?since=0");
        let error = connect_async(socket_request(&substituted, grant)).await.expect_err("scope substitution rejected before upgrade");
        assert!(matches!(error, tokio_tungstenite::tungstenite::Error::Http(response) if response.status() == StatusCode::UNAUTHORIZED));

        let since = state.directory.head_seq().await.expect("directory head");
        let url = format!("ws://{addr}/directory/spaces/{STUDIO}/documents/scoped-document-a/socket/v1?since={since}");
        let (mut socket, _) = connect_async(socket_request(&url, grant)).await.expect("exact scoped socket");
        socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("scoped socket hello");
        let unaffected_authorization = format!("Bearer {}", unaffected.token);
        let unaffected_issue_path = format!("/directory/spaces/{unaffected_space}/documents/{unaffected_document}/socket-grants");
        let unaffected_issued = raw_http_request(addr, "POST", &unaffected_issue_path, &[("Authorization", &unaffected_authorization)], &[]).await;
        assert_eq!(unaffected_issued.status, 200);
        let unaffected_receipt: serde_json::Value = serde_json::from_slice(&unaffected_issued.body).expect("unaffected scoped grant JSON");
        let unaffected_grant = unaffected_receipt["grant"].as_str().expect("unaffected scoped grant");
        let unaffected_url = format!("ws://{addr}/directory/spaces/{unaffected_space}/documents/{unaffected_document}/socket/v1?since={since}");
        let (mut unaffected_socket, _) = connect_async(socket_request(&unaffected_url, unaffected_grant)).await.expect("unaffected scoped socket");
        unaffected_socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("unaffected scoped hello");
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        let pending = raw_http_request(addr, "POST", &issue_path, &[("Authorization", &authorization)], &[]).await;
        assert_eq!(pending.status, 200);
        let pending_receipt: serde_json::Value = serde_json::from_slice(&pending.body).expect("pending scoped grant JSON");
        let pending_grant = pending_receipt["grant"].as_str().expect("pending scoped grant").to_string();
        announce_document_for_test(&state, STUDIO, "scoped-document-c").await;
        assert!(tokio::time::timeout(std::time::Duration::from_millis(100), socket.next()).await.is_err(), "same-space foreign document never serializes");

        let command = DirectoryCommand::RemoveMember { space_id: STUDIO.into(), user_id: member.user_id.clone() };
        let removed = post_directory_command_for_test(addr, &owner_token, "5c0ded0c0ffee0bad0f00d0dead0beef", command).await;
        assert_eq!(removed.status, 202);
        let removed_body = String::from_utf8(removed.body).expect("remove response UTF-8");
        assert!(removed_body.contains("member.removed"));
        assert_eq!(next_close_code(&mut socket, false).await, 4401, "durable removal invalidates the scoped lease without exposing its event");
        let unaffected_published = publish_checkpoint_for_test(&state, &unaffected_space, &unaffected_document).await;
        let unaffected_delivered = next_directory_message(&mut unaffected_socket).await;
        assert!(
            matches!(
                &unaffected_delivered,
                DirectoryStreamMessage::Event { event }
                    if matches!(event.body, os_directory::DirectoryEventBody::ArtifactCheckpointPublished { ref checkpoint }
                        if checkpoint.scope == DocumentScope::new(&unaffected_space, &unaffected_document) && checkpoint.checkpoint_id == unaffected_published.checkpoint_id)
            ),
            "another user's exact scoped subscription remains live: {unaffected_delivered:?}"
        );

        for stale in [grant.to_string(), pending_grant] {
            let error = connect_async(socket_request(&url, &stale)).await.expect_err("consumed or pending pre-removal grant remains invalid");
            assert!(matches!(error, tokio_tungstenite::tungstenite::Error::Http(response) if response.status() == StatusCode::UNAUTHORIZED));
        }

        let denied = raw_http_request(addr, "POST", &issue_path, &[("Authorization", &authorization)], &[]).await;
        assert_eq!(denied.status, 401, "removed member cannot reacquire the scoped grant");
    });
}

#[test]
fn scoped_directory_socket_admin_removal_uses_the_same_membership_fence() {
    run_socket_test(|| async {
        let mut state = test_state().await;
        let mut admin_headers = authorize_test_admin(&mut state, "scoped-admin@example.com").await;
        admin_headers.insert(axum::http::header::CONTENT_TYPE, "application/json".parse().expect("content type"));
        let member = issue_test_session(&state, "scoped-admin-target@example.com").await;
        upsert_member_for_test(&state, STUDIO, "scoped-admin-target@example.com", DirectorySpaceRole::Spectator).await;
        announce_document_for_test(&state, STUDIO, "scoped-admin-document").await;
        let addr = spawn_server(state.clone()).await;
        let authorization = format!("Bearer {}", member.token);
        let issue_path = format!("/directory/spaces/{STUDIO}/documents/scoped-admin-document/socket-grants");
        let issued = raw_http_request(addr, "POST", &issue_path, &[("Authorization", &authorization)], &[]).await;
        assert_eq!(issued.status, 200);
        let receipt: serde_json::Value = serde_json::from_slice(&issued.body).expect("scoped grant JSON");
        let grant = receipt["grant"].as_str().expect("scoped grant");
        let since = state.directory.head_seq().await.expect("directory head");
        let url = format!("ws://{addr}/directory/spaces/{STUDIO}/documents/scoped-admin-document/socket/v1?since={since}");
        let (mut socket, _) = connect_async(socket_request(&url, grant)).await.expect("admin target scoped socket");
        socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("scoped socket hello");
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;

        let intent = AdminIntentV1::RemoveSpaceMember { request_id: "request:scoped-admin-removal".into(), space_id: STUDIO.into(), user_id: member.user_id };
        let body = Bytes::from(semio_framework_pack_json::to_json_string(&intent));
        let (status, receipt) = admin_intents(admin_headers, loopback_peer(), State(state), body).await.expect("admin removal response");
        assert_eq!(status, StatusCode::OK);
        assert_eq!(receipt.0.state, AdminIntentStateV1::Succeeded);
        assert_eq!(next_close_code(&mut socket, false).await, 4401, "admin removal uses the same no-event membership fence");
    });
}
