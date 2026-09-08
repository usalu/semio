//! 🧪️ Real PostgreSQL genesis acknowledgement, lock-time authority, and recovery ownership laws.

use super::tests::{test_directory, PostgresContainer};
use super::*;
use crate::artifact_authority::chunk_cas::{artifact_cas_manifest_locator_v1, prepare_artifact_cas_manifest_v1, prepare_artifact_cas_ownership_v1};
use crate::artifact_authority::creation::{artifact_creation_command_digest_v1, ArtifactCreationPreparedV1, ARTIFACT_CREATION_DEADLINE_MS};
use crate::artifact_authority::{checkpoint_id_encoding_v1, ArtifactPair};
use crate::directory::published_artifact_checkpoint;
use directory::os_directory::schema::space_artifact_creation::SpaceArtifactCreationPhaseV1;

struct GenesisFixture {
    directory: std::sync::Arc<PostgresDirectory>,
    container: PostgresContainer,
    intent: ArtifactCreationIntentV1,
    prepared: ArtifactCreationPreparedV1,
    checkpoint: ArtifactCheckpoint,
    reservation: ArtifactCasReservation,
}

impl GenesisFixture {
    fn packet(&self) -> DocumentGenesisAppendV1 {
        let mut clock = HubClock::new();
        let author = DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#artifact-creation", self.intent.actor.user_id) };
        DocumentGenesisAppendV1 {
            intent: self.intent.clone(),
            checkpoint: self.checkpoint.clone(),
            reservation: self.reservation.clone(),
            now_ms: now_ms() as u64,
            events: [
                NewDirectoryEvent {
                    hlc: clock.tick(),
                    actor: author.clone(),
                    space_id: Some(self.intent.scope.space_id.clone()),
                    user_id: Some(self.intent.actor.user_id.clone()),
                    body: DirectoryEventBody::DocumentAnnounced { descriptor: self.prepared.descriptor.clone() },
                },
                NewDirectoryEvent {
                    hlc: clock.tick(),
                    actor: author,
                    space_id: Some(self.intent.scope.space_id.clone()),
                    user_id: Some(self.intent.actor.user_id.clone()),
                    body: DirectoryEventBody::DocumentIndexed {
                        scope: self.intent.scope.clone(),
                        descriptor_digest_v1: self.checkpoint.descriptor_digest_v1,
                        entry: directory::os_directory::DocumentIndexEntryV1 { name: self.intent.request.name.clone(), dialect: self.intent.parent_dialect.clone() },
                    },
                },
                NewDirectoryEvent {
                    hlc: clock.tick(),
                    actor: system(),
                    space_id: Some(self.intent.scope.space_id.clone()),
                    user_id: None,
                    body: DirectoryEventBody::ArtifactCheckpointPublished { checkpoint: published_artifact_checkpoint(&self.checkpoint) },
                },
            ],
        }
    }

    async fn operation(&self) -> ArtifactCreationOperationV1 {
        ArtifactCreationOperationV1::fold(&self.directory.read_artifact_creation(&self.intent.actor.user_id, &self.intent.request.request_id).await.unwrap()).unwrap()
    }

    async fn create() -> Self {
        let (directory, container) = test_directory().await;
        directory.seed().await.unwrap();
        let issued = directory
            .issue_auth_session(&AuthSessionIssue {
                user_id: "seed".into(),
                identity_provider: "genesis-test".into(),
                identity_subject_digest: crate::directory::identity_subject_digest("genesis-test", "seed").unwrap(),
                ttl_secs: 60,
                device_instance_id: "genesis-device".into(),
                session_kind: AuthSessionKind::DevelopmentLocal,
                correlation_id: "genesis-session".into(),
                peer_class: "loopback-test".into(),
            })
            .await
            .unwrap();
        let source: serde_json::Value = serde_json::from_str(include_str!("../../../🗿️artifact-authority/🌱️creation/🧫️fixtures/📚️operation-v1/🔣️.json")).unwrap();
        let mut intent: ArtifactCreationIntentV1 = directory::os_pack::json::from_json_str(&source["intent"].to_string()).unwrap();
        let mut prepared: ArtifactCreationPreparedV1 = directory::os_pack::json::from_json_str(&source["prepared"].to_string()).unwrap();
        intent.actor = ArtifactCreationActorV1 { user_id: "seed".into(), session_id: issued.record.id, authorization_generation: issued.record.authorization_generation };
        intent.scope.space_id = "default".into();
        intent.accepted_at_ms = now_ms() as u64;
        intent.deadline_ms = intent.accepted_at_ms + ARTIFACT_CREATION_DEADLINE_MS;
        intent.command_sha256 = artifact_creation_command_digest_v1(&intent.scope.space_id, &intent.request).unwrap();
        prepared.descriptor.space_id = intent.scope.space_id.clone();
        prepared.checkpoint.scope = intent.scope.clone();
        prepared.checkpoint.descriptor_digest_v1 = directory::os_directory::descriptor_digest_v1(&prepared.descriptor).unwrap();
        prepared.checkpoint.published_at_ms = intent.accepted_at_ms;
        prepared.checkpoint.checkpoint_id = ArtifactHash(semio_framework_hash::Sha256::digest(&checkpoint_id_encoding_v1(&prepared.checkpoint).unwrap()));
        prepared.validate(&intent).unwrap();
        assert!(matches!(directory.claim_artifact_creation(&intent).await.unwrap(), ArtifactCreationClaimV1::Accepted(_)));
        directory
            .append_artifact_creation_fact(&ArtifactCreationFactAppendV1 {
                actor: intent.actor.clone(),
                space_id: intent.scope.space_id.clone(),
                request_id: intent.request.request_id.clone(),
                command_sha256: intent.command_sha256.clone(),
                expected_revision: 1,
                recorded_at_ms: now_ms() as u64,
                body: ArtifactCreationFactBodyV1::Prepared { candidate: prepared.clone() },
            })
            .await
            .unwrap();
        let mut checkpoint = prepared.checkpoint.clone();
        checkpoint.pack.storage_key = artifact_cas_manifest_locator_v1(prepare_artifact_cas_manifest_v1(&intent.scope.space_id, &prepared.pack).unwrap().manifest_id);
        checkpoint.spr.storage_key = artifact_cas_manifest_locator_v1(prepare_artifact_cas_manifest_v1(&intent.scope.space_id, &prepared.spr).unwrap().manifest_id);
        let plan = prepare_artifact_cas_ownership_v1(&checkpoint, &ArtifactPair { pack: prepared.pack.clone(), spr: prepared.spr.clone() }).unwrap();
        let reservation = directory.reserve_artifact_cas(&plan, intent.deadline_ms, now_ms() as u64).await.unwrap();
        Self { directory: std::sync::Arc::new(directory), container, intent, prepared, checkpoint, reservation }
    }
}

fn system() -> DirectoryActor {
    DirectoryActor { kind: DirectoryActorKind::System, id: "system:artifact-creation".into() }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn genesis_commit_ack_and_locked_expiry_are_atomic_postgres() {
    let committed = GenesisFixture::create().await;
    committed.directory.genesis_test_control.fail_commit_ack.store(true, std::sync::atomic::Ordering::SeqCst);
    let head = committed.directory.head_seq().await.unwrap();
    assert!(matches!(committed.directory.append_document_genesis(&committed.packet()).await.unwrap(), DocumentGenesisCommitV1::Indeterminate));
    let operation = committed.operation().await;
    assert_eq!(operation.phase, SpaceArtifactCreationPhaseV1::Ready);
    let events = committed.directory.events_since(head, 3).await.unwrap();
    assert_eq!(events.len(), 3);
    assert_eq!(operation.receipt.as_ref().unwrap().event_ids, events.iter().map(|event| event.id.clone()).collect::<Vec<_>>());
    assert!(matches!(committed.directory.append_document_genesis(&committed.packet()).await.unwrap(), DocumentGenesisCommitV1::Existing(_)));

    let expired = GenesisFixture::create().await;
    let expires_at = now_ms() + 60_000;
    sqlx_core::query::query("UPDATE hub_auth_session SET expires_at = $2 WHERE id = $1").bind(&expired.intent.actor.session_id).bind(expires_at).execute(&expired.directory.pool).await.unwrap();
    let head = expired.directory.head_seq().await.unwrap();
    expired.directory.genesis_test_control.pause_before_authority.store(true, std::sync::atomic::Ordering::SeqCst);
    let directory = expired.directory.clone();
    let packet = expired.packet();
    let append = tokio::spawn(async move { directory.append_document_genesis(&packet).await });
    tokio::time::timeout(std::time::Duration::from_secs(5), expired.directory.genesis_test_control.reached_before_authority.acquire()).await.unwrap().unwrap().forget();
    expired.directory.genesis_test_control.observed_now_ms.store(u64::try_from(expires_at + 1).unwrap(), std::sync::atomic::Ordering::SeqCst);
    expired.directory.genesis_test_control.resume_before_authority.add_permits(1);
    assert!(matches!(append.await.unwrap(), Err(DirectoryError::Unauthorized)));
    assert_eq!(expired.directory.head_seq().await.unwrap(), head);
    assert_eq!(expired.operation().await.phase, SpaceArtifactCreationPhaseV1::Preparing);
    sqlx_core::query::query("UPDATE hub_auth_session SET expires_at = 0 WHERE id = $1").bind(&expired.intent.actor.session_id).execute(&expired.directory.pool).await.unwrap();

    let secondary = expired.container.connect().await;
    let barrier = std::sync::Arc::new(tokio::sync::Barrier::new(3));
    let first = {
        let directory = expired.directory.clone();
        let intent = expired.intent.clone();
        let barrier = barrier.clone();
        tokio::spawn(async move {
            barrier.wait().await;
            directory.artifact_creation_terminate_uncommitted(&intent, now_ms() as u64).await
        })
    };
    let second = {
        let intent = expired.intent.clone();
        let barrier = barrier.clone();
        tokio::spawn(async move {
            barrier.wait().await;
            secondary.artifact_creation_terminate_uncommitted(&intent, now_ms() as u64).await
        })
    };
    barrier.wait().await;
    assert_eq!(first.await.unwrap().unwrap().phase, SpaceArtifactCreationPhaseV1::Failed);
    assert_eq!(second.await.unwrap().unwrap().phase, SpaceArtifactCreationPhaseV1::Failed);
    assert_eq!(expired.directory.read_artifact_creation(&expired.intent.actor.user_id, &expired.intent.request.request_id).await.unwrap().len(), 3);
    println!("[DEBUG] PostgreSQL genesis: post-commit ACK loss=1 exact receipt/events=3 retry existing=1 lock-crossed expiry refusal=1 two-service terminalization=2 facts=3; no DirectoryService broadcast or real factory execution");
}
