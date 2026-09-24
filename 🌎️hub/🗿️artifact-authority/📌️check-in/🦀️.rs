//! 📌️ Hub-materialized Check In. A document author names a committed head of the hub's own ledger;
//! the hub folds the ledger between the active checkpoint and that head onto the active checkpoint's
//! pair through the package codec's replica gate (`codec.replay-envelopes`,
//! `store::replay_envelopes_onto_pair`), validates the result, and publishes it as the document's
//! next active checkpoint. No client pair, pair hash, or parent checkpoint is ever accepted.
//!
//! This module owns the authority half ([`ReplayingArtifactAuthority`]) and the job's observable
//! state ([`DocumentCheckInJob`], [`DocumentCheckInJobs`]); the HTTP routes, the ledger read and the
//! fenced publication live in `🏗️bootstrap/🦀️.rs` (`📌️CheckIn`). Contract:
//! `schema://os.directory/DocumentCheckInV1` and `DocumentCheckInStatusV1`.

use super::{
    blob_reference, checkpoint_id_encoding_v1, validate_pair_budget, ArtifactPair, ArtifactValidationStage, AuthorityError, AuthorityOperationControl, AuthorityProgress, AuthorityProgressStage, CheckpointCandidate, OperationContext,
    TrustedArtifactCatalog, TrustedArtifactCodec, TrustedArtifactIdentity, TrustedArtifactReplayCodec, ValidatingCanonicalArtifactAuthority, AUTHORITY_MAX_OPERATION_BYTES,
};
use ::directory::os_directory::{
    descriptor_digest_v1, ArtifactCheckpoint, ArtifactFrontier, ArtifactHash, DocumentCheckInPhaseV1, DocumentCheckInProgressV1, DocumentCheckInReadyV1, DocumentCheckInRefusalV1, DocumentCheckInStatusV1, DocumentDescriptor, DocumentScope,
    EditedArtifactFrontierV1, DOCUMENT_CHECK_IN_STATUS_SCHEMA_V1,
};
use semio_framework_hash::Sha256;
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

/// 🔢️ Fixed progress units of one Check In: active pair read, ledger read, codec resolved, input
/// validated, ledger replayed and output validated, blobs staged and verified, published, ready.
pub const DOCUMENT_CHECK_IN_TOTAL_UNITS: u64 = 8;
/// 🧗️ A Check In is refused only when it reached no checkpoint for this span; the guest codec's fuel
/// progress counts, so a long honest replay is never refused for its length.
pub const DOCUMENT_CHECK_IN_STALL_BOUND_MS: u64 = 30_000;
/// 🧯️ Live (non-terminal) Check In jobs one hub runs at once.
pub const DOCUMENT_CHECK_IN_MAX_LIVE: usize = 256;
/// 🧯️ Terminal statuses one hub keeps answering polls for, oldest evicted first.
pub const DOCUMENT_CHECK_IN_MAX_RETAINED: usize = 1024;

/// 📥️ Owned authority input of one Check In: the active checkpoint's own pair, its id as the parent,
/// and the committed ledger stream between that checkpoint's baseline and `head`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CheckInMaterialization {
    pub descriptor: DocumentDescriptor,
    pub scope: DocumentScope,
    pub parent_checkpoint_id: ArtifactHash,
    pub base_pair: ArtifactPair,
    pub head: ArtifactFrontier,
    pub envelopes: Vec<u8>,
}

/// 🏗️ The Check In half of the hub's canonical authority.
pub trait ReplayingArtifactAuthority {
    /// 📌️ Folds the ledger stream onto the base pair, validates both sides, and transfers one
    /// checkpoint candidate whose baseline is exactly `head`.
    async fn materialize_check_in(&self, request: CheckInMaterialization, context: &OperationContext<'_>) -> Result<CheckpointCandidate, AuthorityError>;
}

fn validate_check_in(request: &CheckInMaterialization, context: &OperationContext<'_>) -> Result<(), AuthorityError> {
    if request.scope.space_id != request.descriptor.space_id || request.scope.document_id != request.descriptor.document_id {
        return Err(AuthorityError::InvalidScope);
    }
    if request.parent_checkpoint_id.0 == [0; 32] {
        return Err(AuthorityError::InvalidParentCheckpoint);
    }
    if request.head.document_id != request.scope.document_id || request.head.head_edit_id.is_empty() || request.head.head_edit_ordinal == 0 || request.head.last_commit_seq == 0 || request.head.chain_hash.0 == [0; 32] {
        return Err(AuthorityError::InvalidFrontier);
    }
    if request.base_pair.pack.is_empty() || request.base_pair.spr.is_empty() {
        return Err(AuthorityError::Codec { stage: ArtifactValidationStage::Input, message: "pack and SPR must both be nonempty".to_string() });
    }
    if request.envelopes.is_empty() {
        return Err(AuthorityError::InvalidOperationOrder);
    }
    if u64::try_from(request.envelopes.len()).map_or(true, |length| length > AUTHORITY_MAX_OPERATION_BYTES.min(context.limits().max_operation_bytes)) {
        return Err(AuthorityError::ResourceLimit("operation byte"));
    }
    validate_pair_budget(&request.base_pair, context.limits(), ArtifactValidationStage::Input)
}

impl<C> ReplayingArtifactAuthority for ValidatingCanonicalArtifactAuthority<C>
where
    C: TrustedArtifactCatalog,
    C::Codec: TrustedArtifactReplayCodec,
{
    async fn materialize_check_in(&self, request: CheckInMaterialization, context: &OperationContext<'_>) -> Result<CheckpointCandidate, AuthorityError> {
        context.checkpoint()?;
        validate_check_in(&request, context)?;
        let descriptor_digest = descriptor_digest_v1(&request.descriptor).map_err(|error| AuthorityError::InvalidDescriptor(error.to_string()))?;
        let required_identity = TrustedArtifactIdentity::from_descriptor(&request.descriptor);
        let codec = self.catalog.resolve(&required_identity).await?;
        if codec.identity() != &required_identity {
            return Err(AuthorityError::CodecIdentityMismatch);
        }
        context.report(AuthorityProgress { stage: AuthorityProgressStage::CatalogResolved, completed_units: 3, total_units: DOCUMENT_CHECK_IN_TOTAL_UNITS })?;
        codec.validate_pair(&request.base_pair, ArtifactValidationStage::Input, context).await?;
        context.report(AuthorityProgress { stage: AuthorityProgressStage::InputValidated, completed_units: 4, total_units: DOCUMENT_CHECK_IN_TOTAL_UNITS })?;
        let pair = codec.replay_envelopes(request.base_pair, &request.envelopes, context).await?;
        validate_pair_budget(&pair, context.limits(), ArtifactValidationStage::Output)?;
        codec.validate_pair(&pair, ArtifactValidationStage::Output, context).await?;
        if pair.pack.is_empty() || pair.spr.is_empty() {
            return Err(AuthorityError::Codec { stage: ArtifactValidationStage::Output, message: "pack and SPR must both be nonempty".to_string() });
        }
        context.report(AuthorityProgress { stage: AuthorityProgressStage::OutputValidated, completed_units: 5, total_units: DOCUMENT_CHECK_IN_TOTAL_UNITS })?;
        let mut aggregate = Sha256::new();
        aggregate.update(&pair.pack);
        aggregate.update(&pair.spr);
        let mut checkpoint = ArtifactCheckpoint {
            scope: request.scope,
            checkpoint_id: ArtifactHash([0; 32]),
            parent_checkpoint_id: Some(request.parent_checkpoint_id),
            descriptor_digest_v1: descriptor_digest,
            baseline_frontier: request.head,
            pack: blob_reference(&pair.pack)?,
            spr: blob_reference(&pair.spr)?,
            aggregate_sha256: ArtifactHash(aggregate.finalize()),
            published_at_ms: context.now_ms(),
        };
        checkpoint.checkpoint_id = ArtifactHash(Sha256::digest(&checkpoint_id_encoding_v1(&checkpoint)?));
        context.report(AuthorityProgress { stage: AuthorityProgressStage::Derived, completed_units: 5, total_units: DOCUMENT_CHECK_IN_TOTAL_UNITS })?;
        Ok(CheckpointCandidate { checkpoint, pair })
    }
}

/// ⛔️ The refusal an authority failure maps to; `None` for cancellation, which ends a Check In as
/// `cancelled` rather than `failed`.
pub fn check_in_refusal_of_authority_error(error: &AuthorityError) -> Option<DocumentCheckInRefusalV1> {
    match error {
        AuthorityError::Cancelled => None,
        AuthorityError::Codec { .. } | AuthorityError::CodecIdentityMismatch | AuthorityError::PairResourceLimit(_) | AuthorityError::ResourceLimit(_) | AuthorityError::InvalidOperationOrder => Some(DocumentCheckInRefusalV1::CodecRefused),
        AuthorityError::InvalidFrontier => Some(DocumentCheckInRefusalV1::UnknownHead),
        AuthorityError::InvalidDescriptor(_) | AuthorityError::InvalidScope => Some(DocumentCheckInRefusalV1::AuthorityChanged),
        AuthorityError::Publication(_) => Some(DocumentCheckInRefusalV1::ActiveCheckpointChanged),
        AuthorityError::DeadlineExceeded | AuthorityError::Stalled | AuthorityError::InvalidParentCheckpoint | AuthorityError::InvalidLimits | AuthorityError::Catalog(_) | AuthorityError::Store(_) | AuthorityError::BlobIntegrity(_) => {
            Some(DocumentCheckInRefusalV1::Unavailable)
        }
    }
}

/// 🔢️ The unit an authority progress stage completes, or `None` for a liveness-only observation.
fn check_in_units_of_stage(stage: AuthorityProgressStage) -> Option<u64> {
    match stage {
        AuthorityProgressStage::CatalogResolved => Some(3),
        AuthorityProgressStage::InputValidated => Some(4),
        AuthorityProgressStage::OutputValidated | AuthorityProgressStage::Derived => Some(5),
        AuthorityProgressStage::PackStaged | AuthorityProgressStage::SprStaged | AuthorityProgressStage::PackVerified | AuthorityProgressStage::SprVerified => Some(6),
        AuthorityProgressStage::Published => Some(7),
        _ => None,
    }
}

struct DocumentCheckInJobState {
    phase: DocumentCheckInPhaseV1,
    completed_units: u64,
    ready: Option<DocumentCheckInReadyV1>,
    refusal: Option<DocumentCheckInRefusalV1>,
}

/// 📣️ One Check In's observable state: monotonic progress, one terminal outcome, cancellation.
/// It is also the job's [`AuthorityOperationControl`], so authority progress lands in the status the
/// client polls and the client's cancellation reaches every authority checkpoint.
pub struct DocumentCheckInJob {
    request_id: String,
    command_sha256: String,
    cancelled: Arc<AtomicBool>,
    revoked: AtomicBool,
    state: Mutex<DocumentCheckInJobState>,
}

impl DocumentCheckInJob {
    /// 🆕️ An accepted Check In with no progress, bound to the exact canonical request it serves.
    pub fn new(request_id: impl Into<String>, command_sha256: impl Into<String>) -> Arc<Self> {
        Arc::new(Self {
            request_id: request_id.into(),
            command_sha256: command_sha256.into(),
            cancelled: Arc::new(AtomicBool::new(false)),
            revoked: AtomicBool::new(false),
            state: Mutex::new(DocumentCheckInJobState { phase: DocumentCheckInPhaseV1::Accepted, completed_units: 0, ready: None, refusal: None }),
        })
    }

    fn state(&self) -> std::sync::MutexGuard<'_, DocumentCheckInJobState> {
        self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    /// 🪪️ The client's request id.
    pub fn request_id(&self) -> &str {
        &self.request_id
    }

    /// 🔏️ SHA-256 of the canonical request this job serves.
    pub fn command_sha256(&self) -> &str {
        &self.command_sha256
    }

    /// 📣️ The exact wire status.
    pub fn status(&self) -> DocumentCheckInStatusV1 {
        let state = self.state();
        DocumentCheckInStatusV1 {
            schema: DOCUMENT_CHECK_IN_STATUS_SCHEMA_V1.to_string(),
            request_id: self.request_id.clone(),
            phase: state.phase,
            progress: DocumentCheckInProgressV1 { completed_units: state.completed_units, total_units: DOCUMENT_CHECK_IN_TOTAL_UNITS },
            ready: state.ready.clone(),
            refusal: state.refusal,
        }
    }

    /// 🏁️ Whether the job reached ready, failed, or cancelled.
    pub fn is_terminal(&self) -> bool {
        self.state().phase.is_terminal()
    }

    /// ➡️ Moves a live job to `phase` with at least `completed_units`; progress never goes back and a
    /// terminal job never changes.
    pub fn advance(&self, phase: DocumentCheckInPhaseV1, completed_units: u64) {
        let mut state = self.state();
        if state.phase.is_terminal() || phase.is_terminal() {
            return;
        }
        state.phase = phase;
        state.completed_units = state.completed_units.max(completed_units.min(DOCUMENT_CHECK_IN_TOTAL_UNITS - 1));
    }

    /// 🚩️ Ends the job with the checkpoint it made active.
    pub fn finish_ready(&self, checkpoint_id: ArtifactHash, parent_checkpoint_id: ArtifactHash, baseline: EditedArtifactFrontierV1) {
        let mut state = self.state();
        if state.phase.is_terminal() {
            return;
        }
        state.phase = DocumentCheckInPhaseV1::Ready;
        state.completed_units = DOCUMENT_CHECK_IN_TOTAL_UNITS;
        state.ready = Some(DocumentCheckInReadyV1 { checkpoint_id: checkpoint_id.hex(), parent_checkpoint_id: parent_checkpoint_id.hex(), baseline });
    }

    /// ⛔️ Ends the job with a refusal; a revoked author's job always reads `authority-changed`, and a
    /// job the client cancelled first reads `cancelled`.
    pub fn finish_refused(&self, refusal: DocumentCheckInRefusalV1) {
        let mut state = self.state();
        if state.phase.is_terminal() {
            return;
        }
        if self.revoked.load(Ordering::Acquire) {
            state.phase = DocumentCheckInPhaseV1::Failed;
            state.refusal = Some(DocumentCheckInRefusalV1::AuthorityChanged);
            return;
        }
        if self.cancelled.load(Ordering::Acquire) {
            state.phase = DocumentCheckInPhaseV1::Cancelled;
            return;
        }
        state.phase = DocumentCheckInPhaseV1::Failed;
        state.refusal = Some(refusal);
    }

    /// 🛑️ Ends the job as cancelled, or as `authority-changed` when the author was revoked.
    pub fn finish_cancelled(&self) {
        self.finish_refused(DocumentCheckInRefusalV1::Unavailable);
    }

    /// 🛑️ Requests cancellation; the job ends at its next checkpoint.
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
    }

    /// 🚷️ The author lost write access mid-flight: the job stops at its next checkpoint and ends
    /// `failed`/`authority-changed`, never `cancelled`.
    pub fn revoke(&self) {
        self.revoked.store(true, Ordering::Release);
        self.cancelled.store(true, Ordering::Release);
    }

    /// 🛑️ The shared cancellation flag for storage reads that poll one.
    pub fn cancellation(&self) -> Arc<AtomicBool> {
        self.cancelled.clone()
    }
}

impl AuthorityOperationControl for DocumentCheckInJob {
    fn now_ms(&self) -> u64 {
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |duration| u64::try_from(duration.as_millis()).unwrap_or(u64::MAX))
    }

    fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }

    fn report(&self, progress: AuthorityProgress) {
        if let Some(units) = check_in_units_of_stage(progress.stage) {
            let phase = if units >= 6 { DocumentCheckInPhaseV1::Publishing } else { DocumentCheckInPhaseV1::Materializing };
            self.advance(phase, units);
        }
    }
}

/// 🔑️ A Check In belongs to one author, one document, and one request id.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DocumentCheckInKey {
    pub user_id: String,
    pub space_id: String,
    pub document_id: String,
    pub request_id: String,
}

/// 🎫️ The outcome of admitting one Check In request.
pub enum DocumentCheckInAdmission {
    /// This request owns a fresh job and must run it.
    Owner(Arc<DocumentCheckInJob>),
    /// The same request is already known; answer its status.
    Existing(Arc<DocumentCheckInJob>),
    /// The same request id already names a different request.
    Conflict,
    /// Too many live Check Ins.
    Unavailable,
}

/// 🗂️ The bounded registry of one hub's Check In jobs, keyed by author, document, and request id.
#[derive(Default)]
pub struct DocumentCheckInJobs {
    jobs: Mutex<BTreeMap<DocumentCheckInKey, (u64, Arc<DocumentCheckInJob>)>>,
    sequence: std::sync::atomic::AtomicU64,
}

impl DocumentCheckInJobs {
    fn jobs(&self) -> std::sync::MutexGuard<'_, BTreeMap<DocumentCheckInKey, (u64, Arc<DocumentCheckInJob>)>> {
        self.jobs.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    /// 🎫️ Joins a known request or reserves a fresh job, evicting the oldest terminal statuses beyond
    /// the retention ceiling.
    pub fn admit(&self, key: DocumentCheckInKey, command_sha256: &str) -> DocumentCheckInAdmission {
        let mut jobs = self.jobs();
        if let Some((_, job)) = jobs.get(&key) {
            return if job.command_sha256() == command_sha256 { DocumentCheckInAdmission::Existing(job.clone()) } else { DocumentCheckInAdmission::Conflict };
        }
        if jobs.values().filter(|(_, job)| !job.is_terminal()).count() >= DOCUMENT_CHECK_IN_MAX_LIVE {
            return DocumentCheckInAdmission::Unavailable;
        }
        while jobs.len() >= DOCUMENT_CHECK_IN_MAX_RETAINED {
            let Some(oldest) = jobs.iter().filter(|(_, (_, job))| job.is_terminal()).min_by_key(|(_, (sequence, _))| *sequence).map(|(key, _)| key.clone()) else { return DocumentCheckInAdmission::Unavailable };
            jobs.remove(&oldest);
        }
        let job = DocumentCheckInJob::new(key.request_id.clone(), command_sha256);
        let sequence = self.sequence.fetch_add(1, Ordering::Relaxed);
        jobs.insert(key, (sequence, job.clone()));
        DocumentCheckInAdmission::Owner(job)
    }

    /// 🔍️ The job of one exact key.
    pub fn get(&self, key: &DocumentCheckInKey) -> Option<Arc<DocumentCheckInJob>> {
        self.jobs().get(key).map(|(_, job)| job.clone())
    }

    /// 🗑️ Forgets a job whose owner could not start it, so the same request can be admitted again.
    pub fn forget(&self, key: &DocumentCheckInKey) {
        self.jobs().remove(key);
    }

    /// 🛑️ Cancels every live job; the hub's shutdown does this before it closes its database, so no
    /// Check In task keeps the database shared past the drain.
    pub fn cancel_all(&self) {
        for (_, job) in self.jobs().values() {
            job.cancel();
        }
    }

    /// 🔢️ Jobs that have not reached a terminal status.
    pub fn live_count(&self) -> usize {
        self.jobs().values().filter(|(_, job)| !job.is_terminal()).count()
    }

    /// 🛑️ Cancels every live job of one document; used when the document's authority changes.
    pub fn cancel_document(&self, space_id: &str, document_id: &str) {
        for (key, (_, job)) in self.jobs().iter() {
            if key.space_id == space_id && key.document_id == document_id {
                job.cancel();
            }
        }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
