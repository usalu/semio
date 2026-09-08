//! 🛡️ Hub-owned canonical artifact authority and failure-atomic immutable publication ports. Package-host
//! catalog resolution and database blob staging live in [`adapters`]; directory events, retention
//! advancement, and WebSocket production remain the P2-B/P2-C seams.

use ::directory::os_directory::{descriptor_digest_v1, hex_lower, ArtifactBlobRef, ArtifactCheckpoint, ArtifactFrontier, ArtifactHash, DocumentDescriptor, DocumentScope};
use semio_framework_hash::Sha256;

#[path = "🧱️chunk-cas/🦀️.rs"]
pub mod chunk_cas;

#[path = "🌱️creation/🦀️.rs"]
pub mod creation;

#[path = "🔒️file-fence/🦀️.rs"]
mod file_fence;

/// 🔐️ Domain prefix for a canonical checkpoint identity.
pub const CHECKPOINT_ID_V1_DOMAIN: &[u8] = b"semio.hub.artifact-checkpoint.v1\0";

/// 🌱️ A zero-history root has an identity domain distinct from edited checkpoints.
pub const GENESIS_CHECKPOINT_ID_V1_DOMAIN: &[u8] = b"semio.hub.artifact-genesis.v1\0";

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

    /// ⏱️ Returns the exact operation clock observation shared with reservation and publication.
    pub fn now_ms(&self) -> u64 {
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

/// 🌱️ Creation is executable package authority, separate from ordinary codec registration.
pub trait TrustedArtifactGenesisCodec: TrustedArtifactCodec {
    /// 🪺️ Creates the selected artifact's own initial snapshot with no accepted edit.
    async fn initial_pair(&self, document_id: &str, dialect: &::directory::os_io::ArtifactDialect, context: &OperationContext<'_>) -> Result<ArtifactPair, AuthorityError>;
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
    if checkpoint.baseline_frontier.is_genesis_for(&checkpoint.scope) {
        if checkpoint.parent_checkpoint_id.is_some() { return Err(AuthorityError::InvalidParentCheckpoint); }
        output.extend_from_slice(GENESIS_CHECKPOINT_ID_V1_DOMAIN);
    } else {
        output.extend_from_slice(CHECKPOINT_ID_V1_DOMAIN);
    }
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
        if candidate.checkpoint.aggregate_sha256 != ArtifactHash(aggregate.finalize()) || candidate.checkpoint.checkpoint_id != ArtifactHash(Sha256::digest(&checkpoint_id_encoding_v1(&candidate.checkpoint)?)) {
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

#[path = "🔏️trusted-catalog/🦀️.rs"]
pub mod trusted_catalog;

#[cfg(feature = "native-artifact-execution")]
#[path = "📇️native-openable-provider/🦀️.rs"]
pub mod native_openable_provider;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
