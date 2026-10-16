//! 🧾️ The commit contract: route errors, approval requests, receipts, the ingress and write authority ports, the checkpoint publisher port and the unavailable committer.

use std::sync::{Arc, Mutex};
use directory::os_directory::{ArtifactFrontier, DocumentScope};
use semio_hub_inference::wal::CommittedInferenceWalWitnessV1;
use std::future::Future;
use semio_hub_inference::{InferenceErrorV1, InferencePrivateBytesV1};
use crate::job_types_and_port::InferenceMapBaseV1;
use semio_hub_inference_binding::MapBindingRefusalV1;

/// 🚦️ The one stable failure vocabulary the four authenticated inference routes may publish.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InferenceRouteErrorV1 {
    Unavailable,
    Denied,
    NotFound,
    Invalid,
    Bounds,
    Conflict,
    Capacity,
    Expired,
    Cancelled,
    CommitUnavailable,
    Storage,
}

impl InferenceRouteErrorV1 {
    /// 🏷️ Returns the exact wire code; a caller never learns which private object was missing.
    pub const fn code(self) -> &'static str {
        match self {
            Self::Unavailable => "inference.unavailable",
            Self::Denied => "inference.denied",
            Self::NotFound => "inference.not-found",
            Self::Invalid => "inference.invalid",
            Self::Bounds => "inference.bounds",
            Self::Conflict => "inference.conflict",
            Self::Capacity => "inference.capacity",
            Self::Expired => "inference.expired",
            Self::Cancelled => "inference.cancelled",
            Self::CommitUnavailable => "approval.commit-unavailable",
            Self::Storage => "inference.storage",
        }
    }

    /// 🔢️ Returns the exact HTTP status this stable code publishes.
    pub const fn status(self) -> u16 {
        match self {
            Self::Unavailable | Self::CommitUnavailable | Self::Storage => 503,
            Self::Denied => 403,
            Self::NotFound => 404,
            Self::Invalid => 400,
            Self::Bounds => 413,
            Self::Conflict | Self::Cancelled => 409,
            Self::Capacity => 429,
            Self::Expired => 410,
        }
    }
}

impl From<InferenceErrorV1> for InferenceRouteErrorV1 {
    fn from(error: InferenceErrorV1) -> Self {
        match error {
            InferenceErrorV1::Invalid => Self::Invalid,
            InferenceErrorV1::Bounds => Self::Bounds,
            InferenceErrorV1::Denied => Self::Denied,
            InferenceErrorV1::Conflict => Self::Conflict,
            InferenceErrorV1::Capacity => Self::Capacity,
            InferenceErrorV1::Expired => Self::Expired,
            InferenceErrorV1::Cancelled => Self::Cancelled,
            InferenceErrorV1::Storage => Self::Storage,
        }
    }
}

impl From<MapBindingRefusalV1> for InferenceRouteErrorV1 {
    fn from(refusal: MapBindingRefusalV1) -> Self {
        match refusal {
            MapBindingRefusalV1::Undecodable | MapBindingRefusalV1::Execution => Self::Invalid,
            MapBindingRefusalV1::Arity | MapBindingRefusalV1::Inapplicable | MapBindingRefusalV1::Composition => Self::Conflict,
            MapBindingRefusalV1::Bounds => Self::Bounds,
        }
    }
}

/// 🧾️ Everything the atomic parent+existing-child composition transaction needs, all server-derived.
pub struct GisMapApprovalCommitRequestV1<'a> {
    /// 🕸️ The composed child members the frozen base Map already owns, in stable-member order.
    ///
    /// A `CreateRegion` is never parent-only when these are present: `create_region_group_work`
    /// pairs it with a `gismap-drawing` `CreateNode` and a `gismap-value` `insertListItem`, and the
    /// Map's own apply function does not keep them in sync. The retained committer accepts only the
    /// exact ordered drawing/value pair and refuses every parent-only or substituted membership.
    pub composed_children: &'a [String],
    pub scope: &'a DocumentScope,
    pub actor: &'a str,
    pub mutation_id: &'a str,
    pub command_hash: &'a str,
    pub job_id: &'a str,
    pub proposal_hash: &'a str,
    pub command: &'a [u8],
    pub base: &'a InferenceMapBaseV1,
    pub base_frontier: &'a ArtifactFrontier,
    pub deadline_ms: u64,
    pub now_ms: u64,
    pub document_write: Arc<tokio::sync::Mutex<()>>,
    pub ingress: Arc<dyn GisMapApprovalIngressAuthorityV1>,
}

/// ↩️ Server-owned durable inverse request; the client contributes no mutation bytes.
pub struct GisMapApprovalUndoCommitRequestV1<'a> {
    pub composed_children: &'a [String],
    pub scope: &'a DocumentScope,
    pub actor: &'a str,
    pub target: &'a semio_hub_inference_ledger::GisMapApprovalUndoTargetV1,
    pub idempotency_key: &'a str,
    pub mutation_id: &'a str,
    pub command_hash: &'a str,
    pub operation_id: &'a str,
    pub proposal_hash: &'a str,
    pub command: &'a [u8],
    pub base: &'a InferenceMapBaseV1,
    pub deadline_ms: u64,
    pub now_ms: u64,
    pub document_write: Arc<tokio::sync::Mutex<()>>,
    pub ingress: Arc<dyn GisMapApprovalIngressAuthorityV1>,
}

/// 🎫️ Hub-owned admission retained across the exact privileged approval operation.
pub trait GisMapApprovalIngressAuthorityV1: Send + Sync {
    fn scope(&self) -> &DocumentScope;
    fn user_id(&self) -> &str;
    fn session_id(&self) -> &str;
    fn authorization_generation(&self) -> u64;
}

/// 🔒 One exact Hub document-write exclusion held from actor recheck through durable verification.
pub struct GisMapDocumentWriteAuthorityV1 {
    pub gate: Arc<tokio::sync::Mutex<()>>,
    pub request: Arc<GisMapApprovalRequestTokenV1>,
    pub(super) _guard: tokio::sync::OwnedMutexGuard<()>,
}

pub struct GisMapApprovalRequestTokenV1;

impl GisMapDocumentWriteAuthorityV1 {
    pub async fn acquire(gate: Arc<tokio::sync::Mutex<()>>, request: Arc<GisMapApprovalRequestTokenV1>) -> Self {
        let guard = gate.clone().lock_owned().await;
        Self { gate, request, _guard: guard }
    }
}

pub type GisMapDocumentWriteLeaseV1 = Arc<GisMapDocumentWriteAuthorityV1>;

/// ⛔️ Why a typed composition publication refused; never a partial or optimistic outcome.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GisMapApprovalCommitErrorV1 {
    Unavailable,
    Rejected,
    Conflict,
    Capacity,
    Storage,
}

/// 🔎️ The exact arm one retained GIS-map approval commit answered `Conflict` from. The inference
/// route maps `Conflict` onto `InferenceRouteErrorV1::Conflict` one-to-one and carries no body, so a
/// law that reads the taxonomy cannot name the refusal. The returned error and the wire are
/// identical in production; only a test build records which arm produced it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GisMapCommitConflictArmV1 {
    PreflightCheckpointDiffers,
    PreflightIdentityOrStoresDiffer,
    PreflightWriterMissing,
    PublishStateMissing,
    PublishIdentityDiffers,
    PublishStateMissingAfterSnapshot,
    PublishIdentityDiffersAfterSnapshot,
    PublishProjectedFrontierDiffers,
    PublishResponseDiffers,
    VerifyFenceStateDiffers,
    VerifyWalVerifier,
    VerifyStateDiffersAfterWitness,
    VerifyPublishedStateDiffers,
    VerifyPublicationFrontierMissing,
    VerifyUndoVerifier,
    VerifyApplyVerifier,
    VerifyCheckpointApplyMissing,
    VerifyReceiptStateDiffers,
    DriveGenerationDiffers,
    DriveUndoGenerationDiffers,
    PublisherRegionDiffers,
    PublisherProposalStateOrGate,
    PublisherCheckpointApplyState,
    DrivePublishedIdentityAndStoresDiffer,
}

#[cfg(any(test, feature = "testkit"))]
static LAST_GIS_MAP_COMMIT_CONFLICT_ARM: std::sync::Mutex<Option<GisMapCommitConflictArmV1>> = std::sync::Mutex::new(None);

/// 🧾️ Answers the unchanged `Conflict` after recording which arm produced it.
pub fn gis_map_commit_conflict(arm: GisMapCommitConflictArmV1) -> GisMapApprovalCommitErrorV1 {
    #[cfg(any(test, feature = "testkit"))]
    {
        *LAST_GIS_MAP_COMMIT_CONFLICT_ARM.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(arm);
    }
    let _ = arm;
    GisMapApprovalCommitErrorV1::Conflict
}

/// 🔬️ The last recorded conflict arm of this process, for laws that only see the taxonomy.
#[cfg(any(test, feature = "testkit"))]
pub fn last_gis_map_commit_conflict_arm() -> Option<GisMapCommitConflictArmV1> {
    *LAST_GIS_MAP_COMMIT_CONFLICT_ARM.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
}

static LAST_ABANDONED_ASSEMBLY_TURN: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None);

/// 🧾️ Records what ONE abandoned `Assembly` turn did, so a law that only sees "still Assembly after
/// its budget" learns whether the turn is spinning on `Progress(<phase>)` or looping on a named
/// `Err`. `Continue` and `Retry` are the same symptom and different defects.
pub fn record_abandoned_assembly_turn(outcome: &str) {
    #[cfg(any(test, feature = "testkit"))]
    {
        *LAST_ABANDONED_ASSEMBLY_TURN.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(outcome.to_owned());
    }
    let _ = outcome;
}

static LAST_GIS_MAP_PREPARE_CONFLICT: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None);

/// 🧾️ Records WHICH retained state refused a joining request and on which conjunct.
/// `prepare_retained_document` returns a bare `Conflict` from six arms, so a request refused before
/// the drive loop ever runs prints neither a conflict arm nor an identity mismatch — it names
/// nothing at all. This names the state and the false conjunct.
pub fn record_prepare_conflict(reason: &str) {
    #[cfg(any(test, feature = "testkit"))]
    {
        *LAST_GIS_MAP_PREPARE_CONFLICT.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(reason.to_owned());
    }
    let _ = reason;
}

/// 🔬️ The last retained-document join refusal of this process.
#[cfg(any(test, feature = "testkit"))]
pub fn last_gis_map_prepare_conflict() -> Option<String> {
    LAST_GIS_MAP_PREPARE_CONFLICT.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone()
}

static LAST_GIS_MAP_IDENTITY_MISMATCH: std::sync::Mutex<Option<&'static str>> = std::sync::Mutex::new(None);

/// 🧾️ Records which conjunct of a retained-identity join was false, so a `Conflict` against an
/// already-retained publication names a field instead of a bool.
pub fn record_identity_mismatch(field: &'static str) {
    #[cfg(any(test, feature = "testkit"))]
    {
        *LAST_GIS_MAP_IDENTITY_MISMATCH.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(field);
    }
    let _ = field;
}

/// 🔬️ The last retained-identity join mismatch of this process.
#[cfg(any(test, feature = "testkit"))]
pub fn last_gis_map_identity_mismatch() -> Option<&'static str> {
    *LAST_GIS_MAP_IDENTITY_MISMATCH.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
}

/// 🔬️ The last abandoned `Assembly` turn of this process.
#[cfg(any(test, feature = "testkit"))]
pub fn last_abandoned_assembly_turn() -> Option<String> {
    LAST_ABANDONED_ASSEMBLY_TURN.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone()
}

#[cfg(any(test, feature = "testkit"))]
static LAST_GIS_MAP_STORAGE_REFUSAL: Mutex<Option<String>> = Mutex::new(None);

/// 🧾️ The one constructor of a retained approval's `Storage` refusal. It records the exact source
/// line that refused and the authority's own error, so a law that only sees `Err(Storage)` learns
/// where and why without a second diagnostic channel.
#[track_caller]
pub fn gis_map_storage_refusal(error: &dyn std::fmt::Debug) -> GisMapApprovalCommitErrorV1 {
    #[cfg(any(test, feature = "testkit"))]
    {
        *LAST_GIS_MAP_STORAGE_REFUSAL.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(format!("{}: {error:?}", std::panic::Location::caller()));
    }
    let _ = error;
    GisMapApprovalCommitErrorV1::Storage
}

/// 🔬️ The last `Storage` refusal of a retained approval in this process.
#[cfg(any(test, feature = "testkit"))]
pub fn last_gis_map_storage_refusal() -> Option<String> {
    LAST_GIS_MAP_STORAGE_REFUSAL.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone()
}

static LAST_CLOSE_STORE_STEP: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None);

/// 🧾️ Records the LAST owned-Store retirement step `close()` drove, named by slot. `Pending` is
/// real progress and loops; `Blocked` is the close cursor saying it has nothing returned to retire
/// while the lease registry is not terminal — a state only a LEASE HOLDER can leave, never the
/// closer. Retrying it spun `close().await` on `yield_now` for ever, so a caller awaiting close
/// without a timeout hung with nothing named. This says which of `value`/`drawing`/`parent` refused
/// and with how many reads still out.
pub fn record_close_store_step(outcome: &str) {
    #[cfg(any(test, feature = "testkit"))]
    {
        *LAST_CLOSE_STORE_STEP.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(outcome.to_owned());
    }
    let _ = outcome;
}

/// 🔬️ The last owned-Store retirement step of this process.
#[cfg(any(test, feature = "testkit"))]
pub fn last_close_store_step() -> Option<String> {
    LAST_CLOSE_STORE_STEP.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone()
}

/// 🧾️ The recorded reason of the turn the driver is leaving on, so the driver's own exit record can
/// carry the arm that produced it instead of overwriting it. Reads the empty string outside tests,
/// where nothing is recorded at all.
pub fn recorded_abandoned_assembly_turn() -> String {
    #[cfg(any(test, feature = "testkit"))]
    {
        return LAST_ABANDONED_ASSEMBLY_TURN.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone().unwrap_or_else(|| String::from("unrecorded"));
    }
    #[cfg(not(test))]
    String::new()
}

/// 🧾️ A committed publication receipt: only a real committed-WAL witness may reconcile the outbox.
pub struct GisMapApprovalReceiptV1 {
    pub witness: CommittedInferenceWalWitnessV1,
    pub document_generation: u64,
    pub applied: bool,
    pub undo: semio_hub_inference::schema::GisMapApprovalUndoHandleV1,
    pub frontier: directory::os_directory::EditedArtifactFrontierV1,
}

/// 🎁️ Public approval result stripped of the private WAL witness.
pub struct GisMapApprovalOutcomeV1 {
    pub applied: bool,
    pub undo: semio_hub_inference::schema::GisMapApprovalUndoHandleV1,
}

pub struct GisMapPreparedUndoCommandV1 {
    pub operation_id: String,
    pub proposal_hash: String,
    pub mutation_id: String,
    pub command_hash: String,
    pub command: InferencePrivateBytesV1,
}

/// ↩️ Internal durable inverse outcome with its second committed-WAL witness.
pub struct GisMapApprovalUndoCommitReceiptV1 {
    pub witness: CommittedInferenceWalWitnessV1,
    pub document_generation: u64,
    pub applied: bool,
    pub frontier: directory::os_directory::EditedArtifactFrontierV1,
}

/// 📸️ Exact post-decision actor/store state offered to the process-owned checkpoint publisher.
pub struct GisMapApprovalCheckpointRequestV1 {
    pub scope: DocumentScope,
    pub descriptor_digest: String,
    pub base_frontier: ArtifactFrontier,
    pub handle: db::ArtifactHandle,
    pub actor_snapshot: db::CheckpointPublicationSnapshot,
    pub pair: semio_hub_artifact_authority::ArtifactPair,
    pub journal_receipt: directory::os_store::durable_group::DurableOwnedGroupJournalReceiptV1,
}

/// 📣️ Process boundary that must durably publish the exact post-decision pair before approval.
pub trait GisMapApprovalCheckpointPublisherV1: Send + Sync {
    fn publish<'a>(
        &'a self,
        request: GisMapApprovalCheckpointRequestV1,
        document_write: Arc<GisMapDocumentWriteAuthorityV1>,
        attempt_lifetime_ms: u64,
        decision_now_ms: u64,
    ) -> std::pin::Pin<Box<dyn Future<Output = Result<directory::os_directory::PublishedArtifactCheckpoint, GisMapApprovalCommitErrorV1>> + Send + 'a>>;

    /// 📡️ Announces an already-published and ledger-applied checkpoint without yielding or releasing DocumentWrite.
    fn checkpoint_applied(&self, checkpoint: &directory::os_directory::PublishedArtifactCheckpoint) -> Result<(), GisMapApprovalCommitErrorV1>;
}

/// 🔌️ The private port an atomic parent+existing-child composition transaction implements.
///
/// Its contract is deliberately narrow: it receives one already-prepared server-stamped envelope and
/// must either refuse, or publish that exact envelope in a single visibility flip and return the
/// committed-WAL witness minted by [`super::super::wal::InferenceWalVerifierV1`]. Because the witness has
/// no public constructor, an implementation cannot fabricate durability, and the ledger outbox is
/// reconciled only against that real proof. It must never use `ArtifactHandle::submit` or the
/// generic `db.pathmap.v1` receiver, and it must never apply anything without explicit approval.
pub trait GisMapApprovalCommitterV1: Send + Sync {
    fn commit<'a>(&'a self, request: GisMapApprovalCommitRequestV1<'a>) -> std::pin::Pin<Box<dyn Future<Output = Result<GisMapApprovalReceiptV1, GisMapApprovalCommitErrorV1>> + Send + 'a>>;
    fn undo<'a>(&'a self, request: GisMapApprovalUndoCommitRequestV1<'a>) -> std::pin::Pin<Box<dyn Future<Output = Result<GisMapApprovalUndoCommitReceiptV1, GisMapApprovalCommitErrorV1>> + Send + 'a>>;
    fn close<'a>(&'a self) -> std::pin::Pin<Box<dyn Future<Output = Result<(), GisMapApprovalCommitErrorV1>> + Send + 'a>>;

    /// ⏰️ The document's OWN hybrid logical clock, as the retained parent Store carries it, or
    /// `None` while no Store is mounted for this scope — which is exactly the genesis case, where the
    /// document has no history to be after. The approval stamp is minted from this, never from the
    /// hub's wall clock alone, so the stamp the command hash is computed over is strictly after the
    /// document's last edit by construction.
    fn document_clock<'a>(&'a self, scope: &'a DocumentScope) -> std::pin::Pin<Box<dyn Future<Output = Option<protocol::HybridLogicalTimestamp>> + Send + 'a>>;
}

/// 🚧️ Fail-closed committer for every deployment where no composition transaction is registered.
///
/// A first single-document committer is deliberately NOT registered here. `bounds_proposal` produces
/// the parent `CreateRegion` alone, while `create_region_group_work` shows the semantically complete
/// approval is a fixed three-member group (parent + `gismap-drawing` `CreateNode` + `gismap-value`
/// `insertListItem`); the Map's own apply function does not keep the two children in sync. Publishing
/// the parent alone would therefore durably corrupt any Map that owns children, so
/// `commit_prepared_approval` refuses a child-bearing Map before a committer is ever consulted, and
/// the remaining zero-children case waits for the typed composition transaction rather than being
/// shortcut through the generic document receiver.
pub struct UnavailableGisMapApprovalCommitterV1;

impl GisMapApprovalCommitterV1 for UnavailableGisMapApprovalCommitterV1 {
    fn commit<'a>(&'a self, _request: GisMapApprovalCommitRequestV1<'a>) -> std::pin::Pin<Box<dyn Future<Output = Result<GisMapApprovalReceiptV1, GisMapApprovalCommitErrorV1>> + Send + 'a>> {
        Box::pin(async { Err(GisMapApprovalCommitErrorV1::Unavailable) })
    }

    fn undo<'a>(&'a self, _request: GisMapApprovalUndoCommitRequestV1<'a>) -> std::pin::Pin<Box<dyn Future<Output = Result<GisMapApprovalUndoCommitReceiptV1, GisMapApprovalCommitErrorV1>> + Send + 'a>> {
        Box::pin(async { Err(GisMapApprovalCommitErrorV1::Unavailable) })
    }

    fn close<'a>(&'a self) -> std::pin::Pin<Box<dyn Future<Output = Result<(), GisMapApprovalCommitErrorV1>> + Send + 'a>> {
        Box::pin(async { Ok(()) })
    }

    fn document_clock<'a>(&'a self, _scope: &'a DocumentScope) -> std::pin::Pin<Box<dyn Future<Output = Option<protocol::HybridLogicalTimestamp>> + Send + 'a>> {
        Box::pin(async { None })
    }
}
