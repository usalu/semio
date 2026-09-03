//! 🛡️ Hub-owned canonical artifact authority and failure-atomic immutable publication ports. Package-host
//! catalog resolution and database blob staging live in [`adapters`]; directory events, retention
//! advancement, and WebSocket production remain the P2-B/P2-C seams.

use ::directory::os_directory::{descriptor_digest_v1, hex_lower, ArtifactBlobRef, ArtifactCheckpoint, ArtifactFrontier, ArtifactHash, DocumentDescriptor, DocumentScope};
use semio_framework_hash::Sha256;

#[path = "🗂️chunk-cas/🦀️.rs"]
pub mod chunk_cas;

/// 🔐️ Domain prefix for a canonical checkpoint identity.
pub const CHECKPOINT_ID_V1_DOMAIN: &[u8] = b"semio.hub.artifact-checkpoint.v1\0";

/// 🧯️ Immutable production ceiling for one checkpoint's accepted operation count.
pub const AUTHORITY_MAX_OPERATIONS: usize = 16_384;
/// 🧯️ Immutable production ceiling for one checkpoint's total accepted operation bytes.
pub const AUTHORITY_MAX_OPERATION_BYTES: u64 = 64 * 1024 * 1024;
/// 🧯️ Immutable production ceiling for a canonical pack plus SPR pair.
pub const AUTHORITY_MAX_PAIR_BYTES: u64 = 64 * 1024 * 1024;

/// 🪢️ The canonical artifact pair whose ownership transfers only inside a successful candidate.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArtifactPair {
    pub pack: Vec<u8>,
    pub spr: Vec<u8>,
}

/// 🪪️ Exact descriptor-pinned codec identity a trusted catalog must resolve.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrustedArtifactIdentity {
    pub plugin_id: String,
    pub package_id: String,
    pub version: String,
    pub package_hash: String,
    pub artifact_kind: String,
    pub artifact_schema: String,
    pub pack_schema_hash: String,
}

impl TrustedArtifactIdentity {
    /// 🧬️ Copies the complete immutable codec identity from a validated descriptor.
    pub fn from_descriptor(descriptor: &DocumentDescriptor) -> Self {
        Self {
            plugin_id: descriptor.owner.plugin_id.clone(),
            package_id: descriptor.owner.package_id.clone(),
            version: descriptor.owner.version.clone(),
            package_hash: descriptor.owner.package_hash.clone(),
            artifact_kind: descriptor.artifact_kind.clone(),
            artifact_schema: descriptor.artifact_schema.clone(),
            pack_schema_hash: descriptor.pack_schema_hash.clone(),
        }
    }
}

/// 📜️ One accepted operation and the exact authenticated frontier after applying it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AcceptedArtifactOperation {
    pub sequence: u64,
    pub encoded: Vec<u8>,
    pub resulting_frontier: ArtifactFrontier,
}

/// 📥️ Owned authority input; no client-provided artifact or checkpoint hash is accepted.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CheckpointRequest {
    pub descriptor: DocumentDescriptor,
    pub scope: DocumentScope,
    pub parent_checkpoint_id: Option<ArtifactHash>,
    pub base_frontier: ArtifactFrontier,
    pub input_pair: ArtifactPair,
    pub operations: Vec<AcceptedArtifactOperation>,
}

/// 🧯️ Hard request budgets checked before and after every trusted-codec operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AuthorityLimits {
    pub max_operations: usize,
    pub max_operation_bytes: u64,
    pub max_pair_bytes: u64,
}

impl AuthorityLimits {
    /// 🛡️ Returns the immutable production ceilings; callers may only choose lower positive limits.
    pub const fn maximum() -> Self {
        Self { max_operations: AUTHORITY_MAX_OPERATIONS, max_operation_bytes: AUTHORITY_MAX_OPERATION_BYTES, max_pair_bytes: AUTHORITY_MAX_PAIR_BYTES }
    }
}

/// 🚶️ Observable deterministic stages of one materialization attempt.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuthorityProgressStage {
    Preflight,
    CatalogLoading,
    CatalogResolved,
    InputValidated,
    ApplyingOperations,
    OutputValidated,
    Derived,
    CasChunkStored,
    CasChunkVerified,
    CasManifestStored,
    CasManifestVerified,
    PackStaged,
    SprStaged,
    PackVerified,
    SprVerified,
    CasSweep,
    Published,
}

/// 📈️ Monotonic bounded authority progress.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AuthorityProgress {
    pub stage: AuthorityProgressStage,
    pub completed_units: u64,
    pub total_units: u64,
}

/// 🎛️ Host-owned cancellation, clock, and progress port with no runtime type leakage.
pub trait AuthorityOperationControl: Send + Sync {
    /// 🕰️ Returns the current host wall-clock milliseconds.
    fn now_ms(&self) -> u64;

    /// 🛑️ Reports whether the caller cancelled this operation.
    fn is_cancelled(&self) -> bool;

    /// 📡️ Observes monotonic bounded progress without taking candidate ownership.
    fn report(&self, progress: AuthorityProgress);
}

/// ⏱️ Bounded operation context shared with trusted codecs so long work stays cancellable.
pub struct OperationContext<'a> {
    deadline_ms: u64,
    limits: AuthorityLimits,
    control: &'a dyn AuthorityOperationControl,
}

impl<'a> OperationContext<'a> {
    /// 🏛️ Creates one authority context with an absolute exclusive deadline.
    pub const fn new(deadline_ms: u64, limits: AuthorityLimits, control: &'a dyn AuthorityOperationControl) -> Self {
        Self { deadline_ms, limits, control }
    }

    /// 🧯️ Returns the immutable request budgets.
    pub const fn limits(&self) -> AuthorityLimits {
        self.limits
    }

    /// 🛑️ Enforces cancellation and the exclusive deadline at a safe boundary.
    pub fn checkpoint(&self) -> Result<(), AuthorityError> {
        if self.control.is_cancelled() {
            return Err(AuthorityError::Cancelled);
        }
        if self.control.now_ms() >= self.deadline_ms {
            return Err(AuthorityError::DeadlineExceeded);
        }
        Ok(())
    }

    pub(crate) fn report(&self, progress: AuthorityProgress) -> Result<(), AuthorityError> {
        self.control.report(progress);
        self.checkpoint()
    }

    /// 📡️ Observes a completed durable transition without reinterpreting it as cancellation.
    pub(crate) fn report_committed(&self, progress: AuthorityProgress) {
        self.control.report(progress);
    }

    pub(crate) fn now_ms(&self) -> u64 {
        self.control.now_ms()
    }

    /// ⏱️ Returns the immutable absolute publication deadline.
    pub(crate) const fn deadline_ms(&self) -> u64 {
        self.deadline_ms
    }
}

/// 🔎️ Distinguishes semantic validation before and after operation application.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArtifactValidationStage {
    Input,
    Output,
}

/// 🚨️ Catalog/codec/authority failures never contain a publishable checkpoint candidate.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AuthorityError {
    Cancelled,
    DeadlineExceeded,
    InvalidDescriptor(String),
    InvalidScope,
    InvalidFrontier,
    InvalidParentCheckpoint,
    InvalidOperationOrder,
    InvalidLimits,
    ResourceLimit(&'static str),
    PairResourceLimit(ArtifactValidationStage),
    Catalog(String),
    CodecIdentityMismatch,
    Codec { stage: ArtifactValidationStage, message: String },
    Store(String),
    BlobIntegrity(&'static str),
    Publication(String),
}

impl std::fmt::Display for AuthorityError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cancelled => formatter.write_str("artifact authority operation cancelled"),
            Self::DeadlineExceeded => formatter.write_str("artifact authority deadline exceeded"),
            Self::InvalidDescriptor(message) => write!(formatter, "invalid document descriptor: {message}"),
            Self::InvalidScope => formatter.write_str("artifact authority scope does not match the descriptor"),
            Self::InvalidFrontier => formatter.write_str("artifact authority frontier is invalid"),
            Self::InvalidParentCheckpoint => formatter.write_str("artifact authority parent checkpoint is zero"),
            Self::InvalidOperationOrder => formatter.write_str("accepted operations are not an exact ordered frontier chain"),
            Self::InvalidLimits => formatter.write_str("artifact authority limits must be positive and within immutable production ceilings"),
            Self::ResourceLimit(resource) => write!(formatter, "artifact authority {resource} budget exceeded"),
            Self::PairResourceLimit(stage) => write!(formatter, "artifact authority pair byte budget exceeded at {stage:?} validation"),
            Self::Catalog(message) => write!(formatter, "trusted artifact catalog failed: {message}"),
            Self::CodecIdentityMismatch => formatter.write_str("trusted codec identity does not exactly match the descriptor"),
            Self::Codec { stage, message } => write!(formatter, "trusted artifact codec {stage:?} failed: {message}"),
            Self::Store(message) => write!(formatter, "immutable artifact blob store failed: {message}"),
            Self::BlobIntegrity(blob) => write!(formatter, "staged artifact {blob} failed exact integrity verification"),
            Self::Publication(message) => write!(formatter, "verified checkpoint publication failed: {message}"),
        }
    }
}

impl std::error::Error for AuthorityError {}

/// 🧪️ Trusted codec port; the authority validates both sides and applies operations one-by-one.
pub trait TrustedArtifactCodec: Send + Sync {
    /// 🪪️ Returns the exact catalog identity of this codec.
    fn identity(&self) -> &TrustedArtifactIdentity;

    /// ✅️ Semantically validates one complete pair at the named authority boundary.
    async fn validate_pair(&self, pair: &ArtifactPair, stage: ArtifactValidationStage, context: &OperationContext<'_>) -> Result<(), AuthorityError>;

    /// ➡️ Applies exactly one already-accepted operation, transferring pair ownership on success.
    async fn apply_operation(&self, pair: ArtifactPair, operation: &AcceptedArtifactOperation, context: &OperationContext<'_>) -> Result<ArtifactPair, AuthorityError>;
}

/// 🗂️ Trusted package-hash catalog port implemented by the production plugin-host adapter.
pub trait TrustedArtifactCatalog: Send + Sync {
    type Codec: TrustedArtifactCodec;

    /// 🔍️ Resolves only the complete descriptor-pinned identity.
    async fn resolve<'a>(&'a self, required: &TrustedArtifactIdentity) -> Result<&'a Self::Codec, AuthorityError>;
}

/// 📦️ Success-only transfer containing server-derived metadata and canonical pair bytes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CheckpointCandidate {
    pub checkpoint: ArtifactCheckpoint,
    pub pair: ArtifactPair,
}

/// 🪪️ Server-derived public integrity expected from one immutable blob stage.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArtifactBlobIntegrity {
    pub sha256: ArtifactHash,
    pub byte_length: u64,
}

/// 🛢️ Opaque backend-private locator returned only after immutable storage accepts a blob.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StagedArtifactBlob {
    pub storage_key: String,
    pub integrity: ArtifactBlobIntegrity,
}

/// 🫙️ Project-owned immutable blob port; storage implementation types never cross it.
pub trait ImmutableArtifactBlobStore: Send + Sync {
    /// 📥️ Stores server-owned bytes under immutable content identity.
    async fn stage(&self, space_id: &str, expected: ArtifactBlobIntegrity, bytes: &[u8], context: &OperationContext<'_>) -> Result<StagedArtifactBlob, AuthorityError>;

    /// 📖️ Reads the exact bytes behind one opaque staged locator.
    async fn read(&self, space_id: &str, staged: &StagedArtifactBlob, context: &OperationContext<'_>) -> Result<Vec<u8>, AuthorityError>;
}

/// 🎉️ Success-only publication result; canonical pair bytes are no longer exposed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VerifiedCheckpointPublication {
    pub checkpoint: ArtifactCheckpoint,
}

/// 📣️ Project-owned P2-B seam invoked only after both immutable blobs pass exact read-back.
pub trait VerifiedCheckpointPublisher: Send + Sync {
    /// 🎫️ Commits the exact reachability reservation before the first physical CAS write.
    async fn reserve(&self, plan: &chunk_cas::ArtifactCasOwnershipPlanV1, context: &OperationContext<'_>) -> Result<chunk_cas::ArtifactCasReservation, AuthorityError>;

    /// 🛡️ Atomically consumes the reservation with the verified public/private checkpoint commit.
    async fn publish_reserved(&self, checkpoint: &ArtifactCheckpoint, reservation: &chunk_cas::ArtifactCasReservation, context: &OperationContext<'_>) -> Result<(), AuthorityError>;
}

/// 🚚️ Failure-atomic candidate staging and verified-publication orchestrator.
pub struct CheckpointPublicationOrchestrator<S, P> {
    store: S,
    publisher: P,
}

impl<S, P> CheckpointPublicationOrchestrator<S, P> {
    /// 🧩️ Constructs the orchestrator without taking candidate ownership.
    pub const fn new(store: S, publisher: P) -> Self {
        Self { store, publisher }
    }
}

/// 🏗️ Hub production authority contract, independent of plugin-host and durable-store APIs.
pub trait CanonicalArtifactAuthority {
    /// 🛠️ Materializes, validates, hashes, and transfers one candidate only after complete success.
    async fn materialize_checkpoint(&self, request: CheckpointRequest, context: &OperationContext<'_>) -> Result<CheckpointCandidate, AuthorityError>;
}

/// 🛡️ Contract implementation over a trusted project-owned catalog port.
pub struct ValidatingCanonicalArtifactAuthority<C> {
    catalog: C,
}

impl<C> ValidatingCanonicalArtifactAuthority<C> {
    /// 🧩️ Constructs the authority without resolving or executing a codec.
    pub const fn new(catalog: C) -> Self {
        Self { catalog }
    }
}

fn pair_bytes(pair: &ArtifactPair, stage: ArtifactValidationStage) -> Result<u64, AuthorityError> {
    let pack = u64::try_from(pair.pack.len()).map_err(|_| AuthorityError::PairResourceLimit(stage))?;
    let spr = u64::try_from(pair.spr.len()).map_err(|_| AuthorityError::PairResourceLimit(stage))?;
    pack.checked_add(spr).ok_or(AuthorityError::PairResourceLimit(stage))
}

fn validate_pair_budget(pair: &ArtifactPair, limits: AuthorityLimits, stage: ArtifactValidationStage) -> Result<(), AuthorityError> {
    if pair_bytes(pair, stage)? > limits.max_pair_bytes {
        return Err(AuthorityError::PairResourceLimit(stage));
    }
    Ok(())
}

fn validate_request(request: &CheckpointRequest, limits: AuthorityLimits) -> Result<(), AuthorityError> {
    if limits.max_operations == 0
        || limits.max_operations > AUTHORITY_MAX_OPERATIONS
        || limits.max_operation_bytes == 0
        || limits.max_operation_bytes > AUTHORITY_MAX_OPERATION_BYTES
        || limits.max_pair_bytes == 0
        || limits.max_pair_bytes > AUTHORITY_MAX_PAIR_BYTES
    {
        return Err(AuthorityError::InvalidLimits);
    }
    if request.scope.space_id != request.descriptor.space_id || request.scope.document_id != request.descriptor.document_id {
        return Err(AuthorityError::InvalidScope);
    }
    if request.parent_checkpoint_id.is_some_and(|hash| hash.0 == [0; 32]) {
        return Err(AuthorityError::InvalidParentCheckpoint);
    }
    if request.base_frontier.document_id != request.scope.document_id || request.base_frontier.head_edit_id.is_empty() {
        return Err(AuthorityError::InvalidFrontier);
    }
    if request.input_pair.pack.is_empty() || request.input_pair.spr.is_empty() {
        return Err(AuthorityError::Codec { stage: ArtifactValidationStage::Input, message: "pack and SPR must both be nonempty".to_string() });
    }
    if request.operations.len() > limits.max_operations {
        return Err(AuthorityError::ResourceLimit("operation count"));
    }
    let mut operation_bytes = 0u64;
    let mut previous = &request.base_frontier;
    for operation in &request.operations {
        let length = u64::try_from(operation.encoded.len()).map_err(|_| AuthorityError::ResourceLimit("operation byte"))?;
        operation_bytes = operation_bytes.checked_add(length).ok_or(AuthorityError::ResourceLimit("operation byte"))?;
        let expected_sequence = previous.last_commit_seq.checked_add(1).ok_or(AuthorityError::InvalidOperationOrder)?;
        let frontier = &operation.resulting_frontier;
        if operation.encoded.is_empty()
            || operation.sequence != expected_sequence
            || frontier.last_commit_seq != operation.sequence
            || frontier.document_id != request.scope.document_id
            || frontier.head_edit_id.is_empty()
            || frontier.head_edit_ordinal <= previous.head_edit_ordinal
            || frontier.chain_hash.0 == [0; 32]
        {
            return Err(AuthorityError::InvalidOperationOrder);
        }
        previous = frontier;
    }
    if operation_bytes > limits.max_operation_bytes {
        return Err(AuthorityError::ResourceLimit("operation byte"));
    }
    validate_pair_budget(&request.input_pair, limits, ArtifactValidationStage::Input)
}

fn append_field(output: &mut Vec<u8>, bytes: &[u8]) -> Result<(), AuthorityError> {
    let length = u64::try_from(bytes.len()).map_err(|_| AuthorityError::ResourceLimit("checkpoint identity byte"))?;
    output.extend_from_slice(&length.to_be_bytes());
    output.extend_from_slice(bytes);
    Ok(())
}

/// 🧬️ Encodes checkpoint identity fields after [`CHECKPOINT_ID_V1_DOMAIN`] in declared order as
/// `u64_be(payload byte length) || payload`: scope space/document UTF-8, parent bytes (empty or 32),
/// descriptor digest, frontier document UTF-8/ordinal/head-id UTF-8/commit/chain, pack hash/length,
/// SPR hash/length, and aggregate hash. Integers are fixed-width big-endian payloads. Storage keys
/// and publication time are excluded because neither changes canonical artifact content or lineage.
pub fn checkpoint_id_encoding_v1(checkpoint: &ArtifactCheckpoint) -> Result<Vec<u8>, AuthorityError> {
    let mut output = Vec::with_capacity(CHECKPOINT_ID_V1_DOMAIN.len() + 384);
    output.extend_from_slice(CHECKPOINT_ID_V1_DOMAIN);
    append_field(&mut output, checkpoint.scope.space_id.as_bytes())?;
    append_field(&mut output, checkpoint.scope.document_id.as_bytes())?;
    append_field(&mut output, checkpoint.parent_checkpoint_id.as_ref().map_or(&[][..], |hash| &hash.0))?;
    append_field(&mut output, &checkpoint.descriptor_digest_v1.0)?;
    append_field(&mut output, checkpoint.baseline_frontier.document_id.as_bytes())?;
    append_field(&mut output, &checkpoint.baseline_frontier.head_edit_ordinal.to_be_bytes())?;
    append_field(&mut output, checkpoint.baseline_frontier.head_edit_id.as_bytes())?;
    append_field(&mut output, &checkpoint.baseline_frontier.last_commit_seq.to_be_bytes())?;
    append_field(&mut output, &checkpoint.baseline_frontier.chain_hash.0)?;
    append_field(&mut output, &checkpoint.pack.sha256.0)?;
    append_field(&mut output, &checkpoint.pack.byte_length.to_be_bytes())?;
    append_field(&mut output, &checkpoint.spr.sha256.0)?;
    append_field(&mut output, &checkpoint.spr.byte_length.to_be_bytes())?;
    append_field(&mut output, &checkpoint.aggregate_sha256.0)?;
    Ok(output)
}

fn exact_blob_integrity(bytes: &[u8]) -> Result<ArtifactBlobIntegrity, AuthorityError> {
    Ok(ArtifactBlobIntegrity { sha256: ArtifactHash(Sha256::digest(bytes)), byte_length: u64::try_from(bytes.len()).map_err(|_| AuthorityError::ResourceLimit("pair byte"))? })
}

fn verify_staged_blob(kind: &'static str, expected: ArtifactBlobIntegrity, staged: &StagedArtifactBlob, bytes: &[u8]) -> Result<(), AuthorityError> {
    if staged.storage_key.is_empty() || staged.integrity != expected || exact_blob_integrity(bytes)? != expected {
        return Err(AuthorityError::BlobIntegrity(kind));
    }
    Ok(())
}

impl<S: ImmutableArtifactBlobStore, P: VerifiedCheckpointPublisher> CheckpointPublicationOrchestrator<S, P> {
    /// 🚀️ Stages, reads back, and verifies both blobs before the sole publication call.
    pub async fn publish_candidate(&self, candidate: CheckpointCandidate, context: &OperationContext<'_>) -> Result<VerifiedCheckpointPublication, AuthorityError> {
        context.checkpoint()?;
        validate_pair_budget(&candidate.pair, context.limits(), ArtifactValidationStage::Output)?;
        let pack_integrity = exact_blob_integrity(&candidate.pair.pack)?;
        let spr_integrity = exact_blob_integrity(&candidate.pair.spr)?;
        if candidate.checkpoint.pack.sha256 != pack_integrity.sha256
            || candidate.checkpoint.pack.byte_length != pack_integrity.byte_length
            || candidate.checkpoint.spr.sha256 != spr_integrity.sha256
            || candidate.checkpoint.spr.byte_length != spr_integrity.byte_length
        {
            return Err(AuthorityError::BlobIntegrity("candidate"));
        }
        let mut aggregate = Sha256::new();
        aggregate.update(&candidate.pair.pack);
        aggregate.update(&candidate.pair.spr);
        if candidate.checkpoint.aggregate_sha256 != ArtifactHash(aggregate.finalize())
            || candidate.checkpoint.checkpoint_id != ArtifactHash(Sha256::digest(&checkpoint_id_encoding_v1(&candidate.checkpoint)?))
        {
            return Err(AuthorityError::BlobIntegrity("candidate"));
        }

        let ownership = chunk_cas::prepare_artifact_cas_ownership_v1(&candidate.checkpoint, &candidate.pair)?;
        let reservation = self.publisher.reserve(&ownership, context).await?;
        if reservation.plan != ownership {
            return Err(AuthorityError::Publication("directory returned a different artifact CAS reservation".into()));
        }

        let space_id = candidate.checkpoint.scope.space_id.as_str();
        let pack = self.store.stage(space_id, pack_integrity, &candidate.pair.pack, context).await?;
        context.report(AuthorityProgress { stage: AuthorityProgressStage::PackStaged, completed_units: 1, total_units: 5 })?;
        let spr = self.store.stage(space_id, spr_integrity, &candidate.pair.spr, context).await?;
        context.report(AuthorityProgress { stage: AuthorityProgressStage::SprStaged, completed_units: 2, total_units: 5 })?;

        let read_pack = self.store.read(space_id, &pack, context).await?;
        if read_pack != candidate.pair.pack {
            return Err(AuthorityError::BlobIntegrity("pack"));
        }
        verify_staged_blob("pack", pack_integrity, &pack, &read_pack)?;
        context.report(AuthorityProgress { stage: AuthorityProgressStage::PackVerified, completed_units: 3, total_units: 5 })?;

        let read_spr = self.store.read(space_id, &spr, context).await?;
        if read_spr != candidate.pair.spr {
            return Err(AuthorityError::BlobIntegrity("SPR"));
        }
        verify_staged_blob("SPR", spr_integrity, &spr, &read_spr)?;
        context.report(AuthorityProgress { stage: AuthorityProgressStage::SprVerified, completed_units: 4, total_units: 5 })?;

        let mut checkpoint = candidate.checkpoint;
        checkpoint.pack.storage_key = pack.storage_key;
        checkpoint.spr.storage_key = spr.storage_key;
        chunk_cas::validate_artifact_cas_publication_v1(&reservation.plan, &checkpoint)?;
        context.checkpoint()?;
        self.publisher.publish_reserved(&checkpoint, &reservation, context).await?;
        context.control.report(AuthorityProgress { stage: AuthorityProgressStage::Published, completed_units: 5, total_units: 5 });
        Ok(VerifiedCheckpointPublication { checkpoint })
    }
}

fn blob_reference(bytes: &[u8]) -> Result<ArtifactBlobRef, AuthorityError> {
    let integrity = exact_blob_integrity(bytes)?;
    Ok(ArtifactBlobRef { sha256: integrity.sha256, byte_length: integrity.byte_length, storage_key: format!("sha256/{}", hex_lower(&integrity.sha256.0)) })
}

impl<C: TrustedArtifactCatalog> CanonicalArtifactAuthority for ValidatingCanonicalArtifactAuthority<C> {
    async fn materialize_checkpoint(&self, request: CheckpointRequest, context: &OperationContext<'_>) -> Result<CheckpointCandidate, AuthorityError> {
        context.checkpoint()?;
        validate_request(&request, context.limits())?;
        let descriptor_digest = descriptor_digest_v1(&request.descriptor).map_err(|error| AuthorityError::InvalidDescriptor(error.to_string()))?;
        let operation_count = u64::try_from(request.operations.len()).map_err(|_| AuthorityError::ResourceLimit("operation count"))?;
        let total_units = operation_count.checked_add(5).ok_or(AuthorityError::ResourceLimit("operation count"))?;
        context.report(AuthorityProgress { stage: AuthorityProgressStage::Preflight, completed_units: 1, total_units })?;

        let required_identity = TrustedArtifactIdentity::from_descriptor(&request.descriptor);
        let codec = self.catalog.resolve(&required_identity).await?;
        if codec.identity() != &required_identity {
            return Err(AuthorityError::CodecIdentityMismatch);
        }
        context.report(AuthorityProgress { stage: AuthorityProgressStage::CatalogResolved, completed_units: 2, total_units })?;

        codec.validate_pair(&request.input_pair, ArtifactValidationStage::Input, context).await?;
        context.checkpoint()?;
        context.report(AuthorityProgress { stage: AuthorityProgressStage::InputValidated, completed_units: 3, total_units })?;

        let mut pair = request.input_pair;
        for (index, operation) in request.operations.iter().enumerate() {
            context.checkpoint()?;
            pair = codec.apply_operation(pair, operation, context).await?;
            validate_pair_budget(&pair, context.limits(), ArtifactValidationStage::Output)?;
            let completed_units = u64::try_from(index).map_err(|_| AuthorityError::ResourceLimit("operation count"))?.checked_add(4).ok_or(AuthorityError::ResourceLimit("operation count"))?;
            context.report(AuthorityProgress { stage: AuthorityProgressStage::ApplyingOperations, completed_units, total_units })?;
        }

        codec.validate_pair(&pair, ArtifactValidationStage::Output, context).await?;
        context.checkpoint()?;
        if pair.pack.is_empty() || pair.spr.is_empty() {
            return Err(AuthorityError::Codec { stage: ArtifactValidationStage::Output, message: "pack and SPR must both be nonempty".to_string() });
        }
        let output_validated_units = operation_count.checked_add(4).ok_or(AuthorityError::ResourceLimit("operation count"))?;
        context.report(AuthorityProgress { stage: AuthorityProgressStage::OutputValidated, completed_units: output_validated_units, total_units })?;

        let pack = blob_reference(&pair.pack)?;
        let spr = blob_reference(&pair.spr)?;
        let mut aggregate = Sha256::new();
        aggregate.update(&pair.pack);
        aggregate.update(&pair.spr);
        let aggregate_sha256 = ArtifactHash(aggregate.finalize());
        let baseline_frontier = request.operations.last().map_or(request.base_frontier, |operation| operation.resulting_frontier.clone());
        let mut checkpoint = ArtifactCheckpoint {
            scope: request.scope,
            checkpoint_id: ArtifactHash([0; 32]),
            parent_checkpoint_id: request.parent_checkpoint_id,
            descriptor_digest_v1: descriptor_digest,
            baseline_frontier,
            pack,
            spr,
            aggregate_sha256,
            published_at_ms: context.now_ms(),
        };
        checkpoint.checkpoint_id = ArtifactHash(Sha256::digest(&checkpoint_id_encoding_v1(&checkpoint)?));
        context.report(AuthorityProgress { stage: AuthorityProgressStage::Derived, completed_units: total_units, total_units })?;
        Ok(CheckpointCandidate { checkpoint, pair })
    }
}

#[path = "🔌️adapters/🦀️.rs"]
pub mod adapters;

#[path = "🗂️trusted-catalog/🦀️.rs"]
pub mod trusted_catalog;

#[cfg(test)]
mod tests {
    use super::*;
    use super::adapters::{bounded_message, LivePluginPackageBinding, PluginHostTrustedArtifactCatalog, AUTHORITY_MAX_DIAGNOSTIC_BYTES};
    use ::directory::os_directory::{DocumentFrontier, DocumentOwner};
    use ::directory::os_store::{register_document_codec, ArtifactCodec, ArtifactPackFiles, ArtifactTextFiles, VcsError};
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

    fn fixture_apply<'a>(pack: &'a [u8], spr: &'a [u8], operations: &'a [u8]) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(Vec<u8>, Vec<u8>, String), VcsError>> + 'a>> {
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
        ArtifactCodec {
            schema: "fixture.number@1".to_string(),
            extension: "fixture",
            pack_schema_hash: [0x11; 32],
            compile_dsl: fixture_compile,
            print_mirror: fixture_print,
            edit_text_from_envelope: fixture_edit,
            apply_ops_binary: fixture_apply,
        }
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
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🧬️canonical-authority/🔣️.json")).expect("valid authority fixture");
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
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🧬️authority-adapter/🔣️.json")).expect("adapter fixture");
        register_document_codec(fixture_artifact_codec()).await.expect("register fixture codec");
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
            .apply_operation(
                ArtifactPair { pack: b"10".to_vec(), spr: b"seed".to_vec() },
                &AcceptedArtifactOperation { sequence: 1, encoded: b"5".to_vec(), resulting_frontier: frontier(1, 1) },
                &context(&control, limits()),
            )
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
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🧬️authority-adapter/🔣️.json")).expect("adapter fixture");
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

}
