use super::sqlite::SqliteDirectory;
use super::*;
use crate::artifact_authority::chunk_cas::{
    ArtifactCasObjectKind, ArtifactChunkBlobStore, FsArtifactChunkCasStorage, MemoryArtifactChunkCasStorage, artifact_cas_manifest_locator_v1, prepare_artifact_cas_manifest_v1, prepare_artifact_cas_ownership_v1,
};
use crate::artifact_authority::{ArtifactBlobIntegrity, ArtifactPair, AuthorityLimits, AuthorityOperationControl, AuthorityProgress, AuthorityProgressStage, ImmutableArtifactBlobStore, OperationContext, StagedArtifactBlob};
use db::db_storage::{DB_IO_PAGE_BYTES, DbIoPageWriter, MemoryStorage as GenericMemoryStorage, PayloadStorage};
use directory::{DslValue, FromValue, ToValue};
use std::collections::BTreeSet;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

struct IdentityProbe {
    now: i64,
    cancelled: bool,
    progress: std::sync::Mutex<Vec<IdentityVerificationProgress>>,
}

impl IdentityVerificationControl for IdentityProbe {
    fn now_ms(&self) -> i64 {
        self.now
    }

    fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    fn report(&self, progress: IdentityVerificationProgress) {
        self.progress.lock().expect("identity progress lock").push(progress);
    }
}

struct TestIdentityVerifier {
    fail: bool,
}

impl IdentityAssertionVerifier for TestIdentityVerifier {
    fn verify<'a>(&'a self, assertion: &'a IdentityAssertion, context: &'a IdentityVerificationContext<'a>) -> IdentityVerificationFuture<'a> {
        Box::pin(async move {
            context.checkpoint(0, 2)?;
            semio_framework_async::yield_once().await;
            context.checkpoint(1, 2)?;
            if self.fail || assertion.as_bytes() != b"signed-test-assertion" {
                return Err(DirectoryError::Unauthorized);
            }
            context.checkpoint(2, 2)?;
            Ok(VerifiedIdentity {
                provider: "test-verifier".into(),
                subject: "test-subject".into(),
                verified_email: Some("verified@example.com".into()),
                display_name: Some("Verified".into()),
                issued_at: 1,
                expires_at: 2,
                assurance: IdentityAssurance::ExternalVerified,
            })
        })
    }
}

#[test]
fn typed_capabilities_match_neutral_sha256_vectors_and_fixed_boundaries() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🔐️auth/🧫️fixtures/🔑️capability-v1/🔣️.json")).expect("auth capability fixture");
    let session = SessionCapability::parse(fixture["session"]["capability"].as_str().expect("session capability")).expect("session parser");
    let share = ShareCapability::parse(fixture["share"]["capability"].as_str().expect("share capability")).expect("share parser");
    let invite = InviteCapability::parse(fixture["invite"]["capability"].as_str().expect("invite capability")).expect("invite parser");
    let socket = SocketGrantCapability::parse(fixture["socket"]["capability"].as_str().expect("socket capability")).expect("socket parser");
    assert_eq!(session.selector(), fixture["session"]["selector"].as_str().expect("session selector"));
    assert_eq!(encode_capability_bytes(&session.secret_digest()), fixture["session"]["digestHex"].as_str().expect("session digest"));
    assert_eq!(encode_capability_bytes(&share.secret_digest()), fixture["share"]["digestHex"].as_str().expect("share digest"));
    assert_eq!(encode_capability_bytes(&invite.secret_digest()), fixture["invite"]["digestHex"].as_str().expect("invite digest"));
    assert_eq!(socket.selector(), fixture["socket"]["selector"].as_str().expect("socket selector"));
    assert_eq!(encode_capability_bytes(&socket.secret_digest()), fixture["socket"]["digestHex"].as_str().expect("socket digest"));
    assert_eq!(socket.expose_once().len(), 107);
    assert!(HubCapability::parse(&socket.expose_once()).is_err(), "socket grants are never HTTP hub capabilities");
    let socket_rejections = fixture["socket"]["rejectedCapabilities"].as_array().expect("socket rejection vectors");
    for rejected in &socket_rejections[..4] {
        assert!(SocketGrantCapability::parse(rejected.as_str().expect("socket rejection")).is_err());
    }
    let wrong_secret = SocketGrantCapability::parse(socket_rejections[4].as_str().expect("wrong-secret socket capability")).expect("wrong-secret grammar is valid");
    assert!(!constant_time_digest_eq(&wrong_secret.secret_digest(), &socket.secret_digest()));
    assert!(SessionCapability::parse(fixture["share"]["capability"].as_str().expect("share capability")).is_err());
    assert!(ShareCapability::parse(&share.expose_once().to_uppercase()).is_err());
    assert_eq!(capability_window(1, CAPABILITY_MAX_TTL_SECS).expect("maximum ttl").1, 1 + CAPABILITY_MAX_TTL_SECS * 1_000);
    assert!(capability_window(1, CAPABILITY_MAX_TTL_SECS + 1).is_err());
    assert!(IdentityAssertion::new(vec![0; AUTH_ASSERTION_MAX_BYTES].into_boxed_slice()).is_ok());
    assert!(IdentityAssertion::new(vec![0; AUTH_ASSERTION_MAX_BYTES + 1].into_boxed_slice()).is_err());
    let mut issue = AuthSessionIssue {
        user_id: "fixture-user".into(),
        identity_provider: "oidc.example".into(),
        identity_subject_digest: [7; 32],
        ttl_secs: 60,
        device_instance_id: "d".repeat(DEVICE_INSTANCE_MAX_BYTES),
        session_kind: AuthSessionKind::External,
        correlation_id: "fixture-correlation".into(),
        peer_class: "fixture".into(),
    };
    assert!(prepare_auth_session(&issue, 1).is_ok());
    issue.device_instance_id.push('d');
    assert!(prepare_auth_session(&issue, 1).is_err());
    assert!(constant_time_digest_eq(&[9; 32], &[9; 32]));
    assert!(!constant_time_digest_eq(&[9; 32], &[8; 32]));
    assert_eq!(
        encode_capability_bytes(&identity_subject_digest(fixture["identity"]["provider"].as_str().expect("provider"), fixture["identity"]["subject"].as_str().expect("subject")).expect("identity digest")),
        fixture["identity"]["digestHex"].as_str().expect("identity digest vector"),
    );
}

#[tokio::test]
async fn identity_verifier_port_honors_progress_cancel_deadline_and_provider_error() {
    let assertion = IdentityAssertion::new(b"signed-test-assertion".to_vec().into_boxed_slice()).expect("bounded assertion");
    let success = IdentityProbe { now: 10, cancelled: false, progress: std::sync::Mutex::new(Vec::new()) };
    let verified = TestIdentityVerifier { fail: false }.verify(&assertion, &IdentityVerificationContext { deadline_ms: 10, control: &success }).await.expect("verified identity");
    assert_eq!(verified.subject, "test-subject");
    assert_eq!(
        success.progress.into_inner().expect("success progress"),
        vec![IdentityVerificationProgress { completed_units: 0, total_units: 2 }, IdentityVerificationProgress { completed_units: 1, total_units: 2 }, IdentityVerificationProgress { completed_units: 2, total_units: 2 }]
    );

    let cancelled = IdentityProbe { now: 10, cancelled: true, progress: std::sync::Mutex::new(Vec::new()) };
    assert!(matches!(TestIdentityVerifier { fail: false }.verify(&assertion, &IdentityVerificationContext { deadline_ms: 10, control: &cancelled }).await, Err(DirectoryError::Conflict(_))));
    let expired = IdentityProbe { now: 11, cancelled: false, progress: std::sync::Mutex::new(Vec::new()) };
    assert!(matches!(TestIdentityVerifier { fail: false }.verify(&assertion, &IdentityVerificationContext { deadline_ms: 10, control: &expired }).await, Err(DirectoryError::Conflict(_))));
    let provider_error = IdentityProbe { now: 10, cancelled: false, progress: std::sync::Mutex::new(Vec::new()) };
    assert!(matches!(TestIdentityVerifier { fail: true }.verify(&assertion, &IdentityVerificationContext { deadline_ms: 10, control: &provider_error }).await, Err(DirectoryError::Unauthorized)));
}

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
    let pack_manifest_id = crate::artifact_authority::chunk_cas::decode_artifact_cas_manifest_locator_v1(&checkpoint.pack.storage_key).expect("pack manifest locator");
    let spr_manifest_id = crate::artifact_authority::chunk_cas::decode_artifact_cas_manifest_locator_v1(&checkpoint.spr.storage_key).expect("SPR manifest locator");
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

async fn publish_fixture_genesis<S: ArtifactChunkCasStorage>(service: &DirectoryService, user_id: &str, mut descriptor: DocumentDescriptor, storage: Arc<S>, context: &OperationContext<'_>) -> (DocumentDescriptor, ArtifactCheckpoint) {
    use crate::artifact_authority::creation::{ARTIFACT_CREATION_DEADLINE_MS, ArtifactCreationActorV1, ArtifactCreationFactBodyV1, ArtifactCreationPreparedV1, artifact_creation_command_digest_v1};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🗿️artifact-authority/🌱️creation/🧫️fixtures/📚️operation-v1/🔣️.json")).expect("creation fixture");
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
    intent.parent_dialect.artifact_kind = descriptor.artifact_kind.clone();
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
    prepared.checkpoint.pack.storage_key = format!("sha256/{}", checkpoint.pack.sha256.hex());
    prepared.checkpoint.spr.storage_key = format!("sha256/{}", checkpoint.spr.sha256.hex());
    prepared.validate(&intent).expect("exact prepared genesis");
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
    async fn configure_coordinator(&self, coordinator_id: [u8; 32], context: &OperationContext<'_>) -> Result<(), crate::artifact_authority::AuthorityError> {
        self.inner.configure_coordinator(coordinator_id, context).await
    }

    async fn advance_physical_epoch(&self, coordinator_id: [u8; 32], space_id: &str, epoch: u64, context: &OperationContext<'_>) -> Result<(), crate::artifact_authority::AuthorityError> {
        self.inner.advance_physical_epoch(coordinator_id, space_id, epoch, context).await
    }

    async fn put_if_absent(&self, key: &ArtifactCasObjectKey, bytes: &[u8], context: &OperationContext<'_>) -> Result<crate::artifact_authority::chunk_cas::ArtifactCasPutOutcome, crate::artifact_authority::AuthorityError> {
        self.inner.put_if_absent(key, bytes, context).await
    }

    async fn get(&self, key: &ArtifactCasObjectKey, context: &OperationContext<'_>) -> Result<Vec<u8>, crate::artifact_authority::AuthorityError> {
        self.inner.get(key, context).await
    }

    async fn delete_if_unreferenced(&self, key: &ArtifactCasObjectKey, fence: &ArtifactCasDeleteFence, context: &OperationContext<'_>) -> Result<ArtifactCasDeleteOutcome, crate::artifact_authority::AuthorityError> {
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
    async fn configure_coordinator(&self, coordinator_id: [u8; 32], context: &OperationContext<'_>) -> Result<(), crate::artifact_authority::AuthorityError> {
        self.inner.configure_coordinator(coordinator_id, context).await
    }

    async fn advance_physical_epoch(&self, coordinator_id: [u8; 32], space_id: &str, epoch: u64, context: &OperationContext<'_>) -> Result<(), crate::artifact_authority::AuthorityError> {
        self.inner.advance_physical_epoch(coordinator_id, space_id, epoch, context).await
    }

    async fn put_if_absent(&self, key: &ArtifactCasObjectKey, bytes: &[u8], context: &OperationContext<'_>) -> Result<crate::artifact_authority::chunk_cas::ArtifactCasPutOutcome, crate::artifact_authority::AuthorityError> {
        self.inner.put_if_absent(key, bytes, context).await
    }

    async fn get(&self, key: &ArtifactCasObjectKey, context: &OperationContext<'_>) -> Result<Vec<u8>, crate::artifact_authority::AuthorityError> {
        self.inner.get(key, context).await
    }

    async fn delete_if_unreferenced(&self, key: &ArtifactCasObjectKey, fence: &ArtifactCasDeleteFence, context: &OperationContext<'_>) -> Result<ArtifactCasDeleteOutcome, crate::artifact_authority::AuthorityError> {
        std::fs::write(&self.entered, []).map_err(|_| crate::artifact_authority::AuthorityError::Store("artifact CAS process race readiness write failed".into()))?;
        let wait_deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(30);
        while !self.release.exists() {
            context.checkpoint()?;
            if tokio::time::Instant::now() >= wait_deadline {
                return Err(crate::artifact_authority::AuthorityError::Store("artifact CAS process race release timed out".into()));
            }
            tokio::time::sleep(std::time::Duration::from_millis(5)).await;
        }
        self.inner.delete_if_unreferenced(key, fence, context).await
    }
}

struct FailingAdvanceArtifactCas(MemoryArtifactChunkCasStorage);

impl ArtifactChunkCasStorage for FailingAdvanceArtifactCas {
    async fn configure_coordinator(&self, coordinator_id: [u8; 32], context: &OperationContext<'_>) -> Result<(), crate::artifact_authority::AuthorityError> {
        self.0.configure_coordinator(coordinator_id, context).await
    }

    async fn advance_physical_epoch(&self, _: [u8; 32], _: &str, _: u64, _: &OperationContext<'_>) -> Result<(), crate::artifact_authority::AuthorityError> {
        Err(crate::artifact_authority::AuthorityError::Cancelled)
    }

    async fn put_if_absent(&self, key: &ArtifactCasObjectKey, bytes: &[u8], context: &OperationContext<'_>) -> Result<crate::artifact_authority::chunk_cas::ArtifactCasPutOutcome, crate::artifact_authority::AuthorityError> {
        self.0.put_if_absent(key, bytes, context).await
    }

    async fn get(&self, key: &ArtifactCasObjectKey, context: &OperationContext<'_>) -> Result<Vec<u8>, crate::artifact_authority::AuthorityError> {
        self.0.get(key, context).await
    }

    async fn delete_if_unreferenced(&self, key: &ArtifactCasObjectKey, fence: &ArtifactCasDeleteFence, context: &OperationContext<'_>) -> Result<ArtifactCasDeleteOutcome, crate::artifact_authority::AuthorityError> {
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
    public.checkpoint_id = ArtifactHash(Sha256::digest(&crate::artifact_authority::checkpoint_id_encoding_v1(&checkpoint_identity_input(&public)).expect("checkpoint identity")));
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

fn generic_payload_pages(bytes: &[u8]) -> db::db_storage::DbIoPages {
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

// 🌱️ Every test in this module creates a space owned by `user_actor("u-owner")`; `decide`'s
// `CreateSpace` arm (correctly) never mints the owner's `hub_user` row itself — it only has an
// actor id, no email, so it cannot self-heal a missing user the way `UpsertMember` can. In
// production the owner's `hub_user` row must predate `create-space`; the trusted identity
// completion boundary provisions it before issuing a session. This fixture reproduces that precondition by
// appending a bare `user.created` event for `"u-owner"` under a `System` actor, mirroring
// `SqliteDirectory::seed`'s own pattern one file over.
async fn fresh_dir() -> Arc<HubDirectories> {
    let dir = SqliteDirectory::connect(":memory:").await.expect("connect");
    let seed_actor = DirectoryActor { kind: DirectoryActorKind::System, id: "system:test-seed".into() };
    let mut clock = HubClock::new();
    let events = vec![new_event(&mut clock, &seed_actor, None, Some("u-owner".into()), DirectoryEventBody::UserCreated { user_id: "u-owner".into(), email: "u-owner@example.com".into(), display_name: "Owner".into() })];
    dir.append_events(&events).await.expect("seed owner user");
    Arc::new(HubDirectories::from(dir))
}

fn admin_audit_fact(request_id: &str, digest_byte: &str, phase: &str, outcome: &str) -> NewAdminOperationAuditRecord {
    NewAdminOperationAuditRecord {
        request_id: request_id.into(),
        intent_digest: digest_byte.repeat(64),
        operation_id: format!("operation:{request_id}"),
        occurred_at: 1,
        phase: phase.into(),
        intent_kind: "delete-space".into(),
        target_kind: "space".into(),
        target_id: "space:one".into(),
        principal_user_id: "user:admin".into(),
        principal_session_id: "session:admin".into(),
        principal_generation: 7,
        correlation_id: "correlation:admin".into(),
        event_seq_first: None,
        event_seq_last: None,
        outcome_code: outcome.into(),
        reason_code: None,
    }
}

#[tokio::test]
async fn admin_operation_audit_concurrent_first_writer_is_idempotent_and_first_terminal_wins() {
    let directory = fresh_dir().await;
    let accepted = admin_audit_fact("request:race", "1", "accepted", "accepted");
    let barrier = Arc::new(tokio::sync::Barrier::new(33));
    let mut writers = Vec::new();
    for _ in 0..32 {
        let directory = directory.clone();
        let accepted = accepted.clone();
        let barrier = barrier.clone();
        writers.push(tokio::spawn(async move {
            barrier.wait().await;
            directory.append_admin_operation_audit(&accepted).await
        }));
    }
    barrier.wait().await;
    let mut sequences = BTreeSet::new();
    for writer in writers {
        sequences.insert(writer.await.expect("race writer").expect("idempotent append").sequence);
    }
    assert_eq!(sequences.len(), 1);
    assert_eq!(directory.admin_operation_audit_for_request("request:race").await.expect("race audit").len(), 1);
    let collision = admin_audit_fact("request:race", "2", "accepted", "accepted");
    assert!(matches!(directory.append_admin_operation_audit(&collision).await, Err(DirectoryError::Conflict(_))));

    let cancelled = admin_audit_fact("request:race", "1", "cancelled", "operator-cancelled");
    let failed = admin_audit_fact("request:race", "1", "failed", "late-failure");
    assert_eq!(directory.append_admin_operation_audit(&cancelled).await.expect("cancel terminal").fact.phase, "cancelled");
    assert!(matches!(directory.append_admin_operation_audit(&failed).await, Err(DirectoryError::Conflict(_))), "a different later terminal cannot masquerade as the winning cancellation");
    assert_eq!(directory.append_admin_operation_audit(&cancelled).await.expect("idempotent winning terminal").fact.phase, "cancelled");
    let operation_rows = directory.admin_operation_audit_for_operation("operation:request:race").await.expect("operation-id status lookup");
    assert_eq!(operation_rows.len(), 2);
    assert!(operation_rows.iter().all(|row| row.fact.request_id == "request:race"));
    let rows = directory.list_admin_operation_audit(0, ADMIN_PAGE_MAX).await.expect("bounded audit page");
    assert_eq!(rows.len(), 2);
    assert!(directory.list_admin_operation_audit(0, ADMIN_PAGE_MAX + 1).await.is_err());
}

#[tokio::test]
async fn admin_bounded_overview_space_and_document_projections_enforce_exact_page_boundary() {
    let directory = fresh_dir().await;
    let service = DirectoryService::new(directory.clone(), 64);
    let owner = user_actor("u-owner");
    let space_id = create_space(&service, &owner, DirectorySpaceKind::Studio).await;
    for index in 0..=ADMIN_PAGE_MAX {
        service.execute(owner.clone(), DirectoryCommand::AnnounceDocument { descriptor: Box::new(descriptor(&space_id, &format!("document:{index:03}"))) }).await.expect("announce bounded-page document");
    }

    assert_eq!(directory.admin_overview_counts().await.expect("constant-space overview counts"), AdminDirectoryOverviewCounts { spaces: 1, users: 1, connections: 0 },);
    let spaces = directory.list_admin_space_summaries_page(None, 0, ADMIN_PAGE_FETCH_MAX).await.expect("bounded space summary page");
    assert_eq!(spaces.len(), 1);
    assert_eq!(spaces[0].member_count, 1);
    assert_eq!(spaces[0].document_count, u64::try_from(ADMIN_PAGE_FETCH_MAX).expect("page maximum fits u64"));
    assert_eq!(spaces[0].active_connections, 0);
    assert_eq!(directory.list_admin_space_summaries_page(Some(&space_id), 0, 1).await.expect("exact space summary").len(), 1);
    assert_eq!(directory.list_admin_space_members_page(&space_id, 0, ADMIN_PAGE_FETCH_MAX).await.expect("bounded member page").len(), 1);
    assert!(directory.list_admin_space_summaries_page(None, 0, 0).await.is_err());
    assert!(directory.list_admin_space_summaries_page(None, 0, ADMIN_PAGE_FETCH_MAX + 1).await.is_err());
    assert!(directory.list_admin_space_members_page(&space_id, 0, 0).await.is_err());
    assert!(directory.list_admin_space_members_page(&space_id, 0, ADMIN_PAGE_FETCH_MAX + 1).await.is_err());
    let first = directory.list_document_descriptors_page(None, 0, ADMIN_PAGE_FETCH_MAX).await.expect("page plus continuation probe");
    assert_eq!(first.len(), ADMIN_PAGE_FETCH_MAX);
    assert_eq!(first.first().expect("first descriptor").document_id, "document:000");
    assert_eq!(first.last().expect("continuation descriptor").document_id, "document:100");
    let continuation = directory.list_document_descriptors_page(Some(&space_id), ADMIN_PAGE_MAX, ADMIN_PAGE_FETCH_MAX).await.expect("scoped continuation page");
    assert_eq!(continuation.len(), 1);
    assert_eq!(continuation[0].document_id, "document:100");
    assert!(directory.list_document_descriptors_page(None, 0, 0).await.is_err());
    assert!(directory.list_document_descriptors_page(None, 0, ADMIN_PAGE_FETCH_MAX + 1).await.is_err());
}

async fn create_space(service: &DirectoryService, owner: &DirectoryActor, kind: DirectorySpaceKind) -> String {
    let (events, _) = service.execute(owner.clone(), DirectoryCommand::CreateSpace { name: "Space".into(), space_kind: kind, visibility: DirectorySpaceVisibility::Private }).await.expect("create-space");
    events[0].space_id.clone().expect("space id on space.created")
}

/// 📣️ A committed page cannot be overtaken on the live channel while it still owns the writer guard.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn directory_append_and_live_broadcast_share_one_writer_guard_and_projection_order() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📣️ordered-append-broadcast-v1/🔣️.json")).expect("language-neutral ordered publication fixture");
    assert_eq!(fixture["schema"], "semio.hub.directory.ordered-append-broadcast/v1");
    assert_eq!(fixture["cases"].as_array().expect("ordered publication cases").len(), 4);

    let dir = fresh_dir().await;
    let service = Arc::new(DirectoryService::new(dir.clone(), 16));
    let owner = user_actor("u-owner");
    let space_id = create_space(service.as_ref(), &owner, DirectorySpaceKind::Studio).await;
    service.execute(owner.clone(), DirectoryCommand::UpsertMember { space_id: space_id.clone(), email: "ordered@example.com".into(), role: DirectorySpaceRole::Spectator }).await.expect("seed ordered member");
    let member_id = dir.list_members(&space_id).await.expect("seeded member projection").into_iter().find(|(user, _)| user.email == "ordered@example.com").expect("ordered member").0.id;
    let since = dir.head_seq().await.expect("head before concurrent publication");
    let mut receiver = service.subscribe();
    let fence = service.arm_publication_test_fence();

    let first_service = service.clone();
    let first_owner = owner.clone();
    let first_space = space_id.clone();
    let first =
        tokio::spawn(async move { first_service.execute(first_owner, DirectoryCommand::UpsertMember { space_id: first_space, email: "ordered@example.com".into(), role: DirectorySpaceRole::Author }).await.expect("first concurrent member update").0 });
    tokio::time::timeout(std::time::Duration::from_secs(1), fence.reached.notified()).await.expect("first committed page reached publication fence");

    let second_service = service.clone();
    let second_owner = owner.clone();
    let second_space = space_id.clone();
    let second = tokio::spawn(async move {
        second_service.execute(second_owner, DirectoryCommand::UpsertMember { space_id: second_space, email: "ordered@example.com".into(), role: DirectorySpaceRole::Spectator }).await.expect("second concurrent member update").0
    });
    assert!(tokio::time::timeout(std::time::Duration::from_millis(50), receiver.recv()).await.is_err(), "a later writer cannot append or broadcast while the committed first page is fenced");
    fence.release.notify_one();

    let first_events = first.await.expect("first writer task");
    let second_events = second.await.expect("second writer task");
    assert_eq!(first_events.len(), 1);
    assert_eq!(second_events.len(), 1);
    assert_eq!([first_events[0].seq, second_events[0].seq], [since + 1, since + 2]);
    let mut observed = Vec::new();
    for _ in 0..2 {
        match tokio::time::timeout(std::time::Duration::from_secs(1), receiver.recv()).await.expect("ordered live event deadline").expect("ordered live event") {
            DirectoryStreamMessage::Event { event } => observed.push(event.seq),
            _ => panic!("directory writer emitted a non-event message"),
        }
    }
    assert_eq!(observed, vec![since + 1, since + 2]);
    assert_eq!(dir.events_since(since, 2).await.expect("durable concurrent event page").iter().map(|event| event.seq).collect::<Vec<_>>(), observed);
    assert_eq!(dir.list_members(&space_id).await.expect("final member projection").into_iter().find(|(user, _)| user.id == member_id).expect("final ordered member").1, SpaceRole::Spectator,);
}

// 🔬️ Replaying the whole log through `rebuild_projections` reproduces the exact same
// projections a live command stream built, and `events_since(0)` is dense `1..=head_seq()`.
#[tokio::test]
async fn event_log_replay_matches_projections() {
    let dir = fresh_dir().await;
    let service = DirectoryService::new(dir.clone(), 64);
    let owner = user_actor("u-owner");
    let space_id = create_space(&service, &owner, DirectorySpaceKind::Studio).await;
    service.execute(owner.clone(), DirectoryCommand::UpsertMember { space_id: space_id.clone(), email: "member@example.com".into(), role: DirectorySpaceRole::Spectator }).await.expect("upsert-member");
    service.execute(owner.clone(), DirectoryCommand::RenameSpace { space_id: space_id.clone(), name: "Renamed".into() }).await.expect("rename-space");

    let head = dir.head_seq().await.expect("head seq");
    let seqs: Vec<u64> = dir.events_since(0, 100).await.expect("events since").iter().map(|event| event.seq).collect();
    assert_eq!(seqs, (1..=head).collect::<Vec<_>>(), "seq is dense 1..n");

    let spaces_before = dir.list_spaces(100, 0).await.expect("list spaces");
    let members_before = dir.list_members(&space_id).await.expect("list members");
    let users_before = dir.list_users(100, 0).await.expect("list users");

    let replayed = dir.rebuild_projections().await.expect("rebuild");
    assert_eq!(replayed, head);
    assert_eq!(dir.list_spaces(100, 0).await.expect("list spaces"), spaces_before);
    assert_eq!(dir.list_members(&space_id).await.expect("list members"), members_before);
    assert_eq!(dir.list_users(100, 0).await.expect("list users"), users_before);
}

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
        Err(crate::artifact_authority::AuthorityError::Cancelled)
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
async fn artifact_chunk_cas_opaque_continuation_converges_after_page_overflow_cancel_and_resume() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🗿️artifact-authority/🧫️fixtures/🧱️artifact-chunk-cas/🔣️.json")).expect("artifact CAS fixture");
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
        Err(crate::artifact_authority::AuthorityError::ResourceLimit("artifact CAS sweep object"))
    ));
    let first = service.sweep_artifact_cas(&storage, ArtifactCasSweepRequest { execute: true, max_objects: maximum, continuation: None }, &context).await.expect("first bounded sweep");
    let continuation = first.continuation.expect("first sweep continuation");
    assert_eq!(first.examined_objects, law["expectedExaminedPerRequest"][0].as_u64().expect("first examined"));
    assert_eq!(format!("{continuation:?}"), "ArtifactCasSweepContinuation(<opaque>)");
    let first_position = service.artifact_cas_sweep_position(continuation, true).expect("decode owned continuation");
    assert_eq!(first_position.observed_generation, law["expectedFirstCursor"]["observedGeneration"].as_u64().expect("cursor generation"));
    assert_eq!(first_position.after_generation, law["expectedFirstCursor"]["afterGeneration"].as_u64().expect("cursor page"));
    assert_eq!(first_position.object_offset as u64, law["expectedFirstCursor"]["objectOffset"].as_u64().expect("cursor offset"));
    assert!(matches!(service.sweep_artifact_cas(&storage, ArtifactCasSweepRequest { execute: false, max_objects: maximum, continuation: Some(continuation) }, &context).await, Err(crate::artifact_authority::AuthorityError::Store(_))));

    let restarted = DirectoryService::new(dir.clone(), 64);
    assert!(matches!(restarted.sweep_artifact_cas(&storage, ArtifactCasSweepRequest { execute: true, max_objects: maximum, continuation: Some(continuation) }, &context).await, Err(crate::artifact_authority::AuthorityError::Store(_))));
    let cancel_probe = ArtifactCasProbe::new(201, Some(1));
    let cancel_context = OperationContext::new(10_000, AuthorityLimits::maximum(), &cancel_probe);
    assert!(matches!(service.sweep_artifact_cas(&storage, ArtifactCasSweepRequest { execute: true, max_objects: maximum, continuation: Some(continuation) }, &cancel_context).await, Err(crate::artifact_authority::AuthorityError::Cancelled)));
    assert_eq!(cancel_probe.progress.lock().expect("cancel progress").iter().filter(|progress| progress.stage == AuthorityProgressStage::CasSweep).count(), 1);
    let second = service.sweep_artifact_cas(&storage, ArtifactCasSweepRequest { execute: true, max_objects: maximum, continuation: Some(continuation) }, &context).await.expect("resume bounded sweep");
    assert_eq!(second.examined_objects, law["expectedExaminedPerRequest"][1].as_u64().expect("second examined"));
    assert!(second.continuation.is_none());
    assert_eq!(first.examined_objects + second.examined_objects, total_objects);

    let changed = service.sweep_artifact_cas(&storage, ArtifactCasSweepRequest { execute: true, max_objects: 1, continuation: None }, &context).await.expect("generation-bound continuation").continuation.expect("generation-bound token");
    service.reserve_artifact_cas(system, sweep_convergence_plan(object_counts.len(), 2), 500, 300).await.expect("advance sweep generation");
    assert!(matches!(service.sweep_artifact_cas(&storage, ArtifactCasSweepRequest { execute: true, max_objects: maximum, continuation: Some(changed) }, &context).await, Err(crate::artifact_authority::AuthorityError::Store(_))));
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
    assert!(matches!(service.sweep_artifact_cas(&storage, ArtifactCasSweepRequest { execute: true, max_objects: 1, continuation: None }, &context).await, Err(crate::artifact_authority::AuthorityError::Cancelled)));
    service.reserve_artifact_cas(system, plan, 400, 201).await.expect("failure cleanup releases live lease immediately");
}

#[tokio::test]
async fn artifact_chunk_cas_two_service_sweep_and_reservation_race_is_serialized_before_rewrite() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🗿️artifact-authority/🧫️fixtures/🧱️artifact-chunk-cas/🔣️.json")).expect("artifact CAS fixture");
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
    assert!(matches!(&error, crate::artifact_authority::AuthorityError::Store(message) if message == "artifact CAS deletion fence is stale"), "unexpected raced sweep error: {error:?}");
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
            assert!(matches!(&error, crate::artifact_authority::AuthorityError::Store(message) if message == "artifact CAS deletion fence is stale"), "unexpected old process sweep error: {error:?}");
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

    let root = std::path::PathBuf::from(std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").expect("ticket generated artifact root")).join(format!("semio-artifact-cas-directory-process-race-{}", directory::os_identity::time_ordered_id()));
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
    println!("[DEBUG] Filesystem CAS process race: dedicated genesis=1 edited successor=1 separate-process stale deletion refusal=1 exact pair readbacks=4");
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
    first.checkpoint_id = ArtifactHash(Sha256::digest(&crate::artifact_authority::checkpoint_id_encoding_v1(&checkpoint_identity_input(&first)).expect("first identity")));
    second.parent_checkpoint_id = Some(first.checkpoint_id);
    second.checkpoint_id = ArtifactHash(Sha256::digest(&crate::artifact_authority::checkpoint_id_encoding_v1(&checkpoint_identity_input(&second)).expect("second identity")));
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
    late_root.checkpoint_id = ArtifactHash(Sha256::digest(&crate::artifact_authority::checkpoint_id_encoding_v1(&checkpoint_identity_input(&late_root)).expect("late root identity")));
    assert!(projection.fold_atomically(&[artifact_event(7, DirectoryEventBody::ArtifactCheckpointPublished { checkpoint: late_root })]).is_err());

    projection.checkpoints.insert(first.scope.clone(), vec![first.clone(); fixture_maximum as usize]);
    assert!(projection.fold_atomically(&[artifact_event(8, DirectoryEventBody::ArtifactCheckpointPublished { checkpoint: second })]).is_err());
    let probe = RebuildProbe { cancelled: AtomicBool::new(false), cancel_after_first: false, progress: std::sync::Mutex::new(Vec::new()) };
    checkpoint_projection_rebuild(&probe, DIRECTORY_PROJECTION_REBUILD_MAX_EVENTS, DIRECTORY_PROJECTION_REBUILD_MAX_EVENTS).expect("rebuild exact maximum");
    assert!(checkpoint_projection_rebuild(&probe, 0, DIRECTORY_PROJECTION_REBUILD_MAX_EVENTS + 1).is_err());
    println!("[DEBUG] Neutral checkpoint projection: genuine indexed zero-history root=1 edited children=2 indexless root refusal=1 rootless child refusal=1 late root refusal=1 atomic retention rollback=1 fixed lineage/rebuild caps=2");
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
    frontier_max.checkpoint_id = ArtifactHash(Sha256::digest(&crate::artifact_authority::checkpoint_id_encoding_v1(&checkpoint_identity_input(&frontier_max)).expect("max identity")));
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
    let mut root = std::path::PathBuf::from(std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").expect("ticket generated artifact root"));
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
    println!("[DEBUG] SQLite filesystem checkpoint restart: dedicated genesis=1 edited child=1 same-root reopen=1 exact authority/pair/ledger after rebuild=1");
}

#[tokio::test]
async fn document_descriptor_is_immutable_space_scoped_and_survives_restart() {
    let mut root = std::env::temp_dir();
    root.push(format!("semio-document-descriptor-{}", directory::os_identity::time_ordered_id()));
    std::fs::create_dir_all(&root).expect("create test directory");
    let path = root.join("directory.sqlite3");
    let path_text = path.to_str().expect("utf8 test path").to_string();

    let persisted = descriptor("default", "shared-document");
    let mut zero_identity = persisted.clone();
    zero_identity.pack_schema_hash = "0".repeat(64);
    assert!(matches!(validate_document_descriptor(&zero_identity), Err(DirectoryError::Conflict(_))));
    {
        let directory = SqliteDirectory::connect(&path_text).await.expect("connect");
        directory.seed().await.expect("seed");
        let directories = Arc::new(HubDirectories::from(directory));
        let service = DirectoryService::new(directories.clone(), 16);
        let (events, _) = service.execute(user_actor("seed"), DirectoryCommand::AnnounceDocument { descriptor: Box::new(persisted.clone()) }).await.expect("announce");
        assert!(matches!(&events[0].body, DirectoryEventBody::DocumentAnnounced { descriptor } if descriptor == &persisted));

        let mut conflict = persisted.clone();
        conflict.pack_schema_hash = "44".repeat(32);
        assert!(matches!(service.execute(user_actor("seed"), DirectoryCommand::AnnounceDocument { descriptor: Box::new(conflict) }).await, Err(DirectoryError::Conflict(_))));

        let other = descriptor("other-space", "shared-document");
        let actor = user_actor("seed");
        let (created, _) = service.execute(actor.clone(), DirectoryCommand::CreateSpace { name: "Other".into(), space_kind: DirectorySpaceKind::Studio, visibility: DirectorySpaceVisibility::Private }).await.expect("create other space");
        let other_space = created[0].space_id.clone().expect("space id");
        let mut other = other;
        other.space_id = other_space.clone();
        service.execute(actor, DirectoryCommand::AnnounceDocument { descriptor: Box::new(other.clone()) }).await.expect("announce same document id in other space");
        assert_eq!(directories.list_document_descriptors(&other_space).await.expect("other descriptors"), vec![other]);
    }

    let reopened = SqliteDirectory::connect(&path_text).await.expect("reopen");
    assert_eq!(reopened.get_document_descriptor(&DocumentScope::new("default", "shared-document")).await.expect("read after restart"), Some(persisted.clone()));
    let before = reopened.list_document_descriptors("default").await.expect("before rebuild");
    reopened.rebuild_projections().await.expect("rebuild");
    assert_eq!(reopened.list_document_descriptors("default").await.expect("after rebuild"), before);

    drop(reopened);
    for target in [&path, &root.join("directory.sqlite3-wal"), &root.join("directory.sqlite3-shm")] {
        if target.exists() {
            std::fs::remove_file(target).expect("remove exact sqlite test file");
        }
    }
    std::fs::remove_dir(&root).expect("remove empty test directory");
}

// 🔬️ Decider law: an atelier rejects a second, distinct author (re-upserting the sole existing
// author is not exercised here — that is the sqlite backend's own membership round-trip test).
#[tokio::test]
async fn atelier_rejects_a_second_distinct_author() {
    let dir = fresh_dir().await;
    let service = DirectoryService::new(dir, 16);
    let owner = user_actor("u-owner");
    let space_id = create_space(&service, &owner, DirectorySpaceKind::Atelier).await;
    let err = service.execute(owner, DirectoryCommand::UpsertMember { space_id, email: "other@example.com".into(), role: DirectorySpaceRole::Author }).await.unwrap_err();
    assert!(matches!(err, DirectoryError::Conflict(_)));
}

// 🔬️ Decider law: `archive-space` first demotes every current author to spectator, then
// archives — nobody is left an author of a frozen space.
#[tokio::test]
async fn archive_space_demotes_every_author_to_spectator() {
    let dir = fresh_dir().await;
    let service = DirectoryService::new(dir.clone(), 16);
    let owner = user_actor("u-owner");
    let space_id = create_space(&service, &owner, DirectorySpaceKind::Studio).await;
    service.execute(owner.clone(), DirectoryCommand::UpsertMember { space_id: space_id.clone(), email: "second@example.com".into(), role: DirectorySpaceRole::Author }).await.expect("second author");
    service.execute(owner.clone(), DirectoryCommand::ArchiveSpace { space_id: space_id.clone() }).await.expect("archive-space");
    let members = dir.list_members(&space_id).await.expect("list members");
    assert_eq!(members.len(), 2);
    assert!(members.iter().all(|(_, role)| *role == SpaceRole::Spectator));
    assert_eq!(dir.get_space(&space_id).await.expect("get space").expect("space exists").kind, "archive");
}

// 🔬️ Decider law: the owner's membership can never be removed via `remove-member`.
#[tokio::test]
async fn owner_membership_can_never_be_removed() {
    let dir = fresh_dir().await;
    let service = DirectoryService::new(dir, 16);
    let owner = user_actor("u-owner");
    let space_id = create_space(&service, &owner, DirectorySpaceKind::Studio).await;
    let err = service.execute(owner, DirectoryCommand::RemoveMember { space_id, user_id: "u-owner".into() }).await.unwrap_err();
    assert!(matches!(err, DirectoryError::Conflict(_)));
}

// 🔬️ Decider law: any command naming a deleted (or otherwise missing) space is `NotFound`.
#[tokio::test]
async fn command_naming_a_deleted_space_is_not_found() {
    let dir = fresh_dir().await;
    let service = DirectoryService::new(dir, 16);
    let owner = user_actor("u-owner");
    let space_id = create_space(&service, &owner, DirectorySpaceKind::Studio).await;
    service.execute(owner.clone(), DirectoryCommand::DeleteSpace { space_id: space_id.clone() }).await.expect("delete-space");
    let err = service.execute(owner, DirectoryCommand::RenameSpace { space_id, name: "Nope".into() }).await.unwrap_err();
    assert!(matches!(err, DirectoryError::NotFound(_)));
}

// 🔬️ Decider law: `upsert-member` with an email that has no `UserRecord` yet emits
// `user.created` before `member.upserted`, both under one decision.
#[tokio::test]
async fn upsert_member_with_unknown_email_creates_the_user_first() {
    let dir = fresh_dir().await;
    let service = DirectoryService::new(dir, 16);
    let owner = user_actor("u-owner");
    let space_id = create_space(&service, &owner, DirectorySpaceKind::Studio).await;
    let (events, _) = service.execute(owner, DirectoryCommand::UpsertMember { space_id, email: "new@example.com".into(), role: DirectorySpaceRole::Spectator }).await.expect("upsert-member");
    assert_eq!(events.len(), 2);
    assert!(matches!(events[0].body, DirectoryEventBody::UserCreated { .. }));
    assert!(matches!(events[1].body, DirectoryEventBody::MemberUpserted { .. }));
}

// 🔬️ Invite create -> redeem -> revoke round-trip: redemption grants membership and emits
// `invite.redeemed`; a revoked invite still round-trips its `revoked_at`.
#[tokio::test]
async fn invite_create_redeem_revoke_round_trip() {
    let dir = fresh_dir().await;
    let service = DirectoryService::new(dir.clone(), 16);
    let owner = user_actor("u-owner");
    let space_id = create_space(&service, &owner, DirectorySpaceKind::Studio).await;

    let (_, result) = service.execute(owner.clone(), DirectoryCommand::CreateInvite { space_id: space_id.clone(), role: DirectorySpaceRole::Spectator, ttl_secs: 3600 }).await.expect("create-invite");
    let token = result.expect("command result").invite_token.expect("invite token");
    let capability = InviteCapability::parse(&token).expect("typed invite capability");
    let invited = dir.create_user("invited@example.com", "Invited", None, None, None).await.expect("create invited user");

    let redeemed = service.redeem_invite(user_actor(&invited.id), &capability, &invited.id).await.expect("redeem");
    assert!(matches!(redeemed.event().body, DirectoryEventBody::InviteRedeemed { .. }));
    let members = dir.list_members(&space_id).await.expect("list members");
    assert!(members.iter().any(|(user, role)| user.email == "invited@example.com" && *role == SpaceRole::Spectator));

    let invites = dir.list_invites(&space_id).await.expect("list invites");
    assert_eq!(invites.len(), 1);
    assert_eq!(invites[0].accepted_at, Some(redeemed.event().recorded_at_ms));
    assert_eq!(invites[0].accepted_event_id.as_deref(), Some(redeemed.event().id.as_str()));
    let retried = service.redeem_invite(user_actor(&invited.id), &capability, &invited.id).await.expect("idempotent same-user retry");
    assert!(matches!(redeemed, InviteRedemptionCommit::NewlyCommitted { .. }));
    assert!(matches!(retried, InviteRedemptionCommit::AlreadyCommitted { .. }));
    assert_eq!(retried.event(), redeemed.event());
    assert!(matches!(dir.revoke_invite(&space_id, &invites[0].id, "test-revoke", "invite-round-trip").await, Err(DirectoryError::Conflict(message)) if message == "invite already accepted"));
}

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
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎟️invite-redemption-transaction-v1/🔣️.json")).unwrap();
    for row in fixture["backendOrders"].as_array().unwrap() {
        let root = std::path::PathBuf::from(std::env::var("SEMIO_TEST_ARTIFACT_DIR").expect("ticket artifacts")).join(format!("archive-writers-{}", time_ordered_id()));
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
            command_sha256: directory::os_directory::directory_command_sha256(&command),
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
        eprintln!("[DEBUG] independent directory archive case={} accepted={} role=spectator rebuilt=1 connections=2", row["name"], result.is_ok());
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
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎟️invite-redemption-transaction-v1/🔣️.json")).unwrap();
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
        command_sha256: directory::os_directory::directory_command_sha256(&command),
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
    eprintln!("[DEBUG] directory uncertain commit durable=1 claim=pending repeated=0 publication=0");
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
