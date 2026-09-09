
use super::adapters::{AUTHORITY_MAX_DIAGNOSTIC_BYTES, LivePluginPackageBinding, PluginHostTrustedArtifactCatalog, bounded_message};
use super::*;
use ::directory::os_directory::{DocumentFrontier, DocumentOwner};
use ::directory::os_store::{ArtifactCodec, ArtifactPackFiles, ArtifactTextFiles, VcsError, register_document_codec};
use semio_framework_plugin_host::{PackageHash, PackageId, PackageRef, PluginGraph};
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

struct FakeCodec {
    identity: TrustedArtifactIdentity,
    fail_output: AtomicBool,
    validations: AtomicUsize,
    applications: AtomicUsize,
}

impl TrustedArtifactCodec for FakeCodec {
    fn identity(&self) -> &TrustedArtifactIdentity {
        &self.identity
    }

    async fn validate_pair(&self, pair: &ArtifactPair, stage: ArtifactValidationStage, context: &OperationContext<'_>) -> Result<(), AuthorityError> {
        context.checkpoint()?;
        self.validations.fetch_add(1, Ordering::SeqCst);
        if stage == ArtifactValidationStage::Output && self.fail_output.load(Ordering::SeqCst) {
            return Err(AuthorityError::Codec { stage, message: "forced invalid output".to_string() });
        }
        let text = std::str::from_utf8(&pair.pack).map_err(|error| AuthorityError::Codec { stage, message: error.to_string() })?;
        text.parse::<i64>().map_err(|error| AuthorityError::Codec { stage, message: error.to_string() })?;
        if !pair.spr.starts_with(b"seed") {
            return Err(AuthorityError::Codec { stage, message: "SPR lacks seed".to_string() });
        }
        Ok(())
    }

    async fn apply_operation(&self, mut pair: ArtifactPair, operation: &AcceptedArtifactOperation, context: &OperationContext<'_>) -> Result<ArtifactPair, AuthorityError> {
        context.checkpoint()?;
        self.applications.fetch_add(1, Ordering::SeqCst);
        let current = std::str::from_utf8(&pair.pack)
            .map_err(|error| AuthorityError::Codec { stage: ArtifactValidationStage::Input, message: error.to_string() })?
            .parse::<i64>()
            .map_err(|error| AuthorityError::Codec { stage: ArtifactValidationStage::Input, message: error.to_string() })?;
        let delta = std::str::from_utf8(&operation.encoded)
            .map_err(|error| AuthorityError::Codec { stage: ArtifactValidationStage::Input, message: error.to_string() })?
            .parse::<i64>()
            .map_err(|error| AuthorityError::Codec { stage: ArtifactValidationStage::Input, message: error.to_string() })?;
        pair.pack = current.checked_add(delta).ok_or_else(|| AuthorityError::Codec { stage: ArtifactValidationStage::Output, message: "integer overflow".to_string() })?.to_string().into_bytes();
        let length = u32::try_from(operation.encoded.len()).map_err(|_| AuthorityError::ResourceLimit("operation byte"))?;
        pair.spr.extend_from_slice(&length.to_be_bytes());
        pair.spr.extend_from_slice(&operation.encoded);
        Ok(pair)
    }
}

struct FakeCatalog {
    codec: FakeCodec,
    fail: AtomicBool,
    resolutions: AtomicUsize,
}

impl TrustedArtifactCatalog for FakeCatalog {
    type Codec = FakeCodec;

    async fn resolve<'a>(&'a self, _required: &TrustedArtifactIdentity) -> Result<&'a Self::Codec, AuthorityError> {
        self.resolutions.fetch_add(1, Ordering::SeqCst);
        if self.fail.load(Ordering::SeqCst) {
            return Err(AuthorityError::Catalog("forced missing package".to_string()));
        }
        Ok(&self.codec)
    }
}

struct FakeControl {
    now_ms: AtomicU64,
    cancelled: Arc<AtomicBool>,
    cancel_after_operation: bool,
    progress: Mutex<Vec<AuthorityProgress>>,
}

impl AuthorityOperationControl for FakeControl {
    fn now_ms(&self) -> u64 {
        self.now_ms.load(Ordering::SeqCst)
    }

    fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::SeqCst)
    }

    fn report(&self, progress: AuthorityProgress) {
        if self.cancel_after_operation && progress.stage == AuthorityProgressStage::ApplyingOperations {
            self.cancelled.store(true, Ordering::SeqCst);
        }
        self.progress.lock().expect("progress lock").push(progress);
    }
}

fn descriptor() -> DocumentDescriptor {
    DocumentDescriptor {
        space_id: "raum:ä".to_string(),
        document_id: "plan:東京".to_string(),
        artifact_kind: "s.gis:gismap".to_string(),
        artifact_schema: "s.gis.gismap@1/*:精密".to_string(),
        owner: DocumentOwner { plugin_id: "s.gis:地図".to_string(), package_id: "s.gis.gismap:codec".to_string(), version: "1.0.0:β".to_string(), package_hash: "22".repeat(32) },
        pack_schema_hash: "11".repeat(32),
        bootstrap_version: 1,
        bootstrap_frontier: DocumentFrontier { head_seq: 7, commit_seq: 6, epoch: 2 },
        bootstrap_snapshot_hash: "33".repeat(32),
    }
}

fn frontier(sequence: u64, byte: u8) -> ArtifactFrontier {
    ArtifactFrontier { document_id: "plan:東京".to_string(), head_edit_ordinal: sequence, head_edit_id: format!("edit:{sequence}"), last_commit_seq: sequence, chain_hash: ArtifactHash([byte; 32]) }
}

fn request() -> CheckpointRequest {
    CheckpointRequest {
        descriptor: descriptor(),
        scope: DocumentScope::new("raum:ä", "plan:東京"),
        parent_checkpoint_id: None,
        base_frontier: ArtifactFrontier { document_id: "plan:東京".to_string(), head_edit_ordinal: 0, head_edit_id: "genesis".to_string(), last_commit_seq: 0, chain_hash: ArtifactHash([0; 32]) },
        input_pair: ArtifactPair { pack: b"10".to_vec(), spr: b"seed".to_vec() },
        operations: vec![AcceptedArtifactOperation { sequence: 1, encoded: b"5".to_vec(), resulting_frontier: frontier(1, 1) }, AcceptedArtifactOperation { sequence: 2, encoded: b"-2".to_vec(), resulting_frontier: frontier(2, 2) }],
    }
}

fn catalog(identity: TrustedArtifactIdentity) -> FakeCatalog {
    FakeCatalog { codec: FakeCodec { identity, fail_output: AtomicBool::new(false), validations: AtomicUsize::new(0), applications: AtomicUsize::new(0) }, fail: AtomicBool::new(false), resolutions: AtomicUsize::new(0) }
}

fn control() -> FakeControl {
    FakeControl { now_ms: AtomicU64::new(100), cancelled: Arc::new(AtomicBool::new(false)), cancel_after_operation: false, progress: Mutex::new(Vec::new()) }
}

fn context<'a>(control: &'a FakeControl, limits: AuthorityLimits) -> OperationContext<'a> {
    OperationContext::new(200, limits, control)
}

fn limits() -> AuthorityLimits {
    AuthorityLimits { max_operations: 4, max_operation_bytes: 16, max_pair_bytes: 64 }
}

fn fixture_compile<'a>(_dsl: &'a str, _ops: &'a str) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(ArtifactPackFiles, String), VcsError>> + Send + 'a>> {
    Box::pin(async { Err(VcsError::Deserialize("fixture compile is outside the authority boundary".to_string())) })
}

fn fixture_print<'a>(pack: &'a [u8], spr: &'a [u8]) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<ArtifactTextFiles, VcsError>> + Send + 'a>> {
    Box::pin(async move {
        std::str::from_utf8(pack).map_err(|error| VcsError::Deserialize(error.to_string()))?.parse::<i64>().map_err(|error| VcsError::Deserialize(error.to_string()))?;
        if !spr.starts_with(b"seed") {
            return Err(VcsError::Deserialize("SPR lacks seed".to_string()));
        }
        Ok(ArtifactTextFiles { dsl: String::from_utf8(pack.to_vec()).map_err(|error| VcsError::Deserialize(error.to_string()))?, ops: hex_lower(spr) })
    })
}

fn fixture_edit<'a>(_envelope: &'a directory::os_spr::MutationEnvelope) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String, VcsError>> + 'a>> {
    Box::pin(async { Ok(String::new()) })
}

fn fixture_apply<'a>(pack: &'a [u8], spr: &'a [u8], operations: &'a [u8]) -> directory::os_store::ArtifactCodecApplyFuture<'a> {
    Box::pin(async move {
        let operations = directory::os_spr::decode_ops_vec(operations).map_err(|error| VcsError::Deserialize(error.to_string()))?;
        if operations.len() != 1 {
            return Err(VcsError::Deserialize("fixture codec requires exactly one operation".to_string()));
        }
        let current = std::str::from_utf8(pack).map_err(|error| VcsError::Deserialize(error.to_string()))?.parse::<i64>().map_err(|error| VcsError::Deserialize(error.to_string()))?;
        let delta = std::str::from_utf8(&operations[0]).map_err(|error| VcsError::Deserialize(error.to_string()))?.parse::<i64>().map_err(|error| VcsError::Deserialize(error.to_string()))?;
        let mut next_spr = spr.to_vec();
        next_spr.extend_from_slice(&(operations[0].len() as u32).to_be_bytes());
        next_spr.extend_from_slice(&operations[0]);
        Ok((current.checked_add(delta).ok_or_else(|| VcsError::Deserialize("integer overflow".to_string()))?.to_string().into_bytes(), next_spr, String::new()))
    })
}

fn fixture_artifact_codec() -> ArtifactCodec {
    ArtifactCodec { schema: "fixture.number@1".to_string(), extension: "fixture", pack_schema_hash: [0x11; 32], compile_dsl: fixture_compile, print_mirror: fixture_print, edit_text_from_envelope: fixture_edit, apply_ops_binary: fixture_apply }
}

fn fixture_manifest() -> semio_framework::PluginManifest {
    semio_framework::PluginManifest {
        plugin_id: "fixture.authority".to_string(),
        label: "Fixture Authority".to_string(),
        version: "1.2.3".to_string(),
        apps: Vec::new(),
        examples: Vec::new(),
        capabilities: Vec::new(),
        topic_contributions: Vec::new(),
        commands: Vec::new(),
        artifact_kinds: vec![semio_framework::ArtifactKindSpec {
            id: "fixture.number".to_string(),
            name: "Fixture Number".to_string(),
            source_format: "fixture".to_string(),
            component_kind: "document".to_string(),
            dimension: "data".to_string(),
            media_capability: semio_framework::OsMediaCapability::MeshOnly,
            media_type: semio_framework::MediaType { class: semio_framework::MediaClass::Data, form: semio_framework::MediaForm::Value },
            schema: "fixture.number@1".to_string(),
            export_formats: Vec::new(),
            import_formats: Vec::new(),
            export_stdio_kinds: Vec::new(),
            import_stdio_kinds: Vec::new(),
        }],
        dependencies: Vec::new(),
        contributions: Vec::new(),
    }
}

struct FakeImmutableStore {
    fail_on: usize,
    corrupt_on: usize,
    operations: AtomicUsize,
    bytes: Mutex<Vec<Vec<u8>>>,
}

impl ImmutableArtifactBlobStore for FakeImmutableStore {
    async fn stage(&self, space_id: &str, expected: ArtifactBlobIntegrity, bytes: &[u8], context: &OperationContext<'_>) -> Result<StagedArtifactBlob, AuthorityError> {
        context.checkpoint()?;
        let operation = self.operations.fetch_add(1, Ordering::SeqCst) + 1;
        if operation == self.fail_on {
            return Err(AuthorityError::Store("forced stage failure".to_string()));
        }
        let mut stored = self.bytes.lock().expect("store lock");
        stored.push(bytes.to_vec());
        let manifest = chunk_cas::prepare_artifact_cas_manifest_v1(space_id, bytes)?;
        Ok(StagedArtifactBlob { storage_key: chunk_cas::artifact_cas_manifest_locator_v1(manifest.manifest_id), integrity: expected })
    }

    async fn read(&self, _space_id: &str, staged: &StagedArtifactBlob, context: &OperationContext<'_>) -> Result<Vec<u8>, AuthorityError> {
        context.checkpoint()?;
        let operation = self.operations.fetch_add(1, Ordering::SeqCst) + 1;
        if operation == self.fail_on {
            return Err(AuthorityError::Store("forced read failure".to_string()));
        }
        let _ = chunk_cas::decode_artifact_cas_manifest_locator_v1(&staged.storage_key)?;
        let mut bytes = self.bytes.lock().expect("store lock").iter().find(|bytes| exact_blob_integrity(bytes).ok() == Some(staged.integrity)).cloned().ok_or_else(|| AuthorityError::Store("missing fake blob".to_string()))?;
        if operation == self.corrupt_on {
            bytes.push(0xff);
        }
        Ok(bytes)
    }
}

struct FakePublisher {
    fail: bool,
    cancel_after_commit: Option<Arc<AtomicBool>>,
    attempts: AtomicUsize,
    committed: Mutex<Vec<ArtifactCheckpoint>>,
}

impl VerifiedCheckpointPublisher for FakePublisher {
    async fn reserve(&self, plan: &chunk_cas::ArtifactCasOwnershipPlanV1, context: &OperationContext<'_>) -> Result<chunk_cas::ArtifactCasReservation, AuthorityError> {
        context.checkpoint()?;
        Ok(chunk_cas::ArtifactCasReservation::unfenced(plan.clone(), 1, 1, context.deadline_ms()))
    }

    async fn publish_reserved(&self, checkpoint: &ArtifactCheckpoint, reservation: &chunk_cas::ArtifactCasReservation, context: &OperationContext<'_>) -> Result<(), AuthorityError> {
        context.checkpoint()?;
        chunk_cas::validate_artifact_cas_publication_v1(&reservation.plan, checkpoint)?;
        self.attempts.fetch_add(1, Ordering::SeqCst);
        if self.fail {
            return Err(AuthorityError::Publication("forced publication failure".to_string()));
        }
        self.committed.lock().expect("publisher lock").push(checkpoint.clone());
        if let Some(cancelled) = &self.cancel_after_commit {
            cancelled.store(true, Ordering::SeqCst);
        }
        Ok(())
    }
}

async fn candidate(control: &FakeControl) -> CheckpointCandidate {
    let required = TrustedArtifactIdentity::from_descriptor(&descriptor());
    ValidatingCanonicalArtifactAuthority::new(catalog(required)).materialize_checkpoint(request(), &context(control, limits())).await.expect("candidate")
}

#[tokio::test]
async fn canonical_authority_contract_matches_the_language_neutral_checkpoint_vector() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🏛️canonical-authority/🔣️.json")).expect("valid authority fixture");
    let required = TrustedArtifactIdentity::from_descriptor(&descriptor());
    let authority = ValidatingCanonicalArtifactAuthority::new(catalog(required));
    let control = control();
    let candidate = authority.materialize_checkpoint(request(), &context(&control, limits())).await.expect("candidate");
    let bytes = |name: &str| fixture[name].as_array().expect("byte vector").iter().map(|value| value.as_u64().expect("byte") as u8).collect::<Vec<_>>();

    assert_eq!(candidate.pair.pack, bytes("outputPack"));
    assert_eq!(candidate.pair.spr, bytes("outputSpr"));
    assert_eq!(candidate.checkpoint.descriptor_digest_v1.0.as_slice(), bytes("descriptorDigestV1"));
    assert_eq!(candidate.checkpoint.pack.sha256.0.as_slice(), bytes("packSha256"));
    assert_eq!(candidate.checkpoint.spr.sha256.0.as_slice(), bytes("sprSha256"));
    assert_eq!(candidate.checkpoint.aggregate_sha256.0.as_slice(), bytes("aggregateSha256"));
    assert_eq!(hex_lower(&checkpoint_id_encoding_v1(&candidate.checkpoint).expect("checkpoint encoding")), fixture["checkpointIdEncodingHexV1"].as_str().expect("encoding hex"));
    assert_eq!(candidate.checkpoint.checkpoint_id.0.as_slice(), bytes("checkpointIdV1"));
    assert_eq!(candidate.checkpoint.baseline_frontier, frontier(2, 2));
    assert_eq!(candidate.checkpoint.published_at_ms, 100);
    assert_eq!(authority.catalog.codec.validations.load(Ordering::SeqCst), 2);
    assert_eq!(authority.catalog.codec.applications.load(Ordering::SeqCst), 2);
    let progress = control.progress.lock().expect("progress lock");
    assert!(progress.windows(2).all(|pair| pair[0].completed_units <= pair[1].completed_units));
    assert_eq!(progress.last().map(|item| (item.stage, item.completed_units, item.total_units)), Some((AuthorityProgressStage::Derived, 7, 7)));
}

#[tokio::test]
async fn canonical_authority_contract_rejects_catalog_and_exact_identity_mismatch_before_codec_execution() {
    let descriptor = descriptor();
    let mut wrong = TrustedArtifactIdentity::from_descriptor(&descriptor);
    wrong.package_hash = "44".repeat(32);
    let authority = ValidatingCanonicalArtifactAuthority::new(catalog(wrong));
    let control = control();
    assert_eq!(authority.materialize_checkpoint(request(), &context(&control, limits())).await, Err(AuthorityError::CodecIdentityMismatch));
    assert_eq!(authority.catalog.codec.validations.load(Ordering::SeqCst), 0);
    assert_eq!(authority.catalog.codec.applications.load(Ordering::SeqCst), 0);

    let authority = ValidatingCanonicalArtifactAuthority::new(catalog(TrustedArtifactIdentity::from_descriptor(&descriptor)));
    authority.catalog.fail.store(true, Ordering::SeqCst);
    assert!(matches!(authority.materialize_checkpoint(request(), &context(&control, limits())).await, Err(AuthorityError::Catalog(_))));
    assert_eq!(authority.catalog.codec.validations.load(Ordering::SeqCst), 0);
}

#[tokio::test]
async fn canonical_authority_contract_enforces_order_frontier_budgets_deadline_and_cancellation_without_candidate() {
    let required = TrustedArtifactIdentity::from_descriptor(&descriptor());
    let authority = ValidatingCanonicalArtifactAuthority::new(catalog(required));
    let control = control();
    let mut unordered = request();
    unordered.operations[1].sequence = 3;
    assert_eq!(authority.materialize_checkpoint(unordered, &context(&control, limits())).await, Err(AuthorityError::InvalidOperationOrder));

    let mut foreign = request();
    foreign.operations[1].resulting_frontier.document_id = "foreign".to_string();
    assert_eq!(authority.materialize_checkpoint(foreign, &context(&control, limits())).await, Err(AuthorityError::InvalidOperationOrder));

    let tiny = AuthorityLimits { max_operations: 1, max_operation_bytes: 16, max_pair_bytes: 64 };
    assert_eq!(authority.materialize_checkpoint(request(), &context(&control, tiny)).await, Err(AuthorityError::ResourceLimit("operation count")));

    let tiny = AuthorityLimits { max_operations: 4, max_operation_bytes: 2, max_pair_bytes: 64 };
    assert_eq!(authority.materialize_checkpoint(request(), &context(&control, tiny)).await, Err(AuthorityError::ResourceLimit("operation byte")));

    let tiny = AuthorityLimits { max_operations: 4, max_operation_bytes: 16, max_pair_bytes: 8 };
    assert_eq!(authority.materialize_checkpoint(request(), &context(&control, tiny)).await, Err(AuthorityError::PairResourceLimit(ArtifactValidationStage::Output)));

    let tiny = AuthorityLimits { max_operations: 4, max_operation_bytes: 16, max_pair_bytes: 5 };
    assert_eq!(authority.materialize_checkpoint(request(), &context(&control, tiny)).await, Err(AuthorityError::PairResourceLimit(ArtifactValidationStage::Input)));

    let invalid = AuthorityLimits { max_operations: 0, max_operation_bytes: 16, max_pair_bytes: 64 };
    assert_eq!(authority.materialize_checkpoint(request(), &context(&control, invalid)).await, Err(AuthorityError::InvalidLimits));

    for invalid in [
        AuthorityLimits { max_operations: AUTHORITY_MAX_OPERATIONS + 1, ..AuthorityLimits::maximum() },
        AuthorityLimits { max_operation_bytes: AUTHORITY_MAX_OPERATION_BYTES + 1, ..AuthorityLimits::maximum() },
        AuthorityLimits { max_pair_bytes: AUTHORITY_MAX_PAIR_BYTES + 1, ..AuthorityLimits::maximum() },
    ] {
        assert_eq!(authority.materialize_checkpoint(request(), &context(&control, invalid)).await, Err(AuthorityError::InvalidLimits));
    }

    control.now_ms.store(200, Ordering::SeqCst);
    assert_eq!(authority.materialize_checkpoint(request(), &context(&control, limits())).await, Err(AuthorityError::DeadlineExceeded));

    let cancelled = FakeControl { now_ms: AtomicU64::new(100), cancelled: Arc::new(AtomicBool::new(false)), cancel_after_operation: true, progress: Mutex::new(Vec::new()) };
    assert_eq!(authority.materialize_checkpoint(request(), &context(&cancelled, limits())).await, Err(AuthorityError::Cancelled));
    assert!(cancelled.progress.lock().expect("progress lock").iter().all(|item| item.stage != AuthorityProgressStage::Derived));
}

#[tokio::test]
async fn canonical_authority_contract_validates_input_and_output_and_failure_returns_no_checkpoint_candidate() {
    let required = TrustedArtifactIdentity::from_descriptor(&descriptor());
    let authority = ValidatingCanonicalArtifactAuthority::new(catalog(required));
    let control = control();
    let mut invalid_input = request();
    invalid_input.input_pair.pack = b"not-a-number".to_vec();
    assert!(matches!(authority.materialize_checkpoint(invalid_input, &context(&control, limits())).await, Err(AuthorityError::Codec { stage: ArtifactValidationStage::Input, .. })));

    authority.catalog.codec.fail_output.store(true, Ordering::SeqCst);
    let result = authority.materialize_checkpoint(request(), &context(&control, limits())).await;
    assert!(matches!(result, Err(AuthorityError::Codec { stage: ArtifactValidationStage::Output, .. })));
    assert!(control.progress.lock().expect("progress lock").iter().all(|item| item.stage != AuthorityProgressStage::Derived));
}

#[tokio::test]
async fn plugin_host_catalog_resolves_only_the_exact_live_package_manifest_kind_schema_and_codec_hash() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔌️authority-adapter/🔣️.json")).expect("adapter fixture");
    register_document_codec(fixture_artifact_codec()).expect("register fixture codec");
    let graph = PluginGraph::new();
    graph.register(fixture_manifest()).await.expect("register fixture manifest");
    let package = PackageRef { package: PackageId("fixture.authority.package".to_string()), hash: PackageHash([0x22; 32]) };
    let binding = LivePluginPackageBinding::from_host("fixture.authority", &package);
    let control = control();
    let catalog = PluginHostTrustedArtifactCatalog::load(&graph, &[binding], &context(&control, limits())).await.expect("trusted catalog");
    let identity = &fixture["identity"];
    let required = TrustedArtifactIdentity {
        plugin_id: identity["pluginId"].as_str().expect("plugin id").to_string(),
        package_id: identity["packageId"].as_str().expect("package id").to_string(),
        version: identity["version"].as_str().expect("version").to_string(),
        package_hash: identity["packageHash"].as_str().expect("package hash").to_string(),
        artifact_kind: identity["artifactKind"].as_str().expect("kind").to_string(),
        artifact_schema: identity["artifactSchema"].as_str().expect("schema").to_string(),
        pack_schema_hash: identity["packSchemaHash"].as_str().expect("pack schema hash").to_string(),
    };
    let codec = catalog.resolve(&required).await.expect("exact codec");
    assert_eq!(codec.identity(), &required);
    codec.validate_pair(&ArtifactPair { pack: b"10".to_vec(), spr: b"seed".to_vec() }, ArtifactValidationStage::Input, &context(&control, limits())).await.expect("real registered codec validation");
    let pair = codec
        .apply_operation(ArtifactPair { pack: b"10".to_vec(), spr: b"seed".to_vec() }, &AcceptedArtifactOperation { sequence: 1, encoded: b"5".to_vec(), resulting_frontier: frontier(1, 1) }, &context(&control, limits()))
        .await
        .expect("real registered codec application");
    assert_eq!(pair, ArtifactPair { pack: b"15".to_vec(), spr: vec![b's', b'e', b'e', b'd', 0, 0, 0, 1, b'5'] });

    for field in ["plugin_id", "package_id", "package_hash", "version", "artifact_kind", "artifact_schema", "pack_schema_hash"] {
        let mut mismatch = required.clone();
        match field {
            "plugin_id" => mismatch.plugin_id = "fixture.other-plugin".to_string(),
            "package_id" => mismatch.package_id = "fixture.other-package".to_string(),
            "package_hash" => mismatch.package_hash = "33".repeat(32),
            "version" => mismatch.version = "1.2.4".to_string(),
            "artifact_kind" => mismatch.artifact_kind = "fixture.other".to_string(),
            "artifact_schema" => mismatch.artifact_schema = "fixture.other@1".to_string(),
            _ => mismatch.pack_schema_hash = "44".repeat(32),
        }
        assert!(matches!(catalog.resolve(&mismatch).await, Err(AuthorityError::Catalog(_))), "{field} mismatch must not resolve");
    }
}

#[tokio::test]
async fn publication_orchestrator_never_calls_the_publisher_before_both_exact_blob_readbacks() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔌️authority-adapter/🔣️.json")).expect("adapter fixture");
    assert_eq!(fixture["failureStages"].as_array().expect("failure stages").len(), 5);
    for fail_on in 1..=4 {
        let control = control();
        let store = FakeImmutableStore { fail_on, corrupt_on: 0, operations: AtomicUsize::new(0), bytes: Mutex::new(Vec::new()) };
        let publisher = FakePublisher { fail: false, cancel_after_commit: None, attempts: AtomicUsize::new(0), committed: Mutex::new(Vec::new()) };
        let orchestrator = CheckpointPublicationOrchestrator::new(store, publisher);
        assert!(orchestrator.publish_candidate(candidate(&control).await, &context(&control, limits())).await.is_err());
        assert_eq!(orchestrator.publisher.attempts.load(Ordering::SeqCst), 0);
        assert!(orchestrator.publisher.committed.lock().expect("publisher lock").is_empty());
    }

    for corrupt_on in 3..=4 {
        let control = control();
        let store = FakeImmutableStore { fail_on: 0, corrupt_on, operations: AtomicUsize::new(0), bytes: Mutex::new(Vec::new()) };
        let publisher = FakePublisher { fail: false, cancel_after_commit: None, attempts: AtomicUsize::new(0), committed: Mutex::new(Vec::new()) };
        let orchestrator = CheckpointPublicationOrchestrator::new(store, publisher);
        assert!(matches!(orchestrator.publish_candidate(candidate(&control).await, &context(&control, limits())).await, Err(AuthorityError::BlobIntegrity(_))));
        assert_eq!(orchestrator.publisher.attempts.load(Ordering::SeqCst), 0);
        assert!(orchestrator.publisher.committed.lock().expect("publisher lock").is_empty());
    }

    let control = control();
    let store = FakeImmutableStore { fail_on: 0, corrupt_on: 0, operations: AtomicUsize::new(0), bytes: Mutex::new(Vec::new()) };
    let publisher = FakePublisher { fail: true, cancel_after_commit: None, attempts: AtomicUsize::new(0), committed: Mutex::new(Vec::new()) };
    let orchestrator = CheckpointPublicationOrchestrator::new(store, publisher);
    assert!(matches!(orchestrator.publish_candidate(candidate(&control).await, &context(&control, limits())).await, Err(AuthorityError::Publication(_))));
    assert_eq!(orchestrator.publisher.attempts.load(Ordering::SeqCst), 1);
    assert!(orchestrator.publisher.committed.lock().expect("publisher lock").is_empty());
}

#[tokio::test]
async fn publication_orchestrator_rewrites_only_private_locators_after_success_and_rejects_tampering_before_staging() {
    let publication_control = control();
    let store = FakeImmutableStore { fail_on: 0, corrupt_on: 0, operations: AtomicUsize::new(0), bytes: Mutex::new(Vec::new()) };
    let publisher = FakePublisher { fail: false, cancel_after_commit: Some(publication_control.cancelled.clone()), attempts: AtomicUsize::new(0), committed: Mutex::new(Vec::new()) };
    let orchestrator = CheckpointPublicationOrchestrator::new(store, publisher);
    let original = candidate(&publication_control).await;
    let original_id = original.checkpoint.checkpoint_id;
    let publication = orchestrator.publish_candidate(original, &context(&publication_control, limits())).await.expect("publication");
    assert!(publication_control.cancelled.load(Ordering::SeqCst));
    assert_eq!(publication.checkpoint.checkpoint_id, original_id);
    assert!(publication.checkpoint.pack.storage_key.starts_with(chunk_cas::ARTIFACT_CAS_MANIFEST_LOCATOR_PREFIX));
    assert!(publication.checkpoint.spr.storage_key.starts_with(chunk_cas::ARTIFACT_CAS_MANIFEST_LOCATOR_PREFIX));
    assert_eq!(orchestrator.publisher.committed.lock().expect("publisher lock").as_slice(), &[publication.checkpoint]);

    let tamper_control = control();
    let mut tampered = candidate(&tamper_control).await;
    tampered.pair.pack.push(0xff);
    let store = FakeImmutableStore { fail_on: 0, corrupt_on: 0, operations: AtomicUsize::new(0), bytes: Mutex::new(Vec::new()) };
    let publisher = FakePublisher { fail: false, cancel_after_commit: None, attempts: AtomicUsize::new(0), committed: Mutex::new(Vec::new()) };
    let orchestrator = CheckpointPublicationOrchestrator::new(store, publisher);
    assert_eq!(orchestrator.publish_candidate(tampered, &context(&tamper_control, limits())).await, Err(AuthorityError::BlobIntegrity("candidate")));
    assert_eq!(orchestrator.store.operations.load(Ordering::SeqCst), 0);
    assert_eq!(orchestrator.publisher.attempts.load(Ordering::SeqCst), 0);
}

#[test]
fn authority_diagnostics_are_utf8_safe_and_fixed_bounded_before_retention() {
    let message = bounded_message("é".repeat(AUTHORITY_MAX_DIAGNOSTIC_BYTES));
    assert!(message.len() <= AUTHORITY_MAX_DIAGNOSTIC_BYTES);
    assert!(message.is_char_boundary(message.len()));
}
