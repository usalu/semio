//! 🧪️ Neutral genesis transaction cuts run against real SQLite writers and independent row reads.

use super::*;
use crate::artifact_authority::chunk_cas::{ArtifactChunkBlobStore, ArtifactChunkCasStorage, FsArtifactChunkCasStorage, artifact_cas_manifest_locator_v1, prepare_artifact_cas_manifest_v1, prepare_artifact_cas_ownership_v1};
use crate::artifact_authority::creation::{ARTIFACT_CREATION_DEADLINE_MS, ArtifactCreationPreparedV1, artifact_creation_command_digest_v1};
use crate::artifact_authority::{ArtifactBlobIntegrity, ArtifactPair, AuthorityLimits, AuthorityOperationControl, AuthorityProgress, ImmutableArtifactBlobStore, OperationContext, checkpoint_id_encoding_v1};
use crate::directory::{DirectoryService, HubDirectories, published_artifact_checkpoint};
use directory::os_directory::schema::space_artifact_creation::SpaceArtifactCreationPhaseV1;
use semio_framework_hash::Sha256;

struct GenesisFixture {
    backend: Arc<HubDirectories>,
    service: DirectoryService,
    intent: ArtifactCreationIntentV1,
    prepared: ArtifactCreationPreparedV1,
    checkpoint: ArtifactCheckpoint,
    reservation: ArtifactCasReservation,
}

impl GenesisFixture {
    fn sqlite(&self) -> &SqliteDirectory {
        match self.backend.as_ref() {
            HubDirectories::Sqlite(backend) => backend,
            #[allow(unreachable_patterns)]
            _ => panic!("SQLite fixture has another backend"),
        }
    }

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

    fn snapshot(&self) -> Vec<Vec<Vec<String>>> {
        let conn = self.sqlite().lock().unwrap();
        [
            "hub_directory_event",
            "hub_document_descriptor",
            "hub_document_index",
            "hub_artifact_checkpoint",
            "hub_artifact_checkpoint_private",
            "hub_artifact_authority_journal",
            "hub_artifact_cas_ledger_head",
            "hub_artifact_cas_ledger_journal",
            "hub_artifact_cas_reservation",
            "hub_artifact_cas_reservation_object",
            "hub_artifact_cas_reference",
            "hub_artifact_cas_reference_object",
            "hub_artifact_creation_fact",
        ]
        .iter()
        .map(|table| {
            let mut query = conn.prepare(&format!("SELECT * FROM {table} ORDER BY rowid")).unwrap();
            let columns = query.column_count();
            query.query_map([], |row| (0..columns).map(|index| row.get_ref(index).map(|value| format!("{value:?}"))).collect::<rusqlite::Result<Vec<_>>>()).unwrap().collect::<rusqlite::Result<Vec<_>>>().unwrap()
        })
        .collect()
    }

    async fn operation(&self) -> ArtifactCreationOperationV1 {
        ArtifactCreationOperationV1::fold(&self.sqlite().read_artifact_creation(&self.intent.actor.user_id, &self.intent.request.request_id).await.unwrap()).unwrap()
    }

    async fn accepted() -> (SqliteDirectory, ArtifactCreationIntentV1, ArtifactCreationPreparedV1) {
        Self::accepted_at(":memory:").await
    }

    async fn accepted_at(path: &str) -> (SqliteDirectory, ArtifactCreationIntentV1, ArtifactCreationPreparedV1) {
        let sqlite = SqliteDirectory::connect(path).await.unwrap();
        sqlite.seed().await.unwrap();
        let issued = sqlite
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
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🗿️artifact-authority/🌱️creation/🧫️fixtures/📚️operation-v1/🔣️.json")).unwrap();
        let mut intent: ArtifactCreationIntentV1 = directory::os_pack::json::from_json_str(&fixture["intent"].to_string()).unwrap();
        let mut prepared: ArtifactCreationPreparedV1 = directory::os_pack::json::from_json_str(&fixture["prepared"].to_string()).unwrap();
        intent.actor = ArtifactCreationActorV1 { user_id: "seed".into(), session_id: issued.record.id, authorization_generation: issued.record.authorization_generation };
        intent.scope.space_id = "default".into();
        intent.accepted_at_ms = now_ms() as u64;
        intent.deadline_ms = intent.accepted_at_ms + ARTIFACT_CREATION_DEADLINE_MS;
        intent.command_sha256 = artifact_creation_command_digest_v1(&intent.scope.space_id, &intent.request).unwrap();
        prepared.descriptor.space_id = intent.scope.space_id.clone();
        prepared.checkpoint.scope = intent.scope.clone();
        prepared.checkpoint.descriptor_digest_v1 = directory::os_directory::descriptor_digest_v1(&prepared.descriptor).unwrap();
        prepared.checkpoint.published_at_ms = intent.accepted_at_ms;
        prepared.checkpoint.checkpoint_id = ArtifactHash(Sha256::digest(&checkpoint_id_encoding_v1(&prepared.checkpoint).unwrap()));
        prepared.validate(&intent).unwrap();
        assert!(matches!(sqlite.claim_artifact_creation(&intent).await.unwrap(), ArtifactCreationClaimV1::Accepted(_)));
        assert!(matches!(sqlite.claim_artifact_creation(&intent).await.unwrap(), ArtifactCreationClaimV1::Existing(_)));
        (sqlite, intent, prepared)
    }

    async fn create() -> Self {
        Self::create_at(":memory:").await
    }

    async fn create_at(path: &str) -> Self {
        let (sqlite, intent, prepared) = Self::accepted_at(path).await;
        sqlite
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
        let backend = Arc::new(HubDirectories::Sqlite(sqlite));
        let service = DirectoryService::new(backend.clone(), 32);
        let reservation = service.reserve_artifact_cas(system(), plan, intent.deadline_ms, now_ms() as u64).await.unwrap();
        Self { backend, service, intent, prepared, checkpoint, reservation }
    }
}

fn system() -> DirectoryActor {
    DirectoryActor { kind: DirectoryActorKind::System, id: "system:artifact-creation".into() }
}

struct GenesisControl;

impl AuthorityOperationControl for GenesisControl {
    fn now_ms(&self) -> u64 {
        now_ms() as u64
    }
    fn is_cancelled(&self) -> bool {
        false
    }
    fn report(&self, _progress: AuthorityProgress) {}
}

#[tokio::test]
async fn genesis_physical_pair_and_sqlite_restart_preserve_exact_receipt() {
    let generated = std::path::PathBuf::from(std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").expect("ticket generated artifact root"));
    let root = generated.join(format!("genesis-restart-{}", time_ordered_id()));
    std::fs::create_dir_all(&root).unwrap();
    let db_path = root.join("directory.sqlite");
    let cas_path = root.join("artifact-cas");
    let f = GenesisFixture::create_at(db_path.to_str().unwrap()).await;
    let context = OperationContext::new(f.intent.deadline_ms, AuthorityLimits::maximum(), &GenesisControl);
    let storage = Arc::new(FsArtifactChunkCasStorage::open(&cas_path).await.unwrap());
    storage.configure_coordinator(*f.reservation.coordinator_id(), &context).await.unwrap();
    storage.advance_physical_epoch(*f.reservation.coordinator_id(), &f.intent.scope.space_id, f.reservation.physical_epoch(), &context).await.unwrap();
    let blobs = ArtifactChunkBlobStore::new(storage.clone());
    let pack = blobs.stage(&f.intent.scope.space_id, ArtifactBlobIntegrity { sha256: f.checkpoint.pack.sha256, byte_length: f.checkpoint.pack.byte_length }, &f.prepared.pack, &context).await.unwrap();
    let spr = blobs.stage(&f.intent.scope.space_id, ArtifactBlobIntegrity { sha256: f.checkpoint.spr.sha256, byte_length: f.checkpoint.spr.byte_length }, &f.prepared.spr, &context).await.unwrap();
    assert_eq!(pack.storage_key, f.checkpoint.pack.storage_key);
    assert_eq!(spr.storage_key, f.checkpoint.spr.storage_key);
    assert_eq!(blobs.read(&f.intent.scope.space_id, &pack, &context).await.unwrap(), f.prepared.pack);
    assert_eq!(blobs.read(&f.intent.scope.space_id, &spr, &context).await.unwrap(), f.prepared.spr);
    let head = f.sqlite().head_seq().await.unwrap();
    let operation = f.service.publish_document_genesis(f.intent.clone(), &f.prepared, f.checkpoint.clone(), f.reservation.clone(), now_ms() as u64).await.unwrap();
    let receipt = operation.receipt.clone().unwrap();
    let intent = f.intent.clone();
    let expected = f.prepared.clone();
    let checkpoint = f.checkpoint.clone();
    drop(blobs);
    drop(storage);
    drop(f);
    let sqlite = SqliteDirectory::connect(db_path.to_str().unwrap()).await.unwrap();
    let blobs = ArtifactChunkBlobStore::new(Arc::new(FsArtifactChunkCasStorage::open(&cas_path).await.unwrap()));
    let generation = sqlite.artifact_cas_ledger_generation().await.unwrap();
    for stage in ["reopened", "rebuilt"] {
        if stage == "rebuilt" {
            sqlite.rebuild_projections().await.unwrap();
        }
        assert_eq!(sqlite.artifact_cas_ledger_generation().await.unwrap(), generation, "{stage}");
        let facts = sqlite.read_artifact_creation(&intent.actor.user_id, &intent.request.request_id).await.unwrap();
        assert_eq!(ArtifactCreationOperationV1::fold(&facts).unwrap(), operation, "{stage}");
        assert_eq!(sqlite.head_seq().await.unwrap(), head + 3, "{stage}");
        let events = sqlite.events_since(head, 3).await.unwrap();
        assert_eq!(events.iter().map(|event| event.id.clone()).collect::<Vec<_>>(), receipt.event_ids, "{stage}");
        assert_eq!(sqlite.get_document_descriptor(&intent.scope).await.unwrap(), Some(expected.descriptor.clone()));
        assert_eq!(sqlite.get_verified_artifact_checkpoint(&intent.scope, checkpoint.checkpoint_id).await.unwrap(), Some(checkpoint.clone()));
        assert_eq!(blobs.read(&intent.scope.space_id, &pack, &context).await.unwrap(), expected.pack, "{stage}");
        assert_eq!(blobs.read(&intent.scope.space_id, &spr, &context).await.unwrap(), expected.spr, "{stage}");
    }
    drop(blobs);
    drop(sqlite);
    std::fs::remove_dir_all(&root).unwrap();
    println!("[DEBUG] SQLite genesis physical restart: neutral pair=1 exact CAS readbacks=6 descriptor/checkpoint/receipt/head/ledger preserved=2 same-root reopen=1 projection rebuild=1; real codec and Hub process restart not executed");
}

#[tokio::test]
async fn genesis_accepted_only_recovery_has_no_prepared_or_public_side_effects() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🗿️artifact-authority/🌱️creation/🧫️fixtures/🧪️transaction-v1/🧯️accepted-recovery.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let (sqlite, intent, _) = GenesisFixture::accepted().await;
        let head = sqlite.head_seq().await.unwrap();
        let kind = row["kind"].as_str().unwrap();
        if kind == "revoked" {
            sqlite.revoke_auth_session(&intent.actor.session_id, "test", Some("seed"), "accepted-recovery").await.unwrap();
        }
        if kind == "expired" {
            let remaining = intent.deadline_ms.saturating_sub(now_ms() as u64);
            println!("[DEBUG] Accepted-only recovery waits {remaining}ms for its real persisted 30-second deadline; no factory or CAS work scheduled");
            tokio::time::sleep(std::time::Duration::from_millis(remaining + 1)).await;
            assert!(now_ms() as u64 >= intent.deadline_ms);
        }
        let candidates = sqlite.artifact_creation_recovery_candidates(now_ms() as u64, 32).await.unwrap();
        assert_eq!(candidates.len() as u64, row["recoveryCandidates"].as_u64().unwrap(), "{kind}");
        if !candidates.is_empty() {
            assert_eq!(candidates[0], intent);
        }
        let result = sqlite.artifact_creation_terminate_uncommitted(&intent, now_ms() as u64).await;
        assert_eq!(result.is_err(), kind == "live", "{kind}");
        let facts = sqlite.read_artifact_creation(&intent.actor.user_id, &intent.request.request_id).await.unwrap();
        let operation = ArtifactCreationOperationV1::fold(&facts).unwrap();
        assert_eq!(facts.len() as u64, row["facts"].as_u64().unwrap(), "{kind}");
        assert_eq!(format!("{:?}", operation.phase).to_lowercase(), row["terminal"].as_str().unwrap(), "{kind}");
        assert!(operation.prepared.is_none() && operation.receipt.is_none());
        assert_eq!(sqlite.head_seq().await.unwrap(), head);
        let conn = sqlite.lock().unwrap();
        for table in ["hub_document_descriptor", "hub_document_index", "hub_artifact_checkpoint", "hub_artifact_authority_journal", "hub_artifact_cas_ledger_journal", "hub_artifact_cas_reservation", "hub_artifact_cas_reference"] {
            assert_eq!(conn.query_row(&format!("SELECT COUNT(*) FROM {table}"), [], |row| row.get::<_, i64>(0)).unwrap(), 0, "{kind}: {table}");
        }
        drop(conn);
        if kind != "live" {
            assert_eq!(sqlite.artifact_creation_terminate_uncommitted(&intent, now_ms() as u64).await.unwrap(), operation);
            assert_eq!(sqlite.read_artifact_creation(&intent.actor.user_id, &intent.request.request_id).await.unwrap(), facts);
        }
    }
    println!("[DEBUG] SQLite Accepted-only recovery: neutral cases=3 actual deadline=30000ms terminal retries=2 no prepared/private/public/CAS rows=21; real factory and HTTP supervisor not executed");
}

#[tokio::test]
async fn genesis_neutral_transactions_are_atomic_replayable_and_authorized() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🗿️artifact-authority/🌱️creation/🧫️fixtures/🧪️transaction-v1/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let kind = row["kind"].as_str().unwrap();
        let f = GenesisFixture::create().await;
        let mut packet = f.packet();
        let table = match kind {
            "fault-descriptor" => Some("hub_document_descriptor"),
            "fault-index" => Some("hub_document_index"),
            "fault-checkpoint" => Some("hub_artifact_checkpoint"),
            "fault-authority" => Some("hub_artifact_authority_journal"),
            "fault-cas-ledger" => Some("hub_artifact_cas_ledger_journal"),
            "fault-cas-reference" => Some("hub_artifact_cas_reference"),
            "fault-completion" => Some("hub_artifact_creation_fact"),
            _ => None,
        };
        if let Some(table) = table {
            f.sqlite().lock().unwrap().execute_batch(&format!("CREATE TEMP TRIGGER genesis_fault BEFORE INSERT ON {table} BEGIN SELECT RAISE(ABORT, 'injected genesis cut'); END;")).unwrap();
        }
        if kind == "fault-index-missing" {
            f.sqlite().lock().unwrap().execute_batch("CREATE TEMP TRIGGER genesis_index_fault AFTER INSERT ON hub_document_index BEGIN DELETE FROM hub_document_index WHERE space_id = NEW.space_id AND document_id = NEW.document_id; END;").unwrap();
        }
        if kind == "fault-index-digest" {
            let digest = serde_json::to_string(&vec![0; 32]).unwrap();
            f.sqlite().lock().unwrap().execute_batch(&format!("CREATE TEMP TRIGGER genesis_index_fault AFTER INSERT ON hub_document_index BEGIN UPDATE hub_document_index SET payload = json_set(NEW.payload, '$.descriptorDigestV1', json('{digest}')) WHERE space_id = NEW.space_id AND document_id = NEW.document_id; END;")).unwrap();
        }
        match kind {
            "revoked-session" => {
                f.sqlite().revoke_auth_session(&f.intent.actor.session_id, "test", Some("seed"), "genesis-revoke").await.unwrap();
            }
            "expired-session" => {
                f.sqlite().lock().unwrap().execute("UPDATE hub_auth_session SET expires_at = 0 WHERE id = ?1", [&f.intent.actor.session_id]).unwrap();
            }
            "changed-generation" => {
                f.sqlite().lock().unwrap().execute("UPDATE hub_auth_session SET authorization_generation = authorization_generation + 1 WHERE id = ?1", [&f.intent.actor.session_id]).unwrap();
            }
            "archived-space" => {
                f.sqlite().lock().unwrap().execute("UPDATE hub_space SET kind = 'archive' WHERE id = 'default'", []).unwrap();
            }
            "removed-member" => {
                f.sqlite().lock().unwrap().execute("DELETE FROM hub_space_membership WHERE space_id = 'default' AND user_id = 'seed'", []).unwrap();
            }
            "spectator" => {
                f.sqlite().lock().unwrap().execute("UPDATE hub_space_membership SET role = 'spectator' WHERE space_id = 'default' AND user_id = 'seed'", []).unwrap();
            }
            "future-clock" => packet.now_ms = f.intent.deadline_ms + 1,
            "wrong-event-order" => packet.events.swap(0, 1),
            "wrong-event-actor" => packet.events[0].actor = system(),
            "wrong-event-user" => packet.events[0].user_id = Some("other".into()),
            "wrong-event-space" => packet.events[0].space_id = Some("other".into()),
            "wrong-index-name" => {
                if let DirectoryEventBody::DocumentIndexed { entry, .. } = &mut packet.events[1].body {
                    entry.name = "Other".into();
                }
            }
            "wrong-checkpoint-id" => packet.checkpoint.checkpoint_id = ArtifactHash([1; 32]),
            "wrong-pack-locator" => packet.checkpoint.pack.storage_key.push('x'),
            "wrong-spr-locator" => packet.checkpoint.spr.storage_key.push('x'),
            "wrong-reservation-generation" => packet.reservation.generation += 1,
            "wrong-reservation-epoch" => packet.reservation.write_epoch += 1,
            "wrong-reservation-expiry" => packet.reservation.expires_at_ms += 1,
            "cancelled" => {
                f.sqlite()
                    .append_artifact_creation_fact(&ArtifactCreationFactAppendV1 {
                        actor: f.intent.actor.clone(),
                        space_id: f.intent.scope.space_id.clone(),
                        request_id: f.intent.request.request_id.clone(),
                        command_sha256: f.intent.command_sha256.clone(),
                        expected_revision: 2,
                        recorded_at_ms: now_ms() as u64,
                        body: ArtifactCreationFactBodyV1::Cancelled,
                    })
                    .await
                    .unwrap();
            }
            "ack-loss" => f.sqlite().fail_next_genesis_commit_ack(),
            "ack-receipt-unreadable" => f.sqlite().fail_next_genesis_reconciliation(false),
            "ack-events-unreadable" => f.sqlite().fail_next_genesis_reconciliation(true),
            _ => {}
        }
        let before = f.snapshot();
        let head = f.sqlite().head_seq().await.unwrap();
        let mut messages = f.service.subscribe();
        let mut invalidations = f.service.subscribe_delivery_invalidations();
        let epoch = f.service.delivery_epoch(Some("default"));
        if row["committed"] == true {
            let result = f.service.publish_document_genesis(f.intent.clone(), &f.prepared, f.checkpoint.clone(), f.reservation.clone(), now_ms() as u64).await;
            let reconnect = row["delivery"] == "reconnect";
            assert_eq!(result.is_err(), reconnect, "{kind}");
            let operation = if reconnect { f.operation().await } else { result.unwrap() };
            assert_eq!(operation.phase, SpaceArtifactCreationPhaseV1::Ready, "{kind}");
            let receipt = operation.receipt.unwrap();
            let events = f.sqlite().events_since(head, 3).await.unwrap();
            assert_eq!(receipt.event_ids, events.iter().map(|event| event.id.clone()).collect::<Vec<_>>(), "{kind}");
            for event in events.iter().filter(|_| !reconnect) {
                let directory::os_directory::DirectoryStreamMessage::Event { event: observed } = messages.try_recv().unwrap() else { panic!("expected creation event") };
                assert_eq!(*observed, *event);
            }
            assert!(messages.try_recv().is_err());
            if reconnect {
                assert_eq!(invalidations.try_recv().unwrap(), "default");
                assert!(f.service.delivery_epoch(Some("default")) > epoch);
                assert!(f.service.delivery_epoch(None) > epoch);
                assert_eq!(f.service.delivery_epoch(Some("unaffected")), 0);
                assert!(f.service.acquire_delivery_lease(Some("default"), epoch).await.is_none());
                assert!(f.service.acquire_delivery_lease(Some("unaffected"), 0).await.is_some());
            } else {
                assert!(invalidations.try_recv().is_err());
                assert_eq!(f.service.delivery_epoch(Some("default")), epoch);
            }
            let committed = f.snapshot();
            assert!(matches!(f.sqlite().append_document_genesis(&f.packet()).await.unwrap(), DocumentGenesisCommitV1::Existing(_)));
            f.service.publish_document_genesis(f.intent.clone(), &f.prepared, f.checkpoint.clone(), f.reservation.clone(), now_ms() as u64).await.unwrap();
            assert_eq!(f.snapshot(), committed, "{kind}: replay changed durable state");
            assert!(messages.try_recv().is_err(), "{kind}: replay rebroadcast");
            assert_eq!(f.sqlite().get_document_descriptor(&f.intent.scope).await.unwrap(), Some(f.prepared.descriptor.clone()));
            assert_eq!(f.sqlite().get_verified_artifact_checkpoint(&f.intent.scope, f.checkpoint.checkpoint_id).await.unwrap(), Some(f.checkpoint.clone()));
            f.sqlite().rebuild_projections().await.unwrap();
            assert_eq!(f.snapshot(), committed, "{kind}: rebuild changed exact state");
        } else {
            assert!(f.sqlite().append_document_genesis(&packet).await.is_err(), "{kind}");
            assert_eq!(f.snapshot(), before, "{kind}: refused transaction mutated durable state");
            assert!(messages.try_recv().is_err(), "{kind}: refused transaction broadcast");
        }
        assert_eq!(f.sqlite().head_seq().await.unwrap() - head, row["publicEvents"].as_u64().unwrap(), "{kind}");
        let operation = f.operation().await;
        let independent: serde_json::Value = serde_json::from_str(&directory::os_pack::json::to_json_string(&operation.status())).unwrap();
        let recovered: directory::os_directory::schema::space_artifact_creation::SpaceArtifactCreationStatusV1 = directory::os_pack::json::from_json_str(&serde_json::to_string(&independent).unwrap()).unwrap();
        assert_eq!(recovered, operation.status(), "{kind}: independent JSON semantic roundtrip");
        assert_eq!(format!("{:?}", operation.phase).to_lowercase(), row["terminal"].as_str().unwrap(), "{kind}");
        if matches!(kind, "revoked-session" | "expired-session" | "changed-generation" | "archived-space" | "removed-member" | "spectator") {
            let terminal = f.sqlite().artifact_creation_terminate_uncommitted(&f.intent, now_ms() as u64).await.unwrap();
            assert_eq!(terminal.phase, SpaceArtifactCreationPhaseV1::Failed, "{kind}: revoked recovery");
            assert_eq!(f.sqlite().head_seq().await.unwrap(), head, "{kind}: recovery published events");
            assert_eq!(f.sqlite().artifact_creation_terminate_uncommitted(&f.intent, now_ms() as u64).await.unwrap(), terminal);
        }
    }
    println!(
        "[DEBUG] SQLite genesis: neutral transactions={} rollback cuts=7 index binding cuts=2 authority refusals=6 exact receipt/replay/rebuild=4 ack-loss ordered broadcast=1 forced replay epochs=2 revoked recovery=6; socket reconnect, physical CAS staging and real factory not executed",
        fixture["cases"].as_array().unwrap().len()
    );
}

#[tokio::test]
async fn genesis_delivery_lease_serializes_invalidation_and_requires_replay() {
    use std::future::Future;
    use std::task::Poll;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🗿️artifact-authority/🌱️creation/🧫️fixtures/🧪️transaction-v1/🔣️.json")).unwrap();
    assert_eq!(fixture["cases"].as_array().unwrap().iter().filter(|row| row["delivery"] == "reconnect").count(), 2);
    let f = GenesisFixture::create().await;
    let mut invalidations = f.service.subscribe_delivery_invalidations();
    let epoch = f.service.delivery_epoch(Some("default"));
    let lease = f.service.acquire_delivery_lease(Some("default"), epoch).await.unwrap();
    let clock = f.service.write.lock().await;
    let mut invalidate = Box::pin(f.service.invalidate_delivery_locked(&clock, "default"));
    std::future::poll_fn(|context| match invalidate.as_mut().poll(context) {
        Poll::Pending => Poll::Ready(()),
        Poll::Ready(()) => panic!("invalidation crossed an active send lease"),
    })
    .await;
    assert_eq!(f.service.delivery_epoch(Some("default")), epoch);
    assert!(invalidations.try_recv().is_err());
    drop(lease);
    invalidate.await;
    assert_eq!(invalidations.try_recv().unwrap(), "default");
    assert!(f.service.acquire_delivery_lease(Some("default"), epoch).await.is_none());
    assert!(f.service.acquire_delivery_lease(None, epoch).await.is_none());
    assert!(f.service.acquire_delivery_lease(Some("other"), 0).await.is_some());
    let current = f.service.delivery_epoch(Some("default"));
    assert!(f.service.acquire_delivery_lease(Some("default"), current).await.is_some());
    for index in 0..crate::directory::DIRECTORY_DELIVERY_SCOPE_MAX {
        f.service.invalidate_delivery_locked(&clock, &format!("space-{index}")).await;
    }
    assert!(f.service.delivery_epochs.lock().unwrap().2.len() <= crate::directory::DIRECTORY_DELIVERY_SCOPE_MAX);
    assert!(f.service.delivery_epoch(Some("other")) > current);
    assert!(f.service.acquire_delivery_lease(Some("other"), 0).await.is_none());
    println!("[DEBUG] Directory delivery lease: pending invalidation=1 released send=1 scoped/global stale refusal=2 unrelated space retained=1 new replay epoch admitted=1 bounded-scope fallback=4096; no real socket send executed");
}
