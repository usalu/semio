/// 🧰️ Artifact fixtures: descriptors, checkpoints, ownership plans, genesis publication and the CAS and rebuild probes.

fn user_actor(user_id: &str) -> DirectoryActor {
    DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{user_id}#s1") }
}

fn descriptor(space_id: &str, document_id: &str) -> DocumentDescriptor {
    DocumentDescriptor {
        space_id: space_id.into(),
        document_id: document_id.into(),
        artifact_kind: "s.gis.gismap".into(),
        artifact_schema: "s.gis.gismap@1/*".into(),
        owner: directory::os_directory::DocumentOwner { plugin_id: "s.gis".into(), package_id: "s.gis.gismap".into(), version: "1.0.0".into(), package_hash: "22".repeat(32) },
        pack_schema_hash: "11".repeat(32),
        bootstrap_version: 1,
        bootstrap_frontier: directory::os_directory::DocumentFrontier { head_seq: 7, commit_seq: 7, epoch: 2 },
        bootstrap_snapshot_hash: "33".repeat(32),
    }
}

fn artifact_projection_fixture() -> (DocumentDescriptor, PublishedArtifactCheckpoint, PublishedArtifactCheckpoint, ArtifactRetention, u64) {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📸️artifact-checkpoint-projection/🔣️.json")).expect("checkpoint projection fixture");
    let decode = |field: &str| DslValue::from(fixture.get(field).expect("fixture field").clone());
    (
        DocumentDescriptor::from_value(decode("descriptor")).expect("fixture descriptor"),
        PublishedArtifactCheckpoint::from_value(decode("checkpoint1")).expect("fixture checkpoint 1"),
        PublishedArtifactCheckpoint::from_value(decode("checkpoint2")).expect("fixture checkpoint 2"),
        ArtifactRetention::from_value(decode("retention")).expect("fixture retention"),
        fixture["lineageMaximum"].as_u64().expect("fixture maximum"),
    )
}

fn verified_checkpoint(checkpoint: &PublishedArtifactCheckpoint, locator_suffix: &str) -> ArtifactCheckpoint {
    let mut verified = checkpoint_identity_input(checkpoint);
    verified.pack.storage_key = format!("semio.artifact-cas.manifest/v1/{}", semio_framework_hash::hex_lower(&Sha256::digest(format!("pack:{locator_suffix}").as_bytes())));
    verified.spr.storage_key = format!("semio.artifact-cas.manifest/v1/{}", semio_framework_hash::hex_lower(&Sha256::digest(format!("spr:{locator_suffix}").as_bytes())));
    verified
}

fn ownership_plan(checkpoint: &ArtifactCheckpoint) -> ArtifactCasOwnershipPlanV1 {
    let pack_manifest_id = semio_hub_artifact_authority::chunk_cas::decode_artifact_cas_manifest_locator_v1(&checkpoint.pack.storage_key).expect("pack manifest locator");
    let spr_manifest_id = semio_hub_artifact_authority::chunk_cas::decode_artifact_cas_manifest_locator_v1(&checkpoint.spr.storage_key).expect("SPR manifest locator");
    let mut objects = vec![
        ArtifactCasObjectKey { space_id: checkpoint.scope.space_id.clone(), kind: ArtifactCasObjectKind::Manifest, digest: pack_manifest_id },
        ArtifactCasObjectKey { space_id: checkpoint.scope.space_id.clone(), kind: ArtifactCasObjectKind::Manifest, digest: spr_manifest_id },
    ];
    objects.sort_by_key(|object| (object.kind, object.digest.0));
    objects.dedup();
    ArtifactCasOwnershipPlanV1 { scope: checkpoint.scope.clone(), checkpoint_id: checkpoint.checkpoint_id, pack_manifest_id, spr_manifest_id, objects }
}

async fn publish_reserved(service: &DirectoryService, actor: DirectoryActor, checkpoint: ArtifactCheckpoint) -> DirectoryResult<Vec<DirectoryEvent>> {
    let reservation = service.reserve_artifact_cas(actor.clone(), ownership_plan(&checkpoint), 1_000, 100).await?;
    service.publish_reserved_artifact_checkpoint(actor, checkpoint, reservation, 100).await
}

/// 🪢 The owning app's `Dialect` coordinate these fixtures index under. It is deliberately NOT the
/// descriptor's `artifact_kind`: the descriptor carries the manifest `ArtifactKindSpec::id`, which
/// this fixture family spells with neutral punctuation (`s.gis:gismap`) to prove the descriptor is
/// carried as opaque text, while a dialect is a canonical `s.<plugin>.<artifact>` coordinate. Every
/// plugin but `gis` really has the two in different spaces, so deriving one from the other here made
/// the hub's own genesis helper test a shape only `gis` can produce.
const FIXTURE_PARENT_DIALECT_KIND: &str = "s.gis.gismap";

async fn publish_fixture_genesis<S: ArtifactChunkCasStorage>(service: &DirectoryService, user_id: &str, mut descriptor: DocumentDescriptor, storage: Arc<S>, context: &OperationContext<'_>) -> (DocumentDescriptor, ArtifactCheckpoint) {
    use semio_hub_artifact_creation::{ARTIFACT_CREATION_DEADLINE_MS, ArtifactCreationActorV1, ArtifactCreationFactBodyV1, ArtifactCreationPreparedV1};
    let fixture: serde_json::Value = serde_json::from_str(semio_hub_testing::hub_source("🗿️artifact-authority/🌱️creation/🧪️tests/🧫️fixtures/📚️operation-v1/🔣️.json")).expect("creation fixture");
    let mut intent = ArtifactCreationIntentV1::from_value(DslValue::from(fixture["intent"].clone())).expect("creation intent");
    let mut prepared = ArtifactCreationPreparedV1::from_value(DslValue::from(fixture["prepared"].clone())).expect("creation pair");
    let issued = service
        .dir
        .issue_auth_session(&AuthSessionIssue {
            user_id: user_id.into(),
            identity_provider: "genesis-test".into(),
            identity_subject_digest: identity_subject_digest("genesis-test", user_id).expect("fixture identity"),
            ttl_secs: 60,
            device_instance_id: "genesis-device".into(),
            session_kind: AuthSessionKind::DevelopmentLocal,
            correlation_id: "genesis-session".into(),
            peer_class: "loopback-test".into(),
        })
        .await
        .expect("creation session");
    descriptor.bootstrap_frontier = directory::os_directory::DocumentFrontier { head_seq: 0, commit_seq: 0, epoch: 0 };
    descriptor.artifact_schema = "s.gis.gismap/1/any".into();
    descriptor.bootstrap_snapshot_hash = semio_framework_hash::hex_lower(&Sha256::digest(&prepared.pack));
    intent.actor = ArtifactCreationActorV1 { user_id: user_id.into(), session_id: issued.record.id, authorization_generation: issued.record.authorization_generation };
    intent.scope = DocumentScope::new(&descriptor.space_id, &descriptor.document_id);
    intent.request.request_id = descriptor.document_id.strip_prefix("artifact-").expect("minted fixture document").into();
    intent.request.kind_id = descriptor.artifact_kind.clone();
    intent.owner = descriptor.owner.clone();
    intent.artifact_schema = descriptor.artifact_schema.clone();
    intent.pack_schema_hash = descriptor.pack_schema_hash.clone();
    intent.parent_dialect.artifact_kind = FIXTURE_PARENT_DIALECT_KIND.into();
    intent.accepted_at_ms = now_ms() as u64;
    intent.deadline_ms = intent.accepted_at_ms + ARTIFACT_CREATION_DEADLINE_MS;
    intent.command_sha256 = artifact_creation_command_digest_v1(&intent.scope.space_id, &intent.request).expect("creation command digest");
    prepared.descriptor = descriptor.clone();
    let mut public = published_artifact_checkpoint(&prepared.checkpoint);
    public.scope = intent.scope.clone();
    public.baseline_frontier.document_id = descriptor.document_id.clone();
    public.descriptor_digest_v1 = descriptor_digest_v1(&descriptor).expect("creation descriptor digest");
    public.published_at_ms = intent.accepted_at_ms;
    let pair = ArtifactPair { pack: prepared.pack.clone(), spr: prepared.spr.clone() };
    let checkpoint = materialized_checkpoint(public, &pair);
    prepared.checkpoint = checkpoint.clone();
    prepared.checkpoint.pack.storage_key = format!("sha256/{}", directory::os_directory::io::binary::artifact_hash::artifact_hash_hex(&checkpoint.pack.sha256));
    prepared.checkpoint.spr.storage_key = format!("sha256/{}", directory::os_directory::io::binary::artifact_hash::artifact_hash_hex(&checkpoint.spr.sha256));
    semio_hub_artifact_creation::io::validate_artifact_creation_prepared_v1(&prepared, &intent).expect("exact prepared genesis");
    assert!(matches!(service.dir.claim_artifact_creation(&intent).await.expect("claim genesis"), ArtifactCreationClaimV1::Accepted(_)));
    service
        .dir
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
        .expect("prepare genesis");
    let system = DirectoryActor { kind: DirectoryActorKind::System, id: "system:artifact-creation".into() };
    let reservation = stage_reserved_checkpoint(service, storage, system, &checkpoint, &pair, intent.deadline_ms, now_ms() as u64, context).await.expect("stage genesis");
    service.publish_document_genesis(intent, &prepared, checkpoint.clone(), reservation, now_ms() as u64).await.expect("publish genesis");
    (descriptor, checkpoint)
}

struct ArtifactCasProbe {
    now_ms: AtomicU64,
    cancel_after_sweep: Option<u64>,
    cancelled: AtomicBool,
    progress: std::sync::Mutex<Vec<AuthorityProgress>>,
}

impl ArtifactCasProbe {
    fn new(now_ms: u64, cancel_after_sweep: Option<u64>) -> Self {
        Self { now_ms: AtomicU64::new(now_ms), cancel_after_sweep, cancelled: AtomicBool::new(false), progress: std::sync::Mutex::new(Vec::new()) }
    }
}

impl AuthorityOperationControl for ArtifactCasProbe {
    fn now_ms(&self) -> u64 {
        self.now_ms.load(Ordering::SeqCst)
    }

    fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::SeqCst)
    }

    fn report(&self, progress: AuthorityProgress) {
        self.progress.lock().expect("artifact CAS progress").push(progress);
        if progress.stage == AuthorityProgressStage::CasSweep && self.cancel_after_sweep.is_some_and(|after| progress.completed_units >= after) {
            self.cancelled.store(true, Ordering::SeqCst);
        }
    }
}

struct BlockingDeleteArtifactCas {
    inner: Arc<MemoryArtifactChunkCasStorage>,
    block_next: AtomicBool,
    entered: tokio::sync::Notify,
    release: tokio::sync::Notify,
}

impl BlockingDeleteArtifactCas {
    fn new() -> Self {
        Self { inner: Arc::new(MemoryArtifactChunkCasStorage::default()), block_next: AtomicBool::new(true), entered: tokio::sync::Notify::new(), release: tokio::sync::Notify::new() }
    }
}

impl ArtifactChunkCasStorage for BlockingDeleteArtifactCas {
    async fn configure_coordinator(&self, coordinator_id: [u8; 32], context: &OperationContext<'_>) -> Result<(), semio_hub_artifact_authority::AuthorityError> {
        self.inner.configure_coordinator(coordinator_id, context).await
    }

    async fn advance_physical_epoch(&self, coordinator_id: [u8; 32], space_id: &str, epoch: u64, context: &OperationContext<'_>) -> Result<(), semio_hub_artifact_authority::AuthorityError> {
        self.inner.advance_physical_epoch(coordinator_id, space_id, epoch, context).await
    }

    async fn put_if_absent(&self, key: &ArtifactCasObjectKey, bytes: &[u8], context: &OperationContext<'_>) -> Result<semio_hub_artifact_authority::chunk_cas::ArtifactCasPutOutcome, semio_hub_artifact_authority::AuthorityError> {
        self.inner.put_if_absent(key, bytes, context).await
    }

    async fn get(&self, key: &ArtifactCasObjectKey, context: &OperationContext<'_>) -> Result<Vec<u8>, semio_hub_artifact_authority::AuthorityError> {
        self.inner.get(key, context).await
    }

    async fn delete_if_unreferenced(&self, key: &ArtifactCasObjectKey, fence: &ArtifactCasDeleteFence, context: &OperationContext<'_>) -> Result<ArtifactCasDeleteOutcome, semio_hub_artifact_authority::AuthorityError> {
        if self.block_next.swap(false, Ordering::SeqCst) {
            self.entered.notify_one();
            self.release.notified().await;
        }
        self.inner.delete_if_unreferenced(key, fence, context).await
    }
}

struct ProcessBlockingDeleteArtifactCas {
    inner: FsArtifactChunkCasStorage,
    entered: std::path::PathBuf,
    release: std::path::PathBuf,
}

impl ArtifactChunkCasStorage for ProcessBlockingDeleteArtifactCas {
    async fn configure_coordinator(&self, coordinator_id: [u8; 32], context: &OperationContext<'_>) -> Result<(), semio_hub_artifact_authority::AuthorityError> {
        self.inner.configure_coordinator(coordinator_id, context).await
    }

    async fn advance_physical_epoch(&self, coordinator_id: [u8; 32], space_id: &str, epoch: u64, context: &OperationContext<'_>) -> Result<(), semio_hub_artifact_authority::AuthorityError> {
        self.inner.advance_physical_epoch(coordinator_id, space_id, epoch, context).await
    }

    async fn put_if_absent(&self, key: &ArtifactCasObjectKey, bytes: &[u8], context: &OperationContext<'_>) -> Result<semio_hub_artifact_authority::chunk_cas::ArtifactCasPutOutcome, semio_hub_artifact_authority::AuthorityError> {
        self.inner.put_if_absent(key, bytes, context).await
    }

    async fn get(&self, key: &ArtifactCasObjectKey, context: &OperationContext<'_>) -> Result<Vec<u8>, semio_hub_artifact_authority::AuthorityError> {
        self.inner.get(key, context).await
    }

    async fn delete_if_unreferenced(&self, key: &ArtifactCasObjectKey, fence: &ArtifactCasDeleteFence, context: &OperationContext<'_>) -> Result<ArtifactCasDeleteOutcome, semio_hub_artifact_authority::AuthorityError> {
        std::fs::write(&self.entered, []).map_err(|_| semio_hub_artifact_authority::AuthorityError::Store("artifact CAS process race readiness write failed".into()))?;
        let wait_deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(30);
        while !self.release.exists() {
            context.checkpoint()?;
            if tokio::time::Instant::now() >= wait_deadline {
                return Err(semio_hub_artifact_authority::AuthorityError::Store("artifact CAS process race release timed out".into()));
            }
            tokio::time::sleep(std::time::Duration::from_millis(5)).await;
        }
        self.inner.delete_if_unreferenced(key, fence, context).await
    }
}

struct FailingAdvanceArtifactCas(MemoryArtifactChunkCasStorage);

impl ArtifactChunkCasStorage for FailingAdvanceArtifactCas {
    async fn configure_coordinator(&self, coordinator_id: [u8; 32], context: &OperationContext<'_>) -> Result<(), semio_hub_artifact_authority::AuthorityError> {
        self.0.configure_coordinator(coordinator_id, context).await
    }

    async fn advance_physical_epoch(&self, _: [u8; 32], _: &str, _: u64, _: &OperationContext<'_>) -> Result<(), semio_hub_artifact_authority::AuthorityError> {
        Err(semio_hub_artifact_authority::AuthorityError::Cancelled)
    }

    async fn put_if_absent(&self, key: &ArtifactCasObjectKey, bytes: &[u8], context: &OperationContext<'_>) -> Result<semio_hub_artifact_authority::chunk_cas::ArtifactCasPutOutcome, semio_hub_artifact_authority::AuthorityError> {
        self.0.put_if_absent(key, bytes, context).await
    }

    async fn get(&self, key: &ArtifactCasObjectKey, context: &OperationContext<'_>) -> Result<Vec<u8>, semio_hub_artifact_authority::AuthorityError> {
        self.0.get(key, context).await
    }

    async fn delete_if_unreferenced(&self, key: &ArtifactCasObjectKey, fence: &ArtifactCasDeleteFence, context: &OperationContext<'_>) -> Result<ArtifactCasDeleteOutcome, semio_hub_artifact_authority::AuthorityError> {
        self.0.delete_if_unreferenced(key, fence, context).await
    }
}

fn materialized_checkpoint(mut public: PublishedArtifactCheckpoint, pair: &ArtifactPair) -> ArtifactCheckpoint {
    public.pack = PublishedArtifactBlob { sha256: ArtifactHash(Sha256::digest(&pair.pack)), byte_length: pair.pack.len() as u64 };
    public.spr = PublishedArtifactBlob { sha256: ArtifactHash(Sha256::digest(&pair.spr)), byte_length: pair.spr.len() as u64 };
    let mut aggregate = Sha256::new();
    aggregate.update(&pair.pack);
    aggregate.update(&pair.spr);
    public.aggregate_sha256 = ArtifactHash(aggregate.finalize());
    public.checkpoint_id = ArtifactHash(Sha256::digest(&semio_hub_artifact_authority::checkpoint_id_encoding_v1(&checkpoint_identity_input(&public)).expect("checkpoint identity")));
    let mut checkpoint = checkpoint_identity_input(&public);
    checkpoint.pack.storage_key = artifact_cas_manifest_locator_v1(prepare_artifact_cas_manifest_v1(&checkpoint.scope.space_id, &pair.pack).expect("pack plan").manifest_id);
    checkpoint.spr.storage_key = artifact_cas_manifest_locator_v1(prepare_artifact_cas_manifest_v1(&checkpoint.scope.space_id, &pair.spr).expect("SPR plan").manifest_id);
    checkpoint
}

fn scoped_materialized_checkpoint(mut public: PublishedArtifactCheckpoint, descriptor: &DocumentDescriptor, pair: &ArtifactPair, parent_checkpoint_id: Option<ArtifactHash>, ordinal: u64) -> ArtifactCheckpoint {
    public.scope = DocumentScope::new(descriptor.space_id.clone(), descriptor.document_id.clone());
    public.parent_checkpoint_id = parent_checkpoint_id;
    public.descriptor_digest_v1 = descriptor_digest_v1(descriptor).expect("descriptor digest");
    public.baseline_frontier.document_id = descriptor.document_id.clone();
    public.baseline_frontier.head_edit_ordinal = ordinal;
    public.baseline_frontier.head_edit_id = format!("edit-{ordinal}");
    public.baseline_frontier.last_commit_seq = ordinal;
    public.baseline_frontier.chain_hash = ArtifactHash(Sha256::digest(&ordinal.to_be_bytes()));
    public.published_at_ms = ordinal;
    materialized_checkpoint(public, pair)
}

fn generic_payload_pages(bytes: &[u8]) -> db::storage::DbIoPages {
    let mut writer = DbIoPageWriter::try_reserve(bytes.len().div_ceil(DB_IO_PAGE_BYTES)).expect("generic payload pages");
    for fragment in bytes.chunks(DB_IO_PAGE_BYTES) {
        assert_eq!(writer.write_fragment(fragment).expect("generic payload fragment"), fragment.len());
    }
    loop {
        if let Some(pages) = writer.seal_retained_step().expect("generic payload pages seal") {
            return pages;
        }
    }
}

async fn stage_reserved_checkpoint<S: ArtifactChunkCasStorage>(
    service: &DirectoryService,
    storage: Arc<S>,
    actor: DirectoryActor,
    checkpoint: &ArtifactCheckpoint,
    pair: &ArtifactPair,
    expires_at_ms: u64,
    now_ms: u64,
    context: &OperationContext<'_>,
) -> DirectoryResult<ArtifactCasReservation> {
    let plan = prepare_artifact_cas_ownership_v1(checkpoint, pair).map_err(|error| DirectoryError::Conflict(error.to_string()))?;
    let reservation = service.reserve_artifact_cas(actor, plan, expires_at_ms, now_ms).await?;
    let coordinator_id = *reservation.coordinator_id();
    storage.configure_coordinator(coordinator_id, context).await.map_err(|error| DirectoryError::Backend(error.to_string()))?;
    storage.advance_physical_epoch(coordinator_id, &checkpoint.scope.space_id, reservation.physical_epoch(), context).await.map_err(|error| DirectoryError::Backend(error.to_string()))?;
    let blobs = ArtifactChunkBlobStore::new(storage);
    let pack = blobs.stage(&checkpoint.scope.space_id, ArtifactBlobIntegrity { sha256: checkpoint.pack.sha256, byte_length: checkpoint.pack.byte_length }, &pair.pack, context).await.map_err(|error| DirectoryError::Backend(error.to_string()))?;
    let spr = blobs.stage(&checkpoint.scope.space_id, ArtifactBlobIntegrity { sha256: checkpoint.spr.sha256, byte_length: checkpoint.spr.byte_length }, &pair.spr, context).await.map_err(|error| DirectoryError::Backend(error.to_string()))?;
    if pack.storage_key != checkpoint.pack.storage_key || spr.storage_key != checkpoint.spr.storage_key {
        return Err(DirectoryError::Conflict("artifact CAS pre-write plan changed while staging".into()));
    }
    Ok(reservation)
}

fn sweep_convergence_plan(index: usize, object_count: usize) -> ArtifactCasOwnershipPlanV1 {
    let digest = |kind: &str, object: usize| ArtifactHash(Sha256::digest(format!("sweep-continuation:{index}:{kind}:{object}").as_bytes()));
    let pack_manifest_id = digest("pack-manifest", 0);
    let spr_manifest_id = digest("spr-manifest", 0);
    let mut objects: Vec<_> = (0..object_count.saturating_sub(2)).map(|object| ArtifactCasObjectKey { space_id: "sweep-space".into(), kind: ArtifactCasObjectKind::Chunk, digest: digest("chunk", object) }).collect();
    objects.push(ArtifactCasObjectKey { space_id: "sweep-space".into(), kind: ArtifactCasObjectKind::Manifest, digest: pack_manifest_id });
    objects.push(ArtifactCasObjectKey { space_id: "sweep-space".into(), kind: ArtifactCasObjectKind::Manifest, digest: spr_manifest_id });
    objects.sort_by_key(|object| (object.kind, object.digest.0));
    ArtifactCasOwnershipPlanV1 { scope: DocumentScope::new("sweep-space", format!("sweep-document-{index}")), checkpoint_id: digest("checkpoint", 0), pack_manifest_id, spr_manifest_id, objects }
}

fn artifact_event(seq: u64, body: DirectoryEventBody) -> DirectoryEvent {
    DirectoryEvent {
        seq,
        id: format!("event-{seq}"),
        hlc: Hlc { physical_ms: seq as i64, logical: 0 },
        actor: DirectoryActor { kind: DirectoryActorKind::System, id: "system:artifact-authority".into() },
        space_id: Some("raum:ä".into()),
        user_id: None,
        body,
        recorded_at_ms: seq as i64,
    }
}

struct RebuildProbe {
    cancelled: AtomicBool,
    cancel_after_first: bool,
    progress: std::sync::Mutex<Vec<ProjectionRebuildProgress>>,
}

impl ProjectionRebuildControl for RebuildProbe {
    fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::SeqCst)
    }

    fn report(&self, progress: ProjectionRebuildProgress) {
        self.progress.lock().expect("progress").push(progress);
        if self.cancel_after_first && progress.completed_events == 1 {
            self.cancelled.store(true, Ordering::SeqCst);
        }
    }
}
