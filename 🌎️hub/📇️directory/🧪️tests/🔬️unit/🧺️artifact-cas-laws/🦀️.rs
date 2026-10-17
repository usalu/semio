/// 🧺️ Laws of artifact chunk CAS retention, sweeps and races, checkpoint publication and the exact bounds of public scalars.

#[tokio::test]
async fn artifact_chunk_cas_retention_space_delete_and_dry_run_preserve_live_bytes() {
    let (template_descriptor, first_public, second_public, _, _) = artifact_projection_fixture();
    let dir = fresh_dir().await;
    let service = DirectoryService::new(dir.clone(), 64);
    let owner = user_actor("u-owner");
    let space_id = create_space(&service, &owner, DirectorySpaceKind::Studio).await;
    let mut descriptor = template_descriptor;
    descriptor.space_id = space_id.clone();
    descriptor.document_id = "artifact-00000000000000000000000000000002".into();
    let system = DirectoryActor { kind: DirectoryActorKind::System, id: "system:artifact-authority".into() };
    let first_pair = ArtifactPair { pack: b"shared-pack".to_vec(), spr: b"old-spr".to_vec() };
    let second_pair = ArtifactPair { pack: first_pair.pack.clone(), spr: b"new-spr".to_vec() };
    let generic_pool = Arc::new(db::semio_framework_async::process_worker_pool(db::semio_framework_async::WorkerPoolConfig::new(db::semio_framework_async::ProcessKind::HeadlessBatch, 2)));
    let generic = GenericMemoryStorage::new(generic_pool).await.expect("open generic payload storage");
    let generic_hash = generic.put(generic_payload_pages(&first_pair.spr)).await.expect("seed identical generic payload bytes");
    let storage = Arc::new(MemoryArtifactChunkCasStorage::default());
    let control = ArtifactCasProbe::new(100, None);
    let context = OperationContext::new(10_000, AuthorityLimits::maximum(), &control);

    let (descriptor, genesis) = publish_fixture_genesis(&service, "u-owner", descriptor, storage.clone(), &context).await;
    let first = scoped_materialized_checkpoint(first_public, &descriptor, &first_pair, Some(genesis.checkpoint_id), 1);
    let second = scoped_materialized_checkpoint(second_public, &descriptor, &second_pair, Some(first.checkpoint_id), 2);

    let first_reservation = stage_reserved_checkpoint(&service, storage.clone(), system.clone(), &first, &first_pair, 1_000, 100, &context).await.expect("reserve and stage first");
    service.publish_reserved_artifact_checkpoint(system.clone(), first.clone(), first_reservation, 100).await.expect("publish first");
    let second_reservation = stage_reserved_checkpoint(&service, storage.clone(), system.clone(), &second, &second_pair, 1_001, 101, &context).await.expect("reserve and stage second");
    service.publish_reserved_artifact_checkpoint(system.clone(), second.clone(), second_reservation, 101).await.expect("publish second");
    let retention = ArtifactRetention { scope: second.scope.clone(), retained_checkpoint_id: second.checkpoint_id, retained_floor: second.baseline_frontier.clone(), checkpoint_lineage_head: second.checkpoint_id };
    service.execute_artifact_authority(system.clone(), ArtifactDirectoryCommand::AdvanceRetention { retention }).await.expect("advance retention");
    assert_eq!(dir.get_verified_artifact_checkpoint(&first.scope, first.checkpoint_id).await.expect("released private checkpoint"), None);
    assert_eq!(dir.get_verified_artifact_checkpoint(&second.scope, second.checkpoint_id).await.expect("live private checkpoint"), Some(second.clone()));
    assert!(matches!(service.reserve_artifact_cas(system.clone(), prepare_artifact_cas_ownership_v1(&first, &first_pair).expect("first ownership"), 2_000, 200).await, Err(DirectoryError::Conflict(_))));

    let old_spr = StagedArtifactBlob { storage_key: first.spr.storage_key.clone(), integrity: ArtifactBlobIntegrity { sha256: first.spr.sha256, byte_length: first.spr.byte_length } };
    let live_spr = StagedArtifactBlob { storage_key: second.spr.storage_key.clone(), integrity: ArtifactBlobIntegrity { sha256: second.spr.sha256, byte_length: second.spr.byte_length } };
    let blobs = ArtifactChunkBlobStore::new(storage.clone());
    let dry = service.sweep_artifact_cas(storage.as_ref(), ArtifactCasSweepRequest { execute: false, max_objects: ARTIFACT_CAS_SWEEP_OBJECT_MAX, continuation: None }, &context).await.expect("dry sweep");
    assert_eq!(dry.observed_generation, dry.final_generation);
    assert!(dry.eligible_objects >= 2);
    assert!(dry.protected_objects >= 2);
    assert_eq!(dry.deleted_objects, 0);
    assert_eq!(blobs.read(&space_id, &old_spr, &context).await.expect("dry run preserves released bytes"), first_pair.spr);

    let swept = service.sweep_artifact_cas(storage.as_ref(), ArtifactCasSweepRequest { execute: true, max_objects: ARTIFACT_CAS_SWEEP_OBJECT_MAX, continuation: None }, &context).await.expect("retention sweep");
    assert_eq!(swept.deleted_objects, swept.eligible_objects);
    assert!(blobs.read(&space_id, &old_spr, &context).await.is_err());
    assert_eq!(blobs.read(&space_id, &live_spr, &context).await.expect("retained checkpoint survives"), second_pair.spr);

    service.execute(owner, DirectoryCommand::DeleteSpace { space_id: space_id.clone() }).await.expect("delete space");
    assert_eq!(dir.get_active_artifact_checkpoint(&second.scope).await.expect("deleted active checkpoint"), None);
    let deleted = service.sweep_artifact_cas(storage.as_ref(), ArtifactCasSweepRequest { execute: true, max_objects: ARTIFACT_CAS_SWEEP_OBJECT_MAX, continuation: None }, &context).await.expect("space deletion sweep");
    assert!(deleted.deleted_objects >= 2);
    assert!(deleted.missing_objects >= 2);
    assert!(blobs.read(&space_id, &live_spr, &context).await.is_err());
    assert!(generic.contains(&generic_hash).await.expect("generic payload survives dedicated CAS sweep"));
}

#[tokio::test]
async fn artifact_chunk_cas_expiry_supersedes_tokens_and_sweep_cancellation_commits_one_delete() {
    let (template_descriptor, public, _, _, _) = artifact_projection_fixture();
    let dir = fresh_dir().await;
    let service = DirectoryService::new(dir, 64);
    let owner = user_actor("u-owner");
    let space_id = create_space(&service, &owner, DirectorySpaceKind::Studio).await;
    let mut descriptor = template_descriptor;
    descriptor.space_id = space_id;
    descriptor.document_id = "artifact-cas-expiry".into();
    service.execute(owner, DirectoryCommand::AnnounceDocument { descriptor: Box::new(descriptor.clone()) }).await.expect("announce expiry document");
    let pair = ArtifactPair { pack: b"expired-pack".to_vec(), spr: b"expired-spr".to_vec() };
    let checkpoint = scoped_materialized_checkpoint(public, &descriptor, &pair, None, 1);
    let plan = prepare_artifact_cas_ownership_v1(&checkpoint, &pair).expect("ownership");
    let system = DirectoryActor { kind: DirectoryActorKind::System, id: "system:artifact-authority".into() };
    let expired = service.reserve_artifact_cas(system.clone(), plan.clone(), 200, 100).await.expect("initial reservation");
    let preview_storage = MemoryArtifactChunkCasStorage::default();
    let preview_control = ArtifactCasProbe::new(201, None);
    let preview_context = OperationContext::new(10_000, AuthorityLimits::maximum(), &preview_control);
    let preview = service.sweep_artifact_cas(&preview_storage, ArtifactCasSweepRequest { execute: false, max_objects: ARTIFACT_CAS_SWEEP_OBJECT_MAX, continuation: None }, &preview_context).await.expect("mutation-free expiry preview");
    assert!(preview.eligible_objects > 0);
    let replacement = service.reserve_artifact_cas(system.clone(), plan.clone(), 400, 201).await.expect("replacement reservation");
    assert!(replacement.write_epoch > expired.write_epoch);
    assert_eq!(replacement.physical_epoch(), expired.physical_epoch() + 1, "dry-run does not consume a fence epoch");
    assert!(matches!(service.publish_reserved_artifact_checkpoint(system.clone(), checkpoint.clone(), expired, 201).await, Err(DirectoryError::Conflict(_))));
    let storage = Arc::new(MemoryArtifactChunkCasStorage::default());
    let stage_control = ArtifactCasProbe::new(201, None);
    let stage_context = OperationContext::new(10_000, AuthorityLimits::maximum(), &stage_control);
    let blobs = ArtifactChunkBlobStore::new(storage.clone());
    blobs.stage(&checkpoint.scope.space_id, ArtifactBlobIntegrity { sha256: checkpoint.pack.sha256, byte_length: checkpoint.pack.byte_length }, &pair.pack, &stage_context).await.expect("stage expired pack");
    blobs.stage(&checkpoint.scope.space_id, ArtifactBlobIntegrity { sha256: checkpoint.spr.sha256, byte_length: checkpoint.spr.byte_length }, &pair.spr, &stage_context).await.expect("stage expired SPR");
    let protected = service.sweep_artifact_cas(storage.as_ref(), ArtifactCasSweepRequest { execute: false, max_objects: ARTIFACT_CAS_SWEEP_OBJECT_MAX, continuation: None }, &stage_context).await.expect("replacement protects bytes");
    assert_eq!(protected.eligible_objects, 0);
    assert_eq!(protected.protected_objects, plan.objects.len() as u64);
    assert!(matches!(service.publish_reserved_artifact_checkpoint(system, checkpoint, replacement, 400).await, Err(DirectoryError::Conflict(_))));

    stage_control.now_ms.store(401, Ordering::SeqCst);
    let cancel = ArtifactCasProbe::new(401, Some(1));
    let cancel_context = OperationContext::new(10_000, AuthorityLimits::maximum(), &cancel);
    assert!(matches!(
        service.sweep_artifact_cas(storage.as_ref(), ArtifactCasSweepRequest { execute: true, max_objects: ARTIFACT_CAS_SWEEP_OBJECT_MAX, continuation: None }, &cancel_context).await,
        Err(semio_hub_artifact_authority::AuthorityError::Cancelled)
    ));
    let progress = cancel.progress.lock().expect("cancel progress");
    let swept: Vec<_> = progress.iter().filter(|item| item.stage == AuthorityProgressStage::CasSweep).copied().collect();
    assert_eq!(swept.len(), 1);
    assert_eq!(swept[0].completed_units, 1);
    drop(progress);
    let mut live_objects = 0usize;
    for key in &plan.objects {
        live_objects += usize::from(storage.get(key, &stage_context).await.is_ok());
    }
    assert_eq!(live_objects + 1, plan.objects.len());
}

#[tokio::test]
async fn artifact_chunk_cas_failed_epoch_advance_releases_directory_lease() {
    let dir = fresh_dir().await;
    let service = DirectoryService::new(dir, 64);
    let system = DirectoryActor { kind: DirectoryActorKind::System, id: "system:artifact-authority".into() };
    let plan = sweep_convergence_plan(9_999, 2);
    service.reserve_artifact_cas(system.clone(), plan.clone(), 200, 100).await.expect("expired cleanup reservation");
    let storage = FailingAdvanceArtifactCas(MemoryArtifactChunkCasStorage::default());
    let probe = ArtifactCasProbe::new(201, None);
    let context = OperationContext::new(10_000, AuthorityLimits::maximum(), &probe);
    assert!(matches!(service.sweep_artifact_cas(&storage, ArtifactCasSweepRequest { execute: true, max_objects: 1, continuation: None }, &context).await, Err(semio_hub_artifact_authority::AuthorityError::Cancelled)));
    service.reserve_artifact_cas(system, plan, 400, 201).await.expect("failure cleanup releases live lease immediately");
}

#[tokio::test]
async fn artifact_chunk_cas_two_service_sweep_and_reservation_race_is_serialized_before_rewrite() {
    let fixture: serde_json::Value = serde_json::from_str(semio_hub_testing::hub_source("🗿️artifact-authority/🧪️tests/🧫️fixtures/🧱️artifact-chunk-cas/🔣️.json")).expect("artifact CAS fixture");
    let barrier = &fixture["deleteBarrier"];
    assert_eq!(barrier["leaseMaximumMs"].as_u64(), Some(ARTIFACT_CAS_DELETE_LEASE_TTL_MS));
    assert_eq!(barrier["dryRunAdvancesEpoch"].as_bool(), Some(false));
    assert_eq!(barrier["orders"][1]["oldDeleteOutcome"].as_str(), Some("stale-fence-rejected"));
    assert_eq!(barrier["orders"][1]["publishedReadOutcome"].as_str(), Some("exact"));
    let (mut orphan_descriptor, orphan_public, live_public, _, _) = artifact_projection_fixture();
    let dir = fresh_dir().await;
    let service = Arc::new(DirectoryService::new(dir.clone(), 64));
    let owner = user_actor("u-owner");
    let space_id = create_space(service.as_ref(), &owner, DirectorySpaceKind::Studio).await;
    orphan_descriptor.space_id = space_id.clone();
    orphan_descriptor.document_id = "artifact-cas-race-orphan".into();
    let mut live_descriptor = orphan_descriptor.clone();
    live_descriptor.document_id = "artifact-00000000000000000000000000000003".into();
    service.execute(owner.clone(), DirectoryCommand::AnnounceDocument { descriptor: Box::new(orphan_descriptor.clone()) }).await.expect("announce orphan document");
    let pair = ArtifactPair { pack: b"race-pack".to_vec(), spr: b"race-spr".to_vec() };
    let orphan = scoped_materialized_checkpoint(orphan_public, &orphan_descriptor, &pair, None, 1);
    let storage = Arc::new(BlockingDeleteArtifactCas::new());
    let system = DirectoryActor { kind: DirectoryActorKind::System, id: "system:artifact-authority".into() };
    let stage_control = ArtifactCasProbe::new(100, None);
    let stage_context = OperationContext::new(10_000, AuthorityLimits::maximum(), &stage_control);
    let (live_descriptor, genesis) = publish_fixture_genesis(service.as_ref(), "u-owner", live_descriptor, storage.clone(), &stage_context).await;
    let live = scoped_materialized_checkpoint(live_public, &live_descriptor, &pair, Some(genesis.checkpoint_id), 1);
    stage_reserved_checkpoint(service.as_ref(), storage.clone(), system.clone(), &orphan, &pair, 200, 100, &stage_context).await.expect("stage orphan");
    let live_plan = prepare_artifact_cas_ownership_v1(&live, &pair).expect("live ownership");

    let sweep_service = Arc::new(DirectoryService::new(dir.clone(), 64));
    let sweep_storage = storage.clone();
    let mut sweep_tasks = tokio::task::JoinSet::new();
    sweep_tasks.spawn(async move {
        let control = ArtifactCasProbe::new(300, None);
        let context = OperationContext::new(10_000, AuthorityLimits::maximum(), &control);
        sweep_service.sweep_artifact_cas(sweep_storage.as_ref(), ArtifactCasSweepRequest { execute: true, max_objects: ARTIFACT_CAS_SWEEP_OBJECT_MAX, continuation: None }, &context).await
    });
    tokio::time::timeout(std::time::Duration::from_secs(30), storage.entered.notified()).await.expect("sweep reaches an eligible orphan object");
    let reserve_service = DirectoryService::new(dir, 64);
    assert!(matches!(reserve_service.reserve_artifact_cas(system.clone(), live_plan.clone(), 1_000, 300).await, Err(DirectoryError::Conflict(_))));
    let reservation = reserve_service.reserve_artifact_cas(system.clone(), live_plan, 10_000, 5_301).await.expect("reserve after deletion lease expiry");
    let rewrite_control = ArtifactCasProbe::new(5_301, None);
    let rewrite_context = OperationContext::new(20_000, AuthorityLimits::maximum(), &rewrite_control);
    storage.advance_physical_epoch(*reservation.coordinator_id(), &space_id, reservation.physical_epoch(), &rewrite_context).await.expect("advance raced reservation epoch before stage");
    let blobs = ArtifactChunkBlobStore::new(storage.clone());
    let pack = blobs.stage(&space_id, ArtifactBlobIntegrity { sha256: live.pack.sha256, byte_length: live.pack.byte_length }, &pair.pack, &rewrite_context).await.expect("stage raced pack");
    let spr = blobs.stage(&space_id, ArtifactBlobIntegrity { sha256: live.spr.sha256, byte_length: live.spr.byte_length }, &pair.spr, &rewrite_context).await.expect("stage raced SPR");
    reserve_service.publish_reserved_artifact_checkpoint(system, live.clone(), reservation, 5_301).await.expect("publish raced checkpoint");
    storage.release.notify_one();
    let error = tokio::time::timeout(std::time::Duration::from_secs(30), sweep_tasks.join_next()).await.expect("sweep completion deadline").expect("owned sweep task").expect("join sweep").expect_err("stale deletion fence");
    assert!(matches!(&error, semio_hub_artifact_authority::AuthorityError::Store(message) if message == "artifact CAS deletion fence is stale"), "unexpected raced sweep error: {error:?}");
    assert_eq!(pack.storage_key, live.pack.storage_key);
    assert_eq!(spr.storage_key, live.spr.storage_key);
    assert_eq!(blobs.read(&space_id, &pack, &rewrite_context).await.expect("read raced pack"), pair.pack);
    assert_eq!(blobs.read(&space_id, &spr, &rewrite_context).await.expect("read raced SPR"), pair.spr);
}

#[tokio::test]
async fn artifact_chunk_cas_filesystem_process_sweep_and_publication_race_preserves_exact_bytes() {
    const ROOT_ENV: &str = "SEMIO_ARTIFACT_CAS_DIRECTORY_PROCESS_RACE_ROOT";
    const MODE_ENV: &str = "SEMIO_ARTIFACT_CAS_DIRECTORY_PROCESS_RACE_MODE";
    if let (Ok(root), Ok(mode)) = (std::env::var(ROOT_ENV), std::env::var(MODE_ENV)) {
        let root = std::path::PathBuf::from(root);
        let path_text = root.join("directory.sqlite3").to_str().expect("UTF-8 process race path").to_string();
        let directory = SqliteDirectory::connect(&path_text).await.expect("child opens shared directory");
        let service = DirectoryService::new(Arc::new(HubDirectories::from(directory)), 16);
        let storage = FsArtifactChunkCasStorage::open(&root.join("artifact-cas").join("v1")).await.expect("child opens shared filesystem CAS");
        let system = DirectoryActor { kind: DirectoryActorKind::System, id: "system:artifact-authority".into() };
        let (_, _, live_public, _, _) = artifact_projection_fixture();
        let scope = DocumentScope::new("default", "artifact-00000000000000000000000000000004");
        let descriptor = service.dir.get_document_descriptor(&scope).await.expect("live descriptor").expect("persisted genesis descriptor");
        let lineage = service.dir.list_artifact_checkpoint_lineage(&scope, 2).await.expect("live lineage");
        let genesis = lineage.first().expect("persisted genesis");
        assert!(genesis.baseline_frontier.is_genesis_for(&scope));
        let pair = ArtifactPair { pack: b"process-race-pack".to_vec(), spr: b"process-race-spr".to_vec() };
        let live = scoped_materialized_checkpoint(live_public, &descriptor, &pair, Some(genesis.checkpoint_id), 1);
        if mode == "old-sweep" {
            let storage = ProcessBlockingDeleteArtifactCas { inner: storage, entered: root.join("old-delete-entered"), release: root.join("old-delete-release") };
            let control = ArtifactCasProbe::new(300, None);
            let context = OperationContext::new(20_000, AuthorityLimits::maximum(), &control);
            let error = service.sweep_artifact_cas(&storage, ArtifactCasSweepRequest { execute: true, max_objects: ARTIFACT_CAS_SWEEP_OBJECT_MAX, continuation: None }, &context).await.expect_err("old process deletion fence is stale");
            assert!(matches!(&error, semio_hub_artifact_authority::AuthorityError::Store(message) if message == "artifact CAS deletion fence is stale"), "unexpected old process sweep error: {error:?}");
        } else {
            assert_eq!(mode, "successor-publication");
            let storage = Arc::new(storage);
            let control = ArtifactCasProbe::new(5_301, None);
            let context = OperationContext::new(20_000, AuthorityLimits::maximum(), &control);
            let reservation = stage_reserved_checkpoint(&service, storage.clone(), system.clone(), &live, &pair, 10_000, 5_301, &context).await.expect("successor reserves, advances, and stages");
            service.publish_reserved_artifact_checkpoint(system, live.clone(), reservation, 5_301).await.expect("successor publishes");
            let blobs = ArtifactChunkBlobStore::new(storage);
            let pack = StagedArtifactBlob { storage_key: live.pack.storage_key.clone(), integrity: ArtifactBlobIntegrity { sha256: live.pack.sha256, byte_length: live.pack.byte_length } };
            let spr = StagedArtifactBlob { storage_key: live.spr.storage_key.clone(), integrity: ArtifactBlobIntegrity { sha256: live.spr.sha256, byte_length: live.spr.byte_length } };
            assert_eq!(blobs.read("default", &pack, &context).await.expect("successor pack read"), pair.pack);
            assert_eq!(blobs.read("default", &spr, &context).await.expect("successor SPR read"), pair.spr);
        }
        return;
    }

    let root = semio_hub_testing::test_artifact_root().join(format!("semio-artifact-cas-directory-process-race-{}", directory::os_identity::time_ordered_id()));
    std::fs::create_dir_all(&root).expect("create process race root");
    let path_text = root.join("directory.sqlite3").to_str().expect("UTF-8 process race path").to_string();
    let cas_root = root.join("artifact-cas").join("v1");
    let (mut orphan_descriptor, orphan_public, _, _, _) = artifact_projection_fixture();
    orphan_descriptor.space_id = "default".into();
    orphan_descriptor.document_id = "artifact-cas-process-race-orphan".into();
    let mut live_descriptor = orphan_descriptor.clone();
    live_descriptor.document_id = "artifact-00000000000000000000000000000004".into();
    let pair = ArtifactPair { pack: b"process-race-pack".to_vec(), spr: b"process-race-spr".to_vec() };
    let orphan = scoped_materialized_checkpoint(orphan_public, &orphan_descriptor, &pair, None, 1);
    {
        let directory = SqliteDirectory::connect(&path_text).await.expect("open process race directory");
        directory.seed().await.expect("seed process race directory");
        let service = DirectoryService::new(Arc::new(HubDirectories::from(directory)), 16);
        service.execute(user_actor("seed"), DirectoryCommand::AnnounceDocument { descriptor: Box::new(orphan_descriptor) }).await.expect("announce process race orphan");
        let storage = Arc::new(FsArtifactChunkCasStorage::open(&cas_root).await.expect("open process race filesystem CAS"));
        let system = DirectoryActor { kind: DirectoryActorKind::System, id: "system:artifact-authority".into() };
        let control = ArtifactCasProbe::new(100, None);
        let context = OperationContext::new(20_000, AuthorityLimits::maximum(), &control);
        publish_fixture_genesis(&service, "seed", live_descriptor, storage.clone(), &context).await;
        stage_reserved_checkpoint(&service, storage, system, &orphan, &pair, 200, 100, &context).await.expect("stage expired process race orphan");
    }

    let executable = std::env::current_exe().expect("process race test executable");
    let spawn = |mode: &str| {
        tokio::process::Command::new(&executable)
            .arg("artifact_chunk_cas_filesystem_process_sweep_and_publication_race_preserves_exact_bytes")
            .arg("--test-threads=1")
            .env(ROOT_ENV, &root)
            .env(MODE_ENV, mode)
            .kill_on_drop(true)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .expect("spawn process race child")
    };
    let mut old = spawn("old-sweep");
    let entered = root.join("old-delete-entered");
    tokio::time::timeout(std::time::Duration::from_secs(30), async {
        while !entered.exists() {
            assert!(old.try_wait().expect("poll old sweep child").is_none(), "old sweep child exited before conditional deletion");
            tokio::time::sleep(std::time::Duration::from_millis(5)).await;
        }
    })
    .await
    .expect("old sweep conditional filesystem deletion deadline");
    let successor = tokio::time::timeout(std::time::Duration::from_secs(30), spawn("successor-publication").wait_with_output()).await.expect("successor deadline").expect("wait successor publication child");
    assert!(successor.status.success(), "successor failed: {} {}", String::from_utf8_lossy(&successor.stdout), String::from_utf8_lossy(&successor.stderr));
    std::fs::write(root.join("old-delete-release"), []).expect("release old process delete");
    let old = tokio::time::timeout(std::time::Duration::from_secs(30), old.wait_with_output()).await.expect("old sweep deadline").expect("wait old sweep child");
    assert!(old.status.success(), "old sweep failed: {} {}", String::from_utf8_lossy(&old.stdout), String::from_utf8_lossy(&old.stderr));

    let directory = SqliteDirectory::connect(&path_text).await.expect("reopen process race directory");
    let service = DirectoryService::new(Arc::new(HubDirectories::from(directory)), 16);
    let (_, _, live_public, _, _) = artifact_projection_fixture();
    let scope = DocumentScope::new("default", "artifact-00000000000000000000000000000004");
    let live_descriptor = service.dir.get_document_descriptor(&scope).await.expect("live descriptor").expect("persisted genesis descriptor");
    let lineage = service.dir.list_artifact_checkpoint_lineage(&scope, 2).await.expect("live lineage");
    let genesis = lineage.first().expect("persisted genesis");
    assert!(genesis.baseline_frontier.is_genesis_for(&scope));
    let live = scoped_materialized_checkpoint(live_public, &live_descriptor, &pair, Some(genesis.checkpoint_id), 1);
    assert_eq!(service.dir.get_verified_artifact_checkpoint(&live.scope, live.checkpoint_id).await.expect("published process race reference"), Some(live.clone()));
    let storage = Arc::new(FsArtifactChunkCasStorage::open(&cas_root).await.expect("reopen process race filesystem CAS"));
    let blobs = ArtifactChunkBlobStore::new(storage);
    let control = ArtifactCasProbe::new(5_302, None);
    let context = OperationContext::new(20_000, AuthorityLimits::maximum(), &control);
    let pack = StagedArtifactBlob { storage_key: live.pack.storage_key.clone(), integrity: ArtifactBlobIntegrity { sha256: live.pack.sha256, byte_length: live.pack.byte_length } };
    let spr = StagedArtifactBlob { storage_key: live.spr.storage_key.clone(), integrity: ArtifactBlobIntegrity { sha256: live.spr.sha256, byte_length: live.spr.byte_length } };
    assert_eq!(blobs.read("default", &pack, &context).await.expect("parent exact pack read"), pair.pack);
    assert_eq!(blobs.read("default", &spr, &context).await.expect("parent exact SPR read"), pair.spr);
    drop(blobs);
    drop(service);
    std::fs::remove_dir_all(root).expect("remove process race root");
}

#[tokio::test]
async fn artifact_checkpoint_publication_is_atomic_bounded_idempotent_and_replayable() {
    let (descriptor, first, second, first_retention, fixture_maximum) = artifact_projection_fixture();
    assert_eq!(fixture_maximum, ARTIFACT_CHECKPOINT_LINEAGE_MAX);
    let dir = fresh_dir().await;
    let service = DirectoryService::new(dir.clone(), 64);
    let owner = user_actor("u-owner");
    let space_id = create_space(&service, &owner, DirectorySpaceKind::Studio).await;
    assert_ne!(space_id, descriptor.space_id);
    service.execute(owner.clone(), DirectoryCommand::CreateSpace { name: "Fixture".into(), space_kind: DirectorySpaceKind::Studio, visibility: DirectorySpaceVisibility::Private }).await.expect("second space");
    let fixture_space = dir.list_spaces(100, 0).await.expect("spaces").into_iter().find(|space| space.name == "Fixture").expect("fixture space");
    let mut announced = descriptor.clone();
    announced.space_id = fixture_space.id.clone();
    announced.document_id = "artifact-00000000000000000000000000000001".into();
    let control = ArtifactCasProbe::new(now_ms() as u64, None);
    let context = OperationContext::new(now_ms() as u64 + 30_000, AuthorityLimits::maximum(), &control);
    let (announced, genesis) = publish_fixture_genesis(&service, "u-owner", announced, Arc::new(MemoryArtifactChunkCasStorage::default()), &context).await;
    let mut first = first;
    let mut second = second;
    let mut first_retention = first_retention;
    first.scope.space_id = fixture_space.id.clone();
    first.scope.document_id = announced.document_id.clone();
    first.baseline_frontier.document_id = announced.document_id.clone();
    second.scope.space_id = fixture_space.id.clone();
    second.scope.document_id = announced.document_id.clone();
    second.baseline_frontier.document_id = announced.document_id.clone();
    first_retention.scope.space_id = fixture_space.id.clone();
    first_retention.scope.document_id = announced.document_id.clone();
    first_retention.retained_floor.document_id = announced.document_id.clone();
    let digest = descriptor_digest_v1(&announced).expect("digest");
    first.descriptor_digest_v1 = digest;
    second.descriptor_digest_v1 = digest;
    first.parent_checkpoint_id = Some(genesis.checkpoint_id);
    first.checkpoint_id = ArtifactHash(Sha256::digest(&semio_hub_artifact_authority::checkpoint_id_encoding_v1(&checkpoint_identity_input(&first)).expect("first identity")));
    second.parent_checkpoint_id = Some(first.checkpoint_id);
    second.checkpoint_id = ArtifactHash(Sha256::digest(&semio_hub_artifact_authority::checkpoint_id_encoding_v1(&checkpoint_identity_input(&second)).expect("second identity")));
    first_retention.retained_checkpoint_id = first.checkpoint_id;
    first_retention.checkpoint_lineage_head = second.checkpoint_id;
    let system = DirectoryActor { kind: DirectoryActorKind::System, id: "system:artifact-authority".into() };

    let first_verified = verified_checkpoint(&first, "one");
    let second_verified = verified_checkpoint(&second, "two");
    let admin = DirectoryActor { kind: DirectoryActorKind::Admin, id: "admin:not-authority".into() };
    assert!(matches!(publish_reserved(&service, admin, first_verified.clone()).await, Err(DirectoryError::Unauthorized)));
    let first_events = publish_reserved(&service, system.clone(), first_verified.clone()).await.expect("publish first");
    assert_eq!(first_events.len(), 1);
    let public_json = serde_json::Value::from(&first_events[0].body.to_value()).to_string();
    assert!(!public_json.contains("storageKey"));
    assert!(!public_json.contains(&first_verified.pack.storage_key));
    assert!(!public_json.contains(&first_verified.spr.storage_key));
    let head = dir.head_seq().await.expect("head");
    let mut failed_publication_stream = service.subscribe();
    assert!(publish_reserved(&service, system.clone(), first_verified.clone()).await.expect("idempotent first").is_empty());
    assert_eq!(dir.head_seq().await.expect("head unchanged"), head);
    let mut private_conflict = first_verified.clone();
    private_conflict.pack.storage_key = artifact_cas_manifest_locator_v1(ArtifactHash([88; 32]));
    assert!(matches!(publish_reserved(&service, system.clone(), private_conflict).await, Err(DirectoryError::Conflict(_))));
    let mut public_conflict = first_verified.clone();
    public_conflict.published_at_ms += 1;
    assert!(matches!(publish_reserved(&service, system.clone(), public_conflict).await, Err(DirectoryError::Conflict(_))));
    let forged_public =
        NewDirectoryEvent { hlc: Hlc { physical_ms: 1, logical: 0 }, actor: system.clone(), space_id: Some(first.scope.space_id.clone()), user_id: None, body: DirectoryEventBody::ArtifactCheckpointPublished { checkpoint: first.clone() } };
    assert!(matches!(dir.append_events(&[forged_public]).await, Err(DirectoryError::Conflict(_))));
    assert_eq!(dir.head_seq().await.expect("append failure leaves head"), head);
    assert!(matches!(failed_publication_stream.try_recv(), Err(tokio::sync::broadcast::error::TryRecvError::Empty)));

    publish_reserved(&service, system.clone(), second_verified.clone()).await.expect("publish second");
    service.execute_artifact_authority(system.clone(), ArtifactDirectoryCommand::AdvanceRetention { retention: first_retention.clone() }).await.expect("retain first");
    assert!(service.execute_artifact_authority(system.clone(), ArtifactDirectoryCommand::AdvanceRetention { retention: first_retention.clone() }).await.expect("idempotent retention").is_empty());
    let second_retention = ArtifactRetention { scope: second.scope.clone(), retained_checkpoint_id: second.checkpoint_id, retained_floor: second.baseline_frontier.clone(), checkpoint_lineage_head: second.checkpoint_id };
    service.execute_artifact_authority(system.clone(), ArtifactDirectoryCommand::AdvanceRetention { retention: second_retention.clone() }).await.expect("retain second");
    assert!(matches!(service.execute_artifact_authority(system, ArtifactDirectoryCommand::AdvanceRetention { retention: first_retention }).await, Err(DirectoryError::Conflict(_))));

    let scope = second.scope.clone();
    assert_eq!(dir.get_active_artifact_checkpoint(&scope).await.expect("active"), Some(second.clone()));
    assert_eq!(dir.get_verified_artifact_checkpoint(&scope, first.checkpoint_id).await.expect("released private"), None);
    assert_eq!(dir.get_verified_artifact_checkpoint(&scope, second.checkpoint_id).await.expect("private active"), Some(second_verified.clone()));
    assert_eq!(dir.list_artifact_checkpoint_lineage(&scope, ARTIFACT_CHECKPOINT_LINEAGE_MAX as usize).await.expect("lineage"), vec![published_artifact_checkpoint(&genesis), first.clone(), second.clone()]);
    assert_eq!(dir.get_artifact_retention(&scope).await.expect("retention"), Some(second_retention.clone()));
    assert!(matches!(dir.list_artifact_checkpoint_lineage(&scope, ARTIFACT_CHECKPOINT_LINEAGE_MAX as usize + 1).await, Err(DirectoryError::Conflict(_))));

    let before = (dir.get_active_artifact_checkpoint(&scope).await.expect("before active"), dir.get_artifact_retention(&scope).await.expect("before retention"));
    let cancel = RebuildProbe { cancelled: AtomicBool::new(false), cancel_after_first: true, progress: std::sync::Mutex::new(Vec::new()) };
    assert!(matches!(dir.rebuild_projections_controlled(&cancel).await, Err(DirectoryError::Conflict(_))));
    assert_eq!((dir.get_active_artifact_checkpoint(&scope).await.expect("rollback active"), dir.get_artifact_retention(&scope).await.expect("rollback retention")), before);
    let complete = RebuildProbe { cancelled: AtomicBool::new(false), cancel_after_first: false, progress: std::sync::Mutex::new(Vec::new()) };
    let replayed = dir.rebuild_projections_controlled(&complete).await.expect("controlled rebuild");
    let progress = complete.progress.lock().expect("progress");
    assert_eq!(progress.first().expect("initial").completed_events, 0);
    assert_eq!(progress.last().expect("final"), &ProjectionRebuildProgress { completed_events: replayed, total_events: replayed });
    assert_eq!(dir.get_active_artifact_checkpoint(&scope).await.expect("rebuilt active"), Some(second));
    assert_eq!(dir.get_verified_artifact_checkpoint(&scope, first.checkpoint_id).await.expect("rebuilt released private"), None);
    assert_eq!(dir.get_verified_artifact_checkpoint(&scope, second_verified.checkpoint_id).await.expect("rebuilt private"), Some(second_verified));
    assert_eq!(dir.get_artifact_retention(&scope).await.expect("rebuilt retention"), Some(second_retention));
}

#[test]
fn memory_projection_is_atomic_and_fixed_caps_reject_max_plus_one() {
    let (descriptor, first, second, retention, fixture_maximum) = artifact_projection_fixture();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📸️artifact-checkpoint-projection/🔣️.json")).expect("checkpoint projection fixture");
    let genesis = PublishedArtifactCheckpoint::from_value(DslValue::from(fixture["genesis"].clone())).expect("fixture genesis");
    let entry = directory::os_directory::DocumentIndexEntryV1::from_value(DslValue::from(fixture["indexEntry"].clone())).expect("fixture index entry");
    let mut announced = artifact_event(1, DirectoryEventBody::DocumentAnnounced { descriptor });
    announced.actor = user_actor("creator");
    announced.user_id = Some("creator".into());
    let mut indexed = artifact_event(2, DirectoryEventBody::DocumentIndexed { scope: genesis.scope.clone(), descriptor_digest_v1: genesis.descriptor_digest_v1, entry });
    indexed.actor = announced.actor.clone();
    indexed.user_id = announced.user_id.clone();
    let events = vec![
        announced,
        indexed,
        artifact_event(3, DirectoryEventBody::ArtifactCheckpointPublished { checkpoint: genesis.clone() }),
        artifact_event(4, DirectoryEventBody::ArtifactCheckpointPublished { checkpoint: first.clone() }),
        artifact_event(5, DirectoryEventBody::ArtifactCheckpointPublished { checkpoint: second.clone() }),
        artifact_event(6, DirectoryEventBody::ArtifactRetentionAdvanced { retention: retention.clone() }),
    ];
    let mut projection = MemoryArtifactProjection::default();
    projection.fold_atomically(&events).expect("memory fold");
    assert_eq!(projection.active_checkpoint(&second.scope), Some(&second));
    assert_eq!(projection.retention(&second.scope), Some(&retention));
    let before = projection.clone();
    let mut backward = retention;
    backward.checkpoint_lineage_head = first.checkpoint_id;
    assert!(projection.fold_atomically(&[artifact_event(7, DirectoryEventBody::ArtifactRetentionAdvanced { retention: backward })]).is_err());
    assert_eq!(projection, before);

    let mut rootless = MemoryArtifactProjection::default();
    rootless.fold_atomically(&events[..1]).expect("descriptor only");
    let descriptor_only = rootless.clone();
    assert!(rootless.fold_atomically(&events[2..3]).is_err());
    assert_eq!(rootless, descriptor_only);
    rootless.fold_atomically(&events[1..2]).expect("descriptor bound index");
    let indexed_only = rootless.clone();
    assert!(rootless.fold_atomically(&events[3..4]).is_err());
    assert_eq!(rootless, indexed_only);
    let mut late_root = genesis;
    late_root.spr.sha256.0[0] ^= 1;
    late_root.checkpoint_id = ArtifactHash(Sha256::digest(&semio_hub_artifact_authority::checkpoint_id_encoding_v1(&checkpoint_identity_input(&late_root)).expect("late root identity")));
    assert!(projection.fold_atomically(&[artifact_event(7, DirectoryEventBody::ArtifactCheckpointPublished { checkpoint: late_root })]).is_err());

    projection.checkpoints.insert(first.scope.clone(), vec![first.clone(); fixture_maximum as usize]);
    assert!(projection.fold_atomically(&[artifact_event(8, DirectoryEventBody::ArtifactCheckpointPublished { checkpoint: second })]).is_err());
    let probe = RebuildProbe { cancelled: AtomicBool::new(false), cancel_after_first: false, progress: std::sync::Mutex::new(Vec::new()) };
    checkpoint_projection_rebuild(&probe, DIRECTORY_PROJECTION_REBUILD_MAX_EVENTS, DIRECTORY_PROJECTION_REBUILD_MAX_EVENTS).expect("rebuild exact maximum");
    assert!(checkpoint_projection_rebuild(&probe, 0, DIRECTORY_PROJECTION_REBUILD_MAX_EVENTS + 1).is_err());
}

#[test]
fn artifact_public_scalars_and_private_locators_obey_exact_max_plus_one_laws() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📸️artifact-checkpoint-projection/🔣️.json")).expect("checkpoint projection fixture");
    assert_eq!(fixture["wireIntegerMaximum"].as_u64(), Some(DIRECTORY_WIRE_INTEGER_MAX));
    assert_eq!(fixture["privateLocatorMaximumBytes"].as_u64(), Some(ARTIFACT_PRIVATE_LOCATOR_MAX_BYTES as u64));
    assert_eq!(fixture["eventReadMaximum"].as_u64(), Some(DIRECTORY_EVENT_READ_MAX as u64));
    bounded_event_read(DIRECTORY_WIRE_INTEGER_MAX, DIRECTORY_EVENT_READ_MAX).expect("event read exact maxima");
    assert!(bounded_event_read(DIRECTORY_WIRE_INTEGER_MAX + 1, DIRECTORY_EVENT_READ_MAX).is_err());
    assert!(bounded_event_read(0, DIRECTORY_EVENT_READ_MAX + 1).is_err());
    let (_, first, _, retention, _) = artifact_projection_fixture();
    let mut public_max = first.clone();
    public_max.published_at_ms = DIRECTORY_WIRE_INTEGER_MAX;
    validate_checkpoint_shape(&public_max).expect("wire integer exact maximum");
    public_max.published_at_ms = DIRECTORY_WIRE_INTEGER_MAX + 1;
    assert!(validate_checkpoint_shape(&public_max).is_err());

    let mut frontier_max = first.clone();
    frontier_max.baseline_frontier.head_edit_ordinal = DIRECTORY_WIRE_INTEGER_MAX;
    frontier_max.baseline_frontier.last_commit_seq = DIRECTORY_WIRE_INTEGER_MAX;
    frontier_max.pack.byte_length = DIRECTORY_WIRE_INTEGER_MAX;
    frontier_max.spr.byte_length = DIRECTORY_WIRE_INTEGER_MAX;
    frontier_max.checkpoint_id = ArtifactHash(Sha256::digest(&semio_hub_artifact_authority::checkpoint_id_encoding_v1(&checkpoint_identity_input(&frontier_max)).expect("max identity")));
    validate_checkpoint_shape(&frontier_max).expect("all exact wire maxima");
    frontier_max.baseline_frontier.head_edit_ordinal += 1;
    assert!(validate_checkpoint_shape(&frontier_max).is_err());

    let mut retention_max = retention;
    retention_max.retained_floor.head_edit_ordinal = DIRECTORY_WIRE_INTEGER_MAX;
    retention_max.retained_floor.last_commit_seq = DIRECTORY_WIRE_INTEGER_MAX;
    validate_retention_shape(&retention_max).expect("retention exact maximum");
    retention_max.retained_floor.last_commit_seq += 1;
    assert!(validate_retention_shape(&retention_max).is_err());

    let mut private = verified_checkpoint(&first, "limits");
    private.pack.storage_key = "p".repeat(ARTIFACT_PRIVATE_LOCATOR_MAX_BYTES);
    private.spr.storage_key = "s".repeat(ARTIFACT_PRIVATE_LOCATOR_MAX_BYTES);
    let event = NewDirectoryEvent {
        hlc: Hlc { physical_ms: 1, logical: 0 },
        actor: DirectoryActor { kind: DirectoryActorKind::System, id: "system:artifact-authority".into() },
        space_id: Some(private.scope.space_id.clone()),
        user_id: None,
        body: DirectoryEventBody::ArtifactCheckpointPublished { checkpoint: published_artifact_checkpoint(&private) },
    };
    validate_verified_checkpoint_append(&event, &private).expect("locator exact maximum");
    private.pack.storage_key.push('x');
    assert!(validate_verified_checkpoint_append(&event, &private).is_err());
}

#[tokio::test]
async fn artifact_chunk_cas_sqlite_and_filesystem_restart_rebuild_restore_exact_authority() {
    let (mut descriptor, public, _, _, _) = artifact_projection_fixture();
    descriptor.space_id = "default".into();
    descriptor.document_id = "artifact-00000000000000000000000000000005".into();
    let pair = ArtifactPair { pack: b"restart-pack".to_vec(), spr: b"restart-spr".to_vec() };
    let verified;
    let mut root = semio_hub_testing::test_artifact_root();
    root.push(format!("semio-artifact-checkpoint-{}", directory::os_identity::time_ordered_id()));
    std::fs::create_dir_all(&root).expect("create test directory");
    let path = root.join("directory.sqlite3");
    let cas_root = root.join("artifact-cas").join("v1");
    let path_text = path.to_str().expect("utf8 test path").to_string();
    let control = ArtifactCasProbe::new(100, None);
    let context = OperationContext::new(10_000, AuthorityLimits::maximum(), &control);
    {
        let directory = SqliteDirectory::connect(&path_text).await.expect("connect");
        directory.seed().await.expect("seed");
        let directories = Arc::new(HubDirectories::from(directory));
        let service = DirectoryService::new(directories, 16);
        let storage = Arc::new(FsArtifactChunkCasStorage::open(&cas_root).await.expect("open filesystem CAS"));
        let (descriptor, genesis) = publish_fixture_genesis(&service, "seed", descriptor, storage.clone(), &context).await;
        verified = scoped_materialized_checkpoint(public, &descriptor, &pair, Some(genesis.checkpoint_id), 1);
        let system = DirectoryActor { kind: DirectoryActorKind::System, id: "system:artifact-authority".into() };
        let reservation = stage_reserved_checkpoint(&service, storage, system.clone(), &verified, &pair, 1_000, 100, &context).await.expect("reserve and stage restart fixture");
        service.publish_reserved_artifact_checkpoint(system, verified.clone(), reservation, 100).await.expect("publish verified checkpoint");
    }
    let reopened = SqliteDirectory::connect(&path_text).await.expect("reopen");
    assert_eq!(reopened.get_verified_artifact_checkpoint(&verified.scope, verified.checkpoint_id).await.expect("restart private"), Some(verified.clone()));
    let generation = reopened.artifact_cas_ledger_generation().await.expect("restart ledger generation");
    let storage = Arc::new(FsArtifactChunkCasStorage::open(&cas_root).await.expect("reopen filesystem CAS"));
    let blobs = ArtifactChunkBlobStore::new(storage);
    let pack = StagedArtifactBlob { storage_key: verified.pack.storage_key.clone(), integrity: ArtifactBlobIntegrity { sha256: verified.pack.sha256, byte_length: verified.pack.byte_length } };
    assert_eq!(blobs.read("default", &pack, &context).await.expect("restart CAS read"), pair.pack);
    reopened.rebuild_projections().await.expect("rebuild");
    assert_eq!(reopened.artifact_cas_ledger_generation().await.expect("rebuilt ledger generation"), generation);
    assert_eq!(reopened.get_verified_artifact_checkpoint(&verified.scope, verified.checkpoint_id).await.expect("rebuilt private"), Some(verified.clone()));
    assert_eq!(blobs.read("default", &pack, &context).await.expect("rebuilt CAS read"), pair.pack);
    drop(blobs);
    drop(reopened);
    std::fs::remove_dir_all(&root).expect("remove exact restart test directory");
}
